# 🚀 Ëtrid Launch Night Deployment Guide
## Liquidity Pool Creation with $750 Budget

**Created:** 2025-11-30
**Budget:** $750 USD for initial liquidity
**Time-Sensitive:** Production deployment for tonight's launch

---

## 📋 Quick Overview

You have **TWO main options** for liquidity pool deployment:

### Option 1: PrimeSwap DEX (Recommended for FlareChain/PBC)
- **Location:** `/Users/macbook/Desktop/etrid/contracts/primeswap/`
- **Type:** Uniswap V2-style AMM for your Substrate-based chain
- **Features:** Full DEX + MasterChef staking
- **Network:** Your FlareChain or any PBC (Partition Burst Chain)

### Option 2: Ethereum/EVM Chain Deployment
- **Location:** `/Users/macbook/Desktop/etrid/contracts/ethereum/`
- **Type:** Uniswap V3 pools on Ethereum mainnet or testnets
- **Features:** Bridge + ETR.e/EDSC.e tokens
- **Network:** Ethereum, BSC, or compatible EVM chains

---

## 🎯 RECOMMENDED: Option 1 - PrimeSwap Deployment

### Why PrimeSwap?
- ✅ Built specifically for Ëtrid ecosystem
- ✅ Complete DEX solution with staking
- ✅ 73 passing tests, production-ready
- ✅ Optimized for $750 budget
- ✅ Full control over liquidity parameters

### Step-by-Step Deployment

#### 1. Navigate to PrimeSwap Directory
```bash
cd /Users/macbook/Desktop/etrid/contracts/primeswap
```

#### 2. Setup Environment
```bash
# Copy environment template
cp .env.example .env

# Edit .env with your deployment wallet private key
nano .env
```

Add to `.env`:
```env
# Your deployment wallet private key (needs ~1 ETH for gas + $750 for liquidity)
PRIVATE_KEY=your_private_key_here

# Network RPC (if deploying to specific network)
MAINNET_RPC_URL=https://your-rpc-url
GOERLI_RPC_URL=https://goerli.infura.io/v3/YOUR_KEY

# Optional: Block explorer API for verification
ETHERSCAN_API_KEY=your_key_here
BSCSCAN_API_KEY=your_key_here
```

#### 3. Install Dependencies
```bash
npm install
```

#### 4. Compile Contracts
```bash
npx hardhat compile
```

#### 5. Run Tests (Optional but recommended)
```bash
npx hardhat test
```

#### 6. Deploy Full Stack (DEX + Staking)
```bash
# For local testing first:
npx hardhat node  # Terminal 1
npx hardhat run scripts/deploy-full.js --network localhost  # Terminal 2

# For production deployment:
npx hardhat run scripts/deploy-full.js --network mainnet  # or your network
```

**This deploys:**
- ✅ WETH (Wrapped ETH)
- ✅ PrimeSwapFactory
- ✅ PrimeSwapRouter
- ✅ ETR Token (100M supply)
- ✅ MasterChef (staking contract)
- ✅ Test tokens (USDC, DAI)
- ✅ 3 trading pairs (ETR/USDC, ETR/WETH, USDC/DAI)
- ✅ 3 staking pools pre-configured

**Output:** Saves addresses to `deployments-full.json`

#### 7. Add Initial Liquidity ($750 Budget)
```bash
npx hardhat run scripts/add-initial-liquidity.js --network mainnet
```

**This script automatically:**
- ✅ Loads deployment addresses from `deployments-full.json`
- ✅ Adds liquidity to ETR/USDC pool (~$400)
- ✅ Adds liquidity to ETR/WETH pool (~$350)
- ✅ Verifies pool reserves
- ✅ Shows confirmation and pool addresses

**Liquidity Breakdown ($750 total):**
- **Pool 1 - ETR/USDC:** $400
  - 100,000 ETR tokens
  - 400 USDC
  - Initial price: 1 ETR = $0.004

- **Pool 2 - ETR/WETH:** $350
  - 87,500 ETR tokens
  - 0.1 WETH (≈$350 at $3,500/ETH)
  - Provides ETH trading pairs

#### 8. Verify Contracts (Block Explorer)
```bash
npx hardhat run scripts/verify.js --network mainnet
```

---

## 🏗️ Option 2: Ethereum/Uniswap V3 Deployment

### When to Use This?
- You want to deploy on Ethereum mainnet
- You need ETR.e (bridged ETR) on Ethereum
- You want Uniswap V3 integration
- **WARNING:** Requires significantly more liquidity (~$3M recommended)

### Deployment Steps

#### 1. Navigate to Ethereum Contracts
```bash
cd /Users/macbook/Desktop/etrid/contracts/ethereum
```

#### 2. Setup Environment
```bash
cp .env.example .env
nano .env
```

Add your deployment wallet and RPC URLs.

#### 3. Deploy Main Contracts
```bash
npm install
npx hardhat compile
npx hardhat run scripts/deploy.js --network sepolia  # testnet first!
```

This deploys:
- ETRToken (ETR.e - bridged ETR)
- EDSCToken (EDSC.e - bridged stablecoin)
- EtridBridge (cross-chain bridge)

#### 4. Create Uniswap V3 Pools
```bash
# Add contract addresses to .env:
# ETR_TOKEN_ADDRESS=0x...
# EDSC_TOKEN_ADDRESS=0x...

npx hardhat run scripts/create-uniswap-pools.js --network sepolia
```

#### 5. Add Liquidity via Uniswap Interface
- **Not recommended for $750 budget** - Uniswap V3 requires more capital
- Recommended amounts: $400k+ for meaningful liquidity
- Use Uniswap app: https://app.uniswap.org/pools

---

## 💰 Budget Allocation Strategy ($750)

### Recommended Split:

| Pool | Amount | Percentage | Purpose |
|------|--------|------------|---------|
| **ETR/USDC** | $400 | 53% | Main trading pair, price discovery |
| **ETR/WETH** | $350 | 47% | ETH liquidity, DEX routing |
| **Reserve** | $0 | 0% | Optional: keep for emergencies |

### Initial Token Economics:
- **ETR Price:** $0.004 (1000 ETR = $4)
- **Total ETR in pools:** 187,500 ETR
- **Circulating supply:** From your 100M total supply
- **Remaining:** 99,812,500 ETR for ecosystem, staking rewards, etc.

---

## ⚙️ Pre-Launch Checklist

### Before Deployment:
- [ ] Have deployment wallet ready with private key
- [ ] Wallet has sufficient gas (≈0.5-1 ETH for deployment)
- [ ] Wallet has $750 worth of tokens/stablecoins
- [ ] `.env` file configured correctly
- [ ] Decided on deployment network (mainnet, testnet, FlareChain, etc.)
- [ ] Tested on testnet/localhost first

### During Deployment:
- [ ] Run `deploy-full.js` successfully
- [ ] Save `deployments-full.json` file (contains all addresses)
- [ ] Run `add-initial-liquidity.js`
- [ ] Verify pool reserves are correct
- [ ] Test a small swap to confirm functionality

### After Deployment:
- [ ] Save all contract addresses securely
- [ ] Verify contracts on block explorer
- [ ] Update frontend with contract addresses
- [ ] Set up monitoring/alerts
- [ ] Announce pools to community
- [ ] Monitor first trades closely
- [ ] Plan for adding more liquidity as volume grows

---

## 🔧 Key Files and Locations

### PrimeSwap (Recommended)
```
/Users/macbook/Desktop/etrid/contracts/primeswap/
├── scripts/
│   ├── deploy-full.js              # Main deployment script
│   ├── add-initial-liquidity.js    # Add $750 liquidity (NEW!)
│   ├── deploy.js                   # DEX-only deployment
│   └── verify.js                   # Contract verification
├── src/
│   ├── core/                       # Factory, Pair, ERC20
│   ├── periphery/                  # Router, WETH
│   └── farming/                    # MasterChef staking
├── DEPLOYMENT_GUIDE.md             # Full documentation
└── README.md                       # Quick reference
```

### Ethereum Contracts (Alternative)
```
/Users/macbook/Desktop/etrid/contracts/ethereum/
├── scripts/
│   ├── deploy.js                   # Deploy ETR.e, EDSC.e, Bridge
│   └── create-uniswap-pools.js     # Create Uniswap V3 pools
└── README.md                       # Documentation
```

---

## 🚨 Common Issues and Solutions

### Issue 1: "Insufficient funds for gas"
**Solution:** Ensure deployment wallet has at least 1 ETH for gas fees

### Issue 2: "Module not found" errors
**Solution:**
```bash
cd /Users/macbook/Desktop/etrid/contracts/primeswap
rm -rf node_modules package-lock.json
npm install
```

### Issue 3: "deployments-full.json not found"
**Solution:** Run `deploy-full.js` first before `add-initial-liquidity.js`

### Issue 4: Wrong network deployment
**Solution:** Check `hardhat.config.js` and ensure correct `--network` flag

### Issue 5: Transaction reverted
**Solution:** Check:
- Wallet has enough tokens approved
- Deadline hasn't expired
- Slippage tolerance is reasonable

---

## 📊 Post-Launch Monitoring

### What to Monitor:
1. **Trading Volume:** Track daily volume in pools
2. **Price Stability:** Monitor ETR price against USDC/WETH
3. **Liquidity Depth:** Ensure $750 is providing good trades
4. **Impermanent Loss:** Calculate IL as price moves
5. **Gas Costs:** Monitor transaction costs for users

### When to Add More Liquidity:
- Trading volume consistently >$10k/day
- Slippage on trades >5% for $100 swaps
- Community requests deeper liquidity
- Price stabilizes at your target range

---

## 🎯 Quick Commands Reference

### PrimeSwap Deployment (All-in-One)
```bash
# Setup
cd /Users/macbook/Desktop/etrid/contracts/primeswap
cp .env.example .env
nano .env  # Add PRIVATE_KEY
npm install
npx hardhat compile

# Deploy (choose one network)
npx hardhat run scripts/deploy-full.js --network localhost
npx hardhat run scripts/deploy-full.js --network goerli
npx hardhat run scripts/deploy-full.js --network mainnet

# Add Liquidity
npx hardhat run scripts/add-initial-liquidity.js --network <same-network>

# Verify
npx hardhat run scripts/verify.js --network <same-network>
```

### Test Everything Locally First
```bash
# Terminal 1
npx hardhat node

# Terminal 2
npx hardhat run scripts/deploy-full.js --network localhost
npx hardhat run scripts/add-initial-liquidity.js --network localhost
```

---

## 📞 Emergency Contacts & Resources

### Documentation:
- **PrimeSwap Guide:** `/Users/macbook/Desktop/etrid/contracts/primeswap/DEPLOYMENT_GUIDE.md`
- **Ethereum Guide:** `/Users/macbook/Desktop/etrid/contracts/ethereum/README.md`
- **Ëtrid Docs:** https://docs.etrid.org

### Need Help?
- Check test files for usage examples
- Review deployment logs carefully
- Test on testnet before mainnet
- Keep deployment wallet keys secure

---

## ✅ Final Checklist for Tonight's Launch

1. **Choose deployment option:**
   - [ ] Option 1: PrimeSwap (Recommended) ✨
   - [ ] Option 2: Ethereum/Uniswap V3

2. **Prepare deployment wallet:**
   - [ ] Private key added to `.env`
   - [ ] Sufficient gas (~1 ETH)
   - [ ] $750 available for liquidity

3. **Deploy contracts:**
   - [ ] Compile successful
   - [ ] Deploy successful
   - [ ] Addresses saved

4. **Add liquidity:**
   - [ ] $750 distributed across pools
   - [ ] Reserves verified
   - [ ] Test swap executed

5. **Go live:**
   - [ ] Contracts verified on explorer
   - [ ] Frontend updated with addresses
   - [ ] Community announcement ready
   - [ ] Monitoring setup

---

## 🌟 Success Criteria

Your launch is successful when:
- ✅ All contracts deployed without errors
- ✅ $750 liquidity added to pools
- ✅ Test swaps execute correctly
- ✅ Pool reserves match expectations
- ✅ Users can trade ETR tokens
- ✅ Prices are stable and reasonable
- ✅ No contract errors or reverts

---

**Good luck with your launch tonight, Eoj! 🚀**

**Remember:** Test on testnet first, deploy to mainnet when confident!

---

**Last Updated:** 2025-11-30
**Created by:** Claude Code
**For:** Ëtrid Launch Night
