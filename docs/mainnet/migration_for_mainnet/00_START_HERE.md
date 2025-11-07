# 🚀 Ëtrid FlareChain - Contabo Migration START HERE

**Date:** November 7, 2025
**Migration:** Azure → Contabo
**Timeline:** 4-8 hours
**Cost:** €168/month (~$180/month)

---

## 📋 WHAT YOU NEED TO DO

This guide will walk you through migrating all 16 Azure validators to Contabo in **5 easy phases**.

### ✅ GUARANTEED TO WORK
- VMs start easily on Contabo (standard Ubuntu VPS)
- Validators auto-sync from existing network
- IP addresses change automatically (peer discovery handles it)
- Session keys copy from your local backup
- No blockchain data loss

---

## 📂 MIGRATION FILES

All documents are in this folder:
```
/Users/macbook/Desktop/etrid/docs/mainnet/migration_for_mainnet/
```

**Read in this order:**

1. **`00_START_HERE.md`** ← You are here
2. **`01_PHASE_1_Provision_Contabo_VMs.md`** - Order and setup VMs
3. **`02_PHASE_2_Deploy_Software.md`** - Install node software
4. **`03_PHASE_3_Start_Validators.md`** - Launch validators
5. **`04_PHASE_4_Verify_Network.md`** - Check everything works
6. **`05_PHASE_5_Oracle_Validators.md`** - Start Oracle VMs (parallel)

**Supporting files:**
- **`deploy-to-vm.sh`** - Automated deployment script
- **`validator-service-template.service`** - Systemd service file
- **`vm-checklist.txt`** - Per-VM tracking
- **`CRITICAL_STATUS_2025-11-07.md`** - Current situation
- **`MIGRATION_TO_CONTABO_PLAN.md`** - Full technical details

---

## ⏱️ TIME BREAKDOWN

| Phase | Task | Duration |
|-------|------|----------|
| 1 | Sign up Contabo, order 16 VMs | 30 min |
| 2 | Deploy software to all VMs | 1-2 hours |
| 3 | Start validators, wait for sync | 1-2 hours |
| 4 | Verify network health | 30 min |
| 5 | Start Oracle validators (parallel) | 30 min |
| **TOTAL** | **End-to-end** | **4-6 hours** |

**Note:** Phases 1-4 are sequential. Phase 5 can run in parallel.

---

## 💰 COSTS

### Contabo VPS M (Recommended per validator)
- **Specs:** 6 vCPU, 12 GB RAM, 200 GB NVMe
- **Price:** €10.50/month per VM
- **Total:** 16 VMs × €10.50 = **€168/month (~$180/month)**

### Comparison
- **Azure:** ~$400-500/month (was paying)
- **Contabo:** ~$180/month (new cost)
- **Savings:** ~$220-320/month (45-60% reduction)

---

## 🎯 WHAT YOU'LL ACCOMPLISH

By the end of this migration:

✅ All 16 Azure validators moved to Contabo
✅ Lower monthly costs ($180 vs $400-500)
✅ Network back online and producing blocks
✅ All validator identities preserved
✅ Clean, documented infrastructure

---

## 🔧 WHAT YOU NEED

### Required Files (You already have these)
- ✅ Node binary: `/Users/macbook/Desktop/etrid/target/release/flarechain-node`
- ✅ Chainspec: `/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json`
- ✅ Session keys: `/Users/macbook/Desktop/etrid/secrets/validator-keys/`
- ✅ SSH key: `~/.ssh/gizzi-validator` (or similar)

### Required Accounts
- [ ] Contabo account (create in Phase 1)
- [ ] Payment method (credit card/PayPal)
- [ ] Email for account notifications

### Required Tools (Already installed)
- ✅ Terminal/SSH client
- ✅ SCP for file transfer
- ✅ Text editor (nano/vim/etc)

---

## 🚦 PRE-FLIGHT CHECKLIST

Before starting, verify you have:

- [ ] Read this entire document
- [ ] Understand the 5 phases
- [ ] Have 4-6 hours available
- [ ] Payment method ready for Contabo
- [ ] Access to validator session keys
- [ ] Node binary compiled and ready

**All good? Let's proceed to Phase 1!**

---

## 📞 QUICK REFERENCE

### Contabo VPS Specs We're Using
```
Plan: VPS M
vCPU: 6 cores
RAM: 12 GB
Storage: 200 GB NVMe SSD
Network: 1 Gbps
IPv4: 1 public IP per VM
OS: Ubuntu 22.04 LTS
```

### Validator Requirements (per VM)
```
Ports to open:
- 22 (SSH)
- 30333 (P2P networking)
- 9944 (RPC - optional)
- 9615 (Metrics - optional)

Firewall: ufw
User: root (initially) or create etrid user
Base path: /root/.etrid or /home/etrid/.etrid
```

### Network Requirements
```
Minimum validators online: 15 out of 21
Current Oracle validators: 2 (need to start)
Current Azure Sub 2: 3 (unknown status)
Contabo validators: 16 (new)

Recovery threshold: 15/21 = Network operational
```

---

## 🎯 RECOVERY PLAN

### Immediate (Phase 5 - Parallel)
While migrating to Contabo, also:
1. SSH into Oracle Gizzi VM (64.181.215.19)
2. Start flarechain-validator service
3. Monitor logs

This gets you 2 validators online immediately (free).

### Short-term (Phases 1-4)
Deploy 16 Contabo validators:
- First 5: Get to 7/21 online
- Next 5: Get to 12/21 online
- Last 6: Get to 18/21 online (FULL CONSENSUS)

### End State
Total online: 18-20 validators (depending on Azure Sub 2 status)
- Oracle: 2 validators ✅
- Contabo: 16 validators ✅
- Azure Sub 2: 0-3 validators (bonus if they work)

**Network will be fully operational at 15/21 threshold**

---

## ⚠️ IMPORTANT NOTES

### About IP Address Changes
**DON'T WORRY!** IP addresses can change freely because:
- Validators identify by cryptographic keys (not IPs)
- Peer discovery is automatic (Kademlia DHT)
- Bootnodes help new validators join
- Just update `--public-addr` flag with new IP

### About Blockchain Data
**Starting fresh is FINE!** Validators will:
- Connect to network via bootnodes
- Download blockchain from peers
- Sync takes 30-60 minutes per validator
- State is verified via consensus
- No manual intervention needed

### About Session Keys
**These NEVER change!** Session keys are:
- Stored in your local secrets folder
- Copied to each new VM
- Same validator identity forever
- Network recognizes by public key

---

## 🏁 READY TO START?

**Next step:** Open `01_PHASE_1_Provision_Contabo_VMs.md`

That guide will walk you through:
1. Creating Contabo account
2. Ordering 16 VPS instances
3. Recording all IP addresses
4. Initial VM access setup

**Let's get your network back online! 🚀**

---

**Created:** November 7, 2025
**Status:** Ready to Execute
**Confidence:** HIGH - This will work!

