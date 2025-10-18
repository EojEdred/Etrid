#!/bin/bash

# Test runtime integration for all 12 PBC runtimes
# Validates that bridge pallets are properly integrated

set -e

echo "========================================="
echo "🔬 Testing Runtime Bridge Integration"
echo "========================================="
echo ""

PASS=0
FAIL=0
TOTAL=12

# PBC runtimes
PBCS=("btc" "eth" "doge" "xlm" "xrp" "bnb" "trx" "ada" "link" "matic" "sc-usdt" "sol")

for pbc in "${PBCS[@]}"; do
    echo -n "Testing $pbc-pbc-runtime... "

    runtime_path="05-multichain/partition-burst-chains/pbc-chains/$pbc-pbc/runtime"

    if [ ! -d "$runtime_path" ]; then
        echo "❌ MISSING"
        ((FAIL++))
        continue
    fi

    # Check if runtime compiles
    if env SKIP_WASM_BUILD=1 cargo check -p "$pbc-pbc-runtime" 2>&1 | grep -q "Finished"; then
        echo "✅ PASS"
        ((PASS++))
    else
        echo "❌ FAIL"
        ((FAIL++))
    fi
done

echo ""
echo "========================================="
echo "Results: $PASS/$TOTAL runtimes compile"
echo "✅ Pass: $PASS"
echo "❌ Fail: $FAIL"
echo "========================================="

if [ $FAIL -eq 0 ]; then
    exit 0
else
    exit 1
fi
