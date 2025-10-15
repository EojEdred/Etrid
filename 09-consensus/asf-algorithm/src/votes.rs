//! # Vote Types and Validation
//!
//! This module defines the vote structure used in HotStuff consensus
//! and provides validation logic for votes.

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    AsfError, AsfResult, Balance, BlockNumber, ConsensusPhase, Hash, ValidatorId,
};

// ═══════════════════════════════════════════════════════════════════════════════
// VOTE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// A vote from a validator in the HotStuff protocol
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct Vote {
    /// Block hash being voted on
    pub block_hash: Hash,
    
    /// Block number
    pub block_number: BlockNumber,
    
    /// Consensus phase this vote is for
    pub phase: ConsensusPhase,
    
    /// Validator who cast this vote
    pub validator: ValidatorId,
    
    /// Stake weight of the validator
    pub stake_weight: Balance,
    
    /// Epoch when vote was cast
    pub epoch: u32,
    
    /// Timestamp (Unix milliseconds)
    pub timestamp: u64,
    
    /// Signature (placeholder - in production this would be a real signature)
    pub signature: [u8; 64],
}

impl Vote {
    /// Create a new vote
    pub fn new(
        block_hash: Hash,
        block_number: BlockNumber,
        phase: ConsensusPhase,
        validator: ValidatorId,
        stake_weight: Balance,
        epoch: u32,
        timestamp: u64,
    ) -> Self {
        Self {
            block_hash,
            block_number,
            phase,
            validator,
            stake_weight,
            epoch,
            timestamp,
            signature: [0u8; 64], // Placeholder
        }
    }

    /// Validate this vote
    pub fn validate(&self, current_epoch: u32) -> AsfResult<()> {
        // Check epoch is not in the future
        if self.epoch > current_epoch {
            return Err(AsfError::InvalidVote("Vote from future epoch"));
        }

        // Check stake weight is non-zero
        if self.stake_weight == 0 {
            return Err(AsfError::InvalidVote("Zero stake weight"));
        }

        // In production, verify signature here
        // For now, we skip signature verification

        Ok(())
    }

    /// Get the voting power of this vote
    pub fn voting_power(&self) -> Balance {
        self.stake_weight
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// VOTE COLLECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Collection of votes for a specific block and phase
#[derive(Debug, Clone, Default)]
pub struct VoteCollection {
    /// All votes received
    votes: Vec<Vote>,
    
    /// Total stake weight accumulated
    total_stake: Balance,
}

impl VoteCollection {
    /// Create a new empty vote collection
    pub fn new() -> Self {
        Self {
            votes: Vec::new(),
            total_stake: 0,
        }
    }

    /// Add a vote to the collection
    pub fn add_vote(&mut self, vote: Vote) -> AsfResult<()> {
        // Check for duplicate votes from same validator
        if self.votes.iter().any(|v| v.validator == vote.validator) {
            return Err(AsfError::DuplicateVote);
        }

        self.total_stake += vote.stake_weight;
        self.votes.push(vote);

        Ok(())
    }

    /// Get the number of votes
    pub fn count(&self) -> usize {
        self.votes.len()
    }

    /// Get total stake weight
    pub fn total_stake(&self) -> Balance {
        self.total_stake
    }

    /// Get all votes
    pub fn votes(&self) -> &[Vote] {
        &self.votes
    }

    /// Check if we have enough votes to meet BFT threshold
    pub fn meets_threshold(&self, total_validators: u32) -> bool {
        let threshold = crate::bft_threshold(total_validators);
        self.votes.len() >= threshold as usize
    }

    /// Check if we have enough stake to meet BFT threshold
    pub fn meets_stake_threshold(&self, total_stake: Balance) -> bool {
        crate::meets_stake_threshold(self.total_stake, total_stake)
    }

    /// Clear all votes
    pub fn clear(&mut self) {
        self.votes.clear();
        self.total_stake = 0;
    }

    /// Get votes for a specific phase
    pub fn votes_for_phase(&self, phase: ConsensusPhase) -> Vec<&Vote> {
        self.votes.iter().filter(|v| v.phase == phase).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// VOTE AGGREGATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Aggregated vote information for threshold checking
#[derive(Debug, Clone)]
pub struct VoteAggregate {
    /// Block being voted on
    pub block_hash: Hash,
    
    /// Block number
    pub block_number: BlockNumber,
    
    /// Phase being voted on
    pub phase: ConsensusPhase,
    
    /// Number of unique validators
    pub validator_count: u32,
    
    /// Total stake weight
    pub total_stake: Balance,
    
    /// List of validators who voted
    pub validators: Vec<ValidatorId>,
}

impl VoteAggregate {
    /// Create aggregate from vote collection
    pub fn from_votes(votes: &[Vote]) -> Self {
        let mut validators = Vec::new();
        let mut total_stake = 0u128;

        for vote in votes {
            if !validators.contains(&vote.validator) {
                validators.push(vote.validator.clone());
                total_stake += vote.stake_weight;
            }
        }

        // Use first vote for block info (all should be same)
        let (block_hash, block_number, phase) = if let Some(first) = votes.first() {
            (first.block_hash, first.block_number, first.phase)
        } else {
            (Hash::default(), 0, ConsensusPhase::default())
        };

        Self {
            block_hash,
            block_number,
            phase,
            validator_count: validators.len() as u32,
            total_stake,
            validators,
        }
    }

    /// Check if this aggregate meets vote count threshold
    pub fn meets_threshold(&self, total_validators: u32) -> bool {
        self.validator_count >= crate::bft_threshold(total_validators)
    }

    /// Check if this aggregate meets stake threshold
    pub fn meets_stake_threshold(&self, total_stake: Balance) -> bool {
        crate::meets_stake_threshold(self.total_stake, total_stake)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    fn create_test_vote(validator_id: u8, stake: Balance) -> Vote {
        let mut account_bytes = [0u8; 32];
        account_bytes[0] = validator_id;
        
        Vote::new(
            Hash::default(),
            1,
            ConsensusPhase::Prepare,
            AccountId32::from(account_bytes),
            stake,
            1,
            1000,
        )
    }

    #[test]
    fn test_vote_creation() {
        let vote = create_test_vote(1, 1000);
        assert_eq!(vote.block_number, 1);
        assert_eq!(vote.phase, ConsensusPhase::Prepare);
        assert_eq!(vote.stake_weight, 1000);
    }

    #[test]
    fn test_vote_validation() {
        let vote = create_test_vote(1, 1000);
        assert!(vote.validate(1).is_ok());
        assert!(vote.validate(0).is_err()); // Future epoch
    }

    #[test]
    fn test_vote_collection() {
        let mut collection = VoteCollection::new();
        
        let vote1 = create_test_vote(1, 1000);
        let vote2 = create_test_vote(2, 2000);
        
        assert!(collection.add_vote(vote1.clone()).is_ok());
        assert!(collection.add_vote(vote2).is_ok());
        
        assert_eq!(collection.count(), 2);
        assert_eq!(collection.total_stake(), 3000);
        
        // Duplicate vote should fail
        assert!(collection.add_vote(vote1).is_err());
    }

    #[test]
    fn test_vote_threshold() {
        let mut collection = VoteCollection::new();
        
        // Add 15 votes (threshold for 21 validators is 15)
        for i in 0..15 {
            let vote = create_test_vote(i, 1000);
            collection.add_vote(vote).unwrap();
        }
        
        assert!(collection.meets_threshold(21));
        assert!(!collection.meets_threshold(22)); // Would need 16 votes
    }

    #[test]
    fn test_stake_threshold() {
        let mut collection = VoteCollection::new();
        
        // Add votes totaling 700k (threshold for 1M is ~667k)
        let vote1 = create_test_vote(1, 400_000);
        let vote2 = create_test_vote(2, 300_000);
        
        collection.add_vote(vote1).unwrap();
        collection.add_vote(vote2).unwrap();
        
        assert!(collection.meets_stake_threshold(1_000_000));
        assert!(!collection.meets_stake_threshold(2_000_000)); // Would need ~1.33M
    }

    #[test]
    fn test_vote_aggregate() {
        let votes = vec![
            create_test_vote(1, 1000),
            create_test_vote(2, 2000),
            create_test_vote(3, 3000),
        ];
        
        let aggregate = VoteAggregate::from_votes(&votes);
        
        assert_eq!(aggregate.validator_count, 3);
        assert_eq!(aggregate.total_stake, 6000);
        assert_eq!(aggregate.validators.len(), 3);
    }
}