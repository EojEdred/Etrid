#!/bin/bash

# Test compilation of all 12 PBC runtimes

echo "=========================================="
echo "🧪 Testing All 12 PBC Runtime Compilation"
echo "=========================================="
echo ""

PASS=0
FAIL=0
TOTAL=12

PBCS=("btc" "eth" "doge" "xlm" "xrp" "bnb" "trx" "ada" "link" "matic" "sc-usdt" "sol")

for pbc in "${PBCS[@]}"; do
    echo -n "Testing $pbc-pbc-runtime... "

    if env SKIP_WASM_BUILD=1 cargo check -p $pbc-pbc-runtime 2>&1 | grep -q "Finished"; then
        echo "✅ PASS"
        ((PASS++))
    else
        echo "❌ FAIL"
        ((FAIL++))
    fi
done

echo ""
echo "=========================================="
echo "Results: $PASS/$TOTAL runtimes compile"
echo "✅ Pass: $PASS"
echo "❌ Fail: $FAIL"
echo "=========================================="

if [ $FAIL -eq 0 ]; then
    echo ""
    echo "🎉 SUCCESS! All 12/12 bridges properly integrated!"
    exit 0
else
    exit 1
fi
