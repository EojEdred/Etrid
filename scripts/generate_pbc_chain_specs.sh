#!/bin/bash
# Generate chain specs for all 13 PBC collators

set -e

CHAIN_SPECS_DIR="chain-specs"
PBCS=(sol xlm xrp bnb trx ada link matic sc-usdt edsc)

echo "=========================================="
echo "Generating Chain Specs for 10 PBCs"
echo "=========================================="
echo ""

for PBC in "${PBCS[@]}"; do
    SPEC_FILE="$CHAIN_SPECS_DIR/pbc-${PBC}-local.json"
    
    if [ -f "$SPEC_FILE" ]; then
        echo "✓ $SPEC_FILE already exists, skipping"
        continue
    fi
    
    echo "Creating $SPEC_FILE..."
    
    # Generate chain spec using the collator binary
    if [ -f "./target/release/${PBC}-pbc-collator" ]; then
        ./target/release/${PBC}-pbc-collator build-spec --chain local > "$SPEC_FILE"
        echo "✓ Generated $SPEC_FILE"
    else
        echo "⚠ Binary ./target/release/${PBC}-pbc-collator not found, creating template"
        
        # Create template chain spec
        cat > "$SPEC_FILE" << SPEC_EOF
{
  "name": "${PBC^^} Partition Burst Chain",
  "id": "${PBC}-pbc-local",
  "chainType": "Local",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "${PBC}-pbc",
  "properties": {
    "ss58Format": 42,
    "tokenDecimals": 18,
    "tokenSymbol": "${PBC^^}"
  },
  "relay_chain": "flarechain-local",
  "para_id": 2000,
  "codeSubstitutes": {},
  "genesis": {
    "runtime": {
      "system": {
        "code": "0x"
      },
      "balances": {
        "balances": []
      }
    }
  }
}
SPEC_EOF
        echo "✓ Created template $SPEC_FILE"
    fi
done

echo ""
echo "=========================================="
echo "Chain Spec Generation Complete"
echo "=========================================="
echo ""
echo "Generated specs for:"
for PBC in "${PBCS[@]}"; do
    echo "  - ${PBC}-pbc-local.json"
done
echo ""
echo "Next: Run ./test_full_multichain.sh to test all chains"
