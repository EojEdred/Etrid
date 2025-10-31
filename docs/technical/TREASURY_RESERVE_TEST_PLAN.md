# Treasury & Reserve System - Comprehensive Test Plan

## Overview

This document outlines the comprehensive testing strategy for the Ëtrid Treasury and Reserve System, covering all critical integration points and flows.

## Test Coverage Goals

- **Unit Tests**: >80% code coverage on critical paths
- **Integration Tests**: All end-to-end flows validated
- **Security Tests**: Authorization, overflow/underflow, edge cases
- **Performance Tests**: Large-scale operations with realistic amounts

## Pallets Under Test

1. **pallet-treasury** - Main treasury fund management
2. **pallet-consensus-day** - Annual governance and minting
3. **pallet-validator-rewards** - Validator payment processing
4. **pallet-reserve-vault** - EDSC reserve management
5. **pallet-edsc-stability** - Stability fee collection

## Critical Flows to Test

### 1. Transaction Fees → Treasury

**Test Cases:**
- ✅ Happy Path: 50% of fees go to treasury, 50% burned
- ✅ Edge Case: Zero fee transactions
- ✅ Edge Case: Maximum fee (u128::MAX / 2)
- ✅ Error Case: Overflow protection
- ✅ Integration: Fee handler calls treasury funding

**Files:**
- `pallet-treasury/src/tests/test_fee_collection.rs`

### 2. Consensus Day Minting → Treasury

**Test Cases:**
- ✅ Happy Path: Exactly 200M ETR minted per year
- ✅ Happy Path: Correct category allocations (40/20/15/15/10%)
- ✅ Edge Case: Zero approved proposals (no minting)
- ✅ Edge Case: Multiple approved proposals (sum ≤ 200M)
- ✅ Error Case: Inflation cap enforcement (5% max)
- ✅ Security: Only governance can trigger minting
- ✅ Integration: Treasury receives minted funds with categories

**Files:**
- `pallet-consensus-day/src/tests/test_minting_phase.rs`
- `pallet-consensus-day/src/tests/test_treasury_integration.rs`

### 3. Slashing → Treasury

**Test Cases:**
- ✅ Happy Path: 50% of slash to treasury, 50% burned
- ✅ Edge Case: Small slash amounts (< 1 ETR)
- ✅ Edge Case: Large slash amounts (> 10M ETR)
- ✅ Error Case: Validator has insufficient stake
- ✅ Integration: Slashing event triggers treasury deposit

**Files:**
- `pallet-validator-rewards/src/tests/test_slashing.rs`

### 4. Bridge Fees → Treasury

**Test Cases:**
- ✅ Happy Path: 10% of bridge fees to treasury
- ✅ Edge Case: Cross-chain transfers of various sizes
- ✅ Error Case: Bridge pallet authorization required
- ✅ Integration: Multiple bridge pallets funding treasury

**Files:**
- `pallet-treasury/src/tests/test_bridge_fees.rs`

### 5. EDSC Fees → Treasury

**Test Cases:**
- ✅ Happy Path: Stability fees accumulate to treasury
- ✅ Happy Path: Liquidation penalties to treasury
- ✅ Edge Case: Interest accrual over long periods
- ✅ Error Case: Fee calculation overflow protection
- ✅ Integration: EDSC pallet calls treasury funding

**Files:**
- `pallet-edsc-stability/src/tests/test_treasury_fees.rs`

### 6. Treasury → Validator Rewards Payments

**Test Cases:**
- ✅ Happy Path: Disbursement with 6-of-9 approval
- ✅ Happy Path: Payment to registered payment accounts (not session keys)
- ✅ Edge Case: Disbursement expires after 7 days
- ✅ Edge Case: Multiple concurrent disbursements
- ✅ Error Case: Insufficient category allocation
- ✅ Error Case: Insufficient treasury balance
- ✅ Security: Only directors can propose
- ✅ Security: Cannot double-approve
- ✅ Performance: Large disbursement (100M+ ETR)

**Files:**
- `pallet-treasury/src/tests/test_disbursement.rs`
- `pallet-treasury/src/tests/test_multisig.rs`

### 7. Treasury ↔ Reserve Swaps

**Test Cases:**
- ✅ Happy Path: Treasury allocates funds to reserve
- ✅ Happy Path: Reserve returns excess to treasury
- ✅ Edge Case: Reserve ratio calculations
- ✅ Error Case: Reserve below minimum threshold
- ✅ Security: Only authorized accounts can swap
- ✅ Integration: Reserve vault and treasury coordination

**Files:**
- `pallet-reserve-vault/src/tests/test_treasury_swaps.rs`

### 8. Emergency Fund Recovery

**Test Cases:**
- ✅ Happy Path: 7-of-9 approval for emergency withdrawal
- ✅ Happy Path: Emergency reserve balance deducted
- ✅ Edge Case: Partial emergency fund usage
- ✅ Error Case: Insufficient emergency reserve
- ✅ Error Case: Only 6-of-9 approvals (should fail)
- ✅ Security: Higher threshold than normal disbursement
- ✅ Security: Cannot use emergency reserve for normal spending

**Files:**
- `pallet-treasury/src/tests/test_emergency.rs`

## Additional Test Categories

### Multisig Treasury Operations

**Test Cases:**
- ✅ Add/remove directors (governance only)
- ✅ Maintain exactly 9 directors
- ✅ Approval thresholds (6-of-9 normal, 7-of-9 emergency)
- ✅ Director replacement workflow
- ✅ Concurrent proposal approvals

**Files:**
- `pallet-treasury/src/tests/test_director_management.rs`

### Annual Minting Cap

**Test Cases:**
- ✅ Exactly 200M ETR per year
- ✅ Inflation rate voting (0-5%)
- ✅ Budget allocation percentages sum to 100%
- ✅ Multiple Consensus Days over years
- ✅ Supply tracking and inflation calculation

**Files:**
- `pallet-consensus-day/src/tests/test_inflation.rs`

### Fee Collection from All Sources

**Test Cases:**
- ✅ Transaction fees tracking
- ✅ Consensus Day minting tracking
- ✅ Slashing proceeds tracking
- ✅ Bridge fees tracking
- ✅ EDSC stability fees tracking
- ✅ Historical totals per source
- ✅ Total treasury balance reconciliation

**Files:**
- `pallet-treasury/src/tests/test_funding_sources.rs`

### Validator Payment Processing

**Test Cases:**
- ✅ Session account → payment account mapping
- ✅ Rewards sent to payment account (not session account)
- ✅ Performance multipliers (uptime, finality, blocks)
- ✅ Delegator vs validator split (50/50)
- ✅ Epoch reward calculation
- ✅ Pending rewards accumulation
- ✅ Claim rewards workflow

**Files:**
- `pallet-validator-rewards/src/tests/test_payment_accounts.rs`
- `pallet-validator-rewards/src/tests/test_rewards.rs`

### Cold Storage Security

**Test Cases:**
- ✅ Payment accounts remain cold (no active keys)
- ✅ Session keys separate from payment keys
- ✅ Controller account hierarchy
- ✅ Payment account updates
- ✅ Security: Cannot send rewards to session account

**Files:**
- `pallet-validator-rewards/src/tests/test_security.rs`

### Emergency Recovery Procedures

**Test Cases:**
- ✅ Emergency withdrawal workflow
- ✅ 7-of-9 threshold enforcement
- ✅ Emergency reserve balance management
- ✅ Audit trail for emergency actions
- ✅ Recovery from various failure scenarios

**Files:**
- `pallet-treasury/src/tests/test_emergency_recovery.rs`

## Test Execution Commands

### Run All Tests
```bash
# All treasury system tests
cargo test --package pallet-treasury --package pallet-consensus-day --package pallet-validator-rewards

# With verbose output
cargo test --package pallet-treasury -- --nocapture

# Specific test module
cargo test --package pallet-treasury test_disbursement
```

### Run Integration Tests
```bash
# Integration tests only
cargo test --package pallet-treasury --test integration

# End-to-end flow tests
cargo test --package etrid-runtime test_treasury_flows
```

### Coverage Reports
```bash
# Generate coverage (requires cargo-tarpaulin)
cargo tarpaulin --package pallet-treasury --package pallet-consensus-day --package pallet-validator-rewards

# Target coverage: >80% on critical paths
```

## Mock Runtime Configuration

All tests use a mock runtime with:
- Block time: 1 second
- Epoch duration: 3600 blocks (1 hour)
- Consensus Day phases: Registration (6h), Voting (12h), Minting (3h), Distribution (1h)
- Test accounts: 1-100 (u64)
- Director accounts: 1-9
- Validator accounts: 10-30

## Test Data Sizes

**Realistic amounts for performance testing:**
- Small: 1-1,000 ETR
- Medium: 1,000-100,000 ETR
- Large: 100,000-10,000,000 ETR
- Massive: 10M-200M ETR (annual minting)

**Edge cases:**
- Zero: 0 ETR
- Minimum: 1 Wei (10^-18 ETR)
- Maximum: u128::MAX

## Security Test Checklist

- [ ] Authorization checks on all privileged operations
- [ ] Overflow/underflow protection in all arithmetic
- [ ] Reentrancy protection on state changes
- [ ] Input validation on all extrinsics
- [ ] Rate limiting on critical operations
- [ ] Event emission for audit trail
- [ ] Storage migration safety
- [ ] Weight calculations accuracy

## Performance Benchmarks

- [ ] Disbursement approval (target: <50ms)
- [ ] Epoch reward distribution (target: <5s for 100 validators)
- [ ] Consensus Day minting (target: <10s)
- [ ] Emergency withdrawal (target: <100ms)
- [ ] Fee collection (target: <10ms)

## Test File Structure

```
src/pallets/
├── pallet-treasury/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── migrations.rs
│   │   ├── mock.rs          # Mock runtime
│   │   ├── tests.rs         # Main test module
│   │   └── tests/
│   │       ├── test_disbursement.rs
│   │       ├── test_multisig.rs
│   │       ├── test_emergency.rs
│   │       ├── test_funding_sources.rs
│   │       ├── test_fee_collection.rs
│   │       └── test_integration.rs
│   └── Cargo.toml
├── pallet-consensus-day/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── mock.rs
│   │   ├── tests.rs
│   │   └── tests/
│   │       ├── test_phases.rs
│   │       ├── test_voting.rs
│   │       ├── test_minting_phase.rs
│   │       ├── test_treasury_integration.rs
│   │       └── test_inflation.rs
│   └── Cargo.toml
└── pallet-validator-rewards/
    ├── src/
    │   ├── lib.rs
    │   ├── migrations.rs
    │   ├── mock.rs
    │   ├── tests.rs
    │   └── tests/
    │       ├── test_payment_accounts.rs
    │       ├── test_rewards.rs
    │       ├── test_slashing.rs
    │       └── test_security.rs
    └── Cargo.toml
```

## Next Steps

1. ✅ Create mock runtime for each pallet
2. ✅ Implement unit tests for each pallet
3. ✅ Write integration tests for cross-pallet flows
4. ✅ Add security and edge case tests
5. ✅ Performance testing with large datasets
6. ✅ Document test coverage gaps
7. ✅ Set up CI/CD for automated testing

## Success Criteria

- ✅ All critical flows have >80% code coverage
- ✅ All integration tests pass
- ✅ No security vulnerabilities identified
- ✅ Performance benchmarks met
- ✅ Documentation complete
- ✅ CI/CD pipeline green
