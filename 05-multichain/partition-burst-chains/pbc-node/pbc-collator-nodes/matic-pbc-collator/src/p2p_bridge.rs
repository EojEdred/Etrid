//! P2P Bridge for MATIC-PBC Collator
//!
//! This module bridges DETR P2P networking with Substrate's block sync
//! and provides message handling for distributed consensus.

use codec::{Decode, Encode};
use detrp2p::{P2PNetwork, Message, PeerId};
use etrid_protocol::{
    BlockAnnounceMessage,
    BlockRequestMessage,
    BlockResponseMessage,
    BlockSyncMessage,
    StatusRequestMessage,
    StatusResponseMessage,
};
use sc_client_api::{BlockBackend, HeaderBackend};
use sp_runtime::generic::SignedBlock;
use sp_runtime::traits::{Header as HeaderT, NumberFor, SaturatedConversion, Zero};
use std::sync::Arc;
use std::time::Duration;

use crate::service::FullClient;

type Block = <FullClient as BlockBackend>::Block;

/// P2P Bridge manages the interaction between DETR P2P and Substrate
pub struct P2PBridge {
    network: Arc<P2PNetwork>,
    client: Arc<FullClient>,
    running: Arc<tokio::sync::Mutex<bool>>,
}

impl P2PBridge {
    /// Create a new P2P bridge
    pub fn new(network: Arc<P2PNetwork>, client: Arc<FullClient>) -> Self {
        Self {
            network,
            client,
            running: Arc::new(tokio::sync::Mutex::new(false)),
        }
    }

    /// Start the P2P bridge
    pub async fn start(&self) {
        let mut running = self.running.lock().await;
        if *running {
            log::warn!("P2P bridge already running");
            return;
        }

        log::info!("🌉 Starting P2P Bridge...");

        // Start block announcement handler
        self.start_block_announcements();

        // Start incoming message handler
        self.start_message_handler();

        *running = true;
        log::info!("✅ P2P Bridge started successfully");
    }

    /// Handle block announcements - broadcast new blocks to P2P network
    fn start_block_announcements(&self) {
        let network = self.network.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            let mut last_announced_block = 0u32;

            log::info!("📢 Block announcement handler started");

            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;

                let best_number = client.info().best_number;

                // Check if we have a new block to announce
                if best_number > last_announced_block {
                    let best_hash = client.info().best_hash;

                    match client.header(best_hash) {
                        Ok(Some(header)) => {
                            let parent_hash = *header.parent_hash();
                            let block_number = *header.number() as u64;

                            // Create block hash array
                            let block_hash_bytes: [u8; 32] = best_hash.as_ref().try_into()
                                .unwrap_or([0u8; 32]);
                            let parent_hash_bytes: [u8; 32] = parent_hash.as_ref().try_into()
                                .unwrap_or([0u8; 32]);

                            let encoded_block = match client.block(best_hash) {
                                Ok(Some(signed_block)) => signed_block.encode(),
                                Ok(None) => {
                                    log::warn!("⚠️ Block #{} not found for announce", block_number);
                                    Vec::new()
                                }
                                Err(e) => {
                                    log::error!("❌ Error fetching block #{}: {:?}", block_number, e);
                                    Vec::new()
                                }
                            };

                            if encoded_block.is_empty() {
                                log::warn!("⚠️ Skipping BlockAnnounce #{} (empty block)", block_number);
                                continue;
                            }

                            let message = BlockAnnounceMessage {
                                block_number,
                                block_hash: block_hash_bytes,
                                parent_hash: parent_hash_bytes,
                                encoded_block,
                            };
                            let message: Message = message.into();

                            // Broadcast to all connected peers
                            match network.broadcast(message).await {
                                Ok(()) => {
                                    log::info!(
                                        "📢 Announced block #{}",
                                        block_number
                                    );
                                }
                                Err(e) => {
                                    log::warn!(
                                        "⚠️ Failed to announce block #{}: {}",
                                        block_number,
                                        e
                                    );
                                }
                            }

                            last_announced_block = best_number;
                        }
                        Ok(None) => {
                            log::warn!("⚠️ Header not found for block #{}", best_number);
                        }
                        Err(e) => {
                            log::error!(
                                "❌ Error reading header for block #{}: {:?}",
                                best_number,
                                e
                            );
                        }
                    }
                }
            }
        });
    }

    /// Handle incoming messages from P2P network
    fn start_message_handler(&self) {
        let network = self.network.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            log::info!("📥 Incoming message handler started");

            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;

                // Retrieve messages from inbox (one at a time)
                while let Some((peer_id, message)) = network.receive_message().await {
                    Self::handle_message(&network, &client, peer_id, message).await;
                }
            }
        });
    }

    /// Handle a single message from a peer
    async fn handle_message(
        network: &Arc<P2PNetwork>,
        client: &Arc<FullClient>,
        peer_id: PeerId,
        message: Message,
    ) {
        if let Some(block_sync) = message.as_block_sync() {
            match block_sync {
                BlockSyncMessage::BlockAnnounce(msg) => {
                    log::info!(
                        "📥 Received block announcement #{} from peer {:?}",
                        msg.block_number,
                        peer_id
                    );
                    log::debug!("  Block hash: {}", hex::encode(msg.block_hash));
                    log::debug!("  Parent hash: {}", hex::encode(msg.parent_hash));

                    match SignedBlock::<Block>::decode(&mut &msg.encoded_block[..]) {
                        Ok(signed_block) => {
                            let decoded_hash = signed_block.block.header.hash();
                            log::debug!(
                                "  Decoded block #{} (hash: {})",
                                msg.block_number,
                                hex::encode(decoded_hash.as_ref())
                            );
                        }
                        Err(e) => {
                            log::warn!(
                                "⚠️ Failed to decode BlockAnnounce #{} from {:?}: {:?}",
                                msg.block_number,
                                peer_id,
                                e
                            );
                        }
                    }
                }
                BlockSyncMessage::BlockRequest(req) => {
                    log::info!(
                        "📥 Received block request (id: {}) from peer {:?}",
                        req.request_id,
                        peer_id
                    );
                    Self::handle_block_request(network, client, peer_id, req).await;
                }
                BlockSyncMessage::BlockResponse(resp) => {
                    log::info!(
                        "📥 Received block response #{} from peer {:?}",
                        resp.block_number,
                        peer_id
                    );
                    match SignedBlock::<Block>::decode(&mut &resp.encoded_block[..]) {
                        Ok(signed_block) => {
                            let decoded_hash = signed_block.block.header.hash();
                            if decoded_hash.as_ref() != resp.block_hash.as_ref() {
                                log::warn!(
                                    "⚠️ BlockResponse hash mismatch (expected {}, got {})",
                                    hex::encode(resp.block_hash),
                                    hex::encode(decoded_hash.as_ref())
                                );
                            }
                        }
                        Err(e) => {
                            log::warn!(
                                "⚠️ Failed to decode BlockResponse #{} from {:?}: {:?}",
                                resp.block_number,
                                peer_id,
                                e
                            );
                        }
                    }
                }
                BlockSyncMessage::StatusRequest(req) => {
                    let info = client.info();
                    let best_number: u64 = info.best_number.saturated_into();
                    let best_hash = info.best_hash;
                    let genesis_hash = client.hash(Zero::zero()).ok().flatten().unwrap_or(best_hash);
                    let mut best_hash_bytes = [0u8; 32];
                    best_hash_bytes.copy_from_slice(best_hash.as_ref());
                    let mut genesis_bytes = [0u8; 32];
                    genesis_bytes.copy_from_slice(genesis_hash.as_ref());

                    let response = StatusResponseMessage {
                        request_id: req.request_id,
                        best_number,
                        best_hash: best_hash_bytes,
                        genesis_hash: genesis_bytes,
                    };
                    let response: Message = response.into();
                    if let Err(e) = network.unicast(peer_id, response).await {
                        log::warn!("⚠️ Failed to send status response: {}", e);
                    }
                }
                BlockSyncMessage::StatusResponse(resp) => {
                    let our_best: u64 = client.info().best_number.saturated_into();
                    if resp.best_number > our_best + 2 {
                        log::info!(
                            "Peer {:?} ahead (their best #{}, ours #{})",
                            peer_id,
                            resp.best_number,
                            our_best
                        );
                    }
                }
            }
            return;
        }

        match message {
            Message::Vote { data } => {
                log::info!(
                    "🗳️ Received vote message from peer {:?} ({} bytes)",
                    peer_id,
                    data.len()
                );
                // Forward to consensus module
            }

            Message::Certificate { data } => {
                log::info!(
                    "📜 Received certificate message from peer {:?} ({} bytes)",
                    peer_id,
                    data.len()
                );
                // Forward to consensus module
            }

            Message::Ping { nonce } => {
                // Respond with pong using unicast
                let pong = Message::Pong { nonce };
                if let Err(e) = network.unicast(peer_id, pong).await {
                    log::warn!("⚠️ Failed to send pong to peer {:?}: {}", peer_id, e);
                }
            }

            Message::Pong { nonce: _ } => {
                // Pong received, connection is alive
            }

            Message::Announce { peer, pow_nonce: _ } => {
                log::info!(
                    "📢 Received peer announcement from {:?}: {:?}",
                    peer_id,
                    peer.address
                );
                // Note: Peer remapping is handled internally by detrp2p connection manager
                log::debug!(
                    "  Announced peer ID: {:?}, address: {:?}",
                    peer.id,
                    peer.address
                );
            }

            _ => {
                log::debug!("📥 Received message from peer {:?}: {:?}", peer_id, message);
            }
        }
    }

    /// Handle block request by fetching and sending the block
    async fn handle_block_request(
        network: &Arc<P2PNetwork>,
        client: &Arc<FullClient>,
        peer_id: PeerId,
        request: BlockRequestMessage,
    ) {
        // Try to fetch the block
        let block_result = if let Some(number) = request.by_number {
            // Fetch by number
            let number_nf: NumberFor<Block> = number.saturated_into();
            match client.hash(number_nf) {
                Ok(Some(hash)) => {
                    log::debug!("Found block #{} with hash {:?}", number, hash);
                    Some((number, hash))
                }
                Ok(None) => {
                    log::warn!("Block #{} not found", number);
                    None
                }
                Err(e) => {
                    log::error!("Error fetching block #{}: {:?}", number, e);
                    None
                }
            }
        } else if let Some(hash_bytes) = request.by_hash {
            // Fetch by hash
            let hash = sp_core::H256::from_slice(&hash_bytes);
            match client.header(hash) {
                Ok(Some(header)) => {
                    let number: u64 = (*header.number()).saturated_into();
                    log::debug!("Found block with hash {:?} at #{}", hash, number);
                    Some((number, hash))
                }
                Ok(None) => {
                    log::warn!("Block with hash {:?} not found", hash);
                    None
                }
                Err(e) => {
                    log::error!("Error fetching block by hash {:?}: {:?}", hash, e);
                    None
                }
            }
        } else {
            log::warn!("Block request without number or hash");
            None
        };

        // Send response only if block was found
        if let Some((number, hash)) = block_result {
            let hash_bytes: [u8; 32] = hash.as_ref().try_into().unwrap_or([0u8; 32]);
            let parent_hash = client.header(hash)
                .ok()
                .flatten()
                .map(|h| {
                    let p: [u8; 32] = h.parent_hash().as_ref().try_into().unwrap_or([0u8; 32]);
                    p
                })
                .unwrap_or([0u8; 32]);

            let encoded_block = match client.block(hash) {
                Ok(Some(signed_block)) => signed_block.encode(),
                Ok(None) => {
                    log::warn!("Block #{} not found for response", number);
                    Vec::new()
                }
                Err(e) => {
                    log::error!("Error fetching block #{}: {:?}", number, e);
                    Vec::new()
                }
            };

            if encoded_block.is_empty() {
                log::warn!("Skipping BlockResponse #{} (empty block)", number);
                return;
            }

            let response = BlockResponseMessage {
                request_id: request.request_id,
                block_number: number,
                block_hash: hash_bytes,
                parent_hash,
                encoded_block,
            };
            let response: Message = response.into();

            // Send response to peer using unicast
            if let Err(e) = network.unicast(peer_id, response).await {
                log::warn!(
                    "⚠️ Failed to send block response to peer {:?}: {}",
                    peer_id,
                    e
                );
            }
        } else {
            log::debug!("Block not found, not sending response");
        }
    }

    /// Stop the P2P bridge
    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        if !*running {
            return;
        }

        log::info!("🛑 Stopping P2P Bridge...");
        *running = false;
        log::info!("✅ P2P Bridge stopped");
    }
}
