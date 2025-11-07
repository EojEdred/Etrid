#!/bin/bash
set -e

# Build all PBC collators for Linux using Docker
# This produces x86_64 Linux binaries suitable for Oracle Cloud VMs

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "=== Building PBC Collators for Linux (x86_64) ==="
echo "This will take 60-90 minutes..."
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Error: Docker is not running. Please start Docker Desktop."
    exit 1
fi

# Build the Docker image
echo "Step 1/3: Building Docker image with all 12 PBC collators..."
docker build -f Dockerfile.pbc-builder -t etrid-pbc-builder:latest . 2>&1 | tee /tmp/docker-pbc-build.log

# Create output directory for Linux binaries
OUTPUT_DIR="$SCRIPT_DIR/target/linux-release"
mkdir -p "$OUTPUT_DIR"

# Extract binaries from Docker image
echo ""
echo "Step 2/3: Extracting Linux binaries from Docker image..."
CONTAINER_ID=$(docker create etrid-pbc-builder:latest)

# Extract all 12 collators
echo "Copying binaries..."
docker cp "$CONTAINER_ID:/usr/local/bin/btc-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/sol-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/bnb-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/edsc-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/xrp-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/matic-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/sc-usdt-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/xlm-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/trx-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/ada-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/link-pbc-collator" "$OUTPUT_DIR/"
docker cp "$CONTAINER_ID:/usr/local/bin/doge-pbc-collator" "$OUTPUT_DIR/"

docker rm "$CONTAINER_ID"

echo ""
echo "Step 3/3: Verifying Linux binaries..."
ls -lh "$OUTPUT_DIR"/*-pbc-collator

# Verify they are Linux binaries
echo ""
echo "Binary architecture check:"
file "$OUTPUT_DIR/btc-pbc-collator" | grep -q "ELF 64-bit" && echo "✓ Confirmed: Linux x86_64 binaries" || echo "⚠ Warning: Not Linux binaries!"

echo ""
echo "=== BUILD COMPLETE ==="
echo "Linux binaries saved to: $OUTPUT_DIR"
echo "Total size: $(du -sh $OUTPUT_DIR | cut -f1)"
echo ""
echo "Next steps:"
echo "1. Create deployment package: ./package-pbc-deployment.sh"
echo "2. Upload to Oracle Cloud VMs"
echo "3. Deploy using systemd services"
