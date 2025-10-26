#!/bin/bash

# Test all smart contract examples
# Usage: ./test-all.sh

set -e  # Exit on error

echo "🧪 Testing all Ëtrid smart contract examples..."
echo ""

EXAMPLES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_CONTRACTS=()

# Array of contract directories
CONTRACTS=(
    "01-hello-world"
    "02-counter"
    "03-erc20-token"
    "04-simple-dao"
    "05-escrow"
)

for contract in "${CONTRACTS[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📦 Testing: $contract"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    cd "$EXAMPLES_DIR/$contract"

    if cargo test --lib 2>&1 | tee /tmp/test-output.txt; then
        # Extract test count from output
        TEST_COUNT=$(grep -oP '\d+(?= passed)' /tmp/test-output.txt | tail -1)
        if [ -n "$TEST_COUNT" ]; then
            TOTAL_TESTS=$((TOTAL_TESTS + TEST_COUNT))
            PASSED_TESTS=$((PASSED_TESTS + TEST_COUNT))
            echo "✅ $contract: $TEST_COUNT tests passed"
        else
            echo "✅ $contract: Tests passed (count unknown)"
        fi
    else
        echo "❌ $contract: Tests failed"
        FAILED_CONTRACTS+=("$contract")
    fi

    echo ""
    cd "$EXAMPLES_DIR"
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Test Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Total contracts tested: ${#CONTRACTS[@]}"
echo "Total tests passed: $PASSED_TESTS"

if [ ${#FAILED_CONTRACTS[@]} -eq 0 ]; then
    echo ""
    echo "🎉 SUCCESS! All contracts passed testing!"
    echo ""
    exit 0
else
    echo ""
    echo "❌ FAILED contracts:"
    for failed in "${FAILED_CONTRACTS[@]}"; do
        echo "  - $failed"
    done
    echo ""
    exit 1
fi
