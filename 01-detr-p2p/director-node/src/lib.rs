// director-node/src/lib.rs
// DIRECTOR NODE LIBRARY
// Core components for director-anchored peering model

use detrp2p::{Message, PeerId, PeerAddr};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use blake2::Digest;

pub mod relay;
pub mod slashing;

pub use relay::{MessageRelay, RelayMetrics};
pub use slashing::{SlashingDetector, SlashingEvidence, CheckpointSignature};

// ============================================================================
// DIRECTOR CONFIGURATION
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectorConfig {
    /// Director's own peer ID
    pub peer_id: PeerId,

    /// Director's Tailscale private IP
    pub tailscale_ip: String,

    /// Other director Tailscale IPs (for director-to-director mesh)
    pub peer_directors: Vec<String>,

    /// Maximum validators this director can handle (load balancing)
    pub max_validators_per_director: usize,

    /// Require validators to be in authorized registry
    pub require_authorization: bool,

    /// Listening port for validators (public internet)
    pub validator_port: u16,

    /// Listening port for directors (Tailscale private)
    pub director_port: u16,
}

impl Default for DirectorConfig {
    fn default() -> Self {
        Self {
            peer_id: PeerId::new([0u8; 32]),
            tailscale_ip: "100.64.0.1".to_string(),
            peer_directors: vec![],
            max_validators_per_director: 7,  // 21 validators / 3 directors
            require_authorization: true,
            validator_port: 30333,
            director_port: 30444,
        }
    }
}

// ============================================================================
// VALIDATOR AUTHORIZATION
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub peer_id: PeerId,
    pub address: SocketAddr,
    pub stake: u128,
    pub role: ValidatorRole,
    pub authorized_at: u64,  // Timestamp
    pub authorized_by: Vec<PeerId>,  // Director signatures
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidatorRole {
    DecentralizedDirector,  // Core validator with high stake
    ValidityNode,           // Rotating validator
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationProof {
    pub validator_id: PeerId,
    pub stake_proof: Vec<u8>,           // On-chain stake proof
    pub director_signatures: Vec<Vec<u8>>,  // Signatures from 2/3+ directors
    pub timestamp: u64,
}

impl AuthorizationProof {
    pub fn verify(&self, directors: &[PeerId]) -> bool {
        // Require 2/3+ director signatures
        let required_sigs = (directors.len() * 2) / 3 + 1;
        self.director_signatures.len() >= required_sigs
    }
}

pub struct ValidatorRegistry {
    /// Authorized validators (on-chain source of truth)
    authorized: Arc<RwLock<HashMap<PeerId, ValidatorInfo>>>,

    /// Pending authorization requests
    pending: Arc<RwLock<HashMap<PeerId, AuthorizationProof>>>,
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            authorized: Arc::new(RwLock::new(HashMap::new())),
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add validator to authorized registry
    pub async fn authorize_validator(
        &self,
        info: ValidatorInfo,
    ) -> Result<(), String> {
        let mut auth = self.authorized.write().await;

        // Verify not already authorized
        if auth.contains_key(&info.peer_id) {
            return Err("Validator already authorized".to_string());
        }

        log::info!(
            "✅ Authorized validator {:?} with role {:?}",
            info.peer_id,
            info.role
        );

        auth.insert(info.peer_id, info);
        Ok(())
    }

    /// Remove validator from registry
    pub async fn revoke_validator(&self, peer_id: PeerId) -> Result<(), String> {
        let mut auth = self.authorized.write().await;

        if auth.remove(&peer_id).is_some() {
            log::warn!("⚠️ Revoked validator {:?}", peer_id);
            Ok(())
        } else {
            Err("Validator not found".to_string())
        }
    }

    /// Check if validator is authorized
    pub async fn is_authorized(&self, peer_id: &PeerId) -> bool {
        self.authorized.read().await.contains_key(peer_id)
    }

    /// Get validator info
    pub async fn get_validator(&self, peer_id: &PeerId) -> Option<ValidatorInfo> {
        self.authorized.read().await.get(peer_id).cloned()
    }

    /// Get all authorized validators
    pub async fn get_all_validators(&self) -> Vec<ValidatorInfo> {
        self.authorized.read().await.values().cloned().collect()
    }

    /// Submit authorization request (pending director approval)
    pub async fn submit_authorization_request(
        &self,
        proof: AuthorizationProof,
    ) -> Result<(), String> {
        let mut pending = self.pending.write().await;
        pending.insert(proof.validator_id, proof);
        Ok(())
    }

    /// Get pending requests
    pub async fn get_pending_requests(&self) -> Vec<AuthorizationProof> {
        self.pending.read().await.values().cloned().collect()
    }

    /// Clear pending request
    pub async fn clear_pending(&self, peer_id: &PeerId) {
        self.pending.write().await.remove(peer_id);
    }
}

// ============================================================================
// DIRECTOR NODE
// ============================================================================

pub struct DirectorNode {
    /// Director configuration
    pub config: DirectorConfig,

    /// P2P network instance (from detrp2p)
    pub p2p: Option<Arc<detrp2p::P2PNetwork>>,

    /// Authorized validators registry
    pub validator_registry: Arc<ValidatorRegistry>,

    /// Currently connected validators
    pub connected_validators: Arc<RwLock<HashMap<PeerId, ValidatorInfo>>>,

    /// Peer directors (for director-to-director relay)
    pub peer_directors: Arc<RwLock<Vec<PeerId>>>,

    /// Message relay system
    pub message_relay: Arc<MessageRelay>,

    /// Slashing detector (double-sign detection)
    pub slashing_detector: Arc<RwLock<SlashingDetector>>,
}

impl DirectorNode {
    pub fn new(config: DirectorConfig) -> Self {
        Self {
            config,
            p2p: None,
            validator_registry: Arc::new(ValidatorRegistry::new()),
            connected_validators: Arc::new(RwLock::new(HashMap::new())),
            peer_directors: Arc::new(RwLock::new(vec![])),
            message_relay: Arc::new(MessageRelay::new()),
            slashing_detector: Arc::new(RwLock::new(SlashingDetector::new())),
        }
    }

    /// Initialize P2P network for director
    pub async fn start(&mut self, bootstrap_peers: Vec<PeerAddr>) -> Result<(), String> {
        let local_addr = format!("{}:{}", self.config.tailscale_ip, self.config.director_port)
            .parse()
            .map_err(|e| format!("Invalid director address: {}", e))?;

        let p2p = Arc::new(detrp2p::P2PNetwork::new(
            self.config.peer_id,
            local_addr,
            bootstrap_peers,
        ));

        p2p.start().await?;
        self.p2p = Some(p2p.clone());

        log::info!(
            "🎯 Director node started on {} (Tailscale)",
            local_addr
        );

        Ok(())
    }

    /// Connect to peer directors (Tailscale mesh)
    pub async fn connect_to_directors(&self) -> Result<(), String> {
        let p2p = self.p2p.as_ref().ok_or("P2P not initialized")?;

        for director_ip in &self.config.peer_directors {
            let addr = format!("{}:{}", director_ip, self.config.director_port)
                .parse()
                .map_err(|e| format!("Invalid director IP {}: {}", director_ip, e))?;

            let peer_id = PeerId::new(blake2::Blake2b512::digest(director_ip.as_bytes()).as_slice()[..32].try_into().unwrap());

            let peer_addr = PeerAddr {
                id: peer_id,
                address: addr,
            };

            p2p.add_peer(peer_addr).await?;

            let mut directors = self.peer_directors.write().await;
            if !directors.contains(&peer_id) {
                directors.push(peer_id);
            }

            log::info!("🔗 Connected to director: {}", director_ip);
        }

        Ok(())
    }

    /// Relay checkpoint message to other directors and validators
    pub async fn relay_checkpoint_message(
        &self,
        from: PeerId,
        msg: Message,
    ) -> Result<(), String> {
        let p2p = self.p2p.as_ref().ok_or("P2P not initialized")?;

        // Validate sender is authorized (if enforcement enabled)
        if self.config.require_authorization {
            if !self.validator_registry.is_authorized(&from).await {
                log::warn!("❌ Rejected message from unauthorized peer: {:?}", from);
                return Err("Unauthorized peer".to_string());
            }
        }

        // Check for message loops
        let msg_hash = self.message_relay.hash_message(&msg)?;
        if self.message_relay.is_seen(&msg_hash).await {
            log::trace!("♻️ Skipping already-seen message (loop prevention)");
            return Ok(()); // Already relayed, prevent loop
        }

        // Detect double-signing attacks
        if let Message::Certificate { data } = &msg {
            let mut detector = self.slashing_detector.write().await;
            if let Some(evidence) = detector.check_certificate(&from, data) {
                log::error!("🚨 SLASHING EVIDENCE DETECTED: {:?}", evidence);
                // TODO: Submit to on-chain slashing module
            }
        }

        log::info!("📡 Relaying message from {:?} to network", from);

        // Relay to peer directors (Tailscale mesh)
        let directors = self.peer_directors.read().await;
        for director_id in directors.iter() {
            if let Err(e) = p2p.unicast(*director_id, msg.clone()).await {
                log::warn!("Failed to relay to director {:?}: {}", director_id, e);
            }
        }

        // Relay to connected validators (except sender)
        let validators = self.connected_validators.read().await;
        for (peer_id, _) in validators.iter() {
            if *peer_id != from {
                if let Err(e) = p2p.unicast(*peer_id, msg.clone()).await {
                    log::warn!("Failed to relay to validator {:?}: {}", peer_id, e);
                }
            }
        }

        // Mark message as seen
        self.message_relay.mark_seen(msg_hash).await;

        // Update metrics
        self.message_relay.increment_relay_count().await;

        Ok(())
    }

    /// Authorize new validator (requires 2/3 director approval)
    pub async fn authorize_validator(
        &self,
        proof: AuthorizationProof,
    ) -> Result<(), String> {
        // Verify proof has enough director signatures
        let directors = self.peer_directors.read().await;
        let all_directors: Vec<PeerId> = directors.iter().cloned().collect();

        if !proof.verify(&all_directors) {
            return Err("Insufficient director signatures".to_string());
        }

        // Create validator info
        let info = ValidatorInfo {
            peer_id: proof.validator_id,
            address: "0.0.0.0:0".parse().unwrap(), // Updated when validator connects
            stake: 0, // Updated from proof
            role: ValidatorRole::ValidityNode,
            authorized_at: proof.timestamp,
            authorized_by: all_directors.clone(),
        };

        // Add to registry
        self.validator_registry.authorize_validator(info).await?;

        log::info!("✅ Validator {:?} authorized by director consensus", proof.validator_id);

        // Broadcast authorization to other directors
        // TODO: Send DirectorMessage::ValidatorAuthorization

        Ok(())
    }

    /// Revoke validator access
    pub async fn revoke_validator(&self, peer_id: PeerId, reason: String) -> Result<(), String> {
        // Remove from registry
        self.validator_registry.revoke_validator(peer_id).await?;

        // Disconnect validator (handled automatically when connection closes)
        log::info!("🔌 Disconnecting validator {:?} (reason: {})", peer_id, reason);

        // Remove from connected validators
        self.connected_validators.write().await.remove(&peer_id);

        // Broadcast revocation to other directors
        // TODO: Send DirectorMessage::ValidatorRevocation

        Ok(())
    }

    /// Get relay metrics
    pub async fn get_metrics(&self) -> RelayMetrics {
        self.message_relay.get_metrics().await
    }

    /// Get connected validator count
    pub async fn get_validator_count(&self) -> usize {
        self.connected_validators.read().await.len()
    }

    /// Check if accepting new validators (load balancing)
    pub async fn can_accept_validators(&self) -> bool {
        self.get_validator_count().await < self.config.max_validators_per_director
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validator_registry() {
        let registry = ValidatorRegistry::new();

        let validator = ValidatorInfo {
            peer_id: PeerId::new([1u8; 32]),
            address: "127.0.0.1:30333".parse().unwrap(),
            stake: 1000,
            role: ValidatorRole::ValidityNode,
            authorized_at: 0,
            authorized_by: vec![],
        };

        // Authorize
        registry.authorize_validator(validator.clone()).await.unwrap();
        assert!(registry.is_authorized(&validator.peer_id).await);

        // Revoke
        registry.revoke_validator(validator.peer_id).await.unwrap();
        assert!(!registry.is_authorized(&validator.peer_id).await);
    }

    #[tokio::test]
    async fn test_authorization_proof_verification() {
        let directors = vec![
            PeerId::new([1u8; 32]),
            PeerId::new([2u8; 32]),
            PeerId::new([3u8; 32]),
        ];

        // 2/3 = 2 signatures required
        let proof = AuthorizationProof {
            validator_id: PeerId::new([99u8; 32]),
            stake_proof: vec![],
            director_signatures: vec![vec![1, 2, 3], vec![4, 5, 6]], // 2 sigs
            timestamp: 0,
        };

        assert!(proof.verify(&directors));

        // Insufficient signatures
        let proof_insufficient = AuthorizationProof {
            validator_id: PeerId::new([99u8; 32]),
            stake_proof: vec![],
            director_signatures: vec![vec![1, 2, 3]], // Only 1 sig
            timestamp: 0,
        };

        assert!(!proof_insufficient.verify(&directors));
    }

    #[test]
    fn test_director_config_default() {
        let config = DirectorConfig::default();
        assert_eq!(config.max_validators_per_director, 7);
        assert_eq!(config.require_authorization, true);
        assert_eq!(config.validator_port, 30333);
        assert_eq!(config.director_port, 30444);
    }
}
