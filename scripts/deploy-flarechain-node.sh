#!/bin/bash
set -e

################################################################################
# FlareChain Node Deployment Script
#
# This script deploys and runs the FlareChain validator node on production VMs
################################################################################

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO_DIR="${REPO_DIR:-/opt/etrid}"
CHAIN_SPEC="${CHAIN_SPEC:-flarechain-mainnet-v108-raw.json}"
NODE_NAME="${NODE_NAME:-FlareChain-Validator-$(hostname)}"
VALIDATOR_KEY="${VALIDATOR_KEY:-}"
BUILD_ON_VM="${BUILD_ON_VM:-true}"
PURGE_CHAIN="${PURGE_CHAIN:-false}"
BASE_PATH="${BASE_PATH:-/var/lib/flarechain}"

# Git configuration
GIT_BRANCH="${GIT_BRANCH:-claude/chain-spec-local-testing-01Vsk2ZiZSovJrb9upMzxcYv}"

################################################################################
# Helper Functions
################################################################################

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        log_error "$1 is not installed. Please install it first."
        exit 1
    fi
}

################################################################################
# Pre-flight Checks
################################################################################

log_info "Running pre-flight checks..."

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   log_warn "Running as root. Consider using a dedicated user for security."
fi

# Check required commands
check_command git
check_command cargo

# Check for Rust
if ! command -v rustc &> /dev/null; then
    log_error "Rust is not installed. Install from https://rustup.rs/"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
log_info "Rust version: $RUST_VERSION"

################################################################################
# Step 1: Clone/Pull Repository
################################################################################

log_info "Step 1: Updating repository..."

if [ -d "$REPO_DIR" ]; then
    log_info "Repository exists. Pulling latest changes..."
    cd "$REPO_DIR"

    # Stash any local changes
    git stash

    # Fetch all branches
    git fetch --all

    # Checkout the specified branch
    git checkout "$GIT_BRANCH"

    # Pull latest changes
    git pull origin "$GIT_BRANCH"

    log_info "Repository updated successfully"
else
    log_info "Cloning repository..."
    git clone https://github.com/EojEdred/Etrid.git "$REPO_DIR"
    cd "$REPO_DIR"
    git checkout "$GIT_BRANCH"
    log_info "Repository cloned successfully"
fi

################################################################################
# Step 2: Install Dependencies
################################################################################

log_info "Step 2: Installing system dependencies..."

if command -v apt-get &> /dev/null; then
    log_info "Installing dependencies via apt-get..."
    sudo apt-get update
    sudo apt-get install -y \
        build-essential \
        git \
        clang \
        curl \
        libssl-dev \
        llvm \
        libudev-dev \
        protobuf-compiler \
        pkg-config
elif command -v yum &> /dev/null; then
    log_info "Installing dependencies via yum..."
    sudo yum groupinstall -y "Development Tools"
    sudo yum install -y \
        clang \
        curl \
        openssl-devel \
        llvm \
        systemd-devel \
        protobuf-compiler
else
    log_warn "Could not detect package manager. Please install dependencies manually."
fi

################################################################################
# Step 3: Add WASM Target
################################################################################

log_info "Step 3: Adding WASM build target..."
rustup target add wasm32-unknown-unknown
rustup component add rust-src

################################################################################
# Step 4: Build Binary
################################################################################

if [ "$BUILD_ON_VM" = "true" ]; then
    log_info "Step 4: Building FlareChain node binary (this may take 20-30 minutes)..."

    cd "$REPO_DIR/05-multichain/flare-chain/node"

    # Clean build (optional, comment out for faster rebuilds)
    # cargo clean

    # Build in release mode
    log_info "Starting release build..."
    RUST_LOG=info cargo build --release 2>&1 | tee /tmp/flarechain-build.log

    if [ ${PIPESTATUS[0]} -ne 0 ]; then
        log_error "Build failed! Check /tmp/flarechain-build.log for details"
        exit 1
    fi

    BINARY_PATH="$REPO_DIR/target/release/flarechain-node"
    log_info "Build completed successfully!"
else
    log_info "Step 4: Skipping build (using pre-built binary)"
    BINARY_PATH="$REPO_DIR/target/release/flarechain-node"

    if [ ! -f "$BINARY_PATH" ]; then
        log_error "Binary not found at $BINARY_PATH. Set BUILD_ON_VM=true to build."
        exit 1
    fi
fi

# Verify binary
log_info "Verifying binary..."
$BINARY_PATH --version

################################################################################
# Step 5: Setup Chain Spec
################################################################################

log_info "Step 5: Setting up chain specification..."

CHAIN_SPEC_PATH="$REPO_DIR/$CHAIN_SPEC"

if [ ! -f "$CHAIN_SPEC_PATH" ]; then
    log_error "Chain spec not found at $CHAIN_SPEC_PATH"
    log_info "Generating chain spec from mainnet preset..."

    $BINARY_PATH build-spec \
        --chain=mainnet \
        --raw \
        --disable-default-bootnode \
        > "$CHAIN_SPEC_PATH" 2>&1 | tail -n +2

    log_info "Chain spec generated at $CHAIN_SPEC_PATH"
fi

log_info "Chain spec ready: $CHAIN_SPEC_PATH"

################################################################################
# Step 6: Purge Chain (if requested)
################################################################################

if [ "$PURGE_CHAIN" = "true" ]; then
    log_warn "Step 6: Purging chain data..."
    read -p "Are you sure you want to purge the chain? This will delete all blockchain data. (yes/no): " confirm

    if [ "$confirm" = "yes" ]; then
        $BINARY_PATH purge-chain \
            --chain="$CHAIN_SPEC_PATH" \
            --base-path="$BASE_PATH" \
            -y
        log_info "Chain purged successfully"
    else
        log_info "Chain purge cancelled"
    fi
else
    log_info "Step 6: Skipping chain purge (set PURGE_CHAIN=true to purge)"
fi

################################################################################
# Step 7: Create Systemd Service
################################################################################

log_info "Step 7: Creating systemd service..."

# Create base path directory
sudo mkdir -p "$BASE_PATH"
sudo chown -R $(whoami):$(whoami) "$BASE_PATH"

SERVICE_FILE="/etc/systemd/system/flarechain-node.service"

sudo tee "$SERVICE_FILE" > /dev/null <<EOF
[Unit]
Description=FlareChain Validator Node
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=$(whoami)
Group=$(whoami)
WorkingDirectory=$REPO_DIR
ExecStart=$BINARY_PATH \\
    --chain=$CHAIN_SPEC_PATH \\
    --base-path=$BASE_PATH \\
    --name="$NODE_NAME" \\
    --validator \\
    --rpc-cors=all \\
    --rpc-methods=Unsafe \\
    --rpc-external \\
    --prometheus-external \\
    --telemetry-url='wss://telemetry.polkadot.io/submit/ 0'

Restart=always
RestartSec=10
LimitNOFILE=65536

StandardOutput=journal
StandardError=journal
SyslogIdentifier=flarechain-node

[Install]
WantedBy=multi-user.target
EOF

log_info "Systemd service created at $SERVICE_FILE"

################################################################################
# Step 8: Generate/Insert Validator Keys
################################################################################

log_info "Step 8: Validator key management..."

if [ -n "$VALIDATOR_KEY" ]; then
    log_info "Inserting validator key..."

    # Insert aura key
    $BINARY_PATH key insert \
        --base-path="$BASE_PATH" \
        --chain="$CHAIN_SPEC_PATH" \
        --key-type aura \
        --suri "$VALIDATOR_KEY"

    # Insert grandpa key (if needed)
    # $BINARY_PATH key insert \
    #     --base-path="$BASE_PATH" \
    #     --chain="$CHAIN_SPEC_PATH" \
    #     --key-type gran \
    #     --suri "$VALIDATOR_KEY"

    log_info "Validator keys inserted"
else
    log_warn "No validator key provided. Set VALIDATOR_KEY environment variable."
    log_info "You can generate keys with: $BINARY_PATH key generate"
fi

################################################################################
# Step 9: Enable and Start Service
################################################################################

log_info "Step 9: Starting FlareChain node service..."

# Reload systemd
sudo systemctl daemon-reload

# Enable service to start on boot
sudo systemctl enable flarechain-node

# Start the service
sudo systemctl start flarechain-node

# Check status
sleep 3
sudo systemctl status flarechain-node --no-pager

################################################################################
# Step 10: Display Information
################################################################################

log_info "========================================="
log_info "FlareChain Node Deployment Complete!"
log_info "========================================="
echo ""
log_info "Node Details:"
echo "  Name: $NODE_NAME"
echo "  Chain: $CHAIN_SPEC_PATH"
echo "  Data Directory: $BASE_PATH"
echo "  Binary: $BINARY_PATH"
echo ""
log_info "Service Management:"
echo "  Start:   sudo systemctl start flarechain-node"
echo "  Stop:    sudo systemctl stop flarechain-node"
echo "  Restart: sudo systemctl restart flarechain-node"
echo "  Status:  sudo systemctl status flarechain-node"
echo "  Logs:    sudo journalctl -u flarechain-node -f"
echo ""
log_info "Monitoring:"
echo "  Prometheus: http://$(hostname -I | awk '{print $1}'):9615/metrics"
echo "  RPC:        http://$(hostname -I | awk '{print $1}'):9944"
echo ""
log_info "To view live logs:"
echo "  sudo journalctl -u flarechain-node -f --output=cat"
echo ""
log_info "Deployment completed successfully!"
