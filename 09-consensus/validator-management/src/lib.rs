//! # Validator Management
//!
//! This module provides validator orchestration for Ëtrid's FODDoS ASF consensus.
//! It manages:
//!
//! - Committee membership (PPFA panels)
//! - Validator networking and peering
//! - Reward distribution and slashing
//! - Network health monitoring
//! - Validator state synchronization
//!
//! This is the "coordinator" layer between core consensus logic and networking.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

// Re-export asf-algorithm types
pub use asf_algorithm::{
    Balance, BlockNumber, ConsensusPhase, FinalityLevel, Hash, ValidatorId,
};

pub mod committee;
pub mod coordinator;
pub mod networking;
pub mod rewards;
pub mod health;
pub mod state_sync;

pub use committee::*;
pub use coordinator::*;
pub use networking::*;
pub use rewards::*;
pub use health::*;
pub use state_sync::*;

// ═══════════════════════════════════════════════════════════════════════════════
// CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Peer types from Ivory Papers
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
pub enum PeerType {
    /// Common Peer - No stake required
    Common,
    /// Common Stake Peer - Minimum 1 ËTR
    StakingCommon,
    /// Validity Node - Minimum 64 ËTR (PBC validator)
    ValidityNode,
    /// Flare Node - FlareChain validator
    FlareNode,
    /// Decentralized Director - Minimum 128 ËTR (governance)
    DecentralizedDirector,
}

impl PeerType {
    /// Get minimum stake required for this peer type
    pub fn min_stake(&self) -> Balance {
        match self {
            PeerType::Common => 0,
            PeerType::StakingCommon => 1_000_000_000_000_000_000_000, // 1 ËTR
            PeerType::ValidityNode => 64_000_000_000_000_000_000_000, // 64 ËTR
            PeerType::FlareNode => 64_000_000_000_000_000_000_000,    // 64 ËTR
            PeerType::DecentralizedDirector => 128_000_000_000_000_000_000_000, // 128 ËTR
        }
    }

    /// Check if this peer type can be a validator
    pub fn is_validator_type(&self) -> bool {
        matches!(self, PeerType::ValidityNode | PeerType::FlareNode)
    }

    /// Check if this peer type can be in PPFA committee
    pub fn can_be_in_committee(&self) -> bool {
        self.is_validator_type()
    }
}

/// Validator information
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct ValidatorInfo {
    /// Validator account ID
    pub id: ValidatorId,
    
    /// Staked amount
    pub stake: Balance,
    
    /// Reputation score (0-100)
    pub reputation: u64,
    
    /// Peer type classification
    pub peer_type: PeerType,
    
    /// Last block produced
    pub last_block: BlockNumber,
    
    /// Active status
    pub active: bool,
    
    /// Last active epoch
    pub last_epoch: u32,
    
    /// Total blocks produced
    pub blocks_produced: u32,
    
    /// Total certificates issued
    pub certificates_issued: u32,
    
    /// Network address (for peering)
    #[codec(skip)]
    pub network_address: Option<alloc::string::String>,
}

impl ValidatorInfo {
    /// Create new validator info
    pub fn new(id: ValidatorId, stake: Balance, peer_type: PeerType) -> Self {
        Self {
            id,
            stake,
            reputation: 100, // Start with perfect reputation
            peer_type,
            last_block: 0,
            active: true,
            last_epoch: 0,
            blocks_produced: 0,
            certificates_issued: 0,
            network_address: None,
        }
    }

    /// Check if validator can participate in consensus
    pub fn can_participate(&self) -> bool {
        self.active && self.stake >= self.peer_type.min_stake()
    }

    /// Update reputation score
    pub fn update_reputation(&mut self, delta: i64) {
        if delta < 0 {
            self.reputation = self.reputation.saturating_sub(delta.unsigned_abs());
        } else {
            self.reputation = self.reputation.saturating_add(delta as u64).min(100);
        }
    }
}

/// Committee member information
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct CommitteeMember {
    /// Validator ID
    pub validator: ValidatorId,
    
    /// Stake weight in committee
    pub stake: Balance,
    
    /// Index in PPFA panel (0-based)
    pub ppfa_index: u32,
    
    /// Epoch when joined committee
    pub joined_epoch: u32,
}

/// Validator statistics
#[derive(Debug, Clone, Default)]
pub struct ValidatorStats {
    /// Total validators registered
    pub total_validators: u32,
    
    /// Active validators
    pub active_validators: u32,
    
    /// Current committee size
    pub committee_size: u32,
    
    /// Average reputation
    pub avg_reputation: u64,
    
    /// Total stake
    pub total_stake: Balance,
    
    /// Average stake per validator
    pub avg_stake: Balance,
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Validator management errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum ValidatorError {
    /// Validator not found
    #[cfg_attr(feature = "std", error("Validator not found"))]
    NotFound,

    /// Not in committee
    #[cfg_attr(feature = "std", error("Validator not in current committee"))]
    NotInCommittee,

    /// Insufficient stake
    #[cfg_attr(feature = "std", error("Insufficient stake: need {need}, got {got}"))]
    InsufficientStake { need: Balance, got: Balance },

    /// Invalid peer type
    #[cfg_attr(feature = "std", error("Invalid peer type for operation"))]
    InvalidPeerType,

    /// Already in committee
    #[cfg_attr(feature = "std", error("Validator already in committee"))]
    AlreadyInCommittee,

    /// Committee full
    #[cfg_attr(feature = "std", error("Committee is full"))]
    CommitteeFull,

    /// Validator inactive
    #[cfg_attr(feature = "std", error("Validator is inactive"))]
    Inactive,

    /// Network error
    #[cfg_attr(feature = "std", error("Network error: {0}"))]
    NetworkError(&'static str),

    /// Reward calculation error
    #[cfg_attr(feature = "std", error("Reward calculation error"))]
    RewardError,

    /// Invalid epoch
    #[cfg_attr(feature = "std", error("Invalid epoch"))]
    InvalidEpoch,
}

/// Result type for validator operations
pub type ValidatorResult<T> = Result<T, ValidatorError>;

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Maximum committee size (21 validators per PPFA panel)
pub const MAX_COMMITTEE_SIZE: u32 = 21;

/// Minimum committee size for safety
pub const MIN_COMMITTEE_SIZE: u32 = 4;

/// Reputation threshold for committee selection (0-100)
pub const MIN_REPUTATION_FOR_COMMITTEE: u64 = 50;

/// Maximum validators that can be tracked
pub const MAX_VALIDATORS: usize = 1000;

/// Default epoch duration in blocks (2400 blocks ≈ 4 hours at 6s)
pub const EPOCH_DURATION: u32 = 2400;

/// Health check interval in blocks
pub const HEALTH_CHECK_INTERVAL: u32 = 100;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_type_min_stake() {
        assert_eq!(PeerType::Common.min_stake(), 0);
        assert_eq!(PeerType::StakingCommon.min_stake(), 1_000_000_000_000_000_000_000);
        assert_eq!(PeerType::ValidityNode.min_stake(), 64_000_000_000_000_000_000_000);
        assert_eq!(PeerType::DecentralizedDirector.min_stake(), 128_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_peer_type_validator_check() {
        assert!(!PeerType::Common.is_validator_type());
        assert!(!PeerType::StakingCommon.is_validator_type());
        assert!(PeerType::ValidityNode.is_validator_type());
        assert!(PeerType::FlareNode.is_validator_type());
        assert!(!PeerType::DecentralizedDirector.is_validator_type());
    }

    #[test]
    fn test_validator_info_creation() {
        let id = ValidatorId::from([1u8; 32]);
        let stake = 100_000_000_000_000_000_000_000;
        let info = ValidatorInfo::new(id.clone(), stake, PeerType::ValidityNode);

        assert_eq!(info.id, id);
        assert_eq!(info.stake, stake);
        assert_eq!(info.reputation, 100);
        assert!(info.can_participate());
    }

    #[test]
    fn test_validator_reputation_update() {
        let id = ValidatorId::from([1u8; 32]);
        let mut info = ValidatorInfo::new(id, 1000, PeerType::ValidityNode);

        info.update_reputation(-20);
        assert_eq!(info.reputation, 80);

        info.update_reputation(30);
        assert_eq!(info.reputation, 100); // Capped at 100

        info.update_reputation(-150);
        assert_eq!(info.reputation, 0); // Floored at 0
    }

    #[test]
    fn test_validator_participation() {
        let id = ValidatorId::from([1u8; 32]);
        
        // Sufficient stake
        let mut info = ValidatorInfo::new(id.clone(), 64_000_000_000_000_000_000_000, PeerType::ValidityNode);
        assert!(info.can_participate());

        // Insufficient stake
        info.stake = 1000;
        assert!(!info.can_participate());

        // Inactive
        info.stake = 64_000_000_000_000_000_000_000;
        info.active = false;
        assert!(!info.can_participate());
    }

    #[test]
    fn test_committee_member() {
        let id = ValidatorId::from([1u8; 32]);
        let member = CommitteeMember {
            validator: id.clone(),
            stake: 100_000,
            ppfa_index: 5,
            joined_epoch: 10,
        };

        assert_eq!(member.validator, id);
        assert_eq!(member.ppfa_index, 5);
        assert_eq!(member.joined_epoch, 10);
    }
}