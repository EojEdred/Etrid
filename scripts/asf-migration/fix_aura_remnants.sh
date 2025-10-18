#!/bin/bash
# Fix remaining AURA references that the first script missed

set -e

PBC_CHAINS="btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt"

echo "🔧 Fixing remaining AURA remnants..."
echo "======================================"

for pbc in $PBC_CHAINS; do
    RUNTIME_FILE="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/src/lib.rs"

    if [ ! -f "$RUNTIME_FILE" ]; then
        echo "⚠️  Skipping ${pbc}-pbc: Runtime file not found"
        continue
    fi

    echo "📝 Fixing ${pbc}-pbc-runtime..."

    # Fix 1: Change type OnTimestampSet = Aura; to type OnTimestampSet = ();
    sed -i.bak 's/type OnTimestampSet = Aura;/type OnTimestampSet = ();/g' "$RUNTIME_FILE"

    # Fix 2: Remove AuraApi implementation block (more aggressive multi-line delete)
    perl -i.bak2 -0pe 's/impl sp_consensus_aura::AuraApi.*?\n.*?\n.*?fn slot_duration.*?\n.*?\n.*?\n.*?fn authorities.*?\n.*?\n.*?\n.*?\}//gs' "$RUNTIME_FILE"

    # Clean up backup files
    rm -f "${RUNTIME_FILE}.bak"*

    echo "✅ ${pbc}-pbc-runtime: Fixed!"
done

echo ""
echo "======================================"
echo "✨ All AURA remnants removed!"
