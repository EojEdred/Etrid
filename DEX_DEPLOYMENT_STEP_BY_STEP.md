# DEX Deployment - Step-by-Step Guide

**Date:** October 28, 2025
**Status:** 🚀 Let's Deploy!
**Goal:** Get ÉTR trading on PancakeSwap (BSC) and Raydium (Solana)

---

## 📋 Prerequisites Checklist

Before we start, you need:

### 1. Wallets & Keys

**BSC Deployment:**
- [ ] MetaMask wallet installed (or other Web3 wallet)
- [ ] Wallet has ~0.1 BNB for gas fees (~$30)
- [ ] Private key exported (for command-line deployment)

**Solana Deployment:**
- [ ] Phantom wallet installed (or Solflare)
- [ ] Wallet has ~5-10 SOL for rent + fees (~$750-1500)
- [ ] Solana CLI installed on your computer

### 2. API Keys

- [ ] BSCScan API key (free): https://bscscan.com/apis
- [ ] (Optional) Etherscan API key if deploying to Ethereum

### 3. Funds for Liquidity

**Initial liquidity needs:**
- 50,000,000 ÉTR (for BSC PancakeSwap)
- 50,000,000 ÉTR (for Solana Raydium)
- ~8,333 BNB (~$2.5M at $300/BNB) OR start smaller
- ~16,667 SOL (~$2.5M at $150/SOL) OR start smaller

**💡 Recommendation:** Start with **smaller test amounts** first:
- 1,000,000 ÉTR + 167 BNB (~$50k liquidity on BSC)
- 1,000,000 ÉTR + 333 SOL (~$50k liquidity on Solana)

### 4. Technical Setup

- [ ] Node.js installed (v18+): `node --version`
- [ ] npm or yarn installed
- [ ] Git installed
- [ ] Code editor (VS Code recommended)

---

## 🎯 Deployment Strategy

We'll deploy in this order:

1. **BSC Testnet** (practice run, free)
2. **BSC Mainnet** (real deployment)
3. **Solana Devnet** (practice run, free)
4. **Solana Mainnet** (real deployment)

---

## Part 1: BSC Deployment (PancakeSwap)

### Step 1.1: Set Up Project Environment

Open terminal and run:

```bash
# Navigate to project root
cd /Users/macbook/Desktop/etrid

# Check if contracts directory exists
ls -la contracts/

# If no contracts directory, create one
mkdir -p contracts/ethereum
cd contracts/ethereum

# Initialize npm project
npm init -y

# Install Hardhat (Ethereum development environment)
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox

# Install OpenZeppelin (secure contract templates)
npm install @openzeppelin/contracts

# Initialize Hardhat
npx hardhat init
# Choose: "Create a JavaScript project"
```

### Step 1.2: Create ÉTR Token Contract

Create file: `contracts/ethereum/contracts/EtridToken.sol`

```bash
# Create contract file
touch contracts/EtridToken.sol
```

Now open `contracts/EtridToken.sol` in your editor and paste:

```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title EtridToken
 * @dev ÉTR token on BSC/Ethereum - bridged from FlareChain
 */
contract EtridToken is ERC20, ERC20Burnable, Pausable, AccessControl {
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /**
     * @dev Constructor - no initial supply, bridge mints as needed
     */
    constructor() ERC20("Etrid Coin", "ETR") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(BRIDGE_ROLE, msg.sender); // Will transfer to bridge later
        _grantRole(PAUSER_ROLE, msg.sender);
    }

    /**
     * @dev Mint tokens (only bridge can call)
     * @param to Recipient address
     * @param amount Amount to mint (18 decimals)
     */
    function mint(address to, uint256 amount) public onlyRole(BRIDGE_ROLE) {
        _mint(to, amount);
    }

    /**
     * @dev Pause token transfers (emergency only)
     */
    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @dev Unpause token transfers
     */
    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    /**
     * @dev Override to prevent transfers when paused
     */
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }
}
```

### Step 1.3: Configure Hardhat for BSC

Edit `hardhat.config.js`:

```javascript
require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    // BSC Testnet (for testing)
    bscTestnet: {
      url: "https://data-seed-prebsc-1-s1.binance.org:8545",
      chainId: 97,
      accounts: process.env.DEPLOYER_PRIVATE_KEY ? [process.env.DEPLOYER_PRIVATE_KEY] : []
    },
    // BSC Mainnet (for production)
    bsc: {
      url: "https://bsc-dataseed.binance.org",
      chainId: 56,
      accounts: process.env.DEPLOYER_PRIVATE_KEY ? [process.env.DEPLOYER_PRIVATE_KEY] : []
    }
  },
  etherscan: {
    apiKey: {
      bsc: process.env.BSCSCAN_API_KEY || "",
      bscTestnet: process.env.BSCSCAN_API_KEY || ""
    }
  }
};
```

### Step 1.4: Create Environment Variables

Create `.env` file in `contracts/ethereum/`:

```bash
# Create .env file
touch .env
```

Edit `.env`:

```bash
# Your wallet private key (NEVER commit this!)
DEPLOYER_PRIVATE_KEY=your_private_key_here

# BSCScan API key (get from https://bscscan.com/apis)
BSCSCAN_API_KEY=your_bscscan_api_key_here

# Foundation multisig address (will receive admin role)
FOUNDATION_MULTISIG=0xYourMultisigAddressHere
```

**⚠️ IMPORTANT:** Add `.env` to `.gitignore`:

```bash
echo ".env" >> .gitignore
```

### Step 1.5: Create Deployment Script

Create `scripts/deploy-bsc.js`:

```javascript
const hre = require("hardhat");

async function main() {
  console.log("🚀 Deploying ÉTR token to BSC...\n");

  // Get deployer account
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying with account:", deployer.address);

  const balance = await hre.ethers.provider.getBalance(deployer.address);
  console.log("Account balance:", hre.ethers.formatEther(balance), "BNB\n");

  // Deploy token contract
  console.log("Deploying EtridToken contract...");
  const EtridToken = await hre.ethers.getContractFactory("EtridToken");
  const token = await EtridToken.deploy();

  await token.waitForDeployment();
  const tokenAddress = await token.getAddress();

  console.log("✅ ÉTR Token deployed to:", tokenAddress);
  console.log("\nToken Details:");
  console.log("  Name:", await token.name());
  console.log("  Symbol:", await token.symbol());
  console.log("  Decimals:", await token.decimals());
  console.log("  Total Supply:", await token.totalSupply());

  // Mint initial supply for liquidity
  console.log("\n💰 Minting initial supply for liquidity...");
  const liquidityAmount = hre.ethers.parseEther("50000000"); // 50M ÉTR
  const mintTx = await token.mint(deployer.address, liquidityAmount);
  await mintTx.wait();

  console.log("✅ Minted", hre.ethers.formatEther(liquidityAmount), "ÉTR to deployer");
  console.log("   Deployer balance:", hre.ethers.formatEther(await token.balanceOf(deployer.address)), "ÉTR");

  console.log("\n📝 Next Steps:");
  console.log("1. Verify contract on BSCScan:");
  console.log(`   npx hardhat verify --network ${hre.network.name} ${tokenAddress}`);
  console.log("\n2. Add to MetaMask:");
  console.log("   Token Address:", tokenAddress);
  console.log("   Token Symbol: ÉTR");
  console.log("   Decimals: 18");
  console.log("\n3. Create PancakeSwap pool at:");
  console.log("   https://pancakeswap.finance/add");

  // Save deployment info
  const fs = require('fs');
  const deploymentInfo = {
    network: hre.network.name,
    tokenAddress: tokenAddress,
    deployer: deployer.address,
    timestamp: new Date().toISOString(),
    initialMint: hre.ethers.formatEther(liquidityAmount)
  };

  fs.writeFileSync(
    `deployment-${hre.network.name}.json`,
    JSON.stringify(deploymentInfo, null, 2)
  );
  console.log("\n✅ Deployment info saved to deployment-" + hre.network.name + ".json");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

### Step 1.6: Deploy to BSC Testnet (Practice Run)

```bash
# Compile contracts
npx hardhat compile

# Deploy to BSC Testnet
npx hardhat run scripts/deploy-bsc.js --network bscTestnet

# If successful, verify on BSCScan Testnet
npx hardhat verify --network bscTestnet <TOKEN_ADDRESS>
```

**Expected output:**
```
🚀 Deploying ÉTR token to BSC...

Deploying with account: 0x123...
Account balance: 0.5 BNB

Deploying EtridToken contract...
✅ ÉTR Token deployed to: 0xABC123...

Token Details:
  Name: Etrid Coin
  Symbol: ÉTR
  Decimals: 18
  Total Supply: 0

💰 Minting initial supply for liquidity...
✅ Minted 50000000.0 ÉTR to deployer
```

### Step 1.7: Test on Testnet

1. **Add token to MetaMask:**
   - Open MetaMask
   - Switch to BSC Testnet
   - Click "Import tokens"
   - Paste token address from deployment
   - Symbol: ÉTR, Decimals: 18

2. **Create test liquidity pool:**
   - Go to PancakeSwap Testnet
   - Try adding small amount of liquidity
   - Verify pool creation works

### Step 1.8: Deploy to BSC Mainnet (Production)

**⚠️ ONLY after testnet success and review!**

```bash
# Deploy to BSC Mainnet
npx hardhat run scripts/deploy-bsc.js --network bsc

# Verify on BSCScan
npx hardhat verify --network bsc <TOKEN_ADDRESS>
```

---

## Part 2: Create PancakeSwap Liquidity Pool

### Step 2.1: Prepare Funds

You need:
- ÉTR tokens (already minted to your wallet)
- BNB for the pair (buy on exchange)

**Recommended starting amounts:**
- 1,000,000 ÉTR
- 167 BNB (~$50,000 liquidity at $300/BNB)

### Step 2.2: Add Liquidity on PancakeSwap

**Via PancakeSwap UI (easiest):**

1. Go to https://pancakeswap.finance/liquidity
2. Connect your MetaMask wallet
3. Click "Add Liquidity"
4. Select tokens:
   - Token A: BNB
   - Token B: ÉTR (paste contract address)
5. Choose pool type:
   - Select "V3" (more efficient)
   - Fee tier: 0.25% (recommended for new tokens)
6. Set price range:
   - For now, use "Full Range"
   - Initial price will be set by ratio of tokens you add
7. Enter amounts:
   - Example: 167 BNB + 1,000,000 ÉTR
   - This sets initial price: ~$0.10 per ÉTR
8. Click "Add" and approve transactions
9. **SAVE your LP token/NFT position ID!**

### Step 2.3: Lock Liquidity (Important for Trust)

**Option A: Use PancakeSwap's liquidity lock**
- Many projects use third-party services
- Check PancakeSwap docs for current options

**Option B: Transfer LP to multisig**
- Transfer LP NFT to your Gnosis Safe multisig
- Set timelock (6-12 months)

---

## Part 3: Solana Deployment (Raydium)

### Step 3.1: Install Solana Tools

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add to PATH
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Verify installation
solana --version

# Install SPL Token CLI
cargo install spl-token-cli

# Verify
spl-token --version
```

### Step 3.2: Set Up Solana Wallet

```bash
# Create new keypair (or import existing)
solana-keygen new --outfile ~/etrid-deployer.json

# Set as default keypair
solana config set --keypair ~/etrid-deployer.json

# Connect to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Check your address
solana address

# Check balance
solana balance
```

**⚠️ You need ~5-10 SOL in this wallet for:**
- Token creation: ~0.01 SOL
- Metadata creation: ~0.01 SOL
- Liquidity pool: ~16,667 SOL (or start smaller)

### Step 3.3: Create SPL Token

```bash
# Create token mint (this creates the ÉTR token on Solana)
spl-token create-token --decimals 9

# Save the output!
# Example output: "Creating token ABC123..."
# ABC123 is your MINT ADDRESS - save it!

export ETR_MINT=<your_mint_address_here>

# Create token account for yourself
spl-token create-account $ETR_MINT

# Mint initial supply (50M ÉTR with 9 decimals = 50,000,000,000,000,000)
spl-token mint $ETR_MINT 50000000000000000

# Check balance
spl-token balance $ETR_MINT
```

### Step 3.4: Add Token Metadata (Makes it show up nicely)

First, create metadata JSON and upload to your website:

**Create `token-metadata.json`:**
```json
{
  "name": "Etrid Coin",
  "symbol": "ÉTR",
  "description": "Native token of the Ëtrid blockchain - a multichain ecosystem with adaptive streaming finality consensus",
  "image": "https://etrid.org/assets/logo-512.png",
  "external_url": "https://etrid.org",
  "properties": {
    "category": "fungible-token",
    "files": [{
      "uri": "https://etrid.org/assets/logo-512.png",
      "type": "image/png"
    }]
  }
}
```

Upload this to: `https://etrid.org/token-metadata.json`

**Add metadata using Metaplex:**

```bash
# Install Metaplex CLI
npm install -g @metaplex-foundation/js @solana/web3.js

# Create metadata (this requires Node.js script)
```

Create `add-metadata.js`:

```javascript
const { Metaplex, keypairIdentity } = require("@metaplex-foundation/js");
const { Connection, Keypair } = require("@solana/web3.js");
const fs = require("fs");

async function main() {
  // Load your keypair
  const keypairFile = fs.readFileSync(
    process.env.HOME + "/etrid-deployer.json",
    "utf-8"
  );
  const secretKey = Uint8Array.from(JSON.parse(keypairFile));
  const wallet = Keypair.fromSecretKey(secretKey);

  // Connect to Solana
  const connection = new Connection("https://api.mainnet-beta.solana.com");
  const metaplex = Metaplex.make(connection).use(keypairIdentity(wallet));

  console.log("Creating metadata for ÉTR token...");

  // Add metadata
  const { nft } = await metaplex.nfts().create({
    uri: "https://etrid.org/token-metadata.json",
    name: "Etrid Coin",
    symbol: "ÉTR",
    sellerFeeBasisPoints: 0,
  });

  console.log("✅ Metadata created!");
  console.log("   Metadata address:", nft.address.toString());
}

main();
```

Run it:
```bash
node add-metadata.js
```

### Step 3.5: Create Raydium Pool

**Via Raydium UI (easiest):**

1. Go to https://raydium.io/liquidity/create
2. Connect Phantom wallet
3. Select pool type: "CLMM Pool" (Concentrated Liquidity)
4. Choose tokens:
   - Token A: SOL
   - Token B: ÉTR (paste mint address)
5. Set fee tier: 0.25%
6. Set initial price and range
7. Add liquidity:
   - Example: 333 SOL + 1,000,000 ÉTR (~$50k)
8. Confirm transactions
9. **SAVE your pool address!**

---

## Part 4: Submit to Aggregators

### CoinGecko Submission

1. Go to https://www.coingecko.com/en/coins/new
2. Fill out form:
   - Project name: Ëtrid
   - Token symbol: ÉTR
   - Contract addresses:
     - BSC: `<your_bsc_address>`
     - Solana: `<your_solana_mint>`
   - Website: https://etrid.org
   - Logo: Upload 200x200 PNG
3. Submit and wait (usually 1-2 weeks)

### CoinMarketCap Submission

1. Go to https://support.coinmarketcap.com/hc/en-us/requests/new
2. Select "Add new cryptocurrency"
3. Provide same info as CoinGecko
4. Submit and wait (usually 2-4 weeks)

---

## 📊 Monitoring Your Pools

**PancakeSwap (BSC):**
- Pool info: https://pancakeswap.finance/info/v3/bsc/pools
- DexTools: https://www.dextools.io/app/en/bnb/pair-explorer/<pool_address>
- DexScreener: https://dexscreener.com/bsc/<pool_address>

**Raydium (Solana):**
- Pool info: https://raydium.io/liquidity/
- Birdeye: https://birdeye.so/token/<mint_address>
- Jupiter: https://jup.ag/swap/SOL-<mint_address>

---

## ⚠️ Important Safety Notes

1. **Never share your private keys!**
2. **Test on testnets first**
3. **Start with small amounts** (you can add more liquidity later)
4. **Lock your LP tokens** (prevents rug pull concerns)
5. **Set up multisig** for admin roles
6. **Get contracts audited** before large deployments

---

## 🆘 Troubleshooting

**"Insufficient gas":**
- Add more BNB/SOL to your wallet

**"Transaction failed":**
- Increase gas limit in MetaMask settings
- Check network congestion

**"Token not showing in MetaMask":**
- Manually import using contract address

**"Pool creation failed":**
- Ensure you approved token spending
- Check you have enough of both tokens
- Try refreshing page and reconnecting wallet

---

## ✅ Success Checklist

After deployment, you should have:

- [ ] ÉTR token deployed on BSC (verified on BSCScan)
- [ ] ÉTR token deployed on Solana (with metadata)
- [ ] PancakeSwap liquidity pool created (BSC)
- [ ] Raydium liquidity pool created (Solana)
- [ ] LP tokens locked or in multisig
- [ ] Token visible in wallets (MetaMask, Phantom)
- [ ] Submitted to CoinGecko
- [ ] Submitted to CoinMarketCap
- [ ] Announced to community

---

**Need Help?**

Create an issue on GitHub or reach out on Discord!

**Next:** Once pools are live, monitor volume and prepare for Phase 2 (more DEXes) or Phase 3 (CEX listings)
