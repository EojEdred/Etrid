# Ëtrid FlareChain - Committee Analysis & Integration Plan

**Date:** 2025-11-03
**Status:** Azure VMs Running but NOT in Active Committee
**Network State:** 16 validators (6-21) producing blocks at #6,941+

---

## Executive Summary

**Problem:** 3 Azure VMs (validators 2, 3, 4) are running with session keys installed but are NOT part of the 16-member active committee that is producing and finalizing blocks.

**Active Committee:** Validators 6-21 (16 members, 273 total peers, all synced)

**Key Question:** Why were validators 2-4 excluded from the active set when validators 6-21 were selected?

---

## Section 1: Current State Analysis

### Network Architecture

```
┌─────────────────────────────────────────────────────────┐
│  RUNNING BUT NOT VALIDATING                             │
├─────────────────────────────────────────────────────────┤
│  Validator 2 - VM1 (20.69.26.209)                       │
│    Peer ID: 12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumN... │
│    Status: Connected (12 peers), session keys installed │
│    Block: #6,941 (synced)                               │
│                                                          │
│  Validator 3 - VM2 (20.186.91.207)                      │
│    Peer ID: 12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj... │
│    Status: Connected (13 peers), session keys installed │
│    Block: #6,941 (synced)                               │
│                                                          │
│  Validator 4 - VM3 (52.252.142.146)                     │
│    Peer ID: 12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kr... │
│    Status: Connected (13 peers), session keys installed │
│    Block: #6,941 (synced)                               │
└─────────────────────────────────────────────────────────┘
                         ↕ Network Gossip
┌─────────────────────────────────────────────────────────┐
│  ACTIVE COMMITTEE (PRODUCING & FINALIZING BLOCKS)       │
├─────────────────────────────────────────────────────────┤
│  Validators 6-21 (16 members)                           │
│    Status: Producing blocks, GRANDPA finality active    │
│    Block: #6,941 (2-block finality lag)                 │
│    Peer IDs: UNKNOWN (no SSH access)                    │
│    Total Network Peers: 273                             │
└─────────────────────────────────────────────────────────┘
```

### What We Know

1. **Azure VMs are network-healthy:**
   - Connected to network (12-13 peers each)
   - Fully synced to block #6,941
   - Session keys installed and verified
   - Each has unique AURA, GRANDPA, and ASFK keys

2. **Active committee is validators 6-21:**
   - 16 validators producing blocks
   - GRANDPA finality working (2-block lag)
   - Network has been stable for 9+ hours
   - Total network: 273 peers

3. **Azure VMs have different identities:**
   - Each has unique peer ID (12D3Koo...)
   - Each has unique session keys in keystore
   - Not the same validator repeated 3 times

### What We Don't Know

1. **Committee selection mechanism:**
   - How were validators 6-21 chosen?
   - Is it genesis-defined, stake-based, or algorithmic?
   - When does validator set rotation occur?

2. **Peer IDs of active committee:**
   - Cannot SSH to validators 6-21
   - Don't know their peer IDs to map structure
   - Cannot verify network topology

3. **Why Azure VMs excluded:**
   - Are validators 2-4 intended to be inactive?
   - Are they waiting for epoch change?
   - Is there an on-chain registration required?

---

## Section 2: Root Cause Analysis

### Hypothesis 1: Genesis Configuration Order (MOST LIKELY)

**Theory:** The genesis chainspec defines exactly which validator positions are in the initial active set.

**Evidence:**
- Active committee is precisely validators 6-21 (16 consecutive positions)
- Validators 2-4 are excluded (3 consecutive positions)
- This pattern suggests explicit configuration, not random selection

**Test Method:**
```bash
# Examine genesis chainspec
grep -A 50 "validators\|session\|authorities" \
  ~/Desktop/etrid/runtime/flare-chain/chainspec-raw.json

# Look for:
# - "session.keys" array (defines validator set)
# - "aura.authorities" (defines block producers)
# - "grandpa.authorities" (defines finality voters)
```

**Expected Finding:** Genesis likely configured validators 6-21 as initial authorities, omitting validators 2-4.

**Why This Matters:** If genesis defines initial set, validators 2-4 need to be ADDED via governance or staking.

---

### Hypothesis 2: Session Keys Not Registered On-Chain (LIKELY)

**Theory:** While session keys are installed in keystores, they may not be registered in the runtime's session storage.

**Evidence:**
- Azure VMs show in network but don't produce blocks
- Active committee members must have keys registered in `session.nextKeys`
- On Substrate, validators must call `session.setKeys()` extrinsic

**Test Method:**
```bash
# Query on-chain session keys for validators 2-4
# This requires RPC access to any validator (Azure VMs work)

# Check if validator 2's keys are registered:
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"state_getStorage",
    "params":["0x<hash_of_session_nextKeys(validator_2_account)>"],
    "id":1
  }' \
  http://20.69.26.209:9944

# Or use polkadot.js API:
# api.query.session.nextKeys(<validator_2_account>)
```

**Expected Finding:** Session keys for validators 2-4 may return `null`, indicating they're not registered.

**Why This Matters:** Would require submitting `session.setKeys(keys, proof)` extrinsic to register.

---

### Hypothesis 3: Validator Positions Are Index-Based (POSSIBLE)

**Theory:** FlareChain ASF consensus selects validators by index position, and positions 2-4 are reserved for special roles.

**Evidence:**
- All validators show "PPFA index: 16" in Azure VM logs
- PPFA (Probabilistic Probabilistic Finality Algorithm) uses index-based slot assignment
- Validators 6-21 may occupy PPFA indices 0-15

**Test Method:**
```bash
# Check genesis configuration for ASF/PPFA settings
grep -A 20 "asf\|ppfa\|consensus" \
  ~/Desktop/etrid/runtime/flare-chain/chainspec-raw.json

# Look for:
# - "ppfa.validatorIndices"
# - "asf.committee"
# - Index to validator mapping
```

**Expected Finding:** Genesis may assign specific PPFA indices to validators 6-21.

**Why This Matters:** Would need to reconfigure genesis OR wait for runtime upgrade to add validators 2-4.

---

### Hypothesis 4: Staking/Bonding Required (LESS LIKELY)

**Theory:** Validators must stake/bond tokens before joining active set.

**Evidence:**
- Many Substrate chains require `staking.bond()` + `staking.validate()`
- Validators 6-21 may have pre-bonded stake in genesis
- Azure VMs may need to bond tokens on-chain

**Test Method:**
```bash
# Query staking status for validator accounts
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"state_call",
    "params":["StakingApi_nominators", "0x"],
    "id":1
  }' \
  http://20.69.26.209:9944

# Or check if staking pallet exists:
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"state_getMetadata",
    "params":[],
    "id":1
  }' \
  http://20.69.26.209:9944 | jq '.result' | grep -i staking
```

**Expected Finding:** May find that validators 6-21 have bonded stake, validators 2-4 do not.

**Why This Matters:** Would need to fund accounts and call staking extrinsics.

---

## Section 3: Strategic Plan

### Phase 1: Information Gathering (No SSH Required)

**Objective:** Understand committee selection mechanism and validator registration requirements.

#### Task 1.1: Analyze Genesis Chainspec
```bash
# Location of chainspec
CHAINSPEC="/Users/macbook/Desktop/etrid/runtime/flare-chain/chainspec-raw.json"

# Extract validator configuration
jq '.genesis.runtime' "$CHAINSPEC" | grep -A 100 "session\|aura\|grandpa\|asf"

# Key questions to answer:
# 1. Which accounts are in "session.keys"?
# 2. Are there exactly 16 authorities configured?
# 3. Do validator indices 2-4 appear anywhere?
```

**Expected Output:** List of genesis authorities with their session keys.

**Decision Point:**
- If validators 2-4 are NOT in genesis → Need on-chain registration (go to Phase 2A)
- If validators 2-4 ARE in genesis → Session keys mismatch (go to Phase 2B)

---

#### Task 1.2: Query On-Chain Validator Set via RPC
```bash
# Use Azure VM RPC endpoint to query runtime state
VM1_RPC="http://20.69.26.209:9944"

# Get current session validators
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"state_call",
    "params":["SessionApi_validators", "0x"],
    "id":1
  }' \
  $VM1_RPC | jq

# Get session keys for each validator
# (requires knowing validator account IDs from genesis)
```

**Expected Output:** List of active validator accounts in current session.

**Decision Point:**
- If 16 validators returned → Confirm these are validators 6-21
- If validators 2-4 accounts NOT in list → They need to be added (go to Phase 2A)

---

#### Task 1.3: Check Runtime Metadata for Required Pallets
```bash
# Determine what pallets/extrinsics are available
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"state_getMetadata",
    "params":[],
    "id":1
  }' \
  http://20.69.26.209:9944 > runtime_metadata.json

# Check for key pallets:
grep -i "session\|staking\|asf\|committee" runtime_metadata.json

# Key questions:
# 1. Is "session.setKeys" available?
# 2. Is there a "staking" pallet?
# 3. Is there an "asf" or "committee" pallet with registration methods?
```

**Expected Output:** List of available pallets and extrinsics.

**Decision Point:** Determines what actions are possible (set keys, bond stake, join committee, etc.)

---

#### Task 1.4: Examine Validator Logs for Committee Messages
```bash
# Check Azure VM logs for any committee/session messages
# These VMs ARE accessible via SSH

ssh -i ~/.ssh/gizzi-validator ubuntu@20.69.26.209 \
  "sudo journalctl -u flarechain-validator --since '9 hours ago' | \
   grep -i 'session\|committee\|validator\|authority\|grandpa' | \
   head -100"

# Look for:
# - "Not a validator" messages
# - "Waiting for next session" messages
# - "Session keys not found" errors
# - Any authority/committee election logs
```

**Expected Output:** Clues about why these nodes aren't validating.

**Decision Point:** May reveal explicit error messages guiding next steps.

---

### Phase 2A: On-Chain Registration (If Validators 2-4 Not in Genesis)

**Objective:** Register Azure VM validators to join active set.

#### Option 2A.1: Session Key Registration
```bash
# If runtime has session.setKeys extrinsic:

# 1. Extract public keys from keystores
AURA_KEY=$(cat ~/.etrid/validator/chains/flarechain_mainnet/keystore/6175726166... | jq -r '.publicKey')
GRANDPA_KEY=$(cat ~/.etrid/validator/chains/flarechain_mainnet/keystore/6772616e0a... | jq -r '.publicKey')
ASFK_KEY=$(cat ~/.etrid/validator/chains/flarechain_mainnet/keystore/6173666b66... | jq -r '.publicKey')

# 2. Submit setKeys extrinsic (requires validator account with funds)
# Using polkadot.js/api or subxt:

# api.tx.session.setKeys(
#   {
#     aura: AURA_KEY,
#     grandpa: GRANDPA_KEY,
#     asfk: ASFK_KEY
#   },
#   proof
# ).signAndSend(validatorAccount)

# 3. Wait for next session (epoch change)
# Session changes typically occur every N blocks (check session.period)
```

**Success Criteria:** Validators 2-4 appear in `session.nextKeys` storage.

**Timeline:** Takes effect in NEXT session (may be hours or days depending on epoch length).

---

#### Option 2A.2: Staking Bond + Validate
```bash
# If runtime requires staking:

# 1. Fund validator accounts with ETR tokens
# 2. Bond tokens:
# api.tx.staking.bond(controller, amount, payee).signAndSend(stash)

# 3. Set session keys:
# api.tx.session.setKeys(keys, proof).signAndSend(controller)

# 4. Declare validation intent:
# api.tx.staking.validate(commission).signAndSend(controller)

# 5. Wait for next election/epoch
```

**Success Criteria:** Validators appear in staking candidates, elected in next era.

**Timeline:** Next election cycle (could be 1-24 hours depending on configuration).

---

#### Option 2A.3: Governance Proposal to Add Validators
```bash
# If validator set is governance-controlled:

# 1. Submit governance proposal to add validators 2-4
# Proposal type: "Add authority" or "Update validator set"

# 2. Validators 6-21 vote on proposal
# Requires 10/16 approval (62.5% threshold)

# 3. Proposal execution adds validators to active set

# 4. Session keys become active immediately or next epoch
```

**Success Criteria:** Proposal passes, validators added to authority set.

**Timeline:** Depends on governance voting period (typically 7-30 days).

---

### Phase 2B: Session Key Correction (If Validators 2-4 ARE in Genesis)

**Objective:** Align Azure VM session keys with what genesis expects.

#### Option 2B.1: Extract Expected Keys from Genesis
```bash
# 1. Find validator 2, 3, 4 accounts in genesis
jq '.genesis.runtime.session.keys' \
  /Users/macbook/Desktop/etrid/runtime/flare-chain/chainspec-raw.json

# 2. Export expected session keys for these accounts

# 3. Compare with keys in Azure VM keystores

# 4. If mismatch:
#    - Either import correct keys to keystores
#    - Or regenerate genesis with current keys
```

**Success Criteria:** Session keys in keystores match genesis configuration.

**Timeline:** Immediate (requires node restart).

---

#### Option 2B.2: Regenerate Genesis with Current Keys
```bash
# If current keys are correct, update genesis:

# 1. Extract session keys from all 3 Azure VMs
VM1_AURA="0x..."
VM1_GRANDPA="0x..."
VM1_ASFK="0x..."
# ... (repeat for VM2, VM3)

# 2. Rebuild chainspec with these keys:
# Edit chainspec JSON to include validators 2, 3, 4 with their keys

# 3. Rebuild raw chainspec:
./flarechain-node build-spec --chain modified-chainspec.json --raw \
  > new-chainspec-raw.json

# 4. Distribute to ALL validators (requires coordination)

# 5. Full network restart with new genesis
```

**Success Criteria:** All 19 validators (2-4 + 6-21) in new genesis.

**Timeline:** Requires network-wide coordination, could take days.

---

### Phase 3: Network Integration Strategy

#### Strategy 3.1: Bootstrap Node Approach (RECOMMENDED)

**Use Case:** Azure VMs serve as bootstrap nodes for network, don't need to validate.

**Implementation:**
```bash
# 1. Azure VMs are already functioning as bootnodes
# Their peer IDs are known and stable

# 2. Update validators 6-21 to use Azure VMs as bootnodes
# Create script: update-validators-bootnode.sh

# 3. Advantages:
#    - No consensus changes needed
#    - Improves network stability
#    - Provides reliable peer discovery
#    - Azure VMs still participate in gossip
```

**Success Criteria:**
- Validators 6-21 list Azure VMs in their peer sets
- Network topology shows Azure VMs as hub nodes
- Improved peer connectivity across network

**Timeline:** Immediate (requires SSH access to validators 6-21 to update configs).

---

#### Strategy 3.2: Expand Committee to 19 Validators

**Use Case:** Integrate all validators into active consensus.

**Implementation:**
```bash
# 1. Determine if committee size is runtime-configurable
# Check runtime code for MAX_AUTHORITIES constant

# 2. If configurable via extrinsic:
#    Submit "Increase validator set size to 19" proposal

# 3. If requires runtime upgrade:
#    - Modify runtime constant
#    - Compile new runtime WASM
#    - Submit setCode proposal for runtime upgrade

# 4. Once committee size = 19:
#    Follow Phase 2A to add validators 2-4
```

**Success Criteria:** All 19 validators producing blocks and voting.

**Timeline:** 1-4 weeks (requires runtime upgrade + governance).

---

#### Strategy 3.3: Replace 3 of Validators 6-21 with Azure VMs

**Use Case:** Keep committee size = 16, swap out 3 existing validators.

**Implementation:**
```bash
# 1. Identify 3 validators from 6-21 to deactivate
# Criteria: lowest performance, highest latency, least reliable

# 2. Submit governance proposal:
#    "Remove validators X, Y, Z and add validators 2, 3, 4"

# 3. Session keys for validators 2-4 become active
# Session keys for X, Y, Z become inactive

# 4. Validators X, Y, Z can remain as full nodes
```

**Success Criteria:** Azure VMs producing blocks, 3 former validators gracefully demoted.

**Timeline:** 1-2 weeks (governance proposal + voting).

---

#### Strategy 3.4: Staged Rollout (MOST CAUTIOUS)

**Use Case:** Minimize risk by adding validators one at a time.

**Implementation:**
```bash
# Week 1: Add validator 2 only
# - Register session keys for VM1
# - Committee size: 17 validators
# - Monitor for 7 days

# Week 2: Add validator 3
# - Register session keys for VM2
# - Committee size: 18 validators
# - Monitor for 7 days

# Week 3: Add validator 4
# - Register session keys for VM3
# - Committee size: 19 validators
# - Full monitoring

# Week 4: Evaluate performance
# - If stable: maintain 19 validators
# - If issues: investigate and potentially revert
```

**Success Criteria:** Progressive integration with monitoring at each stage.

**Timeline:** 4 weeks total for full integration.

---

## Section 4: Alternative Peer ID Discovery Methods

Since SSH access to validators 6-21 is unavailable, here are alternative methods:

### Method A: RPC Peer Discovery
```bash
# Use system_peers RPC to get peer information from Azure VMs
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"system_peers",
    "params":[],
    "id":1
  }' \
  http://20.69.26.209:9944 | jq '.result[] | {peerId, roles, knownAddresses}'

# This returns peer IDs of all connected peers
# Azure VMs are connected to validators 6-21
# Therefore, this reveals peer IDs of active committee
```

**Expected Output:** 12-13 peer objects with peer IDs.

**Action:** Cross-reference these peer IDs with known Azure VM peer IDs to identify validators 6-21.

---

### Method B: Network Telemetry Service
```bash
# If telemetry is enabled on validators:

# 1. Check if telemetry service is running
# Substrate telemetry typically at telemetry.polkadot.io or custom endpoint

# 2. Query telemetry API for FlareChain network
curl https://telemetry.polkadot.io/feed/

# 3. Look for "flarechain" or "etrid" network

# 4. Telemetry shows:
#    - Validator names
#    - Peer IDs
#    - Block heights
#    - Network topology
```

**Expected Output:** Web dashboard or JSON feed with all validator info.

**Action:** If telemetry is enabled, provides complete network map.

---

### Method C: Blockchain Explorer / Block Author Analysis
```bash
# Analyze block headers to identify authors:

# 1. Query recent blocks via RPC
for block in {6900..6941}; do
  curl -X POST -H "Content-Type: application/json" \
    --data "{
      \"jsonrpc\":\"2.0\",
      \"method\":\"chain_getBlockHash\",
      \"params\":[$block],
      \"id\":1
    }" \
    http://20.69.26.209:9944 | jq -r '.result' | \
  xargs -I {} curl -X POST -H "Content-Type: application/json" \
    --data "{
      \"jsonrpc\":\"2.0\",
      \"method\":\"chain_getBlock\",
      \"params\":[\"{}"],
      \"id\":1
    }" \
    http://20.69.26.209:9944 | \
  jq '.result.block.header.author'
done

# 2. Count frequency of each author
# Authors are account IDs, not peer IDs, but shows active validators

# 3. Map account IDs to validator indices
# Requires knowing genesis account assignments
```

**Expected Output:** List of account IDs that authored blocks (the active committee).

**Action:** Identify which accounts are in positions 6-21.

---

### Method D: DHT Crawl via Polkadot.js
```javascript
// Using @polkadot/api to connect and inspect network

const { ApiPromise, WsProvider } = require('@polkadot/api');

const provider = new WsProvider('ws://20.69.26.209:9944');
const api = await ApiPromise.create({ provider });

// Get session validators
const validators = await api.query.session.validators();
console.log('Active validators:', validators.toHuman());

// Get session keys for each validator
for (const validator of validators) {
  const keys = await api.query.session.nextKeys(validator);
  console.log(`Validator ${validator}: Keys ${keys.toHuman()}`);
}

// Get peer info
const peers = await api.rpc.system.peers();
console.log('Connected peers:', peers.toJSON());
```

**Expected Output:** Programmatic access to session data and peer information.

**Action:** Build complete validator-to-peer-ID mapping.

---

## Section 5: Decision Matrix

| Scenario | Recommended Action | Timeline | Risk | Effort |
|----------|-------------------|----------|------|--------|
| **Azure VMs not in genesis** | Phase 2A.1: Register session keys | 1-2 epochs | Low | Low |
| **Session keys mismatch** | Phase 2B.1: Correct keys from genesis | Immediate | Low | Low |
| **Staking required** | Phase 2A.2: Bond + Validate | 1 era | Medium | Medium |
| **Committee size fixed at 16** | Strategy 3.3: Replace 3 validators | 2-4 weeks | High | High |
| **Want all 19 validators** | Strategy 3.2: Expand committee | 2-4 weeks | Medium | High |
| **Just want network stability** | Strategy 3.1: Bootnodes only | Immediate | Low | Low |
| **Maximum safety** | Strategy 3.4: Staged rollout | 4 weeks | Low | Medium |

---

## Section 6: Immediate Next Steps (Prioritized)

### Step 1: Execute Information Gathering (Today)
```bash
# Run all Phase 1 tasks
cd /Users/macbook/Desktop/etrid/docs/mainnet

# Task 1.1: Analyze genesis
./analyze-genesis.sh > genesis_analysis.txt

# Task 1.2: Query validator set via RPC
./query-validator-set.sh > validator_set.json

# Task 1.3: Check runtime metadata
./check-runtime-metadata.sh > runtime_metadata.txt

# Task 1.4: Examine Azure VM logs
./check-validator-logs.sh > validator_logs.txt
```

**Output:** 4 files with complete information about validator configuration.

**Decision Point:** Choose between Phase 2A (registration) or Phase 2B (key correction).

---

### Step 2: Map Peer IDs Using RPC (Today)
```bash
# Use Method A: RPC Peer Discovery
./discover-peers-via-rpc.sh

# This queries Azure VM peer lists to identify validators 6-21
# No SSH needed - uses RPC only
```

**Output:** `peer_id_mapping.json` showing which peer IDs belong to validators 6-21.

**Success Criteria:** Able to map at least 12 of 16 committee peer IDs.

---

### Step 3: Validate Network Health (Today)
```bash
# Ensure Azure VMs are actually helping the network
./monitor-azure-vms.sh

# Check:
# - Are validators 6-21 peering with Azure VMs?
# - Is block gossip flowing through Azure VMs?
# - Are Azure VMs reducing network latency?
```

**Output:** Network health report showing Azure VM contribution.

**Decision Point:** If Azure VMs already providing value as bootnodes, may not need to integrate into committee.

---

### Step 4: Create Registration Scripts (Tomorrow)
```bash
# Prepare scripts for whichever strategy is chosen:

# For session key registration:
./register-session-keys.sh VM1 VM2 VM3

# For staking:
./stake-and-validate.sh VM1 VM2 VM3

# For governance proposal:
./create-add-validator-proposal.sh
```

**Output:** Ready-to-execute scripts for integration.

**Decision Point:** Ready to execute Phase 2 once strategy is confirmed.

---

## Section 7: Risk Assessment

### Low Risk Actions
- ✅ Query RPC endpoints to gather information
- ✅ Analyze genesis chainspec
- ✅ Check Azure VM logs
- ✅ Use Azure VMs as bootnodes only
- ✅ Monitor network without changes

### Medium Risk Actions
- ⚠️ Register session keys (reversible via session.purgeKeys)
- ⚠️ Submit governance proposals (requires voting, may be rejected)
- ⚠️ Staged rollout of new validators (monitored at each step)

### High Risk Actions
- ❌ Regenerate genesis (requires full network restart)
- ❌ Remove existing validators (may impact consensus)
- ❌ Runtime upgrade without thorough testing

---

## Section 8: Success Metrics

### If Azure VMs Become Validators:
- [ ] Peer IDs appear in active committee queries
- [ ] Azure VMs author blocks (visible in block headers)
- [ ] Azure VMs participate in GRANDPA finalization (see finality lag ≤ 2)
- [ ] No increase in uncle/fork rate
- [ ] Network latency remains stable or improves
- [ ] All 19 validators show healthy peer counts (10+)

### If Azure VMs Remain Bootnodes:
- [ ] Validators 6-21 list Azure VMs as connected peers
- [ ] Average peer count per validator increases
- [ ] Network partition events decrease
- [ ] New validators can join by connecting to Azure VMs
- [ ] Azure VMs maintain 95%+ uptime

---

## Conclusion

**Key Finding:** Azure VMs are network-healthy but excluded from active committee due to one of:
1. Not configured in genesis authorities (most likely)
2. Session keys not registered on-chain (likely)
3. Validator indices reserved for specific roles (possible)
4. Staking/bonding not completed (less likely)

**Recommended Immediate Action:**
1. Execute Phase 1 information gathering (RPC queries, genesis analysis, log inspection)
2. Use Method A (RPC peer discovery) to map committee peer IDs
3. Based on findings, choose between:
   - **Quick win:** Register session keys if runtime supports it (1-2 days)
   - **Safe win:** Use Azure VMs as bootnodes only (immediate, no risk)
   - **Complete win:** Expand committee to 19 via governance (2-4 weeks)

**Next Decision Required:**
After completing Phase 1 information gathering, we'll know definitively:
- What the barrier to committee membership is
- What action is required to overcome it
- What timeline is realistic for integration

**Status:** Ready to begin Phase 1 information gathering upon approval.
