#!/bin/bash
# Bootstrap Script for EojEdred (Validator 2) - Primary Bootstrap Node
# Run this on VM1 (20.12.114.226)

set -e

echo "🚀 ËTRID FlareChain - EojEdred Bootstrap Validator Setup"
echo "========================================================"

# EojEdred Session Keys (from COMPLETE_VALIDATOR_NETWORK_MAP.md)
ACCOUNT_ID="5HYpUK51E1BzhEfiRikhjkNivJiw2WAEG5Uxsrbj5ZE669EM"
AURA_KEY="0xf29e4e1cfc2867fcda12ac9b190bea017868a0d1f3f7d5cc59af6c7d3ce6c45c"
GRANDPA_KEY="0x0a9442f63cd6019b8d6f0cd2dd6cc84d302d8eeb616bb12d7f439172107dbd2b"
ASF_KEY="0xf29e4e1cfc2867fcda12ac9b190bea017868a0d1f3f7d5cc59af6c7d3ce6c45c"
NETWORK_SECRET="56192ada80719b2c7ca6f6e96d41ac008952bf98f1c91a34bb8713ef46fe114d"

# Paths
NODE_DIR="$HOME/flarechain-node"
BINARY="$NODE_DIR/flarechain-node"
CHAINSPEC="$NODE_DIR/chainspec-mainnet-raw.json"
BASE_PATH="$NODE_DIR/data"

echo ""
echo "📋 Configuration:"
echo "   Account: $ACCOUNT_ID"
echo "   Node Dir: $NODE_DIR"
echo "   Base Path: $BASE_PATH"
echo ""

# Step 1: Stop any running node
echo "🛑 Step 1: Stopping any running validator..."
pkill -f flarechain-node || echo "   No running node found"
sleep 3

# Step 2: Purge chain database
echo ""
echo "🗑️  Step 2: Purging chain database..."
read -p "⚠️  This will DELETE all blockchain data. Continue? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    echo "❌ Aborted by user"
    exit 1
fi

if [ -d "$BASE_PATH" ]; then
    echo "   Removing $BASE_PATH..."
    rm -rf "$BASE_PATH"
    echo "   ✅ Database purged"
else
    echo "   No existing database found"
fi

# Step 3: Create fresh base path
echo ""
echo "📁 Step 3: Creating fresh data directory..."
mkdir -p "$BASE_PATH"

# Step 4: Insert session keys
echo ""
echo "🔑 Step 4: Inserting EojEdred session keys..."

# Start node temporarily to insert keys
echo "   Starting temporary node for key insertion..."
"$BINARY" \
  --base-path "$BASE_PATH" \
  --chain "$CHAINSPEC" \
  --rpc-port 9944 \
  --rpc-cors all \
  --unsafe-rpc-external \
  > /tmp/flarechain-temp.log 2>&1 &

TEMP_PID=$!
echo "   Node PID: $TEMP_PID"

# Wait for RPC to be ready
echo "   Waiting for RPC to be ready..."
for i in {1..30}; do
    if curl -s -X POST -H "Content-Type: application/json" \
       --data '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
       http://localhost:9944 > /dev/null 2>&1; then
        echo "   ✅ RPC ready"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "   ❌ RPC failed to start. Check /tmp/flarechain-temp.log"
        kill $TEMP_PID
        exit 1
    fi
    sleep 1
done

# Insert AURA key
echo "   Inserting AURA key..."
curl -s -X POST -H "Content-Type: application/json" \
  --data "{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"aura\",\"$AURA_KEY\",\"$AURA_KEY\"],\"id\":1}" \
  http://localhost:9944 > /dev/null

# Insert GRANDPA key
echo "   Inserting GRANDPA key..."
curl -s -X POST -H "Content-Type: application/json" \
  --data "{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"gran\",\"$GRANDPA_KEY\",\"$GRANDPA_KEY\"],\"id\":1}" \
  http://localhost:9944 > /dev/null

# Insert ASF key
echo "   Inserting ASF key..."
curl -s -X POST -H "Content-Type: application/json" \
  --data "{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"ppfa\",\"$ASF_KEY\",\"$ASF_KEY\"],\"id\":1}" \
  http://localhost:9944 > /dev/null

# Verify keys
echo "   Verifying session keys..."
KEYS=$(curl -s -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"author_hasSessionKeys","params":["'"$AURA_KEY$GRANDPA_KEY$ASF_KEY"'"],"id":1}' \
  http://localhost:9944)

if echo "$KEYS" | grep -q "true"; then
    echo "   ✅ All session keys inserted successfully"
else
    echo "   ⚠️  Key verification unclear: $KEYS"
fi

# Stop temporary node
echo "   Stopping temporary node..."
kill $TEMP_PID
sleep 2

# Step 5: Start as bootstrap validator
echo ""
echo "🔥 Step 5: Starting EojEdred as bootstrap validator..."

# Get public IP for bootnode address
PUBLIC_IP=$(curl -s ifconfig.me)
BOOTNODE_ADDR="/ip4/$PUBLIC_IP/tcp/30333/p2p/12D3KooW..."  # Will be generated on first run

cat > "$NODE_DIR/start-eoj-bootstrap.sh" << 'STARTSCRIPT'
#!/bin/bash
# EojEdred Bootstrap Validator Startup Script

NODE_DIR="$HOME/flarechain-node"
BINARY="$NODE_DIR/flarechain-node"
CHAINSPEC="$NODE_DIR/chainspec-mainnet-raw.json"
BASE_PATH="$NODE_DIR/data"
NETWORK_SECRET="56192ada80719b2c7ca6f6e96d41ac008952bf98f1c91a34bb8713ef46fe114d"

# Start validator
"$BINARY" \
  --base-path "$BASE_PATH" \
  --chain "$CHAINSPEC" \
  --name "EojEdred (Founder)" \
  --validator \
  --rpc-port 9944 \
  --rpc-cors all \
  --unsafe-rpc-external \
  --port 30333 \
  --node-key "$NETWORK_SECRET" \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --prometheus-external \
  --prometheus-port 9615
STARTSCRIPT

chmod +x "$NODE_DIR/start-eoj-bootstrap.sh"

echo ""
echo "✅ Bootstrap setup complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Start the validator: $NODE_DIR/start-eoj-bootstrap.sh"
echo "   2. Get bootnode address from logs"
echo "   3. Share bootnode address with other validators"
echo ""
echo "🔍 Monitor logs:"
echo "   tail -f /tmp/flarechain-temp.log"
echo ""
echo "🌐 RPC endpoint:"
echo "   http://$PUBLIC_IP:9944"
echo ""
