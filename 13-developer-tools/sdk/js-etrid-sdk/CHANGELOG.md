# Changelog

All notable changes to the Ëtrid SDK will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- TypeDoc configuration for API documentation generation
- Comprehensive tutorial series (Getting Started, Advanced Features, Migration)
- NPM publication guide
- Integration test framework
- Python SDK foundation

## [0.1.0] - 2025-11-16

### Added

#### Core Wrappers
- **LightningBlocWrapper** - Layer 3 payment channels (500K+ TPS)
  - `openChannel()`, `sendPayment()`, `closeChannel()`
  - `getChannel()`, `getRoute()`, `estimateChannelFee()`
  - Channel monitoring and event subscriptions
  
- **DistributionPayWrapper** - Daily rewards distribution (27,397 ÉTR/day)
  - `claimReward()` for all 5 categories
  - `getPendingRewards()`, `getDistributionSchedule()`
  - `isEligible()`, `getEligibleCategories()`
  
- **EtwasmVMWrapper** - WebAssembly smart contracts
  - `uploadCode()`, `instantiate()`, `deployContract()`
  - `callContract()`, `queryContract()`
  - `estimateGas()`, `getContractInfo()`
  
- **AIDidWrapper** - AI identity standard (world's first)
  - `registerAI()`, `getAIProfile()`, `updateAIMetadata()`
  - `updateReputation()`, `getReputationTier()`
  - `grantPermission()`, `revokePermission()`
  
- **BridgeWrapper** - Cross-chain transfers (13 chains)
  - `bridge()` for cross-chain transfers
  - `getTransferStatus()`, `getSupportedChains()`
  - `getBridgeFee()`, `estimateBridgeTime()`
  
- **OracleWrapper** - Decentralized price feeds
  - `getPrice()`, `getPriceWithMetadata()`
  - `getTWAP()` - Time-weighted average price
  - `submitPrice()`, `subscribeToPriceUpdates()`
  
- **ReserveVaultWrapper** - DeFi lending and borrowing
  - `createVault()`, `depositCollateral()`, `borrow()`
  - `repay()`, `getHealthFactor()`, `isLiquidatable()`
  - `liquidate()`, `calculateBorrowLimit()`
  
- **StakingWrapper** - Validator staking operations
  - `bond()`, `unbond()`, `nominate()`
  - `getValidatorStatus()`, `getStakingInfo()`
  - `getNominators()`, `getCommissionHistory()`
  - `setCommission()`, `getNetworkStats()`
  - `estimateRewards()` with APY calculation
  
- **GovernanceWrapper** - On-chain governance
  - `createProposal()`, `vote()`, `executeProposal()`
  - `getActiveProposals()`, `getProposalResults()`
  - `getProposalHistory()`, `getDelegations()`
  - `getVotingStatistics()`, `getProposalTimeline()`
  - `estimateProposalOutcome()`, `delegateVotes()`
  - `getGovernanceStats()`
  
- **AccountsWrapper** - Basic account operations
  - `getBalance()`, `transfer()`, `transferWithMemo()`
  - `batchTransfer()`, `getAccountInfo()`

#### Testing
- Complete Jest configuration with coverage thresholds (80%+)
- Test utilities and mocks (`tests/utils/testHelpers.ts`)
- 152 unit tests across all wrappers:
  - LightningBlocWrapper: 25 tests
  - DistributionPayWrapper: 10 tests
  - EtwasmVMWrapper: 12 tests
  - AIDidWrapper: 24 tests
  - BridgeWrapper: 20 tests
  - OracleWrapper: 18 tests
  - ReserveVaultWrapper: 23 tests
  - StakingWrapper: 20 tests
  - GovernanceWrapper: 17 tests

#### Examples
- 7 comprehensive examples (2,094 lines total):
  1. Lightning-Bloc payment channels (`lightning-bloc-payment.ts`)
  2. Claim daily rewards (`claim-rewards.ts`)
  3. Deploy smart contracts (`deploy-contract.ts`)
  4. AI registration (`ai-registration.ts`)
  5. Cross-chain bridge (`cross-chain-bridge.ts`)
  6. Price oracles (`price-oracle.ts`)
  7. DeFi vault lending (`vault-lending.ts`)
- Complete README with setup and usage instructions

#### Documentation
- Comprehensive inline JSDoc for all methods
- TypeScript type definitions for all interfaces
- SDK Implementation Plan (1,050 lines)
- Test Plan with 152 test case specifications
- Tutorial 1: Getting Started
- Tutorial 2: Advanced Features
- Tutorial 3: Migration Guide (Polkadot.js → Ëtrid)
- NPM Publication Guide
- Integration Tests README

#### Error Handling
- Custom error classes for all domains:
  - `NotConnectedError`, `InvalidAddressError`
  - `ChannelError`, `RouteNotFoundError`
  - `DistributionError`, `NotEligibleError`
  - `EtwasmError`, `ContractNotFoundError`
  - `AIDidError`, `InvalidProfileError`
  - `BridgeError`, `UnsupportedChainError`
  - `OracleError`, `PriceNotFoundError`
  - `VaultError`, `InsufficientCollateralError`
  - `StakingError`, `GovernanceError`

#### Type Definitions
- Full TypeScript support with exported types
- Generic types for flexibility
- Enum definitions for constants:
  - `AIType`, `ReputationTier`
  - `SupportedChain`, `BridgeStatus`
  - `DistributionCategory`, `VaultStatus`

#### Build & Package
- TypeScript compilation configuration
- ESLint configuration
- Package.json with all scripts
- Index exports for all wrappers
- NPM-ready package structure

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed
- N/A (initial release)

### Security
- Input validation on all wrapper methods
- Address validation using Substrate utilities
- Safe BigInt handling for all amounts
- No hardcoded private keys or sensitive data

## [0.0.1] - 2025-10-01

### Added
- Initial project structure
- Basic client connection
- Preliminary wrapper interfaces

---

## Version History

- **0.1.0** (2025-11-16) - First production release
  - 10 complete wrappers
  - 152 unit tests
  - 7 examples
  - Comprehensive documentation
  
- **0.0.1** (2025-10-01) - Initial setup

---

## Upcoming Features

### [0.2.0] - Planned
- Integration tests for all wrappers
- React hooks for SDK methods
- WebSocket reconnection handling
- Transaction batching utilities
- Gas estimation improvements

### [0.3.0] - Planned
- Event streaming utilities
- Subscription management helpers
- Advanced error recovery
- Performance optimizations
- Caching layer

### [1.0.0] - Planned
- Stable API
- Full documentation site
- Video tutorials
- Production deployment examples
- Benchmark results

---

## Migration Guides

### From Polkadot.js to Ëtrid SDK v0.1.0

See [docs/tutorials/03-migration-guide.md](./docs/tutorials/03-migration-guide.md)

### From v0.0.1 to v0.1.0

**Breaking Changes**:
- Complete rewrite with new wrapper architecture
- All methods now return Promises
- BigInt used for all token amounts

**Migration Steps**:
1. Update import paths
2. Replace direct API calls with wrapper methods
3. Convert amounts to BigInt
4. Update error handling

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development workflow and guidelines.

---

## Support

- **Documentation**: https://docs.etrid.io/sdk
- **Discord**: https://discord.gg/etrid
- **GitHub Issues**: https://github.com/etrid/etrid-protocol/issues
- **Email**: dev@etrid.io

---

**Maintained by**: Ëtrid Foundation  
**License**: Apache-2.0
