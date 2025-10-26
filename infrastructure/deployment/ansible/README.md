# Ëtrid Infrastructure Deployment

**Automated infrastructure deployment for both Ember Testnet and Etrid Mainnet**

---

## 📋 Overview

This directory contains Ansible playbooks and deployment scripts for deploying Ëtrid blockchain infrastructure to both testnet and mainnet environments.

**Supported Environments:**
- ✅ **Ember Testnet** - Public test network (low-cost, rapid iteration)
- ✅ **Etrid Mainnet** - Production network (enterprise security, multi-region)

**Infrastructure Components:**
- Validator nodes (3 testnet / 10 mainnet)
- Backup validators (2 testnet / 3 mainnet)
- RPC nodes (2 testnet / 10+ mainnet)
- Monitoring stack (Prometheus, Grafana)
- Block explorer (optional)

---

## 🗂️ Directory Structure

```
infrastructure/ansible/
├── README.md                          ← You are here
├── environments/
│   ├── testnet/
│   │   └── inventory/
│   │       └── hosts.yml              ← Testnet server inventory
│   └── mainnet/
│       └── inventory/
│           └── hosts.yml              ← Mainnet server inventory
├── playbooks/
│   ├── 01-provision-base.yml          ← Base system provisioning
│   ├── 02-deploy-validator.yml        ← Validator deployment
│   └── 03-setup-monitoring.yml        ← Monitoring stack
├── templates/
│   ├── etrid-validator.service.j2     ← Systemd service template
│   └── prometheus.yml.j2              ← Prometheus config template
├── scripts/
│   ├── deploy-testnet.sh              ← Testnet deployment script
│   └── deploy-mainnet.sh              ← Mainnet deployment script
├── files/
│   ├── etrid                          ← Substrate binary (place here)
│   └── {network}-chainspec.json      ← Chain specifications
└── backups/
    └── keys/
        ├── testnet/                   ← Testnet validator keys
        └── mainnet/                   ← Mainnet validator keys
```

---

## 🚀 Quick Start

### Testnet Deployment (Ember)

```bash
cd infrastructure/ansible

# 1. Check prerequisites
./scripts/deploy-testnet.sh check

# 2. Deploy everything
./scripts/deploy-testnet.sh all

# 3. Verify
./scripts/deploy-testnet.sh status
```

### Mainnet Deployment (Etrid)

```bash
cd infrastructure/ansible

# 1. Check prerequisites (includes safety prompts)
./scripts/deploy-mainnet.sh check

# 2. Deploy everything (with safety checks)
./scripts/deploy-mainnet.sh all

# 3. Verify
./scripts/deploy-mainnet.sh status
```

---

## 📝 Prerequisites

### 1. Local Machine Requirements

**macOS:**
```bash
brew install ansible python3
pip3 install jmespath
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install ansible python3-pip
pip3 install jmespath
```

**Verify:**
```bash
ansible --version  # Should be 2.9+
python3 --version  # Should be 3.8+
```

### 2. Cloud Provider Accounts

#### Testnet (Ember)
- **Hetzner Cloud** (primary provider)
  - Account: https://www.hetzner.com/cloud
  - Enable 2FA
  - Generate API token
- **OVH** (backup validators)
  - Account: https://www.ovhcloud.com/
  - Enable 2FA
  - Generate API credentials

#### Mainnet (Etrid)
- **AWS** (global infrastructure)
- **Hetzner** (EU validators)
- **OVH** (backup validators)

### 3. SSH Keys

**Testnet:**
```bash
ssh-keygen -t ed25519 -C "etrid-ember-testnet" -f ~/.ssh/etrid_ember
```

**Mainnet:**
```bash
ssh-keygen -t ed25519 -C "etrid-mainnet" -f ~/.ssh/etrid_mainnet
# Use a STRONG passphrase for mainnet!
```

**Upload to cloud providers:**
- Hetzner Console → Security → SSH Keys
- OVH Console → Public Cloud → SSH Keys

---

## 🔧 Configuration

### Step 1: Build Substrate Binary

```bash
cd /Users/macbook/Desktop/etrid

# Build optimized release
cargo build --release --locked

# Copy to ansible files directory
cp target/release/etrid infrastructure/ansible/files/

# Verify
infrastructure/ansible/files/etrid --version
```

### Step 2: Generate Chain Specification

**Testnet:**
```bash
cd /Users/macbook/Desktop/etrid

./target/release/etrid build-spec \
  --chain staging \
  --disable-default-bootnode \
  > infrastructure/ansible/files/ember-chainspec.json
```

**Mainnet:**
```bash
# Customize genesis for mainnet
./target/release/etrid build-spec \
  --chain mainnet \
  > infrastructure/ansible/files/etrid-chainspec-raw.json

# Edit genesis parameters (token distribution, validators, etc.)
vim infrastructure/ansible/files/etrid-chainspec-raw.json

# Generate raw chain spec
./target/release/etrid build-spec \
  --chain infrastructure/ansible/files/etrid-chainspec-raw.json \
  --raw \
  > infrastructure/ansible/files/etrid-chainspec.json
```

### Step 3: Provision Cloud Servers

#### Using Hetzner CLI (Recommended)

**Install:**
```bash
# macOS
brew install hcloud

# Linux
curl -L https://github.com/hetznercloud/cli/releases/latest/download/hcloud-linux-amd64.tar.gz | tar xz
sudo mv hcloud /usr/local/bin/
```

**Configure:**
```bash
hcloud context create etrid-testnet
# Enter your Hetzner API token
```

**Provision Testnet Servers:**
```bash
SSH_KEY_ID=$(hcloud ssh-key list -o columns=id -o noheader | head -1)

# 3 Validators (CPX51: 16 vCPU, 32GB RAM)
for i in {1..3}; do
  hcloud server create \
    --name validator$i \
    --type cpx51 \
    --image ubuntu-22.04 \
    --ssh-key $SSH_KEY_ID \
    --location fsn1
done

# 2 RPC Nodes (CPX41: 8 vCPU, 16GB RAM)
for i in {1..2}; do
  hcloud server create \
    --name rpc$i \
    --type cpx41 \
    --image ubuntu-22.04 \
    --ssh-key $SSH_KEY_ID \
    --location fsn1
done

# Monitoring (CPX31)
hcloud server create \
  --name monitoring1 \
  --type cpx31 \
  --image ubuntu-22.04 \
  --ssh-key $SSH_KEY_ID \
  --location fsn1

# Get server IPs
hcloud server list -o columns=name,ipv4
```

### Step 4: Update Inventory

```bash
# Get server IPs
hcloud server list

# Edit testnet inventory
vim environments/testnet/inventory/hosts.yml

# Replace all "0.0.0.0" with actual server IPs
# Example:
#   validator1:
#     ansible_host: 116.203.X.X  # ← UPDATE THIS
```

### Step 5: Test Connectivity

```bash
# Testnet
ansible all -i environments/testnet/inventory/hosts.yml -m ping

# Mainnet
ansible all -i environments/mainnet/inventory/hosts.yml -m ping

# Expected output: SUCCESS → pong
```

---

## 📦 Deployment

### Testnet Deployment

#### Option 1: Automated (Recommended)

```bash
./scripts/deploy-testnet.sh all
```

This will:
1. ✅ Check prerequisites
2. ✅ Provision base infrastructure (15 min)
3. ✅ Deploy validators (30 min)
4. ✅ Setup monitoring (10 min)
5. ✅ Verify deployment (5 min)

#### Option 2: Step-by-Step

```bash
# Step 1: Base provisioning
./scripts/deploy-testnet.sh base

# Step 2: Deploy validators
./scripts/deploy-testnet.sh validators

# Step 3: Setup monitoring
./scripts/deploy-testnet.sh monitoring

# Step 4: Verify
./scripts/deploy-testnet.sh verify
```

#### Option 3: Manual Ansible

```bash
# Base provisioning
ansible-playbook -i environments/testnet/inventory/hosts.yml \
  playbooks/01-provision-base.yml

# Deploy first validator
ansible-playbook -i environments/testnet/inventory/hosts.yml \
  playbooks/02-deploy-validator.yml \
  --limit validator1

# Deploy remaining validators
ansible-playbook -i environments/testnet/inventory/hosts.yml \
  playbooks/02-deploy-validator.yml

# Setup monitoring
ansible-playbook -i environments/testnet/inventory/hosts.yml \
  playbooks/03-setup-monitoring.yml
```

### Mainnet Deployment

⚠️ **WARNING:** Mainnet deployment includes safety prompts and requires manual confirmation.

```bash
# Full deployment (with safety checks)
./scripts/deploy-mainnet.sh all

# The script will prompt you to confirm:
# - Prerequisites completed
# - Security audit passed
# - Key ceremonies completed
# - Testnet successfully deployed
```

---

## 🔑 Session Keys Management

### During Deployment

Session keys are automatically generated and displayed during validator deployment:

```
================================================
🔑 SESSION KEYS FOR: validator1
VALIDATOR: Ember-Validator-1-EU
================================================
{
  "publicKey": "0x...",
  "secretPhrase": "word word word ...",
  "ss58Address": "5..."
}
================================================
⚠️  CRITICAL: Save these keys securely!
================================================
```

### Testnet Keys

Keys are automatically backed up to:
```
backups/keys/testnet/validator1-session-keys.json
backups/keys/testnet/validator2-session-keys.json
backups/keys/testnet/validator3-session-keys.json
```

**Storage:**
- 1Password
- Encrypted file (gpg)
- Hardware wallet (YubiKey)

### Mainnet Keys (CRITICAL)

**Mainnet keys require enhanced security:**

1. **Generate keys in ceremony:**
   - Air-gapped computer
   - Multiple witnesses
   - Video recorded

2. **Store in hardware security modules (HSM):**
   - Ledger Nano X (stash keys)
   - YubiHSM (session keys)
   - AWS CloudHSM (backup)

3. **Multi-signature setup:**
   - 3-of-5 threshold
   - Distributed geographically

4. **Backup procedures:**
   - Shamir's Secret Sharing (5-of-9 shards)
   - Bank vault storage
   - Regular recovery drills

---

## 📊 Monitoring

### Access Grafana

**Testnet:**
```
URL: http://<MONITORING_IP>:3000
Login: admin / CHANGE_ME_IMMEDIATELY_TESTNET
```

**Mainnet:**
```
URL: http://<MONITORING_IP>:3000
Login: admin / CHANGE_ME_IMMEDIATELY_MAINNET

⚠️ CRITICAL: Enable 2FA immediately!
```

### Import Dashboards

1. Login to Grafana
2. Click **+** → **Import**
3. Enter dashboard IDs:
   - **13759** - Substrate Node Metrics
   - **1860** - Node Exporter Full

### View Metrics

```bash
# Prometheus metrics
curl http://<VALIDATOR_IP>:9615/metrics

# Node health
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
     http://<VALIDATOR_IP>:9933

# Block height
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
     http://<VALIDATOR_IP>:9933 | jq .result.number
```

---

## 🔍 Verification

### Check Validator Status

```bash
# Service status
ssh validator1 systemctl status etrid

# View logs
ssh validator1 journalctl -u etrid -f

# Peer count
ssh validator1 curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' | jq

# Block production
ssh validator1 curl -s http://localhost:9933 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' | jq .result.number
```

### Check Network Connectivity

```bash
# All validators
ansible validators -i environments/testnet/inventory/hosts.yml \
  -m shell \
  -a "curl -s http://localhost:9933 -H 'Content-Type: application/json' \
      -d '{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"id\":1}' | jq .result"

# Expected output: {"peers": 2+, "isSyncing": false, "shouldHavePeers": true}
```

---

## 🛠️ Operations

### Restart Validator

```bash
# Single validator
ssh validator1 sudo systemctl restart etrid

# All validators (sequential)
ansible validators -i environments/testnet/inventory/hosts.yml \
  -a "systemctl restart etrid" \
  --forks 1
```

### View Logs

```bash
# Real-time logs
ssh validator1 journalctl -u etrid -f

# Last 100 lines
ssh validator1 journalctl -u etrid -n 100

# Logs from specific time
ssh validator1 journalctl -u etrid --since "1 hour ago"
```

### Backup Validator Data

```bash
# Run backup script (runs daily at 2 AM)
ssh validator1 /opt/etrid/backup-validator.sh

# Manual backup
ssh validator1
sudo tar -czf /var/backups/etrid/manual-backup-$(date +%Y%m%d).tar.gz \
  /var/lib/etrid/chains \
  /opt/etrid
```

### Update Substrate Binary

```bash
# Build new binary
cd /Users/macbook/Desktop/etrid
cargo build --release --locked

# Copy to ansible files
cp target/release/etrid infrastructure/ansible/files/

# Deploy to validators (one at a time to avoid downtime)
ansible-playbook -i environments/testnet/inventory/hosts.yml \
  playbooks/02-deploy-validator.yml \
  --limit validator1 \
  --tags binary

# Wait for validator1 to sync, then continue with validator2, etc.
```

---

## 🚨 Troubleshooting

### Validators Not Connecting

**Check firewall:**
```bash
ssh validator1 sudo ufw status
```

**Check P2P port:**
```bash
ssh validator1 netstat -tuln | grep 30333
```

**Check logs:**
```bash
ssh validator1 journalctl -u etrid -n 100
```

**Common issues:**
- Firewall blocking P2P port (30333)
- Wrong bootnode address in config
- Chain spec mismatch
- Insufficient peers

### Node Won't Start

**Check service status:**
```bash
ssh validator1 systemctl status etrid
```

**Check binary:**
```bash
ssh validator1 /opt/etrid/etrid --version
```

**Check permissions:**
```bash
ssh validator1 ls -la /var/lib/etrid
ssh validator1 ls -la /opt/etrid
```

**Reset and restart:**
```bash
ssh validator1
sudo systemctl stop etrid
sudo rm -rf /var/lib/etrid/chains
sudo systemctl start etrid
sudo journalctl -u etrid -f
```

### Low Performance

**Check resources:**
```bash
ssh validator1 htop
ssh validator1 iostat -x 1 5
```

**Optimize database cache:**
Edit `/etc/systemd/system/etrid.service`:
```
--db-cache 4096  # Increase from 2048
```

Then:
```bash
sudo systemctl daemon-reload
sudo systemctl restart etrid
```

---

## 📚 Additional Documentation

- **Game Plan:** `/ai-devs/EMBER_TESTNET_INTEGRATION_GAMEPLAN.md`
- **Mainnet Reusability:** `/ai-devs/MAINNET_REUSABILITY_ASSESSMENT.md`
- **Infrastructure Plan:** `/docs/deployment/EMBER_TESTNET_INFRASTRUCTURE_PLAN.md`

---

## 💰 Cost Estimates

### Testnet (Ember)

| Component | Qty | Monthly |
|-----------|-----|---------|
| Validators (CPX51) | 3 | $330 |
| Backup Validators (OVH) | 2 | $80 |
| RPC Nodes (CPX41) | 2 | $220 |
| Monitoring (CPX31) | 1 | $25 |
| **Total** | | **~$655/month** |

**Annual: ~$8,000**

### Mainnet (Etrid)

| Component | Qty | Monthly |
|-----------|-----|---------|
| Foundation Validators | 10 | $1,100 |
| Backup Validators | 3 | $120 |
| RPC Nodes (multi-region) | 10 | $1,650 |
| Monitoring (HA) | 2 | $200 |
| Security (HSM, VPN, SIEM) | - | $3,000 |
| CDN & DDoS Protection | - | $5,000 |
| **Total** | | **~$11,000/month** |

**Annual: ~$132,000** (infrastructure only)
**Total First Year: ~$1.2M** (including team, legal, compliance)

---

## ✅ Success Criteria

### Testnet Launch (Ember)

- [ ] 3 validators producing blocks
- [ ] Network achieving finality (<60s)
- [ ] Uptime >99%
- [ ] Peer count >4 per validator
- [ ] Monitoring operational
- [ ] Grafana dashboards functional
- [ ] Session keys backed up

### Mainnet Launch (Etrid)

- [ ] 10 Foundation validators operational
- [ ] Multi-region RPC infrastructure
- [ ] 99.99% uptime SLA
- [ ] Enterprise monitoring (PagerDuty)
- [ ] HSM key management
- [ ] Security audit passed
- [ ] Legal compliance complete
- [ ] 50+ community validators onboarded

---

## 🆘 Support

**Issues:** Open issue at https://github.com/EojEdred/Etrid/issues

**Emergency Contacts:**
- **Testnet:** #ember-infrastructure (Discord/Slack)
- **Mainnet:** 24/7 on-call rotation (PagerDuty)

---

**Status:** 🟢 Ready for Deployment
**Last Updated:** October 24, 2025
**Version:** 1.0

**Let's build Ëtrid! 🚀**
