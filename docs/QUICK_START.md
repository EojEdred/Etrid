# ETRID Quick Start Guide

Get your ETRID node running in 5 minutes!

## Overview

ETRID is a multichain protocol with:
- **FlareChain**: Root chain with custom FODDoS ASF consensus
- **13 PBC (Partition Burst Chains)**: Specialized chains for BTC, ETH, SOL, XLM, XRP, BNB, TRX, ADA, LINK, MATIC, USDT, DOGE, and EDSC

The unified `etrid` binary can run as either a FlareChain validator or a PBC collator.

## Prerequisites

- Rust toolchain (stable, latest recommended)
- 16GB+ RAM recommended
- 100GB+ free disk space
- Linux, macOS, or Windows (WSL2)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
```

### Install System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev pkg-config
```

**macOS:**
```bash
brew install openssl cmake llvm
```

## Building the Node

### 1. Clone the Repository

```bash
git clone https://github.com/etrid/etrid.git
cd etrid
```

### 2. Build the Unified Node Binary

The main node binary is named `etrid` and is defined in the root `Cargo.toml`:

```bash
# Build in release mode (optimized, recommended)
cargo build --release -p etrid

# Or for faster development builds (slower runtime)
cargo build -p etrid
```

**Build time:** 20-40 minutes on first build (downloads and compiles dependencies)

The binary will be located at:
- Release: `./target/release/etrid`
- Debug: `./target/debug/etrid`

### 3. Verify Installation

```bash
# Check version
./target/release/etrid --version

# View help
./target/release/etrid --help
```

## Running a Development Chain

### Option 1: FlareChain (Root Chain)

Start a single-node development chain with FlareChain:

```bash
# Run FlareChain in development mode
./target/release/etrid --chain flare --dev

# Or with custom base path
./target/release/etrid --chain flare --dev --base-path /tmp/flare-dev
```

**What this does:**
- Starts FlareChain with ASF consensus
- Creates a temporary development chain
- Enables sudo access for testing
- Clears chain data on restart

### Option 2: PBC Collator (Partition Burst Chain)

Run a specific PBC collator in development mode:

```bash
# Bitcoin PBC
./target/release/etrid --chain btc-pbc --collator --dev

# Ethereum PBC
./target/release/etrid --chain eth-pbc --collator --dev

# Solana PBC
./target/release/etrid --chain sol-pbc --collator --dev
```

**Available PBC chains:**
- `btc-pbc` - Bitcoin
- `eth-pbc` - Ethereum
- `sol-pbc` - Solana
- `xlm-pbc` - Stellar
- `xrp-pbc` - Ripple
- `bnb-pbc` - Binance Chain
- `trx-pbc` - Tron
- `ada-pbc` - Cardano
- `link-pbc` - Chainlink
- `matic-pbc` - Polygon
- `sc-usdt-pbc` - Smart Contract USDT
- `doge-pbc` - Dogecoin
- `edsc-pbc` - ETRID Dollar Stablecoin (EDSC)

**Note:** PBC collators require FlareChain to be running as the relay chain.

## Accessing the Wallet UI

### Web Wallet

The ETRID web wallet is a Next.js application with Polkadot.js integration:

```bash
cd apps/wallet-web/etrid-crypto-website

# Install dependencies
npm install

# Start development server
npm run dev
```

Access the wallet at: **http://localhost:3000**

**Features:**
- Multi-chain wallet interface
- Polkadot.js browser extension support
- Account management
- Transaction signing
- Balance checking across chains

### Production Build

```bash
cd apps/wallet-web/etrid-crypto-website
npm run build
npm start
```

Access at: **http://localhost:3000**

## Node Configuration

### Common Options

```bash
# Custom RPC port
./target/release/etrid --chain flare --dev --rpc-port 9944

# Custom WebSocket port
./target/release/etrid --chain flare --dev --ws-port 9945

# Enable RPC external access
./target/release/etrid --chain flare --dev --rpc-external --ws-external

# Custom node name
./target/release/etrid --chain flare --dev --name "My-ETRID-Node"

# Increase log verbosity
./target/release/etrid --chain flare --dev -lruntime=debug
```

### Connecting via Polkadot.js

1. Open [Polkadot.js Apps](https://polkadot.js.org/apps/)
2. Click top-left corner "Development" dropdown
3. Select "Local Node" or add custom endpoint:
   - **Local WebSocket:** `ws://127.0.0.1:9944`
   - **Local HTTP:** `http://127.0.0.1:9933`

## Troubleshooting

### Build Issues

**Problem:** Compilation errors or dependency conflicts

**Solution:**
```bash
# Clean build artifacts
cargo clean

# Update Rust toolchain
rustup update

# Try building again
cargo build --release -p etrid
```

**Problem:** Out of memory during compilation

**Solution:**
```bash
# Reduce parallel compilation jobs
cargo build --release -p etrid -j 2
```

**Problem:** `schnorrkel` version conflicts

**Solution:** The workspace uses Polkadot SDK `polkadot-stable2506` with patches in `Cargo.toml`. Ensure you're on the latest commit:
```bash
git pull origin main
cargo update
```

### Runtime Issues

**Problem:** Node won't start or crashes immediately

**Solution:**
```bash
# Clear development chain data
rm -rf /tmp/substrate-dev

# Or with custom base-path
rm -rf /tmp/flare-dev

# Start fresh
./target/release/etrid --chain flare --dev
```

**Problem:** "Database version mismatch" error

**Solution:**
```bash
# Purge chain data
./target/release/etrid purge-chain --chain flare --dev -y
```

**Problem:** RPC/WebSocket connection refused

**Solution:**
```bash
# Ensure ports aren't blocked by firewall
# Check if another node is already running on the port

# Try different ports
./target/release/etrid --chain flare --dev --rpc-port 9955 --ws-port 9956
```

### Wallet UI Issues

**Problem:** Wallet won't connect to node

**Solution:**
1. Ensure node is running and accessible
2. Check WebSocket endpoint matches node configuration
3. Try disabling browser extensions that may interfere
4. Clear browser cache and reload

**Problem:** Polkadot.js extension not detected

**Solution:**
1. Install [Polkadot.js extension](https://polkadot.js.org/extension/)
2. Refresh the wallet page
3. Grant permission when prompted

**Problem:** npm install fails

**Solution:**
```bash
cd apps/wallet-web/etrid-crypto-website

# Clear cache
rm -rf node_modules package-lock.json

# Reinstall
npm install

# Or use different package manager
yarn install
# or
pnpm install
```

## Key Management

### Generate Development Keys

```bash
# Generate a new account
./target/release/etrid key generate

# Generate with specific scheme
./target/release/etrid key generate --scheme Sr25519
```

### Insert Keys for Validation

```bash
# Insert session keys (for validators)
./target/release/etrid key insert \
  --base-path /tmp/flare-validator \
  --chain flare-dev \
  --scheme Sr25519 \
  --suri "//Alice" \
  --key-type aura
```

## Next Steps

1. **Explore the Runtime:** Check `05-multichain/flare-chain/runtime/` for FlareChain runtime code
2. **Read Documentation:** See `docs/` directory for detailed architecture
3. **Run Tests:** Execute `cargo test` to verify system integrity
4. **Join Network:** See deployment guides for joining public testnets/mainnet
5. **Build dApps:** Use the wallet UI and SDK to build decentralized applications

## Production Deployment

**Warning:** Development mode (`--dev`) is NOT secure for production!

For production deployment:
1. Generate proper chain specifications
2. Configure validator keys securely
3. Set up monitoring and backups
4. Use systemd or Docker for process management
5. Configure firewall rules properly

See `deployment/` directory for production deployment guides.

## Resources

- **Documentation:** [docs/README.md](docs/README.md)
- **GitHub:** https://github.com/etrid/etrid
- **Website:** https://etrid.io
- **Issue Tracker:** https://github.com/etrid/etrid/issues

## Quick Reference Card

```bash
# Build node
cargo build --release -p etrid

# Run FlareChain dev
./target/release/etrid --chain flare --dev

# Run BTC PBC collator
./target/release/etrid --chain btc-pbc --collator --dev

# Start wallet UI
cd apps/wallet-web/etrid-crypto-website && npm run dev

# Connect to node
# Browser: http://localhost:3000 (wallet)
# RPC: http://127.0.0.1:9933
# WebSocket: ws://127.0.0.1:9944

# Clean chain data
./target/release/etrid purge-chain --chain flare --dev -y

# View logs
./target/release/etrid --chain flare --dev -lruntime=debug

# Generate keys
./target/release/etrid key generate
```

---

**Welcome to ETRID!** You're now running a multichain node with custom ASF consensus. Happy building!
