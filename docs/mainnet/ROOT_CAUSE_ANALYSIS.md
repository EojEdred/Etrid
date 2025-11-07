# Root Cause Analysis: PBC Build Failures

**Date:** November 4, 2025
**Issue:** 12 out of 13 PBC collators failing to build
**Status:** Root cause identified, systematic fix in progress

---

## Problem Summary

All PBC collators except `edsc-pbc-collator` are failing to build with the error:
```
error[E0277]: the trait bound `Runtime: pallet_etr_lock::pallet::Config` is not satisfied
```

---

## Architecture Analysis

### Dependency Chain

```
PBC Runtime (e.g., btc-pbc-runtime)
    ↓ uses
Bridge Pallet (e.g., pallet_bitcoin_bridge)
    ↓ requires (trait bound)
pallet_etr_lock::Config
    ↓ provides
Currency trait + lock_for_bridge() + unlock_from_bridge()
```

### Key Code Evidence

**From `bitcoin-bridge/src/lib.rs:43`:**
```rust
pub trait Config: frame_system::Config + pallet_etr_lock::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    // Note: Currency is inherited from pallet_etr_lock::Config
    ...
}
```

**Every bridge pallet explicitly requires pallet_etr_lock::Config** as a trait bound.

---

## Root Cause

When PBC runtimes were initially created, the following configuration was **incomplete**:

### ✅ What Was Done:
1. Bridge pallet added to Cargo.toml
2. Bridge pallet Config implemented
3. Bridge pallet added to construct_runtime! macro

### ❌ What Was Missing:
1. **pallet-etr-lock not added to Cargo.toml dependencies**
2. **`impl pallet_etr_lock::Config for Runtime` not implemented**
3. **EtrLock not added to construct_runtime! macro**

---

## Why EDSC-PBC Works

`edsc-pbc` is the **native ËDSC stablecoin chain** with a different architecture:
- Uses CCTP-style burn-and-mint (not lock-and-unlock)
- Has custom pallets: pallet_edsc_token, pallet_edsc_receipts, etc.
- **Does not use external bridge pallets that require pallet-etr-lock**
- Handles cross-chain transfers via message passing, not ETR locking

---

## Affected PBCs

All 12 external bridge PBCs are affected:

| PBC | Bridge Protocol | Bridge Type |
|-----|----------------|-------------|
| btc-pbc | bitcoin-bridge | BTC ↔ ETR |
| eth-pbc | ethereum-bridge | ETH ↔ ETR (+ EVM) |
| sol-pbc | solana-bridge | SOL ↔ ETR |
| xrp-pbc | xrp-bridge | XRP ↔ ETR |
| bnb-pbc | bnb-bridge | BNB ↔ ETR |
| trx-pbc | tron-bridge | TRX ↔ ETR |
| ada-pbc | cardano-bridge | ADA ↔ ETR |
| matic-pbc | polygon-bridge | MATIC ↔ ETR |
| link-pbc | chainlink-bridge | LINK ↔ ETR |
| sc-usdt-pbc | stablecoin-usdt-bridge | USDT ↔ ETR |
| doge-pbc | doge-bridge | DOGE ↔ ETR |
| xlm-pbc | stellar-bridge | XLM ↔ ETR |

---

## Secondary Issue: eth-pbc Wasm Build

**Error:**
```
error[E0152]: duplicate lang item in crate `sp_io`: `panic_impl`
```

**Cause:** Conflicting versions of `sp_io` in Wasm build target
**Context:** eth-pbc uses Frontier (Ethereum EVM) which has different Polkadot SDK version tags
**Fix Required:** Dependency version alignment

---

## Solution Design

### Required Changes Per PBC:

**1. Cargo.toml:**
```toml
[dependencies]
# Add after pallet-consensus
pallet-etr-lock = { path = "../../../../../pallets/pallet-etr-lock", default-features = false }

[features]
std = [
    # Add after "pallet-consensus/std"
    "pallet-etr-lock/std",
]
```

**2. Runtime lib.rs - Config Implementation:**
```rust
// ETR Lock Configuration (required by bridge pallets)
parameter_types! {
    pub const MinLockAmount: Balance = 1_000_000; // 0.001 ETR
    pub const MaxLockAmount: Balance = 1_000_000_000_000_000; // 1M ETR
    pub const LockPeriod: BlockNumber = 7 * DAYS;
}

impl pallet_etr_lock::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinLockAmount = MinLockAmount;
    type MaxLockAmount = MaxLockAmount;
    type DefaultLockPeriod = LockPeriod;
}
```

**3. Runtime lib.rs - construct_runtime! macro:**
```rust
construct_runtime!(
    pub struct Runtime {
        // ... existing pallets ...
        Consensus: pallet_consensus,
        EtrLock: pallet_etr_lock,  // ← ADD THIS

        // Bridge pallet
        BitcoinBridge: pallet_bitcoin_bridge,
    }
);
```

---

## Implementation Strategy

### Phase 1: Automated Cargo.toml Fixes (5 minutes)
- Script to add pallet-etr-lock dependencies to all 12 PBCs
- Add to both dependencies and std features

### Phase 2: Template-Based Runtime Fixes (15 minutes)
- Create template etr-lock configuration
- Apply to all 12 PBC runtimes
- Verify EtrLock in construct_runtime! macro

### Phase 3: eth-pbc Special Fix (10 minutes)
- Resolve sp_io version conflict
- Align Frontier dependencies with Polkadot SDK stable2509

### Phase 4: Rebuild All (3-4 hours)
- Clean build artifacts
- Rebuild all 13 PBC collators in parallel if possible
- Verify binaries

---

## Prevention

### For Future PBC Development:

1. **Create PBC Template:** Include pallet-etr-lock by default
2. **Documentation:** Add to PBC development guide
3. **CI Check:** Add build test for all PBC runtimes
4. **Dependency Audit:** Run `cargo tree` to verify all trait bounds satisfied

---

## Timeline

- **Root Cause Analysis:** ✅ Complete (30 minutes)
- **Fix Implementation:** 🔄 In Progress (30-45 minutes)
- **Rebuild & Verify:** ⏳ Pending (3-4 hours)
- **Total Time:** ~4-5 hours from start to completion

---

## Next Steps

1. ✅ Root cause documented (this file)
2. 🔄 Create systematic fix script
3. ⏳ Apply fixes to all 12 PBC runtimes
4. ⏳ Resolve eth-pbc Wasm conflict
5. ⏳ Clean build and rebuild all 13 collators
6. ⏳ Verify all binaries created successfully
7. ⏳ Document prevention measures

---

**Conclusion:** This was a systematic configuration oversight during initial PBC development. The fix is straightforward but needs to be applied consistently across all 12 external bridge PBCs.
