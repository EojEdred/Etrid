//! # Validity Certificates
//!
//! This module implements validity certificate generation, validation,
//! and aggregation according to the Ascending Scale of Finality.

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    AsfError, AsfResult, Balance, BlockNumber, ConsensusPhase, FinalityLevel, Hash,
    ValidatorId, Vote, VoteAggregate,
};

// ═══════════════════════════════════════════════════════════════════════════════
// CERTIFICATE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// A validity certificate proving consensus was reached for a phase
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct ValidityCertificate {
    /// Block hash being certified
    pub block_hash: Hash,
    
    /// Block number
    pub block_number: BlockNumber,
    
    /// Consensus phase achieved
    pub phase: ConsensusPhase,
    
    /// Validator who issued this certificate
    pub validator: ValidatorId,
    
    /// Validator's stake weight
    pub stake_weight: Balance,
    
    /// Epoch when issued
    pub epoch: u32,
    
    /// Timestamp (Unix milliseconds)
    pub timestamp: u64,
    
    /// Aggregate vote information proving threshold was met
    pub vote_aggregate: VoteAggregate,
}

impl ValidityCertificate {
    /// Create a certificate from a vote aggregate
    pub fn from_aggregate(
        aggregate: VoteAggregate,
        issuer: ValidatorId,
        issuer_stake: Balance,
        epoch: u32,
        timestamp: u64,
    ) -> Self {
        Self {
            block_hash: aggregate.block_hash,
            block_number: aggregate.block_number,
            phase: aggregate.phase,
            validator: issuer,
            stake_weight: issuer_stake,
            epoch,
            timestamp,
            vote_aggregate: aggregate,
        }
    }

    /// Validate this certificate
    pub fn validate(
        &self,
        total_validators: u32,
        total_stake: Balance,
        current_epoch: u32,
    ) -> AsfResult<()> {
        // Check epoch
        if self.epoch > current_epoch {
            return Err(AsfError::InvalidCertificate("Future epoch"));
        }

        // Check stake weight
        if self.stake_weight == 0 {
            return Err(AsfError::InvalidCertificate("Zero stake weight"));
        }

        // Verify vote aggregate meets thresholds
        if !self.vote_aggregate.meets_threshold(total_validators) {
            return Err(AsfError::InsufficientVotes {
                got: self.vote_aggregate.validator_count,
                need: crate::bft_threshold(total_validators),
            });
        }

        if !self.vote_aggregate.meets_stake_threshold(total_stake) {
            return Err(AsfError::InsufficientStake {
                got: self.vote_aggregate.total_stake,
                need: crate::bft_stake_threshold(total_stake),
            });
        }

        Ok(())
    }

    /// Check if this certificate is for the same block and phase as another
    pub fn matches(&self, block_hash: &Hash, phase: &ConsensusPhase) -> bool {
        self.block_hash == *block_hash && self.phase == *phase
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CERTIFICATE COLLECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Collection of validity certificates for a block
#[derive(Debug, Clone, Default)]
pub struct CertificateCollection {
    /// All certificates received
    certificates: Vec<ValidityCertificate>,
    
    /// Certificates grouped by phase
    prepare_certs: Vec<ValidityCertificate>,
    precommit_certs: Vec<ValidityCertificate>,
    commit_certs: Vec<ValidityCertificate>,
    decide_certs: Vec<ValidityCertificate>,
}

impl CertificateCollection {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a certificate to the collection
    pub fn add_certificate(&mut self, cert: ValidityCertificate) -> AsfResult<()> {
        // Check for duplicate from same validator
        if self
            .certificates
            .iter()
            .any(|c| c.validator == cert.validator && c.phase == cert.phase)
        {
            return Err(AsfError::InvalidCertificate("Duplicate certificate"));
        }

        // Add to phase-specific collection
        match cert.phase {
            ConsensusPhase::Prepare => self.prepare_certs.push(cert.clone()),
            ConsensusPhase::PreCommit => self.precommit_certs.push(cert.clone()),
            ConsensusPhase::Commit => self.commit_certs.push(cert.clone()),
            ConsensusPhase::Decide => self.decide_certs.push(cert.clone()),
        }

        self.certificates.push(cert);

        Ok(())
    }

    /// Get total number of certificates
    pub fn total_count(&self) -> u32 {
        self.certificates.len() as u32
    }

    /// Get certificates for a specific phase
    pub fn certificates_for_phase(&self, phase: ConsensusPhase) -> &[ValidityCertificate] {
        match phase {
            ConsensusPhase::Prepare => &self.prepare_certs,
            ConsensusPhase::PreCommit => &self.precommit_certs,
            ConsensusPhase::Commit => &self.commit_certs,
            ConsensusPhase::Decide => &self.decide_certs,
        }
    }

    /// Get count of certificates for a phase
    pub fn count_for_phase(&self, phase: ConsensusPhase) -> u32 {
        self.certificates_for_phase(phase).len() as u32
    }

    /// Get all certificates
    pub fn all_certificates(&self) -> &[ValidityCertificate] {
        &self.certificates
    }

    /// Calculate finality level based on total certificates
    pub fn finality_level(&self) -> FinalityLevel {
        FinalityLevel::from(self.total_count())
    }

    /// Check if we have reached a specific phase
    pub fn has_reached_phase(&self, phase: ConsensusPhase, threshold: u32) -> bool {
        self.count_for_phase(phase) >= threshold
    }

    /// Clear all certificates
    pub fn clear(&mut self) {
        self.certificates.clear();
        self.prepare_certs.clear();
        self.precommit_certs.clear();
        self.commit_certs.clear();
        self.decide_certs.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CERTIFICATE GENERATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Certificate generator that creates certificates from votes
pub struct CertificateGenerator {
    /// Total validators in the committee
    total_validators: u32,
    
    /// Total stake in the committee
    total_stake: Balance,
    
    /// Current epoch
    current_epoch: u32,
}

impl CertificateGenerator {
    /// Create a new certificate generator
    pub fn new(total_validators: u32, total_stake: Balance, current_epoch: u32) -> Self {
        Self {
            total_validators,
            total_stake,
            current_epoch,
        }
    }

    /// Try to generate a certificate from votes
    pub fn try_generate(
        &self,
        votes: &[Vote],
        issuer: ValidatorId,
        issuer_stake: Balance,
        timestamp: u64,
    ) -> AsfResult<ValidityCertificate> {
        if votes.is_empty() {
            return Err(AsfError::InvalidCertificate("No votes provided"));
        }

        // Create aggregate
        let aggregate = VoteAggregate::from_votes(votes);

        // Check thresholds
        if !aggregate.meets_threshold(self.total_validators) {
            return Err(AsfError::InsufficientVotes {
                got: aggregate.validator_count,
                need: crate::bft_threshold(self.total_validators),
            });
        }

        if !aggregate.meets_stake_threshold(self.total_stake) {
            return Err(AsfError::InsufficientStake {
                got: aggregate.total_stake,
                need: crate::bft_stake_threshold(self.total_stake),
            });
        }

        // Generate certificate
        let cert = ValidityCertificate::from_aggregate(
            aggregate,
            issuer,
            issuer_stake,
            self.current_epoch,
            timestamp,
        );

        Ok(cert)
    }

    /// Update epoch
    pub fn set_epoch(&mut self, epoch: u32) {
        self.current_epoch = epoch;
    }

    /// Update committee size
    pub fn update_committee(&mut self, validators: u32, stake: Balance) {
        self.total_validators = validators;
        self.total_stake = stake;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vote;
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

    fn create_test_validator(id: u8) -> ValidatorId {
        let mut account_bytes = [0u8; 32];
        account_bytes[0] = id;
        AccountId32::from(account_bytes)
    }

    #[test]
    fn test_certificate_creation() {
        let votes = vec![
            create_test_vote(1, 1000),
            create_test_vote(2, 1000),
            create_test_vote(3, 1000),
        ];
        
        let aggregate = VoteAggregate::from_votes(&votes);
        let cert = ValidityCertificate::from_aggregate(
            aggregate,
            create_test_validator(1),
            1000,
            1,
            1000,
        );
        
        assert_eq!(cert.block_number, 1);
        assert_eq!(cert.phase, ConsensusPhase::Prepare);
        assert_eq!(cert.vote_aggregate.validator_count, 3);
    }

    #[test]
    fn test_certificate_validation() {
        let votes = vec![
            create_test_vote(1, 400_000),
            create_test_vote(2, 300_000),
        ];
        
        let aggregate = VoteAggregate::from_votes(&votes);
        let cert = ValidityCertificate::from_aggregate(
            aggregate,
            create_test_validator(1),
            1000,
            1,
            1000,
        );
        
        // Should pass with sufficient stake
        assert!(cert.validate(3, 1_000_000, 1).is_ok());
        
        // Should fail with insufficient stake
        assert!(cert.validate(3, 2_000_000, 1).is_err());
    }

    #[test]
    fn test_certificate_collection() {
        let mut collection = CertificateCollection::new();
        
        let votes = vec![create_test_vote(1, 1000), create_test_vote(2, 1000)];
        let aggregate = VoteAggregate::from_votes(&votes);
        
        let cert1 = ValidityCertificate::from_aggregate(
            aggregate.clone(),
            create_test_validator(1),
            1000,
            1,
            1000,
        );
        
        let cert2 = ValidityCertificate::from_aggregate(
            aggregate,
            create_test_validator(2),
            1000,
            1,
            2000,
        );
        
        assert!(collection.add_certificate(cert1).is_ok());
        assert!(collection.add_certificate(cert2).is_ok());
        
        assert_eq!(collection.total_count(), 2);
        assert_eq!(collection.count_for_phase(ConsensusPhase::Prepare), 2);
    }

    #[test]
    fn test_finality_levels() {
        let mut collection = CertificateCollection::new();
        
        // Add certificates to reach different finality levels
        for i in 0..15 {
            let votes = vec![create_test_vote(1, 1000)];
            let aggregate = VoteAggregate::from_votes(&votes);
            let cert = ValidityCertificate::from_aggregate(
                aggregate,
                create_test_validator(i),
                1000,
                1,
                (i as u64) * 1000,
            );
            collection.add_certificate(cert).unwrap();
        }
        
        assert_eq!(collection.finality_level(), FinalityLevel::Weak);
        assert!(collection.finality_level().is_finalized());
    }

    #[test]
    fn test_certificate_generator() {
        let generator = CertificateGenerator::new(21, 21_000, 1);
        
        // Create 15 votes (meets threshold for 21 validators)
        let votes: Vec<Vote> = (0..15).map(|i| create_test_vote(i, 1000)).collect();
        
        let result = generator.try_generate(
            &votes,
            create_test_validator(1),
            1000,
            1000,
        );
        
        assert!(result.is_ok());
        
        let cert = result.unwrap();
        assert_eq!(cert.vote_aggregate.validator_count, 15);
    }

    #[test]
    fn test_certificate_generator_insufficient_votes() {
        let generator = CertificateGenerator::new(21, 21_000, 1);
        
        // Only 5 votes (not enough)
        let votes: Vec<Vote> = (0..5).map(|i| create_test_vote(i, 1000)).collect();
        
        let result = generator.try_generate(
            &votes,
            create_test_validator(1),
            1000,
            1000,
        );
        
        assert!(result.is_err());
    }
}