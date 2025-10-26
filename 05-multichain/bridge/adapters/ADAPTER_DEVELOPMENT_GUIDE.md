# Adapter Development Guide

**Version**: 1.0
**Last Updated**: October 24, 2025
**Purpose**: Template and best practices for creating new exchange/chain adapters

---

## Table of Contents

1. [Overview](#overview)
2. [When to Use Adapters vs. PBCs](#when-to-use-adapters-vs-pbcs)
3. [Adapter Architecture](#adapter-architecture)
4. [Development Template](#development-template)
5. [Testing Strategy](#testing-strategy)
6. [Deployment Checklist](#deployment-checklist)

---

## Overview

**Adapters** are lightweight integration modules that connect Ëtrid's existing PBC infrastructure to new chains, DEXs, or exchange platforms **without** requiring new Partition Burst Chains (PBCs).

### Benefits of Adapter Model

| Benefit | Description |
|---------|-------------|
| **4x Faster** | 10-15 hours vs. 40+ hours for new PBC |
| **10x Cheaper** | $1-2k vs. $10-15k per chain |
| **Reuses Security** | Leverages existing multi-sig custodians |
| **Scalable** | Add 10+ chains in a week |
| **Lower Maintenance** | No new collators to operate |

### When to Use Adapters

✅ **Use Adapters When**:
- Target chain is EVM-compatible (Ethereum L2s, sidechains)
- DEX integration only needs token presence (Uniswap, PancakeSwap, Raydium)
- Hybrid DEX/API integration (Hyperliquid, BullEx)
- Wallet SDK integration (Phantom, MetaMask extensions)

❌ **Create New PBC When**:
- Fundamentally different architecture (e.g., Cosmos IBC, Aptos Move)
- Top 20 blockchain by market cap with >1M active users
- Strategic ecosystem partnership requiring native integration
- Cannot reuse existing PBC bridges (BTC UTXO, SOL SPL, etc.)

---

## Adapter Architecture

### Three Adapter Types

#### Type 1: EVM Bridge Adapter (Base L2, Arbitrum, Avalanche)

**Pattern**: Deploy ERC-20 wrapper → Connect to existing ETH-PBC bridge

```
Ëtrid FlareChain
    ↓
ETH-PBC (Ethereum bridge)
    ↓ (reuse)
ERC-20 Wrapper on Target Chain (Base, Arbitrum, etc.)
    ↓
DEX Liquidity Pools (Uniswap V3, etc.)
```

**Files Needed**:
1. `deploy-tokens.ts` - ERC-20 deployment script
2. `bridge.ts` - Event monitoring adapter
3. `README.md` - Setup guide

**Example**: See `/adapters/base/`

---

#### Type 2: API Integration Adapter (Hyperliquid, BullEx)

**Pattern**: API wrapper → Bridge routing → Exchange listing

```
Ëtrid Tokens (via ETH/BSC/SOL PBC)
    ↓
API Adapter (REST/WebSocket)
    ↓
Exchange Platform (market creation, orderbook, trading)
```

**Files Needed**:
1. `api.ts` - REST/WebSocket client
2. `auth.ts` - API key management
3. `README.md` - Integration guide

**Example**: See `/adapters/hyperliquid/`

---

#### Type 3: Non-EVM Native Adapter (Solana, Cosmos, Aptos)

**Pattern**: Native token program → Existing PBC bridge → DEX integration

```
Ëtrid FlareChain
    ↓
Chain-Specific PBC (SOL-PBC, etc.)
    ↓
Native Token Deployment (SPL, IBC, Move)
    ↓
Native DEX (Raydium, Osmosis, etc.)
```

**Files Needed**:
1. Deployment guide (chain-specific, e.g., Rust/Anchor for Solana)
2. Bridge adapter (TypeScript/Rust)
3. Testing scripts

**Example**: See `/adapters/solana/`

---

## Development Template

### Step-by-Step Checklist

#### 1. Create Adapter Directory

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/
mkdir <chain-name>  # e.g., "avalanche", "optimism", "arbitrum"
cd <chain-name>
```

#### 2. Copy Template Files

Choose template based on adapter type:

**For EVM chains**:
```bash
cp ../base/deploy-tokens.ts ./
cp ../base/bridge.ts ./
cp ../base/README.md ./
```

**For API integrations**:
```bash
cp ../hyperliquid/api.ts ./
cp ../hyperliquid/README.md ./
```

**For non-EVM chains**:
```bash
cp ../solana/RAYDIUM_DEPLOYMENT_GUIDE.md ./<CHAIN>_DEPLOYMENT_GUIDE.md
```

#### 3. Customize Configuration

Update configuration interfaces for your target chain:

```typescript
// Example: avalanche/deploy-tokens.ts

interface AvalancheDeploymentConfig {
  rpcUrl: string; // https://api.avax.network/ext/bc/C/rpc
  deployerPrivateKey: string;
  bridgeAddress: string; // Ëtrid ETH-PBC bridge (reuse)
  dexRouter: string; // Trader Joe router address
}

const DEFAULT_CONFIG = {
  rpcUrl: process.env.AVAX_RPC_URL || 'https://api.avax.network/ext/bc/C/rpc',
  deployerPrivateKey: process.env.DEPLOYER_PRIVATE_KEY || '',
  bridgeAddress: process.env.BRIDGE_ADDRESS || '',
  dexRouter: '0x60aE616a2155Ee3d9A68541Ba4544862310933d4' // Trader Joe V2
};
```

#### 4. Update Token Metadata

```typescript
// Update token names to reflect target chain

const ETR_METADATA = {
  name: 'Etrid Coin (Avalanche)', // Change "Avalanche" to your chain
  symbol: 'ÉTR',
  decimals: 18 // Or 9 for Solana
};

const EDSC_METADATA = {
  name: 'Etrid Dollar Stablecoin (Avalanche)',
  symbol: 'EDSC',
  decimals: 18
};
```

#### 5. Update DEX Addresses

Research and add DEX-specific addresses:

```typescript
// Example: Avalanche / Trader Joe

const DEX_ADDRESSES = {
  // Trader Joe V2 on Avalanche
  router: '0x60aE616a2155Ee3d9A68541Ba4544862310933d4',
  factory: '0x9Ad6C38BE94206cA50bb0d90783181662f0Cfa10',

  // Common token pairs
  WAVAX: '0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7',
  USDC: '0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E',
  USDT: '0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7'
};
```

#### 6. Implement Chain-Specific Logic

Adapt for chain-specific features:

```typescript
// Example: Avalanche has lower gas prices

async deployToken(metadata: TokenMetadata): Promise<string> {
  const gasPrice = await this.provider.getFeeData();

  // Avalanche gas is much cheaper than Ethereum
  const deployment = await factory.deploy(
    metadata.name,
    metadata.symbol,
    metadata.decimals,
    this.config.bridgeAddress,
    {
      gasLimit: 2000000,
      gasPrice: gasPrice.gasPrice // Typically ~25-50 gwei
    }
  );

  // ... rest of deployment logic
}
```

#### 7. Add Testing Scripts

Create `test-deployment.sh`:

```bash
#!/bin/bash

# Test deployment on testnet first
export AVAX_RPC_URL="https://api.avax-test.network/ext/bc/C/rpc"
export DEPLOYER_PRIVATE_KEY="<TESTNET_KEY>"
export BRIDGE_ADDRESS="<TESTNET_BRIDGE>"

# Deploy test tokens
ts-node deploy-tokens.ts

# Verify deployment
echo "Check tokens at: https://testnet.snowtrace.io/"
```

#### 8. Document Integration

Update `README.md` with:
- Chain-specific setup (RPC URLs, explorers)
- Gas requirements (typical costs)
- DEX integration steps
- Verification links
- Troubleshooting common issues

---

## Testing Strategy

### 1. Local Testing (Hardhat/Foundry Fork)

```typescript
// test/adapter.test.ts

import { ethers } from "hardhat";
import { expect } from "chai";

describe("Avalanche Adapter", function () {
  it("Should deploy ÉTR token with correct metadata", async function () {
    const factory = await ethers.getContractFactory("BridgeableERC20");
    const token = await factory.deploy("Etrid Coin (Avalanche)", "ÉTR", 18, bridgeAddress);

    expect(await token.name()).to.equal("Etrid Coin (Avalanche)");
    expect(await token.symbol()).to.equal("ÉTR");
    expect(await token.decimals()).to.equal(18);
  });

  it("Should allow bridge to mint tokens", async function () {
    // Test bridge mint functionality
    const amount = ethers.parseEther("1000");
    await token.connect(bridge).bridgeMint(user.address, amount, txHash);

    expect(await token.balanceOf(user.address)).to.equal(amount);
  });

  it("Should allow users to burn tokens for bridge transfer", async function () {
    // Test bridge burn functionality
    const amount = ethers.parseEther("100");
    await token.connect(user).bridgeBurn(amount, "etrid1abc...");

    // Verify burn event emitted
  });
});
```

### 2. Testnet Deployment

```bash
# Always deploy to testnet first!

# Avalanche Fuji testnet
export AVAX_RPC_URL="https://api.avax-test.network/ext/bc/C/rpc"

# Get testnet AVAX from faucet
# https://faucet.avax.network/

# Deploy
ts-node deploy-tokens.ts

# Verify on testnet explorer
# https://testnet.snowtrace.io/
```

### 3. Integration Testing

Test full bridge flow:

1. Lock ÉTR on Ëtrid FlareChain
2. Mint ÉTR.avax on Avalanche (via adapter)
3. Swap on Trader Joe
4. Burn ÉTR.avax
5. Release ÉTR on FlareChain

### 4. Mainnet Dry Run

Before deploying:
- [ ] Testnet deployment successful
- [ ] Bridge flow tested end-to-end
- [ ] Gas costs estimated
- [ ] Liquidity budget confirmed
- [ ] Team reviewed code
- [ ] Security audit (for new contract code)

---

## Deployment Checklist

### Pre-Deployment

- [ ] **Adapter code complete** (deployment scripts + bridge adapter)
- [ ] **Testnet testing complete** (tokens deployed, tested)
- [ ] **Gas budget allocated** (estimate 2-5x expected cost as buffer)
- [ ] **Liquidity ready** (tokens + paired assets)
- [ ] **Bridge configuration** (PBC bridge address, multisig custodians)
- [ ] **Team sign-off** (dev lead + security review)

### Deployment Steps

1. **Deploy Tokens**
   ```bash
   # Set mainnet RPC
   export CHAIN_RPC_URL="<MAINNET_RPC>"
   export DEPLOYER_PRIVATE_KEY="<MAINNET_KEY>"
   export BRIDGE_ADDRESS="<ETH_PBC_BRIDGE_ADDRESS>"

   # Deploy ÉTR and EDSC
   ts-node deploy-tokens.ts
   ```

2. **Verify Contracts**
   - Submit source code to block explorer (Etherscan, BscScan, etc.)
   - Verify constructor arguments match

3. **Create DEX Pools**
   - Use DEX UI (recommended for first deployment)
   - Or run pool creation script

4. **Seed Liquidity**
   - Transfer tokens + paired assets to pools
   - Lock LP tokens in multisig (transparency)

5. **Configure Bridge Adapter**
   - Update `bridge.ts` with deployed token addresses
   - Start bridge monitoring service
   - Test lock → mint flow

6. **Post-Deployment Verification**
   - Tokens visible on block explorer
   - DEX pools created and visible on DEX UI
   - Bridge adapter monitoring events
   - Test small cross-chain transfer

### Post-Deployment

- [ ] **Documentation updated** (token addresses, pool links, explorer links)
- [ ] **Community announcement** (blog post, Twitter, Discord)
- [ ] **CoinGecko/CMC submission** (if new chain/DEX)
- [ ] **Monitoring setup** (alerts for bridge events, low liquidity)
- [ ] **Incident response plan** (pause mechanism, emergency contacts)

---

## Example: Creating Arbitrum Adapter

Here's a complete walkthrough:

### 1. Setup

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/
mkdir arbitrum
cd arbitrum
```

### 2. Copy Base Template

```bash
cp ../base/deploy-tokens.ts ./
cp ../base/bridge.ts ./
```

### 3. Update Configuration

**`deploy-tokens.ts`**:
```typescript
const config = {
  rpcUrl: 'https://arb1.arbitrum.io/rpc',
  deployerPrivateKey: process.env.DEPLOYER_PRIVATE_KEY,
  bridgeAddress: process.env.ETH_BRIDGE_ADDRESS, // Reuse ETH-PBC
  uniswapV3Factory: '0x1F98431c8aD98523631AE4a59f267346ea31F984' // Arbitrum Uniswap V3
};

const metadata = {
  etr: {
    name: 'Etrid Coin (Arbitrum)',
    symbol: 'ÉTR',
    decimals: 18
  },
  edsc: {
    name: 'Etrid Dollar Stablecoin (Arbitrum)',
    symbol: 'EDSC',
    decimals: 18
  }
};
```

### 4. Test on Testnet

```bash
# Arbitrum Sepolia testnet
export ARB_RPC_URL="https://sepolia-rollup.arbitrum.io/rpc"
export DEPLOYER_PRIVATE_KEY="<TESTNET_KEY>"
export ETH_BRIDGE_ADDRESS="<TESTNET_BRIDGE>"

ts-node deploy-tokens.ts
```

### 5. Deploy to Mainnet

```bash
# Switch to mainnet
export ARB_RPC_URL="https://arb1.arbitrum.io/rpc"
export DEPLOYER_PRIVATE_KEY="<MAINNET_KEY>"
export ETH_BRIDGE_ADDRESS="<MAINNET_BRIDGE>"

ts-node deploy-tokens.ts

# Output:
# ✅ ÉTR deployed at: 0x...
# ✅ EDSC deployed at: 0x...
```

### 6. Create Uniswap V3 Pools

```bash
# Use script or Uniswap UI
# https://app.uniswap.org/#/add
```

### 7. Start Bridge Adapter

```bash
ts-node bridge.ts

# Output:
# 🌉 Starting Arbitrum Bridge Adapter...
# ✅ Connected to Ëtrid ETH-PBC
# ✅ Connected to Arbitrum (Chain ID: 42161)
# 👀 Monitoring for lock/burn events...
```

### 8. Test Bridge Flow

```bash
# On Ëtrid: Lock 100 ÉTR → Arbitrum
# Bridge adapter detects lock event
# Mints 100 ÉTR.arb on Arbitrum
# User receives tokens

# User burns 50 ÉTR.arb on Arbitrum
# Bridge adapter detects burn event
# Releases 50 ÉTR on Ëtrid
```

**Total time**: ~10 hours (vs. 40+ for new PBC)
**Total cost**: ~$1,500 (gas + dev time) vs. $10-15k for new PBC

---

## Adapter Comparison Matrix

| Adapter Target | Type | Difficulty | Dev Time | Cost | Example |
|----------------|------|------------|----------|------|---------|
| **Ethereum L2** (Base, Arbitrum, Optimism) | EVM Bridge | ★☆☆ | 10h | $1-2k | `/adapters/base/` |
| **EVM Sidechain** (BSC, Polygon, Avalanche) | EVM Bridge | ★☆☆ | 10h | $1-2k | `/adapters/bsc/` |
| **Solana** | Non-EVM Native | ★★☆ | 15h | $2-3k | `/adapters/solana/` |
| **Hybrid DEX** (Hyperliquid) | API Integration | ★★☆ | 10h | $1k | `/adapters/hyperliquid/` |
| **Multi-Chain Router** (BullEx) | API Integration | ★★☆ | 8h | $1k | `/adapters/bullish/` |
| **Cosmos IBC** | Non-EVM Native | ★★★ | 20h | $3-5k | _Not yet implemented_ |
| **Aptos/Sui (Move VM)** | Non-EVM Native | ★★★ | 25h | $5-7k | _Not yet implemented_ |

---

## Best Practices

### Security

1. **Always test on testnets first**
2. **Use multi-sig for bridge authority** (not single EOA)
3. **Implement replay attack prevention** (track processed transaction hashes)
4. **Add circuit breakers** (pause mechanism for emergencies)
5. **Audit bridge contracts** (especially mint/burn logic)

### Performance

1. **Batch event processing** (reduce RPC calls)
2. **Use WebSocket subscriptions** (lower latency than polling)
3. **Implement exponential backoff** (for RPC rate limits)
4. **Monitor adapter health** (alert on downtime)

### Maintainability

1. **Clear configuration** (use `.env` files, never hardcode keys)
2. **Comprehensive logging** (debug-level for development)
3. **Version deployment artifacts** (save addresses, ABIs, tx hashes)
4. **Document chain-specific quirks** (gas pricing, block times, finality)

---

## Resources

### EVM Chains
- **Ethereum**: https://ethereum.org/developers
- **Base**: https://docs.base.org/
- **Arbitrum**: https://docs.arbitrum.io/
- **Optimism**: https://docs.optimism.io/
- **Avalanche**: https://docs.avax.network/
- **BSC**: https://docs.bnbchain.org/

### Non-EVM Chains
- **Solana**: https://docs.solana.com/
- **Cosmos**: https://docs.cosmos.network/
- **Aptos**: https://aptos.dev/
- **Sui**: https://docs.sui.io/

### DEX Integrations
- **Uniswap**: https://docs.uniswap.org/
- **PancakeSwap**: https://docs.pancakeswap.finance/
- **Raydium**: https://docs.raydium.io/
- **Trader Joe**: https://docs.traderjoexyz.com/

---

**Document Version**: 1.0
**Last Updated**: October 24, 2025
**Next Review**: After 5+ adapter deployments (gather lessons learned)
