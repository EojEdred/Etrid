# Ëtrid Exchange Listing Adapters

**Purpose**: Lightweight integration modules for connecting Ëtrid tokens to new chains and DEXs without creating new PBCs.

---

## 📁 Directory Structure

```
adapters/
├── README.md (this file)
├── ADAPTER_DEVELOPMENT_GUIDE.md (template & best practices)
│
├── base/                         # Base L2 (Ethereum L2)
│   ├── deploy-tokens.ts          # ERC-20 deployment script
│   ├── bridge.ts                 # Event monitoring adapter
│   └── README.md
│
├── bsc/                          # Binance Smart Chain
│   ├── deploy-pancakeswap.ts    # BEP-20 + PancakeSwap pools
│   └── README.md
│
├── solana/                       # Solana
│   ├── RAYDIUM_DEPLOYMENT_GUIDE.md  # SPL token + Raydium guide
│   └── bridge-adapter.ts         # SOL-PBC integration (WIP)
│
├── hyperliquid/                  # Hyperliquid hybrid DEX
│   ├── api.ts                    # REST/WebSocket API client
│   └── README.md
│
└── bullish/                      # BullEx multi-chain DEX
    ├── bridge-listing.ts         # Multi-chain listing automation
    └── README.md
```

---

## 🚀 Quick Start

### 1. Choose Your Target

| Target | Adapter Type | Use This |
|--------|--------------|----------|
| **Ethereum L2** (Base, Arbitrum, Optimism) | EVM Bridge | `base/` template |
| **EVM Sidechain** (BSC, Polygon, Avalanche) | EVM Bridge | `bsc/` template |
| **Solana** | Non-EVM Native | `solana/` guide |
| **Hybrid DEX** (Hyperliquid) | API Integration | `hyperliquid/` template |
| **Multi-Chain Router** (BullEx) | API Integration | `bullish/` template |

### 2. Copy Template

```bash
# Example: Creating Arbitrum adapter
cd adapters/
cp -r base/ arbitrum/

# Update configuration
cd arbitrum/
vim deploy-tokens.ts  # Change chain-specific settings
vim bridge.ts         # Update RPC URLs and addresses
```

### 3. Deploy

```bash
# Configure environment
export CHAIN_RPC_URL="https://arb1.arbitrum.io/rpc"
export DEPLOYER_PRIVATE_KEY="<YOUR_KEY>"
export BRIDGE_ADDRESS="<ETH_PBC_BRIDGE>"

# Deploy tokens
ts-node deploy-tokens.ts

# Start bridge adapter
ts-node bridge.ts
```

---

## 📊 Adapter Status Matrix

| Adapter | Status | ÉTR Deployed | EDSC Deployed | Pools Created | Bridge Active |
|---------|--------|--------------|---------------|---------------|---------------|
| **Base L2** | 🟡 Ready | ⏳ Pending | ⏳ Pending | ⏳ Pending | ⏳ Pending |
| **BSC** (PancakeSwap) | 🟡 Ready | ⏳ Pending | ⏳ Pending | ⏳ Pending | ⏳ Pending |
| **Solana** (Raydium) | 🟡 Ready | ⏳ Pending | ⏳ Pending | ⏳ Pending | ⏳ Pending |
| **Hyperliquid** | 🟡 Ready | ⏳ Pending | ⏳ Pending | N/A | N/A |
| **BullEx** | 🟡 Ready | ⏳ Pending | ⏳ Pending | N/A | N/A |
| **Arbitrum** | ⏸️ Planned | - | - | - | - |
| **Optimism** | ⏸️ Planned | - | - | - | - |
| **Avalanche** | ⏸️ Planned | - | - | - | - |

Legend:
- 🟢 **Active** - Live on mainnet
- 🟡 **Ready** - Code complete, pending deployment
- ⏸️ **Planned** - Scheduled for future development
- ⏳ **Pending** - Awaiting deployment/configuration

---

## 💰 Cost Comparison: Adapters vs. PBCs

| Metric | Adapter Approach | New PBC Approach |
|--------|------------------|------------------|
| **Development Time** | 10-15 hours | 40+ hours |
| **Cost** | $1-2k | $10-15k |
| **Deployment Gas** | $50-500 (chain-dependent) | $500-2k |
| **Ongoing Maintenance** | Minimal (reuse infra) | High (new collator) |
| **Security Model** | Reuses existing multi-sig | Requires new setup |
| **Time to Market** | 1-2 weeks | 4-6 weeks |

**Verdict**: Adapters are **4x faster** and **10x cheaper** than new PBCs for most use cases.

---

## 🎯 Adapter Development Workflow

### Step 1: Research Target Chain/DEX

- [ ] Identify chain type (EVM, Non-EVM, API-only)
- [ ] Find DEX documentation (contracts, APIs, SDKs)
- [ ] Check existing PBC compatibility (can we reuse ETH-PBC, SOL-PBC, etc.?)
- [ ] Estimate gas costs and liquidity requirements

### Step 2: Copy Appropriate Template

```bash
# For EVM chains
cp -r base/ <new-chain>/

# For non-EVM chains
cp -r solana/ <new-chain>/

# For API integrations
cp -r hyperliquid/ <new-dex>/
```

### Step 3: Customize Configuration

- Update RPC URLs
- Add DEX contract addresses
- Set token metadata (name, symbol, decimals)
- Configure bridge addresses (from existing PBCs)

### Step 4: Test on Testnet

```bash
# Always test first!
export CHAIN_RPC_URL="<TESTNET_RPC>"
export DEPLOYER_PRIVATE_KEY="<TESTNET_KEY>"

ts-node deploy-tokens.ts
```

### Step 5: Deploy to Mainnet

```bash
# After testnet success
export CHAIN_RPC_URL="<MAINNET_RPC>"
export DEPLOYER_PRIVATE_KEY="<MAINNET_KEY>"

ts-node deploy-tokens.ts
ts-node create-pools.ts  # If applicable
ts-node bridge.ts        # Start monitoring
```

### Step 6: Post-Deployment

- Verify contracts on block explorer
- Submit to DEX UI (if permissioned)
- Update documentation with addresses
- Announce to community

---

## 🔐 Security Checklist

Before deploying any adapter:

- [ ] **Testnet deployment successful** (at least 3 test transactions)
- [ ] **Bridge authority is multi-sig** (not single EOA)
- [ ] **Replay protection implemented** (transaction hash tracking)
- [ ] **Code reviewed** (by at least 2 developers)
- [ ] **Gas costs estimated** (with 2-5x buffer)
- [ ] **Emergency pause mechanism** (circuit breaker)
- [ ] **Monitoring setup** (alerts for downtime)
- [ ] **Incident response plan** (contacts, procedures)

---

## 📚 Documentation

### For Developers

- **[Adapter Development Guide](ADAPTER_DEVELOPMENT_GUIDE.md)** - Complete template and best practices
- **[Exchange Expansion Master Plan](../../../docs/EXCHANGE_EXPANSION_MASTER_PLAN.md)** - Strategic overview

### Adapter-Specific Guides

- **[Base L2](base/README.md)** - Ethereum L2 deployment
- **[BSC/PancakeSwap](bsc/README.md)** - Binance Smart Chain
- **[Solana/Raydium](solana/RAYDIUM_DEPLOYMENT_GUIDE.md)** - Solana SPL tokens
- **[Hyperliquid](hyperliquid/README.md)** - Hybrid DEX integration
- **[BullEx](bullish/README.md)** - Multi-chain DEX router

---

## 🛠️ Common Tasks

### Deploy New Token on Existing Adapter

```typescript
// Example: Deploy new USDT wrapper on Base L2
import BaseTokenDeployer from './base/deploy-tokens';

const deployer = new BaseTokenDeployer(config);

const usdtAddress = await deployer.deployToken({
  name: 'Tether USD (Base)',
  symbol: 'USDT',
  decimals: 6, // USDT uses 6 decimals
  initialSupply: '0'
});
```

### Create New Liquidity Pool

```typescript
// Example: Create ÉTR/USDC pool on Base
await deployer.createUniswapPool({
  tokenAddress: etrAddress,
  pairTokenAddress: '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913', // USDC on Base
  feeTier: 3000, // 0.3%
  initialPrice: '1.0' // 1 ÉTR = $1 USDC
});
```

### Monitor Bridge Events

```typescript
// Example: Start bridge monitoring
import BaseBridgeAdapter from './base/bridge';

const adapter = new BaseBridgeAdapter(config);
await adapter.start();

// Adapter will now:
// 1. Monitor Ëtrid for lock events → mint on Base
// 2. Monitor Base for burn events → release on Ëtrid
```

---

## 🚨 Troubleshooting

### "Insufficient funds for gas"

```bash
# Check balance
cast balance <YOUR_ADDRESS> --rpc-url <RPC_URL>

# Top up wallet from exchange or faucet (testnet)
```

### "Bridge mint failed: Unauthorized"

- Ensure bridge address in config matches actual bridge authority
- Check multi-sig has approved transaction
- Verify bridge adapter has correct private key

### "Pool creation failed: Pair already exists"

- Pool may already be created by another user
- Use existing pool instead of creating new one
- Check DEX UI for pool address

### "RPC rate limit exceeded"

- Use paid RPC provider (Alchemy, Infura, QuickNode)
- Implement exponential backoff
- Reduce polling frequency

---

## 📞 Support

- **Documentation Issues**: Create PR with fixes
- **Adapter Bugs**: Open GitHub issue with label `adapter:<name>`
- **New Adapter Requests**: Propose in #exchange-expansion Discord channel
- **Security Concerns**: Email security@etrid.io (do not post publicly)

---

## 🎉 Contributing

We welcome adapter contributions! To add a new adapter:

1. Fork the repo
2. Create adapter directory: `adapters/<chain-name>/`
3. Follow [Adapter Development Guide](ADAPTER_DEVELOPMENT_GUIDE.md)
4. Test on testnet (document results)
5. Submit PR with:
   - Deployment scripts
   - README with setup instructions
   - Test results/screenshots
   - Cost analysis (gas + dev time)

---

**Last Updated**: October 24, 2025
**Maintainer**: Ëtrid Protocol Team
**Status**: Active Development (Phase 1 of Exchange Expansion)
