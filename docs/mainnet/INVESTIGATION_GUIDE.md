# Ëtrid FlareChain - Committee Investigation & Integration Guide

**Purpose:** Understand why Azure VMs (validators 2-4) are not in the active committee and develop integration strategy.

**Status:** Investigation scripts ready to execute

---

## Quick Start - Run These Scripts in Order

### Phase 1: Information Gathering (15 minutes)

Execute these 4 scripts to gather comprehensive information about the network:

```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet

# 1. Analyze genesis configuration
./analyze-genesis.sh
# Output: genesis_analysis.txt
# Shows: Which validators are configured in genesis

# 2. Query active validator set via RPC
./query-validator-set.sh
# Output: validator_set.json
# Shows: Which validators are producing blocks right now

# 3. Discover peer IDs via RPC (no SSH needed)
./discover-peers-via-rpc.sh
# Output: peer_id_mapping.json, peer_id_mapping.txt
# Shows: Peer IDs of all validators in network

# 4. Monitor Azure VM network contribution
./monitor-azure-vms.sh
# Output: azure_vm_monitoring.txt
# Shows: How Azure VMs are contributing to network health
```

### Phase 2: Analysis (10 minutes)

Review the output files and determine root cause:

```bash
# Open analysis document
open COMMITTEE_ANALYSIS_AND_PLAN.md

# Review findings
cat genesis_analysis.txt | grep -A 10 "ANALYSIS SUMMARY"
cat validator_set.json | jq '.result | length'
cat peer_id_mapping.txt | grep -A 5 "DISCOVERY SUMMARY"
cat azure_vm_monitoring.txt | grep -A 10 "NETWORK VALUE ASSESSMENT"
```

**Key Questions to Answer:**
1. How many validators are in genesis? (Expected: 16)
2. Are validators 2-4 in genesis? (Likely: NO)
3. How many peer IDs discovered? (Expected: 16 for validators 6-21)
4. Are Azure VMs providing network value? (Check gossip, peers, uptime)

### Phase 3: Decision (5 minutes)

Based on findings, choose integration strategy:

| Finding | Strategy | Timeline | Risk |
|---------|----------|----------|------|
| **Genesis has 16 validators (6-21 only)** | Register session keys or governance proposal | 1-4 weeks | Medium |
| **Genesis has 19 validators (2-21 all)** | Correct session key mismatch | Immediate | Low |
| **Azure VMs provide high network value** | Use as bootnodes, integrate later | Immediate | Low |
| **Network is stable as-is** | Keep Azure VMs as hot standbys | Immediate | Low |

---

## Investigation Scripts Reference

### `analyze-genesis.sh`
**Purpose:** Extract validator configuration from genesis chainspec

**What it checks:**
- Session pallet: initial authorities and their session keys
- AURA pallet: block production authorities
- GRANDPA pallet: finality authorities
- ASF/PPFA pallet: ASF consensus configuration
- Staking pallet: validator stakes (if applicable)
- Balances: validator account endowments

**Key Output:**
```
Genesis Validator Counts:
  Session Keys:       16
  AURA Authorities:   16
  GRANDPA Authorities: 16

✓ Genesis defines exactly 16 authorities
  This matches the active committee size (validators 6-21)
```

**Success Criteria:**
- Identifies exact number of validators in genesis
- Shows if validators 2-4 are included or excluded
- Provides validator account IDs for cross-reference

---

### `query-validator-set.sh`
**Purpose:** Query active validators via RPC (no SSH required)

**What it queries:**
- Runtime version and available APIs
- Session validators (current active set)
- AURA authorities (block producers)
- Recent block authors (last 20 blocks)
- GRANDPA voter set (finality authorities)
- System peers (network connectivity)

**Key Output:**
```
Block authors (last 20 blocks):
Block #6920: 0x1234... (account ID)
Block #6921: 0x5678... (account ID)
...

Total unique block authors: 16
✓ Exactly 16 unique block authors (matches committee size)
```

**Success Criteria:**
- Confirms exactly 16 validators producing blocks
- Shows validator account IDs for mapping
- Verifies network health and peer connectivity

---

### `discover-peers-via-rpc.sh`
**Purpose:** Map peer IDs to validators using RPC (Method A from plan)

**What it does:**
- Queries `system_peers` RPC on all 3 Azure VMs
- Aggregates unique peer IDs discovered in network
- Eliminates Azure VM peer IDs to isolate committee members
- Analyzes peer roles and block sync status
- Attempts to map peer IDs to validator IPs

**Key Output:**
```
Total unique peer IDs discovered: 19
Potential committee members: 16

✓ SUCCESS: Discovered all 16 committee member peer IDs

Committee peer IDs (validators 6-21):
  1  12D3KooWXxxx...
  2  12D3KooWYxxx...
  ...
  16 12D3KooWZxxx...
```

**Success Criteria:**
- Discovers exactly 16 peer IDs (validators 6-21)
- Confirms Azure VMs are NOT among block producers
- Provides peer ID list for network topology mapping

**Limitations:**
- Cannot determine exact validator index → peer ID mapping without telemetry
- May not discover validators behind strict firewalls
- Requires at least one Azure VM RPC endpoint accessible

---

### `monitor-azure-vms.sh`
**Purpose:** Assess Azure VM contribution to network health

**What it checks:**
- Process status and uptime
- Peer count and connectivity
- Block sync status
- Peer discovery activity
- Network ports (30333, 9944)
- Block gossip propagation
- RPC endpoint availability
- System resource usage
- Blockchain data size

**Key Output:**
```
NETWORK VALUE ASSESSMENT

Azure VMs provide value as:
1. BOOTNODE SERVICES (stable peer discovery)
2. BLOCK GOSSIP PROPAGATION (relay blocks)
3. RPC ENDPOINTS (query and monitoring)
4. NETWORK REDUNDANCY (hot standbys)

OPTION A: Keep as Bootnodes (Low Risk, Immediate Value)
OPTION B: Integrate into Committee (Higher Value, More Risk)
OPTION C: Staged Approach (Recommended)
```

**Success Criteria:**
- All 3 Azure VMs running and healthy
- Connected to 10+ peers each
- Actively gossiping blocks
- RPC endpoints responsive

---

## Common Scenarios and Actions

### Scenario 1: Genesis has 16 validators, Azure VMs NOT included

**Finding:**
```
genesis_analysis.txt shows:
  Session Keys: 16
  (Validators 6-21 only, 2-4 excluded)
```

**Action:** Choose one of:

**Option A: Register Session Keys (Fastest)**
```bash
# If runtime supports session.setKeys extrinsic:
cd /Users/macbook/Desktop/etrid/scripts
./register-session-keys.sh VM1 VM2 VM3

# Wait for next session/epoch
# Timeline: 1-2 epochs (hours to days depending on epoch length)
```

**Option B: Governance Proposal (Safest)**
```bash
# Submit proposal to add validators 2-4
cd /Users/macbook/Desktop/etrid/scripts
./create-add-validator-proposal.sh

# Requires 10/16 validator approval
# Timeline: 7-30 days (governance voting period)
```

**Option C: Use as Bootnodes (Immediate Value)**
```bash
# Update validators 6-21 to peer with Azure VMs
cd /Users/macbook/Desktop/etrid/docs/mainnet
./update-validators-bootnode.sh

# Requires SSH access to validators 6-21
# Timeline: Immediate (but need SSH access)
```

---

### Scenario 2: Genesis has 19 validators, but only 16 active

**Finding:**
```
genesis_analysis.txt shows:
  Session Keys: 19
  (All validators 2-21 included)

query-validator-set.sh shows:
  Active block authors: 16
```

**Analysis:**
- Session keys in Azure VM keystores may not match genesis
- OR validators waiting for session rotation
- OR validators not properly registered

**Action:**
```bash
# 1. Extract expected session keys from genesis
cd /Users/macbook/Desktop/etrid/docs/mainnet
./compare-genesis-keys.sh

# 2. If mismatch found, either:
#    a) Import correct keys to Azure VM keystores
#    b) Update genesis with current keys (requires network restart)

# 3. If keys match, check session rotation:
#    Query: api.query.session.currentIndex()
#    Wait for next session change
```

---

### Scenario 3: Azure VMs providing high network value as bootnodes

**Finding:**
```
azure_vm_monitoring.txt shows:
  VM1: 13 peers, gossiping 50+ blocks/min
  VM2: 12 peers, gossiping 45+ blocks/min
  VM3: 13 peers, gossiping 48+ blocks/min

  All validators 6-21 peer with Azure VMs
```

**Analysis:**
- Azure VMs already serving critical bootnode role
- Network health improved by their presence
- May not need to integrate into consensus immediately

**Action:**
```bash
# Strategic recommendation: Staged approach

# Phase 1 (Week 1): Keep as bootnodes
# - Monitor network stability
# - Verify all validators can connect

# Phase 2 (Week 2-3): Prepare integration
# - Test session key registration on testnet
# - Draft governance proposal
# - Build community support

# Phase 3 (Week 4): Execute integration
# - Submit governance proposal OR register keys
# - Monitor closely during activation
# - Be ready to revert if issues arise
```

---

## Troubleshooting

### Cannot Connect to Azure VM RPC

**Symptom:**
```
query-validator-set.sh fails:
  ✗ Failed to connect to 20.69.26.209:9944
```

**Solutions:**
```bash
# 1. Check if RPC port is open
telnet 20.69.26.209 9944

# 2. Verify validator is running
ssh -i ~/.ssh/gizzi-validator ubuntu@20.69.26.209 "pgrep flarechain-node"

# 3. Check firewall rules
ssh -i ~/.ssh/gizzi-validator ubuntu@20.69.26.209 "sudo ufw status"

# 4. Verify RPC flags in service
ssh -i ~/.ssh/gizzi-validator ubuntu@20.69.26.209 \
  "sudo systemctl cat flarechain-validator | grep rpc"

# Should see: --rpc-port 9944 --rpc-external (or --unsafe-rpc-external)
```

---

### Peer Discovery Returns 0 Peers

**Symptom:**
```
discover-peers-via-rpc.sh shows:
  Total unique peer IDs discovered: 0
```

**Solutions:**
```bash
# 1. Check if Azure VMs have network connectivity
ssh -i ~/.ssh/gizzi-validator ubuntu@20.69.26.209 \
  "sudo journalctl -u flarechain-validator --since '10 minutes ago' | grep -i peer"

# 2. Verify port 30333 is open
ssh -i ~/.ssh/gizzi-validator ubuntu@20.69.26.209 \
  "sudo netstat -tlnp | grep 30333"

# 3. Check if --public-addr is set
ssh -i ~/.ssh/gizzi-validator ubuntu@20.69.26.209 \
  "sudo systemctl cat flarechain-validator | grep public-addr"

# Should see: --public-addr /ip4/PUBLIC_IP/tcp/30333

# 4. If missing, add --public-addr flag
# See: VALIDATOR_FINAL_CONFIG.md for detailed instructions
```

---

### Genesis Analysis Shows Unexpected Configuration

**Symptom:**
```
analyze-genesis.sh shows:
  Session Keys: 0
  AURA Authorities: 0
  ⚠ Configuration not found
```

**Solutions:**
```bash
# 1. Verify chainspec location
find /Users/macbook/Desktop/etrid -name "chainspec*.json" -type f

# 2. Check if using raw or human-readable chainspec
# Raw chainspec has hex-encoded data
# Human-readable has readable JSON

# 3. Try alternative chainspec locations
./analyze-genesis.sh  # will auto-search multiple locations

# 4. If chainspec not on local machine, copy from validator
scp -i ~/.ssh/gizzi-validator \
  ubuntu@20.69.26.209:~/chainspec.json \
  /Users/macbook/Desktop/etrid/runtime/flare-chain/
```

---

## Next Steps After Investigation

Once all 4 scripts have been executed and analyzed:

### 1. Review Summary Documents
- `genesis_analysis.txt` → Understand genesis configuration
- `validator_set.json` → Identify active block producers
- `peer_id_mapping.txt` → Map network topology
- `azure_vm_monitoring.txt` → Assess current value

### 2. Make Strategic Decision
Refer to decision matrix in `COMMITTEE_ANALYSIS_AND_PLAN.md` Section 5

### 3. Execute Integration Plan
- **Fast track (1-2 days):** Session key registration (if supported)
- **Safe track (1-2 weeks):** Governance proposal + voting
- **Immediate value:** Bootnode configuration update
- **Maximum safety:** Staged rollout over 4 weeks

### 4. Monitor and Verify
```bash
# After any changes, monitor for:
# - All validators synced to same block
# - No increase in finality lag
# - No uncle/fork blocks
# - Peer counts remain healthy (10+)
# - Block production continues smoothly
```

---

## Files Generated by Investigation

```
/Users/macbook/Desktop/etrid/docs/mainnet/
├── COMMITTEE_ANALYSIS_AND_PLAN.md        # Master strategy document
├── INVESTIGATION_GUIDE.md                # This file
│
├── analyze-genesis.sh                    # Script 1
├── query-validator-set.sh                # Script 2
├── discover-peers-via-rpc.sh             # Script 3
├── monitor-azure-vms.sh                  # Script 4
│
├── genesis_analysis.txt                  # Output from script 1
├── validator_set.json                    # Output from script 2
├── peer_id_mapping.json                  # Output from script 3
├── peer_id_mapping.txt                   # Output from script 3 (human)
├── azure_vm_monitoring.txt               # Output from script 4
│
└── /tmp/etrid_rpc_discovery/             # Raw peer data
    ├── peers_VM1.json
    ├── peers_VM2.json
    ├── peers_VM3.json
    ├── all_peers.txt
    ├── unique_peers.txt
    └── committee_peers.txt
```

---

## Support and References

- **Bootnode Quick Reference:** `BOOTNODE_QUICK_REFERENCE.md`
- **Validator Configuration:** `VALIDATOR_FINAL_CONFIG.md`
- **Network Topology:** Run `discover-peers-via-rpc.sh` for current state
- **Session Keys:** Check keystores in `~/.etrid/validator/chains/flarechain_mainnet/keystore/`

---

**Ready to begin investigation!**

Run the 4 scripts in order, then review `COMMITTEE_ANALYSIS_AND_PLAN.md` for detailed strategy options.
