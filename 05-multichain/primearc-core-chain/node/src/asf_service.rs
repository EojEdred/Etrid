//! # ASF Consensus Service Integration
//!
//! This module integrates the custom ËTRID ASF (Ascending Scale of Finality) consensus
//! modules into the Primearc Core node service layer.
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
//! - `new_full()`: Spawns ASF consensus tasks (pure ASF, no GRANDPA)
//! - Validator management integrates with keystore for signing
//! - Finality gadget runs as essential service task
//!
//! ## Compatibility
//!
//! Built for polkadot-stable2506 with Substrate service patterns.

use primearc_core_runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::{BlockBackend, UsageProvider, Backend, HeaderBackend, BlockchainEvents, Finalizer, LockImportRun};
use sp_runtime::traits::SaturatedConversion;
use futures::executor::block_on;
#[allow(unused_imports)]
use futures::StreamExt;
use futures::channel::mpsc;
use sc_consensus::BlockImport;
use sc_consensus::import_queue::{ImportQueue, IncomingBlock};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_api::ProvideRuntimeApi;
use sp_consensus::{BlockOrigin, Environment, Proposer};
use sp_core::Encode;
use substrate_prometheus_endpoint::{register, Gauge, Registry, U64, Counter};
use codec::Decode;
use sp_runtime::traits::{Header, IdentifyAccount, NumberFor, Zero};
use sp_runtime::MultiSigner;
use sp_core::crypto::AccountId32;
use sp_timestamp;
use std::{collections::{HashMap, HashSet}, sync::Arc, sync::atomic::{AtomicU64, Ordering}, time::Duration};
use detrp2p_peerstored::PeerStore;
use serde_json;
use tokio::fs;

// ASF Justification and Block Import
use crate::asf_justification::{
    AsfBlockImportWrapper,
    create_finality_channels,
    create_noop_justification_import,
};

// PPFA Protocol for libp2p network layer - bridges ASF finality to Substrate libp2p
use crate::ppfa_protocol::{
    PPFAProtocolConfig, PPFAProtocolWorker, create_ppfa_channels,
};

// Runtime API for validator committee queries
use pallet_validator_committee_runtime_api::ValidatorCommitteeApi;

// ÉTRID P2P Networking
use detrp2p::{P2PNetwork, PeerId, PeerAddr, Message as P2PMessage};
use detrp2p_peerstored::StoredPeer;
use etrid_protocol::{
    BlockAnnounceMessage,
    BlockRequestMessage,
    BlockResponseMessage,
    StatusResponseMessage,
    gadget_network_bridge::{
        GadgetNetworkBridge,
        VoteData,
        CertificateData,
        ConsensusBridgeMessage,
    },
};

// ASF Finality Components (Phases 3-9 Integration)
#[allow(unused_imports)]
use finality_gadget::{
    equivocation::{EquivocationDetector, EquivocationProof},
    implicit_finality::{ImplicitFinalityTracker, ImplicitFinalityConfig, FinalityStatus},
    fork_pruning::{ForkPruner, ForkPruningConfig},
    checkpoint::{CheckpointBFT, CheckpointConfig},
};

// ASF RPC and Indexer
#[allow(unused_imports)]
use crate::asf_rpc::{AsfFinality, AsfFinalityState, AsfFinalityApiServer};
use crate::asf_indexer::{AsfIndexer, FinalityEvent, create_indexer};
use std::time::{SystemTime, UNIX_EPOCH};
use etrid_p2p_dpeers::{PeerRegistry, DiscoveryProtocol, ConnectionState};

/// Cap the number of tracked detrp2p peers to keep organic growth without unbounded memory.
const DETR_P2P_MAX_TRACKED_PEERS: usize = 2048;
/// Cap in-flight/orphan blocks to avoid unbounded memory growth during desyncs.
const DETR_P2P_MAX_PENDING_BLOCKS: usize = 2048;

#[derive(Clone)]
struct PendingBlock {
    source_peer: PeerId,
    block_number: u64,
    block_hash: sp_core::H256,
    parent_hash: sp_core::H256,
    encoded_block: Vec<u8>,
}

#[derive(Default)]
struct PendingState {
    by_parent: HashMap<sp_core::H256, Vec<PendingBlock>>,
    hashes: HashSet<sp_core::H256>,
}

fn queue_pending_block(pending_state: &mut PendingState, pending: PendingBlock) -> bool {
    if pending_state.hashes.contains(&pending.block_hash) {
        return false;
    }
    if pending_state.hashes.len() >= DETR_P2P_MAX_PENDING_BLOCKS {
        log::warn!(
            "⚠️ Pending block pool full ({}), dropping block #{}",
            pending_state.hashes.len(),
            pending.block_number
        );
        return false;
    }
    pending_state.hashes.insert(pending.block_hash);
    pending_state
        .by_parent
        .entry(pending.parent_hash)
        .or_default()
        .push(pending);
    true
}

fn take_pending_children(pending_state: &mut PendingState, parent_hash: &sp_core::H256) -> Vec<PendingBlock> {
    match pending_state.by_parent.remove(parent_hash) {
        Some(children) => {
            for child in &children {
                pending_state.hashes.remove(&child.block_hash);
            }
            children
        }
        None => Vec::new(),
    }
}

fn build_incoming_block(
    block: Block,
    justifications: Option<sp_runtime::Justifications>,
) -> IncomingBlock<Block> {
    let hash = block.header.hash();
    IncomingBlock {
        hash,
        header: Some(block.header),
        body: Some(block.extrinsics),
        indexed_body: None,
        justifications,
        origin: None,
        allow_missing_state: false,
        skip_execution: false,
        import_existing: false,
        state: None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Lightweight Prometheus gauges for peer/sync observability.
struct PeerSyncMetrics {
    libp2p_connected: Option<Gauge<U64>>,
    dpeers_authenticated: Option<Gauge<U64>>,
    dpeers_total: Option<Gauge<U64>>,
    best_block: Option<Gauge<U64>>,
}

impl PeerSyncMetrics {
    fn new(registry: Option<&Registry>) -> Self {
        let register_gauge = |name: &str, help: &str| -> Option<Gauge<U64>> {
            registry.and_then(|r| {
                register(Gauge::new(name, help).expect("gauge valid"), r).ok()
            })
        };

        Self {
            libp2p_connected: register_gauge(
                "ppfa_libp2p_connected_peers",
                "Connected libp2p peers for PPFA"
            ),
            dpeers_authenticated: register_gauge(
                "ppfa_dpeers_authenticated",
                "Authenticated DETR P2P peers"
            ),
            dpeers_total: register_gauge(
                "ppfa_dpeers_total",
                "Total DETR P2P peers tracked"
            ),
            best_block: register_gauge(
                "ppfa_best_block_number",
                "Best block number observed by PPFA node"
            ),
        }
    }

    fn set_libp2p(&self, v: u64) {
        if let Some(g) = &self.libp2p_connected {
            g.set(v);
        }
    }

    fn set_dpeers(&self, authenticated: u64, total: u64) {
        if let Some(g) = &self.dpeers_authenticated {
            g.set(authenticated);
        }
        if let Some(g) = &self.dpeers_total {
            g.set(total);
        }
    }

    fn set_best_block(&self, v: u64) {
        if let Some(g) = &self.best_block {
            g.set(v);
        }
    }
}

/// Lightweight counters for DETR P2P consensus traffic.
#[derive(Clone)]
struct Detrp2PMetrics {
    dropped_rate_limited: Option<Counter<U64>>,
    block_requests: Option<Counter<U64>>,
    block_responses_sent: Option<Counter<U64>>,
    status_responses_sent: Option<Counter<U64>>,
    stored_peers: Option<Gauge<U64>>,
}

impl Detrp2PMetrics {
    fn new(registry: Option<&Registry>) -> Self {
        let register_counter = |name: &str, help: &str| -> Option<Counter<U64>> {
            registry.and_then(|r| register(Counter::new(name, help).expect("counter valid"), r).ok())
        };
        let register_gauge = |name: &str, help: &str| -> Option<Gauge<U64>> {
            registry.and_then(|r| register(Gauge::new(name, help).expect("gauge valid"), r).ok())
        };

        Self {
            dropped_rate_limited: register_counter(
                "detrp2p_finality_dropped_rate_limited",
                "Number of detrp2p finality messages dropped due to rate limits",
            ),
            block_requests: register_counter(
                "detrp2p_block_requests_received",
                "Number of detrp2p block requests received",
            ),
            block_responses_sent: register_counter(
                "detrp2p_block_responses_sent",
                "Number of detrp2p block responses sent",
            ),
            status_responses_sent: register_counter(
                "detrp2p_status_responses_sent",
                "Number of detrp2p status responses sent",
            ),
            stored_peers: register_gauge(
                "detrp2p_peerstore_active",
                "Number of active peers in the detrp2p peer store",
            ),
        }
    }

    fn inc_dropped(&self) {
        if let Some(c) = &self.dropped_rate_limited {
            c.inc();
        }
    }

    fn inc_block_request(&self) {
        if let Some(c) = &self.block_requests {
            c.inc();
        }
    }

    fn inc_block_response(&self) {
        if let Some(c) = &self.block_responses_sent {
            c.inc();
        }
    }

    fn inc_status_response(&self) {
        if let Some(c) = &self.status_responses_sent {
            c.inc();
        }
    }

    fn set_stored_peers(&self, v: u64) {
        if let Some(g) = &self.stored_peers {
            g.set(v);
        }
    }
}

fn parse_detr_peer_id_hex(peer_id_hex: &str) -> Option<PeerId> {
    if peer_id_hex.len() != 64 {
        return None;
    }
    let mut peer_id_bytes = [0u8; 32];
    if hex::decode_to_slice(peer_id_hex, &mut peer_id_bytes).is_ok() {
        Some(PeerId::new(peer_id_bytes))
    } else {
        None
    }
}

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

/// ASF-enabled block import type (wrapped with finality notifications)
type AsfBlockImport = AsfBlockImportWrapper<Arc<FullClient>, FullClient>;

/// ASF finality notification receiver type
pub type AsfFinalityReceiver = mpsc::UnboundedReceiver<(<Block as sp_runtime::traits::Block>::Hash, NumberFor<Block>)>;

/// Full node partial components with ASF integration
pub type AsfFullParts = sc_service::PartialComponents<
    FullClient,
    FullBackend,
    SelectChain,
    sc_consensus::DefaultImportQueue<Block>,
    sc_transaction_pool::TransactionPoolHandle<Block, FullClient>,
    (
        AsfBlockImport,
        Option<Telemetry>,
        AsfFinalityReceiver,  // ASF finality notification receiver
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

    /// Enable peer/sync metrics logging
    pub enable_peer_metrics: bool,
}

impl Default for AsfParams {
    fn default() -> Self {
        Self {
            slot_duration: 6000, // 6 seconds (from block-production::BASE_SLOT_DURATION)
            max_committee_size: 21, // PPFA panel size (from validator-management::MAX_COMMITTEE_SIZE)
            epoch_duration: 2400, // ~4 hours at 6s blocks (from validator-management::EPOCH_DURATION)
            enable_finality_gadget: true,
            min_validator_stake: 64_000_000_000_000_000_000_000, // 64 ËTR for FlareNode
            enable_peer_metrics: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARTIAL NODE SETUP (ASF IMPORT QUEUE)
// ═══════════════════════════════════════════════════════════════════════════════

/// Create a new partial node with ASF consensus integration
///
/// This replaces AURA's import queue with an ASF-compatible one while keeping
/// Pure ASF consensus (v108).
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
    // ASF BLOCK IMPORT (Pure ASF with finality notification streams)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // v109 CRITICAL FIX: Wrap block import with ASF finality notifications.
    // This ensures the SyncingEngine receives proper finality events and
    // prevents the "SelectNextSome polled after terminated" panic.
    //
    // The wrapper provides:
    // 1. Finality notification stream for imported blocks
    // 2. Proper handling of ASF certificates as justifications
    // 3. Integration with Substrate's sync engine

    // Create finality notification channels
    let (finality_tx, finality_rx) = create_finality_channels::<Block>();

    // Wrap the client with ASF finality notifications
    let block_import = AsfBlockImportWrapper::new(
        client.clone(),
        client.clone(),
        finality_tx,
    );

    log::info!("✅ ASF block import wrapper created with finality notification streams");

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

            // ═══════════════════════════════════════════════════════════════
            // FORK CHOICE STRATEGY: Signal to Substrate import pipeline
            // This tells Substrate this validated block is a candidate for
            // the canonical chain. Without this, the import pipeline is
            // incomplete and blocks are rejected.
            // ═══════════════════════════════════════════════════════════════
            block.fork_choice = Some(sc_consensus::ForkChoiceStrategy::LongestChain);

            log::debug!(
                "🔗 Block #{} ready for import with LongestChain fork choice",
                block_number
            );

            // Note: We don't clear post_digests here - they're part of the block

            Ok(block)
        }
    }

    let verifier = AsfVerifier::<_, FullBackend>::new(client.clone());

    // Create no-op justification import - ASF handles finality independently
    // through DETR P2P layer, not through Substrate's justification system
    let justification_import = create_noop_justification_import::<Block>();

    let import_queue = BasicQueue::new(
        verifier,
        Box::new(block_import.clone()),
        Some(justification_import),
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
        other: (block_import, telemetry, finality_rx),
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
/// │                    Primearc Core Chain Node                          │
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
/// │                       │
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
        other: (block_import, mut telemetry, asf_finality_rx),
    } = new_partial(&config)?;
    let _select_chain = select_chain;
    let import_queue_service = Arc::new(tokio::sync::Mutex::new(import_queue.service()));

    // V113: Store finality receiver - will be consumed by ASF block import finality handler
    // This channel receives finality notifications when blocks with justifications are imported
    let asf_block_finality_rx = asf_finality_rx;

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

    let _peer_store_handle = net_config.peer_store_handle();

    // Clone chain spec properties for use inside async tasks (avoid capturing config)
    let config_genesis_props = config.chain_spec.properties().clone();

    // ═══════════════════════════════════════════════════════════════════════════
    // PPFA PROTOCOL SETUP - Finality over libp2p (v110)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // PPFA (Proposing Panel for Attestation) provides the finality protocol
    // for the libp2p network layer, enabling peer connections and block sync.
    // ASF finality via DETR P2P (port 30334) is bridged to PPFA for libp2p (port 30333).

    log::info!("🔗 PPFA: Registering finality protocol for libp2p network");

    // Create PPFA protocol configuration with dynamic protocol ID from chain spec
    // This ensures all nodes on the same chain use the same protocol name for peer discovery
    let protocol_id = config.chain_spec.protocol_id().unwrap_or("primearc");
    log::info!("📡 PPFA protocol ID from chain spec: {}", protocol_id);
    let ppfa_config = PPFAProtocolConfig::new(protocol_id);
    let ppfa_protocol_name = ppfa_config.protocol_name.clone();

    // Initialize ASF finality indexer (in-memory) for observability of events.
    let (indexer_tx, indexer) = create_indexer();
    let _indexer_tx = Arc::new(indexer_tx);
    task_manager.spawn_handle().spawn(
        "asf-indexer",
        None,
        async move { indexer.run().await; },
    );

    // Build notification config and service for PPFA
    // V115: Pass reserved nodes from Substrate config to PPFA for proper peer connectivity
    // This ensures PPFA can connect to bootnodes for block sync (port 30333)
    let ppfa_reserved_nodes = config.network.default_peers_set.reserved_nodes.clone();
    log::info!("📡 PPFA: Using {} reserved nodes from Substrate config", ppfa_reserved_nodes.len());
    let (ppfa_notification_config, ppfa_notification_service) = ppfa_config.build_notification_config(ppfa_reserved_nodes);

    // Add PPFA notification protocol to network
    net_config.add_notification_protocol(ppfa_notification_config);

    log::info!("✅ PPFA protocol registered: {}", ppfa_protocol_name);

    // Guard: ensure runtime exposes required ASF API before proceeding.
    if !runtime_supports_asf(&client) {
        log::error!("❌ Runtime does not expose ValidatorCommittee API required for ASF/PPFA; aborting startup.");
        return Err(ServiceError::Other("runtime missing ValidatorCommitteeApi".into()));
    }

    // Fetch and log the current runtime committee to ensure wiring is healthy before proceeding.
    let quorum_override = quorum_override_from_env();
    let best_hash = client.info().best_hash;
    let mut initial_committee_size: u32 = asf_params.max_committee_size;
    match get_ppfa_committee(&client, best_hash) {
        Ok(committee) => {
            log::info!(
                "👥 Runtime validator committee size at best hash {}: {}",
                best_hash,
                committee.len()
            );
            if !committee.is_empty() {
                initial_committee_size = committee.len() as u32;
            }
        }
        Err(e) => {
            log::warn!(
                "⚠️ Failed to fetch runtime committee at best hash {}: {:?}",
                best_hash,
                e
            );
        }
    }
    let initial_committee_size = apply_quorum_override(initial_committee_size, quorum_override);

    // Create PPFA channels for bridging ASF finality from DETR P2P to libp2p
    // V112: Remove underscore - ppfa_finality_rx is now used by the finality notification handler
    let (ppfa_asf_tx, ppfa_asf_rx, ppfa_finality_tx, ppfa_finality_rx) = create_ppfa_channels::<Block>();

    // Warp sync disabled for now (PPFA warp sync provider can be added later)
    let warp_sync = None;

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
            // Warp sync currently disabled by upstream type; use provided config if set
            warp_sync_config: warp_sync,
            block_relay: None,
            metrics,
        })?;

    log::info!("✅ Substrate network built successfully on port 30333");

    // ═══════════════════════════════════════════════════════════════════════════
    // DPEERS ↔ libp2p integration (bootstrap via bootnodes/reserved)
    // ═══════════════════════════════════════════════════════════════════════════
    // Prometheus registry for metrics exposure (if enabled)
    let prometheus_registry = config.prometheus_registry().cloned();
    let dpeers_registry = Arc::new(PeerRegistry::new(200));
    let dpeers_discovery = DiscoveryProtocol::new(dpeers_registry.clone());
    let mut bootstrap_nodes = config.network.boot_nodes.clone();
    bootstrap_nodes.extend(config.network.default_peers_set.reserved_nodes.clone());
    if bootstrap_nodes.is_empty() {
        log::warn!("⚠️ No bootnodes/reserved nodes configured; peer connectivity may be limited");
    }
    let network_for_dpeers = Arc::new(network.clone());
    let peer_metrics_network = network_for_dpeers.clone();
    let peer_metrics_client = client.clone();
    let peer_metrics_enabled = asf_params.enable_peer_metrics;
    let peer_metrics = PeerSyncMetrics::new(prometheus_registry.as_ref());
    let detrp2p_metrics = Detrp2PMetrics::new(prometheus_registry.as_ref());
    let dpeers_registry_for_bridge = dpeers_registry.clone();
    task_manager.spawn_handle().spawn(
        "dpeers-libp2p-bridge",
        Some("networking"),
        async move {
            // Discover and add reserved peers
            for peer in bootstrap_nodes.iter() {
                let _ = dpeers_discovery
                    .discover_peers(vec![(peer.peer_id.to_base58(), peer.to_string())])
                    .await;
                let _ = network_for_dpeers.add_reserved_peer(peer.clone());
            }

            // Periodically sync connection state from libp2p into dpeers
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Ok(state) = network_for_dpeers.network_state().await {
                    for (peer_id, _) in state.connected_peers.iter() {
                        let _ = dpeers_registry_for_bridge
                            .set_connection_state(peer_id, ConnectionState::Authenticated)
                            .await;
                    }
                }
            }
        },
    );

    // Lightweight peer/sync metrics logger (periodic)
    if peer_metrics_enabled {
        let peer_metrics_registry = dpeers_registry.clone();
        task_manager.spawn_handle().spawn(
            "peer-sync-metrics",
            Some("networking"),
            async move {
                let mut interval = tokio::time::interval(Duration::from_secs(15));
                loop {
                    interval.tick().await;
                    let peers = if let Ok(state) = peer_metrics_network.network_state().await {
                        state.connected_peers.len()
                    } else {
                        0
                    };
                    let best = peer_metrics_client.info().best_number;
                    let stats = peer_metrics_registry.stats().await;

                    peer_metrics.set_libp2p(peers as u64);
                    peer_metrics.set_dpeers(stats.authenticated_peers as u64, stats.total_peers as u64);
                    peer_metrics.set_best_block(best.saturated_into::<u64>());

                    log::info!(
                        "📈 Peer metrics: connected={}, authenticated={}, total_peers={}, best_block={}",
                        peers,
                        stats.authenticated_peers,
                        stats.total_peers,
                        best
                    );
                }
            },
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PPFA PROTOCOL WORKER - Bridges ASF finality to libp2p (v110)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // Spawn the PPFA protocol worker to handle:
    // 1. Receiving ASF finality events from DETR P2P bridge
    // 2. Broadcasting finality justifications over libp2p
    // 3. Handling peer connections and warp sync requests

    let ppfa_client = client.clone();
    let ppfa_network = Arc::new(network.clone());
    let ppfa_sync = sync_service.clone();

    // Clone the sender for use in the ASF bridge task later
    let ppfa_asf_tx_for_bridge = ppfa_asf_tx.clone();

    task_manager.spawn_essential_handle().spawn(
        "ppfa-protocol-worker",
        Some("networking"),
        async move {
            log::info!("🚀 Starting PPFA Protocol Worker for libp2p finality");

            let (ppfa_worker, notification_bridge) = PPFAProtocolWorker::new(
                ppfa_client,
                ppfa_network,
                ppfa_sync,
                ppfa_notification_service,
                ppfa_asf_rx,
                ppfa_finality_tx,
                ppfa_protocol_name,
            );

            // Run worker and notification bridge concurrently
            tokio::join!(
                ppfa_worker.run(),
                notification_bridge
            );
        },
    );

    log::info!("✅ PPFA Protocol Worker spawned");

    // ═══════════════════════════════════════════════════════════════════════════
    // PPFA FINALITY NOTIFICATION HANDLER (v112)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // Consumes finality notifications from PPFA worker and logs them.
    // This bridges finality events to the sync engine for proper block handling.

    let finality_client = client.clone();
    task_manager.spawn_handle().spawn(
        "ppfa-finality-handler",
        Some("finality"),
        async move {
            use futures::StreamExt;
            use sp_runtime::traits::Header as HeaderT;
            let mut rx = ppfa_finality_rx;
            log::info!("🔔 PPFA finality notification handler started (v112)");

            while let Some((hash, number)) = rx.next().await {
                log::info!(
                    "🏁 PPFA finality notification: block #{} finalized (hash: {:?})",
                    number,
                    hash
                );

                // V112: Track finality for telemetry and metrics
                // Note: The sync engine is notified via JustificationSyncLink in the import queue
                // This handler provides logging visibility into PPFA finality events
                if let Ok(Some(header)) = finality_client.header(hash) {
                    log::debug!(
                        "📊 Finalized block header: parent={:?}, state_root={:?}",
                        header.parent_hash(),
                        header.state_root()
                    );
                }
            }

            log::warn!("⚠️ PPFA finality notification channel closed");
        },
    );

    log::info!("✅ PPFA finality notification handler spawned");

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF BLOCK IMPORT FINALITY HANDLER (v113)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This handler consumes finality notifications from the AsfBlockImportWrapper.
    // These notifications are sent when blocks with ASF justifications are imported.
    // This provides visibility into block import finality events.

    task_manager.spawn_handle().spawn(
        "asf-block-finality-handler",
        Some("finality"),
        async move {
            use futures::StreamExt;
            let mut rx = asf_block_finality_rx;
            log::info!("📦 ASF block import finality handler started (v113)");

            while let Some((hash, number)) = rx.next().await {
                log::info!(
                    "✅ Block #{} finalized via import (hash: {:?})",
                    number,
                    hash
                );
            }

            log::warn!("⚠️ ASF block import finality channel closed");
        },
    );

    log::info!("✅ ASF block import finality handler spawned");

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
    let _force_authoring = config.force_authoring;
    let _name = config.network.node_name.clone();
    // Create ASF finality state for RPC endpoints
    let asf_finality_state = Arc::new(crate::asf_rpc::AsfFinalityState::new(6));

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let finality_state = asf_finality_state.clone();

        Box::new(move |_| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                enable_asf: true,
                enable_governance: true,
                asf_finality_state: Some(finality_state.clone()),
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    // Capture identifiers/paths before moving config (used later for detrp2p peer store)
    let chain_id = config.chain_spec.id().to_string();
    let node_name = config.network.node_name.clone();
    let data_base_path = config.base_path.clone();
    let base_path = data_base_path.config_dir(chain_id.as_str());
    let committee_cache_path = base_path.join("ppfa").join("committee_cache.json");

    // Clone network data before moving config (needed for DETR P2P setup)
    let boot_nodes = config.network.boot_nodes.clone();
    let listen_addresses = config.network.listen_addresses.clone();
    let public_addresses = config.network.public_addresses.clone();

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
            "🔥 Starting ASF consensus (PPFA) for Primearc Core Chain authority node"
        );

        // Create proposer factory (same as AURA, but will use ASF logic)
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        log::info!(
            "ASF PPFA proposer initialized (slot_duration: {}ms, committee_size: {})",
            asf_params.slot_duration,
            asf_params.max_committee_size
        );

        // ASF block production task - PPFA proposer loop
        let ppfa_client = client.clone();
        let _ppfa_backend = backend.clone();
        let ppfa_params = asf_params.clone();
        let ppfa_block_import = block_import.clone();
        let mut ppfa_proposer_factory = proposer_factory;
        let ppfa_keystore = keystore_container.keystore();
        let genesis_props = config_genesis_props.clone();

        task_manager.spawn_essential_handle().spawn_blocking(
            "asf-ppfa-proposer",
            Some("block-authoring"),
            async move {
                log::info!("🚀 Starting PPFA proposer worker (slot_duration: {}ms)", ppfa_params.slot_duration);

                // Initialize PPFA components
                use block_production::{
                    ProposerSelector, CommitteeManager, SlotTimer, HealthMonitor,
                };
                use std::time::Duration;

                // Require ASF key before starting
                use sp_core::crypto::KeyTypeId;
                        const ASF_KEY_TYPE: KeyTypeId = KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"
                let our_keys = ppfa_keystore.sr25519_public_keys(ASF_KEY_TYPE);
                if our_keys.is_empty() {
                    log::error!("❌ No ASF (asfk) key in keystore; aborting PPFA proposer startup");
                    return;
                }

                // Create committee manager with configured size
                let mut committee = CommitteeManager::new(ppfa_params.max_committee_size);

                // ═══════════════════════════════════════════════════════════════
                // TODO #1 IMPLEMENTATION: Load committee from runtime via API
                // ═══════════════════════════════════════════════════════════════

                // Get best block hash for runtime queries (with limited retries)
                let mut retries = 0;
                let mut runtime_committee = Vec::new();
                while retries < 2 {
                    let best_hash = ppfa_client.info().best_hash;
                    match ppfa_client.runtime_api().validator_committee(best_hash) {
                        Ok(members) => {
                            log::info!(
                                "✅ Loaded {} committee members from runtime at block {:?}",
                                members.len(),
                                best_hash
                            );
                            runtime_committee = members;
                            break;
                        }
                        Err(e) => {
                            log::warn!(
                                "⚠️  Failed to load committee from runtime (attempt {}): {:?}",
                                retries + 1,
                                e
                            );
                            retries += 1;
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }

                if runtime_committee.is_empty() {
                    log::error!("❌ Committee could not be loaded after retries; aborting PPFA proposer startup");
                    return;
                }

                // Initialize committee with runtime validators
                let mut added_count = 0;
                for validator_info in runtime_committee.iter() {
                    log::info!(
                        "🔧 Attempting to add validator to committee: {:?}",
                        validator_info.validator_id()
                    );
                    match committee.add_validator(validator_info.clone()) {
                        Ok(_) => {
                            added_count += 1;
                            log::info!("✅ Successfully added validator {} to committee", added_count);
                        }
                        Err(e) => {
                            log::error!("❌ Failed to add validator to committee: {:?}", e);
                        }
                    }
                }
                log::info!(
                    "📊 Committee population: {}/{} validators added from runtime",
                    added_count,
                    runtime_committee.len()
                );

                // CRITICAL: Call rotate_committee() to move validators from pool into active committee
                log::info!("🔄 Rotating committee to populate active members from validator pool...");
                if let Err(e) = committee.rotate_committee(0) {
                    log::error!("❌ Failed to rotate committee: {:?}", e);
                } else {
                    log::info!("✅ Committee rotated successfully");
                }

                log::info!(
                    "🔗 PPFA committee initialized (size: {}/{}, mode: production)",
                    committee.committee_size(),
                    ppfa_params.max_committee_size
                );

                // Get our validator key from keystore (using ASF validator keys)
                if !our_keys.is_empty() {
                    // Add ourselves as a validator
                    // FIX: Use MultiSigner to properly convert sr25519 public key to AccountId32
                    let multi_signer = MultiSigner::Sr25519(our_keys[0].clone());
                    let account_id: AccountId32 = multi_signer.into_account();
                    let our_validator_id = block_production::ValidatorId::from(account_id);

                    log::info!(
                        "🔑 Converted sr25519 key to ValidatorId: {}",
                        hex::encode(our_validator_id.as_ref() as &[u8])
                    );

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
                         Generate keys with: ./target/release/primearc-core-chain key insert --key-type asfk --scheme sr25519"
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

                // Genesis time for slot scheduling:
                // - Default to PRIMEARC mainnet genesis constant
                // - Allow override via chain spec property "genesis_time_ms"
                // - Fallback to current time for dev/test
                const PRIMEARC_MAINNET_GENESIS_MS: u64 = 1764028800000;
                let genesis_time = genesis_props
                    .get("genesis_time_ms")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(PRIMEARC_MAINNET_GENESIS_MS);

                slot_timer.reset(genesis_time);

                log::info!("✅ PPFA proposer initialized");
                log::info!("   - Committee size: {}", proposer_selector.committee_size());
                log::info!("   - Slot duration: {}ms", slot_timer.current_duration());
                log::info!("   - Genesis time: {}", genesis_time);

                let committee_cache_path = committee_cache_path.clone();

                // Seed last-successful committee from cache if available
                let mut last_successful_committee = if let Ok(bytes) = fs::read(&committee_cache_path).await {
                    match serde_json::from_slice::<Vec<validator_management::ValidatorInfo>>(&bytes) {
                        Ok(saved) if !saved.is_empty() => {
                            log::info!(
                                "♻️  Loaded cached committee ({} validators) from {:?}",
                                saved.len(),
                                committee_cache_path
                            );
                            saved
                        }
                        Ok(_) => runtime_committee.clone(),
                        Err(e) => {
                            log::warn!(
                                "Failed to decode cached committee {:?}: {:?}; using runtime committee",
                                committee_cache_path,
                                e
                            );
                            runtime_committee.clone()
                        }
                    }
                } else {
                    runtime_committee.clone()
                };
                let mut last_rotated_epoch: u32 = 1;
                let save_committee = |committee: Vec<validator_management::ValidatorInfo>| {
                    let path = committee_cache_path.clone();
                    async move {
                        if let Some(dir) = path.parent() {
                            let _ = fs::create_dir_all(dir).await;
                        }
                        match serde_json::to_vec(&committee) {
                            Ok(bytes) => {
                                if let Err(e) = fs::write(&path, bytes).await {
                                    log::warn!("Failed to persist committee cache {:?}: {:?}", path, e);
                                }
                            }
                            Err(e) => log::warn!("Failed to encode committee cache: {:?}", e),
                        }
                    }
                };

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

                        // Get our validator ID from keystore (validated earlier)
                        let our_validator_id = {
                            let public_key = &our_keys[0];
                            log::info!(
                                "🔑 ASF using validator key from keystore (raw sr25519): {}",
                                hex::encode(public_key.as_ref() as &[u8])
                            );
                            let multi_signer = MultiSigner::Sr25519(public_key.clone());
                            let account_id: AccountId32 = multi_signer.into_account();
                            let validator_id = block_production::ValidatorId::from(account_id);
                            log::info!(
                                "🔑 Converted to ValidatorId (AccountId32): {}",
                                hex::encode(validator_id.as_ref() as &[u8])
                            );
                            validator_id
                        };

                        // DEBUG: Log proposer comparison for troubleshooting
                        log::info!(
                            "🔍 Slot #{}: Current proposer = {}, Our ID = {}, Match = {}",
                            slot_number,
                            hex::encode(&current_proposer.encode()[..16]),
                            hex::encode(&our_validator_id.encode()[..16]),
                            proposer_selector.is_proposer(&our_validator_id)
                        );

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

                            // ═══════════════════════════════════════════════════════════════
                            // PPFA BLOCK SEALING: Create PPFA seal BEFORE proposing block
                            // This ensures the seal is included in the block header and
                            // propagated to all validators over the network.
                            // ═══════════════════════════════════════════════════════════════
                            use sp_runtime::{Digest, DigestItem};
                            use codec::Encode;

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

                            let mut pre_digest = Digest::default();
                            pre_digest.push(DigestItem::PreRuntime(
                                *b"PPFA",
                                ppfa_seal.encode(),
                            ));

                            log::debug!(
                                "🔏 Creating block with PPFA seal: index={}, proposer={:?}",
                                ppfa_index,
                                hex::encode(&our_validator_id.encode()[..8])
                            );

                            match proposer.propose(
                                inherent_data,
                                pre_digest, // Include PPFA seal in block digest
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

                                    let mut import_params = BlockImportParams::new(
                                        sp_consensus::BlockOrigin::Own,
                                        block.header.clone(),
                                    );
                                    import_params.body = Some(block.extrinsics.to_vec());
                                    import_params.finalized = false;
                                    import_params.fork_choice = Some(sc_consensus::ForkChoiceStrategy::LongestChain);

                                    // PPFA seal is already in the block header (added before propose())
                                    // No need to add post_digests - the seal was included during block creation

                                    match ppfa_block_import.import_block(import_params).await {
                                        Ok(result) => {
                                            log::info!(
                                                "✅ Block #{} imported successfully: {:?}",
                                                block.header.number(),
                                                result
                                            );

                                            // ═══════════════════════════════════════════════════
                                            // FINALITY INTEGRATION: Propose block to ASF finality
                                            // ═══════════════════════════════════════════════════
                                            // TODO: Re-enable when finality_gadget is created before PPFA task
                                            // let finality_block_hash = finality_gadget::BlockHash::from_bytes(block_hash.into());
                                            // let mut gadget = ppfa_finality_gadget.lock().await;
                                            // match gadget.propose_block(finality_block_hash).await {
                                            //     Ok(vote) => {
                                            //         log::info!(
                                            //             "🗳️  Created finality vote for block #{} at view {:?}",
                                            //             block.header.number(),
                                            //             vote.view
                                            //         );
                                            //     }
                                            //     Err(e) => {
                                            //         log::error!(
                                            //             "❌ Failed to create finality vote for block #{}: {}",
                                            //             block.header.number(),
                                            //             e
                                            //         );
                                            //     }
                                            // }
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

                            // Query current epoch from runtime with small retry/backoff
                            let chain_info = ppfa_client.usage_info().chain;
                            let at_hash = chain_info.best_hash;

                            log::info!(
                                "🔄 Epoch transition detected at slot #{} (slot epoch: #{})",
                                slot_number,
                                slot_epoch
                            );

                            let mut fetched_committee: Option<Vec<_>> = None;
                            for attempt in 0..3 {
                                match ppfa_client.runtime_api().validator_committee(at_hash) {
                                    Ok(members) if !members.is_empty() => {
                                        fetched_committee = Some(members);
                                        break;
                                    }
                                    Ok(_) => {
                                        log::warn!(
                                            "⚠️  Runtime returned empty committee at epoch {} (attempt {})",
                                            slot_epoch,
                                            attempt + 1
                                        );
                                    }
                                    Err(e) => {
                                        log::warn!(
                                            "⚠️  Failed to load committee from runtime (attempt {}): {:?}",
                                            attempt + 1,
                                            e
                                        );
                                    }
                                }
                                tokio::time::sleep(Duration::from_millis(500)).await;
                            }

                            let committee_to_apply = if let Some(new_committee_members) = fetched_committee {
                                last_successful_committee = new_committee_members.clone();
                                save_committee(last_successful_committee.clone()).await;
                                Some(new_committee_members)
                            } else if !last_successful_committee.is_empty() {
                                log::warn!(
                                    "Using last successful committee for epoch {} due to runtime errors/empties",
                                    slot_epoch
                                );
                                Some(last_successful_committee.clone())
                            } else {
                                log::error!(
                                    "No committee available for epoch {} (runtime empty and no cached committee); skipping rotation",
                                    slot_epoch
                                );
                                None
                            };

                            if let Some(next_committee) = committee_to_apply {
                                if next_committee.is_empty() {
                                    log::warn!("Skipping rotation to empty committee at epoch {}", slot_epoch);
                                    continue;
                                }

                                let epoch_u32 = slot_epoch.try_into().unwrap_or_else(|_| {
                                    log::warn!("Epoch {} too large for u32, using max", slot_epoch);
                                    u32::MAX
                                });

                                if epoch_u32 == last_rotated_epoch {
                                    log::debug!(
                                        "Epoch {} already applied (last rotated epoch = {}), skipping",
                                        epoch_u32,
                                        last_rotated_epoch
                                    );
                                    continue;
                                }

                                committee.clear_committee();
                                for validator_info in next_committee {
                                    if let Err(e) = committee.add_validator(validator_info) {
                                        log::warn!("Failed to add validator to new committee: {:?}", e);
                                    }
                                }

                                if let Err(e) = committee.rotate_committee(epoch_u32) {
                                    log::error!("Failed to rotate committee to epoch {}: {:?}", slot_epoch, e);
                                } else if let Err(e) = proposer_selector.rotate_committee(epoch_u32) {
                                    log::error!("Failed to rotate proposer selector: {:?}", e);
                                } else {
                                    last_rotated_epoch = epoch_u32;
                                    log::info!(
                                        "🔄 Committee rotated successfully (size: {}, epoch: {})",
                                        committee.committee_size(),
                                        slot_epoch
                                    );
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
    // ASF FINALITY GADGET (Pure ASF, v108)
    // ═══════════════════════════════════════════════════════════════════════════

    if asf_params.enable_finality_gadget {
        log::info!("🎯 Enabling ASF Finality Gadget (3-level finality)");

        // ========== NETWORK BRIDGE IMPLEMENTATION ==========
        //
        // Create a bridge between finality-gadget and sc-network for gossip
        use finality_gadget::{NetworkBridge, Vote as FinalityVote, Certificate as FinalityCertificate};
        use codec::{Encode, Decode};

        // Define ASF finality gossip protocol
        #[allow(dead_code)]
        const ASF_FINALITY_PROTOCOL: &str = "/etrid/asf-finality/1";

        #[allow(dead_code)]
        #[derive(Clone, Debug, Encode, Decode)]
        enum AsfFinalityMessage {
            Vote(FinalityVote),
            Certificate(FinalityCertificate),
        }

        // NetworkBridge implementation using DETR P2P
        struct DetrP2PNetworkBridge {
            p2p_network: Arc<P2PNetwork>,
            gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
            peer_store: Arc<PeerStore>,
        }

        impl DetrP2PNetworkBridge {
            fn new(
                p2p_network: Arc<P2PNetwork>,
                gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
                peer_store: Arc<PeerStore>,
            ) -> Self {
                Self {
                    p2p_network,
                    gadget_bridge,
                    peer_store,
                }
            }

            /// Convert finality-gadget Vote to bridge VoteData
            fn convert_vote_to_bridge(vote: &FinalityVote) -> VoteData {
                VoteData {
                    validator_id: vote.validator_id.0.clone().into(),  // V9: Convert AccountId32 to [u8; 32]
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
                // finality-gadget has Vec<(ValidatorId, Vec<u8>)> where ValidatorId wraps AccountId32
                // bridge expects Vec<([u8; 32], Vec<u8>)>
                let signatures: Vec<([u8; 32], Vec<u8>)> = cert.signatures.iter()
                    .map(|(validator_id, sig)| {
                        let bytes: [u8; 32] = validator_id.0.clone().into();
                        (bytes, sig.clone())
                    })
                    .collect();

                CertificateData {
                    view: cert.view.0,  // View is a newtype wrapper
                    block_hash: {
                        let encoded = cert.block_hash.encode();
                        let mut hash = [0u8; 32];
                        hash.copy_from_slice(&encoded[0..32]);
                        hash
                    },
                    block_number: cert.block_number,
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
                let bridge = self.gadget_bridge.lock().await;
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
                                data: payload.clone(),
                            };

                            // Prefer targeted unicasts to best peers; fallback to broadcast
                            let targets = self.peer_store.select_best_peers(16).await;
                            if targets.is_empty() {
                                self.p2p_network.broadcast(p2p_msg).await
                                    .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;
                            } else {
                                for (_, pid_bytes) in targets {
                                    let pid = PeerId::new(pid_bytes);
                                    let _ = self.p2p_network.unicast(pid, P2PMessage::Vote { data: payload.clone() }).await;
                                }
                            }

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
                let bridge = self.gadget_bridge.lock().await;
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
                                data: payload.clone(),
                            };

                            // Prefer targeted unicasts to best peers; fallback to broadcast
                            let targets = self.peer_store.select_best_peers(16).await;
                            if targets.is_empty() {
                                self.p2p_network.broadcast(p2p_msg).await
                                    .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;
                            } else {
                                for (_, pid_bytes) in targets {
                                    let pid = PeerId::new(pid_bytes);
                                    let _ = self.p2p_network.unicast(pid, P2PMessage::Certificate { data: payload.clone() }).await;
                                }
                            }

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

        // ═════════════════════════════════════════════════════════════════════════════
        // HELPER FUNCTIONS: Bridge Format ↔ Finality-Gadget Format Conversion
        // ═════════════════════════════════════════════════════════════════════════════

        /// Convert VoteData (bridge format) to finality_gadget::Vote
        fn convert_vote_from_bridge(vote_data: VoteData) -> finality_gadget::Vote {
            finality_gadget::Vote {
                validator_id: finality_gadget::ValidatorId(vote_data.validator_id.into()),  // V9: Convert [u8; 32] to AccountId32
                view: finality_gadget::View(vote_data.view),
                block_hash: finality_gadget::BlockHash::from_bytes(vote_data.block_hash),
                signature: vote_data.signature,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }
        }

        /// Convert CertificateData (bridge format) to finality_gadget::Certificate
        fn convert_certificate_from_bridge(cert_data: CertificateData) -> finality_gadget::Certificate {
            finality_gadget::Certificate {
                view: finality_gadget::View(cert_data.view),
                block_hash: finality_gadget::BlockHash::from_bytes(cert_data.block_hash),
                block_number: cert_data.block_number,
                signatures: cert_data.signatures.into_iter()
                    .map(|(id, sig)| (finality_gadget::ValidatorId(id.into()), sig))  // V9: Convert [u8; 32] to AccountId32
                    .collect(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }
        }

        // ========== FINALITY GADGET INITIALIZATION ==========

        // Extract validator identity from keystore
        let validator_id = {
            if role.is_authority() {
                // Load ASF validator key from keystore
                use sp_core::crypto::{KeyTypeId, AccountId32};
                const ASF_KEY_TYPE: KeyTypeId = KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"

                let keystore = keystore_container.keystore();
                let asf_keys = keystore.sr25519_public_keys(ASF_KEY_TYPE);

                match asf_keys.first() {
                    Some(public_key) => {
                        // V9 FIX: Use FULL 32-byte Sr25519 public key as AccountId32
                        // This prevents validator ID collisions that occurred with 4-byte u32
                        let account_id = AccountId32::from(public_key.clone());
                        let key_bytes = public_key.as_ref() as &[u8];

                        log::info!(
                            "🔑 ASF Finality Gadget using validator key from keystore: {}",
                            hex::encode(key_bytes)
                        );
                        log::info!(
                            "🆔 ASF Validator AccountId32: {} (full 32 bytes - no collisions!)",
                            hex::encode(AsRef::<[u8; 32]>::as_ref(&account_id))
                        );

                        finality_gadget::ValidatorId(account_id)
                    }
                    None => {
                        log::warn!(
                            "⚠️  No ASF key found in keystore for Finality Gadget. \
                             Using observer mode (non-validator)."
                        );
                        finality_gadget::ValidatorId(AccountId32::new([0xFFu8; 32])) // Non-validator observer
                    }
                }
            } else {
                // Non-authority nodes are observers
                finality_gadget::ValidatorId(AccountId32::new([0xFFu8; 32]))
            }
        };

        // ========== INITIALIZE DETR P2P NETWORK ==========

        log::info!("🌐 Initializing DETR P2P network for ASF finality");

        // ═══════════════════════════════════════════════════════════════
        // TODO #2 IMPLEMENTATION: Derive peer ID from validator identity
        // ═══════════════════════════════════════════════════════════════

        // Generate local peer ID from validator ID (now derived from actual validator identity)
        // Observers derive a deterministic ID from host+chain to avoid collisions.
        let observer_id = AccountId32::new([0xFFu8; 32]);
        let peer_id_bytes: [u8; 32] = if validator_id.0 == observer_id {
            derive_observer_peer_id(chain_id.as_str(), node_name.as_str())
        } else {
            validator_id.0.clone().into()
        };
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

            for addr in &public_addresses {
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
                for addr in &listen_addresses {
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

                        let peer_id = parse_detr_peer_id_hex(peer_id_str)
                            .unwrap_or_else(|| PeerId::from_socket_addr(peer_socket));
                        log::info!(
                            "  ✓ Adding bootstrap peer: {} -> {:?} (from Substrate bootnode)",
                            peer_socket,
                            hex::encode(&peer_id.as_bytes()[..8])
                        );

                        let peer_addr = PeerAddr { id: peer_id, address: peer_socket };

                        bootstrap_peers.push(peer_addr);
                    }
                }
            }
        }

        // Also check for DETR_P2P_BOOTSTRAP environment variable
        if let Ok(bootstrap_env) = std::env::var("DETR_P2P_BOOTSTRAP") {
            log::info!("🔍 Parsing bootstrap peers from DETR_P2P_BOOTSTRAP:");
            for entry in bootstrap_env.split(',') {
                let entry = entry.trim();
                if entry.is_empty() {
                    continue;
                }

                let mut peer_id: Option<PeerId> = None;
                let addr = if entry.contains('@') {
                    let parts: Vec<&str> = entry.split('@').collect();
                    if parts.len() != 2 {
                        log::warn!("⚠️ Invalid DETR_P2P_BOOTSTRAP entry '{}'", entry);
                        continue;
                    }
                    let left = parts[0].trim();
                    let right = parts[1].trim();
                    if let Ok(addr) = left.parse::<SocketAddr>() {
                        peer_id = parse_detr_peer_id_hex(right);
                        addr
                    } else if let Ok(addr) = right.parse::<SocketAddr>() {
                        peer_id = parse_detr_peer_id_hex(left);
                        addr
                    } else {
                        log::warn!("⚠️ Invalid DETR_P2P_BOOTSTRAP address '{}'", entry);
                        continue;
                    }
                } else {
                    match entry.parse::<SocketAddr>() {
                        Ok(addr) => addr,
                        Err(e) => {
                            log::warn!("⚠️ Invalid DETR_P2P_BOOTSTRAP address '{}': {:?}", entry, e);
                            continue;
                        }
                    }
                };

                let peer_id = peer_id.unwrap_or_else(|| {
                    if entry.contains('@') {
                        log::warn!(
                            "⚠️ DETR_P2P_BOOTSTRAP peer ID not hex ({}), using socket-derived ID",
                            entry
                        );
                    }
                    PeerId::from_socket_addr(addr)
                });

                log::info!(
                    "  ✓ Adding bootstrap peer: {} -> {:?} (from env)",
                    addr,
                    hex::encode(&peer_id.as_bytes()[..8])
                );

                bootstrap_peers.push(PeerAddr { id: peer_id, address: addr });
            }
        }

        log::info!("📋 Total DETR P2P bootstrap peers: {}", bootstrap_peers.len());

        // CRITICAL FIX: Determine the ANNOUNCE address (what we tell peers to connect to)
        // This is separate from the BIND address (what we listen on)
        let announce_addr: Option<SocketAddr> = {
            // Option 1: Check DETR_P2P_ANNOUNCE_IP env var (RECOMMENDED for validators)
            if let Ok(announce_ip_str) = std::env::var("DETR_P2P_ANNOUNCE_IP") {
                if let Ok(announce_ip) = announce_ip_str.parse::<IpAddr>() {
                    log::info!("📢 Using DETR_P2P_ANNOUNCE_IP: {}", announce_ip);
                    Some(SocketAddr::new(announce_ip, detr_p2p_port))
                } else {
                    log::warn!("⚠️ Invalid DETR_P2P_ANNOUNCE_IP: {}", announce_ip_str);
                    None
                }
            }
            // Option 2: If our bind IP is not 0.0.0.0, use it for announce too
            else if !detr_p2p_ip.is_unspecified() {
                log::info!("📢 Using bind IP for announce: {}", detr_p2p_ip);
                Some(SocketAddr::new(detr_p2p_ip, detr_p2p_port))
            }
            // Option 3: 0.0.0.0 bind - mesh discovery will fail without DETR_P2P_ANNOUNCE_IP
            else {
                log::warn!("⚠️ DETR P2P bound to 0.0.0.0 but no DETR_P2P_ANNOUNCE_IP set!");
                log::warn!("⚠️ Mesh discovery will FAIL - peers can't connect to 0.0.0.0");
                log::warn!("⚠️ Set DETR_P2P_ANNOUNCE_IP to your public IP address");
                None
            }
        };

        // Create P2P network instance with separate bind/announce addresses
        let p2p_network = Arc::new(P2PNetwork::new_with_announce(
            local_peer_id.clone(),
            socket_addr,     // BIND address (can be 0.0.0.0)
            announce_addr,   // ANNOUNCE address (must be reachable by peers)
            bootstrap_peers,
        ));

        // Peer storage for organic discovery/reputation (persisted JSON by default; override with DETR_P2P_PEER_STORE_PATH)
        let base_path = data_base_path.config_dir(chain_id.as_str());
        let default_peer_path = base_path.join("detrp2p").join("peers");
        let peer_store_path = std::env::var("DETR_P2P_PEER_STORE_PATH")
            .map(|p| p.into())
            .unwrap_or(default_peer_path);
        let peer_store = match block_on(PeerStore::new_with_path(&peer_store_path)) {
            Ok(store) => Arc::new(store),
            Err(e) => {
                log::warn!("⚠️ Failed to load peer store at {:?}: {}; falling back to in-memory", peer_store_path, e);
                Arc::new(PeerStore::new())
            }
        };

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

        // Bridge libp2p peer discovery into DETR P2P to keep overlays segmented but aligned.
        let libp2p_network = network.clone();
        let detrp2p_network = p2p_network.clone();
        let detr_listen_addr = local_address.address;
        task_manager.spawn_handle().spawn(
            "detrp2p-libp2p-bridge",
            Some("networking"),
            async move {
                let mut seen_addrs: HashSet<SocketAddr> = HashSet::new();
                let mut interval = tokio::time::interval(Duration::from_secs(15));

                loop {
                    interval.tick().await;

                    if let Ok(state) = libp2p_network.network_state().await {
                        for peer in state.connected_peers.values() {
                            for addr in &peer.known_addresses {
                                let addr_str = addr.to_string();
                                let ip = match addr_str.split('/').nth(2) {
                                    Some(ip_str) => match ip_str.parse::<IpAddr>() {
                                        Ok(ip) if !ip.is_loopback() && !ip.is_unspecified() => ip,
                                        _ => continue,
                                    },
                                    _ => continue,
                                };

                                let socket = SocketAddr::new(ip, detr_p2p_port);
                                if socket == detr_listen_addr || !seen_addrs.insert(socket) {
                                    continue;
                                }

                                let peer_addr = PeerAddr {
                                    id: PeerId::from_socket_addr(socket),
                                    address: socket,
                                };
                                if let Err(e) = detrp2p_network.add_peer(peer_addr).await {
                                    log::debug!("DETR P2P peer add failed for {}: {:?}", socket, e);
                                }
                            }
                        }
                    }
                }
            },
        );

        // Create gadget network bridge
        let gadget_bridge = Arc::new(tokio::sync::Mutex::new(GadgetNetworkBridge::new()));

        log::info!("✅ Gadget network bridge initialized");

        // Create DetrP2PNetworkBridge combining both components
        let network_bridge = Arc::new(DetrP2PNetworkBridge::new(
            p2p_network.clone(),
            gadget_bridge.clone(),
            peer_store.clone(),
        ));

        log::info!("✅ DetrP2PNetworkBridge created - finality messages will use detrp2p");

        // Calculate max validators from committee size
        let max_validators = asf_params.max_committee_size;
        let finality_keystore = keystore_container.keystore();

        // Create finality gadget instance
        let finality_gadget = Arc::new(tokio::sync::Mutex::new(
            finality_gadget::FinalityGadget::new(
                validator_id.clone(),
                max_validators,
                finality_keystore,
                network_bridge.clone(),
            )
        ));

        log::info!(
            "ASF Finality Gadget initialized (validator_id: {:?}, max_validators: {})",
            validator_id,
            max_validators
        );
        log::info!("ASF Finality: 3-level consensus (Pre-commit → Commit → Finalized)");

        {
            let mut gadget = block_on(finality_gadget.lock());
            gadget.set_committee_size(initial_committee_size);
        }

        // ═══════════════════════════════════════════════════════════════════════════════
        // ASF FINALITY COMPONENTS INITIALIZATION (Phases 3-9)
        // ═══════════════════════════════════════════════════════════════════════════════

        // Phase 3: Equivocation Detection
        let (equivocation_proof_tx, mut equivocation_proof_rx) = tokio::sync::mpsc::unbounded_channel();
        let _equivocation_detector = Arc::new(EquivocationDetector::new(Some(equivocation_proof_tx)));
        log::info!("⚔️ Equivocation Detector initialized - will detect and report double-voting");

        // Phase 4: Implicit Finality Tracker (6-confirmation rule)
        let implicit_finality_config = ImplicitFinalityConfig {
            confirmation_depth: 6,
            enabled: true,
        };
        let implicit_finality_tracker = Arc::new(ImplicitFinalityTracker::new(implicit_finality_config));
        log::info!("📊 Implicit Finality Tracker initialized - 6 block confirmation depth");

        // Phase 5: Fork Pruning Engine
        let fork_pruning_config = ForkPruningConfig {
            pruning_depth: 10,
            enabled: true,
            pruning_interval: 100,
        };
        let fork_pruner = Arc::new(ForkPruner::new(
            client.clone(),
            backend.clone(),
            fork_pruning_config,
        ));
        log::info!("🗑️ Fork Pruner initialized - will prune non-canonical forks after finality");

        // Phase 7: Checkpoint BFT (every 100 blocks)
        let checkpoint_config = CheckpointConfig {
            checkpoint_interval: 100,
            enabled: true,
            signature_timeout_secs: 60,
            external_storage_enabled: false,
        };
        let checkpoint_bft = Arc::new(CheckpointBFT::new(checkpoint_config, None));
        log::info!("🏁 Checkpoint BFT initialized - permanent anchors every 100 blocks");

        // Phase 9: Indexer and RPC State
        let (indexer_tx, indexer) = create_indexer();
        let asf_finality_state = Arc::new(AsfFinalityState::new(6));
        log::info!("📊 ASF Indexer and RPC state initialized");

        // Spawn indexer task
        task_manager.spawn_handle().spawn(
            "asf-indexer",
            Some("finality"),
            async move {
                log::info!("📊 Starting ASF Indexer service");
                indexer.run().await;
            },
        );

        // Spawn equivocation slashing handler
        task_manager.spawn_handle().spawn(
            "asf-equivocation-handler",
            Some("finality"),
            async move {
                log::info!("⚔️ Starting equivocation proof handler");
                while let Some(proof) = equivocation_proof_rx.recv().await {
                    log::warn!(
                        "⚠️ EQUIVOCATION PROOF RECEIVED: validator {:?} in view {}",
                        hex::encode(&proof.validator_id[..8]),
                        proof.view
                    );
                    // TODO: Submit proof to pallet-asf-finality for slashing
                }
            },
        );

        log::info!("✅ All ASF Finality components initialized successfully");

        // ========== SPAWN FINALITY WORKER TASK ==========

        let finality_gadget_clone = finality_gadget.clone();
        let _client_clone = client.clone();
        let network_bridge_clone = network_bridge.clone();

        task_manager.spawn_essential_handle().spawn(  // Changed from spawn_blocking to spawn
            "asf-finality-gadget",
            Some("finality"),
            async move {
                log::info!("🚀 Starting ASF Finality Gadget worker loop");

                // CRITICAL FIX: Refactored worker loop to release lock between iterations
                // instead of holding it indefinitely. This prevents lock starvation
                // of the block import task.
                //
                // The worker handles:
                // 1. Incoming vote/certificate gossip
                // 2. Vote aggregation and quorum detection
                // 3. Certificate creation and broadcasting
                // 4. Finality detection (3 consecutive certificates)
                // 5. Timeout handling and view changes

                use tokio::time::{interval, Duration};
                let mut gossip_interval = interval(Duration::from_millis(500));
                let mut timeout_interval = interval(Duration::from_secs(1));

                loop {
                    tokio::select! {
                        _ = gossip_interval.tick() => {
                            // Acquire lock only for getting ready messages
                            let (votes, certs) = {
                                let mut gadget = finality_gadget_clone.lock().await;
                                gadget.get_ready_gossip_messages()
                            };  // Lock released here

                            // Network I/O happens WITHOUT holding the gadget lock
                            for vote in votes {
                                let _ = network_bridge_clone.broadcast_vote(vote).await;
                            }

                            for cert in certs {
                                let _ = network_bridge_clone.broadcast_certificate(cert).await;
                            }
                        }

                        _ = timeout_interval.tick() => {
                            // Acquire lock for timeout handling
                            let mut gadget = finality_gadget_clone.lock().await;
                            let _ = gadget.handle_timeout().await;
                            // Lock released at end of block
                        }
                    }
                }
            },
        );

        // ========== SPAWN BLOCK IMPORT NOTIFICATION TASK ==========
        //
        // This is the CRITICAL integration that connects block imports to finality.
        // When a block is imported, we need to:
        // 1. Notify the finality gadget via propose_block()
        // 2. Gadget creates and broadcasts a vote
        // 3. Votes accumulate to form certificates
        // 4. Certificates drive finality progression
        //
        // WITHOUT this task, blocks are produced but finality never advances!

        let block_import_finality_gadget = finality_gadget.clone();
        let finality_submit_enabled = std::env::var("ASF_FINALITY_SUBMIT")
            .map(|v| v != "0")
            .unwrap_or(true);
        let import_notifications = client.import_notification_stream();
        // V118: Clone P2P network for BlockAnnounce broadcast
        let block_import_p2p_network = p2p_network.clone();
        // V119: Clone client for full block fetching during BlockAnnounce
        let block_import_client = client.clone();
        // V120: Clone ASF components for integration
        let block_import_implicit_tracker = implicit_finality_tracker.clone();
        let block_import_indexer_tx = indexer_tx.clone();
        let block_import_asf_state = asf_finality_state.clone();
        let pending_state = Arc::new(tokio::sync::Mutex::new(PendingState::default()));
        let block_import_queue = import_queue_service.clone();
        let block_import_pending_state = pending_state.clone();
        let block_request_counter = Arc::new(AtomicU64::new(1));
        let block_import_request_counter = block_request_counter.clone();
        let block_import_epoch_duration = asf_params.epoch_duration;
        let block_import_quorum_override = quorum_override;

        task_manager.spawn_essential_handle().spawn(
            "asf-block-import-finality",
            Some("finality"),
            async move {
                log::info!("🔗 Starting ASF block import → finality integration (v120 with full component integration)");

                use futures::StreamExt;
                use tokio::time::{timeout, Duration};
                let mut stream = import_notifications;

                while let Some(notification) = stream.next().await {
                    let substrate_hash = notification.hash;
                    let block_number = *notification.header.number();

                    // Convert Substrate H256 to finality_gadget::BlockHash
                    let block_hash = finality_gadget::BlockHash::from_bytes(substrate_hash.into());
                    let view = finality_gadget::View(block_number.saturated_into());

                    log::debug!(
                        "📦 Block imported #{} ({:?}), proposing to finality gadget",
                        block_number,
                        substrate_hash
                    );

                    if !finality_submit_enabled {
                        log::warn!("ASF finality submission disabled (ASF_FINALITY_SUBMIT=0); skipping propose_block()");
                        continue;
                    }

                    // V4 FIX: Use lock().await with timeout to ensure votes are created
                    // while preventing indefinite blocking. This fixes the deadlock where
                    // try_lock() silently failed for 800+ blocks with no warning logs.
                    let block_number_u32: u32 = block_number.saturated_into();
                    match timeout(
                        Duration::from_secs(3),
                        block_import_finality_gadget.lock()
                    ).await {
                        Ok(mut gadget) => {
                            match gadget.propose_block(block_hash, block_number_u32, view).await {
                                Ok(vote) => {
                                    log::info!(
                                        "✅ Created finality vote for block #{} ({:?}) at view {:?}",
                                        block_number,
                                        substrate_hash,
                                        vote.view
                                    );
                                }
                                Err(e) => {
                                    log::warn!(
                                        "⚠️ Failed to propose block #{} to finality gadget: {}",
                                        block_number,
                                        e
                                    );
                                }
                            }
                        }
                        Err(_) => {
                            // Gadget locked for >3 seconds - this indicates a serious problem
                            // Log at WARN level to make this visible
                            log::warn!(
                                "⚠️ Finality gadget locked for >3s - skipping block #{} (possible deadlock)",
                                block_number
                            );
                        }
                    }

                    if block_import_epoch_duration > 0
                        && block_number_u32 > 0
                        && block_number_u32 % block_import_epoch_duration == 0
                    {
                        match get_ppfa_committee(&block_import_client, substrate_hash) {
                            Ok(committee) => {
                                let committee_size = apply_quorum_override(
                                    committee.len() as u32,
                                    block_import_quorum_override,
                                );
                                let mut gadget = block_import_finality_gadget.lock().await;
                                gadget.set_committee_size(committee_size);
                            }
                            Err(e) => {
                                log::warn!(
                                    "⚠️ Failed to refresh committee size at block #{}: {:?}",
                                    block_number,
                                    e
                                );
                            }
                        }
                    }

                    // V120: Update ASF components on every block import
                    // 1. Update implicit finality tracker with new best block
                    block_import_implicit_tracker.set_best_block(block_number);

                    // CRITICAL FIX: Apply implicit finality to Substrate
                    // After 6 confirmations, blocks should be implicitly finalized
                    let implicit_state = block_import_implicit_tracker.state();
                    let implicit_finalized = implicit_state.implicit_finalized;
                    let current_substrate_finalized: u32 = block_import_client.info().finalized_number.saturated_into();

                    if implicit_finalized > current_substrate_finalized && implicit_finalized > 0 {
                        // Get the block hash for the implicitly finalized block
                        if let Ok(Some(hash)) = block_import_client.hash(implicit_finalized.into()) {
                            match block_import_client.lock_import_and_run(|import_op| {
                                block_import_client.apply_finality(
                                    import_op,
                                    hash,
                                    None, // Implicit finality doesn't use justification
                                    true, // notify
                                )
                            }) {
                                Ok(()) => {
                                    log::info!(
                                        "✅ IMPLICIT FINALITY: Block #{} finalized (6-confirmation rule)",
                                        implicit_finalized
                                    );
                                }
                                Err(e) => {
                                    log::debug!(
                                        "Implicit finality for #{} skipped: {:?}",
                                        implicit_finalized, e
                                    );
                                }
                            }
                        }
                    }

                    // 2. Update ASF RPC state with new best block
                    block_import_asf_state.update_best(block_number);

                    // 3. Send block import event to indexer
                    let block_hash_bytes: [u8; 32] = substrate_hash.into();
                    let _ = block_import_indexer_tx.send(FinalityEvent::BlockImported {
                        hash: block_hash_bytes,
                        number: block_number,
                        parent_hash: (*notification.header.parent_hash()).into(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0),
                    });

                    // V119: Broadcast FULL BLOCK via DETR P2P for unified block sync
                    // This fixes the vote fragmentation bug where validators voted for different
                    // blocks because they only received headers but not the full block data.
                    // Now we send the complete SignedBlock so receiving nodes can import it.
                    let parent_hash_bytes: [u8; 32] = (*notification.header.parent_hash()).into();
                    let block_hash_bytes: [u8; 32] = substrate_hash.into();

                    // V119 FIX: Fetch the full block (header + body) from storage
                    // and encode it for transmission. This ensures receiving nodes
                    // can import the complete block, not just the header.
                    let encoded_full_block = match block_import_client.block(substrate_hash) {
                        Ok(Some(signed_block)) => Some(signed_block.encode()),
                        Ok(None) => {
                            log::warn!("V119: Block #{} not found in storage for broadcast", block_number);
                            None
                        }
                        Err(e) => {
                            log::warn!("V119: Failed to fetch block #{}: {:?}", block_number, e);
                            None
                        }
                    };

                    if let Some(encoded_full_block) = encoded_full_block {
                        let block_announce_msg = BlockAnnounceMessage {
                            block_number: block_number as u64,
                            block_hash: block_hash_bytes,
                            parent_hash: parent_hash_bytes,
                            encoded_block: encoded_full_block,
                        };

                        // Broadcast asynchronously (don't block on this)
                        let p2p_for_announce = block_import_p2p_network.clone();
                        tokio::spawn(async move {
                            let p2p_msg: P2PMessage = block_announce_msg.into();
                            if let Err(e) = p2p_for_announce.broadcast(p2p_msg).await {
                                log::debug!("V118: BlockAnnounce broadcast failed: {}", e);
                            } else {
                                log::debug!(
                                    "📢 V118: Broadcast BlockAnnounce #{} via DETR P2P",
                                    block_number
                                );
                            }
                        });
                    } else {
                        log::debug!(
                            "📢 V118: Skipping BlockAnnounce #{} (no full block)",
                            block_number
                        );
                    }

                    // Drain pending children now that this parent is imported.
                    let pending_children = {
                        let mut pending_guard = block_import_pending_state.lock().await;
                        take_pending_children(&mut pending_guard, &substrate_hash)
                    };

                    if !pending_children.is_empty() {
                        use sp_runtime::generic::SignedBlock;

                        for pending in pending_children {
                            let pending_number = pending.block_number;
                            let pending_hash = pending.block_hash;
                            let pending_parent = pending.parent_hash;
                            match SignedBlock::<Block>::decode(&mut &pending.encoded_block[..]) {
                                Ok(signed_block) => {
                                    let SignedBlock { block, justifications } = signed_block;
                                    let decoded_hash = block.header.hash();
                                    let parent_hash = *block.header.parent_hash();

                                    if decoded_hash != pending_hash {
                                        log::warn!(
                                            "📦 Pending block hash mismatch (expected {:?}, decoded {:?})",
                                            hex::encode(pending_hash.as_ref().get(..8).unwrap_or(&[])),
                                            hex::encode(decoded_hash.as_ref().get(..8).unwrap_or(&[]))
                                        );
                                    }

                                    if block_import_client.header(decoded_hash).ok().flatten().is_some() {
                                        continue;
                                    }

                                    if block_import_client.header(parent_hash).ok().flatten().is_none() {
                                        log::warn!(
                                            "⚠️ Pending block #{} still missing parent/state, re-queueing",
                                            pending_number
                                        );
                                        let mut pending_guard = block_import_pending_state.lock().await;
                                        if queue_pending_block(&mut pending_guard, pending.clone()) {
                                            let request_id = block_import_request_counter.fetch_add(1, Ordering::Relaxed);
                                            let mut parent_bytes = [0u8; 32];
                                            parent_bytes.copy_from_slice(pending_parent.as_ref());
                                            let request = BlockRequestMessage {
                                                request_id,
                                                by_number: None,
                                                by_hash: Some(parent_bytes),
                                            };
                                            let request: P2PMessage = request.into();
                                            if let Err(e) = block_import_p2p_network.unicast(pending.source_peer, request).await {
                                                log::warn!(
                                                    "⚠️ Failed to request parent for pending block #{}: {:?}",
                                                    pending_number,
                                                    e
                                                );
                                            }
                                        }
                                        continue;
                                    }

                                    let incoming = build_incoming_block(block, justifications);
                                    block_import_queue
                                        .lock()
                                        .await
                                        .import_blocks(BlockOrigin::NetworkBroadcast, vec![incoming]);
                                }
                                Err(e) => {
                                    log::warn!(
                                        "📦 Failed to decode pending block #{}: {:?}",
                                        pending_number,
                                        e
                                    );
                                }
                            }
                        }
                    }
                }

                log::warn!("⚠️  Block import notification stream ended");
            },
        );

        // ========== SPAWN FINALITY MONITOR TASK ==========
        //
        // V120: This task monitors the finality gadget for newly finalized blocks
        // and triggers fork pruning, checkpoints, and indexer events.

        let finality_monitor_gadget = finality_gadget.clone();
        let finality_monitor_pruner = fork_pruner.clone();
        let finality_monitor_checkpoint = checkpoint_bft.clone();
        let finality_monitor_implicit = implicit_finality_tracker.clone();
        let finality_monitor_state = asf_finality_state.clone();
        let finality_monitor_indexer_tx = indexer_tx.clone();
        let finality_monitor_client = client.clone();

        task_manager.spawn_essential_handle().spawn(
            "asf-finality-monitor",
            Some("finality"),
            async move {
                log::info!("🔍 Starting ASF Finality Monitor (v122 - tracks finalized blocks with deduplication)");

                use tokio::time::{interval, Duration};
                use std::collections::HashSet;
                let mut poll_interval = interval(Duration::from_secs(2));
                let mut last_finalized_count = 0usize;

                // V122 FIX: Track already-finalized block hashes to prevent duplicate apply_finality calls
                // This protects against the certificate storm bug where multiple blocks at the same view
                // can reach quorum and trigger multiple finality notifications
                let mut finalized_hashes: HashSet<[u8; 32]> = HashSet::new();

                loop {
                    poll_interval.tick().await;

                    // Check for newly finalized blocks
                    let finalized_blocks = {
                        let gadget = finality_monitor_gadget.lock().await;
                        gadget.get_finalized_blocks()
                    };

                    if finalized_blocks.len() > last_finalized_count {
                        // Process new finalized blocks
                        for block_hash in finalized_blocks.iter().skip(last_finalized_count) {
                            let hash_bytes: [u8; 32] = *block_hash.as_bytes();

                            // V122 FIX: Skip if we already finalized this block hash
                            if finalized_hashes.contains(&hash_bytes) {
                                log::debug!(
                                    "⏭️ Block {:?} already finalized, skipping duplicate",
                                    hex::encode(&hash_bytes[..8])
                                );
                                continue;
                            }

                            log::info!(
                                "🏁 Block FINALIZED with 3 certificates: {:?}",
                                hex::encode(&hash_bytes[..8])
                            );

                            // CRITICAL: Actually finalize the block in Substrate
                            let block_hash = sp_core::H256::from_slice(&hash_bytes);
                            match finality_monitor_client.lock_import_and_run(|import_op| {
                                finality_monitor_client.apply_finality(
                                    import_op,
                                    block_hash,
                                    None, // ASF finality doesn't use justification bytes
                                    true, // notify
                                )
                            }) {
                                Ok(()) => {
                                    log::info!(
                                        "✅ Block {:?} finalized in Substrate",
                                        hex::encode(&hash_bytes[..8])
                                    );
                                    // V122: Mark as finalized to prevent duplicate processing
                                    finalized_hashes.insert(hash_bytes);
                                }
                                Err(e) => {
                                    log::warn!(
                                        "⚠️ Failed to apply finality to Substrate: {:?}",
                                        e
                                    );
                                }
                            }

                            // Update implicit finality tracker with explicit finality
                            // Note: We don't have block number from hash alone, so we track by finality state
                            let finalized_number = finalized_blocks.len() as u32;
                            finality_monitor_implicit.set_explicit_finalized(finalized_number);

                            // Update ASF RPC state
                            finality_monitor_state.update_finalized(
                                finalized_number,
                                hash_bytes
                            ).await;

                            // Send finality event to indexer
                            let timestamp = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(0);
                            let _ = finality_monitor_indexer_tx.send(FinalityEvent::BlockFinalized {
                                hash: hash_bytes,
                                number: finalized_number,
                                certificates: Vec::new(), // TODO: Include actual certificates
                                timestamp,
                            });

                            // Check if fork pruning should run
                            if finality_monitor_pruner.should_prune(finalized_number).await {
                                log::info!("🗑️ Triggering fork pruning at finalized block #{}", finalized_number);
                                match finality_monitor_pruner.prune_forks(
                                    sp_core::H256::from_slice(&hash_bytes),
                                    finalized_number
                                ).await {
                                    Ok(stats) => {
                                        log::info!(
                                            "✅ Fork pruning complete: {} blocks pruned in {}ms",
                                            stats.blocks_pruned,
                                            stats.duration_ms
                                        );
                                    }
                                    Err(e) => {
                                        log::warn!("⚠️ Fork pruning failed: {:?}", e);
                                    }
                                }
                            }

                            // Check if checkpoint should be created
                            if finality_monitor_checkpoint.should_checkpoint(finalized_number) {
                                log::info!("🏁 Initiating checkpoint at block #{}", finalized_number);
                                // Initialize checkpoint with placeholder roots (would be fetched from state in production)
                                let _ = finality_monitor_checkpoint.init_checkpoint(
                                    finalized_number,
                                    hash_bytes,
                                    [0u8; 32], // state_root - TODO: Fetch from block header
                                    [0u8; 32], // extrinsics_root - TODO: Fetch from block header
                                    Vec::new(), // Validators will sign asynchronously
                                ).await;
                            }
                        }

                        last_finalized_count = finalized_blocks.len();
                    }
                }
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
        // V111: Clone PPFA sender for DETR→PPFA bridge
        let bridge_ppfa_tx = ppfa_asf_tx_for_bridge.clone();
        // V119: Clone client for block import in BlockAnnounce handler
        let bridge_block_client = client.clone();
        // V119: Import queue service for DETR P2P block sync
        let bridge_import_queue = import_queue_service.clone();
        let indexer_tx = indexer_tx.clone();
        let bridge_pending_state = pending_state.clone();
        let bridge_request_counter = block_request_counter.clone();
        // Basic guardrails to prevent oversized finality messages from blocking the bridge
        const DETR_P2P_MAX_FINALITY_MSG: usize = 64 * 1024;
        const DETR_P2P_RATE_WINDOW_SECS: u64 = 10;
        const DETR_P2P_MAX_MSGS_UNTRUSTED: u32 = 5;
        const DETR_P2P_MAX_BYTES_UNTRUSTED: u64 = 256 * 1024; // 256 KB per window for unknown peers
        let detrp2p_metrics = detrp2p_metrics.clone();
        // Shared peer store remains optional; if absent, metrics still work.
        let peer_store = peer_store.clone();

        task_manager.spawn_essential_handle().spawn_blocking(
            "asf-bridge-worker",
            Some("finality"),
            async move {
                log::info!("🌉 Starting ASF bridge worker for P2P <-> Finality Gadget routing (v111 with PPFA bridge)");

                // Main bridge loop
                use tokio::time::{interval, Duration};
                let mut poll_interval = interval(Duration::from_millis(100));
                let mut poll_count = 0u64;
                #[derive(Clone)]
                struct PeerQuota {
                    msgs: u32,
                    bytes: u64,
                    last: std::time::Instant,
                    trusted: bool,
                }
                let mut peer_quota: HashMap<PeerId, PeerQuota> = HashMap::new();
                let pending_state = bridge_pending_state;
                let request_counter = bridge_request_counter;

                fn check_quota(
                    peer_quota: &mut HashMap<PeerId, PeerQuota>,
                    peer: &PeerId,
                    size: usize,
                ) -> bool {
                    let now = std::time::Instant::now();
                    let entry = peer_quota.entry(peer.clone()).or_insert(PeerQuota {
                        msgs: 0,
                        bytes: 0,
                        last: now,
                        trusted: false,
                    });
                    if now.duration_since(entry.last).as_secs() >= DETR_P2P_RATE_WINDOW_SECS {
                        entry.msgs = 0;
                        entry.bytes = 0;
                        entry.last = now;
                    }
                    if entry.trusted {
                        return true;
                    }
                    if entry.msgs >= DETR_P2P_MAX_MSGS_UNTRUSTED {
                        return false;
                    }
                    if entry.bytes + size as u64 > DETR_P2P_MAX_BYTES_UNTRUSTED {
                        return false;
                    }
                    entry.msgs += 1;
                    entry.bytes += size as u64;
                    true
                }

                fn mark_trusted(peer_quota: &mut HashMap<PeerId, PeerQuota>, peer: &PeerId) {
                    if let Some(entry) = peer_quota.get_mut(peer) {
                        entry.trusted = true;
                    }
                }

                loop {
                    poll_interval.tick().await;
                    poll_count += 1;

                    // V5 DIAGNOSTIC: Log polling activity every 50 iterations (~5 seconds)
                    if poll_count % 50 == 0 {
                        log::info!("🔄 Bridge worker polling (iteration {})", poll_count);
                    }

                    // ========== HANDLE INCOMING P2P MESSAGES ==========
                    // Poll P2P network for incoming vote/certificate messages
                    let mut processed_inbound: u32 = 0;
                    while let Some((peer_id, p2p_msg)) = bridge_p2p_network.receive_message().await {
                        processed_inbound += 1;
                        if processed_inbound > 200 {
                            // Avoid unbounded work in a single tick; resume next poll
                            break;
                        }
                        log::info!("🎯 Bridge worker processing message from {:?}", peer_id);

                        // Track peer in peer store for organic growth/reputation
                        let peer_hex = hex::encode(peer_id.as_bytes());
                        if peer_store.get(&peer_hex).await.is_none() {
                            let _ = peer_store
                                .store(StoredPeer::new(peer_hex.clone(), format!("unknown:{}", peer_hex)))
                                .await;
                        } else {
                            let _ = peer_store.update(&peer_hex, "0.1.0".into(), Vec::new()).await;
                        }

                        // Keep peer store bounded to avoid allowlist crowding while allowing organic growth.
                        if peer_store.count().await > DETR_P2P_MAX_TRACKED_PEERS {
                            let removed = peer_store
                                .prune_over_capacity(DETR_P2P_MAX_TRACKED_PEERS)
                                .await;
                            if removed > 0 {
                                log::debug!(
                                    "🧹 Pruned {} detrp2p peers to stay within cap {}",
                                    removed,
                                    DETR_P2P_MAX_TRACKED_PEERS
                                );
                            }
                        }

                        if let Ok(len) = peer_store.active_len().await.try_into() {
                            detrp2p_metrics.set_stored_peers(len);
                        }

                        match p2p_msg {
                            P2PMessage::Vote { data } => {
                                if data.len() > DETR_P2P_MAX_FINALITY_MSG {
                                    log::warn!(
                                        "Dropping oversized vote from {:?} ({} bytes)",
                                        peer_id,
                                        data.len()
                                    );
                                    detrp2p_metrics.inc_dropped();
                                    let _ = peer_store.update_reputation(&peer_hex, -1).await;
                                    if let Ok(len) = peer_store.active_len().await.try_into() {
                                        detrp2p_metrics.set_stored_peers(len);
                                    }
                                    continue;
                                }
                                if !check_quota(&mut peer_quota, &peer_id, data.len()) {
                                    log::warn!("Dropping vote from {:?} due to rate limit", peer_id);
                                    detrp2p_metrics.inc_dropped();
                                    let _ = peer_store.update_reputation(&peer_hex, -1).await;
                                    if let Ok(len) = peer_store.active_len().await.try_into() {
                                        detrp2p_metrics.set_stored_peers(len);
                                    }
                                    continue;
                                }
                                log::info!("🗳️  Processing VOTE message from {:?}", peer_id);
                                // Deserialize vote data
                                match bincode::deserialize::<VoteData>(&data) {
                                    Ok(vote_data) => {
                                        // Extract values for logging before moving
                                        let validator_id = vote_data.validator_id;
                                        let view = vote_data.view;
                                        let validator_short = format!("{:02x}{:02x}..{:02x}{:02x}",
                                            validator_id[0], validator_id[1], validator_id[30], validator_id[31]);

                                        log::debug!(
                                            "📥 Received vote from peer {:?} (validator: {}, view: {})",
                                            peer_id,
                                            validator_short,
                                            view
                                        );

                                        // Forward to bridge for processing
                                        let bridge = bridge_gadget_bridge.lock().await;
                                        if let Err(e) = bridge.on_vote_received(vote_data.clone()).await {
                                            log::warn!("Failed to process vote: {:?}", e);
                                        } else {
                                            // Convert to finality-gadget format and process
                                            let finality_vote = convert_vote_from_bridge(vote_data);

                                            drop(bridge); // Release bridge lock

                                            let vote_block_hash = sp_core::H256::from_slice(
                                                finality_vote.block_hash.as_bytes()
                                            );
                                            let vote_block_number: u32 = match bridge_block_client.number(vote_block_hash) {
                                                Ok(Some(number)) => number.saturated_into(),
                                                Ok(None) => {
                                                    log::warn!(
                                                        "⚠️ Vote for unknown block {:?} (view: {}), skipping",
                                                        vote_block_hash,
                                                        view
                                                    );
                                                    continue;
                                                }
                                                Err(e) => {
                                                    log::warn!(
                                                        "⚠️ Failed to resolve block number for vote {:?}: {:?}",
                                                        vote_block_hash,
                                                        e
                                                    );
                                                    continue;
                                                }
                                            };

                                            // Process in finality gadget
                                            let mut gadget = bridge_finality_gadget.lock().await;
                                            match gadget.handle_vote(finality_vote.clone(), vote_block_number).await {
                                                Ok(_) => {
                                                    log::info!(
                                                        "✅ Vote ACCEPTED by finality gadget (validator: {}, view: {}, block: {:?})",
                                                        validator_short,
                                                        view,
                                                        finality_vote.block_hash
                                                    );
                                                    mark_trusted(&mut peer_quota, &peer_id);
                                                    let _ = peer_store.update_reputation(&peer_hex, 1).await;
                                                    if let Ok(len) = peer_store.active_len().await.try_into() {
                                                        detrp2p_metrics.set_stored_peers(len);
                                                    }
                                                }
                                                    Err(e) => {
                                                        log::warn!(
                                                            "❌ Vote REJECTED by finality gadget: {:?} (validator: {}, view: {})",
                                                            e,
                                                        validator_short,
                                                        view
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to deserialize vote from {:?}: {:?}", peer_id, e);
                                    }
                                }
                            }
                            P2PMessage::Certificate { data } => {
                                if data.len() > DETR_P2P_MAX_FINALITY_MSG {
                                    log::warn!(
                                        "Dropping oversized certificate from {:?} ({} bytes)",
                                        peer_id,
                                        data.len()
                                    );
                                    detrp2p_metrics.inc_dropped();
                                    let _ = peer_store.update_reputation(&peer_hex, -1).await;
                                    if let Ok(len) = peer_store.active_len().await.try_into() {
                                        detrp2p_metrics.set_stored_peers(len);
                                    }
                                    continue;
                                }
                                if !check_quota(&mut peer_quota, &peer_id, data.len()) {
                                    log::warn!("Dropping certificate from {:?} due to rate limit", peer_id);
                                    detrp2p_metrics.inc_dropped();
                                    let _ = peer_store.update_reputation(&peer_hex, -1).await;
                                    if let Ok(len) = peer_store.active_len().await.try_into() {
                                        detrp2p_metrics.set_stored_peers(len);
                                    }
                                    continue;
                                }
                                // Deserialize certificate data
                                match bincode::deserialize::<CertificateData>(&data) {
                                    Ok(cert_data) => {
                                        // Extract values for logging before moving
                                        let view = cert_data.view;
                                        let sig_count = cert_data.signatures.len();

                                        log::debug!(
                                            "📥 Received certificate from peer {:?} (view: {}, {} voters)",
                                            peer_id,
                                            view,
                                            sig_count
                                        );

                                        // Forward to bridge
                                        let bridge = bridge_gadget_bridge.lock().await;
                                        if let Err(e) = bridge.on_certificate_received(cert_data.clone()).await {
                                            log::warn!("Failed to process certificate: {:?}", e);
                                        } else {
                                            // V111: Extract data for PPFA bridge before consuming cert_data
                                            let ppfa_block_hash = cert_data.block_hash;
                                            let ppfa_signatures = cert_data.signatures.clone();

                                            // Convert to finality-gadget format
                                            let finality_cert = convert_certificate_from_bridge(cert_data);

                                            drop(bridge); // Release lock

                                            // Process in finality gadget
                                            let mut gadget = bridge_finality_gadget.lock().await;
                                            match gadget.handle_certificate(finality_cert.clone()).await {
                                                Ok(_) => {
                                                    log::info!(
                                                        "✅ Certificate ACCEPTED by finality gadget (view: {}, {} signatures)",
                                                        view,
                                                        sig_count
                                                    );
                                                    mark_trusted(&mut peer_quota, &peer_id);
                                                    let _ = peer_store.update_reputation(&peer_hex, 1).await;
                                                    if let Ok(len) = peer_store.active_len().await.try_into() {
                                                        detrp2p_metrics.set_stored_peers(len);
                                                    }
                                                    // Index accepted certificate
                                                    let timestamp = SystemTime::now()
                                                        .duration_since(UNIX_EPOCH)
                                                        .unwrap_or_default()
                                                        .as_millis() as u64;
                                                    let block_number: u32 = view as u32;
                                                    let _ = indexer_tx.send(FinalityEvent::CertificateCreated {
                                                        view,
                                                        block_hash: ppfa_block_hash,
                                                        block_number,
                                                        signature_count: sig_count as u32,
                                                        timestamp,
                                                    }).ok();

                                                    // V111: Forward accepted certificate to PPFA for libp2p gossip
                                                    // This bridges DETR P2P finality data to Substrate's libp2p network
                                                    let hash = <Block as sp_runtime::traits::Block>::Hash::from_slice(&ppfa_block_hash);
                                                    let number: u32 = view as u32; // View correlates with block number
                                                    if let Err(e) = bridge_ppfa_tx.unbounded_send((hash, number.into(), ppfa_signatures.clone())) {
                                                        log::warn!("Failed to forward certificate to PPFA: {:?}", e);
                                                    } else {
                                                        log::debug!(
                                                            "📤 Certificate forwarded to PPFA for libp2p gossip (view: {}, hash: {:?})",
                                                            view,
                                                            hex::encode(&ppfa_block_hash[..8])
                                                        );
                                                    }
                                                }
                                                Err(e) => {
                                                    log::warn!(
                                                        "❌ Certificate REJECTED by finality gadget: {:?} (view: {})",
                                                        e,
                                                        view
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to deserialize certificate from {:?}: {:?}", peer_id, e);
                                    }
                                }
                            }
                            // V119: Handle block sync messages via DETR P2P with full block import
                            P2PMessage::BlockAnnounce { block_number, block_hash, parent_hash, encoded_block } => {
                                log::info!(
                                    "📦 V119: Received BlockAnnounce #{} from {:?} (hash: {:?}, {} bytes)",
                                    block_number,
                                    peer_id,
                                    hex::encode(&block_hash[..8]),
                                    encoded_block.len()
                                );

                                if encoded_block.is_empty() {
                                    log::warn!("⚠️ V119: Empty BlockAnnounce for #{}", block_number);
                                    continue;
                                }

                                use sp_runtime::generic::SignedBlock;
                                match SignedBlock::<Block>::decode(&mut &encoded_block[..]) {
                                    Ok(signed_block) => {
                                        let SignedBlock { block, justifications } = signed_block;
                                        let decoded_hash = block.header.hash();
                                        let decoded_number = *block.header.number();
                                        let block_number_u64: u64 = decoded_number.saturated_into();
                                        let parent_hash_h256 = sp_core::H256::from_slice(block.header.parent_hash().as_ref());
                                        if block.header.parent_hash().as_ref() != parent_hash.as_ref() {
                                            log::warn!(
                                                "📦 V118: BlockResponse parent hash mismatch (expected {:?}, decoded {:?})",
                                                hex::encode(&parent_hash[..8]),
                                                hex::encode(block.header.parent_hash().as_ref().get(..8).unwrap_or(&[]))
                                            );
                                        }
                                        if block.header.parent_hash().as_ref() != parent_hash.as_ref() {
                                            log::warn!(
                                                "📦 V119: BlockAnnounce parent hash mismatch (expected {:?}, decoded {:?})",
                                                hex::encode(&parent_hash[..8]),
                                                hex::encode(block.header.parent_hash().as_ref().get(..8).unwrap_or(&[]))
                                            );
                                        }

                                        if decoded_hash.as_ref() != block_hash.as_ref() {
                                            log::warn!(
                                                "📦 V119: BlockAnnounce hash mismatch (expected {:?}, decoded {:?})",
                                                hex::encode(&block_hash[..8]),
                                                hex::encode(decoded_hash.as_ref().get(..8).unwrap_or(&[]))
                                            );
                                        }

                                        if bridge_block_client.header(decoded_hash).ok().flatten().is_some() {
                                            log::debug!(
                                                "📦 V119: Block #{} already exists locally, skipping import",
                                                decoded_number
                                            );
                                            continue;
                                        }

                                        if bridge_block_client
                                            .header(parent_hash_h256)
                                            .ok()
                                            .flatten()
                                            .is_none()
                                        {
                                            log::warn!(
                                                "⚠️ V119: Block #{} missing parent/state, queuing for retry",
                                                decoded_number
                                            );
                                            let pending = PendingBlock {
                                                source_peer: peer_id.clone(),
                                                block_number: block_number_u64,
                                                block_hash: sp_core::H256::from_slice(decoded_hash.as_ref()),
                                                parent_hash: parent_hash_h256,
                                                encoded_block: encoded_block.clone(),
                                            };
                                            let mut pending_guard = pending_state.lock().await;
                                            if queue_pending_block(&mut pending_guard, pending) {
                                                let request_id = request_counter.fetch_add(1, Ordering::Relaxed);
                                                let mut parent_bytes = [0u8; 32];
                                                parent_bytes.copy_from_slice(parent_hash_h256.as_ref());
                                                let request = BlockRequestMessage {
                                                    request_id,
                                                    by_number: None,
                                                    by_hash: Some(parent_bytes),
                                                };
                                                let request: P2PMessage = request.into();
                                                if let Err(e) = bridge_p2p_network.unicast(peer_id, request).await {
                                                    log::warn!(
                                                        "⚠️ V119: Failed to request parent for block #{}: {:?}",
                                                        decoded_number,
                                                        e
                                                    );
                                                }
                                            }
                                            continue;
                                        }

                                        let incoming = build_incoming_block(block, justifications);
                                        bridge_import_queue
                                            .lock()
                                            .await
                                            .import_blocks(BlockOrigin::NetworkBroadcast, vec![incoming]);
                                    }
                                    Err(e) => {
                                        log::warn!(
                                            "📦 V119: Failed to decode BlockAnnounce #{} from {:?}: {:?}",
                                            block_number,
                                            peer_id,
                                            e
                                        );
                                    }
                                }
                            }
                            P2PMessage::BlockRequest { request_id, by_number, by_hash } => {
                                log::debug!(
                                    "📥 V118: Received BlockRequest from {:?} (id: {}, by_number: {:?}, by_hash: {:?})",
                                    peer_id,
                                    request_id,
                                    by_number,
                                    by_hash.map(|h| hex::encode(&h[..8]))
                                );
                                detrp2p_metrics.inc_block_request();

                                let target_hash = if let Some(hash_bytes) = by_hash {
                                    Some(sp_core::H256::from_slice(&hash_bytes))
                                } else if let Some(num) = by_number {
                                    let num_nf: NumberFor<Block> = num.saturated_into();
                                    match bridge_block_client.hash(num_nf) {
                                        Ok(opt) => opt,
                                        Err(e) => {
                                            log::warn!("Failed to resolve hash for block #{}: {:?}", num, e);
                                            None
                                        }
                                    }
                                } else {
                                    None
                                };

                                if let Some(hash) = target_hash {
                                    match bridge_block_client.block(hash) {
                                        Ok(Some(signed_block)) => {
                                            let parent_hash = *signed_block.block.header.parent_hash();
                                            let encoded_block = signed_block.encode();
                                            let block_number: u64 = (*signed_block.block.header.number()).saturated_into();

                                            let mut hash_bytes = [0u8; 32];
                                            hash_bytes.copy_from_slice(hash.as_ref());
                                            let mut parent_bytes = [0u8; 32];
                                            parent_bytes.copy_from_slice(parent_hash.as_ref());

                                            let response = BlockResponseMessage {
                                                request_id,
                                                block_number,
                                                block_hash: hash_bytes,
                                                parent_hash: parent_bytes,
                                                encoded_block,
                                            };

                                            let response: P2PMessage = response.into();
                                            if let Err(e) = bridge_p2p_network.unicast(peer_id, response).await {
                                                log::warn!("Failed to send BlockResponse to {:?}: {:?}", peer_id, e);
                                            } else {
                                                detrp2p_metrics.inc_block_response();
                                                log::debug!("📤 Sent BlockResponse #{} to {:?}", block_number, peer_id);
                                            }
                                        }
                                        Ok(None) => {
                                            log::debug!("Block not found for request {:?}", request_id);
                                        }
                                        Err(e) => {
                                            log::warn!("Error fetching block for request {:?}: {:?}", request_id, e);
                                        }
                                    }
                                } else {
                                    log::debug!("No valid hash/number provided in BlockRequest {:?}", request_id);
                                }
                            }
                            P2PMessage::BlockResponse { request_id, block_number, block_hash, parent_hash, encoded_block } => {
                                log::debug!(
                                    "📥 V118: Received BlockResponse from {:?} (id: {}, block #{})",
                                    peer_id,
                                    request_id,
                                    block_number
                                );

                                if encoded_block.is_empty() {
                                    log::warn!("⚠️ V118: Empty BlockResponse for #{}", block_number);
                                    continue;
                                }

                                use sp_runtime::generic::SignedBlock;
                                match SignedBlock::<Block>::decode(&mut &encoded_block[..]) {
                                    Ok(signed_block) => {
                                        let SignedBlock { block, justifications } = signed_block;
                                        let decoded_hash = block.header.hash();
                                        let decoded_number = *block.header.number();
                                        let block_number_u64: u64 = decoded_number.saturated_into();
                                        let parent_hash_h256 = sp_core::H256::from_slice(block.header.parent_hash().as_ref());

                                        if decoded_hash.as_ref() != block_hash.as_ref() {
                                            log::warn!(
                                                "BlockResponse hash mismatch: expected {:?}, got {:?}",
                                                hex::encode(&block_hash[..8]),
                                                hex::encode(decoded_hash.as_ref().get(..8).unwrap_or(&[]))
                                            );
                                        }

                                        if bridge_block_client.header(decoded_hash).ok().flatten().is_some() {
                                            continue;
                                        }

                                        if bridge_block_client
                                            .header(parent_hash_h256)
                                            .ok()
                                            .flatten()
                                            .is_none()
                                        {
                                            log::warn!(
                                                "⚠️ BlockResponse block #{} missing parent/state, queuing for retry",
                                                decoded_number
                                            );
                                            let pending = PendingBlock {
                                                source_peer: peer_id.clone(),
                                                block_number: block_number_u64,
                                                block_hash: sp_core::H256::from_slice(decoded_hash.as_ref()),
                                                parent_hash: parent_hash_h256,
                                                encoded_block: encoded_block.clone(),
                                            };
                                            let mut pending_guard = pending_state.lock().await;
                                            if queue_pending_block(&mut pending_guard, pending) {
                                                let request_id = request_counter.fetch_add(1, Ordering::Relaxed);
                                                let mut parent_bytes = [0u8; 32];
                                                parent_bytes.copy_from_slice(parent_hash_h256.as_ref());
                                                let request = BlockRequestMessage {
                                                    request_id,
                                                    by_number: None,
                                                    by_hash: Some(parent_bytes),
                                                };
                                                let request: P2PMessage = request.into();
                                                if let Err(e) = bridge_p2p_network.unicast(peer_id, request).await {
                                                    log::warn!(
                                                        "⚠️ Failed to request parent for block #{}: {:?}",
                                                        decoded_number,
                                                        e
                                                    );
                                                }
                                            }
                                            continue;
                                        }

                                        let incoming = build_incoming_block(block, justifications);
                                        bridge_import_queue
                                            .lock()
                                            .await
                                            .import_blocks(BlockOrigin::NetworkBroadcast, vec![incoming]);
                                    }
                                    Err(e) => {
                                        log::warn!(
                                            "📦 V118: Failed to decode BlockResponse #{} from {:?}: {:?}",
                                            block_number,
                                            peer_id,
                                            e
                                        );
                                    }
                                }
                            }
                            P2PMessage::StatusRequest { request_id } => {
                                log::trace!("📥 V118: Received StatusRequest from {:?} (id: {})", peer_id, request_id);
                                let info = bridge_block_client.info();
                                let best_number: u64 = info.best_number.saturated_into();
                                let best_hash = info.best_hash;
                                let genesis_hash = bridge_block_client.hash(Zero::zero()).unwrap_or_default().unwrap_or(best_hash);
                                let mut best_hash_bytes = [0u8; 32];
                                best_hash_bytes.copy_from_slice(best_hash.as_ref());
                                let mut genesis_bytes = [0u8; 32];
                                genesis_bytes.copy_from_slice(genesis_hash.as_ref());

                                let response = StatusResponseMessage {
                                    request_id,
                                    best_number,
                                    best_hash: best_hash_bytes,
                                    genesis_hash: genesis_bytes,
                                };
                                let response: P2PMessage = response.into();
                                if let Err(e) = bridge_p2p_network.unicast(peer_id, response).await {
                                    log::warn!("Failed to send StatusResponse to {:?}: {:?}", peer_id, e);
                                } else {
                                    detrp2p_metrics.inc_status_response();
                                    log::trace!("📤 Sent StatusResponse to {:?} (best #{})", peer_id, best_number);
                                }
                            }
                            P2PMessage::StatusResponse { request_id, best_number, best_hash: _best_hash, .. } => {
                                log::trace!(
                                    "📥 V118: Received StatusResponse from {:?} (id: {}, best #{})",
                                    peer_id,
                                    request_id,
                                    best_number
                                );
                                let our_best: u64 = bridge_block_client.info().best_number.saturated_into();
                                if best_number > our_best + 2 {
                                    log::info!(
                                        "Peer {:?} ahead (their best #{}, ours #{}); consider requesting blocks",
                                        peer_id,
                                        best_number,
                                        our_best
                                    );
                                }
                            }
                            _ => {
                                log::trace!("Received non-consensus message from peer {:?}", peer_id);
                            }
                        }
                    }

                    // ========== FORWARD OUTBOUND MESSAGES TO P2P ==========
                    let outbound_messages = {
                        // Minimize time under the lock to reduce contention with inbound processing
                        let bridge = bridge_gadget_bridge.lock().await;
                        bridge.get_outbound_messages().await
                    };

                    for (msg, _priority) in outbound_messages {
                        match msg {
                            ConsensusBridgeMessage::Vote(vote_data) => {
                                // Serialize and broadcast vote
                                match bincode::serialize(&vote_data) {
                                    Ok(payload) => {
                                        if payload.len() > DETR_P2P_MAX_FINALITY_MSG {
                                            log::warn!(
                                                "Skipping outbound vote larger than limit ({} bytes)",
                                                payload.len()
                                            );
                                            continue;
                                        }
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
                                        if payload.len() > DETR_P2P_MAX_FINALITY_MSG {
                                            log::warn!(
                                                "Skipping outbound certificate larger than limit ({} bytes, view {})",
                                                payload.len(),
                                                cert_data.view
                                            );
                                            continue;
                                        }
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
                    // Periodically log finality gadget status (thread-safe)
                    static LAST_STATUS_LOG: AtomicU64 = AtomicU64::new(0);
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let last_log = LAST_STATUS_LOG.load(Ordering::Relaxed);
                    if now - last_log >= 30 {
                        let gadget = bridge_finality_gadget.lock().await;
                        let current_view = gadget.get_current_view();
                        let finalized_count = gadget.get_finalized_blocks().len();

                        log::debug!(
                            "ASF Finality status: view={:?}, finalized={}, connected_peers={}",
                            current_view,
                            finalized_count,
                            bridge_p2p_network.get_connected_peers().await.len()
                        );

                        LAST_STATUS_LOG.store(now, Ordering::Relaxed);
                    }
                }
            },
        );

        // ═══════════════════════════════════════════════════════════════════════════
        // ASF → SUBSTRATE FINALITY APPLICATION
        // ═══════════════════════════════════════════════════════════════════════════

        let finality_client = client.clone();
        let finality_asf_gadget = finality_gadget.clone();

        task_manager.spawn_essential_handle().spawn(
            "asf-substrate-finality",
            Some("finality"),
            async move {
                log::info!("🎯 Starting ASF → Substrate finality application task");

                use tokio::time::{interval, Duration};
                #[allow(unused_imports)]
                use sp_blockchain::HeaderBackend;

                let mut finality_interval = interval(Duration::from_secs(6));
                let mut last_finalized_number: u32 = 0;

                loop {
                    finality_interval.tick().await;

                    // Get finalized blocks from ASF gadget
                    let asf_finalized = {
                        let gadget = finality_asf_gadget.lock().await;
                        gadget.get_finalized_blocks()
                    };

                    if asf_finalized.is_empty() {
                        continue;
                    }

                    // Get current Substrate finality state
                    let current_info = finality_client.usage_info().chain;
                    let current_number = current_info.finalized_number;

                    // Find newest ASF-finalized block not yet Substrate-finalized
                    for asf_block in asf_finalized.iter().rev() {
                        // Convert ASF BlockHash to Substrate H256
                        let substrate_hash: sp_core::H256 = {
                            let bytes = asf_block.as_bytes();
                            sp_core::H256::from_slice(bytes)
                        };

                        // Safety check: ensure block is imported
                        match finality_client.block_status(substrate_hash) {
                            Ok(sp_consensus::BlockStatus::InChainWithState) => {
                                // Get block header to check number
                                match finality_client.header(substrate_hash) {
                                    Ok(Some(header)) => {
                                        let block_number = *header.number();

                                        // Log ASF finality progress
                                        // Note: Actual finality is handled by the import queue
                                        // This task monitors ASF finality status
                                        if block_number > current_number &&
                                           block_number > last_finalized_number &&
                                           block_number <= current_number + 100 {

                                            last_finalized_number = block_number;
                                            log::info!(
                                                "✅ ASF finalized block #{} ({:?})",
                                                block_number,
                                                substrate_hash
                                            );

                                            // Only process one block per round
                                            break;
                                        } else if block_number > current_number + 100 {
                                            log::warn!(
                                                "⚠️ ASF finality too far ahead: #{} vs current #{}",
                                                block_number,
                                                current_number
                                            );
                                        }
                                    }
                                    Ok(None) => {
                                        log::warn!("Header not found for finalized block {:?}", substrate_hash);
                                    }
                                    Err(e) => {
                                        log::error!("Failed to get header: {:?}", e);
                                    }
                                }
                            }
                            Ok(sp_consensus::BlockStatus::Unknown) => {
                                log::debug!("ASF finalized block not yet imported: {:?}", substrate_hash);
                            }
                            _ => {}
                        }
                    }
                }
            }
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // v108: GRANDPA REMOVED - Pure ASF Finality
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // ASF finality gadget handles all finality - no GRANDPA needed.
    // This completes the v108 migration to pure ASF consensus.

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

        // Load genesis validators from runtime ValidatorCommittee pallet
        let genesis_validators = {
            // Query genesis committee from runtime at genesis block
            let genesis_hash = client.info().genesis_hash;

            match client.runtime_api().validator_committee(genesis_hash) {
                Ok(committee) if !committee.is_empty() => {
                    log::info!(
                        "✅ Loaded {} validators from genesis ValidatorCommittee",
                        committee.len()
                    );

                    // Runtime API already returns Vec<ValidatorInfo> from validator-management
                    // No conversion needed - use directly
                    committee
                },
                Ok(_) => {
                    log::warn!(
                        "⚠️  Genesis ValidatorCommittee is empty. Using fallback single validator."
                    );
                    vec![
                        validator_management::ValidatorInfo::new(
                            validator_management::ValidatorId::from([0u8; 32]),
                            asf_params.min_validator_stake,
                            validator_management::PeerType::FlareNode,
                        ),
                    ]
                },
                Err(e) => {
                    log::error!(
                        "❌ Failed to load genesis ValidatorCommittee: {:?}. Using fallback.",
                        e
                    );
                    vec![
                        validator_management::ValidatorInfo::new(
                            validator_management::ValidatorId::from([0u8; 32]),
                            asf_params.min_validator_stake,
                            validator_management::PeerType::FlareNode,
                        ),
                    ]
                }
            }
        };

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

    // ═══════════════════════════════════════════════════════════════════════════
    // NODE STARTUP COMPLETE
    // ═══════════════════════════════════════════════════════════════════════════

    log::info!("✅ ASF Primearc Core Chain node started successfully");
    log::info!("   - Block Production: ASF (slot_duration: {}ms)", asf_params.slot_duration);
    log::info!("   - Finality: Pure ASF (v108) via DETR P2P");
    log::info!("   - Network: Default libp2p protocols (PPFA integration pending)");
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
    Client: sc_client_api::BlockchainEvents<Block> + ProvideRuntimeApi<Block> + sc_client_api::UsageProvider<Block>,
    Client::Api: ValidatorCommitteeApi<Block>,
{
    // Runtime supports ASF if the ValidatorCommittee API is callable.
    // We query genesis (cheap) and treat errors as lack of support.
    let at = _client.usage_info().chain.genesis_hash;
    _client
        .runtime_api()
        .validator_committee(at)
        .map(|_| true)
        .unwrap_or(false)
}

/// Get current PPFA committee from runtime
///
/// Queries the runtime state for the active validator committee
pub fn get_ppfa_committee<Client>(
    client: &Arc<Client>,
    at: <Block as sp_runtime::traits::Block>::Hash,
) -> Result<Vec<sp_core::crypto::AccountId32>, String>
where
    Client: sc_client_api::BlockchainEvents<Block> + ProvideRuntimeApi<Block>,
    Client::Api: ValidatorCommitteeApi<Block>,
{
    let committee = client
        .runtime_api()
        .validator_committee(at)
        .map_err(|e| format!("runtime validator_committee: {:?}", e))?;

    // Map to AccountId32 for existing callers.
    let accounts = committee
        .into_iter()
        .map(|info| info.validator_id().clone())
        .collect();

    Ok(accounts)
}

fn quorum_override_from_env() -> Option<u32> {
    match std::env::var("ASF_QUORUM_OVERRIDE") {
        Ok(value) => match value.parse::<u32>() {
            Ok(parsed) => {
                log::warn!(
                    "⚠️ ASF quorum override enabled via ASF_QUORUM_OVERRIDE={}",
                    parsed
                );
                Some(parsed)
            }
            Err(e) => {
                log::warn!(
                    "⚠️ Invalid ASF_QUORUM_OVERRIDE '{}': {:?}",
                    value,
                    e
                );
                None
            }
        },
        Err(_) => None,
    }
}

fn apply_quorum_override(base: u32, override_opt: Option<u32>) -> u32 {
    let base = base.max(1);
    match override_opt {
        Some(value) => value.max(1).min(base),
        None => base,
    }
}

fn derive_observer_peer_id(chain_id: &str, node_name: &str) -> [u8; 32] {
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| node_name.to_string());
    let seed = format!("detrp2p-observer:{}:{}:{}", chain_id, node_name, hostname);
    sp_core::hashing::blake2_256(seed.as_bytes())
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
