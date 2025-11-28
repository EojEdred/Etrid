//! P2P Bridge for XLM-PBC Collator
//!
//! This module bridges DETR P2P networking with Substrate's block sync
//! and provides message handling for distributed consensus.

use detrp2p::{P2PNetwork, Message, PeerId};
use sc_client_api::HeaderBackend;
use sp_runtime::traits::Header as HeaderT;
use std::sync::Arc;
use std::time::Duration;

use crate::service::FullClient;

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

                            // Get encoded block (in production, fetch from client)
                            let encoded_block = Vec::new(); // Simplified for now

                            // Create block announcement message
                            let message = Message::BlockAnnounce {
                                block_number,
                                block_hash: block_hash_bytes,
                                parent_hash: parent_hash_bytes,
                                encoded_block,
                            };

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
        match message {
            Message::BlockAnnounce {
                block_number,
                block_hash,
                parent_hash,
                encoded_block: _,
            } => {
                log::info!(
                    "📥 Received block announcement #{} from peer {:?}",
                    block_number,
                    peer_id
                );

                // In production: validate and import the block
                // For now, just log it
                log::debug!(
                    "  Block hash: {}",
                    hex::encode(block_hash)
                );
                log::debug!(
                    "  Parent hash: {}",
                    hex::encode(parent_hash)
                );
            }

            Message::BlockRequest {
                request_id,
                by_number,
                by_hash,
            } => {
                log::info!(
                    "📥 Received block request (id: {}) from peer {:?}",
                    request_id,
                    peer_id
                );

                // Handle block request
                Self::handle_block_request(
                    network,
                    client,
                    peer_id,
                    request_id,
                    by_number,
                    by_hash,
                )
                .await;
            }

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

            Message::Announce { peer } => {
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
        request_id: u64,
        by_number: Option<u64>,
        by_hash: Option<[u8; 32]>,
    ) {
        // Try to fetch the block
        let block_result = if let Some(number) = by_number {
            // Fetch by number
            match client.hash(number as u32) {
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
        } else if let Some(hash_bytes) = by_hash {
            // Fetch by hash
            let hash = sp_core::H256::from_slice(&hash_bytes);
            match client.header(hash) {
                Ok(Some(header)) => {
                    let number = *header.number() as u64;
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

            // In production: encode the full block
            let encoded_block = Vec::new();

            let response = Message::BlockResponse {
                request_id,
                block_number: number,
                block_hash: hash_bytes,
                parent_hash,
                encoded_block,
            };

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
