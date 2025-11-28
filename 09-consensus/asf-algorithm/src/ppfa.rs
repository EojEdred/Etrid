//! # PPFA (Proposing Panel for Attestation) Sealing
//!
//! This module implements the PPFA block sealing finalization logic for ASF consensus.
//! PPFA is the committee rotation mechanism that determines which validator proposes blocks
//! in each slot, ensuring fair distribution and preventing centralization.
//!
//! ## PPFA Algorithm Overview
//!
//! The PPFA sealing process consists of:
//! 1. **Committee Selection**: Select top N validators by stake weight
//! 2. **Rotation**: Round-robin rotation through committee members
//! 3. **Seal Generation**: Create a seal proving validator authority
//! 4. **Seal Verification**: Verify seal matches expected proposer
//! 5. **Weight Calculation**: Calculate vote weight based on stake
//!
//! ## Seal Structure
//!
//! A PPFA seal contains:
//! - Slot number (determines which committee member should propose)
//! - PPFA index (position in committee rotation)
//! - Validator signature proving authority
//! - Stake weight at time of proposal
//!
//! ## Security Properties
//!
//! - Only the designated validator can produce a valid seal for a given slot
//! - Seals are cryptographically verifiable
//! - Stake weight determines voting power in consensus
//! - Committee rotation prevents long-term centralization

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    crypto::{sign_seal, verify_seal_signature, SignData, Signature},
    AsfError, AsfResult, Balance, BlockNumber, Hash, ValidatorId,
};

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA SEAL TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// PPFA block seal proving validator authority
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct PpfaSeal {
    /// Slot number when block was produced
    pub slot: u64,

    /// PPFA index (position in committee rotation)
    pub ppfa_index: u32,

    /// Validator who produced this block
    pub validator: ValidatorId,

    /// Validator's stake weight
    pub stake_weight: Balance,

    /// Epoch number
    pub epoch: u32,

    /// Block number
    pub block_number: BlockNumber,

    /// Block hash being sealed
    pub block_hash: Hash,

    /// Cryptographic signature by the PPFA leader proving authority
    /// SECURITY: This MUST be a valid signature over the seal data
    pub signature: Signature,
}

impl PpfaSeal {
    /// Create a new PPFA seal with signature
    ///
    /// SECURITY: Signature MUST be cryptographically valid for the seal data
    pub fn new(
        slot: u64,
        ppfa_index: u32,
        validator: ValidatorId,
        stake_weight: Balance,
        epoch: u32,
        block_number: BlockNumber,
        block_hash: Hash,
        signature: Signature,
    ) -> Self {
        Self {
            slot,
            ppfa_index,
            validator,
            stake_weight,
            epoch,
            block_number,
            block_hash,
            signature,
        }
    }

    /// Create a new PPFA seal with a dummy signature for testing
    ///
    /// WARNING: FOR TESTING ONLY - DO NOT USE IN PRODUCTION
    
    pub fn new_unsigned(
        slot: u64,
        ppfa_index: u32,
        validator: ValidatorId,
        stake_weight: Balance,
        epoch: u32,
        block_number: BlockNumber,
        block_hash: Hash,
    ) -> Self {
        Self {
            slot,
            ppfa_index,
            validator,
            stake_weight,
            epoch,
            block_number,
            block_hash,
            signature: Signature::default(),
        }
    }

    /// Update signature (use sparingly - normally sign during creation)
    pub fn set_signature(&mut self, signature: Signature) {
        self.signature = signature;
    }

    /// Verify the cryptographic signature on this seal
    ///
    /// SECURITY: This performs full cryptographic verification of the seal signature.
    /// Rejects seals with invalid or missing signatures.
    pub fn verify_signature(&self) -> AsfResult<()> {
        // PRODUCTION: Real cryptographic verification
        verify_seal_signature(
            &self.signature,
            self.slot,
            self.ppfa_index,
            self.block_number,
            self.block_hash,
            self.epoch,
            &self.validator,
        )
    }

    /// Calculate voting weight for this validator
    pub fn voting_weight(&self, total_stake: Balance) -> u64 {
        if total_stake == 0 {
            return 1; // Equal weight if no stake
        }

        // Weight = (validator_stake / total_stake) * 1_000_000
        // Scaled to avoid floating point
        ((self.stake_weight as u128 * 1_000_000) / total_stake as u128) as u64
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA COMMITTEE STATE
// ═══════════════════════════════════════════════════════════════════════════════

/// PPFA committee member information
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct PpfaMember {
    /// Validator account
    pub validator: ValidatorId,

    /// Current stake amount
    pub stake: Balance,

    /// Position in PPFA rotation
    pub index: u32,

    /// Blocks produced in current epoch
    pub blocks_produced: u32,
}

impl PpfaMember {
    /// Create a new PPFA member
    pub fn new(validator: ValidatorId, stake: Balance, index: u32) -> Self {
        Self {
            validator,
            stake,
            index,
            blocks_produced: 0,
        }
    }
}

/// PPFA committee state
#[derive(Debug, Clone)]
pub struct PpfaCommittee {
    /// Committee members sorted by index
    members: Vec<PpfaMember>,

    /// Total stake in committee
    total_stake: Balance,

    /// Current rotation index
    current_index: u32,

    /// Current epoch
    epoch: u32,
}

impl PpfaCommittee {
    /// Create a new PPFA committee
    pub fn new(members: Vec<PpfaMember>, epoch: u32) -> Self {
        let total_stake = members.iter().map(|m| m.stake).sum();

        Self {
            members,
            total_stake,
            current_index: 0,
            epoch,
        }
    }

    /// Get the current proposer for a given slot
    pub fn get_proposer(&self, slot: u64) -> Option<&PpfaMember> {
        if self.members.is_empty() {
            return None;
        }

        // PPFA rotation: index = slot % committee_size
        let index = (slot as usize) % self.members.len();
        self.members.get(index)
    }

    /// Get member by validator ID
    pub fn get_member(&self, validator: &ValidatorId) -> Option<&PpfaMember> {
        self.members.iter().find(|m| &m.validator == validator)
    }

    /// Get member by PPFA index
    pub fn get_member_by_index(&self, index: u32) -> Option<&PpfaMember> {
        self.members.iter().find(|m| m.index == index)
    }

    /// Get total stake in committee
    pub fn total_stake(&self) -> Balance {
        self.total_stake
    }

    /// Get committee size
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Get all members
    pub fn members(&self) -> &[PpfaMember] {
        &self.members
    }

    /// Advance to next slot
    pub fn advance_slot(&mut self) {
        if !self.members.is_empty() {
            self.current_index = (self.current_index + 1) % self.members.len() as u32;
        }
    }

    /// Record a block produced by a member
    pub fn record_block(&mut self, validator: &ValidatorId) {
        if let Some(member) = self.members.iter_mut().find(|m| &m.validator == validator) {
            member.blocks_produced += 1;
        }
    }

    /// Get current epoch
    pub fn epoch(&self) -> u32 {
        self.epoch
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA SEAL VERIFIER
// ═══════════════════════════════════════════════════════════════════════════════

/// Verifies PPFA seals for block proposals
pub struct PpfaSealVerifier {
    /// Current committee
    committee: PpfaCommittee,
}

impl PpfaSealVerifier {
    /// Create a new seal verifier
    pub fn new(committee: PpfaCommittee) -> Self {
        Self { committee }
    }

    /// Verify a PPFA seal
    pub fn verify_seal(&self, seal: &PpfaSeal) -> AsfResult<()> {
        // 1. Verify signature
        seal.verify_signature()?;

        // 2. Check epoch matches
        if seal.epoch != self.committee.epoch() {
            return Err(AsfError::InvalidVote("Seal epoch mismatch"));
        }

        // 3. Verify validator is in committee
        let member = self
            .committee
            .get_member(&seal.validator)
            .ok_or(AsfError::InvalidVote("Validator not in committee"))?;

        // 4. Verify PPFA index matches expected proposer for this slot
        let expected_proposer = self
            .committee
            .get_proposer(seal.slot)
            .ok_or(AsfError::InvalidVote("No proposer for slot"))?;

        if expected_proposer.validator != seal.validator {
            return Err(AsfError::InvalidVote("Wrong validator for slot"));
        }

        if expected_proposer.index != seal.ppfa_index {
            return Err(AsfError::InvalidVote("PPFA index mismatch"));
        }

        // 5. Verify stake weight matches current stake
        if member.stake != seal.stake_weight {
            return Err(AsfError::InvalidVote("Stake weight mismatch"));
        }

        Ok(())
    }

    /// Calculate voting weight for a seal
    pub fn calculate_vote_weight(&self, seal: &PpfaSeal) -> u64 {
        seal.voting_weight(self.committee.total_stake())
    }

    /// Check if validator should propose in this slot
    pub fn should_propose(&self, validator: &ValidatorId, slot: u64) -> bool {
        self.committee
            .get_proposer(slot)
            .map(|p| &p.validator == validator)
            .unwrap_or(false)
    }

    /// Update committee
    pub fn update_committee(&mut self, committee: PpfaCommittee) {
        self.committee = committee;
    }

    /// Get committee reference
    pub fn committee(&self) -> &PpfaCommittee {
        &self.committee
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA SEALING ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Main PPFA sealing engine
pub struct PpfaSealingEngine {
    /// Seal verifier
    verifier: PpfaSealVerifier,

    /// Current slot
    current_slot: u64,
}

impl PpfaSealingEngine {
    /// Create a new sealing engine
    pub fn new(committee: PpfaCommittee) -> Self {
        Self {
            verifier: PpfaSealVerifier::new(committee),
            current_slot: 0,
        }
    }

    /// Create a seal for the current validator
    ///
    /// SECURITY: Requires a valid keypair to sign the seal.
    /// In production, this should come from the validator's keystore.
    pub fn create_seal(
        &self,
        validator: ValidatorId,
        block_number: BlockNumber,
        block_hash: Hash,
        keypair: &impl SignData,
    ) -> AsfResult<PpfaSeal> {
        // Get validator's committee info
        let member = self
            .verifier
            .committee()
            .get_member(&validator)
            .ok_or(AsfError::InvalidVote("Validator not in committee"))?;

        // Verify validator should propose in current slot
        if !self.verifier.should_propose(&validator, self.current_slot) {
            return Err(AsfError::InvalidVote("Not validator's turn to propose"));
        }

        // Sign the seal data
        let signature = sign_seal(
            keypair,
            self.current_slot,
            member.index,
            block_number,
            block_hash,
            self.verifier.committee().epoch(),
        );

        // Create seal with real signature
        let seal = PpfaSeal::new(
            self.current_slot,
            member.index,
            validator,
            member.stake,
            self.verifier.committee().epoch(),
            block_number,
            block_hash,
            signature,
        );

        Ok(seal)
    }

    /// Create a seal with a dummy signature for testing
    ///
    /// WARNING: FOR TESTING ONLY - DO NOT USE IN PRODUCTION
    
    pub fn create_seal_unsigned(
        &self,
        validator: ValidatorId,
        block_number: BlockNumber,
        block_hash: Hash,
    ) -> AsfResult<PpfaSeal> {
        // Get validator's committee info
        let member = self
            .verifier
            .committee()
            .get_member(&validator)
            .ok_or(AsfError::InvalidVote("Validator not in committee"))?;

        // Verify validator should propose in current slot
        if !self.verifier.should_propose(&validator, self.current_slot) {
            return Err(AsfError::InvalidVote("Not validator's turn to propose"));
        }

        // Create seal with dummy signature
        let seal = PpfaSeal::new_unsigned(
            self.current_slot,
            member.index,
            validator,
            member.stake,
            self.verifier.committee().epoch(),
            block_number,
            block_hash,
        );

        Ok(seal)
    }

    /// Finalize a block with PPFA seal verification
    pub fn finalize_block(
        &mut self,
        seal: PpfaSeal,
        block_hash: Hash,
        block_number: BlockNumber,
    ) -> AsfResult<FinalizedBlock> {
        // 1. Verify the seal
        self.verifier.verify_seal(&seal)?;

        // 2. Verify block hash matches seal
        if seal.block_hash != block_hash {
            return Err(AsfError::InvalidCertificate("Block hash mismatch"));
        }

        // 3. Verify block number matches seal
        if seal.block_number != block_number {
            return Err(AsfError::InvalidCertificate("Block number mismatch"));
        }

        // 4. Calculate voting weight
        let vote_weight = self.verifier.calculate_vote_weight(&seal);

        // 5. Record block production
        self.verifier
            .committee
            .record_block(&seal.validator);

        // 6. Create finalized block info
        let finalized = FinalizedBlock {
            block_hash,
            block_number,
            seal,
            vote_weight,
            finalized_at: 0, // Should be actual timestamp
        };

        Ok(finalized)
    }

    /// Advance to next slot
    pub fn advance_slot(&mut self) {
        self.current_slot += 1;
        self.verifier.committee.advance_slot();
    }

    /// Update committee (for epoch rotation)
    pub fn update_committee(&mut self, committee: PpfaCommittee) {
        self.verifier.update_committee(committee);
    }

    /// Get current slot
    pub fn current_slot(&self) -> u64 {
        self.current_slot
    }

    /// Get committee
    pub fn committee(&self) -> &PpfaCommittee {
        self.verifier.committee()
    }

    /// Get verifier reference
    pub fn verifier(&self) -> &PpfaSealVerifier {
        &self.verifier
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FINALIZED BLOCK
// ═══════════════════════════════════════════════════════════════════════════════

/// A finalized block with PPFA seal
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct FinalizedBlock {
    /// Block hash
    pub block_hash: Hash,

    /// Block number
    pub block_number: BlockNumber,

    /// PPFA seal
    pub seal: PpfaSeal,

    /// Calculated vote weight
    pub vote_weight: u64,

    /// Timestamp when finalized
    pub finalized_at: u64,
}

impl FinalizedBlock {
    /// Get the proposer
    pub fn proposer(&self) -> &ValidatorId {
        &self.seal.validator
    }

    /// Get stake weight
    pub fn stake_weight(&self) -> Balance {
        self.seal.stake_weight
    }

    /// Get slot number
    pub fn slot(&self) -> u64 {
        self.seal.slot
    }

    /// Get PPFA index
    pub fn ppfa_index(&self) -> u32 {
        self.seal.ppfa_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::{crypto::AccountId32, Pair as _};

    fn create_test_validator(id: u8) -> ValidatorId {
        let mut account_bytes = [0u8; 32];
        account_bytes[0] = id;
        AccountId32::from(account_bytes)
    }

    fn create_test_committee(size: u32, stake_per_validator: Balance) -> PpfaCommittee {
        let members: Vec<PpfaMember> = (0..size)
            .map(|i| PpfaMember::new(create_test_validator(i as u8), stake_per_validator, i))
            .collect();

        PpfaCommittee::new(members, 1)
    }

    /// Create a test seal with real cryptographic signature
    fn create_test_seal(
        slot: u64,
        ppfa_index: u32,
        stake_weight: Balance,
        epoch: u32,
        block_number: BlockNumber,
        block_hash: Hash,
    ) -> (PpfaSeal, sp_core::sr25519::Pair) {
        use sp_core::sr25519;

        // Generate a keypair for this validator
        let (pair, _) = sr25519::Pair::generate();
        let validator = ValidatorId::from(pair.public().0);

        // Sign the seal
        let signature = sign_seal(
            &pair,
            slot,
            ppfa_index,
            block_number,
            block_hash,
            epoch,
        );

        let seal = PpfaSeal::new(
            slot,
            ppfa_index,
            validator,
            stake_weight,
            epoch,
            block_number,
            block_hash,
            signature,
        );

        (seal, pair)
    }

    /// Create a test committee with real keypairs
    fn create_test_committee_with_keys(
        size: u32,
        stake_per_validator: Balance,
    ) -> (PpfaCommittee, Vec<sp_core::sr25519::Pair>) {
        use sp_core::sr25519;

        let mut members = Vec::new();
        let mut pairs = Vec::new();

        for i in 0..size {
            let (pair, _) = sr25519::Pair::generate();
            let validator = ValidatorId::from(pair.public().0);
            members.push(PpfaMember::new(validator, stake_per_validator, i));
            pairs.push(pair);
        }

        (PpfaCommittee::new(members, 1), pairs)
    }

    #[test]
    fn test_ppfa_seal_creation() {
        let (seal, _pair) = create_test_seal(100, 5, 10_000, 1, 50, Hash::default());

        assert_eq!(seal.slot, 100);
        assert_eq!(seal.ppfa_index, 5);
        assert_eq!(seal.stake_weight, 10_000);

        // Verify the signature is valid
        assert!(seal.verify_signature().is_ok());
    }

    #[test]
    fn test_voting_weight_calculation() {
        let (seal, _pair) = create_test_seal(100, 0, 10_000, 1, 50, Hash::default());

        // Validator has 10k out of 100k total stake = 10%
        // Weight = (10_000 / 100_000) * 1_000_000 = 100_000
        let weight = seal.voting_weight(100_000);
        assert_eq!(weight, 100_000);
    }

    #[test]
    fn test_ppfa_committee_creation() {
        let committee = create_test_committee(21, 1000);

        assert_eq!(committee.size(), 21);
        assert_eq!(committee.total_stake(), 21_000);
        assert_eq!(committee.epoch(), 1);
    }

    #[test]
    fn test_ppfa_rotation() {
        let committee = create_test_committee(21, 1000);

        // Slot 0 should map to index 0
        let proposer0 = committee.get_proposer(0).unwrap();
        assert_eq!(proposer0.index, 0);

        // Slot 5 should map to index 5
        let proposer5 = committee.get_proposer(5).unwrap();
        assert_eq!(proposer5.index, 5);

        // Slot 21 should wrap to index 0
        let proposer21 = committee.get_proposer(21).unwrap();
        assert_eq!(proposer21.index, 0);

        // Slot 25 should map to index 4
        let proposer25 = committee.get_proposer(25).unwrap();
        assert_eq!(proposer25.index, 4);
    }

    #[test]
    fn test_seal_verification() {
        let (committee, pairs) = create_test_committee_with_keys(21, 1000);
        let verifier = PpfaSealVerifier::new(committee.clone());

        // Get validator at index 0
        let member = committee.get_member_by_index(0).unwrap();
        let validator = member.validator.clone();
        let pair = &pairs[0];

        // Create seal with real signature
        let signature = sign_seal(pair, 0, 0, 1, Hash::default(), 1);
        let seal = PpfaSeal::new(0, 0, validator, 1000, 1, 1, Hash::default(), signature);

        // Should verify successfully
        assert!(verifier.verify_seal(&seal).is_ok());
    }

    #[test]
    fn test_seal_verification_wrong_slot() {
        let (committee, pairs) = create_test_committee_with_keys(21, 1000);
        let verifier = PpfaSealVerifier::new(committee.clone());

        // Get validator at index 0
        let member = committee.get_member_by_index(0).unwrap();
        let validator = member.validator.clone();
        let pair = &pairs[0];

        // Create seal for validator 0 but at slot 5 (wrong validator - slot 5 should be validator 5)
        let signature = sign_seal(pair, 5, 0, 1, Hash::default(), 1);
        let seal = PpfaSeal::new(5, 0, validator, 1000, 1, 1, Hash::default(), signature);

        // Should fail verification (wrong validator for slot)
        assert!(verifier.verify_seal(&seal).is_err());
    }

    #[test]
    fn test_seal_verification_wrong_stake() {
        let (committee, pairs) = create_test_committee_with_keys(21, 1000);
        let verifier = PpfaSealVerifier::new(committee.clone());

        // Get validator at index 0
        let member = committee.get_member_by_index(0).unwrap();
        let validator = member.validator.clone();
        let pair = &pairs[0];

        // Create seal with wrong stake weight
        let signature = sign_seal(pair, 0, 0, 1, Hash::default(), 1);
        let seal = PpfaSeal::new(
            0,
            0,
            validator,
            5000, // Wrong stake (should be 1000)
            1,
            1,
            Hash::default(),
            signature,
        );

        assert!(verifier.verify_seal(&seal).is_err());
    }

    #[test]
    fn test_should_propose() {
        let (committee, _pairs) = create_test_committee_with_keys(21, 1000);
        let verifier = PpfaSealVerifier::new(committee.clone());

        let member0 = committee.get_member_by_index(0).unwrap();
        let validator0 = member0.validator.clone();

        let member5 = committee.get_member_by_index(5).unwrap();
        let validator5 = member5.validator.clone();

        // Validator 0 should propose at slot 0
        assert!(verifier.should_propose(&validator0, 0));
        assert!(!verifier.should_propose(&validator5, 0));

        // Validator 5 should propose at slot 5
        assert!(verifier.should_propose(&validator5, 5));
        assert!(!verifier.should_propose(&validator0, 5));
    }

    #[test]
    fn test_sealing_engine_create_seal() {
        let (committee, pairs) = create_test_committee_with_keys(21, 1000);
        let engine = PpfaSealingEngine::new(committee.clone());

        let member0 = committee.get_member_by_index(0).unwrap();
        let validator0 = member0.validator.clone();
        let pair0 = &pairs[0];

        let seal = engine.create_seal(validator0, 1, Hash::default(), pair0);

        assert!(seal.is_ok());
        let seal = seal.unwrap();
        assert_eq!(seal.ppfa_index, 0);
        assert_eq!(seal.slot, 0);

        // Verify signature is valid
        assert!(seal.verify_signature().is_ok());
    }

    #[test]
    fn test_sealing_engine_wrong_validator() {
        let (committee, pairs) = create_test_committee_with_keys(21, 1000);
        let engine = PpfaSealingEngine::new(committee.clone());

        // Validator 5 shouldn't propose at slot 0
        let member5 = committee.get_member_by_index(5).unwrap();
        let validator5 = member5.validator.clone();
        let pair5 = &pairs[5];

        let seal = engine.create_seal(validator5, 1, Hash::default(), pair5);

        assert!(seal.is_err());
    }

    #[test]
    fn test_finalize_block() {
        let (committee, pairs) = create_test_committee_with_keys(21, 1000);
        let mut engine = PpfaSealingEngine::new(committee.clone());

        let member0 = committee.get_member_by_index(0).unwrap();
        let validator0 = member0.validator.clone();
        let pair0 = &pairs[0];

        let block_hash = Hash::default();
        let seal = engine.create_seal(validator0.clone(), 1, block_hash, pair0).unwrap();

        let finalized = engine.finalize_block(seal, block_hash, 1);

        assert!(finalized.is_ok());
        let finalized = finalized.unwrap();
        assert_eq!(finalized.proposer(), &validator0);
        assert_eq!(finalized.block_number, 1);
    }

    #[test]
    fn test_finalize_block_hash_mismatch() {
        let (committee, pairs) = create_test_committee_with_keys(21, 1000);
        let mut engine = PpfaSealingEngine::new(committee.clone());

        let member0 = committee.get_member_by_index(0).unwrap();
        let validator0 = member0.validator.clone();
        let pair0 = &pairs[0];

        let block_hash = Hash::default();
        let seal = engine.create_seal(validator0, 1, block_hash, pair0).unwrap();

        // Try to finalize with different hash
        let mut wrong_hash_bytes = [0u8; 32];
        wrong_hash_bytes[0] = 1;
        let wrong_hash = Hash::from(wrong_hash_bytes);

        let finalized = engine.finalize_block(seal, wrong_hash, 1);
        assert!(finalized.is_err());
    }

    #[test]
    fn test_slot_advancement() {
        let committee = create_test_committee(21, 1000);
        let mut engine = PpfaSealingEngine::new(committee);

        assert_eq!(engine.current_slot(), 0);

        engine.advance_slot();
        assert_eq!(engine.current_slot(), 1);

        engine.advance_slot();
        assert_eq!(engine.current_slot(), 2);
    }

    #[test]
    fn test_block_production_tracking() {
        let (mut committee, _pairs) = create_test_committee_with_keys(21, 1000);

        let member0 = committee.get_member_by_index(0).unwrap();
        let validator0 = member0.validator.clone();

        let member = committee.get_member(&validator0).unwrap();
        assert_eq!(member.blocks_produced, 0);

        committee.record_block(&validator0);
        let member = committee.get_member(&validator0).unwrap();
        assert_eq!(member.blocks_produced, 1);
    }

    #[test]
    fn test_committee_with_varying_stakes() {
        use sp_core::sr25519;

        let (pair0, _) = sr25519::Pair::generate();
        let (pair1, _) = sr25519::Pair::generate();
        let (pair2, _) = sr25519::Pair::generate();

        let validator0 = ValidatorId::from(pair0.public().0);
        let validator1 = ValidatorId::from(pair1.public().0);
        let validator2 = ValidatorId::from(pair2.public().0);

        let members = vec![
            PpfaMember::new(validator0, 10_000, 0),
            PpfaMember::new(validator1.clone(), 20_000, 1),
            PpfaMember::new(validator2, 5_000, 2),
        ];

        let committee = PpfaCommittee::new(members, 1);
        assert_eq!(committee.total_stake(), 35_000);

        // Validator 1 has highest stake, should have proportional weight
        let signature = sign_seal(&pair1, 1, 1, 1, Hash::default(), 1);
        let seal1 = PpfaSeal::new(1, 1, validator1, 20_000, 1, 1, Hash::default(), signature);

        let weight = seal1.voting_weight(35_000);
        // (20_000 / 35_000) * 1_000_000 ≈ 571_428
        assert!(weight > 571_000 && weight < 572_000);

        // Verify signature is valid
        assert!(seal1.verify_signature().is_ok());
    }

    #[test]
    fn test_seal_rejects_invalid_signature() {
        use sp_core::sr25519;

        let (pair1, _) = sr25519::Pair::generate();
        let (pair2, _) = sr25519::Pair::generate();

        let _validator1 = ValidatorId::from(pair1.public().0);

        // Sign with pair1 but claim it's from validator derived from pair2
        let wrong_validator = ValidatorId::from(pair2.public().0);
        let signature = sign_seal(&pair1, 1, 1, 1, Hash::default(), 1);

        // Create seal with mismatched signature
        let seal = PpfaSeal::new(1, 1, wrong_validator, 20_000, 1, 1, Hash::default(), signature);

        // Should fail signature verification
        assert!(seal.verify_signature().is_err());
    }

    #[test]
    fn test_seal_rejects_tampered_data() {
        use sp_core::sr25519;

        let (pair, _) = sr25519::Pair::generate();
        let validator = ValidatorId::from(pair.public().0);

        // Sign with slot 1
        let signature = sign_seal(&pair, 1, 1, 1, Hash::default(), 1);

        // Create seal with slot 2 (tampered) but signature for slot 1
        let seal = PpfaSeal::new(2, 1, validator, 20_000, 1, 1, Hash::default(), signature);

        // Should fail signature verification (data was tampered)
        assert!(seal.verify_signature().is_err());
    }
}
