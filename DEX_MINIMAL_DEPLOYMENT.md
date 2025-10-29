# Minimal DEX Deployment - $10-20 Budget

**Goal:** Deploy ÉTR token contracts with MINIMAL gas fees, add liquidity LATER
**Budget:** $10-20 for gas fees only
**Strategy:** Deploy contracts → Verify → Add liquidity when funds available

---

## 🎯 What We're Actually Doing

**NOT doing:** Adding $2.5M liquidity pools (that comes later)
**YES doing:**
1. Deploy token contracts on BSC + Solana
2. Verify contracts on explorers
3. Make tokens ready for when you have liquidity

**Total Cost:**
- BSC deployment: ~0.02 BNB (~$6)
- BSC verification: FREE
- Solana token creation: ~0.01 SOL (~$1.50)
- Solana metadata: ~0.01 SOL (~$1.50)
- **TOTAL: ~$10**

---

## Part 1: BSC Token Deployment (No Liquidity)

### Step 1: Get Small Amount of BNB

**You need:** 0.05 BNB (~$15)

**How to get it:**
1. Buy BNB on Binance, Coinbase, or Kraken
2. Withdraw to your MetaMask wallet (BSC network)
3. Or use a DEX swap: USDT → BNB on PancakeSwap

### Step 2: Quick Setup

```bash
# Navigate to your project
cd /Users/macbook/Desktop/etrid

# Create contracts directory
mkdir -p contracts/ethereum
cd contracts/ethereum

# Initialize project
npm init -y

# Install ONLY what we need (saves time)
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npm install @openzeppelin/contracts dotenv

# Initialize Hardhat
npx hardhat init
# Choose: "Create a JavaScript project" → press Enter for all prompts
```

### Step 3: Create Token Contract (Already Written for You)

Create `contracts/EtridToken.sol`:

```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract EtridToken is ERC20, ERC20Burnable, Ownable {
    constructor() ERC20("Etrid Coin", "ETR") Ownable(msg.sender) {
        // No initial mint - mint later when ready
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
```

### Step 4: Configure Hardhat

Edit `hardhat.config.js`:

```javascript
require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

module.exports = {
  solidity: "0.8.20",
  networks: {
    bsc: {
      url: "https://bsc-dataseed.binance.org",
      chainId: 56,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : []
    }
  },
  etherscan: {
    apiKey: {
      bsc: process.env.BSCSCAN_API_KEY || ""
    }
  }
};
```

### Step 5: Create .env File

```bash
# Create .env
touch .env
```

Edit `.env` (use your actual keys):
```
PRIVATE_KEY=your_metamask_private_key_here
BSCSCAN_API_KEY=get_free_from_bscscan.com
```

**⚠️ IMPORTANT:**
```bash
# Add to .gitignore
echo ".env" >> .gitignore
```

### Step 6: Create Simple Deploy Script

Create `scripts/deploy.js`:

```javascript
async function main() {
  console.log("Deploying ÉTR token to BSC...");

  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);

  const balance = await ethers.provider.getBalance(deployer.address);
  console.log("Balance:", ethers.formatEther(balance), "BNB");

  // Deploy token
  const EtridToken = await ethers.getContractFactory("EtridToken");
  const token = await EtridToken.deploy();
  await token.waitForDeployment();

  const address = await token.getAddress();
  console.log("\n✅ ÉTR Token deployed to:", address);
  console.log("\n📋 Add to MetaMask:");
  console.log("   Address:", address);
  console.log("   Symbol: ÉTR");
  console.log("   Decimals: 18");

  console.log("\n📝 Verify with:");
  console.log("   npx hardhat verify --network bsc", address);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
```

### Step 7: Deploy (The Actual Deployment!)

```bash
# Compile contract
npx hardhat compile

# Deploy to BSC mainnet
npx hardhat run scripts/deploy.js --network bsc
```

**Expected cost:** ~0.02 BNB (~$6)

### Step 8: Verify Contract (FREE!)

```bash
# After deployment completes, verify
npx hardhat verify --network bsc <TOKEN_ADDRESS_FROM_STEP_7>
```

**Done! Contract is deployed and verified on BSCScan.**

---

## Part 2: Solana Token (Even Cheaper!)

### Step 1: Install Solana CLI

```bash
# Install Solana
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add to PATH
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Check version
solana --version
```

### Step 2: Create Wallet

```bash
# Create new wallet
solana-keygen new --outfile ~/etrid-sol.json

# Set as default
solana config set --keypair ~/etrid-sol.json

# Connect to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Check your address
solana address
```

**You need:** 0.03 SOL (~$4.50)

Buy small amount of SOL and send to address from `solana address`

### Step 3: Create Token (Super Simple!)

```bash
# Install SPL token CLI
cargo install spl-token-cli

# Create token
spl-token create-token --decimals 9

# OUTPUT: "Creating token ABC123..."
# Save this ABC123 address!
```

**Cost:** ~0.01 SOL (~$1.50)

### Step 4: Create Token Account

```bash
# Replace ABC123 with your token address from step 3
spl-token create-account ABC123
```

**Cost:** ~0.002 SOL (~$0.30)

**Done! You have a Solana token.**

---

## Part 3: What You Have Now

After spending ~$10:

✅ **BSC:**
- ÉTR token contract deployed
- Contract verified on BSCScan
- Ready to mint tokens when needed
- Ready for liquidity when you have funds

✅ **Solana:**
- ÉTR SPL token created
- Token account created
- Ready to mint when needed

**What's NOT done (and that's OK!):**
- ❌ No liquidity pools (that requires $50k-2.5M)
- ❌ No initial token minting (can do anytime)
- ❌ Not tradeable yet (needs liquidity)

---

## Part 4: Next Steps (When You Have Funds)

### When you have $50k-100k:

**Option 1: Bootstrap on BSC**
```bash
# Mint 1M ÉTR to yourself
# In Hardhat console:
npx hardhat console --network bsc

# Then:
const token = await ethers.getContractAt("EtridToken", "YOUR_TOKEN_ADDRESS");
await token.mint("YOUR_ADDRESS", ethers.parseEther("1000000")); // 1M ÉTR

# Buy 167 BNB (~$50k)
# Go to PancakeSwap and add liquidity
```

**Option 2: Start Even Smaller**
- Mint 100k ÉTR
- Pair with 16.7 BNB (~$5k liquidity)
- Small pool, but it's LIVE and tradeable

### When you have $500k+:

- Add more liquidity to existing pools
- Deploy to more chains (Ethereum, Avalanche, etc.)
- Apply to centralized exchanges

---

## Part 5: Free Listing Prep (Do This Now!)

While you're bootstrapping funds, do these FREE tasks:

### 1. Create Social Presence

- Twitter/X account: @etrid
- Discord server: discord.gg/etrid
- Reddit: r/etrid
- Telegram: t.me/etridofficial

### 2. Create Marketing Materials

**Token logo** (need this for CoinGecko/CMC):
- 200x200px PNG
- Transparent background
- Upload to https://etrid.org/logo.png

**Token metadata JSON:**
Create `https://etrid.org/token-metadata.json`:
```json
{
  "name": "Etrid Coin",
  "symbol": "ÉTR",
  "description": "Native token of Ëtrid blockchain",
  "image": "https://etrid.org/logo.png",
  "external_url": "https://etrid.org"
}
```

### 3. Prepare Applications (Don't Submit Yet)

**CoinGecko draft:**
- Project name: Ëtrid
- Ticker: ÉTR
- BSC contract: (your deployed address)
- Solana mint: (your Solana token address)
- Website: https://etrid.org
- Description: (write 2-3 paragraphs)

**CoinMarketCap draft:**
- Same info as CoinGecko
- Wait until you have liquidity pools to submit

---

## Part 6: Realistic Timeline

### Week 1 (NOW): Deploy Contracts
- ✅ Deploy BSC contract (~$6)
- ✅ Deploy Solana token (~$3)
- ✅ Create social accounts (free)
- **Cost: $10**

### Week 2-4: Bootstrap Funds
- Raise $5k-50k from team/investors/community
- Prepare marketing materials
- Build community

### Week 4-6: Launch Trading
- Mint tokens
- Add initial liquidity ($5k-50k)
- Submit to CoinGecko/CMC
- **Cost: $5k-50k + marketing**

### Month 3-6: Grow
- Add more liquidity as token appreciates
- Apply to more DEXes
- Prepare for CEX listings

---

## 🎯 Let's Start RIGHT NOW

**Your immediate action:**

1. **Check if you have Node.js:**
```bash
node --version
```

2. **Get 0.05 BNB (~$15):**
- Buy on Binance/Coinbase
- Send to your MetaMask (BSC network)

3. **Run the setup commands:**
```bash
cd /Users/macbook/Desktop/etrid
mkdir -p contracts/ethereum
cd contracts/ethereum
npm init -y
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npm install @openzeppelin/contracts dotenv
```

**Tell me when you've done step 1-3 and I'll guide you through the actual deployment!**

---

## 💡 Pro Tips

1. **Use BSC Testnet first** (completely free):
   - Test everything before mainnet
   - Change network in hardhat.config.js
   - Get free test BNB from faucet

2. **Start with Solana** (cheaper):
   - Total cost: ~$3
   - Faster deployment
   - Lower barrier

3. **No rush on liquidity:**
   - Having deployed contracts is 50% of the work
   - Can add liquidity anytime
   - Better to launch with $10k than rush with $1k

---

**Ready to deploy with just $10?** Tell me:
1. Do you have Node.js? (`node --version`)
2. Do you have 0.05 BNB in MetaMask?
3. Which do you want to deploy first: BSC or Solana?

Let's get ÉTR deployed TODAY! 🚀
