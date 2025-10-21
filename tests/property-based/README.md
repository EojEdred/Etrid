# Property-Based Testing for Ëtrid Protocol

## Overview

This directory contains property-based tests that verify invariants and properties that should hold for **any** valid inputs to the Ëtrid Protocol. Unlike traditional unit tests that check specific examples, property-based tests generate thousands of random test cases to find edge cases and ensure mathematical properties are preserved.

## Why Property-Based Testing?

For blockchain protocols, certain properties **must always hold**:

- **Total supply conservation**: `sum(mints) - sum(burns) = total_supply`
- **Balance invariants**: `sum(all_balances) ≤ total_supply`
- **Reserve ratio safety**: `collateral / edsc_supply ≥ minimum_ratio`
- **Message uniqueness**: No duplicate nonces, no replay attacks
- **Arithmetic safety**: No overflows, no panics

Property-based testing helps us verify these properties hold for:
- Edge cases (max values, zero values)
- Random sequences of operations
- Adversarial inputs
- State transitions

## Framework

We use [proptest](https://github.com/proptest-rs/proptest) for property-based testing:

```toml
[dev-dependencies]
proptest = "1.4.0"
```

## Test Structure

### 1. EDSC Token Properties (`edsc_token_properties.rs`)

Tests token invariants:
- ✅ Total supply conservation across mints/burns
- ✅ Balance never exceeds total supply
- ✅ Transfers preserve total supply
- ✅ Access control (only oracle can mint)
- ✅ Arithmetic safety (no overflows)
- ✅ Event emission consistency

**Example property:**
```rust
proptest! {
    #[test]
    fn total_supply_equals_minted_minus_burned(
        mint_amounts in prop::collection::vec(0u128..1_000_000, 1..10),
        burn_amounts in prop::collection::vec(0u128..1_000_000, 1..10),
    ) {
        // For ANY sequence of mints and burns:
        // final_supply == initial_supply + sum(mints) - sum(burns)
    }
}
```

### 2. Reserve Ratio Properties (`reserve_ratio_properties.rs`)

Tests collateralization invariants:
- ✅ Reserve ratio never drops below minimum
- ✅ Redemptions maintain or improve ratio
- ✅ Checkpoints capture accurate snapshots
- ✅ Collateral withdrawals respect ratio constraints
- ✅ Emergency shutdown triggers at critical ratio
- ✅ Oracle price feed validation

**Critical property:**
```rust
proptest! {
    #[test]
    fn reserve_ratio_never_below_minimum(
        collateral in 1_000_000u128..100_000_000,
        edsc_supply in 1_000_000u128..100_000_000,
        min_ratio in 100u128..200,
    ) {
        // For ANY amounts of collateral and EDSC:
        // (collateral / edsc_supply) * 100 >= min_ratio
    }
}
```

### 3. Bridge Invariants (`bridge_invariants.rs`)

Tests cross-chain security properties:
- ✅ Message nonces strictly increasing
- ✅ Duplicate messages rejected
- ✅ Threshold signature requirements
- ✅ Invalid signatures rejected
- ✅ Total supply conserved across chains
- ✅ Burn-and-mint message pairing
- ✅ Replay attack prevention
- ✅ Custodian authorization

**Security property:**
```rust
proptest! {
    #[test]
    fn duplicate_message_rejected(
        message_data in prop::collection::vec(any::<u8>(), 32..256),
        nonce in 1u64..10000,
    ) {
        // For ANY message:
        // Processing the same message twice should fail
    }
}
```

## Running Property Tests

### Run all property tests (1000 cases each)
```bash
cd tests/property-based
cargo test
```

### Run with increased test cases (10,000 iterations)
```bash
PROPTEST_CASES=10000 cargo test
```

### Run specific property test suite
```bash
cargo test --test edsc_token_properties
cargo test --test reserve_ratio_properties
cargo test --test bridge_invariants
```

### Run with verbose output
```bash
cargo test -- --nocapture
```

### Continuous fuzzing (run forever until failure)
```bash
PROPTEST_CASES=1000000 cargo test
```

## Configuration

Property tests are configured in each test file:

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn my_property(...) {
        // Test implementation
    }
}
```

### Environment Variables

- `PROPTEST_CASES`: Number of test cases to generate (default: 256)
- `PROPTEST_MAX_SHRINK_ITERS`: Max shrinking iterations when failure found (default: 1024)
- `PROPTEST_VERBOSE`: Enable verbose output (0 or 1)

### CI/CD Integration

Property tests run in GitHub Actions with 1000 cases per test:

```yaml
- name: Run property-based tests
  run: cargo test --release --features proptest
  env:
    PROPTEST_CASES: 1000
```

## Current Status

⚠️ **NOTE**: Property tests are currently **scaffolded with TODOs**.

### Implementation Required:

1. **Mock Runtime Setup**
   - Import mock configurations from existing pallet tests
   - Set up test accounts (ALICE, BOB, CHARLIE)
   - Configure test externalities

2. **Pallet Integration**
   - Complete property test implementations marked with `// TODO`
   - Connect to actual pallet extrinsics
   - Add event verification

3. **Coverage Expansion**
   - Add properties for ASF consensus
   - Add properties for Lightning Bloc state channels
   - Add properties for ËtwasmVM execution

### Next Steps:

```bash
# 1. Complete mock runtime setup
# 2. Implement first property test fully (edsc_token_properties::total_supply_conservation)
# 3. Run and verify it passes
# 4. Gradually implement remaining properties
# 5. Integrate into CI/CD pipeline
```

## Best Practices

### Writing Properties

✅ **DO:**
- State properties as mathematical invariants
- Test for illegal state transitions
- Verify error handling with invalid inputs
- Check boundary conditions (0, max values)
- Ensure operations are idempotent where expected

❌ **DON'T:**
- Test specific values (that's unit testing)
- Assume inputs are valid (test invalid inputs too)
- Ignore error cases
- Skip arithmetic overflow tests

### Example Template

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn property_name(
        // Define input generators
        input1 in 0u128..1_000_000,
        input2 in prop::collection::vec(any::<u8>(), 1..100),
    ) {
        new_test_ext().execute_with(|| {
            // Arrange: Set up test state
            let initial_state = get_initial_state();

            // Act: Perform operation
            let result = Pallet::some_operation(input1, input2);

            // Assert: Verify property holds
            prop_assert!(property_holds(result, initial_state));
        });
    }
}
```

## Resources

- [Proptest Book](https://proptest-rs.github.io/proptest/)
- [Property-Based Testing in Rust](https://www.jakobmeier.ch/blogging/Proptest.html)
- [QuickCheck Tutorial](https://betterprogramming.pub/quickcheck-in-rust-6d9d1b56a5a6)

## Integration with Unit Tests

Property tests complement but don't replace unit tests:

| Test Type | Purpose | Example |
|-----------|---------|---------|
| **Unit Tests** | Verify specific scenarios work correctly | "Minting 1000 EDSC increases supply by 1000" |
| **Property Tests** | Verify invariants hold for any input | "Total supply always equals mints minus burns" |
| **Integration Tests** | Verify multi-component workflows | "Full bridge deposit → mint → redeem → burn" |

## Maintenance

When modifying pallets:
1. ✅ Run property tests to ensure invariants still hold
2. ✅ Add new properties for new functionality
3. ✅ Update properties if protocol rules change
4. ✅ Increase test case count for critical changes

---

**Status**: Framework set up, tests scaffolded, implementation pending
**Priority**: High (required for 80%+ test coverage goal)
**Owner**: CI/CD & Infrastructure Terminal (Terminal 3)
**Created**: October 21, 2025
