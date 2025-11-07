# 🚀 Ëtrid FlareChain - Contabo Migration Package

**Complete migration guide from Azure to Contabo**

---

## 📦 WHAT'S IN THIS FOLDER

This folder contains everything you need to migrate 16 Azure validators to Contabo and restore your FlareChain mainnet to full operation.

---

## 🎯 START HERE

### **If you're ready to migrate:**
👉 **Open: [`00_START_HERE.md`](00_START_HERE.md)**

That document explains:
- What happened (Azure subscription locked)
- Why migration works (IP changes are normal)
- What you'll accomplish (network restored, 45-70% cost savings)
- Time required (4-6 hours)

### **If you want the executive summary:**
👉 **Open: [`MIGRATION_SUMMARY.md`](MIGRATION_SUMMARY.md)**

Quick overview with:
- Time breakdown
- Cost analysis
- Execution checklist
- Quick commands

---

## 📚 ALL DOCUMENTS

### Phase Guides (Read in Order)
1. **[`00_START_HERE.md`](00_START_HERE.md)** - Overview and introduction
2. **[`01_PHASE_1_Provision_Contabo_VMs.md`](01_PHASE_1_Provision_Contabo_VMs.md)** - Order 16 VMs (30 min)
3. **[`02_PHASE_2_Deploy_Software.md`](02_PHASE_2_Deploy_Software.md)** - Install software (1-2 hours)
4. **[`03_PHASE_3_Start_Validators.md`](03_PHASE_3_Start_Validators.md)** - Launch validators (1-2 hours)
5. **[`04_PHASE_4_Verify_Network.md`](04_PHASE_4_Verify_Network.md)** - Verify network (30 min)
6. **[`05_PHASE_5_Oracle_Validators.md`](05_PHASE_5_Oracle_Validators.md)** - Start Oracle VMs (30 min)

### Reference Documents
- **[`MIGRATION_SUMMARY.md`](MIGRATION_SUMMARY.md)** - Quick reference and checklist
- **[`MIGRATION_TO_CONTABO_PLAN.md`](MIGRATION_TO_CONTABO_PLAN.md)** - Full technical details
- **[`CRITICAL_STATUS_2025-11-07.md`](CRITICAL_STATUS_2025-11-07.md)** - Current situation analysis

### Scripts
- **[`deploy-to-single-vm.sh`](deploy-to-single-vm.sh)** - Automated deployment to one VM
  ```bash
  ./deploy-to-single-vm.sh VM_IP VALIDATOR_NUMBER
  ```

---

## ⚡ QUICK START

```bash
# 1. Read the overview
open 00_START_HERE.md

# 2. Sign up at Contabo
# Visit: https://contabo.com

# 3. Order 16 VPS M instances
# Plan: VPS M (6 vCPU, 12 GB RAM, 200 GB NVMe)
# Cost: €10.50/month each = €168/month total

# 4. Deploy to each VM
./deploy-to-single-vm.sh VM_IP VALIDATOR_NUMBER

# 5. Start validators
ssh -i ~/.ssh/contabo-validators root@VM_IP "systemctl start flarechain-validator"

# 6. Monitor
ssh -i ~/.ssh/contabo-validators root@VM_IP "journalctl -u flarechain-validator -f"

# 7. Verify network
cd /Users/macbook/Desktop/etrid/docs/mainnet
bash check-validators-simple.sh
```

---

## 💰 COST SAVINGS

| Provider | VMs | Monthly Cost | Status |
|----------|-----|--------------|--------|
| **Azure (current)** | 16 | $400-500 | 🔴 LOCKED |
| **Contabo (new)** | 16 | €168 (~$180) | 🟢 READY |
| **SAVINGS** | - | **$220-320/month** | **✅ 45-70%** |

---

## ⏱️ TIME REQUIRED

| Phase | Duration | Can Skip? |
|-------|----------|-----------|
| Phase 1: Provision VMs | 30 min | No |
| Phase 2: Deploy Software | 1-2 hours | No |
| Phase 3: Start Validators | 1-2 hours | No |
| Phase 4: Verify Network | 30 min | Optional* |
| Phase 5: Oracle VMs | 30 min | Optional* |
| **TOTAL** | **4-6 hours** | |

*Can do later, but recommended

---

## 📋 PREREQUISITES

### You Need
- ✅ 4-6 hours of time
- ✅ Payment method (credit card/PayPal)
- ✅ Terminal/SSH access
- ✅ The files below (you already have them)

### Files Required
- ✅ Node binary: `/Users/macbook/Desktop/etrid/target/release/flarechain-node` (58 MB)
- ✅ Chainspec: `/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json` (2 MB)
- ✅ Session keys: `/Users/macbook/Desktop/etrid/secrets/validator-keys/` (21 validators)

---

## ✅ WHY THIS WORKS

### IP Addresses Can Change
Validators identify by **cryptographic session keys**, not IP addresses. The network uses Kademlia DHT for automatic peer discovery. Just update the `--public-addr` flag with the new IP.

### Session Keys Are Portable
Your session keys are stored locally. Copy them to new VMs = same validator identity. The network recognizes validators by their public keys, not their location.

### No Data Migration Needed
Validators will sync from the network in 30-60 minutes. The blockchain state is consensus-driven and verifiable. No manual data transfer required.

---

## 🎯 SUCCESS CRITERIA

After migration, you should have:

- [x] 16 Contabo VMs running validators
- [x] 15+ validators online (consensus threshold)
- [x] Block production active
- [x] Finality working (GRANDPA)
- [x] Peer connections healthy (8-15 per validator)
- [x] Monthly cost: ~$180 (down from $400-500)

---

## 🛠️ TROUBLESHOOTING

### "Can't SSH into Contabo VM"
- Check IP address is correct
- Verify SSH key: `~/.ssh/contabo-validators`
- Ensure firewall allows port 22
- Try password if key fails

### "Validator won't start"
- Check logs: `journalctl -u flarechain-validator -n 50`
- Verify binary is executable: `chmod +x /usr/local/bin/flarechain-node`
- Check chainspec exists: `ls /root/chainspec.json`
- Verify firewall: `ufw status` (port 30333 must be open)

### "Network won't achieve consensus"
- Need 15/21 validators minimum
- Check validator peer counts (should be 8-15)
- Verify bootnodes are accessible
- Wait for validators to sync (30-60 min)

**More troubleshooting in each phase guide.**

---

## 📞 SUPPORT

### Documentation
All phase guides include:
- Detailed step-by-step instructions
- Troubleshooting sections
- Verification checkpoints
- Common issues and solutions

### Scripts
Automated deployment script handles:
- Binary deployment
- Chainspec copying
- Session key installation
- Service configuration
- Deployment verification

### Health Monitoring
Use the health check script:
```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet
bash check-validators-simple.sh
```

---

## 🎉 WHAT YOU'LL ACHIEVE

### Immediate Benefits
- ✅ Network back online (4-6 hours)
- ✅ All validators operational
- ✅ Block production resumed
- ✅ Transaction processing restored

### Long-term Benefits
- ✅ 45-70% cost reduction
- ✅ No Azure dependency
- ✅ Better pricing stability
- ✅ Clean, documented infrastructure
- ✅ Easy maintenance and updates

---

## 🚀 READY?

1. **Read:** [`00_START_HERE.md`](00_START_HERE.md)
2. **Proceed:** To Phase 1 when ready
3. **Follow:** Step-by-step instructions
4. **Success:** Network restored in 4-6 hours!

---

## 📊 FOLDER CONTENTS

```
migration_for_mainnet/
├── README.md (this file)
├── 00_START_HERE.md
├── 01_PHASE_1_Provision_Contabo_VMs.md
├── 02_PHASE_2_Deploy_Software.md
├── 03_PHASE_3_Start_Validators.md
├── 04_PHASE_4_Verify_Network.md
├── 05_PHASE_5_Oracle_Validators.md
├── MIGRATION_SUMMARY.md
├── MIGRATION_TO_CONTABO_PLAN.md
├── CRITICAL_STATUS_2025-11-07.md
└── deploy-to-single-vm.sh
```

**Total:** 10 files, ~100 KB of documentation

---

**Created:** November 7, 2025
**Status:** ✅ Ready to Execute
**Confidence:** HIGH

**LET'S RESTORE YOUR NETWORK! 🚀**

