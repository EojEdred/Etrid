#!/bin/bash
# Build all 12 remaining PBC collators with WASM in parallel
# Builds in batches to avoid overwhelming the system

set -e

echo "========================================"
echo "Building All 12 PBC Collators with WASM"
echo "========================================"
echo ""
echo "This will take approximately 60-90 minutes"
echo "Builds will run in parallel batches of 3-4"
echo ""

# Check available disk space
AVAILABLE=$(df -h . | tail -1 | awk '{print $4}')
echo "Available disk space: $AVAILABLE"
echo ""

# Batch 1: ETH, DOGE, SOL, XLM (4 builds)
echo "=== Batch 1: ETH, DOGE, SOL, XLM ==="
cargo build --release -p eth-pbc-collator > .eth-pbc-build.log 2>&1 &
PID_ETH=$!
cargo build --release -p doge-pbc-collator > .doge-pbc-build.log 2>&1 &
PID_DOGE=$!
cargo build --release -p sol-pbc-collator > .sol-pbc-build.log 2>&1 &
PID_SOL=$!
cargo build --release -p xlm-pbc-collator > .xlm-pbc-build.log 2>&1 &
PID_XLM=$!

echo "Started: ETH ($PID_ETH), DOGE ($PID_DOGE), SOL ($PID_SOL), XLM ($PID_XLM)"
echo "Waiting for batch 1 to complete..."
wait $PID_ETH $PID_DOGE $PID_SOL $PID_XLM
echo "✓ Batch 1 complete"
echo ""

# Batch 2: XRP, BNB, TRX, ADA (4 builds)
echo "=== Batch 2: XRP, BNB, TRX, ADA ==="
cargo build --release -p xrp-pbc-collator > .xrp-pbc-build.log 2>&1 &
PID_XRP=$!
cargo build --release -p bnb-pbc-collator > .bnb-pbc-build.log 2>&1 &
PID_BNB=$!
cargo build --release -p trx-pbc-collator > .trx-pbc-build.log 2>&1 &
PID_TRX=$!
cargo build --release -p ada-pbc-collator > .ada-pbc-build.log 2>&1 &
PID_ADA=$!

echo "Started: XRP ($PID_XRP), BNB ($PID_BNB), TRX ($PID_TRX), ADA ($PID_ADA)"
echo "Waiting for batch 2 to complete..."
wait $PID_XRP $PID_BNB $PID_TRX $PID_ADA
echo "✓ Batch 2 complete"
echo ""

# Batch 3: LINK, MATIC, SC-USDT, EDSC (4 builds)
echo "=== Batch 3: LINK, MATIC, SC-USDT, EDSC ==="
cargo build --release -p link-pbc-collator > .link-pbc-build.log 2>&1 &
PID_LINK=$!
cargo build --release -p matic-pbc-collator > .matic-pbc-build.log 2>&1 &
PID_MATIC=$!
cargo build --release -p sc-usdt-pbc-collator > .sc-usdt-pbc-build.log 2>&1 &
PID_USDT=$!
cargo build --release -p edsc-pbc-collator > .edsc-pbc-build.log 2>&1 &
PID_EDSC=$!

echo "Started: LINK ($PID_LINK), MATIC ($PID_MATIC), SC-USDT ($PID_USDT), EDSC ($PID_EDSC)"
echo "Waiting for batch 3 to complete..."
wait $PID_LINK $PID_MATIC $PID_USDT $PID_EDSC
echo "✓ Batch 3 complete"
echo ""

echo "========================================"
echo "All Builds Complete!"
echo "========================================"
echo ""

# Verify all WASM runtimes were generated
echo "Verifying WASM runtimes..."
for pbc in eth doge sol xlm xrp bnb trx ada link matic sc-usdt edsc; do
    WASM_FILE="target/release/wbuild/${pbc}-pbc-runtime/${pbc}_pbc_runtime.compact.compressed.wasm"
    if [ -f "$WASM_FILE" ]; then
        SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
        echo "  ✓ $pbc-pbc: $SIZE"
    else
        echo "  ❌ $pbc-pbc: WASM NOT FOUND"
    fi
done

echo ""
echo "Build logs:"
for pbc in eth doge sol xlm xrp bnb trx ada link matic sc-usdt edsc; do
    if grep -q "Finished.*release.*target" ".${pbc}-pbc-build.log" 2>/dev/null; then
        echo "  ✓ $pbc-pbc build succeeded"
    else
        echo "  ❌ $pbc-pbc build may have failed - check .${pbc}-pbc-build.log"
    fi
done

echo ""
echo "Next step: Test chain spec generation for each collator"
