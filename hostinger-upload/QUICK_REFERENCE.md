# ⚡ ËTRID DUAL-NODE SETUP - QUICK REFERENCE

**Date**: 2025-10-28
**For**: Eoj
**Status**: ✅ All apps configured with automatic failover

---

## 🎯 **Your Two Nodes**

| Node | IP | Endpoint | Role |
|------|-----|----------|------|
| **Alice** (VM #1) | `20.186.91.207` | `ws://20.186.91.207:9944` | Primary |
| **Bob** (VM #2) | `172.177.44.73` | `ws://172.177.44.73:9944` | Fallback |

---

## ✅ **What Was Done**

1. ✅ All apps configured with **BOTH** endpoints
2. ✅ Added **automatic failover** logic
3. ✅ Removed **fake data** from watchtower
4. ✅ Built **documentation portal**
5. ✅ Apps try Alice first, then Bob if Alice fails

---

## 🔥 **3-Step Deployment**

### **Step 1: Open Firewalls** ⚠️ CRITICAL
```
Azure Portal → VM #1 → Networking → Port 9944 → Source: 0.0.0.0/0
Azure Portal → VM #2 → Networking → Port 9944 → Source: 0.0.0.0/0
```

### **Step 2: Rebuild Apps**
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/governance-ui && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor && npm install && npm run build
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard && npm install && npm run build
```

### **Step 3: Create ZIPs & Upload**
```bash
# Create ZIPs
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard/out && zip -r ../../hostinger-upload/validator-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/governance-ui/dist && zip -r ../../hostinger-upload/governance-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website/out && zip -r ../../../hostinger-upload/wallet-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor/out && zip -r ../../hostinger-upload/watchtower-dual-node.zip .
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard/out && zip -r ../../hostinger-upload/masterchef-dual-node.zip .
cd /Users/macbook/Desktop/etrid/hostinger-upload/docs-portal && zip -r ../docs-portal.zip .

# Upload to Hostinger:
# validator-dual-node.zip → /public_html/validator/
# governance-dual-node.zip → /public_html/governance/
# wallet-dual-node.zip → /public_html/wallet/
# watchtower-dual-node.zip → /public_html/watchtower/
# masterchef-dual-node.zip → /public_html/masterchef/
# docs-portal.zip → /public_html/docs/
```

---

## 🧪 **Quick Test**

```bash
# Open browser console (F12)
# Visit: https://validator.etrid.org

# Expected output:
✅ Connected to Ëtrid blockchain at ws://20.186.91.207:9944
# (or ws://172.177.44.73:9944 if Alice is down)
```

---

## 🔄 **How Failover Works**

```
App loads → Try Alice (20.186.91.207:9944)
              ↓
         Alice up? ─YES→ ✅ Use Alice
              │
              NO
              ↓
         Try Bob (172.177.44.73:9944)
              ↓
         Bob up? ─YES→ ✅ Use Bob
              │
              NO
              ↓
         ❌ Show Error
```

---

## 📝 **Files Created**

All in `/Users/macbook/Desktop/etrid/hostinger-upload/`:

1. **`START_HERE.md`** ← Read this first!
2. **`DUAL_NODE_SETUP_COMPLETE.md`** ← Technical details
3. **`COMPLETE_SETUP_SUMMARY.md`** ← Full guide
4. **`AZURE_CONNECTION_COMPLETE.md`** ← File changes list
5. **`docs-portal/`** ← Upload to docs.etrid.org
6. **`QUICK_REFERENCE.md`** ← This file

---

## 🐛 **Common Issues**

| Problem | Solution |
|---------|----------|
| Apps won't connect | Open **both** firewalls (Step 1) |
| Works for you, not others | Firewall needs `0.0.0.0/0` not just your IP |
| App always uses Alice | Normal! Bob is fallback |
| Fake data in watchtower | Rebuild & re-upload after changes |

---

## 🎯 **Success Criteria**

- [ ] Both VMs have port 9944 open to `0.0.0.0/0`
- [ ] All apps rebuilt with new configuration
- [ ] ZIPs uploaded and extracted on Hostinger
- [ ] validator.etrid.org shows validator data
- [ ] wallet.etrid.org can connect
- [ ] watchtower.etrid.org shows ZERO fake alerts
- [ ] Console shows which node it connected to
- [ ] Test: Stop Alice → Apps automatically use Bob

---

## 🚀 **What You Get**

✅ **High Availability**: One node can go down, apps still work
✅ **Redundancy**: Two bootstrap nodes for the network
✅ **Automatic Failover**: No manual intervention needed
✅ **Real Data**: No more fake transactions or alerts
✅ **Beautiful Docs**: Professional documentation portal
✅ **Zero Downtime**: Update one node while other serves traffic

---

## 🔗 **Quick Links**

- **Validator**: https://validator.etrid.org
- **Governance**: https://governance.etrid.org
- **Wallet**: https://wallet.etrid.org
- **Watchtower**: https://watchtower.etrid.org (no more fake data!)
- **MasterChef**: https://masterchef.etrid.org
- **Docs**: https://docs.etrid.org (NEW!)

---

**Need help?** Read `START_HERE.md` or `DUAL_NODE_SETUP_COMPLETE.md`

**Ready to deploy?** Just follow the 3 steps above! 🚀
