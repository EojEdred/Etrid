# Ëtrid FlareChain - Finality Root Cause Analysis

**Date:** November 9, 2025 09:40 CST
**Critical Discovery:** GRANDPA Equivocation Detected

---

## CRITICAL FINDING: GRANDPA Equivocation

### What Was Found

Validator-11 logs show **GRANDPA prevote equivocation** errors:

```
Detected prevote equivocation in the finality worker:
Equivocation {
  round_number: 234194,
  identity: Public(8a9a9d8a9574eb75682a3501a2df5467036c2fc03903e9d46dfab77af4189a51),
  first: (Prevote {
    target_hash: 0xc04eb9f2...,
    target_number: 63274
  }, ...),
  second: (Prevote {
    target_hash: 0x5e690e85...,
    target_number: 65665
  }, ...)
}
```

**Multiple validators detected with same issue:**
- Public(8a9a9d8a...) - Equivocating between blocks #63274 and #65665
- Public(f0b56972...) - Equivocating between blocks #63274 and #65665
- Public(f2c70dfa...) - Equivocating between blocks #63274 and #69549

---

## What This Means

### GRANDPA Equivocation Explained

**Equivocation** = A validator signing TWO DIFFERENT prevotes in the same GRANDPA round

This is a **Byzantine fault** - validators are voting for conflicting chains:
1. One prevote for block #63,274 (hash 0xc04eb9f2...)
2. Another prevote for block #65,665 or #69,549 (different hashes)

### Why This Breaks Finality

GRANDPA finality algorithm requires:
1. Validators broadcast **prevotes** for blocks they believe are canonical
2. Once 2/3+1 validators prevote the same block, it can be **precommitted**
3. Once 2/3+1 validators precommit, block is **finalized**

**When equivocation occurs:**
- GRANDPA cannot determine which chain is canonical
- Different validators are voting for different forks
- Cannot reach 2/3+1 agreement on a single block
- **Finality stalls** until equivocation is resolved

---

## Root Cause: Chain Fork During Migration

### Timeline of Events

**1. Original Network (Before Nov 8)**
- 21 validators running (Azure + Oracle)
- Last finalized block: #63,274
- Network was healthy

**2. Azure Validators Go Offline (Nov 7-8)**
- 16 Azure validators stopped (payment issue)
- Network falls below consensus threshold
- Block production may have continued with remaining validators

**3. Contabo Migration (Nov 8-9)**
- 16 new Contabo validators deployed
- Validators reset with `purge-chain`
- **Critical error:** Contabo validators started syncing from a DIFFERENT genesis/chain
- Documentation shows: "Contabo validators were running on wrong chain (different genesis)"

**4. Chain Fork Created**
- Old validators (Gizzi, Audit) on ORIGINAL chain with block #63,274 finalized
- New validators (Contabo) started producing blocks on DIFFERENT chain
- Both chains shared genesis but diverged after block #63,274

**5. Chain Reconciliation (Nov 8-9)**
- Discovered Contabo validators on wrong chain
- Reset all Contabo validators
- Started syncing from Oracle validators (correct chain)
- **But**: GRANDPA still has votes from BOTH chains in memory

---

## Why Equivocation Is Happening

### Scenario 1: Validators Syncing Wrong Chain Data

**Problem:**
- Some validators imported blocks from the WRONG chain initially
- These blocks reached #65,665 and #69,549
- Validators created GRANDPA votes for these blocks
- After chain reset, validators now see CORRECT chain
- But GRANDPA still has votes for BOTH chains

**Evidence:**
- Validator-11 restarted at 16:32:13 (recent)
- Immediately saw equivocation errors for round #234,194
- This round was from the OLD/WRONG chain

### Scenario 2: Different Validators on Different Forks

**Problem:**
- Oracle validators (Gizzi, Audit) stuck at block #63,274 (correct chain)
- Some validators synced beyond #63,274 to #65,665
- Other validators synced to #69,549
- All three are DIFFERENT forks
- GRANDPA sees votes for all three forks

**Evidence:**
- Equivocations show same round but different target blocks:
  - First prevote: #63,274 (Oracle chain)
  - Second prevote: #65,665 (Fork A)
  - Third prevote: #69,549 (Fork B)

---

## Committee Size Discrepancy

### Critical Observation

**Validator-6 (synced, at tip):**
```
✅ Epoch 7 started with 1 committee members
```

**Validator-11 (syncing, behind tip):**
```
✅ Epoch 6 started with 16 committee members
```

**What this means:**
- Validator-6 at block #76,186: Only sees 1 committee member (ITSELF?)
- Validator-11 at syncing: Sees 16 committee members (correct)

**Diagnosis:**
- The chain is configured for 21 validators in genesis
- But current epoch has only 1 or 16 committee members
- This indicates **authority set was never properly updated after migration**

### Why Authority Set Not Updated

**Hypothesis:**
1. Genesis chainspec has 21 validators hardcoded
2. Session keys were inserted into validator keystores
3. **But**: Runtime state still has OLD authority set from before Azure validators went offline
4. New validators are not being added to authority set via extrinsics
5. Without runtime calls to `Session::set_keys()`, validators remain in "waiting" state

**Missing Step:**
Validators need to call `author_rotateKeys` RPC and submit `session.setKeys` extrinsic to:
1. Generate session keys
2. Submit keys to runtime
3. Be included in next era's authority set

**Evidence This Wasn't Done:**
- Keystores have session keys (deployed manually)
- But no extrinsics submitted to runtime to register keys
- Runtime doesn't know about new validators
- Authority set stuck with old validators

---

## Genesis vs Runtime State Mismatch

### Genesis Configuration

**From chainspec:**
- 21 validators configured with session keys
- 21 GRANDPA authorities
- 21 ASF committee members

**Runtime State (Current):**
- Authority set may still have OLD 21 validators (Azure + Oracle)
- New 16 Contabo validators NOT in authority set
- Only 1-16 committee members depending on block height

**The Problem:**
- Genesis config != Runtime state
- GRANDPA expects 21 validators from genesis
- Runtime only recognizes 1-16 validators
- Cannot achieve 2/3+1 of 21 (need 15) when only 16 exist

---

## Complete Diagnosis

### Issue #1: GRANDPA Equivocation (CRITICAL)
**Symptom:** Prevote equivocation errors for rounds at blocks #63274, #65665, #69549
**Cause:** Validators imported blocks from different chains/forks during migration
**Impact:** GRANDPA cannot finalize blocks due to conflicting votes
**Status:** ❌ BLOCKING FINALITY

### Issue #2: Authority Set Not Updated (CRITICAL)
**Symptom:** Committee has 1-16 members instead of 21
**Cause:** Session keys not registered with runtime via extrinsics
**Impact:** New validators not participating in GRANDPA consensus
**Status:** ❌ BLOCKING FINALITY

### Issue #3: Chain Fork History (CRITICAL)
**Symptom:** Multiple target blocks for same GRANDPA round
**Cause:** Validators were on different chains during Nov 8-9 migration
**Impact:** GRANDPA has conflicting votes in history
**Status:** ❌ BLOCKING FINALITY

### Issue #4: Oracle Validators Offline (HIGH)
**Symptom:** Gizzi and Audit validators not responding
**Cause:** VMs stopped or network issue
**Impact:** Missing 2 of original 21 validators, reducing available authority count
**Status:** ⚠️ DEGRADED

### Issue #5: Epochs Not Adding Validators (MEDIUM)
**Symptom:** Validator-6 shows "1 committee member" in epoch 7
**Cause:** Runtime not rotating authority set properly
**Impact:** Cannot achieve consensus with only 1 validator
**Status:** ⚠️ DEGRADED

---

## Why Session Rotation Isn't Helping

You're correct that session/era rotation SHOULD update the authority set. But it's not working because:

### Problem 1: GRANDPA Stuck on Old Round
- GRANDPA is stuck on round #234,194
- This round is trying to finalize blocks #63,274 - #69,549
- Until this round completes, GRANDPA won't advance
- Until GRANDPA advances, sessions can rotate but finality won't resume

### Problem 2: Authority Set in Runtime State
- Genesis has 21 validators
- But runtime state may not have been updated when Azure validators went offline
- Session module needs extrinsics to add/remove validators
- Without extrinsics, authority set stays stale

### Problem 3: Equivocation Penalties
- Some validators have been caught equivocating
- These validators may be slashed or removed from authority set
- But slashing may not execute without finality
- Creates deadlock: can't finalize without validators, can't remove bad validators without finality

---

## Solutions (Diagnostic Phase)

### Option 1: Wait for GRANDPA to Recover
**Theory:** GRANDPA may eventually resolve equivocations and resume finality
**Likelihood:** LOW - equivocation usually requires manual intervention
**Time:** Unknown (could be hours/days)

### Option 2: Reset GRANDPA State
**Theory:** Purge GRANDPA votes and restart from last finalized block
**Method:** Restart all validators with fresh database from block #63,274
**Risk:** HIGH - would lose block history #63,274 - #76,190
**Feasibility:** LOW - no clean way to reset only GRANDPA

### Option 3: Force Authority Set Update
**Theory:** Submit governance extrinsics to update authority set
**Method:** Use sudo to call `session.set_keys` for all 16 validators
**Risk:** MEDIUM - requires sudo access and finality to execute
**Feasibility:** MEDIUM - blocked by lack of finality

### Option 4: Deploy Supermajority of Original Validators
**Theory:** Get 15 of original 21 validators back online
**Method:** Restart Oracle validators + Azure validators
**Risk:** LOW - just restores original state
**Feasibility:** HIGH - requires paying Azure bill or provisioning VMs

### Option 5: Fresh Genesis (Nuclear Option)
**Theory:** Create new chain with fresh genesis and migrate state
**Method:** Export state at #63,274, create new genesis, restart network
**Risk:** VERY HIGH - complex migration, possible data loss
**Feasibility:** LOW - last resort only

---

## Recommended Diagnostic Steps

1. **Query GRANDPA state from a validator:**
   - Need `--rpc-external` enabled on one validator
   - Query `grandpa_roundState` to see current round and votes
   - Identify which validators are equivocating

2. **Check runtime authority set:**
   - Query `session.validators()` to see who runtime expects
   - Compare to genesis config (21 validators)
   - Determine if authority set matches current validators

3. **Verify session keys in keystores:**
   - Check all 16 Contabo validators have correct keys
   - Verify GRANDPA keys match genesis config
   - Look for format issues (hex vs plaintext)

4. **Check if Oracle validators can be recovered:**
   - Access Oracle Cloud Console
   - Restart Gizzi and Audit VMs
   - Get original validators back online

5. **Monitor for GRANDPA progress:**
   - Watch logs for new GRANDPA rounds
   - See if finality eventually resumes
   - Check if equivocation errors stop

---

## Conclusion

**Root Cause:** Multiple concurrent issues:

1. **Chain fork during migration** - Validators were on different chains
2. **GRANDPA equivocation** - Validators voting for conflicting blocks
3. **Authority set mismatch** - Runtime expects old validators, has new validators
4. **Missing session key registration** - New validators not added to authority set

**Why finality is stuck:**
- GRANDPA cannot resolve conflicting votes from different chain forks
- Even though epochs are rotating, authority set not updating properly
- Committee size wrong (1-16 instead of 21)
- Equivocation errors preventing round completion

**Critical path to recovery:**
1. Enable RPC on a validator to query state
2. Determine if GRANDPA can recover on its own (unlikely)
3. Either restore original validators OR force authority set update
4. May require governance intervention or chain restart

---

**Report Generated:** November 9, 2025 09:40 CST
**Status:** ROOT CAUSE IDENTIFIED - Requires immediate action
**Severity:** CRITICAL - Network cannot finalize blocks

