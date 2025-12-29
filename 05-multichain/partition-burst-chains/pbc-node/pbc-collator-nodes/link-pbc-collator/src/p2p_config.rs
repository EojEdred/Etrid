//! DETR P2P Network Configuration for LINK-PBC Collator
//!
//! This module provides P2P network configuration and initialization
//! using the DETR P2P library with full support for:
//! - Public IP auto-detection
//! - PeerId identity remapping
//! - Automatic reconnection
//! - DHT maintenance
//! - Encrypted communication via aecomms

use detrp2p::{detect_public_ip, PeerId, PeerAddr, P2PNetwork};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

/// P2P Configuration for the collator node
#[derive(Clone, Debug)]
pub struct P2PConfig {
    /// Local node ID (derived from validator key)
    pub local_node_id: PeerId,
    /// Bind address (can be 0.0.0.0 for all interfaces)
    pub bind_address: SocketAddr,
    /// Public announce address (auto-detected if None)
    pub announce_address: Option<SocketAddr>,
    /// Bootstrap peers to connect to
    pub bootstrap_peers: Vec<PeerAddr>,
    /// Enable P2P networking
    pub enabled: bool,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            local_node_id: PeerId::new([0u8; 32]),
            bind_address: "0.0.0.0:30333".parse().unwrap(),
            announce_address: None,
            bootstrap_peers: Vec::new(),
            enabled: true,
        }
    }
}

impl P2PConfig {
    /// Create a new P2P configuration
    pub fn new(
        local_node_id: PeerId,
        bind_address: SocketAddr,
        bootstrap_peers: Vec<PeerAddr>,
    ) -> Self {
        Self {
            local_node_id,
            bind_address,
            announce_address: None,
            bootstrap_peers,
            enabled: true,
        }
    }

    /// Set the public announce address
    pub fn with_announce_address(mut self, address: SocketAddr) -> Self {
        self.announce_address = Some(address);
        self
    }

    /// Auto-detect the public announce address
    pub async fn detect_announce_address(&mut self) -> Result<(), String> {
        // First check environment variable
        if let Ok(ip_str) = std::env::var("DETR_P2P_ANNOUNCE_IP") {
            match ip_str.parse::<IpAddr>() {
                Ok(ip) => {
                    let port = self.bind_address.port();
                    self.announce_address = Some(SocketAddr::new(ip, port));
                    log::info!("📢 Using announce IP from DETR_P2P_ANNOUNCE_IP: {}", ip);
                    return Ok(());
                }
                Err(e) => {
                    log::warn!("⚠️ Failed to parse DETR_P2P_ANNOUNCE_IP: {}", e);
                }
            }
        }

        // Auto-detect using DETR P2P function
        match detect_public_ip().await {
            Some(ip) => {
                let port = self.bind_address.port();
                self.announce_address = Some(SocketAddr::new(ip, port));
                log::info!("📢 Auto-detected public IP: {}", ip);
                Ok(())
            }
            None => {
                log::warn!("⚠️ Could not auto-detect public IP. Using bind address as fallback.");
                log::warn!("⚠️ Set DETR_P2P_ANNOUNCE_IP environment variable to your public IP");
                self.announce_address = Some(self.bind_address);
                Ok(())
            }
        }
    }

    /// Initialize the P2P network
    pub async fn initialize(&mut self) -> Result<Arc<P2PNetwork>, String> {
        if !self.enabled {
            return Err("P2P networking is disabled".to_string());
        }

        // Auto-detect announce address if not set
        if self.announce_address.is_none() {
            self.detect_announce_address().await?;
        }

        log::info!("🌐 Initializing DETR P2P Network:");
        log::info!("  Local Node ID: {:?}", self.local_node_id);
        log::info!("  Bind Address: {}", self.bind_address);
        log::info!("  Announce Address: {:?}", self.announce_address);
        log::info!("  Bootstrap Peers: {}", self.bootstrap_peers.len());

        // Create P2P network with separate bind and announce addresses
        let network = P2PNetwork::new_with_announce(
            self.local_node_id,
            self.bind_address,
            self.announce_address,
            self.bootstrap_peers.clone(),
        );

        Ok(Arc::new(network))
    }
}

/// P2P Network Service wrapper
pub struct P2PNetworkService {
    network: Arc<P2PNetwork>,
    running: Arc<tokio::sync::Mutex<bool>>,
    bootstrap_peers: Vec<PeerAddr>,
}

impl P2PNetworkService {
    /// Create a new P2P network service
    pub fn new(network: Arc<P2PNetwork>, bootstrap_peers: Vec<PeerAddr>) -> Self {
        Self {
            network,
            running: Arc::new(tokio::sync::Mutex::new(false)),
            bootstrap_peers,
        }
    }

    /// Start the P2P network service
    pub async fn start(&self) -> Result<(), String> {
        let mut running = self.running.lock().await;
        if *running {
            return Err("P2P network service already running".to_string());
        }

        log::info!("🚀 Starting DETR P2P Network Service...");

        // Start the P2P network (handles listening and background tasks internally)
        self.network.start().await?;

        // Small delay to ensure listener is ready
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Connect to bootstrap peers
        self.connect_to_bootstrap_peers().await;

        *running = true;
        log::info!("✅ DETR P2P Network Service started successfully");
        Ok(())
    }

    /// Connect to bootstrap peers
    async fn connect_to_bootstrap_peers(&self) {
        log::info!("🔗 Connecting to {} bootstrap peer(s)...", self.bootstrap_peers.len());

        for peer in &self.bootstrap_peers {
            match self.network.add_peer(peer.clone()).await {
                Ok(_) => log::info!("✅ Connected to bootstrap peer: {:?}", peer.address),
                Err(e) => log::warn!("⚠️ Failed to connect to bootstrap peer {:?}: {}", peer.address, e),
            }
        }
    }

    /// Get the underlying P2P network
    pub fn network(&self) -> Arc<P2PNetwork> {
        self.network.clone()
    }

    /// Stop the P2P network service
    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        if !*running {
            return;
        }

        log::info!("🛑 Stopping DETR P2P Network Service...");
        // Note: P2PNetwork doesn't have a shutdown method; it cleans up when dropped
        *running = false;
        log::info!("✅ DETR P2P Network Service stopped");
    }
}

/// Helper function to create a PeerId from a public key
pub fn peer_id_from_public_key(public_key: &[u8]) -> PeerId {
    // Ensure we have 32 bytes
    let mut peer_id_bytes = [0u8; 32];
    let len = std::cmp::min(public_key.len(), 32);
    peer_id_bytes[..len].copy_from_slice(&public_key[..len]);

    PeerId::new(peer_id_bytes)
}

/// Parse bootstrap peers from a comma-separated string
/// Format: "peer_id@ip:port,peer_id@ip:port"
pub fn parse_bootstrap_peers(input: &str) -> Result<Vec<PeerAddr>, String> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut peers = Vec::new();
    for peer_str in input.split(',') {
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
        if left.len() != 64 {
            return Err(format!(
                "Invalid peer ID length: {}. Expected 64 hex chars (32 bytes)",
                left.len()
            ));
        }

        let mut peer_id_bytes = [0u8; 32];
        hex::decode_to_slice(left, &mut peer_id_bytes)
            .map_err(|e| format!("Invalid peer ID hex: {}", e))?;
        PeerId::new(peer_id_bytes)
    };

    Ok(PeerAddr { id: peer_id, address })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bootstrap_peers() {
        let input = "0000000000000000000000000000000000000000000000000000000000000001@127.0.0.1:30333";
        let result = parse_bootstrap_peers(input);
        assert!(result.is_ok());
        let peers = result.unwrap();
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address.to_string(), "127.0.0.1:30333");
    }

    #[test]
    fn test_parse_multiple_bootstrap_peers() {
        let input = "0000000000000000000000000000000000000000000000000000000000000001@127.0.0.1:30333,\
                     0000000000000000000000000000000000000000000000000000000000000002@127.0.0.1:30334";
        let result = parse_bootstrap_peers(input);
        assert!(result.is_ok());
        let peers = result.unwrap();
        assert_eq!(peers.len(), 2);
    }

    #[test]
    fn test_parse_empty_bootstrap_peers() {
        let result = parse_bootstrap_peers("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
