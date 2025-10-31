# Changelog

All notable changes to the Etrid blockchain will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete documentation package (16,251+ lines)
- User Guide for beginners
- API Reference for all 8 pallets
- Operator Guide for validators and watchtowers
- Developer Guide for building on Etrid
- 5 video tutorial scripts (49 minutes total)
- Build automation script (build-all.sh)
- Test automation script (test-all.sh)
- Testnet orchestration script (start-testnet.sh)
- Deployment automation script (deploy-all.sh)
- Documentation generation script (generate-docs.sh)
- Makefile for common development tasks
- GitHub Actions CI/CD pipeline
- Docker Compose development environment
- Contributing guidelines
- Quick reference card

## [1.0.0] - 2025-10-22

### Added
- FlareChain relay chain with ASF consensus
- 13 Partition Burst Chains (PBCs) for cross-chain interoperability
- Lightning-Bloc Layer 2 payment channels
- 8 custom pallets:
  - pallet-reserve-oracle
  - pallet-reserve-vault
  - pallet-circuit-breaker
  - pallet-custodian-registry
  - pallet-xcm-bridge
  - pallet-validator-committee
  - pallet-did-registry
  - pallet-aidid
- JavaScript/TypeScript SDK
- Wallet web application
- Validator dashboard
- Watchtower monitor application
- Comprehensive test suite (412+ tests)
- Security features (reentrancy protection, circuit breakers)
- On-chain governance system
- Annual Consensus Day mechanism

### Security
- Multi-signature support
- Account recovery mechanisms
- Reentrancy protection in ÉtwasmVM
- Circuit breaker emergency stops
- Watchtower fraud detection

## [0.9.0] - 2025-10-15

### Added
- Initial testnet deployment
- Beta testing program
- Community feedback integration

## [0.8.0] - 2025-10-01

### Added
- Cross-chain message passing (XCMP)
- PBC collator implementations
- Light client protocols

## [0.7.0] - 2025-09-15

### Added
- EDSC stablecoin system
- Reserve oracle aggregation
- Custodian registry

[Unreleased]: https://github.com/etrid/etrid/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/etrid/etrid/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/etrid/etrid/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/etrid/etrid/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/etrid/etrid/releases/tag/v0.7.0
