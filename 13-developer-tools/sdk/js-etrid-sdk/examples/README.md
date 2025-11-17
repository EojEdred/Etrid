# Ëtrid SDK Examples

Comprehensive examples demonstrating all major features of the Ëtrid Protocol JavaScript/TypeScript SDK.

## 📚 Examples Overview

### 1. **Lightning-Bloc Payments** (`lightning-bloc-payment.ts`)
- Open and manage payment channels
- Route multi-hop payments
- Update channel state off-chain
- Estimate routing fees
- Close channels cooperatively or via dispute

### 2. **Distribution Rewards** (`claim-rewards.ts`)
- Check eligibility for daily rewards (27,397 ÉTR/day)
- Query pending rewards across 5 categories
- Claim rewards from Voters, Stakers, Validators, etc.
- View claim history
- Estimate future distributions

### 3. **Smart Contract Deployment** (`deploy-contract.ts`)
- Upload WebAssembly contract code
- Instantiate and deploy contracts
- Call contract methods
- Query contract state (free)
- Estimate gas costs in VMw units

### 4. **AI Identity Registration** (`ai-registration.ts`)
- Register AI agents with AIDID (world's first AI DID)
- Update AI profiles and capabilities
- Build and manage reputation scores
- Grant/revoke permissions
- Search AIs by capability

### 5. **Cross-Chain Bridge** (`cross-chain-bridge.ts`)
- Bridge tokens across 13 blockchains
- Monitor bridge transaction status
- Estimate cross-chain fees
- Check bridge limits
- View bridge history

### 6. **Price Oracles** (`price-oracle.ts`)
- Get current prices from decentralized oracles
- Calculate TWAP (Time-Weighted Average Price)
- Subscribe to real-time price updates
- Query historical price data
- Monitor oracle source health

### 7. **DeFi Vault Lending** (`vault-lending.ts`)
- Deposit multi-asset collateral
- Borrow against collateral
- Monitor vault health and ratios
- Repay loans
- Withdraw collateral safely

---

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Compile TypeScript
npm run build

# Run any example
npx ts-node examples/lightning-bloc-payment.ts
npx ts-node examples/claim-rewards.ts
npx ts-node examples/deploy-contract.ts
```

## 🏃 Running Individual Examples

```bash
# Lightning-Bloc instant payments (500K+ TPS)
npx ts-node examples/lightning-bloc-payment.ts

# Claim daily rewards (10M ÉTR/year)
npx ts-node examples/claim-rewards.ts

# Deploy WebAssembly smart contracts
npx ts-node examples/deploy-contract.ts

# Register AI identities (world's first AI DID)
npx ts-node examples/ai-registration.ts

# Bridge across 13 blockchains
npx ts-node examples/cross-chain-bridge.ts

# Decentralized price oracles
npx ts-node examples/price-oracle.ts

# DeFi lending with collateral vaults
npx ts-node examples/vault-lending.ts
```

## 📋 Prerequisites

### Software Requirements
- **Node.js** v16+ or v18+ (LTS recommended)
- **TypeScript** v4.9+
- **Polkadot.js API** v10+ (auto-installed)

### Network Options

**Option 1: Local Development Node**
```bash
# Run local FlareChain node
./flarechain --dev --ws-port 9944
```

**Option 2: Testnet**
```typescript
const provider = new WsProvider('wss://testnet.etrid.io');
```

**Option 3: Mainnet**
```typescript
const provider = new WsProvider('wss://rpc.etrid.io');
```

## 🔗 Resources

- **Documentation**: https://docs.etrid.io
- **API Reference**: https://docs.etrid.io/api
- **Discord Community**: https://discord.gg/etrid
- **GitHub Repository**: https://github.com/etrid/etrid
- **Website**: https://etrid.io

## 🐛 Troubleshooting

**Connection Errors**
```typescript
// Ensure node is running on correct port
const provider = new WsProvider('ws://localhost:9944');
```

**Insufficient Balance**
```bash
# Fund test accounts from faucet or transfer
# Alice, Bob test accounts need ÉTR for gas
```

**Module Not Found**
```bash
# Link SDK locally before NPM publication
npm run build && npm link
```

## 💡 Best Practices

1. Always wrap SDK calls in try-catch blocks
2. Estimate gas before expensive operations
3. Clean up subscriptions when done
4. Use queries (free) for read operations
5. Test on local node before production

---

**Built with ❤️ by the Ëtrid Team**

For questions, join our [Discord](https://discord.gg/etrid) or open an issue on [GitHub](https://github.com/etrid/etrid/issues).
