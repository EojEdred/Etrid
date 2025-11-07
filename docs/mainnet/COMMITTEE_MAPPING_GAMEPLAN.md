# FlareChain Committee Mapping Game Plan
## Finding the Missing 5 Validators

---

## Executive Summary

**Current State:**
- Genesis defines: **21 validators** (5 Directors + 16 ValidityNodes)
- Currently active: **16 validators** in committee
- **Missing: 5 validators** not participating in consensus

**Goal:**
Identify which 5 validators are missing and why they're not in the committee.

---

## What Peer IDs Tell Us

### ✅ What Peer IDs CAN Do:
1. **Identify running nodes** on the network
2. **Build connection graph** (who connects to whom)
3. **Distinguish validators** from full nodes (AUTHORITY vs FULL)
4. **Track network health** (peer counts, connectivity)

### ❌ What Peer IDs CANNOT Do:
1. **Link to blockchain accounts** (no cryptographic connection)
2. **Identify genesis validators** (need GRANDPA keys for that)
3. **Determine block authorship** (need AURA keys)

### 🔑 The Critical Link: GRANDPA Keys

GRANDPA keys bridge everything:

```
Peer ID (12D3KooW...)
    ↓ [query keystore]
GRANDPA Key (0x90bb1f...)
    ↓ [search genesis]
Genesis AccountId (5GrwvaEF...)
    ↓ [lookup deployment]
VM/Node Name (Azure-Validator-3)
```

---

## The Mapping Strategy

### Step 1: Extract All 21 Genesis GRANDPA Keys

**Source**: Query runtime API
```bash
curl -H 'Content-Type: application/json' \
  -d '{"id":1,"jsonrpc":"2.0","method":"state_call",
       "params":["GrandpaApi_grandpa_authorities","0x"]}' \
  http://VALIDATOR_IP:9944
```

**Output**: SCALE-encoded list of 21 GRANDPA keys + weights

**Result**: `genesis_validators.json` with all 21 validators

---

### Step 2: Match 16 Active Validators to Genesis

**You already have 16 GRANDPA keys from active validators:**
```
Val-6:  0x90bb1faa905f0bd0...
Val-7:  0x2975859973decf0c...
...
Val-21: 0x2d1421832d96cb66...
```

**Action**: Search for each in the 21 genesis keys

**Result**: Mapping showing which genesis validator each active node represents

---

### Step 3: Identify the 5 Missing Genesis Validators

**Method**: Find genesis entries NOT in the active 16

**Output**:
```
MISSING VALIDATORS (5):
- Genesis #2 → GRANDPA: 0x0a9442...
- Genesis #5 → GRANDPA: 0x???...
- Genesis #10 → GRANDPA: 0x???...
- Genesis #15 → GRANDPA: 0x???...
- Genesis #20 → GRANDPA: 0x???...
```

---

### Step 4: Map to Deployed VMs

**Known deployment plan:**

**Directors (5):**
1. GizziDirector (oracle_vm1) - ✅ Running (confirmed)
2. EojDirector (oracle_vm2) - ❓ Unknown
3. AzureDirector1 (azure_vm1) - ✅ Likely VM1 bootnode
4. AzureDirector2 (azure_vm2) - ✅ Likely VM2
5. AWSDirector (aws_vm1) - ❓ Unknown

**ValidityNodes (16):**
- oracle_vm3-6: OracleValidator1-4
- azure_vm3-10: AzureValidator1-8  
- aws_vm2-4: AWSValidator1-3
- local_vm1: LocalValidator

**Action**: Match missing genesis validators to VM assignments

---

### Step 5: Diagnose Each Missing Validator

**For each missing validator:**

#### A. Is the VM Running?
```bash
ssh validator@VM_IP "systemctl status flarechain-validator"
```
- ✅ Running → Check B
- ❌ Not running → **CAUSE: VM not started**

#### B. Does it have session keys?
```bash
ssh validator@VM_IP "ls /var/lib/etrid/chains/*/keystore/"
```
**Need 3 files:**
- `6175726144...` (aura - Sr25519)
- `6772616e...` (gran - Ed25519)  
- `6173666b...` (asfk - Sr25519)

- ✅ All present → Check C
- ❌ Missing → **CAUSE: Keys not generated**

#### C. Are keys registered on-chain?
```bash
curl -X POST http://VM_IP:9944 -d \
  '{"jsonrpc":"2.0","method":"author_hasKey",
    "params":["GRANDPA_KEY_HERE","gran"],"id":1}'
```
- ✅ Registered → **CAUSE: Unknown (investigate further)**
- ❌ Not registered → **CAUSE: Never called session.setKeys**

---

## Tools to Build

### Tool 1: Extract Genesis Validators
**File**: `extract_genesis.py`
```python
# Query GrandpaApi_grandpa_authorities
# Decode SCALE encoding
# Output: genesis_validators.json (21 entries)
```

### Tool 2: Map Active to Genesis
**File**: `map_committee.py`  
```python
# Input: genesis_validators.json + your 16 active list
# Match by GRANDPA key
# Output: mapping.json + report showing 5 missing
```

### Tool 3: Health Checker
**File**: `check_validator.sh`
```bash
# For each VM: check service, keys, network
# Output: health report per validator
```

### Tool 4: Network Topology
**File**: `build_topology.py`
```python
# Query system_peers on multiple validators
# Build connection graph
# Output: topology.dot for Graphviz visualization
```

---

## Execution Workflow

### Phase 1: Data Collection (30 min)

1. Run `extract_genesis.py` → get 21 genesis validators
2. Save your 16 active validators to JSON
3. Create VM deployment mapping

### Phase 2: Analysis (20 min)

4. Run `map_committee.py` → identify the 5 missing
5. Review mapping report

### Phase 3: Diagnosis (30 min)

6. For each missing validator:
   - Check if VM is running
   - Check if keys exist
   - Check if keys registered
7. Categorize issues:
   - Not deployed
   - Not running
   - Missing keys
   - Keys not bonded

### Phase 4: Remediation (varies)

8. For each category:
   - **Not deployed**: Deploy VM + binary
   - **Not running**: Start service
   - **Missing keys**: Generate with `author_rotateKeys`
   - **Not bonded**: Submit `session.setKeys` extrinsic

---

## Expected Deliverables

### 1. Committee Mapping Table
```
┌───────┬────────────────┬─────────────┬──────────┬──────────────┐
│ Index │ Genesis Name   │ GRANDPA Key │ Status   │ Issue        │
├───────┼────────────────┼─────────────┼──────────┼──────────────┤
│ 1     │ GizziDirector  │ 0xee75...   │ ✅ ACTIVE │ -            │
│ 2     │ EojDirector    │ 0x0a94...   │ ❌ MISSING│ Not running  │
│ 3     │ Azure1         │ 0x8a9a...   │ ✅ ACTIVE │ -            │
│ ...   │ ...            │ ...         │ ...      │ ...          │
└───────┴────────────────┴─────────────┴──────────┴──────────────┘
```

### 2. Network Topology Graph
```
         [Gizzi]─────[Azure1]
            │            │
         [Azure2]───[Val-1]
            │            │
         [Val-2]───[Val-3]
```

### 3. Remediation Action Plan
```
## 1. EojDirector (Genesis #2)
- Issue: Service not running
- Action: 
  [ ] Deploy binary
  [ ] Start service

## 2. AWSDirector (Genesis #5)
- Issue: Missing session keys
- Action:
  [ ] Generate keys
  [ ] Insert to keystore
  ...
```

---

## Timeline & Resources

**Total Time**: ~2 hours for complete analysis

**Prerequisites:**
- RPC access to 1+ running validator
- SSH access to deployed VMs
- chainspec-mainnet-raw-FIXED.json

**Tools:**
- Python 3.8+
- jq, curl
- Graphviz (optional)

---

## Success Metrics

✅ All 21 genesis validators identified  
✅ 16 active mapped to genesis  
✅ 5 missing identified with reasons  
✅ Network topology visualized  
✅ Remediation plan created  

---

## Key Takeaway

**Peer IDs identify nodes on the network**, but **GRANDPA keys identify validators in consensus**. 

By extracting the 21 genesis GRANDPA keys and matching them to your 16 active nodes, you'll know exactly which 5 validators are missing and can diagnose why.

This is the fastest path to full 21-validator committee! 🎯
