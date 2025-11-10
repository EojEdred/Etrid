# FlareChain Node - Linux Build Guide

**Supported Distributions:** Ubuntu 22.04/24.04, Debian 11/12, CentOS 8+, Fedora 38+
**Build Time:** 30-60 minutes
**Disk Space Required:** 20GB

---

## Prerequisites

### System Requirements

- **CPU:** 2+ cores recommended
- **RAM:** 4GB minimum, 8GB recommended
- **Disk:** 20GB for build, 100GB+ for running validator
- **OS:** 64-bit Linux (x86_64 or aarch64)

### Required Tools

- Rust 1.70 or later
- Git
- Clang/LLVM
- CMake
- OpenSSL
- pkg-config

---

## Quick Start (Ubuntu/Debian)

```bash
# One-command install all dependencies and build
curl -sSf https://raw.githubusercontent.com/etrid/flarechain/main/scripts/build.sh | bash
```

---

## Manual Build Instructions

### Step 1: Install System Dependencies

#### Ubuntu 22.04 / 24.04 / Debian 11 / 12

```bash
sudo apt-get update

sudo apt-get install -y \
    build-essential \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    make \
    protobuf-compiler \
    pkg-config \
    cmake
```

#### CentOS 8+ / RHEL 8+ / Rocky Linux 8+

```bash
sudo dnf update

sudo dnf install -y \
    gcc \
    gcc-c++ \
    git \
    clang \
    curl \
    openssl-devel \
    llvm \
    make \
    protobuf-compiler \
    pkg-config \
    cmake
```

#### Fedora 38+

```bash
sudo dnf update

sudo dnf install -y \
    gcc \
    gcc-c++ \
    git \
    clang \
    curl \
    openssl-devel \
    llvm \
    make \
    protobuf-compiler \
    pkg-config \
    cmake
```

#### Arch Linux

```bash
sudo pacman -Syu

sudo pacman -S --needed \
    base-devel \
    git \
    clang \
    curl \
    openssl \
    llvm \
    make \
    protobuf \
    pkg-config \
    cmake
```

### Step 2: Install Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load Rust environment
source "$HOME/.cargo/env"

# Verify installation
rustc --version
cargo --version

# Install nightly toolchain (required for some dependencies)
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Step 3: Clone FlareChain Repository

```bash
# Clone the repository
git clone https://github.com/etrid/flarechain.git
cd flarechain

# Checkout latest stable release
git checkout main

# Or checkout specific version
# git checkout v1.0.0
```

### Step 4: Build FlareChain Node

```bash
# Build in release mode (optimized, takes 30-60 minutes)
cargo build --release

# Binary will be at: target/release/flarechain-node
```

**Build Options:**

```bash
# Build with specific features
cargo build --release --features runtime-benchmarks

# Build for production (maximum optimizations)
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Parallel build (faster on multi-core systems)
cargo build --release -j $(nproc)
```

### Step 5: Verify Build

```bash
# Check binary exists
ls -lh target/release/flarechain-node

# Test binary
./target/release/flarechain-node --version

# Expected output: flarechain-node 1.x.x-xxxxxxx
```

### Step 6: Install Binary

```bash
# Option A: System-wide installation
sudo cp target/release/flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node

# Option B: User installation
mkdir -p ~/.local/bin
cp target/release/flarechain-node ~/.local/bin/
chmod +x ~/.local/bin/flarechain-node

# Add to PATH if not already (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

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

#### Error: "linking with `cc` failed"

**Cause:** Missing C/C++ compiler or libraries

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# CentOS/RHEL/Fedora
sudo dnf install gcc gcc-c++
```

#### Error: "failed to run custom build command for `openssl-sys`"

**Cause:** Missing OpenSSL development headers

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# CentOS/RHEL/Fedora
sudo dnf install openssl-devel pkg-config
```

#### Error: "error: linker `cc` not found"

**Cause:** Clang/LLVM not installed

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install clang llvm

# CentOS/RHEL/Fedora
sudo dnf install clang llvm
```

#### Error: "could not find native static library `protobuf`"

**Cause:** Missing protobuf compiler

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install protobuf-compiler

# CentOS/RHEL/Fedora
sudo dnf install protobuf-compiler

# Or install from source
wget https://github.com/protocolbuffers/protobuf/releases/download/v21.12/protobuf-all-21.12.tar.gz
tar -xzf protobuf-all-21.12.tar.gz
cd protobuf-21.12
./configure && make && sudo make install
```

### Out of Memory During Build

**Symptoms:** Build killed/terminated unexpectedly

**Fix:**
```bash
# Reduce parallel jobs
cargo build --release -j 1

# Or add swap space
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Make swap permanent
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

### Slow Build Times

**Optimization:**
```bash
# Use faster linker (mold)
cargo install mold
RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build --release

# Or use lld
sudo apt-get install lld  # Ubuntu/Debian
RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build --release

# Enable incremental compilation
export CARGO_INCREMENTAL=1
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

---

## Cross-Compilation

### Build for ARM64 (from x86_64)

```bash
# Install cross-compilation tools
sudo apt-get install gcc-aarch64-linux-gnu

# Add ARM64 target
rustup target add aarch64-unknown-linux-gnu

# Build for ARM64
cargo build --release --target=aarch64-unknown-linux-gnu

# Binary: target/aarch64-unknown-linux-gnu/release/flarechain-node
```

### Build for x86_64 (from ARM64)

```bash
# Add x86_64 target
rustup target add x86_64-unknown-linux-gnu

# Build for x86_64
cargo build --release --target=x86_64-unknown-linux-gnu

# Binary: target/x86_64-unknown-linux-gnu/release/flarechain-node
```

---

## Docker Build (Alternative)

```bash
# Build using Docker (no local dependencies needed)
docker build -t flarechain-node .

# Extract binary from container
docker create --name temp-container flarechain-node
docker cp temp-container:/usr/local/bin/flarechain-node ./flarechain-node
docker rm temp-container

# Or run directly in Docker
docker run -d --name flarechain-validator \
  -p 30333:30333 \
  -p 9615:9615 \
  -v /var/lib/etrid:/data \
  flarechain-node \
    --base-path /data \
    --validator \
    --name "MyValidator"
```

---

## CI/CD Build

### GitHub Actions Example

```yaml
name: Build FlareChain

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable

    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y build-essential git clang curl libssl-dev llvm

    - name: Build
      run: cargo build --release

    - name: Run tests
      run: cargo test --release

    - name: Upload binary
      uses: actions/upload-artifact@v3
      with:
        name: flarechain-node
        path: target/release/flarechain-node
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

## Next Steps

After successful build:

1. ✅ Binary ready at `target/release/flarechain-node`
2. 📝 Follow deployment guide for your cloud provider
3. 🔑 Generate session keys
4. 🚀 Start validator node
5. 📊 Set up monitoring

See: [Cloud Provider Setup](../cloud-providers/CLOUD_PROVIDER_SETUP_MASTER.md)

---

**Last Updated:** November 9, 2025
**Maintainer:** FlareChain Core Team
