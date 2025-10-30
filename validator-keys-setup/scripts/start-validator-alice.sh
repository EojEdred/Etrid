#!/bin/bash
set -e

BASEDIR="/tmp/testchain-alice"

echo "=== Ëtrid FlareChain Validator #1 (Alice) Startup ==="
echo ""

echo "Step 1/4: Generating network key..."
cd /opt/etrid
./flarechain-node key generate-node-key --base-path="$BASEDIR"
echo "✓ Network key generated"
echo ""

echo "Step 2/4: Extracting peer ID..."
PEER_ID=$(./flarechain-node key inspect-node-key --file="$BASEDIR/network/secret_ed25519")
echo "✓ Peer ID: $PEER_ID"
echo ""
echo "IMPORTANT: Node #2 should connect using:"
echo "  --bootnodes /ip4/20.186.91.207/tcp/30333/p2p/$PEER_ID"
echo ""
read -p "Press Enter to continue..."

echo "Step 3/4: Inserting ASF validator key..."
./flarechain-node key insert \
  --base-path="$BASEDIR" \
  --chain=local \
  --key-type=asfk \
  --scheme=sr25519 \
  --suri="//Alice"
echo "✓ ASF key inserted"
echo ""

echo "Step 4/4: Starting validator as bootnode..."
echo "Node will now start and begin producing blocks..."
echo "This node will act as the bootstrap node for the network."
echo ""

exec ./flarechain-node \
  --chain=local \
  --alice \
  --base-path="$BASEDIR" \
  --rpc-external \
  --ws-external \
  --prometheus-external \
  --rpc-cors=all \
  --port=30333 \
  --public-addr=/ip4/20.186.91.207/tcp/30333
