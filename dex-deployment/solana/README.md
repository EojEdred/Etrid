# ÉTR Token Solana Deployment

Deploy ÉTR (Ëtrid Coin) to Solana as SPL token for Raydium CLMM listing.

## Quick Start

### 1. Install Solana CLI

```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

### 2. Install SPL Token CLI

```bash
cargo install spl-token-cli
```

### 3. Configure Wallet

```bash
# Generate new keypair (or use existing)
solana-keygen new --outfile ~/.config/solana/id.json

# Set devnet for testing
solana config set --url https://api.devnet.solana.com

# Get devnet SOL
solana airdrop 2
```

### 4. Deploy Token

```bash
chmod +x deploy-solana.sh
./deploy-solana.sh
```

Follow the interactive prompts to deploy to devnet (testing) or mainnet (production).

## Token Specifications

- **Name:** Etrid Coin
- **Symbol:** ETR
- **Decimals:** 9 (Solana standard)
- **Initial Supply:** 100,000,000 ÉTR
- **Max Supply:** 1,000,000,000 ÉTR
- **Standard:** SPL Token

## Raydium Integration

After deployment, create CLMM pool:

1. Go to [Raydium Liquidity](https://raydium.io/liquidity/create/)
2. Select tokens:
   - Token A: **ÉTR** (your deployed address)
   - Token B: **SOL** (So11111111111111111111111111111111111111112)
3. Choose fee tier: **0.25%**
4. Set initial price (match BSC price)
5. Add liquidity:
   - 25,000,000 ÉTR
   - ~13,333 SOL (~$2M @ $150/SOL)

## Adding Metadata

### Option 1: Using Metaboss (Recommended)

```bash
# Install Metaboss
cargo install metaboss

# Upload metadata to Arweave (requires AR tokens)
metaboss upload metadata-etr.json

# Update token metadata
metaboss update uri -a <TOKEN_ADDRESS> -u <ARWEAVE_URI>
```

### Option 2: Manual with Metaplex

1. Upload `metadata-etr.json` to IPFS or Arweave
2. Use Metaplex Candy Machine UI
3. Set metadata URI to your uploaded JSON

## Token List Submissions

### Solana Token List

```bash
# Fork repo
git clone https://github.com/solana-labs/token-list.git
cd token-list

# Add token to src/tokens/solana.tokenlist.json
# Submit PR
```

### Jupiter Aggregator

Submit at: https://station.jup.ag/token-list

Include:
- Token address
- Symbol: ETR
- Name: Etrid Coin
- Decimals: 9
- Logo URL: https://etrid.org/images/etr-logo.png

## Cost Breakdown

### Devnet (Free)
- Token creation: FREE
- Account creation: FREE
- Minting: FREE
- Get testnet SOL: FREE (faucet)

### Mainnet
- Token creation: ~0.01 SOL (~$1.50)
- Account creation: ~0.002 SOL (~$0.30)
- Minting: ~0.01 SOL (~$1.50)
- **Total: ~0.022 SOL (~$3.30)**

Liquidity separate: 25M ÉTR + 13,333 SOL (~$2M)

## Security

- Token mint authority: Foundation multisig
- Freeze authority: Disabled (fully decentralized)
- Metadata update authority: Foundation
- Bridge integration: Wormhole (cross-chain)

## Governance

Per **FOUNDATION_CHARTER.md**:

- Deployment requires 6-of-9 Director signatures
- Liquidity from Community LP Pool (250M ÉTR)
- Emergency actions require 7-of-9 signatures

## Support

- Technical: dev@etrid.org
- Foundation: directors@etrid.org
- Discord: #dex-deployment channel

## References

- [Solana Token Program](https://spl.solana.com/token)
- [Raydium CLMM Docs](https://docs.raydium.io/)
- [Complete DEX Deployment Guide](../../COMPLETE_DEX_DEPLOYMENT_GUIDE.md)
