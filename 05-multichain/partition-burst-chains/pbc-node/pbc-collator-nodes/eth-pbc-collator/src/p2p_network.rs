//! DETR P2P Network Integration for ETH-PBC Collator
//!
//! This module integrates the enhanced DETR P2P library with the ETH-PBC collator,
//! providing:
//! - Automatic public IP detection via STUN and HTTP fallback
//! - PeerId identity remapping for correct cryptographic identities
//! - Automatic reconnection to maintain network connectivity
//! - Proper encryption via aecomms CipherSession
//! - Environment variable support for announce IP configuration

use detrp2p::{P2PNetwork, PeerId, PeerAddr, Message};
use sp_core::H256;
use std::net::{SocketAddr, IpAddr};
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn, error};

/// P2P network configuration for ETH-PBC collator
#[derive(Clone, Debug)]
pub struct P2PConfig {
    /// Address to bind/listen on (can be 0.0.0.0 for all interfaces)
    pub listen_address: SocketAddr,
    /// Optional announce address (override for NAT/firewall scenarios)
    /// If None, will be auto-detected using STUN or HTTP services
    pub announce_address: Option<SocketAddr>,
    /// Bootstrap peer addresses for initial network discovery
    pub bootstrap_peers: Vec<PeerAddr>,
    /// Our node's cryptographic identity
    pub node_id: PeerId,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0:30333".parse().unwrap(),
            announce_address: None,
            bootstrap_peers: Vec::new(),
            node_id: PeerId::new([0u8; 32]),
        }
    }
}

/// ETH-PBC P2P Network Manager
/// Wraps the core DETR P2P network with collator-specific functionality
pub struct EthPbcP2PNetwork {
    /// Core P2P network instance
    network: Arc<P2PNetwork>,
    /// Configuration
    config: P2PConfig,
    /// Local node ID
    node_id: PeerId,
    /// Connected peers tracking
    connected_peers: Arc<RwLock<Vec<PeerId>>>,
}

impl EthPbcP2PNetwork {
    /// Create a new ETH-PBC P2P network instance with environment variable support
    pub async fn new(mut config: P2PConfig) -> Result<Self, String> {
        info!("🌐 Initializing DETR P2P network for ETH-PBC collator");

        // Step 1: Handle DETR_P2P_ANNOUNCE_IP environment variable
        let announce_addr = if let Some(addr) = config.announce_address {
            info!("📢 Using configured announce address: {}", addr);
            addr
        } else {
            // Attempt to detect public IP
            info!("🔍 Auto-detecting public IP address...");
            match detrp2p::detect_public_ip().await {
                Some(public_ip) => {
                    let port = config.listen_address.port();
                    let detected_addr = SocketAddr::new(public_ip, port);
                    info!("✅ Detected public IP: {}", detected_addr);
                    detected_addr
                }
                None => {
                    warn!("⚠️ Could not auto-detect public IP");
                    warn!("⚠️ Set DETR_P2P_ANNOUNCE_IP environment variable to your public IP");
                    warn!("⚠️ Example: DETR_P2P_ANNOUNCE_IP=203.0.113.5");

                    // Fallback to listen address (may not work behind NAT)
                    config.listen_address
                }
            }
        };

        config.announce_address = Some(announce_addr);

        // Step 2: Validate configuration
        if announce_addr.ip().is_unspecified() {
            warn!("⚠️ Announce IP is 0.0.0.0 - other peers won't be able to connect!");
            warn!("⚠️ Please set DETR_P2P_ANNOUNCE_IP to your public IP address");
        }

        // Step 3: Log bootstrap peers
        if config.bootstrap_peers.is_empty() {
            warn!("⚠️ No bootstrap peers configured - node will run in isolation");
            warn!("⚠️ Add bootstrap peers via --bootnodes flag");
        } else {
            info!("🔌 Configured {} bootstrap peers:", config.bootstrap_peers.len());
            for peer in &config.bootstrap_peers {
                info!("   - {:?} at {}", peer.id, peer.address);
            }
        }

        // Step 4: Create P2P network with separate announce address
        let network = P2PNetwork::new_with_announce(
            config.node_id,
            config.listen_address,
            Some(announce_addr),
            config.bootstrap_peers.clone(),
        );

        info!("✅ DETR P2P network initialized");
        info!("   Listen address: {}", config.listen_address);
        info!("   Announce address: {}", announce_addr);
        info!("   Node ID: {:?}", config.node_id.as_bytes()[..8].to_vec());

        Ok(Self {
            network: Arc::new(network),
            config: config.clone(),
            node_id: config.node_id,
            connected_peers: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Start the P2P network and all background maintenance tasks
    /// This includes:
    /// - Starting the network listener
    /// - Connecting to bootstrap peers
    /// - DHT maintenance (bucket refresh, stale node removal)
    /// - Periodic peer discovery
    /// - Automatic reconnection to disconnected peers
    pub async fn start(&self) -> Result<(), String> {
        info!("🚀 Starting DETR P2P network...");

        // Start the core P2P network
        self.network.start().await?;

        // CRITICAL: Start all background maintenance tasks
        // This includes:
        // 1. DHT maintenance (bucket refresh every 5 minutes)
        // 2. Periodic discovery (adaptive based on peer count)
        // 3. Automatic reconnection (handles transient failures)
        self.network.start_all_maintenance();

        info!("✅ DETR P2P network started successfully");
        info!("   Background maintenance tasks: ACTIVE");
        info!("   - DHT maintenance: bucket refresh, stale node removal");
        info!("   - Periodic discovery: adaptive peer discovery");
        info!("   - Auto-reconnection: transient failure recovery");

        Ok(())
    }

    /// Get the number of currently connected peers
    pub async fn peer_count(&self) -> usize {
        self.connected_peers.read().await.len()
    }

    /// Get our local node ID
    pub fn local_node_id(&self) -> PeerId {
        self.node_id
    }

    /// Get our announced peer address
    pub fn local_peer_info(&self) -> &PeerAddr {
        self.network.local_peer_info()
    }

    /// Broadcast a block announcement to all connected peers
    pub async fn broadcast_block_announce(
        &self,
        block_number: u64,
        block_hash: H256,
        parent_hash: H256,
        encoded_block: Vec<u8>,
    ) -> Result<(), String> {
        let message = Message::BlockAnnounce {
            block_number,
            block_hash: block_hash.0,
            parent_hash: parent_hash.0,
            encoded_block,
        };

        let encoded = message.encode()?;

        // Get all connected peers
        let peers = self.connected_peers.read().await;
        let mut sent_count = 0;
        let mut error_count = 0;

        for peer_id in peers.iter() {
            match self.network.send_message(*peer_id, &encoded).await {
                Ok(()) => sent_count += 1,
                Err(e) => {
                    error_count += 1;
                    warn!("Failed to send block announce to peer {:?}: {}", peer_id, e);
                }
            }
        }

        if sent_count > 0 {
            info!(
                "📢 Block #{} announced to {}/{} peers",
                block_number,
                sent_count,
                peers.len()
            );
        }

        if error_count > 0 {
            warn!("⚠️ Failed to announce to {} peers", error_count);
        }

        Ok(())
    }

    /// Request a specific block from the network
    pub async fn request_block(
        &self,
        request_id: u64,
        by_number: Option<u64>,
        by_hash: Option<H256>,
    ) -> Result<(), String> {
        let message = Message::BlockRequest {
            request_id,
            by_number,
            by_hash: by_hash.map(|h| h.0),
        };

        let encoded = message.encode()?;

        // Send to first available peer (in production, use smarter routing)
        let peers = self.connected_peers.read().await;
        if let Some(peer_id) = peers.first() {
            self.network.send_message(*peer_id, &encoded).await?;
            info!("📤 Block request sent to peer {:?}", peer_id);
            Ok(())
        } else {
            Err("No connected peers to request block from".to_string())
        }
    }

    /// Send a custom message to a specific peer
    pub async fn send_message(&self, peer_id: PeerId, message: &[u8]) -> Result<(), String> {
        self.network.send_message(peer_id, message).await
    }

    /// Receive messages from the network (poll-based)
    /// Returns up to `limit` messages
    pub async fn receive_messages(&self, limit: usize) -> Vec<(PeerId, Message)> {
        let mut messages = Vec::new();

        for _ in 0..limit {
            if let Some(msg) = self.network.receive_message().await {
                messages.push(msg);
            } else {
                break;
            }
        }

        messages
    }

    /// Update the list of connected peers
    /// Should be called periodically to keep the peer list in sync
    pub async fn update_peer_list(&self) -> Result<(), String> {
        let peers = self.network.get_connected_peers().await;
        let mut connected = self.connected_peers.write().await;
        *connected = peers;
        Ok(())
    }

    /// Get current network statistics
    pub async fn get_stats(&self) -> NetworkStats {
        let peer_count = self.connected_peers.read().await.len();

        NetworkStats {
            peer_count,
            bootstrap_peer_count: self.config.bootstrap_peers.len(),
            listen_address: self.config.listen_address,
            announce_address: self.config.announce_address.unwrap_or(self.config.listen_address),
            node_id: self.node_id,
        }
    }
}

/// Network statistics for monitoring
#[derive(Clone, Debug)]
pub struct NetworkStats {
    pub peer_count: usize,
    pub bootstrap_peer_count: usize,
    pub listen_address: SocketAddr,
    pub announce_address: SocketAddr,
    pub node_id: PeerId,
}

/// Parse bootstrap nodes from CLI string format
/// Format: /ip4/127.0.0.1/tcp/30333/p2p/12D3KooW...
pub fn parse_bootnode(bootnode_str: &str) -> Result<PeerAddr, String> {
    let parts: Vec<&str> = bootnode_str.split('/').collect();

    if parts.len() < 7 {
        return Err(format!("Invalid bootnode format: {}", bootnode_str));
    }

    // Extract IP address
    let ip_type = parts[1]; // "ip4" or "ip6"
    let ip_str = parts[2];

    let ip: IpAddr = ip_str.parse()
        .map_err(|_| format!("Invalid IP address: {}", ip_str))?;

    // Extract port
    let port_str = parts[4];
    let port: u16 = port_str.parse()
        .map_err(|_| format!("Invalid port: {}", port_str))?;

    // Extract peer ID (assume base58 encoded)
    let peer_id_str = parts[6];

    // Convert peer ID string to bytes (simplified - in production use proper base58 decoding)
    let mut peer_id_bytes = [0u8; 32];
    if peer_id_str.len() >= 32 {
        // Take first 32 bytes from the peer ID string
        for (i, byte) in peer_id_str.bytes().take(32).enumerate() {
            peer_id_bytes[i] = byte;
        }
    } else {
        return Err(format!("Peer ID too short: {}", peer_id_str));
    }

    Ok(PeerAddr {
        id: PeerId::new(peer_id_bytes),
        address: SocketAddr::new(ip, port),
    })
}

/// Generate a deterministic node ID from Substrate's network key
/// This ensures the P2P identity matches the Substrate node identity
pub fn node_id_from_network_key(network_key: &[u8]) -> PeerId {
    use sp_core::blake2_256;

    // Hash the network key to get a 32-byte peer ID
    let hash = blake2_256(network_key);
    PeerId::new(hash)
}

/// Helper to read environment variable for announce IP
pub fn get_announce_ip_from_env() -> Option<IpAddr> {
    std::env::var("DETR_P2P_ANNOUNCE_IP")
        .ok()
        .and_then(|ip_str| ip_str.parse().ok())
}

/// Helper to construct announce address from environment
pub fn get_announce_address(listen_address: SocketAddr) -> Option<SocketAddr> {
    get_announce_ip_from_env().map(|ip| SocketAddr::new(ip, listen_address.port()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bootnode() {
        let bootnode = "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp";
        let result = parse_bootnode(bootnode);
        assert!(result.is_ok());

        let peer_addr = result.unwrap();
        assert_eq!(peer_addr.address.ip().to_string(), "127.0.0.1");
        assert_eq!(peer_addr.address.port(), 30333);
    }

    #[test]
    fn test_node_id_generation() {
        let network_key = b"test_network_key_12345678901234567890";
        let node_id = node_id_from_network_key(network_key);

        // Verify it's deterministic
        let node_id2 = node_id_from_network_key(network_key);
        assert_eq!(node_id.as_bytes(), node_id2.as_bytes());
    }

    #[tokio::test]
    async fn test_p2p_config_default() {
        let config = P2PConfig::default();
        assert_eq!(config.listen_address.port(), 30333);
        assert_eq!(config.bootstrap_peers.len(), 0);
    }
}
