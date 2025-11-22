#!/bin/bash
# Build Primearc Core binary on VM from git branch

BUILD_VM="vmi2896906"
SSH_KEY="$HOME/.ssh/contabo-validators"
GIT_BRANCH="main"
REPO_URL="https://github.com/EojEdred/Etrid.git"

echo "═══════════════════════════════════════════════════════════════════════"
echo "Building Primearc Core on $BUILD_VM"
echo "═══════════════════════════════════════════════════════════════════════"
echo "Branch: $GIT_BRANCH"
echo ""

ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$BUILD_VM" bash << EOF
set -e

echo "Step 1: Installing build dependencies..."
apt-get update -qq
apt-get install -y build-essential git curl protobuf-compiler clang libssl-dev pkg-config > /dev/null 2>&1
echo "✅ Dependencies installed"

echo ""
echo "Step 2: Installing Rust..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source \$HOME/.cargo/env
    echo "✅ Rust installed"
else
    source \$HOME/.cargo/env
    echo "✅ Rust already installed"
fi

echo ""
echo "Step 3: Adding WASM target..."
rustup target add wasm32-unknown-unknown
rustup component add rust-src
echo "✅ WASM target added"

echo ""
echo "Step 4: Cloning repository..."
if [ -d "/opt/etrid-build" ]; then
    cd /opt/etrid-build
    git fetch origin
    git checkout $GIT_BRANCH
    git pull origin $GIT_BRANCH
    echo "✅ Repository updated"
else
    git clone -b $GIT_BRANCH $REPO_URL /opt/etrid-build
    cd /opt/etrid-build
    echo "✅ Repository cloned"
fi

echo ""
echo "Step 5: Building Primearc Core binary..."
echo "This will take 20-30 minutes..."
cd /opt/etrid-build/05-multichain/primearc-core
cargo build --release 2>&1 | tail -20
echo "✅ Binary built"

echo ""
echo "Step 6: Generating chain spec..."
./target/release/primearc-node build-spec --chain=mainnet --raw > /tmp/primearc-mainnet-raw.json 2>&1 || \
./target/release/primearc-node build-spec --raw > /tmp/primearc-mainnet-raw.json 2>&1
echo "✅ Chain spec generated"

echo ""
echo "Binary location: /opt/etrid-build/05-multichain/primearc-core/target/release/primearc-node"
echo "Chain spec location: /tmp/primearc-mainnet-raw.json"
ls -lh /opt/etrid-build/05-multichain/primearc-core/target/release/primearc-node
ls -lh /tmp/primearc-mainnet-raw.json
EOF

echo ""
echo "═══════════════════════════════════════════════════════════════════════"
echo "Build completed on $BUILD_VM!"
echo "═══════════════════════════════════════════════════════════════════════"
