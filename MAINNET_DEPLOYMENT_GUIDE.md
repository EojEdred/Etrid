# Ëtrid FlareChain Mainnet Deployment Guide

**Purpose:** Coordinate deployment of mainnet across all 21 validator VMs

---

## 🎯 Overview

Mainnet requires **at least 15 out of 21 validators** online to achieve consensus (>2/3 GRANDPA threshold). This guide covers synchronized deployment to all validator VMs.

---

## 📋 Prerequisites

- ✅ Genesis accounts generated (`genesis-accounts-20251030-152748/`)
- ✅ Genesis config created (`flarechain_mainnet_genesis.json`)
- ✅ Validator session keys exist (`validator-keys-complete.json`)
- ✅ All 21 VMs provisioned with SSH access
- ⚠️ Foundation multisig created (replace sudo key in genesis config)

---

## 🔧 Phase 1: Prepare Mainnet Build

### Step 1.1: Copy Genesis Config to Runtime

```bash
# Copy genesis config to runtime presets
cp flarechain_mainnet_genesis.json 05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json
```

### Step 1.2: Build Mainnet Binary

```bash
# Build release binary (takes 15-30 minutes)
cd 05-multichain/flare-chain/node
cargo build --release --locked

# Verify binary
ls -lh ../../../target/release/flarechain-node
```

### Step 1.3: Generate Raw Chain Spec

```bash
# Generate raw chain specification
cd ../../..
./target/release/flarechain-node build-spec --chain mainnet --raw > flarechain-mainnet-raw.json

# Verify chain spec
head -20 flarechain-mainnet-raw.json

# Calculate genesis hash (for verification)
cat flarechain-mainnet-raw.json | grep -o '"genesis":{"raw":{"top":{[^}]*}}}' | shasum -a 256
```

---

## 📦 Phase 2: Package Deployment Files

### Step 2.1: Create Deployment Package

```bash
# Create deployment directory
mkdir -p mainnet-deployment-package

# Copy binary
cp target/release/flarechain-node mainnet-deployment-package/

# Copy chain spec
cp flarechain-mainnet-raw.json mainnet-deployment-package/

# Copy validator keys
cp validator-keys-setup/generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json mainnet-deployment-package/

# Create tarball
tar -czf mainnet-deployment.tar.gz mainnet-deployment-package/

# Verify package size
ls -lh mainnet-deployment.tar.gz
```

---

## 🚀 Phase 3: Deploy to All Validators

### Step 3.1: Upload to All VMs

You have two approaches:

#### Option A: Sequential Upload (slower but safer)
```bash
# Create VM list
cat > validator-vms.txt << 'EOF'
ubuntu@64.181.215.19  # Validator 1 (Gizzi)
ubuntu@<validator-2-ip>
ubuntu@<validator-3-ip>
# ... (all 21 validators)
EOF

# Upload to each VM
while read vm; do
  echo "Uploading to $vm..."
  scp -i ~/.ssh/gizzi-validator mainnet-deployment.tar.gz "$vm:/home/ubuntu/"
done < validator-vms.txt
```

#### Option B: Parallel Upload (faster)
```bash
# Upload to all VMs in parallel
cat validator-vms.txt | xargs -P 21 -I {} sh -c 'echo "Uploading to {}..." && scp -i ~/.ssh/gizzi-validator mainnet-deployment.tar.gz {}:/home/ubuntu/'
```

### Step 3.2: Extract on All VMs

```bash
# Extract deployment package on all VMs
while read vm; do
  echo "Extracting on $vm..."
  ssh -i ~/.ssh/gizzi-validator "$vm" "cd /home/ubuntu && tar -xzf mainnet-deployment.tar.gz"
done < validator-vms.txt
```

### Step 3.3: Install Binary on All VMs

```bash
# Install binary to /usr/local/bin on all VMs
while read vm; do
  echo "Installing binary on $vm..."
  ssh -i ~/.ssh/gizzi-validator "$vm" "sudo cp /home/ubuntu/mainnet-deployment-package/flarechain-node /usr/local/bin/ && sudo chmod +x /usr/local/bin/flarechain-node"
done < validator-vms.txt
```

---

## 🔑 Phase 4: Insert Session Keys

### Step 4.1: Extract Session Keys for Each Validator

```bash
# Create script to insert keys for a specific validator
cat > insert-validator-keys.sh << 'EOF'
#!/bin/bash
# Usage: ./insert-validator-keys.sh <validator-number> <vm-address>

VALIDATOR_NUM=$1
VM_ADDRESS=$2
KEYS_FILE="mainnet-deployment-package/validator-keys-complete.json"

echo "Inserting session keys for Validator $VALIDATOR_NUM on $VM_ADDRESS"

# Extract keys from JSON (using jq on local machine)
AURA_SEED=$(jq -r ".validators[$((VALIDATOR_NUM-1))].sessionKeys.auraSeed" "$KEYS_FILE")
GRANDPA_SEED=$(jq -r ".validators[$((VALIDATOR_NUM-1))].sessionKeys.grandpaSeed" "$KEYS_FILE")
ASF_SEED=$(jq -r ".validators[$((VALIDATOR_NUM-1))].asfKeys.secretSeed" "$KEYS_FILE")

echo "Extracted keys for Validator $VALIDATOR_NUM"

# SSH to VM and insert keys
ssh -i ~/.ssh/gizzi-validator "$VM_ADDRESS" bash << EOSSH
  echo "Inserting AURA key..."
  /usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
    --scheme Sr25519 \
    --suri "$AURA_SEED" \
    --key-type aura

  echo "Inserting GRANDPA key..."
  /usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
    --scheme Ed25519 \
    --suri "$GRANDPA_SEED" \
    --key-type gran

  echo "Inserting ASF key..."
  /usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
    --scheme Sr25519 \
    --suri "$ASF_SEED" \
    --key-type afsf

  echo "✅ Keys inserted successfully"
EOSSH

echo "✅ Validator $VALIDATOR_NUM configured"
EOF

chmod +x insert-validator-keys.sh
```

### Step 4.2: Insert Keys on All Validators

```bash
# Insert keys for all 21 validators
./insert-validator-keys.sh 1 ubuntu@64.181.215.19
./insert-validator-keys.sh 2 ubuntu@<validator-2-ip>
./insert-validator-keys.sh 3 ubuntu@<validator-3-ip>
# ... (repeat for all 21)
```

Or automate with your VM list:

```bash
# Create VM list with validator numbers
cat > validator-vms-numbered.txt << 'EOF'
1 ubuntu@64.181.215.19
2 ubuntu@<validator-2-ip>
3 ubuntu@<validator-3-ip>
# ... (all 21)
EOF

# Insert keys for all validators in parallel
cat validator-vms-numbered.txt | xargs -P 21 -l bash -c './insert-validator-keys.sh $0 $1'
```

---

## 🎬 Phase 5: Coordinated Startup

### Step 5.1: Create Systemd Service on All VMs

```bash
# Create systemd service file
cat > create-validator-service.sh << 'EOF'
#!/bin/bash
# Run on each validator VM

sudo tee /etc/systemd/system/flarechain-validator.service > /dev/null << 'EOSVC'
[Unit]
Description=Ëtrid FlareChain Mainnet Validator
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu
ExecStart=/usr/local/bin/flarechain-node \
  --base-path /var/lib/etrid \
  --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
  --name VALIDATOR_NAME_PLACEHOLDER \
  --validator \
  --port 30333 \
  --rpc-port 9944 \
  --prometheus-port 9615 \
  --rpc-methods Safe \
  --rpc-cors all \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'

Restart=always
RestartSec=10
LimitNOFILE=10000

[Install]
WantedBy=multi-user.target
EOSVC

sudo systemctl daemon-reload
echo "✅ Systemd service created"
EOF

chmod +x create-validator-service.sh
```

### Step 5.2: Deploy Service to All VMs

```bash
# Upload service creation script
while read num vm; do
  VALIDATOR_NAME=$(jq -r ".validators[$((num-1))].name" mainnet-deployment-package/validator-keys-complete.json)
  echo "Creating service on $vm ($VALIDATOR_NAME)..."

  scp -i ~/.ssh/gizzi-validator create-validator-service.sh "$vm:/home/ubuntu/"
  ssh -i ~/.ssh/gizzi-validator "$vm" "bash /home/ubuntu/create-validator-service.sh && sudo sed -i 's/VALIDATOR_NAME_PLACEHOLDER/$VALIDATOR_NAME/' /etc/systemd/system/flarechain-validator.service"
done < validator-vms-numbered.txt
```

### Step 5.3: Enable Services (Don't Start Yet!)

```bash
# Enable services on all VMs (but don't start)
while read num vm; do
  echo "Enabling service on $vm..."
  ssh -i ~/.ssh/gizzi-validator "$vm" "sudo systemctl enable flarechain-validator"
done < validator-vms-numbered.txt
```

### Step 5.4: Synchronized Startup

**CRITICAL:** All validators should start within a few minutes of each other.

#### Option A: Manual Coordinated Start
```bash
# Send start command to all VMs simultaneously
echo "Starting all validators in 3 seconds..."
sleep 3

cat validator-vms-numbered.txt | xargs -P 21 -l bash -c 'ssh -i ~/.ssh/gizzi-validator $1 "sudo systemctl start flarechain-validator"'

echo "✅ All validators started!"
```

#### Option B: Scheduled Start (Recommended)
```bash
# Schedule all validators to start at specific time (e.g., 2 minutes from now)
START_TIME=$(date -u -d "+2 minutes" "+%Y-%m-%d %H:%M:%S")

while read num vm; do
  echo "Scheduling start on $vm at $START_TIME UTC..."
  ssh -i ~/.ssh/gizzi-validator "$vm" "echo 'sudo systemctl start flarechain-validator' | at $START_TIME"
done < validator-vms-numbered.txt

echo "✅ All validators scheduled to start at $START_TIME UTC"
echo "⏰ Countdown to mainnet launch..."
```

---

## 📊 Phase 6: Monitor Launch

### Step 6.1: Check Validator Status

```bash
# Check status on all validators
while read num vm; do
  echo "=== Validator $num ($vm) ==="
  ssh -i ~/.ssh/gizzi-validator "$vm" "sudo systemctl status flarechain-validator --no-pager | head -15"
  echo ""
done < validator-vms-numbered.txt
```

### Step 6.2: Monitor Block Production

```bash
# Check current block on each validator
while read num vm; do
  BLOCK=$(ssh -i ~/.ssh/gizzi-validator "$vm" "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}' http://localhost:9944 | grep -o '\"number\":\"0x[^\"]*' | cut -d'\"' -f4")
  BLOCK_DEC=$((16#${BLOCK#0x}))
  echo "Validator $num: Block #$BLOCK_DEC"
done < validator-vms-numbered.txt
```

### Step 6.3: Check Peer Connections

```bash
# Check peer count on each validator
while read num vm; do
  PEERS=$(ssh -i ~/.ssh/gizzi-validator "$vm" "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:9944 | grep -o '\"peers\":[0-9]*' | cut -d':' -f2")
  echo "Validator $num: $PEERS peers"
done < validator-vms-numbered.txt
```

### Step 6.4: Verify Finalization

```bash
# Check if blocks are being finalized
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getFinalizedHead\"}' http://localhost:9944"
```

---

## ⚠️ Troubleshooting

### Issue: Chain Not Starting

**Cause:** Insufficient validators online (need 15 out of 21)

**Solution:**
```bash
# Check how many validators are online
cat validator-vms-numbered.txt | wc -l  # Should be 21

# Start remaining validators
./start-all-validators.sh
```

### Issue: No Peer Connections

**Cause:** Bootnodes not configured or firewall blocking port 30333

**Solution:**
```bash
# Add bootnode addresses to all validators
# Extract first validator's peer ID
BOOTNODE_PEER_ID=$(ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 "/usr/local/bin/flarechain-node key inspect-node-key --file /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519")

# Add to other validators with --bootnodes flag
echo "Bootnode: /ip4/64.181.215.19/tcp/30333/p2p/$BOOTNODE_PEER_ID"
```

### Issue: Keys Not Inserted

**Solution:**
```bash
# Verify keys on a validator
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 "ls -la /var/lib/etrid/chains/flarechain_mainnet/keystore/"

# Should show 3 key files (aura, grandpa, asf)
```

---

## ✅ Success Criteria

Your mainnet is successfully launched when:

1. ✅ All 21 validators are running
2. ✅ Peer count >10 on each validator
3. ✅ Blocks are being produced (check block number increasing)
4. ✅ Blocks are being finalized (check finalized head)
5. ✅ No errors in validator logs
6. ✅ Telemetry shows all validators online

---

## 🎉 Post-Launch Tasks

1. **Monitor for 24 hours** - Watch for any issues
2. **Create governance proposals** - Test on-chain governance
3. **Test EDSC functionality** - Verify stablecoin operations
4. **Announce mainnet launch** - Notify community
5. **Update documentation** - Record mainnet addresses and endpoints

---

**Next Steps:**
1. Review this guide
2. Test deployment on 2-3 validators first (devnet)
3. Prepare all 21 VMs
4. Schedule coordinated mainnet launch

---

**Status:** 📝 Deployment guide ready - Review before executing
