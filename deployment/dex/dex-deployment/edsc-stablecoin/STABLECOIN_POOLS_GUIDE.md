# Creating Stablecoin Pools for EDSC

**Purpose:** Create low-slippage trading pools for EDSC paired with other stablecoins (USDC, USDT, DAI)

---

## Why Stablecoin Pools Are Different

### Regular AMM Pools (ÉTR)
- **Curve:** xy = k (constant product)
- **Pairs:** Volatile assets (ÉTR/ETH, ÉTR/SOL)
- **Price range:** 0 to infinity
- **Slippage:** High (2-5%) for large trades

### Stablecoin Pools (EDSC)
- **Curve:** StableSwap (optimized for $0.99-$1.01)
- **Pairs:** Stable assets (EDSC/USDC, EDSC/USDT)
- **Price range:** Narrow ($0.99-$1.01)
- **Slippage:** Ultra-low (0.01-0.1%) for large trades

---

## Pool Creation by Chain

### 1. Curve Finance (Ethereum & Polygon)

**Best for:** Maximum liquidity, lowest slippage
**Fee:** 0.04%
**Supported chains:** Ethereum, Polygon, Arbitrum

**Steps:**
1. Visit: https://curve.fi/#/ethereum/create-pool
2. Select "Stableswap" pool type
3. Add tokens:
   - Token 1: EDSC (your deployed contract)
   - Token 2: USDC
   - Token 3: USDT (optional)
   - Token 4: DAI (optional)
4. Set amplification coefficient:
   - **A = 200** (recommended for stablecoins)
   - Higher A = tighter peg, lower slippage
5. Set fee: **0.04%** (Curve standard)
6. Add initial liquidity:
   - Minimum: $50K each token
   - Recommended: $200K+ each token
7. Confirm transaction

**Cost:** ~$50-100 gas on Ethereum, ~$1-2 on Polygon

---

### 2. Balancer (Polygon, Arbitrum, Base)

**Best for:** Composable pools, flexible weighting
**Fee:** 0.01-0.10% (configurable)
**Supported chains:** Polygon, Arbitrum, Base

**Steps:**
1. Visit: https://app.balancer.fi/#/polygon/pool/create
2. Select "Composable Stable Pool"
3. Add tokens:
   - EDSC: 50% weight
   - USDC: 50% weight
   - (Or 33% each for 3-token pool)
4. Set amplification: **A = 200-500**
5. Set swap fee: **0.01%** (ultra-low for stables)
6. Add liquidity:
   - Minimum: $50K total
   - Recommended: $200K+ total
7. Create pool

**Cost:** ~$1-5 gas on Polygon/Arbitrum

---

### 3. PancakeSwap StableSwap (BSC)

**Best for:** BSC ecosystem, low fees
**Fee:** 0.01%
**Chain:** BSC only

**Steps:**
1. Visit: https://pancakeswap.finance/add
2. Select "Stable Pool" mode
3. Add tokens:
   - EDSC (your deployed address)
   - BUSD or USDT
4. Set initial liquidity:
   - Minimum: $25K each token
   - Recommended: $100K+ each token
5. Confirm on PancakeSwap
6. Approve tokens
7. Add liquidity

**Cost:** ~$0.50-1 BNB (~$3-6)

---

### 4. Uniswap V3 (All EVM Chains)

**Best for:** Capital efficiency, all chains
**Fee:** 0.01% or 0.05%
**Chains:** Ethereum, Base, Arbitrum, Polygon, BSC

**Steps:**
1. Visit: https://app.uniswap.org/#/add
2. Select network
3. Add tokens:
   - EDSC (import custom token)
   - USDC
4. Select fee tier: **0.01%** (best for stables)
5. **Set narrow range:**
   - Min price: **$0.98** EDSC/USDC
   - Max price: **$1.02** EDSC/USDC
   - This concentrates liquidity at peg
6. Add liquidity amount:
   - Minimum: $25K each side
   - Recommended: $100K+ each side
7. Approve and supply

**Cost:** $1-5 on L2s, $50-100 on Ethereum

**Important:** Rebalance frequently! If EDSC moves outside $0.98-$1.02, your position goes inactive.

---

### 5. Raydium Stable Pool (Solana)

**Best for:** Solana ecosystem, high throughput
**Fee:** 0.01%
**Chain:** Solana

**Steps:**
1. Visit: https://raydium.io/liquidity/create/
2. Connect Phantom wallet
3. Select "Stable Pool" (NOT standard pool!)
4. Add tokens:
   - EDSC (paste your mint address)
   - USDC: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
5. Set initial price: **~$1.00**
6. Add liquidity:
   - Minimum: 50K EDSC + 50K USDC
   - Recommended: 200K+ each
7. Set fee: **0.01%**
8. Create pool

**Cost:** ~0.5 SOL (~$75)

---

## Liquidity Requirements

### Minimum to Start (Per Chain)

| Chain | Minimum EDSC | Minimum Stables | Total Value |
|-------|--------------|-----------------|-------------|
| Ethereum | 100K | 100K USDC | $200K |
| Base | 50K | 50K USDC | $100K |
| Arbitrum | 50K | 50K USDC | $100K |
| Polygon | 50K | 50K USDC | $100K |
| BSC | 50K | 50K BUSD | $100K |
| Solana | 50K | 50K USDC | $100K |

**Total minimum across all chains:** $700K ($350K EDSC + $350K stables)

### Recommended for Good Liquidity

| Chain | EDSC | Stables | Total Value |
|-------|------|---------|-------------|
| Ethereum | 500K | 500K | $1M |
| All Others | 200K each | 200K each | $400K each |

**Total recommended:** ~$3M across all chains

---

## Pool Configuration Best Practices

### 1. Amplification Factor (A)
- **A = 100:** Loose peg, higher slippage (not recommended)
- **A = 200:** Standard for stablecoins (RECOMMENDED)
- **A = 500:** Very tight peg, lowest slippage (advanced)
- **A = 1000+:** Ultra-tight, requires active management

**Use A = 200** unless you have advanced pool management

### 2. Fee Tiers
- **0.01%:** Best for stablecoins with deep liquidity
- **0.04%:** Curve default, good balance
- **0.05-0.10%:** Higher fees for lower liquidity

**Use 0.01-0.04%** for EDSC pools

### 3. Price Ranges (Uniswap V3 only)
- **Narrow ($0.98-$1.02):** Maximum capital efficiency, requires rebalancing
- **Medium ($0.95-$1.05):** Less rebalancing, lower efficiency
- **Wide ($0.90-$1.10):** Rarely needs rebalancing, lowest efficiency

**Use $0.98-$1.02** and rebalance weekly

---

## After Pool Creation

### 1. Verify Pool is Live
- Check DEX interface shows your pool
- Verify liquidity displays correctly
- Test small swap (1 EDSC for USDC)

### 2. Submit to Aggregators
**Jupiter (Solana):**
- https://station.jup.ag/token-list
- Automatic indexing within 1-2 hours

**1inch (EVM chains):**
- https://github.com/1inch/token-lists
- Submit PR with your token addresses

**CoinGecko:**
- https://www.coingecko.com/en/coins/new
- Add EDSC with all pool addresses

### 3. Monitor Peg Stability
Check daily:
- EDSC price on each DEX
- Alert if deviation > 1% from $1.00
- Rebalance if needed

### 4. Lock on FlareChain
**Critical:** Maintain 1:1 backing!

For each pool created:
```
Created 100K EDSC pool on Base → Lock 100K EDSC on FlareChain
Created 100K EDSC pool on BSC → Lock 100K EDSC on FlareChain
Total minted: 200K → Total locked: 200K
```

---

## Troubleshooting

**Q: Pool creation fails?**
A: Check:
- Sufficient gas tokens
- Token approvals granted
- Liquidity amounts balanced
- Contract not paused

**Q: High slippage in stable pool?**
A:
- Increase liquidity (2-5x current amount)
- Check amplification factor (should be 200+)
- Verify it's a stable pool, not regular AMM

**Q: EDSC trading off peg?**
A:
- Increase interest rate on FlareChain (for <$0.99)
- Decrease interest rate (for >$1.01)
- Add more liquidity to pools
- Check reserve ratio on FlareChain (must be >150%)

**Q: Uniswap V3 position went inactive?**
A:
- Price moved outside your range
- Rebalance position to current price
- Consider wider range or use stable pool DEX

---

## Pool Management Tips

### Weekly Tasks
- [ ] Check EDSC price on all DEXes
- [ ] Rebalance Uniswap V3 positions if needed
- [ ] Monitor pool TVL and volume
- [ ] Collect LP fees

### Monthly Tasks
- [ ] Review amplification factor performance
- [ ] Analyze fee tier effectiveness
- [ ] Compare to other stablecoin pools (USDC/USDT)
- [ ] Adjust liquidity allocation across chains

### When to Rebalance
- EDSC consistently off peg (>1% deviation)
- Pool slippage increases significantly
- Competing pools have better rates
- Major liquidity added/removed

---

## Cost Summary

| Chain | Pool Type | Creation Cost | Min Liquidity |
|-------|-----------|---------------|---------------|
| Ethereum | Curve | ~$50-100 | $200K |
| Polygon | Balancer | ~$1-2 | $100K |
| Arbitrum | Balancer | ~$1-2 | $100K |
| Base | Uniswap V3 | ~$1-2 | $100K |
| BSC | PancakeSwap | ~$3-6 | $100K |
| Solana | Raydium | ~$75 | $100K |

**Total cost:** ~$130-180 to create all pools
**Total liquidity needed:** $700K minimum, $3M recommended

---

## Ready to Create Pools?

1. **Deploy EDSC contracts first** (if not done)
2. **Acquire stablecoins** (USDC, USDT, etc.)
3. **Choose pool type** (Curve for Ethereum, Balancer for Polygon, etc.)
4. **Follow chain-specific guide above**
5. **Lock equivalent on FlareChain**
6. **Submit to aggregators**
7. **Monitor and manage**

**Need help?** See `README.md` in edsc-stablecoin folder for full deployment guide.
