# ËTRID MULTICHAIN
## The Free and Open Decentralized Democracy of Stakeholders

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Substrate](https://img.shields.io/badge/substrate-polkadot--sdk-blue.svg)](https://substrate.io/)
[![Status](https://img.shields.io/badge/status-alpha-red.svg)]()

> **A multichain blockchain architecture with Adaptive Stake Finality, partition burst chains, and on-chain governance via annual Consensus Day.**

---

## 🌟 What is Ëtrid?

Ëtrid is a next-generation blockchain multichain that implements:

- **🔗 Multichain Architecture**: Main Flare Chain + unlimited Partition Burst Chains (PBCs)
- **⚡ Adaptive Stake Finality (ASF)**: Novel consensus algorithm combining stake-weighted voting with coinage
- **🗳️ Consensus Day Governance**: Annual on-chain voting event for fiscal policy and network upgrades
- **💰 Dual Token System**: ÉTR (native coin) + EDSC (dollar-pegged stablecoin)
- **🔐 Post-Quantum Security**: Future-proof cryptography
- **🌐 ËtwasmVM**: Custom WebAssembly VM for smart contracts
- **🆔 OpenDID**: Self-sovereign identity system

---

## 🏗️ Architecture: E³20 Protocol

The **Essential Elements to Operate (E³20)** protocol defines 13 core components:

| # | Component | Purpose | Status |
|---|-----------|---------|--------|
| 01 | **DETR p2p** | Multi-protocol peer networking | 🟡 In Progress |
| 02 | **OpenDID** | Self-sovereign identity | 🔴 Planned |
| 03 | **Security** | Post-quantum encryption | 🔴 Planned |
| 04 | **Accounts** | Account types (EBCA, RCA, SCA) | 🟢 Alpha |
| 05 | **Multichain** | Flare Chain + PBCs | 🟢 Alpha |
| 06 | **Crypto** | ÉTR, EDSC, VMw tokens | 🟡 In Progress |
| 07 | **Transactions** | Regular, smart, cross-chain | 🟡 In Progress |
| 08 | **ËtwasmVM** | WebAssembly runtime | 🟢 Alpha |
| 09 | **Consensus** | ASF Finality algorithm | 🟢 Alpha |
| 10 | **Foundation** | Legal/organizational DAO | 🟡 In Progress |
| 11 | **Roles** | Peer roles and permissions | 🟡 In Progress |
| 12 | **Governance** | Consensus Day voting | 🟢 Alpha |
| 13 | **Clients** | CLI, web, mobile wallets | 🟡 In Progress |

**Legend:** 🟢 Alpha | 🟡 In Progress | 🔴 Planned

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+ (for web apps)
- Flutter 3.0+ (for mobile wallet)

### Clone & Build

```bash
# Clone repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid

# Install Rust dependencies
rustup update
rustup target add wasm32-unknown-unknown

# Build workspace (when dependencies stabilize)
cargo build --release

# Run tests
cargo test --workspace
```

### Current Status: Known Issues
⚠️ **Note:** Rust compilation currently blocked by polkadot-sdk dependency instability. See [KNOWN_ISSUES.md](KNOWN_ISSUES.md) for details.

**Non-Rust components work now:**
- ✅ Documentation
- ✅ Mobile wallet (Flutter)
- ✅ Web UI (React)
- ✅ CLI tools

---

## 💎 Core Features

### 1. Adaptive Stake Finality (ASF)
Novel consensus mechanism that:
- Combines Proof of Stake with "coinage" (time × stake)
- Dilutes voting power over time to prevent centralization
- No mining, energy-efficient
- Fast finality (3-5 seconds)

### 2. Partition Burst Chains (PBCs)
Unlimited sidechains that:
- Run parallel to main Flare Chain
- Process high-throughput transactions
- Periodically merge state to Flare Chain
- Enable application-specific chains

### 3. Consensus Day
Annual on-chain governance event:
- Vote on fiscal policy (inflation rate, distribution)
- Approve network upgrades
- Mint new supply based on vote outcomes
- Fully on-chain, transparent, democratic

### 4. Dual Token Economics
- **ÉTR**: Native coin for transactions, staking, governance
- **EDSC**: USD-pegged stablecoin for stable payments
- **VMw (VMwattage)**: Gas token for smart contract execution

### 5. Account Types
- **EBCA**: External Blockchain Accounts (standard wallets)
- **RCA**: Regular Contract Accounts (basic smart contracts)
- **SCA**: Smart Contract Accounts (full EVM compatibility)
- **SDCA**: Stake Deposit Contract Accounts (staking)

---

## 📖 Documentation

- **[Whitepaper](docs/whitepaper/)** - Full technical specification
- **[Architecture Guide](docs/architecture/ARCHITECTURE.md)** - E³20 component details
- **[API Reference](docs/api/)** - Pallet APIs and RPC methods
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute
- **[Known Issues](KNOWN_ISSUES.md)** - Current blockers and workarounds
- **[Roadmap](docs/ROADMAP.md)** - Development timeline

---

## 🗂️ Repository Structure

```
etrid/
├── 01-detr-p2p/              # Networking layer
├── 02-opendid/               # Identity system
├── 03-security/              # Cryptography
├── 04-accounts/              # Account management
│   └── pallet/               # FRAME pallet
├── 05-multichain/            # Multichain logic
│   ├── primitives/           # Core types
│   └── partition-burst-chains/ # PBC implementation
├── 06-crypto/                # Token economics
├── 07-transactions/          # Transaction processing
├── 08-etwasm-vm/             # WebAssembly runtime
│   └── pallet/               # VM pallet
├── 09-consensus/             # ASF consensus
│   └── pallet/               # Consensus pallet
├── 10-foundation/            # DAO governance
│   └── governance/pallet/    # Governance pallet
├── 11-roles/                 # Peer role management
├── 12-governance/            # Consensus Day logic
├── 13-clients/               # User interfaces
│   ├── cli/                  # Command-line tools
│   ├── web/                  # Web dashboard
│   └── mobile/               # Mobile wallet
├── apps/                     # Frontend applications
├── docs/                     # Documentation
└── scripts/                  # Build & deployment scripts
```

---

## 🛠️ Technology Stack

### Core Blockchain
- **Framework**: Substrate (Polkadot SDK)
- **Language**: Rust
- **Runtime**: FRAME pallets
- **VM**: Custom ËtwasmVM (WebAssembly)

### Clients & Apps
- **Web**: React, TypeScript, TailwindCSS
- **Mobile**: Flutter, Dart
- **CLI**: Rust (clap, tokio)

### Infrastructure
- **Networking**: libp2p, QUIC
- **Database**: RocksDB, ParityDB
- **Monitoring**: Prometheus, Grafana

---

## 🌐 Network Details

### Mainnet (Planned)
- **Launch**: Q2 2026 (target)
- **Initial Supply**: 1 billion ÉTR
- **Block Time**: 5 seconds
- **Finality**: 3 blocks (~15 seconds)

### Testnet
- Coming soon
- Faucet available
- Public RPC endpoints

---

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Code of conduct
- Development setup
- Pull request process
- Coding standards

### Areas Needing Help
- 🔴 Frontend development (React, Flutter)
- 🟡 Runtime pallet development (Rust)
- 🟢 Documentation and tutorials
- 🟢 Testing and QA
- 🟢 Community management

---

## 📜 Tokenomics Summary

| Token | Symbol | Purpose | Supply |
|-------|--------|---------|--------|
| Ëtrid Coin | ÉTR | Native token, staking, governance | 1B initial (inflationary) |
| Ëtrid Dollar | EDSC | USD-pegged stablecoin | Minted on demand |
| VMwattage | VMw | Smart contract gas | Burned on use |

**Distribution (Initial 1B ÉTR):**
- 40% - Community airdrop & rewards
- 25% - Ecosystem development fund
- 20% - Foundation reserves
- 10% - Team (4-year vesting)
- 5% - Early investors

---

## 🔐 Security

- **Audits**: Planned for mainnet (CertiK, Trail of Bits)
- **Bug Bounty**: Coming soon
- **Responsible Disclosure**: security@etrid.io

---

## 📞 Community & Support

- **Website**: [etrid.io](https://etrid.io) (coming soon)
- **Twitter**: [@EtridMultichain](https://twitter.com/EtridMultichain)
- **Discord**: [discord.gg/etrid](https://discord.gg/etrid)
- **Telegram**: [t.me/EtridOfficial](https://t.me/EtridOfficial)
- **Email**: hello@etrid.io

---

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

Built with:
- [Substrate](https://substrate.io/) by Parity Technologies
- [Polkadot SDK](https://github.com/paritytech/polkadot-sdk)
- Inspired by Ethereum, Polkadot, Cosmos, and other pioneering blockchain projects

---

## ⚠️ Disclaimer

**Alpha Software**: Ëtrid is under active development. Do not use in production. Test thoroughly. Always practice good security hygiene.

---

<p align="center">
  <strong>Built with ❤️ by the Ëtrid community</strong>
</p>

<p align="center">
  <sub>The Free and Open Decentralized Democracy of Stakeholders</sub>
</p>
