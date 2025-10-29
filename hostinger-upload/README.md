# ËTRID Hostinger Upload Package - Dual Bootstrap Node Edition

**Created**: 2025-10-28
**For**: Eoj
**Status**: ✅ Complete - Ready to Deploy

---

## 📦 **What's in This Package**

This directory contains everything you need to connect your ËTRID apps to your Azure blockchain nodes with automatic failover.

---

## 🎯 **Start Here**

**If you want to deploy quickly:**
→ Read **`START_HERE.md`** (5-minute overview)

**If you want all the details:**
→ Read **`DUAL_NODE_SETUP_COMPLETE.md`** (complete guide)

**If you want a quick reference:**
→ Read **`QUICK_REFERENCE.md`** (commands only)

---

## 📚 **Files & What They Do**

### **Quick Guides**
- **`START_HERE.md`** - Simple 3-step deployment guide ⭐ **READ FIRST**
- **`QUICK_REFERENCE.md`** - Quick command reference card
- **`README.md`** - This file

### **Technical Documentation**
- **`DUAL_NODE_SETUP_COMPLETE.md`** - Complete dual-node setup guide with failover logic
- **`AZURE_CONNECTION_COMPLETE.md`** - Technical details of all changes made
- **`COMPLETE_SETUP_SUMMARY.md`** - Full deployment summary with testing checklist
- **`APP_STATUS_EXPLAINED.md`** - Why apps weren't working before (root cause analysis)
- **`FINAL_SETUP_GUIDE.md`** - Original Hostinger setup guide

### **Documentation Portal**
- **`docs-portal/`** - Beautiful documentation hub (upload to docs.etrid.org)
  - `index.html` - Main docs page
  - `.htaccess` - Hostinger configuration
  - `README.md` - Deployment instructions

---

## ✅ **What Was Configured**

### **Your Two Azure Bootstrap Nodes**
1. **VM #1 (Alice)**: `20.186.91.207:9944` - Primary
2. **VM #2 (Bob)**: `172.177.44.73:9944` - Fallback

### **Apps Updated**
All 5 apps configured with automatic failover:
- ✅ Validator Dashboard → Tries both nodes
- ✅ Governance UI → Dual RPC URLs
- ✅ Wallet → Bootstrap nodes array
- ✅ Watchtower → Fake data REMOVED + dual nodes
- ✅ MasterChef → Dual endpoint support

### **Files Modified**
**Configuration Files:**
- `/apps/validator-dashboard/.env`
- `/apps/governance-ui/.env`

**Source Code:**
- `/apps/validator-dashboard/src/lib/polkadot.ts` - Added failover logic
- `/apps/wallet-web/etrid-crypto-website/lib/api/flarechain.ts` - Added bootstrap nodes array
- `/apps/wallet-web/etrid-crypto-website/lib/polkadot/chains.ts` - Updated RPC endpoint
- `/apps/watchtower-monitor/src/hooks/useFraudDetection.ts` - Removed all stub/mock data

---

## 🚀 **Deployment Steps**

### **1. Open Firewalls** (5 minutes)
Open port 9944 on **BOTH** Azure VMs to `0.0.0.0/0`

### **2. Rebuild Apps** (10-15 minutes)
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/governance-ui && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard && npm install && npm run build
```

### **3. Create Deployment Packages** (2 minutes)
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard/out && zip -r ../../hostinger-upload/validator-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/governance-ui/dist && zip -r ../../hostinger-upload/governance-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website/out && zip -r ../../../hostinger-upload/wallet-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor/out && zip -r ../../hostinger-upload/watchtower-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard/out && zip -r ../../hostinger-upload/masterchef-dual-node.zip .
cd /Users/macbook/Desktop/etrid/hostinger-upload/docs-portal && zip -r ../docs-portal.zip .
```

### **4. Upload to Hostinger** (10 minutes)
Upload and extract each ZIP to its subdomain directory

---

## 🎯 **Expected Results**

### **Before Deployment**
- ❌ Apps show "connection failed"
- ❌ Watchtower has FAKE transaction data
- ❌ Users can't connect wallets
- ❌ Everything localhost/demo mode

### **After Deployment**
- ✅ Apps connect to real Azure nodes
- ✅ **Automatic failover** between Alice and Bob
- ✅ Watchtower shows REAL blockchain data
- ✅ Users can send real transactions
- ✅ **High availability** - one node can go down and apps still work
- ✅ Beautiful documentation portal at docs.etrid.org

---

## ✅ **Testing Checklist**

- [ ] Open both Azure firewalls (port 9944 to `0.0.0.0/0`)
- [ ] Rebuild all 5 apps
- [ ] Create deployment ZIPs
- [ ] Upload to Hostinger
- [ ] Test validator.etrid.org (should show validators)
- [ ] Test wallet.etrid.org (should connect)
- [ ] Test watchtower.etrid.org (should show ZERO fake alerts)
- [ ] Test failover (stop Alice, app should use Bob)
- [ ] Test docs.etrid.org (should show documentation hub)

---

## 🔄 **How Failover Works**

When apps load, they:
1. Try to connect to **Alice** (20.186.91.207:9944)
2. If Alice responds → ✅ Use Alice
3. If Alice fails → Try **Bob** (172.177.44.73:9944)
4. If Bob responds → ✅ Use Bob
5. If both fail → ❌ Show clear error message

**Result**: Apps stay online even if one node goes down!

---

## 📊 **Key Features**

### **High Availability**
- Two bootstrap nodes
- Automatic failover
- No single point of failure

### **Real Data**
- Connects to actual blockchain
- No mock/stub transactions
- Real validator data

### **Beautiful Documentation**
- 12 organized sections
- Network statistics
- Links to all resources

### **Developer-Friendly**
- Clear console logging
- Shows which node connected
- Easy to debug

---

## 🐛 **Troubleshooting**

| Issue | Solution |
|-------|----------|
| Apps won't connect | Open both firewalls |
| Works for you, not others | Firewall needs `0.0.0.0/0` |
| Still seeing fake data | Rebuild & re-upload |
| Apps always use Alice | Normal behavior (Bob is fallback) |

**Full troubleshooting guide**: See `DUAL_NODE_SETUP_COMPLETE.md`

---

## 🔗 **Live URLs After Deployment**

- https://validator.etrid.org - Validator Dashboard
- https://governance.etrid.org - CONSËNSUS Governance
- https://wallet.etrid.org - Web Wallet
- https://watchtower.etrid.org - Network Monitor (no more fake data!)
- https://masterchef.etrid.org - Staking Dashboard
- https://docs.etrid.org - Documentation Hub (NEW!)
- https://etrid.org - Main Site
- https://etrid.org/whitepaper/ - Ivory Papers

---

## 💡 **Pro Tips**

1. **Always open both firewalls first** - Apps won't work without it
2. **Check browser console** (F12) - Shows which node it connected to
3. **Test failover** - Stop one node, verify app switches to the other
4. **Clear cache** - If you don't see changes after upload
5. **Use cellular data** - To test from different IP than yours

---

## 📞 **Need Help?**

**Quick Help**:
- Read `START_HERE.md` first
- Check `QUICK_REFERENCE.md` for commands

**Detailed Help**:
- `DUAL_NODE_SETUP_COMPLETE.md` - Full technical guide
- `COMPLETE_SETUP_SUMMARY.md` - Deployment checklist
- `AZURE_CONNECTION_COMPLETE.md` - All changes documented

---

## ✨ **Summary**

This package contains:
- ✅ All 5 apps configured for dual bootstrap nodes
- ✅ Automatic failover logic implemented
- ✅ Fake data removed from watchtower
- ✅ Beautiful documentation portal ready
- ✅ Complete deployment guides
- ✅ Testing instructions

**You're ready to deploy!**

---

**Created with ❤️ for ËTRID Protocol** | **The Free and Open Decentralized Democracy of Stakeholders**
