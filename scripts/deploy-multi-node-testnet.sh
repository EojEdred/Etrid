#!/bin/bash
# Ëtrid Protocol - Multi-Node Testnet Deployment
# Deploys 4-validator testnet for consensus and network testing

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

ETRID_ROOT="${ETRID_ROOT:-$(pwd)}"
TESTNET_DIR="$ETRID_ROOT/data/multi-node-testnet"
BINARY="$ETRID_ROOT/target/release/flarechain-node"

# Configuration
NUM_VALIDATORS="${NUM_VALIDATORS:-4}"
BASE_P2P_PORT=30333
BASE_RPC_PORT=9944
BASE_WS_PORT=9945
BASE_PROM_PORT=9615

echo -e "${PURPLE}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║     ËTRID MULTI-NODE TESTNET DEPLOYMENT                     ║"
echo "║     $NUM_VALIDATORS-Validator Test Network                                  ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ============================================================================
# Check Prerequisites
# ============================================================================

check_prerequisites() {
    log_info "Checking prerequisites..."

    if [ ! -f "$BINARY" ]; then
        log_error "FlareChain binary not found: $BINARY"
        log_info "Build with: cargo build --release -p flarechain-node"
        exit 1
    fi

    log_success "Binary found: $BINARY"

    # Create testnet directory
    mkdir -p "$TESTNET_DIR"
    log_success "Testnet directory: $TESTNET_DIR"
}

# ============================================================================
# Generate Node Keys
# ============================================================================

generate_node_keys() {
    log_info "Generating node keys for $NUM_VALIDATORS validators..."

    VALIDATOR_KEYS=()

    for i in $(seq 0 $((NUM_VALIDATORS - 1))); do
        NODE_NAME="validator-$i"
        NODE_DIR="$TESTNET_DIR/$NODE_NAME"
        mkdir -p "$NODE_DIR"

        # Generate node key
        NODE_KEY=$($BINARY key generate-node-key --file "$NODE_DIR/node-key" 2>&1 | grep -v "^2" || true)

        # Get peer ID
        PEER_ID=$($BINARY key inspect-node-key --file "$NODE_DIR/node-key" 2>&1 | grep -v "^2" || true)

        VALIDATOR_KEYS+=("$NODE_NAME:$PEER_ID")

        log_success "  $NODE_NAME: $PEER_ID"
    done
}

# ============================================================================
# Generate Chain Spec
# ============================================================================

generate_chain_spec() {
    log_info "Generating chain specification..."

    # Generate initial chain spec
    $BINARY build-spec --chain dev > "$TESTNET_DIR/chain-spec-plain.json" 2>/dev/null || {
        log_warning "Could not generate chain spec (may not be implemented yet)"
        log_info "Using dev chain configuration instead"
        return 0
    }

    log_success "Chain spec generated: $TESTNET_DIR/chain-spec-plain.json"

    # Convert to raw format
    $BINARY build-spec --chain "$TESTNET_DIR/chain-spec-plain.json" --raw > "$TESTNET_DIR/chain-spec-raw.json" 2>/dev/null || true

    log_success "Raw chain spec: $TESTNET_DIR/chain-spec-raw.json"
}

# ============================================================================
# Create Node Startup Scripts
# ============================================================================

create_startup_scripts() {
    log_info "Creating node startup scripts..."

    for i in $(seq 0 $((NUM_VALIDATORS - 1))); do
        NODE_NAME="validator-$i"
        NODE_DIR="$TESTNET_DIR/$NODE_NAME"

        # Calculate ports
        P2P_PORT=$((BASE_P2P_PORT + i))
        RPC_PORT=$((BASE_RPC_PORT + i))
        WS_PORT=$((BASE_WS_PORT + i))
        PROM_PORT=$((BASE_PROM_PORT + i))

        # Determine validator account
        case $i in
            0) VALIDATOR="alice" ;;
            1) VALIDATOR="bob" ;;
            2) VALIDATOR="charlie" ;;
            3) VALIDATOR="dave" ;;
            *) VALIDATOR="eve" ;;
        esac

        # Create startup script
        cat > "$NODE_DIR/start.sh" <<EOF
#!/bin/bash
# Start $NODE_NAME

cd "$ETRID_ROOT"

$BINARY \\
  --chain dev \\
  --$VALIDATOR \\
  --base-path "$NODE_DIR/data" \\
  --node-key-file "$NODE_DIR/node-key" \\
  --name "$NODE_NAME" \\
  --validator \\
  \`# Network ports\` \\
  --port $P2P_PORT \\
  --rpc-port $RPC_PORT \\
  --ws-port $WS_PORT \\
  --rpc-cors all \\
  \`# Performance\` \\
  --db-cache 2048 \\
  --state-cache-size 536870912 \\
  --pruning 256 \\
  \`# Monitoring\` \\
  --prometheus-external \\
  --prometheus-port $PROM_PORT \\
  \`# Logging\` \\
  --log info,runtime=debug \\
  "\$@"
EOF

        chmod +x "$NODE_DIR/start.sh"

        log_success "  Created: $NODE_DIR/start.sh"
    done
}

# ============================================================================
# Create Management Scripts
# ============================================================================

create_management_scripts() {
    log_info "Creating management scripts..."

    # Start all nodes
    cat > "$TESTNET_DIR/start-all.sh" <<'EOF'
#!/bin/bash
# Start all validator nodes

TESTNET_DIR="$(dirname "$0")"

echo "Starting all validators..."

for node_dir in "$TESTNET_DIR"/validator-*; do
    if [ -d "$node_dir" ] && [ -f "$node_dir/start.sh" ]; then
        NODE_NAME=$(basename "$node_dir")
        echo "Starting $NODE_NAME..."

        # Start in background and redirect output to log
        "$node_dir/start.sh" > "$node_dir/node.log" 2>&1 &
        PID=$!
        echo $PID > "$node_dir/node.pid"

        echo "  PID: $PID"
        echo "  Log: $node_dir/node.log"
    fi
done

echo ""
echo "All nodes started. Check status with: ./status.sh"
EOF

    chmod +x "$TESTNET_DIR/start-all.sh"

    # Stop all nodes
    cat > "$TESTNET_DIR/stop-all.sh" <<'EOF'
#!/bin/bash
# Stop all validator nodes

TESTNET_DIR="$(dirname "$0")"

echo "Stopping all validators..."

for node_dir in "$TESTNET_DIR"/validator-*; do
    if [ -f "$node_dir/node.pid" ]; then
        NODE_NAME=$(basename "$node_dir")
        PID=$(cat "$node_dir/node.pid")

        echo "Stopping $NODE_NAME (PID: $PID)..."

        if kill -0 $PID 2>/dev/null; then
            kill -TERM $PID
            sleep 2
            if kill -0 $PID 2>/dev/null; then
                kill -KILL $PID
            fi
            echo "  Stopped"
        else
            echo "  Not running"
        fi

        rm "$node_dir/node.pid"
    fi
done

echo "All nodes stopped"
EOF

    chmod +x "$TESTNET_DIR/stop-all.sh"

    # Status check
    cat > "$TESTNET_DIR/status.sh" <<'EOF'
#!/bin/bash
# Check status of all nodes

TESTNET_DIR="$(dirname "$0")"

echo "=== Ëtrid Multi-Node Testnet Status ==="
echo ""

RUNNING=0
STOPPED=0

for node_dir in "$TESTNET_DIR"/validator-*; do
    if [ -d "$node_dir" ]; then
        NODE_NAME=$(basename "$node_dir")

        if [ -f "$node_dir/node.pid" ]; then
            PID=$(cat "$node_dir/node.pid")

            if kill -0 $PID 2>/dev/null; then
                # Extract port from start script
                RPC_PORT=$(grep "rpc-port" "$node_dir/start.sh" | grep -o '[0-9]*')
                PROM_PORT=$(grep "prometheus-port" "$node_dir/start.sh" | grep -o '[0-9]*')

                # Check if responsive
                if curl -s http://localhost:$RPC_PORT &>/dev/null; then
                    echo "✓ $NODE_NAME (PID: $PID)"
                    echo "    RPC: http://localhost:$RPC_PORT"
                    echo "    Metrics: http://localhost:$PROM_PORT"

                    # Get block height
                    BLOCK_RESP=$(curl -s -H "Content-Type: application/json" \
                        -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
                        http://localhost:$RPC_PORT 2>/dev/null)

                    BLOCK_HEX=$(echo "$BLOCK_RESP" | grep -o '"number":"0x[^"]*"' | sed 's/"number":"0x//' | sed 's/"//')
                    if [ -n "$BLOCK_HEX" ]; then
                        BLOCK_DEC=$((16#$BLOCK_HEX))
                        echo "    Block: $BLOCK_DEC"
                    fi

                    RUNNING=$((RUNNING + 1))
                else
                    echo "⚠ $NODE_NAME (PID: $PID) - Not responding"
                fi
            else
                echo "✗ $NODE_NAME - Not running (stale PID)"
                STOPPED=$((STOPPED + 1))
            fi
        else
            echo "✗ $NODE_NAME - Not started"
            STOPPED=$((STOPPED + 1))
        fi

        echo ""
    fi
done

echo "Summary: $RUNNING running, $STOPPED stopped"
EOF

    chmod +x "$TESTNET_DIR/status.sh"

    # Logs viewer
    cat > "$TESTNET_DIR/view-logs.sh" <<'EOF'
#!/bin/bash
# View logs from all nodes

TESTNET_DIR="$(dirname "$0")"
NODE="${1:-validator-0}"

if [ -f "$TESTNET_DIR/$NODE/node.log" ]; then
    tail -f "$TESTNET_DIR/$NODE/node.log"
else
    echo "Node log not found: $NODE"
    echo "Available nodes:"
    ls -1 "$TESTNET_DIR" | grep "validator-"
fi
EOF

    chmod +x "$TESTNET_DIR/view-logs.sh"

    log_success "Management scripts created"
}

# ============================================================================
# Create README
# ============================================================================

create_readme() {
    cat > "$TESTNET_DIR/README.md" <<EOF
# Ëtrid Multi-Node Testnet

**Validators:** $NUM_VALIDATORS
**Created:** $(date)

## Quick Start

\`\`\`bash
# Start all nodes
./start-all.sh

# Check status
./status.sh

# View logs
./view-logs.sh validator-0

# Stop all nodes
./stop-all.sh
\`\`\`

## Node Configuration

EOF

    for i in $(seq 0 $((NUM_VALIDATORS - 1))); do
        P2P_PORT=$((BASE_P2P_PORT + i))
        RPC_PORT=$((BASE_RPC_PORT + i))
        WS_PORT=$((BASE_WS_PORT + i))
        PROM_PORT=$((BASE_PROM_PORT + i))

        cat >> "$TESTNET_DIR/README.md" <<EOF
### Validator $i

- **Name:** validator-$i
- **P2P Port:** $P2P_PORT
- **RPC Port:** $RPC_PORT
- **WebSocket Port:** $WS_PORT
- **Prometheus Port:** $PROM_PORT
- **Start:** \`validator-$i/start.sh\`
- **Logs:** \`validator-$i/node.log\`

EOF
    done

    cat >> "$TESTNET_DIR/README.md" <<EOF

## Testing Scenarios

### 1. Consensus Testing

\`\`\`bash
# Start all validators
./start-all.sh

# Wait 30 seconds for startup
sleep 30

# Check block production
./status.sh

# Should see all nodes at same block height
\`\`\`

### 2. Network Partition Test

\`\`\`bash
# Stop 1 validator (minority)
kill \$(cat validator-0/node.pid)

# Check if remaining 3 validators continue producing blocks
./status.sh

# Restart stopped validator
./validator-0/start.sh &

# Check if it syncs back
./status.sh
\`\`\`

### 3. Performance Test

\`\`\`bash
# Start all validators
./start-all.sh

# Run stress test against one node
cd ../..
TARGET_TPS=1000 RPC_ENDPOINT=http://localhost:9944 ./scripts/testnet/stress_test_harness.sh
\`\`\`

## Monitoring

Access Prometheus metrics for each node:

- Validator 0: http://localhost:9615/metrics
- Validator 1: http://localhost:9616/metrics
- Validator 2: http://localhost:9617/metrics
- Validator 3: http://localhost:9618/metrics

## Troubleshooting

### Nodes not starting

\`\`\`bash
# Check logs
tail -f validator-0/node.log

# Check if ports are in use
lsof -i :9944
\`\`\`

### Nodes not syncing

\`\`\`bash
# Check peer connections
curl -s http://localhost:9944 -H "Content-Type: application/json" \\
  -d '{"jsonrpc":"2.0","method":"system_peers","params":[],"id":1}'
\`\`\`

### Reset testnet

\`\`\`bash
# Stop all nodes
./stop-all.sh

# Clear data
rm -rf validator-*/data

# Restart
./start-all.sh
\`\`\`

---

**Testnet Directory:** $TESTNET_DIR
**Binary:** $BINARY
EOF

    log_success "README created: $TESTNET_DIR/README.md"
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    check_prerequisites
    echo ""

    generate_node_keys
    echo ""

    generate_chain_spec
    echo ""

    create_startup_scripts
    echo ""

    create_management_scripts
    echo ""

    create_readme

    echo ""
    echo -e "${GREEN}"
    echo "═══════════════════════════════════════════════════════════════"
    echo "        MULTI-NODE TESTNET DEPLOYMENT COMPLETE                 "
    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo ""
    echo "Testnet Location: $TESTNET_DIR"
    echo "Validators: $NUM_VALIDATORS"
    echo ""
    echo "Next Steps:"
    echo "  1. cd $TESTNET_DIR"
    echo "  2. ./start-all.sh"
    echo "  3. ./status.sh"
    echo "  4. Read README.md for testing scenarios"
    echo ""
    echo "Quick Commands:"
    echo "  Start:  cd $TESTNET_DIR && ./start-all.sh"
    echo "  Status: cd $TESTNET_DIR && ./status.sh"
    echo "  Stop:   cd $TESTNET_DIR && ./stop-all.sh"
    echo ""
}

main "$@"
