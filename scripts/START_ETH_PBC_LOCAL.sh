#!/bin/bash

# Script to start ETH PBC collator with Frontier EVM RPC enabled

echo "🚀 Starting ETH PBC Collator with EVM RPC..."
echo ""

cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/eth-pbc-collator

# Check if binary exists
if [ ! -f "./target/release/eth-pbc-collator" ]; then
    echo "❌ Error: eth-pbc-collator binary not found!"
    echo "Building it now..."
    cargo build --release
fi

echo "Starting ETH PBC with following configuration:"
echo "  Substrate RPC: ws://localhost:9944"
echo "  Ethereum RPC: http://localhost:8545"
echo "  Chain: dev"
echo "  Storage: temporary"
echo ""

./target/release/eth-pbc-collator \
  --dev \
  --tmp \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-external \
  --rpc-methods Unsafe \
  --eth-rpc-url http://localhost:8545 \
  --ethapi debug,eth,net,txpool,web3 \
  --base-path /tmp/eth-pbc-dev

echo ""
echo "✅ ETH PBC node started!"
echo ""
echo "You can now deploy with:"
echo "  cd /Users/macbook/Desktop/etrid/05-multichain/unified-contracts"
echo "  npm run deploy:eth-pbc"
