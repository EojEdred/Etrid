//! DETR P2P Configuration for EDSC-PBC Collator
//!
//! This module handles P2P network configuration including:
//! - Public IP auto-detection via STUN
//! - Environment variable configuration (DETR_P2P_ANNOUNCE_IP)
//! - Bootstrap peer management
//! - Proper announce address vs bind address separation

use std::net::{IpAddr, SocketAddr};
use detrp2p::{PeerId, PeerAddr};

/// P2P Configuration for EDSC-PBC collator
#[derive(Clone, Debug)]
pub struct P2PConfig {
    /// Address to bind/listen on (can be 0.0.0.0 for all interfaces)
    pub bind_address: SocketAddr,
    /// Address to announce to peers (must be publicly reachable)
    pub announce_address: Option<SocketAddr>,
    /// Our local node ID (cryptographic identity)
    pub local_node_id: PeerId,
    /// Bootstrap peers for initial network discovery
    pub bootstrap_peers: Vec<PeerAddr>,
}

impl P2PConfig {
    /// Create a new P2P configuration
    pub fn new(
        bind_address: SocketAddr,
        local_node_id: PeerId,
        bootstrap_peers: Vec<PeerAddr>,
    ) -> Self {
        Self {
            bind_address,
            announce_address: None,
            local_node_id,
            bootstrap_peers,
        }
    }

    /// Auto-detect public IP and set announce address
    /// This uses DETR P2P's detect_public_ip() which tries:
    /// 1. DETR_P2P_ANNOUNCE_IP environment variable (highest priority)
    /// 2. STUN-based UDP detection (fast, no external HTTP)
    /// 3. HTTP API fallback (if STUN fails)
    pub async fn with_auto_detected_ip(mut self) -> Self {
        log::info!("🔍 Auto-detecting public IP for P2P announce address...");

        match detrp2p::detect_public_ip().await {
            Some(public_ip) => {
                let announce_addr = SocketAddr::new(public_ip, self.bind_address.port());
                log::info!("✅ Public IP detected: {} -> announce address: {}", public_ip, announce_addr);
                self.announce_address = Some(announce_addr);
            }
            None => {
                log::warn!("⚠️ Could not auto-detect public IP");
                log::warn!("⚠️ Set DETR_P2P_ANNOUNCE_IP environment variable to your public IP");
                log::warn!("⚠️ Example: export DETR_P2P_ANNOUNCE_IP=203.0.113.42");

                // If bind address is not unspecified, use it as fallback
                if !self.bind_address.ip().is_unspecified() {
                    log::info!("📌 Using bind address as announce address: {}", self.bind_address);
                    self.announce_address = Some(self.bind_address);
                }
            }
        }

        self
    }

    /// Manually set the announce address (overrides auto-detection)
    pub fn with_announce_address(mut self, announce_address: SocketAddr) -> Self {
        log::info!("📢 Manually setting announce address: {}", announce_address);
        self.announce_address = Some(announce_address);
        self
    }

    /// Add a bootstrap peer
    pub fn add_bootstrap_peer(mut self, peer: PeerAddr) -> Self {
        self.bootstrap_peers.push(peer);
        self
    }

    /// Set bootstrap peers from a list of addresses
    pub fn with_bootstrap_peers(mut self, peers: Vec<PeerAddr>) -> Self {
        self.bootstrap_peers = peers;
        self
    }

    /// Get the effective announce address (fallback to bind if not set)
    pub fn effective_announce_address(&self) -> SocketAddr {
        self.announce_address.unwrap_or(self.bind_address)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        // Check if announce address is unspecified (0.0.0.0 or ::)
        let announce_addr = self.effective_announce_address();
        if announce_addr.ip().is_unspecified() {
            return Err(
                "Announce address is 0.0.0.0 - peers won't be able to connect! \
                Set DETR_P2P_ANNOUNCE_IP to your public IP address.".to_string()
            );
        }

        // Warn if no bootstrap peers
        if self.bootstrap_peers.is_empty() {
            log::warn!("⚠️ No bootstrap peers configured - node will not discover other peers automatically");
        }

        Ok(())
    }
}

/// Parse a bootstrap peer from string format "peer_id@ip:port"
/// Example: "a1b2c3d4...@192.168.1.100:30333"
pub fn parse_bootstrap_peer(s: &str) -> Result<PeerAddr, String> {
    let (mut left, mut right) = match s.split_once('@') {
        Some((a, b)) => (a.trim(), b.trim()),
        None => ("", s.trim()),
    };

    if right.is_empty() {
        return Err(format!("Invalid bootstrap peer format: {}", s));
    }

    if left.contains(':') && !right.contains(':') {
        std::mem::swap(&mut left, &mut right);
    }

    let address: SocketAddr = right
        .parse()
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

/// Load P2P configuration from environment variables
pub fn load_from_env() -> P2PConfig {
    // Generate a deterministic node ID from validator key or use random
    // In production, this should be derived from the validator's session keys
    let local_node_id = generate_node_id();

    // Default bind address (listen on all interfaces)
    let bind_address = std::env::var("DETR_P2P_BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:30333".to_string())
        .parse()
        .expect("Invalid DETR_P2P_BIND_ADDRESS");

    // Parse bootstrap peers from comma-separated list
    let bootstrap_peers = std::env::var("DETR_P2P_BOOTSTRAP_PEERS")
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
        .unwrap_or_default();

    P2PConfig::new(bind_address, local_node_id, bootstrap_peers)
}

/// Generate a node ID from validator keys or random
/// In production, this should be derived from the validator's cryptographic identity
fn generate_node_id() -> PeerId {
    use sp_core::crypto::Ss58Codec;
    use sp_core::sr25519;

    // Try to load from keystore or use deterministic generation
    // For now, we'll generate from the validator's public key if available
    if let Ok(key_str) = std::env::var("VALIDATOR_PUBLIC_KEY") {
        if let Ok(public_key) = sr25519::Public::from_ss58check(&key_str) {
            // Use first 32 bytes of the public key as node ID
            let mut node_id_bytes = [0u8; 32];
            node_id_bytes.copy_from_slice(&public_key.0);
            return PeerId::new(node_id_bytes);
        }
    }

    // Fallback: Generate from hostname + process info (semi-stable)
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| "edsc-collator".to_string());

    let pid = std::process::id();
    let seed = format!("edsc-pbc-{}-{}", hostname, pid);

    // Hash the seed to get a 32-byte node ID
    let mut node_id_bytes = [0u8; 32];
    use sp_core::hashing::blake2_256;
    node_id_bytes.copy_from_slice(&blake2_256(seed.as_bytes()));

    log::info!("🔑 Generated node ID from seed: {}", seed);
    log::info!("🆔 Node ID: {}", hex::encode(&node_id_bytes));

    PeerId::new(node_id_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bootstrap_peer() {
        let peer_str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef@192.168.1.100:30333";
        let peer = parse_bootstrap_peer(peer_str).unwrap();

        assert_eq!(peer.address.to_string(), "192.168.1.100:30333");
    }

    #[test]
    fn test_parse_bootstrap_peer_invalid() {
        assert!(parse_bootstrap_peer("invalid").is_err());
        assert!(parse_bootstrap_peer("abc@192.168.1.100:30333").is_err()); // Invalid peer ID length
    }

    #[test]
    fn test_config_validation() {
        let node_id = PeerId::new([1u8; 32]);

        // Valid configuration
        let config = P2PConfig::new(
            "0.0.0.0:30333".parse().unwrap(),
            node_id,
            vec![],
        ).with_announce_address("203.0.113.42:30333".parse().unwrap());

        assert!(config.validate().is_ok());

        // Invalid configuration (unspecified announce address)
        let invalid_config = P2PConfig::new(
            "0.0.0.0:30333".parse().unwrap(),
            node_id,
            vec![],
        );

        assert!(invalid_config.validate().is_err());
    }
}
