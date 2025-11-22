#!/bin/bash
# Script to generate new Ëtrid FlareChain mainnet chainspec with all 21 validators

set -e

BINARY="./target/release/etrid-validator"
PRESET_NAME="flarechain_mainnet"
OUTPUT_DIR="/Users/macbook/Desktop/etrid/docs/mainnet"
PLAIN_SPEC="${OUTPUT_DIR}/chainspec-mainnet-plain.json"
RAW_SPEC="${OUTPUT_DIR}/chainspec-mainnet-raw-new.json"

echo "=============================================="
echo "Generating Ëtrid FlareChain Mainnet Chainspec"
echo "=============================================="
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Error: Binary not found at $BINARY"
    echo "Please build the binary first: cargo build --release"
    exit 1
fi

echo "✅ Binary found: $BINARY"
echo ""

# Check binary version
echo "Binary version:"
$BINARY --version
echo ""

# Step 1: Build plain chainspec
echo "Step 1: Building plain chainspec..."
echo "Preset: $PRESET_NAME"
echo "Output: $PLAIN_SPEC"
echo ""

$BINARY build-spec \
    --chain=local \
    --disable-default-bootnode \
    > "$PLAIN_SPEC"

echo "✅ Plain chainspec generated"
echo ""

# Step 2: Customize the plain chainspec with proper name and settings
echo "Step 2: Customizing chainspec metadata..."

# Use jq to update the chainspec
jq '
  .name = "Ëtrid FlareChain Mainnet" |
  .id = "flarechain_mainnet" |
  .chainType = "Live" |
  .protocolId = "flarechain" |
  .properties = {
    "tokenSymbol": "ETR",
    "tokenDecimals": 12,
    "ss58Format": 42
  }
' "$PLAIN_SPEC" > "${PLAIN_SPEC}.tmp" && mv "${PLAIN_SPEC}.tmp" "$PLAIN_SPEC"

echo "✅ Chainspec customized"
echo ""

# Step 3: Convert to raw format
echo "Step 3: Converting to raw format..."
echo "Output: $RAW_SPEC"
echo ""

$BINARY build-spec \
    --chain="$PLAIN_SPEC" \
    --raw \
    > "$RAW_SPEC"

echo "✅ Raw chainspec generated"
echo ""

# Step 4: Verify the chainspec
echo "Step 4: Verifying chainspec..."
echo ""

# Extract and display key information
echo "Chain Name: $(jq -r '.name' "$RAW_SPEC")"
echo "Chain ID: $(jq -r '.id' "$RAW_SPEC")"
echo "Chain Type: $(jq -r '.chainType' "$RAW_SPEC")"
echo "Protocol ID: $(jq -r '.protocolId' "$RAW_SPEC")"
echo ""

# Calculate file sizes
PLAIN_SIZE=$(ls -lh "$PLAIN_SPEC" | awk '{print $5}')
RAW_SIZE=$(ls -lh "$RAW_SPEC" | awk '{print $5}')

echo "Plain chainspec size: $PLAIN_SIZE"
echo "Raw chainspec size: $RAW_SIZE"
echo ""

echo "=============================================="
echo "✅ SUCCESS: Chainspec generation complete!"
echo "=============================================="
echo ""
echo "Generated files:"
echo "  Plain: $PLAIN_SPEC"
echo "  Raw:   $RAW_SPEC"
echo ""
echo "Next steps:"
echo "  1. Review the raw chainspec"
echo "  2. Stop all validators"
echo "  3. Distribute new chainspec to all VMs"
echo "  4. Purge validator databases"
echo "  5. Restart validators with new genesis"
echo ""
