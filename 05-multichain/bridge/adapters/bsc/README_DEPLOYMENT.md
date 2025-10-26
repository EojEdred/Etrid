# ÉTR BSC Deployment Guide

**Quick Start Guide for Deploying ÉTR Token on Binance Smart Chain**

Last Updated: October 24, 2025

---

## 📋 Prerequisites

- Node.js 18+ installed
- npm or yarn installed
- ~0.5 BNB for testnet (free from faucet) OR ~$20 worth of BNB for mainnet

---

## 🚀 Quick Start (5 Steps)

### Step 1: Install Dependencies

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc

# Install all required packages
npm install
```

**Expected output:**
```
added 487 packages in 45s
```

---

### Step 2: Generate Wallet

```bash
# Generate new wallet for deployment
npm run generate-wallet
```

**Expected output:**
```
🔐 Generating new Ethereum wallet...

✅ New Wallet Generated!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Address:     0x1234...abcd
Private Key: 0xabcdef...123456
Mnemonic:    word1 word2 word3 ... word12
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️  SECURITY WARNINGS:
1. NEVER share your private key or mnemonic with anyone!
2. NEVER commit .env file to version control!
3. Back up your mnemonic in a secure location
```

**Action:**
1. Copy the `Private Key` (starts with `0x`)
2. Save the `Mnemonic` somewhere safe (write it down on paper!)

---

### Step 3: Configure Environment

```bash
# Copy example env file
cp .env.example .env

# Edit .env file
nano .env  # or use your favorite editor
```

**Paste your private key:**
```bash
DEPLOYER_PRIVATE_KEY=0xYOUR_PRIVATE_KEY_HERE

# Optional: Add BscScan API key for contract verification
# Get free key at: https://bscscan.com/register
BSCSCAN_API_KEY=YOUR_API_KEY_HERE
```

**Save and exit** (Ctrl+X, then Y, then Enter in nano)

---

### Step 4: Get Testnet BNB

**Option A: Official BSC Faucet (Recommended)**

1. Visit: https://testnet.bnbchain.org/faucet-smart
2. Paste your wallet address (from Step 2)
3. Complete CAPTCHA
4. Click "Give me BNB"
5. Wait 1-2 minutes for BNB to arrive

**Option B: Alternative Faucets**

- https://www.bnbchain.org/en/testnet-faucet
- https://testnet.help/en/bnbfaucet/testnet (requires Twitter)

**Verify you received BNB:**
```bash
npm run check-balance
```

**Expected output:**
```
🔍 Checking balance on BSC Testnet (Chain ID: 97)

Wallet 1:
  Address: 0x1234...abcd
  Balance: 0.5 BNB
  Gas Price: 10 gwei
  Est. Deployment Cost: ~0.02 BNB
  ✅ Sufficient balance for deployment
```

---

### Step 5: Deploy to Testnet

```bash
# Deploy ÉTR token contract
npm run deploy:testnet
```

**Expected output:**
```
🚀 Deploying ÉTR Token to BSC Testnet...

📍 Deployment Details:
  Deployer: 0x1234...abcd
  Balance:  0.5 BNB
  Network:  BSC Testnet (Chain ID: 97)

📝 Token Configuration:
  Name:     Etrid Coin (BSC Testnet)
  Symbol:   ÉTR
  Decimals: 18
  Supply:   0 (minted via bridge only)

⏳ Deploying contract...
  Transaction submitted, waiting for confirmation...
  ✅ Contract deployed!

🎉 DEPLOYMENT SUCCESSFUL!

📋 Contract Details:
  Address: 0xABCD...1234
  Explorer: https://testnet.bscscan.com/address/0xABCD...1234

📝 Next Steps:
1. Save contract address to .env
2. Verify contract on BscScan
3. Add token to MetaMask
4. Test minting
```

**Action:**
1. **Copy the contract address** (0xABCD...1234)
2. **Save it to .env:**
   ```bash
   echo "ETR_TOKEN_ADDRESS_TESTNET=0xABCD...1234" >> .env
   ```

---

## ✅ Verification (Optional but Recommended)

### Verify Contract on BscScan Testnet

```bash
# Replace with your actual contract address
npx hardhat verify --network bscTestnet 0xABCD...1234 \
  "Etrid Coin (BSC Testnet)" \
  "ÉTR"
```

**Expected output:**
```
Successfully submitted source code for contract
contracts/EtridToken.sol:EtridToken at 0xABCD...1234
for verification on the block explorer. Waiting for verification result...

Successfully verified contract EtridToken on BscScan.
https://testnet.bscscan.com/address/0xABCD...1234#code
```

**Why verify?**
- Anyone can read your contract code (transparency)
- Users can interact with contract directly on BscScan
- Required for most DEX integrations

---

## 🧪 Testing Your Deployment

### 1. Add Token to MetaMask

1. Open MetaMask
2. Switch network to "BSC Testnet" (add custom network if needed)
3. Click "Import tokens"
4. Paste contract address: `0xABCD...1234`
5. Symbol should auto-fill as "ÉTR"
6. Click "Add"

### 2. Test Minting (Bridge Simulation)

Visit BscScan → Your Contract → Write Contract:

1. Connect MetaMask wallet
2. Find `bridgeMint` function
3. Enter:
   - `to`: Your wallet address
   - `amount`: 1000000000000000000 (= 1 ÉTR in wei)
   - `txHash`: 0x1234... (fake hash for testing)
4. Click "Write" and confirm transaction

### 3. Verify Balance

Check MetaMask - you should see **1 ÉTR** in your wallet!

### 4. Test Transfer

1. In MetaMask, send 0.5 ÉTR to another address
2. Verify transaction on BscScan

### 5. Test Burning (Bridge Back Simulation)

1. BscScan → Your Contract → Write Contract
2. Find `bridgeBurn` function
3. Enter:
   - `amount`: 500000000000000000 (= 0.5 ÉTR)
   - `etridAddress`: "etrid1abc..." (your Ëtrid address)
4. Click "Write" and confirm

---

## 🎯 Next Steps After Testnet Success

### Ready for Mainnet?

**Checklist before mainnet:**
- [ ] Tested mint function (at least 3 times)
- [ ] Tested transfer function
- [ ] Tested burn function
- [ ] Verified contract on BscScan testnet
- [ ] Reviewed contract code for errors
- [ ] Multi-sig wallet ready for admin control
- [ ] Have **real BNB** (~$20 worth for gas)

### Deploy to Mainnet

**⚠️ WARNING: This costs real money!**

```bash
# Make ABSOLUTELY SURE you're ready
npm run deploy:mainnet
```

**Differences from testnet:**
- Uses real BNB (costs $5-20 in gas)
- Irreversible (can't undo!)
- Contract address will be different
- Use production RPC URLs

---

## 📁 Project Structure

```
bsc/
├── contracts/
│   └── EtridToken.sol          # ÉTR token contract
├── scripts/
│   ├── generate-wallet.ts      # Create new wallet
│   ├── check-balance.ts        # Check BNB balance
│   ├── deploy-etr-testnet.ts   # Deploy to testnet
│   └── deploy-etr-mainnet.ts   # Deploy to mainnet (TBD)
├── test/
│   └── EtridToken.test.ts      # Unit tests (TBD)
├── .env                        # Your secrets (NEVER commit!)
├── .env.example                # Template
├── hardhat.config.ts           # Network configuration
├── package.json                # Dependencies
└── README_DEPLOYMENT.md        # This file
```

---

## 🐛 Troubleshooting

### Error: "insufficient funds for gas"

**Solution:**
```bash
# Check balance
npm run check-balance

# Get more testnet BNB from faucet
# Visit: https://testnet.bnbchain.org/faucet-smart
```

### Error: "cannot find module 'hardhat'"

**Solution:**
```bash
# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

### Error: "invalid sender" or "no private key"

**Solution:**
```bash
# Make sure .env file exists and has DEPLOYER_PRIVATE_KEY
cat .env | grep DEPLOYER_PRIVATE_KEY

# If missing, regenerate wallet
npm run generate-wallet
# Then add private key to .env
```

### Error: "nonce too low"

**Solution:**
- Reset nonce in MetaMask: Settings → Advanced → Reset Account
- Or wait a few minutes and try again

### Contract deployed but can't verify

**Solution:**
```bash
# Make sure you have BSCSCAN_API_KEY in .env
# Get free key: https://bscscan.com/register

# Try verifying again with exact same parameters
npx hardhat verify --network bscTestnet <ADDRESS> \
  "Etrid Coin (BSC Testnet)" \
  "ÉTR"
```

---

## 🔐 Security Best Practices

### ❌ DON'T:
- Commit .env file to Git
- Share private key with anyone
- Use testnet wallet for mainnet
- Deploy without testing first
- Give MINTER_ROLE to untrusted addresses

### ✅ DO:
- Back up mnemonic on paper (offline)
- Use hardware wallet for mainnet
- Transfer admin roles to multi-sig wallet after deployment
- Test thoroughly on testnet first
- Verify contracts on BscScan

---

## 📞 Need Help?

**Stuck? Have questions?**

1. **Check docs**: `/docs/TESTING_ENVIRONMENT_SETUP.md`
2. **Discord**: #dev-support channel
3. **GitHub Issues**: github.com/etrid-protocol/etrid/issues
4. **Email**: eoj@etrid.io

---

## 📚 Additional Resources

- **BSC Documentation**: https://docs.bnbchain.org/
- **Hardhat Docs**: https://hardhat.org/docs
- **OpenZeppelin**: https://docs.openzeppelin.com/contracts/
- **BscScan**: https://testnet.bscscan.com/

---

**Last Updated**: October 24, 2025
**Maintainer**: Ëtrid Protocol Team
**Status**: Ready for Testnet Deployment
