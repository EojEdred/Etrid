//! Tests for pallet-asf

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_asf::{ConsensusPhase, FinalityLevel, SlashingSeverity};

// ═══════════════════════════════════════════════════════════════════════════
// VOTE SUBMISSION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_submit_vote_works() {
    new_test_ext().execute_with(|| {
        let validator = validator_id(1);
        let block_hash = test_block_hash(1);
        let vote = create_test_vote(
            validator.clone(),
            block_hash,
            ConsensusPhase::Prepare,
            10_000_000,
        );

        // Submit vote should succeed
        assert_ok!(Asf::submit_vote(RuntimeOrigin::signed(validator.clone()), vote.clone()));

        // Verify event was emitted
        System::assert_has_event(
            Event::VoteSubmitted {
                validator: validator.clone(),
                block_hash,
                phase: ConsensusPhase::Prepare,
                epoch: 0,
            }
            .into(),
        );

        // Verify vote was stored
        let votes = Asf::votes(block_hash, ConsensusPhase::Prepare);
        assert_eq!(votes.len(), 1);
        assert_eq!(votes[0].validator, validator);
    });
}

#[test]
fn test_submit_duplicate_vote_fails() {
    new_test_ext().execute_with(|| {
        let validator = validator_id(1);
        let block_hash = test_block_hash(1);
        let vote = create_test_vote(
            validator.clone(),
            block_hash,
            ConsensusPhase::Prepare,
            10_000_000,
        );

        // First vote should succeed
        assert_ok!(Asf::submit_vote(RuntimeOrigin::signed(validator.clone()), vote.clone()));

        // Second vote should fail with DuplicateVote
        assert_noop!(
            Asf::submit_vote(RuntimeOrigin::signed(validator.clone()), vote.clone()),
            Error::<Test>::DuplicateVote
        );
    });
}

#[test]
fn test_submit_vote_not_validator_fails() {
    new_test_ext().execute_with(|| {
        let non_validator = validator_id(99); // Not in genesis
        let block_hash = test_block_hash(1);
        let vote = create_test_vote(
            non_validator.clone(),
            block_hash,
            ConsensusPhase::Prepare,
            10_000_000,
        );

        // Should fail with NotValidator
        assert_noop!(
            Asf::submit_vote(RuntimeOrigin::signed(non_validator), vote),
            Error::<Test>::NotValidator
        );
    });
}

#[test]
fn test_submit_vote_invalid_stake_fails() {
    new_test_ext().execute_with(|| {
        let validator = validator_id(1);
        let block_hash = test_block_hash(1);
        // Create vote with incorrect stake (actual is 10_000_000)
        let vote = create_test_vote(
            validator.clone(),
            block_hash,
            ConsensusPhase::Prepare,
            5_000_000, // Wrong stake
        );

        // Should fail with InvalidVote
        assert_noop!(
            Asf::submit_vote(RuntimeOrigin::signed(validator), vote),
            Error::<Test>::InvalidVote
        );
    });
}

#[test]
fn test_consensus_threshold_reached() {
    new_test_ext().execute_with(|| {
        let block_hash = test_block_hash(1);

        // Submit votes from 15 validators (BFT threshold for 21 is 15)
        for i in 1..=15 {
            let validator = validator_id(i);
            let vote = create_test_vote(
                validator.clone(),
                block_hash,
                ConsensusPhase::Prepare,
                10_000_000,
            );
            assert_ok!(Asf::submit_vote(RuntimeOrigin::signed(validator), vote));
        }

        // Verify threshold event was emitted
        System::assert_has_event(
            Event::ThresholdMet {
                block_hash,
                phase: ConsensusPhase::Prepare,
                vote_count: 15,
                total_stake: 150_000_000,
            }
            .into(),
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// CERTIFICATE SUBMISSION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_submit_certificate_works() {
    new_test_ext().execute_with(|| {
        let block_hash = test_block_hash(1);
        let issuer = validator_id(1);

        // Create votes from 15 validators (meets BFT threshold)
        let mut votes = Vec::new();
        for i in 1..=15 {
            let vote = create_test_vote(
                validator_id(i),
                block_hash,
                ConsensusPhase::Prepare,
                10_000_000,
            );
            votes.push(vote);
        }

        // Create certificate
        let cert = create_test_certificate(
            issuer.clone(),
            block_hash,
            ConsensusPhase::Prepare,
            &votes,
        );

        // Submit certificate should succeed
        assert_ok!(Asf::submit_certificate(RuntimeOrigin::signed(issuer.clone()), cert.clone()));

        // Verify event was emitted
        System::assert_has_event(
            Event::CertificateGenerated {
                block_hash,
                phase: ConsensusPhase::Prepare,
                validator: issuer,
                certificate_count: 1,
            }
            .into(),
        );

        // Verify certificate was stored
        let certs = Asf::pending_certificates(block_hash);
        assert_eq!(certs.len(), 1);
    });
}

#[test]
fn test_certificate_invalid_threshold_fails() {
    new_test_ext().execute_with(|| {
        let block_hash = test_block_hash(1);
        let issuer = validator_id(1);

        // Create votes from only 10 validators (below BFT threshold of 15)
        let mut votes = Vec::new();
        for i in 1..=10 {
            let vote = create_test_vote(
                validator_id(i),
                block_hash,
                ConsensusPhase::Prepare,
                10_000_000,
            );
            votes.push(vote);
        }

        // Create certificate with insufficient votes
        let cert = create_test_certificate(
            issuer.clone(),
            block_hash,
            ConsensusPhase::Prepare,
            &votes,
        );

        // Should fail with InvalidCertificate
        assert_noop!(
            Asf::submit_certificate(RuntimeOrigin::signed(issuer), cert),
            Error::<Test>::InvalidCertificate
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// FINALITY LEVEL TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_finality_level_progression() {
    new_test_ext().execute_with(|| {
        let block_hash = test_block_hash(1);
        let issuer = validator_id(1);

        // Helper to submit a certificate
        let submit_cert = |count: u32| {
            let mut votes = Vec::new();
            for i in 1..=15 {
                let vote = create_test_vote(
                    validator_id(i),
                    block_hash,
                    ConsensusPhase::Prepare,
                    10_000_000,
                );
                votes.push(vote);
            }
            let cert = create_test_certificate(issuer.clone(), block_hash, ConsensusPhase::Prepare, &votes);
            assert_ok!(Asf::submit_certificate(RuntimeOrigin::signed(issuer.clone()), cert));
            count
        };

        // Initial finality: None
        assert_eq!(Asf::block_finality(block_hash), FinalityLevel::None);

        // Submit 10 certificates -> Weak finality
        for _ in 0..10 {
            submit_cert(1);
        }
        assert_eq!(Asf::block_finality(block_hash), FinalityLevel::Weak);

        // Submit 10 more (20 total) -> Moderate finality
        for _ in 0..10 {
            submit_cert(1);
        }
        assert_eq!(Asf::block_finality(block_hash), FinalityLevel::Moderate);

        // Note: We can't easily reach Strong/Irreversible in unit tests
        // due to MAX_PENDING_CERTIFICATES limit
    });
}

#[test]
fn test_finality_level_changed_event() {
    new_test_ext().execute_with(|| {
        let block_hash = test_block_hash(1);
        let issuer = validator_id(1);

        // Submit 10 certificates to reach Weak finality
        for _ in 0..10 {
            let mut votes = Vec::new();
            for i in 1..=15 {
                let vote = create_test_vote(
                    validator_id(i),
                    block_hash,
                    ConsensusPhase::Prepare,
                    10_000_000,
                );
                votes.push(vote);
            }
            let cert = create_test_certificate(issuer.clone(), block_hash, ConsensusPhase::Prepare, &votes);
            assert_ok!(Asf::submit_certificate(RuntimeOrigin::signed(issuer.clone()), cert));
        }

        // Verify finality level changed event
        System::assert_has_event(
            Event::FinalityLevelChanged {
                block_hash,
                old_level: FinalityLevel::None,
                new_level: FinalityLevel::Weak,
            }
            .into(),
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// VALIDATOR ROTATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_rotate_validators_works() {
    new_test_ext().execute_with(|| {
        // New validator set (5 validators)
        let new_validators = vec![
            (validator_id(30), 20_000_000),
            (validator_id(31), 20_000_000),
            (validator_id(32), 20_000_000),
            (validator_id(33), 20_000_000),
            (validator_id(34), 20_000_000),
        ];

        // Rotate validators (requires root)
        assert_ok!(Asf::rotate_validators(RuntimeOrigin::root(), new_validators.clone()));

        // Verify new validators are active
        assert_eq!(Asf::validator_set().len(), 5);
        assert_eq!(Asf::validators(validator_id(30)), Some(20_000_000));
        assert_eq!(Asf::validators(validator_id(1)), None); // Old validator removed

        // Verify epoch incremented
        assert_eq!(Asf::current_epoch(), 1);

        // Verify event
        System::assert_has_event(
            Event::ValidatorSetRotated {
                epoch: 1,
                validator_count: 5,
            }
            .into(),
        );
    });
}

#[test]
fn test_rotate_validators_requires_root() {
    new_test_ext().execute_with(|| {
        let new_validators = vec![(validator_id(30), 20_000_000)];

        // Should fail if not root
        assert_noop!(
            Asf::rotate_validators(RuntimeOrigin::signed(validator_id(1)), new_validators),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// SLASHING TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_slash_validator_works() {
    new_test_ext().execute_with(|| {
        let validator = validator_id(1);
        let reason = b"Byzantine behavior detected".to_vec();

        // Slash validator with moderate severity
        assert_ok!(Asf::slash_validator(
            RuntimeOrigin::root(),
            validator.clone(),
            SlashingSeverity::Moderate,
            reason.clone()
        ));

        // Verify slashing event
        System::assert_has_event(
            Event::ValidatorSlashed {
                validator: validator.clone(),
                amount: 1_500_000, // 15% of 10_000_000
                severity: SlashingSeverity::Moderate,
                reason: alloc::string::String::from_utf8_lossy(&reason).into(),
            }
            .into(),
        );

        // Verify slashing record stored
        assert!(Asf::slashed_validators(validator).is_some());
    });
}

#[test]
fn test_slash_validator_critical_excludes() {
    new_test_ext().execute_with(|| {
        let validator = validator_id(1);
        let reason = b"Critical Byzantine attack".to_vec();

        // Verify validator is active
        assert!(Asf::validators(validator.clone()).is_some());

        // Slash with Critical severity
        assert_ok!(Asf::slash_validator(
            RuntimeOrigin::root(),
            validator.clone(),
            SlashingSeverity::Critical,
            reason.clone()
        ));

        // Verify validator was excluded
        assert!(Asf::validators(validator.clone()).is_none());

        // Verify exclusion event
        System::assert_has_event(
            Event::ValidatorExcluded {
                validator: validator.clone(),
                reason: alloc::string::String::from_utf8_lossy(&reason).into(),
            }
            .into(),
        );
    });
}

#[test]
fn test_slashed_validator_cannot_vote() {
    new_test_ext().execute_with(|| {
        let validator = validator_id(1);
        let block_hash = test_block_hash(1);

        // Slash validator
        assert_ok!(Asf::slash_validator(
            RuntimeOrigin::root(),
            validator.clone(),
            SlashingSeverity::Moderate,
            b"Misbehavior".to_vec()
        ));

        // Try to submit vote - should fail
        let vote = create_test_vote(
            validator.clone(),
            block_hash,
            ConsensusPhase::Prepare,
            10_000_000,
        );

        assert_noop!(
            Asf::submit_vote(RuntimeOrigin::signed(validator), vote),
            Error::<Test>::ValidatorSlashed
        );
    });
}

#[test]
fn test_slash_requires_root() {
    new_test_ext().execute_with(|| {
        let validator = validator_id(1);

        // Should fail if not root
        assert_noop!(
            Asf::slash_validator(
                RuntimeOrigin::signed(validator_id(2)),
                validator,
                SlashingSeverity::Minor,
                b"Test".to_vec()
            ),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// EPOCH ROTATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_epoch_rotation_on_initialize() {
    new_test_ext().execute_with(|| {
        // Initial epoch is 0
        assert_eq!(Asf::current_epoch(), 0);

        // Run blocks up to epoch boundary (100 blocks)
        for i in 1..=100 {
            System::set_block_number(i);
            Asf::on_initialize(i);
        }

        // Epoch should have incremented
        assert_eq!(Asf::current_epoch(), 1);
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// GENESIS CONFIGURATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_genesis_validators_initialized() {
    new_test_ext().execute_with(|| {
        // Verify 21 validators were initialized
        assert_eq!(Asf::validator_set().len(), 21);

        // Verify first validator
        assert_eq!(Asf::validators(validator_id(1)), Some(10_000_000));

        // Verify last validator
        assert_eq!(Asf::validators(validator_id(21)), Some(10_000_000));

        // Verify total stake
        assert_eq!(Asf::total_stake(), 210_000_000); // 21 * 10_000_000
    });
}

#[test]
fn test_genesis_epoch_zero() {
    new_test_ext().execute_with(|| {
        assert_eq!(Asf::current_epoch(), 0);
    });
}

#[test]
fn test_genesis_finality_none() {
    new_test_ext().execute_with(|| {
        assert_eq!(Asf::current_finality_level(), FinalityLevel::None);
    });
}
