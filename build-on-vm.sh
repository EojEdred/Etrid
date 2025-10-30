#!/bin/bash
# Build Ëtrid binary on Ubuntu VM
# Usage: Run this script on the VM after cloning the repo

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Ëtrid Binary Build Script (Ubuntu VM)             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if running as root or with sudo
if [ "$EUID" -ne 0 ]; then
    echo "⚠️  This script should be run as root or with sudo"
    echo "Run: sudo ./build-on-vm.sh"
    exit 1
fi

echo "✅ Running as root"
echo ""

# Update system
echo "📦 Updating system packages..."
apt-get update -qq

# Install build dependencies
echo "📦 Installing build dependencies..."
apt-get install -y \
    build-essential \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    pkg-config \
    protobuf-compiler \
    cmake

echo "✅ Build dependencies installed"
echo ""

# Install Rust
if ! command -v rustc &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "✅ Rust installed"
else
    echo "✅ Rust already installed: $(rustc --version)"
fi

# Make sure we're using the right Rust version
source $HOME/.cargo/env
rustup default stable
rustup update

echo ""
echo "🔍 Rust version: $(rustc --version)"
echo "🔍 Cargo version: $(cargo --version)"
echo ""

# Install WASM compilation targets (required for Substrate runtime)
echo "🎯 Installing WASM compilation targets..."
rustup target add wasm32-unknown-unknown
rustup component add rust-src
echo "✅ WASM targets installed"
echo ""

# Clone or update repo
if [ ! -d "/root/etrid" ]; then
    echo "📥 Cloning Ëtrid repository..."
    cd /root
    git clone https://github.com/EojEdred/Etrid.git etrid
    cd etrid
else
    echo "📥 Updating Ëtrid repository..."
    cd /root/etrid
    git pull
fi

echo "✅ Repository ready"
echo ""

# Build the binary
echo "🔨 Building flarechain-node binary..."
echo "⏰ This will take 10-30 minutes depending on VM specs..."
echo ""

cd /root/etrid/05-multichain/flare-chain/node

# Build with optimizations
cargo build --release --features runtime-benchmarks

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "📍 Binary location:"
    echo "   /root/etrid/target/release/flarechain-node"
    echo ""

    # Show binary info
    BINARY="/root/etrid/target/release/flarechain-node"
    if [ -f "$BINARY" ]; then
        SIZE=$(du -h "$BINARY" | cut -f1)
        echo "📊 Binary size: $SIZE"
        echo ""
        echo "🧪 Testing binary..."
        $BINARY --help > /dev/null 2>&1 && echo "✅ Binary is working!" || echo "⚠️  Binary may have issues"
        echo ""
    fi
else
    echo ""
    echo "❌ Build failed!"
    echo "Check the error messages above"
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                   Build Complete!                          ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo "1. Install binary: cp /root/etrid/target/release/flarechain-node /usr/local/bin/"
echo "2. Insert validator keys (see validator-keys-setup/)"
echo "3. Start validator service"
echo ""
