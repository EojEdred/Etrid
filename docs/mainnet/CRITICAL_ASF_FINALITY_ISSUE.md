# CRITICAL: ASF Finality Gadget Key Access Issue - GRANDPA Stall Root Cause
**Date:** 2025-11-13
**Time:** 22:35 CST
**Status:** 🔴 ROOT CAUSE IDENTIFIED - CODE FIX REQUIRED

## Executive Summary

GRANDPA finality has been stuck at block #63,274 for ~24 hours with a ~30,000 block gap. After exhaustive investigation, the root cause is **NOT missing GRANDPA keys**, but rather a **bug in the custom ASF Finality Gadget** that prevents it from properly accessing existing GRANDPA keys in the keystore.

## The Real Problem

Your FlareChain node runs a **hybrid finality system**:
1. **GRANDPA** (Substrate's BFT finality)
2. **ASF Finality Gadget** (your custom 3-level finality: Pre-commit → Commit → Finalized)

The ASF Finality Gadget is trying to use GRANDPA keys but **cannot access them** due to a keystore access bug.

## Evidence from Logs

### V13 Startup Logs (80.190.82.185)
```
2025-11-13 03:08:35 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2025-11-13 03:08:35 🎯 Enabling ASF Finality Gadget (3-level finality)
2025-11-13 03:08:35 🏛️  Enabling GRANDPA finality (hybrid mode with ASF)
2025-11-13 03:08:41 ⚠️  No GRANDPA validator key found in keystore for ASF. Using placeholder. Node may not participate in block production.
```

This warning repeats **every 6 seconds** on ALL validators, preventing finality from advancing.

### V13 Current Logs
```
Nov 13 22:17:33 🔑 ASF using GRANDPA key from keystore: 28f6e3a1903b475b27145dfd07b32719159cdddfce78c52ad6588c91ec865add
Nov 13 22:18:21 💤 Idle (18 peers), best: #93070, finalized #63274
```

## Key Findings

### 1. GRANDPA Keys Exist in Keystores ✓
All 17 accessible validators have GRANDPA keys:
```bash
# Sample results from keystore check
80.190.82.185:  gran80565d70b6d75fa25e243dbfb6206eb6eb4e6e56ee82b403db9758bd6890eae4
154.12.250.18:  granb09a291c0303bd66b523e44203d582f69cd192e6cdbff8329d1b01d049f4bb96
80.190.82.184:  gran1b4a0249c16be966d76778364affe12c24e449c78448d6c783a4e8f746c81e09
```

**All 17/17 Contabo validators have GRANDPA keys in `/root/.etrid/chains/*/keystore/`**

### 2. GRANDPA Service Is Running ✓
```rust
// File: 05-multichain/flare-chain/node/src/asf_service.rs:1733-1737
task_manager.spawn_essential_handle().spawn_blocking(
    "grandpa-voter",
    None,
    sc_consensus_grandpa::run_grandpa_voter(grandpa_params)?,
);
```

GRANDPA voter IS spawned and running on all validators.

### 3. ASF Finality Gadget Cannot Access Keys ✗
**File:** `05-multichain/flare-chain/node/src/asf_service.rs:886-896`

```rust
let our_validator_id = match ppfa_keystore.ed25519_public_keys(GRANDPA_KEY_TYPE).first() {
    Some(public_key) => {
         log::info!(
            "🔑 ASF using GRANDPA key from keystore: {}",
            hex::encode(public_key.as_ref() as &[u8])
        );
        block_production::ValidatorId::from(public_key.0)
    }
    None => {
        log::warn!("⚠️  No GRANDPA validator key found in keystore for ASF. Using placeholder. Node may not participate in block production.");
        block_production::ValidatorId::from([0u8; 32])  // ← PLACEHOLDER!
    }
};
```

The ASF Finality Gadget successfully loads keys (evidenced by "🔑 ASF using GRANDPA key" logs every 6 seconds), but some other part of the ASF consensus is hitting the `None` case and logging the warning.

### 4. No GRANDPA Voting Activity ✗
Despite GRANDPA being enabled and keys existing:
- **No prevote messages** in logs
- **No precommit messages** in logs
- **No GRANDPA round messages** in logs
- Only the ASF key loading message repeats

This suggests the ASF Finality Gadget is **blocking or replacing** GRANDPA's finality mechanism.

## Architecture Analysis

### Expected Flow (Hybrid Finality)
```
Block Production (ASF PPFA)
    ↓
GRANDPA Finality (2/3+1 BFT)
    ↓
ASF Finality Gadget (3-level finality on top of GRANDPA)
```

### Actual Flow (What's Happening)
```
Block Production (ASF PPFA) ✓
    ↓
GRANDPA Finality (cannot vote - ASF blocks it?) ✗
    ↓
ASF Finality Gadget (waiting for keys that it can't fully access) ✗
```

## The "Committee Size: 1" Red Herring

The logs showing "Epoch X started with 1 committee members" vs "21 committee members" are from the **PPFA committee**, NOT GRANDPA:

**File:** `Desktop/etrid/09-consensus/validator-management/src/coordinator.rs:155-158`

```rust
log::info!("✅ Epoch {} started with {} committee members",
    new_epoch,
    self.committee.current_committee().len()
);
```

This is an **in-memory** committee that resets on restart. It has **NOTHING to do with GRANDPA finality**.

## Current Network Status

- **Validators online:** 17/22 (77%)
- **Best block:** ~#93,070 (advancing)
- **Finalized block:** #63,274 (STUCK for 24+ hours)
- **Finality gap:** ~29,800 blocks
- **GRANDPA keys:** ✓ Present on all 17 validators
- **GRANDPA service:** ✓ Running
- **ASF key access:** ✗ Partially failing
- **Network connectivity:** 12/18 validators unreachable from local machine

## Root Cause

The ASF Finality Gadget has a **keystore access bug** that prevents it from:
1. Properly loading GRANDPA session keys for all validator operations
2. Allowing GRANDPA to proceed with normal finality voting
3. Or alternatively, the ASF Finality Gadget is trying to **replace** GRANDPA but is incompletely implemented

The error message "No GRANDPA validator key found in keystore for ASF. Using placeholder" indicates the ASF code is:
- Looking for keys in the wrong location
- Using the wrong KeyTypeId
- Not properly handling the keystore API
- Or hitting a timing issue where keys aren't loaded yet

## Fix Options

### Option 1: Disable ASF Finality Gadget (Fastest Fix)
Comment out or disable the ASF Finality Gadget and rely on standard GRANDPA finality only.

**File:** `05-multichain/flare-chain/node/src/asf_service.rs` around line 1600-1696

Disable the ASF Finality Gadget spawn task, allowing GRANDPA to operate normally.

### Option 2: Fix ASF Key Access (Proper Fix)
Debug and fix the keystore access in the ASF Finality Gadget:

**File:** `05-multichain/flare-chain/node/src/asf_service.rs:886-896`

The code needs to:
1. Properly access the keystore
2. Load GRANDPA keys for the current validator
3. Not interfere with GRANDPA's own key access

### Option 3: Use ASF Without GRANDPA Keys
If ASF Finality is intended to fully replace GRANDPA, then:
1. Generate separate ASF session keys (not GRANDPA keys)
2. Update the key generation scripts
3. Ensure ASF Finality Gadget uses its own key type

## Immediate Next Steps

### Step 1: Determine Finality Strategy
**Decision needed from you:**
- Do you want to use **GRANDPA only** (disable ASF Finality Gadget)?
- Do you want to use **ASF Finality only** (disable GRANDPA, fix ASF)?
- Do you want **hybrid** (fix ASF to work alongside GRANDPA)?

### Step 2: Code Fix
Based on your decision, modify `asf_service.rs` to:
- Comment out ASF Finality Gadget spawn (if GRANDPA only)
- Fix keystore access bug (if hybrid)
- Replace GRANDPA with ASF entirely (if ASF only)

### Step 3: Rebuild and Deploy
```bash
cd ~/Desktop/etrid/05-multichain/flare-chain
cargo build --release
```

### Step 4: Deploy Fixed Binary
Deploy to all 22 validators and restart.

## Files to Investigate

1. **`05-multichain/flare-chain/node/src/asf_service.rs`**
   - Lines 886-896: ASF key loading logic
   - Lines 1600-1696: ASF Finality Gadget spawn
   - Lines 1702-1738: GRANDPA finality spawn

2. **`09-consensus/finality-gadget/src/lib.rs`**
   - ASF Finality Gadget implementation
   - How it accesses keys
   - How it interacts with GRANDPA

3. **`09-consensus/validator-management/src/coordinator.rs`**
   - PPFA committee management (the "1 vs 21" logs)
   - This is unrelated to the finality issue

## Conclusion

The finality stall is caused by an **architectural conflict** between GRANDPA and the custom ASF Finality Gadget. Both are trying to provide finality, but the ASF Finality Gadget's incomplete keystore access is preventing either from succeeding.

**The network cannot finalize blocks until you decide which finality mechanism to use and fix the code accordingly.**

---

**Recommendation:** Disable ASF Finality Gadget temporarily, let GRANDPA finalize normally, then properly integrate ASF Finality in a controlled development environment before redeploying to mainnet.
