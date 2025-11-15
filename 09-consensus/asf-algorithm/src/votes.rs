//! # Vote Types and Validation
//!
//! This module defines the vote structure used in HotStuff consensus
//! and provides validation logic for votes.

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    crypto::{verify_vote_signature, Signature},
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

    /// Cryptographic signature from the validator (REQUIRED for security)
    pub signature: Signature,
}

impl Vote {
    /// Create a new vote with cryptographic signature (PRODUCTION)
    ///
    /// # Security
    /// This is the ONLY way to create a valid vote. All votes MUST be signed.
    pub fn new(
        block_hash: Hash,
        block_number: BlockNumber,
        phase: ConsensusPhase,
        validator: ValidatorId,
        stake_weight: Balance,
        epoch: u32,
        timestamp: u64,
        signature: Signature,
    ) -> Self {
        Self {
            block_hash,
            block_number,
            phase,
            validator,
            stake_weight,
            epoch,
            timestamp,
            signature,
        }
    }

    /// Create an unsigned vote for testing purposes ONLY
    ///
    /// # Warning
    /// This function is only available in test builds and creates an INVALID vote
    /// that will FAIL validation. Only use this for unit testing infrastructure.
    #[cfg(test)]
    pub fn new_unsigned(
        block_hash: Hash,
        block_number: BlockNumber,
        phase: ConsensusPhase,
        validator: ValidatorId,
        stake_weight: Balance,
        epoch: u32,
        timestamp: u64,
    ) -> Self {
        use crate::crypto::Signature;
        // Create a dummy signature for testing
        Self {
            block_hash,
            block_number,
            phase,
            validator,
            stake_weight,
            epoch,
            timestamp,
            signature: Signature::from_sr25519_bytes([0u8; 64]),
        }
    }

    /// Validate this vote with FULL cryptographic signature verification
    ///
    /// # Security
    /// This function performs THREE critical security checks:
    /// 1. Epoch validation (prevents future votes)
    /// 2. Stake validation (prevents zero-stake attacks)
    /// 3. Cryptographic signature verification (prevents forgery)
    ///
    /// ALL three checks MUST pass for a vote to be valid.
    pub fn validate(&self, current_epoch: u32) -> AsfResult<()> {
        // Check epoch is not in the future
        if self.epoch > current_epoch {
            return Err(AsfError::InvalidVote("Vote from future epoch"));
        }

        // Check stake weight is non-zero
        if self.stake_weight == 0 {
            return Err(AsfError::InvalidVote("Zero stake weight"));
        }

        // CRITICAL SECURITY: Verify cryptographic signature
        // This ensures the vote actually came from the claimed validator
        verify_vote_signature(
            &self.signature,
            self.block_hash,
            self.block_number,
            self.phase as u8,
            self.epoch,
            self.timestamp,
            &self.validator,
        )?;

        Ok(())
    }

    /// Validate vote WITHOUT signature check (TESTING ONLY)
    ///
    /// # Warning
    /// This function SKIPS cryptographic verification and should NEVER be used
    /// in production code. It exists solely for unit testing infrastructure.
    #[cfg(test)]
    pub fn validate_unsigned(&self, current_epoch: u32) -> AsfResult<()> {
        if self.epoch > current_epoch {
            return Err(AsfError::InvalidVote("Vote from future epoch"));
        }
        if self.stake_weight == 0 {
            return Err(AsfError::InvalidVote("Zero stake weight"));
        }
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
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
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
    use sp_core::Pair as _;
    use crate::crypto::{sign_vote, SignData};

    /// Helper to create a properly signed test vote
    fn create_test_vote(validator_id: u8, stake: Balance) -> Vote {
        use sp_core::sr25519;

        // Generate a deterministic keypair for testing
        let seed = [validator_id; 32];
        let pair = sr25519::Pair::from_seed(&seed);

        let block_hash = Hash::default();
        let block_number = 1;
        let phase = ConsensusPhase::Prepare;
        let epoch = 1;
        let timestamp = 1000;

        // Sign the vote
        let signature = sign_vote(
            &pair,
            block_hash,
            block_number,
            phase as u8,
            epoch,
            timestamp,
        );

        Vote::new(
            block_hash,
            block_number,
            phase,
            ValidatorId::from(pair.public().0),
            stake,
            epoch,
            timestamp,
            signature,
        )
    }

    /// Helper to create an unsigned test vote (will fail validation)
    fn create_test_vote_unsigned(validator_id: u8, stake: Balance) -> Vote {
        let mut account_bytes = [0u8; 32];
        account_bytes[0] = validator_id;

        Vote::new_unsigned(
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
    fn test_vote_validation_with_signature() {
        let vote = create_test_vote(1, 1000);
        // Should pass full validation with correct signature
        assert!(vote.validate(1).is_ok());
        // Should fail with future epoch
        assert!(vote.validate(0).is_err());
    }

    #[test]
    fn test_vote_validation_unsigned() {
        let vote = create_test_vote_unsigned(1, 1000);
        // Unsigned validation should work
        assert!(vote.validate_unsigned(1).is_ok());
        assert!(vote.validate_unsigned(0).is_err()); // Future epoch
    }

    #[test]
    fn test_vote_signature_verification() {
        use sp_core::sr25519;

        // Create vote with valid signature
        let valid_vote = create_test_vote(1, 1000);
        assert!(valid_vote.validate(1).is_ok());

        // Create vote with WRONG signature (from different validator)
        let seed1 = [1u8; 32];
        let seed2 = [2u8; 32];
        let pair1 = sr25519::Pair::from_seed(&seed1);
        let pair2 = sr25519::Pair::from_seed(&seed2);

        let block_hash = Hash::default();
        let signature_from_validator1 = sign_vote(&pair1, block_hash, 1, 0, 1, 1000);

        // Try to use validator1's signature with validator2's ID
        let invalid_vote = Vote::new(
            block_hash,
            1,
            ConsensusPhase::Prepare,
            ValidatorId::from(pair2.public().0), // Wrong validator!
            1000,
            1,
            1000,
            signature_from_validator1,
        );

        // This should FAIL validation because signature doesn't match validator
        assert!(invalid_vote.validate(1).is_err());
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
        assert!(collection.meets_threshold(22)); // 22 * 2/3 + 1 = 15, so 15 votes meets threshold
        assert!(!collection.meets_threshold(23)); // 23 * 2/3 + 1 = 16, so would need 16 votes
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