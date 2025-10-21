#!/bin/bash
# Script to remove AURA consensus and keep only ASF + GRANDPA in all 12 PBC runtimes

set -e

PBC_CHAINS="btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt"

echo "🔧 Removing AURA from 12 PBC runtimes..."
echo "======================================"

for pbc in $PBC_CHAINS; do
    RUNTIME_FILE="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/src/lib.rs"

    if [ ! -f "$RUNTIME_FILE" ]; then
        echo "⚠️  Skipping ${pbc}-pbc: Runtime file not found"
        continue
    fi

    echo "📝 Processing ${pbc}-pbc-runtime..."

    # Backup original file
    cp "$RUNTIME_FILE" "${RUNTIME_FILE}.backup"

    # Create temporary file for processing
    TEMP_FILE="${RUNTIME_FILE}.tmp"

    # Step 1: Fix SessionKeys - Remove Aura, keep only Grandpa
    sed -i.bak1 '/impl_opaque_keys! {/,/^    }$/ {
        s/pub aura: Aura,//g
        /pub struct SessionKeys {$/,/^        }$/ {
            /pub grandpa: Grandpa,/! {
                /pub struct SessionKeys {/b
                /^        }$/b
                d
            }
        }
    }' "$RUNTIME_FILE"

    # Step 2: Remove pallet_aura::Config implementation
    perl -i.bak2 -0pe 's/impl pallet_aura::Config for Runtime \{[^}]*\n[^}]*\n[^}]*\n[^}]*\n[^}]*\n[^}]*\}//gs' "$RUNTIME_FILE"

    # Step 3: Remove Aura from construct_runtime!
    sed -i.bak3 's/Aura: pallet_aura,//g' "$RUNTIME_FILE"

    # Step 4: Remove AuraApi implementation from runtime APIs
    perl -i.bak4 -0pe 's/impl sp_consensus_aura::AuraApi<Block[^}]*\n[^}]*\n[^}]*\n[^}]*\n[^}]*\n[^}]*\n[^}]*\n    }//gs' "$RUNTIME_FILE"

    # Clean up backup files
    rm -f "${RUNTIME_FILE}.bak"*

    echo "✅ ${pbc}-pbc-runtime: AURA removed successfully"
done

echo ""
echo "======================================"
echo "✨ All 12 PBC runtimes updated!"
echo "Next: Verify builds with ASF-only consensus"
