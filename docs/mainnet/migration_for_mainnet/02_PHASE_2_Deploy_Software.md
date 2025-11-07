# Phase 2: Deploy Software to All VMs

**Duration:** 1-2 hours
**Goal:** Install node binary, chainspec, and configure services

---

## Overview

In this phase, you'll copy the necessary files to each VM and configure them to run as validators.

**What we're deploying:**
1. FlareChain node binary
2. Mainnet chainspec file
3. Validator session keys
4. Systemd service configuration

---

## Step 1: Prepare Files on Local Machine (5 minutes)

**Verify you have these files:**

```bash
# Node binary
ls -lh /Users/macbook/Desktop/etrid/target/release/flarechain-node

# Should show ~58 MB file

# Chainspec
ls -lh /Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json

# Should show ~2 MB file

# Session keys
ls /Users/macbook/Desktop/etrid/secrets/validator-keys/

# Should show folders for each validator (validator-6 through validator-21)
```

✅ **Checkpoint:** All files present and accessible

---

## Step 2: Copy Node Binary to All VMs (20 minutes)

### Option A: Copy to Each VM Individually

```bash
# For each VM
scp -i ~/.ssh/contabo-validators \
    /Users/macbook/Desktop/etrid/target/release/flarechain-node \
    root@VM_IP:/usr/local/bin/

# Make executable
ssh -i ~/.ssh/contabo-validators root@VM_IP "chmod +x /usr/local/bin/flarechain-node"

# Verify
ssh -i ~/.ssh/contabo-validators root@VM_IP "/usr/local/bin/flarechain-node --version"
```

**Repeat for all 16 VMs.**

### Option B: Use Parallel Script (Faster)

Create a script to deploy to all VMs:

```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet/migration_for_mainnet

# Create deployment script
cat > deploy-binary-all.sh <<'EOF'
#!/bin/bash
# Deploy binary to all Contabo VMs

# Read VM IPs from inventory
VMS=(
    "IP1"
    "IP2"
    "IP3"
    # ... add all 16 IPs
)

for ip in "${VMS[@]}"; do
    echo "Deploying to $ip..."
    scp -i ~/.ssh/contabo-validators \
        /Users/macbook/Desktop/etrid/target/release/flarechain-node \
        root@$ip:/usr/local/bin/ &
done

wait
echo "All deployments complete!"

# Make executable on all VMs
for ip in "${VMS[@]}"; do
    ssh -i ~/.ssh/contabo-validators root@$ip \
        "chmod +x /usr/local/bin/flarechain-node" &
done

wait
echo "All binaries are now executable!"
EOF

chmod +x deploy-binary-all.sh

# Edit the script to add your IPs
nano deploy-binary-all.sh

# Run it
./deploy-binary-all.sh
```

✅ **Checkpoint:** Binary deployed to all 16 VMs

---

## Step 3: Copy Chainspec to All VMs (10 minutes)

```bash
# For each VM
scp -i ~/.ssh/contabo-validators \
    /Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json \
    root@VM_IP:/root/chainspec.json

# Verify
ssh -i ~/.ssh/contabo-validators root@VM_IP "ls -lh /root/chainspec.json"
```

**Or use batch script:**

```bash
cat > deploy-chainspec-all.sh <<'EOF'
#!/bin/bash
VMS=(
    # Add your IPs here
)

for ip in "${VMS[@]}"; do
    echo "Deploying chainspec to $ip..."
    scp -i ~/.ssh/contabo-validators \
        /Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json \
        root@$ip:/root/chainspec.json &
done

wait
echo "Chainspec deployed to all VMs!"
EOF

chmod +x deploy-chainspec-all.sh
nano deploy-chainspec-all.sh  # Add IPs
./deploy-chainspec-all.sh
```

✅ **Checkpoint:** Chainspec on all VMs

---

## Step 4: Copy Session Keys (15 minutes)

**Important:** Each VM gets different session keys based on which validator it represents.

**Mapping:**
- VM01 (first Contabo VM) → Validator 6 keys
- VM02 → Validator 7 keys
- VM03 → Validator 8 keys
- ... and so on
- VM16 → Validator 21 keys

```bash
# For VM01 (becomes Validator 6)
scp -r -i ~/.ssh/contabo-validators \
    /Users/macbook/Desktop/etrid/secrets/validator-keys/validator-6/* \
    root@VM01_IP:/root/.etrid/keys/

# For VM02 (becomes Validator 7)
scp -r -i ~/.ssh/contabo-validators \
    /Users/macbook/Desktop/etrid/secrets/validator-keys/validator-7/* \
    root@VM02_IP:/root/.etrid/keys/

# ... repeat for all 16 VMs
```

**Or create mapping script:**

```bash
cat > deploy-keys-all.sh <<'EOF'
#!/bin/bash

# VM IP to Validator Number mapping
declare -A VM_MAP=(
    ["VM01_IP"]="6"
    ["VM02_IP"]="7"
    ["VM03_IP"]="8"
    ["VM04_IP"]="9"
    ["VM05_IP"]="10"
    ["VM06_IP"]="11"
    ["VM07_IP"]="12"
    ["VM08_IP"]="13"
    ["VM09_IP"]="14"
    ["VM10_IP"]="15"
    ["VM11_IP"]="16"
    ["VM12_IP"]="17"
    ["VM13_IP"]="18"
    ["VM14_IP"]="19"
    ["VM15_IP"]="20"
    ["VM16_IP"]="21"
)

for vm_ip in "${!VM_MAP[@]}"; do
    validator_num="${VM_MAP[$vm_ip]}"
    echo "Copying validator-${validator_num} keys to $vm_ip..."

    ssh -i ~/.ssh/contabo-validators root@$vm_ip "mkdir -p /root/.etrid/keys"

    scp -r -i ~/.ssh/contabo-validators \
        /Users/macbook/Desktop/etrid/secrets/validator-keys/validator-${validator_num}/* \
        root@$vm_ip:/root/.etrid/keys/
done

echo "All keys deployed!"
EOF

chmod +x deploy-keys-all.sh
nano deploy-keys-all.sh  # Replace VM01_IP etc with actual IPs
./deploy-keys-all.sh
```

✅ **Checkpoint:** Session keys deployed to all VMs

---

## Step 5: Create Systemd Service Files (20 minutes)

For each VM, create a systemd service that will auto-start the validator.

**Service template:**

```bash
# On each VM, create service file
ssh -i ~/.ssh/contabo-validators root@VM_IP

cat > /etc/systemd/system/flarechain-validator.service <<EOF
[Unit]
Description=Ëtrid FlareChain Validator - Validator NUMBER
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=/usr/local/bin/flarechain-node \\
  --chain /root/chainspec.json \\
  --base-path /root/.etrid/validator \\
  --validator \\
  --name "Validator-NUMBER-Contabo" \\
  --public-addr /ip4/THIS_VM_PUBLIC_IP/tcp/30333 \\
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/BOOTNODE_PEER_ID \\
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/BOOTNODE_PEER_ID \\
  --rpc-cors all \\
  --rpc-external \\
  --port 30333 \\
  --rpc-port 9944 \\
  --prometheus-external \\
  --prometheus-port 9615

Restart=always
RestartSec=10s
KillSignal=SIGTERM
TimeoutStopSec=60s

LimitNOFILE=65536
LimitNPROC=4096

StandardOutput=journal
StandardError=journal
SyslogIdentifier=flarechain-validator

[Install]
WantedBy=multi-user.target
EOF

# Enable service (but don't start yet)
systemctl daemon-reload
systemctl enable flarechain-validator

exit
```

**Important:** Replace these values:
- `NUMBER` → Validator number (6-21)
- `THIS_VM_PUBLIC_IP` → The VM's actual public IP
- `BOOTNODE_PEER_ID` → Get from existing bootnodes (we'll handle this in Phase 3)

**Batch script for service deployment:**

```bash
cat > deploy-services-all.sh <<'EOF'
#!/bin/bash

# VM IP to Validator Number mapping
declare -A VM_MAP=(
    ["VM01_IP"]="6"
    ["VM02_IP"]="7"
    # ... add all mappings
)

for vm_ip in "${!VM_MAP[@]}"; do
    validator_num="${VM_MAP[$vm_ip]}"
    echo "Creating service on $vm_ip (Validator $validator_num)..."

    ssh -i ~/.ssh/contabo-validators root@$vm_ip <<REMOTE
cat > /etc/systemd/system/flarechain-validator.service <<'SERVICE'
[Unit]
Description=Ëtrid FlareChain Validator - Validator $validator_num
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/flarechain-node \\
  --chain /root/chainspec.json \\
  --base-path /root/.etrid/validator \\
  --validator \\
  --name "Validator-${validator_num}-Contabo" \\
  --public-addr /ip4/${vm_ip}/tcp/30333 \\
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/PEER_ID_HERE \\
  --rpc-cors all \\
  --port 30333 \\
  --rpc-port 9944

Restart=always
RestartSec=10s

[Install]
WantedBy=multi-user.target
SERVICE

systemctl daemon-reload
systemctl enable flarechain-validator
echo "Service created on $vm_ip"
REMOTE
done

echo "All services deployed!"
EOF

chmod +x deploy-services-all.sh
nano deploy-services-all.sh  # Add your IPs and mappings
./deploy-services-all.sh
```

✅ **Checkpoint:** Systemd services created on all VMs

---

## Step 6: Verify Deployment (10 minutes)

Check that everything is in place on all VMs:

```bash
# For each VM
ssh -i ~/.ssh/contabo-validators root@VM_IP

# Check binary
/usr/local/bin/flarechain-node --version

# Check chainspec
ls -lh /root/chainspec.json

# Check keys
ls -la /root/.etrid/keys/

# Check service
systemctl status flarechain-validator

# Should show "disabled" or "enabled" but not "running" yet

exit
```

✅ **Checkpoint:** All files deployed and verified

---

## Phase 2 Complete! ✅

You should now have on all 16 VMs:

- [x] FlareChain node binary installed
- [x] Mainnet chainspec deployed
- [x] Session keys installed
- [x] Systemd service configured (but not started)
- [x] Firewall rules allowing required ports

---

## Next Step

**Open:** `03_PHASE_3_Start_Validators.md`

That guide will walk you through:
- Getting bootnode peer IDs
- Starting validators in batches
- Monitoring sync progress
- Waiting for consensus

---

## Troubleshooting

### "scp: Permission denied"
- Check SSH key path is correct
- Verify you have root access
- Try with password: `scp /path/to/file root@IP:/destination`

### "Binary won't execute"
- Make sure it's executable: `chmod +x /usr/local/bin/flarechain-node`
- Check architecture: `uname -m` should show x86_64
- Verify binary is complete: `ls -lh /usr/local/bin/flarechain-node`

### "Session keys missing"
- Check local path: `/Users/macbook/Desktop/etrid/secrets/validator-keys/`
- Verify folder names match validator numbers
- Re-copy specific keys if needed

---

**Phase 2 Duration:** ~1-2 hours
**Status:** Ready to proceed to Phase 3

