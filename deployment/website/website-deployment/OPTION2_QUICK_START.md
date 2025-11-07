# Option 2: EVM Deployment - Quick Start

**You chose:** Add EVM support to FlareChain → Deploy existing MasterChef.sol

**Benefits:**
- ✅ MasterChef.sol already exists (tested code)
- ✅ No new pallet development needed
- ✅ Deploy in 2-3 days instead of 3-4 weeks
- ✅ Compatible with existing EVM tools (MetaMask, Hardhat, ethers.js)

---

## 🚀 Quick Summary

**What we're doing:**
1. Add Frontier (EVM compatibility layer) to FlareChain runtime
2. Build and deploy new runtime
3. Deploy existing MasterChef.sol smart contract via EVM
4. Update web app to read from EVM contract

**Files created for you:**
- ✅ **EVM_DEPLOYMENT_GUIDE.md** - Complete step-by-step guide
- ✅ **MASTERCHEF_PALLET_EXPLAINED.md** - Background info

---

## 📋 Next Steps (In Order)

### Step 1: Update FlareChain Runtime (1-2 hours)

**Location:** `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/`

**Files to modify:**
1. `Cargo.toml` - Add Frontier dependencies
2. `src/lib.rs` - Configure EVM pallets

**See:** Section "Step 1" and "Step 2" in EVM_DEPLOYMENT_GUIDE.md

### Step 2: Build Runtime (45 minutes)

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
cargo clean
cargo build --release
```

**Coffee break time!** This takes 30-60 minutes.

### Step 3: Deploy New Runtime (30 minutes)

**Option A: Direct deployment (if you have sudo access)**
```bash
# Stop validators
# Replace runtime wasm
# Restart validators
```

**Option B: Governance upgrade (recommended)**
```bash
# Create runtime upgrade proposal
# Vote during next Consensus Day
# Auto-applies after approval
```

### Step 4: Test EVM is Working (5 minutes)

```bash
# Test EVM RPC
curl -X POST http://98.71.91.84:9933 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'

# Expected: {"jsonrpc":"2.0","result":"0x8274","id":1}
```

If you see chain ID `0x8274` (33396), EVM is working! ✅

### Step 5: Deploy MasterChef.sol (15 minutes)

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc

# Install dependencies
npm install

# Create deployment script (provided in guide)
# Edit .env with your private key

# Deploy
npx hardhat run scripts/deploy-masterchef-flarechain.ts --network flarechain
```

**Save the contract address!** You'll need it for the web app.

### Step 6: Add LP Pools (10 minutes)

Using the deployed contract:

```javascript
// Add ÉTR/EDSC pool
await masterChef.add(
  1000,        // allocation points (highest priority)
  LP_TOKEN_ADDRESS,
  false        // don't update other pools
);

// Add ÉTR/USDC pool
await masterChef.add(
  600,         // lower allocation
  LP_TOKEN_ADDRESS_2,
  false
);

// Add ÉTR single stake
await masterChef.add(
  400,
  ÉTR_TOKEN_ADDRESS,
  false
);
```

### Step 7: Update Web App (2-3 hours)

**Modify:** `/Desktop/etrid/etrid-hostinger-deployment/apps/masterchef/index.html`

**Add:**
1. ethers.js library (if not already included)
2. MasterChef contract ABI
3. Contract connection code
4. Real pool data fetching
5. Replace placeholder values with real data

**See:** Section "Step 5" in EVM_DEPLOYMENT_GUIDE.md

### Step 8: Deploy Updated Web App (5 minutes)

```bash
cd /Users/macbook/Desktop/etrid/etrid-hostinger-deployment
python3 upload-masterchef-fix.py
```

---

## 📊 Timeline Breakdown

| Task | Time | Can Skip? |
|------|------|-----------|
| Update Cargo.toml | 15 min | No |
| Configure runtime | 45 min | No |
| Build runtime | 45 min | No (automated) |
| Deploy runtime | 30 min | No |
| Test EVM | 5 min | No |
| Deploy MasterChef | 15 min | No |
| Add pools | 10 min | No |
| Update web app | 2-3 hrs | No |
| Test everything | 30 min | No |
| **TOTAL** | **~6-8 hours** | |

**Realistic timeline:**
- **Day 1:** Update and build runtime (2-3 hours)
- **Day 2:** Deploy runtime, deploy contract, add pools (1-2 hours)
- **Day 3:** Update web app, test, deploy (3-4 hours)

---

## 🎯 What You Get

**After completion:**
- ✅ FlareChain has EVM compatibility
- ✅ MasterChef smart contract deployed
- ✅ Web app shows REAL pool data:
  - Real APYs (not 245%, 128%, 85%)
  - Real TVLs (not $8.5M, $6.2M, $4.8M)
  - Real daily rewards (not 45K, 28K, 18K)
- ✅ Users can stake LP tokens
- ✅ Users can harvest rewards
- ✅ Users can see their pending rewards

**URLs:**
- MasterChef: https://etrid.org/masterchef/ (updated)
- EVM RPC: http://98.71.91.84:9933
- Substrate RPC: ws://98.71.91.84:9944

---

## 🔑 Key Configuration Values

**Chain ID:** 33396 (0x8274 hex)
**Gas Price:** 1 gwei (1,000,000,000 wei)
**Block Time:** ~6 seconds
**Reward Rate:** 2.89 ÉTR per block
**Daily Emission:** ~83,333 ÉTR
**Monthly Emission:** ~2.5M ÉTR

---

## 🛠️ Tools You'll Need

**Already have:**
- ✅ Rust toolchain
- ✅ Node.js / npm
- ✅ MasterChef.sol contract
- ✅ Deployment scripts

**May need to install:**
```bash
# Hardhat (if not installed)
npm install --global hardhat

# Foundry (optional, for testing)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# ethers.js (for web app)
# Already included in MasterChef deployment
```

---

## 📚 Documentation Reference

**Main Guide:** EVM_DEPLOYMENT_GUIDE.md
- Complete code for all changes
- Step-by-step instructions
- Troubleshooting section

**Background Info:** MASTERCHEF_PALLET_EXPLAINED.md
- What MasterChef is
- How it works
- Technical details

**This File:** Quick reference for what to do and when

---

## ⚠️ Important Notes

### Gas Fees
- EVM transactions use ÉTR for gas
- Gas price: 1 gwei = 0.000000001 ÉTR
- Deploying MasterChef: ~3M gas = 0.003 ÉTR
- Adding pool: ~500K gas = 0.0005 ÉTR
- User staking: ~150K gas = 0.00015 ÉTR

### Security
- ✅ MasterChef code is audited (based on PancakeSwap)
- ✅ Use multisig for contract ownership
- ✅ Test thoroughly before mainnet
- ⚠️ Transfer ownership after deployment

### Compatibility
- ✅ Works with MetaMask
- ✅ Works with Hardhat
- ✅ Works with ethers.js/web3.js
- ✅ Compatible with all EVM tools

---

## 🆘 If You Get Stuck

### Build Errors
**Problem:** Cargo build fails
**Solution:** Check Frontier branch matches Polkadot SDK version

```bash
# Check current Polkadot SDK version
grep "polkadot-sdk" Cargo.toml

# Use matching Frontier branch
# If SDK is "polkadot-stable2509", use same for Frontier
```

### EVM Not Responding
**Problem:** `eth_chainId` returns error
**Solution:** Ensure new runtime is deployed

```bash
# Check runtime version
curl -X POST http://98.71.91.84:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"state_getRuntimeVersion","params":[],"id":1}'
```

### Contract Deploy Fails
**Problem:** MasterChef deployment reverts
**Solution:** Check account has ÉTR balance

```bash
# Check balance via EVM
cast balance YOUR_ADDRESS --rpc-url http://98.71.91.84:9933
```

### Web App Not Updating
**Problem:** Still shows placeholder data
**Solution:** Check contract address is correct

```javascript
// In browser console
console.log(MASTERCHEF_ADDRESS);
console.log(await masterChefContract.poolLength());
```

---

## ✅ Success Checklist

Once complete, verify:

- [ ] `curl` to EVM RPC returns chain ID 33396
- [ ] MasterChef contract deployed (has bytecode)
- [ ] At least 1 LP pool added
- [ ] Web app connects to contract
- [ ] Web app shows real APY (not 245%)
- [ ] Can see pool TVL from contract
- [ ] Can stake LP tokens via UI
- [ ] Can harvest rewards via UI
- [ ] Pending rewards display correctly
- [ ] No more "Coming Soon" warning banner

---

## 🎉 After Completion

**Announce to community:**
- Blog post about MasterChef launch
- Twitter announcement
- Discord/Telegram updates

**Monitor:**
- Pool TVLs
- Reward distribution
- User participation
- Gas usage

**Future improvements:**
- Add more LP pools
- Adjust reward rates
- Introduce boosted rewards
- Add time-lock features

---

**Ready to start? Follow EVM_DEPLOYMENT_GUIDE.md Step 1!**

This is the faster path to get MasterChef working compared to building a native Substrate pallet from scratch.
