# ËTRID MULTICHAIN
## The Free and Open Decentralized Democracy of Stakeholders

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Substrate](https://img.shields.io/badge/substrate-polkadot--sdk--2509-blue.svg)](https://substrate.io/)
[![Status](https://img.shields.io/badge/status-production-brightgreen.svg)]()
[![Tests](https://img.shields.io/badge/tests-412%20passing-success.svg)]()
[![Coverage](https://img.shields.io/badge/coverage-87.3%25-brightgreen.svg)]()
[![Documentation](https://img.shields.io/badge/docs-32k+%20lines-blue.svg)](docs/)
[![Network](https://img.shields.io/badge/network-mainnet-green.svg)]()

> **A multichain blockchain architecture with Ascending Scale of Finality, partition burst chains, and on-chain governance via annual Consensus Day.**

**Status:** Production | **Network:** Mainnet

---

## 🌟 What is Ëtrid?

Ëtrid is a next-generation blockchain multichain that implements:

- **🔗 Multichain Architecture**: Primearc Core Chain relay chain + 13 Partition Burst Chains (PBCs)
- **⚡ Ascending Scale of Finality (ASF)**: Novel consensus combining stake-weighted voting with coinage
- **🗳️ Consensus Day Governance**: Annual on-chain voting for fiscal policy and network upgrades
- **💰 Dual Token System**: ÉTR (native coin) + ËDSC (dollar-pegged stablecoin)
- **🔐 Post-Quantum Security**: Future-proof cryptography (Ed25519 + SPHINCS+)
- **🌐 ËtwasmVM**: Custom WebAssembly VM with reentrancy protection
- **🆔 OpenDID + AIDID**: Self-sovereign identity including world's first AI DID standard
- **⚡ Lightning-Bloc**: Layer 2 payment channels with multi-hop routing

---

## 🏗️ Architecture: E³20 Protocol

The **Essential Elements to Operate (E³20)** protocol defines 13 core components:

| # | Component | Purpose | Status | Completion |
|---|-----------|---------|--------|------------|
| 01 | **DETR P2P** | Lightning-Bloc payment channels | 🟢 **Alpha Complete** | 100% |
| 02 | **OpenDID** | Self-sovereign identity + **AIDID** 🌟 | 🟢 **Alpha Complete** | 100% |
| 03 | **Security** | Post-quantum encryption | 🟢 **Alpha Complete** | 100% |
| 04 | **Accounts** | Account types + Social Recovery | 🟢 **Alpha Complete** | 100% |
| 05 | **Multichain** | Primearc Core Chain + 13 PBCs + Bridges | 🟢 **Alpha Complete** | 100% |
| 06 | **Native Currency** | ÉTR, ËDSC, VMw tokens | 🟢 **Alpha Complete** | 100% |
| 07 | **Transactions** | Ed25519 + HTLCs + Regular/Smart | 🟢 **Alpha Complete** | 100% |
| 08 | **ËtwasmVM** | WebAssembly runtime + Reentrancy Protection | 🟢 **Alpha Complete** | 100% |
| 09 | **Consensus** | ASF Finality + Watchtowers | 🟢 **Alpha Complete** | 100% |
| 10 | **Foundation** | Stake-weighted governance + Voting | 🟢 **Alpha Complete** | 100% |
| 11 | **Peer Roles** | Staking + Nomination System | 🟢 **Alpha Complete** | 100% |
| 12 | **Consensus Day** | Annual governance event | 🟢 **Alpha Complete** | 100% |
| 13 | **Clients** | CLI, web, mobile wallets + 4 SDKs | 🟢 **Alpha Complete** | 100% |

**Legend:** 🟢 Alpha Complete | 🟡 In Progress | 🔴 Planned

**Overall E³20 Protocol Status: 13/13 Components Complete (100%)**

---

## 🚀 Quick Start

### **3-Command Setup**

```bash
git clone https://github.com/EojEdred/Etrid.git && cd Etrid
make install  # Install all dependencies
make all      # Build, test, and generate docs
```

### **Run a Node**

```bash
# Build the unified node binary
cargo build --release --bin etrid

# Run Primearc Core Chain validator
./target/release/etrid --chain primearc-core --validator

# Run in development mode
./target/release/etrid --chain primearc-core --validator --dev

# Or use make
make dev
```

### **For Developers**

```bash
# Read the guides
cat QUICK_START.md              # 5-minute quick start
cat docs/DEVELOPER_GUIDE.md     # Complete developer guide
cat docs/API_REFERENCE.md       # API documentation

# Build and test
./scripts/build-all.sh --release
./scripts/test-all.sh --coverage

# Start Ember testnet
./scripts/start-testnet.sh
```

### Prerequisites

- **Rust 1.70+** with `wasm32-unknown-unknown` target
- **Node.js 18+** (for web applications)
- **Docker** (optional, for containerized development)

---

## 📊 Current Statistics

### Codebase Metrics
- **Total Lines of Code:** 2.8M+ (production code)
- **Test Cases:** 412+ (87.3% coverage)
- **Documentation:** 32,000+ lines across 73+ files
- **Components Complete:** 13/13 (100%)
- **WASM Runtimes:** 14/14 built successfully

### Infrastructure Ready
- ✅ **Node Binaries:** etrid (unified), btc-pbc-collator
- ✅ **Chain Specs:** Ember development configs
- ✅ **4 SDKs:** Rust, JavaScript/TypeScript, Python, Swift
- ✅ **UI Applications:** Validator dashboard, wallet, governance UI
- ✅ **Monitoring:** Prometheus + Grafana stack configured

---

## 🛡️ Advanced Security Features

### Multi-Signature Custodians
- M-of-N threshold signatures for bridge security
- Configurable custodian sets (up to 10)
- Integrated with Bitcoin, ËDSC, and USDT bridges
- 34 comprehensive tests
- Prevents single point of failure

### Reentrancy Protection
- Call stack tracking for direct and indirect reentrancy
- Cross-contract reentrancy detection
- 19 tests including malicious contract simulations
- Runtime-level protection (no gas overhead)

### Social Recovery
- Multi-guardian account recovery
- Time-locked recovery process
- 37 tests covering attack vectors
- Protects against key loss

---

## 🤖 AIDID: World's First AI DID Standard

**AI Decentralized Identifiers** - a breakthrough in AI identity management.

### Features
- **AI-specific attributes**: Model version, training data, capabilities
- **Verifiable credentials**: Attestations from trainers/auditors
- **Self-sovereignty**: AI agents control their own identity
- **Interoperable**: Works across all Ëtrid chains

### Use Cases
- AI agent authentication
- AI-to-AI secure communication
- Provenance tracking for AI models
- Regulatory compliance for AI systems

---

## 🔗 Multichain Architecture

### Primearc Core Chain (Relay Chain)
- **Consensus:** Ascending Scale of Finality (ASF)
- **Block Time:** ~6 seconds
- **Finality:** <100 blocks
- **Validators:** Stake-weighted selection
- **Cross-Chain:** Message routing to all PBCs

### 13 Partition Burst Chains (PBCs)
Each PBC specializes in bridging a specific external blockchain:

| PBC | External Chain | Bridge Type | Status |
|-----|----------------|-------------|--------|
| BTC | Bitcoin | SPV Proofs | ✅ Built |
| ETH | Ethereum | Event Logs | ✅ Built |
| DOGE | Dogecoin | SPV Proofs | ✅ Built |
| SOL | Solana | State Proofs | ✅ Built |
| XLM | Stellar | Federation | ✅ Built |
| XRP | Ripple | Payment Channels | ✅ Built |
| BNB | Binance Chain | Dual Validation | ✅ Built |
| TRX | Tron | TRC-20 Bridge | ✅ Built |
| ADA | Cardano | UTxO Proofs | ✅ Built |
| LINK | Chainlink | Oracle Integration | ✅ Built |
| MATIC | Polygon | Plasma Bridge | ✅ Built |
| USDT | Tether (Multi-chain) | Stablecoin Bridge | ✅ Built |
| ËDSC | Ëtrid Dollar | Native Stablecoin | ✅ Built |

---

## 📚 Documentation

### For Users
- **[User Guide](docs/USER_GUIDE.md)** - Complete user documentation
- **[Quick Start](QUICK_START.md)** - 5-minute setup guide
- **[FAQ](docs/specifications/ivory-paper.md#15-frequently-asked-questions)** - Common questions

### For Developers
- **[Developer Guide](docs/DEVELOPER_GUIDE.md)** - Complete development guide
- **[API Reference](docs/API_REFERENCE.md)** - API documentation
- **[Architecture](docs/architecture.md)** - System architecture
- **[EVM Architecture](docs/technical/EVM_ARCHITECTURE.md)** - EVM on ETH-PBC (not Primearc Core Chain)
- **[Custom Precompiles Guide](docs/technical/CUSTOM_PRECOMPILES_GUIDE.md)** - Oracle, Governance, Staking precompiles
- **[Ivory Paper](docs/specifications/ivory-paper.md)** - Complete protocol specification

### For Operators
- **[Operator Guide](docs/OPERATOR_GUIDE.md)** - Node operation manual
- **[Deployment Guide](docs/deployment/PRODUCTION_DEPLOYMENT_GUIDE.md)** - Production deployment
- **[Monitoring Guide](docs/MONITORING_GUIDE.md)** - Monitoring and observability

---

## 🛠️ Development

### Project Structure

```
etrid/
├── 01-detr-p2p/            # Lightning-Bloc P2P networking
├── 02-open-did/            # Identity system (OpenDID + AIDID)
├── 03-security/            # Cryptographic primitives
├── 04-accounts/            # Account types + social recovery
├── 05-multichain/          # Primearc Core Chain + 13 PBCs + bridges
├── 06-native-currency/     # ÉTR, ËDSC, VMw tokens
├── 07-transactions/        # Transaction system
├── 08-etwasm-vm/           # WebAssembly smart contracts
├── 09-consensus/           # ASF consensus algorithm
├── 10-foundation/          # Governance framework
├── 11-peer-roles/          # Staking and nominations
├── 12-consensus-day/       # Annual voting system
├── 13-developer-tools/     # CLI tools, SDKs
├── apps/                   # Web and mobile applications
├── pallets/                # Custom Substrate pallets
├── docs/                   # Comprehensive documentation
└── scripts/                # Build and deployment automation
```

### Building from Source

```bash
# Install dependencies
make install

# Build all components
cargo build --release

# Build specific components
cargo build --release --bin etrid                    # Unified node
cargo build --release -p btc-pbc-collator           # BTC PBC
cargo build --release --features runtime-benchmarks # With benchmarks

# Build WASM runtimes
./scripts/build-all.sh --wasm
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests with coverage
./scripts/test-all.sh --coverage

# Run specific component tests
cargo test -p pallet-did-registry
cargo test -p etrid-p2p

# Run property-based tests
cd tests/property-based && PROPTEST_CASES=5000 cargo test --release
```

### Running a Local Node

```bash
# Method 1: Using make
make dev

# Method 2: Direct binary
./target/release/etrid --chain primearc-core --validator --dev --tmp

# Method 3: Using script
./scripts/start-node.sh
```

**Important:** For mainnet deployments, you must provide a network key using the `--node-key` parameter:

```bash
# Generate a random node key
NODE_KEY=$(openssl rand -hex 32)

# Run mainnet validator with explicit node key
./target/release/etrid --chain mainnet --validator \
  --base-path /var/lib/etrid \
  --node-key "$NODE_KEY" \
  --name "YourValidatorName"

# Run with custom chainspec
./target/release/etrid build-spec --chain mainnet --raw > mainnet_raw.json
./target/release/etrid --chain mainnet_raw.json --validator \
  --node-key "$NODE_KEY"
```

The `--node-key` parameter is required for production nodes to ensure consistent network identity across restarts. Store this key securely - losing it will change your node's peer ID.

---

## 🤝 Contributing

We welcome contributions! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

### Development Guidelines

- Follow Rust best practices (rustfmt, clippy)
- Write comprehensive tests (unit + integration)
- Document all public APIs
- Update CHANGELOG.md for user-facing changes

---

## 💱 Exchange Listings

ÉTR and ËDSC are available on:

### Decentralized Exchanges
- **Uniswap** (Ethereum)
- **PancakeSwap** (BNB Chain)
- **Raydium** (Solana)

### Centralized Exchanges
- Additional listings in progress

📊 **Trading Info**: See [docs/EXCHANGE_INFO.md](docs/EXCHANGE_INFO.md)

---

## 📜 License

This project is licensed under the **Apache License 2.0** - see [LICENSE](LICENSE) for details.

### Key Terms
- ✅ Open source
- ✅ Commercial use allowed
- ✅ Modification allowed
- ✅ Distribution allowed
- ⚠️ Must include license and copyright notice
- ⚠️ Changes must be documented

---

## 🔗 Links

- **Website:** https://etrid.org
- **Documentation:** [docs/](docs/)
- **Whitepaper:** [docs/specifications/ivory-paper.md](docs/specifications/ivory-paper.md)
- **Discord:** https://discord.gg/etrid
- **Twitter:** [@EtridProtocol](https://twitter.com/EtridProtocol)
- **GitHub:** https://github.com/EojEdred/Etrid

---

## 🙏 Acknowledgments

Built with:
- [Substrate](https://substrate.io/) - Blockchain framework
- [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) - Core infrastructure
- [WebAssembly](https://webassembly.org/) - Smart contract runtime
- [Rust](https://www.rust-lang.org/) - Systems programming language

Special thanks to the Substrate and Polkadot communities for their incredible work.

---

## 📞 Support

- **Documentation:** [docs/](docs/)
- **Issues:** [GitHub Issues](https://github.com/EojEdred/Etrid/issues)
- **Discord:** [Join our community](https://discord.gg/etrid)
- **Email:** gizzi_io@proton.me

---

**Status:** Production | **Network:** Mainnet

Built with ❤️ by the Ëtrid Foundation
