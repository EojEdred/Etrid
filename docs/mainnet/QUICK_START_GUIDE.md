# Quick Start: Bootstrap FlareChain Mainnet with 21 Validators

## Current Situation
- Mainnet genesis has 21 GRANDPA authorities defined
- We have ALL 21 validators' session keys (see `ALL_21_VALIDATOR_KEYS.md`)
- VMs are currently offline
- Need fresh database start with correct keys for finality

---

## Step-by-Step Procedure

### 1. Start Your Azure VMs
First, make sure your VMs are running and accessible via SSH.

Test connectivity:
```bash
ssh etridvalidator1@20.12.114.226
ssh etridvalidator2@172.174.138.167
ssh etridvalidator3@172.174.151.33
ssh etridvalidator4@172.174.155.60
```

---

### 2. Purge Old Databases on All VMs

Run this on your Mac to purge all VMs:
```bash
cd /Users/macbook/Desktop/etrid
chmod +x scripts/1-purge-all-vms.sh
./scripts/1-purge-all-vms.sh
```

Or manually on each VM:
```bash
# SSH into each VM
ssh etridvalidator1@20.12.114.226

# Stop node and purge
sudo pkill -f flarechain-node
rm -rf ~/flarechain-node/data
mkdir -p ~/flarechain-node/data
```

---

### 3. Deploy Chainspec to All VMs

From your Mac:
```bash
# First make sure you have the latest
cd /Users/macbook/Desktop/etrid
git pull origin main

# Then deploy to all VMs
chmod +x scripts/2-deploy-chainspec.sh
./scripts/2-deploy-chainspec.sh
```

---

### 4. Insert Session Keys on Each Validator

**CRITICAL:** Each validator MUST have their correct GRANDPA key inserted!

Reference: `ALL_21_VALIDATOR_KEYS.md` has all keys.

**Example for Validator 2 (EojEdred) on VM1:**

```bash
# SSH into VM1
ssh etridvalidator1@20.12.114.226

# Set the keys for EojEdred (Validator 2)
AURA_KEY="0xf29e4e1cfc2867fcda12ac9b190bea017868a0d1f3f7d5cc59af6c7d3ce6c45c"
GRANDPA_KEY="0x0a9442f63cd6019b8d6f0cd2dd6cc84d302d8eeb616bb12d7f439172107dbd2b"
ASF_KEY="0xf29e4e1cfc2867fcda12ac9b190bea017868a0d1f3f7d5cc59af6c7d3ce6c45c"

# Start node temporarily
cd ~/flarechain-node
./flarechain-node \
  --base-path ./data \
  --chain ./chainspec-mainnet-raw.json \
  --rpc-port 9944 \
  --rpc-cors all > /tmp/temp.log 2>&1 &

TEMP_PID=$!
sleep 10

# Insert keys
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_insertKey","params":["aura","'$AURA_KEY'","'$AURA_KEY'"],"id":1}' \
  http://localhost:9944

curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_insertKey","params":["gran","'$GRANDPA_KEY'","'$GRANDPA_KEY'"],"id":1}' \
  http://localhost:9944

curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_insertKey","params":["ppfa","'$ASF_KEY'","'$ASF_KEY'"],"id":1}' \
  http://localhost:9944

# Stop temp node
kill $TEMP_PID
```

**Repeat this for EACH validator with their specific keys from `ALL_21_VALIDATOR_KEYS.md`**

You need to do this for at minimum **15 validators** (2/3+1) for finality to work.

---

### 5. Start EojEdred as Bootstrap Node (VM1)

```bash
# On VM1: 20.12.114.226
cd ~/flarechain-node

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

# Watch for the peer ID
tail -f /tmp/eoj-validator.log | grep "Local node identity"

# You'll see something like:
# Local node identity is: 12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
```

**Save that peer ID!** You'll need it for other validators.

---

### 6. Start Other Validators (Connect to Bootstrap)

On each other VM, start the validator pointing to EojEdred's bootnode:

```bash
# Example for Validator 3 on VM2
cd ~/flarechain-node

./flarechain-node \
  --base-path ./data \
  --chain ./chainspec-mainnet-raw.json \
  --name "validator-3" \
  --validator \
  --rpc-port 9944 \
  --port 30333 \
  --bootnodes /ip4/20.12.114.226/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  > /tmp/validator.log 2>&1 &
```

**Replace `12D3KooW...` with the actual peer ID from step 5!**

---

### 7. Verify Finality is Working

Check if blocks are finalizing on any validator:

```bash
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
  http://localhost:9944 | jq
```

Watch the logs:
```bash
tail -f /tmp/validator.log | grep -i "finalized"

# Should see:
# 💤 Idle (15 peers), best: #42 (0x...), finalized #40 (0x...)
# Where finalized # keeps increasing!
```

---

## Success Criteria

✅ At least 15 validators running with correct GRANDPA keys inserted
✅ All validators connected to the network (check peer count)
✅ Blocks finalizing beyond #0
✅ Finalized block number increasing in logs

---

## Troubleshooting

**Finality not working?**
1. Check at least 15 validators are running: `pgrep -f flarechain-node` on each VM
2. Verify GRANDPA keys were inserted correctly on each
3. Check all validators connected: look for peer count in logs
4. Verify all using same chainspec (same genesis hash)
5. Check for GRANDPA errors in logs: `grep -i grandpa /tmp/validator.log`

**Can't connect to VMs?**
- Make sure Azure VMs are running
- Check security groups allow SSH (port 22)
- Verify IP addresses are correct

---

## Files Reference

- **Chainspec:** `/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json`
- **All Keys:** `/Users/macbook/Desktop/etrid/docs/mainnet/ALL_21_VALIDATOR_KEYS.md`
- **Full Procedure:** `/Users/macbook/Desktop/etrid/docs/mainnet/BOOTSTRAP_PROCEDURE.md`
- **Scripts:** `/Users/macbook/Desktop/etrid/scripts/`
