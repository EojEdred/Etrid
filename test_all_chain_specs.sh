#!/bin/bash
# Test chain spec generation for all 12 PBC collators
# This verifies the GenesisBuilder API is working correctly

echo "========================================"
echo "Testing Chain Spec Generation - All PBCs"
echo "========================================"
echo ""

PBCS=(btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt)
SUCCESS=0
FAILED=0

for pbc in "${PBCS[@]}"; do
    echo -n "Testing ${pbc}-pbc chain spec generation... "

    if [ ! -f "./target/release/${pbc}-pbc-collator" ]; then
        echo "⏭️  SKIPPED (binary not found)"
        continue
    fi

    if ./target/release/${pbc}-pbc-collator build-spec --chain dev > /dev/null 2>&1; then
        echo "✅ SUCCESS"
        ((SUCCESS++))
    else
        echo "❌ FAILED"
        ((FAILED++))
        echo "  Error details:"
        ./target/release/${pbc}-pbc-collator build-spec --chain dev 2>&1 | grep -i error | head -3 | sed 's/^/    /'
    fi
done

echo ""
echo "========================================"
echo "Summary: $SUCCESS passed, $FAILED failed"
echo "========================================"

if [ $FAILED -eq 0 ]; then
    echo "✅ All available PBC collators can generate chain specs!"
    exit 0
else
    echo "❌ Some PBC collators failed chain spec generation"
    exit 1
fi
