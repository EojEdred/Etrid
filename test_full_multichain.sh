#!/bin/bash
# Full Multichain Integration Test
# Tests FlareChain + all 12 PBCs simultaneously

set -e

echo "==========================================="
echo "Ëtrid Full Multichain Integration Test"
echo "==========================================="
echo ""
echo "This test will start:"
echo "  - 1 FlareChain validator (Alice)"
echo "  - 13 PBC collators (BTC, ETH, DOGE, SOL, XLM, XRP, BNB, TRX, ADA, LINK, MATIC, SC-USDT, EDSC)"
echo ""
echo "Prerequisites:"
echo "  - All binaries built in target/release/"
echo "  - Port 9944-9956 available"
echo "  - At least 8GB RAM available"
echo ""

# Check prerequisites
echo "Step 1: Checking prerequisites..."
if [ ! -f "./target/release/flarechain-node" ]; then
    echo "❌ FlareChain node binary not found"
    echo "Run: cargo build --release -p flarechain-node"
    exit 1
fi

MISSING_PBCS=()
for pbc in btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt edsc; do
    if [ ! -f "./target/release/${pbc}-pbc-collator" ]; then
        MISSING_PBCS+=("$pbc")
    fi
done

if [ ${#MISSING_PBCS[@]} -gt 0 ]; then
    echo "❌ Missing PBC collators: ${MISSING_PBCS[@]}"
    echo "Run: ./build_all_remaining_pbcs.sh"
    exit 1
fi

echo "✅ All binaries present"
echo ""

# Create test directories
TEST_DIR=".multichain-test"
mkdir -p "$TEST_DIR"/{logs,data}

# Cleanup function
cleanup() {
    echo ""
    echo "Cleaning up processes..."
    pkill -f flarechain-node 2>/dev/null || true
    pkill -f pbc-collator 2>/dev/null || true
    echo "✅ Cleanup complete"
}

trap cleanup EXIT INT TERM

# Step 2: Start FlareChain
echo "Step 2: Starting FlareChain validator (Alice)..."
./target/release/flarechain-node \
    --alice \
    --validator \
    --base-path "$TEST_DIR/data/flarechain" \
    --rpc-port 9944 \
    --port 30333 \
    --rpc-cors all \
    --rpc-methods unsafe \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    > "$TEST_DIR/logs/flarechain.log" 2>&1 &

FLARECHAIN_PID=$!
echo "  Started FlareChain (PID: $FLARECHAIN_PID)"
echo "  RPC: ws://127.0.0.1:9944"
echo "  Log: $TEST_DIR/logs/flarechain.log"

# Wait for FlareChain to start
echo "  Waiting for FlareChain to initialize..."
sleep 10

# Check if FlareChain is running
if ! ps -p $FLARECHAIN_PID > /dev/null; then
    echo "❌ FlareChain failed to start"
    cat "$TEST_DIR/logs/flarechain.log" | tail -20
    exit 1
fi

echo "✅ FlareChain running"
echo ""

# Step 3: Start all 13 PBC collators
echo "Step 3: Starting all 13 PBC collators..."

PBCS=(btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt edsc)
BASE_RPC_PORT=8000
BASE_P2P_PORT=40000
PIDS=()

for i in "${!PBCS[@]}"; do
    pbc="${PBCS[$i]}"
    rpc_port=$((BASE_RPC_PORT + i))
    p2p_port=$((BASE_P2P_PORT + i))

    echo "  Starting ${pbc}-pbc-collator..."
    echo "    RPC: ws://127.0.0.1:$rpc_port"
    echo "    P2P: $p2p_port"

    ./target/release/${pbc}-pbc-collator \
        --dev \
        --base-path "$TEST_DIR/data/${pbc}-pbc" \
        --rpc-port $rpc_port \
        --port $p2p_port \
        --rpc-cors all \
        --rpc-methods unsafe \
        --relay-chain-rpc ws://127.0.0.1:9944 \
        > "$TEST_DIR/logs/${pbc}-pbc.log" 2>&1 &

    PID=$!
    PIDS+=("$PID")
    echo "    PID: $PID"
    echo "    Log: $TEST_DIR/logs/${pbc}-pbc.log"
done

echo ""
echo "✅ All 13 PBC collators started"
echo ""

# Wait for all chains to initialize
echo "Step 4: Waiting for all chains to initialize (30 seconds)..."
sleep 30

# Step 5: Health checks
echo "Step 5: Running health checks..."
echo ""

# Check FlareChain
echo "  Checking FlareChain..."
if ps -p $FLARECHAIN_PID > /dev/null; then
    # Try to query RPC
    if curl -s -m 5 -H "Content-Type: application/json" \
         -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
         http://127.0.0.1:9944 > /dev/null 2>&1; then
        echo "    ✅ FlareChain: HEALTHY"
    else
        echo "    ⚠️  FlareChain: Running but RPC not responding"
    fi
else
    echo "    ❌ FlareChain: NOT RUNNING"
fi

# Check each PBC
HEALTHY=0
UNHEALTHY=0

for i in "${!PBCS[@]}"; do
    pbc="${PBCS[$i]}"
    pid="${PIDS[$i]}"
    rpc_port=$((BASE_RPC_PORT + i))

    if ps -p $pid > /dev/null; then
        if curl -s -m 5 -H "Content-Type: application/json" \
             -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
             http://127.0.0.1:$rpc_port > /dev/null 2>&1; then
            echo "    ✅ ${pbc}-pbc: HEALTHY"
            ((HEALTHY++))
        else
            echo "    ⚠️  ${pbc}-pbc: Running but RPC not responding"
            ((UNHEALTHY++))
        fi
    else
        echo "    ❌ ${pbc}-pbc: NOT RUNNING (check log: $TEST_DIR/logs/${pbc}-pbc.log)"
        ((UNHEALTHY++))
    fi
done

echo ""
echo "==========================================="
echo "Multichain Test Results"
echo "==========================================="
echo ""
echo "Summary:"
echo "  - FlareChain: Running"
echo "  - Healthy PBCs: $HEALTHY / 13"
echo "  - Unhealthy PBCs: $UNHEALTHY / 13"
echo ""

if [ $HEALTHY -eq 13 ]; then
    echo "✅ SUCCESS: All chains are healthy!"
    echo ""
    echo "Next steps:"
    echo "  1. Test cross-chain transactions"
    echo "  2. Monitor logs for errors"
    echo "  3. Check bridge functionality"
    echo ""
    echo "Logs location: $TEST_DIR/logs/"
    echo "FlareChain RPC: ws://127.0.0.1:9944"
    echo "PBC RPCs: ws://127.0.0.1:8000 - ws://127.0.0.1:8012"
    echo ""
    echo "Press Ctrl+C to stop all chains..."

    # Keep running until interrupted
    wait
else
    echo "⚠️  WARNING: Some chains are not healthy"
    echo ""
    echo "Check logs in: $TEST_DIR/logs/"
    echo ""
    echo "Keeping chains running for 60 seconds for debugging..."
    sleep 60
fi

# Cleanup happens automatically via trap
