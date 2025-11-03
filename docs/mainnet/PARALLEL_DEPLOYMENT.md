# Parallel Deployment Guide - 21 Validators Across 4 Terminals

**Challenge:** Deploy chainspec to 21 VMs using 4 different terminal sessions

**Solution:** Simple SCP + parallel execution

---

## Quick Answer

**The chainspec is just a JSON file!** No rebuilding needed.

1. Copy it once from your local machine to each VM
2. Each VM uses the same `chainspec-mainnet-raw-FIXED.json` file
3. Process takes ~2-3 minutes total with parallel deployment

---

## Prerequisites

```bash
# On your local machine, verify you have the files:
ls -lh /Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json
ls -lh /Users/macbook/Desktop/etrid/target/release/flarechain-node
```

---

## Deployment Method: Parallel SCP

### Terminal 1: Oracle Cloud VMs (Example: 6 validators)

```bash
#!/bin/bash
# deploy-oracle.sh

CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"

# List of Oracle Cloud validator IPs
ORACLE_VMS=(
  "64.181.215.19"      # Gizzi
  "129.80.122.34"      # audit-dev01
  # Add 4 more Oracle VMs here
)

for VM in "${ORACLE_VMS[@]}"; do
  echo "Deploying to $VM..."

  # Create directory and copy files in parallel (background with &)
  (
    ssh ubuntu@$VM "sudo mkdir -p /var/lib/flarechain /usr/local/bin" &
    scp $CHAINSPEC ubuntu@$VM:/tmp/chainspec.json &
    scp $BINARY ubuntu@$VM:/tmp/flarechain-node &
    wait

    # Move files to final location
    ssh ubuntu@$VM "sudo mv /tmp/chainspec.json /var/lib/flarechain/chainspec-mainnet-raw.json"
    ssh ubuntu@$VM "sudo mv /tmp/flarechain-node /usr/local/bin/flarechain-node"
    ssh ubuntu@$VM "sudo chmod +x /usr/local/bin/flarechain-node"

    echo "✓ $VM deployment complete"
  ) &
done

wait
echo "✓ All Oracle Cloud VMs deployed!"
```

### Terminal 2: Azure VMs (Example: 6 validators)

```bash
#!/bin/bash
# deploy-azure.sh

CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"

AZURE_VMS=(
  "52.252.142.146"     # security-dev01
  # Add 5 more Azure VMs here
)

for VM in "${AZURE_VMS[@]}"; do
  echo "Deploying to $VM..."

  (
    ssh azureuser@$VM "sudo mkdir -p /var/lib/flarechain /usr/local/bin" &
    scp $CHAINSPEC azureuser@$VM:/tmp/chainspec.json &
    scp $BINARY azureuser@$VM:/tmp/flarechain-node &
    wait

    ssh azureuser@$VM "sudo mv /tmp/chainspec.json /var/lib/flarechain/chainspec-mainnet-raw.json"
    ssh azureuser@$VM "sudo mv /tmp/flarechain-node /usr/local/bin/flarechain-node"
    ssh azureuser@$VM "sudo chmod +x /usr/local/bin/flarechain-node"

    echo "✓ $VM deployment complete"
  ) &
done

wait
echo "✓ All Azure VMs deployed!"
```

### Terminal 3: AWS/Other VMs (Example: 5 validators)

```bash
#!/bin/bash
# deploy-aws.sh

CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"

AWS_VMS=(
  # Add AWS VM IPs here
)

for VM in "${AWS_VMS[@]}"; do
  echo "Deploying to $VM..."

  (
    ssh ec2-user@$VM "sudo mkdir -p /var/lib/flarechain /usr/local/bin" &
    scp $CHAINSPEC ec2-user@$VM:/tmp/chainspec.json &
    scp $BINARY ec2-user@$VM:/tmp/flarechain-node &
    wait

    ssh ec2-user@$VM "sudo mv /tmp/chainspec.json /var/lib/flarechain/chainspec-mainnet-raw.json"
    ssh ec2-user@$VM "sudo mv /tmp/flarechain-node /usr/local/bin/flarechain-node"
    ssh ec2-user@$VM "sudo chmod +x /usr/local/bin/flarechain-node"

    echo "✓ $VM deployment complete"
  ) &
done

wait
echo "✓ All AWS VMs deployed!"
```

### Terminal 4: Founder/Local Validators (Example: 4 validators)

```bash
#!/bin/bash
# deploy-local.sh

CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"

LOCAL_VMS=(
  "localhost"          # EojEdred (your machine)
  # governance-dev01 IP
  # Add other local/VPN VMs
)

for VM in "${LOCAL_VMS[@]}"; do
  if [ "$VM" == "localhost" ]; then
    echo "Deploying locally..."
    sudo mkdir -p /var/lib/flarechain /usr/local/bin
    sudo cp $CHAINSPEC /var/lib/flarechain/chainspec-mainnet-raw.json
    sudo cp $BINARY /usr/local/bin/flarechain-node
    sudo chmod +x /usr/local/bin/flarechain-node
    echo "✓ localhost deployment complete"
  else
    echo "Deploying to $VM..."

    (
      ssh ubuntu@$VM "sudo mkdir -p /var/lib/flarechain /usr/local/bin" &
      scp $CHAINSPEC ubuntu@$VM:/tmp/chainspec.json &
      scp $BINARY ubuntu@$VM:/tmp/flarechain-node &
      wait

      ssh ubuntu@$VM "sudo mv /tmp/chainspec.json /var/lib/flarechain/chainspec-mainnet-raw.json"
      ssh ubuntu@$VM "sudo mv /tmp/flarechain-node /usr/local/bin/flarechain-node"
      ssh ubuntu@$VM "sudo chmod +x /usr/local/bin/flarechain-node"

      echo "✓ $VM deployment complete"
    ) &
  fi
done

wait
echo "✓ All local VMs deployed!"
```

---

## Usage

### Step 1: Create the deployment scripts

```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet

# Make scripts executable
chmod +x deploy-oracle.sh deploy-azure.sh deploy-aws.sh deploy-local.sh
```

### Step 2: Open 4 terminals and run in parallel

**Terminal 1:**
```bash
./deploy-oracle.sh
```

**Terminal 2:**
```bash
./deploy-azure.sh
```

**Terminal 3:**
```bash
./deploy-aws.sh
```

**Terminal 4:**
```bash
./deploy-local.sh
```

### Step 3: Wait for completion

All 4 scripts will run in parallel. Total time: **~2-3 minutes** depending on network speed.

---

## Alternative: Single Terminal with tmux

If you prefer a single terminal:

```bash
# Install tmux if not already installed
# brew install tmux  (on macOS)

# Start tmux session
tmux new -s deploy

# Split into 4 panes
Ctrl+b "    # Split horizontally
Ctrl+b "    # Split again
Ctrl+b "    # Split again

# Navigate between panes: Ctrl+b + arrow keys

# Run each script in a different pane
# Pane 1: ./deploy-oracle.sh
# Pane 2: ./deploy-azure.sh
# Pane 3: ./deploy-aws.sh
# Pane 4: ./deploy-local.sh
```

---

## Can the Chainspec Be Rebuilt on VMs?

**Short Answer:** Yes, but not recommended.

**Rebuilding requires:**
1. Rust toolchain (~2GB download)
2. Repository clone (~500MB)
3. Cargo build (~10-20 minutes per VM)
4. Preset file must be identical

**Much simpler to just copy the 2MB JSON file!**

---

## Verification Script

After deployment, verify on each VM:

```bash
#!/bin/bash
# verify-deployment.sh

for VM in ${ALL_VMS[@]}; do
  echo "Checking $VM..."
  ssh user@$VM "
    if [ -f /var/lib/flarechain/chainspec-mainnet-raw.json ] && \
       [ -f /usr/local/bin/flarechain-node ]; then
      echo '✓ Files present'

      # Verify chainspec hash (should match for all VMs)
      HASH=\$(sha256sum /var/lib/flarechain/chainspec-mainnet-raw.json | awk '{print \$1}')
      echo \"  Chainspec hash: \$HASH\"

      # Verify binary is executable
      /usr/local/bin/flarechain-node --version && echo '✓ Binary executable' || echo '✗ Binary not executable'
    else
      echo '✗ Files missing!'
    fi
  "
done
```

---

## What Each VM Needs

**Files:**
- `/usr/local/bin/flarechain-node` (58MB binary)
- `/var/lib/flarechain/chainspec-mainnet-raw.json` (2MB chainspec)

**Directories:**
- `/var/lib/flarechain/` (node data directory)

**Ports Open:**
- 30333 (Substrate P2P)
- 30334 (DETR P2P - ASF finality)
- 9933/9944 (RPC - only for insertion of session keys)

---

## Summary

✅ **Chainspec deployment is SIMPLE:**
- Just copy 2 files (binary + JSON chainspec)
- No rebuilding needed
- Parallel deployment across 4 terminals: ~2-3 minutes

✅ **All VMs use the SAME chainspec:**
- Everyone gets `chainspec-mainnet-raw-FIXED.json`
- No customization per validator
- Validator identity comes from session keys (inserted after node starts)

✅ **Process:**
1. Copy files to VMs (parallel)
2. Start nodes (all use same chainspec)
3. Insert session keys (unique per validator)
4. Network forms automatically

---

**Next Steps:** See `QUICK_START.md` for node startup commands and session key insertion
