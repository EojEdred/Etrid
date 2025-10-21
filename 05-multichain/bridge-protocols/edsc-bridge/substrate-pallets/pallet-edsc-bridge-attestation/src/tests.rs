//! Unit tests for pallet-edsc-bridge-attestation

#![cfg(test)]

use crate::{mock::*, *};
use frame_support::{assert_ok, assert_noop, BoundedVec};
use sp_core::{H256, ConstU32};

// Type alias to avoid collision with Attestation struct
type AttestationPallet = crate::Pallet<Test>;

// Helper function to generate test public keys
fn test_pubkey(id: u8) -> Vec<u8> {
	let mut key = vec![0u8; 32];
	key[0] = id;
	key
}

#[test]
fn register_attester_works() {
	new_test_ext().execute_with(|| {
		let pubkey = test_pubkey(1);

		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey.clone()
		));

		// Check attester was registered
		let attester = AttestationPallet::attester(0).unwrap();
		assert_eq!(attester.status, AttesterStatus::Active);
		assert_eq!(attester.messages_signed, 0);

		// Check mapping exists
		let bounded_key: BoundedVec<u8, ConstU32<64>> = pubkey.try_into().unwrap();
		assert_eq!(AttestationPallet::attester_by_pubkey(&bounded_key), Some(0));

		// Check counter incremented
		assert_eq!(AttestationPallet::next_attester_id(), 1);
		assert_eq!(AttestationPallet::active_attester_count(), 1);
	});
}

#[test]
fn register_attester_duplicate_fails() {
	new_test_ext().execute_with(|| {
		let pubkey = test_pubkey(1);

		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey.clone()
		));

		// Try to register again
		assert_noop!(
			AttestationPallet::register_attester(RuntimeOrigin::root(), pubkey),
			Error::<Test>::AttesterAlreadyExists
		);
	});
}

#[test]
fn register_attester_invalid_key_fails() {
	new_test_ext().execute_with(|| {
		// Invalid key length (not 32 or 33 bytes)
		let invalid_key = vec![0u8; 16];

		assert_noop!(
			AttestationPallet::register_attester(RuntimeOrigin::root(), invalid_key),
			Error::<Test>::InvalidPublicKey
		);
	});
}

#[test]
fn disable_attester_works() {
	new_test_ext().execute_with(|| {
		let pubkey = test_pubkey(1);

		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey
		));

		assert_eq!(AttestationPallet::active_attester_count(), 1);

		assert_ok!(AttestationPallet::disable_attester(RuntimeOrigin::root(), 0));

		let attester = AttestationPallet::attester(0).unwrap();
		assert_eq!(attester.status, AttesterStatus::Disabled);
		assert_eq!(AttestationPallet::active_attester_count(), 0);
	});
}

#[test]
fn enable_attester_works() {
	new_test_ext().execute_with(|| {
		let pubkey = test_pubkey(1);

		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey
		));

		assert_ok!(AttestationPallet::disable_attester(RuntimeOrigin::root(), 0));
		assert_eq!(AttestationPallet::active_attester_count(), 0);

		assert_ok!(AttestationPallet::enable_attester(RuntimeOrigin::root(), 0));

		let attester = AttestationPallet::attester(0).unwrap();
		assert_eq!(attester.status, AttesterStatus::Active);
		assert_eq!(AttestationPallet::active_attester_count(), 1);
	});
}

#[test]
fn remove_attester_works() {
	new_test_ext().execute_with(|| {
		let pubkey = test_pubkey(1);

		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey.clone()
		));

		assert_eq!(AttestationPallet::active_attester_count(), 1);

		assert_ok!(AttestationPallet::remove_attester(RuntimeOrigin::root(), 0));

		// Check attester removed
		assert_eq!(AttestationPallet::attester(0), None);

		let bounded_key: BoundedVec<u8, ConstU32<64>> = pubkey.try_into().unwrap();
		assert_eq!(AttestationPallet::attester_by_pubkey(&bounded_key), None);

		assert_eq!(AttestationPallet::active_attester_count(), 0);
	});
}

#[test]
fn submit_signature_works() {
	new_test_ext().execute_with(|| {
		// Register attester
		let pubkey = test_pubkey(1);
		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey
		));

		// Submit signature
		let message_hash = H256::from([1u8; 32]);
		let signature = vec![0u8; 65];

		assert_ok!(AttestationPallet::submit_signature(
			RuntimeOrigin::signed(1),
			0,  // attester_id
			message_hash,
			signature
		));

		// Check attestation created
		let attestation = AttestationPallet::attestation(message_hash).unwrap();
		assert_eq!(attestation.signature_count, 1);
		assert_eq!(attestation.signatures.len(), 1);

		// Check attester stats updated
		let attester = AttestationPallet::attester(0).unwrap();
		assert_eq!(attester.messages_signed, 1);
	});
}

#[test]
fn submit_signature_duplicate_fails() {
	new_test_ext().execute_with(|| {
		let pubkey = test_pubkey(1);
		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey
		));

		let message_hash = H256::from([1u8; 32]);
		let signature = vec![0u8; 65];

		assert_ok!(AttestationPallet::submit_signature(
			RuntimeOrigin::signed(1),
			0,
			message_hash,
			signature.clone()
		));

		// Try to submit again
		assert_noop!(
			AttestationPallet::submit_signature(
				RuntimeOrigin::signed(1),
				0,
				message_hash,
				signature
			),
			Error::<Test>::SignatureAlreadySubmitted
		);
	});
}

#[test]
fn submit_signature_disabled_attester_fails() {
	new_test_ext().execute_with(|| {
		let pubkey = test_pubkey(1);
		assert_ok!(AttestationPallet::register_attester(
			RuntimeOrigin::root(),
			pubkey
		));

		assert_ok!(AttestationPallet::disable_attester(RuntimeOrigin::root(), 0));

		let message_hash = H256::from([1u8; 32]);
		let signature = vec![0u8; 65];

		assert_noop!(
			AttestationPallet::submit_signature(
				RuntimeOrigin::signed(1),
				0,
				message_hash,
				signature
			),
			Error::<Test>::AttesterNotActive
		);
	});
}

#[test]
fn multiple_attesters_can_sign_same_message() {
	new_test_ext().execute_with(|| {
		// Register 5 attesters
		for i in 1..=5 {
			assert_ok!(AttestationPallet::register_attester(
				RuntimeOrigin::root(),
				test_pubkey(i)
			));
		}

		let message_hash = H256::from([1u8; 32]);

		// Each attester signs
		for i in 0..5 {
			let signature = vec![i as u8; 65];
			assert_ok!(AttestationPallet::submit_signature(
				RuntimeOrigin::signed(1),
				i,
				message_hash,
				signature
			));
		}

		// Check attestation has all signatures
		let attestation = AttestationPallet::attestation(message_hash).unwrap();
		assert_eq!(attestation.signature_count, 5);
		assert_eq!(attestation.signatures.len(), 5);
	});
}

#[test]
fn configure_threshold_works() {
	new_test_ext().execute_with(|| {
		// Configure global threshold
		assert_ok!(AttestationPallet::configure_threshold(
			RuntimeOrigin::root(),
			None,  // Global
			3,     // min_signatures
			5      // total_attesters
		));

		let config = AttestationPallet::global_threshold().unwrap();
		assert_eq!(config.min_signatures, 3);
		assert_eq!(config.total_attesters, 5);
		assert_eq!(config.enabled, true);
	});
}

#[test]
fn configure_threshold_per_domain_works() {
	new_test_ext().execute_with(|| {
		// Configure domain-specific threshold
		assert_ok!(AttestationPallet::configure_threshold(
			RuntimeOrigin::root(),
			Some(0),  // Ethereum
			4,        // min_signatures
			7         // total_attesters
		));

		let config = AttestationPallet::threshold_config(0).unwrap();
		assert_eq!(config.min_signatures, 4);
		assert_eq!(config.total_attesters, 7);
	});
}

#[test]
fn configure_threshold_invalid_fails() {
	new_test_ext().execute_with(|| {
		// min_signatures > total_attesters
		assert_noop!(
			AttestationPallet::configure_threshold(
				RuntimeOrigin::root(),
				None,
				10,  // min
				5    // total
			),
			Error::<Test>::InvalidThreshold
		);

		// min_signatures = 0
		assert_noop!(
			AttestationPallet::configure_threshold(
				RuntimeOrigin::root(),
				None,
				0,  // min
				5   // total
			),
			Error::<Test>::InvalidThreshold
		);
	});
}

#[test]
fn verify_attestation_works() {
	new_test_ext().execute_with(|| {
		// Register 5 attesters
		for i in 1..=5 {
			assert_ok!(AttestationPallet::register_attester(
				RuntimeOrigin::root(),
				test_pubkey(i)
			));
		}

		// Configure threshold (3-of-5)
		assert_ok!(AttestationPallet::configure_threshold(
			RuntimeOrigin::root(),
			None,
			3,
			5
		));

		let message = b"test message".to_vec();
		let message_hash = AttestationPallet::hash_message(&message);

		// Submit 3 signatures (meets threshold)
		for i in 0..3 {
			let signature = vec![i as u8; 65];
			assert_ok!(AttestationPallet::submit_signature(
				RuntimeOrigin::signed(1),
				i,
				message_hash,
				signature
			));
		}

		// Verify attestation
		assert_ok!(AttestationPallet::verify_attestation(
			RuntimeOrigin::signed(1),
			message,
			message_hash
		));

		assert_eq!(AttestationPallet::total_attestations(), 1);
	});
}

#[test]
fn verify_attestation_insufficient_signatures_fails() {
	new_test_ext().execute_with(|| {
		// Register 5 attesters
		for i in 1..=5 {
			assert_ok!(AttestationPallet::register_attester(
				RuntimeOrigin::root(),
				test_pubkey(i)
			));
		}

		// Configure threshold (3-of-5)
		assert_ok!(AttestationPallet::configure_threshold(
			RuntimeOrigin::root(),
			None,
			3,
			5
		));

		let message = b"test message".to_vec();
		let message_hash = AttestationPallet::hash_message(&message);

		// Submit only 2 signatures (below threshold)
		for i in 0..2 {
			let signature = vec![i as u8; 65];
			assert_ok!(AttestationPallet::submit_signature(
				RuntimeOrigin::signed(1),
				i,
				message_hash,
				signature
			));
		}

		// Verify attestation should fail
		assert_noop!(
			AttestationPallet::verify_attestation(
				RuntimeOrigin::signed(1),
				message,
				message_hash
			),
			Error::<Test>::InsufficientSignatures
		);
	});
}

#[test]
fn verify_attestation_hash_mismatch_fails() {
	new_test_ext().execute_with(|| {
		let message = b"test message".to_vec();
		let wrong_hash = H256::from([99u8; 32]);

		assert_noop!(
			AttestationPallet::verify_attestation(
				RuntimeOrigin::signed(1),
				message,
				wrong_hash
			),
			Error::<Test>::MessageHashMismatch
		);
	});
}

#[test]
fn pause_unpause_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(AttestationPallet::is_paused(), false);

		assert_ok!(AttestationPallet::pause_attestation(RuntimeOrigin::root()));
		assert_eq!(AttestationPallet::is_paused(), true);

		assert_ok!(AttestationPallet::unpause_attestation(RuntimeOrigin::root()));
		assert_eq!(AttestationPallet::is_paused(), false);
	});
}

#[test]
fn paused_blocks_operations() {
	new_test_ext().execute_with(|| {
		assert_ok!(AttestationPallet::pause_attestation(RuntimeOrigin::root()));

		// Cannot register attester
		assert_noop!(
			AttestationPallet::register_attester(
				RuntimeOrigin::root(),
				test_pubkey(1)
			),
			Error::<Test>::AttestationPaused
		);
	});
}

#[test]
fn hash_message_is_deterministic() {
	new_test_ext().execute_with(|| {
		let message = b"test message".to_vec();

		let hash1 = AttestationPallet::hash_message(&message);
		let hash2 = AttestationPallet::hash_message(&message);

		assert_eq!(hash1, hash2);
	});
}

#[test]
fn hash_message_different_messages_different_hashes() {
	new_test_ext().execute_with(|| {
		let message1 = b"test message 1".to_vec();
		let message2 = b"test message 2".to_vec();

		let hash1 = AttestationPallet::hash_message(&message1);
		let hash2 = AttestationPallet::hash_message(&message2);

		assert_ne!(hash1, hash2);
	});
}
