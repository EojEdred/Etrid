#!/bin/bash
set -e

BASEDIR="/tmp/testchain-bob"
ALICE_IP="20.186.91.207"
ALICE_PORT="30333"

echo "=== Ëtrid FlareChain Validator #2 (Bob) Startup ==="
echo ""
echo "This validator will connect to Alice as the bootnode"
echo ""

echo "Step 1/4: Requesting Alice's peer ID..."
echo "Please enter Alice's peer ID from VM #1:"
read -p "Peer ID: " ALICE_PEER_ID

if [ -z "$ALICE_PEER_ID" ]; then
  echo "❌ Error: Peer ID cannot be empty"
  exit 1
fi

BOOTNODE_ADDR="/ip4/$ALICE_IP/tcp/$ALICE_PORT/p2p/$ALICE_PEER_ID"
echo "✓ Will connect to bootnode: $BOOTNODE_ADDR"
echo ""

echo "Step 2/4: Generating network key..."
cd /opt/etrid
./flarechain-node key generate-node-key --base-path="$BASEDIR"
echo "✓ Network key generated"
echo ""

echo "Step 3/4: Inserting ASF validator key..."
./flarechain-node key insert \
  --base-path="$BASEDIR" \
  --chain=local \
  --key-type=asfk \
  --scheme=sr25519 \
  --suri="//Bob"
echo "✓ ASF key inserted"
echo ""

echo "Step 4/4: Starting validator with bootnode..."
echo "Node will now start and connect to Alice..."
echo ""

exec ./flarechain-node \
  --chain=local \
  --bob \
  --base-path="$BASEDIR" \
  --rpc-external \
  --ws-external \
  --prometheus-external \
  --rpc-cors=all \
  --port=30333 \
  --bootnodes "$BOOTNODE_ADDR"
