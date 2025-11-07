#!/bin/bash
# Basic bridge functionality test
# Tests FlareChain + BTC PBC collator integration

echo "========================================"
echo "Bridge Functionality Test - FlareChain + BTC PBC"
echo "========================================"
echo ""

# Check binaries exist
if [ ! -f "./target/release/flarechain-node" ]; then
    echo "❌ FlareChain node binary not found"
    exit 1
fi

if [ ! -f "./target/release/btc-pbc-collator" ]; then
    echo "❌ BTC PBC collator binary not found"
    exit 1
fi

echo "✅ Binaries found"
echo ""

# Check chain spec exists
if [ ! -f "./chain-specs/flarechain-shared.json" ]; then
    echo "⚠️  FlareChain chain spec not found, generating..."
    ./target/release/flarechain-node build-spec --chain local > ./chain-specs/flarechain-shared.json
    echo "✅ Chain spec generated"
else
    echo "✅ FlareChain chain spec exists"
fi

echo ""
echo "========================================"
echo "Test Instructions"
echo "========================================"
echo ""
echo "To test the bridge, run these commands in separate terminals:"
echo ""
echo "Terminal 1 - Start FlareChain:"
echo "  ./target/release/flarechain-node \\"
echo "    --chain chain-specs/flarechain-shared.json \\"
echo "    --alice \\"
echo "    --validator \\"
echo "    --node-key 0000000000000000000000000000000000000000000000000000000000000004 \\"
echo "    --base-path /tmp/flarechain-alice"
echo ""
echo "Terminal 2 - Start BTC PBC Collator:"
echo "  ./target/release/btc-pbc-collator \\"
echo "    --dev \\"
echo "    --relay-chain-rpc-url ws://127.0.0.1:9944 \\"
echo "    --base-path /tmp/btc-pbc-collator"
echo ""
echo "Expected Results:"
echo "  - FlareChain starts and produces blocks"
echo "  - BTC PBC collator connects to FlareChain"
echo "  - BTC PBC collator starts producing blocks"
echo "  - No GenesisBuilder errors in logs"
echo ""
echo "To verify bridge functionality:"
echo "  1. Check both chains are producing blocks"
echo "  2. Submit a cross-chain transaction"
echo "  3. Verify state updates on both chains"
echo ""
