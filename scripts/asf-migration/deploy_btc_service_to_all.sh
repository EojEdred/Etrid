#!/bin/bash
# Copy working btc-pbc-collator service.rs to all 9 broken collators

BROKEN_COLLATORS="doge xrp bnb trx ada link matic sc-usdt sol"
BASE_DIR="05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes"

echo "🔧 Deploying working service.rs to all 9 collators..."
echo ""

for collator in $BROKEN_COLLATORS; do
    echo "=== ${collator}-pbc-collator ==="

    source_file="$BASE_DIR/btc-pbc-collator/src/service.rs"
    dest_file="$BASE_DIR/${collator}-pbc-collator/src/service.rs"

    # Backup current file
    cp "$dest_file" "${dest_file}.broken_backup"

    # Copy btc service.rs
    cp "$source_file" "$dest_file"

    # Replace all "btc" with the current collator name
    sed -i '' "s/btc_pbc/${collator//-/_}_pbc/g" "$dest_file"
    sed -i '' "s/btc-pbc/${collator}-pbc/g" "$dest_file"

    # Fix specific capitalization issues
    if [ "$collator" = "sc-usdt" ]; then
        sed -i '' "s/Sc_usdt/ScUsdt/g" "$dest_file"
    fi

    echo "  ✅ Deployed and customized service.rs"
done

echo ""
echo "✅ All 9 collator service.rs files deployed!"
