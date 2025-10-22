# Financial Arithmetic Property Tests

## Overview

This document describes the comprehensive property-based tests for all critical arithmetic operations in the Ëtrid Protocol. These tests ensure that financial calculations maintain safety invariants and never panic, overflow, or produce invalid results.

**File**: `tests/property-based/tests/financial_arithmetic.rs`
**Total Tests**: 44 property tests
**Total Test Cases**: 44,000 (1,000 cases per test)
**Status**: ✅ All tests passing

## Test Coverage

### 1. Staking Arithmetic (6 tests, 6,000 cases)

Tests for staking operations including bonding, unbonding, and stake management.

#### Properties Tested:

- **`stake_unbond_never_negative`**: Verifies that `stake - min(stake, unbond) >= 0`
  - Ensures remaining stake after unbonding is never negative
  - Tests with stakes: 1 to 1,000,000
  - Tests with unbond requests: 1 to 1,000,000

- **`unbond_never_exceeds_bonded_stake`**: Unbond amount never exceeds bonded stake
  - Validates that actual unbond ≤ bonded stake
  - Handles cases where unbond request > available stake

- **`stake_increase_monotonic`**: Adding stake always increases total
  - Property: new_stake > initial_stake
  - Tests checked arithmetic (no overflow)

- **`stake_decrease_monotonic`**: Decreasing stake always decreases total
  - Property: new_stake < initial_stake (when decrease ≤ stake)
  - Validates safe subtraction

- **`full_unbond_zeroes_stake`**: Unbonding full stake results in zero
  - Property: unbond(stake, stake) = 0
  - Ensures complete unstaking is possible

- **`partial_unbond_leaves_remainder`**: Partial unbond leaves non-zero stake
  - Tests 1-99% unbonding scenarios
  - Validates: 0 < remaining < initial_stake

### 2. Reward Distribution (6 tests, 6,000 cases)

Tests for proportional reward distribution among stakers.

#### Properties Tested:

- **`single_staker_gets_all_rewards`**: Single staker receives 100% of rewards
  - Property: distribute_reward(stake, stake, rewards) = rewards
  - Validates correct calculation when only one participant

- **`sum_of_rewards_equals_total`**: Sum of individual rewards == total_rewards
  - Tests 2-10 stakers with various stakes
  - Allows for small rounding error due to integer division
  - Maximum rounding error: number of stakers

- **`no_staker_gets_negative_rewards`**: No staker gets negative rewards
  - Property: reward ≥ 0 for all stakers
  - Validates non-negative arithmetic

- **`larger_stake_gets_proportionally_more_rewards`**: Larger stake = more rewards
  - Tests stake multipliers: 2x to 10x
  - Verifies proportionality: reward_ratio ≈ stake_ratio (within rounding)

- **`zero_total_staked_safe`**: Zero total staked returns None safely
  - Handles edge case of zero total stake
  - No division by zero panic

- **`reward_distribution_no_overflow`**: Reward calculation never overflows
  - Tests large values up to u128::MAX / 1,000
  - Ensures checked arithmetic prevents overflow

### 3. Governance Vote Weight (6 tests, 6,000 cases)

Tests for vote weight calculations with time-based coinage factors.

#### Properties Tested:

- **`vote_weight_proportional_to_stake`**: vote_weight = stake × coinage_factor
  - Formula: `(stake × coinage_factor) / 100`
  - Tests coinage factors: 100% (1x) to 200% (2x)

- **`coinage_factor_decreases_over_time`**: Coinage factor decreases monotonically
  - Tests sequential time steps
  - Property: factor(t+1) ≤ factor(t)
  - Linear decay model: 200% → 100% over time

- **`fresh_stake_gets_max_weight`**: Fresh stake (time=0) gets maximum coinage
  - Property: coinage_factor(0, max_time) = 200%
  - Vote weight = 2 × stake

- **`old_stake_gets_min_weight`**: Old stake (time=max) gets minimum coinage
  - Property: coinage_factor(max_time, max_time) = 100%
  - Vote weight = 1 × stake

- **`vote_weight_never_negative`**: Vote weight is always non-negative
  - Tests various coinage factors: 50% to 300%
  - Validates non-negative results

- **`larger_stake_more_weight`**: Larger stake always gets more vote weight
  - Tests stake multipliers: 2x to 10x
  - Property: weight(large_stake) > weight(small_stake)

### 4. Transaction Fees (6 tests, 6,000 cases)

Tests for transaction fee calculations including base fees, per-byte fees, and congestion multipliers.

#### Properties Tested:

- **`fee_at_least_base_fee`**: fee ≥ base_fee
  - Formula: `base_fee + (tx_size × per_byte_fee)`
  - Tests base fees: 1 to 1,000

- **`fee_increases_with_tx_size`**: fee increases with tx_size
  - Property: fee(large_tx) > fee(small_tx)
  - Tests transaction sizes: 0 to 10,000 bytes

- **`fee_calculation_no_overflow`**: Fee calculation never overflows
  - Tests large transaction sizes up to 100,000 bytes
  - Checked arithmetic prevents overflow

- **`zero_size_tx_pays_base_fee`**: Zero-size tx pays only base fee
  - Property: fee(tx_size=0) = base_fee
  - Validates correct handling of minimal transactions

- **`congestion_multiplier_increases_fee`**: Congestion multiplier increases fee
  - Tests multipliers: 1.5x to 5x (150% to 500%)
  - Property: multiplied_fee > base_fee when multiplier > 100%

- **`fee_multiplier_proportional`**: Fee multiplier is proportional
  - Formula: `(fee × multiplier_pct) / 100`
  - Validates exact proportional scaling

### 5. Token Balance Invariants (7 tests, 7,000 cases)

Tests for token transfer invariants and balance safety.

#### Properties Tested:

- **`transfer_preserves_total_supply`**: sender + receiver balance = constant
  - Critical invariant: total supply never changes during transfers
  - Tests transfers: 1,000 to 99,000
  - Property: initial_total = final_total

- **`balance_never_negative`**: Balance never goes negative
  - Validates checked subtraction
  - Rejects transfers exceeding balance

- **`balance_never_overflows`**: Balance never overflows
  - Tests near u128::MAX values
  - Detects potential overflow before occurrence

- **`transfer_reduces_sender_balance`**: Transfer reduces sender balance
  - Property: new_sender = old_sender - amount
  - Validates correct deduction

- **`transfer_increases_receiver_balance`**: Transfer increases receiver balance
  - Property: new_receiver = old_receiver + amount
  - Validates correct addition

- **`zero_transfer_preserves_balances`**: Zero transfer doesn't change balances
  - Edge case: transfer(0) should be no-op
  - Both balances remain unchanged

- **`insufficient_balance_transfer_rejected`**: Transfer exceeding balance fails
  - Property: transfer(amount > balance) returns None
  - Validates proper rejection of invalid transfers

### 6. Multi-Asset Arithmetic (6 tests, 6,000 cases)

Tests for multi-asset portfolio calculations and weighted averages.

#### Properties Tested:

- **`total_value_sum_correct`**: Total value = sum of (amount × price)
  - Tests 2-10 different assets
  - Formula: `Σ(amount_i × price_i)`

- **`weighted_average_within_bounds`**: Weighted average price within min/max
  - Property: min(prices) ≤ avg_price ≤ max(prices)
  - Tests 2-5 assets with varying amounts

- **`adding_asset_increases_total_value`**: Adding asset increases total
  - Property: total_with_new_asset > initial_total
  - Validates monotonic increase

- **`single_asset_avg_equals_price`**: Single asset average = its price
  - Edge case for portfolio with one asset
  - Property: avg_price([amount], [price]) = price

- **`multi_asset_calculation_safe`**: Multi-asset calculation never panics
  - Tests up to 20 assets
  - Large value handling up to u128::MAX / 100,000

- **`equal_amounts_avg_is_simple_mean`**: Equal amounts → weighted avg = simple mean
  - When all amounts equal, weighted avg = arithmetic mean
  - Allows small rounding error (≤2)

### 7. Overflow Safety (4 tests, 4,000 cases)

Tests for overflow and edge case handling.

#### Properties Tested:

- **`max_values_no_panic`**: Operations near max values don't panic
  - Tests values: u128::MAX/2 to u128::MAX
  - Uses checked arithmetic (returns Some/None, never panics)

- **`division_by_zero_safe`**: Division by zero returns None
  - Property: x.checked_div(0) = None
  - No panic on division by zero

- **`subtraction_underflow_safe`**: Subtraction underflow returns None
  - Property: a.checked_sub(b) = None when b > a
  - Detects underflow safely

- **`multiplication_overflow_safe`**: Multiplication overflow returns None
  - Tests large values near u128::MAX
  - Property: overflow detected when a × b > u128::MAX

### 8. Rounding and Precision (3 tests, 3,000 cases)

Tests for integer arithmetic precision and rounding behavior.

#### Properties Tested:

- **`integer_division_truncates_consistently`**: Integer division always truncates
  - Property: (result × denominator) ≤ numerator
  - Remainder < denominator

- **`percentage_calculation_bounded`**: Percentage result ≤ original value
  - Formula: `(value × percentage) / 100`
  - Property: result ≤ value for percentage ≤ 100

- **`basis_points_precision`**: Basis points (0.01%) calculations precise
  - Formula: `(value × basis_points) / 10,000`
  - Property: 10,000bp = 100% = original value

## Test Configuration

- **Framework**: proptest 1.4.0
- **Cases per test**: 1,000
- **Total test cases**: 44,000
- **Execution time**: ~1.25 seconds (parallel), ~3 seconds (sequential)
- **Regression tracking**: Enabled (via .proptest-regressions files)

## Running the Tests

```bash
# Run all financial arithmetic tests
cd tests/property-based
cargo test --test financial_arithmetic

# Run with sequential execution
cargo test --test financial_arithmetic -- --test-threads=1

# Run specific test module
cargo test --test financial_arithmetic staking_arithmetic

# Run with output
cargo test --test financial_arithmetic -- --nocapture
```

## Test Statistics

```
Module                    | Tests | Coverage
--------------------------|-------|------------------------------------------
staking_arithmetic        |   6   | Stake, unbond, increase, decrease
reward_distribution       |   6   | Proportional rewards, overflow safety
governance_vote_weight    |   6   | Vote weight, coinage factor, time decay
transaction_fees          |   6   | Base fee, size fee, congestion multiplier
balance_invariants        |   7   | Transfer safety, overflow, underflow
multi_asset_arithmetic    |   6   | Portfolio value, weighted averages
overflow_safety           |   4   | Max values, division by zero, overflow
precision_tests           |   3   | Integer division, percentages, basis points
--------------------------|-------|------------------------------------------
TOTAL                     |  44   | All critical arithmetic operations
```

## Coverage of Audit Requirements

This test suite addresses the following audit findings:

1. **Property-based testing for financial logic** ✅
   - 44 property tests with 1,000 cases each
   - Comprehensive coverage of all arithmetic operations

2. **Overflow/underflow protection** ✅
   - All operations use checked arithmetic
   - Explicit overflow safety tests
   - Tests near u128::MAX boundaries

3. **Division by zero handling** ✅
   - Explicit tests for zero denominators
   - Safe Option<T> return types

4. **Rounding error documentation** ✅
   - Integer division truncation tests
   - Percentage and basis point precision tests
   - Documented acceptable rounding tolerances

5. **Invariant preservation** ✅
   - Total supply preservation in transfers
   - Balance non-negativity
   - Reward distribution sum preservation

## Edge Cases Tested

1. **Boundary values**:
   - Zero amounts
   - Maximum u128 values
   - Near-overflow scenarios

2. **Special conditions**:
   - Zero total staked (reward distribution)
   - Division by zero (all calculations)
   - Single participant scenarios
   - Empty collections

3. **Rounding scenarios**:
   - Integer division truncation
   - Multiple stakers with dust amounts
   - Basis point precision (0.01%)

## Failure Examples

Proptest generates minimal failing examples when properties are violated. Examples are stored in:
- `tests/financial_arithmetic.proptest-regressions` (if failures occur)

To reproduce a failure:
```bash
# Proptest automatically replays regression cases
cargo test --test financial_arithmetic
```

## Future Enhancements

Potential additions for even more comprehensive coverage:

1. **Cross-chain arithmetic**:
   - Bridge fee calculations
   - Currency conversion precision
   - Cross-chain balance consistency

2. **Economic security**:
   - Slashing calculations
   - Penalty distribution
   - Security deposit requirements

3. **Liquidation scenarios**:
   - Partial liquidations
   - Multi-collateral liquidations
   - Liquidation bonus calculations

4. **Interest calculations**:
   - Compound interest
   - Time-weighted returns
   - Annualized percentages

## Maintenance

- Run tests before each commit
- Update test cases when adding new arithmetic operations
- Document any acceptable rounding tolerances
- Keep regression files in version control
- Review failures in CI/CD pipeline

## Integration with CI/CD

```yaml
# Example GitHub Actions workflow
- name: Run Property Tests
  run: |
    cd tests/property-based
    cargo test --test financial_arithmetic -- --test-threads=1
    cargo test --all-features
```

## References

- [proptest documentation](https://docs.rs/proptest/)
- Ëtrid Protocol specification
- Substrate runtime arithmetic best practices
- IEEE 754 floating-point standard (for precision requirements)

---

**Last Updated**: October 22, 2025
**Test Suite Version**: 1.0.0
**Status**: All tests passing ✅
