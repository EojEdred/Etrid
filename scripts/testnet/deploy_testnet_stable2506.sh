#!/bin/bash
# Ëtrid Protocol Testnet Deployment (polkadot-stable2506)
# Deploys FlareChain with ASF consensus, PPFA block sealing, and ËDSC bridge
# Version: 1.0.0 (Post-Phase 3 Integration)

set -e

# ============================================================================
# Configuration
# ============================================================================

ETRID_ROOT="${ETRID_ROOT:-/Users/macbook/Desktop/etrid}"
BIN_DIR="$ETRID_ROOT/target/release"
DATA_DIR="${TESTNET_DATA_DIR:-$ETRID_ROOT/.testnet-stable2506}"
LOGS_DIR="$DATA_DIR/logs"
CHAIN_SPEC="$DATA_DIR/chain-spec.json"
CHAIN_SPEC_RAW="$DATA_DIR/chain-spec-raw.json"

# Network configuration
CHAIN_ID="etrid-testnet-stable2506"
PROTOCOL_ID="etr"
BLOCK_TIME=6  # 6 seconds per block
EPOCH_DURATION=600  # 100 blocks per epoch

# Validator configuration
VALIDATORS=("alice" "bob" "charlie" "dave" "eve")
VALIDATOR_COUNT=${#VALIDATORS[@]}
BASE_P2P_PORT=30333
BASE_RPC_PORT=9944
BASE_WS_PORT=9944

# ASF consensus parameters
ASF_TIMEOUT=30000  # 30 seconds
FINALITY_THRESHOLD=67  # 67% for 2/3 supermajority

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# ============================================================================
# Helper Functions
# ============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_section() {
    echo ""
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${PURPLE}  $1${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

cleanup() {
    echo ""
    log_warning "Shutting down testnet..."
    pkill -f flarechain-node || true
    sleep 2
    log_success "Testnet stopped"
    exit 0
}

trap cleanup EXIT INT TERM

# ============================================================================
# Pre-Flight Checks
# ============================================================================

preflight_checks() {
    log_section "Pre-Flight Checks"

    # Check binaries
    if [ ! -f "$BIN_DIR/flarechain-node" ]; then
        log_error "FlareChain node binary not found at $BIN_DIR/flarechain-node"
        log_info "Build with: cd $ETRID_ROOT && cargo build --release -p flarechain-node"
        exit 1
    fi
    log_success "FlareChain binary found"

    # Check binary version
    VERSION=$($BIN_DIR/flarechain-node --version 2>&1 | head -1)
    log_info "Binary version: $VERSION"

    # Create directories
    mkdir -p "$DATA_DIR"
    mkdir -p "$LOGS_DIR"
    for validator in "${VALIDATORS[@]}"; do
        mkdir -p "$DATA_DIR/node-$validator"
    done
    log_success "Directories created"

    # Check available ports
    for i in "${!VALIDATORS[@]}"; do
        P2P_PORT=$((BASE_P2P_PORT + i))
        RPC_PORT=$((BASE_RPC_PORT + i))
        if lsof -Pi :$P2P_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
            log_error "Port $P2P_PORT already in use"
            exit 1
        fi
        if lsof -Pi :$RPC_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
            log_error "Port $RPC_PORT already in use"
            exit 1
        fi
    done
    log_success "All ports available"

    log_success "Pre-flight checks complete"
}

# ============================================================================
# Chain Specification Generation
# ============================================================================

generate_chain_spec() {
    log_section "Generating Chain Specification"

    log_info "Generating chain spec for $CHAIN_ID..."

    # Generate base chain spec
    $BIN_DIR/flarechain-node build-spec \
        --chain local \
        --disable-default-bootnode \
        > "$CHAIN_SPEC" 2>&1

    if [ $? -ne 0 ]; then
        log_error "Failed to generate chain spec"
        exit 1
    fi

    log_success "Base chain spec generated"

    # Customize chain spec with ASF consensus parameters
    log_info "Customizing chain spec with ASF parameters..."

    # Note: This requires jq for JSON manipulation
    if command -v jq &> /dev/null; then
        cat "$CHAIN_SPEC" | jq \
            --arg chain_id "$CHAIN_ID" \
            --arg protocol_id "$PROTOCOL_ID" \
            --argjson block_time "$BLOCK_TIME" \
            --argjson epoch_duration "$EPOCH_DURATION" \
            --argjson asf_timeout "$ASF_TIMEOUT" \
            --argjson finality_threshold "$FINALITY_THRESHOLD" \
            '.name = $chain_id |
             .id = $chain_id |
             .protocolId = $protocol_id |
             .properties.blockTime = $block_time |
             .properties.epochDuration = $epoch_duration |
             .genesis.runtime.asfConsensus.timeout = $asf_timeout |
             .genesis.runtime.asfConsensus.finalityThreshold = $finality_threshold' \
            > "$CHAIN_SPEC.tmp" && mv "$CHAIN_SPEC.tmp" "$CHAIN_SPEC"

        log_success "Chain spec customized"
    else
        log_warning "jq not installed, using default chain spec"
    fi

    # Generate raw chain spec
    log_info "Converting to raw chain spec..."
    $BIN_DIR/flarechain-node build-spec \
        --chain "$CHAIN_SPEC" \
        --raw \
        --disable-default-bootnode \
        > "$CHAIN_SPEC_RAW" 2>&1

    if [ $? -ne 0 ]; then
        log_error "Failed to generate raw chain spec"
        exit 1
    fi

    log_success "Raw chain spec generated: $CHAIN_SPEC_RAW"
}

# ============================================================================
# Validator Key Generation
# ============================================================================

generate_validator_keys() {
    log_section "Generating Validator Keys"

    for validator in "${VALIDATORS[@]}"; do
        log_info "Generating keys for $validator..."

        NODE_DIR="$DATA_DIR/node-$validator"

        # Generate session keys
        # Note: In production, use proper key generation and storage
        log_info "  Session keys will be auto-generated on first start"
    done

    log_success "Validator key generation configured"
}

# ============================================================================
# Start Validator Nodes
# ============================================================================

start_validators() {
    log_section "Starting Validator Nodes"

    # Get bootnode peer ID from Alice
    log_info "Waiting for bootnode (Alice) to start..."

    for i in "${!VALIDATORS[@]}"; do
        validator="${VALIDATORS[$i]}"
        P2P_PORT=$((BASE_P2P_PORT + i))
        RPC_PORT=$((BASE_RPC_PORT + i))
        WS_PORT=$((BASE_WS_PORT + i))
        NODE_DIR="$DATA_DIR/node-$validator"
        LOG_FILE="$LOGS_DIR/$validator.log"

        log_info "Starting validator: $validator"
        log_info "  P2P Port:  $P2P_PORT"
        log_info "  RPC Port:  $RPC_PORT"
        log_info "  WS Port:   $WS_PORT"
        log_info "  Data Dir:  $NODE_DIR"
        log_info "  Log File:  $LOG_FILE"

        # Build command
        CMD="$BIN_DIR/flarechain-node"

        # Common arguments
        ARGS=(
            "--chain" "$CHAIN_SPEC_RAW"
            "--base-path" "$NODE_DIR"
            "--port" "$P2P_PORT"
            "--rpc-port" "$RPC_PORT"
            "--ws-port" "$WS_PORT"
            "--rpc-cors" "all"
            "--rpc-methods" "unsafe"
            "--rpc-external"
            "--ws-external"
            "--name" "Etrid-$validator"
            "--validator"
            "--$validator"
        )

        # Bootnode configuration (all nodes except Alice connect to Alice)
        if [ $i -eq 0 ]; then
            # Alice is the bootnode
            ARGS+=(
                "--node-key" "0000000000000000000000000000000000000000000000000000000000000001"
            )
        else
            # Other nodes connect to Alice
            ALICE_P2P_PORT=$BASE_P2P_PORT
            ARGS+=(
                "--bootnodes" "/ip4/127.0.0.1/tcp/$ALICE_P2P_PORT/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
            )
        fi

        # Start node
        "$CMD" "${ARGS[@]}" > "$LOG_FILE" 2>&1 &
        NODE_PID=$!

        echo $NODE_PID > "$NODE_DIR/node.pid"
        log_success "Started $validator (PID: $NODE_PID)"

        # Wait for node to initialize before starting next
        sleep 3
    done

    log_success "All validators started"
}

# ============================================================================
# Monitor Network Health
# ============================================================================

monitor_network() {
    log_section "Network Status Monitor"

    log_info "Monitoring network health (Ctrl+C to stop)..."
    echo ""

    # Display connection info
    echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║           ËTRID TESTNET - CONNECTION INFO                  ║${NC}"
    echo -e "${CYAN}╠════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${CYAN}║  Chain ID:    ${NC}$CHAIN_ID"
    echo -e "${CYAN}║  Validators:  ${NC}$VALIDATOR_COUNT nodes"
    echo -e "${CYAN}║  Consensus:   ${NC}ASF (Asynchronous Synergistic Finality)"
    echo -e "${CYAN}║  Block Time:  ${NC}${BLOCK_TIME}s"
    echo -e "${CYAN}║  Epoch:       ${NC}${EPOCH_DURATION} blocks"
    echo -e "${CYAN}╠════════════════════════════════════════════════════════════╣${NC}"

    for i in "${!VALIDATORS[@]}"; do
        validator="${VALIDATORS[$i]}"
        RPC_PORT=$((BASE_RPC_PORT + i))
        WS_PORT=$((BASE_WS_PORT + i))
        echo -e "${CYAN}║  ${NC}$validator:"
        echo -e "${CYAN}║    ${NC}RPC: http://127.0.0.1:$RPC_PORT"
        echo -e "${CYAN}║    ${NC}WS:  ws://127.0.0.1:$WS_PORT"
    done

    echo -e "${CYAN}╠════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${CYAN}║  Logs:        ${NC}$LOGS_DIR"
    echo -e "${CYAN}║  Data:        ${NC}$DATA_DIR"
    echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    log_info "Polkadot.js Apps: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:$BASE_RPC_PORT"
    echo ""

    # Monitor logs
    log_info "Tailing logs (showing Alice)..."
    echo ""

    tail -f "$LOGS_DIR/alice.log" | while read line; do
        # Highlight important log lines
        if echo "$line" | grep -q "Imported"; then
            echo -e "${GREEN}$line${NC}"
        elif echo "$line" | grep -q "Finalized"; then
            echo -e "${CYAN}$line${NC}"
        elif echo "$line" | grep -q "ERROR"; then
            echo -e "${RED}$line${NC}"
        elif echo "$line" | grep -q "WARN"; then
            echo -e "${YELLOW}$line${NC}"
        else
            echo "$line"
        fi
    done
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    clear

    echo -e "${PURPLE}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                                                              ║"
    echo "║         ËTRID PROTOCOL TESTNET DEPLOYMENT                   ║"
    echo "║         Polkadot SDK: stable2506                            ║"
    echo "║         Consensus: ASF with PPFA Block Sealing              ║"
    echo "║         Version: 1.0.0 (Post-Phase 3)                       ║"
    echo "║                                                              ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"

    # Execute deployment steps
    preflight_checks
    generate_chain_spec
    generate_validator_keys
    start_validators

    # Wait for network to stabilize
    log_info "Waiting for network to stabilize..."
    sleep 10

    # Monitor network
    monitor_network
}

# Run main
main
