//! DETR P2P Network Configuration and Integration for BNB PBC Collator
//!
//! This module integrates the DETR P2P networking layer with the BNB PBC collator node,
//! providing:
//! - Automatic public IP detection
//! - PeerId identity remapping for proper cryptographic identity handling
//! - Automatic reconnection logic
//! - Background maintenance tasks (DHT, discovery, reconnection)
//! - Environment variable configuration support

use detrp2p::{P2PNetwork, PeerId, PeerAddr, detect_public_ip};
use std::net::{SocketAddr, IpAddr};
use std::sync::Arc;
use tokio::sync::RwLock;

/// P2P Network Configuration
#[derive(Clone, Debug)]
pub struct P2PConfig {
    /// Local node's cryptographic identity (32 bytes)
    pub node_id: PeerId,

    /// Address to bind the P2P listener (e.g., 0.0.0.0:30333)
    pub bind_address: SocketAddr,

    /// Public address to announce to peers (auto-detected if None)
    pub announce_address: Option<SocketAddr>,

    /// Bootstrap peers to connect to on startup
    pub bootstrap_peers: Vec<PeerAddr>,

    /// Enable automatic public IP detection
    pub auto_detect_ip: bool,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            node_id: PeerId::new(rand::random()),
            bind_address: "0.0.0.0:30333".parse().unwrap(),
            announce_address: None,
            bootstrap_peers: Vec::new(),
            auto_detect_ip: true,
        }
    }
}

impl P2PConfig {
    /// Create a new P2P configuration with specified parameters
    pub fn new(
        node_id: PeerId,
        bind_address: SocketAddr,
        bootstrap_peers: Vec<PeerAddr>,
    ) -> Self {
        Self {
            node_id,
            bind_address,
            announce_address: None,
            bootstrap_peers,
            auto_detect_ip: true,
        }
    }

    /// Set the announce address explicitly (disables auto-detection)
    pub fn with_announce_address(mut self, address: SocketAddr) -> Self {
        self.announce_address = Some(address);
        self.auto_detect_ip = false;
        self
    }

    /// Load configuration from environment variables
    /// Supports:
    /// - DETR_P2P_ANNOUNCE_IP: Public IP address to announce
    /// - DETR_P2P_BIND_PORT: Port to bind (default: 30333)
    /// - DETR_P2P_NODE_ID: Hex-encoded node ID (32 bytes)
    pub fn from_env(mut self) -> Self {
        // Check for explicit announce IP
        if let Ok(announce_ip) = std::env::var("DETR_P2P_ANNOUNCE_IP") {
            if let Ok(ip) = announce_ip.parse::<IpAddr>() {
                let port = self.bind_address.port();
                self.announce_address = Some(SocketAddr::new(ip, port));
                self.auto_detect_ip = false;
                log::info!("📢 Using announce IP from env: {}", ip);
            }
        }

        // Check for custom bind port
        if let Ok(port_str) = std::env::var("DETR_P2P_BIND_PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                self.bind_address.set_port(port);
                if let Some(ref mut announce) = self.announce_address {
                    announce.set_port(port);
                }
                log::info!("🔌 Using bind port from env: {}", port);
            }
        }

        // Check for custom node ID
        if let Ok(node_id_hex) = std::env::var("DETR_P2P_NODE_ID") {
            if let Ok(bytes) = hex::decode(&node_id_hex) {
                if bytes.len() == 32 {
                    let mut arr = [0u8; 32];
                    arr.copy_from_slice(&bytes);
                    self.node_id = PeerId::new(arr);
                    log::info!("🔑 Using node ID from env");
                }
            }
        }

        self
    }

    /// Auto-detect and configure the announce address
    pub async fn detect_announce_address(&mut self) -> Result<(), String> {
        if !self.auto_detect_ip || self.announce_address.is_some() {
            return Ok(());
        }

        log::info!("🔍 Auto-detecting public IP address...");

        match detect_public_ip().await {
            Some(ip) => {
                let port = self.bind_address.port();
                self.announce_address = Some(SocketAddr::new(ip, port));
                log::info!("✅ Detected announce address: {}:{}", ip, port);
                Ok(())
            }
            None => {
                log::warn!("⚠️ Could not auto-detect public IP");
                log::warn!("⚠️ Set DETR_P2P_ANNOUNCE_IP environment variable");
                Err("Failed to auto-detect public IP".to_string())
            }
        }
    }
}

/// P2P Network Manager
/// Wraps the DETR P2P network and provides lifecycle management
pub struct P2PNetworkManager {
    network: Arc<P2PNetwork>,
    config: P2PConfig,
    running: Arc<RwLock<bool>>,
}

impl P2PNetworkManager {
    /// Create a new P2P network manager
    pub async fn new(mut config: P2PConfig) -> Result<Self, String> {
        // Auto-detect announce address if needed
        if config.auto_detect_ip && config.announce_address.is_none() {
            config.detect_announce_address().await?;
        }

        log::info!("🌐 Initializing DETR P2P Network for BNB PBC Collator");
        log::info!("   Node ID: {:?}", config.node_id);
        log::info!("   Bind Address: {}", config.bind_address);
        if let Some(announce) = config.announce_address {
            log::info!("   Announce Address: {}", announce);
        }
        log::info!("   Bootstrap Peers: {}", config.bootstrap_peers.len());

        // Create P2P network with announce address support
        let network = Arc::new(P2PNetwork::new_with_announce(
            config.node_id,
            config.bind_address,
            config.announce_address,
            config.bootstrap_peers.clone(),
        ));

        Ok(Self {
            network,
            config,
            running: Arc::new(RwLock::new(false)),
        })
    }

    /// Start the P2P network
    /// This will:
    /// 1. Start listening for incoming connections
    /// 2. Connect to bootstrap peers
    /// 3. Start all background maintenance tasks
    pub async fn start(&self) -> Result<(), String> {
        let mut running = self.running.write().await;
        if *running {
            return Err("P2P network already running".to_string());
        }

        log::info!("🚀 Starting DETR P2P Network...");

        // Start the P2P network
        self.network.start().await?;

        // Start all background maintenance tasks
        // This includes:
        // - DHT maintenance (bucket refresh, republishing)
        // - Periodic peer discovery
        // - Automatic reconnection to failed peers
        self.network.start_all_maintenance();

        *running = true;

        log::info!("✅ DETR P2P Network started successfully");
        log::info!("   Listening on: {}", self.config.bind_address);
        if let Some(announce) = self.config.announce_address {
            log::info!("   Announcing: {}", announce);
        }

        Ok(())
    }

    /// Stop the P2P network
    pub async fn stop(&self) -> Result<(), String> {
        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }

        log::info!("🛑 Stopping DETR P2P Network...");

        // Note: P2PNetwork doesn't have a stop() method yet
        // Background tasks will naturally stop when the network is dropped

        *running = false;

        log::info!("✅ DETR P2P Network stopped");

        Ok(())
    }

    /// Check if the network is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Get a reference to the underlying P2P network
    pub fn network(&self) -> Arc<P2PNetwork> {
        self.network.clone()
    }

    /// Get the local node ID
    pub fn local_node_id(&self) -> PeerId {
        self.network.local_node_id()
    }

    /// Get the local peer info (ID + address)
    pub fn local_peer_info(&self) -> &PeerAddr {
        self.network.local_peer_info()
    }

    /// Get network statistics
    pub async fn get_stats(&self) -> NetworkStats {
        NetworkStats {
            local_node_id: self.local_node_id(),
            bind_address: self.config.bind_address,
            announce_address: self.config.announce_address,
            is_running: self.is_running().await,
            bootstrap_peers: self.config.bootstrap_peers.len(),
        }
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub local_node_id: PeerId,
    pub bind_address: SocketAddr,
    pub announce_address: Option<SocketAddr>,
    pub is_running: bool,
    pub bootstrap_peers: usize,
}

/// Helper function to generate a deterministic node ID from chain ID and validator key
pub fn generate_node_id_from_validator_key(validator_key: &[u8]) -> PeerId {
    use sp_core::Hasher;
    use sp_runtime::traits::BlakeTwo256;

    let hash = BlakeTwo256::hash(validator_key);
    let mut node_id = [0u8; 32];
    node_id.copy_from_slice(hash.as_ref());

    PeerId::new(node_id)
}

/// Helper function to parse bootstrap peers from string
/// Format: "node_id@ip:port,node_id@ip:port,..."
pub fn parse_bootstrap_peers(peers_str: &str) -> Result<Vec<PeerAddr>, String> {
    if peers_str.is_empty() {
        return Ok(Vec::new());
    }

    let mut peers = Vec::new();

    for peer_str in peers_str.split(',') {
        let peer_str = peer_str.trim();
        if peer_str.is_empty() {
            continue;
        }

        peers.push(parse_bootstrap_peer(peer_str)?);
    }

    Ok(peers)
}

fn parse_bootstrap_peer(peer_str: &str) -> Result<PeerAddr, String> {
    let (mut left, mut right) = match peer_str.split_once('@') {
        Some((a, b)) => (a.trim(), b.trim()),
        None => ("", peer_str.trim()),
    };

    if right.is_empty() {
        return Err(format!("Invalid peer format: {}", peer_str));
    }

    if left.contains(':') && !right.contains(':') {
        std::mem::swap(&mut left, &mut right);
    }

    let address = right
        .parse::<SocketAddr>()
        .map_err(|e| format!("Invalid socket address: {}", e))?;

    let peer_id = if left.is_empty() {
        PeerId::from_socket_addr(address)
    } else {
        let node_id_bytes = hex::decode(left)
            .map_err(|e| format!("Invalid node ID hex: {}", e))?;

        if node_id_bytes.len() != 32 {
            return Err(format!("Node ID must be 32 bytes, got {}", node_id_bytes.len()));
        }

        let mut node_id = [0u8; 32];
        node_id.copy_from_slice(&node_id_bytes);
        PeerId::new(node_id)
    };

    Ok(PeerAddr { id: peer_id, address })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bootstrap_peers() {
        let peers_str = "0000000000000000000000000000000000000000000000000000000000000001@127.0.0.1:30333";
        let peers = parse_bootstrap_peers(peers_str).unwrap();

        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address.to_string(), "127.0.0.1:30333");
    }

    #[test]
    fn test_generate_node_id() {
        let key = b"test_validator_key";
        let node_id = generate_node_id_from_validator_key(key);

        // Should be deterministic
        let node_id2 = generate_node_id_from_validator_key(key);
        assert_eq!(node_id.as_bytes(), node_id2.as_bytes());
    }
}
