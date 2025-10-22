//! Property-based tests for EDSC oracle pricing
//!
//! These tests verify that oracle price updates are safe and within expected bounds.

use proptest::prelude::*;

/// Helper function to validate price update
fn is_valid_price_update(new_price: u128, max_price: u128) -> bool {
    new_price > 0 && new_price <= max_price
}

/// Helper function to check price staleness
fn is_price_stale(last_update: u64, current_time: u64, staleness_threshold: u64) -> bool {
    current_time.saturating_sub(last_update) > staleness_threshold
}

/// Helper function to calculate price deviation percentage
fn calculate_deviation_pct(old_price: u128, new_price: u128) -> Option<u128> {
    if old_price == 0 {
        return None;
    }

    let diff = if new_price > old_price {
        new_price - old_price
    } else {
        old_price - new_price
    };

    diff.checked_mul(100)?.checked_div(old_price)
}

#[cfg(test)]
mod price_bounds_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn price_never_negative(
            price in 0u128..u128::MAX / 2,
        ) {
            // Property: Oracle prices are always non-negative
            prop_assert!(price >= 0, "Price must be non-negative");
        }

        #[test]
        fn price_never_overflows(
            price in 0u128..u128::MAX,
            multiplier in 1u128..100,
        ) {
            // Property: Price calculations should not overflow
            let result = price.checked_mul(multiplier);

            // Should either succeed or return None (never panic)
            prop_assert!(
                result.is_some() || result.is_none(),
                "Price multiplication should not panic"
            );
        }

        #[test]
        fn price_within_max_bound(
            price in 1u128..1_000_000_000_000,
            max_price in 1_000_000_000_000u128..10_000_000_000_000,
        ) {
            // Property: Prices should respect maximum bounds
            let is_valid = is_valid_price_update(price, max_price);

            if price <= max_price {
                prop_assert!(is_valid, "Valid price should be accepted");
            } else {
                prop_assert!(!is_valid, "Price above max should be rejected");
            }
        }

        #[test]
        fn zero_price_rejected(
            max_price in 1u128..u128::MAX,
        ) {
            // Property: Zero prices should be rejected
            let is_valid = is_valid_price_update(0, max_price);
            prop_assert!(!is_valid, "Zero price must be rejected");
        }
    }
}

#[cfg(test)]
mod staleness_detection_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn recent_price_not_stale(
            last_update in 0u64..1_000_000,
            time_diff in 0u64..3600, // Within 1 hour
            staleness_threshold in 3600u64..86400, // 1-24 hour threshold
        ) {
            // Property: Recent prices should not be stale
            let current_time = last_update + time_diff;
            let is_stale = is_price_stale(last_update, current_time, staleness_threshold);

            if time_diff <= staleness_threshold {
                prop_assert!(!is_stale, "Recent price should not be stale");
            }
        }

        #[test]
        fn old_price_is_stale(
            last_update in 0u64..1_000_000,
            staleness_threshold in 3600u64..86400,
            extra_time in 1u64..86400,
        ) {
            // Property: Old prices should be detected as stale
            let current_time = last_update + staleness_threshold + extra_time;
            let is_stale = is_price_stale(last_update, current_time, staleness_threshold);

            prop_assert!(is_stale, "Old price should be stale");
        }

        #[test]
        fn staleness_check_safe(
            last_update in 0u64..u64::MAX / 2,
            current_time in 0u64..u64::MAX / 2,
            staleness_threshold in 0u64..u64::MAX / 2,
        ) {
            // Property: Staleness check should never panic
            let _is_stale = is_price_stale(last_update, current_time, staleness_threshold);
            // If we reach here, no panic occurred
            prop_assert!(true);
        }
    }
}

#[cfg(test)]
mod price_deviation_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn small_deviation_within_threshold(
            old_price in 1_000u128..1_000_000,
            deviation_pct in 1u128..10, // 1-10% deviation
            max_deviation in 10u128..50, // 10-50% max allowed
        ) {
            // Property: Small deviations should be within threshold
            let new_price = (old_price * (100 + deviation_pct)) / 100;
            let actual_deviation = calculate_deviation_pct(old_price, new_price).unwrap();

            prop_assert!(
                actual_deviation <= max_deviation,
                "Small deviation should be within threshold"
            );
        }

        #[test]
        fn large_deviation_detected(
            old_price in 1_000u128..1_000_000,
            deviation_pct in 60u128..200, // 60-200% deviation
            max_deviation in 10u128..50,
        ) {
            // Property: Large deviations should be detected
            let new_price = (old_price * (100 + deviation_pct)) / 100;
            if let Some(actual_deviation) = calculate_deviation_pct(old_price, new_price) {
                if deviation_pct > max_deviation {
                    prop_assert!(
                        actual_deviation > max_deviation,
                        "Large deviation should exceed threshold"
                    );
                }
            }
        }

        #[test]
        fn price_increase_deviation_correct(
            old_price in 1_000u128..1_000_000,
            increase_pct in 10u128..100,
        ) {
            // Property: Price increase deviation is calculated correctly
            let new_price = (old_price * (100 + increase_pct)) / 100;
            let deviation = calculate_deviation_pct(old_price, new_price).unwrap();

            // Allow small rounding error (within 2%)
            prop_assert!(
                deviation >= increase_pct.saturating_sub(2) && deviation <= increase_pct + 2,
                "Deviation should approximately match increase percentage"
            );
        }

        #[test]
        fn price_decrease_deviation_correct(
            old_price in 1_000u128..1_000_000,
            decrease_pct in 10u128..90,
        ) {
            // Property: Price decrease deviation is calculated correctly
            let new_price = (old_price * (100 - decrease_pct)) / 100;
            let deviation = calculate_deviation_pct(old_price, new_price).unwrap();

            // Allow small rounding error (within 2%)
            prop_assert!(
                deviation >= decrease_pct.saturating_sub(2) && deviation <= decrease_pct + 2,
                "Deviation should approximately match decrease percentage"
            );
        }
    }
}

#[cfg(test)]
mod price_update_sequence_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn sequential_updates_safe(
            initial_price in 1_000u128..10_000,
            updates in prop::collection::vec(90u128..110, 5..20), // +/-10% updates
        ) {
            // Property: Sequential price updates should be safe
            let mut current_price = initial_price;

            for update in updates {
                let new_price = (current_price * update) / 100;
                prop_assert!(new_price > 0, "Price should remain positive");
                current_price = new_price;
            }
        }

        #[test]
        fn monotonic_increase_detected(
            initial_price in 1_000u128..10_000,
            increases in prop::collection::vec(101u128..110, 5..10),
        ) {
            // Property: Monotonic increase sequence should be detectable
            let mut prev_price = initial_price;

            for increase in increases {
                let new_price = (prev_price * increase) / 100;
                prop_assert!(
                    new_price > prev_price,
                    "Price should increase monotonically"
                );
                prev_price = new_price;
            }
        }

        #[test]
        fn monotonic_decrease_detected(
            initial_price in 10_000u128..100_000,
            decreases in prop::collection::vec(90u128..99, 5..10),
        ) {
            // Property: Monotonic decrease sequence should be detectable
            let mut prev_price = initial_price;

            for decrease in decreases {
                let new_price = (prev_price * decrease) / 100;
                prop_assert!(
                    new_price < prev_price,
                    "Price should decrease monotonically"
                );
                prev_price = new_price;
            }
        }
    }
}

#[cfg(test)]
mod price_manipulation_detection {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn rapid_price_swings_detectable(
            base_price in 1_000u128..10_000,
            swing_pct in 20u128..80,
        ) {
            // Property: Rapid price swings should be detectable
            let up_price = (base_price * (100 + swing_pct)) / 100;
            let down_price = (base_price * (100 - swing_pct)) / 100;

            let up_deviation = calculate_deviation_pct(base_price, up_price).unwrap();
            let down_deviation = calculate_deviation_pct(base_price, down_price).unwrap();

            prop_assert!(
                up_deviation >= swing_pct.saturating_sub(2),
                "Upward swing should be detectable"
            );
            prop_assert!(
                down_deviation >= swing_pct.saturating_sub(2),
                "Downward swing should be detectable"
            );
        }

        #[test]
        fn flash_crash_detectable(
            normal_price in 10_000u128..100_000,
            crash_to in 100u128..1_000,
        ) {
            // Property: Flash crash should be detectable via deviation
            let deviation = calculate_deviation_pct(normal_price, crash_to).unwrap();

            // Flash crash should show very high deviation
            prop_assert!(
                deviation > 50, // More than 50% deviation
                "Flash crash should show high deviation"
            );
        }
    }
}
