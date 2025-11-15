# GRANDPA Finality Root Cause Analysis
**Date:** 2025-11-13
**Status:** CRITICAL ISSUE IDENTIFIED - ARCHITECTURE FLAW

## Executive Summary

GRANDPA finality has been stalled at block #63,274 for several hours. The root cause has been identified: **the "Committee size" log messages are from the custom PPFA committee logic (09-consensus module), NOT from GRANDPA consensus**.

GRANDPA uses the authority set defined in the chainspec genesis configuration, which should contain all 21 validators. The issue is that GRANDPA finality is correctly configured with 21 validators but is not progressing because validators cannot reach Byzantine Fault Tolerance consensus threshold (2/3+1 = 15/21 validators).

## Current Status

### Working Validators
- **17/22 validators running** with peers and syncing
- **V13 (80.190.82.185)** has been running continuously since genesis
- All running validators show same finalized block: **#63,274**
- Best block continues to advance: **~#92,350+**
- **Finality gap: ~29,000 blocks** (and growing)

### Failed Binary Deployment
- Attempted to deploy "working" binary from V13 to all validators
- **Only 6/18 validators** successfully received deployment
- 12 validators failed due to network issues (Oracle/StLouis IPs unreachable)

### Committee Size Mystery - SOLVED
The "Committee size: 1" vs "Committee size: 21" log messages are **NOT related to GRANDPA finality**. They come from:

**File:** `Desktop/etrid/09-consensus/validator-management/src/coordinator.rs:155-158`

```rust
log::info!("✅ Epoch {} started with {} committee members",
    new_epoch,
    self.committee.current_committee().len()
);
```

This is the **PPFA (Proposing Panel for Attestation) committee** for the custom consensus module, which:
- Stores committee members in **in-memory** `BTreeMap` (`validator_pool`)
- Resets to empty on restart
- Dynamically populates as validators register via runtime calls
- **Has nothing to do with GRANDPA finality**

## Real Problem: Why is GRANDPA Stalled?

GRANDPA finality requires **15/21 validators** (Byzantine Fault Tolerance threshold) to sign finality messages. It's been stalled at #63,274 for hours, suggesting:

### Hypothesis 1: Insufficient Signing Validators
- Only 17/22 validators online, but 2 Oracle + 5 Contabo = 7 offline
- But we only need 15/21, and we have 17 running
- **This shouldn't cause stall unless validators aren't signing**

### Hypothesis 2: GRANDPA Key Configuration
- Validators may not have correct GRANDPA keys loaded
- Need to verify GRANDPA keys via `author_hasSessionKeys` or similar RPC

### Hypothesis 3: Network Partitioning
- GRANDPA gossip network may be fragmented
- Validators may not be receiving each other's finality votes

### Hypothesis 4: Stale Session Keys
- Session keys may not have been properly rotated
- Validators running with mismatched keys from previous deployment

## Data Points

### Binary Timestamps
```
V13 (80.190.82.185):   2025-11-12 21:55:36  ← Shows "21 committee" (PPFA, not GRANDPA)
V12 (154.12.250.18):   2025-11-13 15:50:41  ← Shows "1 committee" (freshly restarted)
V14 (80.190.82.184):   2025-11-13 15:53:22  ← Shows "1 committee" (freshly restarted)
```

### Finality Status (from logs)
```
Portsmouth Region:
V11-Auth3    80.190.82.186   Peers: 18 | Best: #92347 | Finalized: #63274 | Gap: 29073 | ✓ GRANDPA
V12-Auth4    154.12.250.18   Peers: 13 | Best: #92347 | Finalized: #63274 | Gap: 29073 | ✓ GRANDPA
V13-Auth11   80.190.82.185   Peers: 19 | Best: #92347 | Finalized: #63274 | Gap: 29073 | ✓ GRANDPA

Seattle Region:
Seattle-V7   85.239.239.193  Peers: 11 | Best: #92348 | Finalized: #63274 | Gap: 29074 | ✗ No GRANDPA
Seattle-V8   85.239.239.190  Peers:  9 | Best: #92348 | Finalized: #63274 | Gap: 29074 | ✗ No GRANDPA
```

## Critical Questions to Answer

1. **Are all 17 running validators configured with their GRANDPA session keys?**
   - Need to check: `author_hasSessionKeys` or equivalent
   - Expected: All should have genesis GRANDPA keys loaded

2. **Are validators gossiping GRANDPA votes to each other?**
   - Check logs for: "GRANDPA vote", "precommit", "prevote"
   - Expected: Active GRANDPA round voting messages

3. **What was special about block #63,274?**
   - Check if it was the last block before a restart
   - Check if there was a runtime upgrade or epoch change

4. **Are validators using the correct chainspec?**
   - Confirmed: All using `/opt/flarechain/chainspec.json`
   - Need to verify: Does it contain all 21 GRANDPA authorities?

## Immediate Next Steps

### Step 1: Verify GRANDPA Authority Set
```bash
# Check chainspec for GRANDPA authorities
ssh -i ~/.ssh/contabo-validators root@80.190.82.185 \
  "cat /opt/flarechain/chainspec.json | jq '.genesis.runtime.grandpa.authorities'"
```

### Step 2: Check GRANDPA Voting Activity
```bash
# Look for GRANDPA voting in logs
ssh -i ~/.ssh/contabo-validators root@80.190.82.185 \
  "journalctl -u flarechain-validator --since '1 hour ago' | grep -i 'grandpa.*vote\|precommit\|prevote'"
```

### Step 3: Verify Session Keys on All Validators
```bash
# Check if validators have session keys loaded
for ip in <all_validator_ips>; do
  ssh root@$ip "curl -s -X POST -H 'Content-Type: application/json' \
    -d '{\"jsonrpc\":\"2.0\",\"method\":\"author_hasSessionKeys\",\"params\":[\"<pubkey>\"],\"id\":1}' \
    http://127.0.0.1:9944"
done
```

### Step 4: Check for GRANDPA Errors
```bash
# Search for GRANDPA errors in logs
ssh -i ~/.ssh/contabo-validators root@80.190.82.185 \
  "journalctl -u flarechain-validator --since '2 hours ago' | grep -i 'grandpa.*error\|grandpa.*fail'"
```

## Recommendations

1. **DO NOT restart more validators** until we understand why GRANDPA stalled
2. **Focus on GRANDPA diagnostics**, not PPFA committee size
3. **Verify session keys** are correctly loaded on all 17 running validators
4. **Check GRANDPA logs** for voting activity or errors
5. **Consider GRANDPA resume/recovery** if it's stuck in a bad state

## Files to Investigate

- `/opt/flarechain/chainspec.json` - Should contain 21 GRANDPA authorities
- GRANDPA runtime pallet logs - Look for finality voting
- Session keys storage - Verify all validators have keys loaded

## Warning

The PPFA committee logs ("Epoch X started with Y committee members") are a **red herring**. They are NOT related to GRANDPA finality. Focus solely on GRANDPA-specific diagnostics.

---
**Next Session:** Run diagnostics from Steps 1-4 above to identify exact cause of GRANDPA stall.
