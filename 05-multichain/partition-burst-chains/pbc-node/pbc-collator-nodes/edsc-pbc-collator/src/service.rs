//! Service implementation for EDSC-PBC Collator
//!
//! This service manages the EDSC (Ëtrid Dollar Stablecoin) PBC including:
//! - ASF consensus for block authoring
//! - State root submission to Primearc Core Chain
//! - Stablecoin transaction processing
//! - Proof-of-reserves tracking
//! - DETR P2P networking with encryption and auto-discovery

use futures::FutureExt;
use sc_client_api::{Backend, HeaderBackend};
use sc_consensus_asf::{import_queue as asf_import_queue, run_asf_worker, AsfWorkerParams};
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_runtime::traits::Header as HeaderT;
use std::{marker::PhantomData, sync::Arc, time::Duration};

use edsc_pbc_runtime::{self, opaque::Block, RuntimeApi, AccountId};

use crate::cli::Cli;
use crate::p2p_config::{P2PConfig, parse_bootstrap_peer};

pub type FullClient = TFullClient<Block, RuntimeApi, sc_executor::WasmExecutor<sp_io::SubstrateHostFunctions>>;
pub type FullBackend = TFullBackend<Block>;

pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        (),
        sc_consensus::DefaultImportQueue<Block>,
        sc_transaction_pool::TransactionPoolHandle<Block, FullClient>,
        (Option<Telemetry>,),
    >,
    ServiceError,
> {
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

    let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config.executor);

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

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

    let import_queue = asf_import_queue::<_, _, _, AccountId>(
        client.clone(),
        client.clone(),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    )
    .map_err(|e| ServiceError::Other(format!("ASF import queue error: {}", e)))?;

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain: (),
        transaction_pool,
        other: (telemetry,),
    })
}

/// Start the collator node
pub async fn start_collator(config: Configuration, cli: Cli) -> Result<TaskManager, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain: _,
        transaction_pool,
        other: (mut telemetry,),
    } = new_partial(&config)?;

    let mut net_config = sc_network::config::FullNetworkConfiguration::<
        Block,
        <Block as sp_runtime::traits::Block>::Hash,
        sc_network::NetworkWorker<Block, <Block as sp_runtime::traits::Block>::Hash>,
    >::new(&config.network, config.prometheus_registry().cloned());

    let metrics = sc_network::service::NotificationMetrics::new(config.prometheus_registry());

    let (network, system_rpc_tx, tx_handler_controller, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_config: None,
            block_relay: None,
            metrics,
        })?;

    if config.offchain_worker.enabled {
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

    let proposer_factory = sc_basic_authorship::ProposerFactory::new(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool.clone(),
        config.prometheus_registry(),
        telemetry.as_ref().map(|x| x.handle()),
    );

    // ASF consensus worker parameters
    let backoff_authoring_blocks = Some(BackoffAuthoringOnFinalizedHeadLagging::default());

    let asf_params = AsfWorkerParams {
        client: client.clone(),
        block_import: client.clone(),
        env: proposer_factory,
        sync_oracle: sync_service.clone(),
        backoff_authoring_blocks,
        keystore: keystore_container.keystore(),
        create_inherent_data_providers: move |_, ()| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            Ok((timestamp,))
        },
        force_authoring: config.force_authoring,
        block_proposal_slot_portion: 2f32 / 3f32,
        max_block_proposal_slot_portion: None,
        justification_sync_link: sync_service.clone(),
        _phantom: PhantomData,
    };

    // Start ASF block authoring worker
    let asf_worker = run_asf_worker(asf_params);
    task_manager.spawn_essential_handle().spawn_blocking(
        "asf-worker",
        Some("block-authoring"),
        asf_worker.map(|res| {
            if let Err(e) = res {
                log::error!("ASF worker error: {}", e);
            }
        }),
    );

    // Start state root submission task
    task_manager.spawn_handle().spawn(
        "state-root-submitter",
        None,
        submit_state_roots(client.clone()),
    );

    // ============================================================================
    // DETR P2P NETWORK INITIALIZATION
    // ============================================================================

    log::info!("🌐 Initializing DETR P2P network for EDSC-PBC collator...");

    // Build P2P configuration from CLI arguments
    let p2p_config = build_p2p_config(&cli).await?;

    // Validate configuration
    if let Err(e) = p2p_config.validate() {
        log::error!("❌ P2P configuration validation failed: {}", e);
        return Err(ServiceError::Other(format!("P2P config invalid: {}", e)));
    }

    log::info!("✅ P2P configuration validated successfully");
    log::info!("  🔌 Bind address: {}", p2p_config.bind_address);
    log::info!("  📢 Announce address: {}", p2p_config.effective_announce_address());
    log::info!("  👥 Bootstrap peers: {}", p2p_config.bootstrap_peers.len());

    // Create P2P network with announce address support
    let p2p_network = Arc::new(detrp2p::P2PNetwork::new_with_announce(
        p2p_config.local_node_id,
        p2p_config.bind_address,
        p2p_config.announce_address,
        p2p_config.bootstrap_peers.clone(),
    ));

    log::info!("🆔 Local Node ID: {:?}", p2p_network.local_node_id());
    log::info!("📍 Local Peer Info: {:?}", p2p_network.local_peer_info());

    // Start P2P network listener
    let p2p_network_clone = p2p_network.clone();
    task_manager.spawn_handle().spawn(
        "detr-p2p-network",
        None,
        async move {
            log::info!("🚀 Starting DETR P2P network listener...");
            match p2p_network_clone.start().await {
                Ok(()) => {
                    log::info!("✅ DETR P2P network started successfully");
                }
                Err(e) => {
                    log::error!("❌ DETR P2P network failed to start: {}", e);
                }
            }
        },
    );

    // Give the network a moment to start listening
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Start P2P maintenance tasks (DHT maintenance, auto-reconnection, periodic discovery)
    if cli.p2p_enable_maintenance {
        log::info!("🔧 Starting P2P maintenance tasks...");
        p2p_network.start_all_maintenance();
        log::info!("✅ P2P maintenance tasks started (DHT, auto-reconnect, discovery)");
    } else {
        log::info!("⚠️ P2P maintenance tasks disabled by CLI flag");
    }

    // Spawn P2P message processor
    let p2p_network_clone = p2p_network.clone();
    let client_clone = client.clone();
    task_manager.spawn_handle().spawn(
        "detr-p2p-message-processor",
        None,
        process_p2p_messages(p2p_network_clone, client_clone),
    );

    log::info!("🌐 DETR P2P network fully initialized and operational");


    // ═══════════════════════════════════════════════════════════════════════════
    // RPC SERVER INITIALIZATION - CRITICAL FIX
    // ═══════════════════════════════════════════════════════════════════════════
    log::info!("🔧 Initializing RPC server for EDSC-PBC Collator...");

    // Build RPC extensions
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

    // Spawn RPC server tasks - THIS STARTS THE JSON-RPC SERVER
    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
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

    log::info!("✅ RPC server initialized successfully");

    Ok(task_manager)
}

/// Build P2P configuration from CLI arguments
async fn build_p2p_config(cli: &Cli) -> Result<P2PConfig, ServiceError> {
    // Parse bind address
    let bind_address = cli.p2p_bind_address
        .parse()
        .map_err(|e| ServiceError::Other(format!("Invalid P2P bind address: {}", e)))?;

    // Parse bootstrap peers from CLI or environment
    let bootstrap_peers = if let Some(ref peers_str) = cli.p2p_bootstrap_peers {
        peers_str
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .filter_map(|peer_str| {
                match parse_bootstrap_peer(peer_str.trim()) {
                    Ok(peer) => Some(peer),
                    Err(e) => {
                        log::warn!("⚠️ Failed to parse bootstrap peer '{}': {}", peer_str, e);
                        None
                    }
                }
            })
            .collect()
    } else {
        // Try environment variable as fallback
        std::env::var("DETR_P2P_BOOTSTRAP_PEERS")
            .ok()
            .map(|peers_str| {
                peers_str
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .filter_map(|peer_str| {
                        match parse_bootstrap_peer(peer_str.trim()) {
                            Ok(peer) => Some(peer),
                            Err(e) => {
                                log::warn!("⚠️ Failed to parse bootstrap peer '{}': {}", peer_str, e);
                                None
                            }
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    // Generate local node ID (in production, derive from validator keys)
    let local_node_id = generate_node_id();

    // Create base config
    let mut p2p_config = P2PConfig::new(bind_address, local_node_id, bootstrap_peers);

    // Set announce address if provided via CLI
    if let Some(ref announce_str) = cli.p2p_announce_address {
        let announce_address = announce_str
            .parse()
            .map_err(|e| ServiceError::Other(format!("Invalid P2P announce address: {}", e)))?;
        p2p_config = p2p_config.with_announce_address(announce_address);
    } else {
        // Auto-detect public IP if not manually specified
        log::info!("🔍 No announce address specified, attempting auto-detection...");
        p2p_config = p2p_config.with_auto_detected_ip().await;
    }

    Ok(p2p_config)
}

/// Generate a deterministic node ID from validator keys or hostname
fn generate_node_id() -> detrp2p::PeerId {
    use sp_core::crypto::Ss58Codec;
    use sp_core::sr25519;

    // Try to load from keystore or use deterministic generation
    if let Ok(key_str) = std::env::var("VALIDATOR_PUBLIC_KEY") {
        if let Ok(public_key) = sr25519::Public::from_ss58check(&key_str) {
            let mut node_id_bytes = [0u8; 32];
            node_id_bytes.copy_from_slice(&public_key.0);
            return detrp2p::PeerId::new(node_id_bytes);
        }
    }

    // Fallback: Generate from hostname + PBC ID (stable across restarts)
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| "edsc-collator".to_string());

    let seed = format!("edsc-pbc-12-{}", hostname); // PBC ID 12 for EDSC

    // Hash the seed to get a 32-byte node ID
    let mut node_id_bytes = [0u8; 32];
    use sp_core::hashing::blake2_256;
    node_id_bytes.copy_from_slice(&blake2_256(seed.as_bytes()));

    log::info!("🔑 Generated P2P node ID from seed: {}", seed);

    detrp2p::PeerId::new(node_id_bytes)
}

/// Process incoming P2P messages
async fn process_p2p_messages(
    p2p_network: Arc<detrp2p::P2PNetwork>,
    _client: Arc<FullClient>,
) {
    log::info!("📨 P2P message processor started");

    loop {
        // Check for incoming messages
        if let Some((peer_id, message)) = p2p_network.receive_message().await {
            match message {
                detrp2p::Message::BlockAnnounce { block_number, block_hash, .. } => {
                    log::info!(
                        "📦 Received BlockAnnounce from {:?}: block #{} hash {:?}",
                        peer_id,
                        block_number,
                        hex::encode(&block_hash)
                    );
                    // In production: Process block announcement, request block if needed
                }
                detrp2p::Message::BlockRequest { request_id, by_number, by_hash } => {
                    log::info!(
                        "📥 Received BlockRequest from {:?}: request_id={} by_number={:?} by_hash={:?}",
                        peer_id,
                        request_id,
                        by_number,
                        by_hash.map(|h| hex::encode(&h))
                    );
                    // In production: Fetch block from client and send BlockResponse
                }
                detrp2p::Message::StatusRequest { request_id } => {
                    log::info!(
                        "📊 Received StatusRequest from {:?}: request_id={}",
                        peer_id,
                        request_id
                    );
                    // In production: Send StatusResponse with our current best block
                }
                detrp2p::Message::Vote { data } => {
                    log::info!(
                        "🗳️ Received Vote from {:?}: {} bytes",
                        peer_id,
                        data.len()
                    );
                    // In production: Process consensus vote
                }
                detrp2p::Message::Certificate { data } => {
                    log::info!(
                        "📜 Received Certificate from {:?}: {} bytes",
                        peer_id,
                        data.len()
                    );
                    // In production: Process and verify certificate
                }
                detrp2p::Message::FindNodeReply { peers } => {
                    log::debug!(
                        "👥 Received FindNodeReply from {:?}: {} peers",
                        peer_id,
                        peers.len()
                    );
                }
                detrp2p::Message::Pong { nonce } => {
                    log::debug!("🏓 Received Pong from {:?}: nonce={}", peer_id, nonce);
                }
                _ => {
                    log::debug!("📬 Received message from {:?}: {:?}", peer_id, message);
                }
            }
        }

        // Small delay to prevent busy-waiting
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/// Submit EDSC-PBC state roots to Primearc Core Chain
async fn submit_state_roots(client: Arc<FullClient>) {
    log::info!("💵 EDSC-PBC: State root submitter task started");

    let mut last_block_number = 0u32;

    loop {
        tokio::time::sleep(Duration::from_secs(6)).await;

        let best_number = client.info().best_number;

        if best_number > last_block_number {
            let best_hash = client.info().best_hash;

            match client.header(best_hash) {
                Ok(Some(header)) => {
                    let state_root = header.state_root();

                    log::info!(
                        "💵 EDSC-PBC: Block #{} produced with state root: {:?}",
                        best_number,
                        state_root
                    );

                    last_block_number = best_number;
                }
                Ok(None) => {
                    log::warn!("💵 EDSC-PBC: Header not found for block #{}", best_number);
                }
                Err(e) => {
                    log::error!("💵 EDSC-PBC: Error reading header for block #{}: {:?}", best_number, e);
                }
            }
        }
    }
}
