//! Service implementation for TRX-PBC Collator

use futures::{FutureExt, StreamExt};
use etrid_protocol::gadget_network_bridge::GadgetNetworkBridge;
use finality_gadget::NetworkBridge;
use sc_client_api::{Backend, BlockchainEvents, HeaderBackend};
use sc_consensus_asf::{import_queue as asf_import_queue, run_asf_worker, AsfWorkerParams};
use sc_consensus::import_queue::ImportQueue;
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_runtime::codec::Decode;
use sp_runtime::traits::{Get, Header as HeaderT, SaturatedConversion};
use std::{collections::HashSet, marker::PhantomData, net::{IpAddr, SocketAddr}, sync::Arc, sync::atomic::AtomicU64, time::Duration};

use trx_pbc_runtime::{self, opaque::Block, RuntimeApi, AccountId};

use crate::p2p_config::{P2PConfig, P2PNetworkService, peer_id_from_public_key, parse_bootstrap_peers};
use crate::p2p_bridge::{DetrP2PNetworkBridge, P2PBridge, PendingState};

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

/// Start the collator node with P2P networking
pub async fn start_collator(config: Configuration) -> Result<TaskManager, ServiceError> {
    start_collator_with_p2p(config, None).await
}

/// Start the collator node with optional P2P configuration
pub async fn start_collator_with_p2p(
    config: Configuration,
    p2p_config_override: Option<P2PConfig>,
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

    let import_queue_service = Arc::new(tokio::sync::Mutex::new(import_queue.service()));
    let pending_state = Arc::new(tokio::sync::Mutex::new(PendingState::default()));
    let request_counter = Arc::new(AtomicU64::new(1));

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

    // Initialize DETR P2P Network
    let p2p_config = match p2p_config_override {
        Some(p2p_config) => p2p_config,
        None => build_p2p_config(&config, &keystore_container)?,
    };

    if p2p_config.enabled {
            log::info!("🌐 Initializing DETR P2P Network for TRX-PBC Collator...");

            // Initialize P2P network with configuration
            let mut p2p_cfg = p2p_config;
            match p2p_cfg.initialize().await {
                Ok(p2p_network) => {
                    log::info!("✅ P2P network initialized successfully");

                    // Create P2P network service with bootstrap peers
                    let p2p_service = P2PNetworkService::new(p2p_network.clone(), p2p_cfg.bootstrap_peers.clone());

                    // Start P2P network service (includes start_all_maintenance)
                    if let Err(e) = p2p_service.start().await {
                        log::error!("❌ Failed to start P2P network service: {}", e);
                    } else {
                        log::info!("✅ P2P network service started with all maintenance tasks");

                        // Bridge libp2p peer discovery into DETR P2P to keep overlays segmented but aligned.
                        let libp2p_network = network.clone();
                        let detrp2p_network = p2p_network.clone();
                        let detr_listen_addr = p2p_cfg.announce_address.unwrap_or(p2p_cfg.bind_address);
                        let detr_p2p_port = p2p_cfg.bind_address.port();
                        task_manager.spawn_handle().spawn(
                            "detrp2p-libp2p-bridge",
                            None,
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

                                                let peer_addr = detrp2p::PeerAddr {
                                                    id: detrp2p::PeerId::from_socket_addr(socket),
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

                        // Initialize finality gadget bridge components
                        let gadget_bridge = Arc::new(tokio::sync::Mutex::new(GadgetNetworkBridge::new()));

                        let validator_id = {
                            if config.role.is_authority() {
                                use sp_core::crypto::{AccountId32, KeyTypeId};
                                const ASF_KEY_TYPE: KeyTypeId = KeyTypeId(*b"asfk");

                                let keystore = keystore_container.keystore();
                                let asf_keys = keystore.sr25519_public_keys(ASF_KEY_TYPE);

                                match asf_keys.first() {
                                    Some(public_key) => {
                                        let account_id = AccountId32::from(public_key.clone());
                                        log::info!(
                                            "🔑 ASF Finality Gadget using validator key: {}",
                                            hex::encode(public_key.as_ref() as &[u8])
                                        );
                                        finality_gadget::ValidatorId(account_id)
                                    }
                                    None => {
                                        log::warn!(
                                            "⚠️ No ASF key found in keystore; running as observer"
                                        );
                                        finality_gadget::ValidatorId(AccountId32::new([0xFFu8; 32]))
                                    }
                                }
                            } else {
                                finality_gadget::ValidatorId(sp_core::crypto::AccountId32::new([0xFFu8; 32]))
                            }
                        };

                        let max_validators = trx_pbc_runtime::asf_config::AsfMaxCommitteeSize::get();
                        let finality_keystore = keystore_container.keystore();
                        let network_bridge = Arc::new(DetrP2PNetworkBridge::new(
                            p2p_network.clone(),
                            gadget_bridge.clone(),
                        ));
                        let finality_gadget = Arc::new(tokio::sync::Mutex::new(
                            finality_gadget::FinalityGadget::new(
                                validator_id.clone(),
                                max_validators,
                                finality_keystore,
                                network_bridge.clone(),
                            ),
                        ));

                        // Finality gossip + timeout loop
                        let gadget_loop = finality_gadget.clone();
                        let network_loop = network_bridge.clone();
                        task_manager.spawn_handle().spawn(
                            "asf-finality-gadget-loop",
                            None,
                            async move {
                                let mut tick = tokio::time::interval(Duration::from_secs(2));
                                loop {
                                    tick.tick().await;
                                    let (votes, certs) = {
                                        let mut gadget = gadget_loop.lock().await;
                                        gadget.get_ready_gossip_messages()
                                    };

                                    for vote in votes {
                                        let _ = network_loop.broadcast_vote(vote).await;
                                    }
                                    for cert in certs {
                                        let _ = network_loop.broadcast_certificate(cert).await;
                                    }

                                    let mut gadget = gadget_loop.lock().await;
                                    let _ = gadget.handle_timeout().await;
                                }
                            },
                        );

                        // Block import -> finality voting
                        let block_import_gadget = finality_gadget.clone();
                        let import_notifications = client.import_notification_stream();
                        task_manager.spawn_handle().spawn(
                            "asf-block-import-finality",
                            None,
                            async move {
                                let mut stream = import_notifications;
                                while let Some(notification) = stream.next().await {
                                    let block_number = *notification.header.number();
                                    let block_hash_bytes: [u8; 32] = notification.hash.into();
                                    let block_hash = finality_gadget::BlockHash::from_bytes(block_hash_bytes);
                                    let view = view_from_header(&notification.header);
                                    let block_number_u32: u32 = block_number.saturated_into();

                                    match tokio::time::timeout(
                                        Duration::from_secs(3),
                                        block_import_gadget.lock(),
                                    )
                                    .await
                                    {
                                        Ok(mut gadget) => {
                                            let _ = gadget
                                                .propose_block(block_hash, block_number_u32, view)
                                                .await;
                                        }
                                        Err(_) => {
                                            log::warn!(
                                                "⚠️ Finality gadget lock timeout for block #{}",
                                                block_number
                                            );
                                        }
                                    }
                                }
                            },
                        );

                        // Create and start P2P bridge for block sync + finality
                        let p2p_bridge = P2PBridge::new(
                            p2p_network.clone(),
                            client.clone(),
                            gadget_bridge.clone(),
                            finality_gadget.clone(),
                            import_queue_service.clone(),
                            pending_state.clone(),
                            request_counter.clone(),
                        );
                        p2p_bridge.start().await;
                        log::info!("✅ P2P bridge started for block announcements and sync");

                        // Keep P2P service alive by spawning a task
                        task_manager.spawn_handle().spawn(
                            "p2p-network-keeper",
                            None,
                            async move {
                                // Hold references to keep services alive
                                let _service = p2p_service;
                                let _bridge = p2p_bridge;

                                // Wait indefinitely
                                loop {
                                    tokio::time::sleep(Duration::from_secs(3600)).await;
                                }
                            },
                        );
                    }
                }
                Err(e) => {
                    log::error!("❌ Failed to initialize P2P network: {}", e);
                    log::warn!("⚠️ Continuing without P2P networking");
                }
            }
        } else {
            log::info!("ℹ️ P2P networking is disabled");
        }
    } else {
        log::info!("ℹ️ No P2P configuration provided - running without DETR P2P");
    }


    // ═══════════════════════════════════════════════════════════════════════════
    // RPC SERVER INITIALIZATION - CRITICAL FIX
    // ═══════════════════════════════════════════════════════════════════════════
    log::info!("🔧 Initializing RPC server for TRX-PBC Collator...");

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

async fn submit_state_roots(client: Arc<FullClient>) {
    log::info!("🔗 TRX-PBC: State root submitter task started");

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
                        "🔗 TRX-PBC: Block #{} produced with state root: {:?}",
                        best_number,
                        state_root
                    );

                    last_block_number = best_number;
                }
                Ok(None) => {
                    log::warn!("🔗 TRX-PBC: Header not found for block #{}", best_number);
                }
                Err(e) => {
                    log::error!("🔗 TRX-PBC: Error reading header for block #{}: {:?}", best_number, e);
                }
            }
        }
    }
}


fn build_p2p_config(
    config: &Configuration,
    keystore_container: &sc_service::KeystoreContainer,
) -> Result<P2PConfig, ServiceError> {
    let local_node_id = derive_detrp2p_peer_id(
        keystore_container,
        config.chain_spec.id(),
        &config.network.node_name,
    );

    let bind_address_str = std::env::var("DETR_P2P_BIND_ADDRESS")
        .or_else(|_| std::env::var("DETR_P2P_BIND"))
        .unwrap_or_else(|_| "0.0.0.0:30333".to_string());
    let bind_address = bind_address_str
        .parse()
        .map_err(|e| ServiceError::Other(format!(
            "Invalid DETR P2P bind address '{}': {}",
            bind_address_str,
            e
        )))?;

    let bootstrap_peers = bootstrap_peers_from_env();

    let mut p2p_config = P2PConfig::new(local_node_id, bind_address, bootstrap_peers);
    p2p_config.enabled = std::env::var("DETR_P2P_ENABLE")
        .map(|v| v != "0")
        .unwrap_or(true);

    if let Ok(announce_str) = std::env::var("DETR_P2P_ANNOUNCE") {
        match announce_str.parse::<SocketAddr>() {
            Ok(addr) => {
                p2p_config = p2p_config.with_announce_address(addr);
            }
            Err(e) => {
                log::warn!("Invalid DETR_P2P_ANNOUNCE '{}': {}", announce_str, e);
            }
        }
    }

    Ok(p2p_config)
}

fn bootstrap_peers_from_env() -> Vec<detrp2p::PeerAddr> {
    let env = std::env::var("DETR_P2P_BOOTSTRAP")
        .or_else(|_| std::env::var("DETR_P2P_BOOTSTRAP_PEERS"))
        .ok();

    match env {
        Some(value) => match parse_bootstrap_peers(&value) {
            Ok(peers) => peers,
            Err(e) => {
                log::warn!("Failed to parse DETR P2P bootstrap peers '{}': {}", value, e);
                Vec::new()
            }
        },
        None => Vec::new(),
    }
}

fn derive_detrp2p_peer_id(
    keystore_container: &sc_service::KeystoreContainer,
    chain_id: &str,
    node_name: &str,
) -> detrp2p::PeerId {
    use sp_core::crypto::KeyTypeId;

    const ASF_KEY_TYPE: KeyTypeId = KeyTypeId(*b"asfk");
    let keystore = keystore_container.keystore();
    if let Some(public_key) = keystore.sr25519_public_keys(ASF_KEY_TYPE).first() {
        return peer_id_from_public_key(public_key.as_ref());
    }

    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| node_name.to_string());
    let seed = format!("detrp2p-observer:{}:{}:{}", chain_id, node_name, hostname);
    detrp2p::PeerId::new(sp_core::hashing::blake2_256(seed.as_bytes()))
}


fn view_from_header<H: HeaderT>(header: &H) -> finality_gadget::View {
    finality_gadget::View((*header.number()).saturated_into())
}

