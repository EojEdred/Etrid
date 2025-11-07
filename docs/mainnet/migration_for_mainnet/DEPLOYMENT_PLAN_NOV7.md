# FlareChain + PBC Deployment Plan
**Date:** November 7, 2025
**Strategy:** Canary + Rolling Upgrade
**Target:** All 16 Contabo Validators + PBC Collators

---

## 🎯 Deployment Objectives

1. **Upgrade FlareChain** from Nov 5 to Nov 7 binary (latest HEAD)
2. **Deploy 13 PBC Collators** for cross-chain functionality
3. **Maintain network uptime** throughout (canary + rolling)
4. **Zero data loss** (keep old binaries as backup)

---

## 📦 Binaries to Deploy

### FlareChain Node
- **Current:** `flarechain-node` (Nov 5, commit 67b7d7b4)
- **New:** `flarechain-node` (Nov 7, commit 592de040)
- **Build:** In progress locally (ARM → need x86_64 Linux)
- **Size:** ~75MB
- **Target:** All 16 Contabo validators

### PBC Collators (13 total)
Building via GitHub Actions:
1. btc-pbc-collator
2. sol-pbc-collator
3. bnb-pbc-collator
4. edsc-pbc-collator
5. eth-pbc-collator
6. xrp-pbc-collator
7. matic-pbc-collator
8. sc-usdt-pbc-collator
9. xlm-pbc-collator
10. trx-pbc-collator
11. ada-pbc-collator
12. link-pbc-collator
13. doge-pbc-collator

---

## 🎪 Phase 1: Canary Deployment (Est: 1 hour)

### Step 1.1: Prepare Canary (Validator-21)
**Target:** 154.12.249.182 (New York)

```bash
# SSH into canary
ssh -i ~/.ssh/contabo-validators root@154.12.249.182

# Backup current binary
cp /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup-nov5
ls -lh /usr/local/bin/flarechain-node*

# Stop validator
systemctl stop flarechain-validator
```

### Step 1.2: Deploy New Binary
```bash
# Upload new binary (from local build or GitHub Actions)
scp -i ~/.ssh/contabo-validators \
  /Users/macbook/Desktop/etrid/target/release/flarechain-node \
  root@154.12.249.182:/usr/local/bin/flarechain-node

# Set permissions
ssh -i ~/.ssh/contabo-validators root@154.12.249.182 \
  "chmod +x /usr/local/bin/flarechain-node"
```

### Step 1.3: Start and Monitor
```bash
# Start validator
systemctl start flarechain-validator

# Monitor logs
journalctl -u flarechain-validator -f
```

### Step 1.4: Validation Checklist (30 mins)
Monitor for these indicators:

- ✅ Service starts successfully
- ✅ Connects to bootnodes
- ✅ Discovers peers (should see 10-15 peers)
- ✅ Imports blocks (syncing)
- ✅ No consensus errors
- ✅ No state migration errors
- ✅ Memory usage stable (~3-4GB)
- ✅ No crashes for 30 minutes

**Decision Point:**
- ✅ All checks pass → Proceed to Phase 2
- ❌ Any failures → Rollback canary, investigate

### Step 1.5: Rollback Procedure (if needed)
```bash
systemctl stop flarechain-validator
cp /usr/local/bin/flarechain-node.backup-nov5 /usr/local/bin/flarechain-node
systemctl start flarechain-validator
```

---

## 🔄 Phase 2: Rolling Upgrade (Est: 1-2 hours)

### Wave 1: Seattle Validators (5 VMs)
**Time:** 15 mins per wave
**Network Status:** 12/21 validators during upgrade (>15 needed, safe)

```bash
# Upgrade these in parallel:
85.239.239.194  # Validator-6
85.239.239.193  # Validator-7
85.239.239.190  # Validator-8
85.239.239.189  # Validator-9
85.239.239.188  # Validator-10
```

**Script:**
```bash
for ip in 85.239.239.194 85.239.239.193 85.239.239.190 85.239.239.189 85.239.239.188; do
  echo "Upgrading $ip..."
  ssh -i ~/.ssh/contabo-validators root@$ip \
    "cp /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup-nov5 && \
     systemctl stop flarechain-validator"

  scp -i ~/.ssh/contabo-validators \
    /Users/macbook/Desktop/etrid/target/release/flarechain-node \
    root@$ip:/usr/local/bin/flarechain-node

  ssh -i ~/.ssh/contabo-validators root@$ip \
    "chmod +x /usr/local/bin/flarechain-node && \
     systemctl start flarechain-validator"
done

# Wait and verify
sleep 60
for ip in 85.239.239.194 85.239.239.193 85.239.239.190 85.239.239.189 85.239.239.188; do
  echo "$ip status:"
  ssh -i ~/.ssh/contabo-validators root@$ip "systemctl is-active flarechain-validator"
done
```

**Validation:** All 5 should be ACTIVE before proceeding

---

### Wave 2: Portsmouth Validators (6 VMs)
**Time:** 15 mins per wave
**Network Status:** 11/21 validators during upgrade (still >15, safe)

```bash
# Upgrade these in parallel:
80.190.82.186   # Validator-11
80.190.82.185   # Validator-12
80.190.82.184   # Validator-13
80.190.82.183   # Validator-14
158.220.83.146  # Validator-15
158.220.83.66   # Validator-16
```

**Script:** (same as Wave 1, different IPs)

---

### Wave 3: New York Validators (4 remaining)
**Time:** 15 mins
**Network Status:** Canary already upgraded, so 4 left

```bash
# Upgrade these in parallel:
154.12.250.18   # Validator-17
154.12.250.17   # Validator-18
154.12.250.15   # Validator-19
154.12.249.223  # Validator-20
# 154.12.249.182 (Validator-21) = CANARY, already done
```

---

## 🚀 Phase 3: PBC Deployment (Est: 2-3 hours)

### Step 3.1: Download PBC Binaries
```bash
# Wait for GitHub Actions to complete (~30-60 mins)
gh run watch

# Download artifacts
gh run download <run-id> --name pbc-collators-deployment-package
tar -xzf pbc-collators-*.tar.gz
```

### Step 3.2: Choose PBC Deployment Strategy

**Option A: Deploy to Existing Validators (Simpler)**
- Run PBC collators alongside FlareChain validators
- Each VM runs: 1 FlareChain validator + 1-2 PBC collators
- Uses same VMs, lower infrastructure cost

**Option B: Dedicated PBC VMs (Cleaner)**
- Provision separate VMs for PBC collators
- Better isolation
- Higher cost

**Recommendation:** Start with Option A (same VMs)

### Step 3.3: PBC Validator Assignment

**Priority PBCs to deploy first:**
1. **EDSC-PBC** (Validators 6-9: 4 validators)
2. **BTC-PBC** (Validators 10-13: 4 validators)
3. **ETH-PBC** (Validators 14-17: 4 validators)
4. **SOL-PBC** (Validators 18-21: 4 validators)

**Deployment per VM:**
```bash
# Example: Validator-6 runs EDSC-PBC
ssh -i ~/.ssh/contabo-validators root@85.239.239.194

# Install EDSC-PBC binary
cp binaries/edsc-pbc-collator /usr/local/bin/
chmod +x /usr/local/bin/edsc-pbc-collator

# Create systemd service
cat > /etc/systemd/system/edsc-pbc-collator.service <<'EOF'
[Unit]
Description=EDSC PBC Collator
After=network.target flarechain-validator.service
Wants=network-online.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=/usr/local/bin/edsc-pbc-collator \
  --chain /root/edsc-pbc-chainspec.json \
  --base-path /root/.etrid/edsc-pbc \
  --validator \
  --name "EDSC-PBC-Validator-6" \
  --port 30334 \
  --rpc-port 9945 \
  --prometheus-port 9616 \
  --rpc-cors all

Restart=always
RestartSec=10s
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable edsc-pbc-collator
systemctl start edsc-pbc-collator
```

### Step 3.4: Generate PBC Chainspecs
```bash
# Generate for each PBC
for pbc in edsc btc eth sol; do
  /usr/local/bin/${pbc}-pbc-collator build-spec \
    --chain local \
    --raw \
    --disable-default-bootnode \
    > /root/${pbc}-pbc-chainspec.json
done
```

### Step 3.5: Generate & Insert PBC Session Keys
```bash
# For each PBC collator
curl -H "Content-Type: application/json" \
  -d '{"id":1,"jsonrpc":"2.0","method":"author_rotateKeys","params":[]}' \
  http://localhost:9945

# Insert generated keys
# (Save keys in secrets folder)
```

---

## 📊 Monitoring & Validation

### FlareChain Validators
```bash
# Check all validators
for ip in <all-16-ips>; do
  echo "$ip:"
  ssh -i ~/.ssh/contabo-validators root@$ip \
    "systemctl is-active flarechain-validator && \
     journalctl -u flarechain-validator -n 3 --no-pager | grep Imported"
done
```

### PBC Collators
```bash
# Check PBC status
systemctl status edsc-pbc-collator
journalctl -u edsc-pbc-collator -f
```

### Network Health
- **Blocks producing:** Check latest block number increasing
- **Finality:** GRANDPA finalizing blocks
- **Peer count:** Each validator should have 10-15 peers
- **No forks:** All validators on same best block hash

---

## 🛟 Rollback Procedures

### FlareChain Rollback (per validator)
```bash
systemctl stop flarechain-validator
cp /usr/local/bin/flarechain-node.backup-nov5 /usr/local/bin/flarechain-node
systemctl start flarechain-validator
```

### PBC Rollback
```bash
systemctl stop edsc-pbc-collator
systemctl disable edsc-pbc-collator
# Old state preserved in /root/.etrid/edsc-pbc/
```

---

## ✅ Success Criteria

### FlareChain Upgrade Complete When:
- ✅ All 16 validators upgraded
- ✅ All validators ACTIVE and importing blocks
- ✅ Network consensus maintained
- ✅ No crashes or errors for 1 hour
- ✅ Peers discovering each other

### PBC Deployment Complete When:
- ✅ At least 2 PBCs deployed (EDSC + BTC minimum)
- ✅ 4 collators per PBC chain
- ✅ All collators producing blocks
- ✅ Session keys inserted
- ✅ No errors in logs for 30 mins

---

## 📅 Timeline

**Total Est: 4-6 hours**

| Phase | Task | Duration | Status |
|-------|------|----------|--------|
| 0 | Build binaries | 30-60 mins | 🔄 In Progress |
| 1 | Canary deployment | 1 hour | ⏳ Pending |
| 2 | Rolling upgrade (Wave 1) | 15 mins | ⏳ Pending |
| 2 | Rolling upgrade (Wave 2) | 15 mins | ⏳ Pending |
| 2 | Rolling upgrade (Wave 3) | 15 mins | ⏳ Pending |
| 3 | Download PBC binaries | 5 mins | ⏳ Pending |
| 3 | Deploy PBC to 16 VMs | 1-2 hours | ⏳ Pending |
| 4 | Monitoring & verification | 1 hour | ⏳ Pending |

---

## 🎯 Current Status

- ✅ Changes analyzed (LOW-MEDIUM risk)
- ✅ GitHub Actions triggered for PBC build
- 🔄 FlareChain binary building locally
- ⏳ Waiting for builds to complete
- ⏳ Ready to begin canary deployment

**Next Action:** Wait for binary builds, then deploy to Validator-21

---

*Deployment plan created November 7, 2025*
