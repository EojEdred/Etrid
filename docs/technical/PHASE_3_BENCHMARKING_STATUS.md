# Phase 3: FRAME Benchmarking Infrastructure - Status

**Date**: November 6, 2025
**Branch**: `claude/document-build-warnings-summary-011CUsDJK1yS2Q57wqNDWXcC`

---

## ✅ Completed Tasks

### 1. Runtime Benchmarking Infrastructure Setup

**Runtime Cargo.toml** (`05-multichain/flare-chain/runtime/Cargo.toml`)
- ✅ `frame-benchmarking` already configured as optional dependency (line 117)
- ✅ `frame-system-benchmarking` already configured as optional dependency (line 118)
- ✅ Added `pallet-treasury-etrid/runtime-benchmarks` to runtime-benchmarks feature (line 154)

**Runtime lib.rs** (`05-multichain/flare-chain/runtime/src/lib.rs`)
- ✅ `define_benchmarks!` macro already exists (lines 1435-1444)
- ✅ Added `pallet_treasury_etrid` to benchmarks list (line 1443)
- ✅ Fixed pallet names to match runtime module names:
  - `PalletAccounts` → `Accounts`
  - `PalletConsensus` → `Consensus`
  - `PalletGovernance` → `Governance`

### 2. Pallet-Accounts Benchmarking Setup

**Cargo.toml** (`04-accounts/pallet/Cargo.toml`)
- ✅ Added `frame-benchmarking` as optional dependency (line 14)
- ✅ Added `frame-benchmarking/runtime-benchmarks` to runtime-benchmarks feature (line 35)

**lib.rs** (`04-accounts/pallet/src/lib.rs`)
- ✅ Added `mod benchmarking;` declaration with `#[cfg(feature = "runtime-benchmarks")]` (line 8-9)

**benchmarking.rs** (`04-accounts/pallet/src/benchmarking.rs`)
- ✅ Created benchmark module with v2 API
- ✅ Defined benchmarks for 9 extrinsics:
  - transfer
  - mint_etr
  - mint_etd
  - burn
  - create_recovery
  - initiate_recovery
  - approve_recovery
  - execute_recovery
  - cancel_recovery

### 3. Pallet-Treasury Benchmarking Setup

**Cargo.toml** (`src/pallets/pallet-treasury/Cargo.toml`)
- ✅ `frame-benchmarking` already configured as optional dependency (line 12)
- ✅ Updated runtime-benchmarks feature to include frame-support and frame-system (lines 37-41)

**lib.rs** (`src/pallets/pallet-treasury/src/lib.rs`)
- ✅ Added `mod benchmarking;` declaration with `#[cfg(feature = "runtime-benchmarks")]` (lines 76-77)

**benchmarking.rs** (`src/pallets/pallet-treasury/src/benchmarking.rs`)
- ✅ Created benchmark module with v2 API
- ✅ Defined benchmarks for 4 extrinsics:
  - fund_treasury
  - propose_disbursement
  - approve_disbursement
  - emergency_withdrawal

---

## ⚠️ Issues Found - Benchmarking Code Needs Fixes

### Pallet-Accounts Benchmark Errors

The benchmark implementations don't match the actual pallet structure:

**Type Mismatches:**
1. ❌ Using `BalanceOf<T>` - not found in scope
2. ❌ Using `AccountInfo` - actual struct is `AccountData`
3. ❌ `Accounts::<T>::get()` returns `AccountData`, not `Option<AccountData>`
4. ❌ `RecoveryConfig` fields are `guardians` and `delay_period`, not `friends`, `approvals`, `new_account_id`

**Function Signature Mismatches:**
1. ❌ `transfer()` - actual signature different than assumed
2. ❌ `burn()` - actual signature different than assumed
3. ❌ `create_recovery()` - takes 3 arguments, not 2
4. ❌ `cancel_recovery()` - takes 1 argument, not 0

**Example Error:**
```rust
error[E0412]: cannot find type `BalanceOf` in this scope
  --> 04-accounts/pallet/src/benchmarking.rs:20:21
   |
20 |         let amount: BalanceOf<T> = 1000u32.into();
   |                     ^^^^^^^^^ not found in this scope
```

### Pallet-Treasury Benchmark Errors

**Type Mismatches:**
1. ❌ `BalanceOf<T>` - type is private (inside pallet module)
2. ❌ `DisbursementProposal` - struct is private
3. ❌ `ProposalStatus` - enum is private
4. ❌ `Proposals` - storage is private
5. ❌ `ProposalCount` - storage is private

**Function Signature Mismatches:**
1. ❌ `fund_treasury()` - actual signature different
2. ❌ `propose_disbursement()` - takes 4 arguments, not 3
3. ❌ `emergency_withdrawal()` - takes 3 arguments, not 1

**Trait Method Mismatches:**
1. ❌ `T::Currency::mint_into()` - Currency trait doesn't have this method
2. ❌ `T::Currency::balance()` - should be `free_balance()`

**Enum Variant Mismatches:**
1. ❌ `FundingSource::ExternalDonation` - variant doesn't exist
2. ❌ `BudgetCategory::Emergency` - variant doesn't exist

**Example Errors:**
```rust
error[E0412]: cannot find type `BalanceOf` in this scope
   --> src/pallets/pallet-treasury/src/benchmarking.rs:20:21
    |
 20 |         let amount: BalanceOf<T> = 10000u32.into();
    |                     ^^^^^^^^^ not found in this scope
    |
note: type alias `crate::pallet::BalanceOf` exists but is inaccessible

error[E0599]: no function or associated item named `mint_into` found for associated type
  --> src/pallets/pallet-treasury/src/benchmarking.rs:23:22
```

---

## 🔧 How to Fix Benchmark Modules

### Step 1: Import Required Types

For both pallets, the benchmarking module needs access to private types. Two solutions:

**Option A: Re-export types from pallet module**
```rust
// In lib.rs, inside the pallet module
pub use super::*;  // Re-export parent scope items
```

**Option B: Import from pallet namespace**
```rust
// In benchmarking.rs
use crate::pallet::{BalanceOf, AccountData, RecoveryConfig, etc.};
```

### Step 2: Read Actual Extrinsic Signatures

For each pallet, check the actual function signatures in lib.rs:

```bash
# For pallet-accounts
grep -A 10 "pub fn transfer\|pub fn mint_etr\|pub fn burn" 04-accounts/pallet/src/lib.rs

# For pallet-treasury
grep -A 10 "pub fn fund_treasury\|pub fn propose_disbursement" src/pallets/pallet-treasury/src/lib.rs
```

### Step 3: Understand Storage Structure

Check how storage items work:
```rust
// If Accounts returns AccountData directly:
let account_data = Accounts::<T>::get(&account_id);
assert_eq!(account_data.etr_balance, expected);

// If Accounts returns Option<AccountData>:
let account_data = Accounts::<T>::get(&account_id).unwrap();
assert_eq!(account_data.etr_balance, expected);
```

### Step 4: Use Correct Currency Trait Methods

For `pallet_treasury`, the Currency trait from FRAME v2 uses:
```rust
// Instead of mint_into:
use frame_support::traits::Currency;
let _ = T::Currency::deposit_creating(&account, amount);

// Instead of balance:
let balance = T::Currency::free_balance(&account);
```

### Step 5: Match Enum Variants

Check actual enum definitions:
```bash
# Check FundingSource variants
grep -A 10 "pub enum FundingSource" src/pallets/pallet-treasury/src/lib.rs

# Check BudgetCategory variants
grep -A 10 "pub enum BudgetCategory" src/pallets/pallet-treasury/src/lib.rs
```

---

## 📝 Recommended Next Steps

### Immediate Actions (1-2 hours)

1. **Read pallet implementations carefully**
   - Study `04-accounts/pallet/src/lib.rs` (especially lines 200-450)
   - Study `src/pallets/pallet-treasury/src/lib.rs` (especially lines 400-700)
   - Understand exact function signatures
   - Note all type definitions and storage structures

2. **Fix pallet-accounts benchmarking.rs**
   - Import correct types: `AccountData`, `RecoveryConfig`
   - Define `BalanceOf<T>` type alias locally
   - Fix all function call signatures to match actual extrinsics
   - Fix RecoveryConfig structure (use `guardians` and `delay_period`)

3. **Fix pallet-treasury benchmarking.rs**
   - Import or re-export private types
   - Use correct Currency trait methods
   - Use correct FundingSource and BudgetCategory variants
   - Fix all function call signatures

4. **Test compilation**
   ```bash
   SKIP_WASM_BUILD=1 cargo check -p pallet-accounts --features runtime-benchmarks
   SKIP_WASM_BUILD=1 cargo check -p pallet-treasury --features runtime-benchmarks
   SKIP_WASM_BUILD=1 cargo check -p flare-chain-runtime --features runtime-benchmarks
   ```

### After Fixing Benchmarks (2-3 hours)

5. **Create mock.rs for testing** (if not exists)
   - Both pallets need mock runtime for benchmark tests
   - See existing `src/pallets/pallet-treasury/src/mock.rs` as reference

6. **Run benchmark tests**
   ```bash
   cargo test --features runtime-benchmarks -p pallet-accounts
   cargo test --features runtime-benchmarks -p pallet-treasury
   ```

7. **Generate actual weight files**
   ```bash
   # From node binary
   ./target/release/flarechain-node benchmark pallet \
     --pallet pallet_accounts \
     --extrinsic '*' \
     --steps 50 \
     --repeat 20 \
     --output ./04-accounts/pallet/src/weights.rs
   ```

8. **Replace conservative weights with generated weights**
   - Update WeightInfo implementations in lib.rs
   - Test that runtime still compiles

---

## 📊 Current Progress Summary

### Infrastructure ✅ (100%)
- [x] Runtime benchmarking feature configured
- [x] Pallet Cargo.toml files configured
- [x] Benchmark modules created
- [x] Module declarations added

### Implementation ⚠️ (20%)
- [x] Benchmark skeletons created
- [ ] Benchmark implementations corrected (need to match actual pallets)
- [ ] Compilation errors fixed
- [ ] Tests passing

### Deployment ❌ (0%)
- [ ] Benchmark CLI configured in node
- [ ] Weight generation tested
- [ ] Generated weights integrated
- [ ] Production verification

**Overall Progress: ~40% complete**

---

## 🎯 Alternative Approach: Simpler Benchmarks

If fixing the complex benchmark implementations is too time-consuming, consider this approach:

### Minimal Benchmark Pattern

```rust
// Simple benchmark that just calls the function
#[benchmark]
fn transfer() {
    let caller: T::AccountId = whitelisted_caller();
    let recipient: T::AccountId = account("recipient", 0, 0);

    // Minimal setup
    // ...

    #[extrinsic_call]
    transfer(RawOrigin::Signed(caller), recipient, /* other args */);

    // No assertions needed for weight generation
}
```

The benchmarking framework measures actual execution time, so minimal assertions are fine for weight generation purposes.

---

## 📚 Reference Documentation

- [FRAME Benchmarking v2 Guide](https://docs.substrate.io/reference/how-to-guides/weights/add-benchmarks/)
- [Benchmarking Macro Documentation](https://paritytech.github.io/substrate/master/frame_benchmarking/v2/index.html)
- [Weight Generation CLI](https://docs.substrate.io/reference/command-line-tools/benchmark/)

---

## ✅ Files Modified in This Session

1. `05-multichain/flare-chain/runtime/Cargo.toml` - Added pallet-treasury to runtime-benchmarks
2. `05-multichain/flare-chain/runtime/src/lib.rs` - Fixed define_benchmarks! macro
3. `04-accounts/pallet/Cargo.toml` - Added frame-benchmarking dependency
4. `04-accounts/pallet/src/lib.rs` - Added benchmarking module declaration
5. `04-accounts/pallet/src/benchmarking.rs` - Created (needs fixes)
6. `src/pallets/pallet-treasury/Cargo.toml` - Updated runtime-benchmarks feature
7. `src/pallets/pallet-treasury/src/lib.rs` - Added benchmarking module declaration
8. `src/pallets/pallet-treasury/src/benchmarking.rs` - Created (needs fixes)

---

**Status**: Infrastructure setup complete. Benchmark implementations need corrections to match actual pallet code.

**Next**: Fix benchmark module implementations or use minimal benchmark pattern for weight generation.
