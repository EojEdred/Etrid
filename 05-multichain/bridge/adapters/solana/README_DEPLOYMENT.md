# ÉTR Solana Deployment Guide

**Quick Start Guide for Deploying ÉTR SPL Token on Solana**

Last Updated: October 24, 2025

---

## 📋 Prerequisites

- macOS, Linux, or Windows WSL2
- Node.js 18+ installed
- ~2 SOL for devnet (free from airdrop) OR ~0.02 SOL for mainnet (~$5)

---

## 🚀 Quick Start (5 Steps)

### Step 1: Install Solana CLI

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add to PATH (add this to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Reload shell
source ~/.bashrc  # or source ~/.zshrc

# Verify installation
solana --version
# Expected: solana-cli 1.17.x or newer
```

---

### Step 2: Install SPL Token CLI

```bash
# Install SPL Token CLI (for creating tokens)
cargo install spl-token-cli

# Verify installation
spl-token --version
# Expected: spl-token-cli 3.x.x
```

**Don't have Rust/Cargo?** Install first:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

### Step 3: Configure Solana for Devnet

```bash
# Set cluster to devnet
solana config set --url https://api.devnet.solana.com

# Verify configuration
solana config get

# Expected:
# RPC URL: https://api.devnet.solana.com
# WebSocket URL: wss://api.devnet.solana.com/
```

---

### Step 4: Generate Wallet & Get Devnet SOL

```bash
# Generate new keypair (wallet)
solana-keygen new --outfile ~/.config/solana/devnet-wallet.json

# Set as default keypair
solana config set --keypair ~/.config/solana/devnet-wallet.json

# Check your public key (wallet address)
solana address
# Expected: Base58 address like "5Yb8X9pZ..."

# Request airdrop (2 SOL)
solana airdrop 2

# Check balance
solana balance
# Expected: 2 SOL
```

**Airdrop failed?** Try these backup faucets:
- https://solfaucet.com/ (web-based)
- https://faucet.solana.com/ (official, requires CAPTCHA)

---

### Step 5: Create ÉTR SPL Token

```bash
# Create SPL token (9 decimals standard for Solana)
spl-token create-token --decimals 9

# Expected output:
# Creating token <TOKEN_ADDRESS>
# Signature: <TRANSACTION_SIGNATURE>

# SAVE THE TOKEN_ADDRESS! You'll need it.
# Example: 9Yb8X9pZqR3...

# Create token account to hold ÉTR
spl-token create-account <TOKEN_ADDRESS>

# Expected output:
# Creating account <ACCOUNT_ADDRESS>

# Optional: Mint test tokens (for testing only, 1000 ÉTR)
spl-token mint <TOKEN_ADDRESS> 1000

# Check balance
spl-token balance <TOKEN_ADDRESS>
# Expected: 1000
```

**🎉 DEPLOYMENT SUCCESSFUL!**

Your ÉTR SPL token is now live on Solana devnet!

---

## 📊 Cost Breakdown

| Action | Cost (Devnet) | Cost (Mainnet) |
|--------|---------------|----------------|
| Create token | $0 (airdrop) | ~$0.50 |
| Create account | $0 (airdrop) | ~$0.002 |
| Set metadata | $0 (airdrop) | ~$2-5 |
| **TOTAL** | **$0** | **~$2.50-5.50** |

**Comparison:**
- Solana mainnet: $2.50-5.50
- BSC mainnet: $5-20
- Ethereum mainnet: $500-2,000

**Solana is 100x cheaper than Ethereum!** 🎉

---

## 🏷️ Setting Token Metadata (Optional for Devnet)

```bash
# Install Metaplex Token Metadata CLI
npm install -g @metaplex-foundation/mpl-token-metadata

# Create metadata.json file
cat > metadata.json << EOF
{
  "name": "Etrid Coin",
  "symbol": "ÉTR",
  "description": "Ëtrid Protocol governance and utility token",
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
EOF

# Upload metadata (mainnet only - costs ~0.01 SOL)
# For devnet, we'll skip this to save time
# You can add metadata manually via Solana Explorer
```

---

## 🧪 Testing Your Deployment

### 1. View on Solana Explorer

**Devnet:**
- https://explorer.solana.com/?cluster=devnet
- Paste your token address

**You'll see:**
- Token name, symbol, supply
- All transactions
- Holders

### 2. Test Transfer

```bash
# Generate second wallet for testing
solana-keygen new --outfile ~/.config/solana/devnet-wallet-2.json

# Get its address
solana address --keypair ~/.config/solana/devnet-wallet-2.json
# Save this: <RECIPIENT_ADDRESS>

# Create token account for recipient
spl-token create-account <TOKEN_ADDRESS> \
  --owner <RECIPIENT_ADDRESS>

# Transfer 100 ÉTR to recipient
spl-token transfer <TOKEN_ADDRESS> 100 <RECIPIENT_ADDRESS>

# Check recipient balance
spl-token balance <TOKEN_ADDRESS> --owner <RECIPIENT_ADDRESS>
# Expected: 100
```

### 3. Test Burn

```bash
# Burn 50 ÉTR
spl-token burn <TOKEN_ADDRESS> 50

# Check balance (should be 50 less)
spl-token balance <TOKEN_ADDRESS>
```

---

## 🎯 Adding to Raydium (DEX)

### Prerequisites
- Token deployed on Solana mainnet
- At least 0.5 SOL + equivalent ÉTR for initial liquidity

### Create Liquidity Pool

**Option 1: Raydium UI (Easiest)**

1. Visit: https://raydium.io/liquidity/create/
2. Connect Phantom or Solflare wallet
3. Select ÉTR and SOL
4. Enter amounts (e.g., 0.5 SOL + 50,000 ÉTR)
5. Click "Create Pool"
6. Confirm transaction

**Option 2: Raydium SDK (Programmatic)**

See: https://docs.raydium.io/raydium/pool-creation/creating-a-pool

---

## 🔄 Bridge Integration

### Connecting Solana ÉTR to Ëtrid Chain

**Architecture:**
```
Ëtrid Chain ←→ Solana PBC ←→ Solana ÉTR SPL Token
```

**Bridge Contract:** (TBD - will be in `solana/bridge-adapter.ts`)

**Key Functions:**
- `lock_on_etrid()` → `mint_on_solana()`
- `burn_on_solana()` → `release_on_etrid()`

**See:** `/05-multichain/partition-burst-chains/pbc-chains/sol-pbc/` (existing Solana PBC)

---

## 📋 Mainnet Deployment Checklist

**Before deploying to mainnet:**

- [ ] Tested token creation on devnet
- [ ] Tested transfers on devnet
- [ ] Tested burning on devnet
- [ ] Have 0.02-0.05 SOL for mainnet gas
- [ ] Logo image uploaded and accessible
- [ ] Token name/symbol finalized
- [ ] Bridge contract tested on devnet

### Deploy to Mainnet

**⚠️ WARNING: This costs real money (but only ~$5)!**

```bash
# Switch to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Check balance (you need real SOL)
solana balance

# Create mainnet wallet if needed
solana-keygen new --outfile ~/.config/solana/mainnet-wallet.json
solana config set --keypair ~/.config/solana/mainnet-wallet.json

# Buy SOL on exchange (Binance, Coinbase, etc.)
# Send to your wallet address

# Create token (REAL MONEY!)
spl-token create-token --decimals 9

# SAVE THE ADDRESS!
# This is your permanent mainnet ÉTR SPL token address

# Create token account
spl-token create-account <TOKEN_ADDRESS>

# Set metadata (optional, costs ~0.01 SOL)
# Use Metaplex Token Metadata program
```

---

## 📁 File Structure

```
solana/
├── scripts/
│   ├── create-token.sh         # Automated token creation
│   ├── check-balance.sh        # Check SOL balance
│   └── test-transfer.sh        # Test token transfer
├── metadata.json               # Token metadata
├── bridge-adapter.ts           # Bridge integration (TBD)
└── README_DEPLOYMENT.md        # This file
```

---

## 🐛 Troubleshooting

### Error: "Airdrop failed"

**Solution:**
```bash
# Try requesting less SOL
solana airdrop 1  # Instead of 2

# Or use web faucet
# Visit: https://solfaucet.com/
```

### Error: "insufficient funds for rent"

**Solution:**
```bash
# Solana requires minimum balance for "rent-exempt" accounts
# Need at least 0.002 SOL per account

# Request more from airdrop
solana airdrop 1
```

### Error: "RPC request failed"

**Solution:**
```bash
# Network congestion - try different RPC
solana config set --url https://solana-api.projectserum.com

# Or wait a few minutes and try again
```

### Can't find `spl-token` command

**Solution:**
```bash
# Make sure Rust/Cargo is installed
cargo --version

# If not installed:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Reinstall SPL Token CLI
cargo install spl-token-cli
```

---

## 🔐 Security Best Practices

### ❌ DON'T:
- Commit wallet keypairs to Git
- Share private keys
- Use devnet wallet for mainnet
- Deploy to mainnet without testing first

### ✅ DO:
- Back up wallet keypair file (save to USB drive)
- Use hardware wallet for mainnet (Ledger supported)
- Test thoroughly on devnet first
- Keep devnet and mainnet wallets separate

---

## 📊 Comparison: Solana vs BSC

| Metric | Solana | BSC |
|--------|--------|-----|
| **Block Time** | 400ms | 3 seconds |
| **Deployment Cost** | $2.50-5.50 | $5-20 |
| **Transfer Cost** | $0.00025 | $0.05-0.15 |
| **TPS** | 65,000+ | ~100 |
| **Finality** | 13 seconds | ~15 seconds |
| **Token Standard** | SPL | BEP-20 (ERC-20) |

**Winner for cost:** Solana (10x cheaper)
**Winner for ecosystem:** BSC (more mature DeFi)
**Our strategy:** Deploy to BOTH! 🎉

---

## 📞 Need Help?

**Stuck? Have questions?**

1. **Check Solana docs**: https://docs.solana.com/
2. **SPL Token guide**: https://spl.solana.com/token
3. **Discord**: #dev-support channel
4. **Solana Discord**: https://discord.gg/solana

---

## 📚 Additional Resources

- **Solana CLI Reference**: https://docs.solana.com/cli
- **SPL Token CLI**: https://spl.solana.com/token
- **Raydium Docs**: https://docs.raydium.io/
- **Phantom Wallet**: https://phantom.app/
- **Solflare Wallet**: https://solflare.com/

---

## 🎯 Next Steps After Successful Deployment

1. **Add liquidity on Raydium** (ÉTR/SOL pool)
2. **Submit to Jupiter Aggregator** (https://jup.ag/)
3. **Deploy MasterChef rewards contract** (Anchor program)
4. **Integrate bridge** with existing Solana PBC
5. **Announce to community**

---

**Last Updated**: October 24, 2025
**Maintainer**: Ëtrid Protocol Team
**Status**: Ready for Devnet Deployment
**Estimated Time**: 15 minutes total
**Estimated Cost (Mainnet)**: $2.50-5.50
