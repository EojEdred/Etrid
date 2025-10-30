//! Unit tests for pallet-multiasset-reserve

use crate::{mock::*, Error, Event, AllocationStrategy};
use frame_support::{assert_noop, assert_ok};
use sp_arithmetic::Permill;

// ===================== ASSET MANAGEMENT TESTS =====================

#[test]
fn add_asset_works() {
	new_test_ext().execute_with(|| {
		// Add BTC asset
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1, // asset_id
			b"BTC".to_vec(),
			8, // decimals
			1_000_000, // min 0.01 BTC
			100_000_000_000, // max 1000 BTC
			Permill::from_percent(40), // 40% allocation
		));

		// Verify asset was added
		let asset = MultiassetReserve::asset_config(1).unwrap();
		assert_eq!(asset.symbol.to_vec(), b"BTC".to_vec());
		assert_eq!(asset.decimals, 8);
		assert_eq!(asset.target_allocation, Permill::from_percent(40));
		assert_eq!(asset.is_active, true);

		// Verify whitelisted
		assert_eq!(MultiassetReserve::is_whitelisted(1), true);

		// Verify asset count
		assert_eq!(MultiassetReserve::asset_count(), 1);

		// Check event
		System::assert_has_event(Event::AssetAdded {
			asset_id: 1,
			symbol: b"BTC".to_vec(),
		}.into());
	});
}

#[test]
fn add_duplicate_asset_fails() {
	new_test_ext().execute_with(|| {
		// Add BTC
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(40),
		));

		// Try to add again - should fail
		assert_noop!(
			MultiassetReserve::add_asset(
				RuntimeOrigin::root(),
				1, // Same ID
				b"ETH".to_vec(),
				18,
				1_000_000,
				100_000_000_000,
				Permill::from_percent(30),
			),
			Error::<Test>::AssetAlreadyExists
		);
	});
}

#[test]
fn add_asset_requires_root() {
	new_test_ext().execute_with(|| {
		// Try to add asset as non-root user
		assert_noop!(
			MultiassetReserve::add_asset(
				RuntimeOrigin::signed(1), // Not root
				1,
				b"BTC".to_vec(),
				8,
				1_000_000,
				100_000_000_000,
				Permill::from_percent(40),
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn remove_asset_works() {
	new_test_ext().execute_with(|| {
		// Add asset
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(40),
		));

		assert_eq!(MultiassetReserve::asset_count(), 1);

		// Remove asset (no holdings, so should work)
		assert_ok!(MultiassetReserve::remove_asset(RuntimeOrigin::root(), 1));

		// Verify removed
		assert_eq!(MultiassetReserve::asset_config(1), None);
		assert_eq!(MultiassetReserve::is_whitelisted(1), false);
		assert_eq!(MultiassetReserve::asset_count(), 0);

		// Check event
		System::assert_has_event(Event::AssetRemoved {
			asset_id: 1,
		}.into());
	});
}

#[test]
fn remove_nonexistent_asset_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MultiassetReserve::remove_asset(RuntimeOrigin::root(), 99),
			Error::<Test>::AssetNotFound
		);
	});
}

// ===================== ALLOCATION TESTS =====================

#[test]
fn set_target_allocation_works() {
	new_test_ext().execute_with(|| {
		// Add asset
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(40),
		));

		// Change allocation
		assert_ok!(MultiassetReserve::set_target_allocation(
			RuntimeOrigin::root(),
			1,
			Permill::from_percent(50),
		));

		// Verify updated
		let asset = MultiassetReserve::asset_config(1).unwrap();
		assert_eq!(asset.target_allocation, Permill::from_percent(50));

		// Check event
		System::assert_has_event(Event::AllocationUpdated {
			asset_id: 1,
			target: Permill::from_percent(50),
		}.into());
	});
}

#[test]
fn set_allocation_for_nonexistent_asset_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MultiassetReserve::set_target_allocation(
				RuntimeOrigin::root(),
				99,
				Permill::from_percent(50),
			),
			Error::<Test>::AssetNotFound
		);
	});
}

// ===================== DEPOSIT/WITHDRAWAL TESTS =====================

#[test]
fn deposit_to_reserve_works() {
	new_test_ext().execute_with(|| {
		// Add BTC asset
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(40),
		));

		// Deposit 10 BTC
		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			10_00000000, // 10 BTC with 8 decimals
		));

		// Verify holding
		let holding = MultiassetReserve::asset_holding(1).unwrap();
		assert_eq!(holding.amount, 10_00000000);
		assert_eq!(holding.asset_id, 1);

		// Check event
		System::assert_has_event(Event::DepositedToReserve {
			asset_id: 1,
			amount: 10_00000000,
			depositor: 1,
		}.into());
	});
}

#[test]
fn deposit_to_inactive_asset_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MultiassetReserve::deposit_to_reserve(
				RuntimeOrigin::signed(1),
				99, // Nonexistent asset
				1000000,
			),
			Error::<Test>::AssetNotFound
		);
	});
}

#[test]
fn withdraw_from_reserve_works() {
	new_test_ext().execute_with(|| {
		// Setup: Add asset and deposit
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(40),
		));

		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			10_00000000,
		));

		// Withdraw 5 BTC
		assert_ok!(MultiassetReserve::withdraw_from_reserve(
			RuntimeOrigin::root(),
			1,
			5_00000000,
		));

		// Verify remaining balance
		let holding = MultiassetReserve::asset_holding(1).unwrap();
		assert_eq!(holding.amount, 5_00000000);
	});
}

#[test]
fn withdraw_more_than_balance_fails() {
	new_test_ext().execute_with(|| {
		// Setup: Add asset and deposit
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(40),
		));

		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			10_00000000,
		));

		// Try to withdraw more than deposited
		assert_noop!(
			MultiassetReserve::withdraw_from_reserve(
				RuntimeOrigin::root(),
				1,
				20_00000000, // More than 10 BTC
			),
			Error::<Test>::InsufficientBalance
		);
	});
}

// ===================== ALLOCATION STRATEGY TESTS =====================

#[test]
fn set_allocation_strategy_works() {
	new_test_ext().execute_with(|| {
		// Default should be Custom (from mock or default)
		// Set to EqualWeight (code 0)
		assert_ok!(MultiassetReserve::set_allocation_strategy(
			RuntimeOrigin::root(),
			0, // EqualWeight
		));

		assert_eq!(
			MultiassetReserve::allocation_strategy(),
			AllocationStrategy::EqualWeight
		);

		// Check event
		System::assert_has_event(Event::StrategyChanged {
			strategy_code: 0,
		}.into());
	});
}

#[test]
fn set_allocation_strategy_requires_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MultiassetReserve::set_allocation_strategy(
				RuntimeOrigin::signed(1),
				1, // MarketCapWeighted
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

// ===================== REBALANCING TESTS =====================

#[test]
fn trigger_rebalance_works() {
	new_test_ext().execute_with(|| {
		// Setup: Add multiple assets
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(50),
		));

		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			2,
			b"ETH".to_vec(),
			18,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(50),
		));

		// Enable rebalancing
		assert_ok!(MultiassetReserve::set_rebalancing_enabled(
			RuntimeOrigin::root(),
			true,
		));

		// Trigger rebalance
		assert_ok!(MultiassetReserve::trigger_rebalance(RuntimeOrigin::root()));

		// Check event
		System::assert_has_event(Event::RebalanceTriggered {
			total_value: 0, // No deposits yet
			assets_count: 2,
		}.into());

		System::assert_has_event(Event::RebalanceCompleted {
			success: true,
			assets_rebalanced: 2,
		}.into());
	});
}

#[test]
fn trigger_rebalance_when_disabled_fails() {
	new_test_ext().execute_with(|| {
		// Rebalancing disabled by default or set to false
		assert_ok!(MultiassetReserve::set_rebalancing_enabled(
			RuntimeOrigin::root(),
			false,
		));

		assert_noop!(
			MultiassetReserve::trigger_rebalance(RuntimeOrigin::root()),
			Error::<Test>::RebalancingDisabled
		);
	});
}

#[test]
fn enable_disable_rebalancing_works() {
	new_test_ext().execute_with(|| {
		// Enable
		assert_ok!(MultiassetReserve::set_rebalancing_enabled(
			RuntimeOrigin::root(),
			true,
		));
		assert_eq!(MultiassetReserve::rebalancing_enabled(), true);

		// Disable
		assert_ok!(MultiassetReserve::set_rebalancing_enabled(
			RuntimeOrigin::root(),
			false,
		));
		assert_eq!(MultiassetReserve::rebalancing_enabled(), false);
	});
}

// ===================== HELPER FUNCTION TESTS =====================

#[test]
fn calculate_total_value_works() {
	new_test_ext().execute_with(|| {
		// Setup assets
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(50),
		));

		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			2,
			b"ETH".to_vec(),
			18,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(50),
		));

		// Deposit to both
		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			10_00000000, // 10 BTC
		));

		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			2,
			100_000000000000000000, // 100 ETH
		));

		// Total value should be sum of both (using placeholder value = amount)
		let total = MultiassetReserve::total_reserve_value();
		assert!(total > 0);
	});
}

#[test]
fn get_asset_allocation_works() {
	new_test_ext().execute_with(|| {
		// Add two assets with equal target allocations
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(50),
		));

		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			2,
			b"ETH".to_vec(),
			18,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(50),
		));

		// Deposit equal amounts
		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			1000000,
		));

		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			2,
			1000000,
		));

		// Each should have ~50% allocation
		let btc_alloc = MultiassetReserve::get_asset_allocation(1);
		assert!(btc_alloc.is_some());

		let eth_alloc = MultiassetReserve::get_asset_allocation(2);
		assert!(eth_alloc.is_some());

		// Both should be approximately equal
		assert_eq!(btc_alloc.unwrap(), eth_alloc.unwrap());
	});
}

#[test]
fn max_assets_limit_enforced() {
	new_test_ext().execute_with(|| {
		// Add assets up to MaxAssets limit (50 in mock)
		for i in 0..50 {
			assert_ok!(MultiassetReserve::add_asset(
				RuntimeOrigin::root(),
				i,
				b"ASSET".to_vec(),
				8,
				1_000_000,
				100_000_000_000,
				Permill::from_percent(1),
			));
		}

		// Try to add one more - should fail
		assert_noop!(
			MultiassetReserve::add_asset(
				RuntimeOrigin::root(),
				50,
				b"ASSET51".to_vec(),
				8,
				1_000_000,
				100_000_000_000,
				Permill::from_percent(1),
			),
			Error::<Test>::TooManyAssets
		);
	});
}

// ===================== EDGE CASE TESTS =====================

#[test]
fn zero_deposit_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			0, // Allow zero min
			100_000_000_000,
			Permill::from_percent(50),
		));

		// Deposit zero amount
		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			0,
		));

		let holding = MultiassetReserve::asset_holding(1).unwrap();
		assert_eq!(holding.amount, 0);
	});
}

#[test]
fn multiple_deposits_accumulate() {
	new_test_ext().execute_with(|| {
		assert_ok!(MultiassetReserve::add_asset(
			RuntimeOrigin::root(),
			1,
			b"BTC".to_vec(),
			8,
			1_000_000,
			100_000_000_000,
			Permill::from_percent(50),
		));

		// Deposit multiple times
		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			1_00000000,
		));

		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			2_00000000,
		));

		assert_ok!(MultiassetReserve::deposit_to_reserve(
			RuntimeOrigin::signed(1),
			1,
			3_00000000,
		));

		// Total should be 6 BTC
		let holding = MultiassetReserve::asset_holding(1).unwrap();
		assert_eq!(holding.amount, 6_00000000);
	});
}
