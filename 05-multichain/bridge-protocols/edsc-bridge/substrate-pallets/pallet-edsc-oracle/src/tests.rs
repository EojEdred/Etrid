//! # pallet-edsc-oracle Tests
//!
//! Comprehensive test suite for EDSC Oracle pallet covering:
//! - RBAC (Role-Based Access Control)
//! - Price feed submission
//! - TWAP calculation
//! - Outlier detection
//! - Staleness monitoring
//! - Circuit breakers
//! - Edge cases

use super::*;
use crate::mock::*;
use frame_support::{assert_err, assert_ok, traits::Hooks};

// Helper: Authorize a feeder
fn authorize_feeder(account: u64) {
	assert_ok!(EdscOracle::authorize_feeder(RuntimeOrigin::root(), account));
}

// Helper: Submit price from multiple sources
fn submit_test_prices(prices: Vec<(u64, u8, u128, u128)>) {
	for (account, source, price, volume) in prices {
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(account),
			source,
			price,
			volume
		));
	}
}

// Helper: Seed baseline prices to establish median for outlier detection
// This prevents InsufficientSources errors when testing TWAP calculations
fn seed_baseline_prices() {
	// Authorize feeder if not already authorized
	if !EdscOracle::is_price_feeder(1) {
		authorize_feeder(1);
	}

	// Submit 5 baseline prices at $1.00 to establish median
	for source in 0..5 {
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(1),
			source,
			100, // $1.00 baseline
			1_000_000 // 1M volume
		));
	}
}

// =============================================================================
// RBAC TESTS (4 tests)
// =============================================================================

#[test]
fn test_authorize_feeder_works() {
	new_test_ext().execute_with(|| {
		// Only root can authorize
		assert_ok!(EdscOracle::authorize_feeder(RuntimeOrigin::root(), 1));
		assert_eq!(EdscOracle::is_price_feeder(1), true);

		// Check event
		System::assert_last_event(Event::FeederAuthorized { feeder: 1 }.into());
	});
}

#[test]
fn test_authorize_feeder_requires_root() {
	new_test_ext().execute_with(|| {
		// Non-root cannot authorize
		assert_err!(
			EdscOracle::authorize_feeder(RuntimeOrigin::signed(1), 2),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn test_revoke_feeder_works() {
	new_test_ext().execute_with(|| {
		// Authorize then revoke
		authorize_feeder(1);
		assert_eq!(EdscOracle::is_price_feeder(1), true);

		assert_ok!(EdscOracle::revoke_feeder(RuntimeOrigin::root(), 1));
		assert_eq!(EdscOracle::is_price_feeder(1), false);

		// Check event
		System::assert_last_event(Event::FeederRevoked { feeder: 1 }.into());
	});
}

#[test]
fn test_unauthorized_cannot_submit_price() {
	new_test_ext().execute_with(|| {
		// Unauthorized account cannot submit price
		assert_err!(
			EdscOracle::submit_price(RuntimeOrigin::signed(1), 0, 100, 1_000_000),
			Error::<Test>::NotAuthorizedFeeder
		);
	});
}

// =============================================================================
// PRICE FEED SUBMISSION TESTS (5 tests)
// =============================================================================

#[test]
fn test_submit_price_works() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit price from Binance (source 0)
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(1),
			0, // Binance
			100, // $1.00
			1_000_000 // 1M volume
		));

		// Check event
		System::assert_has_event(Event::PriceSubmitted {
			source: 0,
			price: 100,
			volume: 1_000_000,
			feeder: 1,
		}.into());

		// Check price added to history
		let history = EdscOracle::price_history();
		assert_eq!(history.len(), 1);
		assert_eq!(history[0].price, 100);
		assert_eq!(history[0].volume, 1_000_000);
		assert_eq!(history[0].source, PriceSource::Binance);
	});
}

#[test]
fn test_submit_price_rejects_invalid_price() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Price too low ($0.49)
		assert_err!(
			EdscOracle::submit_price(RuntimeOrigin::signed(1), 0, 49, 1_000_000),
			Error::<Test>::InvalidPrice
		);

		// Price too high ($2.01)
		assert_err!(
			EdscOracle::submit_price(RuntimeOrigin::signed(1), 0, 201, 1_000_000),
			Error::<Test>::InvalidPrice
		);
	});
}

#[test]
fn test_submit_price_rejects_invalid_source() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Invalid source ID (only 0-7 are valid)
		assert_err!(
			EdscOracle::submit_price(RuntimeOrigin::signed(1), 99, 100, 1_000_000),
			Error::<Test>::InvalidPrice
		);
	});
}

#[test]
fn test_submit_price_fifo_behavior() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Fill history to max capacity (1000 entries)
		for i in 0..1000 {
			let source = (i % 8) as u8; // Cycle through all 8 sources
			assert_ok!(EdscOracle::submit_price(
				RuntimeOrigin::signed(1),
				source,
				100 + (i % 10) as u128, // Prices from 100-109
				1_000_000
			));
		}

		let history = EdscOracle::price_history();
		assert_eq!(history.len(), 1000);

		// Add one more - should remove oldest
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(1),
			0,
			105,
			1_000_000
		));

		let history = EdscOracle::price_history();
		assert_eq!(history.len(), 1000); // Still at max
		// Most recent should be 105
		assert_eq!(history[999].price, 105);
	});
}

#[test]
fn test_submit_price_blocked_when_paused() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Pause oracle
		assert_ok!(EdscOracle::pause_oracle(RuntimeOrigin::root()));

		// Submission should fail
		assert_err!(
			EdscOracle::submit_price(RuntimeOrigin::signed(1), 0, 100, 1_000_000),
			Error::<Test>::OraclePaused
		);
	});
}

// =============================================================================
// TWAP CALCULATION TESTS (6 tests)
// =============================================================================

#[test]
fn test_twap_calculation_simple_average() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);
		authorize_feeder(3);
		authorize_feeder(4);
		authorize_feeder(5);

		// Submit 5 prices from different sources (equal volumes)
		let prices = vec![
			(1, 0, 98, 1_000_000),  // Binance: $0.98
			(2, 1, 100, 1_000_000), // Coinbase: $1.00
			(3, 2, 102, 1_000_000), // Kraken: $1.02
			(4, 3, 99, 1_000_000),  // UniswapV3: $0.99
			(5, 4, 101, 1_000_000), // Curve: $1.01
		];
		submit_test_prices(prices);

		// Trigger TWAP calculation
		assert_ok!(EdscOracle::calculate_twap(RuntimeOrigin::signed(1)));

		// TWAP should be volume-weighted average: (98+100+102+99+101) / 5 = 100
		let twap = EdscOracle::current_twap().unwrap();
		assert_eq!(twap.price, 100);
		assert_eq!(twap.data_points, 5);
		assert_eq!(twap.sources_used, 5);
		assert_eq!(twap.using_fallback, false);
	});
}

#[test]
fn test_twap_volume_weighting() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);
		authorize_feeder(3);
		authorize_feeder(4);
		authorize_feeder(5);

		// Submit prices with varying volumes
		let prices = vec![
			(1, 0, 90, 10_000_000),   // Binance: $0.90, 10M vol (high volume, low price)
			(2, 1, 100, 1_000_000),   // Coinbase: $1.00, 1M vol
			(3, 2, 100, 1_000_000),   // Kraken: $1.00, 1M vol
			(4, 3, 100, 1_000_000),   // UniswapV3: $1.00, 1M vol
			(5, 4, 100, 1_000_000),   // Curve: $1.00, 1M vol
		];
		submit_test_prices(prices);

		// Trigger TWAP calculation
		assert_ok!(EdscOracle::calculate_twap(RuntimeOrigin::signed(1)));

		// Volume-weighted: (90*10M + 100*1M + 100*1M + 100*1M + 100*1M) / 14M
		// = (900M + 400M) / 14M = 1300M / 14M = 92.857... ≈ 92
		let twap = EdscOracle::current_twap().unwrap();
		// Should be closer to 90 due to high volume weighting
		assert!(twap.price >= 92 && twap.price <= 93);
	});
}

#[test]
fn test_twap_insufficient_sources() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit only 3 prices (minimum is 5)
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Manual TWAP calculation should fail
		assert_err!(
			EdscOracle::calculate_twap(RuntimeOrigin::signed(1)),
			Error::<Test>::InsufficientSources
		);
	});
}

#[test]
fn test_twap_fallback_window() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);
		authorize_feeder(3);
		authorize_feeder(4);
		authorize_feeder(5);

		// Submit prices at block 1
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(2, 1, 100, 1_000_000),
			(3, 2, 100, 1_000_000),
			(4, 3, 100, 1_000_000),
			(5, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Advance block past primary window (14400 blocks) but within fallback (100800)
		System::set_block_number(15_000);

		// Manually trigger TWAP
		assert_ok!(EdscOracle::calculate_twap(RuntimeOrigin::signed(1)));

		// Should use fallback window
		let twap = EdscOracle::current_twap().unwrap();
		assert_eq!(twap.using_fallback, true);

		// Check fallback event was emitted
		System::assert_has_event(Event::FallbackWindowActivated.into());
	});
}

#[test]
fn test_twap_variance_calculation() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);
		authorize_feeder(3);
		authorize_feeder(4);
		authorize_feeder(5);

		// Submit prices with high variance
		let prices = vec![
			(1, 0, 90, 1_000_000),  // $0.90
			(2, 1, 95, 1_000_000),  // $0.95
			(3, 2, 100, 1_000_000), // $1.00
			(4, 3, 105, 1_000_000), // $1.05
			(5, 4, 110, 1_000_000), // $1.10
		];
		submit_test_prices(prices);

		// Trigger TWAP calculation
		assert_ok!(EdscOracle::calculate_twap(RuntimeOrigin::signed(1)));

		let twap = EdscOracle::current_twap().unwrap();
		// Mean = 100, variance = [(10)^2 + (5)^2 + 0 + (5)^2 + (10)^2] / 5 = 250 / 5 = 50
		assert_eq!(twap.variance, 50);
	});
}

#[test]
fn test_twap_auto_recalculation() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);
		authorize_feeder(3);
		authorize_feeder(4);
		authorize_feeder(5);

		// Submit initial prices
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(2, 1, 100, 1_000_000),
			(3, 2, 100, 1_000_000),
			(4, 3, 100, 1_000_000),
			(5, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		let initial_block = EdscOracle::last_twap_block();

		// Advance 100 blocks (auto-recalc threshold)
		System::set_block_number(initial_block + 100);

		// Trigger on_finalize hook
		EdscOracle::on_finalize(System::block_number());

		// TWAP should have been recalculated
		let new_block = EdscOracle::last_twap_block();
		assert!(new_block > initial_block);
	});
}

// =============================================================================
// OUTLIER DETECTION TESTS (4 tests)
// =============================================================================

#[test]
fn test_outlier_rejection() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);

		// Submit normal prices to establish median
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 100, 1_000_000),
			(1, 3, 100, 1_000_000),
			(1, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Try to submit outlier (>2% deviation from median of 100)
		// 100 * 1.02 = 102, so 103 should be rejected
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(2),
			5, // PancakeSwap
			103,
			1_000_000
		));

		// Check outlier rejection event
		System::assert_has_event(Event::OutlierRejected {
			source: 5,
			price: 103,
			median: 100,
		}.into());

		// Price should NOT be in history
		let history = EdscOracle::price_history();
		assert_eq!(history.len(), 5); // Still 5 prices (outlier not added)
	});
}

#[test]
fn test_outlier_acceptance_within_threshold() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit normal prices
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 100, 1_000_000),
			(1, 3, 100, 1_000_000),
			(1, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Submit price at edge of threshold (2% = 102)
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(1),
			5,
			102,
			1_000_000
		));

		// Should be accepted
		let history = EdscOracle::price_history();
		assert_eq!(history.len(), 6);
		assert_eq!(history[5].price, 102);
	});
}

#[test]
fn test_outlier_negative_deviation() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);

		// Submit normal prices
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 100, 1_000_000),
			(1, 3, 100, 1_000_000),
			(1, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Try outlier on low side (100 * 0.98 = 98, so 97 should be rejected)
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(2),
			5,
			97,
			1_000_000
		));

		// Check rejection event
		System::assert_has_event(Event::OutlierRejected {
			source: 5,
			price: 97,
			median: 100,
		}.into());
	});
}

#[test]
fn test_median_calculation_even_count() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit 4 prices (even count)
		let prices = vec![
			(1, 0, 95, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 105, 1_000_000),
			(1, 3, 110, 1_000_000),
		];
		submit_test_prices(prices);

		// Median of [95, 100, 105, 110] = (100 + 105) / 2 = 102.5 ≈ 102
		// Submit price within 2% of 102 should work
		assert_ok!(EdscOracle::submit_price(
			RuntimeOrigin::signed(1),
			4,
			103,
			1_000_000
		));

		let history = EdscOracle::price_history();
		assert_eq!(history.len(), 5);
	});
}

// =============================================================================
// STALENESS TESTS (3 tests)
// =============================================================================

#[test]
fn test_staleness_detection() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit prices
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 100, 1_000_000),
			(1, 3, 100, 1_000_000),
			(1, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Initially not stale
		assert_eq!(EdscOracle::is_stale(), false);

		// Advance past staleness timeout (100 blocks)
		System::set_block_number(150);

		// Should be stale now
		assert_eq!(EdscOracle::is_stale(), true);
	});
}

#[test]
fn test_get_price_fails_when_stale() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit prices
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 100, 1_000_000),
			(1, 3, 100, 1_000_000),
			(1, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Price retrieval should work initially
		assert_ok!(EdscOracle::get_price());

		// Advance to stale state
		System::set_block_number(150);

		// Price retrieval should fail
		assert_err!(
			EdscOracle::get_price(),
			Error::<Test>::OracleStale
		);
	});
}

#[test]
fn test_stale_event_emission() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit prices
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(1, 1, 100, 1_000_000),
			(1, 2, 100, 1_000_000),
			(1, 3, 100, 1_000_000),
			(1, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Advance to stale state
		System::set_block_number(150);

		// Trigger on_finalize
		EdscOracle::on_finalize(System::block_number());

		// Should emit stale event
		System::assert_has_event(Event::OracleStale.into());
	});
}

// =============================================================================
// CIRCUIT BREAKER TESTS (2 tests)
// =============================================================================

#[test]
fn test_pause_unpause_oracle() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Initially not paused
		assert_eq!(EdscOracle::oracle_paused(), false);

		// Pause
		assert_ok!(EdscOracle::pause_oracle(RuntimeOrigin::root()));
		assert_eq!(EdscOracle::oracle_paused(), true);
		System::assert_last_event(Event::OraclePaused.into());

		// Cannot submit while paused
		assert_err!(
			EdscOracle::submit_price(RuntimeOrigin::signed(1), 0, 100, 1_000_000),
			Error::<Test>::OraclePaused
		);

		// Unpause
		assert_ok!(EdscOracle::unpause_oracle(RuntimeOrigin::root()));
		assert_eq!(EdscOracle::oracle_paused(), false);
		System::assert_last_event(Event::OracleUnpaused.into());

		// Can submit again
		assert_ok!(EdscOracle::submit_price(RuntimeOrigin::signed(1), 0, 100, 1_000_000));
	});
}

#[test]
fn test_pause_requires_root() {
	new_test_ext().execute_with(|| {
		// Non-root cannot pause
		assert_err!(
			EdscOracle::pause_oracle(RuntimeOrigin::signed(1)),
			sp_runtime::DispatchError::BadOrigin
		);

		// Non-root cannot unpause
		assert_err!(
			EdscOracle::unpause_oracle(RuntimeOrigin::signed(1)),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

// =============================================================================
// EDGE CASE TESTS (3 tests)
// =============================================================================

#[test]
fn test_empty_price_history() {
	new_test_ext().execute_with(|| {
		// No prices submitted
		assert_eq!(EdscOracle::price_history().len(), 0);

		// TWAP calculation should fail
		assert_err!(
			EdscOracle::calculate_twap(RuntimeOrigin::signed(1)),
			Error::<Test>::InsufficientSources
		);

		// Get price should fail
		assert_err!(
			EdscOracle::get_price(),
			Error::<Test>::InsufficientSources
		);
	});
}

#[test]
fn test_zero_volume_handling() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);

		// Submit prices with zero volume
		let prices = vec![
			(1, 0, 100, 0), // Zero volume
			(1, 1, 100, 0),
			(1, 2, 100, 0),
			(1, 3, 100, 0),
			(1, 4, 100, 0),
		];
		submit_test_prices(prices);

		// Trigger TWAP calculation
		assert_ok!(EdscOracle::calculate_twap(RuntimeOrigin::signed(1)));

		// Should use default weight (1M) for zero volumes
		let twap = EdscOracle::current_twap().unwrap();
		assert_eq!(twap.price, 100);
		assert_eq!(twap.data_points, 5);
	});
}

#[test]
fn test_get_health_returns_complete_status() {
	new_test_ext().execute_with(|| {
		authorize_feeder(1);
		authorize_feeder(2);
		authorize_feeder(3);
		authorize_feeder(4);
		authorize_feeder(5);

		// Submit prices
		let prices = vec![
			(1, 0, 100, 1_000_000),
			(2, 1, 100, 1_000_000),
			(3, 2, 100, 1_000_000),
			(4, 3, 100, 1_000_000),
			(5, 4, 100, 1_000_000),
		];
		submit_test_prices(prices);

		// Get health status
		let (data_points, sources, variance, is_stale) = EdscOracle::get_health().unwrap();
		assert_eq!(data_points, 5);
		assert_eq!(sources, 5);
		assert_eq!(variance, 0); // All prices are identical
		assert_eq!(is_stale, false);
	});
}
