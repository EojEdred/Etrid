# Ëtrid Developer Guide

> Comprehensive guide for developers contributing to the Ëtrid multichain blockchain

**Last Updated:** October 20, 2025
**Version:** 1.0.0

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Development Environment Setup](#development-environment-setup)
3. [Project Structure](#project-structure)
4. [Development Workflow](#development-workflow)
5. [Building & Testing](#building--testing)
6. [Component Development](#component-development)
7. [Common Development Tasks](#common-development-tasks)
8. [Coding Standards](#coding-standards)
9. [Debugging & Troubleshooting](#debugging--troubleshooting)
10. [Contributing](#contributing)

---

## Quick Start

### Prerequisites

**Required:**
- **Rust:** 1.70+ with `wasm32-unknown-unknown` target
- **Node.js:** 18+ (for services and frontend)
- **Git:** Latest version

**Optional:**
- **Go:** 1.21+ (for DETR P2P core)
- **Flutter:** 3.0+ (for mobile wallet)
- **Docker:** For containerized development

### First-Time Setup

```bash
# Clone repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid

# Install Rust toolchain
rustup update stable
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Verify installation
rustc --version
cargo --version
```

### Your First Build

```bash
# Build FlareChain
cd 05-multichain/flare-chain
cargo build --release

# Run in dev mode
./target/release/flarechain-node --dev --tmp
```

### Your First Test

```bash
# Run all tests
cargo test --workspace

# Run specific component tests
cargo test -p etrid-cryptography
```

---

## Development Environment Setup

### Rust Environment

#### Install Toolchain

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install toolchains
rustup toolchain install stable nightly

# Add WASM target
rustup target add wasm32-unknown-unknown
rustup component add rust-src
```

#### Development Tools

```bash
# Auto-rebuild on changes
cargo install cargo-watch

# Macro expansion
cargo install cargo-expand

# Security auditing
cargo install cargo-audit

# Performance profiling
cargo install flamegraph

# Compilation caching
cargo install sccache
```

#### Configure sccache (optional)

```bash
# Add to ~/.bashrc or ~/.zshrc
export RUSTC_WRAPPER=sccache
export SCCACHE_CACHE_SIZE="10G"
```

### Node.js Environment

```bash
# Install Node.js 18+ using nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
nvm install 18
nvm use 18

# Global tools
npm install -g typescript ts-node nodemon
```

### Go Environment (for DETR P2P)

```bash
# Install Go 1.21+
wget https://go.dev/dl/go1.21.5.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.21.5.linux-amd64.tar.gz

# Add to PATH
export PATH=$PATH:/usr/local/go/bin

# Verify
go version
```

### IDE Setup

#### VS Code (Recommended)

**Extensions:**
- rust-analyzer
- CodeLLDB
- Better TOML
- Error Lens
- GitLens

**Settings (`.vscode/settings.json`):**
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "editor.rulers": [100],
  "files.trimTrailingWhitespace": true
}
```

---

## Project Structure

### Repository Layout

```
etrid/
├── 01-detr-p2p/           # P2P networking [ARCHITECTURE.md]
│   ├── core/              # Go discovery & transport
│   ├── detrp2p/           # Rust P2P protocol
│   ├── aecomms/           # Encrypted communications
│   ├── dpeers/            # Peer management
│   ├── etrid-protocol/    # Message definitions
│   ├── fluent/            # Flow control
│   └── stored/            # Peer storage
│
├── 02-open-did/           # Identity system [ARCHITECTURE.md]
│   ├── types/             # DID types
│   ├── registry/          # On-chain registry
│   ├── resolver/          # DID resolver
│   └── aidid/             # AI identity (world's first!)
│
├── 03-security/           # Cryptography [ARCHITECTURE.md]
│   ├── cryptography/      # Core primitives
│   ├── key-management/    # Key lifecycle
│   └── post-quantum/      # PQC (planned)
│
├── 04-accounts/           # Account system [ARCHITECTURE.md]
│   ├── account-types/     # EBCA, RCA, RCWA, SCA, SSCA
│   └── accounts/          # Account pallet
│
├── 05-multichain/         # Core multichain [ARCHITECTURE.md]
│   ├── flare-chain/       # Main FlareChain
│   ├── bridge-protocols/  # 19 bridge pallets
│   └── partition-burst-chains/  # 13 PBCs
│
├── 06-native-currency/    # Token economics [ARCHITECTURE.md]
│   ├── economics/         # Models
│   ├── vmw-gas/           # Gas metering
│   ├── etrid-coin/        # ÉTR token
│   └── etd-stablecoin/    # EDSC stablecoin
│
├── 07-transactions/       # Transaction processing [ARCHITECTURE.md]
│   ├── transaction-types/ # 5 types
│   ├── lightning-bloc/    # Layer 2 payment channels
│   └── txpool/            # Transaction pool
│
├── 08-etwasm-vm/          # WebAssembly VM [ARCHITECTURE.md]
│   ├── gas-metering/      # Gas computation
│   ├── opcodes/           # 150+ opcodes
│   ├── runtime/           # WASM runtime
│   └── pallet/            # ETWasm pallet
│
├── 09-consensus/          # ASF consensus [ARCHITECTURE.md]
│   ├── hotstuff/          # HotStuff protocol
│   ├── votes/             # Vote aggregation
│   ├── certificates/      # Quorum certificates
│   ├── finality/          # 5-level finality
│   └── validator/         # Validator logic
│
├── 10-foundation/         # Governance [ARCHITECTURE.md]
│   └── governance/        # On-chain governance
│
├── 11-peer-roles/         # Staking & roles [ARCHITECTURE.md]
│   ├── peer-types/        # 5-tier hierarchy
│   ├── staking/           # Staking logic
│   └── rewards/           # Distribution
│
├── 12-consensus-day/      # Annual governance [ARCHITECTURE.md]
│   ├── proposal-system/   # Proposals
│   ├── voting-protocol/   # Quadratic voting
│   └── minting-logic/     # Supply minting
│
├── 13-clients/            # CLI & SDKs [ARCHITECTURE.md]
│   ├── etrust/            # Rust CLI
│   ├── etrcpp/            # C++ CLI
│   ├── pyE/               # Python CLI
│   └── *-sdk/             # SDKs
│
├── apps/                  # Frontend applications
├── contracts/             # Smart contracts
├── services/              # Off-chain services
├── tests/                 # Testing infrastructure
├── deployment/            # Deployment guides
└── docs/                  # Documentation
```

### Component Documentation

Every component has comprehensive `ARCHITECTURE.md`:

| Component | Documentation | Lines | Description |
|-----------|---------------|-------|-------------|
| 01-detr-p2p | [ARCHITECTURE.md](01-detr-p2p/ARCHITECTURE.md) | 550 | P2P networking |
| 02-open-did | [ARCHITECTURE.md](02-open-did/ARCHITECTURE.md) | 600 | Identity + AIDID |
| 03-security | [ARCHITECTURE.md](03-security/ARCHITECTURE.md) | 550 | Cryptography |
| 04-accounts | [ARCHITECTURE.md](04-accounts/ARCHITECTURE.md) | 829 | Account system |
| 05-multichain | [ARCHITECTURE.md](05-multichain/ARCHITECTURE.md) | 2,262 | Multichain architecture |
| 06-native-currency | [ARCHITECTURE.md](06-native-currency/ARCHITECTURE.md) | 1,268 | Token economics |
| 07-transactions | [ARCHITECTURE.md](07-transactions/ARCHITECTURE.md) | 600 | Transaction processing |
| 08-etwasm-vm | [ARCHITECTURE.md](08-etwasm-vm/ARCHITECTURE.md) | 650 | WebAssembly VM |
| 09-consensus | [ARCHITECTURE.md](09-consensus/ARCHITECTURE.md) | 1,432 | ASF consensus |
| 10-foundation | [ARCHITECTURE.md](10-foundation/ARCHITECTURE.md) | 968 | Governance |
| 11-peer-roles | [ARCHITECTURE.md](11-peer-roles/ARCHITECTURE.md) | 1,430 | Staking & roles |
| 12-consensus-day | [ARCHITECTURE.md](12-consensus-day/ARCHITECTURE.md) | 1,206 | Annual governance |
| 13-clients | [ARCHITECTURE.md](13-clients/ARCHITECTURE.md) | 1,363 | CLI tools & SDKs |

**Total:** 13,708 lines of technical documentation

---

## Development Workflow

### Git Workflow

#### Branch Strategy

```
main              # Production-ready
├── develop       # Integration branch
    ├── feature/xxx   # New features
    ├── fix/xxx       # Bug fixes
    └── docs/xxx      # Documentation
```

#### Creating Feature Branch

```bash
# Update develop
git checkout develop
git pull origin develop

# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and commit
git add .
git commit -m "feat(component): description"

# Push
git push origin feature/your-feature-name

# Create PR on GitHub
```

#### Commit Message Format

Follow conventional commits:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:** `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`

**Examples:**
```
feat(consensus): Add 5-level finality tracking

Implement finality levels (Weak, Moderate, Strong, Irreversible)
with configurable thresholds.

Closes #123
```

### Code Review Process

1. **Self-Review:** Review your changes
2. **Create PR:** Clear description
3. **CI Checks:** Must pass
4. **Peer Review:** 1+ approval required
5. **Merge:** Squash and merge

---

## Building & Testing

### Building

#### FlareChain

```bash
# Development build
cd 05-multichain/flare-chain
cargo build

# Release build (optimized)
cargo build --release

# With features
cargo build --release --features runtime-benchmarks
```

#### All PBCs

```bash
# Build all 13 PBCs
cd 05-multichain/partition-burst-chains/pbc-chains
for pbc in */; do
    cd "$pbc" && cargo build --release && cd ..
done
```

Or use automated script:
```bash
./scripts/build_all_pbcs.sh
```

#### WASM Runtimes

```bash
# FlareChain WASM
cd 05-multichain/flare-chain/runtime
cargo build --release --features on-chain-release-build

# Output: target/release/wbuild/flarechain-runtime/flarechain_runtime.compact.wasm
```

#### Services

```bash
# Attestation service
cd services/attestation-service
npm install && npm run build

# Relayer service
cd services/relayer-service
npm install && npm run build
```

#### Frontend

```bash
# Web wallet
cd apps/wallet-web
npm install && npm run build

# Mobile wallet
cd apps/wallet-mobile
flutter pub get
flutter build apk  # Android
flutter build ios  # iOS
```

### Testing

#### Unit Tests

```bash
# All tests
cargo test --workspace

# Specific package
cargo test -p etrid-cryptography
cargo test -p flarechain-runtime

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name

# Release mode (faster)
cargo test --release
```

#### Integration Tests

```bash
# Integration tests
cargo test --test integration_tests

# Bridge tests
cd tests/bridge && cargo test

# E2E tests
cd tests/e2e && cargo test
```

#### Service Tests

```bash
# Attestation service
cd services/attestation-service && npm test

# Relayer service
cd services/relayer-service && npm test
```

#### Benchmarks

```bash
# Runtime benchmarks
cd 05-multichain/flare-chain/runtime
cargo build --release --features runtime-benchmarks

./target/release/flarechain-node benchmark pallet \
    --chain dev \
    --pallet pallet_consensus_asf \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20
```

### Running Nodes

#### FlareChain (Dev Mode)

```bash
cd 05-multichain/flare-chain
./target/release/flarechain-node --dev --tmp

# With options
./target/release/flarechain-node \
    --dev \
    --tmp \
    --rpc-cors all \
    --rpc-port 9944 \
    --port 30333
```

#### Multi-Node Testnet

```bash
# Terminal 1: Alice (validator)
./target/release/flarechain-node \
    --chain local \
    --alice \
    --port 30333 \
    --rpc-port 9944 \
    --validator

# Terminal 2: Bob (validator)
./target/release/flarechain-node \
    --chain local \
    --bob \
    --port 30334 \
    --rpc-port 9945 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_PEER_ID \
    --validator
```

#### PBC Collator

```bash
cd 05-multichain/partition-burst-chains/pbc-chains/btc-pbc

./target/release/btc-pbc-collator \
    --collator \
    --force-authoring \
    --chain btc-pbc-local \
    --port 40333 \
    --rpc-port 8844
```

---

## Component Development

### Developing a New Pallet

#### 1. Create Structure

```bash
cd pallets
cargo new --lib my-pallet
```

#### 2. Configure Cargo.toml

```toml
[package]
name = "pallet-my-pallet"
version = "0.1.0"
edition = "2021"

[dependencies]
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false }
scale-info = { version = "2.10.0", default-features = false }
frame-support = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
frame-system = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }

[features]
default = ["std"]
std = [
    "codec/std",
    "frame-support/std",
    "frame-system/std",
]
```

#### 3. Implement Pallet

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomethingStored { value: u32, who: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn do_something(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            <Something<T>>::put(value);
            Self::deposit_event(Event::SomethingStored { value, who });
            Ok(())
        }
    }
}
```

#### 4. Write Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::assert_ok;

    #[test]
    fn it_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(MyPallet::do_something(RuntimeOrigin::signed(1), 42));
        });
    }
}
```

#### 5. Integrate into Runtime

**runtime/Cargo.toml:**
```toml
[dependencies]
pallet-my-pallet = { path = "../../pallets/my-pallet", default-features = false }
```

**runtime/src/lib.rs:**
```rust
impl pallet_my_pallet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

construct_runtime!(
    pub enum Runtime {
        MyPallet: pallet_my_pallet,
        // ...
    }
);
```

---

## Common Development Tasks

### Add New Transaction Type

```rust
// 1. Define in transaction-types
#[derive(Encode, Decode, Clone)]
pub struct MyTransaction {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: Balance,
}

// 2. Add to enum
pub enum TransactionType {
    Regular,
    MyTransaction,  // New
}

// 3. Implement validation
impl MyTransaction {
    pub fn validate(&self) -> Result<(), Error> {
        // Validation logic
        Ok(())
    }
}
```

### Add New PBC

**WARNING:** Current architecture has 92.6% duplication. Use template (planned).

```bash
# Current (not recommended)
cp -r btc-pbc my-pbc
# Rename all references

# Recommended (planned)
./scripts/create_pbc.sh my-pbc
```

### Add New Bridge

```rust
// 1. Create pallet
impl BridgeConfig for MyChainBridge {
    type ChainId = u32;
    type Address = Vec<u8>;

    fn validate_address(addr: &Self::Address) -> bool {
        // Validation
    }
}

// 2. Implement transfer logic
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn deposit_for_burn(...) -> DispatchResult {
        // Lock/burn tokens
        Ok(())
    }

    pub fn receive_message(...) -> DispatchResult {
        // Verify & mint tokens
        Ok(())
    }
}
```

### Update Chain Spec

```bash
# Generate
./target/release/flarechain-node build-spec --chain local > spec.json

# Edit genesis config

# Convert to raw
./target/release/flarechain-node build-spec --chain spec.json --raw > spec-raw.json

# Test
./target/release/flarechain-node --chain spec-raw.json --alice
```

---

## Coding Standards

### Rust Style

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy --all-targets

# Fix warnings
cargo clippy --fix
```

**Conventions:**
- `snake_case` for variables/functions
- `PascalCase` for types/traits
- `SCREAMING_SNAKE_CASE` for constants
- Max line length: 100 characters
- Document all public APIs

**Example:**
```rust
/// Calculate finality level from confirmations.
///
/// # Arguments
/// * `confirmations` - Block confirmations
///
/// # Returns
/// Finality level (0-4)
pub fn calculate_finality(confirmations: u32) -> FinalityLevel {
    match confirmations {
        0..=2 => FinalityLevel::Weak,
        3..=5 => FinalityLevel::Moderate,
        _ => FinalityLevel::Strong,
    }
}
```

### Error Handling

```rust
pub fn transfer(
    from: AccountId,
    to: AccountId,
    amount: Balance,
) -> Result<(), TransferError> {
    ensure!(amount > 0, TransferError::ZeroAmount);
    ensure!(has_balance(from, amount), TransferError::InsufficientBalance);
    // Perform transfer
    Ok(())
}
```

### Testing Standards

```rust
#[test]
fn test_transfer_success() {
    // Arrange
    let from = AccountId::from([1u8; 32]);
    let to = AccountId::from([2u8; 32]);

    // Act
    let result = transfer(from, to, 100);

    // Assert
    assert!(result.is_ok());
}
```

### Security Best Practices

1. **Validate input:**
   ```rust
   ensure!(amount > 0, Error::ZeroAmount);
   ```

2. **Safe math:**
   ```rust
   let new_balance = balance.checked_add(amount).ok_or(Error::Overflow)?;
   ```

3. **No panics:**
   ```rust
   // Good
   let value = map.get(key).ok_or(Error::NotFound)?;
   ```

4. **Least privilege:**
   ```rust
   ensure_root(origin)?;  // Only root
   ```

---

## Debugging & Troubleshooting

### Common Build Errors

#### Dependency conflicts

```bash
cargo clean
cargo update
cargo build
```

#### Linking errors

```bash
# Ubuntu/Debian
sudo apt-get install build-essential cmake clang

# macOS
xcode-select --install
```

### Runtime Debugging

#### Enable Logging

```bash
export RUST_LOG=runtime=debug,pallet_my_pallet=trace
./target/release/flarechain-node --dev
```

**Levels:** `error`, `warn`, `info`, `debug`, `trace`

#### GDB Debugger

```bash
# Build with debug symbols
cargo build

# Run with GDB
gdb --args ./target/debug/flarechain-node --dev

(gdb) break pallet_my_pallet::do_something
(gdb) run
(gdb) step
(gdb) print variable
```

### Performance Profiling

#### Flamegraph

```bash
cargo install flamegraph
cargo flamegraph --bin flarechain-node
open flamegraph.svg
```

### Network Debugging

```bash
# Check peers
curl -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' http://localhost:9944

# Get chain info
curl -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' http://localhost:9944

# Monitor logs
tail -f /tmp/flarechain.log
```

### Known Issues

See [KNOWN_ISSUES.md](KNOWN_ISSUES.md)

**Common:**
1. **Polkadot SDK conflicts** - Use exact tag `polkadot-stable2506`
2. **WASM build failures** - `cargo clean && cargo build`
3. **PBC connectivity** - Match spec versions

---

## Contributing

### First-Time Contributors

1. **Find issue:** Look for "good first issue" label
2. **Setup:** Follow [Development Environment Setup](#development-environment-setup)
3. **Create PR:** Fork, branch, commit, push, PR

### Areas Needing Help

- 🔴 Frontend (React, Flutter)
- 🟡 Runtime pallets (Rust)
- 🟢 Documentation
- 🟢 Testing & QA

### Code Review Guidelines

**As reviewer:**
- Be constructive
- Explain reasoning
- Approve when ready

**As author:**
- Respond to comments
- Make changes or explain
- Request re-review

### Communication

- **Discord:** [discord.gg/etrid](https://discord.gg/etrid)
- **Telegram:** [t.me/EtridOfficial](https://t.me/EtridOfficial)
- **GitHub:** Issues & Discussions
- **Email:** dev@etrid.io

---

## Additional Resources

### Core Documentation

- [README.md](README.md) - Project overview
- [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) - Production deployment
- [TESTING_GUIDE.md](TESTING_GUIDE.md) - Testing procedures
- [KNOWN_ISSUES.md](KNOWN_ISSUES.md) - Current blockers

### Component Architecture

All 13 components have comprehensive ARCHITECTURE.md files. See [README.md](README.md#component-architecture-documentation-new) for links.

### External Resources

- [Substrate Docs](https://docs.substrate.io/)
- [Polkadot SDK](https://github.com/paritytech/polkadot-sdk)
- [FRAME Pallets](https://docs.substrate.io/reference/frame-pallets/)
- [Rust Book](https://doc.rust-lang.org/book/)

---

**Last Updated:** October 20, 2025
**Maintainers:** Ëtrid Core Team
**License:** MIT

---

<p align="center">
  <strong>Happy coding!</strong>
</p>

<p align="center">
  <sub>The Free and Open Decentralized Democracy of Stakeholders</sub>
</p>
