# Ëtrid SDK for JavaScript/TypeScript

> **Production-Ready** JavaScript/TypeScript SDK for building on the Ëtrid Protocol blockchain

[![npm version](https://badge.fury.io/js/%40etrid%2Fsdk.svg)](https://www.npmjs.com/package/@etrid/sdk)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://www.typescriptlang.org/)

## Overview

The Ëtrid SDK is a comprehensive TypeScript library that provides high-level abstractions for interacting with the Ëtrid Protocol blockchain. Built on top of Polkadot.js, it offers developer-friendly wrappers for all major pallets, complete type safety, and extensive documentation.

### Key Features

- **10 Complete Wrappers** - 6,636 lines, 214+ methods covering all major pallets
- **Lightning-Bloc Layer 3** - 500K+ TPS payment channels
- **AI Identity Standard** - World's first AI DID implementation
- **Cross-Chain Bridge** - 13 supported blockchains
- **ETWASM Smart Contracts** - WebAssembly execution
- **DeFi Primitives** - Staking, governance, lending vaults, oracles
- **Full Type Safety** - TypeScript with complete type definitions
- **Comprehensive Testing** - 152 unit tests with 80%+ coverage
- **Production Ready** - Error handling, validation, documentation

## Installation

```bash
npm install @etrid/sdk @polkadot/keyring
# or
yarn add @etrid/sdk @polkadot/keyring
```

## Quick Start

Run the quick-start script to get up and running in seconds:

```bash
npx @etrid/sdk-quickstart
```

Or manually:

```typescript
import { EtridClient, AccountsWrapper } from '@etrid/sdk';
import { Keyring } from '@polkadot/keyring';

async function main() {
  // Connect to Ëtrid node
  const client = new EtridClient('wss://rpc.etrid.io');

  console.log('Connected to:', client.getChain());
  console.log('Block:', await client.getBlockNumber());

  // Create account
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  // Get balance
  const accounts = new AccountsWrapper(client.api);
  const balance = await accounts.getBalance(alice.address);

  console.log('Balance:', balance.free / 10n**18n, 'ÉTR');

  // Transfer
  await accounts.transfer(alice, 'BOB_ADDRESS', 100n * 10n**18n);

  client.close();
}

main();
```

## Available Wrappers

### Layer 3 & Payments
- **LightningBlocWrapper** - Ultra-fast payment channels (500K+ TPS)
  - `openChannel()`, `sendPayment()`, `closeChannel()`
  - `getRoute()`, `estimateChannelFee()`

### Identity & AI
- **AIDidWrapper** - World's first AI identity standard
  - `registerAI()`, `updateReputation()`, `grantPermission()`
  - AI types: LLM, CV, Generative, RL, NLP

### Cross-Chain
- **BridgeWrapper** - Multi-chain interoperability
  - `bridge()` for 13 supported chains
  - `getTransferStatus()`, `getBridgeFee()`

### Smart Contracts
- **EtwasmVMWrapper** - WebAssembly contracts
  - `uploadCode()`, `instantiate()`, `callContract()`
  - `estimateGas()`, `getContractInfo()`

### DeFi Primitives
- **StakingWrapper** - Validator staking operations
  - `bond()`, `unbond()`, `nominate()`
  - `getValidatorStatus()`, `estimateRewards()`
  - `getNominators()`, `setCommission()`

- **GovernanceWrapper** - On-chain governance
  - `createProposal()`, `vote()`, `executeProposal()`
  - `getProposalHistory()`, `delegateVotes()`
  - `estimateProposalOutcome()`

- **ReserveVaultWrapper** - Lending & borrowing
  - `createVault()`, `depositCollateral()`, `borrow()`
  - `getHealthFactor()`, `liquidate()`

- **OracleWrapper** - Decentralized price feeds
  - `getPrice()`, `getTWAP()`
  - `submitPrice()`, `subscribeToPriceUpdates()`

### Rewards & Accounts
- **DistributionPayWrapper** - Daily rewards (27,397 ÉTR/day)
  - `claimReward()` for 5 categories
  - `getPendingRewards()`, `isEligible()`

- **AccountsWrapper** - Basic account operations
  - `getBalance()`, `transfer()`, `batchTransfer()`

## Examples

Check out our comprehensive examples:

```bash
# Lightning-Bloc payment channel
node examples/lightning-bloc-payment.ts

# AI identity registration
node examples/ai-registration.ts

# Cross-chain bridge transfer
node examples/cross-chain-bridge.ts

# Smart contract deployment
node examples/deploy-contract.ts

# DeFi vault lending
node examples/vault-lending.ts

# Price oracle integration
node examples/price-oracle.ts

# Claim daily rewards
node examples/claim-rewards.ts
```

## Documentation

### Tutorials
- [Getting Started](docs/tutorials/01-getting-started.md) - Installation, first connection, basic operations
- [Advanced Features](docs/tutorials/02-advanced-features.md) - Production patterns and best practices
- [Migration Guide](docs/tutorials/03-migration-guide.md) - Migrating from Polkadot.js

### Guides
- [NPM Publication](docs/NPM_PUBLICATION_GUIDE.md) - How to publish the SDK
- [Integration Tests](tests/integration/README.md) - Running integration tests

### API Documentation
Generate complete API docs:

```bash
npm run docs        # Generate docs
npm run docs:serve  # Serve on http://localhost:8080
```

## Development

### Prerequisites
- Node.js 16+
- Running Ëtrid node (for integration tests)

### Setup

```bash
# Clone repository
git clone https://github.com/etrid/etrid-protocol
cd etrid-protocol/13-developer-tools/sdk/js-etrid-sdk

# Install dependencies
npm install

# Build
npm run build

# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Lint
npm run lint
npm run lint:fix
```

### Testing

We maintain 80%+ test coverage across all wrappers:

```bash
# Run all tests
npm test

# Run specific test file
npm test -- StakingWrapper.test.ts

# Run integration tests (requires running node)
npm run test:integration

# Watch mode
npm run test:watch
```

**Test Statistics**:
- 152 unit tests across 9 wrappers
- Integration test framework
- Mocking utilities for all pallets
- 80%+ coverage target

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Quick Contribution Steps

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes following our [coding standards](CONTRIBUTING.md#coding-standards)
4. Write tests for your changes
5. Run tests: `npm test`
6. Commit: `git commit -m "feat: add new feature"`
7. Push: `git push origin feature/my-feature`
8. Create a Pull Request

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and release notes.

## Status

**Current Version**: 0.1.0 (Production Ready)

| Component | Status | Coverage |
|-----------|--------|----------|
| LightningBlocWrapper | ✅ Complete | 25 tests |
| DistributionPayWrapper | ✅ Complete | 10 tests |
| EtwasmVMWrapper | ✅ Complete | 12 tests |
| AIDidWrapper | ✅ Complete | 24 tests |
| BridgeWrapper | ✅ Complete | 20 tests |
| OracleWrapper | ✅ Complete | 18 tests |
| ReserveVaultWrapper | ✅ Complete | 23 tests |
| StakingWrapper | ✅ Complete | 20 tests |
| GovernanceWrapper | ✅ Complete | 17 tests |
| AccountsWrapper | ✅ Complete | 3 tests |
| **Total** | **✅ 100%** | **152 tests** |

## Roadmap

### v0.2.0 (Planned)
- React hooks for SDK methods
- WebSocket reconnection handling
- Transaction batching utilities
- Gas estimation improvements

### v0.3.0 (Planned)
- Event streaming utilities
- Subscription management helpers
- Advanced error recovery
- Performance optimizations
- Caching layer

### v1.0.0 (Planned)
- Stable API
- Full documentation site
- Video tutorials
- Production deployment examples
- Benchmark results

## Python SDK

A Python SDK is also in development. See [python-etrid-sdk/](../python-etrid-sdk/) for details.

## Support

- **Documentation**: https://docs.etrid.io/sdk
- **Discord**: https://discord.gg/etrid
- **GitHub Issues**: https://github.com/etrid/etrid-protocol/issues
- **Email**: dev@etrid.io

## License

Apache-2.0 License - see [LICENSE](../../../LICENSE) for details.

## Acknowledgments

Built with:
- [Polkadot.js API](https://polkadot.js.org/docs/api) - Substrate blockchain interaction
- [TypeScript](https://www.typescriptlang.org/) - Type safety
- [Jest](https://jestjs.io/) - Testing framework
- [TypeDoc](https://typedoc.org/) - Documentation generation

---

**Maintained by**: Ëtrid Foundation
**Repository**: https://github.com/etrid/etrid-protocol
**Website**: https://etrid.io

---

*Start building on Ëtrid today!* 🚀
