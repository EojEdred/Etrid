//! # Block Production
//!
//! This module handles block authoring and production for Ëtrid's FODDoS ASF consensus.
//! It implements:
//!
//! - PPFA (Proposing Panel for Attestation) proposer selection
//! - Adaptive slot timing based on network health
//! - Main block (Queen) and secondary block (Ant) creation
//! - Transaction selection and block building
//! - Block validation before proposing
//!
//! The block production flow:
//! 1. Check if we're the current PPFA proposer
//! 2. Calculate adaptive slot timing
//! 3. Build block with transactions
//! 4. Validate block
//! 5. Propose to network
//! 6. Handle secondary blocks (Ants) if needed

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

// Re-export types from dependencies
pub use asf_algorithm::{
    Balance, BlockNumber, Certificate, FinalityLevel, Hash, ValidatorId, Vote,
};
pub use validator_management::{
    CommitteeManager, HealthMonitor, NetworkManager, RewardsManager, ValidatorInfo,
};

pub mod proposer;
pub mod author;
pub mod slot_timing;
pub mod ant_handler;
pub mod validation;

pub use proposer::*;
pub use author::*;
pub use slot_timing::*;
pub use ant_handler::*;
pub use validation::*;

// ═══════════════════════════════════════════════════════════════════════════════
// CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Block type classification
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    TypeInfo,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BlockType {
    /// Main block (Queen) - proposed by PPFA validator
    Queen,
    
    /// Secondary block (Ant) - proposed when main block fails
    Ant,
}

/// Block header
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct BlockHeader {
    /// Block number
    pub number: BlockNumber,
    
    /// Parent block hash
    pub parent_hash: Hash,
    
    /// State root (transactions applied)
    pub state_root: Hash,
    
    /// Extrinsics root
    pub extrinsics_root: Hash,
    
    /// Block type (Queen or Ant)
    pub block_type: BlockType,
    
    /// Proposer validator ID
    pub proposer: ValidatorId,
    
    /// PPFA index at time of proposal
    pub ppfa_index: u32,
    
    /// Timestamp (milliseconds since epoch)
    pub timestamp: u64,
    
    /// Epoch number
    pub epoch: u32,
}

/// Block body with transactions
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct BlockBody {
    /// Transactions (extrinsics)
    pub transactions: Vec<Vec<u8>>, // Encoded transactions
}

/// Complete block
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    
    /// Block body
    pub body: BlockBody,
}

impl Block {
    /// Create a new block
    pub fn new(header: BlockHeader, body: BlockBody) -> Self {
        Self { header, body }
    }

    /// Get block hash
    pub fn hash(&self) -> Hash {
        use sp_core::hashing::blake2_256;
        Hash::from(blake2_256(&self.header.encode()))
    }

    /// Get block number
    pub fn number(&self) -> BlockNumber {
        self.header.number
    }

    /// Get parent hash
    pub fn parent_hash(&self) -> Hash {
        self.header.parent_hash
    }

    /// Get proposer
    pub fn proposer(&self) -> &ValidatorId {
        &self.header.proposer
    }

    /// Check if this is a Queen block
    pub fn is_queen(&self) -> bool {
        self.header.block_type == BlockType::Queen
    }

    /// Check if this is an Ant block
    pub fn is_ant(&self) -> bool {
        self.header.block_type == BlockType::Ant
    }

    /// Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.body.transactions.len()
    }
}

/// Block proposal
#[derive(Debug, Clone, Encode, Decode)]
pub struct BlockProposal {
    /// The proposed block
    pub block: Block,
    
    /// Proposer signature
    pub signature: Vec<u8>,
    
    /// Certificate from previous block (if any)
    pub parent_certificate: Option<Certificate>,
}

impl BlockProposal {
    /// Create a new block proposal
    pub fn new(block: Block, signature: Vec<u8>) -> Self {
        Self {
            block,
            signature,
            parent_certificate: None,
        }
    }

    /// Add parent certificate
    pub fn with_certificate(mut self, certificate: Certificate) -> Self {
        self.parent_certificate = Some(certificate);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLOT INFORMATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Slot information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotInfo {
    /// Slot number
    pub slot: u64,
    
    /// Slot duration (milliseconds)
    pub duration: u64,
    
    /// Expected proposer PPFA index
    pub ppfa_index: u32,
    
    /// Expected proposer validator ID
    pub proposer: ValidatorId,
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSACTION SELECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Transaction selection strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStrategy {
    /// Select highest priority transactions first
    HighestPriority,
    
    /// Select oldest transactions first (FIFO)
    OldestFirst,
    
    /// Select by highest fee
    HighestFee,
}

/// Transaction priority
pub type TransactionPriority = u64;

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Block production errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum BlockProductionError {
    /// Not the current proposer
    #[cfg_attr(feature = "std", error("Not the current PPFA proposer"))]
    NotProposer,

    /// Committee not initialized
    #[cfg_attr(feature = "std", error("Committee not initialized"))]
    NoCommittee,

    /// Invalid slot
    #[cfg_attr(feature = "std", error("Invalid slot timing"))]
    InvalidSlot,

    /// No transactions available
    #[cfg_attr(feature = "std", error("No transactions in pool"))]
    NoTransactions,

    /// Block validation failed
    #[cfg_attr(feature = "std", error("Block validation failed: {0}"))]
    ValidationFailed(&'static str),

    /// Parent block not found
    #[cfg_attr(feature = "std", error("Parent block not found"))]
    ParentNotFound,

    /// Certificate required but missing
    #[cfg_attr(feature = "std", error("Parent certificate required"))]
    CertificateRequired,

    /// Signature error
    #[cfg_attr(feature = "std", error("Signature error"))]
    SignatureError,

    /// Network health too low
    #[cfg_attr(feature = "std", error("Network health too low for block production"))]
    NetworkUnhealthy,

    /// Too many Ants
    #[cfg_attr(feature = "std", error("Maximum Ant blocks reached for this slot"))]
    TooManyAnts,
}

/// Result type for block production
pub type BlockProductionResult<T> = Result<T, BlockProductionError>;

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Base slot duration (6 seconds = 6000 milliseconds)
pub const BASE_SLOT_DURATION: u64 = 6000;

/// Maximum transactions per block
pub const MAX_TRANSACTIONS_PER_BLOCK: usize = 1000;

/// Maximum block size (bytes)
pub const MAX_BLOCK_SIZE: usize = 5_000_000; // 5 MB

/// Maximum Ant blocks per slot
pub const MAX_ANTS_PER_SLOT: u32 = 3;

/// Block production timeout (milliseconds)
pub const BLOCK_PRODUCTION_TIMEOUT: u64 = 5000; // 5 seconds

// ═══════════════════════════════════════════════════════════════════════════════
// STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Block production statistics
#[derive(Debug, Clone, Default)]
pub struct ProductionStats {
    /// Total blocks produced
    pub total_blocks: u64,
    
    /// Queen blocks produced
    pub queen_blocks: u64,
    
    /// Ant blocks produced
    pub ant_blocks: u64,
    
    /// Average transactions per block
    pub avg_transactions: u64,
    
    /// Average block production time (milliseconds)
    pub avg_production_time: u64,
    
    /// Failed proposals
    pub failed_proposals: u64,
    
    /// Missed slots
    pub missed_slots: u64,
}

impl ProductionStats {
    /// Record a successful block
    pub fn record_block(&mut self, block: &Block, production_time: u64) {
        self.total_blocks += 1;
        
        match block.header.block_type {
            BlockType::Queen => self.queen_blocks += 1,
            BlockType::Ant => self.ant_blocks += 1,
        }
        
        // Update average transactions
        let tx_count = block.transaction_count() as u64;
        self.avg_transactions = (self.avg_transactions * (self.total_blocks - 1) + tx_count) / self.total_blocks;
        
        // Update average production time
        self.avg_production_time = (self.avg_production_time * (self.total_blocks - 1) + production_time) / self.total_blocks;
    }

    /// Record a failed proposal
    pub fn record_failure(&mut self) {
        self.failed_proposals += 1;
    }

    /// Record a missed slot
    pub fn record_missed_slot(&mut self) {
        self.missed_slots += 1;
    }

    /// Get success rate (0-100)
    pub fn success_rate(&self) -> u8 {
        let total_attempts = self.total_blocks + self.failed_proposals;
        if total_attempts == 0 {
            return 100;
        }
        ((self.total_blocks * 100) / total_attempts) as u8
    }

    /// Get Queen block percentage
    pub fn queen_percentage(&self) -> u8 {
        if self.total_blocks == 0 {
            return 0;
        }
        ((self.queen_blocks * 100) / self.total_blocks) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_header(number: u64) -> BlockHeader {
        BlockHeader {
            number,
            parent_hash: Hash::default(),
            state_root: Hash::default(),
            extrinsics_root: Hash::default(),
            block_type: BlockType::Queen,
            proposer: ValidatorId::from([1u8; 32]),
            ppfa_index: 0,
            timestamp: 0,
            epoch: 0,
        }
    }

    fn create_test_block(number: u64) -> Block {
        let header = create_test_header(number);
        let body = BlockBody {
            transactions: Vec::new(),
        };
        Block::new(header, body)
    }

    #[test]
    fn test_block_creation() {
        let block = create_test_block(1);
        assert_eq!(block.number(), 1);
        assert!(block.is_queen());
        assert!(!block.is_ant());
    }

    #[test]
    fn test_block_hash() {
        let block1 = create_test_block(1);
        let block2 = create_test_block(1);
        
        assert_eq!(block1.hash(), block2.hash()); // Same block = same hash
    }

    #[test]
    fn test_block_type() {
        let mut block = create_test_block(1);
        assert_eq!(block.header.block_type, BlockType::Queen);
        
        block.header.block_type = BlockType::Ant;
        assert_eq!(block.header.block_type, BlockType::Ant);
        assert!(block.is_ant());
    }

    #[test]
    fn test_transaction_count() {
        let mut block = create_test_block(1);
        assert_eq!(block.transaction_count(), 0);
        
        block.body.transactions.push(vec![1, 2, 3]);
        block.body.transactions.push(vec![4, 5, 6]);
        assert_eq!(block.transaction_count(), 2);
    }

    #[test]
    fn test_block_proposal() {
        let block = create_test_block(1);
        let signature = vec![0u8; 64];
        let proposal = BlockProposal::new(block, signature);
        
        assert_eq!(proposal.block.number(), 1);
        assert!(proposal.parent_certificate.is_none());
    }

    #[test]
    fn test_production_stats() {
        let mut stats = ProductionStats::default();
        
        let block = create_test_block(1);
        stats.record_block(&block, 1000);
        
        assert_eq!(stats.total_blocks, 1);
        assert_eq!(stats.queen_blocks, 1);
        assert_eq!(stats.ant_blocks, 0);
        assert_eq!(stats.success_rate(), 100);
    }

    #[test]
    fn test_success_rate() {
        let mut stats = ProductionStats::default();
        
        let block = create_test_block(1);
        stats.record_block(&block, 1000);
        stats.record_failure();
        
        assert_eq!(stats.success_rate(), 50); // 1 success, 1 failure = 50%
    }

    #[test]
    fn test_queen_percentage() {
        let mut stats = ProductionStats::default();
        
        let mut queen_block = create_test_block(1);
        queen_block.header.block_type = BlockType::Queen;
        stats.record_block(&queen_block, 1000);
        
        let mut ant_block = create_test_block(2);
        ant_block.header.block_type = BlockType::Ant;
        stats.record_block(&ant_block, 1000);
        
        assert_eq!(stats.queen_percentage(), 50); // 1 Queen, 1 Ant = 50%
    }
}
