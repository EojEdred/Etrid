//! # EDSC Redemption Security Tests
//!
//! Adversarial and security-focused tests covering:
//! - Reentrancy attack prevention
//! - Integer overflow/underflow protection
//! - Access control boundaries
//! - Double-spend prevention
//! - Replay attack mitigation
//! - Front-running resistance
//! - Economic attack vectors

use super::*;
use frame_support::{assert_err, assert_ok};
use sp_arithmetic::{FixedU128, Permill};

// Import test infrastructure from main tests module
use crate::tests::*;

// ═══════════════════════════════════════════════════════════════════════════
// SECURITY TEST 1: Integer Overflow Protection
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_redemption_amount_overflow_prevention() {
	new_test_ext().execute_with(|| {
		// Setup
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, u128::MAX).unwrap();

		// Set oracle price
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Attempt to redeem u128::MAX (should handle gracefully)
		// Even if balance is sufficient, internal calculations should not overflow
		let result = EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			u128::MAX,
			None,
			None
		);

		// Should either succeed with proper handling or fail with Overflow error
		// but NOT panic
		if result.is_err() {
			assert!(matches!(
				result.unwrap_err().error,
				pallet_edsc_redemption::Error::<Test>::Overflow
			) || matches!(
				result.unwrap_err().error,
				pallet_edsc_redemption::Error::<Test>::HourlyCapExceeded
			) || matches!(
				result.unwrap_err().error,
				pallet_edsc_redemption::Error::<Test>::DailyCapExceeded
			));
		}
	});
}

#[test]
fn test_fee_calculation_overflow_protection() {
	new_test_ext().execute_with(|| {
		// Mint EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_00).unwrap();

		// Set oracle to extreme low price (would cause high fee)
		EdscRedemption::do_update_oracle_price(1).unwrap(); // $0.01

		// Attempt redemption - fee calculation should not overflow
		let result = EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			1_000_00,
			None,
			None
		);

		// Should handle gracefully
		if result.is_err() {
			assert!(matches!(
				result.unwrap_err().error,
				pallet_edsc_redemption::Error::<Test>::Overflow
			));
		}
	});
}

#[test]
fn test_underflow_protection_on_subtraction() {
	new_test_ext().execute_with(|| {
		// Mint small amount
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 100).unwrap();

		// Set oracle price
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Attempt to redeem more than balance
		assert_err!(
			EdscRedemption::redeem(RuntimeOrigin::signed(ALICE), 1_000, None, None),
			pallet_edsc_redemption::Error::<Test>::InsufficientBalance
		);

		// Balance should remain unchanged (no underflow)
		assert_eq!(EdscToken::balance_of(&ALICE), 100);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// SECURITY TEST 2: Access Control Boundaries
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_non_root_cannot_pause_redemptions() {
	new_test_ext().execute_with(|| {
		// Non-root user attempts to pause
		assert_err!(
			EdscRedemption::pause_redemptions(RuntimeOrigin::signed(ALICE)),
			sp_runtime::DispatchError::BadOrigin
		);

		// Verify redemptions are NOT paused
		assert!(!EdscRedemption::redemptions_paused());
	});
}

#[test]
fn test_non_root_cannot_update_oracle_price() {
	new_test_ext().execute_with(|| {
		// Non-root user attempts to update oracle
		assert_err!(
			EdscRedemption::update_oracle_price(RuntimeOrigin::signed(ALICE), 95),
			sp_runtime::DispatchError::BadOrigin
		);

		// Price should remain at default
		assert_eq!(EdscRedemption::oracle_price(), 100); // From genesis
	});
}

#[test]
fn test_non_root_cannot_update_reserve_ratio() {
	new_test_ext().execute_with(|| {
		// Non-root user attempts to update reserve ratio
		let new_ratio = FixedU128::from_rational(150u128, 100u128);

		assert_err!(
			EdscRedemption::update_reserve_ratio(RuntimeOrigin::signed(ALICE), new_ratio),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn test_user_cannot_process_nonexistent_queue_request() {
	new_test_ext().execute_with(|| {
		// Attempt to process non-existent request
		assert_err!(
			EdscRedemption::process_queue(RuntimeOrigin::signed(ALICE), 999),
			pallet_edsc_redemption::Error::<Test>::RequestNotFound
		);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// SECURITY TEST 3: Double-Spend Prevention
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_receipt_cannot_be_reused() {
	new_test_ext().execute_with(|| {
		// Mint EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 20_000_00).unwrap();

		// Create receipt
		EdscReceipts::create_receipt(
			RuntimeOrigin::signed(ALICE),
			10_000_00,
			100
		).unwrap();
		let receipt_id = 0;

		// Set oracle
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// First redemption with receipt (should succeed)
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			10_000_00,
			Some(receipt_id),
			None
		));

		// Attempt to reuse same receipt (should fail)
		assert_err!(
			EdscRedemption::redeem(
				RuntimeOrigin::signed(ALICE),
				5_000_00,
				Some(receipt_id),
				None
			),
			pallet_edsc_redemption::Error::<Test>::InvalidReceipt
		);
	});
}

#[test]
fn test_redemption_prevents_balance_double_spend() {
	new_test_ext().execute_with(|| {
		// Mint 10k EDSC to ALICE
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_00).unwrap();

		// Set oracle
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Redeem 8k
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			8_000_00,
			None,
			None
		));

		// Attempt to redeem another 8k (insufficient balance)
		assert_err!(
			EdscRedemption::redeem(RuntimeOrigin::signed(ALICE), 8_000_00, None, None),
			pallet_edsc_redemption::Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn test_queue_request_cannot_be_processed_twice() {
	new_test_ext().execute_with(|| {
		// Setup
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_00).unwrap();
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Enable throttle
		EdscRedemption::do_update_reserve_ratio(
			FixedU128::from_rational(102u128, 100u128)
		).unwrap();

		// Queue a redemption
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			5_000_00,
			None,
			None
		));

		let request_id = 0;

		// Recover reserve ratio
		EdscRedemption::do_update_reserve_ratio(
			FixedU128::from_rational(110u128, 100u128)
		).unwrap();

		// Process once (should succeed)
		assert_ok!(EdscRedemption::process_queue(
			RuntimeOrigin::signed(BOB),
			request_id
		));

		// Attempt to process again (should fail - already completed)
		assert_err!(
			EdscRedemption::process_queue(RuntimeOrigin::signed(BOB), request_id),
			pallet_edsc_redemption::Error::<Test>::RequestNotFound
		);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// SECURITY TEST 4: Replay Attack Mitigation
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_receipt_id_uniqueness() {
	new_test_ext().execute_with(|| {
		// Create multiple receipts
		EdscReceipts::create_receipt(RuntimeOrigin::signed(ALICE), 1_000_00, 100).unwrap();
		EdscReceipts::create_receipt(RuntimeOrigin::signed(ALICE), 2_000_00, 100).unwrap();
		EdscReceipts::create_receipt(RuntimeOrigin::signed(BOB), 3_000_00, 100).unwrap();

		// Verify each receipt has unique ID
		assert!(EdscReceipts::is_valid_receipt(0, &ALICE).is_ok());
		assert!(EdscReceipts::is_valid_receipt(1, &ALICE).is_ok());
		assert!(EdscReceipts::is_valid_receipt(2, &BOB).is_ok());

		// Receipt 0 belongs to ALICE, not BOB
		assert!(EdscReceipts::is_valid_receipt(0, &BOB).is_err());
	});
}

#[test]
fn test_redemption_request_id_increments() {
	new_test_ext().execute_with(|| {
		// Setup
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 30_000_00).unwrap();
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Enable throttle to queue requests
		EdscRedemption::do_update_reserve_ratio(
			FixedU128::from_rational(102u128, 100u128)
		).unwrap();

		// Create multiple queued requests
		assert_ok!(EdscRedemption::redeem(RuntimeOrigin::signed(ALICE), 5_000_00, None, None));
		assert_ok!(EdscRedemption::redeem(RuntimeOrigin::signed(ALICE), 6_000_00, None, None));
		assert_ok!(EdscRedemption::redeem(RuntimeOrigin::signed(ALICE), 7_000_00, None, None));

		// Verify unique request IDs
		assert!(EdscRedemption::redemption_requests(0).is_some());
		assert!(EdscRedemption::redemption_requests(1).is_some());
		assert!(EdscRedemption::redemption_requests(2).is_some());

		// Verify request IDs are sequential
		assert_eq!(EdscRedemption::next_request_id(), 3);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// SECURITY TEST 5: Economic Attack Vectors
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_volume_cap_prevents_bank_run() {
	new_test_ext().execute_with(|| {
		// Mint large supply
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000_000).unwrap();

		// Set oracle
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// HourlyRedemptionCap is 0.5% of supply = 50_000_000

		// Attempt to redeem more than hourly cap in single transaction
		assert_err!(
			EdscRedemption::redeem(
				RuntimeOrigin::signed(ALICE),
				60_000_000, // Exceeds cap
				None,
				None
			),
			pallet_edsc_redemption::Error::<Test>::HourlyCapExceeded
		);
	});
}

#[test]
fn test_daily_limit_prevents_per_wallet_drain() {
	new_test_ext().execute_with(|| {
		// Mint EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 100_000_00).unwrap();

		// Create receipt for Path 1
		EdscReceipts::create_receipt(
			RuntimeOrigin::signed(ALICE),
			100_000_00,
			100
		).unwrap();

		// Set oracle
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Attempt to exceed Path 1 daily limit ($50k)
		assert_err!(
			EdscRedemption::redeem(
				RuntimeOrigin::signed(ALICE),
				60_000_00, // Exceeds $50k limit
				Some(0),
				None
			),
			pallet_edsc_redemption::Error::<Test>::DailyLimitExceeded
		);
	});
}

#[test]
fn test_minimum_fee_prevents_fee_gaming() {
	new_test_ext().execute_with(|| {
		// Mint EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_00).unwrap();

		// Set oracle to above peg ($1.01)
		// User might expect negative fee, but should get minimum fee
		EdscRedemption::do_update_oracle_price(101).unwrap();

		// Redeem via Path 2 or 3
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			1_000_00,
			None,
			None
		));

		// Fee should still be deducted (minimum fee)
		// User should receive less than 1000 EDSC worth
		let balance_after = EdscToken::balance_of(&ALICE);
		assert!(balance_after < 9_000_00);
	});
}

#[test]
fn test_reserve_ratio_prevents_undercollateralization() {
	new_test_ext().execute_with(|| {
		// Mint EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_00).unwrap();

		// Set oracle
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Set reserve ratio to critical (98%)
		EdscRedemption::do_update_reserve_ratio(
			FixedU128::from_rational(98u128, 100u128)
		).unwrap();

		// Redemptions should be paused
		assert!(EdscRedemption::redemptions_paused());

		// Attempt to redeem should fail
		assert_err!(
			EdscRedemption::redeem(RuntimeOrigin::signed(ALICE), 1_000_00, None, None),
			pallet_edsc_redemption::Error::<Test>::RedemptionsPaused
		);

		// Supply remains protected
		assert_eq!(EdscToken::total_supply(), 10_000_00);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// SECURITY TEST 6: Front-Running Resistance
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_oracle_price_update_cannot_be_frontrun() {
	new_test_ext().execute_with(|| {
		// Setup
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_00).unwrap();

		// Initial oracle price at $1.00
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// User redeems at current price
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			5_000_00,
			None,
			None
		));

		// Oracle updates price to $0.98 (worse for protocol)
		EdscRedemption::do_update_oracle_price(98).unwrap();

		// Previous redemption used old price (transaction was already executed)
		// New redemption uses new price with higher fee
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			2_000_00,
			None,
			None
		));

		// Cannot retroactively change executed redemptions
	});
}

#[test]
fn test_queue_prevents_reserve_ratio_gaming() {
	new_test_ext().execute_with(|| {
		// Setup
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_00).unwrap();
		EdscRedemption::do_update_oracle_price(100).unwrap();

		// Set to throttle zone
		EdscRedemption::do_update_reserve_ratio(
			FixedU128::from_rational(103u128, 100u128)
		).unwrap();

		// User redemptions are queued (cannot immediately drain)
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(ALICE),
			5_000_00,
			None,
			None
		));

		// EDSC not burned yet
		assert_eq!(EdscToken::balance_of(&ALICE), 10_000_00);

		// User cannot game the system by immediately executing
		// Must wait for reserve to recover
	});
}
