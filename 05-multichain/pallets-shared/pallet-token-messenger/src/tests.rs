//! Tests for pallet-token-messenger

use crate::{mock::*, Error, pallet::*};
use codec::Encode;
use frame_support::{assert_noop, assert_ok};

#[test]
fn configure_domain_works() {
	new_test_ext().execute_with(|| {
		// Configure Ethereum domain
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0, // Ethereum
			true,
			1_000_000_000_000_000_000_000_u128, // 1000 tokens max
			10_000_000_000_000_000_000_000_u128, // 10000 tokens daily
			1_000_000_000_000_000_u128, // 0.001 min
		));

		// Check configuration
		let config = TokenMessenger::domain_config(0).unwrap();
		assert_eq!(config.enabled, true);
		assert_eq!(config.max_burn_amount, 1_000_000_000_000_000_000_000_u128);
		assert_eq!(config.daily_burn_limit, 10_000_000_000_000_000_000_000_u128);
		assert_eq!(config.min_burn_amount, 1_000_000_000_000_000_u128);

		// Check supported domains
		assert_eq!(TokenMessenger::supported_domains(0), true);
	});
}

#[test]
fn configure_domain_requires_root() {
	new_test_ext().execute_with(|| {
		// Non-root cannot configure
		assert_noop!(
			TokenMessenger::configure_domain(
				RuntimeOrigin::signed(1),
				0,
				true,
				1_000_000_000_000_000_000_000_u128,
				10_000_000_000_000_000_000_000_u128,
				1_000_000_000_000_000_u128,
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn deposit_for_burn_works() {
	new_test_ext().execute_with(|| {
		// Setup: Configure destination domain
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0, // Ethereum
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		// Burn tokens for cross-chain transfer
		let amount = 100_000_000_000_000_000_000_u128; // 100 tokens
		let recipient = vec![0x42u8; 20]; // Ethereum address

		assert_ok!(TokenMessenger::deposit_for_burn(
			RuntimeOrigin::signed(1),
			amount,
			0, // Ethereum
			recipient.clone(),
		));

		// Check nonce incremented
		assert_eq!(TokenMessenger::nonce(0), 1);

		// Check statistics
		assert_eq!(TokenMessenger::total_sent(), 1);
		assert_eq!(TokenMessenger::total_volume_sent(), amount);

		// Check daily volume
		let (_, volume) = TokenMessenger::daily_burn_volume(0);
		assert_eq!(volume, amount);

		// Check outbound message stored
		let message = TokenMessenger::outbound_messages(0).unwrap();
		assert_eq!(message.source_domain, 100); // LocalDomain
		assert_eq!(message.destination_domain, 0);
		assert_eq!(message.nonce, 0);
	});
}

#[test]
fn deposit_for_burn_enforces_limits() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0,
			true,
			1_000_000_000_000_000_000_000_u128, // 1000 max
			10_000_000_000_000_000_000_000_u128, // 10000 daily
			1_000_000_000_000_000_u128, // 0.001 min
		));

		let recipient = vec![0x42u8; 20];

		// Test minimum amount
		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				500_000_000_000_000_u128, // Below minimum
				0,
				recipient.clone(),
			),
			Error::<Test>::AmountBelowMin
		);

		// Test maximum amount
		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				2_000_000_000_000_000_000_000_u128, // Above max
				0,
				recipient.clone(),
			),
			Error::<Test>::AmountExceedsMax
		);

		// Test daily limit
		// First transfer should work (within max per-tx and daily)
		assert_ok!(TokenMessenger::deposit_for_burn(
			RuntimeOrigin::signed(1),
			900_000_000_000_000_000_000_u128, // 900 tokens (under 1000 max)
			0,
			recipient.clone(),
		));

		// Second transfer (total would be 1800, within daily 10000)
		assert_ok!(TokenMessenger::deposit_for_burn(
			RuntimeOrigin::signed(1),
			900_000_000_000_000_000_000_u128, // Another 900 tokens
			0,
			recipient.clone(),
		));

		// Keep adding until we hit daily limit (10000 total)
		// Already used: 1800, so we can do more 900s
		for _ in 0..9 {
			assert_ok!(TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				900_000_000_000_000_000_000_u128,
				0,
				recipient.clone(),
			));
		}
		// Total now: 1800 + (9 * 900) = 9900

		// Next transfer pushing over daily limit should fail
		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				200_000_000_000_000_000_000_u128, // 200 tokens (total 10100 > 10000)
				0,
				recipient.clone(),
			),
			Error::<Test>::DailyLimitExceeded
		);
	});
}

#[test]
fn deposit_for_burn_checks_domain() {
	new_test_ext().execute_with(|| {
		let recipient = vec![0x42u8; 20];

		// Unconfigured domain
		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				100_000_000_000_000_000_000_u128,
				999, // Not configured
				recipient.clone(),
			),
			Error::<Test>::InvalidDomain
		);

		// Configure but disable domain
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			1,
			false, // Disabled
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				100_000_000_000_000_000_000_u128,
				1,
				recipient.clone(),
			),
			Error::<Test>::DomainNotEnabled
		);
	});
}

#[test]
fn deposit_for_burn_prevents_self_transfer() {
	new_test_ext().execute_with(|| {
		// Configure local domain (should fail)
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			100, // Local domain
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		let recipient = vec![0x42u8; 20];

		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				100_000_000_000_000_000_000_u128,
				100, // Same as LocalDomain
				recipient,
			),
			Error::<Test>::SourceEqualsDestination
		);
	});
}

#[test]
fn receive_message_works() {
	new_test_ext().execute_with(|| {
		// Setup: Configure source domain as supported
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0, // Ethereum as source
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		// Create a valid message
		let amount = 100_000_000_000_000_000_000_u128;
		let recipient: u64 = 2;

		let burn_msg = BurnMessage {
			version: 1,
			burn_token: b"GENERIC".to_vec().try_into().unwrap(),
			mint_recipient: recipient.encode().try_into().unwrap(),
			amount,
			memo: Default::default(),
		};

		let message = CrossChainMessage {
			version: 1,
			source_domain: 0, // Ethereum
			destination_domain: 100, // LocalDomain
			nonce: 42,
			sender: vec![0x11u8; 20].try_into().unwrap(),
			recipient: recipient.encode().try_into().unwrap(),
			message_body: burn_msg.encode().try_into().unwrap(),
		};

		let encoded_message = message.encode();
		let attestation = vec![]; // Mock verifier accepts empty

		assert_ok!(TokenMessenger::receive_message(
			RuntimeOrigin::signed(1), // Relayer
			encoded_message,
			attestation,
		));

		// Check nonce marked as used
		assert_eq!(TokenMessenger::used_nonces(0, 42), true);

		// Check statistics
		assert_eq!(TokenMessenger::total_received(), 1);
		assert_eq!(TokenMessenger::total_volume_received(), amount);
	});
}

#[test]
fn receive_message_prevents_replay() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		let recipient: u64 = 2;
		let burn_msg = BurnMessage {
			version: 1,
			burn_token: b"GENERIC".to_vec().try_into().unwrap(),
			mint_recipient: recipient.encode().try_into().unwrap(),
			amount: 100_000_000_000_000_000_000_u128,
			memo: Default::default(),
		};

		let message = CrossChainMessage {
			version: 1,
			source_domain: 0,
			destination_domain: 100,
			nonce: 42,
			sender: vec![0x11u8; 20].try_into().unwrap(),
			recipient: recipient.encode().try_into().unwrap(),
			message_body: burn_msg.encode().try_into().unwrap(),
		};

		let encoded_message = message.encode();
		let attestation = vec![];

		// First message succeeds
		assert_ok!(TokenMessenger::receive_message(
			RuntimeOrigin::signed(1),
			encoded_message.clone(),
			attestation.clone(),
		));

		// Replay attempt fails
		assert_noop!(
			TokenMessenger::receive_message(
				RuntimeOrigin::signed(1),
				encoded_message,
				attestation,
			),
			Error::<Test>::MessageAlreadyProcessed
		);
	});
}

#[test]
fn receive_message_validates_destination() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		let recipient: u64 = 2;
		let burn_msg = BurnMessage {
			version: 1,
			burn_token: b"GENERIC".to_vec().try_into().unwrap(),
			mint_recipient: recipient.encode().try_into().unwrap(),
			amount: 100_000_000_000_000_000_000_u128,
			memo: Default::default(),
		};

		// Message for wrong destination
		let message = CrossChainMessage {
			version: 1,
			source_domain: 0,
			destination_domain: 1, // Solana, not LocalDomain (100)
			nonce: 42,
			sender: vec![0x11u8; 20].try_into().unwrap(),
			recipient: recipient.encode().try_into().unwrap(),
			message_body: burn_msg.encode().try_into().unwrap(),
		};

		let encoded_message = message.encode();
		let attestation = vec![];

		assert_noop!(
			TokenMessenger::receive_message(
				RuntimeOrigin::signed(1),
				encoded_message,
				attestation,
			),
			Error::<Test>::InvalidDestination
		);
	});
}

#[test]
fn pause_unpause_works() {
	new_test_ext().execute_with(|| {
		// Setup domain
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		let recipient = vec![0x42u8; 20];
		let amount = 100_000_000_000_000_000_000_u128;

		// Normal operation works
		assert_ok!(TokenMessenger::deposit_for_burn(
			RuntimeOrigin::signed(1),
			amount,
			0,
			recipient.clone(),
		));

		// Pause bridge
		assert_ok!(TokenMessenger::pause_bridge(RuntimeOrigin::root()));
		assert_eq!(TokenMessenger::is_paused(), true);

		// Operations should fail when paused
		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				amount,
				0,
				recipient.clone(),
			),
			Error::<Test>::BridgePaused
		);

		// Unpause bridge
		assert_ok!(TokenMessenger::unpause_bridge(RuntimeOrigin::root()));
		assert_eq!(TokenMessenger::is_paused(), false);

		// Operations work again
		assert_ok!(TokenMessenger::deposit_for_burn(
			RuntimeOrigin::signed(1),
			amount,
			0,
			recipient,
		));
	});
}

#[test]
fn pause_requires_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			TokenMessenger::pause_bridge(RuntimeOrigin::signed(1)),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn remove_domain_works() {
	new_test_ext().execute_with(|| {
		// Configure domain
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		assert!(TokenMessenger::domain_config(0).is_some());
		assert_eq!(TokenMessenger::supported_domains(0), true);

		// Remove domain
		assert_ok!(TokenMessenger::remove_domain(RuntimeOrigin::root(), 0));

		assert!(TokenMessenger::domain_config(0).is_none());
		assert_eq!(TokenMessenger::supported_domains(0), false);
	});
}

#[test]
fn daily_limit_resets() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0,
			true,
			6_000_000_000_000_000_000_000_u128, // 6000 max per tx
			10_000_000_000_000_000_000_000_u128, // 10000 daily
			1_000_000_000_000_000_u128,
		));

		let recipient = vec![0x42u8; 20];
		let amount = 6_000_000_000_000_000_000_000_u128; // 6000 tokens

		// First transfer
		assert_ok!(TokenMessenger::deposit_for_burn(
			RuntimeOrigin::signed(1),
			amount,
			0,
			recipient.clone(),
		));

		// Second transfer (total 12000, over limit)
		assert_noop!(
			TokenMessenger::deposit_for_burn(
				RuntimeOrigin::signed(1),
				amount,
				0,
				recipient.clone(),
			),
			Error::<Test>::DailyLimitExceeded
		);

		// Advance blocks past daily reset (14400 blocks)
		System::set_block_number(14401);

		// Should work now after reset
		assert_ok!(TokenMessenger::deposit_for_burn(
			RuntimeOrigin::signed(1),
			amount,
			0,
			recipient,
		));
	});
}

#[test]
fn get_message_hash_is_deterministic() {
	new_test_ext().execute_with(|| {
		let message = CrossChainMessage {
			version: 1,
			source_domain: 100,
			destination_domain: 0,
			nonce: 42,
			sender: vec![0x11u8; 20].try_into().unwrap(),
			recipient: vec![0x22u8; 20].try_into().unwrap(),
			message_body: vec![1, 2, 3].try_into().unwrap(),
		};

		let hash1 = TokenMessenger::get_message_hash(&message);
		let hash2 = TokenMessenger::get_message_hash(&message);

		assert_eq!(hash1, hash2);
	});
}
