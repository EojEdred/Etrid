#!/bin/bash
# Lightning Bloc Testnet Deployment Script
# Deploys a test network with FlareChain + 3 Lightning nodes

set -e

echo "=========================================="
echo "Lightning Bloc Testnet Deployment"
echo "=========================================="
echo ""

# Configuration
TESTNET_DIR=".lightning-testnet"
FLARE_PORT=9944
NODE_A_PORT=9945
NODE_B_PORT=9946
NODE_C_PORT=9947

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check prerequisites
echo "Step 1: Checking prerequisites..."
if [ ! -f "./target/release/flarechain-node" ]; then
    echo "❌ FlareChain node binary not found"
    echo "Run: cargo build --release -p flarechain-node"
    exit 1
fi

if [ ! -f "./target/release/btc-pbc-collator" ]; then
    echo "⚠️  BTC-PBC collator not found (optional for Lightning)"
    echo "Run: cargo build --release -p btc-pbc-collator"
fi

echo "✅ Prerequisites check complete"
echo ""

# Create testnet directory structure
echo "Step 2: Creating testnet directory structure..."
mkdir -p "$TESTNET_DIR"/{data,logs,channels}
echo "✅ Directory structure created"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo "Cleaning up processes..."
    pkill -f flarechain-node 2>/dev/null || true
    pkill -f lightning-node 2>/dev/null || true
    echo "✅ Cleanup complete"
}

trap cleanup EXIT INT TERM

# Step 3: Start FlareChain (Settlement Layer)
echo "Step 3: Starting FlareChain settlement layer..."
./target/release/flarechain-node \
    --dev \
    --base-path "$TESTNET_DIR/data/flarechain" \
    --rpc-port $FLARE_PORT \
    --port 30333 \
    --rpc-cors all \
    --rpc-methods unsafe \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    > "$TESTNET_DIR/logs/flarechain.log" 2>&1 &

FLARE_PID=$!
echo -e "${GREEN}✓${NC} FlareChain started (PID: $FLARE_PID)"
echo "  RPC: ws://127.0.0.1:$FLARE_PORT"
echo "  Log: $TESTNET_DIR/logs/flarechain.log"
echo ""

# Wait for FlareChain to initialize
echo "Waiting for FlareChain to initialize (10 seconds)..."
sleep 10

# Check if FlareChain is running
if ! ps -p $FLARE_PID > /dev/null; then
    echo "❌ FlareChain failed to start"
    cat "$TESTNET_DIR/logs/flarechain.log" | tail -20
    exit 1
fi

echo -e "${GREEN}✓${NC} FlareChain is running"
echo ""

# Step 4: Initialize Lightning Network Topology
echo "Step 4: Creating Lightning network topology..."
cat > "$TESTNET_DIR/network-topology.json" <<EOF
{
  "network": "Lightning Bloc Testnet",
  "settlement_chain": {
    "name": "FlareChain",
    "endpoint": "ws://127.0.0.1:$FLARE_PORT"
  },
  "nodes": [
    {
      "id": "alice",
      "name": "Alice's Node",
      "endpoint": "ws://127.0.0.1:$NODE_A_PORT",
      "account": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "initial_balance": 100000000000000000000000
    },
    {
      "id": "bob",
      "name": "Bob's Hub",
      "endpoint": "ws://127.0.0.1:$NODE_B_PORT",
      "account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      "initial_balance": 100000000000000000000000
    },
    {
      "id": "charlie",
      "name": "Charlie's Node",
      "endpoint": "ws://127.0.0.1:$NODE_C_PORT",
      "account": "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
      "initial_balance": 100000000000000000000000
    }
  ],
  "channels": [
    {
      "id": "alice-bob",
      "from": "alice",
      "to": "bob",
      "capacity": 10000000000000000000000,
      "balance_from": 5000000000000000000000,
      "balance_to": 5000000000000000000000,
      "status": "pending"
    },
    {
      "id": "bob-charlie",
      "from": "bob",
      "to": "charlie",
      "capacity": 15000000000000000000000,
      "balance_from": 7500000000000000000000,
      "balance_to": 7500000000000000000000,
      "status": "pending"
    }
  ]
}
EOF

echo -e "${GREEN}✓${NC} Network topology created: $TESTNET_DIR/network-topology.json"
echo ""

# Step 5: Create channel configuration
echo "Step 5: Creating channel configurations..."

# Alice <-> Bob channel
cat > "$TESTNET_DIR/channels/alice-bob.json" <<EOF
{
  "channel_id": "alice-bob",
  "party_a": "alice",
  "party_b": "bob",
  "party_a_account": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "party_b_account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
  "initial_balance_a": "5000000000000000000000",
  "initial_balance_b": "5000000000000000000000",
  "duration_blocks": 28800,
  "min_htlc": "1000000000000000000",
  "max_htlc": "5000000000000000000000",
  "base_fee": 1,
  "fee_rate": 100,
  "time_lock_delta": 40
}
EOF

# Bob <-> Charlie channel
cat > "$TESTNET_DIR/channels/bob-charlie.json" <<EOF
{
  "channel_id": "bob-charlie",
  "party_a": "bob",
  "party_b": "charlie",
  "party_a_account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
  "party_b_account": "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
  "initial_balance_a": "7500000000000000000000",
  "initial_balance_b": "7500000000000000000000",
  "duration_blocks": 28800,
  "min_htlc": "1000000000000000000",
  "max_htlc": "7500000000000000000000",
  "base_fee": 2,
  "fee_rate": 50,
  "time_lock_delta": 40
}
EOF

echo -e "${GREEN}✓${NC} Channel configurations created"
echo "  alice-bob: $TESTNET_DIR/channels/alice-bob.json"
echo "  bob-charlie: $TESTNET_DIR/channels/bob-charlie.json"
echo ""

# Step 6: Display network information
echo "=========================================="
echo "Lightning Bloc Testnet Ready"
echo "=========================================="
echo ""
echo "Network Topology:"
echo "  Alice <--10,000 ETR--> Bob <--15,000 ETR--> Charlie"
echo ""
echo "Settlement Layer:"
echo "  FlareChain: ws://127.0.0.1:$FLARE_PORT"
echo ""
echo "Lightning Nodes:"
echo "  Alice:   ws://127.0.0.1:$NODE_A_PORT (pending)"
echo "  Bob:     ws://127.0.0.1:$NODE_B_PORT (pending)"
echo "  Charlie: ws://127.0.0.1:$NODE_C_PORT (pending)"
echo ""
echo "Next Steps:"
echo "  1. Use test_lightning_channels.sh to open channels"
echo "  2. Use test_multi_hop_payment.sh to test routing"
echo "  3. Monitor with watchtower_service.sh"
echo ""
echo "Press Ctrl+C to stop the testnet..."
echo ""

# Keep running
wait $FLARE_PID
