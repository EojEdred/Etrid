# eth-pbc-collator Build Issue Summary

**Date:** November 4, 2025
**Status:** ⚠️ Blocked - Requires Architecture Decision

---

## Issue

eth-pbc-collator fails to build with Wasm duplicate lang item error:

```
error[E0152]: duplicate lang item in crate `sp_io`: `panic_impl`
  = note: first definition in `sp_io` loaded from libsp_io-e2be6dc52d86ee79.rmeta
  = note: second definition in `sp_io` loaded from libsp_io-5243031e5c09cfd2.rmeta
```

## Root Cause

**Version Conflict:**
- **Frontier EVM Framework:** Uses polkadot-stable**2506**
- **Ëtrid Workspace:** Uses polkadot-stable**2509**
- **Result:** Two versions of `sp_io` crate in Wasm build, each defining `panic_impl` lang item (must be unique)

**Why eth-pbc is Special:**
- eth-pbc provides direct EVM compatibility via Frontier
- Frontier is ONLY available for stable2506 (frontier-stable2509 does not exist)
- Cannot upgrade Frontier to match workspace stable2509

---

## Attempted Solutions

### Option 1: Upgrade Frontier to stable2509 ❌ FAILED
**Attempt:** Upgrade all Frontier dependencies to frontier-stable2509
**Result:** Tag does not exist in Frontier repository
**Latest Available:** frontier-stable2506 (August 2024)

### Option 2: Downgrade Workspace to stable2506 ⛔ EXCLUDED
**Reason:** User explicitly excluded this option ("not in the equation")

### Option 3: Patch Resolution ❌ FAILED
**Attempt:** Force single sp_io version via `[patch."https://github.com/paritytech/polkadot-sdk.git"]`
**Result:** Cargo error - "patch must point to different sources"
**Issue:** Cannot patch GitHub dependency to same GitHub source

### Option 4: Isolate eth-pbc Workspace ⛔ EXCLUDED
**Reason:** User explicitly excluded this option ("not in the equation")

---

## Current Architecture

**eth-pbc runtime Cargo.toml** (lines 13-46):
```toml
# Frontier EVM Support (using frontier-stable2506)
pallet-evm = { git = "https://github.com/polkadot-evm/frontier", tag = "frontier-stable2506", ... }
pallet-ethereum = { git = "https://github.com/polkadot-evm/frontier", tag = "frontier-stable2506", ... }

# Substrate (using stable2506 for Frontier compatibility)
frame-support = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", ... }
sp-io = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", ... }
```

**Workspace Cargo.toml** (lines 237-247):
```toml
# All workspace members use polkadot-stable2509
frame-support = { ..., tag = "polkadot-stable2509" }
sp-io = { ..., tag = "polkadot-stable2509" }
```

**Conflict:** eth-pbc is in workspace but uses different Polkadot SDK version than workspace

---

## Why This Matters

eth-pbc is critical for EVM compatibility:
- Allows Ethereum smart contracts to run on Ëtrid
- Enables Ethereum wallet compatibility (MetaMask, etc.)
- Required for DeFi protocol bridging

---

## Possible Solutions (Require Architecture Decision)

### A. Wait for Frontier stable2509 Release
- **Timeline:** Unknown (could be weeks/months)
- **Risk:** Delays eth-pbc deployment indefinitely

### B. Fork Frontier and Port to stable2509
- **Effort:** High (2-4 weeks of work)
- **Risk:** Maintenance burden, potential bugs
- **Benefit:** Full control over EVM stack

### C. Deploy eth-pbc Without EVM Runtime
- **Approach:** Use Ethereum Light Client + Bridge instead of full EVM
- **Tradeoff:** Loses native smart contract execution
- **Benefit:** Can deploy immediately with bridge functionality

### D. Separate eth-pbc Workspace (Option 4)
- **Approach:** Move eth-pbc to isolated cargo workspace
- **Benefit:** Resolves version conflict completely
- **Note:** User previously excluded this ("not in the equation")

---

## Recommendation

**Short-term:** Deploy 11 working PBCs (btc, sol, xrp, bnb, trx, ada, matic, link, usdt, doge, xlm)
**Medium-term:** Re-evaluate Option D (separate workspace) or wait for frontier-stable2509
**Long-term:** Consider forking Frontier if EVM functionality is critical

---

## Files Modified

1. `/Users/macbook/Desktop/etrid/Cargo.toml` - Attempted patch (reverted)
2. `/Users/macbook/Desktop/etrid/docs/mainnet/build-11-pbcs.sh` - Build script excluding eth-pbc

---

## Related Documents

- `ROOT_CAUSE_ANALYSIS.md` - Analysis of pallet-etr-lock issue (resolved for 11 PBCs)
- `FIX_SUMMARY.md` - Summary of systematic fixes applied to 11 PBCs
- `CCTP_INTEGRATION_ARCHITECTURE.md` - CCTP integration documentation

---

**Last Updated:** November 4, 2025 11:10 AM CST
