# ĒTRID Integration Tests

**Status:** 📋 Ready for Implementation
**Assignee:** External Developer
**Priority:** High
**Estimated Time:** 8-12 hours

---

## Quick Start for Implementer

### 1. Read the Implementation Guide

**FILE:** `INTEGRATION_TEST_GUIDE.md`

This guide contains:
- Complete step-by-step instructions
- File structure to create
- Code templates and pseudocode
- Priority order (what to implement first)
- Expected results
- Troubleshooting tips

### 2. Set Up Your Environment

```bash
# Install required tools
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
cargo install cargo-contract --force

# Start test node (in separate terminal)
contracts-node --dev --tmp

# Verify setup
contracts-node --version
cargo contract --version
```

### 3. Create Test Structure

```bash
cd /Users/macbook/Desktop/etrid/contracts/tests

# Create directories
mkdir -p common integration e2e

# Create files (see INTEGRATION_TEST_GUIDE.md for content)
touch common/mod.rs common/setup.rs common/helpers.rs common/constants.rs
touch integration/test_two_tier_flow.rs
touch integration/test_edsc_minting.rs
touch integration/test_intent_router.rs
# ... etc
```

### 4. Implement in Priority Order

**Phase 1 (Critical):**
1. `common/setup.rs` - Deploy all contracts
2. `common/helpers.rs` - Test utilities
3. `test_two_tier_flow.rs` - Basic liquidity flow ⭐
4. `test_edsc_minting.rs` - Stablecoin minting ⭐
5. `test_intent_router.rs` - User abstraction ⭐

**Phase 2 (Important):**
6. `test_bridge_attestation.rs` - Bridge security
7. `test_permissions.rs` - Access control
8. `test_edge_cases.rs` - Error handling

**Phase 3 (Nice to Have):**
9. `test_performance.rs` - Benchmarks
10. `test_full_user_flow.rs` - Complete E2E
11. `test_multi_user.rs` - Concurrent ops

### 5. Run Tests

```bash
# Run all tests
cargo test --all

# Run specific test
cargo test test_btc_to_etr_flow -- --nocapture

# With detailed output
cargo test --all -- --nocapture
```

---

## What You're Testing

**System Architecture:**
```
User
  ↓
Intent Router (hide complexity)
  ↓
AutoSwapExecutor (orchestrate)
  ↓
Two-Tier Pools (BTC → wBTC → ÉTR)
  ↓
Bridge (M-of-N attestation)
```

**Test Coverage:**
- ✅ 47 contracts working together
- ✅ Multi-step atomic operations
- ✅ Permission enforcement
- ✅ Error handling
- ✅ Edge cases
- ✅ Performance under load

---

## Documentation References

**Must Read:**
1. `INTEGRATION_TEST_GUIDE.md` - **START HERE** ⭐
2. `../WIRING_CONFIGURATION.md` - Permission matrix
3. `../INTEGRATION_PLAN.md` - Deployment plan

**Reference:**
4. `../primeswap/TWO_TIER_ARCHITECTURE.md`
5. `../edsc/EDSC_RESERVE_ARCHITECTURE.md`
6. `../intent-router/INTENT_ROUTER_ARCHITECTURE.md`

**Examples:**
7. `../scripts/deploy_phase*.sh` - Deployment reference
8. Individual contract `lib.rs` files - Unit test examples

---

## Success Criteria

When you're done:

✅ **Code Complete:**
- [ ] All 11 test files implemented
- [ ] Common utilities complete
- [ ] Tests compile without errors

✅ **Tests Pass:**
- [ ] `cargo test --all` passes
- [ ] >80% code coverage
- [ ] All critical paths tested

✅ **Documentation:**
- [ ] Test README updated
- [ ] Coverage report generated
- [ ] Any bugs documented as issues

---

## Expected Timeline

| Phase | Tasks | Time |
|-------|-------|------|
| Setup | Environment + structure | 1 hour |
| Phase 1 | Critical tests (1-5) | 4-6 hours |
| Phase 2 | Important tests (6-8) | 2-3 hours |
| Phase 3 | Nice-to-have (9-11) | 2-3 hours |
| **Total** | **All tests complete** | **8-12 hours** |

---

## Help & Support

**If you get stuck:**

1. Check `INTEGRATION_TEST_GUIDE.md` - Most answers are there
2. Review deployment scripts in `../scripts/` - They show the correct order
3. Look at unit tests in contract `lib.rs` files - Examples of calling contracts
4. Check ink! docs: https://use.ink/basics/contract-testing

**Common Issues:**
- Node not running → Start `contracts-node --dev --tmp`
- Deployment fails → Check order in `INTEGRATION_PLAN.md`
- Balance assertions fail → Check decimal conversions (BTC=8, ÉTR=18)
- Permission errors → Verify roles in `WIRING_CONFIGURATION.md`

---

## Deliverables

Submit a PR with:

1. All test files implemented
2. Tests passing (`cargo test --all`)
3. Coverage report
4. Brief summary of any issues found

**Target:** 100% test pass rate with >80% coverage

---

**Ready to start? Read `INTEGRATION_TEST_GUIDE.md` first!** 📖
