# EDSC (Ëtrid Dollar Stablecoin) - Multi-Chain Deployment

**Token:** EDSC - USD-pegged stablecoin
**Peg:** $1.00 USD
**Backing:** 150% collateralized with ËTR, sBTC, sETH
**Decimals:** 18 (all EVM chains), 9 (Solana)

---

## What is EDSC?

**EDSC** (Ëtrid Dollar Stablecoin) is Ëtrid's native USD-pegged stablecoin, designed for:
- ✅ Stable payments and transactions
- ✅ Trading pairs with low slippage
- ✅ Cross-chain transfers via Ëtrid bridge
- ✅ Over-collateralized reserve backing (150% minimum)

**Key Difference from ÉTR:**
- **ÉTR** = Volatile native token (like ETH, BNB)
- **EDSC** = Stable USD token (like USDC, DAI)

---

## Deployment Structure

```
edsc-stablecoin/
├── base/           EDSC on Base L2
├── arbitrum/       EDSC on Arbitrum
├── polygon/        EDSC on Polygon
├── bsc/            EDSC on BSC
├── ethereum/       EDSC on Ethereum
└── solana/         EDSC SPL token on Solana
```

Each chain folder contains:
- `EdscStablecoin.sol` - ERC-20 stablecoin contract
- `deploy-edsc.js` - Deployment script
- `hardhat.config.js` - Chain configuration
- `.env` - Private keys (same as ÉTR deployment)

---

## Stablecoin-Specific DEXes

EDSC will be deployed to **stablecoin-optimized DEXes**:

### Curve Finance (Ethereum, Polygon)
- Specialist in stablecoin swaps
- Ultra-low slippage (0.01-0.04%)
- EDSC/USDC/USDT/DAI pools

### Balancer (Polygon, Arbitrum)
- Stable pools with low fees
- Multi-token pools (EDSC/USDC/USDT)
- Composable stable pools

### PancakeSwap (BSC)
- StableSwap pools
- EDSC/BUSD/USDT pairs
- Low fees on BSC

### Uniswap V3 (All chains)
- Concentrated liquidity
- Narrow ranges ($0.99-$1.01)
- Efficient capital usage

### Raydium (Solana)
- Stable pool AMM
- EDSC/USDC on Solana
- High throughput, low fees

---

## Deployment Chains & Costs

| Chain | Cost | Stablecoin DEXes | Pairs |
|-------|------|------------------|-------|
| **Ethereum** | ~$150 | Curve, Uniswap V3 | EDSC/USDC/DAI |
| **Base** | ~$1 | Uniswap V3, Aerodrome | EDSC/USDC |
| **Arbitrum** | ~$1 | Curve, Balancer, Uniswap | EDSC/USDC/USDT |
| **Polygon** | ~$5 | Curve, Balancer, QuickSwap | EDSC/USDC/USDT |
| **BSC** | ~$6 | PancakeSwap StableSwap | EDSC/BUSD/USDT |
| **Solana** | ~$15 | Raydium Stable Pools | EDSC/USDC |

**Total (all 6 chains):** ~$28-$178 (depending on if you include Ethereum)

**Recommended:** Skip Ethereum ($150), deploy to other 5 chains ($28 total)

---

## Key Features

### 1. USD Peg Stability ($1.00)
- Backed by 150% collateral on FlareChain
- Reserve includes ËTR, sBTC, sETH
- Interest rate adjustments maintain peg
- Liquidation system ensures solvency

### 2. Cross-Chain Bridging
- Native bridge to FlareChain
- Burn on one chain, mint on another
- Attestation-based security (3-of-5 multisig)
- Sub-minute finality

### 3. Stablecoin Pools
- Paired with USDC, USDT, DAI
- StableSwap AMM curves (low slippage)
- Optimal for $0.99-$1.01 trading range
- Deep liquidity with minimal capital

### 4. Low Transaction Fees
- L2 chains (Base, Arbitrum): <$0.01
- Solana: ~$0.0001
- Polygon: ~$0.01
- BSC: ~$0.10

---

## Deployment vs ÉTR

### ÉTR Deployment (Native Token)
- Paired with: ETH, SOL, BNB (volatile assets)
- DEXes: Uniswap, PancakeSwap, Raydium (regular AMM)
- Price: Fluctuates based on market
- Use case: Governance, staking, speculation

### EDSC Deployment (Stablecoin)
- Paired with: USDC, USDT, DAI (stable assets)
- DEXes: Curve, Balancer, StableSwap AMMs
- Price: Stable at $1.00 USD
- Use case: Payments, trading, stability

**Both deploy together:** Full Ëtrid ecosystem!

---

## Deployment Order

### Phase 1: Deploy Contracts ($28-33)
Deploy EDSC token contracts to all chains:
```bash
# Base
cd base && npm run deploy:edsc

# Arbitrum
cd arbitrum && npm run deploy:edsc

# Polygon, BSC, etc.
```

### Phase 2: Create Stablecoin Pools ($200+)
Create EDSC liquidity pools with other stablecoins:

**Curve (Ethereum/Polygon):**
- EDSC/USDC/USDT/DAI pool
- Need: 100K EDSC + 100K USDC + 100K USDT

**Balancer (Polygon/Arbitrum):**
- EDSC/USDC stable pool
- Need: 100K EDSC + 100K USDC

**PancakeSwap (BSC):**
- EDSC/BUSD stable pool
- Need: 100K EDSC + 100K BUSD

**Raydium (Solana):**
- EDSC/USDC stable pool
- Need: 100K EDSC + 100K USDC

### Phase 3: Lock on FlareChain
For every EDSC minted on DEX chains, lock equivalent on FlareChain:
```
Mint 100K EDSC on Base → Lock 100K EDSC on FlareChain
Mint 100K EDSC on BSC → Lock 100K EDSC on FlareChain
```

Maintains 1:1 backing between chains.

---

## Requirements

### Gas Tokens (Same as ÉTR)
- Base: 0.001 ETH (~$3)
- Arbitrum: 0.001 ETH (~$3)
- Polygon: 10 MATIC (~$5)
- BSC: 0.02 BNB (~$6)
- Solana: 0.1 SOL (~$15)

### Stablecoin Liquidity (For pools)
- USDC: $500K-$1M total (across all chains)
- USDT: $200K-$500K (Ethereum, Polygon, BSC)
- DAI: $100K-$200K (Ethereum, Polygon)
- BUSD: $100K-$200K (BSC only)

**Initial minimum:** $50K USDC to start (distribute across chains)

---

## Contract Addresses (After Deployment)

Will be filled in after deployment:

```
Base:        0x... (EDSC)
Arbitrum:    0x... (EDSC)
Polygon:     0x... (EDSC)
BSC:         0x... (EDSC)
Ethereum:    0x... (EDSC)
Solana:      [EDSC mint address]
```

---

## Security Features

### 1. Collateral Backing
- 150% over-collateralization on FlareChain
- Real-time reserve monitoring
- Automatic rebalancing

### 2. Bridge Security
- 3-of-5 multisig attestation
- Time-delayed large transfers
- Circuit breaker for emergencies

### 3. Peg Stability
- Interest rate adjustments
- Liquidation mechanisms
- Emergency pause functionality

### 4. Ownership
- Governance multisig control
- Time-locked admin functions
- Transparent operations

---

## Quick Start

1. **Check gas tokens** (same addresses as ÉTR deployment)
2. **Deploy EDSC contracts:**
   ```bash
   cd /Users/macbook/Desktop/etrid/dex-deployment/edsc-stablecoin
   ./deploy-all-edsc.sh
   ```
3. **Create stablecoin pools** (after accumulating USDC/USDT)
4. **Lock on FlareChain** (maintain 1:1 backing)

---

## Monitoring & Verification

### After Deployment
- Verify contracts on block explorers
- Check token metadata (name, symbol, decimals)
- Confirm ownership transferred to multisig
- Test minting/burning functions

### Peg Monitoring
- Track EDSC price on DEXes
- Alert if deviation > 1% from $1.00
- Monitor reserve ratio on FlareChain
- Watch for liquidation events

---

## Documentation Files

- `DEPLOY_EDSC_GUIDE.md` - Step-by-step deployment
- `STABLECOIN_POOLS_GUIDE.md` - How to create stable pools
- `EDSC_VS_ETR_COMPARISON.md` - Differences explained
- Each chain folder has specific README

---

**Ready to deploy EDSC alongside ÉTR!** 🚀
