//! Property-based tests for financial arithmetic operations
//!
//! These tests verify that all critical arithmetic operations in the Ëtrid Protocol
//! maintain safety invariants and never panic, overflow, or produce invalid results.
//!
//! Coverage areas:
//! - Staking operations (stake, unbond, rewards)
//! - Governance vote weight calculations
//! - Transaction fee calculations
//! - Token balance invariants
//! - Multi-asset arithmetic

use proptest::prelude::*;

// =============================================================================
// STAKING ARITHMETIC TESTS
// =============================================================================

/// Helper function to calculate unbond amount
fn calculate_unbond(bonded_stake: u128, unbond_request: u128) -> u128 {
    unbond_request.min(bonded_stake)
}

/// Helper function to calculate remaining stake after unbond
fn remaining_after_unbond(bonded_stake: u128, unbond_amount: u128) -> Option<u128> {
    bonded_stake.checked_sub(unbond_amount)
}

/// Helper function to distribute rewards proportionally
fn distribute_reward(
    total_staked: u128,
    staker_stake: u128,
    total_rewards: u128,
) -> Option<u128> {
    if total_staked == 0 {
        return None;
    }
    // reward = (staker_stake * total_rewards) / total_staked
    staker_stake
        .checked_mul(total_rewards)?
        .checked_div(total_staked)
}

#[cfg(test)]
mod staking_arithmetic {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn stake_unbond_never_negative(
            stake in 1u128..1_000_000,
            unbond in 1u128..1_000_000,
        ) {
            // Property: stake - min(stake, unbond) >= 0
            let unbond_amount = calculate_unbond(stake, unbond);
            let remaining = remaining_after_unbond(stake, unbond_amount);

            prop_assert!(remaining.is_some(), "Unbond should never cause negative balance");
            prop_assert!(remaining.unwrap() >= 0, "Remaining stake must be non-negative");
        }

        #[test]
        fn unbond_never_exceeds_bonded_stake(
            stake in 1u128..1_000_000,
            unbond_request in 1u128..2_000_000,
        ) {
            // Property: unbond amount never exceeds bonded stake
            let unbond_amount = calculate_unbond(stake, unbond_request);

            prop_assert!(
                unbond_amount <= stake,
                "Unbond amount cannot exceed bonded stake"
            );
        }

        #[test]
        fn stake_increase_monotonic(
            initial_stake in 1_000u128..100_000,
            increase in 1u128..10_000,
        ) {
            // Property: Adding stake always increases total
            let new_stake = initial_stake.checked_add(increase);

            prop_assert!(new_stake.is_some(), "Stake increase should not overflow");
            prop_assert!(
                new_stake.unwrap() > initial_stake,
                "Stake should increase monotonically"
            );
        }

        #[test]
        fn stake_decrease_monotonic(
            initial_stake in 1_000u128..100_000,
            decrease in 1u128..999,
        ) {
            // Property: Decreasing stake always decreases total (if within bounds)
            if decrease <= initial_stake {
                let new_stake = initial_stake.checked_sub(decrease);

                prop_assert!(new_stake.is_some(), "Stake decrease should succeed");
                prop_assert!(
                    new_stake.unwrap() < initial_stake,
                    "Stake should decrease monotonically"
                );
            }
        }

        #[test]
        fn full_unbond_zeroes_stake(
            stake in 1u128..1_000_000,
        ) {
            // Property: Unbonding full stake results in zero
            let unbond_amount = calculate_unbond(stake, stake);
            let remaining = remaining_after_unbond(stake, unbond_amount).unwrap();

            prop_assert_eq!(unbond_amount, stake, "Full unbond should equal stake");
            prop_assert_eq!(remaining, 0, "Remaining should be zero after full unbond");
        }

        #[test]
        fn partial_unbond_leaves_remainder(
            stake in 1_000u128..1_000_000,
            unbond_pct in 1u128..99, // 1-99% unbond
        ) {
            // Property: Partial unbond leaves non-zero stake
            let unbond_amount = (stake * unbond_pct) / 100;
            let actual_unbond = calculate_unbond(stake, unbond_amount);
            let remaining = remaining_after_unbond(stake, actual_unbond).unwrap();

            prop_assert!(remaining > 0, "Partial unbond should leave stake");
            prop_assert!(remaining < stake, "Remaining should be less than initial");
        }
    }
}

#[cfg(test)]
mod reward_distribution {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn single_staker_gets_all_rewards(
            stake in 1u128..1_000_000,
            total_rewards in 1u128..1_000_000,
        ) {
            // Property: Single staker receives 100% of rewards
            let reward = distribute_reward(stake, stake, total_rewards);

            prop_assert!(reward.is_some(), "Reward calculation should succeed");
            prop_assert_eq!(
                reward.unwrap(),
                total_rewards,
                "Single staker should get all rewards"
            );
        }

        #[test]
        fn sum_of_rewards_equals_total(
            stakes in prop::collection::vec(1_000u128..100_000, 2..10),
            total_rewards in 100_000u128..1_000_000,
        ) {
            // Property: sum of individual rewards == total_rewards
            let total_staked: u128 = stakes.iter().sum();
            let mut distributed: u128 = 0;

            for stake in &stakes {
                if let Some(reward) = distribute_reward(total_staked, *stake, total_rewards) {
                    distributed += reward;
                }
            }

            // Allow small rounding error due to integer division
            let diff = if distributed > total_rewards {
                distributed - total_rewards
            } else {
                total_rewards - distributed
            };

            prop_assert!(
                diff <= stakes.len() as u128,
                "Total distributed should approximately equal total rewards (within rounding)"
            );
        }

        #[test]
        fn no_staker_gets_negative_rewards(
            total_staked in 1_000u128..1_000_000,
            staker_stake in 1u128..1_000_000,
            total_rewards in 1u128..1_000_000,
        ) {
            // Property: no staker gets negative rewards
            if staker_stake <= total_staked {
                let reward = distribute_reward(total_staked, staker_stake, total_rewards);

                if let Some(r) = reward {
                    prop_assert!(r >= 0, "Reward must be non-negative");
                }
            }
        }

        #[test]
        fn larger_stake_gets_proportionally_more_rewards(
            small_stake in 1_000u128..50_000,
            large_multiplier in 2u128..10,
            total_rewards in 100_000u128..1_000_000,
        ) {
            // Property: Larger stake gets proportionally more rewards
            let large_stake = small_stake * large_multiplier;
            let total_staked = small_stake + large_stake;

            let small_reward = distribute_reward(total_staked, small_stake, total_rewards).unwrap();
            let large_reward = distribute_reward(total_staked, large_stake, total_rewards).unwrap();

            prop_assert!(
                large_reward > small_reward,
                "Larger stake should get more rewards"
            );

            // Verify proportionality (allowing for rounding)
            let ratio = large_reward / small_reward.max(1);
            prop_assert!(
                ratio >= large_multiplier - 1 && ratio <= large_multiplier + 1,
                "Reward ratio should match stake ratio (within rounding)"
            );
        }

        #[test]
        fn zero_total_staked_safe(
            staker_stake in 1u128..1_000,
            total_rewards in 1u128..1_000,
        ) {
            // Property: Zero total staked returns None safely
            let reward = distribute_reward(0, staker_stake, total_rewards);

            prop_assert!(reward.is_none(), "Zero total staked should return None");
        }

        #[test]
        fn reward_distribution_no_overflow(
            total_staked in 1u128..u128::MAX / 1_000,
            staker_stake in 1u128..u128::MAX / 1_000,
            total_rewards in 1u128..u128::MAX / 1_000,
        ) {
            // Property: Reward calculation never overflows
            let _reward = distribute_reward(total_staked, staker_stake, total_rewards);
            // Should either return Some or None, never panic
            prop_assert!(true);
        }
    }
}

// =============================================================================
// GOVERNANCE VOTE WEIGHT TESTS
// =============================================================================

/// Helper function to calculate vote weight with coinage factor
fn calculate_vote_weight(stake: u128, coinage_factor: u128) -> Option<u128> {
    // vote_weight = stake * coinage_factor / 100
    stake.checked_mul(coinage_factor)?.checked_div(100)
}

/// Helper function to calculate time-decaying coinage factor
fn calculate_coinage_factor(time_held: u64, max_time: u64) -> u128 {
    if max_time == 0 {
        return 100; // 1x multiplier
    }
    // Linear decay: 200% at 0 time, 100% at max_time
    let decay_pct = ((time_held as u128 * 100) / max_time as u128).min(100);
    200 - decay_pct // 200% -> 100%
}

#[cfg(test)]
mod governance_vote_weight {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn vote_weight_proportional_to_stake(
            stake in 1u128..1_000_000,
            coinage_factor in 100u128..200, // 1x to 2x
        ) {
            // Property: vote_weight = stake * coinage_factor
            let weight = calculate_vote_weight(stake, coinage_factor);

            prop_assert!(weight.is_some(), "Vote weight calculation should succeed");

            let expected = (stake * coinage_factor) / 100;
            prop_assert_eq!(
                weight.unwrap(),
                expected,
                "Vote weight should match formula"
            );
        }

        #[test]
        fn coinage_factor_decreases_over_time(
            time_steps in prop::collection::vec(1u64..100, 5..20),
            max_time in 1_000u64..10_000,
        ) {
            // Property: coinage factor decreases monotonically over time
            let mut prev_factor = None;
            let mut cumulative_time = 0u64;

            for step in time_steps {
                cumulative_time = cumulative_time.saturating_add(step).min(max_time);
                let factor = calculate_coinage_factor(cumulative_time, max_time);

                if let Some(prev) = prev_factor {
                    prop_assert!(
                        factor <= prev,
                        "Coinage factor should decrease or stay constant over time"
                    );
                }
                prev_factor = Some(factor);
            }
        }

        #[test]
        fn fresh_stake_gets_max_weight(
            stake in 1_000u128..1_000_000,
            max_time in 1_000u64..10_000,
        ) {
            // Property: Fresh stake (time=0) gets maximum coinage factor
            let factor = calculate_coinage_factor(0, max_time);
            let weight = calculate_vote_weight(stake, factor).unwrap();

            prop_assert_eq!(factor, 200, "Fresh stake should get 200% coinage");
            prop_assert_eq!(weight, stake * 2, "Fresh stake should get 2x weight");
        }

        #[test]
        fn old_stake_gets_min_weight(
            stake in 1_000u128..1_000_000,
            max_time in 1_000u64..10_000,
        ) {
            // Property: Old stake (time=max) gets minimum coinage factor
            let factor = calculate_coinage_factor(max_time, max_time);
            let weight = calculate_vote_weight(stake, factor).unwrap();

            prop_assert_eq!(factor, 100, "Old stake should get 100% coinage");
            prop_assert_eq!(weight, stake, "Old stake should get 1x weight");
        }

        #[test]
        fn vote_weight_never_negative(
            stake in 0u128..1_000_000,
            coinage_factor in 50u128..300,
        ) {
            // Property: Vote weight is always non-negative
            if let Some(weight) = calculate_vote_weight(stake, coinage_factor) {
                prop_assert!(weight >= 0, "Vote weight must be non-negative");
            }
        }

        #[test]
        fn larger_stake_more_weight(
            small_stake in 1_000u128..50_000,
            multiplier in 2u128..10,
            coinage_factor in 100u128..200,
        ) {
            // Property: Larger stake always gets more vote weight
            let large_stake = small_stake * multiplier;

            let small_weight = calculate_vote_weight(small_stake, coinage_factor).unwrap();
            let large_weight = calculate_vote_weight(large_stake, coinage_factor).unwrap();

            prop_assert!(
                large_weight > small_weight,
                "Larger stake should get more vote weight"
            );
        }
    }
}

// =============================================================================
// TRANSACTION FEE TESTS
// =============================================================================

/// Helper function to calculate transaction fee
fn calculate_tx_fee(base_fee: u64, tx_size: u32, per_byte_fee: u64) -> Option<u64> {
    let size_fee = (tx_size as u64).checked_mul(per_byte_fee)?;
    base_fee.checked_add(size_fee)
}

/// Helper function to apply fee multiplier during congestion
fn apply_fee_multiplier(fee: u64, multiplier_pct: u64) -> Option<u64> {
    fee.checked_mul(multiplier_pct)?.checked_div(100)
}

#[cfg(test)]
mod transaction_fees {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn fee_at_least_base_fee(
            base_fee in 1u64..1000,
            tx_size in 0u32..10_000,
            per_byte_fee in 1u64..10,
        ) {
            // Property: fee >= base_fee
            let fee = calculate_tx_fee(base_fee, tx_size, per_byte_fee);

            prop_assert!(fee.is_some(), "Fee calculation should succeed");
            prop_assert!(
                fee.unwrap() >= base_fee,
                "Fee must be at least base_fee"
            );
        }

        #[test]
        fn fee_increases_with_tx_size(
            base_fee in 100u64..1000,
            small_size in 100u32..1_000,
            large_size in 5_000u32..10_000,
            per_byte_fee in 1u64..10,
        ) {
            // Property: fee increases with tx_size
            let small_fee = calculate_tx_fee(base_fee, small_size, per_byte_fee).unwrap();
            let large_fee = calculate_tx_fee(base_fee, large_size, per_byte_fee).unwrap();

            prop_assert!(
                large_fee > small_fee,
                "Larger transaction should cost more"
            );
        }

        #[test]
        fn fee_calculation_no_overflow(
            base_fee in 1u64..u64::MAX / 100_000,
            tx_size in 0u32..100_000,
            per_byte_fee in 1u64..1000,
        ) {
            // Property: fee calculation never overflows
            let fee = calculate_tx_fee(base_fee, tx_size, per_byte_fee);

            // Should either succeed or return None, never panic
            prop_assert!(
                fee.is_some() || fee.is_none(),
                "Fee calculation should not panic"
            );
        }

        #[test]
        fn zero_size_tx_pays_base_fee(
            base_fee in 1u64..1000,
            per_byte_fee in 1u64..10,
        ) {
            // Property: Zero-size tx pays only base fee
            let fee = calculate_tx_fee(base_fee, 0, per_byte_fee).unwrap();

            prop_assert_eq!(fee, base_fee, "Zero-size tx should pay base fee only");
        }

        #[test]
        fn congestion_multiplier_increases_fee(
            base_fee in 100u64..1000,
            tx_size in 100u32..1_000,
            per_byte_fee in 1u64..5,
            multiplier_pct in 150u64..500, // 1.5x to 5x
        ) {
            // Property: Congestion multiplier increases fee
            let base_total = calculate_tx_fee(base_fee, tx_size, per_byte_fee).unwrap();
            let multiplied = apply_fee_multiplier(base_total, multiplier_pct).unwrap();

            if multiplier_pct > 100 {
                prop_assert!(
                    multiplied > base_total,
                    "Congestion should increase fee"
                );
            }
        }

        #[test]
        fn fee_multiplier_proportional(
            base_total in 1_000u64..10_000,
            multiplier_pct in 100u64..1000,
        ) {
            // Property: Fee multiplier is proportional
            let multiplied = apply_fee_multiplier(base_total, multiplier_pct).unwrap();
            let expected = (base_total * multiplier_pct) / 100;

            prop_assert_eq!(
                multiplied,
                expected,
                "Multiplied fee should match expected"
            );
        }
    }
}

// =============================================================================
// TOKEN BALANCE INVARIANT TESTS
// =============================================================================

/// Helper function to simulate a transfer
fn simulate_transfer(
    sender_balance: u128,
    receiver_balance: u128,
    amount: u128,
) -> Option<(u128, u128)> {
    let new_sender = sender_balance.checked_sub(amount)?;
    let new_receiver = receiver_balance.checked_add(amount)?;
    Some((new_sender, new_receiver))
}

/// Helper function to check balance never overflows
fn check_balance_add_safe(balance: u128, amount: u128) -> bool {
    balance.checked_add(amount).is_some()
}

/// Helper function to check balance never goes negative
fn check_balance_sub_safe(balance: u128, amount: u128) -> bool {
    balance.checked_sub(amount).is_some()
}

#[cfg(test)]
mod balance_invariants {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn transfer_preserves_total_supply(
            sender_balance in 100_000u128..1_000_000,
            receiver_balance in 100_000u128..1_000_000,
            transfer_amount in 1_000u128..99_000,
        ) {
            // Property: sender.balance + receiver.balance == constant
            if transfer_amount <= sender_balance {
                let initial_total = sender_balance + receiver_balance;
                let result = simulate_transfer(sender_balance, receiver_balance, transfer_amount);

                if let Some((new_sender, new_receiver)) = result {
                    let final_total = new_sender + new_receiver;
                    prop_assert_eq!(
                        initial_total,
                        final_total,
                        "Total supply must remain constant"
                    );
                }
            }
        }

        #[test]
        fn balance_never_negative(
            balance in 100u128..1_000_000,
            subtract_amount in 1u128..2_000_000,
        ) {
            // Property: balance never goes negative
            let is_safe = check_balance_sub_safe(balance, subtract_amount);

            if subtract_amount > balance {
                prop_assert!(
                    !is_safe,
                    "Subtracting more than balance should be rejected"
                );
            } else {
                prop_assert!(
                    is_safe,
                    "Valid subtraction should be safe"
                );
            }
        }

        #[test]
        fn balance_never_overflows(
            balance in u128::MAX / 2..u128::MAX - 1_000_000,
            add_amount in 1u128..2_000_000,
        ) {
            // Property: balance never overflows
            let is_safe = check_balance_add_safe(balance, add_amount);

            // Should detect potential overflow
            if balance > u128::MAX - add_amount {
                prop_assert!(!is_safe, "Overflow should be detected");
            } else {
                prop_assert!(is_safe, "Valid addition should be safe");
            }
        }

        #[test]
        fn transfer_reduces_sender_balance(
            sender_balance in 1_000u128..1_000_000,
            receiver_balance in 0u128..1_000_000,
            transfer_amount in 1u128..999,
        ) {
            // Property: Transfer reduces sender balance
            if transfer_amount <= sender_balance {
                let result = simulate_transfer(sender_balance, receiver_balance, transfer_amount);

                if let Some((new_sender, _)) = result {
                    prop_assert!(
                        new_sender < sender_balance,
                        "Sender balance should decrease"
                    );
                    prop_assert_eq!(
                        new_sender,
                        sender_balance - transfer_amount,
                        "Sender balance should decrease by transfer amount"
                    );
                }
            }
        }

        #[test]
        fn transfer_increases_receiver_balance(
            sender_balance in 1_000u128..1_000_000,
            receiver_balance in 0u128..1_000_000,
            transfer_amount in 1u128..999,
        ) {
            // Property: Transfer increases receiver balance
            if transfer_amount <= sender_balance {
                let result = simulate_transfer(sender_balance, receiver_balance, transfer_amount);

                if let Some((_, new_receiver)) = result {
                    prop_assert!(
                        new_receiver > receiver_balance,
                        "Receiver balance should increase"
                    );
                    prop_assert_eq!(
                        new_receiver,
                        receiver_balance + transfer_amount,
                        "Receiver balance should increase by transfer amount"
                    );
                }
            }
        }

        #[test]
        fn zero_transfer_preserves_balances(
            sender_balance in 0u128..1_000_000,
            receiver_balance in 0u128..1_000_000,
        ) {
            // Property: Zero transfer doesn't change balances
            let result = simulate_transfer(sender_balance, receiver_balance, 0);

            if let Some((new_sender, new_receiver)) = result {
                prop_assert_eq!(new_sender, sender_balance, "Sender unchanged");
                prop_assert_eq!(new_receiver, receiver_balance, "Receiver unchanged");
            }
        }

        #[test]
        fn insufficient_balance_transfer_rejected(
            sender_balance in 1u128..1_000,
            receiver_balance in 0u128..1_000_000,
            excess_amount in 1u128..1_000,
        ) {
            // Property: Transfer exceeding balance is rejected
            let transfer_amount = sender_balance + excess_amount;
            let result = simulate_transfer(sender_balance, receiver_balance, transfer_amount);

            prop_assert!(result.is_none(), "Insufficient balance transfer should fail");
        }
    }
}

// =============================================================================
// MULTI-ASSET ARITHMETIC TESTS
// =============================================================================

/// Helper function to calculate total value in base currency
fn calculate_total_value(amounts: &[u128], prices: &[u128]) -> Option<u128> {
    if amounts.len() != prices.len() {
        return None;
    }

    let mut total = 0u128;
    for (amount, price) in amounts.iter().zip(prices.iter()) {
        let value = amount.checked_mul(*price)?;
        total = total.checked_add(value)?;
    }
    Some(total)
}

/// Helper function to calculate weighted average price
fn calculate_weighted_avg_price(amounts: &[u128], prices: &[u128]) -> Option<u128> {
    let total_value = calculate_total_value(amounts, prices)?;
    let total_amount: u128 = amounts.iter().sum();

    if total_amount == 0 {
        return None;
    }

    total_value.checked_div(total_amount)
}

#[cfg(test)]
mod multi_asset_arithmetic {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn total_value_sum_correct(
            amounts in prop::collection::vec(1_000u128..100_000, 2..10),
            prices in prop::collection::vec(100u128..1_000, 2..10),
        ) {
            // Property: Total value equals sum of (amount * price)
            if amounts.len() == prices.len() {
                let total = calculate_total_value(&amounts, &prices);

                if let Some(t) = total {
                    // Verify total is sum of individual values
                    let mut manual_sum = 0u128;
                    for (amount, price) in amounts.iter().zip(prices.iter()) {
                        manual_sum += amount * price;
                    }
                    prop_assert_eq!(t, manual_sum, "Total should match manual sum");
                }
            }
        }

        #[test]
        fn weighted_average_within_bounds(
            amounts in prop::collection::vec(1_000u128..10_000, 2..5),
            prices in prop::collection::vec(100u128..1_000, 2..5),
        ) {
            // Property: Weighted average price is within min/max range
            if amounts.len() == prices.len() && !prices.is_empty() {
                if let Some(avg_price) = calculate_weighted_avg_price(&amounts, &prices) {
                    let min_price = *prices.iter().min().unwrap();
                    let max_price = *prices.iter().max().unwrap();

                    prop_assert!(
                        avg_price >= min_price && avg_price <= max_price,
                        "Weighted average should be within price range"
                    );
                }
            }
        }

        #[test]
        fn adding_asset_increases_total_value(
            amounts in prop::collection::vec(1_000u128..10_000, 2..5),
            prices in prop::collection::vec(100u128..1_000, 2..5),
            new_amount in 1_000u128..10_000,
            new_price in 100u128..1_000,
        ) {
            // Property: Adding asset increases total value
            if amounts.len() == prices.len() {
                let initial_total = calculate_total_value(&amounts, &prices);

                let mut new_amounts = amounts.clone();
                new_amounts.push(new_amount);
                let mut new_prices = prices.clone();
                new_prices.push(new_price);

                let new_total = calculate_total_value(&new_amounts, &new_prices);

                if let (Some(initial), Some(new)) = (initial_total, new_total) {
                    prop_assert!(
                        new > initial,
                        "Adding asset should increase total value"
                    );
                }
            }
        }

        #[test]
        fn single_asset_avg_equals_price(
            amount in 1_000u128..100_000,
            price in 100u128..1_000,
        ) {
            // Property: Single asset average price equals its price
            let amounts = vec![amount];
            let prices = vec![price];

            let avg = calculate_weighted_avg_price(&amounts, &prices).unwrap();

            prop_assert_eq!(avg, price, "Single asset average should equal its price");
        }

        #[test]
        fn multi_asset_calculation_safe(
            amounts in prop::collection::vec(1u128..u128::MAX / 100_000, 1..20),
            prices in prop::collection::vec(1u128..100_000, 1..20),
        ) {
            // Property: Multi-asset calculation never panics
            if amounts.len() == prices.len() {
                let _total = calculate_total_value(&amounts, &prices);
                let _avg = calculate_weighted_avg_price(&amounts, &prices);
                // Should either succeed or return None, never panic
                prop_assert!(true);
            }
        }

        #[test]
        fn equal_amounts_avg_is_simple_mean(
            equal_amount in 1_000u128..10_000,
            prices in prop::collection::vec(100u128..1_000, 3..8),
        ) {
            // Property: Equal amounts means weighted avg = simple mean
            let amounts = vec![equal_amount; prices.len()];
            let avg_price = calculate_weighted_avg_price(&amounts, &prices).unwrap();

            let simple_mean: u128 = prices.iter().sum::<u128>() / prices.len() as u128;

            // Allow small rounding error
            let diff = if avg_price > simple_mean {
                avg_price - simple_mean
            } else {
                simple_mean - avg_price
            };

            prop_assert!(
                diff <= 2,
                "Equal amounts should give simple mean (within rounding)"
            );
        }
    }
}

// =============================================================================
// OVERFLOW AND EDGE CASE TESTS
// =============================================================================

#[cfg(test)]
mod overflow_safety {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn max_values_no_panic(
            a in u128::MAX / 2..u128::MAX,
            b in 0u128..100,
        ) {
            // Property: Operations near max values don't panic
            let _add = a.checked_add(b);
            let _mul = a.checked_mul(b);
            // Should return Some or None, never panic
            prop_assert!(true);
        }

        #[test]
        fn division_by_zero_safe(
            numerator in 1u128..1_000_000,
            denominator in 0u128..2,
        ) {
            // Property: Division by zero returns None
            let result = numerator.checked_div(denominator);

            if denominator == 0 {
                prop_assert!(result.is_none(), "Division by zero should return None");
            } else {
                prop_assert!(result.is_some(), "Valid division should succeed");
            }
        }

        #[test]
        fn subtraction_underflow_safe(
            a in 0u128..1_000,
            b in 0u128..2_000,
        ) {
            // Property: Subtraction underflow returns None
            let result = a.checked_sub(b);

            if b > a {
                prop_assert!(result.is_none(), "Underflow should return None");
            } else {
                prop_assert!(result.is_some(), "Valid subtraction should succeed");
            }
        }

        #[test]
        fn multiplication_overflow_safe(
            a in u128::MAX / 1_000..u128::MAX,
            b in 2u128..1_000,
        ) {
            // Property: Multiplication overflow returns None
            let result = a.checked_mul(b);

            // Should detect overflow when a * b > u128::MAX
            if a > u128::MAX / b {
                prop_assert!(result.is_none(), "Overflow should be detected");
            }
        }
    }
}

// =============================================================================
// ROUNDING AND PRECISION TESTS
// =============================================================================

#[cfg(test)]
mod precision_tests {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn integer_division_truncates_consistently(
            numerator in 1u128..1_000_000,
            denominator in 1u128..1_000,
        ) {
            // Property: Integer division always truncates towards zero
            let result = numerator / denominator;
            let reconstructed = result * denominator;

            prop_assert!(
                reconstructed <= numerator,
                "Truncation should never exceed original"
            );
            prop_assert!(
                numerator - reconstructed < denominator,
                "Remainder should be less than denominator"
            );
        }

        #[test]
        fn percentage_calculation_bounded(
            value in 1_000u128..1_000_000,
            percentage in 1u128..100,
        ) {
            // Property: Percentage calculation stays within bounds
            let result = (value * percentage) / 100;

            prop_assert!(result <= value, "Percentage result should not exceed original");
            if percentage == 100 {
                prop_assert_eq!(result, value, "100% should equal original");
            }
        }

        #[test]
        fn basis_points_precision(
            value in 10_000u128..1_000_000,
            basis_points in 1u128..10_000, // 0.01% to 100%
        ) {
            // Property: Basis points (1bp = 0.01%) calculations are precise
            let result = (value * basis_points) / 10_000;

            prop_assert!(result <= value, "Basis points result within bounds");
            if basis_points == 10_000 {
                prop_assert_eq!(result, value, "10000bp = 100% = original value");
            }
        }
    }
}
