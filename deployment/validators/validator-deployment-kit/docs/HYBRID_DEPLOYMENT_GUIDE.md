# Hybrid Multi-Provider Deployment Guide

**Date:** October 29, 2025
**Purpose:** Complete guide for deploying 21 validators across multiple providers

---

## Overview

This guide covers TWO approaches:

**Option A: Automated (Recommended)** - Use scripts, deploy in 1 hour
**Option B: Manual (Beginner-Friendly)** - Click through UIs, deploy in 4-6 hours

---

## Exact Specifications Summary

### What You Need for Each Validator

**Standard Validator (18 nodes):**
- **CPU:** 4 cores @ 2.5+ GHz
- **RAM:** 16 GB
- **Storage:** 500 GB NVMe SSD (or 360 GB minimum)
- **Network:** 100 Mbps, 5 TB/month traffic
- **OS:** Ubuntu 22.04 LTS

**Critical Validator (3 nodes - Gizzi, Eoj, governance):**
- **CPU:** 6 cores @ 3.0+ GHz
- **RAM:** 32-64 GB
- **Storage:** 1 TB NVMe SSD
- **Network:** 1 Gbps, unlimited traffic
- **OS:** Ubuntu 22.04 LTS
- **Type:** Bare metal preferred

---

## Option A: Automated Deployment (FAST)

### Prerequisites

**Install provider CLIs:**

```bash
# Hetzner CLI
brew install hcloud

# DigitalOcean CLI
brew install doctl

# Vultr CLI
brew install vultr/vultr-cli/vultr-cli

# (Optional) Akash CLI
brew install akash
```

### Setup API Authentication

**Hetzner:**
```bash
# Get API token from: https://console.hetzner.cloud/
hcloud context create etrid-project
# Paste your API token when prompted
```

**DigitalOcean:**
```bash
# Get API token from: https://cloud.digitalocean.com/account/api/tokens
doctl auth init
# Paste your API token
```

**Vultr:**
```bash
# Get API key from: https://my.vultr.com/settings/#settingsapi
export VULTR_API_KEY=your_api_key_here
```

### Run Deployment Script

```bash
cd /Users/macbook/Desktop/etrid/scripts

# Make executable
chmod +x deploy-hybrid-multi-provider.sh

# Run deployment
./deploy-hybrid-multi-provider.sh
```

**What it does:**
1. Checks prerequisites (binary, keys, SSH)
2. Deploys 10 VPS to Hetzner (CPX31)
3. Deploys 4 VPS to Vultr (High Frequency)
4. Deploys 3 VPS to DigitalOcean
5. Creates Akash deployment manifest
6. Generates inventory file with all IPs

**Time:** 10-15 minutes

**Cost:** ~$550/month for VPS (+ $150 for bare metal ordered manually)

### Manual Steps After Script

**1. Order Hetzner Bare Metal (Manual):**

The script can't auto-order bare metal. You must:

1. Go to: https://robot.hetzner.com/order/index
2. Click "Order" → "Dedicated Root Server"
3. Select **AX41-NVMe** (3 servers)
4. Choose location: Falkenstein or Helsinki
5. Select "Ubuntu 22.04" as OS
6. Add your SSH key
7. Complete order

**Delivery:** 2-24 hours

**Cost:** €46.41/month each × 3 = €139.23/month (~$150)

**2. Deploy Validator Software:**

After all VMs are ready:

```bash
cd /Users/macbook/Desktop/etrid/scripts
./deploy-validator-software.sh
```

(I'll create this script next)

---

## Option B: Manual Deployment (UI-Based)

### Deployment Plan

**You'll manually create 21 VMs across providers:**

1. **Hetzner:** 3 bare metal + 10 VPS = 13 total
2. **Vultr:** 4 VPS
3. **DigitalOcean:** 3 VPS
4. **Akash:** 1 deployment

### Part 1: Hetzner Deployment

#### **1A. Order 3 Bare Metal Servers**

**For:** Gizzi, EojEdred, governance-dev01

**Steps:**

1. Go to: https://robot.hetzner.com/order/index
2. Click **"Order Server"**
3. Filter by:
   - **Server Type:** Dedicated Root Server
   - **CPU:** AMD Ryzen
   - **Location:** Germany or Finland
4. Select **AX41-NVMe:**
   - CPU: AMD Ryzen 5 3600 (6 cores)
   - RAM: 64 GB DDR4
   - Storage: 2×512 GB NVMe RAID
   - Price: €46.41/month
5. Quantity: **3 servers**
6. Operating System: **Ubuntu 22.04 LTS**
7. Add SSH key:
   - Copy: `cat ~/.ssh/id_rsa.pub`
   - Paste in "SSH Keys" section
8. Server names:
   - `gizzi-bootstrap-1`
   - `eojedred-bootstrap-2`
   - `governance-validator-03`
9. Complete order

**Wait:** 2-24 hours for activation
**Cost:** €139.23/month ($150)

---

#### **1B. Create 10 Cloud VPS (CPX31)**

**For:** Validators 04-13

**Steps:**

1. Go to: https://console.hetzner.cloud/
2. Create project: "Ëtrid Validators"
3. Click **"Add Server"**
4. **Location:** Falkenstein (Germany) - repeat later for Nuremberg, Helsinki
5. **Image:** Ubuntu 22.04
6. **Type:** CPX → **CPX31**
   - 4 dedicated vCPU AMD
   - 16 GB RAM
   - 360 GB NVMe SSD
   - Price: €23.79/month
7. **Networking:**
   - IPv4 (automatic)
   - IPv6 (optional)
8. **SSH Key:**
   - Click "Add SSH Key"
   - Paste: `cat ~/.ssh/id_rsa.pub`
   - Name: "etrid-validators"
9. **Server name:** `validator-04`
10. Click **"Create & Buy now"**

**Repeat 9 more times** for validators 05-13:
- Mix locations: 4 in Falkenstein, 3 in Nuremberg, 3 in Helsinki
- Names: validator-05, validator-06, ... validator-13

**Time:** 30 minutes (10 servers × 3 min each)
**Cost:** €237.90/month ($255)

---

### Part 2: Vultr Deployment

#### **Create 4 High Frequency VPS**

**For:** Validators 14-17

**Steps:**

1. Go to: https://my.vultr.com/
2. Click **"Deploy +"** → **"Deploy New Server"**
3. **Choose Server:**
   - Type: **Cloud Compute - High Frequency**
4. **Server Location:**
   - First: **New Jersey (EWR)**
   - Repeat with: Los Angeles, Singapore, Tokyo
5. **Server Type:** Ubuntu 22.04 LTS
6. **Server Size:** 180 GB NVMe
   - 4 CPU
   - 16 GB Memory
   - 180 GB NVMe SSD
   - 5 TB Bandwidth
   - **Price:** $48/month
7. **Additional Features:**
   - ✅ Enable IPv6
   - ✅ Enable Auto Backups (optional - $4.80/mo)
   - ✅ Enable DDoS Protection (included)
8. **SSH Keys:**
   - Click "Add New"
   - Paste: `cat ~/.ssh/id_rsa.pub`
   - Name: "etrid-validators"
9. **Server Hostname:** `validator-14`
10. **Server Label:** `validator-14-ewr`
11. Click **"Deploy Now"**

**Repeat 3 more times** for validators 15-17:
- validator-15: Los Angeles (LAX)
- validator-16: Singapore (SGP)
- validator-17: Tokyo (NRT)

**Time:** 20 minutes (4 servers × 5 min each)
**Cost:** $192/month

---

### Part 3: DigitalOcean Deployment

#### **Create 3 Droplets**

**For:** Validators 18-20

**Steps:**

1. Go to: https://cloud.digitalocean.com/
2. Click **"Create"** → **"Droplets"**
3. **Choose Region:**
   - First: **New York (NYC3)**
   - Repeat with: San Francisco, London
4. **Choose Image:** Ubuntu 22.04 (LTS) x64
5. **Choose Size:**
   - **Droplet Type:** Regular
   - **CPU options:** Regular
   - Select: **4 GB / 2 CPUs - $48/mo**

   **WAIT - That's only 4GB RAM!**

   We need 16 GB. Better option:
   - Click **"Premium AMD"** tab
   - Select: **16 GB / 4 AMD CPUs**
   - 100 GB SSD
   - 5 TB transfer
   - **Price:** $84/month

6. **Additional storage:**
   - We need more than 100 GB
   - Click "Add Volume"
   - Size: 400 GB
   - Name: validator-18-data
   - **Cost:** +$40/month
   - **Total:** $124/month per droplet

**Alternative (cheaper):**
Use Basic droplet without volume, monitor storage:
- **16 GB / 4 CPUs - $84/month**
- 100 GB SSD (tight but manageable initially)
- Upgrade storage later if needed

7. **Choose Authentication:**
   - Select: **SSH keys**
   - Click "New SSH Key"
   - Paste: `cat ~/.ssh/id_rsa.pub`
   - Name: "etrid-validators"

8. **Finalize Details:**
   - Quantity: 1
   - Hostname: `validator-18`
   - Tags: `etrid`, `validator`

9. Click **"Create Droplet"**

**Repeat 2 more times** for validators 19-20:
- validator-19: San Francisco (SFO3)
- validator-20: London (LON1)

**Time:** 15 minutes (3 droplets × 5 min each)
**Cost:** $252/month (with storage) OR $252/month (without)

---

### Part 4: Akash Deployment (Advanced)

**For:** Validator 21 (decentralized)

**Prerequisites:**
1. Install Akash CLI: `brew install akash`
2. Create Akash wallet
3. Fund wallet with AKT tokens (~50 AKT for deployment + $20/month)

**Steps:**

1. Create deployment manifest (`akash-validator.yml`):

```yaml
---
version: "2.0"

services:
  etrid-validator:
    image: ubuntu:22.04
    expose:
      - port: 30333
        as: 30333
        to:
          - global: true
    env:
      - "VALIDATOR_NAME=validator-21"

profiles:
  compute:
    etrid-validator:
      resources:
        cpu:
          units: 4.0
        memory:
          size: 16Gi
        storage:
          size: 500Gi

  placement:
    akash:
      pricing:
        etrid-validator:
          denom: uakt
          amount: 1000  # Max 1000 uakt per block

deployment:
  etrid-validator:
    akash:
      profile: etrid-validator
      count: 1
```

2. Deploy:

```bash
akash tx deployment create akash-validator.yml \
  --from wallet \
  --node https://rpc.akash.network:443 \
  --chain-id akashnet-2
```

3. View bids:

```bash
akash query market bid list --owner your_address
```

4. Accept lowest bid from reputable provider

**Time:** 30 minutes (if familiar with Akash)
**Cost:** ~$20/month in AKT

---

## Total Cost Comparison

### Manual Deployment Costs

| Provider | Servers | Specs | Monthly | Annual |
|----------|---------|-------|---------|--------|
| **Hetzner Bare** | 3 | AX41-NVMe | $150 | $1,800 |
| **Hetzner VPS** | 10 | CPX31 | $255 | $3,060 |
| **Vultr** | 4 | High Freq | $192 | $2,304 |
| **DigitalOcean** | 3 | 16GB AMD | $252 | $3,024 |
| **Akash** | 1 | 4CPU/16GB | $20 | $240 |
| **TOTAL** | **21** | | **$869/mo** | **$10,428/yr** |

**vs Azure:** Save $1,231/month ($14,772/year)

---

## After Creating VMs - Software Deployment

### Step 1: Collect All VM IPs

Create a file `validator-ips.txt`:

```
# Hetzner Bare Metal
gizzi-bootstrap-1       65.108.x.x
eojedred-bootstrap-2    65.108.x.x
governance-validator-03 65.108.x.x

# Hetzner VPS
validator-04    157.90.x.x
validator-05    157.90.x.x
...

# Vultr
validator-14    45.76.x.x
...

# DigitalOcean
validator-18    164.90.x.x
...
```

### Step 2: Upload Validator Binary

**For each VM:**

```bash
# Upload binary
scp /Users/macbook/Desktop/etrid/target/release/flarechain-node \
    root@VM_IP:/usr/local/bin/

# Make executable
ssh root@VM_IP "chmod +x /usr/local/bin/flarechain-node"
```

**Or use a loop:**

```bash
for ip in $(cat validator-ips.txt | grep -v "#" | awk '{print $2}'); do
  echo "Uploading to $ip..."
  scp /Users/macbook/Desktop/etrid/target/release/flarechain-node root@$ip:/usr/local/bin/
  ssh root@$ip "chmod +x /usr/local/bin/flarechain-node"
done
```

### Step 3: Upload Validator Keys

**For each validator, upload its specific keys:**

```bash
# Example for validator-01 (Gizzi)
ssh root@GIZZI_IP "mkdir -p /var/lib/etrid/keys"

# Extract keys from JSON
GIZZI_SESSION_SEED=$(jq -r '.validators[0].sessionKeys.seed' validator-keys-complete.json)

# Insert keys on VM
ssh root@GIZZI_IP "echo '$GIZZI_SESSION_SEED' | /usr/local/bin/flarechain-node key insert --base-path /var/lib/etrid --chain mainnet --scheme sr25519 --suri - --key-type aura"
```

(I'll create a full script for this)

### Step 4: Create systemd Service

**On each VM:**

```bash
ssh root@VM_IP << 'EOF'
cat > /etc/systemd/system/etrid-validator.service <<'SERVICE'
[Unit]
Description=Ëtrid Validator
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/flarechain-node \
  --base-path /var/lib/etrid \
  --chain mainnet \
  --name VALIDATOR_NAME \
  --validator \
  --port 30333 \
  --rpc-port 9944 \
  --prometheus-port 9615
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
SERVICE

systemctl daemon-reload
systemctl enable etrid-validator
systemctl start etrid-validator
EOF
```

### Step 5: Verify All Validators

**Check running:**

```bash
for ip in $(cat validator-ips.txt | grep -v "#" | awk '{print $2}'); do
  echo -n "$ip: "
  ssh root@$ip "systemctl is-active etrid-validator"
done
```

**Check committee size:**

```bash
curl -s http://FIRST_VM_IP:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "etrid_getCommittee"}' \
  | jq '.result | length'
```

**Should output: 21**

---

## Which Approach Should You Use?

### Use Automated (Option A) if:
- ✅ You're comfortable with command line
- ✅ You want to deploy fast (1 hour vs 6 hours)
- ✅ You may need to redeploy/scale later
- ✅ You value reproducibility

### Use Manual (Option B) if:
- ✅ You're new to command line
- ✅ You want to see each step visually
- ✅ You want full control over each setting
- ✅ You prefer UI/dashboard interfaces
- ✅ You're deploying for the first time

### Hybrid Approach (Recommended for you):
1. **Manual:** Order Hetzner bare metal (3 servers) - can't automate anyway
2. **Automated:** Deploy VPS to Hetzner, Vultr, DO via scripts
3. **Manual:** Deploy to Akash (requires learning Akash anyway)
4. **Automated:** Upload software and keys via scripts

**Best of both worlds!**

---

## Next Steps

**I'll create:**

1. ✅ `deploy-validator-software.sh` - Uploads binary, keys, creates services
2. ✅ `monitor-validators.sh` - Checks all 21 validators health
3. ✅ Manual deployment checklist with screenshots

**Which would you like me to create first?**

Or would you like to start with Option A (automated) or Option B (manual)?
