//! ÉTRID Protocol
//!
//! Defines all ÉTRID network message types, serialization format (bincode),
//! and protocol validation logic.

use serde::{Deserialize, Serialize};

// Gadget Network Bridge module (integration with finality gadget)
#[path = "../gadget-network-bridge/src/lib.rs"]
pub mod gadget_network_bridge;

/// Protocol version
pub const PROTOCOL_VERSION: u16 = 1;

/// Message type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Ping = 0,
    Pong = 1,
    PeerDiscovery = 2,
    Vote = 10,
    Block = 11,
    Transaction = 12,
    Certificate = 13,
    StateSync = 14,
    HeartBeat = 15,
    // V17: Checkpoint BFT messages
    CheckpointSignature = 16,
    CheckpointCertificate = 17,
    RequestCheckpointSignatures = 18,
    CheckpointSignaturesResponse = 19,
    Error = 255,
}

impl MessageType {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(MessageType::Ping),
            1 => Some(MessageType::Pong),
            2 => Some(MessageType::PeerDiscovery),
            10 => Some(MessageType::Vote),
            11 => Some(MessageType::Block),
            12 => Some(MessageType::Transaction),
            13 => Some(MessageType::Certificate),
            14 => Some(MessageType::StateSync),
            15 => Some(MessageType::HeartBeat),
            16 => Some(MessageType::CheckpointSignature),
            17 => Some(MessageType::CheckpointCertificate),
            18 => Some(MessageType::RequestCheckpointSignatures),
            19 => Some(MessageType::CheckpointSignaturesResponse),
            255 => Some(MessageType::Error),
            _ => None,
        }
    }
}

/// Ping message for keep-alive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingMessage {
    pub nonce: u64,
    pub timestamp: u64,
}

impl PingMessage {
    pub fn new(nonce: u64) -> Self {
        Self {
            nonce,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Pong response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PongMessage {
    pub nonce: u64,
    pub timestamp: u64,
}

impl PongMessage {
    pub fn respond_to(ping: &PingMessage) -> Self {
        Self {
            nonce: ping.nonce,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Peer discovery request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDiscoveryMessage {
    pub requester_id: Vec<u8>,
    pub target_id: Vec<u8>,
    pub request_id: u64,
}

impl PeerDiscoveryMessage {
    pub fn new(requester_id: Vec<u8>, target_id: Vec<u8>) -> Self {
        Self {
            requester_id,
            target_id,
            request_id: rand::random(),
        }
    }
}

/// Vote message for consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteMessage {
    pub validator_id: String,
    pub round: u64,
    pub block_hash: Vec<u8>,
    pub signature: Vec<u8>,
}

impl VoteMessage {
    pub fn new(validator_id: String, round: u64, block_hash: Vec<u8>, signature: Vec<u8>) -> Self {
        Self {
            validator_id,
            round,
            block_hash,
            signature,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.validator_id.is_empty() {
            return Err("Validator ID cannot be empty".to_string());
        }
        if self.block_hash.is_empty() {
            return Err("Block hash cannot be empty".to_string());
        }
        if self.signature.is_empty() {
            return Err("Signature cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Block message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMessage {
    pub block_height: u64,
    pub block_hash: Vec<u8>,
    pub parent_hash: Vec<u8>,
    pub timestamp: u64,
    pub validator: String,
    pub transactions: Vec<Vec<u8>>,
    pub signature: Vec<u8>,
}

impl BlockMessage {
    pub fn new(
        block_height: u64,
        block_hash: Vec<u8>,
        parent_hash: Vec<u8>,
        validator: String,
        transactions: Vec<Vec<u8>>,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            block_height,
            block_hash,
            parent_hash,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            validator,
            transactions,
            signature,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.block_hash.is_empty() {
            return Err("Block hash cannot be empty".to_string());
        }
        if self.parent_hash.is_empty() && self.block_height > 0 {
            return Err("Parent hash required for non-genesis blocks".to_string());
        }
        if self.signature.is_empty() {
            return Err("Block signature cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Transaction message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    pub tx_hash: Vec<u8>,
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub nonce: u64,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

impl TransactionMessage {
    pub fn new(
        from: String,
        to: String,
        amount: u128,
        nonce: u64,
        signature: Vec<u8>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut tx = Self {
            tx_hash: vec![],
            from,
            to,
            amount,
            nonce,
            signature,
            timestamp: now,
        };

        // Compute hash
        tx.tx_hash = Self::compute_hash(&tx);
        tx
    }

    fn compute_hash(tx: &Self) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(tx.from.as_bytes());
        hasher.update(tx.to.as_bytes());
        hasher.update(tx.amount.to_le_bytes());
        hasher.update(tx.nonce.to_le_bytes());
        hasher.finalize().to_vec()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.from.is_empty() || self.to.is_empty() {
            return Err("From/To addresses required".to_string());
        }
        if self.amount == 0 {
            return Err("Amount must be > 0".to_string());
        }
        if self.signature.is_empty() {
            return Err("Signature required".to_string());
        }
        Ok(())
    }
}

/// Certificate message (proof of finality)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateMessage {
    pub block_hash: Vec<u8>,
    pub round: u64,
    pub voter_count: u32,
    pub signatures: Vec<Vec<u8>>,
}

impl CertificateMessage {
    pub fn new(block_hash: Vec<u8>, round: u64) -> Self {
        Self {
            block_hash,
            round,
            voter_count: 0,
            signatures: vec![],
        }
    }

    pub fn add_signature(&mut self, signature: Vec<u8>) {
        self.signatures.push(signature);
        self.voter_count = self.signatures.len() as u32;
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.block_hash.is_empty() {
            return Err("Block hash required".to_string());
        }
        if self.voter_count as usize != self.signatures.len() {
            return Err("Voter count mismatch".to_string());
        }
        Ok(())
    }
}

/// State synchronization message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSyncMessage {
    pub from_block: u64,
    pub to_block: u64,
    pub state_root: Vec<u8>,
    pub block_hashes: Vec<Vec<u8>>,
}

impl StateSyncMessage {
    pub fn new(from_block: u64, to_block: u64, state_root: Vec<u8>) -> Self {
        Self {
            from_block,
            to_block,
            state_root,
            block_hashes: vec![],
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.from_block > self.to_block {
            return Err("Invalid block range".to_string());
        }
        if self.state_root.is_empty() {
            return Err("State root required".to_string());
        }
        Ok(())
    }
}

/// Heartbeat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartBeatMessage {
    pub peer_id: String,
    pub block_height: u64,
    pub timestamp: u64,
}

impl HeartBeatMessage {
    pub fn new(peer_id: String, block_height: u64) -> Self {
        Self {
            peer_id,
            block_height,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Error message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error_code: u32,
    pub message: String,
}

impl ErrorMessage {
    pub fn new(error_code: u32, message: String) -> Self {
        Self { error_code, message }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// V17: CHECKPOINT BFT MESSAGES
// ═══════════════════════════════════════════════════════════════════════════

/// Checkpoint signature broadcast (V17)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointSignatureMsg {
    pub block_number: u32,
    pub block_hash: [u8; 32],
    pub validator_id: u32,
    pub authority_set_id: u64,
    pub signature: Vec<u8>,
    pub timestamp_ms: u64,
}

impl CheckpointSignatureMsg {
    pub fn new(
        block_number: u32,
        block_hash: [u8; 32],
        validator_id: u32,
        authority_set_id: u64,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            block_number,
            block_hash,
            validator_id,
            authority_set_id,
            signature,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.signature.is_empty() {
            return Err("Signature required".to_string());
        }
        if self.block_number == 0 {
            return Err("Block number must be > 0".to_string());
        }
        Ok(())
    }
}

/// Checkpoint certificate broadcast (V17)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointCertificateMsg {
    pub block_number: u32,
    pub block_hash: [u8; 32],
    pub authority_set_id: u64,
    pub signatures: Vec<CheckpointSignatureMsg>,
    pub finalized_at_ms: u64,
}

impl CheckpointCertificateMsg {
    pub fn new(
        block_number: u32,
        block_hash: [u8; 32],
        authority_set_id: u64,
        signatures: Vec<CheckpointSignatureMsg>,
    ) -> Self {
        Self {
            block_number,
            block_hash,
            authority_set_id,
            signatures,
            finalized_at_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.signatures.is_empty() {
            return Err("Certificate must have signatures".to_string());
        }
        if self.block_number == 0 {
            return Err("Block number must be > 0".to_string());
        }
        for sig in &self.signatures {
            sig.validate()?;
        }
        Ok(())
    }
}

/// Request checkpoint signatures for a block (V17)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCheckpointSignaturesMsg {
    pub block_number: u32,
}

/// Response with checkpoint signatures (V17)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointSignaturesResponseMsg {
    pub block_number: u32,
    pub signatures: Vec<CheckpointSignatureMsg>,
}

/// Protocol message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub version: u16,
    pub msg_type: u8,
    pub payload: Vec<u8>,
}

impl ProtocolMessage {
    pub fn new(msg_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            msg_type: msg_type as u8,
            payload,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        bincode::serialize(self).map_err(|e| format!("Serialization error: {}", e))
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        bincode::deserialize(bytes).map_err(|e| format!("Deserialization error: {}", e))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.version != PROTOCOL_VERSION {
            return Err("Invalid protocol version".to_string());
        }
        if MessageType::from_u8(self.msg_type).is_none() {
            return Err("Unknown message type".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_conversion() {
        assert_eq!(MessageType::from_u8(0), Some(MessageType::Ping));
        assert_eq!(MessageType::from_u8(255), Some(MessageType::Error));
        assert_eq!(MessageType::from_u8(99), None);
    }

    #[test]
    fn test_ping_message() {
        let ping = PingMessage::new(42);
        assert_eq!(ping.nonce, 42);
    }

    #[test]
    fn test_pong_message() {
        let ping = PingMessage::new(42);
        let pong = PongMessage::respond_to(&ping);
        assert_eq!(pong.nonce, ping.nonce);
    }

    #[test]
    fn test_vote_validation() {
        let vote = VoteMessage::new(
            "validator1".to_string(),
            1,
            vec![1, 2, 3],
            vec![1, 2, 3],
        );
        assert!(vote.validate().is_ok());
    }

    #[test]
    fn test_vote_validation_empty_validator() {
        let vote = VoteMessage::new(
            "".to_string(),
            1,
            vec![1, 2, 3],
            vec![1, 2, 3],
        );
        assert!(vote.validate().is_err());
    }

    #[test]
    fn test_block_validation() {
        let block = BlockMessage::new(
            1,
            vec![1, 2, 3],
            vec![0],
            "validator1".to_string(),
            vec![],
            vec![1, 2, 3],
        );
        assert!(block.validate().is_ok());
    }

    #[test]
    fn test_transaction_creation() {
        let tx = TransactionMessage::new(
            "alice".to_string(),
            "bob".to_string(),
            100,
            1,
            vec![1, 2, 3],
        );
        assert_eq!(tx.amount, 100);
        assert!(!tx.tx_hash.is_empty());
    }

    #[test]
    fn test_transaction_validation() {
        let tx = TransactionMessage::new(
            "alice".to_string(),
            "bob".to_string(),
            100,
            1,
            vec![1, 2, 3],
        );
        assert!(tx.validate().is_ok());
    }

    #[test]
    fn test_transaction_validation_zero_amount() {
        let tx = TransactionMessage {
            tx_hash: vec![],
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 0,
            nonce: 1,
            signature: vec![1, 2, 3],
            timestamp: 0,
        };
        assert!(tx.validate().is_err());
    }

    #[test]
    fn test_certificate_creation() {
        let cert = CertificateMessage::new(vec![1, 2, 3], 1);
        assert_eq!(cert.voter_count, 0);
    }

    #[test]
    fn test_certificate_add_signature() {
        let mut cert = CertificateMessage::new(vec![1, 2, 3], 1);
        cert.add_signature(vec![1, 2, 3]);
        assert_eq!(cert.voter_count, 1);
    }

    #[test]
    fn test_state_sync_validation() {
        let sync = StateSyncMessage::new(0, 100, vec![1, 2, 3]);
        assert!(sync.validate().is_ok());
    }

    #[test]
    fn test_state_sync_invalid_range() {
        let sync = StateSyncMessage::new(100, 50, vec![1, 2, 3]);
        assert!(sync.validate().is_err());
    }

    #[test]
    fn test_heartbeat_message() {
        let hb = HeartBeatMessage::new("peer1".to_string(), 100);
        assert_eq!(hb.block_height, 100);
    }

    #[test]
    fn test_protocol_message_serialization() {
        let ping = PingMessage::new(42);
        let payload = bincode::serialize(&ping).unwrap();
        let msg = ProtocolMessage::new(MessageType::Ping, payload);
        
        let serialized = msg.serialize().unwrap();
        let deserialized = ProtocolMessage::deserialize(&serialized).unwrap();
        
        assert_eq!(msg.msg_type, deserialized.msg_type);
    }

    #[test]
    fn test_protocol_message_validation() {
        let ping = PingMessage::new(42);
        let payload = bincode::serialize(&ping).unwrap();
        let msg = ProtocolMessage::new(MessageType::Ping, payload);
        
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_protocol_message_version_check() {
        let mut msg = ProtocolMessage::new(MessageType::Ping, vec![]);
        msg.version = 999;
        
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_error_message() {
        let err = ErrorMessage::new(500, "Internal error".to_string());
        assert_eq!(err.error_code, 500);
    }

    #[test]
    fn test_peer_discovery_message() {
        let msg = PeerDiscoveryMessage::new(vec![1, 2], vec![3, 4]);
        assert_eq!(msg.requester_id, vec![1, 2]);
        assert_eq!(msg.target_id, vec![3, 4]);
    }
}
