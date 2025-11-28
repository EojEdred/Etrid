//! # ASF Collator Consensus
//!
//! Adapts ASF (Ascending Scale of Finality) for PBC collator consensus.
//!
//! Key differences from relay chain ASF:
//! - Smaller collator committees (7-11 collators vs 21+ validators)
//! - Modified finality thresholds for parachain context
//! - Relay chain coordination and finality inheritance
//! - Fast rotation (every 6 blocks)
//! - Cross-chain finality attestations for bridge security

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

pub mod collator_rotation;
pub mod finality_inheritance;
pub mod cross_chain_attestations;
pub mod committee_management;

pub use collator_rotation::*;
pub use finality_inheritance::*;
pub use cross_chain_attestations::*;
pub use committee_management::*;

// Re-export ASF core types
pub use asf_algorithm::{
    ConsensusPhase, FinalityLevel, Hash, BlockNumber, Balance, ValidatorId,
    AsfError, AsfResult, bft_threshold, bft_stake_threshold,
};

/// Type alias for collators (same as validator IDs)
pub type CollatorId = ValidatorId;

/// Parachain ID type
pub type ParaId = u32;

// ═══════════════════════════════════════════════════════════════════════════════
// COLLATOR-SPECIFIC TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Collator committee configuration
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CollatorCommittee {
    /// Parachain ID this committee serves
    pub para_id: ParaId,
    /// Active collators in this committee
    pub collators: Vec<CollatorId>,
    /// Minimum collators required
    pub min_collators: u32,
    /// Maximum collators allowed
    pub max_collators: u32,
    /// Current rotation round
    pub rotation_round: u64,
    /// Total stake in committee
    pub total_stake: Balance,
}

impl CollatorCommittee {
    /// Create new collator committee
    pub fn new(para_id: ParaId, min: u32, max: u32) -> Self {
        Self {
            para_id,
            collators: Vec::new(),
            min_collators: min,
            max_collators: max,
            rotation_round: 0,
            total_stake: 0,
        }
    }

    /// Check if committee has minimum collators
    pub fn is_valid(&self) -> bool {
        self.collators.len() >= self.min_collators as usize
    }

    /// Get BFT threshold for this committee
    pub fn bft_threshold(&self) -> u32 {
        bft_threshold(self.collators.len() as u32)
    }

    /// Add collator to committee
    pub fn add_collator(&mut self, collator: CollatorId, stake: Balance) -> AsfResult<()> {
        if self.collators.len() >= self.max_collators as usize {
            return Err(AsfError::InvalidVote("Committee full"));
        }

        if self.collators.contains(&collator) {
            return Err(AsfError::DuplicateVote);
        }

        self.collators.push(collator);
        self.total_stake += stake;
        Ok(())
    }

    /// Remove collator from committee
    pub fn remove_collator(&mut self, collator: &CollatorId, stake: Balance) -> AsfResult<()> {
        let pos = self.collators.iter().position(|c| c == collator)
            .ok_or(AsfError::BlockNotFound)?;

        self.collators.remove(pos);
        self.total_stake = self.total_stake.saturating_sub(stake);
        Ok(())
    }

    /// Increment rotation round
    pub fn next_round(&mut self) {
        self.rotation_round += 1;
    }
}

/// Collator-specific finality thresholds (lower than relay chain)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum CollatorFinalityLevel {
    /// Not finalized (0-4 certificates)
    None = 0,
    /// Weak finality (5-9 certificates)
    Weak = 1,
    /// Moderate finality (10-19 certificates)
    Moderate = 2,
    /// Strong finality (20-49 certificates)
    Strong = 3,
    /// Irreversible finality (50+ certificates OR relay chain finalized)
    Irreversible = 4,
}

impl From<u32> for CollatorFinalityLevel {
    fn from(count: u32) -> Self {
        match count {
            0..=4 => CollatorFinalityLevel::None,
            5..=9 => CollatorFinalityLevel::Weak,
            10..=19 => CollatorFinalityLevel::Moderate,
            20..=49 => CollatorFinalityLevel::Strong,
            _ => CollatorFinalityLevel::Irreversible,
        }
    }
}

impl CollatorFinalityLevel {
    /// Get minimum certificates for this level
    pub fn min_certificates(&self) -> u32 {
        match self {
            CollatorFinalityLevel::None => 0,
            CollatorFinalityLevel::Weak => 5,
            CollatorFinalityLevel::Moderate => 10,
            CollatorFinalityLevel::Strong => 20,
            CollatorFinalityLevel::Irreversible => 50,
        }
    }

    /// Check if finalized
    pub fn is_finalized(&self) -> bool {
        *self > CollatorFinalityLevel::None
    }
}

/// Relay chain finality proof for parachain blocks
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct RelayChainFinalityProof {
    /// Relay chain block number
    pub relay_block: BlockNumber,
    /// Relay chain block hash
    pub relay_hash: Hash,
    /// Parachain block included in relay chain
    pub para_block: BlockNumber,
    /// Parachain block hash
    pub para_hash: Hash,
    /// Relay chain finality level
    pub relay_finality: FinalityLevel,
}

impl RelayChainFinalityProof {
    /// Check if relay chain has finalized this parachain block
    pub fn is_finalized(&self) -> bool {
        self.relay_finality.is_finalized()
    }

    /// Inherit finality from relay chain
    pub fn inherit_finality(&self) -> CollatorFinalityLevel {
        if self.relay_finality >= FinalityLevel::Strong {
            CollatorFinalityLevel::Irreversible
        } else if self.relay_finality >= FinalityLevel::Moderate {
            CollatorFinalityLevel::Strong
        } else if self.relay_finality >= FinalityLevel::Weak {
            CollatorFinalityLevel::Moderate
        } else {
            CollatorFinalityLevel::None
        }
    }
}

/// Collator block proposal with ASF consensus data
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CollatorProposal {
    /// Parachain ID
    pub para_id: ParaId,
    /// Block hash
    pub block_hash: Hash,
    /// Block number
    pub block_number: BlockNumber,
    /// Proposing collator
    pub collator: CollatorId,
    /// Current consensus phase
    pub phase: ConsensusPhase,
    /// Rotation round
    pub rotation_round: u64,
    /// Relay chain parent hash
    pub relay_parent: Hash,
}

/// Collator vote (extends ASF Vote for collator context)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CollatorVote {
    /// Parachain ID
    pub para_id: ParaId,
    /// Block being voted on
    pub block_hash: Hash,
    /// Block number
    pub block_number: BlockNumber,
    /// Voting collator
    pub collator: CollatorId,
    /// Consensus phase
    pub phase: ConsensusPhase,
    /// Rotation round
    pub rotation_round: u64,
    /// Collator signature
    pub signature: Vec<u8>,
}

impl CollatorVote {
    /// Create new collator vote
    pub fn new(
        para_id: ParaId,
        block_hash: Hash,
        block_number: BlockNumber,
        collator: CollatorId,
        phase: ConsensusPhase,
        rotation_round: u64,
    ) -> Self {
        Self {
            para_id,
            block_hash,
            block_number,
            collator,
            phase,
            rotation_round,
            signature: Vec::new(),
        }
    }

    /// Get vote hash for signing
    pub fn vote_hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&self.para_id.to_le_bytes());
        data.extend_from_slice(self.block_hash.as_ref());
        data.extend_from_slice(&self.block_number.to_le_bytes());
        data.extend_from_slice(self.collator.as_ref());
        data.push(self.phase as u8);
        data.extend_from_slice(&self.rotation_round.to_le_bytes());
        BlakeTwo256::hash(&data)
    }
}

/// Collator validity certificate
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CollatorCertificate {
    /// Parachain ID
    pub para_id: ParaId,
    /// Block hash
    pub block_hash: Hash,
    /// Block number
    pub block_number: BlockNumber,
    /// Consensus phase this certificate represents
    pub phase: ConsensusPhase,
    /// Rotation round
    pub rotation_round: u64,
    /// Votes that form this certificate
    pub votes: Vec<CollatorVote>,
    /// Total stake weight
    pub stake_weight: Balance,
}

impl CollatorCertificate {
    /// Create empty certificate
    pub fn new(
        para_id: ParaId,
        block_hash: Hash,
        block_number: BlockNumber,
        phase: ConsensusPhase,
        rotation_round: u64,
    ) -> Self {
        Self {
            para_id,
            block_hash,
            block_number,
            phase,
            rotation_round,
            votes: Vec::new(),
            stake_weight: 0,
        }
    }

    /// Add vote to certificate
    pub fn add_vote(&mut self, vote: CollatorVote, stake: Balance) -> AsfResult<()> {
        // Verify vote matches certificate
        if vote.para_id != self.para_id
            || vote.block_hash != self.block_hash
            || vote.block_number != self.block_number
            || vote.phase != self.phase
            || vote.rotation_round != self.rotation_round
        {
            return Err(AsfError::InvalidVote("Vote mismatch"));
        }

        // Check for duplicate
        if self.votes.iter().any(|v| v.collator == vote.collator) {
            return Err(AsfError::DuplicateVote);
        }

        self.votes.push(vote);
        self.stake_weight += stake;
        Ok(())
    }

    /// Check if certificate meets BFT threshold
    pub fn is_valid(&self, committee: &CollatorCommittee) -> bool {
        let threshold = committee.bft_threshold();
        self.votes.len() >= threshold as usize
            && self.stake_weight >= bft_stake_threshold(committee.total_stake)
    }

    /// Get certificate hash
    pub fn certificate_hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&self.para_id.to_le_bytes());
        data.extend_from_slice(self.block_hash.as_ref());
        data.extend_from_slice(&self.block_number.to_le_bytes());
        data.push(self.phase as u8);
        data.extend_from_slice(&self.rotation_round.to_le_bytes());
        for vote in &self.votes {
            data.extend_from_slice(&vote.vote_hash().as_ref());
        }
        BlakeTwo256::hash(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    #[test]
    fn test_collator_committee() {
        let mut committee = CollatorCommittee::new(1000, 7, 11);

        let collator1 = AccountId32::new([1u8; 32]);
        let collator2 = AccountId32::new([2u8; 32]);

        assert!(committee.add_collator(collator1.clone(), 100_000).is_ok());
        assert!(committee.add_collator(collator2.clone(), 100_000).is_ok());

        assert_eq!(committee.collators.len(), 2);
        assert_eq!(committee.total_stake, 200_000);

        // Duplicate should fail
        assert!(committee.add_collator(collator1.clone(), 100_000).is_err());
    }

    #[test]
    fn test_collator_finality_levels() {
        assert_eq!(CollatorFinalityLevel::from(3), CollatorFinalityLevel::None);
        assert_eq!(CollatorFinalityLevel::from(7), CollatorFinalityLevel::Weak);
        assert_eq!(CollatorFinalityLevel::from(15), CollatorFinalityLevel::Moderate);
        assert_eq!(CollatorFinalityLevel::from(30), CollatorFinalityLevel::Strong);
        assert_eq!(CollatorFinalityLevel::from(100), CollatorFinalityLevel::Irreversible);
    }

    #[test]
    fn test_collator_bft_threshold() {
        let mut committee = CollatorCommittee::new(1000, 7, 11);

        // Add 7 collators
        for i in 0..7 {
            let collator = AccountId32::new([i as u8; 32]);
            committee.add_collator(collator, 100_000).unwrap();
        }

        // BFT threshold for 7 = (7 * 2) / 3 + 1 = 4 + 1 = 5
        assert_eq!(committee.bft_threshold(), 5);
    }

    #[test]
    fn test_relay_chain_finality_inheritance() {
        let proof = RelayChainFinalityProof {
            relay_block: 1000,
            relay_hash: H256::random(),
            para_block: 500,
            para_hash: H256::random(),
            relay_finality: FinalityLevel::Strong,
        };

        assert_eq!(proof.inherit_finality(), CollatorFinalityLevel::Irreversible);
    }

    #[test]
    fn test_collator_certificate_validation() {
        let mut committee = CollatorCommittee::new(1000, 7, 11);

        // Add 7 collators with equal stake
        for i in 0..7 {
            let collator = AccountId32::new([i as u8; 32]);
            committee.add_collator(collator, 100_000).unwrap();
        }

        let block_hash = H256::random();
        let mut cert = CollatorCertificate::new(
            1000,
            block_hash,
            100,
            ConsensusPhase::Prepare,
            0,
        );

        // Add 5 votes (meets BFT threshold of 5 for 7 collators)
        for i in 0..5 {
            let collator = AccountId32::new([i as u8; 32]);
            let vote = CollatorVote::new(
                1000,
                block_hash,
                100,
                collator,
                ConsensusPhase::Prepare,
                0,
            );
            cert.add_vote(vote, 100_000).unwrap();
        }

        assert!(cert.is_valid(&committee));
    }
}
