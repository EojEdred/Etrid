//! Service implementation for BTC-PBC Collator

use futures::FutureExt;
use sc_client_api::{Backend, HeaderBackend};
use sc_consensus_asf::{import_queue as asf_import_queue, run_asf_worker, AsfWorkerParams};
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_runtime::traits::Header as HeaderT;
use std::{marker::PhantomData, sync::Arc, time::Duration};

use bnb_pbc_runtime::{self, opaque::Block, RuntimeApi, AccountId};
use crate::p2p_network::{P2PConfig, P2PNetworkManager, parse_bootstrap_peers, generate_node_id_from_validator_key};

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
pub async fn start_collator(config: Configuration) -> Result<TaskManager, ServiceError> {
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

    // =====================================================================
    // DETR P2P Network Initialization
    // =====================================================================

    log::info!("🌐 Initializing DETR P2P Network for BNB PBC Collator...");

    // Generate deterministic node ID from keystore
    let node_id = if let Some(local_key) = keystore_container.keystore().sr25519_public_keys(sp_core::crypto::key_types::AURA).first() {
        generate_node_id_from_validator_key(local_key.as_ref())
    } else {
        log::warn!("⚠️ No validator key found, using random node ID");
        detrp2p::PeerId::new(rand::random())
    };

    // Parse P2P configuration from CLI/environment
    let p2p_bind_addr = std::env::var("DETR_P2P_BIND")
        .unwrap_or_else(|_| "0.0.0.0:30333".to_string())
        .parse()
        .map_err(|e| ServiceError::Other(format!("Invalid P2P bind address: {}", e)))?;

    let p2p_announce_addr = std::env::var("DETR_P2P_ANNOUNCE")
        .ok()
        .and_then(|s| s.parse().ok());

    let bootstrap_peers = std::env::var("DETR_P2P_BOOTSTRAP")
        .ok()
        .and_then(|s| parse_bootstrap_peers(&s).ok())
        .unwrap_or_default();

    // Create P2P configuration
    let mut p2p_config = P2PConfig::new(node_id, p2p_bind_addr, bootstrap_peers)
        .from_env();

    if let Some(announce) = p2p_announce_addr {
        p2p_config = p2p_config.with_announce_address(announce);
    }

    // Initialize P2P network manager
    let p2p_manager = Arc::new(
        P2PNetworkManager::new(p2p_config).await
            .map_err(|e| ServiceError::Other(format!("Failed to initialize P2P network: {}", e)))?
    );

    log::info!("✅ DETR P2P Network initialized");
    log::info!("   Local Node ID: {:?}", p2p_manager.local_node_id());
    log::info!("   Local Peer Info: {:?}", p2p_manager.local_peer_info());

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

    // =====================================================================
    // Start DETR P2P Network
    // =====================================================================

    log::info!("🚀 Starting DETR P2P Network...");

    // Start the P2P network (includes all maintenance tasks)
    let p2p_manager_clone = p2p_manager.clone();
    task_manager.spawn_essential_handle().spawn_blocking(
        "detr-p2p-network",
        Some("networking"),
        async move {
            match p2p_manager_clone.start().await {
                Ok(_) => {
                    log::info!("✅ DETR P2P Network started successfully");

                    // Keep the network running
                    loop {
                        tokio::time::sleep(Duration::from_secs(60)).await;

                        if !p2p_manager_clone.is_running().await {
                            log::error!("❌ DETR P2P Network stopped unexpectedly");
                            break;
                        }

                        // Log network stats periodically
                        let stats = p2p_manager_clone.get_stats().await;
                        log::debug!("📊 DETR P2P Network stats: {:?}", stats);
                    }
                }
                Err(e) => {
                    log::error!("❌ Failed to start DETR P2P Network: {}", e);
                }
            }
        },
    );

    log::info!("✅ BNB PBC Collator service started with DETR P2P integration");

    Ok(task_manager)
}

async fn submit_state_roots(client: Arc<FullClient>) {
    log::info!("🔗 BTC-PBC: State root submitter task started");

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
                        "🔗 BTC-PBC: Block #{} produced with state root: {:?}",
                        best_number,
                        state_root
                    );

                    last_block_number = best_number;
                }
                Ok(None) => {
                    log::warn!("🔗 BTC-PBC: Header not found for block #{}", best_number);
                }
                Err(e) => {
                    log::error!("🔗 BTC-PBC: Error reading header for block #{}: {:?}", best_number, e);
                }
            }
        }
    }
}
