# CRITICAL: Network Fork Analysis
**Generated:** 2025-11-22
**Severity:** 🔴 **HIGH** - Multiple validators on separate forks

---

## Executive Summary

**Discovered:** 3 separate blockchain forks across 21 validators

- **Main Network (Correct):** 18 validators at block #17,360
- **Fork #1 (Val 13 & 14):** 2 validators at block #96,004
- **Fork #2 (Gizzi):** 1 validator at block #106,947

**Impact:**
- ASF consensus still functional (18/21 validators on main chain)
- 3 validators isolated and not participating in consensus
- Network security reduced (need 15+ signatures, have 18 available = 3 buffer)

**Root Cause:** Network partition - forked validators isolated from main network peers

---

## Fork Detection Evidence

### Block Hash Comparison

Compared block #100 and #1000 hashes across forks:

| Fork | Validators | Current Block | Block #100 Hash | Block #1000 Hash |
|------|-----------|---------------|-----------------|------------------|
| **Main (Correct)** | Val 10, 12, + 16 others | #17,360 | `0x731d3746...` | `0x3bf7202a...` |
| **Fork #1** | Val 13, 14 | #96,004 | `0xa89e7248...` | `0xc4ba1e5a...` |
| **Fork #2** | Gizzi (Dir 1) | #106,947 | `0x941def48...` | `0x5ce988d32...` |

**Conclusion:** Different block hashes prove these are separate chains, not sync delays.

---

## Complete Network Status (All 21 Validators)

### ✅ Main Network (18 validators)

| Validator | Role | Block | Peers | Status |
|-----------|------|-------|-------|--------|
| Dir 7 Economics | Director | #17,357 | 12 | ✅ At tip |
| Dir 8 Compiler | Director | #17,357 | 12 | ✅ At tip |
| Val 9 | ValidityNode | #17,357 | 12 | ✅ At tip |
| Val 10 | ValidityNode | #17,357 | 12 | ✅ At tip |
| Val 11 | ValidityNode | #17,357 | 13 | ✅ At tip |
| Val 12 | ValidityNode | #17,357 | 12 | ✅ At tip |
| Val 15 | ValidityNode | #17,357 | 5 | ✅ At tip |
| Val 16 | ValidityNode | #17,357 | 12 | ✅ At tip |
| Val 17 | ValidityNode | #17,357 | 12 | ✅ At tip |
| Val 18 | ValidityNode | #17,357 | 12 | ✅ At tip |
| Val 19 | ValidityNode | #17,357 | 13 | ✅ At tip |
| Val 20 | ValidityNode | #17,357 | 12 | ✅ At tip |
| Dir 0 Eoj | Director | #16,801 | 6 | 🔄 Syncing |
| Dir 2 Audit | Director | #16,801 | 5 | 🔄 Syncing |
| Dir 3 Security | Director | #16,801 | 5 | 🔄 Syncing |
| Dir 4 Governance | Director | #16,801 | 5 | 🔄 Syncing |
| Dir 5 Oracle | Director | #16,801 | 5 | 🔄 Syncing |
| Dir 6 Consensus | Director | #16,802 | 5 | 🔄 Syncing |

**Status:** 12 validators at tip, 6 syncing (normal, ~550 blocks behind)

---

### ❌ Fork #1: Val 13 & 14 (2 validators)

| Validator | Role | Block | Peers | Fork Hash |
|-----------|------|-------|-------|-----------|
| Val 13 | ValidityNode | #96,004 | 3 | `0xa89e7248...` |
| Val 14 | ValidityNode | #96,004 | 3 | `0xa89e7248...` |

**Analysis:**
- Both validators on same fork (matching hashes)
- Only 3 peers each (likely peering with each other + 1 external)
- Block #96,004 is ~78,600 blocks ahead of main network
- Suggests these validators forked early and diverged independently

**Tailscale Info:**
- Val 13: `100.74.84.28` / Public: `85.239.239.188`
- Val 14: `100.71.242.104` / Public: `154.12.250.18`

---

### ❌ Fork #2: Gizzi (1 validator)

| Validator | Role | Block | Peers | Fork Hash |
|-----------|------|-------|-------|-----------|
| Gizzi (Dir 1) | Director | #106,947 | 1 | `0x941def48...` |

**Analysis:**
- Isolated on own fork (only 1 peer)
- Block #106,947 is ~89,500 blocks ahead of main network
- Oldest fork - diverged very early
- Critical: Gizzi is a **Director** with governance powers

**Tailscale Info:**
- Gizzi: `100.96.84.69` / Public: `64.181.215.19`
- Also runs bootnode: `/ip4/64.181.215.19/tcp/30333/p2p/12D3KooW...`

---

## Impact Assessment

### Consensus Impact

**ASF Consensus Requirements:**
- Total validators: 21
- Required signatures: 15+ (71%+)
- Current on main network: 18 validators
- Buffer: 3 validators (18 - 15 = 3)

**Status:**
- ✅ Consensus functional (18 > 15)
- ⚠️ Reduced redundancy (only 3 buffer vs ideal 6)
- 🔴 If 4+ more validators fail, consensus breaks

### Governance Impact

**Director Voting:**
- Total directors: 9
- Quorum required: 6 out of 9
- Directors on main network: 8 (Gizzi forked)
- Directors syncing: 6
- Directors at tip: 2

**Status:**
- ✅ Quorum achievable (8 available > 6 required)
- ⚠️ Gizzi (CTO) isolated - missing governance participation
- ✅ Can pass emergency proposals with 6+ directors

### Network Security

**Risks:**
1. **Reduced Fault Tolerance:** Only 3 validator buffer before consensus failure
2. **Director Isolation:** Gizzi (CTO) not participating in governance
3. **Bootnode Concerns:** If Gizzi runs primary bootnode, new nodes may join wrong fork
4. **Fork Propagation:** New validators might sync to wrong fork if they connect to Val 13/14

---

## Root Cause Analysis

### Why Did These Forks Occur?

**Hypothesis 1: Network Partition**
- Forked validators lost connection to main network
- Continued producing blocks in isolation
- Built separate chain histories

**Hypothesis 2: Peer Discovery Failure**
- Validators not discovering main network peers via Tailscale
- Only connected to small subset of peers
- Insufficient peer diversity to find correct chain

**Hypothesis 3: Bootnode Issue**
- If Gizzi's bootnode advertises forked chain
- New validators may sync to wrong fork
- Explains why multiple validators forked separately

**Evidence:**
- Low peer counts on forked validators (1-3 peers vs 12-13 on main network)
- Different fork heights suggest independent divergences
- All forked validators show high block numbers (early fork, long isolation)

---

## Resolution Plan

### Phase 1: Fix Val 13 & Val 14 (URGENT)

**Priority:** HIGH - These validators can be fixed remotely if SSH access granted

**Steps:**
1. SSH to Val 13 (`100.74.84.28`)
2. Stop flarechain-validator service
3. Backup forked database
4. Purge `db/` and `network/` directories
5. Restart service (will resync from main network)
6. Repeat for Val 14 (`100.71.242.104`)

**Expected Time:** 10-30 minutes per validator

**Current Blocker:** ❌ No SSH access (permission denied)

---

### Phase 2: Fix Gizzi (CRITICAL)

**Priority:** CRITICAL - Director with governance powers + bootnode operator

**Steps:**
1. SSH to Gizzi (`100.96.84.69` or `64.181.215.19`)
2. Verify bootnode configuration (may need to update to prevent fork propagation)
3. Stop flarechain-validator service
4. Backup forked database
5. Purge `db/` and `network/` directories
6. Ensure correct bootnodes in config
7. Restart service

**Expected Time:** 10-30 minutes

**Current Blocker:** ❌ No SSH access (permission denied)

**Additional Concern:** If Gizzi's bootnode is advertising forked chain, need to:
- Temporarily disable bootnode
- Or update bootnode to advertise correct peers
- Verify bootnode peer discovery

---

### Phase 3: Verify Network Convergence

**After all fixes:**
1. Monitor all 21 validators for 1 hour
2. Verify all converge to same block height
3. Check block hashes match across all validators
4. Confirm peer counts are healthy (10+ peers each)
5. Verify ASF consensus has 20-21 signatures

---

## Manual Fix Instructions

### For Validator Operators

**Required for:**
- Val 13 (Tailscale: `100.74.84.28`)
- Val 14 (Tailscale: `100.71.242.104`)
- Gizzi (Tailscale: `100.96.84.69`)

**Commands:**

```bash
# SSH to validator
ssh root@<TAILSCALE_IP>

# Check which service is running
systemctl status flarechain-validator || systemctl status flarechain-node

# Stop the service
sudo systemctl stop flarechain-validator
# OR
sudo systemctl stop flarechain-node

# Find chain data directory
CHAIN_DATA=$(find /root /home -name "flarechain_mainnet" -type d 2>/dev/null | grep chains | head -1)
echo "Chain data: $CHAIN_DATA"

# Backup forked database (OPTIONAL - keeps fork data)
sudo mv $CHAIN_DATA/db $CHAIN_DATA/db.forked.backup.$(date +%Y%m%d_%H%M%S)

# Purge forked state (REQUIRED)
sudo rm -rf $CHAIN_DATA/db
sudo rm -rf $CHAIN_DATA/network

# Restart service (will resync from correct network)
sudo systemctl restart flarechain-validator
# OR
sudo systemctl restart flarechain-node

# Monitor sync progress (should start from block 0)
journalctl -u flarechain-validator -f --since "1 min ago"

# Verify syncing (in another terminal)
watch -n 10 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[],\"id\":1}" | jq ".result.number"'
```

**Expected Output:**
- Block starts at `0x00` (genesis)
- Rapidly syncs through blocks: `0x10`, `0x100`, `0x1000`, etc.
- Reaches main network height (~`0x43D0` = #17,360) in 10-30 minutes
- Peer count increases to 10-15 peers

**Success Criteria:**
- Block height matches main network (±10 blocks)
- Peer count: 10+ peers
- Block hash at #100 matches main network: `0x731d374643b4b75d93...`

---

## Monitoring Scripts

Created monitoring tools:

1. **`/tmp/check_all_21_validators.sh`**
   - Comprehensive status of all 21 validators
   - Shows block heights, peer counts, fork status

2. **`/tmp/verify_forks.sh`**
   - Compares block hashes to detect forks
   - Verifies validators are on same chain

3. **`/tmp/monitor_gizzi_resync.sh`**
   - Real-time monitoring of Gizzi's resync progress
   - Auto-updates every 10 seconds

4. **`/tmp/fix_val13_val14.sh`**
   - Automated fix script for Val 13 & 14
   - ❌ Blocked by SSH permission denied

---

## Recommendations

### Immediate Actions Required

1. **Grant SSH Access**
   - Add SSH public keys to all validator servers
   - Enables remote emergency fixes
   - Critical for production network operations

2. **Fix Val 13, Val 14, Gizzi**
   - Manual intervention required by server operators
   - Follow instructions in "Manual Fix Instructions" section
   - Priority: Val 13/14 first, then Gizzi

3. **Verify Bootnode Configuration**
   - Check if Gizzi's bootnode is advertising correct peers
   - May need to temporarily disable while Gizzi resyncs
   - Update bootnode list if necessary

### Short-term (Next 24 hours)

4. **Set up Telemetry (Task A2)**
   - Monitor all 21 validators continuously
   - Alert on peer count drops below 5
   - Detect forks within minutes, not hours

5. **Implement Health Checks**
   - Automated script checking all validators every 5 minutes
   - Compare block hashes to detect forks early
   - Alert operators on divergence

6. **Peer Diversity Requirements**
   - Configure minimum peer count (10+ peers)
   - Ensure validators connect to diverse peer set
   - Prevent isolation and fork formation

### Long-term

7. **Fork Detection & Auto-Recovery**
   - Automated fork detection via block hash comparison
   - Auto-restart forked validators with database purge
   - Requires SSH access automation

8. **Network Monitoring Dashboard**
   - Real-time view of all 21 validators
   - Visual fork detection (color-coded by block hash)
   - Historical tracking of peer counts and sync status

9. **Distributed Bootnode Architecture**
   - Multiple bootnodes on main network
   - Reduce single point of failure (Gizzi's bootnode)
   - Automatic bootnode health checks

---

## Next Steps

**Current Status:** Awaiting SSH access to fix forked validators

**Options:**

**Option A:** Wait for manual fixes by validator operators
- Slower resolution (depends on operator availability)
- Requires coordination with 3 separate operators
- Network continues with reduced redundancy

**Option B:** Grant SSH access for automated fixes
- Add SSH public key to all validator servers
- Enables immediate remote fixes (10-30 minutes)
- Enables future automation and emergency responses

**Option C:** Proceed with other tasks while network is operational
- Network still functional (18/21 validators on main chain)
- Set up telemetry to prevent future forks (Task A2)
- Return to fix validators when SSH access granted

---

**Recommendation:** Proceed with **Option C** (telemetry setup) while coordinating SSH access for validator fixes.

The network is currently operational and producing blocks. Setting up monitoring will prevent this issue from recurring and provide early warning of future forks.
