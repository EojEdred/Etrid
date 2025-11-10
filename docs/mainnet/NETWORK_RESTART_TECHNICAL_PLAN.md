# Ëtrid FlareChain Network Restart - Technical Execution Plan

**Date:** November 9, 2025
**Type:** Technical Operations Playbook
**Status:** READY FOR EXECUTION
**Target Restart Date:** November 13-14, 2025

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Phase 1: State Export](#phase-1-state-export)
3. [Phase 2: Genesis Creation](#phase-2-genesis-creation)
4. [Phase 3: Testing and Validation](#phase-3-testing-and-validation)
5. [Phase 4: Distribution](#phase-4-distribution)
6. [Phase 5: Network Restart](#phase-5-network-restart)
7. [Phase 6: Verification](#phase-6-verification)
8. [Rollback Procedures](#rollback-procedures)
9. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Access

**Validator Access:**
```bash
# Oracle Cloud Directors
ssh ubuntu@64.181.215.19 -i ~/.ssh/gizzi-validator    # Gizzi (Director 1)
ssh ubuntu@129.80.122.34 -i ~/.ssh/gizzi-validator    # AuditDev (Director 2)
ssh ubuntu@157.173.200.86 -i ~/.ssh/oracle-directors  # Director 3
ssh ubuntu@157.173.200.84 -i ~/.ssh/oracle-directors  # Director 4
ssh ubuntu@157.173.200.81 -i ~/.ssh/oracle-directors  # Director 5
ssh ubuntu@157.173.200.80 -i ~/.ssh/oracle-directors  # Director 6
# Directors 7-9 (provision if needed)

# Contabo Validity Nodes (16 validators)
ssh root@85.239.239.194 -i ~/.ssh/contabo-validators  # Validator 10
ssh root@85.239.239.193 -i ~/.ssh/contabo-validators  # Validator 11
# ... (all 16 Contabo validators)
```

**Required Tools on Local Machine:**
```bash
# Verify tools installed
flarechain-node --version  # Should show latest build
jq --version               # JSON processing
curl --version             # API queries
```

**Backup Storage:**
```bash
# Prepare backup directory
mkdir -p ~/Desktop/etrid/network-restart-backup
cd ~/Desktop/etrid/network-restart-backup
```

---

## Phase 1: State Export

**Duration:** 2-4 hours
**Responsibility:** Core development team
**Prerequisites:** Access to synced validator (Gizzi or AuditDev)

### Step 1.1: Verify Current Chain State

```bash
# Connect to Gizzi validator
ssh ubuntu@64.181.215.19 -i ~/.ssh/gizzi-validator

# Check current block height
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"chain_getHeader",
    "params":[]
}' http://localhost:9944 | jq .

# Expected output:
# {
#   "result": {
#     "number": "0x12a91",  // Block #76,401 (example)
#     "parentHash": "0x...",
#     "stateRoot": "0x..."
#   }
# }

# Check finalized block
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"chain_getFinalizedHead",
    "params":[]
}' http://localhost:9944 | jq .

# Expected output: Block hash for #63,274
```

**Verification Checklist:**
- [ ] Best block > #76,000
- [ ] Finalized block = #63,274
- [ ] State root accessible
- [ ] GRANDPA stalled (expected)

### Step 1.2: Export State at Block #63,274

```bash
# Get block hash for #63,274
FINALIZED_BLOCK=$(curl -s -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"chain_getBlockHash",
    "params":[63274]
}' http://localhost:9944 | jq -r .result)

echo "Finalized block #63,274 hash: $FINALIZED_BLOCK"

# Export full state at block #63,274
curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"state_getPairs\",
    \"params\":[\"0x\", \"$FINALIZED_BLOCK\"]
}" http://localhost:9944 > ~/state_export_63274.json

# Verify export file size (should be several MB)
ls -lh ~/state_export_63274.json

# Expected: 50-500 MB depending on chain activity
```

### Step 1.3: Export Specific Pallet States

Export critical pallets separately for verification:

```bash
# Export System pallet
curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"state_getStorage\",
    \"params\":[\"0x26aa394eea5630e07c48ae0c9558cef7\", \"$FINALIZED_BLOCK\"]
}" http://localhost:9944 > ~/state_system.json

# Export Balances pallet (account balances)
curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"state_getPairs\",
    \"params\":[\"0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9\", \"$FINALIZED_BLOCK\"]
}" http://localhost:9944 > ~/state_balances.json

# Export Session pallet (validator keys)
curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"state_getPairs\",
    \"params\":[\"0xcec5070d609dd3497f72bde07fc96ba0\", \"$FINALIZED_BLOCK\"]
}" http://localhost:9944 > ~/state_session.json

# Export Staking pallet
curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"state_getPairs\",
    \"params\":[\"0x5f3e4907f716ac89b6347d15ececedca\", \"$FINALIZED_BLOCK\"]
}" http://localhost:9944 > ~/state_staking.json

# Export GRANDPA authorities (for verification)
curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"state_getStorage\",
    \"params\":[\"0x3a6772616e6470615f617574686f726974696573\", \"$FINALIZED_BLOCK\"]
}" http://localhost:9944 > ~/state_grandpa_authorities.json

# Expected: Will show 0 authorities (this is the bug we're fixing)
cat ~/state_grandpa_authorities.json | jq .
```

### Step 1.4: Backup to Local Machine

```bash
# On local machine
scp -i ~/.ssh/gizzi-validator \
    ubuntu@64.181.215.19:~/state_*.json \
    ~/Desktop/etrid/network-restart-backup/

# Verify all exports downloaded
ls -lh ~/Desktop/etrid/network-restart-backup/state_*.json

# Create checksums
cd ~/Desktop/etrid/network-restart-backup
sha256sum state_*.json > state_export_checksums.txt
cat state_export_checksums.txt
```

**Verification Checklist:**
- [ ] state_export_63274.json (full state)
- [ ] state_system.json
- [ ] state_balances.json
- [ ] state_session.json
- [ ] state_staking.json
- [ ] state_grandpa_authorities.json
- [ ] Checksums generated
- [ ] All files backed up locally

---

## Phase 2: Genesis Creation

**Duration:** 4-6 hours
**Responsibility:** Core development team
**Prerequisites:** State exports from Phase 1

### Step 2.1: Create Base Genesis Spec

```bash
# On local machine
cd ~/Desktop/etrid

# Generate fresh genesis spec (plain format)
./target/release/flarechain-node build-spec \
    --chain mainnet \
    --disable-default-bootnode \
    > docs/mainnet/chainspec-mainnet-v2-plain.json

# Verify generated
ls -lh docs/mainnet/chainspec-mainnet-v2-plain.json
```

### Step 2.2: Configure GRANDPA Authorities (CRITICAL FIX)

Edit `chainspec-mainnet-v2-plain.json`:

```json
{
  "name": "Ëtrid FlareChain Mainnet",
  "id": "flarechain_mainnet",
  "chainType": "Live",
  "bootNodes": [
    "/ip4/64.181.215.19/tcp/30333/p2p/12D3KooWPyfp2DECPKTmJ1AhxB6midHnp7wYTP15vBAxbTewxaq1",
    "/ip4/85.239.239.194/tcp/30333/p2p/12D3KooWSrYpSQ6SiDR3uduqbiepyfVp8xmaC8mzY6RmU29MEHGv"
  ],
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 18
  },
  "genesis": {
    "runtime": {
      "system": {},
      "balances": {
        "balances": [
          // Import from state_balances.json
          // All account balances from block #63,274
        ]
      },
      "session": {
        "keys": [
          // DIRECTORS ONLY (9 validators)
          [
            "DIRECTOR_1_ACCOUNT_ID",  // Gizzi
            "DIRECTOR_1_ACCOUNT_ID",
            {
              "aura": "DIRECTOR_1_AURA_KEY",
              "grandpa": "DIRECTOR_1_GRANDPA_KEY",
              "asf": "DIRECTOR_1_ASF_KEY"
            }
          ],
          [
            "DIRECTOR_2_ACCOUNT_ID",  // AuditDev
            "DIRECTOR_2_ACCOUNT_ID",
            {
              "aura": "DIRECTOR_2_AURA_KEY",
              "grandpa": "DIRECTOR_2_GRANDPA_KEY",
              "asf": "DIRECTOR_2_ASF_KEY"
            }
          ],
          // ... Directors 3-9 (GRANDPA authorities)

          // VALIDITY NODES (16 validators) - NO GRANDPA KEYS
          [
            "VALIDATOR_10_ACCOUNT_ID",
            "VALIDATOR_10_ACCOUNT_ID",
            {
              "aura": "VALIDATOR_10_AURA_KEY",
              // NO grandpa key - not a finality authority
              "asf": "VALIDATOR_10_ASF_KEY"
            }
          ],
          // ... Validators 11-25 (no GRANDPA keys)
        ]
      },
      "grandpa": {
        "authorities": [
          // ONLY 9 DIRECTORS (CRITICAL FIX)
          ["DIRECTOR_1_GRANDPA_KEY", 1],
          ["DIRECTOR_2_GRANDPA_KEY", 1],
          ["DIRECTOR_3_GRANDPA_KEY", 1],
          ["DIRECTOR_4_GRANDPA_KEY", 1],
          ["DIRECTOR_5_GRANDPA_KEY", 1],
          ["DIRECTOR_6_GRANDPA_KEY", 1],
          ["DIRECTOR_7_GRANDPA_KEY", 1],
          ["DIRECTOR_8_GRANDPA_KEY", 1],
          ["DIRECTOR_9_GRANDPA_KEY", 1]
          // NO Validity Nodes (validators 10-25) in GRANDPA
        ]
      },
      "aura": {
        "authorities": [
          // ALL 25 VALIDATORS (Directors + Validity Nodes)
          "DIRECTOR_1_AURA_KEY",
          "DIRECTOR_2_AURA_KEY",
          // ... all 9 directors
          "VALIDATOR_10_AURA_KEY",
          "VALIDATOR_11_AURA_KEY",
          // ... all 16 validity nodes
        ]
      },
      "staking": {
        // Import from state_staking.json
        // Preserve staking state from block #63,274
      },
      "council": {
        // Import from state export
      },
      "treasury": {
        // Import from state export
      }
    }
  }
}
```

**CRITICAL CONFIGURATION REQUIREMENTS:**

1. **GRANDPA Authorities:** EXACTLY 9 directors, NO validity nodes
2. **AURA Authorities:** All 25 validators (directors + validity nodes)
3. **Session Keys:** Directors have GRANDPA keys, validity nodes do NOT
4. **Balances:** Imported from block #63,274 state
5. **Staking:** Preserved from block #63,274

**Session Keys Source Document:**

All session keys for the 25 validators are documented in:
```
/Users/macbook/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md
```

This master document contains:
- AURA keys (sr25519) for all 25 validators
- GRANDPA keys (ed25519) for 9 Directors only
- ASF keys (sr25519) for all 25 validators
- Account IDs and public addresses
- VM IP addresses and SSH access

**Use this document as the authoritative source when populating the genesis session keys.**

### Step 2.3: Import State from Block #63,274

```bash
# Create import script
cat > ~/Desktop/etrid/import_state_to_genesis.py << 'EOF'
#!/usr/bin/env python3
import json

# Load exported state
with open('network-restart-backup/state_export_63274.json', 'r') as f:
    state_export = json.load(f)

# Load genesis template
with open('docs/mainnet/chainspec-mainnet-v2-plain.json', 'r') as f:
    genesis = json.load(f)

# Import balances
balances = []
for item in state_export['result']:
    key = item[0]
    value = item[1]

    # Balances pallet keys start with 0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9
    if key.startswith('0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9'):
        # Decode account and balance
        # (Substrate storage key decoding logic)
        account_id = decode_account_from_key(key)
        balance = decode_balance_from_value(value)
        balances.append([account_id, balance])

genesis['genesis']['runtime']['balances']['balances'] = balances

# Write updated genesis
with open('docs/mainnet/chainspec-mainnet-v2-plain-with-state.json', 'w') as f:
    json.dump(genesis, f, indent=2)

print(f"Imported {len(balances)} account balances")
EOF

chmod +x ~/Desktop/etrid/import_state_to_genesis.py

# Run import (may need to implement decode functions)
# For now, manual import recommended for accuracy
```

**Manual Import Process:**

1. Open `state_balances.json` in text editor
2. Decode SCALE-encoded balances
3. Copy to genesis `balances.balances` array
4. Verify total supply matches expected

**Alternative: Use Existing Genesis + State Fork:**

If full state import complex, can use:
```bash
# Start with existing genesis up to block #63,274
# Then use substrate-archive or fork-off-substrate tools
# to create new genesis with state snapshot
```

### Step 2.4: Update Validator Configuration

Update with current 25-validator set:

**Directors (1-9):**
```json
{
  "validators": [
    {
      "name": "Gizzi-Director-1",
      "account": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "aura": "0x...",
      "grandpa": "0x...",
      "asf": "0x...",
      "ip": "64.181.215.19",
      "role": "Director"
    },
    {
      "name": "AuditDev-Director-2",
      "account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      "aura": "0x...",
      "grandpa": "0x...",
      "asf": "0x...",
      "ip": "129.80.122.34",
      "role": "Director"
    }
    // ... Directors 3-9
  ]
}
```

**Validity Nodes (10-25):**
```json
{
  "validity_nodes": [
    {
      "name": "Contabo-Validator-10",
      "account": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
      "aura": "0x...",
      "grandpa": null,  // NO GRANDPA KEY
      "asf": "0x...",
      "ip": "85.239.239.194",
      "role": "ValidityNode"
    }
    // ... Validators 11-25
  ]
}
```

### Step 2.5: Build Raw Chainspec

```bash
# Convert plain chainspec to raw (compiled) format
./target/release/flarechain-node build-spec \
    --chain docs/mainnet/chainspec-mainnet-v2-plain.json \
    --raw \
    --disable-default-bootnode \
    > docs/mainnet/chainspec-mainnet-v2-raw.json

# Verify raw chainspec generated
ls -lh docs/mainnet/chainspec-mainnet-v2-raw.json

# Expected size: 1-3 MB (encoded genesis state)
```

### Step 2.6: Verify Genesis Hash

```bash
# Calculate genesis hash
./target/release/flarechain-node build-spec \
    --chain docs/mainnet/chainspec-mainnet-v2-raw.json \
    | jq -r .genesis.raw.top

# Or start node and check genesis hash
./target/release/flarechain-node \
    --chain docs/mainnet/chainspec-mainnet-v2-raw.json \
    --tmp \
    --alice \
    2>&1 | grep -i "genesis"

# Expected output:
# Local node identity is: ...
# 📋 Chainspec: Ëtrid FlareChain Mainnet
# 🔨 Initializing Genesis block/state
# ⛓  Genesis hash: 0xNEW_GENESIS_HASH_HERE...
```

**CRITICAL:** New genesis hash will be DIFFERENT from original mainnet. This is expected and correct.

**Verification Checklist:**
- [ ] Plain chainspec created
- [ ] GRANDPA authorities = 9 (CRITICAL)
- [ ] AURA authorities = 25
- [ ] Session keys configured for all 25 validators
- [ ] Balances imported from block #63,274
- [ ] Raw chainspec generated
- [ ] Genesis hash calculated
- [ ] No errors during build-spec

---

## Phase 3: Testing and Validation

**Duration:** 6-12 hours
**Responsibility:** Core development + 2-3 test validators
**Prerequisites:** Raw chainspec from Phase 2

### Step 3.1: Deploy Test Network (3 Validators)

```bash
# On local machine, start test validator 1
./target/release/flarechain-node \
    --chain docs/mainnet/chainspec-mainnet-v2-raw.json \
    --base-path /tmp/test-validator-1 \
    --validator \
    --name "Test-Director-1" \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-external \
    --rpc-cors all

# Expected output:
# ⛓  Genesis hash: 0x...
# 🏷  Local node identity: 12D3KooW...
# 👶 Starting GRANDPA worker with 9 authorities

# CRITICAL CHECK: "Starting GRANDPA worker with 9 authorities" (NOT 25!)
```

Start 2 more test validators (Directors 2-3):

```bash
# Terminal 2: Test validator 2
./target/release/flarechain-node \
    --chain docs/mainnet/chainspec-mainnet-v2-raw.json \
    --base-path /tmp/test-validator-2 \
    --validator \
    --name "Test-Director-2" \
    --node-key 0000000000000000000000000000000000000000000000000000000000000002 \
    --port 30334 \
    --rpc-port 9945 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID_FROM_VALIDATOR_1

# Terminal 3: Test validator 3
./target/release/flarechain-node \
    --chain docs/mainnet/chainspec-mainnet-v2-raw.json \
    --base-path /tmp/test-validator-3 \
    --validator \
    --name "Test-Director-3" \
    --node-key 0000000000000000000000000000000000000000000000000000000000000003 \
    --port 30335 \
    --rpc-port 9946 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID_FROM_VALIDATOR_1
```

### Step 3.2: Verify Block Production

```bash
# Monitor logs for block production
# Validator 1 terminal should show:
# 💤 Idle (0 peers), best: #0 (0x...)
# 🙌 Starting consensus session on top of parent #0
# 🎁 Prepared block for proposing at 1
# ✨ Imported #1 (0x...)
# 💤 Idle (2 peers), best: #1 (0x...)

# Query current block via RPC
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"chain_getHeader"
}' http://localhost:9944 | jq .

# Expected: Block number increasing every 12 seconds
```

### Step 3.3: Verify GRANDPA Finality (CRITICAL TEST)

```bash
# Query finalized block
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"chain_getFinalizedHead"
}' http://localhost:9944 | jq -r .result

# Convert hex block hash to number
FINALIZED_HASH=$(curl -s -H "Content-Type: application/json" -d '{
    "method":"chain_getFinalizedHead"
}' http://localhost:9944 | jq -r .result)

curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"chain_getHeader\",
    \"params\":[\"$FINALIZED_HASH\"]
}" http://localhost:9944 | jq .

# Expected: Finalized block advancing (not stuck at #0)
# CRITICAL: If finalized block stuck, GRANDPA not working - DO NOT PROCEED
```

**Success Criteria:**
- [ ] Best block advancing (new block every 12 seconds)
- [ ] Finalized block advancing (every 20-30 seconds)
- [ ] Logs show "finalized #1", "finalized #2", etc.
- [ ] GRANDPA round messages visible
- [ ] Peer count = 2 (for 3-validator test)

**Failure Indicators:**
- ❌ Finalized block stuck at #0
- ❌ Logs show GRANDPA errors
- ❌ No "Starting GRANDPA worker" message
- ❌ "GRANDPA authorities: 0" or "GRANDPA authorities: 25"

### Step 3.4: Query GRANDPA Authority Set

```bash
# Verify GRANDPA authority set in runtime storage
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"state_getStorage",
    "params":["0x3a6772616e6470615f617574686f726974696573"]
}' http://localhost:9944 | jq .

# Decode result (SCALE-encoded)
# Should show 9 authority keys (not 0, not 25)
```

```bash
# Alternative: Query via grandpa RPC
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"grandpa_roundState"
}' http://localhost:9944 | jq .

# Expected output:
# {
#   "result": {
#     "round": 1,
#     "totalWeight": 9,  // 9 authorities
#     "thresholdWeight": 7,  // 2/3+1 of 9 = 7
#     "missingPrevotes": [...],
#     "missingPrecommits": [...]
#   }
# }

# CRITICAL: totalWeight should be 9 (not 25)
```

### Step 3.5: Test Session Rotation

```bash
# Wait for session change (default: every 100 blocks)
# Monitor logs for "New session" messages

# After session change, verify GRANDPA still working
curl -H "Content-Type: application/json" -d '{
    "method":"chain_getFinalizedHead"
}' http://localhost:9944 | jq .

# Expected: Finalized block still advancing after session change
```

### Step 3.6: Test with Validity Node (Non-GRANDPA)

```bash
# Start a 4th validator (simulating Validity Node)
./target/release/flarechain-node \
    --chain docs/mainnet/chainspec-mainnet-v2-raw.json \
    --base-path /tmp/test-validity-node-10 \
    --validator \
    --name "Test-ValidityNode-10" \
    --node-key 000000000000000000000000000000000000000000000000000000000000000a \
    --port 30336 \
    --rpc-port 9947 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID_FROM_VALIDATOR_1

# Expected:
# - Validity node connects to network
# - Participates in block production (AURA)
# - Does NOT participate in GRANDPA finality
# - Logs show "Idle" (not producing GRANDPA votes)
```

### Step 3.7: Verify State Import

```bash
# Query account balances
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"system_accountNextIndex",
    "params":["FOUNDATION_ACCOUNT_ID"]
}' http://localhost:9944 | jq .

# Query specific balance
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"system_account",
    "params":["VALIDATOR_ACCOUNT_ID"]
}' http://localhost:9944 | jq .

# Verify balances match exported state from block #63,274
```

**Verification Checklist:**
- [ ] 3 directors producing blocks
- [ ] GRANDPA finalizing blocks
- [ ] GRANDPA authority count = 9
- [ ] Finality advancing consistently
- [ ] Session rotations successful
- [ ] Validity node can join (non-GRANDPA)
- [ ] Account balances correct
- [ ] No GRANDPA equivocation errors
- [ ] No authority set mismatches

**If ANY test fails, DO NOT PROCEED to Phase 4. Fix issues first.**

---

## Phase 4: Distribution

**Duration:** 2-4 hours
**Responsibility:** Operations team
**Prerequisites:** Successful Phase 3 testing

### Step 4.1: Prepare Distribution Package

```bash
# Create distribution directory
mkdir -p ~/Desktop/etrid/network-restart-distribution
cd ~/Desktop/etrid/network-restart-distribution

# Copy new chainspec
cp ~/Desktop/etrid/docs/mainnet/chainspec-mainnet-v2-raw.json .

# Create checksums
sha256sum chainspec-mainnet-v2-raw.json > chainspec_checksum.txt
cat chainspec_checksum.txt

# Create README for validators
cat > RESTART_INSTRUCTIONS.md << 'EOF'
# FlareChain Mainnet Network Restart - Validator Instructions

## What You Need to Do

1. **Stop your current validator** (if running)
2. **Backup your data** (optional but recommended)
3. **Download new chainspec** (chainspec-mainnet-v2-raw.json)
4. **Update startup command** (use new chainspec path)
5. **Restart validator** (wait for coordinator signal)

## Detailed Steps

### Step 1: Stop Current Validator

```bash
sudo systemctl stop flarechain-validator
```

### Step 2: Backup Current Chain Data (Optional)

```bash
sudo mv /var/lib/etrid/chains /var/lib/etrid/chains-backup-nov9
```

### Step 3: Download New Chainspec

```bash
# File will be provided via secure channel
# Verify checksum matches:
sha256sum chainspec-mainnet-v2-raw.json
# Should match: [CHECKSUM_HERE]
```

### Step 4: Update Systemd Service

```bash
sudo nano /etc/systemd/system/flarechain-validator.service

# Update --chain flag:
ExecStart=/usr/local/bin/flarechain-node \
    --chain /var/lib/etrid/chainspec-mainnet-v2-raw.json \
    # ... rest of flags unchanged

sudo systemctl daemon-reload
```

### Step 5: Wait for Restart Signal

**DO NOT START YET!**

Wait for coordinator message on operations channel.

Directors (1-9) start first at TIME_TBD.
Validity Nodes (10-25) start 10 minutes later.

### Step 6: Start Validator

```bash
sudo systemctl start flarechain-validator
sudo journalctl -u flarechain-validator -f
```

### Expected Log Output

```
⛓  Genesis hash: 0xNEW_HASH...
🏷  Local node identity: 12D3KooW...
👶 Starting GRANDPA worker with 9 authorities  (Directors only)
💤 Idle (0 peers), best: #0
🙌 Starting consensus session on top of parent #0
✨ Imported #1
💤 Idle (5 peers), best: #10
finalized #8
```

### Verification

```bash
# Check peers connected
curl -H "Content-Type: application/json" -d '{
    "method":"system_peers"
}' http://localhost:9944 | jq '.result | length'

# Should see 8-20 peers within 30 minutes

# Check finality
curl -H "Content-Type: application/json" -d '{
    "method":"chain_getFinalizedHead"
}' http://localhost:9944

# Finalized block should be advancing
```

### Troubleshooting

**No peers connecting:**
- Check firewall (port 30333 open)
- Verify bootnode addresses correct
- Check public IP in --public-addr flag

**Finality not advancing:**
- Verify genesis hash matches others
- Check GRANDPA authority count in logs
- Ensure session keys inserted correctly

**Support:** Operations channel (24/7 during restart window)
EOF
```

### Step 4.2: Create Validator-Specific Packages

```bash
# Create startup commands for each validator

# Directors (1-9) - GRANDPA authorities
for i in {1..9}; do
    cat > director-${i}-start-command.sh << EOF
#!/bin/bash
# Director ${i} Startup Command

/usr/local/bin/flarechain-node \\
    --chain /var/lib/etrid/chainspec-mainnet-v2-raw.json \\
    --base-path /var/lib/etrid \\
    --validator \\
    --name "Director-${i}" \\
    --public-addr /ip4/DIRECTOR_${i}_IP/tcp/30333 \\
    --bootnodes "/ip4/64.181.215.19/tcp/30333/p2p/BOOTNODE_PEER_ID" \\
    --port 30333 \\
    --rpc-port 9944 \\
    --prometheus-port 9615
EOF
    chmod +x director-${i}-start-command.sh
done

# Validity Nodes (10-25) - Non-GRANDPA
for i in {10..25}; do
    cat > validator-${i}-start-command.sh << EOF
#!/bin/bash
# Validity Node ${i} Startup Command

/usr/local/bin/flarechain-node \\
    --chain /var/lib/etrid/chainspec-mainnet-v2-raw.json \\
    --base-path /var/lib/etrid \\
    --validator \\
    --name "ValidityNode-${i}" \\
    --public-addr /ip4/VALIDATOR_${i}_IP/tcp/30333 \\
    --bootnodes "/ip4/64.181.215.19/tcp/30333/p2p/BOOTNODE_PEER_ID" \\
    --port 30333 \\
    --rpc-port 9944 \\
    --prometheus-port 9615
EOF
    chmod +x validator-${i}-start-command.sh
done
```

### Step 4.3: Distribute to Validators

**Directors (1-9):**
```bash
# Gizzi (Director 1)
scp -i ~/.ssh/gizzi-validator \
    chainspec-mainnet-v2-raw.json \
    RESTART_INSTRUCTIONS.md \
    director-1-start-command.sh \
    ubuntu@64.181.215.19:~/

# AuditDev (Director 2)
scp -i ~/.ssh/gizzi-validator \
    chainspec-mainnet-v2-raw.json \
    RESTART_INSTRUCTIONS.md \
    director-2-start-command.sh \
    ubuntu@129.80.122.34:~/

# Directors 3-9 (similar)
# ...
```

**Validity Nodes (10-25):**
```bash
# Contabo Validator 10
scp -i ~/.ssh/contabo-validators \
    chainspec-mainnet-v2-raw.json \
    RESTART_INSTRUCTIONS.md \
    validator-10-start-command.sh \
    root@85.239.239.194:~/

# Validators 11-25 (loop through all)
CONTABO_IPS=(
    "85.239.239.194"  # Val 10
    "85.239.239.193"  # Val 11
    "85.239.239.190"  # Val 12
    # ... all 16 IPs
)

for i in {0..15}; do
    VAL_NUM=$((i + 10))
    scp -i ~/.ssh/contabo-validators \
        chainspec-mainnet-v2-raw.json \
        RESTART_INSTRUCTIONS.md \
        validator-${VAL_NUM}-start-command.sh \
        root@${CONTABO_IPS[$i]}:~/
done
```

### Step 4.4: Verify Distribution

```bash
# Create verification script
cat > verify_distribution.sh << 'EOF'
#!/bin/bash
# Verify all validators received new chainspec

VALIDATORS=(
    "ubuntu@64.181.215.19"    # Gizzi
    "ubuntu@129.80.122.34"    # AuditDev
    # ... all 25 validators
)

EXPECTED_CHECKSUM="[CHECKSUM_FROM_PHASE_4.1]"

for val in "${VALIDATORS[@]}"; do
    echo "Checking $val..."
    REMOTE_CHECKSUM=$(ssh $val "sha256sum ~/chainspec-mainnet-v2-raw.json | cut -d' ' -f1")

    if [ "$REMOTE_CHECKSUM" == "$EXPECTED_CHECKSUM" ]; then
        echo "✅ $val - OK"
    else
        echo "❌ $val - CHECKSUM MISMATCH!"
    fi
done
EOF

chmod +x verify_distribution.sh
./verify_distribution.sh
```

**Distribution Checklist:**
- [ ] Chainspec uploaded to all 25 validators
- [ ] Checksums verified on all validators
- [ ] Restart instructions provided
- [ ] Startup commands customized per validator
- [ ] All validators acknowledged receipt
- [ ] Restart time coordinated

---

## Phase 5: Network Restart

**Duration:** 2-4 hours
**Responsibility:** Operations coordinator + all validators
**Prerequisites:** All validators ready with new chainspec

### Step 5.1: Pre-Restart Coordination

```bash
# Announce restart window (e.g., November 13, 2025 14:00 UTC)
# Send message to all validator operators:

SUBJECT: FlareChain Mainnet Network Restart - CONFIRMED TIME

Restart Time: November 13, 2025 at 14:00 UTC
Your Local Time: [CONVERTED_TIME]

Phase 1 (14:00 UTC): Directors 1-9 start
Phase 2 (14:15 UTC): Validity Nodes 10-25 start
Phase 3 (14:30-16:00 UTC): Monitoring and verification

Please be ready 15 minutes before your phase.
Operations channel will be active for real-time support.
```

### Step 5.2: Stop All Current Validators

**15 minutes before restart (13:45 UTC):**

```bash
# All validators execute:
sudo systemctl stop flarechain-validator

# Verify stopped:
sudo systemctl status flarechain-validator
# Should show: "inactive (dead)"
```

### Step 5.3: Phase 1 - Start Directors (14:00 UTC)

**Directors 1-9 ONLY:**

```bash
# Update systemd service to use new chainspec
sudo nano /etc/systemd/system/flarechain-validator.service

# Change --chain flag:
ExecStart=/usr/local/bin/flarechain-node \
    --chain /var/lib/etrid/chainspec-mainnet-v2-raw.json \
    # ... rest unchanged

sudo systemctl daemon-reload

# Start validator
sudo systemctl start flarechain-validator

# Monitor logs
sudo journalctl -u flarechain-validator -f
```

**Expected logs for Directors:**
```
⛓  Genesis hash: 0xNEW_GENESIS_HASH...
🏷  Local node identity: 12D3KooW...
👶 Starting GRANDPA worker with 9 authorities
💤 Idle (0 peers), best: #0
💤 Idle (1 peers), best: #0
💤 Idle (3 peers), best: #0
🙌 Starting consensus session on top of parent #0
🎁 Prepared block for proposing at 1
✨ Imported #1 (0x...)
💤 Idle (5 peers), best: #1
finalized #1
finalized #2
```

**CRITICAL CHECKS:**
- [ ] "Starting GRANDPA worker with 9 authorities" (NOT 0, NOT 25)
- [ ] Peer count increasing (should reach 8 within 5 minutes)
- [ ] Block production active (new block every 12 seconds)
- [ ] Finality advancing ("finalized #1", "finalized #2", etc.)

**If ANY director sees errors, STOP and troubleshoot before Phase 2.**

### Step 5.4: Verify Director Consensus (14:10 UTC)

```bash
# On coordination machine, query all directors

DIRECTORS=(
    "64.181.215.19:9944"     # Director 1 (Gizzi)
    "129.80.122.34:9944"     # Director 2 (AuditDev)
    # ... Directors 3-9
)

for dir in "${DIRECTORS[@]}"; do
    echo "Checking $dir..."

    # Get best block
    BEST=$(curl -s -H "Content-Type: application/json" -d '{
        "method":"chain_getHeader"
    }' http://$dir | jq -r '.result.number')

    # Get finalized block
    FIN_HASH=$(curl -s -H "Content-Type: application/json" -d '{
        "method":"chain_getFinalizedHead"
    }' http://$dir | jq -r .result)

    FIN=$(curl -s -H "Content-Type: application/json" -d "{
        \"method\":\"chain_getHeader\",
        \"params\":[\"$FIN_HASH\"]
    }" http://$dir | jq -r '.result.number')

    # Get peer count
    PEERS=$(curl -s -H "Content-Type: application/json" -d '{
        "method":"system_peers"
    }' http://$dir | jq '.result | length')

    echo "$dir: Best=$BEST, Finalized=$FIN, Peers=$PEERS"
done

# Expected output:
# 64.181.215.19:9944: Best=0xa (10), Finalized=0x8 (8), Peers=8
# 129.80.122.34:9944: Best=0xa (10), Finalized=0x8 (8), Peers=8
# ... (all should have similar block heights)
```

**GO/NO-GO Decision:**

✅ **GO to Phase 2 if:**
- All 9 directors online
- Best block > 5 on all directors
- Finalized block advancing
- Peer count 7-8 on all directors
- No GRANDPA errors in logs

❌ **NO-GO if:**
- Any director stuck at block #0
- Finalized block not advancing
- GRANDPA errors visible
- Peer count < 5
- Authority count ≠ 9

### Step 5.5: Phase 2 - Start Validity Nodes (14:15 UTC)

**After confirming directors healthy, proceed with Validity Nodes:**

**Validity Nodes 10-25:**

```bash
# Update systemd service
sudo nano /etc/systemd/system/flarechain-validator.service

# Change --chain flag:
ExecStart=/usr/local/bin/flarechain-node \
    --chain /var/lib/etrid/chainspec-mainnet-v2-raw.json \
    # ... rest unchanged

sudo systemctl daemon-reload

# Start validator
sudo systemctl start flarechain-validator

# Monitor logs
sudo journalctl -u flarechain-validator -f
```

**Expected logs for Validity Nodes:**
```
⛓  Genesis hash: 0xNEW_GENESIS_HASH...  (same as Directors)
🏷  Local node identity: 12D3KooW...
💤 Idle (0 peers), best: #0
💤 Syncing 5.0 bps, target=#20 (8 peers)
✨ Imported #1 (0x...)
✨ Imported #2 (0x...)
💤 Syncing 15.0 bps, target=#25 (12 peers)
💤 Idle (15 peers), best: #25
✨ Imported #26 (0x...)
🙌 Starting consensus session on top of parent #26
🎁 Prepared block for proposing at 27
```

**NOTE:** Validity Nodes will NOT show "Starting GRANDPA worker" - this is correct!

### Step 5.6: Monitor Network Convergence (14:20-14:45 UTC)

```bash
# Check all 25 validators status
cat > check_all_validators.sh << 'EOF'
#!/bin/bash

ALL_VALIDATORS=(
    "64.181.215.19:9944"     # Director 1
    "129.80.122.34:9944"     # Director 2
    # ... all 25 validators
)

echo "Validator Status Report - $(date)"
echo "=========================================="

for val in "${ALL_VALIDATORS[@]}"; do
    NAME=$(echo $val | cut -d: -f1)

    # Get best block
    BEST=$(curl -s -m 5 -H "Content-Type: application/json" -d '{
        "method":"chain_getHeader"
    }' http://$val 2>/dev/null | jq -r '.result.number // "ERROR"')

    # Get peer count
    PEERS=$(curl -s -m 5 -H "Content-Type: application/json" -d '{
        "method":"system_health"
    }' http://$val 2>/dev/null | jq -r '.result.peers // 0')

    # Get sync status
    SYNC=$(curl -s -m 5 -H "Content-Type: application/json" -d '{
        "method":"system_health"
    }' http://$val 2>/dev/null | jq -r '.result.isSyncing // false')

    if [ "$BEST" != "ERROR" ]; then
        echo "✅ $NAME: Block=$BEST, Peers=$PEERS, Syncing=$SYNC"
    else
        echo "❌ $NAME: UNREACHABLE"
    fi
done
EOF

chmod +x check_all_validators.sh

# Run every 2 minutes
while true; do
    ./check_all_validators.sh
    sleep 120
done
```

**Success Indicators:**
- [ ] All 25 validators online
- [ ] Best block within 5 blocks of each other
- [ ] Finalized block advancing on directors
- [ ] Peer count 15-24 on most validators
- [ ] No ERROR or UNREACHABLE validators

---

## Phase 6: Verification

**Duration:** 24-48 hours
**Responsibility:** Operations team + monitoring
**Prerequisites:** All 25 validators online

### Step 6.1: Initial Verification (First 30 Minutes)

```bash
# 1. Verify GRANDPA finality
curl -H "Content-Type: application/json" -d '{
    "method":"chain_getFinalizedHead"
}' http://64.181.215.19:9944 | jq .

# Check finalized block advancing every 30 seconds
# Should increase by 1-2 blocks per check

# 2. Verify GRANDPA authority count
curl -H "Content-Type: application/json" -d '{
    "method":"grandpa_roundState"
}' http://64.181.215.19:9944 | jq .

# Expected: totalWeight = 9 (NOT 25)

# 3. Query all validator session keys
curl -H "Content-Type: application/json" -d '{
    "method":"author_hasSessionKeys",
    "params":["0x..."]  # Director session keys
}' http://64.181.215.19:9944 | jq .

# Should return true for all 9 directors
```

### Step 6.2: Block Production Verification

```bash
# Monitor block authorship for 10 minutes
for i in {1..50}; do
    BLOCK=$(curl -s -H "Content-Type: application/json" -d '{
        "method":"chain_getHeader"
    }' http://64.181.215.19:9944 | jq -r .result.number)

    AUTHOR=$(curl -s -H "Content-Type: application/json" -d '{
        "method":"chain_getBlock"
    }' http://64.181.215.19:9944 | jq -r .result.block.header.author)

    echo "Block $BLOCK produced by $AUTHOR"
    sleep 12
done

# Verify:
# - All 25 validators producing blocks (round-robin via AURA)
# - No single validator dominating
# - Block times ~12 seconds consistent
```

### Step 6.3: State Verification

```bash
# Verify account balances match exported state
FOUNDATION_ACCOUNT="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"system_account\",
    \"params\":[\"$FOUNDATION_ACCOUNT\"]
}" http://64.181.215.19:9944 | jq .

# Compare balance to exported state from Phase 1
# Should match block #63,274 state

# Verify total issuance
curl -H "Content-Type: application/json" -d '{
    "method":"state_call",
    "params":["Balances_total_issuance", "0x"]
}' http://64.181.215.19:9944 | jq .

# Should match expected total from genesis
```

### Step 6.4: Era/Epoch Monitoring

```bash
# Wait for first era change (typically 24 hours)
# Monitor logs for session rotation

# Check session keys updated
curl -H "Content-Type: application/json" -d '{
    "method":"session_validators"
}' http://64.181.215.19:9944 | jq .

# Should return 25 validator account IDs

# Verify rewards distribution after first era
curl -H "Content-Type: application/json" -d '{
    "method":"state_call",
    "params":["Staking_eras_reward_points", "0x00"]
}' http://64.181.215.19:9944 | jq .
```

### Step 6.5: 24-Hour Health Check

```bash
# Create monitoring dashboard
cat > network_health_dashboard.sh << 'EOF'
#!/bin/bash

clear
echo "=== FlareChain Mainnet Health Dashboard ==="
echo "Updated: $(date)"
echo ""

# Best block
BEST=$(curl -s -H "Content-Type: application/json" -d '{
    "method":"chain_getHeader"
}' http://64.181.215.19:9944 | jq -r .result.number)
echo "Best Block: $BEST"

# Finalized block
FIN_HASH=$(curl -s -H "Content-Type: application/json" -d '{
    "method":"chain_getFinalizedHead"
}' http://64.181.215.19:9944 | jq -r .result)

FIN=$(curl -s -H "Content-Type: application/json" -d "{
    \"method\":\"chain_getHeader\",
    \"params\":[\"$FIN_HASH\"]
}" http://64.181.215.19:9944 | jq -r .result.number)
echo "Finalized Block: $FIN"
echo "Finality Lag: $((BEST - FIN)) blocks"

# Peer count
PEERS=$(curl -s -H "Content-Type: application/json" -d '{
    "method":"system_peers"
}' http://64.181.215.19:9944 | jq '.result | length')
echo "Peers Connected: $PEERS"

# GRANDPA round state
ROUND=$(curl -s -H "Content-Type: application/json" -d '{
    "method":"grandpa_roundState"
}' http://64.181.215.19:9944 | jq -r .result.round)
echo "GRANDPA Round: $ROUND"

# Health check
HEALTH=$(curl -s -H "Content-Type: application/json" -d '{
    "method":"system_health"
}' http://64.181.215.19:9944 | jq -r .result)
echo "Node Health: $HEALTH"

echo ""
echo "==========================================="
EOF

chmod +x network_health_dashboard.sh

# Run continuously
watch -n 60 ./network_health_dashboard.sh
```

### Step 6.6: Session Management Verification (NEW)

**Purpose:** Verify proper session key management is working for future validator onboarding

```bash
# 1. Verify session pallet is operational
echo "=== Checking Session Pallet Configuration ==="

# Check runtime metadata includes session module
curl -s -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"state_getMetadata",
    "params":[]
}' http://64.181.215.19:9944 | jq -r '.result' | xxd -r -p | strings | grep -i "session"

# Expected output: Should show "session" pallet with methods like:
# - setKeys
# - purgeKeys
# - validators

# 2. Verify author_rotateKeys RPC is available
echo ""
echo "=== Testing author_rotateKeys RPC ==="
curl -s -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"rpc_methods",
    "params":[]
}' http://64.181.215.19:9944 | jq -r '.result.methods[]' | grep -i "author"

# Expected: Should show author_rotateKeys, author_hasSessionKeys, author_insertKey

# 3. Test session key query for a Director
echo ""
echo "=== Querying Session Keys for Director 1 (Gizzi) ==="

# Get session keys from chain state
curl -s -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"state_call",
    "params":["SessionKeys_decode_session_keys", "0x..."]
}' http://64.181.215.19:9944 | jq .

# 4. Verify all 25 validators have session keys registered
echo ""
echo "=== Verifying All Validators Have Session Keys ==="
curl -s -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"state_call",
    "params":["SessionApi_session_keys", []]
}' http://64.181.215.19:9944 | jq .

# Should return session keys for all 25 validators
```

**Verification Checklist:**
- [ ] Session pallet visible in runtime metadata
- [ ] `author_rotateKeys` RPC method available
- [ ] `author_hasSessionKeys` RPC method available
- [ ] All 25 validators have session keys registered on-chain
- [ ] Session keys match those in genesis configuration
- [ ] No errors when querying session state

### Step 6.7: Future Validator Onboarding Process Documentation

**Document proper onboarding for future validators joining the network:**

```markdown
## Future Validator Onboarding Process

After the network restart, future validators can join using the proper Substrate workflow:

### Prerequisites
- Validator node fully synced to FlareChain mainnet
- Controller account with sufficient ETR for transaction fees
- Staking requirements met (64K ETR for Validity Node, 128K ETR for Director)

### Step 1: Generate Session Keys

SSH to your validator node and generate session keys via RPC:

```bash
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"author_rotateKeys",
    "params":[]
}' http://localhost:9944

# Returns: "0x<hex_encoded_session_keys>"
# Example: "0xabcd1234...7890" (concatenated AURA + GRANDPA + ASF keys)
```

**Save this hex string - you'll need it for Step 2.**

### Step 2: Register Session Keys On-Chain

Using polkadot.js or CLI tool, submit the `session.setKeys` extrinsic:

```javascript
// Using polkadot.js
const sessionKeys = "0xabcd1234...7890"; // From Step 1
const proof = null; // No proof needed

await api.tx.session.setKeys(sessionKeys, proof)
    .signAndSend(controllerAccount);
```

Or via CLI:
```bash
# Using substrate-api-cli or polkadot-js-cli
polkadot-js-api tx.session.setKeys \
    --seed "YOUR_CONTROLLER_SEED" \
    --params '["0xabcd1234...7890", null]' \
    --ws ws://64.181.215.19:9944
```

### Step 3: Wait for Session Rotation

Session keys take effect in the **next session/era** after registration.

Monitor when your keys become active:
```bash
# Check current session
curl -s -H "Content-Type: application/json" -d '{
    "method":"session_currentIndex"
}' http://localhost:9944 | jq .

# Check when next session starts
curl -s -H "Content-Type: application/json" -d '{
    "method":"session_nextKeys",
    "params":["YOUR_ACCOUNT_ID"]
}' http://localhost:9944 | jq .
```

### Step 4: Verify Participation

After session rotation, verify your validator is participating:

```bash
# Check if your validator is in active set
curl -s -H "Content-Type: application/json" -d '{
    "method":"session_validators"
}' http://localhost:9944 | jq '.result' | grep "YOUR_ACCOUNT_ID"

# For Directors: Check GRANDPA participation
curl -s -H "Content-Type: application/json" -d '{
    "method":"grandpa_roundState"
}' http://localhost:9944 | jq '.result.setId'
```

### Benefits of This Approach

✅ **No Manual Keystore Files:** Keys generated and registered via RPC
✅ **On-Chain Registration:** Keys stored in blockchain state
✅ **Proper Session Management:** Keys rotate cleanly during era transitions
✅ **Future-Proof:** Standard Substrate validator workflow
✅ **No Coordinator Needed:** Validators self-register independently

### Troubleshooting

**Issue:** `author_rotateKeys` returns error
- **Fix:** Ensure validator node running with `--rpc-methods=Unsafe` flag (validator nodes only!)

**Issue:** `session.setKeys` extrinsic fails
- **Fix:** Ensure controller account has sufficient ETR for transaction fee (~0.01 ETR)

**Issue:** Keys not taking effect after multiple sessions
- **Fix:** Verify keys registered correctly: `session.nextKeys(accountId)`
- Check session module logs for errors

---

**END OF FUTURE VALIDATOR ONBOARDING DOCUMENTATION**
```

### Step 6.8: Test Session Key Registration (Optional - Post-Restart)

**Optional test to confirm session management working:**

```bash
# Test on a non-critical validator (e.g., one of the St. Louis validators)
# This verifies the onboarding process works correctly

echo "=== Testing Session Key Re-Registration ==="
echo "Testing on: stlouis-vn01 (Validator #22)"
echo ""

# 1. SSH to test validator
ssh root@VALIDATOR_22_IP

# 2. Generate NEW session keys
NEW_KEYS=$(curl -s -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"author_rotateKeys",
    "params":[]
}' http://localhost:9944 | jq -r .result)

echo "New session keys generated: $NEW_KEYS"

# 3. Submit setKeys extrinsic (from controller account)
# This requires funded controller account and signed transaction
echo "Submit session.setKeys extrinsic with keys: $NEW_KEYS"
echo "Waiting for next session..."

# 4. Monitor session rotation
watch -n 30 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d "{\"method\":\"session_currentIndex\"}" | jq .result'

# 5. Verify new keys active after rotation
echo "After session rotation, verify new keys active"
```

**Success Criteria:**
- [ ] `author_rotateKeys` generates new keys successfully
- [ ] `session.setKeys` extrinsic executes without error
- [ ] New keys take effect in next session
- [ ] Validator continues producing blocks with new keys
- [ ] No GRANDPA errors or equivocation

**24-Hour Verification Checklist:**
- [ ] Finality continuously advancing (no stalls)
- [ ] All 25 validators online
- [ ] Block production distributed across all validators
- [ ] No GRANDPA equivocation errors
- [ ] Era transitions successful
- [ ] Peer count stable (15-24 peers)
- [ ] No network partitions detected
- [ ] Account balances correct
- [ ] No runtime panics or errors
- [ ] Session pallet operational ✨ **NEW**
- [ ] `author_rotateKeys` RPC available ✨ **NEW**
- [ ] Session key registration tested ✨ **NEW**

---

## Rollback Procedures

### When to Rollback

**Immediate rollback if:**
- GRANDPA finality stalled for > 10 minutes
- Authority count shows 0 or wrong number
- Multiple validators show equivocation errors
- Network partition detected (validators on different chains)
- Critical state corruption detected

**DO NOT rollback for:**
- Single validator offline (network tolerates 33% down)
- Temporary peer connection issues
- Minor log warnings
- Individual validator configuration errors

### Rollback Procedure

```bash
# STOP ALL VALIDATORS IMMEDIATELY
# Send emergency message to all operators:

SUBJECT: EMERGENCY - NETWORK RESTART ROLLBACK

ALL VALIDATORS: STOP YOUR NODES IMMEDIATELY

sudo systemctl stop flarechain-validator

DO NOT RESTART until further notice.
Emergency investigation in progress.

# Investigate issue
# Fix genesis configuration
# Return to Phase 2 (Genesis Creation)
# Re-test in Phase 3
# Schedule new restart date
```

### Emergency Contacts

**Operations Coordinator:** [CONTACT_INFO]
**Core Developer (Genesis):** [CONTACT_INFO]
**Infrastructure Lead:** [CONTACT_INFO]

---

## Troubleshooting

### Issue: Finality Not Advancing

**Symptoms:**
- Best block increasing but finalized block stuck
- Logs show "GRANDPA voter not in set"

**Diagnosis:**
```bash
# Check GRANDPA authority count
curl -H "Content-Type: application/json" -d '{
    "method":"grandpa_roundState"
}' http://localhost:9944 | jq .

# If totalWeight != 9, GRANDPA authority config wrong
```

**Fix:**
- Return to Phase 2, correct GRANDPA authorities in genesis
- Ensure EXACTLY 9 directors in grandpa.authorities array
- Rebuild raw chainspec
- Restart testing (Phase 3)

---

### Issue: Wrong Genesis Hash

**Symptoms:**
- Validator can't sync, shows different genesis hash in logs

**Diagnosis:**
```bash
# On affected validator, check genesis hash
grep "Genesis hash" /var/log/syslog | tail -1

# Compare to expected genesis hash from Phase 2
```

**Fix:**
```bash
# Re-download correct chainspec
scp coordinator@SERVER:chainspec-mainnet-v2-raw.json ~/
sha256sum chainspec-mainnet-v2-raw.json  # Verify checksum

# Update service, restart
sudo systemctl restart flarechain-validator
```

---

### Issue: No Peers Connecting

**Symptoms:**
- Validator stuck at "0 peers" for > 5 minutes

**Diagnosis:**
```bash
# Check firewall
sudo ufw status
# Port 30333 should be ALLOW

# Check bootnode connectivity
curl -H "Content-Type: application/json" -d '{
    "method":"system_peers"
}' http://64.181.215.19:9944 | jq .

# Verify bootnode reachable
nc -zv 64.181.215.19 30333
```

**Fix:**
```bash
# Open firewall
sudo ufw allow 30333/tcp

# Verify public IP correct in --public-addr flag
# Restart validator
sudo systemctl restart flarechain-validator
```

---

### Issue: GRANDPA Equivocation Errors

**Symptoms:**
- Logs show "Detected prevote equivocation"

**Diagnosis:**
```bash
# Check which validator is equivocating
grep -i "equivocation" /var/log/syslog
# Look for Public(...) identity in error message
```

**Fix:**
- This should NOT happen with new genesis
- If occurs, indicates validator has wrong session keys or chain state
- Stop affected validator
- Purge chain database: `flarechain-node purge-chain`
- Restart with fresh sync from network

---

## Success Criteria Summary

**Network restart considered SUCCESSFUL when:**

1. **Finality Working:**
   - [ ] GRANDPA finalizing blocks continuously
   - [ ] Finality lag < 10 blocks
   - [ ] No finality stalls for 24 hours

2. **Validator Participation:**
   - [ ] All 25 validators online
   - [ ] All validators producing blocks
   - [ ] 9 directors participating in GRANDPA
   - [ ] 16 validity nodes producing blocks via AURA

3. **Configuration Correct:**
   - [ ] GRANDPA authority count = 9
   - [ ] Session keys correct for all 25 validators
   - [ ] Genesis hash consistent across all validators

4. **State Preserved:**
   - [ ] Account balances match block #63,274 export
   - [ ] Total issuance correct
   - [ ] No state corruption detected

5. **Network Health:**
   - [ ] Era transitions successful
   - [ ] Peer connectivity healthy (15-24 peers)
   - [ ] No network partitions
   - [ ] No equivocation errors

6. **Performance:**
   - [ ] Block time ~12 seconds consistent
   - [ ] Finality time < 30 seconds average
   - [ ] No runtime panics or errors

---

**Document Version:** 1.0
**Last Updated:** November 9, 2025
**Next Review:** After successful restart (November 15, 2025)

**Authorized for Execution:** Pending final approval
**Operations Coordinator:** [NAME]
**Emergency Contact:** [24/7 PHONE]
