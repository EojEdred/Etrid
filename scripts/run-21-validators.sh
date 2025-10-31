#!/bin/bash
# Script to run 21 validators locally for testing GRANDPA finality
# Each validator runs on different ports with minimal disk usage

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CHAIN_SPEC="$PROJECT_ROOT/05-multichain/flare-chain/chainspec-21validator-raw.json"
BINARY="$PROJECT_ROOT/target/release/etrid"
BASE_DIR="/tmp/etrid-21-validators"

# Validator names (must match the preset)
VALIDATORS=(
    "Alice" "Bob" "Charlie" "Dave" "Eve" "Ferdie"
    "Alice//stash" "Bob//stash" "Charlie//stash" "Dave//stash" "Eve//stash" "Ferdie//stash"
    "Validator1" "Validator2" "Validator3" "Validator4" "Validator5"
    "Validator6" "Validator7" "Validator8" "Validator9"
)

# Port configuration: P2P, RPC, WS
BASE_P2P_PORT=30333
BASE_RPC_PORT=9944
BASE_WS_PORT=9933

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 {start|stop|status|logs|clean|insert-keys}"
    echo ""
    echo "Commands:"
    echo "  start        - Start all 21 validators"
    echo "  stop         - Stop all validators"
    echo "  status       - Show status of all validators"
    echo "  logs [NUM]   - Show logs for validator NUM (0-20)"
    echo "  clean        - Remove all validator data"
    echo "  insert-keys  - Insert ASF and GRANDPA keys for all validators"
    exit 1
}

insert_keys_for_validator() {
    local index=$1
    local name=$2
    local base_path="$BASE_DIR/validator-$index"

    echo -e "${YELLOW}Inserting keys for $name (validator-$index)...${NC}"

    # Insert ASF key (Sr25519)
    $BINARY key insert \
        --base-path "$base_path" \
        --chain "$CHAIN_SPEC" \
        --key-type asfk \
        --scheme sr25519 \
        --suri "//$name" 2>/dev/null || true

    # Insert GRANDPA key (Ed25519)
    $BINARY key insert \
        --base-path "$base_path" \
        --chain "$CHAIN_SPEC" \
        --key-type gran \
        --scheme ed25519 \
        --suri "//$name" 2>/dev/null || true

    echo -e "${GREEN}✓ Keys inserted for $name${NC}"
}

insert_all_keys() {
    echo -e "${YELLOW}Inserting ASF and GRANDPA keys for all 21 validators...${NC}"

    for i in "${!VALIDATORS[@]}"; do
        insert_keys_for_validator $i "${VALIDATORS[$i]}"
    done

    echo -e "${GREEN}✅ All keys inserted successfully${NC}"
}

start_validator() {
    local index=$1
    local name=$2
    local p2p_port=$((BASE_P2P_PORT + index))
    local rpc_port=$((BASE_RPC_PORT + index))
    local ws_port=$((BASE_WS_PORT + index))
    local base_path="$BASE_DIR/validator-$index"
    local log_file="$BASE_DIR/logs/validator-$index.log"

    # Generate sequential node key (01, 02, 03, ... 21)
    local node_key=$(printf "%064d" $((index + 1)))

    mkdir -p "$BASE_DIR/logs"
    mkdir -p "$base_path"

    # Build bootnode list (use first 3 validators as bootnodes)
    local bootnodes=""
    if [ $index -gt 2 ]; then
        for bootnode_idx in 0 1 2; do
            local boot_port=$((BASE_P2P_PORT + bootnode_idx))
            # Note: We'll need to get actual peer IDs after first run
            # For now, leave empty and nodes will discover via mDNS
            :
        done
    fi

    echo -e "${YELLOW}Starting validator $index: $name (P2P: $p2p_port, RPC: $rpc_port)${NC}"

    nohup $BINARY \
        --chain "$CHAIN_SPEC" \
        --validator \
        --name "$name" \
        --base-path "$base_path" \
        --port $p2p_port \
        --rpc-port $rpc_port \
        --node-key "$node_key" \
        --rpc-cors all \
        --unsafe-rpc-external \
        --rpc-methods=unsafe \
        --log info,grandpa=debug,ppfa=debug,asf=debug \
        > "$log_file" 2>&1 &

    echo $! > "$BASE_DIR/pids/validator-$index.pid"
    echo -e "${GREEN}✓ Started validator-$index: $name (PID: $!)${NC}"
}

start_all() {
    echo -e "${YELLOW}Starting all 21 validators...${NC}"

    if [ ! -f "$CHAIN_SPEC" ]; then
        echo -e "${RED}Error: Chain spec not found at $CHAIN_SPEC${NC}"
        echo "Please run: ./scripts/build-21-validator-chainspec.sh first"
        exit 1
    fi

    if [ ! -f "$BINARY" ]; then
        echo -e "${RED}Error: Binary not found at $BINARY${NC}"
        echo "Please build the project first: cargo build --release"
        exit 1
    fi

    mkdir -p "$BASE_DIR/pids"

    for i in "${!VALIDATORS[@]}"; do
        start_validator $i "${VALIDATORS[$i]}"
        sleep 0.5  # Small delay between starts
    done

    echo -e "${GREEN}✅ All 21 validators started${NC}"
    echo ""
    echo "View logs: $0 logs [0-20]"
    echo "Check status: $0 status"
}

stop_all() {
    echo -e "${YELLOW}Stopping all validators...${NC}"

    if [ -d "$BASE_DIR/pids" ]; then
        for pid_file in "$BASE_DIR/pids"/*.pid; do
            if [ -f "$pid_file" ]; then
                local pid=$(cat "$pid_file")
                if kill -0 $pid 2>/dev/null; then
                    kill $pid
                    echo -e "${GREEN}✓ Stopped validator (PID: $pid)${NC}"
                fi
                rm "$pid_file"
            fi
        done
    fi

    # Fallback: kill any remaining etrid processes
    pkill -f "etrid.*--validator" || true

    echo -e "${GREEN}✅ All validators stopped${NC}"
}

show_status() {
    echo -e "${YELLOW}Validator Status:${NC}"
    echo ""
    printf "%-4s %-20s %-8s %-10s %-10s\n" "ID" "NAME" "STATUS" "P2P PORT" "RPC PORT"
    echo "────────────────────────────────────────────────────────────"

    for i in "${!VALIDATORS[@]}"; do
        local name="${VALIDATORS[$i]}"
        local pid_file="$BASE_DIR/pids/validator-$i.pid"
        local p2p_port=$((BASE_P2P_PORT + i))
        local rpc_port=$((BASE_RPC_PORT + i))
        local status="${RED}STOPPED${NC}"

        if [ -f "$pid_file" ]; then
            local pid=$(cat "$pid_file")
            if kill -0 $pid 2>/dev/null; then
                status="${GREEN}RUNNING${NC}"
            fi
        fi

        printf "%-4s %-20s " "$i" "$name"
        echo -e "$status $(printf '%-10s %-10s' "$p2p_port" "$rpc_port")"
    done
}

show_logs() {
    local validator_num=$1

    if [ -z "$validator_num" ]; then
        echo -e "${RED}Error: Please specify validator number (0-20)${NC}"
        echo "Usage: $0 logs [0-20]"
        exit 1
    fi

    if [ $validator_num -lt 0 ] || [ $validator_num -gt 20 ]; then
        echo -e "${RED}Error: Validator number must be between 0 and 20${NC}"
        exit 1
    fi

    local log_file="$BASE_DIR/logs/validator-$validator_num.log"

    if [ ! -f "$log_file" ]; then
        echo -e "${RED}Error: Log file not found: $log_file${NC}"
        exit 1
    fi

    echo -e "${YELLOW}Showing logs for validator-$validator_num: ${VALIDATORS[$validator_num]}${NC}"
    echo -e "${YELLOW}Press Ctrl+C to exit${NC}"
    echo ""
    tail -f "$log_file"
}

clean_all() {
    echo -e "${YELLOW}Cleaning all validator data...${NC}"

    stop_all

    if [ -d "$BASE_DIR" ]; then
        rm -rf "$BASE_DIR"
        echo -e "${GREEN}✓ Removed $BASE_DIR${NC}"
    fi

    echo -e "${GREEN}✅ All validator data cleaned${NC}"
}

# Main command dispatcher
case "${1:-}" in
    start)
        start_all
        ;;
    stop)
        stop_all
        ;;
    status)
        show_status
        ;;
    logs)
        show_logs "$2"
        ;;
    clean)
        clean_all
        ;;
    insert-keys)
        insert_all_keys
        ;;
    *)
        usage
        ;;
esac
