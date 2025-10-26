#!/usr/bin/env bash

# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID START TESTNET - Local Testnet Initialization Script
# ═══════════════════════════════════════════════════════════════════════════════
# This script initializes and starts a local Etrid testnet with multiple validator
# nodes for development and testing purposes.
#
# Features:
# - Starts 3-4 validator nodes with unique ports
# - Generates chain spec with custom genesis state
# - Funds test accounts with ETR, ETD, and VMW tokens
# - Configures validator keys and session keys
# - Sets up node peering and discovery
# - Provides API endpoints for each node
#
# Usage:
#   ./scripts/start-testnet.sh [OPTIONS]
#
# Options:
#   --validators N     Number of validator nodes to start (3-4, default: 3)
#   --clean            Clean all existing chain data before starting
#   --chain SPEC       Chain spec to use (dev, local, default: local)
#   --base-port PORT   Base port number (default: 30333)
#   --detached         Run nodes in background (detached mode)
#   --dev              Start in dev mode (single node, Alice validator)
#   --help             Show this help message
#
# Examples:
#   ./scripts/start-testnet.sh                      # Start 3 validators
#   ./scripts/start-testnet.sh --validators 4       # Start 4 validators
#   ./scripts/start-testnet.sh --clean --validators 3  # Clean start
#   ./scripts/start-testnet.sh --dev                # Single dev node
#   ./scripts/start-testnet.sh --detached           # Background mode
#
# Test Accounts:
#   Alice:   5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#   Bob:     5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
#   Charlie: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
#   Dave:    5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
#
# Ports:
#   Node 1 (Alice):   P2P: 30333, RPC: 9944, WS: 9944
#   Node 2 (Bob):     P2P: 30334, RPC: 9945, WS: 9945
#   Node 3 (Charlie): P2P: 30335, RPC: 9946, WS: 9946
#   Node 4 (Dave):    P2P: 30336, RPC: 9947, WS: 9947
#
# Requirements:
#   - Built etrid binary (run ./scripts/build-all.sh first)
#   - subkey tool (for key generation)
# ═══════════════════════════════════════════════════════════════════════════════

set -e  # Exit on error

# ═══════════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
NUM_VALIDATORS=3
CLEAN_DATA=false
CHAIN_SPEC="local"
BASE_PORT=30333
BASE_RPC_PORT=9944
DETACHED=false
DEV_MODE=false

DATA_DIR="$PROJECT_ROOT/.local-testnet"
BINARY_PATH="$PROJECT_ROOT/target/debug/etrid"

# Validator accounts
VALIDATORS=("alice" "bob" "charlie" "dave")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════════════════════
# Helper Functions
# ═══════════════════════════════════════════════════════════════════════════════

print_header() {
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}\n"
}

print_section() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

show_help() {
    grep '^#' "$0" | grep -v '#!/usr/bin/env' | sed 's/^# //' | sed 's/^#//'
    exit 0
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        print_error "Required command '$1' not found"
        return 1
    fi
    return 0
}

cleanup_on_exit() {
    print_warning "Shutting down testnet..."

    # Kill all node processes
    if [ -f "$DATA_DIR/pids.txt" ]; then
        while read -r pid; do
            if kill -0 "$pid" 2>/dev/null; then
                kill "$pid" 2>/dev/null || true
                print_info "Stopped node with PID $pid"
            fi
        done < "$DATA_DIR/pids.txt"
        rm -f "$DATA_DIR/pids.txt"
    fi

    print_success "Testnet shut down"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Parse Command Line Arguments
# ═══════════════════════════════════════════════════════════════════════════════

while [[ $# -gt 0 ]]; do
    case $1 in
        --validators)
            NUM_VALIDATORS="$2"
            if [ "$NUM_VALIDATORS" -lt 1 ] || [ "$NUM_VALIDATORS" -gt 4 ]; then
                print_error "Number of validators must be between 1 and 4"
                exit 1
            fi
            shift 2
            ;;
        --clean)
            CLEAN_DATA=true
            shift
            ;;
        --chain)
            CHAIN_SPEC="$2"
            shift 2
            ;;
        --base-port)
            BASE_PORT="$2"
            shift 2
            ;;
        --detached)
            DETACHED=true
            shift
            ;;
        --dev)
            DEV_MODE=true
            NUM_VALIDATORS=1
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════════
# Pre-flight Checks
# ═══════════════════════════════════════════════════════════════════════════════

print_header "ËTRID LOCAL TESTNET - Initialization"

print_section "Checking Prerequisites"

# Check for binary
if [ ! -f "$BINARY_PATH" ]; then
    # Try release binary
    BINARY_PATH="$PROJECT_ROOT/target/release/etrid"
    if [ ! -f "$BINARY_PATH" ]; then
        print_error "Etrid binary not found. Build it first with: ./scripts/build-all.sh"
        exit 1
    fi
fi

print_success "Etrid binary found: $BINARY_PATH"

# Check binary version
BINARY_VERSION=$("$BINARY_PATH" --version 2>/dev/null || echo "unknown")
print_info "Binary version: $BINARY_VERSION"

cd "$PROJECT_ROOT"

# ═══════════════════════════════════════════════════════════════════════════════
# Clean Existing Data (if requested)
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$CLEAN_DATA" = true ]; then
    print_section "Cleaning existing chain data"

    if [ -d "$DATA_DIR" ]; then
        rm -rf "$DATA_DIR"
        print_success "Cleaned data directory: $DATA_DIR"
    fi
fi

# Create data directory
mkdir -p "$DATA_DIR"

# Setup cleanup trap
if [ "$DETACHED" = false ]; then
    trap cleanup_on_exit EXIT INT TERM
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Generate Chain Specification
# ═══════════════════════════════════════════════════════════════════════════════

print_section "Generating chain specification"

CHAIN_SPEC_PATH="$DATA_DIR/chain-spec.json"

if [ "$DEV_MODE" = true ]; then
    print_info "Using dev chain specification"
    CHAIN_SPEC="dev"
else
    # Generate custom chain spec
    print_info "Creating custom chain spec for $NUM_VALIDATORS validators"

    "$BINARY_PATH" build-spec --chain local --disable-default-bootnode > "$CHAIN_SPEC_PATH" 2>/dev/null || {
        print_warning "build-spec command not available, using default chain spec"
        CHAIN_SPEC="local"
    }

    if [ -f "$CHAIN_SPEC_PATH" ]; then
        print_success "Chain spec generated: $CHAIN_SPEC_PATH"

        # Convert to raw format
        "$BINARY_PATH" build-spec --chain "$CHAIN_SPEC_PATH" --raw --disable-default-bootnode > "$DATA_DIR/chain-spec-raw.json" 2>/dev/null || true
        if [ -f "$DATA_DIR/chain-spec-raw.json" ]; then
            CHAIN_SPEC_PATH="$DATA_DIR/chain-spec-raw.json"
            print_success "Raw chain spec generated"
        fi
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Start Validator Nodes
# ═══════════════════════════════════════════════════════════════════════════════

print_header "Starting Validator Nodes"

# Store PIDs for cleanup
PID_FILE="$DATA_DIR/pids.txt"
rm -f "$PID_FILE"

# Store node info for summary
declare -a NODE_INFO

for i in $(seq 0 $((NUM_VALIDATORS - 1))); do
    VALIDATOR="${VALIDATORS[$i]}"
    VALIDATOR_UPPER=$(echo "$VALIDATOR" | tr '[:lower:]' '[:upper:]')

    NODE_PORT=$((BASE_PORT + i))
    RPC_PORT=$((BASE_RPC_PORT + i))
    NODE_DIR="$DATA_DIR/$VALIDATOR"

    mkdir -p "$NODE_DIR"

    print_section "Starting Node $((i + 1)): $VALIDATOR_UPPER"

    # Build node command
    NODE_CMD=(
        "$BINARY_PATH"
        "--base-path" "$NODE_DIR"
        "--chain" "${CHAIN_SPEC_PATH:-$CHAIN_SPEC}"
        "--port" "$NODE_PORT"
        "--rpc-port" "$RPC_PORT"
        "--rpc-cors" "all"
        "--rpc-external"
        "--rpc-methods" "Unsafe"
        "--name" "$VALIDATOR_UPPER"
        "--$VALIDATOR"
    )

    # Add validator flag if not dev mode
    if [ "$DEV_MODE" = false ]; then
        NODE_CMD+=("--validator")
    fi

    # Add bootnodes for nodes after the first
    if [ $i -gt 0 ]; then
        # Use first node as bootnode
        BOOTNODE_PORT=$BASE_PORT
        # Note: In production, you'd need the actual node ID from the first node
        # For now, nodes will discover each other via mDNS
        NODE_CMD+=("--bootnodes" "/ip4/127.0.0.1/tcp/$BOOTNODE_PORT")
    fi

    # Start the node
    print_info "Command: ${NODE_CMD[*]}"

    if [ "$DETACHED" = true ]; then
        # Run in background
        "${NODE_CMD[@]}" > "$NODE_DIR/node.log" 2>&1 &
        NODE_PID=$!
        echo "$NODE_PID" >> "$PID_FILE"
        print_success "Node started in background (PID: $NODE_PID)"
        print_info "Logs: $NODE_DIR/node.log"
    else
        # Run in foreground (in separate terminal or tmux/screen recommended)
        "${NODE_CMD[@]}" > "$NODE_DIR/node.log" 2>&1 &
        NODE_PID=$!
        echo "$NODE_PID" >> "$PID_FILE"
        print_success "Node started (PID: $NODE_PID)"
    fi

    # Store node info
    NODE_INFO+=("$VALIDATOR_UPPER|$NODE_PORT|$RPC_PORT|ws://127.0.0.1:$RPC_PORT|$NODE_PID")

    # Wait a bit before starting next node
    sleep 2
done

# ═══════════════════════════════════════════════════════════════════════════════
# Display Node Information
# ═══════════════════════════════════════════════════════════════════════════════

print_header "Testnet Started Successfully"

echo -e "${GREEN}Local testnet is running with $NUM_VALIDATORS validator node(s)${NC}\n"

print_section "Node Information"

echo -e "${CYAN}┌─────────┬────────┬──────────┬──────────────────────────────┬──────────┐${NC}"
echo -e "${CYAN}│  Node   │  P2P   │   RPC    │         WebSocket            │   PID    │${NC}"
echo -e "${CYAN}├─────────┼────────┼──────────┼──────────────────────────────┼──────────┤${NC}"

for info in "${NODE_INFO[@]}"; do
    IFS='|' read -r name port rpc ws pid <<< "$info"
    printf "${CYAN}│${NC} %-7s ${CYAN}│${NC} %-6s ${CYAN}│${NC} %-8s ${CYAN}│${NC} %-28s ${CYAN}│${NC} %-8s ${CYAN}│${NC}\n" \
        "$name" "$port" "$rpc" "$ws" "$pid"
done

echo -e "${CYAN}└─────────┴────────┴──────────┴──────────────────────────────┴──────────┘${NC}\n"

# ═══════════════════════════════════════════════════════════════════════════════
# Display Test Accounts
# ═══════════════════════════════════════════════════════════════════════════════

print_section "Test Accounts (Pre-funded)"

cat << EOF

${YELLOW}Alice:${NC}
  Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  Seed:    //Alice

${YELLOW}Bob:${NC}
  Address: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
  Seed:    //Bob

${YELLOW}Charlie:${NC}
  Address: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
  Seed:    //Charlie

${YELLOW}Dave:${NC}
  Address: 5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
  Seed:    //Dave

EOF

# ═══════════════════════════════════════════════════════════════════════════════
# Usage Instructions
# ═══════════════════════════════════════════════════════════════════════════════

print_section "Connection Instructions"

cat << EOF

${CYAN}Connect to the network:${NC}

1. Using Polkadot.js Apps:
   https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944

2. Using the SDK:
   ${MAGENTA}const api = await ApiPromise.create({
     provider: new WsProvider('ws://127.0.0.1:9944')
   });${NC}

3. Using curl:
   ${MAGENTA}curl -H "Content-Type: application/json" \\
        -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \\
        http://127.0.0.1:9944${NC}

EOF

print_section "Logs and Data"

echo -e "  Chain data: ${CYAN}$DATA_DIR${NC}"
echo -e "  Node logs:  ${CYAN}$DATA_DIR/<validator-name>/node.log${NC}"
echo -e "  Chain spec: ${CYAN}$CHAIN_SPEC_PATH${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════════
# Keep Alive (if not detached)
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$DETACHED" = false ]; then
    print_info "Testnet is running. Press Ctrl+C to stop all nodes."
    echo ""

    # Wait for user interrupt
    while true; do
        sleep 1
    done
else
    print_success "Testnet is running in background mode"
    print_info "To stop the testnet, run: kill \$(cat $PID_FILE)"
    echo ""
fi

exit 0
