# 🚀 QUICK START: Deploy ÉTR to DEXes

**Status:** ✅ READY TO DEPLOY
**Estimated Time:** 2-3 hours (Phase 1)
**Estimated Cost:** $350 gas + $7M liquidity

---

## ⚡ Super Quick Deploy (TL;DR)

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment

# 1. Check you have funds
./scripts/check-balances.sh

# 2. Deploy everything (interactive)
./DEPLOY_ALL_DEX.sh

# 3. Verify all contracts
./scripts/verify-all-contracts.sh

# 4. Test deployments
./scripts/test-all-deployments.sh

# 5. Generate report
./scripts/generate-deployment-report.sh
```

**Done! ÉTR is now live on 4 major DEXes! 🎉**

---

## 📋 Pre-Deployment Checklist

### ✅ Prerequisites

- [ ] **Node.js v18+** installed (`node --version`)
- [ ] **npm v9+** installed (`npm --version`)
- [ ] **Solana CLI** installed (for Solana deployment)
- [ ] **Foundation Approval:** 6-of-9 Director signatures obtained
- [ ] **Funds Available:**
  - [ ] 0.05 BNB (~$15) for BSC
  - [ ] 0.1 ETH (~$300) for Ethereum
  - [ ] 10 MATIC (~$10) for Polygon
  - [ ] 0.05 SOL (~$7) for Solana
- [ ] **Liquidity Ready:**
  - [ ] 90M ÉTR (from Community LP Pool allocation)
  - [ ] $7M in native tokens (BNB, ETH, MATIC, SOL)

### 🔐 Security Setup

- [ ] Private key stored securely (hardware wallet recommended)
- [ ] Block explorer API keys obtained:
  - [ ] BSCScan API key: https://bscscan.com/myapikey
  - [ ] Etherscan API key: https://etherscan.io/myapikey
  - [ ] PolygonScan API key: https://polygonscan.com/myapikey
- [ ] 2FA enabled on all accounts
- [ ] Foundation multisig address ready

---

## 🎯 Deployment Steps (Detailed)

### Step 1: Install Dependencies (5 minutes)

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment

# BSC
cd bsc
npm install
cd ..

# Ethereum
cd ethereum
npm install
cd ..

# Polygon
cd polygon
npm install
cd ..
```

### Step 2: Configure Environment (10 minutes)

```bash
# BSC
cd bsc
cp .env.example .env
nano .env  # Add PRIVATE_KEY and BSCSCAN_API_KEY

# Ethereum
cd ../ethereum
cp .env.example .env
nano .env  # Add PRIVATE_KEY and ETHERSCAN_API_KEY

# Polygon
cd ../polygon
cp .env.example .env
nano .env  # Add PRIVATE_KEY and POLYGONSCAN_API_KEY

# Solana (if not already configured)
cd ../solana
# Solana uses ~/.config/solana/id.json
solana config set --url mainnet-beta
```

### Step 3: Test on Testnets (30 minutes - RECOMMENDED)

```bash
# BSC Testnet
cd bsc
npm run deploy:testnet
# Test pool creation, swaps, etc.

# Ethereum Testnet
cd ../ethereum
npm run deploy:testnet
# Test pool creation, swaps, etc.

# Polygon Testnet
cd ../polygon
npm run deploy:testnet
# Test pool creation, swaps, etc.

# Solana Devnet
cd ../solana
./deploy-solana.sh  # Select option 1 (Devnet)
```

**⚠️ IMPORTANT:** Fix any issues on testnet before mainnet deployment!

### Step 4: Deploy to Mainnet (30 minutes)

#### Option A: Use Master Script (Recommended)

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment
./DEPLOY_ALL_DEX.sh
```

Follow the prompts:
1. Select **"Phase 1: Priority DEXes"**
2. Select **"Mainnet (PRODUCTION)"**
3. Type **"I UNDERSTAND"** to confirm
4. Type **"yes"** to proceed
5. Wait for deployments to complete

#### Option B: Deploy Individually

```bash
# BSC (5 min)
cd bsc
npm run deploy:mainnet
# Save contract address!

# Solana (5 min)
cd ../solana
./deploy-solana.sh  # Select option 2 (Mainnet)
# Save token mint address!

# Ethereum (10 min - more expensive, takes longer)
cd ../ethereum
npm run deploy:mainnet
# Save contract address!

# Polygon (5 min)
cd ../polygon
npm run deploy:mainnet
# Save contract address!
```

### Step 5: Verify Contracts (10 minutes)

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment
./scripts/verify-all-contracts.sh
```

Or verify individually:

```bash
# BSC
cd bsc
npx hardhat verify --network bscMainnet <CONTRACT_ADDRESS> <DEPLOYER_ADDRESS>

# Ethereum
cd ../ethereum
npx hardhat verify --network mainnet <CONTRACT_ADDRESS> <DEPLOYER_ADDRESS>

# Polygon
cd ../polygon
npx hardhat verify --network polygon <CONTRACT_ADDRESS> <DEPLOYER_ADDRESS>

# Solana (auto-verified)
# No action needed - SPL tokens are automatically visible on Solscan
```

### Step 6: Transfer Ownership (5 minutes)

**CRITICAL:** Transfer ownership to Foundation multisig!

```javascript
// For each EVM chain (BSC, Ethereum, Polygon)
const token = await ethers.getContractAt("ERC20", "<TOKEN_ADDRESS>");
await token.transferOwnership("<FOUNDATION_MULTISIG_ADDRESS>");
```

### Step 7: Create Liquidity Pools (60 minutes)

#### PancakeSwap (BSC)

1. Go to: https://pancakeswap.finance/liquidity
2. Click **"Add Liquidity"**
3. **Token A:** ÉTR (your BSC contract address)
4. **Token B:** WBNB (0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c)
5. **Fee:** 0.25%
6. **Amount:** 25M ÉTR + 3,333 BNB
7. **Approve** and **Confirm**

#### Raydium (Solana)

1. Go to: https://raydium.io/liquidity/create/
2. **Token A:** ÉTR (your Solana token mint)
3. **Token B:** SOL (So11111111111111111111111111111111111111112)
4. **Fee:** 0.25%
5. **Amount:** 25M ÉTR + 13,333 SOL
6. **Approve** and **Confirm**

#### Uniswap (Ethereum)

1. Go to: https://app.uniswap.org/pools
2. Click **"Create Pool"**
3. **Token 0:** ÉTR (your Ethereum contract address)
4. **Token 1:** WETH (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2)
5. **Fee:** 0.30%
6. **Amount:** 25M ÉTR + 666 ETH
7. **Approve** and **Confirm**

#### QuickSwap (Polygon)

1. Go to: https://quickswap.exchange/#/pools
2. Click **"Create Pool"**
3. **Token 0:** ÉTR (your Polygon contract address)
4. **Token 1:** WMATIC (0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270)
5. **Fee:** 0.30%
6. **Amount:** 15M ÉTR + 1M MATIC
7. **Approve** and **Confirm**

### Step 8: Test Trading (15 minutes)

Perform small test swaps on each DEX:

**Test Swaps:**
- Buy ÉTR: Swap $100 of native token → ÉTR
- Sell ÉTR: Swap $100 of ÉTR → native token
- Check price impact (should be <1%)
- Verify balances update correctly

### Step 9: Submit to Tracking Sites (30 minutes)

#### CoinGecko

1. Go to: https://www.coingecko.com/en/coins/new
2. Fill out form:
   - **Name:** Ëtrid Coin
   - **Symbol:** ÉTR
   - **Website:** https://etrid.org
   - **Description:** (from metadata files)
   - **Contracts:**
     - BSC: `<your BSC address>`
     - Ethereum: `<your ETH address>`
     - Polygon: `<your Polygon address>`
     - Solana: `<your Solana mint>`
   - **Logo:** Upload or link to https://etrid.org/images/etr-logo.png
3. Submit!

#### CoinMarketCap

1. Go to: https://coinmarketcap.com/request/
2. Fill out similar form (same info as CoinGecko)
3. Submit!

**Note:** Approval takes 1-7 days. Be patient!

### Step 10: Announce! (15 minutes)

**Social Media Posts:**

```
🚀 ÉTR is now live on 4 major DEXes!

Buy, sell, and trade ÉTR on:
🥞 PancakeSwap (BSC)
🦄 Uniswap (Ethereum)
⚡ QuickSwap (Polygon)
☀️ Raydium (Solana)

Contract Addresses:
📍 BSC: <address>
📍 ETH: <address>
📍 Polygon: <address>
📍 Solana: <mint>

Learn more: https://etrid.org
#DeFi #Blockchain #ÉTR
```

**Post on:**
- Twitter/X: @EtridProtocol
- Discord: #announcements
- Telegram: t.me/EtridOfficial
- Reddit: r/EtridBlockchain

**Update Website:**
- Add "Buy ÉTR" button with links to DEXes
- Add contract addresses to docs
- Update homepage with "Now Trading!" banner

---

## 🔍 Post-Deployment Checklist

### Immediate (Day 1)

- [ ] All contracts deployed and verified
- [ ] Ownership transferred to Foundation multisig
- [ ] Liquidity pools created (4 DEXes)
- [ ] Initial liquidity added (90M ÉTR + $7M)
- [ ] Test swaps completed successfully
- [ ] CoinGecko submission sent
- [ ] CoinMarketCap submission sent
- [ ] Social media announcements posted
- [ ] Website updated with contract addresses
- [ ] Deployment report generated and shared with Foundation

### Week 1

- [ ] Monitor trading volume (target: $500k/day)
- [ ] Monitor liquidity depth
- [ ] Respond to community questions
- [ ] Track CoinGecko/CMC approval status
- [ ] Adjust price ranges if needed (minimal)
- [ ] Launch LP rewards program (if planned)

### Week 2-4 (Phase 2 Expansion)

- [ ] Deploy to Avalanche (Trader Joe)
- [ ] Deploy to Arbitrum (Camelot)
- [ ] Deploy to Base (Aerodrome)
- [ ] Add to DEX aggregators (Jupiter, 1inch, Matcha)
- [ ] Apply to Gate.io and KuCoin
- [ ] Cross-chain bridge integration

---

## 💰 Cost Summary

| Item | Amount | USD Value |
|------|--------|-----------|
| **Gas Fees** | | |
| BSC deployment | 0.02 BNB | $6 |
| Ethereum deployment | 0.05 ETH | $150 |
| Polygon deployment | 5 MATIC | $5 |
| Solana deployment | 0.03 SOL | $4.50 |
| Pool creation (all) | Various | ~$200 |
| **Subtotal Gas** | | **~$365** |
| | | |
| **Liquidity** | | |
| BSC (PancakeSwap) | 25M ÉTR + 3,333 BNB | $2M |
| Ethereum (Uniswap) | 25M ÉTR + 666 ETH | $2M |
| Polygon (QuickSwap) | 15M ÉTR + 1M MATIC | $1M |
| Solana (Raydium) | 25M ÉTR + 13,333 SOL | $2M |
| **Subtotal Liquidity** | **90M ÉTR** | **$7M** |
| | | |
| **GRAND TOTAL** | | **~$7,000,365** |

**Source:** Community LP Pool (250M ÉTR allocation per Foundation Charter)

---

## 🆘 Troubleshooting

### "Insufficient funds for gas"

**Solution:**
- Check balance: `./scripts/check-balances.sh`
- Add more funds to your address
- For EVM chains, use the same address
- For Solana, fund your Solana keypair

### "Contract verification failed"

**Solution:**
- Wait 1-2 minutes and try again
- Verify manually: `npx hardhat verify --network <network> <address> <constructor args>`
- Check API key is correct in .env

### "Transaction underpriced"

**Solution:**
- Increase gas price in hardhat.config.js
- Or wait for network congestion to clear
- Check current gas: https://etherscan.io/gastracker

### "Pool creation failed"

**Solution:**
- Make sure tokens are approved first
- Check you have enough tokens + native currency
- Try using DEX web interface instead of scripts
- Increase slippage tolerance slightly

### "Can't connect to RPC"

**Solution:**
- Check your internet connection
- Try alternative RPC endpoint in .env
- RPC endpoints are provided in .env.example files

---

## 📞 Support

**Need Help?**
- **Discord:** #dex-deployment channel
- **Email:** dev@etrid.org
- **Emergency:** Contact Foundation Directors (24/7)

**Documentation:**
- Main Guide: [COMPLETE_DEX_DEPLOYMENT_GUIDE.md](COMPLETE_DEX_DEPLOYMENT_GUIDE.md)
- Deployment Summary: [DEX_DEPLOYMENT_READY.md](DEX_DEPLOYMENT_READY.md)
- BSC Guide: [bsc/README.md](bsc/README.md)
- Ethereum Guide: [ethereum/README.md](ethereum/README.md)
- Polygon Guide: [polygon/README.md](polygon/README.md)
- Solana Guide: [solana/README.md](solana/README.md)

---

## ✅ Success Metrics

**Week 1 Targets:**
- ✅ 4 DEXes live
- ✅ $10M total liquidity
- ✅ $500k daily trading volume
- ✅ 5,000+ unique traders
- ✅ CoinGecko + CMC submissions sent

**Month 1 Targets:**
- ✅ 10 DEXes live
- ✅ $20M total liquidity
- ✅ $2M daily trading volume
- ✅ Top 500 on CoinGecko

---

**🎉 You're Ready to Deploy ÉTR to DEXes! 🎉**

Good luck, Eoj! When mainnet goes live and VMs are operational, just follow this guide and you'll have ÉTR trading on 4 major DEXes within 2-3 hours!
