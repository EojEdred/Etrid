//! # EDSC Full Workflow Integration Tests
//!
//! End-to-end integration tests covering complete EDSC workflows:
//! 1. Collateral Deposit → EDSC Minting → Reserve Ratio Update
//! 2. Receipt Creation → Path 1 Redemption (SBT) → Fee Verification
//! 3. Multi-step redemption with throttling and queue processing
//! 4. Circuit breaker activation and recovery workflows
//! 5. Cross-pallet state consistency verification

use codec::{Decode, Encode};
use frame_support::{
	assert_ok, assert_err,
	traits::{OnFinalize, OnInitialize},
};
use sp_runtime::{FixedU128, Permill};

mod common;
use common::*;

type EdscToken = pallet_edsc_token::Pallet<Runtime>;
type EdscReceipts = pallet_edsc_receipts::Pallet<Runtime>;
type EdscRedemption = pallet_edsc_redemption::Pallet<Runtime>;
type ReserveVault = pallet_reserve_vault::Pallet<Runtime>;
type System = frame_system::Pallet<Runtime>;

// Test accounts
const ALICE: AccountId = 1;
const BOB: AccountId = 2;
const CHARLIE: AccountId = 3;

/// Helper function to advance to block number
fn run_to_block(n: u64) {
	while System::block_number() < n {
		let current = System::block_number();

		// Finalize current block
		EdscToken::on_finalize(current);
		EdscRedemption::on_finalize(current);
		ReserveVault::on_finalize(current);
		System::on_finalize(current);

		// Initialize next block
		System::set_block_number(current + 1);
		System::on_initialize(current + 1);
		ReserveVault::on_initialize(current + 1);
		EdscRedemption::on_initialize(current + 1);
		EdscToken::on_initialize(current + 1);
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 1: Complete EDSC Lifecycle
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_complete_edsc_lifecycle() {
	new_test_ext().execute_with(|| {
		// Step 1: Deposit collateral into reserve vault
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			200_000_000 // 2 BTC at $60k = $120k
		));

		// Step 2: Update oracle prices
		assert_ok!(ReserveVault::update_asset_price(
			RuntimeOrigin::root(),
			1, // BTC
			6_000_000 // $60,000
		));

		// Step 3: Recalculate vault values
		assert_ok!(ReserveVault::recalculate_vault_values(
			RuntimeOrigin::signed(ALICE)
		));

		// Step 4: Calculate reserve ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(
			RuntimeOrigin::signed(ALICE)
		));

		// Step 5: Authorize minter and mint EDSC
		assert_ok!(EdscToken::authorize_minter(
			RuntimeOrigin::root(),
			ALICE
		));

		// Mint $90,000 worth of EDSC (maintains >100% reserve ratio)
		// Reserve: $120k * 0.9 (after 10% haircut) = $108k
		// EDSC: $90k
		// Ratio: 108/90 = 120% (optimal)
		assert_ok!(EdscToken::mint(
			RuntimeOrigin::signed(ALICE),
			BOB,
			9_000_000 // $90,000 in cents
		));

		// Verify BOB has EDSC
		assert_eq!(EdscToken::balance_of(&BOB), 9_000_000);

		// Step 6: Recalculate reserve ratio after minting
		assert_ok!(ReserveVault::calculate_reserve_ratio(
			RuntimeOrigin::signed(ALICE)
		));

		let reserve_ratio = ReserveVault::reserve_ratio();

		// Should be in optimal range (110-130%)
		assert!(reserve_ratio >= FixedU128::from_rational(110u128, 100u128));
		assert!(reserve_ratio <= FixedU128::from_rational(130u128, 100u128));

		// Step 7: Update redemption pallet with new reserve ratio
		assert_ok!(EdscRedemption::update_reserve_ratio(
			RuntimeOrigin::root(),
			reserve_ratio
		));

		// Verify redemptions are NOT paused (healthy reserve)
		assert!(!EdscRedemption::redemptions_paused());
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 2: Path 1 Redemption with SBT Receipt
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_path1_sbt_redemption_workflow() {
	new_test_ext().execute_with(|| {
		// Setup: Mint EDSC to BOB
		assert_ok!(EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE));
		assert_ok!(EdscToken::mint(
			RuntimeOrigin::signed(ALICE),
			BOB,
			10_000_00 // $10,000
		));

		// Setup: Set oracle price to $1.00
		assert_ok!(EdscRedemption::update_oracle_price(
			RuntimeOrigin::root(),
			100 // $1.00
		));

		// Step 1: BOB creates a receipt for their EDSC purchase
		assert_ok!(EdscReceipts::create_receipt(
			RuntimeOrigin::signed(BOB),
			10_000_00, // $10,000 EDSC
			100 // Purchased at $1.00
		));

		let receipt_id = 0;

		// Verify receipt created
		assert!(EdscReceipts::is_valid_receipt(receipt_id, &BOB).is_ok());

		// Step 2: BOB redeems via Path 1 (SBT - NO FEE)
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			10_000_00,
			Some(receipt_id),
			None
		));

		// Verify EDSC was burned
		assert_eq!(EdscToken::balance_of(&BOB), 0);

		// Verify receipt was consumed
		assert!(EdscReceipts::is_valid_receipt(receipt_id, &BOB).is_err());

		// Verify total supply decreased
		assert_eq!(EdscToken::total_supply(), 0);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 3: Throttled Redemption Queue Workflow
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_throttled_redemption_queue_workflow() {
	new_test_ext().execute_with(|| {
		// Setup: Mint EDSC
		assert_ok!(EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE));
		assert_ok!(EdscToken::mint(RuntimeOrigin::signed(ALICE), BOB, 5_000_00));
		assert_ok!(EdscToken::mint(RuntimeOrigin::signed(ALICE), CHARLIE, 3_000_00));

		// Setup: Set oracle price
		assert_ok!(EdscRedemption::update_oracle_price(RuntimeOrigin::root(), 100));

		// Step 1: Set reserve ratio to throttle zone (102%)
		let throttle_ratio = FixedU128::from_rational(102u128, 100u128);
		assert_ok!(EdscRedemption::update_reserve_ratio(
			RuntimeOrigin::root(),
			throttle_ratio
		));

		// Verify throttling is enabled
		assert!(EdscRedemption::redemptions_throttled());
		assert!(!EdscRedemption::redemptions_paused());

		// Step 2: BOB attempts redemption (should be queued)
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			1_000_00,
			None,
			None
		));

		// Verify EDSC NOT burned yet (queued)
		assert_eq!(EdscToken::balance_of(&BOB), 5_000_00);

		// Verify request is in queue
		let request = EdscRedemption::redemption_requests(0).unwrap();
		assert_eq!(request.requester, BOB);
		assert_eq!(request.amount, 1_000_00);
		assert_eq!(request.status, pallet_edsc_redemption::RequestStatus::Pending);

		// Step 3: Reserve ratio recovers to 110%
		let healthy_ratio = FixedU128::from_rational(110u128, 100u128);
		assert_ok!(EdscRedemption::update_reserve_ratio(
			RuntimeOrigin::root(),
			healthy_ratio
		));

		// Verify throttling is disabled
		assert!(!EdscRedemption::redemptions_throttled());

		// Step 4: Process queued redemption
		assert_ok!(EdscRedemption::process_queue(
			RuntimeOrigin::signed(ALICE),
			0 // request_id
		));

		// Verify EDSC was burned
		assert!(EdscToken::balance_of(&BOB) < 5_000_00);

		// Verify request marked as completed
		let processed = EdscRedemption::redemption_requests(0).unwrap();
		assert_eq!(processed.status, pallet_edsc_redemption::RequestStatus::Completed);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 4: Circuit Breaker Cascade
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_circuit_breaker_cascade() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE));
		assert_ok!(EdscToken::mint(RuntimeOrigin::signed(ALICE), BOB, 10_000_00));
		assert_ok!(EdscRedemption::update_oracle_price(RuntimeOrigin::root(), 100));

		// Step 1: Healthy state (120% reserve ratio)
		let healthy_ratio = FixedU128::from_rational(120u128, 100u128);
		assert_ok!(EdscRedemption::update_reserve_ratio(
			RuntimeOrigin::root(),
			healthy_ratio
		));

		assert!(!EdscRedemption::redemptions_throttled());
		assert!(!EdscRedemption::redemptions_paused());

		// Redemption should succeed
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			1_000_00,
			None,
			None
		));

		// Step 2: Reserve drops to throttle zone (103%)
		let throttle_ratio = FixedU128::from_rational(103u128, 100u128);
		assert_ok!(EdscRedemption::update_reserve_ratio(
			RuntimeOrigin::root(),
			throttle_ratio
		));

		assert!(EdscRedemption::redemptions_throttled());
		assert!(!EdscRedemption::redemptions_paused());

		// Redemption should be queued
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			1_000_00,
			None,
			None
		));

		// Step 3: Reserve drops to critical (98%)
		let critical_ratio = FixedU128::from_rational(98u128, 100u128);
		assert_ok!(EdscRedemption::update_reserve_ratio(
			RuntimeOrigin::root(),
			critical_ratio
		));

		assert!(!EdscRedemption::redemptions_throttled()); // Overridden by pause
		assert!(EdscRedemption::redemptions_paused());

		// Redemption should fail
		assert_err!(
			EdscRedemption::redeem(RuntimeOrigin::signed(BOB), 1_000_00, None, None),
			pallet_edsc_redemption::Error::<Runtime>::RedemptionsPaused
		);

		// Step 4: Reserve recovers to 120%
		assert_ok!(EdscRedemption::update_reserve_ratio(
			RuntimeOrigin::root(),
			healthy_ratio
		));

		assert!(!EdscRedemption::redemptions_throttled());
		assert!(!EdscRedemption::redemptions_paused());

		// Redemptions should work normally again
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			1_000_00,
			None,
			None
		));
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 5: Multi-Asset Reserve Diversification
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_multi_asset_reserve_diversification() {
	new_test_ext().execute_with(|| {
		// Step 1: Deposit multiple assets into vault

		// BTC: 1 BTC at $60k = $60k raw, $54k adjusted (10% haircut)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		// ETH: 20 ETH at $3k = $60k raw, $51k adjusted (15% haircut)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(BOB),
			2, // ETH
			20_000_000_000_000_000_000 // 20 ETH in wei
		));

		// USDC: $30k = $30k raw, $28.5k adjusted (5% haircut)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(CHARLIE),
			3, // USDC
			30_000_000_000 // 30k USDC
		));

		// Step 2: Update all asset prices
		assert_ok!(ReserveVault::update_asset_price(RuntimeOrigin::root(), 1, 6_000_000));
		assert_ok!(ReserveVault::update_asset_price(RuntimeOrigin::root(), 2, 300_000));
		assert_ok!(ReserveVault::update_asset_price(RuntimeOrigin::root(), 3, 100));

		// Step 3: Recalculate vault values
		assert_ok!(ReserveVault::recalculate_vault_values(RuntimeOrigin::signed(ALICE)));

		// Step 4: Verify total adjusted reserve
		// Total: $54k + $51k + $28.5k = $133.5k

		// Step 5: Mint EDSC up to safe ratio
		assert_ok!(EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE));

		// Mint $100k EDSC (ratio = 133.5/100 = 133.5%, optimal)
		assert_ok!(EdscToken::mint(RuntimeOrigin::signed(ALICE), BOB, 10_000_000));

		// Step 6: Calculate reserve ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		let ratio = ReserveVault::reserve_ratio();

		// Should be in optimal range
		assert!(ratio >= FixedU128::from_rational(110u128, 100u128));
		assert!(ratio <= FixedU128::from_rational(140u128, 100u128));
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 6: Dynamic Fee Adjustment Workflow
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_dynamic_fee_adjustment_workflow() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE));
		assert_ok!(EdscToken::mint(RuntimeOrigin::signed(ALICE), BOB, 10_000_00));

		// Scenario 1: EDSC at peg ($1.00) - minimum fee
		assert_ok!(EdscRedemption::update_oracle_price(RuntimeOrigin::root(), 100));

		// Path 2/3 redemption should have minimum fee (0.25%)
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			1_000_00,
			None,
			None
		));

		// Verify small fee deducted
		let balance_after_peg = EdscToken::balance_of(&BOB);
		assert!(balance_after_peg < 9_000_00); // Some burned
		assert!(balance_after_peg > 8_950_00); // But minimal fee

		// Scenario 2: EDSC depegs to $0.98 - dynamic fee kicks in
		assert_ok!(EdscRedemption::update_oracle_price(RuntimeOrigin::root(), 98));

		// Path 3 redemption should have higher fee
		// Dynamic fee = 1.2 × (1.00 - 0.98) = 1.2 × 0.02 = 2.4%
		// Path 3 penalty = 2x = 4.8%

		let balance_before = EdscToken::balance_of(&BOB);
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			1_000_00,
			None,
			None
		));

		// Verify higher fee deducted
		let balance_after_depeg = EdscToken::balance_of(&BOB);
		let fee_amount = balance_before - balance_after_depeg - 1_000_00;

		// Fee should be approximately 4.8% of 1000 = ~48
		assert!(fee_amount > 40); // More than minimum fee
		assert!(fee_amount < 60); // But not excessive
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 7: Cross-Pallet State Consistency
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_cross_pallet_state_consistency() {
	new_test_ext().execute_with(|| {
		// Step 1: Initial state
		let initial_edsc_supply = EdscToken::total_supply();
		let initial_vault_btc = ReserveVault::vault(pallet_reserve_vault::AssetType::BTC);

		// Step 2: Deposit collateral
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		// Verify vault updated
		let vault_after_deposit = ReserveVault::vault(pallet_reserve_vault::AssetType::BTC);
		assert!(vault_after_deposit.is_some());

		// Step 3: Mint EDSC
		assert_ok!(EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE));
		assert_ok!(EdscToken::mint(RuntimeOrigin::signed(ALICE), BOB, 5_000_00));

		// Verify EDSC supply increased
		assert_eq!(EdscToken::total_supply(), initial_edsc_supply + 5_000_00);

		// Step 4: Calculate reserve ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));
		let ratio = ReserveVault::reserve_ratio();

		// Step 5: Synchronize ratio to redemption pallet
		assert_ok!(EdscRedemption::update_reserve_ratio(RuntimeOrigin::root(), ratio));

		// Verify consistency
		assert_eq!(EdscRedemption::reserve_ratio(), ReserveVault::reserve_ratio());

		// Step 6: Redeem EDSC
		assert_ok!(EdscRedemption::update_oracle_price(RuntimeOrigin::root(), 100));
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			2_000_00,
			None,
			None
		));

		// Verify EDSC supply decreased
		assert!(EdscToken::total_supply() < initial_edsc_supply + 5_000_00);

		// Verify balances are consistent
		let bob_balance = EdscToken::balance_of(&BOB);
		assert!(bob_balance < 5_000_00); // Some redeemed
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TEST 8: Time-Based Daily Limits
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_daily_limits_across_blocks() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE));
		assert_ok!(EdscToken::mint(RuntimeOrigin::signed(ALICE), BOB, 100_000_00));
		assert_ok!(EdscRedemption::update_oracle_price(RuntimeOrigin::root(), 100));

		// Create receipt for Path 1
		assert_ok!(EdscReceipts::create_receipt(
			RuntimeOrigin::signed(BOB),
			100_000_00,
			100
		));

		// Day 1: Redeem up to Path 1 daily limit ($50k)
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			50_000_00,
			Some(0),
			None
		));

		// Attempt to exceed limit same day - should fail
		assert_ok!(EdscReceipts::create_receipt(
			RuntimeOrigin::signed(BOB),
			50_000_00,
			100
		));

		assert_err!(
			EdscRedemption::redeem(RuntimeOrigin::signed(BOB), 10_000_00, Some(1), None),
			pallet_edsc_redemption::Error::<Runtime>::DailyLimitExceeded
		);

		// Advance to next day (14400 blocks at 6 sec = 1 day)
		run_to_block(14401);

		// Day 2: Should be able to redeem again
		assert_ok!(EdscRedemption::redeem(
			RuntimeOrigin::signed(BOB),
			10_000_00,
			Some(1),
			None
		));
	});
}
