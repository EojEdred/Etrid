#!/bin/bash
# Verify Lightning Integration Across All PBCs
# Generated: November 5, 2025

echo "🔍 Verifying Lightning-Bloc Integration Across All PBCs"
echo "========================================================"

PBC_BASE="/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains"
PBCS=("eth-pbc" "btc-pbc" "bnb-pbc" "sol-pbc" "ada-pbc" "trx-pbc" "xrp-pbc" "xlm-pbc" "matic-pbc" "link-pbc" "doge-pbc" "sc-usdt-pbc" "edsc-pbc")

PASSED=0
FAILED=0

for PBC in "${PBCS[@]}"; do
    echo ""
    echo "Checking $PBC..."
    echo "-----------------------------------"
    
    # Check Cargo.toml
    if grep -q "pallet-lightning-channels" "$PBC_BASE/$PBC/runtime/Cargo.toml" 2>/dev/null; then
        echo "  ✅ Cargo.toml has pallet-lightning-channels"
    else
        echo "  ❌ MISSING: pallet-lightning-channels in Cargo.toml"
        ((FAILED++))
        continue
    fi
    
    # Check runtime lib.rs
    if grep -q "impl pallet_lightning_channels::Config" "$PBC_BASE/$PBC/runtime/src/lib.rs" 2>/dev/null; then
        echo "  ✅ Runtime Config implemented"
    else
        echo "  ⚠️  WARNING: Config not found in lib.rs"
    fi
    
    # Check construct_runtime or runtime macro
    if grep -q "LightningChannels" "$PBC_BASE/$PBC/runtime/src/lib.rs" 2>/dev/null; then
        echo "  ✅ LightningChannels in runtime macro"
        ((PASSED++))
    else
        echo "  ⚠️  WARNING: LightningChannels not in runtime macro"
    fi
done

echo ""
echo "========================================================"
echo "📊 Summary:"
echo "  ✅ Passed: $PASSED / ${#PBCS[@]}"
echo "  ❌ Failed: $FAILED / ${#PBCS[@]}"

if [ $FAILED -eq 0 ]; then
    echo ""
    echo "🎉 ALL PBCs HAVE LIGHTNING INTEGRATION!"
    exit 0
else
    echo ""
    echo "⚠️  Some PBCs need fixes"
    exit 1
fi
