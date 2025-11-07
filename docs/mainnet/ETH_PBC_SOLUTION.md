# eth-pbc-collator Build Solution
**Date:** November 4, 2025
**Status:** 🔬 Investigation & Solution Design

---

## Problem Statement

eth-pbc-collator fails to build due to Polkadot SDK version conflict:
- **Frontier EVM** (stable2506) - Latest available Frontier release
- **ASF Consensus** (stable2509) - Ëtrid's core consensus mechanism
- **Result:** Duplicate `sp_io` lang item (`panic_impl`) in Wasm build

## Why ASF Cannot Be Removed

ASF (Adaptive Stake-weighted Finality) is Ëtrid's core consensus mechanism and **must be preserved** across all PBCs for:
1. **Network uniformity** - All PBCs use the same consensus
2. **Consensus day compatibility** - Peer roles (validators, nominators, collators) depend on ASF
3. **Architecture integrity** - Removing ASF would break the multichain design

## Root Cause Analysis

```
eth-pbc-collator (needs stable2506 for Frontier)
    ├── eth-pbc-runtime (stable2506)
    │   └── Frontier pallets (stable2506) ✅
    │
    └── sc-consensus-asf (stable2509) ⚠️
        ├── sp-consensus-asf (stable2509) ⚠️
        └── Polkadot SDK deps (stable2509) ⚠️

CONFLICT: Two sp_io versions in same Wasm build
```

## Solution Options

### Option A: Version-Flexible ASF (RECOMMENDED)
Make ASF consensus modules support both stable2506 and stable2509 via Cargo features.

**Pros:**
- Maintains ASF consensus across all PBCs
- Clean, maintainable solution
- No architectural compromises

**Implementation:**
1. Add `sdk-stable2506` feature to `sc-consensus-asf`
2. Add `sdk-stable2506` feature to `sp-consensus-asf`
3. Use conditional compilation for version-specific code
4. eth-pbc-collator enables the feature flag

**Effort:** 2-4 hours

### Option B: Wait for Frontier stable2509
Wait for Frontier project to release stable2509-compatible version.

**Pros:**
- No code changes needed
- Official support

**Cons:**
- Timeline unknown (could be weeks/months)
- Blocks eth-pbc deployment

### Option C: Fork Frontier and Port to stable2509
Create Ëtrid-maintained Frontier fork targeting stable2509.

**Pros:**
- Full control over EVM stack
- Can add Ëtrid-specific optimizations

**Cons:**
- 2-4 weeks effort
- Ongoing maintenance burden
- Must track upstream changes

### Option D: Separate Cargo Workspace
Move eth-pbc to completely isolated workspace.

**Pros:**
- Complete isolation
- No version conflicts

**Cons:**
- Increases complexity
- Harder to maintain
- User previously rejected this approach

## Recommended Action

**Implement Option A** - Version-flexible ASF consensus modules.

This preserves Ëtrid's architecture while enabling eth-pbc to use Frontier stable2506.

---

## Current Status

- ✅ ASF consensus restored in eth-pbc-collator
- 🔬 Testing current build to confirm exact error
- ⏳ Awaiting build completion for error analysis

## Next Steps

1. Confirm Wasm duplicate lang item error
2. Implement `sdk-stable2506` feature in ASF modules
3. Update eth-pbc-collator to use feature flag
4. Test eth-pbc build with version-flexible ASF
5. Deploy eth-pbc to validators

---

**Note:** 11 other PBCs (btc, sol, xrp, bnb, trx, ada, matic, link, usdt, doge, xlm) build successfully and can be deployed immediately while eth-pbc solution is implemented.
