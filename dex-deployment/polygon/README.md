# ÉTR Token Polygon Deployment

Deploy ÉTR (Ëtrid Coin) to Polygon PoS for QuickSwap V3 listing.

## Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Configure Environment

```bash
cp .env.example .env
nano .env  # Add your PRIVATE_KEY and POLYGONSCAN_API_KEY
```

### 3. Deploy to Testnet (Recommended First)

```bash
# Make sure you have Amoy testnet MATIC first
npm run deploy:testnet
```

### 4. Deploy to Mainnet

```bash
# Make sure you have at least 10 MATIC
npm run deploy:mainnet
```

## Requirements

### Gas Fees
- **Testnet (Amoy):** ~0.1 MATIC (free from faucet)
- **Mainnet:** ~1-10 MATIC (~$1-10)

Get Amoy MATIC: https://faucet.polygon.technology/

### Liquidity for QuickSwap
- 15,000,000 ÉTR
- ~1,000,000 MATIC (~$1M at $1/MATIC)
- Source: Community LP Pool (per Foundation Charter)

## Why Polygon?

**Advantages:**
- **Ultra-low fees:** ~$0.01 per transaction (vs $50+ on Ethereum)
- **Fast finality:** 2-second blocks
- **Ethereum compatibility:** Same tooling, wallets, standards
- **Large DeFi ecosystem:** QuickSwap, Aave, Curve, etc.
- **Great for testing:** Deploy and test cheaply before Ethereum

## Contract Details

**Contract:** EtridPoly.sol
- **Name:** Ëtrid Coin
- **Symbol:** ÉTR
- **Decimals:** 18 (Polygon/EVM standard)
- **Initial Supply:** 15M ÉTR (for QuickSwap liquidity)
- **Max Supply:** 1B ÉTR (bridge enforced)

**Features:**
- ERC-20 compliant (Polygon is EVM compatible)
- Polygon PoS Bridge support (Ethereum ↔ Polygon)
- Cross-chain bridge support (other chains)
- Emergency pause capability
- Foundation multisig controlled
- OpenZeppelin security standards

## Deployment Steps

### Step 1: Deploy Token

```bash
npm run deploy:mainnet
```

This will:
1. Deploy EtridPoly contract
2. Mint 15M ÉTR to deployer
3. Auto-verify on PolygonScan
4. Save deployment info to `deployments/`

### Step 2: Transfer Ownership

```javascript
// Connect to deployed contract
const token = await ethers.getContractAt("EtridPoly", "<TOKEN_ADDRESS>");

// Transfer to Foundation multisig
await token.transferOwnership("<FOUNDATION_MULTISIG_ADDRESS>");
```

### Step 3: Create QuickSwap V3 Pool

Go to: https://quickswap.exchange/#/pools

1. Click **"Create Pool"**
2. **Token 0:** ÉTR (paste deployed address)
3. **Token 1:** WMATIC (0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270)
4. **Fee tier:** 0.30% (3000)
5. **Initial price:** Set based on BSC/Ethereum price

### Step 4: Add Liquidity

1. Amount: 15M ÉTR + 1M MATIC
2. Price range: Full range or concentrated
3. Approve token spending (costs ~$0.01)
4. Confirm transaction (costs ~$0.01)
5. Receive LP NFT position

### Step 5: Configure Bridges

```javascript
// Set Polygon PoS Bridge (for Ethereum ↔ Polygon)
await token.setPolygonBridge("0xA0c68C638235ee32657e8f720a23ceC1bFc77C77");

// Set cross-chain bridge (for other chains)
await token.setCrossChainBridge("<BRIDGE_ADDRESS>");
```

### Step 6: Submit to Tracking Sites

**Update existing listings with Polygon address:**

**CoinGecko:**
- Edit existing ÉTR listing
- Add Polygon contract address
- URL: https://www.coingecko.com/en/coins/etrid-coin

**CoinMarketCap:**
- Edit existing ÉTR listing
- Add Polygon contract address
- URL: https://coinmarketcap.com/currencies/etrid-coin/

## Important Addresses

### Polygon Mainnet
- **WMATIC:** 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270
- **USDC:** 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174
- **QuickSwap V3 Factory:** 0x411b0fAcC3489691f28ad58c47006AF5E3Ab3A28
- **QuickSwap Router:** 0xf5b509bB0909a69B1c207E495f687a596C168E12
- **Polygon PoS Bridge:** 0xA0c68C638235ee32657e8f720a23ceC1bFc77C77

### Polygon Amoy Testnet
- **WMATIC:** 0x360ad4f9a9A8EFe9A8DCB5f461c4Cc1047E1Dcf9
- **QuickSwap (check latest docs)**

## Testing Checklist

Before mainnet deployment:

- [ ] Deploy to Amoy testnet
- [ ] Create test pool on Amoy
- [ ] Add test liquidity (very cheap!)
- [ ] Perform test swaps (ÉTR → WMATIC and WMATIC → ÉTR)
- [ ] Verify contract on PolygonScan
- [ ] Test ownership transfer
- [ ] Test bridge functions (both Polygon and cross-chain)
- [ ] Get Foundation approval (6-of-9 signatures)

## Verification

If auto-verification fails, verify manually:

```bash
npx hardhat verify --network polygon <TOKEN_ADDRESS> <DEPLOYER_ADDRESS>
```

## Cost Breakdown

| Item | Cost (MATIC) | Cost (USD @ $1) |
|------|-------------|-----------------|
| Contract deployment | 1-5 | $1-5 |
| Pool creation | 0.5-1 | $0.50-1 |
| Add liquidity | 0.5-1 | $0.50-1 |
| **Total Gas** | **~3-10** | **~$3-10** |
| **Liquidity** | **1M** | **$1M** |
| **Grand Total** | **~1M** | **~$1M** |

**Polygon is ~100x cheaper than Ethereum!**

## Bridging Between Chains

### Ethereum → Polygon (via Polygon PoS Bridge)

1. Lock ÉTR on Ethereum
2. Wait ~7-8 minutes (checkpoint)
3. Mint ÉTR on Polygon via bridge

### Polygon → Ethereum (via Polygon PoS Bridge)

1. Burn ÉTR on Polygon
2. Wait ~30-45 minutes (checkpoint)
3. Unlock ÉTR on Ethereum

### Other Chains (via Wormhole/LayerZero)

- Near-instant bridging
- Cross-chain liquidity aggregation
- Unified ÉTR experience

## Security

- **Multisig Control:** All admin functions require Foundation 6-of-9 approval
- **OpenZeppelin Contracts:** Battle-tested, audited code
- **Max Supply Cap:** Hard limit at 1B ÉTR across all chains
- **Dual Bridge Support:** Polygon PoS Bridge + Cross-chain bridge
- **Emergency Pause:** 7-of-9 Foundation approval required
- **Low Risk Testing:** Test everything on Polygon before expensive Ethereum ops

## Why Deploy to Polygon Before Ethereum?

1. **Cost savings:** Test everything for ~$10 instead of ~$150
2. **Faster iteration:** Fix bugs cheaply
3. **User onboarding:** Easier for new users (low fees)
4. **QuickSwap volume:** Strong DEX with good liquidity
5. **Confidence:** Proven deployment before expensive Ethereum deployment

## Troubleshooting

### "Insufficient funds"
- Make sure you have at least 10 MATIC
- Bridge MATIC from Ethereum: https://portal.polygon.technology/

### "Nonce too high"
- Reset your MetaMask: Settings → Advanced → Reset Account

### "Transaction underpriced"
- Gas is so cheap this should never happen
- If it does, increase gas slightly in hardhat.config.js

### "Contract verification failed"
- Wait 1 minute and try again
- Verify manually with command above
- PolygonScan is usually very fast

## Support

- **Discord:** #dex-deployment
- **Email:** dev@etrid.org
- **Docs:** https://docs.etrid.org
- **Polygon Docs:** https://wiki.polygon.technology/

## References

- [COMPLETE_DEX_DEPLOYMENT_GUIDE.md](../COMPLETE_DEX_DEPLOYMENT_GUIDE.md)
- [FOUNDATION_CHARTER.md](../../FOUNDATION_CHARTER.md)
- [QuickSwap Docs](https://docs.quickswap.exchange/)
- [Polygon PoS Bridge](https://wiki.polygon.technology/docs/develop/ethereum-polygon/pos/getting-started/)
- [OpenZeppelin ERC20](https://docs.openzeppelin.com/contracts/4.x/erc20)
