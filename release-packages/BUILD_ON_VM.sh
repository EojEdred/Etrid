#!/bin/bash
# Quick script to build FlareChain Linux binary on VM

set -e

echo "🔥 FlareChain Linux Binary Build Script"
echo "========================================"
echo ""

# VM details
VM_IP="98.71.91.84"
VM_USER="ubuntu"

echo "This script will:"
echo "1. SSH to your VM ($VM_USER@$VM_IP)"
echo "2. Clone the Etrid repository from GitHub"
echo "3. Install build dependencies"
echo "4. Build the Linux binary"
echo "5. Download it to your Mac"
echo ""
read -p "Press Enter to continue..."

# SSH and build
echo ""
echo "=== Connecting to VM and building ==="
ssh $VM_USER@$VM_IP << 'EOF'
set -e

echo "✅ Connected to VM"
echo ""

# Install dependencies
echo "📦 Installing build dependencies..."
sudo apt update -qq
sudo apt install -y build-essential protobuf-compiler git curl -qq

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo "✅ Dependencies installed"
echo ""

# Clone repo if not exists
if [ ! -d "Etrid" ]; then
    echo "📥 Cloning Etrid repository..."
    git clone https://github.com/EojEdred/Etrid.git
else
    echo "📥 Updating Etrid repository..."
    cd Etrid
    git pull
    cd ..
fi

echo "✅ Repository ready"
echo ""

# Build
echo "🔨 Building FlareChain (this will take 15-20 minutes)..."
cd Etrid/05-multichain/flare-chain/node
cargo build --release --bin flarechain-node

echo ""
echo "✅ BUILD COMPLETE!"
echo ""
echo "Binary location: ~/Etrid/target/release/flarechain-node"
ls -lh ~/Etrid/target/release/flarechain-node

EOF

# Download binary
echo ""
echo "=== Downloading binary to Mac ==="
scp $VM_USER@$VM_IP:~/Etrid/target/release/flarechain-node ../linux-x86_64/

echo ""
echo "✅ DONE! Linux binary ready at:"
echo "   release-packages/linux-x86_64/flarechain-node"
echo ""
echo "🔥 Both binaries are now ready for deployment!"
