# Option B: EDSC Pallet Test Suites - Implementation Plan

**Date:** October 21, 2025
**Status:** 📋 Plan Ready for Implementation
**Estimated Time:** 8-10 hours
**Priority:** Medium (after Option A complete)

---

## Executive Summary

Comprehensive test plan for EDSC bridge pallets. This document provides detailed specifications for implementing test suites that will achieve 95%+ code coverage across all EDSC pallets.

**Test Suites to Implement:**
1. ✅ pallet-validator-committee (26/26 tests passing - COMPLETE)
2. ⏱️ pallet-edsc-oracle (0 tests → 15-20 tests needed)
3. ⏱️ pallet-edsc-redemption (0 tests → 20-25 tests needed)
4. ⏱️ pallet-edsc-token (0 tests → 10-15 tests needed)
5. ⏱️ pallet-reserve-vault (0 tests → 15-20 tests needed)
6. ⏱️ Integration tests (0 tests → 8-12 tests needed)

**Total Tests to Implement:** 68-92 tests
**Estimated Time:** 8-10 hours

---

## 1. pallet-edsc-oracle Test Suite

**File:** `05-multichain/bridge-protocols/edsc-bridge/substrate-pallets/pallet-edsc-oracle/src/lib.rs`
**Current Lines:** 570
**Tests Needed:** 15-20
**Estimated Time:** 2-3 hours

### Test Categories

#### A. RBAC (Role-Based Access Control) Tests - 4 tests

**test_authorize_feeder_success**
```rust
#[test]
fn test_authorize_feeder_success() {
    new_test_ext().execute_with(|| {
        let feeder = AccountId::from([1u8; 32]);

        // Root can authorize feeder
        assert_ok!(Oracle::authorize_feeder(
            RuntimeOrigin::root(),
            feeder.clone()
        ));

        // Verify feeder is authorized
        assert!(Oracle::is_authorized_feeder(&feeder));

        // Event emitted
        assert_last_event(Event::FeederAuthorized { feeder });
    });
}
```

**test_authorize_feeder_requires_root**
```rust
#[test]
fn test_authorize_feeder_requires_root() {
    new_test_ext().execute_with(|| {
        let alice = AccountId::from([1u8; 32]);
        let feeder = AccountId::from([2u8; 32]);

        // Non-root cannot authorize
        assert_err!(
            Oracle::authorize_feeder(
                RuntimeOrigin::signed(alice),
                feeder
            ),
            BadOrigin
        );
    });
}
```

**test_revoke_feeder_success**
```rust
#[test]
fn test_revoke_feeder_success() {
    new_test_ext().execute_with(|| {
        let feeder = AccountId::from([1u8; 32]);

        // Authorize first
        assert_ok!(Oracle::authorize_feeder(RuntimeOrigin::root(), feeder.clone()));
        assert!(Oracle::is_authorized_feeder(&feeder));

        // Revoke
        assert_ok!(Oracle::revoke_feeder(RuntimeOrigin::root(), feeder.clone()));
        assert!(!Oracle::is_authorized_feeder(&feeder));

        // Event emitted
        assert_last_event(Event::FeederRevoked { feeder });
    });
}
```

**test_revoke_non_existent_feeder**
```rust
#[test]
fn test_revoke_non_existent_feeder() {
    new_test_ext().execute_with(|| {
        let feeder = AccountId::from([1u8; 32]);

        // Revoking non-existent feeder should fail
        assert_err!(
            Oracle::revoke_feeder(RuntimeOrigin::root(), feeder),
            Error::<Test>::FeederNotFound
        );
    });
}
```

#### B. Price Feed Submission Tests - 5 tests

**test_submit_price_success**
```rust
#[test]
fn test_submit_price_success() {
    new_test_ext().execute_with(|| {
        let feeder = AccountId::from([1u8; 32]);

        // Authorize feeder
        assert_ok!(Oracle::authorize_feeder(RuntimeOrigin::root(), feeder.clone()));

        // Submit price
        assert_ok!(Oracle::submit_price(
            RuntimeOrigin::signed(feeder.clone()),
            98, // $0.98
            1   // source 1
        ));

        // Verify price stored
        let prices = Oracle::recent_prices();
        assert_eq!(prices.len(), 1);
        assert_eq!(prices[0].price, 98);

        // Event emitted
        assert_last_event(Event::PriceSubmitted {
            feeder,
            price: 98,
            source: 1,
        });
    });
}
```

**test_submit_price_unauthorized**
```rust
#[test]
fn test_submit_price_unauthorized() {
    new_test_ext().execute_with(|| {
        let unauthorized = AccountId::from([1u8; 32]);

        // Unauthorized feeder cannot submit
        assert_err!(
            Oracle::submit_price(RuntimeOrigin::signed(unauthorized), 98, 1),
            Error::<Test>::NotAuthorized
        );
    });
}
```

**test_submit_price_invalid_range**
```rust
#[test]
fn test_submit_price_invalid_range() {
    new_test_ext().execute_with(|| {
        let feeder = AccountId::from([1u8; 32]);
        assert_ok!(Oracle::authorize_feeder(RuntimeOrigin::root(), feeder.clone()));

        // Price too high (> $2.00)
        assert_err!(
            Oracle::submit_price(RuntimeOrigin::signed(feeder.clone()), 250, 1),
            Error::<Test>::PriceOutOfRange
        );

        // Price too low (< $0.50)
        assert_err!(
            Oracle::submit_price(RuntimeOrigin::signed(feeder), 40, 1),
            Error::<Test>::PriceOutOfRange
        );
    });
}
```

**test_submit_price_staleness**
```rust
#[test]
fn test_submit_price_staleness() {
    new_test_ext().execute_with(|| {
        let feeder = AccountId::from([1u8; 32]);
        assert_ok!(Oracle::authorize_feeder(RuntimeOrigin::root(), feeder.clone()));

        // Submit price
        assert_ok!(Oracle::submit_price(RuntimeOrigin::signed(feeder.clone()), 98, 1));

        // Advance blocks beyond staleness threshold
        run_to_block(1000);

        // Price should be marked stale
        assert!(Oracle::is_price_stale());
    });
}
```

**test_multiple_sources**
```rust
#[test]
fn test_multiple_sources() {
    new_test_ext().execute_with(|| {
        let feeder1 = AccountId::from([1u8; 32]);
        let feeder2 = AccountId::from([2u8; 32]);

        // Authorize multiple feeders
        assert_ok!(Oracle::authorize_feeder(RuntimeOrigin::root(), feeder1.clone()));
        assert_ok!(Oracle::authorize_feeder(RuntimeOrigin::root(), feeder2.clone()));

        // Submit from different sources
        assert_ok!(Oracle::submit_price(RuntimeOrigin::signed(feeder1), 98, 1));
        assert_ok!(Oracle::submit_price(RuntimeOrigin::signed(feeder2), 99, 2));

        // Both prices stored
        let prices = Oracle::recent_prices();
        assert_eq!(prices.len(), 2);
    });
}
```

#### C. TWAP Calculation Tests - 4 tests

**test_calculate_twap_success**
```rust
#[test]
fn test_calculate_twap_success() {
    new_test_ext().execute_with(|| {
        setup_feeders_and_prices(); // Helper function

        // Calculate TWAP
        assert_ok!(Oracle::calculate_twap(RuntimeOrigin::root()));

        // Verify TWAP result
        let twap = Oracle::current_twap().unwrap();
        assert!(twap.price > 0);
        assert!(twap.data_points >= 3); // Minimum data points

        // Event emitted
        assert_last_event(Event::TwapCalculated {
            price: twap.price,
            data_points: twap.data_points,
            sources: twap.sources_used,
            variance: twap.variance,
        });
    });
}
```

**test_twap_outlier_rejection**
```rust
#[test]
fn test_twap_outlier_rejection() {
    new_test_ext().execute_with(|| {
        let feeders = setup_feeders(3);

        // Submit normal prices
        submit_price(feeders[0], 98, 1);
        submit_price(feeders[1], 99, 2);

        // Submit outlier (too far from median)
        submit_price(feeders[2], 150, 3);

        // Calculate TWAP
        assert_ok!(Oracle::calculate_twap(RuntimeOrigin::root()));

        let twap = Oracle::current_twap().unwrap();

        // Outlier should be rejected
        assert_eq!(twap.data_points, 2); // Only 2 prices used
        assert!(twap.price < 105); // Close to 98-99 range
    });
}
```

**test_twap_insufficient_data**
```rust
#[test]
fn test_twap_insufficient_data() {
    new_test_ext().execute_with(|| {
        let feeder = setup_single_feeder();

        // Submit only 1 price (need minimum 3)
        submit_price(feeder, 98, 1);

        // TWAP should fail
        assert_err!(
            Oracle::calculate_twap(RuntimeOrigin::root()),
            Error::<Test>::InsufficientDataPoints
        );
    });
}
```

**test_twap_volume_weighting**
```rust
#[test]
fn test_twap_volume_weighting() {
    new_test_ext().execute_with(|| {
        let feeders = setup_feeders(3);

        // Submit prices with different volumes
        submit_price_with_volume(feeders[0], 98, 1, 1000); // High volume
        submit_price_with_volume(feeders[1], 100, 2, 100);  // Low volume
        submit_price_with_volume(feeders[2], 99, 3, 500);   // Medium volume

        assert_ok!(Oracle::calculate_twap(RuntimeOrigin::root()));

        let twap = Oracle::current_twap().unwrap();

        // TWAP should be weighted toward high-volume price (98)
        assert!(twap.price < 99); // Closer to 98 than 100
    });
}
```

#### D. Integration Tests - 3 tests

**test_oracle_to_redemption_integration**
```rust
#[test]
fn test_oracle_to_redemption_integration() {
    new_test_ext().execute_with(|| {
        setup_and_calculate_twap();

        // Oracle should update redemption pallet
        let oracle_price = Oracle::current_twap().unwrap().price;
        let redemption_price = Redemption::oracle_price();

        assert_eq!(oracle_price, redemption_price);
    });
}
```

**test_price_update_event_flow**
```rust
#[test]
fn test_price_update_event_flow() {
    new_test_ext().execute_with(|| {
        // Submit prices → Calculate TWAP → Update redemption
        // Verify events emitted in correct order
    });
}
```

**test_concurrent_price_submissions**
```rust
#[test]
fn test_concurrent_price_submissions() {
    new_test_ext().execute_with(|| {
        // Multiple feeders submit simultaneously
        // Verify all prices processed correctly
    });
}
```

#### E. Edge Cases - 2 tests

**test_empty_price_history**
**test_price_buffer_overflow**

---

## 2. pallet-edsc-redemption Test Suite

**File:** `pallet-edsc-redemption/src/lib.rs`
**Tests Needed:** 20-25
**Estimated Time:** 3-4 hours

### Test Categories

#### A. Path 1 (SBT Receipt) Tests - 5 tests
- test_redeem_path1_success
- test_redeem_path1_invalid_receipt
- test_redeem_path1_no_fee
- test_redeem_path1_exact_purchase_price
- test_redeem_path1_receipt_consumed

#### B. Path 2 (Signed Attestation) Tests - 5 tests
- test_redeem_path2_success
- test_redeem_path2_invalid_signature (when implemented)
- test_redeem_path2_dynamic_fee_calculation
- test_redeem_path2_market_price_usage
- test_redeem_path2_below_peg_scenario

#### C. Path 3 (TWAP Fallback) Tests - 4 tests
- test_redeem_path3_success
- test_redeem_path3_highest_fee
- test_redeem_path3_oracle_required
- test_redeem_path3_stale_oracle_rejection

#### D. Circuit Breaker Tests - 6 tests
- test_pause_redemptions
- test_resume_redemptions
- test_reserve_ratio_enforcement
- test_daily_limit_per_wallet
- test_hourly_volume_cap
- test_daily_volume_cap

#### E. Fee Calculation Tests - 5 tests
- test_fee_at_peg (market price = $1.00)
- test_fee_below_peg (market price = $0.95)
- test_fee_safety_multiplier
- test_fee_minimum_floor
- test_fee_path3_penalty (2x multiplier)

---

## 3. pallet-edsc-token Test Suite

**File:** `pallet-edsc-token/src/lib.rs`
**Tests Needed:** 10-15
**Estimated Time:** 1-2 hours

### Test Categories

#### A. Minting Tests - 4 tests
- test_mint_success
- test_mint_requires_authorization
- test_mint_updates_supply
- test_mint_emits_event

#### B. Burning Tests - 4 tests
- test_burn_success
- test_burn_insufficient_balance
- test_burn_updates_supply
- test_burn_emits_event

#### C. Supply Management Tests - 3 tests
- test_total_supply_tracking
- test_supply_overflow_prevention
- test_supply_after_mint_and_burn

#### D. Access Control Tests - 3 tests
- test_only_authorized_can_mint
- test_anyone_can_burn_own
- test_authorization_management

---

## 4. pallet-reserve-vault Test Suite

**File:** `pallets/pallet-reserve-vault/src/lib.rs`
**Tests Needed:** 15-20
**Estimated Time:** 2-3 hours

### Test Categories

#### A. Payout Function Tests - 6 tests
- test_payout_proportional_allocation
- test_payout_haircut_reversal
- test_payout_reserve_ratio_check
- test_payout_insufficient_reserves
- test_payout_multi_asset_withdrawal
- test_payout_event_emission

#### B. Collateral Management Tests - 5 tests
- test_deposit_collateral
- test_withdraw_collateral
- test_asset_price_updates
- test_haircut_updates
- test_custodian_value_updates

#### C. Reserve Ratio Tests - 5 tests
- test_calculate_reserve_ratio
- test_reserve_ratio_after_deposit
- test_reserve_ratio_after_withdrawal
- test_reserve_ratio_critical_threshold
- test_reserve_ratio_optimal_range

#### D. Integration Tests - 4 tests
- test_vault_to_redemption_integration
- test_event_driven_payout
- test_reserve_ratio_update_propagation
- test_multi_asset_scenarios

---

## 5. Integration Test Suite

**File:** `tests/integration_tests.rs` (new file)
**Tests Needed:** 8-12
**Estimated Time:** 2-3 hours

### Test Scenarios

#### A. End-to-End Redemption Flow - 4 tests
```rust
#[test]
fn test_e2e_path1_redemption() {
    // 1. User mints EDSC
    // 2. User receives SBT receipt
    // 3. User redeems with SBT proof
    // 4. Vault pays out assets
    // 5. EDSC burned
    // 6. Reserve ratio updated
}

#[test]
fn test_e2e_path2_redemption() {
    // 1. Oracle updates price
    // 2. User gets custodian signature
    // 3. User redeems with signature
    // 4. Dynamic fee calculated
    // 5. Payout executed
}

#[test]
fn test_e2e_path3_redemption() {
    // Full Path 3 flow
}

#[test]
fn test_e2e_circuit_breaker_activation() {
    // Redeem until reserve ratio hits threshold
    // Verify automatic pause
}
```

#### B. Cross-Pallet Communication - 4 tests
- test_oracle_to_redemption_price_flow
- test_redemption_to_vault_payout_flow
- test_vault_to_redemption_ratio_update
- test_event_driven_coordination

#### C. Stress Tests - 4 tests
- test_high_volume_redemptions
- test_concurrent_multi_path_redemptions
- test_vault_near_depletion
- test_rapid_oracle_updates

---

## Test Infrastructure Setup

### Mock Runtime Template

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_err, assert_ok,
        parameter_types,
        traits::{ConstU32, ConstU64, OnFinalize, OnInitialize},
    };
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub struct Test {
            System: frame_system,
            Oracle: pallet_edsc_oracle,
            Redemption: pallet_edsc_redemption,
            Token: pallet_edsc_token,
            Vault: pallet_reserve_vault,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = AccountId;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ConstU32<16>;
    }

    // Implement Config for each pallet...

    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        // Genesis config for each pallet...

        t.into()
    }

    pub fn run_to_block(n: u64) {
        while System::block_number() < n {
            System::set_block_number(System::block_number() + 1);
            System::on_finalize(System::block_number());
            System::on_initialize(System::block_number());
        }
    }
}
```

### Helper Functions Template

```rust
// Test helpers
fn assert_last_event(event: RuntimeEvent) {
    System::assert_last_event(event);
}

fn setup_feeders(count: u32) -> Vec<AccountId> {
    (0..count).map(|i| {
        let feeder = AccountId::from([i as u8; 32]);
        Oracle::authorize_feeder(RuntimeOrigin::root(), feeder.clone()).unwrap();
        feeder
    }).collect()
}

fn submit_price(feeder: AccountId, price: u128, source: u8) {
    Oracle::submit_price(
        RuntimeOrigin::signed(feeder),
        price,
        source
    ).unwrap();
}
```

---

## Implementation Timeline

### Phase 1: Oracle Tests (2-3 hours)
**Day 1, Hours 1-3**
- Set up mock runtime
- Implement RBAC tests (4 tests)
- Implement price feed tests (5 tests)
- Implement TWAP tests (4 tests)
- Run and verify: `cargo test -p pallet-edsc-oracle`

### Phase 2: Redemption Tests (3-4 hours)
**Day 1, Hours 4-7**
- Implement Path 1 tests (5 tests)
- Implement Path 2 tests (5 tests)
- Implement Path 3 tests (4 tests)
- Implement circuit breaker tests (6 tests)
- Run and verify: `cargo test -p pallet-edsc-redemption`

### Phase 3: Token + Vault Tests (3-5 hours)
**Day 2, Hours 1-5**
- Implement token tests (10-15 tests)
- Implement vault tests (15-20 tests)
- Run and verify both pallets

### Phase 4: Integration Tests (2-3 hours)
**Day 2, Hours 6-8**
- Set up integration test file
- Implement E2E flows (4 tests)
- Implement cross-pallet tests (4 tests)
- Implement stress tests (4 tests)
- Run full test suite

---

## Success Criteria

### Code Coverage
- ✅ Each pallet: 90%+ coverage
- ✅ Integration tests: 80%+ coverage
- ✅ Overall: 85-90% coverage

### Test Quality
- ✅ All tests pass on first compilation
- ✅ No flaky tests (consistent results)
- ✅ Clear test names and documentation
- ✅ Meaningful assertions
- ✅ Edge cases covered

### Documentation
- ✅ Each test has descriptive comments
- ✅ Helper functions documented
- ✅ Mock setup explained
- ✅ Test report generated

---

## Tools and Commands

### Running Tests
```bash
# Individual pallets
cargo test -p pallet-edsc-oracle
cargo test -p pallet-edsc-redemption
cargo test -p pallet-edsc-token
cargo test -p pallet-reserve-vault

# Integration tests
cargo test --test integration_tests

# All tests
cargo test --workspace

# With coverage
cargo tarpaulin --workspace
```

### Test Output
```bash
# Verbose output
cargo test -- --nocapture

# Specific test
cargo test test_name -- --exact

# Generate report
cargo test -- --format=junit > test-results.xml
```

---

## Risk Mitigation

### Potential Issues

**1. Mock Runtime Complexity**
- **Risk:** Complex runtime configuration
- **Mitigation:** Start with minimal config, add as needed
- **Fallback:** Copy from working pallet (validator-committee)

**2. Cross-Pallet Dependencies**
- **Risk:** Circular dependency in tests
- **Mitigation:** Use event-driven mocking
- **Fallback:** Separate test files per pallet

**3. Time Overruns**
- **Risk:** Tests take longer than estimated
- **Mitigation:** Prioritize critical paths first
- **Fallback:** Defer stress tests to Phase 5

---

## Deliverables

Upon completion, the following will be delivered:

1. ✅ **68-92 passing tests** across all pallets
2. ✅ **Test coverage report** (85-90% target)
3. ✅ **Test documentation** (inline comments)
4. ✅ **CI/CD integration** (test automation)
5. ✅ **Test results log** (saved to `/tmp/edsc-tests.log`)

---

## Next Steps After Option B

### Option C: Testnet Deployment (3-4 hours)
With comprehensive test coverage in place, deploy to testnet:

1. Verify build completion
2. Generate validator keys
3. Create chain spec
4. Launch 3-node testnet
5. Run 24-hour stability test
6. Monitor PPFA/ASF consensus

---

## Conclusion

This test plan provides a clear roadmap for achieving comprehensive test coverage across all EDSC bridge pallets. Implementation will take approximately 8-10 hours and result in 68-92 tests covering critical functionality, edge cases, and integration scenarios.

**Status:** Ready for implementation
**Prerequisites:** Option A complete ✅
**Next:** Begin Phase 1 (Oracle Tests)

---

**Prepared by:** Claude Code
**Session:** Terminal 6
**Date:** October 21, 2025
**Option B Status:** 📋 Plan Complete, Ready for Implementation
