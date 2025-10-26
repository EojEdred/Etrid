#!/bin/bash
# Comprehensive PBC Test Script
# Tests both runtimes and collators for all 12 PBCs

set -e

PBCS=("btc" "eth" "doge" "xlm" "xrp" "bnb" "trx" "ada" "link" "matic" "sc-usdt" "sol")

echo "========================================"
echo "Comprehensive PBC Compilation Test"
echo "Testing 12 Runtimes + 12 Collators = 24 components"
echo "========================================"
echo ""

RUNTIME_PASS=0
RUNTIME_FAIL=0
COLLATOR_PASS=0
COLLATOR_FAIL=0

echo "PHASE 1: Testing all 12 PBC Runtimes"
echo "========================================"

for pbc in "${PBCS[@]}"; do
    echo -n "Testing $pbc-pbc-runtime... "
    if env SKIP_WASM_BUILD=1 cargo check -p $pbc-pbc-runtime 2>&1 | grep -q "Finished"; then
        echo "✅ PASS"
        ((RUNTIME_PASS++))
    else
        echo "❌ FAIL"
        ((RUNTIME_FAIL++))
    fi
done

echo ""
echo "PHASE 2: Testing all 12 PBC Collators"
echo "========================================"

for pbc in "${PBCS[@]}"; do
    echo -n "Testing $pbc-pbc-collator... "
    if env SKIP_WASM_BUILD=1 cargo check -p $pbc-pbc-collator 2>&1 | grep -q "Finished"; then
        echo "✅ PASS"
        ((COLLATOR_PASS++))
    else
        echo "❌ FAIL"
        ((COLLATOR_FAIL++))
    fi
done

echo ""
echo "========================================"
echo "FINAL RESULTS"
echo "========================================"
echo "Runtimes:  $RUNTIME_PASS/12 passed"
echo "Collators: $COLLATOR_PASS/12 passed"
echo "Total:     $((RUNTIME_PASS + COLLATOR_PASS))/24 components passed"
echo "========================================"

if [ $RUNTIME_FAIL -eq 0 ] && [ $COLLATOR_FAIL -eq 0 ]; then
    echo "✅ ALL TESTS PASSED!"
    exit 0
else
    echo "❌ Some tests failed"
    exit 1
fi
