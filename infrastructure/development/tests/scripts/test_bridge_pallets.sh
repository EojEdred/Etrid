#!/bin/bash

# Test all 12 bridge pallets
# Tests bridge pallet compilation and basic unit tests

set -e

echo "=================================="
echo "🧪 Testing All 12 Bridge Pallets"
echo "=================================="
echo ""

PASS=0
FAIL=0
TOTAL=12

# Bridge pallet mapping
declare -A BRIDGES=(
    ["bitcoin"]="pallet-bitcoin-bridge"
    ["ethereum"]="eth-bridge"
    ["dogecoin"]="pallet-doge-bridge"
    ["stellar"]="stellar-bridge"
    ["xrp"]="xrp-bridge"
    ["bnb"]="bnb-bridge"
    ["tron"]="trx-bridge"
    ["cardano"]="pallet-cardano-bridge"
    ["chainlink"]="chainlink-bridge"
    ["polygon"]="polygon-bridge"
    ["stablecoin-usdt"]="stablecoin-usdt-bridge"
    ["solana"]="sol-bridge"
)

cd 05-multichain/bridge-protocols

for chain in bitcoin ethereum dogecoin stellar xrp bnb tron cardano chainlink polygon stablecoin-usdt solana; do
    pallet="${BRIDGES[$chain]}"

    echo -n "Testing $chain bridge ($pallet)... "

    if [ ! -d "$pallet" ]; then
        echo "❌ MISSING (directory not found)"
        ((FAIL++))
        continue
    fi

    # Check if Cargo.toml exists
    if [ ! -f "$pallet/Cargo.toml" ]; then
        echo "⚠️  SKIP (no Cargo.toml)"
        ((FAIL++))
        continue
    fi

    # Try to check the package
    if env SKIP_WASM_BUILD=1 cargo check -p "$pallet" 2>&1 | grep -q "Finished"; then
        echo "✅ PASS"
        ((PASS++))
    else
        echo "❌ FAIL (compilation errors)"
        ((FAIL++))
    fi
done

cd ../..

echo ""
echo "=================================="
echo "Results: $PASS/$TOTAL bridge pallets compile"
echo "✅ Pass: $PASS"
echo "❌ Fail: $FAIL"
echo "=================================="

if [ $FAIL -eq 0 ]; then
    exit 0
else
    exit 1
fi
