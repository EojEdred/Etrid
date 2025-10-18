#!/bin/bash
# Script to add missing closing brace for SessionKeys impl blocks

set -e

echo "🔧 Adding missing closing braces for SessionKeys implementations..."
echo "=================================================================="

PBCS="doge sol xlm xrp bnb trx ada link matic sc-usdt"

for pbc in $PBCS; do
    RUNTIME_FILE="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/src/lib.rs"

    if [ ! -f "$RUNTIME_FILE" ]; then
        echo "⚠️  Skipping ${pbc}-pbc: Runtime file not found"
        continue
    fi

    echo "📝 Processing ${pbc}-pbc-runtime..."

    # Backup original file
    cp "$RUNTIME_FILE" "${RUNTIME_FILE}.brace_backup"

    # Add closing brace after SessionKeys decode_session_keys method, before GrandpaApi
    # Match the pattern and add a closing brace
    perl -i -0pe 's/(\s+opaque::SessionKeys::decode_into_raw_public_keys\(&encoded\)\s+\}\s+)\n(\s+impl sp_consensus_grandpa::GrandpaApi)/\1}\n\n\2/gs' "$RUNTIME_FILE"

    echo "✅ ${pbc}-pbc-runtime: Closing brace added"
done

echo ""
echo "=================================================================="
echo "✨ All SessionKeys implementations fixed!"
