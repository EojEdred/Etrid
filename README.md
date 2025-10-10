# Ëtrid Blockchain

**The E³20 Multichain Ecosystem**

Ëtrid is a next-generation blockchain platform featuring:

- **Flare Chain**: Main consensus chain with ASF (Adaptive State Finality)
- **Partition Burst Chains (PBCs)**: 12 parallel side chains for scalability
- **Lightning Bloc Networks**: State channel bridges to external blockchains
- **Consensus Day**: Annual decentralized governance event
- **ÉTR, ETD, VMw**: Native multi-token economy

## Repository Structure

This repository contains the complete Ëtrid blockchain implementation:

- **runtime/**: Blockchain runtime (Flare Chain + PBC runtime)
- **node/**: Node implementation and CLI
- **pallets/**: Framework pallets (Substrate + Cosmos adapted)
- **contracts/**: Smart contracts and EtwasmVM
- **network/**: P2P networking and cross-chain bridges
- **identity/**: OpenDID decentralized identity system
- **client/**: SDK libraries (JavaScript, Rust, Swift)
- **apps/**: User-facing applications (wallets, governance UI)
- **tools/**: Developer tools and utilities
- **infra/**: Infrastructure, Docker, and deployment configs
- **docs/**: Comprehensive documentation

## Quick Start

```bash
# Build the blockchain
./scripts/build.sh

# Start local testnet (if script exists)
./scripts/start-testnet.sh

# Run governance UI
cd apps/governance-ui && npm install && npm run dev
```

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - System design
- [Getting Started](docs/GETTING_STARTED.md) - Development setup
- [Whitepaper](docs/WHITEPAPER.md) - Technical specification
- [API Reference](docs/API_REFERENCE.md) - RPC/SDK documentation
- [Contributing](docs/CONTRIBUTING.md) - Contribution guidelines

## Token Economics

- **ÉTR (Ëtrid)**: Native currency for transactions, staking, and governance
- **ETD (Ëtrid Dollar)**: Algorithmic stablecoin
- **VMw (VM Watts)**: Gas units for smart contract execution

Initial supply: 1,000,000,000 ÉTR

## Development

```bash
# Build all Rust components
cargo build --release

# Run tests
cargo test --all

# Format code
cargo fmt --all
```

## Community

- **Website**: https://etrid.io
- **Discord**: https://discord.gg/etrid
- **Twitter**: [@EtridChain](https://twitter.com/EtridChain)
- **Telegram**: https://t.me/etridchain

## License

GPL-3.0 - See [LICENSE](LICENSE)

## Authors

Ëtrid Foundation - Building the decentralized future
