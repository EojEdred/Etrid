# Testing Environment Setup Guide

**Version**: 1.0
**Last Updated**: October 24, 2025
**Purpose**: Complete guide for setting up BSC testnet and Solana devnet for ÉTR deployment testing

---

## 📋 Overview

Before deploying to mainnet, we'll test all functionality on testnets:
- **BSC Testnet**: Test ÉTR token, PancakeSwap pools, LP rewards contract
- **Solana Devnet**: Test ÉTR SPL token, Raydium pools, staking

**Timeline**: Week 1 (Oct 28 - Nov 3, 2025)

**Goal**: Fully validated deployment scripts and contracts before spending real money on mainnet gas.

---

## 🛠️ Prerequisites

### System Requirements

**Operating System**: macOS, Linux, or Windows (WSL2)

**Software to Install**:
- [ ] Node.js 18+ and npm
- [ ] Git
- [ ] Rust and Cargo (for Solana)
- [ ] Code editor (VS Code recommended)

### Check Existing Installations

```bash
# Check Node.js version (should be 18+)
node --version

# Check npm version
npm --version

# Check Git
git --version

# Check Rust (for Solana)
rustc --version
cargo --version
```

If any are missing, install them first:

```bash
# Install Node.js (via nvm - recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
nvm install 18
nvm use 18

# Install Rust (for Solana)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## 🔧 Part 1: BSC Testnet Setup

### Step 1: Install Hardhat (Smart Contract Framework)

```bash
# Navigate to your project directory
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc

# Initialize npm project (if not already done)
npm init -y

# Install Hardhat and dependencies
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox ethers

# Install OpenZeppelin contracts
npm install @openzeppelin/contracts

# Initialize Hardhat project
npx hardhat
# Select: "Create a TypeScript project"
# Accept defaults for project root and .gitignore
```

### Step 2: Configure Hardhat for BSC Testnet

Create or update `hardhat.config.ts`:

```typescript
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import * as dotenv from "dotenv";

dotenv.config();

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    // BSC Testnet
    bscTestnet: {
      url: "https://data-seed-prebsc-1-s1.bnbchain.org:8545",
      chainId: 97,
      accounts: process.env.DEPLOYER_PRIVATE_KEY
        ? [process.env.DEPLOYER_PRIVATE_KEY]
        : [],
      gasPrice: 10000000000 // 10 gwei
    },
    // BSC Mainnet (for future)
    bscMainnet: {
      url: "https://bsc-dataseed.bnbchain.org",
      chainId: 56,
      accounts: process.env.DEPLOYER_PRIVATE_KEY
        ? [process.env.DEPLOYER_PRIVATE_KEY]
        : [],
      gasPrice: 3000000000 // 3 gwei
    }
  },
  etherscan: {
    apiKey: {
      bscTestnet: process.env.BSCSCAN_API_KEY || "",
      bsc: process.env.BSCSCAN_API_KEY || ""
    }
  }
};

export default config;
```

### Step 3: Create `.env` File (BSC Credentials)

Create `.env` in the `bsc/` directory:

```bash
# NEVER commit this file to Git!
# Add .env to .gitignore

# BSC Testnet wallet private key (generate new wallet for testing)
DEPLOYER_PRIVATE_KEY="your_testnet_private_key_here"

# BscScan API key (for contract verification)
BSCSCAN_API_KEY="your_bscscan_api_key_here"

# RPC URLs (backup options)
BSC_TESTNET_RPC="https://data-seed-prebsc-1-s1.bnbchain.org:8545"
BSC_MAINNET_RPC="https://bsc-dataseed.bnbchain.org"
```

**Security Note**: NEVER share your private key or commit `.env` to version control!

### Step 4: Generate Testnet Wallet

```bash
# Option 1: Use Hardhat to generate new wallet
npx hardhat run scripts/generate-wallet.ts

# Option 2: Use MetaMask
# - Install MetaMask extension
# - Create new account
# - Switch network to "BSC Testnet"
# - Copy private key (Settings → Security & Privacy → Export Private Key)
```

Create `scripts/generate-wallet.ts`:

```typescript
import { ethers } from "hardhat";

async function main() {
  // Generate random wallet
  const wallet = ethers.Wallet.createRandom();

  console.log("New Testnet Wallet Generated:");
  console.log("Address:", wallet.address);
  console.log("Private Key:", wallet.privateKey);
  console.log("\n⚠️  SAVE THIS PRIVATE KEY SECURELY!");
  console.log("⚠️  Add it to .env as DEPLOYER_PRIVATE_KEY");
  console.log("\nNext step: Fund this address with testnet BNB from faucet.");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
```

Run it:

```bash
npx hardhat run scripts/generate-wallet.ts
# Copy the private key to .env
```

### Step 5: Get Testnet BNB from Faucet

**Official BSC Testnet Faucet**: https://testnet.bnbchain.org/faucet-smart

```bash
# Visit faucet website
# Paste your testnet wallet address
# Complete CAPTCHA
# Receive 0.5 BNB testnet tokens (usually within 1-2 minutes)

# Check balance
npx hardhat run scripts/check-balance.ts --network bscTestnet
```

Create `scripts/check-balance.ts`:

```typescript
import { ethers } from "hardhat";

async function main() {
  const [deployer] = await ethers.getSigners();
  const balance = await ethers.provider.getBalance(deployer.address);

  console.log("Deployer Address:", deployer.address);
  console.log("Balance:", ethers.formatEther(balance), "BNB");
}

main();
```

**Backup Faucets** (if official is down):
- https://www.bnbchain.org/en/testnet-faucet
- https://testnet.help/en/bnbfaucet/testnet (requires Twitter verification)

### Step 6: Test Deployment Script

Create `scripts/deploy-etr-testnet.ts`:

```typescript
import { ethers } from "hardhat";

async function main() {
  console.log("Deploying ÉTR Token to BSC Testnet...");

  const [deployer] = await ethers.getSigners();
  console.log("Deploying with account:", deployer.address);

  const balance = await ethers.provider.getBalance(deployer.address);
  console.log("Account balance:", ethers.formatEther(balance), "BNB");

  // Deploy ÉTR token
  const EtridToken = await ethers.getContractFactory("EtridToken");
  const etr = await EtridToken.deploy(
    "Etrid Coin (BSC Testnet)",
    "ÉTR",
    ethers.parseEther("0") // Initial supply: 0 (minted via bridge)
  );

  await etr.waitForDeployment();
  const etrAddress = await etr.getAddress();

  console.log("✅ ÉTR Token deployed to:", etrAddress);
  console.log("\nNext steps:");
  console.log("1. Verify contract on BscScan Testnet");
  console.log("2. Add to MetaMask (use address above)");
  console.log("3. Test minting and transfers");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
```

Run deployment:

```bash
npx hardhat run scripts/deploy-etr-testnet.ts --network bscTestnet

# Expected output:
# Deploying ÉTR Token to BSC Testnet...
# Deploying with account: 0x...
# Account balance: 0.5 BNB
# ✅ ÉTR Token deployed to: 0x... (save this address!)
```

### Step 7: Verify Contract on BscScan Testnet

```bash
# Get BscScan API key (free):
# - Visit https://bscscan.com/register
# - Go to "API Keys" → "Add"
# - Copy key to .env as BSCSCAN_API_KEY

# Verify deployed contract
npx hardhat verify --network bscTestnet <ETR_TOKEN_ADDRESS> \
  "Etrid Coin (BSC Testnet)" \
  "ÉTR" \
  "0"

# Expected output:
# Successfully submitted source code for contract
# contracts/EtridToken.sol:EtridToken at 0x...
# Waiting for verification result...
# Successfully verified contract EtridToken on BscScan.
# https://testnet.bscscan.com/address/0x...#code
```

---

## 🌐 Part 2: Solana Devnet Setup

### Step 1: Install Solana CLI

```bash
# Install Solana CLI (official method)
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add to PATH (add this to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Reload shell
source ~/.bashrc  # or source ~/.zshrc

# Verify installation
solana --version
# Expected: solana-cli 1.17.x or newer
```

### Step 2: Install Anchor Framework (Solana Smart Contracts)

```bash
# Install Anchor Version Manager (avm)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# Install latest Anchor version
avm install latest
avm use latest

# Verify installation
anchor --version
# Expected: anchor-cli 0.29.x or newer
```

### Step 3: Install SPL Token CLI

```bash
# Install SPL Token CLI (for creating tokens)
cargo install spl-token-cli

# Verify installation
spl-token --version
# Expected: spl-token-cli 3.x.x
```

### Step 4: Configure Solana CLI for Devnet

```bash
# Set cluster to devnet
solana config set --url https://api.devnet.solana.com

# Verify configuration
solana config get
# Expected:
# Config File: ~/.config/solana/cli/config.yml
# RPC URL: https://api.devnet.solana.com
# WebSocket URL: wss://api.devnet.solana.com/
# Keypair Path: ~/.config/solana/id.json
```

### Step 5: Generate Solana Devnet Wallet

```bash
# Generate new keypair (wallet)
solana-keygen new --outfile ~/.config/solana/devnet-wallet.json

# Set as default keypair
solana config set --keypair ~/.config/solana/devnet-wallet.json

# Check public key (wallet address)
solana address
# Expected: Base58 address like "5Yb8X9pZ..."

# Check balance (should be 0 initially)
solana balance
# Expected: 0 SOL
```

**Security Note**: Keep your keypair file secure! For testnet, it's okay to regenerate, but for mainnet, losing this file = losing access to funds.

### Step 6: Get Devnet SOL from Airdrop

```bash
# Request airdrop (2 SOL)
solana airdrop 2

# Check balance
solana balance
# Expected: 2 SOL (or close, depending on network)

# If airdrop fails (rate-limited), try these backup faucets:
# - https://solfaucet.com/ (web-based)
# - https://faucet.solana.com/ (official, requires CAPTCHA)
# - Join Solana Discord and ask in #devnet-faucet channel
```

### Step 7: Create SPL Token (ÉTR on Solana Devnet)

```bash
# Create new SPL token
spl-token create-token --decimals 9

# Expected output:
# Creating token <TOKEN_ADDRESS>
# Signature: <TRANSACTION_SIGNATURE>
#
# Save the TOKEN_ADDRESS! You'll need it.

# Example: 9Yb8X9pZqR3... (save this!)
export ETR_TOKEN_ADDRESS="<YOUR_TOKEN_ADDRESS>"

# Create token account to hold ÉTR
spl-token create-account $ETR_TOKEN_ADDRESS

# Expected output:
# Creating account <ACCOUNT_ADDRESS>

# Mint test tokens (for testing only, 1000 ÉTR)
spl-token mint $ETR_TOKEN_ADDRESS 1000

# Expected output:
# Minting 1000 tokens
# Signature: <TX_SIGNATURE>

# Check token balance
spl-token balance $ETR_TOKEN_ADDRESS
# Expected: 1000
```

### Step 8: Set Token Metadata (Metaplex)

Install Metaplex Sugar CLI:

```bash
# Install Sugar (Metaplex metadata tool)
bash <(curl -sSf https://sugar.metaplex.com/install.sh)

# Verify installation
sugar --version
```

Create `metadata.json`:

```json
{
  "name": "Etrid Coin (Devnet)",
  "symbol": "ÉTR",
  "description": "Ëtrid Protocol governance and utility token on Solana Devnet",
  "image": "https://etrid.io/images/etr-logo.png",
  "external_url": "https://etrid.io",
  "attributes": [],
  "properties": {
    "files": [
      {
        "uri": "https://etrid.io/images/etr-logo.png",
        "type": "image/png"
      }
    ],
    "category": "fungible"
  }
}
```

Upload metadata:

```bash
# Note: Metaplex metadata is typically for NFTs
# For SPL tokens, we'll use a simpler approach via Token Metadata Program

# Install Token Metadata CLI (alternative to Sugar for fungible tokens)
npm install -g @metaplex-foundation/mpl-token-metadata

# For now, we'll skip metadata on devnet and add it on mainnet
# (Metadata costs ~0.01 SOL, not critical for testing)
```

### Step 9: Test Token Transfers

```bash
# Generate second wallet for testing
solana-keygen new --outfile ~/.config/solana/devnet-wallet-2.json

# Get its address
solana address --keypair ~/.config/solana/devnet-wallet-2.json
# Save this address: <RECIPIENT_ADDRESS>

# Create token account for recipient
spl-token create-account $ETR_TOKEN_ADDRESS --owner <RECIPIENT_ADDRESS>

# Transfer 100 ÉTR to recipient
spl-token transfer $ETR_TOKEN_ADDRESS 100 <RECIPIENT_ADDRESS>

# Check recipient balance
spl-token balance $ETR_TOKEN_ADDRESS --owner <RECIPIENT_ADDRESS>
# Expected: 100
```

---

## 🧪 Part 3: Integration Testing Workflow

### BSC Testnet Testing Checklist

**Day 1: Token Deployment**
- [ ] Deploy ÉTR token to BSC testnet
- [ ] Verify contract on BscScan testnet
- [ ] Add token to MetaMask
- [ ] Test minting function (via multi-sig or owner)
- [ ] Test transfer function (send to another testnet wallet)

**Day 2: PancakeSwap Pool Creation**
- [ ] Visit PancakeSwap testnet: https://pancakeswap.finance/ (switch to testnet mode)
- [ ] Add liquidity: ÉTR/BNB pool (use 0.1 BNB + equivalent ÉTR)
- [ ] Confirm LP tokens received
- [ ] Test swap: BNB → ÉTR and ÉTR → BNB
- [ ] Verify pool appears on PancakeSwap UI

**Day 3: LP Rewards Contract**
- [ ] Deploy MasterChef contract to BSC testnet
- [ ] Transfer test ÉTR allocation (1000 ÉTR) to MasterChef
- [ ] Add ÉTR/BNB pool to MasterChef via `add()`
- [ ] Stake LP tokens via `deposit()`
- [ ] Wait 100 blocks (~5 minutes)
- [ ] Call `harvest()` to claim rewards
- [ ] Verify ÉTR balance increased
- [ ] Call `withdraw()` to unstake LP tokens

### Solana Devnet Testing Checklist

**Day 1: SPL Token Creation**
- [x] Install Solana CLI and SPL Token CLI
- [x] Generate devnet wallet
- [x] Request devnet SOL airdrop
- [x] Create SPL token (ÉTR)
- [x] Mint test tokens
- [ ] Test transfers between wallets

**Day 2: Raydium Pool Setup**
- [ ] Visit Raydium devnet (if available, or test locally)
- [ ] Create ÉTR/SOL pool (0.1 SOL + equivalent ÉTR)
- [ ] Test swap functionality
- [ ] Verify pool liquidity

**Day 3: Anchor Program (LP Rewards)**
- [ ] Initialize Anchor project: `anchor init etr-staking`
- [ ] Write staking program (Rust)
- [ ] Deploy to devnet
- [ ] Test deposit/withdraw/harvest functions
- [ ] Verify reward distribution

---

## 📊 Part 4: Monitoring and Debugging

### Useful Block Explorers

**BSC Testnet**:
- BscScan Testnet: https://testnet.bscscan.com/
- Check transactions, contracts, token balances

**Solana Devnet**:
- Solana Explorer (Devnet): https://explorer.solana.com/?cluster=devnet
- SolScan (Devnet): https://solscan.io/?cluster=devnet

### Common Issues and Solutions

#### BSC Testnet

**Issue**: "Insufficient funds for gas"
```bash
# Solution: Get more testnet BNB from faucet
# Visit https://testnet.bnbchain.org/faucet-smart
```

**Issue**: "Transaction reverted"
```bash
# Solution: Check contract code for errors
# Enable Hardhat debug mode:
npx hardhat run scripts/deploy.ts --network bscTestnet --verbose
```

**Issue**: "Nonce too low"
```bash
# Solution: Reset nonce in MetaMask
# Settings → Advanced → Reset Account
```

#### Solana Devnet

**Issue**: "Airdrop failed"
```bash
# Solution 1: Try again (rate-limited)
solana airdrop 1  # Request less (1 SOL instead of 2)

# Solution 2: Use web faucet
# Visit https://solfaucet.com/ and paste your address
```

**Issue**: "insufficient funds for rent"
```bash
# Solution: Need more SOL for rent-exempt accounts
# Minimum: ~0.002 SOL per account
solana airdrop 1
```

**Issue**: "Transaction simulation failed"
```bash
# Solution: Check Anchor program logs
anchor test --skip-deploy  # Test without redeploying
```

---

## 🎯 Part 5: Pre-Mainnet Validation

### Final Checklist Before Mainnet Deployment

**BSC**:
- [ ] All testnet tests passed (10+ transactions)
- [ ] Gas costs estimated and acceptable
- [ ] Contract verified on BscScan testnet
- [ ] No errors in console/logs
- [ ] Multi-sig wallet tested (if using)
- [ ] Emergency pause function tested

**Solana**:
- [ ] All devnet tests passed (10+ transactions)
- [ ] SPL token transfers work correctly
- [ ] Metadata set properly (name, symbol, decimals)
- [ ] Anchor program deployed and tested
- [ ] No errors in Solana Explorer logs

**Documentation**:
- [ ] All deployment scripts saved and documented
- [ ] Contract addresses recorded
- [ ] Private keys backed up securely
- [ ] Team members trained on deployment process

---

## 📚 Additional Resources

### BSC Development
- **BNB Chain Docs**: https://docs.bnbchain.org/docs/overview
- **Hardhat Docs**: https://hardhat.org/docs
- **OpenZeppelin**: https://docs.openzeppelin.com/contracts/
- **PancakeSwap Docs**: https://docs.pancakeswap.finance/

### Solana Development
- **Solana Cookbook**: https://solanacookbook.com/
- **Anchor Book**: https://book.anchor-lang.com/
- **SPL Token Guide**: https://spl.solana.com/token
- **Metaplex Docs**: https://docs.metaplex.com/

### Video Tutorials
- **Hardhat BSC Deployment**: https://www.youtube.com/watch?v=... (search YouTube)
- **Solana Token Creation**: https://www.youtube.com/watch?v=... (search YouTube)
- **Anchor Programming**: https://www.youtube.com/playlist?list=... (Solana official playlist)

---

## 🆘 Getting Help

**Stuck on setup?**

1. **Check error messages carefully** - most issues are in logs
2. **Search GitHub Issues** - Hardhat, Anchor, Solana repos
3. **Ask in Discord**:
   - Ëtrid Discord: #dev-support channel
   - BNB Chain Discord: https://discord.gg/bnbchain
   - Solana Discord: https://discord.gg/solana
4. **Stack Overflow**: Tag questions with `hardhat`, `solana`, `bsc`

**Emergency Contacts**:
- Lead Developer: eoj@etrid.io
- DevOps: (if available)

---

**Last Updated**: October 24, 2025
**Next Steps**: Begin Week 1 testnet deployment (Oct 28, 2025)
**Maintainer**: Ëtrid Protocol Team
