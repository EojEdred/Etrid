# 🔧 .ENV SETUP COMPLETE GUIDE

**Status:** ✅ .env files created, npm dependencies installed
**What's Left:** Add your PRIVATE_KEY to each .env file

---

## ✅ WHAT I JUST DID FOR YOU

### 1. Created .env Files:
- ✅ `base/.env` (copied from .env.example)
- ✅ `arbitrum/.env` (copied from .env.example)
- ✅ `hyperliquid/.env` (copied from .env.example)

### 2. Installed npm Dependencies:
- ✅ `base/node_modules/` (Hardhat, OpenZeppelin, etc.)
- ✅ `arbitrum/node_modules/` (Hardhat, OpenZeppelin, etc.)
- ✅ `hyperliquid/node_modules/` (Hardhat, OpenZeppelin, etc.)

### 3. What You Need to Do:
- ⚠️ Add your PRIVATE_KEY to each .env file
- ⚠️ Get API keys for block explorers (optional but recommended)
- ⚠️ Get gas tokens for each chain

---

## 🔑 STEP 1: Export Your Private Key

**⚠️ CRITICAL: Keep your private key SECRET! Never share it or commit to git!**

### Option A: Using MetaMask (for Base & Arbitrum)

1. **Open MetaMask**
2. **Click** the account icon (top right)
3. **Click** "Account details"
4. **Click** "Show private key"
5. **Enter** your MetaMask password
6. **Copy** the private key (starts with `0x`)

**Example:** `0xabc123def456789...` (64 characters after 0x)

### Option B: Using Existing Key

If you already have a private key from BSC or Polygon deployment, you can use the SAME key for Base and Arbitrum (they're all EVM chains).

---

## 🔧 STEP 2: Add Private Key to .env Files

### Base:

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/base
nano .env
```

Find this line:
```
PRIVATE_KEY=your_private_key_here
```

Change to:
```
PRIVATE_KEY=0xYOUR_ACTUAL_PRIVATE_KEY_HERE
```

**Save:** Ctrl+O, Enter, Ctrl+X

---

### Arbitrum:

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/arbitrum
nano .env
```

Find this line:
```
PRIVATE_KEY=your_private_key_here
```

Change to:
```
PRIVATE_KEY=0xYOUR_ACTUAL_PRIVATE_KEY_HERE
```

**Save:** Ctrl+O, Enter, Ctrl+X

**Note:** You can use the SAME private key as Base!

---

### Hyperliquid:

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/hyperliquid
nano .env
```

Find this line:
```
PRIVATE_KEY=your_private_key_here
```

Change to:
```
PRIVATE_KEY=0xYOUR_ACTUAL_PRIVATE_KEY_HERE
```

**Save:** Ctrl+O, Enter, Ctrl+X

**Note:** You can use the SAME private key as Base/Arbitrum!

---

## 🎫 STEP 3: Get API Keys (Optional but Recommended)

API keys are for contract verification on block explorers. Not required for deployment, but highly recommended.

### Base - Get BaseScan API Key:

1. **Go to:** https://basescan.org/myapikey
2. **Sign up** for free account
3. **Create** new API key
4. **Copy** the API key

**Add to base/.env:**
```bash
nano base/.env
```

Find:
```
BASESCAN_API_KEY=your_basescan_api_key_here
```

Change to:
```
BASESCAN_API_KEY=YOUR_ACTUAL_API_KEY
```

---

### Arbitrum - Get Arbiscan API Key:

1. **Go to:** https://arbiscan.io/myapikey
2. **Sign up** for free account
3. **Create** new API key
4. **Copy** the API key

**Add to arbitrum/.env:**
```bash
nano arbitrum/.env
```

Find:
```
ARBISCAN_API_KEY=your_arbiscan_api_key_here
```

Change to:
```
ARBISCAN_API_KEY=YOUR_ACTUAL_API_KEY
```

---

### Hyperliquid:

Hyperliquid doesn't have a standard block explorer API yet. Leave as is or check latest docs:
- https://hyperliquid.gitbook.io/

---

## 💰 STEP 4: Get Gas Tokens

You need native tokens on each chain to pay for gas fees.

### Base - Get ETH on Base:

**Amount needed:** ~0.001 ETH (~$3)

**Option 1: Bridge from Ethereum**
```
1. Go to: https://bridge.base.org/
2. Connect MetaMask
3. Bridge 0.002 ETH from Ethereum → Base
4. Wait 5-10 minutes
5. Check Base balance in MetaMask
```

**Option 2: Buy on Exchange**
```
1. Buy ETH on Coinbase
2. Withdraw to Base network
3. Use your MetaMask address
4. Select "Base" as network
```

**Check balance:**
```bash
# Add Base network to MetaMask if not already:
Network Name: Base
RPC URL: https://mainnet.base.org
Chain ID: 8453
Currency Symbol: ETH
Block Explorer: https://basescan.org
```

---

### Arbitrum - Get ETH on Arbitrum:

**Amount needed:** ~0.001 ETH (~$3)

**Option 1: Bridge from Ethereum**
```
1. Go to: https://bridge.arbitrum.io/
2. Connect MetaMask
3. Bridge 0.002 ETH from Ethereum → Arbitrum
4. Wait 5-10 minutes
5. Check Arbitrum balance in MetaMask
```

**Option 2: Buy on Exchange**
```
1. Buy ETH on Coinbase/Binance
2. Withdraw to Arbitrum One network
3. Use your MetaMask address
4. Select "Arbitrum One" as network
```

**Check balance:**
```bash
# Add Arbitrum network to MetaMask:
Network Name: Arbitrum One
RPC URL: https://arb1.arbitrum.io/rpc
Chain ID: 42161
Currency Symbol: ETH
Block Explorer: https://arbiscan.io
```

---

### Hyperliquid - Get HYPE:

**Amount needed:** ~0.01-0.05 HYPE (~$5-25, depends on HYPE price)

**⚠️ This is more complex!**

**Option 1: Bridge from Ethereum**
```
Check Hyperliquid docs for official bridge:
https://hyperliquid.gitbook.io/hyperliquid-docs/

May require:
1. Connect to Hyperliquid platform
2. Bridge ETH → HYPE
3. Wait for confirmation
```

**Option 2: Contact Hyperliquid Team**
```
If you're planning to list ÉTR perpetuals:
1. Join Discord: https://discord.gg/hyperliquid
2. Ask in #support about getting HYPE for gas
3. Team may provide testnet HYPE for testing
```

**Check balance:**
```bash
# Add HyperEVM network to MetaMask:
Network Name: Hyperliquid
RPC URL: https://rpc.hyperliquid.xyz/evm
Chain ID: 999
Currency Symbol: HYPE
Block Explorer: https://explorer.hyperliquid.xyz
```

---

## ✅ VERIFICATION CHECKLIST

Before deploying, verify you have:

### Base:
- [ ] Private key added to `base/.env`
- [ ] BaseScan API key added (optional)
- [ ] 0.001+ ETH on Base network
- [ ] Base network added to MetaMask
- [ ] npm dependencies installed ✅ (already done)

### Arbitrum:
- [ ] Private key added to `arbitrum/.env`
- [ ] Arbiscan API key added (optional)
- [ ] 0.001+ ETH on Arbitrum network
- [ ] Arbitrum network added to MetaMask
- [ ] npm dependencies installed ✅ (already done)

### Hyperliquid:
- [ ] Private key added to `hyperliquid/.env`
- [ ] 0.01+ HYPE on HyperEVM
- [ ] HyperEVM network added to MetaMask
- [ ] Discord account created for team contact
- [ ] npm dependencies installed ✅ (already done)

---

## 🚀 STEP 5: TEST DEPLOYMENT

Once you have everything set up, test on testnets first!

### Base Testnet (Sepolia):

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/base

# Get testnet ETH:
# Go to: https://www.alchemy.com/faucets/base-sepolia
# Or: https://faucet.quicknode.com/base/sepolia

# Deploy to testnet:
npm run deploy:testnet

# If successful, you'll see:
# ✅ Contract deployed to: 0x...
# ✅ Verify at: https://sepolia.basescan.org/address/0x...
```

---

### Arbitrum Testnet (Sepolia):

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/arbitrum

# Get testnet ETH:
# Go to: https://faucet.quicknode.com/arbitrum/sepolia

# Deploy to testnet:
npm run deploy:testnet

# If successful, you'll see:
# ✅ Contract deployed to: 0x...
# ✅ Verify at: https://sepolia.arbiscan.io/address/0x...
```

---

### Hyperliquid Testnet:

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/hyperliquid

# Get testnet HYPE:
# Contact Hyperliquid team in Discord
# Or check faucet: https://hyperliquid.gitbook.io/

# Deploy to testnet:
npm run deploy:testnet

# If successful, you'll see:
# ✅ Contract deployed to: 0x...
```

---

## 🎯 STEP 6: MAINNET DEPLOYMENT

**⚠️ WARNING: This spends REAL money!**

Only proceed if:
- ✅ Tested on testnet successfully
- ✅ Have real gas tokens
- ✅ Ready to spend $1-5 per chain
- ✅ Understand what you're doing

### Deploy Base (Mainnet):

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/base
npm run deploy:mainnet
```

**Cost:** ~$1
**DEXes:** Aerodrome, Uniswap V3
**BullX:** ✅ Will auto-detect after pool creation

---

### Deploy Arbitrum (Mainnet):

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/arbitrum
npm run deploy:mainnet
```

**Cost:** ~$1
**DEXes:** Camelot, Uniswap V3, GMX
**BullX:** ✅ Will auto-detect after pool creation

---

### Deploy Hyperliquid (Mainnet):

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/hyperliquid
npm run deploy:mainnet
```

**Cost:** ~$3-5
**DEXes:** Hyperliquid Perpetuals
**BullX:** ❌ Not supported (different trading model)
**Special:** Need team approval for perp markets

---

## 📊 DEPLOYMENT STATUS

### After You Complete This Guide:

| Chain | .env | npm | Gas | Status |
|-------|------|-----|-----|--------|
| Base | ✅ Created | ✅ Installed | ⚠️ Need | Ready to add keys |
| Arbitrum | ✅ Created | ✅ Installed | ⚠️ Need | Ready to add keys |
| Hyperliquid | ✅ Created | ✅ Installed | ⚠️ Need | Ready to add keys |

---

## 🔒 SECURITY REMINDERS

1. **NEVER commit .env files to git**
   ```bash
   # Already in .gitignore, but double-check:
   cat /Users/macbook/Desktop/etrid/dex-deployment/.gitignore | grep .env
   ```

2. **NEVER share your private key**
   - Not on Discord
   - Not on Telegram
   - Not in screenshots
   - Not anywhere!

3. **Use a dedicated wallet for deployments**
   - Don't use your main wallet
   - Create a new MetaMask account for deployments
   - Only fund with what you need

4. **Backup your private key securely**
   - Write it down on paper
   - Store in password manager
   - Keep multiple copies in safe places

---

## 🆘 TROUBLESHOOTING

### "insufficient funds for gas"

**Solution:** Get more ETH/HYPE on that chain. See Step 4.

### "invalid private key"

**Solution:**
- Make sure it starts with `0x`
- 66 characters total (0x + 64 hex chars)
- No spaces or line breaks

### "network request failed"

**Solution:**
- Check internet connection
- Try alternative RPC in hardhat.config.js
- Wait and retry (network might be congested)

### "contract verification failed"

**Solution:**
- Make sure API key is valid
- Wait 30 seconds after deployment
- Try manual verification:
  ```bash
  npx hardhat verify --network mainnet CONTRACT_ADDRESS
  ```

---

## ✅ QUICK SUMMARY

**What's Done:**
- ✅ .env files created (base, arbitrum, hyperliquid)
- ✅ npm dependencies installed
- ✅ Hardhat configs ready
- ✅ Contracts ready

**What You Need:**
1. Add PRIVATE_KEY to each .env (Step 2)
2. Get API keys (Step 3, optional)
3. Get gas tokens (Step 4)
4. Test on testnet (Step 5)
5. Deploy to mainnet (Step 6)

**Total Time:** 1-2 hours
**Total Cost:** $5-10 (gas tokens + deployment)

---

## 🚀 AFTER DEPLOYMENT

Once all 3 chains are deployed:

1. **Save contract addresses**
2. **Lock equivalent on FlareChain** (1:1 backing)
3. **Accumulate liquidity** ($5k-10k recommended)
4. **Create pools on DEXes**
5. **BullX auto-detects** (1-2 hours)
6. **Launch!** 🎉

---

**Need help?** Read the other guides in this folder:
- `FINAL_ANSWERS_YOUR_QUESTIONS.md`
- `CONFIGURATION_COMPLETE_SUMMARY.md`
- `hyperliquid/HYPERLIQUID_DEPLOYMENT_NOTES.md`

**Ready to deploy?** Follow Steps 1-6 above!

Good luck! 🚀
