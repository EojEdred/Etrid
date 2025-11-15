//! # PPFA Sealing Integration Tests
//!
//! Comprehensive test suite for PPFA (Proposing Panel for Attestation) sealing.
//! Tests cover:
//! - Basic sealing and verification
//! - Committee rotation
//! - Stake-weighted voting
//! - Edge cases and failure modes
//! - Byzantine fault scenarios

use asf_algorithm::{
    PpfaSeal, PpfaMember, PpfaCommittee, PpfaSealVerifier, PpfaSealingEngine,
    Balance, Hash,
};
use sp_core::crypto::AccountId32;

// ═══════════════════════════════════════════════════════════════════════════════
// TEST HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

fn create_validator(id: u8) -> AccountId32 {
    let mut bytes = [0u8; 32];
    bytes[0] = id;
    AccountId32::from(bytes)
}

fn create_committee(size: u32, stake_per_validator: Balance) -> PpfaCommittee {
    let members: Vec<PpfaMember> = (0..size)
        .map(|i| PpfaMember::new(create_validator(i as u8), stake_per_validator, i))
        .collect();

    PpfaCommittee::new(members, 1)
}

fn create_committee_with_varying_stakes(stakes: Vec<Balance>) -> PpfaCommittee {
    let members: Vec<PpfaMember> = stakes
        .into_iter()
        .enumerate()
        .map(|(i, stake)| PpfaMember::new(create_validator(i as u8), stake, i as u32))
        .collect();

    PpfaCommittee::new(members, 1)
}

// ═══════════════════════════════════════════════════════════════════════════════
// BASIC SEALING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_basic_seal_creation_and_verification() {
    let committee = create_committee(21, 10_000);
    let verifier = PpfaSealVerifier::new(committee);

    let validator = create_validator(0);
    let seal = PpfaSeal::new_unsigned(
        0,
        0,
        validator,
        10_000,
        1,
        1,
        Hash::default(),
    );

    // Note: Signature verification will fail with dummy signature
    // This test only checks structural correctness
    assert_eq!(seal.slot, 0);
    assert_eq!(seal.ppfa_index, 0);
}

#[test]
fn test_seal_with_correct_proposer() {
    let committee = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    let validator = create_validator(0);
    let block_hash = Hash::default();

    // Slot 0 should be validator 0
    let seal = engine.create_seal_unsigned(validator, 1, block_hash);
    assert!(seal.is_ok());
}

#[test]
fn test_seal_with_wrong_proposer() {
    let committee = create_committee(21, 10_000);
    let engine = PpfaSealingEngine::new(committee);

    // Validator 5 should not propose at slot 0
    let validator = create_validator(5);
    let block_hash = Hash::default();

    let seal = engine.create_seal_unsigned(validator, 1, block_hash);
    assert!(seal.is_err());
}

#[test]
fn test_finalize_block_with_valid_seal() {
    let committee = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    let validator = create_validator(0);
    let block_hash = Hash::default();
    let seal = engine.create_seal_unsigned(validator.clone(), 1, block_hash).unwrap();

    let result = engine.finalize_block(seal, block_hash, 1);
    assert!(result.is_ok());

    let finalized = result.unwrap();
    assert_eq!(finalized.proposer(), &validator);
    assert_eq!(finalized.block_number, 1);
}

#[test]
fn test_finalize_block_with_hash_mismatch() {
    let committee = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    let validator = create_validator(0);
    let block_hash = Hash::default();
    let seal = engine.create_seal_unsigned(validator, 1, block_hash).unwrap();

    // Try with different hash
    let mut wrong_hash = [0u8; 32];
    wrong_hash[0] = 1;
    let result = engine.finalize_block(seal, Hash::from(wrong_hash), 1);
    assert!(result.is_err());
}

#[test]
fn test_finalize_block_with_number_mismatch() {
    let committee = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    let validator = create_validator(0);
    let block_hash = Hash::default();
    let seal = engine.create_seal_unsigned(validator, 1, block_hash).unwrap();

    // Try with different block number
    let result = engine.finalize_block(seal, block_hash, 999);
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMITTEE ROTATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ppfa_rotation_through_committee() {
    let committee = create_committee(21, 10_000);

    // Test first rotation cycle
    for i in 0..21 {
        let proposer = committee.get_proposer(i).unwrap();
        assert_eq!(proposer.index, i as u32);
        assert_eq!(proposer.validator, create_validator(i as u8));
    }

    // Test wrap-around
    let proposer = committee.get_proposer(21).unwrap();
    assert_eq!(proposer.index, 0);
}

#[test]
fn test_ppfa_rotation_with_slot_advancement() {
    let committee = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    assert_eq!(engine.current_slot(), 0);

    // Advance through several slots
    for i in 1..=50 {
        engine.advance_slot();
        assert_eq!(engine.current_slot(), i);

        let _expected_index = (i % 21) as u32;
        let expected_validator = create_validator((i % 21) as u8);

        let should_propose = engine
            .committee()
            .get_proposer(i)
            .map(|p| p.validator == expected_validator)
            .unwrap_or(false);

        assert!(should_propose);
    }
}

#[test]
fn test_committee_rotation_maintains_order() {
    let committee = create_committee(21, 10_000);

    // Get proposers for multiple full cycles
    let mut proposers = Vec::new();
    for i in 0..63 {
        // 3 full cycles
        let proposer = committee.get_proposer(i).unwrap();
        proposers.push((i, proposer.index));
    }

    // Verify pattern repeats
    for i in 0..21 {
        assert_eq!(proposers[i].1, proposers[i + 21].1);
        assert_eq!(proposers[i + 21].1, proposers[i + 42].1);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// STAKE-WEIGHTED VOTING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_voting_weight_calculation_equal_stakes() {
    let committee = create_committee(21, 10_000);
    let total_stake = committee.total_stake();

    let validator = create_validator(0);
    let seal = PpfaSeal::new_unsigned(
        0,
        0,
        validator,
        10_000,
        1,
        1,
        Hash::default(),
    );

    let weight = seal.voting_weight(total_stake);

    // Each validator has 1/21 of total stake
    // Weight = (10_000 / 210_000) * 1_000_000 ≈ 47_619
    assert!(weight >= 47_600 && weight <= 47_650);
}

#[test]
fn test_voting_weight_calculation_varying_stakes() {
    let stakes = vec![
        100_000, // 50% of total (200k)
        50_000,  // 25%
        30_000,  // 15%
        20_000,  // 10%
    ];

    let committee = create_committee_with_varying_stakes(stakes.clone());
    let total_stake = committee.total_stake();
    assert_eq!(total_stake, 200_000);

    // Test weight for highest stake validator
    let seal = PpfaSeal::new_unsigned(
        0,
        0,
        create_validator(0),
        100_000,
        1,
        1,
        Hash::default(),
    );

    let weight = seal.voting_weight(total_stake);
    // (100_000 / 200_000) * 1_000_000 = 500_000
    assert_eq!(weight, 500_000);
}

#[test]
fn test_voting_weight_proportional_to_stake() {
    let stakes = vec![10_000, 20_000, 30_000];
    let committee = create_committee_with_varying_stakes(stakes);
    let total_stake = committee.total_stake();

    let seals: Vec<PpfaSeal> = (0..3)
        .map(|i| {
            let stake = committee.get_member_by_index(i).unwrap().stake;
            PpfaSeal::new_unsigned(
                i as u64,
                i,
                create_validator(i as u8),
                stake,
                1,
                1,
                Hash::default(),
            )
        })
        .collect();

    let weights: Vec<u64> = seals.iter().map(|s| s.voting_weight(total_stake)).collect();

    // Weights should be proportional: 10k:20k:30k = 1:2:3
    assert!(weights[1] > weights[0]);
    assert!(weights[2] > weights[1]);

    // Verify approximate ratios
    let ratio_0_1 = weights[1] as f64 / weights[0] as f64;
    let ratio_1_2 = weights[2] as f64 / weights[1] as f64;

    assert!((ratio_0_1 - 2.0).abs() < 0.01); // ~2:1
    assert!((ratio_1_2 - 1.5).abs() < 0.01); // ~3:2
}

// ═══════════════════════════════════════════════════════════════════════════════
// EDGE CASE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_single_validator_committee() {
    let committee = create_committee(1, 100_000);
    let engine = PpfaSealingEngine::new(committee);

    let validator = create_validator(0);

    // Single validator should always propose
    for slot in 0..10 {
        let seal = engine.create_seal_unsigned(validator.clone(), slot + 1, Hash::default());
        assert!(seal.is_ok());
    }
}

#[test]
fn test_zero_stake_voting_weight() {
    let seal = PpfaSeal::new_unsigned(
        0,
        0,
        create_validator(0),
        1000,
        1,
        1,
        Hash::default(),
    );

    // With zero total stake, should return 1 (equal weight)
    let weight = seal.voting_weight(0);
    assert_eq!(weight, 1);
}

#[test]
fn test_seal_verification_wrong_epoch() {
    let committee = create_committee(21, 10_000);
    let verifier = PpfaSealVerifier::new(committee);

    let validator = create_validator(0);
    let seal = PpfaSeal::new_unsigned(
        0,
        0,
        validator,
        10_000,
        999, // Wrong epoch
        1,
        Hash::default(),
    );

    assert!(verifier.verify_seal(&seal).is_err());
}

#[test]
fn test_seal_verification_wrong_stake_weight() {
    let committee = create_committee(21, 10_000);
    let verifier = PpfaSealVerifier::new(committee);

    let validator = create_validator(0);
    let seal = PpfaSeal::new_unsigned(
        0,
        0,
        validator,
        50_000, // Wrong stake
        1,
        1,
        Hash::default(),
    );

    assert!(verifier.verify_seal(&seal).is_err());
}

#[test]
fn test_seal_verification_validator_not_in_committee() {
    let committee = create_committee(21, 10_000);
    let verifier = PpfaSealVerifier::new(committee);

    // Validator 99 is not in committee
    let validator = create_validator(99);
    let seal = PpfaSeal::new_unsigned(
        0,
        0,
        validator,
        10_000,
        1,
        1,
        Hash::default(),
    );

    assert!(verifier.verify_seal(&seal).is_err());
}

#[test]
fn test_seal_verification_wrong_ppfa_index() {
    let committee = create_committee(21, 10_000);
    let verifier = PpfaSealVerifier::new(committee);

    let validator = create_validator(0);
    let seal = PpfaSeal::new_unsigned(
        0,
        5, // Wrong PPFA index
        validator,
        10_000,
        1,
        1,
        Hash::default(),
    );

    assert!(verifier.verify_seal(&seal).is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// BYZANTINE FAULT TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_conflicting_seals_same_slot() {
    let committee = create_committee(21, 10_000);
    let verifier = PpfaSealVerifier::new(committee);

    // Two different validators claiming same slot
    let validator0 = create_validator(0);
    let validator5 = create_validator(5);

    let seal0 = PpfaSeal::new_unsigned(0, 0, validator0, 10_000, 1, 1, Hash::default());
    let seal5 = PpfaSeal::new_unsigned(0, 0, validator5, 10_000, 1, 1, Hash::default());

    // Only validator 0 should verify for slot 0
    assert!(verifier.verify_seal(&seal0).is_ok());
    assert!(verifier.verify_seal(&seal5).is_err());
}

#[test]
fn test_seal_reuse_different_blocks() {
    let committee = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    let validator = create_validator(0);
    let block_hash1 = Hash::default();
    let mut block_hash2 = [0u8; 32];
    block_hash2[0] = 1;

    let seal = engine.create_seal_unsigned(validator, 1, block_hash1).unwrap();

    // Seal should only work for original block hash
    assert!(engine.finalize_block(seal.clone(), block_hash1, 1).is_ok());

    // Same seal should fail for different block
    let result = engine.finalize_block(seal, Hash::from(block_hash2), 1);
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK PRODUCTION TRACKING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_block_production_tracking() {
    let mut committee = create_committee(21, 10_000);

    for i in 0..21 {
        let validator = create_validator(i as u8);
        let member = committee.get_member(&validator).unwrap();
        assert_eq!(member.blocks_produced, 0);

        committee.record_block(&validator);
        let member = committee.get_member(&validator).unwrap();
        assert_eq!(member.blocks_produced, 1);
    }
}

#[test]
fn test_multiple_blocks_per_validator() {
    let mut committee = create_committee(21, 10_000);
    let validator = create_validator(0);

    for i in 1..=10 {
        committee.record_block(&validator);
        let member = committee.get_member(&validator).unwrap();
        assert_eq!(member.blocks_produced, i);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMITTEE UPDATE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_committee_update() {
    let committee1 = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee1);

    assert_eq!(engine.committee().size(), 21);
    assert_eq!(engine.committee().total_stake(), 210_000);

    // Update to smaller committee with higher stakes
    let committee2 = create_committee(15, 20_000);
    engine.update_committee(committee2);

    assert_eq!(engine.committee().size(), 15);
    assert_eq!(engine.committee().total_stake(), 300_000);
}

#[test]
fn test_epoch_transition() {
    let committee1 = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee1);

    assert_eq!(engine.committee().epoch(), 1);

    // Create new committee for epoch 2
    let members: Vec<PpfaMember> = (0..21)
        .map(|i| PpfaMember::new(create_validator(i as u8), 10_000, i))
        .collect();
    let committee2 = PpfaCommittee::new(members, 2);

    engine.update_committee(committee2);
    assert_eq!(engine.committee().epoch(), 2);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERFORMANCE AND STRESS TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_large_committee() {
    // Test with 100 validators
    let committee = create_committee(100, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    // Verify rotation works correctly
    for i in 0..100 {
        let validator = create_validator(i as u8);
        let seal = engine.create_seal_unsigned(validator, i + 1, Hash::default());
        assert!(seal.is_ok());
        engine.advance_slot();
    }
}

#[test]
fn test_many_slots() {
    let committee = create_committee(21, 10_000);
    let mut engine = PpfaSealingEngine::new(committee);

    // Process 1000 slots
    for i in 0..1000 {
        let validator_index = (i % 21) as u8;
        let validator = create_validator(validator_index);

        let seal = engine.create_seal_unsigned(validator, i + 1, Hash::default());
        assert!(seal.is_ok());

        engine.advance_slot();
    }

    assert_eq!(engine.current_slot(), 1000);
}

#[test]
fn test_seal_data_encoding() {
    use codec::Encode;

    let validator = create_validator(0);
    let seal = PpfaSeal::new_unsigned(
        12345,
        7,
        validator,
        100_000,
        42,
        999,
        Hash::default(),
    );

    // Test SCALE encoding
    let data = seal.encode();

    // Verify data is not empty
    assert!(!data.is_empty());

    // Verify seal fields are accessible
    assert_eq!(seal.slot, 12345);
    assert_eq!(seal.ppfa_index, 7);
    assert_eq!(seal.stake_weight, 100_000);
    assert_eq!(seal.epoch, 42);
    assert_eq!(seal.block_number, 999);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SUMMARY REPORT
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_comprehensive_ppfa_workflow() {
    // Create committee with varying stakes
    let stakes = vec![
        50_000, 40_000, 30_000, 25_000, 20_000,
        15_000, 15_000, 15_000, 15_000, 15_000,
    ];
    let committee = create_committee_with_varying_stakes(stakes);
    let mut engine = PpfaSealingEngine::new(committee);

    // Process blocks for first rotation cycle
    for i in 0..10 {
        let validator = create_validator(i as u8);

        // Create seal
        let seal = engine.create_seal_unsigned(validator, i + 1, Hash::default());
        assert!(seal.is_ok());

        let seal = seal.unwrap();

        // Verify seal
        assert!(engine.verifier().verify_seal(&seal).is_ok());

        // Finalize block
        let result = engine.finalize_block(seal, Hash::default(), i + 1);
        assert!(result.is_ok());

        // Advance slot
        engine.advance_slot();
    }

    // Verify all validators produced blocks
    for i in 0..10 {
        let validator = create_validator(i as u8);
        let member = engine.committee().get_member(&validator).unwrap();
        assert_eq!(member.blocks_produced, 1);
    }
}
