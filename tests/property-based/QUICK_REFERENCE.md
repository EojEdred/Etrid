# Property Tests Quick Reference

## Quick Stats

| Metric | Value |
|--------|-------|
| Total Tests | 141 |
| Total Cases | 141,000 |
| New Tests | 44 (financial arithmetic) |
| Status | ✅ All passing |
| Execution Time | ~3.8s (sequential) |

## Run Commands

```bash
cd tests/property-based

# Run all tests
cargo test

# Run new financial arithmetic tests only
cargo test --test financial_arithmetic

# Run specific module
cargo test --test financial_arithmetic staking_arithmetic

# Sequential execution (deterministic)
cargo test -- --test-threads=1

# With verbose output
cargo test -- --nocapture
```

## New Test Modules

### Staking Arithmetic (6 tests)
```rust
✅ stake_unbond_never_negative
✅ unbond_never_exceeds_bonded_stake
✅ stake_increase_monotonic
✅ stake_decrease_monotonic
✅ full_unbond_zeroes_stake
✅ partial_unbond_leaves_remainder
```

### Reward Distribution (6 tests)
```rust
✅ single_staker_gets_all_rewards
✅ sum_of_rewards_equals_total
✅ no_staker_gets_negative_rewards
✅ larger_stake_gets_proportionally_more_rewards
✅ zero_total_staked_safe
✅ reward_distribution_no_overflow
```

### Governance Vote Weight (6 tests)
```rust
✅ vote_weight_proportional_to_stake
✅ coinage_factor_decreases_over_time
✅ fresh_stake_gets_max_weight
✅ old_stake_gets_min_weight
✅ vote_weight_never_negative
✅ larger_stake_more_weight
```

### Transaction Fees (6 tests)
```rust
✅ fee_at_least_base_fee
✅ fee_increases_with_tx_size
✅ fee_calculation_no_overflow
✅ zero_size_tx_pays_base_fee
✅ congestion_multiplier_increases_fee
✅ fee_multiplier_proportional
```

### Balance Invariants (7 tests)
```rust
✅ transfer_preserves_total_supply
✅ balance_never_negative
✅ balance_never_overflows
✅ transfer_reduces_sender_balance
✅ transfer_increases_receiver_balance
✅ zero_transfer_preserves_balances
✅ insufficient_balance_transfer_rejected
```

### Multi-Asset Arithmetic (6 tests)
```rust
✅ total_value_sum_correct
✅ weighted_average_within_bounds
✅ adding_asset_increases_total_value
✅ single_asset_avg_equals_price
✅ multi_asset_calculation_safe
✅ equal_amounts_avg_is_simple_mean
```

### Overflow Safety (4 tests)
```rust
✅ max_values_no_panic
✅ division_by_zero_safe
✅ subtraction_underflow_safe
✅ multiplication_overflow_safe
```

### Precision Tests (3 tests)
```rust
✅ integer_division_truncates_consistently
✅ percentage_calculation_bounded
✅ basis_points_precision
```

## Files Created

1. `/tests/property-based/tests/financial_arithmetic.rs` (44 tests, ~900 lines)
2. `/tests/property-based/FINANCIAL_ARITHMETIC_TESTS.md` (detailed documentation)
3. `/tests/property-based/PROPERTY_TEST_SUMMARY.md` (executive summary)
4. `/tests/property-based/QUICK_REFERENCE.md` (this file)

## Key Properties Verified

- ✅ No panics on any arithmetic operation
- ✅ No overflows (all checked arithmetic)
- ✅ No underflows (safe subtraction)
- ✅ No division by zero
- ✅ Total supply preservation
- ✅ Balance consistency
- ✅ Proportional reward distribution
- ✅ Fee calculation correctness
- ✅ Vote weight accuracy
- ✅ Multi-asset portfolio calculations

## Audit Compliance

✅ Property-based testing for financial logic
✅ Overflow/underflow protection
✅ Division by zero handling
✅ Rounding error documentation
✅ Invariant preservation

## Expected Output

```
running 44 tests
test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Test Distribution

```
Financial Arithmetic:   44 tests (31%) ⭐ NEW
Reserve Ratio Tests:    30 tests (21%)
Redemption Tests:       18 tests (13%)
Oracle Tests:           16 tests (11%)
Balance Tests:          13 tests (9%)
Bridge Tests:           10 tests (7%)
EDSC Token Tests:       8 tests  (6%)
Other:                  2 tests  (2%)
───────────────────────────────────
Total:                  141 tests
```

## Documentation

- **Detailed Test Docs**: `FINANCIAL_ARITHMETIC_TESTS.md`
- **Executive Summary**: `PROPERTY_TEST_SUMMARY.md`
- **Quick Reference**: `QUICK_REFERENCE.md` (this file)

## Last Updated

October 22, 2025 - All 141 tests passing ✅
