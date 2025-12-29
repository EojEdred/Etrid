//! Service implementation for XRP-PBC Collator

use async_trait::async_trait;
use futures::{FutureExt, StreamExt};
use etrid_protocol::gadget_network_bridge::{
    CertificateData,
    ConsensusBridgeMessage,
    GadgetNetworkBridge,
    VoteData,
};
use finality_gadget::{Certificate as FinalityCertificate, NetworkBridge, Vote as FinalityVote};
use sc_client_api::{Backend, BlockchainEvents, HeaderBackend};
use sc_consensus_asf::{import_queue as asf_import_queue, run_asf_worker, AsfWorkerParams};
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::codec::Decode;
use sp_runtime::traits::{Get, Header as HeaderT, SaturatedConversion};
use std::{collections::HashSet, marker::PhantomData, net::{IpAddr, SocketAddr}, sync::Arc, time::Duration};

use xrp_pbc_runtime::{self, opaque::Block, RuntimeApi, AccountId};

use crate::p2p_config::{P2PConfig, P2PNetworkService, parse_bootstrap_peers, peer_id_from_public_key};

pub type FullClient = TFullClient<Block, RuntimeApi, sc_executor::WasmExecutor<sp_io::SubstrateHostFunctions>>;
pub type FullBackend = TFullBackend<Block>;

/// NetworkBridge implementation using DETR P2P for ASF finality gossip
pub struct DetrP2PNetworkBridge {
    p2p_network: Arc<detrp2p::P2PNetwork>,
    gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
}

impl DetrP2PNetworkBridge {
    pub fn new(
        p2p_network: Arc<detrp2p::P2PNetwork>,
        gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
    ) -> Self {
        Self {
            p2p_network,
            gadget_bridge,
        }
    }

    fn convert_vote_to_bridge(vote: &FinalityVote) -> VoteData {
        VoteData {
            validator_id: vote.validator_id.0.clone().into(),
            view: vote.view.0,
            block_hash: {
                let mut hash = [0u8; 32];
                hash.copy_from_slice(vote.block_hash.as_bytes());
                hash
            },
            signature: vote.signature.clone(),
        }
    }

    fn convert_certificate_to_bridge(cert: &FinalityCertificate) -> CertificateData {
        let signatures: Vec<([u8; 32], Vec<u8>)> = cert
            .signatures
            .iter()
            .map(|(validator_id, sig)| {
                let bytes: [u8; 32] = validator_id.0.clone().into();
                (bytes, sig.clone())
            })
            .collect();

        CertificateData {
            view: cert.view.0,
            block_hash: {
                let mut hash = [0u8; 32];
                hash.copy_from_slice(cert.block_hash.as_bytes());
                hash
            },
            block_number: cert.block_number,
            signatures,
        }
    }
}

#[async_trait]
impl NetworkBridge for DetrP2PNetworkBridge {
    async fn broadcast_vote(&self, vote: FinalityVote) -> Result<(), String> {
        let vote_data = Self::convert_vote_to_bridge(&vote);

        let bridge = self.gadget_bridge.lock().await;
        bridge
            .send_vote(vote_data.clone())
            .await
            .map_err(|e| format!("Failed to queue vote: {:?}", e))?;
        let messages = bridge.get_outbound_messages().await;
        drop(bridge);

        for (msg, _priority) in messages {
            if let ConsensusBridgeMessage::Vote(vote_data) = msg {
                let payload = bincode::serialize(&vote_data)
                    .map_err(|e| format!("Failed to serialize vote: {:?}", e))?;
                let p2p_msg = detrp2p::Message::Vote { data: payload };
                self.p2p_network
                    .broadcast(p2p_msg)
                    .await
                    .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;
            }
        }

        Ok(())
    }

    async fn broadcast_certificate(&self, cert: FinalityCertificate) -> Result<(), String> {
        let cert_data = Self::convert_certificate_to_bridge(&cert);

        let bridge = self.gadget_bridge.lock().await;
        bridge
            .send_certificate(cert_data.clone())
            .await
            .map_err(|e| format!("Failed to queue certificate: {:?}", e))?;
        let messages = bridge.get_outbound_messages().await;
        drop(bridge);

        for (msg, _priority) in messages {
            if let ConsensusBridgeMessage::Certificate(cert_data) = msg {
                let payload = bincode::serialize(&cert_data)
                    .map_err(|e| format!("Failed to serialize certificate: {:?}", e))?;
                let p2p_msg = detrp2p::Message::Certificate { data: payload };
                self.p2p_network
                    .broadcast(p2p_msg)
                    .await
                    .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;
            }
        }

        Ok(())
    }

    async fn get_connected_peers(&self) -> Vec<String> {
        let peers = self.p2p_network.get_connected_peers().await;
        peers
            .into_iter()
            .map(|peer_id| hex::encode(peer_id.as_bytes()))
            .collect()
    }
}

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
pub async fn start_collator(
    config: Configuration,
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

    // Initialize DETR P2P Network for finality gossip
    let mut p2p_config = build_p2p_config(&config, &keystore_container)?;

    if p2p_config.enabled {
        if let Ok(p2p_network) = p2p_config.initialize().await {
            let p2p_service = P2PNetworkService::new(p2p_network.clone(), p2p_config.bootstrap_peers.clone());
            if let Err(e) = p2p_service.start().await {
                log::warn!("⚠️ Failed to start DETR P2P network: {}", e);
            } else {
                // Bridge libp2p peer discovery into DETR P2P to keep overlays segmented but aligned.
                let libp2p_network = network.clone();
                let detrp2p_network = p2p_network.clone();
                let detr_listen_addr = p2p_config.announce_address.unwrap_or(p2p_config.bind_address);
                let detr_p2p_port = p2p_config.bind_address.port();
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
                            log::warn!("⚠️ No ASF key found in keystore; running as observer");
                            finality_gadget::ValidatorId(AccountId32::new([0xFFu8; 32]))
                        }
                    }
                } else {
                    finality_gadget::ValidatorId(sp_core::crypto::AccountId32::new([0xFFu8; 32]))
                }
            };

            let max_validators = xrp_pbc_runtime::asf_config::AsfMaxCommitteeSize::get();
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
                                log::warn!("⚠️ Finality gadget lock timeout for block #{}", block_number);
                            }
                        }
                    }
                },
            );

            // Spawn P2P message processor for finality gossip
            let p2p_network_clone = p2p_network.clone();
            let client_clone = client.clone();
            let gadget_bridge_clone = gadget_bridge.clone();
            let finality_gadget_clone = finality_gadget.clone();
            task_manager.spawn_handle().spawn(
                "detr-p2p-message-processor",
                None,
                process_p2p_messages(
                    p2p_network_clone,
                    client_clone,
                    gadget_bridge_clone,
                    finality_gadget_clone,
                ),
            );

            // Keep P2P service alive
            task_manager.spawn_handle().spawn(
                "detr-p2p-keeper",
                None,
                async move {
                    let _service = p2p_service;
                    loop {
                        tokio::time::sleep(Duration::from_secs(3600)).await;
                    }
                },
            );
        }
    } else {
        log::warn!("⚠️ Failed to initialize DETR P2P network");
    }

    log::info!("XRP-PBC Collator initialization complete");


    // ═══════════════════════════════════════════════════════════════════════════
    // RPC SERVER INITIALIZATION - CRITICAL FIX
    // ═══════════════════════════════════════════════════════════════════════════
    log::info!("🔧 Initializing RPC server for XRP-PBC Collator...");

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
    log::info!("🔗 XRP-PBC: State root submitter task started");

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
                        "🔗 XRP-PBC: Block #{} produced with state root: {:?}",
                        best_number,
                        state_root
                    );

                    last_block_number = best_number;
                }
                Ok(None) => {
                    log::warn!("🔗 XRP-PBC: Header not found for block #{}", best_number);
                }
                Err(e) => {
                    log::error!("🔗 XRP-PBC: Error reading header for block #{}: {:?}", best_number, e);
                }
            }
        }
    }
}

async fn process_p2p_messages(
    p2p_network: Arc<detrp2p::P2PNetwork>,
    client: Arc<FullClient>,
    gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
    finality_gadget: Arc<tokio::sync::Mutex<finality_gadget::FinalityGadget>>,
) {
    log::info!("📨 XRP-PBC: P2P message processor started");

    loop {
        if let Some((peer_id, message)) = p2p_network.receive_message().await {
            match message {
                detrp2p::Message::Vote { data } => {
                    match bincode::deserialize::<VoteData>(&data) {
                        Ok(vote_data) => {
                            let bridge = gadget_bridge.lock().await;
                            if let Err(e) = bridge.on_vote_received(vote_data.clone()).await {
                                log::warn!("Failed to route vote: {:?}", e);
                            }
                            drop(bridge);

                            let finality_vote = convert_vote_from_bridge(vote_data);
                            let vote_block_hash = H256::from_slice(finality_vote.block_hash.as_bytes());
                            let vote_block_number: u32 = match client.header(vote_block_hash) {
                                Ok(Some(header)) => (*header.number()).saturated_into(),
                                Ok(None) => {
                                    log::warn!(
                                        "⚠️ Vote for unknown block {:?}, skipping",
                                        vote_block_hash
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

                            let mut gadget = finality_gadget.lock().await;
                            if let Err(e) = gadget.handle_vote(finality_vote, vote_block_number).await {
                                log::warn!("❌ Vote rejected by finality gadget: {:?}", e);
                            }
                        }
                        Err(e) => {
                            log::warn!("Failed to deserialize vote from {:?}: {:?}", peer_id, e);
                        }
                    }
                }
                detrp2p::Message::Certificate { data } => {
                    match bincode::deserialize::<CertificateData>(&data) {
                        Ok(cert_data) => {
                            let bridge = gadget_bridge.lock().await;
                            if let Err(e) = bridge.on_certificate_received(cert_data.clone()).await {
                                log::warn!("Failed to route certificate: {:?}", e);
                            }
                            drop(bridge);

                            let finality_cert = convert_certificate_from_bridge(cert_data);
                            let mut gadget = finality_gadget.lock().await;
                            if let Err(e) = gadget.handle_certificate(finality_cert).await {
                                log::warn!("❌ Certificate rejected by finality gadget: {:?}", e);
                            }
                        }
                        Err(e) => {
                            log::warn!(
                                "Failed to deserialize certificate from {:?}: {:?}",
                                peer_id,
                                e
                            );
                        }
                    }
                }
                detrp2p::Message::Ping { nonce } => {
                    let pong = detrp2p::Message::Pong { nonce };
                    let _ = p2p_network.unicast(peer_id, pong).await;
                }
                _ => {
                    log::debug!("📬 P2P message from {:?}: {:?}", peer_id, message);
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
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

fn convert_vote_from_bridge(vote_data: VoteData) -> finality_gadget::Vote {
    finality_gadget::Vote {
        validator_id: finality_gadget::ValidatorId(AccountId32::new(vote_data.validator_id)),
        view: finality_gadget::View(vote_data.view),
        block_hash: finality_gadget::BlockHash::from_bytes(vote_data.block_hash),
        signature: vote_data.signature,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

fn convert_certificate_from_bridge(cert_data: CertificateData) -> finality_gadget::Certificate {
    finality_gadget::Certificate {
        view: finality_gadget::View(cert_data.view),
        block_hash: finality_gadget::BlockHash::from_bytes(cert_data.block_hash),
        block_number: cert_data.block_number,
        signatures: cert_data
            .signatures
            .into_iter()
            .map(|(id, sig)| {
                (
                    finality_gadget::ValidatorId(AccountId32::new(id)),
                    sig,
                )
            })
            .collect(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

fn view_from_header<H: HeaderT>(header: &H) -> finality_gadget::View {
    finality_gadget::View((*header.number()).saturated_into())
}
