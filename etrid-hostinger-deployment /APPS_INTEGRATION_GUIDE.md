# 🚀 ËTRID Apps Integration Guide

## 📱 What Apps Are Available?

You have **7 fully-built Next.js apps** ready to deploy to your Hostinger website!

### Apps in `/etrid-hostinger-deployment /apps/`:

| App | Purpose | Size | URL After Deploy |
|-----|---------|------|------------------|
| **validator** | Validator Dashboard | ~880 KB | https://etrid.org/validator/ |
| **masterchef** | MasterChef LP Rewards Dashboard | ~534 KB | https://etrid.org/masterchef/ |
| **wallet** | Web3 Wallet Interface | ~947 KB | https://etrid.org/wallet/ |
| **telemetry** | Network Telemetry | ~224 KB | https://etrid.org/telemetry/ |
| **governance** | Governance/Voting Interface | ~14 KB | https://etrid.org/governance/ |
| **explorer** | Blockchain Explorer | ~3 KB | https://etrid.org/explorer/ |
| **bridge** | Cross-chain Bridge UI | ~3 KB | https://etrid.org/bridge/ |

---

## 🎯 What These Apps Do

### 1. Validator Dashboard (`/validator/`)
**Purpose:** Monitor and manage ËTRID validators

**Features:**
- Real-time validator status
- Performance analytics (uptime, blocks produced)
- Nominator management
- Reward tracking (90 days history)
- Commission management
- Alert system
- Era & session information

**Who needs it:** Validator operators

---

### 2. MasterChef Dashboard (`/masterchef/`)
**Purpose:** Monitor LP staking rewards

**Features:**
- Real-time pool statistics (TVL, APR)
- Emissions tracking (daily/monthly/yearly)
- MasterChef balance tracking
- TVL distribution charts
- Auto-refresh every 60 seconds

**Who needs it:** LP token stakers, DeFi users

---

### 3. Wallet (`/wallet/`)
**Purpose:** Web3 wallet interface for ËTRID

**Features:**
- Connect Web3 wallets (MetaMask, WalletConnect)
- View balances (ETR, EDSC, VMW)
- Send/receive transactions
- Transaction history
- Token management

**Who needs it:** All users

---

### 4. Telemetry (`/telemetry/`)
**Purpose:** Network health monitoring

**Features:**
- Real-time network stats
- Validator telemetry
- Block production rates
- Network health indicators

**Who needs it:** Network operators, curious users

---

### 5. Governance (`/governance/`)
**Purpose:** Community governance & voting

**Features:**
- View active proposals
- Cast votes
- Track voting history
- Consensus Day information

**Who needs it:** ÉTR token holders, governance participants

---

### 6. Explorer (`/explorer/`)
**Purpose:** Blockchain explorer

**Features:**
- Search blocks, transactions, addresses
- View blockchain state
- Transaction details
- Account information

**Who needs it:** All users

---

### 7. Bridge (`/bridge/`)
**Purpose:** Cross-chain asset bridging

**Features:**
- Bridge assets between chains
- Track bridge transactions
- View supported chains

**Who needs it:** Users bridging assets

---

## 🚀 Quick Deploy (All Apps - 15 Minutes)

### Option 1: Automated Upload (Recommended)

```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "
python3 upload-all-apps.py
```

**What it does:**
1. Checks all 7 apps are present
2. Asks for FTP credentials once
3. Lets you choose which apps to upload (all or select)
4. Uploads each app to its own subdirectory
5. Reports success/failure for each

**Time:** ~15 minutes for all 7 apps

---

### Option 2: Manual Upload (FileZilla)

1. **Connect to Hostinger via FTP**
2. **Navigate to** `public_html/`
3. **For each app:**
   - Create folder (e.g., `validator/`)
   - Upload all files from `/apps/validator/` to `public_html/validator/`
   - Repeat for each app

**Time:** ~30 minutes for all 7 apps

---

## 📋 Deployment Strategy

### Recommended Deployment Order:

**Phase 1 - Essential (Deploy First):**
1. **Wallet** - Core functionality
2. **Explorer** - Basic blockchain interaction
3. **Validator** - For validator operators

**Phase 2 - DeFi (Deploy Second):**
4. **MasterChef** - LP staking rewards
5. **Bridge** - Cross-chain transfers

**Phase 3 - Governance (Deploy Third):**
6. **Governance** - Community voting
7. **Telemetry** - Network monitoring

---

## 🔗 After Deployment - Update Main Website

Once apps are deployed, add links to them on your main website!

### Update Homepage Navigation

Edit `/website/index.html` to add app links:

```html
<!-- In the navigation menu -->
<a href="/wallet/" class="hover:text-etrid-blue transition-colors">Wallet</a>
<a href="/validator/" class="hover:text-etrid-purple transition-colors">Validators</a>
<a href="/masterchef/" class="hover:text-etrid-cyan transition-colors">Staking</a>
<a href="/explorer/" class="hover:text-etrid-blue transition-colors">Explorer</a>
<a href="/bridge/" class="hover:text-etrid-purple transition-colors">Bridge</a>
<a href="/governance/" class="hover:text-etrid-cyan transition-colors">Governance</a>
```

---

## 🧪 Testing After Deployment

For each app, test:

### 1. Check URL Works:
```
https://etrid.org/validator/
https://etrid.org/masterchef/
https://etrid.org/wallet/
... etc
```

### 2. Check Loading:
- Page should load (not 404)
- CSS/JS should load (check Network tab in F12)
- No console errors

### 3. Check Functionality:
- Buttons work
- Links navigate correctly
- Connect wallet feature works (if applicable)

---

## ⚙️ How These Apps Work

### Tech Stack:
- **Framework:** Next.js 14 (React)
- **Export:** Static HTML (no server required!)
- **Styling:** Tailwind CSS
- **Blockchain:** ethers.js / Polkadot.js

### Why They're Static:
- These apps were built with `next export`
- Generates static HTML/CSS/JS files
- No Node.js server needed
- Can be hosted on any static host (like Hostinger)
- Client-side JavaScript handles all interactions

### API/Data Sources:
- **Validator Dashboard:** Connects to Polkadot RPC endpoint
- **MasterChef:** Reads from BSC (Binance Smart Chain)
- **Wallet:** Uses Web3 provider (MetaMask, etc.)
- **Explorer:** Connects to Substrate node
- **Network Monitoring:** Uses existing Grafana (port 3000)

---

## 🔧 Configuration (If Needed)

Some apps may need configuration for your specific setup:

### Validator Dashboard:
- **RPC Endpoint:** Update in app to point to your FlareChain RPC
- **Validator Address:** Users input their own

### MasterChef:
- **Metrics JSON:** Upload latest `metrics.json` to `/masterchef/`
- **Contract Address:** Should already be configured for mainnet

### Wallet:
- **Chain ID:** Should match your FlareChain ID
- **RPC URL:** Point to your node

### Bridge:
- **Supported Chains:** Configure which chains are bridgeable
- **Contract Addresses:** Update for mainnet

---

## 📊 Monitoring After Deployment

### Analytics to Track:
- **Page views** per app (use Hostinger analytics or Google Analytics)
- **Most used apps** (wallet/explorer likely highest)
- **Bounce rate** (if high, UX issues)
- **User feedback** (set up support channel)

### Performance:
- **Load time** (should be <3 seconds)
- **Mobile responsiveness** (test on phone/tablet)
- **Error rates** (check browser console logs)

---

## 🐛 Troubleshooting

### App Shows 404:
- **Cause:** Directory not created or files not uploaded
- **Fix:**
  1. Check FTP - folder should exist at `public_html/validator/`
  2. Files should be inside that folder
  3. Must have `index.html` in the folder

### App Loads But Broken/White Screen:
- **Cause:** Assets not loading (CSS/JS files)
- **Fix:**
  1. Check browser console (F12) for 404 errors
  2. Verify `_next/` folder uploaded correctly
  3. Check `.htaccess` file exists

### "Connect Wallet" Not Working:
- **Cause:** Web3 provider not detected or CORS issues
- **Fix:**
  1. Install MetaMask or compatible wallet
  2. Check console for errors
  3. May need to configure RPC endpoint

### App Shows Old Data:
- **Cause:** Browser cache or static data outdated
- **Fix:**
  1. Clear browser cache (Cmd+Shift+R)
  2. For MasterChef: Upload fresh `metrics.json`
  3. For Validator: RPC endpoint may be down

---

## 🔒 Security Considerations

### HTTPS:
- ✅ Hostinger provides SSL automatically
- Apps will work on https://etrid.org/

### Wallet Security:
- ✅ Apps never store private keys
- ✅ Uses Web3 providers (MetaMask) for security
- ⚠️ Always verify RPC endpoints are official

### API Keys:
- Check apps for hardcoded API keys
- Use environment variables if needed (not possible in static export)
- Consider using server-side proxy for sensitive APIs

---

## 💡 Future Enhancements

### Add to Apps:

1. **Search Functionality**
   - Add search bar to explorer
   - Search validators by address

2. **Real-time Updates**
   - Use WebSocket connections
   - Auto-refresh without page reload

3. **Mobile Apps**
   - Convert to React Native
   - Deploy to App Store / Play Store

4. **Advanced Features**
   - Portfolio tracker (all holdings)
   - Price charts integration
   - Notification system

---

## 📝 Summary

### What You Have:
- ✅ 7 fully-built Next.js apps
- ✅ Static HTML exports (ready to deploy)
- ✅ Professional UI matching your brand
- ✅ Responsive design (mobile/tablet/desktop)

### What You Need to Do:
1. Run `upload-all-apps.py` script
2. Enter FTP credentials
3. Select which apps to deploy
4. Wait ~15 minutes for upload
5. Test each app URL
6. Add links to main website navigation

### Result:
- Users can access all apps via:
  - https://etrid.org/wallet/
  - https://etrid.org/validator/
  - https://etrid.org/masterchef/
  - ... etc

---

## 🚀 Ready to Deploy?

### Quick Deploy All Apps:

```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "
python3 upload-all-apps.py
```

### Or Deploy One App at a Time:

Upload individual folders via FileZilla:
- `/apps/wallet/` → `public_html/wallet/`
- `/apps/validator/` → `public_html/validator/`
- ... etc

---

**Your apps are built and ready! Deploy them to give users access to the full ËTRID ecosystem.** 🚀

---

## 📞 Need Help?

Check documentation in each app folder:
- `/apps/validator/README.md`
- `/apps/masterchef/README.md`
- ... etc

Or review the deployment script:
- `/upload-all-apps.py` (automated upload)
