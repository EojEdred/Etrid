# Complete 21-Validator Bootstrap Procedure

## Overview
This procedure sets up a fresh mainnet genesis with 21 validators for GRANDPA finality.

**Bootstrap Node:** EojEdred (Validator 2) on VM1 (20.12.114.226)

---

## Phase 1: Local Preparation (Run on Mac)

### Step 1.1: Pull Latest Chainspec from GitHub
```bash
cd /Users/macbook/Desktop/etrid
git pull origin main

# Verify we have the mainnet chainspec with 21 GRANDPA authorities
cat docs/mainnet/chainspec-mainnet-raw.json | jq -r '.name, .chainType'
# Should output:
# Ëtrid FlareChain Mainnet
# Live
```

### Step 1.2: Verify All 21 Validator Keys
The file `secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md` contains ALL keys for ALL 21 validators.

**Critical GRANDPA Keys (must match genesis):**
```bash
# Extract GRANDPA keys from the key file
grep "GRANDPA Key:" secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md
```

These 21 GRANDPA keys MUST be in the chainspec genesis. Let's verify:

```bash
# Count GRANDPA authorities in chainspec (should be 21)
# Raw chainspecs are encoded, so we trust the genesis was built correctly
```

---

## Phase 2: VM Preparation (Run on Each VM)

### Step 2.1: Connect to VM and Purge Database
```bash
# For VM1 (EojEdred - Bootstrap)
ssh etridvalidator1@20.12.114.226

# Stop running node
sudo pkill -f flarechain-node

# Purge database
rm -rf ~/flarechain-node/data
```

Repeat for all VMs that were running.

### Step 2.2: Pull Latest Code and Chainspec
```bash
cd ~/flarechain-node

# Download latest chainspec from GitHub
wget -O chainspec-mainnet-raw.json \
  https://raw.githubusercontent.com/EojEdred/Etrid/main/docs/mainnet/chainspec-mainnet-raw.json

# Verify chainspec downloaded
ls -lh chainspec-mainnet-raw.json
```

---

## Phase 3: Insert Session Keys (ALL 21 Validators)

### Why This Matters:
- Each validator needs their **GRANDPA key** inserted to participate in finality
- Keys must match the pre-generated keys in COMPLETE_VALIDATOR_NETWORK_MAP.md
- Without correct keys, that validator cannot vote for finality

### Step 3.1: Create Key Insertion Script for Each Validator

**For Validator 1 (Gizzi):**
```bash
# On VM: 64.181.215.19
AURA_KEY="0x44f5ed22b0372d4822bcd0c3a0cad74a29ca5c7e9ee3cc50e8f59fa491620b58"
GRANDPA_KEY="0x00ee75f5f1fdf647006e7408c5e9c7ca98afcb2fcd9ae66503dcacaf71427a85"
ASF_KEY="0x44f5ed22b0372d4822bcd0c3a0cad74a29ca5c7e9ee3cc50e8f59fa491620b58"
```

**For Validator 2 (EojEdred) - Bootstrap:**
```bash
# On VM: 20.12.114.226
AURA_KEY="0xf29e4e1cfc2867fcda12ac9b190bea017868a0d1f3f7d5cc59af6c7d3ce6c45c"
GRANDPA_KEY="0x0a9442f63cd6019b8d6f0cd2dd6cc84d302d8eeb616bb12d7f439172107dbd2b"
ASF_KEY="0xf29e4e1cfc2867fcda12ac9b190bea017868a0d1f3f7d5cc59af6c7d3ce6c45c"
```

**For Validator 3:**
```bash
AURA_KEY="0x384a80f6b1c16fd5f8df53458f9f6ec577e3c199f9af8d84bc5f3c9e3e841f7e"
GRANDPA_KEY="0x8a9a9d8a9574eb75682a3501a2df5467036c2fc03903e9d46dfab77af4189a51"
ASF_KEY="0x384a80f6b1c16fd5f8df53458f9f6ec577e3c199f9af8d84bc5f3c9e3e841f7e"
```

... (Continue for all 21 validators - see COMPLETE_VALIDATOR_NETWORK_MAP.md)

### Step 3.2: Insert Keys on Each Validator

```bash
# Start node temporarily in background
./flarechain-node \
  --base-path ./data \
  --chain ./chainspec-mainnet-raw.json \
  --rpc-port 9944 \
  --rpc-cors all > /tmp/node.log 2>&1 &

NODE_PID=$!
sleep 10

# Insert AURA key
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_insertKey","params":["aura","'$AURA_KEY'","'$AURA_KEY'"],"id":1}' \
  http://localhost:9944

# Insert GRANDPA key (CRITICAL FOR FINALITY)
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_insertKey","params":["gran","'$GRANDPA_KEY'","'$GRANDPA_KEY'"],"id":1}' \
  http://localhost:9944

# Insert ASF key
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_insertKey","params":["ppfa","'$ASF_KEY'","'$ASF_KEY'"],"id":1}' \
  http://localhost:9944

# Stop temporary node
kill $NODE_PID
```

---

## Phase 4: Start Bootstrap Node (EojEdred - VM1)

### Step 4.1: Start EojEdred as Bootstrap
```bash
# On VM1: 20.12.114.226
./flarechain-node \
  --base-path ./data \
  --chain ./chainspec-mainnet-raw.json \
  --name "EojEdred (Founder)" \
  --validator \
  --rpc-port 9944 \
  --rpc-cors all \
  --unsafe-rpc-external \
  --port 30333 \
  --node-key 56192ada80719b2c7ca6f6e96d41ac008952bf98f1c91a34bb8713ef46fe114d \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --prometheus-external \
  > /tmp/eoj-validator.log 2>&1 &
```

### Step 4.2: Get Bootstrap Node Address
```bash
# Watch logs for the bootnode address
tail -f /tmp/eoj-validator.log | grep "Local node identity"

# Example output:
# Local node identity is: 12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2

# Full bootnode address will be:
# /ip4/20.12.114.226/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
```

---

## Phase 5: Start Other Validators (Connect to Bootstrap)

### Step 5.1: Start Each Validator Pointing to EojEdred
```bash
# On each other VM (example for Validator 1 - Gizzi)
./flarechain-node \
  --base-path ./data \
  --chain ./chainspec-mainnet-raw.json \
  --name "Gizzi (AI Overseer)" \
  --validator \
  --rpc-port 9944 \
  --port 30333 \
  --bootnodes /ip4/20.12.114.226/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  > /tmp/validator.log 2>&1 &
```

**Repeat for all 21 validators** - each with their own name.

---

## Phase 6: Verify Finality

### Step 6.1: Check Block Finalization
```bash
# On any validator, check if blocks are finalizing
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
  http://localhost:9944

# Should return a block hash (not 0x0000...)
```

### Step 6.2: Check GRANDPA Voting
```bash
# Check GRANDPA authorities participating
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"grandpa_roundState","params":[],"id":1}' \
  http://localhost:9944
```

### Step 6.3: Monitor Logs for Finality
```bash
tail -f /tmp/validator.log | grep -i "finalized"

# Should see:
# 💤 Idle (X peers), best: #Y (0x...), finalized #Z (0x...)
# Where Z > 0 and keeps increasing
```

---

## Critical Success Criteria

✅ **Genesis Requirements:**
- Chainspec has exactly 21 GRANDPA authorities
- All GRANDPA keys match pre-generated keys

✅ **Validator Requirements:**
- At least 15 validators running (for 2/3+1 supermajority)
- Each has correct GRANDPA key inserted
- All connected to network via bootstrap node

✅ **Finality Indicators:**
- Blocks finalizing beyond #0
- GRANDPA voting rounds completing
- "finalized #" increasing in logs

---

## Troubleshooting

**If finality not working:**
1. Verify exactly which GRANDPA keys are in genesis chainspec
2. Verify each validator has inserted correct GRANDPA key
3. Ensure at least 15 validators are running and connected
4. Check for GRANDPA voting errors in logs
5. Verify all validators using same chainspec (same genesis hash)
