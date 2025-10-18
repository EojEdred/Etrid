#!/bin/bash
# Fix extra closing braces in all PBC runtimes

PBCS="bnb trx ada link matic sc-usdt"

for pbc in $PBCS; do
    echo "=== Fixing $pbc-pbc ==="
    file="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/src/lib.rs"

    if [ -f "$file" ]; then
        # Remove the extra } after AsfApi implementation (line pattern: "    }")
        # This specifically targets the double brace pattern
        sed -i.brace_fix '
            /impl sp_consensus_asf::AsfApi/,/^    }$/ {
                /^    }$/ {
                    N
                    s/^    }\n    }$/    }/
                }
            }
        ' "$file"
        echo "  ✅ Fixed $pbc-pbc"
    else
        echo "  ❌ File not found: $file"
    fi
done

echo ""
echo "✅ All PBC braces fixed!"
