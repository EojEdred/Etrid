#!/usr/bin/env bash
# Ëtrid FlareChain - Build PBC Collators
# Step 1 of PBC Deployment

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Building PBC Collators                  ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

cd "$ETRID_ROOT"

echo "Available PBCs:"
echo "1. edsc-pbc-collator (ËDSC Stablecoin) 🌟 PRIORITY"
echo "2. btc-pbc-collator (Bitcoin Bridge) 🌟 PRIORITY"
echo "3. eth-pbc-collator (Ethereum Bridge)"
echo "4. sol-pbc-collator (Solana Bridge)"
echo "5. xrp-pbc-collator (Ripple Bridge)"
echo "6. bnb-pbc-collator (Binance Chain Bridge)"
echo "7. trx-pbc-collator (Tron Bridge)"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Building EDSC-PBC Collator (Priority #1)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "This will take 30-60 minutes..."
echo ""

# Check if binary already exists
if [[ -f "target/release/edsc-pbc-collator" ]]; then
    echo "✓ edsc-pbc-collator already built!"
    echo "  Location: $ETRID_ROOT/target/release/edsc-pbc-collator"
    echo "  Size: $(ls -lh target/release/edsc-pbc-collator | awk '{print $5}')"
    echo ""
else
    echo "Building edsc-pbc-collator..."
    cargo build --release -p edsc-pbc-collator 2>&1 | tee build-edsc-pbc.log

    if [[ $? -eq 0 ]]; then
        echo "✓ edsc-pbc-collator built successfully!"
        echo "  Location: $ETRID_ROOT/target/release/edsc-pbc-collator"
        echo "  Size: $(ls -lh target/release/edsc-pbc-collator | awk '{print $5}')"
    else
        echo "✗ Build failed. Check build-edsc-pbc.log for errors"
        exit 1
    fi
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Building BTC-PBC Collator (Priority #2)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if binary already exists
if [[ -f "target/release/btc-pbc-collator" ]]; then
    echo "✓ btc-pbc-collator already built!"
    echo "  Location: $ETRID_ROOT/target/release/btc-pbc-collator"
    echo "  Size: $(ls -lh target/release/btc-pbc-collator | awk '{print $5}')"
    echo ""
else
    echo "Building btc-pbc-collator..."
    cargo build --release -p btc-pbc-collator 2>&1 | tee build-btc-pbc.log

    if [[ $? -eq 0 ]]; then
        echo "✓ btc-pbc-collator built successfully!"
        echo "  Location: $ETRID_ROOT/target/release/btc-pbc-collator"
        echo "  Size: $(ls -lh target/release/btc-pbc-collator | awk '{print $5}')"
    else
        echo "✗ Build failed. Check build-btc-pbc.log for errors"
        exit 1
    fi
fi

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  BUILD COMPLETE                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Summary
echo "Built PBC Collators:"
echo ""
if [[ -f "target/release/edsc-pbc-collator" ]]; then
    echo "  ✓ edsc-pbc-collator: $(ls -lh target/release/edsc-pbc-collator | awk '{print $5}')"
fi
if [[ -f "target/release/btc-pbc-collator" ]]; then
    echo "  ✓ btc-pbc-collator: $(ls -lh target/release/btc-pbc-collator | awk '{print $5}')"
fi

echo ""
echo "Next Steps:"
echo "1. Generate PBC chainspecs"
echo "2. Deploy collators to validators 6-21"
echo "3. Generate and insert session keys"
echo "4. Register PBCs on FlareChain"
echo ""
echo "See: PBC_DEPLOYMENT_GUIDE.md for full instructions"
echo ""
