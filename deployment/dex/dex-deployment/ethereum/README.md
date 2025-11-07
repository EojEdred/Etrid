# ÉTR Token Ethereum Deployment

Deploy ÉTR (Ëtrid Coin) to Ethereum mainnet for Uniswap V3 listing.

## Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Configure Environment

```bash
cp .env.example .env
nano .env  # Add your PRIVATE_KEY and ETHERSCAN_API_KEY
```

### 3. Deploy to Testnet (Recommended First)

```bash
# Make sure you have Sepolia ETH first
npm run deploy:testnet
```

### 4. Deploy to Mainnet

```bash
# Make sure you have at least 0.1 ETH
npm run deploy:mainnet
```

## Requirements

### Gas Fees
- **Testnet (Sepolia):** ~0.01 ETH (free from faucet)
- **Mainnet:** ~0.05-0.1 ETH (~$150-300)

Get Sepolia ETH: https://sepoliafaucet.com/

### Liquidity for Uniswap
- 25,000,000 ÉTR
- ~666 ETH (~$2M at $3000/ETH)
- Source: Community LP Pool (per Foundation Charter)

## Contract Details

**Contract:** EtridETH.sol
- **Name:** Ëtrid Coin
- **Symbol:** ÉTR
- **Decimals:** 18 (Ethereum standard)
- **Initial Supply:** 25M ÉTR (for Uniswap liquidity)
- **Max Supply:** 1B ÉTR (bridge enforced)

**Features:**
- ERC-20 compliant
- Bridge mint/burn for cross-chain transfers
- Emergency pause capability
- Foundation multisig controlled
- OpenZeppelin security standards

## Deployment Steps

### Step 1: Deploy Token

```bash
npm run deploy:mainnet
```

This will:
1. Deploy EtridETH contract
2. Mint 25M ÉTR to deployer
3. Auto-verify on Etherscan
4. Save deployment info to `deployments/`

### Step 2: Transfer Ownership

```javascript
// Connect to deployed contract
const token = await ethers.getContractAt("EtridETH", "<TOKEN_ADDRESS>");

// Transfer to Foundation multisig
await token.transferOwnership("<FOUNDATION_MULTISIG_ADDRESS>");
```

### Step 3: Create Uniswap V3 Pool

Go to: https://app.uniswap.org/pools

1. Click **"Create Pool"**
2. **Token 0:** ÉTR (paste deployed address)
3. **Token 1:** WETH (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2)
4. **Fee tier:** 0.30% (3000)
5. **Initial price:** Set based on BSC/Solana price

### Step 4: Add Liquidity

1. Amount: 25M ÉTR + 666 ETH
2. Price range: Full range or concentrated
3. Approve token spending
4. Confirm transaction
5. Receive LP NFT position

### Step 5: Configure Bridge

```javascript
// Set bridge contract (allows cross-chain minting/burning)
await token.setBridgeContract("<BRIDGE_ADDRESS>");
```

### Step 6: Submit to Tracking Sites

**CoinGecko:**
- URL: https://www.coingecko.com/en/coins/new
- Include: Contract address, logo, website, socials

**CoinMarketCap:**
- URL: https://coinmarketcap.com/request/
- Include: Same info as CoinGecko

## Important Addresses

### Mainnet
- **WETH:** 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
- **Uniswap V3 Factory:** 0x1F98431c8aD98523631AE4a59f267346ea31F984
- **Uniswap V3 Router:** 0xE592427A0AEce92De3Edee1F18E0157C05861564
- **Uniswap Position Manager:** 0xC36442b4a4522E871399CD717aBDD847Ab11FE88

### Testnet (Sepolia)
- **WETH:** 0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14
- **Uniswap V3 Factory:** 0x0227628f3F023bb0B980b67D528571c95c6DaC1c
- **Uniswap V3 Router:** 0x3bFA4769FB09eefC5a80d6E87c3B9C650f7Ae48E

## Testing Checklist

Before mainnet deployment:

- [ ] Deploy to Sepolia testnet
- [ ] Create test pool on Sepolia
- [ ] Add test liquidity
- [ ] Perform test swaps (ÉTR → WETH and WETH → ÉTR)
- [ ] Verify contract on Etherscan
- [ ] Test ownership transfer
- [ ] Test bridge functions (if bridge deployed)
- [ ] Get Foundation approval (6-of-9 signatures)

## Verification

If auto-verification fails, verify manually:

```bash
npx hardhat verify --network mainnet <TOKEN_ADDRESS> <DEPLOYER_ADDRESS>
```

## Cost Breakdown

| Item | Cost (ETH) | Cost (USD @ $3000) |
|------|-----------|-------------------|
| Contract deployment | 0.02-0.05 | $60-150 |
| Pool creation | 0.01-0.02 | $30-60 |
| Add liquidity | 0.01-0.02 | $30-60 |
| **Total Gas** | **~0.05** | **~$150** |
| **Liquidity** | **666** | **$2M** |
| **Grand Total** | **~666** | **~$2M** |

## Security

- **Multisig Control:** All admin functions require Foundation 6-of-9 approval
- **OpenZeppelin Contracts:** Battle-tested, audited code
- **Max Supply Cap:** Hard limit at 1B ÉTR
- **Bridge Security:** Only authorized bridge can mint/burn
- **Emergency Pause:** 7-of-9 Foundation approval required

## Troubleshooting

### "Insufficient funds"
- Make sure you have at least 0.1 ETH
- Check gas prices: https://etherscan.io/gastracker

### "Nonce too high"
- Reset your MetaMask account: Settings → Advanced → Reset Account

### "Contract verification failed"
- Wait 1-2 minutes and try again
- Verify manually with command above

### "Transaction underpriced"
- Increase gas price in hardhat.config.js
- Or wait for lower network congestion

## Support

- **Discord:** #dex-deployment
- **Email:** dev@etrid.org
- **Docs:** https://docs.etrid.org

## References

- [COMPLETE_DEX_DEPLOYMENT_GUIDE.md](../COMPLETE_DEX_DEPLOYMENT_GUIDE.md)
- [FOUNDATION_CHARTER.md](../../FOUNDATION_CHARTER.md)
- [Uniswap V3 Docs](https://docs.uniswap.org/contracts/v3/overview)
- [OpenZeppelin ERC20](https://docs.openzeppelin.com/contracts/4.x/erc20)
