use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::ConstU32, BoundedVec};
use sp_runtime::DispatchError;

// ========== Oracle Management Tests ==========

#[test]
fn add_trusted_oracle_works() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);

		// Add oracle
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Verify oracle is trusted
		assert!(ReserveOracle::is_oracle_trusted(&oracle));

		// Check event
		System::assert_last_event(Event::OracleAdded { oracle }.into());
	});
}

#[test]
fn remove_trusted_oracle_works() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);

		// Add oracle
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));
		assert!(ReserveOracle::is_oracle_trusted(&oracle));

		// Remove oracle
		assert_ok!(ReserveOracle::remove_trusted_oracle(RuntimeOrigin::root(), oracle));
		assert!(!ReserveOracle::is_oracle_trusted(&oracle));

		// Check event
		System::assert_last_event(Event::OracleRemoved { oracle }.into());
	});
}

#[test]
fn only_root_can_add_oracle() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		let non_root = account(2);

		// Try to add oracle as non-root
		assert_noop!(
			ReserveOracle::add_trusted_oracle(RuntimeOrigin::signed(non_root), oracle),
			DispatchError::BadOrigin
		);
	});
}

// ========== Price Submission Tests ==========

#[test]
fn submit_price_works() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);

		// Add trusted oracle
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit price
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"ETH".to_vec(),
			2000_00000000u128, // $2000
			b"chainlink".to_vec(),
			95
		));

		// Check event
		System::assert_has_event(
			Event::PriceSubmitted {
				oracle,
				asset_symbol: b"ETH".to_vec(),
				price: 2000_00000000u128,
				confidence: 95,
			}
			.into(),
		);
	});
}

#[test]
fn untrusted_oracle_cannot_submit_price() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);

		// Try to submit price without being trusted
		assert_noop!(
			ReserveOracle::submit_price(
				RuntimeOrigin::signed(oracle),
				b"ETH".to_vec(),
				2000_00000000u128,
				b"chainlink".to_vec(),
				95
			),
			Error::<Test>::OracleNotTrusted
		);
	});
}

#[test]
fn submit_price_rejects_zero_price() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		assert_noop!(
			ReserveOracle::submit_price(
				RuntimeOrigin::signed(oracle),
				b"ETH".to_vec(),
				0u128,
				b"chainlink".to_vec(),
				95
			),
			Error::<Test>::InvalidPrice
		);
	});
}

#[test]
fn submit_price_rejects_invalid_confidence() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		assert_noop!(
			ReserveOracle::submit_price(
				RuntimeOrigin::signed(oracle),
				b"ETH".to_vec(),
				2000_00000000u128,
				b"chainlink".to_vec(),
				101 // Invalid: > 100
			),
			Error::<Test>::InvalidConfidence
		);
	});
}

#[test]
fn submit_price_rejects_too_long_symbol() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		let long_symbol = vec![b'A'; 17]; // Max is 16

		assert_noop!(
			ReserveOracle::submit_price(
				RuntimeOrigin::signed(oracle),
				long_symbol,
				2000_00000000u128,
				b"chainlink".to_vec(),
				95
			),
			Error::<Test>::AssetSymbolTooLong
		);
	});
}

#[test]
fn submit_price_rejects_too_long_source() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		let long_source = vec![b'A'; 33]; // Max is 32

		assert_noop!(
			ReserveOracle::submit_price(
				RuntimeOrigin::signed(oracle),
				b"ETH".to_vec(),
				2000_00000000u128,
				long_source,
				95
			),
			Error::<Test>::SourceNameTooLong
		);
	});
}

// ========== Multi-Source Price Tests ==========

#[test]
fn multi_source_price_submission_works() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		let oracle3 = account(3);

		// Add oracles
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle3));

		// Submit prices from different sources
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"chainlink".to_vec(),
			95
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"ETH".to_vec(),
			2010_00000000u128,
			b"band".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle3),
			b"ETH".to_vec(),
			1995_00000000u128,
			b"dia".to_vec(),
			92
		));

		// Verify aggregated price exists
		let symbol: BoundedVec<u8, ConstU32<16>> = b"ETH".to_vec().try_into().unwrap();
		let aggregated = ReserveOracle::get_aggregated_price(b"ETH").unwrap();
		assert_eq!(aggregated.sources_count, 3);
	});
}

// ========== Aggregation Tests ==========

#[test]
fn median_calculation_works() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		let oracle3 = account(3);

		// Add oracles
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle3));

		// Submit prices: 100, 102, 101
		// Median should be 101
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"TEST".to_vec(),
			102_00000000u128,
			b"source2".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle3),
			b"TEST".to_vec(),
			101_00000000u128,
			b"source3".to_vec(),
			90
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.median_price, 101_00000000u128);
	});
}

#[test]
fn median_calculation_even_count_works() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		let oracle3 = account(3);
		let oracle4 = account(4);

		// Add oracles
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle3));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle4));

		// Submit prices: 100, 101, 102, 103
		// Median should be (101 + 102) / 2 = 101.5 = 101
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"TEST".to_vec(),
			101_00000000u128,
			b"source2".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle3),
			b"TEST".to_vec(),
			102_00000000u128,
			b"source3".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle4),
			b"TEST".to_vec(),
			103_00000000u128,
			b"source4".to_vec(),
			90
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		// Average of 101 and 102
		assert_eq!(aggregated.median_price, 101_50000000u128);
	});
}

#[test]
fn weighted_mean_calculation_works() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);

		// Add oracles
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));

		// Submit prices with different confidence levels
		// Price 100 with confidence 80
		// Price 120 with confidence 20
		// Weighted mean = (100*80 + 120*20) / (80+20) = 10400/100 = 104
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			80
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"TEST".to_vec(),
			120_00000000u128,
			b"source2".to_vec(),
			20
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.mean_price, 104_00000000u128);
	});
}

#[test]
fn outlier_filtering_works() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		let oracle3 = account(3);
		let oracle4 = account(4);
		let oracle5 = account(5);

		// Add oracles
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle3));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle4));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle5));

		// Submit prices: 100, 101, 102, 103, 1000 (outlier)
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"TEST".to_vec(),
			101_00000000u128,
			b"source2".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle3),
			b"TEST".to_vec(),
			102_00000000u128,
			b"source3".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle4),
			b"TEST".to_vec(),
			103_00000000u128,
			b"source4".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle5),
			b"TEST".to_vec(),
			1000_00000000u128, // Outlier
			b"source5".to_vec(),
			90
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();

		// Outlier should be filtered, so we should have 4 sources
		assert_eq!(aggregated.sources_count, 4);

		// Median of 100, 101, 102, 103 = (101+102)/2 = 101.5
		assert_eq!(aggregated.median_price, 101_50000000u128);
	});
}

#[test]
fn confidence_score_calculation_works() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		let oracle3 = account(3);

		// Add oracles
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle3));

		// Submit prices with confidence 90, 80, 70
		// Average = 80, bonus for 3 sources = 3*4 = 12
		// Total = 80 + 12 = 92
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"TEST".to_vec(),
			101_00000000u128,
			b"source2".to_vec(),
			80
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle3),
			b"TEST".to_vec(),
			102_00000000u128,
			b"source3".to_vec(),
			70
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		// (90+80+70)/3 = 80, bonus = 3*4 = 12, total = 92
		assert_eq!(aggregated.confidence_score, 92);
	});
}

#[test]
fn confidence_score_caps_at_100() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		let oracle3 = account(3);
		let oracle4 = account(4);
		let oracle5 = account(5);

		// Add 5 oracles
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle3));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle4));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle5));

		// All submit with 100 confidence
		// Average = 100, bonus = 5*4 = 20, total = 120 (should cap at 100)
		for i in 1..=5 {
			assert_ok!(ReserveOracle::submit_price(
				RuntimeOrigin::signed(account(i)),
				b"TEST".to_vec(),
				100_00000000u128,
				format!("source{}", i).as_bytes().to_vec(),
				100
			));
		}

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.confidence_score, 100); // Capped at 100
	});
}

// ========== Single Source Edge Cases ==========

#[test]
fn single_source_aggregation_works() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.sources_count, 1);
		assert_eq!(aggregated.median_price, 100_00000000u128);
		assert_eq!(aggregated.mean_price, 100_00000000u128);
	});
}

#[test]
fn two_sources_no_outlier_filtering() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);

		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));

		// Even with very different prices, both should be kept (< 3 sources)
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"TEST".to_vec(),
			200_00000000u128,
			b"source2".to_vec(),
			90
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.sources_count, 2);
	});
}

// ========== Staleness Detection Tests ==========

#[test]
fn price_staleness_detection_works() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit price at block 1
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		// Advance beyond MaxPriceAge (300 blocks)
		run_to_block(305);

		// Check staleness
		assert_ok!(ReserveOracle::check_price_staleness_manual(
			RuntimeOrigin::signed(account(2)),
			b"TEST".to_vec()
		));

		// Should emit PriceStale event
		System::assert_has_event(
			Event::PriceStale {
				asset_symbol: b"TEST".to_vec(),
				age_blocks: 304,
			}
			.into(),
		);

		// Should also emit FailoverTriggered event
		System::assert_has_event(
			Event::FailoverTriggered { asset_symbol: b"TEST".to_vec() }.into(),
		);
	});
}

#[test]
fn price_staleness_not_triggered_when_fresh() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit price at block 1
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		// Advance only 50 blocks (less than MaxPriceAge)
		run_to_block(51);

		// Check staleness
		assert_ok!(ReserveOracle::check_price_staleness_manual(
			RuntimeOrigin::signed(account(2)),
			b"TEST".to_vec()
		));

		// Should NOT emit staleness events
		let events = System::events();
		let has_stale_event = events
			.iter()
			.any(|e| matches!(e.event, RuntimeEvent::ReserveOracle(Event::PriceStale { .. })));
		assert!(!has_stale_event);
	});
}

#[test]
fn check_staleness_fails_for_nonexistent_price() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			ReserveOracle::check_price_staleness_manual(
				RuntimeOrigin::signed(account(1)),
				b"NONEXISTENT".to_vec()
			),
			Error::<Test>::PriceNotFound
		);
	});
}

// ========== Integration Tests ==========

#[test]
fn full_oracle_workflow() {
	new_test_ext().execute_with(|| {
		// 1. Add multiple oracles
		let oracle1 = account(1);
		let oracle2 = account(2);
		let oracle3 = account(3);

		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle3));

		// 2. Submit prices from multiple sources
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"chainlink".to_vec(),
			95
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"ETH".to_vec(),
			2005_00000000u128,
			b"band".to_vec(),
			92
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle3),
			b"ETH".to_vec(),
			1998_00000000u128,
			b"dia".to_vec(),
			90
		));

		// 3. Verify aggregated price
		let aggregated = ReserveOracle::get_aggregated_price(b"ETH").unwrap();
		assert_eq!(aggregated.sources_count, 3);
		assert_eq!(aggregated.median_price, 2000_00000000u128); // Middle value
		assert!(aggregated.confidence_score > 90); // Should have bonus

		// 4. Update price from one source
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"ETH".to_vec(),
			2010_00000000u128,
			b"chainlink".to_vec(),
			96
		));

		// 5. Verify updated aggregation
		let updated = ReserveOracle::get_aggregated_price(b"ETH").unwrap();
		assert_eq!(updated.sources_count, 3);
		// Median should now be 2005 (middle of 1998, 2005, 2010)
		assert_eq!(updated.median_price, 2005_00000000u128);
	});
}

#[test]
fn multiple_assets_work_independently() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit prices for different assets
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"source1".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"BTC".to_vec(),
			40000_00000000u128,
			b"source1".to_vec(),
			90
		));

		// Verify both assets have aggregated prices
		let eth_price = ReserveOracle::get_aggregated_price(b"ETH").unwrap();
		let btc_price = ReserveOracle::get_aggregated_price(b"BTC").unwrap();

		assert_eq!(eth_price.median_price, 2000_00000000u128);
		assert_eq!(btc_price.median_price, 40000_00000000u128);
	});
}

#[test]
fn removing_oracle_prevents_future_submissions() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);

		// Add and remove oracle
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));
		assert_ok!(ReserveOracle::remove_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Try to submit price
		assert_noop!(
			ReserveOracle::submit_price(
				RuntimeOrigin::signed(oracle),
				b"ETH".to_vec(),
				2000_00000000u128,
				b"chainlink".to_vec(),
				95
			),
			Error::<Test>::OracleNotTrusted
		);
	});
}

// ========== Legacy Asset Price Tests ==========

#[test]
fn update_asset_price_legacy_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(ReserveOracle::update_asset_price(
			RuntimeOrigin::root(),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"chainlink".to_vec()
		));

		let price = ReserveOracle::get_asset_price(b"ETH");
		assert_eq!(price, Some(2000_00000000u128));
	});
}

#[test]
fn force_snapshot_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(ReserveOracle::force_snapshot(RuntimeOrigin::root()));

		let snapshot = ReserveOracle::latest_snapshot();
		assert!(snapshot.is_some());
	});
}

#[test]
fn clear_alert_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(ReserveOracle::clear_alert(RuntimeOrigin::root()));
		assert_eq!(ReserveOracle::alert_active(), false);
	});
}

// ========== Additional Comprehensive Tests ==========

#[test]
fn max_source_count_aggregation() {
	new_test_ext().execute_with(|| {
		// Add 10 oracles
		for i in 1..=10 {
			assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), account(i)));
		}

		// Submit prices from 10 different sources
		for i in 1..=10 {
			assert_ok!(ReserveOracle::submit_price(
				RuntimeOrigin::signed(account(i)),
				b"ETH".to_vec(),
				(2000_00000000u128 + (i as u128 * 100000000)),
				format!("source{}", i).as_bytes().to_vec(),
				90
			));
		}

		let aggregated = ReserveOracle::get_aggregated_price(b"ETH").unwrap();
		assert_eq!(aggregated.sources_count, 10);
	});
}

#[test]
fn price_update_overwrites_same_source() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit initial price
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"chainlink".to_vec(),
			90
		));

		// Update price from same source
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"ETH".to_vec(),
			2100_00000000u128,
			b"chainlink".to_vec(),
			95
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"ETH").unwrap();
		// Should still have 1 source (same source updated)
		assert_eq!(aggregated.sources_count, 1);
		assert_eq!(aggregated.median_price, 2100_00000000u128);
	});
}

#[test]
fn extreme_price_values_work() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit very large price (near u128 max)
		let large_price = u128::MAX / 2;
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST".to_vec(),
			large_price,
			b"source1".to_vec(),
			90
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.median_price, large_price);
	});
}

#[test]
fn multiple_outliers_filtered() {
	new_test_ext().execute_with(|| {
		// Add 7 oracles
		for i in 1..=7 {
			assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), account(i)));
		}

		// Submit prices: 100, 101, 102, 103, 104 (normal) + 1000, 2000 (outliers)
		for i in 1..=5 {
			assert_ok!(ReserveOracle::submit_price(
				RuntimeOrigin::signed(account(i)),
				b"TEST".to_vec(),
				(100_00000000u128 + ((i - 1) as u128 * 100000000)),
				format!("source{}", i).as_bytes().to_vec(),
				90
			));
		}

		// Add outliers
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(account(6)),
			b"TEST".to_vec(),
			1000_00000000u128,
			b"source6".to_vec(),
			90
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(account(7)),
			b"TEST".to_vec(),
			2000_00000000u128,
			b"source7".to_vec(),
			90
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		// Should filter the extreme outlier (2000), 1000 may or may not be filtered depending on threshold
		// With 1.85 std dev threshold and this data distribution, typically only 2000 is filtered
		assert_eq!(aggregated.sources_count, 6);
	});
}

#[test]
fn confidence_varies_with_source_count() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Single source
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST1".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			80
		));

		let single = ReserveOracle::get_aggregated_price(b"TEST1").unwrap();
		let single_confidence = single.confidence_score;

		// Add more oracles for multi-source test
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), account(2)));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), account(3)));

		// Three sources with same confidence
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST2".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			80
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(account(2)),
			b"TEST2".to_vec(),
			101_00000000u128,
			b"source2".to_vec(),
			80
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(account(3)),
			b"TEST2".to_vec(),
			102_00000000u128,
			b"source3".to_vec(),
			80
		));

		let multi = ReserveOracle::get_aggregated_price(b"TEST2").unwrap();
		let multi_confidence = multi.confidence_score;

		// Multi-source should have higher confidence due to source bonus
		assert!(multi_confidence > single_confidence);
	});
}

#[test]
fn zero_confidence_handled() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit with 0 confidence (edge case)
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			0
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.sources_count, 1);
		// Should still work, just with low confidence
		assert!(aggregated.confidence_score > 0); // Gets source bonus
	});
}

#[test]
fn all_same_price_no_variance() {
	new_test_ext().execute_with(|| {
		// Add 5 oracles
		for i in 1..=5 {
			assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), account(i)));
		}

		// All submit same price
		for i in 1..=5 {
			assert_ok!(ReserveOracle::submit_price(
				RuntimeOrigin::signed(account(i)),
				b"TEST".to_vec(),
				100_00000000u128,
				format!("source{}", i).as_bytes().to_vec(),
				90
			));
		}

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		assert_eq!(aggregated.median_price, 100_00000000u128);
		assert_eq!(aggregated.mean_price, 100_00000000u128);
		assert_eq!(aggregated.sources_count, 5);
	});
}

#[test]
fn price_aggregation_triggered_automatically() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit price
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"chainlink".to_vec(),
			95
		));

		// Verify PriceAggregated event was emitted
		System::assert_has_event(
			Event::PriceAggregated {
				asset_symbol: b"ETH".to_vec(),
				median_price: 2000_00000000u128,
				sources_count: 1,
			}
			.into(),
		);
	});
}

#[test]
fn staleness_check_boundary_conditions() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// Submit price at block 1
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			90
		));

		// Advance exactly to max age boundary (300 blocks)
		run_to_block(301);

		// At boundary, should not be stale yet (age=300, max=300, 300 > 300 is false)
		let result = ReserveOracle::check_price_staleness_manual(
			RuntimeOrigin::signed(account(2)),
			b"TEST".to_vec(),
		);
		assert_ok!(result);

		// Check for stale event - should NOT be present at boundary
		let events = System::events();
		let has_stale = events
			.iter()
			.any(|e| matches!(e.event, RuntimeEvent::ReserveOracle(Event::PriceStale { .. })));
		assert!(!has_stale); // Not stale yet at age=300 with max=300
	});
}

#[test]
fn different_assets_different_confidences() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));

		// ETH with high confidence
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"source1".to_vec(),
			95
		));
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"ETH".to_vec(),
			2001_00000000u128,
			b"source2".to_vec(),
			94
		));

		// BTC with lower confidence
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"BTC".to_vec(),
			40000_00000000u128,
			b"source1".to_vec(),
			70
		));

		let eth = ReserveOracle::get_aggregated_price(b"ETH").unwrap();
		let btc = ReserveOracle::get_aggregated_price(b"BTC").unwrap();

		// ETH should have higher confidence
		assert!(eth.confidence_score > btc.confidence_score);
	});
}

#[test]
fn weighted_mean_favors_high_confidence() {
	new_test_ext().execute_with(|| {
		let oracle1 = account(1);
		let oracle2 = account(2);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle1));
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle2));

		// Price 100 with very high confidence (95)
		// Price 200 with low confidence (5)
		// Weighted mean should be much closer to 100
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle1),
			b"TEST".to_vec(),
			100_00000000u128,
			b"source1".to_vec(),
			95
		));

		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle2),
			b"TEST".to_vec(),
			200_00000000u128,
			b"source2".to_vec(),
			5
		));

		let aggregated = ReserveOracle::get_aggregated_price(b"TEST").unwrap();
		// Weighted mean = (100*95 + 200*5) / (95+5) = 10500/100 = 105
		assert_eq!(aggregated.mean_price, 105_00000000u128);
		// Should be much closer to 100 than to 200
		assert!(aggregated.mean_price < 150_00000000u128);
	});
}

#[test]
fn sequential_updates_maintain_consistency() {
	new_test_ext().execute_with(|| {
		let oracle = account(1);
		assert_ok!(ReserveOracle::add_trusted_oracle(RuntimeOrigin::root(), oracle));

		// First submission
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"ETH".to_vec(),
			2000_00000000u128,
			b"chainlink".to_vec(),
			90
		));

		let first = ReserveOracle::get_aggregated_price(b"ETH").unwrap();

		// Second submission (update)
		assert_ok!(ReserveOracle::submit_price(
			RuntimeOrigin::signed(oracle),
			b"ETH".to_vec(),
			2100_00000000u128,
			b"chainlink".to_vec(),
			92
		));

		let second = ReserveOracle::get_aggregated_price(b"ETH").unwrap();

		// Verify update worked
		assert_eq!(second.median_price, 2100_00000000u128);
		assert!(second.timestamp >= first.timestamp);
	});
}

#[test]
fn aggregation_module_median_tests() {
	// Test the aggregation module functions
	use crate::aggregation::*;

	// Odd count
	let prices = vec![(100, 90), (102, 90), (101, 90)];
	assert_eq!(calculate_median(&prices), 101);

	// Even count
	let prices = vec![(100, 90), (101, 90), (102, 90), (103, 90)];
	assert_eq!(calculate_median(&prices), 101);

	// Single
	let prices = vec![(100, 90)];
	assert_eq!(calculate_median(&prices), 100);

	// Empty
	let prices: Vec<(u128, u8)> = vec![];
	assert_eq!(calculate_median(&prices), 0);
}

#[test]
fn aggregation_module_outlier_tests() {
	use crate::aggregation::*;

	// With outlier
	let prices = vec![(100, 90), (101, 90), (102, 90), (103, 90), (1000, 90)];
	let (filtered, removed) = filter_outliers(&prices);
	assert_eq!(filtered.len(), 4);
	assert_eq!(removed, 1);

	// Small dataset (no filtering)
	let prices = vec![(100, 90), (1000, 90)];
	let (filtered, removed) = filter_outliers(&prices);
	assert_eq!(filtered.len(), 2);
	assert_eq!(removed, 0);
}

#[test]
fn aggregation_module_confidence_tests() {
	use crate::aggregation::*;

	// Average confidence with source bonus
	let prices = vec![(100, 90), (101, 80), (102, 70)];
	// Avg = 80, bonus = 12, total = 92
	assert_eq!(calculate_confidence_score(&prices), 92);

	// Cap at 100
	let prices = vec![(100, 100), (101, 100), (102, 100), (103, 100), (104, 100)];
	assert_eq!(calculate_confidence_score(&prices), 100);

	// Empty
	let prices: Vec<(u128, u8)> = vec![];
	assert_eq!(calculate_confidence_score(&prices), 0);
}

#[test]
fn aggregation_module_staleness_tests() {
	use crate::aggregation::*;

	assert!(is_price_stale(301, 300));
	assert!(!is_price_stale(300, 300));
	assert!(!is_price_stale(299, 300));
}

#[test]
fn aggregation_module_deviation_tests() {
	use crate::aggregation::*;

	assert_eq!(calculate_deviation_percent(100, 110), 10);
	assert_eq!(calculate_deviation_percent(110, 100), 9);
	assert_eq!(calculate_deviation_percent(100, 100), 0);
	assert_eq!(calculate_deviation_percent(0, 100), 0);
}
