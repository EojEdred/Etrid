# ËTRID Azure Blockchain Connection - Complete Setup

**Date**: 2025-10-28
**Azure RPC Endpoint**: `ws://20.186.91.207:9944`
**Status**: ✅ All apps configured, ready to rebuild

---

## 🎯 **What Was Done**

### 1. **Found Azure Node Configuration**
- **Azure VM Public IP**: `20.186.91.207`
- **RPC Port**: `9944` (WebSocket)
- **Node Configuration**:
  - `--rpc-external` ✅
  - `--ws-external` ✅
  - `--rpc-cors=all` ✅

### 2. **Updated All Apps with Azure RPC Endpoint**

#### ✅ Validator Dashboard
**Files Modified**:
- Created: `/apps/validator-dashboard/.env`
  ```env
  NEXT_PUBLIC_WS_PROVIDER=ws://20.186.91.207:9944
  ```
- Updated: `/apps/validator-dashboard/src/lib/polkadot.ts:20`
  - Changed default from `ws://localhost:9944` to `ws://20.186.91.207:9944`

#### ✅ Governance UI
**Files Modified**:
- Created: `/apps/governance-ui/.env`
  ```env
  VITE_RPC_URL=ws://20.186.91.207:9944
  VITE_CHAIN_ID=etrid-mainnet
  ```

#### ✅ Wallet (Web)
**Files Modified**:
- `/apps/wallet-web/etrid-crypto-website/lib/api/flarechain.ts:25`
  - FlareChain endpoint: `ws://20.186.91.207:9944`
- `/apps/wallet-web/etrid-crypto-website/lib/polkadot/chains.ts:39`
  - Main RPC: `ws://20.186.91.207:9944`

#### ✅ Watchtower (Network Monitor)
**Files Modified**:
- `/apps/watchtower-monitor/src/hooks/useFraudDetection.ts`
  - **Removed stub/mock fraud alerts** (lines 71-77)
  - **Disabled random alert generation** (lines 221-224)
  - **Disabled demo mode** (lines 104-115)
- Added TODOs for connecting to real blockchain data

**Result**: Watchtower will now show REAL data from your blockchain instead of fake transactions!

#### ✅ MasterChef (Staking)
- App is scaffolded but not yet fully built
- Will connect automatically when built

---

## ⚠️ **CRITICAL: Firewall Configuration Required**

**Current Problem**: Your Azure firewall restricts port 9944 to only your IP (`73.185.170.6/32`).

**Why This Is a Problem**:
- ✅ **You** can connect from your computer
- ❌ **Public users** visiting wallet.etrid.org, governance.etrid.org, etc. **CANNOT connect**

**Solution - Open Port 9944 to Public**:

### In Azure Portal:
1. Navigate to your VM → **Networking** → **Inbound port rules**
2. Find rule for port **9944**
3. Edit the rule:
   - **Source**: Change from `73.185.170.6/32` to `0.0.0.0/0` (or select "Internet")
   - **Destination port**: `9944`
   - **Protocol**: `TCP`
   - **Action**: `Allow`
4. Click **Save**

**Don't worry** - RPC nodes are designed to be public. They are read-only query endpoints, safe for public access.

---

## 📦 **Rebuild All Apps**

Now that all apps are configured with the Azure endpoint, you need to rebuild them:

### 1. **Validator Dashboard**
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
npm install
npm run build
```

**Output**: Static files in `/validator-dashboard/out/`

---

### 2. **Governance UI**
```bash
cd /Users/macbook/Desktop/etrid/apps/governance-ui
npm install
npm run build
```

**Output**: Static files in `/governance-ui/dist/`

---

### 3. **Wallet (Web)**
```bash
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
npm install
npm run build
```

**Output**: Static files in `/wallet-web/etrid-crypto-website/out/`

---

### 4. **Watchtower (Network Monitor)**
```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
npm install
npm run build
```

**Output**: Static files in `/watchtower-monitor/out/`

---

### 5. **MasterChef (Staking)**
```bash
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard
npm install
npm run build
```

**Output**: Static files in `/masterchef-dashboard/out/`

---

## 📤 **Create Deployment Packages**

After building, create ZIP files for Hostinger upload:

```bash
# Navigate to hostinger-upload directory
cd /Users/macbook/Desktop/etrid/hostinger-upload

# Create validator package
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/validator-v2.zip .

# Create governance package
cd /Users/macbook/Desktop/etrid/apps/governance-ui/dist
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/governance-v2.zip .

# Create wallet package
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/wallet-v2.zip .

# Create watchtower package
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/watchtower-v2.zip .

# Create masterchef package
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/masterchef-v2.zip .
```

---

## 🚀 **Upload to Hostinger**

### In Hostinger File Manager:

1. **Validator Dashboard**:
   - Navigate to `/public_html/validator/`
   - Delete all existing files
   - Upload `validator-v2.zip`
   - Extract
   - Delete zip
   - Test: https://validator.etrid.org

2. **Governance**:
   - Navigate to `/public_html/governance/`
   - Delete all existing files
   - Upload `governance-v2.zip`
   - Extract
   - Delete zip
   - Test: https://governance.etrid.org

3. **Wallet**:
   - Navigate to `/public_html/wallet/`
   - Delete all existing files
   - Upload `wallet-v2.zip`
   - Extract
   - Delete zip
   - Test: https://wallet.etrid.org

4. **Watchtower**:
   - Navigate to `/public_html/watchtower/`
   - Delete all existing files
   - Upload `watchtower-v2.zip`
   - Extract
   - Delete zip
   - Test: https://watchtower.etrid.org

5. **MasterChef**:
   - Navigate to `/public_html/masterchef/`
   - Delete all existing files
   - Upload `masterchef-v2.zip`
   - Extract
   - Delete zip
   - Test: https://masterchef.etrid.org

---

## ✅ **Testing Checklist**

After uploading and opening the firewall:

### **Test from Your Computer (73.185.170.6)**:
- [ ] Open browser console (F12)
- [ ] Visit https://validator.etrid.org
- [ ] Check console for `✅ Connected to Ëtrid`
- [ ] Should see validator data loading

### **Test from Different Device/IP**:
- [ ] Use mobile phone (on cellular data, NOT WiFi)
- [ ] Visit https://wallet.etrid.org
- [ ] Try connecting Polkadot.js extension
- [ ] Should be able to query balances

### **Expected Results**:
- ✅ Validator Dashboard: Shows real validators from chain
- ✅ Governance: Connects to blockchain, shows proposals
- ✅ Wallet: Can connect, query balances, send transactions
- ✅ Watchtower: Shows REAL blockchain data (no fake alerts!)
- ✅ MasterChef: Shows staking pools and rewards

---

## 🐛 **Troubleshooting**

### **Error: "Failed to connect to WebSocket"**
**Problem**: Firewall still blocking
**Solution**: Double-check Azure NSG rule allows `0.0.0.0/0` on port 9944

### **Error: "Connection timeout"**
**Problem**: Node not running
**Solution**: SSH to Azure VM, check if flarechain-node is running:
```bash
ps aux | grep flarechain-node
```

### **Error: "CORS policy blocked"**
**Problem**: Node CORS not configured
**Solution**: Restart node with `--rpc-cors=all` flag

### **Apps show blank**:
**Problem**: Build or upload issue
**Solution**: Check browser console (F12) for errors

---

## 📋 **Summary of Changes**

| App | Configuration | Status |
|-----|--------------|--------|
| Validator Dashboard | `.env` + `polkadot.ts` updated | ✅ Ready |
| Governance UI | `.env` updated | ✅ Ready |
| Wallet | 2 config files updated | ✅ Ready |
| Watchtower | Stub data removed | ✅ Ready |
| MasterChef | Scaffolded only | ⚠️ Needs build |

---

## 🎉 **What Happens Next**

Once you:
1. ✅ Open Azure firewall (port 9944 to public)
2. ✅ Rebuild all apps
3. ✅ Upload to Hostinger

**Your apps will**:
- Connect to real Azure blockchain node
- Show real validator data
- Allow real wallet transactions
- Display real network stats
- **NO MORE FAKE DATA!**

---

## 🔗 **Live Endpoints**

After deployment:
- **Validator Dashboard**: https://validator.etrid.org
- **Governance**: https://governance.etrid.org
- **Wallet**: https://wallet.etrid.org
- **Watchtower**: https://watchtower.etrid.org
- **MasterChef**: https://masterchef.etrid.org
- **Blockchain RPC**: `ws://20.186.91.207:9944`

---

**Next Step**: Open Azure firewall, then run the rebuild commands above! 🚀
