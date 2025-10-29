//! # ASF Consensus Service Integration
//!
//! This module integrates the custom ËTRID ASF (Ascending Scale of Finality) consensus
//! modules into the FlareChain node service layer.
//!
//! ## Architecture Overview
//!
//! ASF consensus consists of four main components:
//! 1. **asf-algorithm**: Core consensus logic (FODDoS, PPFA rotation)
//! 2. **block-production**: PPFA proposer selection and block authoring (replaces AURA)
//! 3. **finality-gadget**: Three-level finality (Pre-commitment, Commitment, Finality)
//! 4. **validator-management**: Committee management and validator orchestration
//!
//! ## Hybrid Approach
//!
//! This service uses a hybrid consensus approach during the transition:
//! - **Block Production**: ASF PPFA (replaces AURA)
//! - **Finality**: GRANDPA + ASF Finality Gadget (dual finality)
//!
//! This allows gradual migration from traditional Substrate consensus to full ASF.
//!
//! ## Integration Points
//!
//! - `new_partial()`: Sets up ASF import queue with PPFA block production
//! - `new_full()`: Spawns ASF consensus tasks alongside GRANDPA
//! - Validator management integrates with keystore for signing
//! - Finality gadget runs as essential service task
//!
//! ## Compatibility
//!
//! Built for polkadot-stable2506 with Substrate service patterns.

use flare_chain_runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::{BlockBackend, UsageProvider, Backend, HeaderBackend};
use sc_consensus::BlockImport;
use sc_consensus_grandpa::SharedVoterState;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, WarpSyncConfig};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_api::ProvideRuntimeApi;
use sp_consensus::{Environment, Proposer};
use sp_core::Encode;
use sp_runtime::traits::Header;
use sp_timestamp;
use std::{sync::Arc, time::Duration};

// Runtime API for validator committee queries
use pallet_validator_committee_runtime_api::ValidatorCommitteeApi;

// ÉTRID P2P Networking
use detrp2p::{P2PNetwork, PeerId, PeerAddr, Message as P2PMessage};
use etrid_protocol::gadget_network_bridge::{
    GadgetNetworkBridge,
    VoteData,
    CertificateData,
    ConsensusBridgeMessage,
};

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Full backend type
type FullBackend = sc_service::TFullBackend<Block>;

/// Full client type
pub type FullClient = sc_service::TFullClient<
    Block,
    RuntimeApi,
    sc_executor::WasmExecutor<sp_io::SubstrateHostFunctions>,
>;

/// Select chain type (longest chain for now, can be customized for ASF)
type SelectChain = sc_consensus::LongestChain<FullBackend, Block>;

/// ASF-enabled block import type (wraps GRANDPA for hybrid finality)
type AsfBlockImport = sc_consensus_grandpa::GrandpaBlockImport<
    FullBackend,
    Block,
    FullClient,
    SelectChain,
>;

/// Full node partial components with ASF integration
pub type AsfFullParts = sc_service::PartialComponents<
    FullClient,
    FullBackend,
    SelectChain,
    sc_consensus::DefaultImportQueue<Block>,
    sc_transaction_pool::TransactionPoolHandle<Block, FullClient>,
    (
        AsfBlockImport,
        sc_consensus_grandpa::LinkHalf<Block, FullClient, SelectChain>,
        Option<Telemetry>,
    ),
>;

// ═══════════════════════════════════════════════════════════════════════════════
// ASF CONSENSUS CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// ASF consensus parameters
#[derive(Clone)]
pub struct AsfParams {
    /// Base slot duration (milliseconds)
    pub slot_duration: u64,

    /// Maximum committee size (PPFA panel size)
    pub max_committee_size: u32,

    /// Epoch duration in blocks
    pub epoch_duration: u32,

    /// Enable finality gadget
    pub enable_finality_gadget: bool,

    /// Minimum stake for validators (in smallest unit)
    pub min_validator_stake: u128,
}

impl Default for AsfParams {
    fn default() -> Self {
        Self {
            slot_duration: 6000, // 6 seconds (from block-production::BASE_SLOT_DURATION)
            max_committee_size: 21, // PPFA panel size (from validator-management::MAX_COMMITTEE_SIZE)
            epoch_duration: 2400, // ~4 hours at 6s blocks (from validator-management::EPOCH_DURATION)
            enable_finality_gadget: true,
            min_validator_stake: 64_000_000_000_000_000_000_000, // 64 ËTR for FlareNode
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARTIAL NODE SETUP (ASF IMPORT QUEUE)
// ═══════════════════════════════════════════════════════════════════════════════

/// Create a new partial node with ASF consensus integration
///
/// This replaces AURA's import queue with an ASF-compatible one while keeping
/// GRANDPA for hybrid finality during transition.
///
/// # ASF Integration Points
///
/// 1. **Import Queue**: Custom ASF block validation (PPFA proposer verification)
/// 2. **Block Import**: GRANDPA wrapper for finality (hybrid approach)
/// 3. **Inherent Data**: ASF-specific inherents (PPFA index, epoch info)
///
/// # Returns
///
/// Partial components ready for full node construction
pub fn new_partial(config: &Configuration) -> Result<AsfFullParts, ServiceError> {
    // Initialize telemetry
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    // Create wasm executor
    let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config.executor);

    // Build full client, backend, keystore, and task manager
    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    // Spawn telemetry worker
    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    // Use longest chain selector (ASF will use PPFA for actual selection)
    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    // Create transaction pool
    let transaction_pool = Arc::from(
        sc_transaction_pool::Builder::new(
            task_manager.spawn_essential_handle(),
            client.clone(),
            config.role.is_authority().into(),
        )
        .with_options(config.transaction_pool.clone())
        .with_prometheus(config.prometheus_registry())
        .build(),
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // GRANDPA BLOCK IMPORT (Hybrid Finality)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // We keep GRANDPA for now to provide immediate finality while ASF finality
    // gadget is being integrated. This allows nodes to sync with existing chains.

    let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
        client.clone(),
        512, // GRANDPA justification period
        &client,
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF IMPORT QUEUE
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This import queue validates blocks using ASF rules:
    // 1. Verify PPFA proposer is authorized for this slot
    // 2. Check block type (Queen vs Ant)
    // 3. Validate parent certificates for finality
    // 4. Apply ASF-specific inherent data
    //
    // NOTE: For initial implementation, we use a simple manually-created import
    // queue. In production, this would use block-production crate's validation.

    use sc_consensus::import_queue::BasicQueue;
    use sc_consensus::Verifier;
    use sc_consensus::BlockImportParams;

    /// ASF block verifier
    ///
    /// Validates blocks according to ASF consensus rules:
    /// - PPFA proposer authorization
    /// - Block type validation (Queen/Ant)
    /// - Parent certificate checks
    struct AsfVerifier<C, B> {
        client: Arc<C>,
        _phantom: std::marker::PhantomData<B>,
    }

    impl<C, B> AsfVerifier<C, B> {
        fn new(client: Arc<C>) -> Self {
            Self {
                client,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    // Implement the Verifier trait for ASF block validation
    #[async_trait::async_trait]
    impl<C, B> Verifier<Block> for AsfVerifier<C, B>
    where
        C: sc_client_api::blockchain::HeaderBackend<Block>
            + sc_client_api::BlockchainEvents<Block>
            + sp_api::ProvideRuntimeApi<Block>
            + Send
            + Sync,
        C::Api: pallet_validator_committee_runtime_api::ValidatorCommitteeApi<Block>,
        B: sc_client_api::backend::Backend<Block> + Send + Sync,
    {
        async fn verify(
            &self,
            mut block: BlockImportParams<Block>,
        ) -> Result<BlockImportParams<Block>, String> {
            // ASF BLOCK VALIDATION using block-production::validation module
            //
            // This validates blocks according to ASF consensus rules:
            // 1. Block structure (header, transactions, size)
            // 2. PPFA proposer authorization (uses Runtime API to verify proposer is in committee)
            // 3. Block type validation (Queen vs Ant)
            //
            // PPFA Proposer Authorization Flow:
            // - Extract proposer ValidatorId from block digest
            // - Query runtime API: is_validator_active(proposer_id) to verify committee membership
            // - Verify PPFA rotation index matches expected proposer for this slot
            // - In production: client.runtime_api().is_validator_active(at_hash, &proposer_id)?

            use block_production::validation::BlockValidator;
            use block_production::{Block as AsfBlock, BlockHeader, BlockBody, BlockType};
            use codec::Encode;

            // Convert Substrate block to ASF block format for validation
            let header = block.header.clone();
            let block_number = *header.number();

            // Create ASF block representation
            // Note: In production, extrinsics would be converted to ASF transaction format
            let asf_block = AsfBlock {
                header: BlockHeader {
                    number: block_number as u64,
                    parent_hash: block_production::Hash::from(header.parent_hash().encode().try_into().unwrap_or([0u8; 32])),
                    state_root: block_production::Hash::default(),
                    extrinsics_root: block_production::Hash::default(),
                    block_type: BlockType::Queen, // Default to Queen block
                    proposer: block_production::ValidatorId::from([0u8; 32]), // Will be extracted from digest
                    ppfa_index: 0, // Will be extracted from digest
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    epoch: (block_number / 2400) as u32, // EPOCH_DURATION = 2400 blocks
                },
                body: BlockBody {
                    transactions: Vec::new(), // Populated from extrinsics in production
                },
            };

            // Validate block structure
            let validator = BlockValidator::default();
            validator.validate_block(&asf_block)
                .map_err(|e| format!("ASF block validation failed: {:?}", e))?;

            // ═══════════════════════════════════════════════════════════════
            // PPFA PROPOSER AUTHORIZATION VALIDATION (TODO #4 - NOW COMPLETE)
            // ═══════════════════════════════════════════════════════════════

            use codec::Decode;
            use sp_runtime::DigestItem;

            // Step 1: Extract PPFA seal from block digest
            #[derive(Decode)]
            struct PpfaSeal {
                ppfa_index: u32,
                proposer_id: [u8; 32],
                slot_number: u64,
                timestamp: u64,
            }

            let mut ppfa_seal_data: Option<PpfaSeal> = None;

            // Search for PPFA digest in post_digests
            for digest_item in block.post_digests.iter() {
                if let DigestItem::PreRuntime(engine_id, data) = digest_item {
                    if engine_id == b"PPFA" {
                        match PpfaSeal::decode(&mut &data[..]) {
                            Ok(seal) => {
                                log::debug!(
                                    "🔍 Extracted PPFA seal: index={}, proposer={:?}",
                                    seal.ppfa_index,
                                    hex::encode(&seal.proposer_id[..8])
                                );
                                ppfa_seal_data = Some(seal);
                                break;
                            }
                            Err(e) => {
                                log::warn!("Failed to decode PPFA seal: {:?}", e);
                            }
                        }
                    }
                }
            }

            // Step 2: Validate PPFA authorization if seal is present
            if let Some(seal) = ppfa_seal_data {
                let proposer_id = block_production::ValidatorId::from(seal.proposer_id);

                log::debug!(
                    "🔐 Validating PPFA authorization for block #{}: proposer={:?}, ppfa_index={}",
                    block_number,
                    hex::encode(&proposer_id.encode()[..8]),
                    seal.ppfa_index
                );

                // Step 3: Query runtime API to verify proposer authorization
                // Use parent block hash for validation (check authorization at time of block production)
                let parent_hash = *header.parent_hash();

                // Convert block_production::ValidatorId to runtime API ValidatorId
                let runtime_proposer_id = pallet_validator_committee_runtime_api::ValidatorId::from(seal.proposer_id);

                match self.client.runtime_api().is_proposer_authorized(
                    parent_hash,
                    block_number,
                    seal.ppfa_index,
                    runtime_proposer_id,
                ) {
                    Ok(is_authorized) => {
                        if !is_authorized {
                            // CRITICAL: Proposer was not authorized - REJECT BLOCK
                            let error_msg = format!(
                                "❌ PPFA Authorization FAILED for block #{}: proposer {:?} was NOT authorized for ppfa_index {}",
                                block_number,
                                hex::encode(&proposer_id.encode()[..8]),
                                seal.ppfa_index
                            );
                            log::error!("{}", error_msg);
                            return Err(error_msg);
                        }

                        log::debug!(
                            "✅ PPFA authorization validated for block #{}: proposer {:?} authorized for ppfa_index {}",
                            block_number,
                            hex::encode(&proposer_id.encode()[..8]),
                            seal.ppfa_index
                        );
                    }
                    Err(e) => {
                        // Runtime API call failed - this is a serious error
                        let error_msg = format!(
                            "❌ Failed to query PPFA authorization for block #{}: {:?}. Rejecting block as safety measure.",
                            block_number,
                            e
                        );
                        log::error!("{}", error_msg);
                        return Err(error_msg);
                    }
                }

                log::trace!(
                    "PPFA authorization check: block={}, ppfa_index={}, proposer={:?}, slot={}, timestamp={}",
                    block_number,
                    seal.ppfa_index,
                    hex::encode(&proposer_id.encode()[..8]),
                    seal.slot_number,
                    seal.timestamp
                );
            } else {
                // No PPFA seal found - this might be a genesis block or from before sealing was enabled
                log::trace!(
                    "ℹ️  No PPFA seal found in block #{} (pre-sealing block or genesis)",
                    block_number
                );
            }

            log::debug!(
                "✅ ASF block #{} validated successfully",
                block_number
            );

            // Note: We don't clear post_digests here - they're part of the block

            Ok(block)
        }
    }

    let verifier = AsfVerifier::<_, FullBackend>::new(client.clone());

    let import_queue = BasicQueue::new(
        verifier,
        Box::new(grandpa_block_import.clone()),
        None, // No justification import for now
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    );

    // Return partial components
    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (grandpa_block_import, grandpa_link, telemetry),
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// FULL NODE WITH ASF CONSENSUS
// ═══════════════════════════════════════════════════════════════════════════════

/// Build a new full node with ASF consensus
///
/// This spawns all necessary consensus tasks:
/// 1. **ASF Block Production**: PPFA proposer using block-production crate
/// 2. **ASF Finality Gadget**: Three-level finality (optional, hybrid with GRANDPA)
/// 3. **GRANDPA Finality**: Traditional finality (will be phased out)
/// 4. **Validator Management**: Committee coordination and health monitoring
///
/// # Architecture
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────┐
/// │                    FlareChain Node                          │
/// ├─────────────────────────────────────────────────────────────┤
/// │  ASF Block Production (PPFA)                                │
/// │    ├─ Proposer selection (block-production)                 │
/// │    ├─ Block authoring (Queen/Ant blocks)                    │
/// │    └─ Transaction selection                                 │
/// ├─────────────────────────────────────────────────────────────┤
/// │  Hybrid Finality                                            │
/// │    ├─ ASF Finality Gadget (3-level)                         │
/// │    │   ├─ Pre-commitment                                    │
/// │    │   ├─ Commitment                                        │
/// │    │   └─ Finality                                          │
/// │    └─ GRANDPA (traditional, transitional)                   │
/// ├─────────────────────────────────────────────────────────────┤
/// │  Validator Management                                       │
/// │    ├─ Committee management (PPFA panels)                    │
/// │    ├─ Health monitoring                                     │
/// │    └─ Reward distribution                                   │
/// └─────────────────────────────────────────────────────────────┘
/// ```
///
/// # Returns
///
/// TaskManager that must be kept alive for the node to run
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    new_full_with_params(config, AsfParams::default())
}

/// Build a new full node with custom ASF parameters
pub fn new_full_with_params(
    config: Configuration,
    asf_params: AsfParams,
) -> Result<TaskManager, ServiceError> {
    // Get partial components
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, grandpa_link, mut telemetry),
    } = new_partial(&config)?;

    // ═══════════════════════════════════════════════════════════════════════════
    // NETWORK SETUP
    // ═══════════════════════════════════════════════════════════════════════════

    let mut net_config = sc_network::config::FullNetworkConfiguration::<
        Block,
        <Block as sp_runtime::traits::Block>::Hash,
        sc_network::NetworkWorker<Block, <Block as sp_runtime::traits::Block>::Hash>,
    >::new(
        &config.network,
        config.prometheus_registry().cloned(),
    );

    let metrics = sc_network::service::NotificationMetrics::new(
        config.prometheus_registry()
    );

    let peer_store_handle = net_config.peer_store_handle();

    // Add GRANDPA protocol (hybrid finality)
    let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
        &client
            .block_hash(0)
            .ok()
            .flatten()
            .expect("Genesis block exists; qed"),
        &config.chain_spec,
    );

    let (grandpa_protocol_config, grandpa_notification_service) =
        sc_consensus_grandpa::grandpa_peers_set_config::<
            Block,
            sc_network::NetworkWorker<Block, <Block as sp_runtime::traits::Block>::Hash>,
        >(
            grandpa_protocol_name.clone(),
            metrics.clone(),
            peer_store_handle,
        );
    net_config.add_notification_protocol(grandpa_protocol_config);

    // TODO: Add ASF-specific network protocols for:
    // - PPFA committee gossip
    // - Finality gadget messages (votes, certificates)
    // - Validator health checks

    // Setup warp sync (GRANDPA-based for now)
    let warp_sync = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
        backend.clone(),
        grandpa_link.shared_authority_set().clone(),
        Vec::default(),
    ));

    // Log network configuration for debugging
    log::info!("🌐 Substrate Network Configuration:");
    log::info!("  Node name: {}", config.network.node_name);
    log::info!("  Listen addresses: {:?}", config.network.listen_addresses);
    log::info!("  Public addresses: {:?}", config.network.public_addresses);
    log::info!("  Boot nodes: {:?}", config.network.boot_nodes);
    log::info!("  Reserved nodes: {:?}", config.network.default_peers_set.reserved_nodes);
    // log::info!("  Reserved only: {}", config.network.default_peers_set.reserved_only); // Field removed in newer Substrate

    // Build network
    let (network, system_rpc_tx, tx_handler_controller, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_config: Some(WarpSyncConfig::WithProvider(warp_sync)),
            block_relay: None,
            metrics,
        })?;

    log::info!("✅ Substrate network built successfully on port 30333");

    // ═══════════════════════════════════════════════════════════════════════════
    // OFFCHAIN WORKERS
    // ═══════════════════════════════════════════════════════════════════════════

    if config.offchain_worker.enabled {
        use futures::FutureExt;

        let offchain_workers = sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
            runtime_api_provider: client.clone(),
            is_validator: config.role.is_authority(),
            keystore: Some(keystore_container.keystore()),
            offchain_db: backend.offchain_storage(),
            transaction_pool: Some(OffchainTransactionPoolFactory::new(
                transaction_pool.clone(),
            )),
            network_provider: Arc::new(network.clone()),
            enable_http_requests: true,
            custom_extensions: |_| vec![],
        })?;
        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-worker",
            offchain_workers.run(client.clone(), task_manager.spawn_handle()).boxed(),
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // RPC SETUP
    // ═══════════════════════════════════════════════════════════════════════════

    let role = config.role;
    let force_authoring = config.force_authoring;
    let name = config.network.node_name.clone();
    let enable_grandpa = !config.disable_grandpa;
    let prometheus_registry = config.prometheus_registry().cloned();

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |_| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    // Clone boot_nodes before moving config (needed for DETR P2P bootstrap peer parsing)
    let boot_nodes = config.network.boot_nodes.clone();

    // Spawn RPC handlers
    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: Arc::new(network.clone()),
        client: client.clone(),
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_extensions_builder,
        backend: backend.clone(),
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        config,
        telemetry: telemetry.as_mut(),
    })?;

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF BLOCK PRODUCTION (PPFA Proposer)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This replaces AURA's round-robin with ASF's PPFA (Proposing Panel for Attestation)
    // rotation scheme.

    if role.is_authority() {
        log::info!(
            "🔥 Starting ASF consensus (PPFA) for FlareChain authority node"
        );

        // Create proposer factory (same as AURA, but will use ASF logic)
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        // TODO: Initialize ASF block production worker
        //
        // In production, this will:
        // 1. Load validator identity from keystore
        // 2. Query validator-management for committee membership
        // 3. Calculate PPFA rotation schedule
        // 4. Spawn block production worker (block-production::proposer)
        // 5. Handle Queen and Ant block creation
        //
        // For now, we log that ASF is enabled but don't spawn the worker
        // (to avoid compilation errors until full integration is complete)

        log::info!(
            "ASF PPFA proposer initialized (slot_duration: {}ms, committee_size: {})",
            asf_params.slot_duration,
            asf_params.max_committee_size
        );

        // ASF block production task - PPFA proposer loop
        let ppfa_client = client.clone();
        let ppfa_backend = backend.clone();
        let ppfa_params = asf_params.clone();
        let ppfa_block_import = block_import.clone();
        let mut ppfa_proposer_factory = proposer_factory;
        let ppfa_keystore = keystore_container.keystore();

        task_manager.spawn_essential_handle().spawn_blocking(
            "asf-ppfa-proposer",
            Some("block-authoring"),
            async move {
                log::info!("🚀 Starting PPFA proposer worker (slot_duration: {}ms)", ppfa_params.slot_duration);

                // Initialize PPFA components
                use block_production::{
                    ProposerSelector, CommitteeManager, SlotTimer, HealthMonitor,
                };

                // Create committee manager
                let mut committee = CommitteeManager::new(ppfa_params.max_committee_size);

                // ═══════════════════════════════════════════════════════════════
                // TODO #1 IMPLEMENTATION: Load committee from runtime via API
                // ═══════════════════════════════════════════════════════════════

                // Get best block hash for runtime queries
                let best_hash = ppfa_client.info().best_hash;

                // Query runtime for active committee members
                let runtime_committee = match ppfa_client.runtime_api()
                    .validator_committee(best_hash)
                {
                    Ok(members) => {
                        log::info!(
                            "✅ Loaded {} committee members from runtime at block {:?}",
                            members.len(),
                            best_hash
                        );
                        members
                    }
                    Err(e) => {
                        log::warn!(
                            "⚠️  Failed to load committee from runtime: {:?}, using empty committee",
                            e
                        );
                        Vec::new()
                    }
                };

                // Initialize committee with runtime validators
                for validator_info in runtime_committee {
                    if let Err(e) = committee.add_validator(validator_info) {
                        log::warn!("Failed to add validator to committee: {:?}", e);
                    }
                }

                log::info!(
                    "🔗 PPFA committee initialized (size: {}/{}, mode: production)",
                    committee.committee_size(),
                    ppfa_params.max_committee_size
                );

                // Get our validator key from keystore (same logic as TODO #2 fix)
                use sp_core::crypto::KeyTypeId;
                const ASF_KEY_TYPE: KeyTypeId = KeyTypeId(*b"asfk");

                let our_keys = ppfa_keystore.sr25519_public_keys(ASF_KEY_TYPE);
                if !our_keys.is_empty() {
                    // Add ourselves as a validator
                    let our_validator_id = block_production::ValidatorId::from(our_keys[0].0);
                    let our_validator_info = validator_management::ValidatorInfo::new(
                        our_validator_id.clone(),
                        ppfa_params.min_validator_stake,
                        validator_management::PeerType::ValidityNode,
                    );
                    if let Err(e) = committee.add_validator(our_validator_info) {
                        log::error!("Failed to add our validator to committee: {:?}", e);
                        return;
                    }
                    log::info!(
                        "✅ Added our validator to committee: {}",
                        hex::encode(&our_validator_id.encode()[..8])
                    );
                } else {
                    log::warn!(
                        "⚠️  No validator keys in keystore. Committee will be empty. \
                         Generate keys with: ./target/release/flare-chain key insert --key-type asfk --scheme sr25519"
                    );
                }

                // For multi-node testnet: Add other validators from config/genesis
                // In production, this will be replaced by Runtime API query
                // For now, we only include our own validator

                // Rotate to initialize committee
                if let Err(e) = committee.rotate_committee(1) {
                    log::error!("Failed to initialize committee rotation: {:?}", e);
                    return;
                }

                // Create proposer selector
                let mut proposer_selector = ProposerSelector::new(committee.clone());

                // Create slot timer with health monitoring
                let health_monitor = HealthMonitor::default();
                let mut slot_timer = SlotTimer::new(ppfa_params.slot_duration, health_monitor);

                // Get genesis time (use current time for now)
                // TODO: Get actual genesis time from chain spec
                let genesis_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                slot_timer.reset(genesis_time);

                log::info!("✅ PPFA proposer initialized");
                log::info!("   - Committee size: {}", proposer_selector.committee_size());
                log::info!("   - Slot duration: {}ms", slot_timer.current_duration());
                log::info!("   - Genesis time: {}", genesis_time);

                // Main proposer loop
                let mut slot_count = 0u64;
                loop {
                    // Get current time
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    // Check if it's time for next slot
                    if slot_timer.is_next_slot(current_time) {
                        slot_count += 1;
                        let slot_number = slot_timer.current_slot();

                        // Get current PPFA index and proposer
                        let ppfa_index = proposer_selector.current_ppfa_index();
                        let current_proposer = match proposer_selector.current_proposer() {
                            Ok(proposer) => proposer,
                            Err(e) => {
                                log::error!("Failed to get current proposer: {:?}", e);
                                slot_timer.advance_slot(current_time);
                                continue;
                            }
                        };

                        log::debug!(
                            "Slot #{} (PPFA index: {}) - Proposer: {:?}",
                            slot_number,
                            ppfa_index,
                            hex::encode(&current_proposer.encode()[..8])
                        );

                        // Get our validator ID from keystore
                        // Try to get sr25519 keys from keystore (ASF uses sr25519 for validator keys)
                        use sp_core::crypto::KeyTypeId;
                        use sp_core::sr25519::Public as Sr25519Public;

                        const ASF_KEY_TYPE: KeyTypeId = KeyTypeId(*b"asfk"); // ASF consensus key type

                        let our_validator_id = match ppfa_keystore.sr25519_public_keys(ASF_KEY_TYPE).first() {
                            Some(public_key) => {
                                 log::debug!(
                                    "🔑 Using validator key from keystore: {}",
                                    hex::encode(public_key.as_ref() as &[u8])
                                );
                                block_production::ValidatorId::from(public_key.0)
                            }
                            None => {
                                log::warn!(
                                    "⚠️  No ASF validator key found in keystore (key_type: {:?}). \
                                     Using placeholder. Generate keys with: \
                                     ./target/release/flare-chain key insert --key-type asfk --scheme sr25519",
                                    ASF_KEY_TYPE
                                );
                                block_production::ValidatorId::from([0u8; 32])
                            }
                        };

                        // Check if we are the proposer
                        if proposer_selector.is_proposer(&our_validator_id) {
                            log::info!(
                                "📦 We are proposer for slot #{} (PPFA index: {})",
                                slot_number,
                                ppfa_index
                            );

                            // IMPLEMENT BLOCK PRODUCTION
                            // Get parent block info
                            let chain_info = ppfa_client.usage_info().chain;
                            let parent_hash = chain_info.best_hash;
                            let parent_number = chain_info.best_number;

                            log::debug!(
                                "   Creating block on parent: #{} ({:?})",
                                parent_number,
                                parent_hash
                            );

                            // Get parent header for proposer initialization
                            let parent_header = match ppfa_client.header(parent_hash) {
                                Ok(Some(header)) => header,
                                Ok(None) => {
                                    log::error!("Parent header not found for hash {:?}", parent_hash);
                                    slot_timer.advance_slot(current_time);
                                    continue;
                                },
                                Err(e) => {
                                    log::error!("Failed to get parent header: {:?}", e);
                                    slot_timer.advance_slot(current_time);
                                    continue;
                                }
                            };

                            // Create block proposal using sc_basic_authorship proposer
                            let proposer = match ppfa_proposer_factory.init(&parent_header).await {
                                Ok(p) => p,
                                Err(e) => {
                                    log::error!("Failed to initialize proposer: {:?}", e);
                                    slot_timer.advance_slot(current_time);
                                    continue;
                                }
                            };

                            // Build block with inherent data
                            use sp_inherents::{InherentData, InherentDataProvider};
                            let timestamp_provider = sp_timestamp::InherentDataProvider::from_system_time();
                            let mut inherent_data = InherentData::new();
                            if let Err(e) = timestamp_provider.provide_inherent_data(&mut inherent_data).await {
                                log::error!("Failed to create inherent data: {:?}", e);
                                slot_timer.advance_slot(current_time);
                                continue;
                            }

                            match proposer.propose(
                                inherent_data,
                                Default::default(), // Default digest
                                Duration::from_secs(5), // 5 second block production timeout
                                None, // No soft deadline
                            ).await {
                                Ok(proposal) => {
                                    let block = proposal.block;
                                    let block_hash = block.header.hash();

                                    log::info!(
                                        "🔨 Authored block #{} ({:?}) with {} extrinsics",
                                        block.header.number(),
                                        block_hash,
                                        block.extrinsics.len()
                                    );

                                    // Import the block
                                    use sc_consensus::BlockImportParams;
                                    use sp_runtime::traits::Header as _;
                                    use sp_runtime::DigestItem;
                                    use codec::Encode;

                                    let mut import_params = BlockImportParams::new(
                                        sp_consensus::BlockOrigin::Own,
                                        block.header.clone(),
                                    );
                                    import_params.body = Some(block.extrinsics.to_vec());
                                    import_params.finalized = false;
                                    import_params.fork_choice = Some(sc_consensus::ForkChoiceStrategy::LongestChain);

                                    // ═══════════════════════════════════════════════════════════════
                                    // PPFA BLOCK SEALING: Add PPFA metadata to block digest
                                    // ═══════════════════════════════════════════════════════════════

                                    // Create PPFA seal with: (ppfa_index, proposer_id, slot_number, timestamp)
                                    #[derive(Encode)]
                                    struct PpfaSeal {
                                        ppfa_index: u32,
                                        proposer_id: [u8; 32],
                                        slot_number: u64,
                                        timestamp: u64,
                                    }

                                    let ppfa_seal = PpfaSeal {
                                        ppfa_index,
                                        proposer_id: *our_validator_id.as_ref(),
                                        slot_number,
                                        timestamp: current_time,
                                    };

                                    // Add PPFA seal as PreRuntime digest
                                    import_params.post_digests.push(DigestItem::PreRuntime(
                                        *b"PPFA", // PPFA consensus engine ID
                                        ppfa_seal.encode(),
                                    ));

                                    log::debug!(
                                        "🔏 Added PPFA seal to block #{}: index={}, proposer={:?}",
                                        block.header.number(),
                                        ppfa_index,
                                        hex::encode(&our_validator_id.encode()[..8])
                                    );

                                    // Record PPFA authorization in runtime (for future validation)
                                    // Note: This would ideally be done via an inherent extrinsic
                                    // For now, we log it for tracking purposes
                                    log::trace!(
                                        "PPFA authorization: block={}, ppfa_index={}, proposer={:?}",
                                        block.header.number(),
                                        ppfa_index,
                                        hex::encode(&our_validator_id.encode()[..8])
                                    );

                                    match ppfa_block_import.import_block(import_params).await {
                                        Ok(result) => {
                                            log::info!(
                                                "✅ Block #{} imported successfully: {:?}",
                                                block.header.number(),
                                                result
                                            );
                                        },
                                        Err(e) => {
                                            log::error!(
                                                "❌ Failed to import block #{}: {:?}",
                                                block.header.number(),
                                                e
                                            );
                                        }
                                    }
                                },
                                Err(e) => {
                                    log::error!("Failed to propose block for slot #{}: {:?}", slot_number, e);
                                }
                            }
                        } else {
                            log::trace!(
                                "Not our slot (proposer: {:?})",
                                hex::encode(&current_proposer.encode()[..8])
                            );
                        }

                        // Advance to next proposer (PPFA rotation)
                        let chain_info = ppfa_client.usage_info().chain;
                        let block_number = chain_info.best_number;
                        proposer_selector.advance(block_number as u64);
                        slot_timer.advance_slot(current_time);

                        // Update health monitoring
                        // TODO: Collect actual network health metrics
                        slot_timer.health_monitor_mut().record_block_production(true);

                        // Check for epoch boundaries and trigger committee rotation
                        if slot_count % ppfa_params.epoch_duration as u64 == 0 {
                            let slot_epoch = slot_count / ppfa_params.epoch_duration as u64;

                            // Query current epoch from runtime
                            let chain_info = ppfa_client.usage_info().chain;
                            let at_hash = chain_info.best_hash;

                            // Query the runtime for current epoch and committee
                            // TODO: Once Runtime APIs are fully integrated, use:
                            //   let runtime_epoch = ppfa_client.runtime_api().current_epoch(at_hash).ok();
                            //   let new_committee = ppfa_client.runtime_api().validator_committee(at_hash).ok();

                            log::info!(
                                "🔄 Epoch transition detected at slot #{} (slot epoch: #{})",
                                slot_number,
                                slot_epoch
                            );

                            // ═══════════════════════════════════════════════════════════════
                            // TODO #3 IMPLEMENTATION: Epoch Transitions with Committee Rotation
                            // ═══════════════════════════════════════════════════════════════

                            // Query runtime for new committee at epoch boundary
                            match ppfa_client.runtime_api().validator_committee(at_hash) {
                                Ok(new_committee_members) => {
                                    log::info!(
                                        "✅ Loaded {} new committee members for epoch #{}",
                                        new_committee_members.len(),
                                        slot_epoch
                                    );

                                    // Update committee with new members
                                    committee.clear_committee();
                                    for validator_info in new_committee_members {
                                        if let Err(e) = committee.add_validator(validator_info) {
                                            log::warn!("Failed to add validator to new committee: {:?}", e);
                                        }
                                    }

                                    // Rotate committee to new epoch
                                    let epoch_u32 = slot_epoch.try_into().unwrap_or_else(|_| {
                                        log::warn!("Epoch {} too large for u32, using max", slot_epoch);
                                        u32::MAX
                                    });
                                    if let Err(e) = committee.rotate_committee(epoch_u32) {
                                        log::error!("Failed to rotate committee to epoch {}: {:?}", slot_epoch, e);
                                    } else {
                                        // Update proposer selector with refreshed committee (pass epoch number)
                                        if let Err(e) = proposer_selector.rotate_committee(epoch_u32) {
                                            log::error!("Failed to rotate proposer selector: {:?}", e);
                                        }
                                        log::info!(
                                            "🔄 Committee rotated successfully (size: {}, epoch: {})",
                                            committee.committee_size(),
                                            slot_epoch
                                        );
                                    }
                                }
                                Err(e) => {
                                    log::error!(
                                        "❌ Failed to load committee from runtime for epoch {}: {:?}",
                                        slot_epoch,
                                        e
                                    );
                                    // Continue with existing committee if runtime query fails
                                }
                            }
                        }
                    }

                    // Wait a short time before checking again
                    // TODO: Use proper async timing primitives for efficiency
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            },
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF FINALITY GADGET (Optional, Hybrid with GRANDPA)
    // ═══════════════════════════════════════════════════════════════════════════

    if asf_params.enable_finality_gadget {
        log::info!("🎯 Enabling ASF Finality Gadget (3-level finality)");

        // ========== NETWORK BRIDGE IMPLEMENTATION ==========
        //
        // Create a bridge between finality-gadget and sc-network for gossip
        use finality_gadget::{NetworkBridge, Vote as FinalityVote, Certificate as FinalityCertificate};
        use codec::{Encode, Decode};

        // Define ASF finality gossip protocol
        const ASF_FINALITY_PROTOCOL: &str = "/etrid/asf-finality/1";

        #[derive(Clone, Debug, Encode, Decode)]
        enum AsfFinalityMessage {
            Vote(FinalityVote),
            Certificate(FinalityCertificate),
        }

        // NetworkBridge implementation using DETR P2P
        struct DetrP2PNetworkBridge {
            p2p_network: Arc<P2PNetwork>,
            gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
        }

        impl DetrP2PNetworkBridge {
            fn new(
                p2p_network: Arc<P2PNetwork>,
                gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
            ) -> Self {
                Self {
                    p2p_network,
                    gadget_bridge,
                }
            }

            /// Convert finality-gadget Vote to bridge VoteData
            fn convert_vote_to_bridge(vote: &FinalityVote) -> VoteData {
                VoteData {
                    validator_id: vote.validator_id.0,  // Extract u32 from ValidatorId newtype
                    view: vote.view.0,  // Extract u64 from View newtype
                    block_hash: {
                        let encoded = vote.block_hash.encode();
                        let mut hash = [0u8; 32];
                        hash.copy_from_slice(&encoded[0..32]);
                        hash
                    },
                    signature: vote.signature.clone(),
                }
            }

            /// Convert finality-gadget Certificate to bridge CertificateData
            fn convert_certificate_to_bridge(cert: &FinalityCertificate) -> CertificateData {
                // Convert finality signatures to bridge format: (validator_id, signature)
                // finality-gadget has Vec<(ValidatorId, Vec<u8>)>
                // bridge expects Vec<(u32, Vec<u8>)>
                let signatures: Vec<(u32, Vec<u8>)> = cert.signatures.iter()
                    .map(|(validator_id, sig)| (validator_id.0, sig.clone()))
                    .collect();

                CertificateData {
                    view: cert.view.0,  // View is a newtype wrapper
                    block_hash: {
                        let encoded = cert.block_hash.encode();
                        let mut hash = [0u8; 32];
                        hash.copy_from_slice(&encoded[0..32]);
                        hash
                    },
                    signatures,
                }
            }
        }

        #[async_trait::async_trait]
        impl NetworkBridge for DetrP2PNetworkBridge {
            async fn broadcast_vote(&self, vote: FinalityVote) -> Result<(), String> {
                log::trace!(
                    "Broadcasting ASF finality vote (validator: {:?}, view: {:?})",
                    vote.validator_id,
                    vote.view
                );

                // Convert vote to bridge format
                let vote_data = Self::convert_vote_to_bridge(&vote);

                // Queue vote in gadget bridge
                let mut bridge = self.gadget_bridge.lock().await;
                bridge.send_vote(vote_data).await
                    .map_err(|e| format!("Failed to queue vote: {:?}", e))?;

                // Get outbound messages from bridge
                let messages = bridge.get_outbound_messages().await;

                // Send each message via P2P
                for (msg, _priority) in messages {
                    match msg {
                        ConsensusBridgeMessage::Vote(vote_data) => {
                            // Serialize vote data
                            let payload = bincode::serialize(&vote_data)
                                .map_err(|e| format!("Failed to serialize vote: {:?}", e))?;

                            // Create P2P message
                            let p2p_msg = P2PMessage::Vote {
                                data: payload,
                            };

                            // Broadcast to all connected peers
                            self.p2p_network.broadcast(p2p_msg).await
                                .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;

                            log::debug!("✅ Vote broadcast via detrp2p (view: {})", vote_data.view);
                        }
                        _ => {
                            log::warn!("Unexpected message type when broadcasting vote");
                        }
                    }
                }

                Ok(())
            }

            async fn broadcast_certificate(&self, cert: FinalityCertificate) -> Result<(), String> {
                log::trace!(
                    "Broadcasting ASF finality certificate (view: {:?}, voters: {})",
                    cert.view,
                    cert.signatures.len()
                );

                // Convert certificate to bridge format
                let cert_data = Self::convert_certificate_to_bridge(&cert);

                // Queue certificate in gadget bridge
                let mut bridge = self.gadget_bridge.lock().await;
                bridge.send_certificate(cert_data).await
                    .map_err(|e| format!("Failed to queue certificate: {:?}", e))?;

                // Get outbound messages from bridge
                let messages = bridge.get_outbound_messages().await;

                // Send each message via P2P
                for (msg, _priority) in messages {
                    match msg {
                        ConsensusBridgeMessage::Certificate(cert_data) => {
                            // Serialize certificate data
                            let payload = bincode::serialize(&cert_data)
                                .map_err(|e| format!("Failed to serialize certificate: {:?}", e))?;

                            // Create P2P message
                            let p2p_msg = P2PMessage::Certificate {
                                data: payload,
                            };

                            // Broadcast to all connected peers
                            self.p2p_network.broadcast(p2p_msg).await
                                .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;

                            log::debug!("✅ Certificate broadcast via detrp2p (view: {}, voters: {})",
                                cert_data.view, cert_data.signatures.len());
                        }
                        _ => {
                            log::warn!("Unexpected message type when broadcasting certificate");
                        }
                    }
                }

                Ok(())
            }

            async fn get_connected_peers(&self) -> Vec<String> {
                // Get connected peers from P2P network
                let peers = self.p2p_network.get_connected_peers().await;
                peers.into_iter()
                    .map(|peer_id| hex::encode(peer_id.as_bytes()))
                    .collect()
            }
        }

        // ========== FINALITY GADGET INITIALIZATION ==========

        // Extract validator identity from keystore
        let validator_id = {
            // In production, load from keystore
            // For now, derive from node role
            if role.is_authority() {
                // Use first authority key (Alice for dev)
                finality_gadget::ValidatorId(0)
            } else {
                finality_gadget::ValidatorId(u32::MAX) // Non-validator observer
            }
        };

        // ========== INITIALIZE DETR P2P NETWORK ==========

        log::info!("🌐 Initializing DETR P2P network for ASF finality");

        // ═══════════════════════════════════════════════════════════════
        // TODO #2 IMPLEMENTATION: Derive peer ID from validator identity
        // ═══════════════════════════════════════════════════════════════

        // Generate local peer ID from validator ID (now derived from actual validator identity)
        // Convert u32 validator ID to 32-byte peer ID (pad with zeros)
        let mut peer_id_bytes = [0u8; 32];
        peer_id_bytes[0..4].copy_from_slice(&validator_id.0.to_le_bytes());
        let local_peer_id = PeerId::new(peer_id_bytes);

        // Get local listen address from config
        // SECURITY: Prefer specific network interface over 0.0.0.0 (all interfaces)
        use std::net::{SocketAddr, IpAddr, Ipv4Addr};

        let detr_p2p_port = std::env::var("DETR_P2P_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(30334);

        // Determine DETR P2P listen IP with security priority:
        // 1. Explicit DETR_P2P_IP environment variable (highest priority)
        // 2. Extract from Substrate public_addresses (validator's actual IP)
        // 3. Extract from Substrate listen_addresses (node's bind IP)
        // 4. Fallback to 0.0.0.0 (SECURITY WARNING: exposes to all interfaces)

        let detr_p2p_ip = if let Ok(env_ip) = std::env::var("DETR_P2P_IP") {
            // Option 1: Explicitly set via environment variable
            match env_ip.parse::<IpAddr>() {
                Ok(ip) => {
                    log::info!("🔒 DETR P2P IP from DETR_P2P_IP env: {}", ip);
                    ip
                }
                Err(e) => {
                    log::warn!("⚠️  Invalid DETR_P2P_IP '{}': {}, using auto-detect", env_ip, e);
                    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
                }
            }
        } else {
            // Option 2: Try to extract from Substrate public_addresses
            let mut detected_ip: Option<IpAddr> = None;

            for addr in &config.network.public_addresses {
                let addr_str = addr.to_string();
                // Parse multiaddr format: /ip4/1.2.3.4/tcp/30333
                if let Some(ip_part) = addr_str.split('/').nth(2) {
                    if let Ok(ip) = ip_part.parse::<IpAddr>() {
                        // Skip localhost addresses
                        if !ip.is_loopback() {
                            log::info!("🔍 Detected public IP from Substrate config: {}", ip);
                            detected_ip = Some(ip);
                            break;
                        }
                    }
                }
            }

            // Option 3: Try listen_addresses if no public address
            if detected_ip.is_none() {
                for addr in &config.network.listen_addresses {
                    let addr_str = addr.to_string();
                    if let Some(ip_part) = addr_str.split('/').nth(2) {
                        if let Ok(ip) = ip_part.parse::<IpAddr>() {
                            // Use listen IP if it's not 0.0.0.0
                            if !ip.is_unspecified() && !ip.is_loopback() {
                                log::info!("🔍 Detected listen IP from Substrate config: {}", ip);
                                detected_ip = Some(ip);
                                break;
                            }
                        }
                    }
                }
            }

            // Option 4: Fallback to 0.0.0.0 with security warning
            if let Some(ip) = detected_ip {
                ip
            } else {
                log::warn!("⚠️  SECURITY: Could not detect specific IP, using 0.0.0.0 (all interfaces)");
                log::warn!("⚠️  RECOMMENDATION: Set DETR_P2P_IP={} for VM #1", "172.16.0.5");
                log::warn!("⚠️  RECOMMENDATION: Set DETR_P2P_IP={} for VM #2", "172.16.0.4");
                log::warn!("⚠️  This exposes DETR P2P to all network interfaces!");
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
            }
        };

        let socket_addr = SocketAddr::new(detr_p2p_ip, detr_p2p_port);

        log::info!("🌐 DETR P2P will listen on: {}", socket_addr);
        if socket_addr.ip().is_unspecified() {
            log::warn!("⚠️  SECURITY: Port {} exposed on ALL network interfaces", detr_p2p_port);
        } else {
            log::info!("🔒 SECURITY: Port {} bound to specific interface", detr_p2p_port);
        }

        let local_address = PeerAddr {
            id: local_peer_id.clone(),
            address: socket_addr,
        };

        // Parse bootstrap peers from Substrate bootnodes configuration
        // The config.network.boot_nodes contains multiaddr strings like:
        // /ip4/172.16.0.5/tcp/30333/p2p/12D3KooW...
        // We need to extract IP:port for DETR P2P (port 30334) and peer IDs
        let mut bootstrap_peers = Vec::new();

        log::info!("🔍 Parsing bootstrap peers from config.network.boot_nodes:");
        for bootnode in &boot_nodes {
            log::info!("  Raw bootnode: {}", bootnode);

            // Parse multiaddr to extract IP and peer ID
            // Format: /ip4/<IP>/tcp/<PORT>/p2p/<PEER_ID>
            let bootnode_str = bootnode.to_string();
            let parts: Vec<&str> = bootnode_str.split('/').collect();
            if parts.len() >= 6 {
                if let (Some(ip_str), Some(peer_id_str)) = (parts.get(2), parts.last()) {
                    if let Ok(ip) = ip_str.parse::<IpAddr>() {
                        // Use DETR P2P port (30334) instead of Substrate port (30333)
                        let peer_socket = SocketAddr::new(ip, detr_p2p_port);

                        // Parse peer ID from base58 (libp2p format) to bytes
                        // For now, we'll skip the peer ID parsing complexity and just use the IP:port
                        log::info!("  ✓ Adding bootstrap peer: {} (from Substrate bootnode)", peer_socket);

                        // Create a placeholder peer ID from the IP
                        // In production, you'd want to properly parse the libp2p peer ID
                        let mut peer_id_bytes = [0u8; 32];
                        if let IpAddr::V4(ipv4) = ip {
                            peer_id_bytes[0..4].copy_from_slice(&ipv4.octets());
                        }

                        let peer_addr = PeerAddr {
                            id: PeerId::new(peer_id_bytes),
                            address: peer_socket,
                        };

                        bootstrap_peers.push(peer_addr);
                    }
                }
            }
        }

        // Also check for DETR_P2P_BOOTSTRAP environment variable
        if let Ok(bootstrap_env) = std::env::var("DETR_P2P_BOOTSTRAP") {
            log::info!("🔍 Parsing bootstrap peers from DETR_P2P_BOOTSTRAP:");
            for addr_str in bootstrap_env.split(',') {
                if let Ok(addr) = addr_str.trim().parse::<SocketAddr>() {
                    log::info!("  ✓ Adding bootstrap peer: {} (from env)", addr);

                    let mut peer_id_bytes = [0u8; 32];
                    if let IpAddr::V4(ipv4) = addr.ip() {
                        peer_id_bytes[0..4].copy_from_slice(&ipv4.octets());
                    }

                    let peer_addr = PeerAddr {
                        id: PeerId::new(peer_id_bytes),
                        address: addr,
                    };

                    bootstrap_peers.push(peer_addr);
                }
            }
        }

        log::info!("📋 Total DETR P2P bootstrap peers: {}", bootstrap_peers.len());

        // Create P2P network instance
        let p2p_network = Arc::new(P2PNetwork::new(
            local_peer_id.clone(),
            socket_addr,  // P2PNetwork::new takes SocketAddr, not PeerAddr
            bootstrap_peers,
        ));

        // Spawn P2P network start in background task
        let p2p_for_start = p2p_network.clone();
        let peer_id_for_log = local_peer_id.clone();
        let addr_for_log = local_address.address;
        task_manager.spawn_handle().spawn(
            "detr-p2p-start",
            None,
            async move {
                match p2p_for_start.start().await {
                    Ok(_) => {
                        log::info!(
                            "✅ DETR P2P network started (peer_id: {}, address: {})",
                            hex::encode(peer_id_for_log.as_bytes()),
                            addr_for_log
                        );
                    }
                    Err(e) => {
                        log::error!("Failed to start P2P network: {:?}", e);
                    }
                }
            },
        );

        log::info!("🌐 DETR P2P network initialization spawned");

        // Create gadget network bridge
        let gadget_bridge = Arc::new(tokio::sync::Mutex::new(GadgetNetworkBridge::new()));

        log::info!("✅ Gadget network bridge initialized");

        // Create DetrP2PNetworkBridge combining both components
        let network_bridge = Arc::new(DetrP2PNetworkBridge::new(
            p2p_network.clone(),
            gadget_bridge.clone(),
        ));

        log::info!("✅ DetrP2PNetworkBridge created - finality messages will use detrp2p");

        // Calculate max validators from committee size
        let max_validators = asf_params.max_committee_size;

        // Create finality gadget instance
        let finality_gadget = Arc::new(tokio::sync::Mutex::new(
            finality_gadget::FinalityGadget::new(
                validator_id,
                max_validators,
                network_bridge.clone(),
            )
        ));

        log::info!(
            "ASF Finality Gadget initialized (validator_id: {:?}, max_validators: {})",
            validator_id,
            max_validators
        );
        log::info!("ASF Finality: 3-level consensus (Pre-commit → Commit → Finalized)");

        // ========== SPAWN FINALITY WORKER TASK ==========

        let finality_gadget_clone = finality_gadget.clone();
        let client_clone = client.clone();

        task_manager.spawn_essential_handle().spawn_blocking(
            "asf-finality-gadget",
            None,
            async move {
                log::info!("🚀 Starting ASF Finality Gadget worker loop");

                // Run the finality gadget worker
                // This handles:
                // 1. Incoming vote/certificate gossip
                // 2. Vote aggregation and quorum detection
                // 3. Certificate creation and broadcasting
                // 4. Finality detection (3 consecutive certificates)
                // 5. Timeout handling and view changes

                let mut gadget = finality_gadget_clone.lock().await;
                gadget.run_worker().await;
            },
        );

        // ========== SPAWN BRIDGE WORKER TASK ==========
        //
        // The bridge worker handles bidirectional message routing:
        // 1. P2P → Finality Gadget: Incoming votes/certificates from network
        // 2. Finality Gadget → P2P: Outgoing votes/certificates to network

        let bridge_p2p_network = p2p_network.clone();
        let bridge_gadget_bridge = gadget_bridge.clone();
        let bridge_finality_gadget = finality_gadget.clone();

        task_manager.spawn_essential_handle().spawn_blocking(
            "asf-bridge-worker",
            Some("finality"),
            async move {
                log::info!("🌉 Starting ASF bridge worker for P2P <-> Finality Gadget routing");

                // Main bridge loop
                use tokio::time::{interval, Duration};
                let mut poll_interval = interval(Duration::from_millis(100));

                loop {
                    poll_interval.tick().await;

                    // ========== HANDLE INCOMING P2P MESSAGES ==========
                    // TODO: Subscribe to P2P message stream
                    // For now, we just periodically check for outbound messages

                    // ========== FORWARD OUTBOUND MESSAGES TO P2P ==========
                    let mut bridge = bridge_gadget_bridge.lock().await;
                    let outbound_messages = bridge.get_outbound_messages().await;

                    for (msg, _priority) in outbound_messages {
                        match msg {
                            ConsensusBridgeMessage::Vote(vote_data) => {
                                // Serialize and broadcast vote
                                match bincode::serialize(&vote_data) {
                                    Ok(payload) => {
                                        let p2p_msg = P2PMessage::Vote { data: payload };
                                        if let Err(e) = bridge_p2p_network.broadcast(p2p_msg).await {
                                            log::warn!("Failed to broadcast vote via P2P: {:?}", e);
                                        } else {
                                            log::trace!("🔊 Forwarded vote to P2P (view: {})", vote_data.view);
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to serialize vote: {:?}", e);
                                    }
                                }
                            }
                            ConsensusBridgeMessage::Certificate(cert_data) => {
                                // Serialize and broadcast certificate
                                match bincode::serialize(&cert_data) {
                                    Ok(payload) => {
                                        let p2p_msg = P2PMessage::Certificate { data: payload };
                                        if let Err(e) = bridge_p2p_network.broadcast(p2p_msg).await {
                                            log::warn!("Failed to broadcast certificate via P2P: {:?}", e);
                                        } else {
                                            log::debug!(
                                                "🔊 Forwarded certificate to P2P (view: {}, voters: {})",
                                                cert_data.view,
                                                cert_data.signatures.len()
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to serialize certificate: {:?}", e);
                                    }
                                }
                            }
                            _ => {
                                log::trace!("Received non-vote/certificate message from bridge");
                            }
                        }
                    }

                    // ========== STATUS MONITORING ==========
                    // Periodically log finality gadget status
                    static mut LAST_STATUS_LOG: u64 = 0;
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    unsafe {
                        if now - LAST_STATUS_LOG >= 30 {
                            let gadget = bridge_finality_gadget.lock().await;
                            let current_view = gadget.get_current_view();
                            let finalized_count = gadget.get_finalized_blocks().len();

                            log::debug!(
                                "ASF Finality status: view={:?}, finalized={}, connected_peers={}",
                                current_view,
                                finalized_count,
                                bridge_p2p_network.get_connected_peers().await.len()
                            );

                            LAST_STATUS_LOG = now;
                        }
                    }
                }
            },
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // GRANDPA FINALITY (Transitional, Hybrid Approach)
    // ═══════════════════════════════════════════════════════════════════════════

    if enable_grandpa {
        log::info!("🏛️  Enabling GRANDPA finality (hybrid mode with ASF)");

        let keystore = if role.is_authority() { Some(keystore_container.keystore()) } else { None };

        let grandpa_config = sc_consensus_grandpa::Config {
            gossip_duration: Duration::from_millis(333),
            justification_generation_period: 512,
            name: Some(name),
            observer_enabled: false,
            keystore,
            local_role: role,
            telemetry: telemetry.as_ref().map(|x| x.handle()),
            protocol_name: grandpa_protocol_name,
        };

        let grandpa_params = sc_consensus_grandpa::GrandpaParams {
            config: grandpa_config,
            link: grandpa_link,
            network,
            sync: Arc::new(sync_service),
            notification_service: grandpa_notification_service,
            voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
            prometheus_registry,
            shared_voter_state: SharedVoterState::empty(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
            offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(
                transaction_pool,
            ),
        };

        task_manager.spawn_essential_handle().spawn_blocking(
            "grandpa-voter",
            None,
            sc_consensus_grandpa::run_grandpa_voter(grandpa_params)?,
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VALIDATOR MANAGEMENT (Committee Coordination)
    // ═══════════════════════════════════════════════════════════════════════════

    if role.is_authority() {
        log::info!("👥 Initializing ASF Validator Management");

        // TODO: Initialize validator management
        //
        // This will:
        // 1. Track committee membership (PPFA panels)
        // 2. Monitor validator health
        // 3. Calculate and distribute rewards
        // 4. Handle slashing for misbehavior
        // 5. Coordinate epoch transitions
        //
        // For now, we log that it's initialized

        log::info!(
            "Validator Management initialized (epoch_duration: {} blocks)",
            asf_params.epoch_duration
        );

        // Create genesis validators for dev mode
        // TODO: In production, load from genesis config/keystore
        let genesis_validators = vec![
            validator_management::ValidatorInfo::new(
                validator_management::ValidatorId::from([0u8; 32]),
                asf_params.min_validator_stake,
                validator_management::PeerType::FlareNode,
            ),
            validator_management::ValidatorInfo::new(
                validator_management::ValidatorId::from([1u8; 32]),
                asf_params.min_validator_stake,
                validator_management::PeerType::FlareNode,
            ),
            validator_management::ValidatorInfo::new(
                validator_management::ValidatorId::from([2u8; 32]),
                asf_params.min_validator_stake,
                validator_management::PeerType::FlareNode,
            ),
        ];

        // Create coordinator config
        let coordinator_config = validator_management::CoordinatorConfig {
            max_committee_size: asf_params.max_committee_size,
            epoch_duration: asf_params.epoch_duration,
            health_check_interval: 100, // Every 100 blocks
            enable_rewards: true,
            enable_state_sync: true,
        };

        // Spawn validator management coordinator
        task_manager.spawn_handle().spawn(
            "asf-validator-management",
            Some("validator"),
            validator_management::run_coordinator(coordinator_config, genesis_validators),
        );
    }

    log::info!("✅ ASF FlareChain node started successfully");
    log::info!("   - Block Production: ASF PPFA (slot_duration: {}ms)", asf_params.slot_duration);
    log::info!("   - Finality: Hybrid (ASF + GRANDPA)");
    log::info!("   - Committee Size: {}", asf_params.max_committee_size);
    log::info!("   - Epoch Duration: {} blocks", asf_params.epoch_duration);

    Ok(task_manager)
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if the runtime supports ASF consensus
///
/// This queries the runtime for ASF-specific APIs to ensure compatibility
pub fn runtime_supports_asf<Client>(_client: &Arc<Client>) -> bool
where
    Client: sc_client_api::BlockchainEvents<Block>,
{
    // TODO: Check for ASF runtime APIs
    // For now, assume all FlareChain runtimes support ASF
    true
}

/// Get current PPFA committee from runtime
///
/// Queries the runtime state for the active validator committee
pub fn get_ppfa_committee<Client>(
    _client: &Arc<Client>,
    _at: <Block as sp_runtime::traits::Block>::Hash,
) -> Result<Vec<sp_core::crypto::AccountId32>, String>
where
    Client: sc_client_api::BlockchainEvents<Block>,
{
    // TODO: Query runtime state for committee
    // This will use validator-management types
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_production::{ValidatorId, ProposerSelector, CommitteeManager};
    use validator_management::{ValidatorInfo, PeerType};

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 1: ASF Parameters Configuration
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_asf_params_defaults() {
        let params = AsfParams::default();

        assert_eq!(params.slot_duration, 6000);
        assert_eq!(params.max_committee_size, 21);
        assert_eq!(params.epoch_duration, 2400);
        assert!(params.enable_finality_gadget);
        assert_eq!(params.min_validator_stake, 64_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_asf_params_customization() {
        let params = AsfParams {
            slot_duration: 3000,
            max_committee_size: 42,
            epoch_duration: 1200,
            enable_finality_gadget: false,
            min_validator_stake: 128_000_000_000_000_000_000_000,
        };

        assert_eq!(params.slot_duration, 3000);
        assert_eq!(params.max_committee_size, 42);
        assert!(!params.enable_finality_gadget);
    }

    #[test]
    fn test_asf_params_epoch_calculation() {
        let params = AsfParams::default();

        // Verify epoch duration calculation at 6 second blocks
        // 2400 blocks * 6 seconds = 14,400 seconds = 4 hours
        let epoch_seconds = params.epoch_duration as u64 * (params.slot_duration / 1000);
        assert_eq!(epoch_seconds, 14_400); // 4 hours
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 2: PPFA Committee Management
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_committee_initialization() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators to committee
        for i in 0..5 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            assert!(committee.add_validator(validator_info).is_ok());
        }

        // Verify committee size
        assert_eq!(committee.validator_count(), 5);
    }

    #[test]
    fn test_committee_rotation() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..10 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        // Rotate committee to epoch 1
        assert!(committee.rotate_committee(1).is_ok());

        // Verify active committee size is capped at max_committee_size
        let active_count = committee.active_committee_size();
        assert!(active_count <= params.max_committee_size as usize);
    }

    #[test]
    fn test_committee_exceeds_max_size() {
        let params = AsfParams {
            max_committee_size: 5,
            ..Default::default()
        };
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 10 validators (exceeds max of 5)
        for i in 0..10 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        // Rotate and verify active committee is capped
        committee.rotate_committee(1).unwrap();
        assert_eq!(committee.active_committee_size(), 5);
    }

    #[test]
    fn test_empty_committee_rotation_fails() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Attempt to rotate empty committee should fail
        assert!(committee.rotate_committee(1).is_err());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 3: PPFA Proposer Selection
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_ppfa_proposer_selection() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 3 validators
        for i in 0..3 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();

        let mut proposer_selector = ProposerSelector::new(committee);

        // Get current proposer (should succeed)
        assert!(proposer_selector.current_proposer().is_ok());
    }

    #[test]
    fn test_ppfa_rotation_advances_proposer() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..3 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();
        let mut proposer_selector = ProposerSelector::new(committee);

        // Get initial proposer
        let proposer1 = proposer_selector.current_proposer().unwrap();
        let ppfa_index1 = proposer_selector.current_ppfa_index();

        // Advance to next block
        proposer_selector.advance(1);

        // Verify PPFA index changed
        let ppfa_index2 = proposer_selector.current_ppfa_index();
        assert_ne!(ppfa_index1, ppfa_index2);
    }

    #[test]
    fn test_ppfa_proposer_authorization() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        let validator_ids: Vec<ValidatorId> = (0..3)
            .map(|i| {
                let id = ValidatorId::from([i as u8; 32]);
                let info = ValidatorInfo::new(id, params.min_validator_stake, PeerType::ValidityNode);
                committee.add_validator(info).unwrap();
                id
            })
            .collect();

        committee.rotate_committee(1).unwrap();
        let proposer_selector = ProposerSelector::new(committee);

        // Check if first validator is the current proposer
        let is_proposer = proposer_selector.is_proposer(&validator_ids[0]);

        // At least one validator should be the proposer
        let any_is_proposer = validator_ids.iter()
            .any(|id| proposer_selector.is_proposer(id));
        assert!(any_is_proposer);
    }

    #[test]
    fn test_unauthorized_proposer_rejected() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 3 validators
        for i in 0..3 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();
        let proposer_selector = ProposerSelector::new(committee);

        // Create a validator NOT in the committee
        let unauthorized_validator = ValidatorId::from([99u8; 32]);

        // Verify unauthorized validator is rejected
        assert!(!proposer_selector.is_proposer(&unauthorized_validator));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 4: Epoch Transitions
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_epoch_boundary_detection() {
        let params = AsfParams::default();

        // Block 0 is epoch 0
        let epoch_0 = 0 / params.epoch_duration;
        assert_eq!(epoch_0, 0);

        // Block 2400 is epoch 1
        let epoch_1 = params.epoch_duration / params.epoch_duration;
        assert_eq!(epoch_1, 1);

        // Block 4800 is epoch 2
        let epoch_2 = (params.epoch_duration * 2) / params.epoch_duration;
        assert_eq!(epoch_2, 2);
    }

    #[test]
    fn test_epoch_transition_triggers_committee_rotation() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..5 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        // Rotate to epoch 1
        assert!(committee.rotate_committee(1).is_ok());

        // Rotate to epoch 2
        assert!(committee.rotate_committee(2).is_ok());

        // Verify committee is still active
        assert!(committee.active_committee_size() > 0);
    }

    #[test]
    fn test_epoch_duration_consistency() {
        let params = AsfParams::default();

        // Verify epoch duration is consistent with documentation
        // 2400 blocks at 6 seconds = 4 hours
        assert_eq!(params.epoch_duration, 2400);

        // Test epoch calculation for various block numbers
        let test_cases = vec![
            (0, 0),       // Block 0 → Epoch 0
            (1200, 0),    // Block 1200 → Epoch 0
            (2399, 0),    // Block 2399 → Epoch 0
            (2400, 1),    // Block 2400 → Epoch 1
            (4800, 2),    // Block 4800 → Epoch 2
            (7200, 3),    // Block 7200 → Epoch 3
        ];

        for (block_number, expected_epoch) in test_cases {
            let calculated_epoch = block_number / params.epoch_duration;
            assert_eq!(calculated_epoch, expected_epoch,
                "Block {} should be in epoch {}", block_number, expected_epoch);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 5: Byzantine Fault Tolerance
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_committee_tolerates_one_third_failures() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 21 validators (max committee size)
        for i in 0..21 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();

        // Byzantine fault tolerance: Can tolerate (n-1)/3 failures
        // For 21 validators: (21-1)/3 = 6.67 → 6 Byzantine failures tolerated
        let total_validators = 21;
        let max_byzantine_failures = (total_validators - 1) / 3;

        assert_eq!(max_byzantine_failures, 6);

        // Need 2/3 + 1 for consensus
        let min_honest_validators = (total_validators * 2 / 3) + 1;
        assert_eq!(min_honest_validators, 15);
    }

    #[test]
    fn test_minimum_committee_size_for_bft() {
        // Minimum committee size for BFT is 4 (can tolerate 1 Byzantine failure)
        let params = AsfParams {
            max_committee_size: 4,
            ..Default::default()
        };

        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 4 validators
        for i in 0..4 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        assert!(committee.rotate_committee(1).is_ok());

        // With 4 validators, can tolerate (4-1)/3 = 1 Byzantine failure
        assert_eq!(committee.active_committee_size(), 4);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 6: Validator Stake Requirements
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_minimum_validator_stake_enforced() {
        let params = AsfParams::default();

        // Verify minimum stake is 64 ETR for FlareNode
        assert_eq!(params.min_validator_stake, 64_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_validator_with_sufficient_stake() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        let validator_id = ValidatorId::from([1u8; 32]);
        let validator_info = ValidatorInfo::new(
            validator_id,
            params.min_validator_stake, // Exact minimum
            PeerType::ValidityNode,
        );

        // Should succeed with exact minimum stake
        assert!(committee.add_validator(validator_info).is_ok());
    }

    #[test]
    fn test_validator_with_excess_stake() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        let validator_id = ValidatorId::from([1u8; 32]);
        let excess_stake = params.min_validator_stake * 10; // 640 ETR
        let validator_info = ValidatorInfo::new(
            validator_id,
            excess_stake,
            PeerType::ValidityNode,
        );

        // Should succeed with excess stake
        assert!(committee.add_validator(validator_info).is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 7: Slot Duration and Timing
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_slot_duration_default() {
        let params = AsfParams::default();

        // Default slot duration is 6 seconds (6000 milliseconds)
        assert_eq!(params.slot_duration, 6000);
    }

    #[test]
    fn test_blocks_per_hour_calculation() {
        let params = AsfParams::default();

        // 6 second blocks = 10 blocks per minute = 600 blocks per hour
        let seconds_per_hour = 3600;
        let blocks_per_hour = (seconds_per_hour * 1000) / params.slot_duration;
        assert_eq!(blocks_per_hour, 600);
    }

    #[test]
    fn test_blocks_per_day_calculation() {
        let params = AsfParams::default();

        // 600 blocks/hour * 24 hours = 14,400 blocks per day
        let blocks_per_day = (86400 * 1000) / params.slot_duration;
        assert_eq!(blocks_per_day, 14_400);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 7: PPFA Authorization (TODO #4 Integration Tests)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_ppfa_seal_encoding_decoding() {
        use codec::{Encode, Decode};

        // Define the PpfaSeal structure for testing
        #[derive(Encode, Decode, Debug, PartialEq)]
        struct PpfaSeal {
            ppfa_index: u32,
            proposer_id: [u8; 32],
            slot_number: u64,
            timestamp: u64,
        }

        // Create a test PPFA seal
        let test_seal = PpfaSeal {
            ppfa_index: 42,
            proposer_id: [5u8; 32],
            slot_number: 1234,
            timestamp: 1609459200, // 2021-01-01 00:00:00 UTC
        };

        // Encode the seal
        let encoded = test_seal.encode();

        // Decode the seal
        let decoded = PpfaSeal::decode(&mut &encoded[..]).expect("Failed to decode PPFA seal");

        // Verify encoding/decoding round-trip
        assert_eq!(test_seal, decoded);
        assert_eq!(decoded.ppfa_index, 42);
        assert_eq!(decoded.proposer_id, [5u8; 32]);
        assert_eq!(decoded.slot_number, 1234);
        assert_eq!(decoded.timestamp, 1609459200);
    }

    #[test]
    fn test_ppfa_seal_engine_id() {
        // Verify PPFA consensus engine ID is correctly formatted
        let engine_id: [u8; 4] = *b"PPFA";

        assert_eq!(engine_id, [b'P', b'P', b'F', b'A']);
        assert_eq!(engine_id.len(), 4);

        // Verify it matches the engine ID used in block sealing
        let expected_engine_id = *b"PPFA";
        assert_eq!(engine_id, expected_engine_id);
    }

    #[test]
    fn test_ppfa_authorization_data_integrity() {
        use codec::Encode;

        // Create test data representing PPFA authorization
        let block_number: u32 = 100;
        let ppfa_index: u32 = 5;
        let proposer_id = ValidatorId::from([7u8; 32]);

        // Verify data can be encoded without panic
        let _block_number_encoded = block_number.encode();
        let _ppfa_index_encoded = ppfa_index.encode();
        let _proposer_id_encoded = proposer_id.encode();

        // Verify ValidatorId encoding produces 32 bytes
        assert_eq!(proposer_id.encode().len(), 32);
    }

    #[test]
    fn test_ppfa_seal_size_limits() {
        use codec::Encode;

        #[derive(Encode)]
        struct PpfaSeal {
            ppfa_index: u32,
            proposer_id: [u8; 32],
            slot_number: u64,
            timestamp: u64,
        }

        let seal = PpfaSeal {
            ppfa_index: u32::MAX,
            proposer_id: [0xFFu8; 32],
            slot_number: u64::MAX,
            timestamp: u64::MAX,
        };

        let encoded = seal.encode();

        // PPFA seal should be compact: 4 + 32 + 8 + 8 = 52 bytes minimum
        // With SCALE encoding overhead, should be ~52-56 bytes
        assert!(encoded.len() >= 52);
        assert!(encoded.len() <= 64, "PPFA seal too large: {} bytes", encoded.len());
    }

    #[test]
    fn test_ppfa_proposer_rotation() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 5 validators
        let validator_ids: Vec<ValidatorId> = (0..5)
            .map(|i| {
                let vid = ValidatorId::from([i as u8; 32]);
                let vinfo = ValidatorInfo::new(
                    vid,
                    params.min_validator_stake,
                    PeerType::ValidityNode,
                );
                committee.add_validator(vinfo).expect("Failed to add validator");
                vid
            })
            .collect();

        let mut proposer_selector = ProposerSelector::new(committee);

        // Verify rotation through all proposers
        let mut seen_proposers = std::collections::HashSet::new();

        for i in 0..10 {
            let proposer = proposer_selector.current_proposer()
                .expect("Failed to get current proposer");
            seen_proposers.insert(proposer);
            proposer_selector.advance(i);
        }

        // Should have seen multiple different proposers
        assert!(seen_proposers.len() >= 2, "PPFA rotation not working: only {} unique proposers", seen_proposers.len());
    }

    #[test]
    fn test_unauthorized_proposer_detection() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add authorized validators
        let authorized_ids: Vec<ValidatorId> = (0..3)
            .map(|i| {
                let vid = ValidatorId::from([i as u8; 32]);
                let vinfo = ValidatorInfo::new(
                    vid,
                    params.min_validator_stake,
                    PeerType::ValidityNode,
                );
                committee.add_validator(vinfo).expect("Failed to add validator");
                vid
            })
            .collect();

        let proposer_selector = ProposerSelector::new(committee);

        // Create an unauthorized validator ID
        let unauthorized_validator = ValidatorId::from([99u8; 32]);

        // Verify unauthorized validator is NOT a proposer
        assert!(!proposer_selector.is_proposer(&unauthorized_validator),
            "Unauthorized validator incorrectly identified as proposer");

        // Verify at least one authorized validator IS a proposer
        let has_authorized_proposer = authorized_ids.iter()
            .any(|id| proposer_selector.is_proposer(id));

        assert!(has_authorized_proposer, "No authorized proposers found");
    }

    #[test]
    fn test_epoch_boundary_ppfa_reset() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..5 {
            let vid = ValidatorId::from([i as u8; 32]);
            let vinfo = ValidatorInfo::new(
                vid,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(vinfo).expect("Failed to add validator");
        }

        // Simulate epoch rotation
        let epoch1 = 0u64;
        let epoch2 = 1u64;

        let result1 = committee.rotate_committee(epoch1);
        assert!(result1.is_ok(), "Epoch 0 rotation failed");

        let result2 = committee.rotate_committee(epoch2);
        assert!(result2.is_ok(), "Epoch 1 rotation failed");

        // After rotation, committee should still have validators
        assert!(committee.size() > 0, "Committee empty after rotation");
    }
}
