# ÉTR Token BSC Deployment

Deploy ÉTR (Ëtrid Coin) to Binance Smart Chain for PancakeSwap listing.

## Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Configure Environment

Copy `.env.example` to `.env` and fill in:

```bash
cp .env.example .env
nano .env  # Add your PRIVATE_KEY and BSCSCAN_API_KEY
```

**CRITICAL:** Your private key must have:
- At least 0.05 BNB for gas fees
- Access to Foundation multisig for minting

### 3. Deploy to Testnet (Recommended First)

```bash
npm run deploy:testnet
```

This deploys to BSC Testnet (ChainId: 97) for testing.

### 4. Deploy to Mainnet

After testing successfully:

```bash
npm run deploy:mainnet
```

This deploys to BSC Mainnet (ChainId: 56).

### 5. Verify Contract

Automatic verification runs after deployment if `BSCSCAN_API_KEY` is set.

Manual verification:

```bash
npm run verify:mainnet <TOKEN_ADDRESS>
```

## Token Specifications

- **Name:** Etrid Coin
- **Symbol:** ETR
- **Decimals:** 18 (BSC standard)
- **Initial Supply:** 100,000,000 ÉTR
- **Max Supply:** 1,000,000,000 ÉTR
- **Contract:** BEP-20 (OpenZeppelin)

## PancakeSwap Integration

After deployment, create liquidity pool:

1. Go to [PancakeSwap Liquidity](https://pancakeswap.finance/liquidity)
2. Click "Add Liquidity"
3. Select tokens:
   - Token A: **ÉTR** (your deployed address)
   - Token B: **WBNB** (0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c)
4. Choose fee tier: **0.25%**
5. Set price range (full range for initial deployment)
6. Add liquidity:
   - 25,000,000 ÉTR
   - ~3,333 BNB (~$2M @ $600/BNB)

## Security

- Contract uses OpenZeppelin audited code
- Ownership controlled by Foundation 6-of-9 multisig
- Bridge minting/burning only by authorized bridge contract
- Max supply cap enforced on-chain

## Governance

Per **FOUNDATION_CHARTER.md**:

- Deployment requires 6-of-9 Director signatures
- Liquidity additions from Community LP Pool (250M ÉTR allocation)
- Emergency actions require 7-of-9 signatures

## Support

- Technical: dev@etrid.org
- Foundation: directors@etrid.org
- Discord: #dex-deployment channel

## References

- [Foundation Charter](../../FOUNDATION_CHARTER.md)
- [Protocol Charter](../../docs/specifications/protocol-charter.md)
- [Complete DEX Deployment Guide](../../COMPLETE_DEX_DEPLOYMENT_GUIDE.md)
