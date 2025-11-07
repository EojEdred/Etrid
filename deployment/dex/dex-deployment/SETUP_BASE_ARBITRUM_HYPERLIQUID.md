# 🚀 Quick Setup: Base + Arbitrum + Hyperliquid

These 3 chains are not yet configured. Here's how to set them up quickly.

---

## 1️⃣ SETUP BASE (~30 minutes)

Base is an Ethereum L2 by Coinbase. Very cheap gas, growing ecosystem.

### Step 1: Create Base Folder

```bash
cd ~/Desktop/etrid/dex-deployment
mkdir base
cp -r polygon/* base/
cd base
```

### Step 2: Update hardhat.config.js

```bash
nano hardhat.config.js
```

Change the networks section to:

```javascript
module.exports = {
  solidity: "0.8.20",
  networks: {
    mainnet: {
      url: "https://mainnet.base.org",
      accounts: [process.env.PRIVATE_KEY],
      chainId: 8453
    }
  },
  etherscan: {
    apiKey: {
      base: process.env.BASESCAN_API_KEY || ""
    },
    customChains: [
      {
        network: "base",
        chainId: 8453,
        urls: {
          apiURL: "https://api.basescan.org/api",
          browserURL: "https://basescan.org"
        }
      }
    ]
  }
};
```

### Step 3: Update Contract (Use Same as Polygon)

The contract is already correct (ERC-20 standard). No changes needed.

### Step 4: Setup .env

```bash
cp .env.example .env
nano .env
```

Add:
```
PRIVATE_KEY=0xYOUR_PRIVATE_KEY_HERE
BASESCAN_API_KEY=YOUR_BASESCAN_API_KEY
```

Get Basescan API key:
- https://basescan.org/myapikey
- Sign up (free)
- Generate API key

### Step 5: Deploy

```bash
npm install
npm run deploy:mainnet
```

**Cost:** ~$1

**DEXes Available:**
- Aerodrome (largest on Base)
- Uniswap V3
- BaseSwap

**BullX:** ✅ Supported

---

## 2️⃣ SETUP ARBITRUM (~30 minutes)

Arbitrum is an Ethereum L2. Very cheap, high throughput.

### Step 1: Create Arbitrum Folder

```bash
cd ~/Desktop/etrid/dex-deployment
mkdir arbitrum
cp -r polygon/* arbitrum/
cd arbitrum
```

### Step 2: Update hardhat.config.js

```bash
nano hardhat.config.js
```

Change to:

```javascript
module.exports = {
  solidity: "0.8.20",
  networks: {
    mainnet: {
      url: "https://arb1.arbitrum.io/rpc",
      accounts: [process.env.PRIVATE_KEY],
      chainId: 42161
    }
  },
  etherscan: {
    apiKey: {
      arbitrumOne: process.env.ARBISCAN_API_KEY || ""
    }
  }
};
```

### Step 3: Setup .env

```bash
cp .env.example .env
nano .env
```

Add:
```
PRIVATE_KEY=0xYOUR_PRIVATE_KEY_HERE
ARBISCAN_API_KEY=YOUR_ARBISCAN_API_KEY
```

Get Arbiscan API key:
- https://arbiscan.io/myapikey

### Step 4: Deploy

```bash
npm install
npm run deploy:mainnet
```

**Cost:** ~$1

**DEXes Available:**
- Camelot (largest native)
- Uniswap V3
- GMX V2
- SushiSwap

**BullX:** ✅ Supported

---

## 3️⃣ SETUP HYPERLIQUID (~2-3 hours + approval time)

**⚠️ Warning:** Hyperliquid is more complex. May require team approval.

### Option A: Deploy to HyperEVM (Permissionless)

HyperEVM is EVM-compatible, so you can deploy ERC-20 tokens.

#### Step 1: Research HyperEVM RPC

Visit: https://hyperliquid.gitbook.io/hyperliquid-docs/

Find:
- HyperEVM RPC endpoint
- Chain ID
- Block explorer

**As of early 2025, HyperEVM details:**
- RPC: Check official docs (may change)
- Chain ID: Check official docs
- Explorer: Check official docs

#### Step 2: Create Hyperliquid Folder

```bash
cd ~/Desktop/etrid/dex-deployment
mkdir hyperliquid
cp -r polygon/* hyperliquid/
cd hyperliquid
```

#### Step 3: Update hardhat.config.js

```bash
nano hardhat.config.js
```

```javascript
module.exports = {
  solidity: "0.8.20",
  networks: {
    mainnet: {
      url: "https://[HYPEREVM_RPC_ENDPOINT]",  // Get from docs
      accounts: [process.env.PRIVATE_KEY],
      chainId: [CHAIN_ID]  // Get from docs
    }
  }
};
```

#### Step 4: Deploy Token

```bash
npm install
npm run deploy:mainnet
```

**Cost:** ~$3-5 (estimated)

---

### Option B: Request Hyperliquid Perpetual Listing

For **perpetual futures** (not just spot trading), you may need approval.

#### Step 1: Join Hyperliquid Discord

https://discord.gg/hyperliquid

#### Step 2: Contact Team

Navigate to:
- #token-listings channel (if exists)
- Or DM team members

Ask:
```
Hi! I'm launching ÉTR (Etrid Coin) and want to list on Hyperliquid.

Token details:
- Name: Etrid Coin
- Symbol: ÉTR
- Total Supply: 1 billion
- Blockchain: FlareChain (Substrate) + wrapped on EVM chains
- Use case: [Brief description]
- Website: https://etrid.org/

Questions:
1. Can I deploy ERC-20 to HyperEVM permissionlessly?
2. Do you require approval for perpetual markets?
3. Are there any listing fees?
4. What's the typical approval timeline?

Thank you!
```

#### Step 3: Prepare Materials

They may ask for:
- Whitepaper
- Tokenomics
- Team information
- Audit reports (if any)
- Community size
- Liquidity on other DEXes

#### Step 4: Wait for Response

Timeline: Varies (1-2 weeks typical)

#### Step 5: Deploy + Create Market

Once approved:
1. Deploy token to HyperEVM
2. Submit market creation request
3. They create perpetual market
4. Market goes live!

---

## 🎯 RPC ENDPOINTS SUMMARY

### Base (Ready to Use)
```
RPC: https://mainnet.base.org
Chain ID: 8453
Explorer: https://basescan.org
Gas Token: ETH
```

### Arbitrum (Ready to Use)
```
RPC: https://arb1.arbitrum.io/rpc
Alternative: https://arbitrum-one.publicnode.com
Chain ID: 42161
Explorer: https://arbiscan.io
Gas Token: ETH
```

### Hyperliquid (Research Needed)
```
RPC: TBD (check docs)
Chain ID: TBD (check docs)
Explorer: TBD (check docs)
Gas Token: HYPE or HLP (check docs)

Official Docs: https://hyperliquid.gitbook.io/
Discord: https://discord.gg/hyperliquid
```

---

## 💰 COST BREAKDOWN

| Chain | Setup Time | Deploy Cost | Gas Token Needed |
|-------|-----------|-------------|------------------|
| Base | 30 mins | ~$1 | 0.001 ETH (~$3) |
| Arbitrum | 30 mins | ~$1 | 0.001 ETH (~$3) |
| Hyperliquid | 2-3 hours | ~$3-5 | TBD |

**Total:** ~$5-7 deployment + ~$9 for gas tokens = **~$14-16 total**

---

## 🔧 TROUBLESHOOTING

### Issue: "Insufficient funds for gas"

**Solution:**
```bash
# Check your wallet balance
# Base/Arbitrum: Need ETH for gas
# Hyperliquid: Need HYPE or native token

# Get ETH from:
# - Coinbase, Binance, etc.
# - Bridge from Ethereum mainnet

# For Base:
# Bridge ETH: https://bridge.base.org/

# For Arbitrum:
# Bridge ETH: https://bridge.arbitrum.io/
```

### Issue: "Network request failed"

**Solution:**
```bash
# Try alternative RPC endpoints

# Base Alternative:
url: "https://base.publicnode.com"

# Arbitrum Alternative:
url: "https://arbitrum-one.publicnode.com"
```

### Issue: "Contract verification failed"

**Solution:**
```bash
# Make sure you have API key in .env
# Verify manually:

# Base:
npx hardhat verify --network mainnet CONTRACT_ADDRESS

# Arbitrum:
npx hardhat verify --network mainnet CONTRACT_ADDRESS
```

---

## ✅ QUICK DEPLOY ALL THREE

Once configured, deploy all at once:

```bash
cd ~/Desktop/etrid/dex-deployment

# Base
cd base && npm run deploy:mainnet && cd ..

# Arbitrum
cd arbitrum && npm run deploy:mainnet && cd ..

# Hyperliquid (if configured)
cd hyperliquid && npm run deploy:mainnet && cd ..

echo "✅ All deployed!"
```

---

## 📊 AFTER DEPLOYMENT

### Base

**Explorer:** https://basescan.org/address/YOUR_CONTRACT

**Create Pool:**
- Aerodrome: https://aerodrome.finance/liquidity
- Uniswap V3: https://app.uniswap.org/pools

**BullX:** Will auto-detect after pool creation ✅

---

### Arbitrum

**Explorer:** https://arbiscan.io/address/YOUR_CONTRACT

**Create Pool:**
- Camelot: https://app.camelot.exchange/liquidity
- Uniswap V3: https://app.uniswap.org/pools

**BullX:** Will auto-detect after pool creation ✅

---

### Hyperliquid

**Explorer:** [Check official docs for explorer URL]

**Create Market:**
- May need team approval
- Submit request via Discord
- Wait for market creation
- Start trading!

**BullX:** ❌ Not supported (different trading model)

---

## 🎯 RECOMMENDED ORDER

1. **Deploy Base** (30 mins, $1)
   - Easiest
   - Well-documented
   - BullX supported

2. **Deploy Arbitrum** (30 mins, $1)
   - Same process as Base
   - BullX supported

3. **Deploy Hyperliquid** (2-3 hours, $3-5)
   - More complex
   - May need approval
   - Different trading model

---

## 📞 NEED HELP?

### Base Support
- Docs: https://docs.base.org/
- Discord: https://discord.gg/buildonbase

### Arbitrum Support
- Docs: https://docs.arbitrum.io/
- Discord: https://discord.gg/arbitrum

### Hyperliquid Support
- Docs: https://hyperliquid.gitbook.io/
- Discord: https://discord.gg/hyperliquid

---

## 🚀 NEXT STEPS AFTER SETUP

Once all 5 chains are deployed:

```
You'll have ÉTR on:
├─ Solana (Raydium, Orca) ✅ BullX
├─ BSC (PancakeSwap) ✅ BullX
├─ Base (Aerodrome) ✅ BullX
├─ Arbitrum (Camelot) ✅ BullX
└─ Hyperliquid (Perps) ⭐ Advanced

Total Cost: ~$15.50-17.50
Total DEXes: 15+
BullX Compatible: 4 chains

Next:
1. Lock equivalent on FlareChain (1:1 backing)
2. Accumulate liquidity funds ($5k-10k)
3. Create pools on DEXes
4. BullX auto-detects within 1-2 hours!
5. Launch! 🎉
```

Good luck! 🚀
