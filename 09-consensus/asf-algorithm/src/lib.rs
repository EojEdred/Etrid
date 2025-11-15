//! # ASF (Ascending Scale of Finality) Core Algorithm
//!
//! This crate implements the core consensus logic for Ëtrid's FODDoS ASF protocol
//! as specified in the Ivory Papers. It provides:
//!
//! - HotStuff 4-phase Byzantine consensus (Prepare → PreCommit → Commit → Decide)
//! - Validity certificate generation and aggregation
//! - Ascending scale of finality (5 levels: 0-4)
//! - Stake-weighted voting and BFT threshold calculations
//! - Safety and liveness proofs
//!
//! This is the "brain" of the consensus system - all other modules depend on this.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

pub mod hotstuff;
pub mod certificates;
pub mod votes;
pub mod finality;
pub mod safety;
pub mod ppfa;
pub mod crypto;
pub mod slashing;
pub mod network;

pub use hotstuff::*;
pub use certificates::*;
pub use votes::*;
pub use finality::*;
pub use safety::*;
pub use ppfa::*;
pub use crypto::*;
pub use slashing::*;
pub use network::*;

/// Re-export core types
pub use sp_core::crypto::AccountId32;

/// Type alias for ValidityCertificate for easier imports
pub type Certificate = ValidityCertificate;

// ═══════════════════════════════════════════════════════════════════════════════
// CORE TYPES (From Ivory Papers)
// ═══════════════════════════════════════════════════════════════════════════════

/// Consensus phases in HotStuff protocol
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum ConsensusPhase {
    /// Phase 1: Leader collects highest valid branch and prepare votes
    Prepare,
    /// Phase 2: Prepare certificate broadcast, nodes send pre-commit votes
    PreCommit,
    /// Phase 3: Commit certificate broadcast, replicas lock state
    Commit,
    /// Phase 4: Commit certificate finalized, state transition occurs
    Decide,
}

impl Default for ConsensusPhase {
    fn default() -> Self {
        ConsensusPhase::Prepare
    }
}

impl ConsensusPhase {
    /// Get the next phase in the sequence
    pub fn next(&self) -> Option<Self> {
        match self {
            ConsensusPhase::Prepare => Some(ConsensusPhase::PreCommit),
            ConsensusPhase::PreCommit => Some(ConsensusPhase::Commit),
            ConsensusPhase::Commit => Some(ConsensusPhase::Decide),
            ConsensusPhase::Decide => None,
        }
    }

    /// Check if this phase comes before another
    pub fn precedes(&self, other: &ConsensusPhase) -> bool {
        match (self, other) {
            (ConsensusPhase::Prepare, ConsensusPhase::PreCommit)
            | (ConsensusPhase::Prepare, ConsensusPhase::Commit)
            | (ConsensusPhase::Prepare, ConsensusPhase::Decide)
            | (ConsensusPhase::PreCommit, ConsensusPhase::Commit)
            | (ConsensusPhase::PreCommit, ConsensusPhase::Decide)
            | (ConsensusPhase::Commit, ConsensusPhase::Decide) => true,
            _ => false,
        }
    }
}

/// Finality level (0-4) from Ascending Scale of Finality
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum FinalityLevel {
    /// Not finalized (0-9 certificates)
    None = 0,
    /// Weak finality (10-19 certificates)
    Weak = 1,
    /// Moderate finality (20-49 certificates)
    Moderate = 2,
    /// Strong finality (50-99 certificates)
    Strong = 3,
    /// Irreversible finality (100+ certificates)
    Irreversible = 4,
}

impl From<u32> for FinalityLevel {
    fn from(count: u32) -> Self {
        match count {
            0..=9 => FinalityLevel::None,
            10..=19 => FinalityLevel::Weak,
            20..=49 => FinalityLevel::Moderate,
            50..=99 => FinalityLevel::Strong,
            _ => FinalityLevel::Irreversible,
        }
    }
}

impl FinalityLevel {
    /// Get the minimum certificate count for this level
    pub fn min_certificates(&self) -> u32 {
        match self {
            FinalityLevel::None => 0,
            FinalityLevel::Weak => 10,
            FinalityLevel::Moderate => 20,
            FinalityLevel::Strong => 50,
            FinalityLevel::Irreversible => 100,
        }
    }

    /// Check if block is finalized (level > None)
    pub fn is_finalized(&self) -> bool {
        *self > FinalityLevel::None
    }
}

/// Generic hash type
pub type Hash = H256;

/// Generic block number type
pub type BlockNumber = u64;

/// Generic balance type
pub type Balance = u128;

/// Validator identifier (AccountId)
pub type ValidatorId = AccountId32;

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// ASF consensus errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum AsfError {
    /// Invalid vote received
    #[cfg_attr(feature = "std", error("Invalid vote: {0}"))]
    InvalidVote(&'static str),

    /// Invalid certificate
    #[cfg_attr(feature = "std", error("Invalid certificate: {0}"))]
    InvalidCertificate(&'static str),

    /// Insufficient votes for threshold
    #[cfg_attr(feature = "std", error("Insufficient votes: got {got}, need {need}"))]
    InsufficientVotes { got: u32, need: u32 },

    /// Invalid phase transition
    #[cfg_attr(feature = "std", error("Invalid phase transition from {from:?} to {to:?}"))]
    InvalidPhaseTransition {
        from: ConsensusPhase,
        to: ConsensusPhase,
    },

    /// Duplicate vote from same validator
    #[cfg_attr(feature = "std", error("Duplicate vote from validator"))]
    DuplicateVote,

    /// Not enough stake weight
    #[cfg_attr(feature = "std", error("Insufficient stake: got {got}, need {need}"))]
    InsufficientStake { got: Balance, need: Balance },

    /// Safety violation detected
    #[cfg_attr(feature = "std", error("Safety violation: {0}"))]
    SafetyViolation(&'static str),

    /// Block not found
    #[cfg_attr(feature = "std", error("Block not found"))]
    BlockNotFound,

    /// Invalid signature
    #[cfg_attr(feature = "std", error("Invalid signature"))]
    InvalidSignature,
}

/// Result type for ASF operations
pub type AsfResult<T> = Result<T, AsfError>;

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Calculate BFT threshold (2/3 + 1)
pub fn bft_threshold(total: u32) -> u32 {
    ((total * 2) / 3) + 1
}

/// Calculate BFT threshold for stake-weighted voting
pub fn bft_stake_threshold(total_stake: Balance) -> Balance {
    (total_stake * 2) / 3 + 1
}

/// Check if vote count meets BFT threshold
pub fn meets_threshold(votes: u32, total: u32) -> bool {
    votes >= bft_threshold(total)
}

/// Check if stake weight meets BFT threshold
pub fn meets_stake_threshold(stake: Balance, total_stake: Balance) -> bool {
    stake >= bft_stake_threshold(total_stake)
}

/// Hash a block reference (for vote/certificate generation)
pub fn hash_block_ref(block_hash: &Hash, block_number: BlockNumber) -> Hash {
    let mut data = Vec::new();
    data.extend_from_slice(block_hash.as_ref());
    data.extend_from_slice(&block_number.to_le_bytes());
    BlakeTwo256::hash(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_phase_progression() {
        assert_eq!(
            ConsensusPhase::Prepare.next(),
            Some(ConsensusPhase::PreCommit)
        );
        assert_eq!(
            ConsensusPhase::PreCommit.next(),
            Some(ConsensusPhase::Commit)
        );
        assert_eq!(ConsensusPhase::Commit.next(), Some(ConsensusPhase::Decide));
        assert_eq!(ConsensusPhase::Decide.next(), None);
    }

    #[test]
    fn test_phase_precedence() {
        assert!(ConsensusPhase::Prepare.precedes(&ConsensusPhase::Commit));
        assert!(ConsensusPhase::PreCommit.precedes(&ConsensusPhase::Decide));
        assert!(!ConsensusPhase::Commit.precedes(&ConsensusPhase::Prepare));
    }

    #[test]
    fn test_finality_levels() {
        assert_eq!(FinalityLevel::from(5), FinalityLevel::None);
        assert_eq!(FinalityLevel::from(15), FinalityLevel::Weak);
        assert_eq!(FinalityLevel::from(35), FinalityLevel::Moderate);
        assert_eq!(FinalityLevel::from(75), FinalityLevel::Strong);
        assert_eq!(FinalityLevel::from(150), FinalityLevel::Irreversible);
    }

    #[test]
    fn test_bft_threshold() {
        assert_eq!(bft_threshold(21), 15); // 2/3 of 21 = 14, +1 = 15
        assert_eq!(bft_threshold(10), 7); // 2/3 of 10 = 6.66, +1 = 7
        assert_eq!(bft_threshold(100), 67); // 2/3 of 100 = 66.66, +1 = 67
    }

    #[test]
    fn test_meets_threshold() {
        assert!(meets_threshold(15, 21));
        assert!(!meets_threshold(14, 21));
        assert!(meets_threshold(67, 100));
        assert!(!meets_threshold(66, 100));
    }

    #[test]
    fn test_stake_threshold() {
        let total = 1_000_000u128;
        let threshold = bft_stake_threshold(total);
        assert_eq!(threshold, 666_667); // 2/3 + 1 = (1_000_000 * 2) / 3 + 1 = 666_666 + 1

        assert!(meets_stake_threshold(700_000, total));
        assert!(!meets_stake_threshold(600_000, total));
    }
}