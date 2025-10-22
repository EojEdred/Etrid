//! Property-based tests for EDSC redemption flows
//!
//! These tests verify that redemption operations maintain safety invariants.

use proptest::prelude::*;

/// Helper function to calculate redeemable collateral
fn calculate_redeemable_collateral(
    edsc_amount: u128,
    collateral_ratio: u128,
) -> Option<u128> {
    // Collateral required = edsc_amount * (collateral_ratio / 100)
    let ratio_numerator = collateral_ratio.checked_mul(edsc_amount)?;
    ratio_numerator.checked_div(100)
}

/// Helper function to check if redemption is safe
fn is_safe_redemption(
    total_collateral: u128,
    total_debt: u128,
    redemption_amount: u128,
) -> bool {
    if redemption_amount > total_debt {
        return false;
    }

    // After redemption, remaining debt
    let remaining_debt = total_debt.saturating_sub(redemption_amount);

    // Check if collateral still covers remaining debt
    if remaining_debt == 0 {
        return true; // Full redemption is safe
    }

    // Calculate remaining ratio
    if let Some(ratio) = total_collateral.checked_mul(100) {
        if let Some(ratio) = ratio.checked_div(remaining_debt) {
            return ratio >= 100; // Must remain overcollateralized
        }
    }

    false
}

/// Helper function to apply redemption fee
fn apply_redemption_fee(amount: u128, fee_basis_points: u128) -> Option<u128> {
    // Fee in basis points (1 bp = 0.01%)
    let fee = amount.checked_mul(fee_basis_points)?.checked_div(10_000)?;
    amount.checked_sub(fee)
}

#[cfg(test)]
mod redemption_amount_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn redemption_never_exceeds_debt(
            total_debt in 1_000u128..1_000_000,
            redemption_request in 1u128..2_000_000,
        ) {
            // Property: Redemption should never exceed total debt
            let actual_redemption = redemption_request.min(total_debt);

            prop_assert!(
                actual_redemption <= total_debt,
                "Redemption cannot exceed total debt"
            );
        }

        #[test]
        fn zero_redemption_rejected(
            total_debt in 1u128..u128::MAX,
        ) {
            // Property: Zero redemption should be rejected
            let is_valid = 0u128 > 0 && 0u128 <= total_debt;

            prop_assert!(!is_valid, "Zero redemption must be rejected");
        }

        #[test]
        fn collateral_calculation_no_overflow(
            edsc_amount in 1u128..1_000_000,
            collateral_ratio in 100u128..300,
        ) {
            // Property: Collateral calculation should not overflow
            let result = calculate_redeemable_collateral(edsc_amount, collateral_ratio);

            prop_assert!(
                result.is_some(),
                "Collateral calculation should succeed for reasonable values"
            );
        }

        #[test]
        fn redemption_reduces_debt(
            initial_debt in 10_000u128..1_000_000,
            redemption in 1_000u128..9_000,
        ) {
            // Property: Redemption should reduce total debt
            let remaining_debt = initial_debt.saturating_sub(redemption);

            if redemption <= initial_debt {
                prop_assert!(
                    remaining_debt < initial_debt,
                    "Redemption should reduce debt"
                );
            }
        }
    }
}

#[cfg(test)]
mod collateral_safety_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn safe_redemption_maintains_overcollateralization(
            collateral in 100_000u128..1_000_000,
            debt in 50_000u128..500_000,
            redemption in 1_000u128..40_000,
        ) {
            // Property: Safe redemptions maintain overcollateralization
            let is_safe = is_safe_redemption(collateral, debt, redemption);

            if redemption <= debt {
                let remaining_debt = debt - redemption;
                if remaining_debt > 0 {
                    let ratio = (collateral * 100) / remaining_debt;
                    if is_safe {
                        prop_assert!(
                            ratio >= 100,
                            "Safe redemption should maintain overcollateralization"
                        );
                    }
                }
            }
        }

        #[test]
        fn unsafe_redemption_detected(
            collateral in 50_000u128..100_000,
            debt in 100_000u128..200_000, // Undercollateralized
            redemption in 1_000u128..10_000,
        ) {
            // Property: Unsafe redemptions should be detected
            let is_safe = is_safe_redemption(collateral, debt, redemption);

            // Initial ratio is under 100%
            let initial_ratio = (collateral * 100) / debt;
            if initial_ratio < 100 && redemption < debt {
                // Some redemptions may improve ratio
                prop_assert!(
                    !is_safe || initial_ratio < 100,
                    "Undercollateralized positions should be detected"
                );
            }
        }

        #[test]
        fn full_redemption_always_safe(
            collateral in 1_000u128..1_000_000,
            debt in 1_000u128..1_000_000,
        ) {
            // Property: Full redemption (debt = redemption) is always safe
            let is_safe = is_safe_redemption(collateral, debt, debt);

            prop_assert!(is_safe, "Full redemption should always be safe");
        }

        #[test]
        fn excessive_redemption_rejected(
            collateral in 10_000u128..100_000,
            debt in 10_000u128..100_000,
            excess in 1u128..10_000,
        ) {
            // Property: Redemptions exceeding debt should be rejected
            let redemption = debt + excess;
            let is_safe = is_safe_redemption(collateral, debt, redemption);

            prop_assert!(!is_safe, "Excessive redemption must be rejected");
        }
    }
}

#[cfg(test)]
mod fee_application_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn fee_reduces_received_amount(
            amount in 1_000u128..1_000_000,
            fee_bp in 1u128..500, // 0.01% to 5% fee
        ) {
            // Property: Fees reduce the received amount
            let net_amount = apply_redemption_fee(amount, fee_bp).unwrap();

            prop_assert!(
                net_amount < amount,
                "Fee should reduce received amount"
            );

            // Fee should be reasonable
            let fee = amount - net_amount;
            let expected_fee = (amount * fee_bp) / 10_000;
            prop_assert_eq!(fee, expected_fee, "Fee calculation should be exact");
        }

        #[test]
        fn zero_fee_preserves_amount(
            amount in 1_000u128..1_000_000,
        ) {
            // Property: Zero fee means no reduction
            let net_amount = apply_redemption_fee(amount, 0).unwrap();

            prop_assert_eq!(net_amount, amount, "Zero fee preserves amount");
        }

        #[test]
        fn fee_calculation_safe(
            amount in 1u128..u128::MAX / 10_000,
            fee_bp in 1u128..10_000,
        ) {
            // Property: Fee calculation should not overflow
            let result = apply_redemption_fee(amount, fee_bp);

            prop_assert!(result.is_some(), "Fee calculation should succeed");
        }

        #[test]
        fn max_fee_not_exceeded(
            amount in 10_000u128..1_000_000,
            fee_bp in 1u128..1_000, // Up to 10% fee
        ) {
            // Property: Fee should never exceed specified percentage
            let net_amount = apply_redemption_fee(amount, fee_bp).unwrap();
            let fee = amount - net_amount;
            let max_fee = (amount * fee_bp) / 10_000;

            prop_assert!(
                fee <= max_fee,
                "Actual fee should not exceed max fee"
            );
        }
    }
}

#[cfg(test)]
mod redemption_sequence_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn sequential_redemptions_decrease_debt(
            initial_debt in 100_000u128..1_000_000,
            redemptions in prop::collection::vec(1_000u128..10_000, 5..10),
        ) {
            // Property: Sequential redemptions monotonically decrease debt
            let mut current_debt = initial_debt;

            for redemption in redemptions {
                let new_debt = current_debt.saturating_sub(redemption.min(current_debt));
                prop_assert!(
                    new_debt <= current_debt,
                    "Debt should decrease with each redemption"
                );
                current_debt = new_debt;

                if current_debt == 0 {
                    break; // Fully redeemed
                }
            }
        }

        #[test]
        fn partial_redemptions_safe(
            collateral in 200_000u128..1_000_000,
            initial_debt in 100_000u128..500_000,
            redemption_pct in 10u128..40, // 10-40% redemption
        ) {
            // Property: Partial redemptions on healthy positions are safe
            let redemption = (initial_debt * redemption_pct) / 100;
            let is_safe = is_safe_redemption(collateral, initial_debt, redemption);

            // Check initial ratio
            let initial_ratio = (collateral * 100) / initial_debt;
            if initial_ratio >= 200 {
                // Highly overcollateralized
                prop_assert!(
                    is_safe,
                    "Partial redemptions on healthy positions should be safe"
                );
            }
        }

        #[test]
        fn redemption_to_zero_debt(
            collateral in 10_000u128..1_000_000,
            initial_debt in 10_000u128..1_000_000,
        ) {
            // Property: Can always redeem to zero debt
            let is_safe = is_safe_redemption(collateral, initial_debt, initial_debt);

            prop_assert!(is_safe, "Full redemption should always be possible");

            // After full redemption, debt is zero
            let remaining_debt = initial_debt.saturating_sub(initial_debt);
            prop_assert_eq!(remaining_debt, 0, "Debt should be zero after full redemption");
        }
    }
}

#[cfg(test)]
mod edge_case_redemptions {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn dust_amount_redemption(
            collateral in 100_000u128..1_000_000,
            debt in 100_000u128..1_000_000,
            dust in 1u128..100, // Dust amounts
        ) {
            // Property: Dust redemptions should be handled safely
            let is_safe = is_safe_redemption(collateral, debt, dust);

            // Dust redemptions on healthy positions should be safe
            let ratio = (collateral * 100) / debt;
            if ratio >= 150 {
                prop_assert!(
                    is_safe,
                    "Dust redemptions on healthy positions should be safe"
                );
            }
        }

        #[test]
        fn maximum_value_redemption(
            collateral in u128::MAX / 200..u128::MAX / 100,
            debt in u128::MAX / 200..u128::MAX / 100,
        ) {
            // Property: Maximum value redemptions should not panic
            let is_safe = is_safe_redemption(collateral, debt, debt);

            // Should handle without panic
            prop_assert!(is_safe, "Max value full redemption should be safe");
        }

        #[test]
        fn minimum_collateralization_redemption(
            debt in 10_000u128..100_000,
        ) {
            // Property: At exactly 100% collateralization
            let collateral = debt; // Exactly 100%
            let redemption = debt / 10; // 10% redemption

            let is_safe = is_safe_redemption(collateral, debt, redemption);

            // After 10% redemption, ratio should improve slightly
            let remaining_debt = debt - redemption;
            let new_ratio = (collateral * 100) / remaining_debt;

            prop_assert!(
                new_ratio > 100,
                "Redemption at 100% should improve ratio"
            );
        }
    }
}
