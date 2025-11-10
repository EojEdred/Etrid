# FlareChain Node - macOS Build Guide

**Supported Versions:** macOS 12 (Monterey), macOS 13 (Ventura), macOS 14 (Sonoma), macOS 15 (Sequoia)
**Architectures:** Intel (x86_64) and Apple Silicon (arm64)
**Build Time:** 20-45 minutes
**Disk Space Required:** 20GB

---

## Prerequisites

### System Requirements

- **macOS:** 12.0 or later
- **RAM:** 8GB minimum, 16GB recommended
- **Disk:** 20GB for build, 100GB+ for running validator
- **Xcode Command Line Tools**

### Required Tools

- Rust 1.70 or later
- Homebrew (package manager)
- Git
- Clang/LLVM (via Xcode)
- CMake
- OpenSSL

---

## Quick Start

```bash
# One-command install all dependencies and build
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/etrid/flarechain/main/scripts/build-macos.sh)"
```

---

## Manual Build Instructions

### Step 1: Install Xcode Command Line Tools

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Follow the prompts in the GUI

# Verify installation
xcode-select -p
# Should output: /Library/Developer/CommandLineTools

# Accept license
sudo xcodebuild -license accept
```

### Step 2: Install Homebrew

```bash
# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Add Homebrew to PATH (for Apple Silicon Macs)
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
eval "$(/opt/homebrew/bin/brew shellenv)"

# Verify installation
brew --version
```

### Step 3: Install Dependencies

```bash
# Update Homebrew
brew update

# Install required packages
brew install \
    cmake \
    protobuf \
    openssl@3 \
    pkg-config \
    git

# Verify installations
cmake --version
protoc --version
openssl version
```

### Step 4: Install Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load Rust environment
source "$HOME/.cargo/env"

# Verify installation
rustc --version
cargo --version

# Install nightly toolchain
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# Set up environment for OpenSSL
echo 'export OPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3' >> ~/.zprofile
echo 'export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@3/lib/pkgconfig"' >> ~/.zprofile
source ~/.zprofile
```

**For Intel Macs:** Use `/usr/local` instead of `/opt/homebrew`:
```bash
echo 'export OPENSSL_ROOT_DIR=/usr/local/opt/openssl@3' >> ~/.zprofile
echo 'export PKG_CONFIG_PATH="/usr/local/opt/openssl@3/lib/pkgconfig"' >> ~/.zprofile
source ~/.zprofile
```

### Step 5: Clone FlareChain Repository

```bash
# Clone the repository
git clone https://github.com/etrid/flarechain.git
cd flarechain

# Checkout latest stable release
git checkout main

# Or checkout specific version
# git checkout v1.0.0
```

### Step 6: Build FlareChain Node

```bash
# Build in release mode (optimized, takes 20-45 minutes)
cargo build --release

# Binary will be at: target/release/flarechain-node
```

**Apple Silicon Optimizations:**
```bash
# Build with native CPU optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

**Intel Mac Optimizations:**
```bash
# Build with AVX2 support (if your CPU supports it)
RUSTFLAGS="-C target-cpu=haswell" cargo build --release
```

### Step 7: Verify Build

```bash
# Check binary exists
ls -lh target/release/flarechain-node

# Test binary
./target/release/flarechain-node --version

# Expected output: flarechain-node 1.x.x-xxxxxxx

# Check binary architecture
file target/release/flarechain-node
# Apple Silicon: Mach-O 64-bit executable arm64
# Intel: Mach-O 64-bit executable x86_64
```

### Step 8: Install Binary

```bash
# Option A: Install to /usr/local/bin (system-wide)
sudo cp target/release/flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node

# Option B: Install to ~/.local/bin (user-only)
mkdir -p ~/.local/bin
cp target/release/flarechain-node ~/.local/bin/
chmod +x ~/.local/bin/flarechain-node

# Add to PATH if not already (add to ~/.zprofile or ~/.zshrc)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zprofile
source ~/.zprofile

# Verify
flarechain-node --version
```

---

## Building Specific Components

### Build Only Runtime

```bash
# Build FlareChain runtime WASM
cargo build --release -p flarechain-runtime
```

### Build PBC Collators

```bash
# Build all PBC collators
cd 05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes

# Bitcoin PBC
cargo build --release -p btc-pbc-collator

# Ethereum PBC
cargo build --release -p eth-pbc-collator

# Solana PBC
cargo build --release -p sol-pbc-collator

# Binaries will be in: target/release/
```

### Build Lightning Network Module

```bash
cd 07-transactions/lightning-bloc
cargo build --release
cargo test --release
```

---

## Troubleshooting

### Build Errors

#### Error: "xcrun: error: invalid active developer path"

**Cause:** Xcode Command Line Tools not installed

**Fix:**
```bash
xcode-select --install
```

#### Error: "ld: library not found for -lssl"

**Cause:** OpenSSL not found or not linked correctly

**Fix:**
```bash
# Reinstall OpenSSL
brew reinstall openssl@3

# Apple Silicon
export OPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@3/lib/pkgconfig"

# Intel Mac
export OPENSSL_ROOT_DIR=/usr/local/opt/openssl@3
export PKG_CONFIG_PATH="/usr/local/opt/openssl@3/lib/pkgconfig"

# Add to ~/.zprofile to make permanent
echo 'export OPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3' >> ~/.zprofile
echo 'export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@3/lib/pkgconfig"' >> ~/.zprofile
```

#### Error: "could not find system library 'protobuf'"

**Cause:** Protobuf not installed

**Fix:**
```bash
brew install protobuf
```

#### Error: "linker `cc` not found"

**Cause:** Xcode Command Line Tools not properly configured

**Fix:**
```bash
# Reset Xcode Command Line Tools
sudo xcode-select --reset
xcode-select --install

# Accept license
sudo xcodebuild -license accept
```

### Memory Issues During Build

**Symptoms:** Build process killed or extremely slow

**Fix:**
```bash
# Close other applications to free up RAM

# Reduce parallel jobs
cargo build --release -j 2

# For Apple Silicon Macs with 8GB RAM
cargo build --release -j 1
```

### Slow Build Times

**Optimization:**
```bash
# Use faster linker (zld)
cargo install zld
echo '[target.aarch64-apple-darwin]
linker = "zld"' >> ~/.cargo/config.toml

# Enable incremental compilation
export CARGO_INCREMENTAL=1
cargo build --release

# Use sccache for caching
brew install sccache
export RUSTC_WRAPPER=sccache
cargo build --release
```

### Disk Space Issues

```bash
# Clean build artifacts
cargo clean

# Remove unused dependencies
cargo install cargo-cache
cargo cache -a

# Check disk usage
df -h
du -sh target/
```

### Rosetta 2 Issues (Apple Silicon)

**If you need to build for Intel (x86_64) on Apple Silicon:**

```bash
# Install Rosetta 2 (if not already installed)
softwareupdate --install-rosetta --agree-to-license

# Add x86_64 target
rustup target add x86_64-apple-darwin

# Build for Intel
cargo build --release --target=x86_64-apple-darwin

# Binary: target/x86_64-apple-darwin/release/flarechain-node
```

---

## Cross-Compilation

### Build for Intel (from Apple Silicon)

```bash
# Add x86_64 target
rustup target add x86_64-apple-darwin

# Build for Intel
cargo build --release --target=x86_64-apple-darwin

# Binary: target/x86_64-apple-darwin/release/flarechain-node
```

### Build for Apple Silicon (from Intel Mac)

```bash
# Add ARM64 target
rustup target add aarch64-apple-darwin

# Build for Apple Silicon
cargo build --release --target=aarch64-apple-darwin

# Binary: target/aarch64-apple-darwin/release/flarechain-node
```

### Build Universal Binary (Both Architectures)

```bash
# Build for both architectures
cargo build --release --target=x86_64-apple-darwin
cargo build --release --target=aarch64-apple-darwin

# Combine into universal binary
lipo -create \
    target/x86_64-apple-darwin/release/flarechain-node \
    target/aarch64-apple-darwin/release/flarechain-node \
    -output flarechain-node-universal

# Verify
file flarechain-node-universal
# Should show: Mach-O universal binary with 2 architectures
```

---

## macOS-Specific Configuration

### Running as LaunchDaemon (System Service)

Create a plist file:

```bash
sudo nano /Library/LaunchDaemons/com.etrid.flarechain.plist
```

Add content:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.etrid.flarechain</string>

    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/flarechain-node</string>
        <string>--base-path</string>
        <string>/var/lib/etrid</string>
        <string>--chain</string>
        <string>/var/lib/etrid/chainspec-mainnet-raw-FIXED.json</string>
        <string>--validator</string>
        <string>--name</string>
        <string>MyValidator</string>
        <string>--port</string>
        <string>30333</string>
    </array>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <true/>

    <key>StandardOutPath</key>
    <string>/var/log/flarechain/output.log</string>

    <key>StandardErrorPath</key>
    <string>/var/log/flarechain/error.log</string>
</dict>
</plist>
```

Load and start:
```bash
# Create log directory
sudo mkdir -p /var/log/flarechain

# Load daemon
sudo launchctl load /Library/LaunchDaemons/com.etrid.flarechain.plist

# Start service
sudo launchctl start com.etrid.flarechain

# Check status
sudo launchctl list | grep flarechain

# View logs
tail -f /var/log/flarechain/output.log
```

### Firewall Configuration

```bash
# Allow incoming connections on port 30333
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add /usr/local/bin/flarechain-node
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --unblockapp /usr/local/bin/flarechain-node
```

---

## Performance Benchmarks

After building, run benchmarks:

```bash
# Runtime benchmarks
cargo build --release --features runtime-benchmarks
./target/release/flarechain-node benchmark pallet --chain=dev --pallet="*" --extrinsic="*"

# Machine benchmarks
./target/release/flarechain-node benchmark machine --chain=dev
```

---

## Development Setup

### VSCode Configuration

Install recommended extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Better TOML
- crates

Create `.vscode/settings.json`:
```json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.procMacro.enable": true
}
```

### Running Tests

```bash
# Run all tests
cargo test --release

# Run specific test
cargo test --release test_name

# Run with output
cargo test --release -- --nocapture

# Run benchmarks
cargo bench
```

---

## Docker Alternative (for macOS)

If you prefer not to build locally:

```bash
# Pull pre-built image
docker pull etrid/flarechain-node:latest

# Or build using Docker
docker build -t flarechain-node .

# Run in Docker
docker run -d --name flarechain-validator \
  -p 30333:30333 \
  -p 9615:9615 \
  -v ~/etrid-data:/data \
  flarechain-node \
    --base-path /data \
    --validator \
    --name "MyValidator"
```

---

## Troubleshooting macOS-Specific Issues

### "flarechain-node cannot be opened because the developer cannot be verified"

**Fix:**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /usr/local/bin/flarechain-node

# Or allow in System Settings:
# System Settings → Privacy & Security → Allow anyway
```

### Port 30333 already in use

```bash
# Find process using port 30333
lsof -i :30333

# Kill process
kill -9 <PID>
```

### High CPU usage during sync

This is normal during initial sync. To limit CPU:

```bash
# Run with nice (lower priority)
nice -n 10 ./target/release/flarechain-node --validator ...
```

---

## Next Steps

After successful build:

1. ✅ Binary ready at `target/release/flarechain-node`
2. 📝 Configure as LaunchDaemon (optional)
3. 🔑 Generate session keys
4. 🚀 Start validator node
5. 📊 Set up monitoring

**Note:** Running a validator on macOS is suitable for development and testing. For production validators, Linux servers are recommended for stability and lower costs.

---

**Last Updated:** November 9, 2025
**Maintainer:** FlareChain Core Team
