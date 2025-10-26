#!/bin/bash
# Ëtrid Master Deployment Script
# Orchestrates complete deployment of FlareChain mainnet + DEX + Monitoring
#
# Usage: ./scripts/master-deploy.sh [--testnet|--mainnet] [--skip-monitoring] [--skip-dex]

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEPLOY_LOG="$PROJECT_ROOT/deploy-$(date +%Y%m%d-%H%M%S).log"

# Deployment options
NETWORK="testnet"  # Default to testnet for safety
SKIP_MONITORING=false
SKIP_DEX=false
DRY_RUN=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --mainnet)
      NETWORK="mainnet"
      shift
      ;;
    --testnet)
      NETWORK="testnet"
      shift
      ;;
    --skip-monitoring)
      SKIP_MONITORING=true
      shift
      ;;
    --skip-dex)
      SKIP_DEX=true
      shift
      ;;
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [--testnet|--mainnet] [--skip-monitoring] [--skip-dex] [--dry-run]"
      exit 1
      ;;
  esac
done

# Logging function
log() {
  echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$DEPLOY_LOG"
}

log_error() {
  echo -e "${RED}[ERROR]${NC} $1" | tee -a "$DEPLOY_LOG"
}

log_warning() {
  echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$DEPLOY_LOG"
}

log_info() {
  echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$DEPLOY_LOG"
}

# Banner
echo -e "${GREEN}"
cat << "EOF"
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║     ËTRID MASTER DEPLOYMENT ORCHESTRATION                     ║
║     FlareChain Mainnet + DEX + Monitoring                     ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

log_info "Deployment target: $NETWORK"
log_info "Deployment log: $DEPLOY_LOG"

if [ "$DRY_RUN" = true ]; then
  log_warning "DRY RUN MODE - No actual deployment will occur"
fi

# Confirmation for mainnet
if [ "$NETWORK" = "mainnet" ] && [ "$DRY_RUN" = false ]; then
  echo -e "${RED}⚠️  WARNING: You are about to deploy to MAINNET${NC}"
  echo -e "${RED}⚠️  This is a PRODUCTION deployment and cannot be easily reversed${NC}"
  echo ""
  read -p "Are you absolutely sure you want to continue? (type 'DEPLOY MAINNET' to confirm): " confirmation
  if [ "$confirmation" != "DEPLOY MAINNET" ]; then
    log_error "Deployment cancelled by user"
    exit 1
  fi
fi

# ═══════════════════════════════════════════════════════════════
# PHASE 1: PRE-DEPLOYMENT CHECKS
# ═══════════════════════════════════════════════════════════════

log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log "PHASE 1: Pre-Deployment Checks"
log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check required tools
log_info "Checking required tools..."

REQUIRED_TOOLS=(
  "cargo:Rust compiler"
  "node:Node.js"
  "npm:NPM package manager"
  "docker:Docker container runtime"
  "git:Git version control"
)

for tool_info in "${REQUIRED_TOOLS[@]}"; do
  IFS=':' read -r tool desc <<< "$tool_info"
  if command -v "$tool" &> /dev/null; then
    log "  ✓ $desc ($tool) found"
  else
    log_error "  ✗ $desc ($tool) not found - please install"
    exit 1
  fi
done

# Check environment variables
log_info "Checking environment variables..."

if [ ! -f "$PROJECT_ROOT/.env" ]; then
  log_warning ".env file not found, creating from example..."
  if [ -f "$PROJECT_ROOT/.env.example" ]; then
    cp "$PROJECT_ROOT/.env.example" "$PROJECT_ROOT/.env"
    log_error "Please configure .env file and run again"
    exit 1
  fi
fi

# Source environment
source "$PROJECT_ROOT/.env"

REQUIRED_VARS=(
  "DEPLOYER_PRIVATE_KEY:Deployer wallet private key"
  "BSCSCAN_API_KEY:BSCScan API key for verification"
)

if [ "$NETWORK" = "mainnet" ]; then
  REQUIRED_VARS+=(
    "BSC_RPC_URL:BSC mainnet RPC URL"
    "SOLANA_RPC_URL:Solana mainnet RPC URL"
  )
fi

for var_info in "${REQUIRED_VARS[@]}"; do
  IFS=':' read -r var desc <<< "$var_info"
  if [ -z "${!var:-}" ]; then
    log_error "  ✗ $desc ($var) not set in .env"
    exit 1
  else
    log "  ✓ $desc configured"
  fi
done

# Check disk space
log_info "Checking disk space..."
AVAILABLE_SPACE=$(df -BG "$PROJECT_ROOT" | awk 'NR==2 {print $4}' | sed 's/G//')
if [ "$AVAILABLE_SPACE" -lt 50 ]; then
  log_error "Insufficient disk space: ${AVAILABLE_SPACE}GB available, 50GB+ required"
  exit 1
fi
log "  ✓ Sufficient disk space: ${AVAILABLE_SPACE}GB available"

# ═══════════════════════════════════════════════════════════════
# PHASE 2: BUILD RUNTIME
# ═══════════════════════════════════════════════════════════════

log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log "PHASE 2: Building FlareChain Runtime"
log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$DRY_RUN" = false ]; then
  log_info "Running cargo build --release (this may take 30-45 minutes)..."
  cd "$PROJECT_ROOT"

  # Clean build for safety
  log_info "Cleaning previous builds..."
  cargo clean

  # Build with locked dependencies
  log_info "Building runtime..."
  if cargo build --release --locked 2>&1 | tee -a "$DEPLOY_LOG"; then
    log "  ✓ Runtime build successful"
  else
    log_error "Runtime build failed - check $DEPLOY_LOG"
    exit 1
  fi

  # Verify binary
  if [ -f "$PROJECT_ROOT/target/release/etrid" ]; then
    BINARY_SIZE=$(du -h "$PROJECT_ROOT/target/release/etrid" | cut -f1)
    log "  ✓ Binary created: $BINARY_SIZE"
  else
    log_error "Binary not found at expected location"
    exit 1
  fi
else
  log_info "  [DRY RUN] Skipping build"
fi

# ═══════════════════════════════════════════════════════════════
# PHASE 3: GENERATE CHAIN SPECIFICATION
# ═══════════════════════════════════════════════════════════════

log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log "PHASE 3: Generating Chain Specification"
log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$DRY_RUN" = false ]; then
  CHAIN_SPEC_NAME="flarechain_mainnet_with_vesting"
  if [ "$NETWORK" = "testnet" ]; then
    CHAIN_SPEC_NAME="ember_testnet"
  fi

  log_info "Generating chain spec for: $CHAIN_SPEC_NAME"

  # Generate human-readable chain spec
  if "$PROJECT_ROOT/target/release/etrid" build-spec \
    --chain "$CHAIN_SPEC_NAME" \
    > "$PROJECT_ROOT/chain-spec-$NETWORK.json" 2>&1 | tee -a "$DEPLOY_LOG"; then
    log "  ✓ Chain spec generated (human-readable)"
  else
    log_error "Chain spec generation failed"
    exit 1
  fi

  # Generate raw chain spec
  if "$PROJECT_ROOT/target/release/etrid" build-spec \
    --chain "$PROJECT_ROOT/chain-spec-$NETWORK.json" \
    --raw \
    > "$PROJECT_ROOT/chain-spec-$NETWORK-raw.json" 2>&1 | tee -a "$DEPLOY_LOG"; then
    log "  ✓ Raw chain spec generated"
  else
    log_error "Raw chain spec generation failed"
    exit 1
  fi

  # Verify chain spec
  SPEC_SIZE=$(du -h "$PROJECT_ROOT/chain-spec-$NETWORK-raw.json" | cut -f1)
  log "  ✓ Chain spec size: $SPEC_SIZE"
else
  log_info "  [DRY RUN] Skipping chain spec generation"
fi

# ═══════════════════════════════════════════════════════════════
# PHASE 4: DEPLOY DEX TOKENS
# ═══════════════════════════════════════════════════════════════

if [ "$SKIP_DEX" = false ]; then
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  log "PHASE 4: Deploying DEX Tokens"
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  # BSC Token Deployment
  log_info "Deploying ÉTR token to BSC..."
  cd "$PROJECT_ROOT/contracts/ethereum"

  if [ ! -d "node_modules" ]; then
    log_info "Installing contract dependencies..."
    npm install
  fi

  if [ "$DRY_RUN" = false ]; then
    BSC_NETWORK="bscTestnet"
    if [ "$NETWORK" = "mainnet" ]; then
      BSC_NETWORK="bsc"
    fi

    log_info "Deploying to $BSC_NETWORK..."
    if npx hardhat run scripts/deploy-bsc.js --network "$BSC_NETWORK" 2>&1 | tee -a "$DEPLOY_LOG"; then
      log "  ✓ BSC token deployed successfully"
    else
      log_error "BSC token deployment failed"
      exit 1
    fi
  else
    log_info "  [DRY RUN] Skipping BSC deployment"
  fi

  # Solana Token Deployment
  log_info "Solana token deployment requires manual steps (see DEX_DEPLOYMENT_GUIDE.md)"
  log_warning "  ! Run: spl-token create-token --decimals 9"
  log_warning "  ! See: ai-devs/DEX_DEPLOYMENT_GUIDE.md Part 2"

else
  log_info "PHASE 4: Skipping DEX deployment (--skip-dex flag)"
fi

# ═══════════════════════════════════════════════════════════════
# PHASE 5: DEPLOY MONITORING STACK
# ═══════════════════════════════════════════════════════════════

if [ "$SKIP_MONITORING" = false ]; then
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  log "PHASE 5: Deploying Monitoring Stack"
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  if [ "$DRY_RUN" = false ]; then
    if [ -f "$SCRIPT_DIR/deploy-monitoring.sh" ]; then
      log_info "Running monitoring deployment script..."
      bash "$SCRIPT_DIR/deploy-monitoring.sh" 2>&1 | tee -a "$DEPLOY_LOG"
      log "  ✓ Monitoring stack deployed"
    else
      log_warning "Monitoring deployment script not found at $SCRIPT_DIR/deploy-monitoring.sh"
      log_info "See: ai-devs/MONITORING_INFRASTRUCTURE_GUIDE.md"
    fi
  else
    log_info "  [DRY RUN] Skipping monitoring deployment"
  fi
else
  log_info "PHASE 5: Skipping monitoring deployment (--skip-monitoring flag)"
fi

# ═══════════════════════════════════════════════════════════════
# PHASE 6: START VALIDATOR NODES
# ═══════════════════════════════════════════════════════════════

log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log "PHASE 6: Starting Validator Nodes"
log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$DRY_RUN" = false ]; then
  # Create systemd service for validator
  log_info "Creating validator systemd service..."

  sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<EOF
[Unit]
Description=Ëtrid FlareChain Validator
After=network-online.target
Wants=network-online.target

[Service]
User=$USER
Group=$USER
Type=simple
ExecStart=$PROJECT_ROOT/target/release/etrid \\
  --validator \\
  --chain $PROJECT_ROOT/chain-spec-$NETWORK-raw.json \\
  --name "Foundation-Validator-1" \\
  --prometheus-external \\
  --prometheus-port 9615 \\
  --base-path /data/etrid \\
  --rpc-port 9933 \\
  --port 30333 \\
  --rpc-cors all
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

  log "  ✓ Systemd service created"

  # Create data directory
  sudo mkdir -p /data/etrid
  sudo chown -R $USER:$USER /data/etrid

  log_info "Starting validator service..."
  sudo systemctl daemon-reload
  sudo systemctl enable etrid-validator
  sudo systemctl start etrid-validator

  sleep 5

  if sudo systemctl is-active --quiet etrid-validator; then
    log "  ✓ Validator service started successfully"
  else
    log_error "Validator service failed to start"
    sudo journalctl -u etrid-validator -n 50
    exit 1
  fi
else
  log_info "  [DRY RUN] Skipping validator start"
fi

# ═══════════════════════════════════════════════════════════════
# PHASE 7: VERIFY DEPLOYMENT
# ═══════════════════════════════════════════════════════════════

log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log "PHASE 7: Verifying Deployment"
log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$DRY_RUN" = false ]; then
  # Wait for node to sync
  log_info "Waiting for node to start syncing..."
  sleep 10

  # Check RPC endpoint
  if curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
    http://localhost:9933 > /dev/null; then
    log "  ✓ RPC endpoint responsive"
  else
    log_warning "RPC endpoint not responding (may take a few minutes)"
  fi

  # Check Prometheus metrics
  if curl -s http://localhost:9615/metrics > /dev/null; then
    log "  ✓ Prometheus metrics exposed"
  else
    log_warning "Prometheus metrics not available"
  fi

  # Check monitoring stack
  if [ "$SKIP_MONITORING" = false ]; then
    if curl -s http://localhost:9090 > /dev/null; then
      log "  ✓ Prometheus web UI accessible"
    fi

    if curl -s http://localhost:3000 > /dev/null; then
      log "  ✓ Grafana web UI accessible"
    fi
  fi
else
  log_info "  [DRY RUN] Skipping verification"
fi

# ═══════════════════════════════════════════════════════════════
# DEPLOYMENT COMPLETE
# ═══════════════════════════════════════════════════════════════

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                                                               ║${NC}"
echo -e "${GREEN}║          🎉 DEPLOYMENT COMPLETE! 🎉                           ║${NC}"
echo -e "${GREEN}║                                                               ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

log "Deployment Summary:"
log "  Network: $NETWORK"
log "  Runtime version: 103"
log "  Chain spec: $PROJECT_ROOT/chain-spec-$NETWORK-raw.json"
log "  Deployment log: $DEPLOY_LOG"

echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo ""
echo "1. Access your node:"
echo "   - RPC endpoint: http://localhost:9933"
echo "   - Websocket: ws://localhost:9944"
echo "   - Metrics: http://localhost:9615/metrics"
echo ""

if [ "$SKIP_MONITORING" = false ]; then
  echo "2. Access monitoring dashboards:"
  echo "   - Prometheus: http://localhost:9090"
  echo "   - Grafana: http://localhost:3000 (admin/admin)"
  echo "   - Alertmanager: http://localhost:9093"
  echo ""
fi

echo "3. Check validator status:"
echo "   sudo systemctl status etrid-validator"
echo "   sudo journalctl -u etrid-validator -f"
echo ""

echo "4. Monitor logs:"
echo "   tail -f $DEPLOY_LOG"
echo ""

if [ "$NETWORK" = "mainnet" ]; then
  echo -e "${YELLOW}IMPORTANT:${NC}"
  echo "  - Ensure backup keys are stored securely"
  echo "  - Configure firewall rules"
  echo "  - Set up SSL certificates for RPC endpoints"
  echo "  - Test all functionality thoroughly"
  echo ""
fi

log "Deployment orchestration complete!"
exit 0
