#!/bin/bash
# Script to fix syntax errors in GrandpaApi implementations

set -e

echo "🔧 Fixing GrandpaApi syntax errors in PBC runtimes..."
echo "======================================================"

PBCS="eth doge sol xlm xrp bnb trx ada link matic sc-usdt"

for pbc in $PBCS; do
    RUNTIME_FILE="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/src/lib.rs"

    if [ ! -f "$RUNTIME_FILE" ]; then
        echo "⚠️  Skipping ${pbc}-pbc: Runtime file not found"
        continue
    fi

    echo "📝 Processing ${pbc}-pbc-runtime..."

    # Backup original file
    cp "$RUNTIME_FILE" "${RUNTIME_FILE}.syntax_backup"

    # Fix the syntax error: Remove the extra closing brace and ensure GrandpaApi is inside impl_runtime_apis!
    # The issue is line 61 in eth-pbc which has "    }" that closes impl_runtime_apis prematurely
    # We need to remove that line and ensure GrandpaApi is properly indented inside impl_runtime_apis!

    # Find and remove the line that has "    }" right before "impl sp_consensus_grandpa::GrandpaApi"
    perl -i -0pe 's/(\s+}\n\s+\n\s+impl sp_consensus_grandpa::GrandpaApi)/\n    impl sp_consensus_grandpa::GrandpaApi/gs' "$RUNTIME_FILE"

    echo "✅ ${pbc}-pbc-runtime: Syntax fixed"
done

echo ""
echo "======================================================"
echo "✨ All PBC runtimes syntax errors fixed!"
