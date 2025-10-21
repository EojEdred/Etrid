#!/bin/bash
# Test compilation of all PBC runtimes

echo "=========================================="
echo "Testing PBC Runtime Compilations"
echo "=========================================="
echo ""

PBCS="btc eth doge xlm xrp bnb trx ada link matic sc-usdt"
SUCCESS=0
FAILED=0
FAILED_PBCS=""

for pbc in $PBCS; do
    echo -n "Testing ${pbc}-pbc-runtime... "

    if env SKIP_WASM_BUILD=1 cargo check -p ${pbc}-pbc-runtime > /tmp/${pbc}_build.log 2>&1; then
        echo "✅ PASS"
        SUCCESS=$((SUCCESS + 1))
    else
        echo "❌ FAIL"
        FAILED=$((FAILED + 1))
        FAILED_PBCS="${FAILED_PBCS} ${pbc}"
        echo "  Error log: /tmp/${pbc}_build.log"
    fi
done

echo ""
echo "=========================================="
echo "Summary"
echo "=========================================="
echo "Passed: ${SUCCESS}/11"
echo "Failed: ${FAILED}/11"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Failed runtimes:${FAILED_PBCS}"
    echo ""
    echo "To see errors:"
    for pbc in $FAILED_PBCS; do
        echo "  cat /tmp/${pbc}_build.log | grep '^error\\['"
    done
fi

echo ""
