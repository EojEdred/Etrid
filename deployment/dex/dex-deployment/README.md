# ËTRID DEX Deployment Package

Complete deployment system for ÉTR token across all decentralized exchanges.

## Overview

This directory contains everything needed to deploy ÉTR (Ëtrid Coin) to 20+ decentralized exchanges across 7+ blockchain networks.

**Reference Documents:**
- [COMPLETE_DEX_DEPLOYMENT_GUIDE.md](../COMPLETE_DEX_DEPLOYMENT_GUIDE.md) - Full strategy
- [FOUNDATION_CHARTER.md](../FOUNDATION_CHARTER.md) - Governance rules
- [protocol-charter.md](../docs/specifications/protocol-charter.md) - Token specs

## Directory Structure

```
dex-deployment/
├── DEPLOY_ALL_DEX.sh          # Master deployment script
├── README.md                   # This file
├── bsc/                        # Binance Smart Chain (PancakeSwap)
│   ├── EtridBSC.sol           # BEP-20 token contract
│   ├── deploy.js              # Deployment script
│   ├── hardhat.config.js      # Hardhat configuration
│   ├── package.json           # Dependencies
│   ├── .env.example           # Environment template
│   └── README.md              # BSC-specific docs
├── solana/                     # Solana (Raydium)
│   ├── deploy-solana.sh       # SPL token deployment
│   ├── metadata-etr.json      # Token metadata
│   └── README.md              # Solana-specific docs
├── ethereum/                   # Ethereum (Uniswap) [TBD]
├── polygon/                    # Polygon (QuickSwap) [TBD]
├── scripts/                    # Helper scripts
└── deployment-logs/            # Deployment records
```

## Quick Start

### Prerequisites

Install required tools:

```bash
# Node.js & npm (for EVM chains)
node --version   # v18+ required
npm --version    # v9+ required

# Solana CLI (for Solana)
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"

# Rust/Cargo (optional, for SPL tools)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Deployment Options

#### Option 1: Master Script (Recommended)

Deploy to all chains at once:

```bash
./DEPLOY_ALL_DEX.sh
```

Follow interactive prompts to select:
1. Deployment phase (Phase 1, 2, or Full)
2. Environment (Testnet or Mainnet)
3. Confirmation

#### Option 2: Individual Chain Deployment

Deploy to specific chains:

```bash
# BSC (Binance Smart Chain)
cd bsc
npm install
npm run deploy:testnet  # or deploy:mainnet

# Solana
cd solana
./deploy-solana.sh

# Ethereum (coming soon)
cd ethereum
npm run deploy:mainnet

# Polygon (coming soon)
cd polygon
npm run deploy:mainnet
```

## Deployment Phases

### Phase 1: Priority DEXes (Mainnet Launch)

**Chains:** BSC, Solana, Ethereum, Polygon
**ÉTR Allocation:** 90M
**Liquidity:** $7M
**Timeline:** Day 1-3 after mainnet

| DEX | Chain | ÉTR | Liquidity | Status |
|-----|-------|-----|-----------|--------|
| PancakeSwap V3 | BSC | 25M | $2M BNB | 🔴 Priority 1 |
| Raydium CLMM | Solana | 25M | $2M SOL | 🔴 Priority 1 |
| Uniswap V3 | Ethereum | 25M | $2M ETH | 🔴 Priority 1 |
| QuickSwap V3 | Polygon | 15M | $1M MATIC | 🟡 Priority 2 |

**Estimated Cost:**
- Gas fees: ~$500
- Liquidity: $7M in native tokens
- Time: 2-4 hours

### Phase 2: Expansion (Week 2-4)

**Chains:** Avalanche, Arbitrum, Base, + more
**ÉTR Allocation:** 56M
**Liquidity:** $4.4M

Deploy after Phase 1 is established and trading smoothly.

### Phase 3+: Full Ecosystem

Deploy to remaining 10+ DEXes for maximum market coverage.

## Configuration

### Environment Setup

Each chain directory has a `.env.example` file. Copy and configure:

```bash
# Example: BSC
cd bsc
cp .env.example .env
nano .env  # Add your PRIVATE_KEY and BSCSCAN_API_KEY
```

**Required for Each Chain:**
- Private key (Foundation multisig signer)
- RPC endpoint URL (optional, defaults provided)
- Block explorer API key (for verification)
- Native token for gas fees

### Funding Requirements

**Phase 1 Gas Fees:**
- BSC: 0.05 BNB (~$15)
- Solana: 0.05 SOL (~$7.50)
- Ethereum: 0.1 ETH (~$300)
- Polygon: 10 MATIC (~$10)
- **Total: ~$350**

**Phase 1 Liquidity:**
- BSC: 3,333 BNB (~$2M)
- Solana: 13,333 SOL (~$2M)
- Ethereum: 666 ETH (~$2M)
- Polygon: 1M MATIC (~$1M)
- **Total: ~$7M**

## Governance & Approval

Per **FOUNDATION_CHARTER.md** Section IV:

### Treasury Disbursement

**Required:** 6-of-9 Decentralized Director signatures

**Process:**
1. Proposal submitted with deployment plan
2. 3-day review period
3. 4-day voting period
4. 6-of-9 approval required
5. Liquidity released from Community LP Pool (250M ÉTR allocation)

### Emergency Actions

If deployment issues occur:

**Required:** 7-of-9 Director signatures

**Scenarios:**
- Compromised deployment keys
- Failed pool creation
- Unexpected token contract behavior

## Token Specifications

### Native FlareChain
- Decimals: 5
- Total Supply: 1,000,000,000 ÉTR
- Base Unit: Bite (10^-5)

### Cross-Chain Deployments

| Chain | Standard | Decimals | Bridge |
|-------|----------|----------|--------|
| BSC | BEP-20 | 18 | Wormhole |
| Solana | SPL | 9 | Wormhole |
| Ethereum | ERC-20 | 18 | Native/Wormhole |
| Polygon | ERC-20 | 18 | PoS Bridge |
| Avalanche | ERC-20 | 18 | Avalanche Bridge |
| Arbitrum | ERC-20 | 18 | Arbitrum Bridge |
| Base | ERC-20 | 18 | Base Bridge |

**Note:** Decimals differ across chains to match ecosystem standards.

## Post-Deployment

After successful deployment:

### 1. Create Liquidity Pools

Use DEX interfaces to create trading pools:
- Set fee tiers (typically 0.25% or 0.3%)
- Add initial liquidity
- Set price ranges (CLMM pools)

### 2. Submit Listings

**Tracking Aggregators:**
- CoinGecko: https://www.coingecko.com/en/coins/new
- CoinMarketCap: https://coinmarketcap.com/request/

**DEX Aggregators:**
- 1inch: Token list submission
- Jupiter (Solana): https://station.jup.ag/token-list
- Matcha: Automatic detection

### 3. Update Website

Update etrid.org with:
- All token contract addresses
- Links to trade on each DEX
- "Buy ÉTR" button
- Price displays

### 4. Announce Launch

Social media announcement:
- Twitter: @EtridProtocol
- Discord: #announcements
- Telegram: t.me/EtridOfficial
- Reddit: r/Etrid

### 5. Monitor & Report

**First 24 Hours:**
- Monitor liquidity depth
- Watch for price anomalies
- Track trading volume
- Check for exploits/issues

**Quarterly Report:**
- Document all deployments
- Record transaction hashes
- Report volume and liquidity
- Submit to Foundation Directors

## Security

### Smart Contract Audits

Before mainnet deployment:
- [x] OpenZeppelin contracts (audited)
- [ ] Internal code review (3+ developers)
- [ ] External audit (Certik or Trail of Bits)
- [ ] Test coverage >80%

### Deployment Security

- Use hardware wallet for signing
- Verify all contract addresses
- Test on testnet first
- Use multisig for ownership
- Set appropriate gas limits

### Post-Deployment Monitoring

- Set up alerts for:
  - Large transactions (>$10k)
  - Unusual trading patterns
  - Liquidity changes
  - Contract interactions
- Monitor 24/7 for first week
- Foundation on-call rotation

## Troubleshooting

### Common Issues

**1. "Insufficient funds for gas"**
```bash
# Check balance
solana balance  # Solana
cast balance <address>  # Ethereum/EVM

# Get funds
# Testnet: Use faucets
# Mainnet: Buy from exchange
```

**2. "Contract verification failed"**
```bash
# Ensure correct compiler version
# Check constructor arguments
# Verify API key is valid
npx hardhat verify --network <network> <address> [args...]
```

**3. "Pool creation failed"**
- Check token approvals
- Verify sufficient liquidity
- Ensure correct token addresses
- Try increasing slippage

**4. "Transaction underpriced"**
- Increase gas price
- Check network congestion
- Wait and retry

### Support

**Technical Issues:**
- Discord: #dex-deployment channel
- Email: dev@etrid.org
- GitHub Issues: github.com/EojEdred/Etrid/issues

**Foundation Contact:**
- Email: directors@etrid.org
- Emergency: (24/7 on-call Director)

## Testing

Before mainnet deployment, ALWAYS test on testnets:

```bash
# BSC Testnet
cd bsc
npm run deploy:testnet
npm run test-swap:testnet

# Solana Devnet
cd solana
./deploy-solana.sh  # Select option 1 (Devnet)

# Test checklist
# [ ] Token deploys successfully
# [ ] Correct supply and decimals
# [ ] Transfer works
# [ ] Approval mechanism works
# [ ] Pool creation succeeds
# [ ] Swaps work both directions
# [ ] Price impact acceptable
```

## Maintenance

### Updating Token Metadata

```bash
# Solana
metaboss update uri -a <TOKEN_ADDRESS> -u <NEW_URI>

# Ethereum/BSC (if using ERC721 metadata)
# Requires contract upgrade or new deployment
```

### Adding Liquidity

```bash
# PancakeSwap
# Use web interface or router contract

# Raydium
# Use web interface or SDK
```

### Emergency Pause

If critical issue discovered:

1. Call emergency pause (if implemented)
2. Notify Foundation Directors
3. Submit 7-of-9 emergency action
4. Communicate to community
5. Deploy fix
6. Resume operations

## Roadmap

- [x] Phase 1: Priority DEXes (BSC, Solana, Ethereum, Polygon)
- [ ] Phase 2: Expansion (Avalanche, Arbitrum, Base, + 3 more)
- [ ] Phase 3: Full ecosystem (20+ DEXes)
- [ ] CEX listings (Gate.io, KuCoin, etc.)
- [ ] Cross-chain bridges (Wormhole, LayerZero)
- [ ] ËDSC stablecoin DEX listings
- [ ] Perpetuals support (dYdX, GMX)

## License

GPLv3 - See LICENSE file

## Contributors

- Eoj Edred (Founder)
- Ëtrid Foundation
- Community developers

---

**IMPORTANT:** All deployments require Foundation approval per FOUNDATION_CHARTER.md. Never deploy to mainnet without 6-of-9 Director signatures.

For the complete deployment strategy, see [COMPLETE_DEX_DEPLOYMENT_GUIDE.md](../COMPLETE_DEX_DEPLOYMENT_GUIDE.md).
