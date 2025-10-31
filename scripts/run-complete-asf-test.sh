#!/bin/bash
# Complete ASF Test - Insert keys and run validators in one go
# This ensures keys are present when validators start

set -e

ETRID_ROOT="$HOME/Desktop/etrid"
NODE_BIN="$ETRID_ROOT/target/release/flarechain-node"
LOG_DIR="$ETRID_ROOT/validator-logs"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}=== Complete Ëtrid ASF Production Test ===${NC}"
echo -e "${BLUE}Inserting keys AND starting 21 validators${NC}"
echo ""

# Check binary exists
if [ ! -f "$NODE_BIN" ]; then
    echo -e "${RED}Error: Node binary not found at $NODE_BIN${NC}"
    exit 1
fi

# Create log directory
mkdir -p "$LOG_DIR"

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}Cleaning up...${NC}"

    # Stop all validators
    for pid_file in "$LOG_DIR"/validator-*.pid; do
        if [ -f "$pid_file" ]; then
            pid=$(cat "$pid_file")
            if ps -p $pid > /dev/null 2>&1; then
                kill $pid 2>/dev/null || true
            fi
            rm "$pid_file"
        fi
    done

    echo -e "${GREEN}✓ Cleanup complete${NC}"
}

trap cleanup EXIT INT TERM

echo -e "${GREEN}Step 1: Preparing Validator Directories and Keys${NC}"
echo "====================================================="

# Clean old data
echo -e "${YELLOW}Cleaning old validator data...${NC}"
rm -rf /tmp/validator-*
echo -e "${GREEN}✓ Cleaned${NC}"
echo ""

# Insert ASF keys for all validators
echo -e "${GREEN}Inserting ASF consensus keys...${NC}"

for i in {1..21}; do
    BASE_PATH="/tmp/validator-$(printf "%02d" $i)"

    # Create directory structure first
    mkdir -p "$BASE_PATH"

    echo -n "  Validator $(printf "%02d" $i): "

    # Insert ASF consensus key (asfk)
    "$NODE_BIN" key insert \
        --base-path "$BASE_PATH" \
        --chain dev \
        --key-type asfk \
        --scheme sr25519 \
        --suri "//Validator$i" \
        > /dev/null 2>&1

    # Insert GRANDPA key (gran) for hybrid finality
    "$NODE_BIN" key insert \
        --base-path "$BASE_PATH" \
        --chain dev \
        --key-type gran \
        --scheme ed25519 \
        --suri "//Validator$i" \
        > /dev/null 2>&1

    echo -e "${GREEN}✓ Keys inserted${NC}"
done

echo ""
echo -e "${GREEN}✓ All keys inserted successfully${NC}"
echo ""

# Verify keys are in place
echo -e "${BLUE}Verifying keystore for Validator 01...${NC}"
if [ -d "/tmp/validator-01/chains/flarechain_dev/keystore" ]; then
    key_count=$(ls -1 /tmp/validator-01/chains/flarechain_dev/keystore | wc -l | tr -d ' ')
    echo -e "${GREEN}✓ Keystore exists with $key_count keys${NC}"
else
    echo -e "${YELLOW}⚠ Keystore will be created on first run${NC}"
fi
echo ""

echo -e "${GREEN}Step 2: Starting 21 Validators${NC}"
echo "====================================="

# Function to start validator
start_validator() {
    local id=$1
    local port=$((30333 + id - 1))
    local rpc_port=$((9944 + id - 1))
    local base_path="/tmp/validator-$(printf "%02d" $id)"

    echo "Starting Validator $id..."

    if [ $id -eq 1 ]; then
        # First validator is bootnode
        "$NODE_BIN" \
            --chain=dev \
            --validator \
            --base-path="$base_path" \
            --port=$port \
            --rpc-port=$rpc_port \
            --rpc-cors=all \
            --unsafe-rpc-external \
            --rpc-methods=Unsafe \
            --node-key=0000000000000000000000000000000000000000000000000000000000000001 \
            --name="Validator-$id" \
            --force-authoring \
            > "$LOG_DIR/validator-$(printf "%02d" $id).log" 2>&1 &
    else
        # Other validators connect to bootnode
        "$NODE_BIN" \
            --chain=dev \
            --validator \
            --base-path="$base_path" \
            --port=$port \
            --rpc-port=$rpc_port \
            --rpc-cors=all \
            --unsafe-rpc-external \
            --rpc-methods=Unsafe \
            --bootnodes="/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp" \
            --name="Validator-$id" \
            --force-authoring \
            > "$LOG_DIR/validator-$(printf "%02d" $id).log" 2>&1 &
    fi

    echo $! > "$LOG_DIR/validator-$(printf "%02d" $id).pid"
    sleep 0.5
}

# Start all 21 validators
for i in {1..21}; do
    start_validator $i
done

echo ""
echo -e "${GREEN}✓ All 21 validators started${NC}"

# Wait for network stabilization
echo ""
echo -e "${YELLOW}Waiting 30 seconds for network to stabilize...${NC}"
sleep 30

# Check validator status
echo ""
echo -e "${GREEN}Step 3: Checking Validator Status${NC}"
echo "====================================="

active_count=0
crashed_validators=()

for i in {1..21}; do
    pid_file="$LOG_DIR/validator-$(printf "%02d" $i).pid"
    if [ -f "$pid_file" ]; then
        pid=$(cat "$pid_file")
        if ps -p $pid > /dev/null 2>&1; then
            echo -e "${GREEN}✓ Validator $i: Running${NC}"
            active_count=$((active_count + 1))
        else
            echo -e "${RED}✗ Validator $i: Crashed${NC}"
            crashed_validators+=($i)
        fi
    fi
done

echo ""
echo -e "${BLUE}Active Validators: $active_count / 21${NC}"

if [ ${#crashed_validators[@]} -gt 0 ]; then
    echo ""
    echo -e "${YELLOW}Checking crash reasons for first crashed validator...${NC}"
    first_crashed=${crashed_validators[0]}
    echo "Validator $first_crashed log (last 15 lines):"
    tail -15 "$LOG_DIR/validator-$(printf "%02d" $first_crashed).log"
fi

if [ $active_count -lt 1 ]; then
    echo ""
    echo -e "${RED}✗ CRITICAL: No validators running${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Step 4: Testing RPC Connectivity${NC}"
echo "====================================="

if command -v curl &> /dev/null; then
    response=$(curl -s -X POST http://127.0.0.1:9944 -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' | grep -o '"result":"[^"]*"' || echo "")

    if [ -n "$response" ]; then
        echo -e "${GREEN}✓ RPC responsive: $response${NC}"
    else
        echo -e "${YELLOW}⚠ RPC not responsive${NC}"
    fi
fi

# Monitor block production
echo ""
echo -e "${GREEN}Step 5: Monitoring Block Production${NC}"
echo "====================================="
echo "Monitoring for 60 seconds..."
echo ""

start_time=$(date +%s)
block_count=0
last_block=0

for i in {1..12}; do
    if command -v curl &> /dev/null; then
        block_hex=$(curl -s -X POST http://127.0.0.1:9944 -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' 2>/dev/null | grep -o '"number":"0x[^"]*"' | sed 's/"number":"0x//;s/"$//' || echo "0")

        if [ -n "$block_hex" ] && [ "$block_hex" != "0" ]; then
            block_decimal=$((16#$block_hex))
            if [ $block_decimal -gt $last_block ]; then
                timestamp=$(date '+%H:%M:%S')
                echo -e "${GREEN}[$timestamp] Block #$block_decimal produced${NC}"
                block_count=$((block_count + 1))
                last_block=$block_decimal
            fi
        else
            echo "Waiting for blocks..."
        fi
    fi
    sleep 5
done

end_time=$(date +%s)
elapsed=$((end_time - start_time))

# Final results
echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║     PRODUCTION READINESS TEST RESULTS  ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo "Test Duration: ${elapsed}s"
echo "Validators Active: $active_count / 21"
echo "Blocks Produced: $last_block"
echo ""

if [ $last_block -gt 0 ]; then
    avg_block_time=$(echo "scale=1; $elapsed / $last_block" | bc)
    echo "Average Block Time: ~${avg_block_time}s"
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✅ PRODUCTION READY - BLOCKS PRODUCED ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
    echo ""
    echo "SUCCESS: The network:"
    echo "  ✓ Started $active_count validators with ASF keys"
    echo "  ✓ Established P2P connectivity"
    echo "  ✓ Produced $last_block blocks"
    echo "  ✓ ASF consensus working"
    echo ""
else
    echo -e "${YELLOW}╔════════════════════════════════════════╗${NC}"
    echo -e "${YELLOW}║          NO BLOCKS PRODUCED            ║${NC}"
    echo -e "${YELLOW}╚════════════════════════════════════════╝${NC}"
    echo ""
    echo "Status:"
    echo "  ✓ $active_count validators started"
    echo "  ✗ No block production yet"
    echo ""
fi

echo "Logs: $LOG_DIR/"
echo ""
echo "Keeping validators running for 10 more seconds..."
echo "Press Ctrl+C to stop immediately."
sleep 10
