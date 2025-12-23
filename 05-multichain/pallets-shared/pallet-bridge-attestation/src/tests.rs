//! Tests for pallet-bridge-attestation

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

#[test]
fn register_attester_works() {
	new_test_ext().execute_with(|| {
		// Create a test public key (32 bytes for SR25519)
		let public_key = vec![1u8; 32];

		// Register attester with root origin
		assert_ok!(BridgeAttestation::register_attester(
			RuntimeOrigin::root(),
			public_key.clone()
		));

		// Check event was emitted
		System::assert_last_event(
			Event::AttesterRegistered {
				attester_id: 0,
				public_key,
			}
			.into(),
		);

		// Verify attester was stored
		assert!(BridgeAttestation::attester(0).is_some());
		assert_eq!(BridgeAttestation::active_attester_count(), 1);
		assert_eq!(BridgeAttestation::next_attester_id(), 1);
	});
}

#[test]
fn register_duplicate_attester_fails() {
	new_test_ext().execute_with(|| {
		let public_key = vec![1u8; 32];

		// Register first time - should succeed
		assert_ok!(BridgeAttestation::register_attester(
			RuntimeOrigin::root(),
			public_key.clone()
		));

		// Try to register again - should fail
		assert_noop!(
			BridgeAttestation::register_attester(RuntimeOrigin::root(), public_key),
			Error::<Test>::AttesterAlreadyExists
		);
	});
}

#[test]
fn register_attester_invalid_key_fails() {
	new_test_ext().execute_with(|| {
		// Invalid key length (not 32 or 33)
		let invalid_key = vec![1u8; 20];

		assert_noop!(
			BridgeAttestation::register_attester(RuntimeOrigin::root(), invalid_key),
			Error::<Test>::InvalidPublicKey
		);
	});
}

#[test]
fn disable_attester_works() {
	new_test_ext().execute_with(|| {
		let public_key = vec![1u8; 32];

		// Register attester
		assert_ok!(BridgeAttestation::register_attester(
			RuntimeOrigin::root(),
			public_key
		));
		assert_eq!(BridgeAttestation::active_attester_count(), 1);

		// Disable attester
		assert_ok!(BridgeAttestation::disable_attester(RuntimeOrigin::root(), 0));
		assert_eq!(BridgeAttestation::active_attester_count(), 0);

		// Check event
		System::assert_last_event(
			Event::AttesterStatusChanged {
				attester_id: 0,
				old_status: 0, // Active
				new_status: 1, // Disabled
			}
			.into(),
		);
	});
}

#[test]
fn enable_attester_works() {
	new_test_ext().execute_with(|| {
		let public_key = vec![1u8; 32];

		// Register and disable attester
		assert_ok!(BridgeAttestation::register_attester(
			RuntimeOrigin::root(),
			public_key
		));
		assert_ok!(BridgeAttestation::disable_attester(RuntimeOrigin::root(), 0));
		assert_eq!(BridgeAttestation::active_attester_count(), 0);

		// Re-enable attester
		assert_ok!(BridgeAttestation::enable_attester(RuntimeOrigin::root(), 0));
		assert_eq!(BridgeAttestation::active_attester_count(), 1);

		// Check event
		System::assert_last_event(
			Event::AttesterStatusChanged {
				attester_id: 0,
				old_status: 1, // Disabled
				new_status: 0, // Active
			}
			.into(),
		);
	});
}

#[test]
fn remove_attester_works() {
	new_test_ext().execute_with(|| {
		let public_key = vec![1u8; 32];

		// Register attester
		assert_ok!(BridgeAttestation::register_attester(
			RuntimeOrigin::root(),
			public_key
		));
		assert_eq!(BridgeAttestation::active_attester_count(), 1);

		// Remove attester
		assert_ok!(BridgeAttestation::remove_attester(RuntimeOrigin::root(), 0));
		assert_eq!(BridgeAttestation::active_attester_count(), 0);

		// Verify attester was removed
		assert!(BridgeAttestation::attester(0).is_none());

		// Check event
		System::assert_last_event(Event::AttesterRemoved { attester_id: 0 }.into());
	});
}

#[test]
fn configure_threshold_works() {
	new_test_ext().execute_with(|| {
		// Set global threshold
		assert_ok!(BridgeAttestation::configure_threshold(
			RuntimeOrigin::root(),
			None, // Global
			3,    // min_signatures
			5     // total_attesters
		));

		// Check event
		System::assert_last_event(
			Event::ThresholdConfigUpdated {
				domain_id: None,
				min_signatures: 3,
				total_attesters: 5,
			}
			.into(),
		);

		// Verify threshold was stored
		let config = BridgeAttestation::global_threshold().unwrap();
		assert_eq!(config.min_signatures, 3);
		assert_eq!(config.total_attesters, 5);
		assert!(config.enabled);
	});
}

#[test]
fn configure_invalid_threshold_fails() {
	new_test_ext().execute_with(|| {
		// min_signatures > total_attesters
		assert_noop!(
			BridgeAttestation::configure_threshold(RuntimeOrigin::root(), None, 10, 5),
			Error::<Test>::InvalidThreshold
		);

		// min_signatures = 0
		assert_noop!(
			BridgeAttestation::configure_threshold(RuntimeOrigin::root(), None, 0, 5),
			Error::<Test>::InvalidThreshold
		);
	});
}

#[test]
fn pause_and_unpause_works() {
	new_test_ext().execute_with(|| {
		// Initially not paused
		assert!(!BridgeAttestation::is_paused());

		// Pause
		assert_ok!(BridgeAttestation::pause_attestation(RuntimeOrigin::root()));
		assert!(BridgeAttestation::is_paused());
		System::assert_last_event(Event::AttestationPaused.into());

		// Unpause
		assert_ok!(BridgeAttestation::unpause_attestation(RuntimeOrigin::root()));
		assert!(!BridgeAttestation::is_paused());
		System::assert_last_event(Event::AttestationUnpaused.into());
	});
}

#[test]
fn hash_message_works() {
	new_test_ext().execute_with(|| {
		let message = b"test message";
		let hash = BridgeAttestation::hash_message(message);

		// Verify it returns a non-zero hash
		assert_ne!(hash, H256::zero());

		// Same message should produce same hash
		let hash2 = BridgeAttestation::hash_message(message);
		assert_eq!(hash, hash2);

		// Different message should produce different hash
		let different_message = b"different message";
		let hash3 = BridgeAttestation::hash_message(different_message);
		assert_ne!(hash, hash3);
	});
}

#[test]
fn get_chain_id_works() {
	new_test_ext().execute_with(|| {
		// Should return the configured chain ID (1 in mock)
		assert_eq!(BridgeAttestation::get_chain_id(), 1);
	});
}

#[test]
fn get_and_increment_nonce_works() {
	new_test_ext().execute_with(|| {
		// First nonce should be 0
		let nonce1 = BridgeAttestation::get_and_increment_nonce();
		assert_eq!(nonce1, 0);

		// Next nonce should be 1
		let nonce2 = BridgeAttestation::get_and_increment_nonce();
		assert_eq!(nonce2, 1);

		// Next nonce should be 2
		let nonce3 = BridgeAttestation::get_and_increment_nonce();
		assert_eq!(nonce3, 2);
	});
}

#[test]
fn update_threshold_works() {
	new_test_ext().execute_with(|| {
		// Register some attesters first
		for i in 0..5 {
			let mut public_key = vec![0u8; 32];
			public_key[0] = i as u8;
			assert_ok!(BridgeAttestation::register_attester(
				RuntimeOrigin::root(),
				public_key
			));
		}
		assert_eq!(BridgeAttestation::active_attester_count(), 5);

		// Update threshold
		assert_ok!(BridgeAttestation::update_threshold(RuntimeOrigin::root(), 3));

		// Check event
		System::assert_last_event(Event::ThresholdUpdated { new_threshold: 3 }.into());

		// Verify threshold was updated
		let config = BridgeAttestation::global_threshold().unwrap();
		assert_eq!(config.min_signatures, 3);
		assert_eq!(config.total_attesters, 5);
	});
}

#[test]
fn update_threshold_invalid_fails() {
	new_test_ext().execute_with(|| {
		// Register 3 attesters
		for i in 0..3 {
			let mut public_key = vec![0u8; 32];
			public_key[0] = i as u8;
			assert_ok!(BridgeAttestation::register_attester(
				RuntimeOrigin::root(),
				public_key
			));
		}

		// Try to set threshold higher than active count
		assert_noop!(
			BridgeAttestation::update_threshold(RuntimeOrigin::root(), 5),
			Error::<Test>::InvalidThreshold
		);

		// Try to set threshold to 0
		assert_noop!(
			BridgeAttestation::update_threshold(RuntimeOrigin::root(), 0),
			Error::<Test>::InvalidThreshold
		);
	});
}

#[test]
fn get_active_attesters_works() {
	new_test_ext().execute_with(|| {
		// Register 5 attesters
		for i in 0..5 {
			let mut public_key = vec![0u8; 32];
			public_key[0] = i as u8;
			assert_ok!(BridgeAttestation::register_attester(
				RuntimeOrigin::root(),
				public_key
			));
		}

		// Get active attesters
		let active = BridgeAttestation::get_active_attesters();
		assert_eq!(active.len(), 5);
		assert_eq!(active, vec![0, 1, 2, 3, 4]);

		// Disable one attester
		assert_ok!(BridgeAttestation::disable_attester(RuntimeOrigin::root(), 2));

		// Get active attesters again
		let active = BridgeAttestation::get_active_attesters();
		assert_eq!(active.len(), 4);
		assert_eq!(active, vec![0, 1, 3, 4]);
	});
}
