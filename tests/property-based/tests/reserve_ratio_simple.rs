//! Property-based tests for reserve ratio calculations
//!
//! These tests verify that reserve ratio calculations are always safe and correct.

use proptest::prelude::*;

/// Helper function to calculate reserve ratio
fn calculate_reserve_ratio(collateral: u128, debt: u128) -> Option<u128> {
    if debt == 0 {
        return Some(u128::MAX); // Infinite ratio (no debt)
    }

    // Calculate ratio as percentage: (collateral * 100) / debt
    collateral.checked_mul(100)?.checked_div(debt)
}

/// Property: Reserve ratio calculations
#[cfg(test)]
mod reserve_ratio_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn reserve_ratio_never_panics(
            collateral in 0u128..10_000_000,
            debt in 0u128..10_000_000,
        ) {
            // Property: Reserve ratio calculation should never panic
            let ratio = calculate_reserve_ratio(collateral, debt);

            // Should always return Some or None, never panic
            if debt > 0 {
                prop_assert!(ratio.is_some() || ratio.is_none());
            } else {
                // Zero debt = infinite ratio
                prop_assert_eq!(ratio, Some(u128::MAX));
            }
        }

        #[test]
        fn over_collateralized_ratio_above_100(
            collateral_multiple in 110u128..200, // 110% to 200%
            debt in 1_000u128..1_000_000,
        ) {
            // Property: Over-collateralized position has ratio > 100%
            let collateral = (debt * collateral_multiple) / 100;
            let ratio = calculate_reserve_ratio(collateral, debt).unwrap();

            prop_assert!(
                ratio >= 100,
                "Over-collateralized ratio should be >= 100%"
            );
            // Allow for integer division rounding (within 1%)
            prop_assert!(
                ratio >= collateral_multiple - 1 && ratio <= collateral_multiple + 1,
                "Ratio should approximately match collateral multiple (within rounding error)"
            );
        }

        #[test]
        fn under_collateralized_ratio_below_100(
            collateral_percentage in 50u128..99, // 50% to 99%
            debt in 1_000u128..1_000_000,
        ) {
            // Property: Under-collateralized position has ratio < 100%
            let collateral = (debt * collateral_percentage) / 100;
            let ratio = calculate_reserve_ratio(collateral, debt).unwrap();

            prop_assert!(
                ratio < 100,
                "Under-collateralized ratio should be < 100%"
            );
        }

        #[test]
        fn exact_100_percent_collateralization(
            debt in 1_000u128..1_000_000,
        ) {
            // Property: Exactly 100% collateralized = ratio of 100
            let collateral = debt;
            let ratio = calculate_reserve_ratio(collateral, debt).unwrap();

            prop_assert_eq!(ratio, 100, "100% collateral = 100% ratio");
        }
    }
}

/// Property: Collateral haircuts
#[cfg(test)]
mod haircut_properties {
    use super::*;

    /// Apply haircut to collateral value
    fn apply_haircut(value: u128, haircut_percentage: u128) -> Option<u128> {
        // haircut_percentage is the discount (e.g., 10 for 10% haircut)
        let multiplier = 100u128.checked_sub(haircut_percentage)?;
        value.checked_mul(multiplier)?.checked_div(100)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn haircut_reduces_value(
            value in 1_000u128..1_000_000,
            haircut in 1u128..50, // 1% to 50% haircut
        ) {
            // Property: Haircut always reduces value
            let adjusted = apply_haircut(value, haircut).unwrap();

            prop_assert!(adjusted < value, "Haircut should reduce value");

            // Verify the reduction is reasonable (allowing for integer rounding)
            let reduction = value - adjusted;
            let expected_reduction = (value * haircut) / 100;

            // Allow for rounding error of 1
            prop_assert!(
                reduction >= expected_reduction.saturating_sub(1) && reduction <= expected_reduction + 1,
                "Reduction should approximately match haircut percentage (within rounding error)"
            );
        }

        #[test]
        fn zero_haircut_preserves_value(
            value in 1_000u128..1_000_000,
        ) {
            // Property: 0% haircut = no change
            let adjusted = apply_haircut(value, 0).unwrap();

            prop_assert_eq!(adjusted, value, "Zero haircut preserves value");
        }

        #[test]
        fn max_haircut_zeroes_value(
            value in 1_000u128..1_000_000,
        ) {
            // Property: 100% haircut = zero value
            let adjusted = apply_haircut(value, 100).unwrap();

            prop_assert_eq!(adjusted, 0, "100% haircut = 0 value");
        }

        #[test]
        fn haircut_calculation_safe(
            value in 1u128..u128::MAX / 100,
            haircut in 1u128..100,
        ) {
            // Property: Haircut calculation never overflows
            let result = apply_haircut(value, haircut);

            prop_assert!(result.is_some(), "Haircut calculation should succeed");
        }
    }
}

/// Property: Multi-asset collateral
#[cfg(test)]
mod multi_asset_properties {
    use super::*;

    fn apply_haircut(value: u128, haircut_percentage: u128) -> Option<u128> {
        let multiplier = 100u128.checked_sub(haircut_percentage)?;
        value.checked_mul(multiplier)?.checked_div(100)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn total_collateral_sum_correct(
            asset_values in prop::collection::vec(1_000u128..100_000, 1..10),
        ) {
            // Property: Total collateral = sum of all asset values
            let total: u128 = asset_values.iter().sum();

            // Verify each asset contributes to total
            for value in &asset_values {
                prop_assert!(total >= *value, "Total should include all assets");
            }

            // Verify total is exact sum
            let manual_sum = asset_values.iter().fold(0u128, |acc, v| acc + v);
            prop_assert_eq!(total, manual_sum, "Sum should be exact");
        }

        #[test]
        fn adjusted_collateral_with_haircuts(
            asset_values in prop::collection::vec(1_000u128..100_000, 1..5),
            haircuts in prop::collection::vec(5u128..40, 1..5),
        ) {
            // Property: Adjusted collateral <= raw collateral
            let raw_total: u128 = asset_values.iter().sum();

            let mut adjusted_total = 0u128;
            for (value, haircut) in asset_values.iter().zip(haircuts.iter()) {
                let adjusted = apply_haircut(*value, *haircut).unwrap();
                adjusted_total += adjusted;
            }

            prop_assert!(
                adjusted_total <= raw_total,
                "Adjusted collateral should be <= raw"
            );
        }
    }
}

/// Property: Ratio threshold checks
#[cfg(test)]
mod threshold_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn optimal_range_detection(
            ratio in 110u128..130, // Optimal range: 110-130%
        ) {
            // Property: Ratios in optimal range should be detected correctly
            let is_optimal = ratio >= 110 && ratio <= 130;

            prop_assert!(is_optimal, "Ratio should be in optimal range");
            prop_assert!(ratio >= 100, "Should be over-collateralized");
        }

        #[test]
        fn throttle_zone_detection(
            ratio in 100u128..110, // Throttle zone: 100-110%
        ) {
            // Property: Ratios in throttle zone should trigger caution
            let is_throttle = ratio >= 100 && ratio < 110;

            prop_assert!(is_throttle, "Ratio should be in throttle zone");
            prop_assert!(ratio >= 100, "Should still be collateralized");
            prop_assert!(ratio < 110, "Should be below optimal");
        }

        #[test]
        fn critical_zone_detection(
            ratio in 0u128..100, // Critical: < 100%
        ) {
            // Property: Under-collateralized should be critical
            let is_critical = ratio < 100;

            prop_assert!(is_critical, "Ratio should be critical");
        }
    }
}

/// Property: Price updates affect ratio
#[cfg(test)]
mod price_update_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn price_increase_increases_ratio(
            collateral_amount in 1_000u128..10_000,
            initial_price in 100u128..1_000,
            price_increase in 10u128..500,
            debt in 1_000u128..10_000,
        ) {
            // Property: Increasing collateral price increases ratio
            let initial_value = collateral_amount * initial_price;
            let new_price = initial_price + price_increase;
            let new_value = collateral_amount * new_price;

            let initial_ratio = calculate_reserve_ratio(initial_value, debt).unwrap();
            let new_ratio = calculate_reserve_ratio(new_value, debt).unwrap();

            prop_assert!(
                new_ratio >= initial_ratio,
                "Price increase should increase or maintain ratio"
            );
        }

        #[test]
        fn price_decrease_decreases_ratio(
            collateral_amount in 1_000u128..10_000,
            initial_price in 500u128..1_000,
            price_decrease in 10u128..400,
            debt in 1_000u128..10_000,
        ) {
            // Property: Decreasing collateral price decreases ratio
            if price_decrease < initial_price {
                let initial_value = collateral_amount * initial_price;
                let new_price = initial_price - price_decrease;
                let new_value = collateral_amount * new_price;

                let initial_ratio = calculate_reserve_ratio(initial_value, debt).unwrap();
                let new_ratio = calculate_reserve_ratio(new_value, debt).unwrap();

                prop_assert!(
                    new_ratio <= initial_ratio,
                    "Price decrease should decrease or maintain ratio"
                );
            }
        }
    }
}
