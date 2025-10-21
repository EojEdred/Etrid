#!/bin/bash

# Test all 12 PBC collators

echo "🧪 Testing All 12 PBC Collators..."
echo "=================================="
echo ""

COLLATORS=(
    "btc-pbc-collator"
    "eth-pbc-collator"
    "doge-pbc-collator"
    "xlm-pbc-collator"
    "xrp-pbc-collator"
    "bnb-pbc-collator"
    "trx-pbc-collator"
    "ada-pbc-collator"
    "link-pbc-collator"
    "matic-pbc-collator"
    "sc-usdt-pbc-collator"
    "sol-pbc-collator"
)

PASS=0
FAIL=0

for collator in "${COLLATORS[@]}"; do
    echo -n "Testing $collator... "
    if env SKIP_WASM_BUILD=1 cargo check -p "$collator" 2>&1 | grep -q "Finished"; then
        echo "✅ PASS"
        ((PASS++))
    else
        echo "❌ FAIL"
        ((FAIL++))
    fi
done

echo ""
echo "=================================="
echo "Results: $PASS/12 collators compile"
echo "✅ Pass: $PASS"
echo "❌ Fail: $FAIL"
echo "=================================="
