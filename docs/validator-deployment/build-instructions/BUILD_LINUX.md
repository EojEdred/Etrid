# Building FlareChain Node from Source on Linux

**Complete guide for compiling the FlareChain validator binary on Linux systems**

---

## Supported Distributions

- Ubuntu 20.04 / 22.04 LTS (recommended)
- Debian 11 / 12
- CentOS 8 / Rocky Linux 8+
- Fedora 36+
- Arch Linux
- Other modern Linux distributions

---

## System Requirements

### Minimum for Building
- **CPU:** 4+ cores
- **RAM:** 8GB (16GB recommended)
- **Storage:** 50GB free space
- **OS:** 64-bit Linux

### Build Time
- **Fast system (16 cores):** 15-30 minutes
- **Medium system (8 cores):** 30-60 minutes
- **Slow system (4 cores):** 1-2 hours

---

## Quick Build (Ubuntu/Debian)

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y curl git build-essential libssl-dev pkg-config clang

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Clone and build
cd ~/Desktop/etrid
cargo build --release

# Binary will be at: ./target/release/flarechain-node
```

---

## Detailed Instructions by Distribution

### Ubuntu 20.04 / 22.04 LTS

```bash
# Update package list
sudo apt-get update && sudo apt-get upgrade -y

# Install build dependencies
sudo apt-get install -y \
    curl \
    git \
    build-essential \
    clang \
    libclang-dev \
    libssl-dev \
    pkg-config \
    protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Verify Rust installation
rustc --version
cargo --version

# Set default toolchain to stable
rustup default stable
rustup update

# Add wasm target (required for Substrate)
rustup target add wasm32-unknown-unknown

# Clone repository (if not already done)
git clone https://github.com/yourusername/etrid.git ~/Desktop/etrid
cd ~/Desktop/etrid

# Build release binary
cargo build --release

# Binary location
ls -lh ./target/release/flarechain-node
```

### Debian 11 / 12

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y \
    curl \
    git \
    build-essential \
    clang \
    libclang-dev \
    libssl-dev \
    pkg-config \
    protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Setup Rust
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# Build
cd ~/Desktop/etrid
cargo build --release
```

### CentOS 8 / Rocky Linux 8

```bash
# Enable PowerTools repo (for dependencies)
sudo dnf config-manager --set-enabled powertools

# Install dependencies
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y \
    curl \
    git \
    clang \
    openssl-devel \
    pkg-config \
    protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Setup Rust
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# Build
cd ~/Desktop/etrid
cargo build --release
```

### Fedora 36+

```bash
# Install dependencies
sudo dnf install -y \
    curl \
    git \
    gcc \
    gcc-c++ \
    clang \
    openssl-devel \
    pkg-config \
    protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Setup Rust
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# Build
cd ~/Desktop/etrid
cargo build --release
```

### Arch Linux

```bash
# Install dependencies
sudo pacman -Syu
sudo pacman -S --needed \
    base-devel \
    curl \
    git \
    clang \
    openssl \
    pkg-config \
    protobuf

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Setup Rust
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# Build
cd ~/Desktop/etrid
cargo build --release
```

---

## Build Optimization

### Faster Builds

**Use parallel compilation:**
```bash
# Set number of parallel jobs (adjust to your CPU core count)
export CARGO_BUILD_JOBS=8

# Or use all cores
export CARGO_BUILD_JOBS=$(nproc)

# Then build
cargo build --release
```

**Use sccache for faster rebuilds:**
```bash
# Install sccache
cargo install sccache

# Configure Cargo to use sccache
export RUSTC_WRAPPER=sccache

# Build (subsequent builds will be faster)
cargo build --release
```

### Smaller Binaries

**Strip debug symbols:**
```bash
# Build with stripping enabled
cargo build --release

# Manual strip (if needed)
strip ./target/release/flarechain-node
```

**Check binary size:**
```bash
ls -lh ./target/release/flarechain-node
# Should be ~100-200 MB
```

---

## Troubleshooting

### Error: "linker `cc` not found"

**Cause:** Missing C compiler

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install -y build-essential

# CentOS/Fedora
sudo dnf groupinstall -y "Development Tools"

# Arch
sudo pacman -S base-devel
```

### Error: "failed to run custom build command for `openssl-sys`"

**Cause:** Missing OpenSSL development files

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install -y libssl-dev pkg-config

# CentOS/Fedora
sudo dnf install -y openssl-devel pkg-config

# Arch
sudo pacman -S openssl pkg-config
```

### Error: "Unable to find libclang"

**Cause:** Missing Clang/LLVM

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install -y clang libclang-dev

# CentOS/Fedora
sudo dnf install -y clang

# Arch
sudo pacman -S clang
```

### Error: Out of Memory During Build

**Cause:** Insufficient RAM

**Fix:**
```bash
# Add swap space
sudo fallocate -l 8G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Make swap permanent
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

# Reduce parallel jobs
export CARGO_BUILD_JOBS=2

# Build
cargo build --release
```

### Build Hangs or Takes Very Long

**Cause:** Too many parallel jobs for available RAM

**Fix:**
```bash
# Reduce parallel jobs to 2-4
export CARGO_BUILD_JOBS=2

# Clean previous build attempts
cargo clean

# Build again
cargo build --release
```

---

## After Building

### Install Binary System-Wide

```bash
# Copy to /usr/local/bin
sudo cp ./target/release/flarechain-node /usr/local/bin/

# Make executable
sudo chmod +x /usr/local/bin/flarechain-node

# Verify
flarechain-node --version
```

### Test the Binary

```bash
# Check it runs
./target/release/flarechain-node --help

# Generate test keys
./target/release/flarechain-node key generate --scheme sr25519
```

### Deploy to Validator VM

```bash
# Copy to remote validator
scp ./target/release/flarechain-node user@<validator-ip>:/usr/local/bin/

# SSH in and make executable
ssh user@<validator-ip> 'chmod +x /usr/local/bin/flarechain-node'
```

---

## Building Specific Components

### Build Only Runtime

```bash
cargo build --release -p flarechain-runtime
```

### Build Specific PBC Collator

```bash
# Example: BTC PBC collator
cargo build --release -p btc-pbc-collator
```

### Build All PBC Collators

```bash
./scripts/build-all-pbc-collators.sh
```

---

## Cross-Compilation

### For ARM64 (aarch64)

```bash
# Add target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compiler
sudo apt-get install -y gcc-aarch64-linux-gnu

# Build
cargo build --release --target aarch64-unknown-linux-gnu
```

### For Different Architectures

```bash
# List available targets
rustup target list

# Add target
rustup target add <target-triple>

# Build for target
cargo build --release --target <target-triple>
```

---

## Keeping Up to Date

### Update Rust

```bash
rustup update stable
```

### Update Dependencies

```bash
cd ~/Desktop/etrid
cargo update
cargo build --release
```

### Rebuild After Updates

```bash
# Clean previous build
cargo clean

# Build fresh
cargo build --release
```

---

## Build Verification

### Check Binary

```bash
# File type
file ./target/release/flarechain-node

# Dependencies
ldd ./target/release/flarechain-node

# Size
ls -lh ./target/release/flarechain-node
```

### Test Run

```bash
# Version
./target/release/flarechain-node --version

# Help
./target/release/flarechain-node --help

# Key generation (test functionality)
./target/release/flarechain-node key generate
```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Build FlareChain Node

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y clang libssl-dev pkg-config

      - name: Build
        run: cargo build --release

      - name: Upload artifact
        uses: actions/upload-artifact@v2
        with:
          name: flarechain-node
          path: ./target/release/flarechain-node
```

---

## Next Steps

After building:

1. ✅ Binary built successfully
2. 📦 Install to `/usr/local/bin` or deploy to validator VMs
3. 🔑 Generate session keys on validator VMs
4. 🚀 Start validator service

**Continue with:** [Complete Setup Checklist](../guides/COMPLETE_SETUP_CHECKLIST.md)

---

**Last Updated:** November 9, 2025
