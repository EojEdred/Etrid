//! Service implementation for ETH-PBC Collator

use futures::FutureExt;
use sc_client_api::{Backend, HeaderBackend};
use sc_consensus_asf::{import_queue as asf_import_queue, run_asf_worker, AsfWorkerParams};
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_runtime::traits::Header as HeaderT;
use std::{marker::PhantomData, sync::Arc, time::Duration};

use eth_pbc_runtime::{self, opaque::Block, RuntimeApi, AccountId};

use crate::p2p_network::{
    EthPbcP2PNetwork, P2PConfig, parse_bootnode, node_id_from_network_key, get_announce_address,
};

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

/// P2P configuration options for the collator
pub struct CollatorP2POptions {
    pub enable_p2p: bool,
    pub p2p_listen: String,
    pub p2p_announce: Option<String>,
    pub bootnodes: Vec<String>,
}

/// Start the collator node
pub async fn start_collator(
    config: Configuration,
    p2p_options: Option<CollatorP2POptions>,
) -> Result<TaskManager, ServiceError> {
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

    task_manager.spawn_handle().spawn(
        "state-root-submitter",
        None,
        submit_state_roots(client.clone()),
    );

    // Initialize and start DETR P2P network if enabled
    if let Some(p2p_opts) = p2p_options {
        if p2p_opts.enable_p2p {
            log::info!("🌐 Initializing DETR P2P network for ETH-PBC collator...");

            // Parse listen address
            let listen_address = p2p_opts.p2p_listen.parse()
                .map_err(|e| ServiceError::Other(format!("Invalid P2P listen address: {}", e)))?;

            // Parse announce address if provided, otherwise auto-detect
            let announce_address = if let Some(announce) = p2p_opts.p2p_announce {
                Some(announce.parse()
                    .map_err(|e| ServiceError::Other(format!("Invalid P2P announce address: {}", e)))?)
            } else {
                get_announce_address(listen_address)
            };

            // Parse bootstrap peers
            let mut bootstrap_peers = Vec::new();
            for bootnode in &p2p_opts.bootnodes {
                match parse_bootnode(bootnode) {
                    Ok(peer) => {
                        log::info!("  📌 Bootstrap peer: {:?} at {}", peer.id, peer.address);
                        bootstrap_peers.push(peer);
                    }
                    Err(e) => {
                        log::warn!("  ⚠️ Failed to parse bootnode '{}': {}", bootnode, e);
                    }
                }
            }

            // Generate node ID from network key
            let network_key = config.network.node_key.clone().into_keypair()
                .map_err(|e| ServiceError::Other(format!("Failed to get network key: {}", e)))?
                .public()
                .as_ref()
                .to_vec();

            let node_id = node_id_from_network_key(&network_key);

            log::info!("  🔑 P2P Node ID: {:?}", node_id.as_bytes()[..8].to_vec());
            log::info!("  🎧 Listen address: {}", listen_address);
            if let Some(announce) = announce_address {
                log::info!("  📢 Announce address: {}", announce);
            } else {
                log::info!("  📢 Announce address: will auto-detect");
            }

            // Create P2P configuration
            let p2p_config = P2PConfig {
                listen_address,
                announce_address,
                bootstrap_peers,
                node_id,
            };

            // Initialize P2P network
            let p2p_network = EthPbcP2PNetwork::new(p2p_config).await
                .map_err(|e| ServiceError::Other(format!("P2P initialization failed: {}", e)))?;

            // Start P2P network and all maintenance tasks
            // CRITICAL: This starts:
            // 1. Network listener for incoming connections
            // 2. DHT maintenance (bucket refresh, stale node removal)
            // 3. Periodic peer discovery (adaptive based on peer count)
            // 4. Automatic reconnection (handles transient network failures)
            p2p_network.start().await
                .map_err(|e| ServiceError::Other(format!("P2P start failed: {}", e)))?;

            log::info!("✅ DETR P2P network started successfully");
            log::info!("   All background maintenance tasks are ACTIVE");

            // Spawn P2P message handling task
            let p2p_network_clone = Arc::new(p2p_network);
            let client_clone = client.clone();

            task_manager.spawn_handle().spawn(
                "p2p-message-handler",
                None,
                handle_p2p_messages(p2p_network_clone.clone(), client_clone),
            );

            // Spawn periodic peer list updater
            task_manager.spawn_handle().spawn(
                "p2p-peer-list-updater",
                None,
                update_peer_list_periodically(p2p_network_clone.clone()),
            );

            // Spawn periodic network stats logger
            task_manager.spawn_handle().spawn(
                "p2p-stats-logger",
                None,
                log_network_stats_periodically(p2p_network_clone.clone()),
            );

            log::info!("📊 P2P background tasks spawned:");
            log::info!("   - Message handler: processing incoming P2P messages");
            log::info!("   - Peer list updater: maintaining connection list");
            log::info!("   - Stats logger: periodic network statistics");
        } else {
            log::info!("ℹ️ DETR P2P networking is disabled");
        }
    } else {
        log::info!("ℹ️ DETR P2P networking not configured");
    }

    Ok(task_manager)
}

async fn submit_state_roots(client: Arc<FullClient>) {
    log::info!("🔗 ETH-PBC: State root submitter task started");

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
                        "🔗 ETH-PBC: Block #{} produced with state root: {:?}",
                        best_number,
                        state_root
                    );

                    last_block_number = best_number;
                }
                Ok(None) => {
                    log::warn!("🔗 ETH-PBC: Header not found for block #{}", best_number);
                }
                Err(e) => {
                    log::error!("🔗 ETH-PBC: Error reading header for block #{}: {:?}", best_number, e);
                }
            }
        }
    }
}

/// Handle incoming P2P messages
async fn handle_p2p_messages(
    p2p_network: Arc<EthPbcP2PNetwork>,
    client: Arc<FullClient>,
) {
    log::info!("📨 P2P message handler started");

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let messages = p2p_network.receive_messages(10).await;

        for (peer_id, message) in messages {
            match message {
                detrp2p::Message::BlockAnnounce { block_number, block_hash, parent_hash, encoded_block } => {
                    log::info!(
                        "📦 Received block #{} from peer {:?} (hash: {:?})",
                        block_number,
                        peer_id,
                        block_hash
                    );
                    // In production: validate and import the block
                }
                detrp2p::Message::BlockRequest { request_id, by_number, by_hash } => {
                    log::debug!("📥 Block request {} from peer {:?}", request_id, peer_id);
                    // In production: fetch block and send BlockResponse
                }
                detrp2p::Message::StatusRequest { request_id } => {
                    log::debug!("📊 Status request {} from peer {:?}", request_id, peer_id);
                    // In production: send StatusResponse with current chain info
                }
                detrp2p::Message::Ping { nonce } => {
                    log::trace!("🏓 Ping {} from peer {:?}", nonce, peer_id);
                }
                detrp2p::Message::Pong { nonce } => {
                    log::trace!("🏓 Pong {} from peer {:?}", nonce, peer_id);
                }
                detrp2p::Message::Announce { peer } => {
                    log::debug!("📢 Peer announcement from {:?} at {}", peer.id, peer.address);
                }
                detrp2p::Message::FindNode { target } => {
                    log::trace!("🔍 FindNode for {:?} from peer {:?}", target, peer_id);
                }
                _ => {
                    log::trace!("📬 Other message from peer {:?}", peer_id);
                }
            }
        }
    }
}

/// Periodically update the peer list
async fn update_peer_list_periodically(p2p_network: Arc<EthPbcP2PNetwork>) {
    log::info!("🔄 Peer list updater started");

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        if let Err(e) = p2p_network.update_peer_list().await {
            log::warn!("⚠️ Failed to update peer list: {}", e);
        }
    }
}

/// Periodically log network statistics
async fn log_network_stats_periodically(p2p_network: Arc<EthPbcP2PNetwork>) {
    log::info!("📊 Network stats logger started");

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;

        let stats = p2p_network.get_stats().await;

        log::info!("📊 Network Statistics:");
        log::info!("   Connected peers: {}", stats.peer_count);
        log::info!("   Bootstrap peers: {}", stats.bootstrap_peer_count);
        log::info!("   Listen address: {}", stats.listen_address);
        log::info!("   Announce address: {}", stats.announce_address);
        log::info!("   Node ID: {:?}", stats.node_id.as_bytes()[..8].to_vec());
    }
}
