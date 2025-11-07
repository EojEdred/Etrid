# Using ËTRID Bridges

ËTRID connects 13 major blockchains through **Partition Burst Chains (PBCs)**, enabling seamless cross-chain asset transfers and interoperability.

## Supported Chains

ËTRID bridges connect you to:

1. **Bitcoin (BTC)** - Store of value
2. **Ethereum (ETH)** - Smart contracts & DeFi
3. **Solana (SOL)** - High-speed transactions
4. **Cardano (ADA)** - Academic rigor
5. **Polkadot (DOT)** - Interoperability
6. **Binance Smart Chain (BNB)** - DeFi & DEX
7. **Avalanche (AVAX)** - Enterprise blockchain
8. **Polygon (MATIC)** - Ethereum L2
9. **Cosmos (ATOM)** - Internet of blockchains
10. **Algorand (ALGO)** - Pure proof-of-stake
11. **Tezos (XTZ)** - Self-amending blockchain
12. **Near Protocol (NEAR)** - Sharded blockchain
13. **EDSC** - ËTRID stablecoin

---

## How Bridges Work

ËTRID uses **Partition Burst Chains (PBCs)** - specialized chains that:
1. Maintain light clients of external blockchains
2. Verify cross-chain transactions cryptographically
3. Issue wrapped tokens on ËTRID
4. Enable seamless asset movement

**Security:** Multi-signature custodians + light client verification

---

## Bridging Assets to ËTRID

### Example: Bridge ETH from Ethereum

#### Step 1: Access Bridge
1. Visit [bridge.etrid.org](https://bridge.etrid.org)
2. Or click **"Bridge"** in wallet navigation

#### Step 2: Select Chains
- **From:** Ethereum Mainnet
- **To:** ËTRID (PBC-ETH)

#### Step 3: Connect Wallets
1. Connect Ethereum wallet (MetaMask)
2. Connect ËTRID wallet
3. Approve connections

#### Step 4: Initiate Bridge

```
┌─────────────────────────────────────────────┐
│ Bridge Assets                                │
├─────────────────────────────────────────────┤
│ From: Ethereum Mainnet                      │
│ To: ËTRID PBC-ETH                          │
│                                              │
│ Asset: ETH                                   │
│ Amount: 1.0 ETH                             │
│                                              │
│ Bridge Fee: 0.001 ETH (~$2.50)              │
│ Estimated Time: 15-30 minutes                │
│                                              │
│ You will receive: 0.999 ETH on ËTRID       │
│                                              │
│ [ Review Bridge Transaction ]               │
└─────────────────────────────────────────────┘
```

1. Enter amount to bridge
2. Review details carefully
3. Click **"Approve"** (authorize spending)
4. Click **"Bridge"** (execute transaction)

#### Step 5: Wait for Confirmations

**Bridge Process:**
1. **Ethereum confirmation:** 12 blocks (~3 min)
2. **Relay to ËTRID:** Light client verification (~2 min)
3. **PBC minting:** Issue wrapped ETH (~1 min)
4. **Finalization:** 3 ËTRID blocks (~15 sec)

**Total Time:** 15-30 minutes

**Track Status:**
- Click "Track Transaction"
- Monitor real-time progress
- Notification when complete

#### Step 6: Use Bridged Assets
- Bridged ETH appears in ËTRID wallet on PBC-ETH
- Use for DeFi, trading, staking
- Bridge back to Ethereum anytime

---

## Bridging Assets from ËTRID

### Example: Bridge ETH back to Ethereum

1. Navigate to bridge interface
2. Select:
   - **From:** ËTRID (PBC-ETH)
   - **To:** Ethereum Mainnet
3. Enter amount and destination Ethereum address
4. Pay bridge fee (ÉTR or ETH)
5. Wait for multi-sig custodian approval (~10 min)
6. Receive ETH on Ethereum Mainnet

---

## Bridge Fees

**Fee Structure:**
- **Bridge In:** 0.1% of transferred amount (min $0.50)
- **Bridge Out:** 0.1% of transferred amount (min $0.50)
- **Gas Fees:** Paid on source chain (varies by network)

**Example:**
- Bridge 10 ETH: 0.01 ETH fee + Ethereum gas
- Bridge 1,000 USDC: 1 USDC fee + BSC gas

---

## Bridge Times by Chain

| Chain | Bridge In | Bridge Out |
|-------|-----------|------------|
| Bitcoin | 30-60 min | 30-60 min |
| Ethereum | 15-30 min | 15-30 min |
| Solana | 5-10 min | 5-10 min |
| Binance Smart Chain | 3-5 min | 3-5 min |
| Polygon | 10-15 min | 10-15 min |
| Other PBCs | 10-30 min | 10-30 min |

---

## Supported Assets

### Native Tokens
All PBC native tokens are supported:
- BTC (Bitcoin)
- ETH (Ethereum)
- SOL (Solana)
- ADA (Cardano)
- BNB (Binance Smart Chain)
- AVAX (Avalanche)
- MATIC (Polygon)
- And more...

### Stablecoins
Major stablecoins on supported chains:
- USDC (Ethereum, BSC, Polygon, Solana)
- USDT (Ethereum, BSC, Polygon, Tron)
- DAI (Ethereum, BSC, Polygon)
- BUSD (Binance Smart Chain)

### ERC-20 Tokens
Popular ERC-20 tokens:
- WBTC (Wrapped Bitcoin)
- LINK (Chainlink)
- UNI (Uniswap)
- AAVE
- And more...

Check [bridge.etrid.org/assets](https://bridge.etrid.org/assets) for full list.

---

## Using EDSC Stablecoin

**EDSC (ËTRID Digital Standard Currency)** is a USD-pegged stablecoin native to ËTRID.

### Features
- **1 EDSC = $1 USD** (soft peg)
- Backed by multi-asset reserves
- Instant transfers on FlareChain
- Low transaction fees

### Getting EDSC

**Method 1: Mint EDSC**
1. Navigate to "EDSC" in wallet
2. Deposit collateral (BTC, ETH, stablecoins)
3. Receive EDSC at 1:1 USD value

**Method 2: Buy on DEX**
1. Use ËtridSwap
2. Trade ÉTR → EDSC
3. Instant settlement

**Method 3: Receive Payments**
- Accept EDSC from others
- Works like any other token

### Redeeming EDSC
- Burn EDSC to receive underlying collateral
- 1 EDSC redeemable for $1 worth of reserves
- Redemption fee: 0.1%

---

## Security Best Practices

### Before Bridging

✅ **Verify:**
- Bridge URL is correct ([bridge.etrid.org](https://bridge.etrid.org))
- Wallet addresses match (source & destination)
- Asset and amount are correct
- Sufficient funds for fees

### During Bridging

✅ **Check:**
- Transaction details in wallet
- Approvals are for correct contracts
- Fee amounts are reasonable

### After Bridging

✅ **Confirm:**
- Assets arrived in destination wallet
- Transaction finalized on explorer
- Save transaction hash for records

### ❌ Never

- Use unofficial bridge websites
- Share private keys/recovery phrases
- Rush through transaction confirmation
- Bridge more than you can afford to lose (on first try)

---

## Troubleshooting

### Transaction Stuck
**Problem:** Bridge transaction pending for > 1 hour

**Solutions:**
1. Check source chain explorer (transaction confirmed?)
2. Verify bridge status at [status.etrid.org](https://status.etrid.org)
3. Contact support with transaction hash
4. Wait up to 24 hours (may be custodian delay)

### Assets Not Received
**Problem:** Source transaction confirmed, but no assets on ËTRID

**Solutions:**
1. Check transaction hash on bridge explorer
2. Verify PBC chain address (might be wrong PBC)
3. Refresh wallet (hard refresh: Ctrl+Shift+R)
4. Contact support with both transaction hashes

### High Fees
**Problem:** Bridge fees higher than expected

**Solutions:**
1. Check source chain gas prices (may be congested)
2. Wait for off-peak hours
3. Consider bridging larger amounts (fee percentage same, but better value)
4. Use faster/cheaper source chains (e.g., BSC instead of Ethereum)

---

## Advanced: Cross-Chain DeFi

### Using Bridged Assets

Once assets are bridged to ËTRID, you can:

**DeFi Applications:**
- **Lend/Borrow:** Supply assets to lending protocols
- **Liquidity Pools:** Provide liquidity, earn fees
- **Yield Farming:** Stake LP tokens for rewards
- **Trading:** Swap on ËtridSwap DEX

**Cross-Chain Strategies:**
- Arbitrage between chains
- Yield optimization across networks
- Risk diversification

**Example Flow:**
1. Bridge USDC from Ethereum → ËTRID
2. Supply USDC to lending pool
3. Borrow ÉTR against USDC collateral
4. Stake ÉTR to earn 12% APY
5. Repay loan, bridge USDC back to Ethereum

---

## Bridge Explorer

Track all bridge activity:

**View:**
- [explorer.etrid.org/bridges](https://explorer.etrid.org/bridges)

**Features:**
- Real-time bridge transactions
- Volume statistics by chain
- Bridge TVL (Total Value Locked)
- Historical data and charts

---

## Need Help?

**Resources:**
- 📖 [Bridge FAQ](https://docs.etrid.org/bridges-faq)
- 💬 [Discord Support](https://discord.gg/etrid)
- 🎥 [Video Tutorial](https://youtube.com/etrid/bridge-guide)

**Support:**
- Email: bridge-support@etrid.org
- Include: source chain, destination chain, transaction hash

---

**Next Steps:**
- ✅ [Bridge your first assets](https://bridge.etrid.org)
- 📚 [Explore DeFi on ËTRID](../ai-devs/DEX_DEPLOYMENT_GUIDE.md)
- 🎓 [Learn about PBCs](architecture.md#partition-burst-chains)
