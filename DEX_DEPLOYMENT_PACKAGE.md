# ÉTR Token - DEX Deployment Package

**Date Created:** October 28, 2025
**Status:** 🚀 Ready for DEX Deployment
**Purpose:** Complete token information for listing on decentralized exchanges

---

## 📋 Quick Reference - Token Specifications

### Primary Token: ÉTR (Ëtrid Coin)

| Specification | Value |
|---------------|-------|
| **Token Name** | Ëtrid Coin |
| **Token Symbol** | ÉTR |
| **Type** | Native blockchain token (Substrate-based) |
| **Total Supply** | 1,000,000,000 ÉTR (1 billion) |
| **Circulating Supply** | TBD (based on vesting schedule) |
| **Decimals (Native)** | 5 (100,000 atomic units = 1 ÉTR) |
| **Decimals (ERC-20/BEP-20)** | 18 (standard for EVM chains) |
| **Decimals (Solana SPL)** | 9 (standard for Solana) |
| **Atomic Unit Name** | Bitë (smallest unit = 0.00001 ÉTR) |
| **Supply Cap** | Fixed at 1 billion (hard-capped) |
| **Inflation** | None (fixed supply) |
| **Burn Mechanism** | Yes (for cross-chain bridging) |

---

## 🏗️ Token Architecture

### Native Chain: FlareChain (Substrate)

**Chain Details:**
- Framework: Substrate (Polkadot SDK)
- Consensus: ASF (Adaptive Streaming Finality)
- Block Time: ~3 seconds
- Finality: ~6 seconds
- RPC Endpoint: `wss://rpc.etrid.org` (mainnet)
- Testnet RPC: `wss://ember-rpc.etrid.org`

**Token Implementation:**
- Location: `06-native-currency/etr-token/src/lib.rs`
- Pallet: `pallet-etrid-coin`
- Type: Native runtime balance
- Storage: Blake2_128Concat map (AccountId → Balance)

---

## 💰 Token Economics (Tokenomics)

### Supply Distribution

| Allocation | Amount | Percentage | Purpose |
|------------|--------|------------|---------|
| **Community LP Pool** | 250,000,000 ÉTR | 25% | DEX liquidity + LP rewards |
| **Development Fund** | 150,000,000 ÉTR | 15% | Core development, audits |
| **Foundation Treasury** | 200,000,000 ÉTR | 20% | Operations, grants, partnerships |
| **Team & Advisors** | 100,000,000 ÉTR | 10% | Vested over 3-4 years |
| **Ecosystem Incentives** | 150,000,000 ÉTR | 15% | Staking rewards, validator incentives |
| **Public Sale/Airdrop** | 100,000,000 ÉTR | 10% | Community distribution |
| **Reserve** | 50,000,000 ÉTR | 5% | Emergency fund |

### Community LP Pool Breakdown (250M ÉTR)

| Allocation | Amount | Purpose |
|------------|--------|---------|
| **Initial Liquidity** | 100,000,000 ÉTR | Seed liquidity on BSC + Solana + Ethereum |
| **LP Rewards (3 years)** | 150,000,000 ÉTR | Incentivize liquidity providers |

**LP Reward Schedule:**
- Year 1: 75,000,000 ÉTR (50% of rewards) → ~205,479 ÉTR/day
- Year 2: 45,000,000 ÉTR (30% of rewards) → ~123,288 ÉTR/day
- Year 3: 30,000,000 ÉTR (20% of rewards) → ~82,192 ÉTR/day

---

## 🌐 Multi-Chain Deployment Strategy

### Phase 1: Initial DEX Listings (Months 0-2)

**Target Liquidity:** $3M
**Expected Volume:** $250k/day
**Target Holders:** 2,000+

| DEX | Chain | Status | Initial Liquidity |
|-----|-------|--------|-------------------|
| **PancakeSwap V3** | BSC (BEP-20) | 🟡 Ready | 50M ÉTR + equivalent BNB |
| **Raydium CLMM** | Solana (SPL) | 🟡 Ready | 50M ÉTR + equivalent SOL |
| **Uniswap V3** | Ethereum | ✅ Deployed | 25M ÉTR + equivalent ETH |
| **Base L2** | Base (Coinbase L2) | 🔄 In Progress | 25M ÉTR + equivalent ETH |

### Phase 2: DEX Expansion (Months 2-4)

**Target Liquidity:** $10M
**Expected Volume:** $1M/day
**Target Holders:** 5,000+

| DEX | Chain | Priority |
|-----|-------|----------|
| **SushiSwap** | Ethereum + Arbitrum | High |
| **Curve** | Ethereum (stablecoin pools) | High |
| **Trader Joe** | Avalanche | Medium |
| **Hyperliquid** | Ethereum (hybrid DEX) | Medium |

---

## 📦 Token Contract Addresses

### BSC (Binance Smart Chain) - BEP-20

**Contract Address:** `0x[TO_BE_DEPLOYED]`

**Contract Details:**
- Standard: BEP-20 (ERC-20 compatible)
- Name: Ëtrid Coin
- Symbol: ÉTR
- Decimals: 18
- Features:
  - ✅ Mintable (bridge-controlled only)
  - ✅ Burnable (for bridging back)
  - ✅ Pausable (emergency circuit breaker)
  - ✅ Role-based access control

**Contract Source:** `05-multichain/bridge/adapters/bsc/contracts/EtridToken.sol`

**BSCScan Verification:** Required before listing

**Bridge Integration:**
- Bridge Pallet: `pallet-bnb-bridge`
- Mint Authority: Bridge relayer multisig
- Burn Authority: Any holder (for bridging back to FlareChain)

---

### Solana - SPL Token

**Mint Address:** `[TO_BE_CREATED]`

**Token Details:**
- Standard: SPL (Solana Program Library)
- Name: Ëtrid Coin
- Symbol: ÉTR
- Decimals: 9
- Features:
  - ✅ Mint authority: Foundation multisig
  - ✅ Freeze authority: Foundation multisig (security)
  - ✅ Metaplex metadata enabled

**Metadata URI:** `https://etrid.org/token-metadata.json`

**Required Metadata JSON:**
```json
{
  "name": "Ëtrid Coin",
  "symbol": "ÉTR",
  "description": "Native token of the Ëtrid blockchain - a multichain ecosystem bridging 13+ blockchains with adaptive streaming finality consensus",
  "image": "https://etrid.org/assets/logo-512.png",
  "external_url": "https://etrid.org",
  "properties": {
    "category": "fungible-token",
    "files": [
      {
        "uri": "https://etrid.org/assets/logo-512.png",
        "type": "image/png"
      }
    ]
  }
}
```

---

### Ethereum - ERC-20

**Contract Address:** `0x[UNISWAP_DEPLOYED]`

**Contract Details:**
- Standard: ERC-20
- Name: Ëtrid Coin
- Symbol: ÉTR
- Decimals: 18

**Bridge:** ETH-PBC (Partition Burst Chain)

---

## 🔐 Security & Audits

### Smart Contract Audits

**Status:** Recommended before mainnet deployment

**Audit Scope:**
1. BSC token contract (`EtridToken.sol`)
2. Bridge adapters (BSC, Solana, Ethereum)
3. Multi-sig custody contracts

**Recommended Auditors:**
- CertiK (estimate: $25k-40k, 2-3 weeks)
- Quantstamp (estimate: $30k-50k, 3-4 weeks)
- OpenZeppelin (estimate: $40k-60k, 4-6 weeks)

**Audit Requirements for CEX Listings:**
- Gate.io: Preferred but not mandatory
- KuCoin: Recommended
- Binance: **Required**
- Coinbase: **Required**

### Multi-Sig Configuration

**Bridge Custodians:** 5/7 multi-sig (requires 5 of 7 signatures)

**Key Holders:**
- Foundation treasury (1 key)
- Core team members (4 keys)
- Independent validators (2 keys)

**Multi-Sig Addresses:**
- BSC: `[TO_BE_CONFIGURED]`
- Ethereum: `[TO_BE_CONFIGURED]`
- Solana: `[TO_BE_CONFIGURED]`

---

## 📊 Initial Liquidity Setup

### PancakeSwap V3 (BSC)

**Pair:** ÉTR/BNB
**Fee Tier:** 0.25% (recommended for new tokens)
**Price Range:** Full range initially

**Initial Liquidity Allocation:**
```
Target Initial Price: $0.10 per ÉTR
BNB Price: ~$300 (market dependent)

Option A (Conservative):
- 25,000,000 ÉTR
- 8,333 BNB (~$2.5M total liquidity)

Option B (Aggressive):
- 50,000,000 ÉTR
- 16,667 BNB (~$5M total liquidity)
```

**Deployment Steps:**
1. Deploy `EtridToken.sol` to BSC
2. Verify contract on BSCScan
3. Transfer 50M ÉTR to deployer wallet
4. Approve PancakeSwap V3 Position Manager
5. Create ÉTR/WBNB pool with 0.25% fee
6. Add liquidity (full range)
7. Lock LP NFT in Gnosis Safe (6-12 months)

**PancakeSwap Contract:**
- Position Manager: `0x46A15B0b27311cedF172AB29E4f4766fbE7F4364`

---

### Raydium CLMM (Solana)

**Pair:** ÉTR/SOL
**Fee Tier:** 0.25%
**Price Range:** Full range initially

**Initial Liquidity Allocation:**
```
Target Initial Price: $0.10 per ÉTR
SOL Price: ~$150 (market dependent)

Option A (Conservative):
- 25,000,000 ÉTR
- 16,667 SOL (~$2.5M total liquidity)

Option B (Aggressive):
- 50,000,000 ÉTR
- 33,333 SOL (~$5M total liquidity)
```

**Deployment Steps:**
1. Create SPL token mint with 9 decimals
2. Create metadata account (Metaplex)
3. Mint 50M ÉTR to deployer account
4. Create Raydium CLMM pool (ÉTR/WSOL)
5. Add liquidity
6. Set mint authority to Foundation multisig
7. Lock LP position (6-12 months)

---

## 🎯 Price Discovery Strategy

### Initial Pricing

**Target Launch Price:** $0.08 - $0.12 per ÉTR

**Rationale:**
- Market cap at $0.10: $100M (fully diluted)
- Circulating market cap: $25M-50M (depending on vesting)
- Comparable to similar L1/L2 blockchain launches

**Price Stabilization Mechanisms:**
1. **Deep liquidity pools:** $2.5M-5M per chain
2. **LP lock periods:** 6-12 months (prevents rug pulls)
3. **Market makers:** Optional (1-2% of supply for MM operations)
4. **Gradual vesting:** Team/advisor tokens vest over 3-4 years

### Market Making (Optional)

**Purpose:** Reduce spread, improve liquidity, stabilize price

**Allocation:** 10M-20M ÉTR (1-2% of supply)

**Potential Market Makers:**
- Wintermute (recommended for CEX integration)
- GSR Markets
- Kronos Research
- Amber Group

**MM Terms (typical):**
- Fee: 5-10% of allocated tokens
- Duration: 12-24 months
- Spread targets: 0.5-1% on each side
- Minimum uptime: 95%

---

## 📈 Exchange Listing Roadmap

### DEX Listings (Permissionless - No Fees)

**Phase 1 (Months 0-2):**
- ✅ Uniswap V3 (Ethereum)
- 🟡 PancakeSwap V3 (BSC)
- 🟡 Raydium CLMM (Solana)
- 🔄 Base DEX (Base L2)

**Phase 2 (Months 2-4):**
- SushiSwap (Multi-chain)
- Curve (Ethereum - stablecoin pairs)
- Trader Joe (Avalanche)
- Hyperliquid (Hybrid DEX/CEX)

**Estimated Cost:** $50k-100k (gas fees, audits, liquidity deployment)

---

### CEX Listings (Centralized - Requires Fees + KYC)

**Phase 3 (Months 4-6):**
- **Gate.io** (listing fee: ~$100k-150k)
- **KuCoin** (listing fee: ~$100k-150k)

**Requirements:**
- ✅ 3-6 months trading history on DEXs
- ✅ $1M+ daily volume
- ✅ 10,000+ holders
- ✅ Legal entity (foundation registered)
- ✅ AML/KYC compliance documents

**Phase 4 (Months 6-12):**
- **OKX** (listing fee: ~$300k-500k)
- **Binance** (listing fee: ~$500k-1M or equity stake)

**Requirements:**
- ✅ $5M+ daily volume
- ✅ 100,000+ holders
- ✅ Security audit (mandatory for Binance)
- ✅ $100M+ market cap
- ✅ Legal opinion letter

**Phase 5 (Months 12-18):**
- **Coinbase** (no listing fee, but strict requirements)
- **Kraken**
- **Gemini** (US-focused)

**Requirements:**
- ✅ $250M+ market cap
- ✅ Legal clarity (especially for US)
- ✅ Security audit
- ✅ Strong community + institutional interest

---

## 🔗 Official Resources

### Project Information

| Resource | URL |
|----------|-----|
| **Official Website** | https://etrid.org |
| **Documentation** | https://docs.etrid.org |
| **GitHub Repository** | https://github.com/EojEdred/Etrid |
| **Whitepaper** | https://etrid.org/whitepaper.pdf |
| **Tokenomics** | https://etrid.org/tokenomics |
| **Block Explorer** | https://explorer.etrid.org |

### Social Media

| Platform | Handle/URL |
|----------|------------|
| **Twitter (X)** | https://x.com/etrid |
| **Discord** | https://discord.gg/etrid |
| **Reddit** | https://reddit.com/r/etrid |
| **Telegram** | https://t.me/etridofficial |
| **Medium** | https://medium.com/@etrid |

### Contact

| Contact Type | Email |
|--------------|-------|
| **General** | hello@etrid.org |
| **Foundation** | etridfoundation@proton.me |
| **Partnerships** | partnerships@etrid.org |
| **Support** | support@etrid.org |

---

## 🖼️ Brand Assets

### Logo Files

**Download:** https://etrid.org/brand-assets

**Required Sizes:**
- 512x512px (PNG, transparent background)
- 256x256px (PNG, transparent background)
- 128x128px (PNG, transparent background)
- SVG (vector, transparent background)

**Color Schemes:**
- Primary: `#[HEX_CODE]` (update with official brand color)
- Secondary: `#[HEX_CODE]`
- Background (dark mode): `#[HEX_CODE]`

**Logo Usage Guidelines:**
- Minimum size: 32x32px
- Clear space: 10% of logo width on all sides
- Do not distort or skew
- Do not add effects (shadows, gradients)

---

## ✅ Pre-Deployment Checklist

### Technical Readiness

- [ ] BSC token contract deployed and verified
- [ ] Solana SPL token created with metadata
- [ ] Bridge adapters tested (testnet)
- [ ] Multi-sig wallets configured (5/7)
- [ ] RPC endpoints stable (mainnet + testnet)
- [ ] Block explorer operational
- [ ] Token contracts audited (recommended)

### Liquidity Readiness

- [ ] 50M ÉTR allocated for BSC liquidity
- [ ] 50M ÉTR allocated for Solana liquidity
- [ ] BNB acquired for PancakeSwap pool (~8,333-16,667 BNB)
- [ ] SOL acquired for Raydium pool (~16,667-33,333 SOL)
- [ ] Gnosis Safe configured for LP lock
- [ ] LP reward distribution contract deployed

### Compliance & Documentation

- [ ] Foundation legal entity registered
- [ ] AML/KYC compliance documents prepared
- [ ] Terms of service + privacy policy published
- [ ] Community announcement drafted
- [ ] Exchange application materials ready
- [ ] Brand assets package finalized

### Marketing & Community

- [ ] Website live with token information
- [ ] Social media accounts active
- [ ] Discord/Telegram community established
- [ ] CoinGecko application submitted
- [ ] CoinMarketCap application submitted
- [ ] Medium article published (launch announcement)

---

## 📞 Next Steps

### Immediate Actions (This Week)

1. **Review this document** with core team
2. **Approve liquidity allocation** (100M ÉTR from Community LP Pool)
3. **Acquire BNB + SOL** for initial pools (~$5M total)
4. **Deploy BSC contract** (testnet first, then mainnet)
5. **Create Solana token** (devnet first, then mainnet)

### Short-Term (Next 2-4 Weeks)

6. **Launch PancakeSwap pool** (BSC mainnet)
7. **Launch Raydium pool** (Solana mainnet)
8. **Lock LP tokens** in Gnosis Safe (6-12 months)
9. **Submit CoinGecko + CMC applications**
10. **Publish community announcement**

### Medium-Term (Next 2-6 Months)

11. **Monitor liquidity + volume metrics**
12. **Apply to Gate.io + KuCoin** (when metrics hit Phase 3 targets)
13. **Expand to additional DEXs** (SushiSwap, Curve, etc.)
14. **Engage market makers** (if needed for CEX listings)
15. **Prepare for top-tier CEX applications** (Binance, Coinbase)

---

## 📄 Supporting Documents

### Internal Documents
- [06-native-currency/README.md](06-native-currency/README.md) - Token architecture
- [06-native-currency/ARCHITECTURE.md](06-native-currency/ARCHITECTURE.md) - Technical specs
- [14-aidevs/DEX_DEPLOYMENT_GUIDE.md](14-aidevs/DEX_DEPLOYMENT_GUIDE.md) - Detailed deployment steps
- [docs/EXCHANGE_EXPANSION_MASTER_PLAN.md](docs/EXCHANGE_EXPANSION_MASTER_PLAN.md) - Full 18-month strategy
- [docs/EXCHANGE_EXPANSION_QUICK_START.md](docs/EXCHANGE_EXPANSION_QUICK_START.md) - Quick overview

### External Applications
- CoinGecko application: https://www.coingecko.com/en/coins/new
- CoinMarketCap application: https://support.coinmarketcap.com/hc/en-us/articles/360043659351
- Gate.io listing: partnerships@gate.io
- KuCoin listing: listing@kucoin.com

---

**Document Version:** 1.0
**Last Updated:** October 28, 2025
**Maintained By:** Ëtrid Foundation
**Contact:** etridfoundation@proton.me

---

## 🚀 Ready to Deploy?

This package contains everything needed to list ÉTR on decentralized exchanges. Follow the deployment guides in `14-aidevs/DEX_DEPLOYMENT_GUIDE.md` for step-by-step instructions.

**Questions?** Contact the team on Discord (#dex-deployment) or email partnerships@etrid.org
