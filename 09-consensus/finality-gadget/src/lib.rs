// etrid-networking/finality-gadget/src/lib.rs
// LAYER 1: Consensus Protocol - Network Bridge Interface
// Status: Simplified for Checkpoint BFT (V17)
//
// This module provides the network interface types for consensus.
// HotStuff-specific logic has been removed in favor of checkpoint-bft.

use codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

// ============================================================================
// CORE TYPES (kept for P2P compatibility)
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct ValidatorId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct BlockHash([u8; 32]);

impl BlockHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        BlockHash(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct View(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Vote {
    pub validator_id: ValidatorId,
    pub view: View,
    pub block_hash: BlockHash,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Certificate {
    pub view: View,
    pub block_hash: BlockHash,
    pub signatures: Vec<(ValidatorId, Vec<u8>)>,
    pub timestamp: u64,
}

// ============================================================================
// NETWORK BRIDGE TRAIT (Interface to Layer 2 P2P)
// ============================================================================

#[async_trait::async_trait]
pub trait NetworkBridge: Send + Sync {
    async fn broadcast_vote(&self, vote: Vote) -> Result<(), String>;
    async fn broadcast_certificate(&self, cert: Certificate) -> Result<(), String>;
    async fn get_connected_peers(&self) -> Vec<String>;

    // V17: Checkpoint BFT additions
    async fn broadcast_checkpoint_signature(&self, signature: Vec<u8>) -> Result<(), String>;
    async fn broadcast_checkpoint_certificate(&self, certificate: Vec<u8>) -> Result<(), String>;
}

// ============================================================================
// REPUTATION SYSTEM (kept for network quality management)
// ============================================================================

use std::time::Instant;

#[derive(Clone, Debug)]
pub struct PeerReputation {
    pub valid_messages: u32,
    pub invalid_messages: u32,
    pub timeout_responses: u32,
    pub timeout_count: u32,
    pub last_update: Instant,
}

impl PeerReputation {
    pub fn new() -> Self {
        Self {
            valid_messages: 0,
            invalid_messages: 0,
            timeout_responses: 0,
            timeout_count: 0,
            last_update: Instant::now(),
        }
    }

    pub fn score(&self) -> f32 {
        let valid = self.valid_messages as f32;
        let invalid = self.invalid_messages as f32;
        let timeout = self.timeout_count as f32;

        (valid - invalid * 2.0 - timeout * 3.0).max(0.0)
    }

    pub fn record_valid(&mut self) {
        self.valid_messages += 1;
        self.last_update = Instant::now();
    }

    pub fn record_invalid(&mut self) {
        self.invalid_messages += 1;
        self.last_update = Instant::now();
    }

    pub fn record_timeout(&mut self) {
        self.timeout_count += 1;
        self.last_update = Instant::now();
    }

    pub fn is_reliable(&self) -> bool {
        self.score() > 50.0 && self.timeout_count < 10
    }

    pub fn should_isolate(&self) -> bool {
        self.score() < -50.0 || self.timeout_count > 20
    }
}

// ============================================================================
// NOTE: HotStuff-specific code removed in V17
// ============================================================================
//
// The following components have been removed:
// - ViewChangeTimer and NewViewMessage (view synchronization)
// - BlockProposal and leader election
// - CertificateGossip (3-consecutive finality detection)
// - GossipScheduler (exponential backoff)
// - FinalityGadget (HotStuff orchestration)
// - VoteCollector (HotStuff quorum detection)
//
// Replaced by: checkpoint-bft module (09-consensus/checkpoint-bft)
//
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash_creation() {
        let bytes = [1u8; 32];
        let hash = BlockHash::from_bytes(bytes);
        assert_eq!(hash.as_bytes(), &bytes);
    }

    #[test]
    fn test_peer_reputation_scoring() {
        let mut rep = PeerReputation::new();
        assert_eq!(rep.score(), 0.0);

        rep.record_valid();
        assert!(rep.score() > 0.0);

        rep.record_invalid();
        rep.record_invalid();
        assert!(rep.score() < 0.0);
    }

    #[test]
    fn test_peer_reputation_isolation() {
        let mut rep = PeerReputation::new();
        assert!(!rep.should_isolate());

        // Record many invalid messages
        for _ in 0..30 {
            rep.record_invalid();
        }
        assert!(rep.should_isolate());
    }
}
