#!/bin/bash
#
# PBC Collator Batch Build Script
# Builds all 12 working PBC collators (excludes eth-pbc due to dependency conflicts)
#
set -e

ETRID_DIR="/Users/macbook/Desktop/etrid"
cd "$ETRID_DIR"

# Color output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "======================================"
echo "PBC Collator Batch Build"
echo "======================================"
echo ""

# List of collators to build (excluding eth-pbc)
COLLATORS=(
    "btc-pbc-collator"
    "sol-pbc-collator"
    "bnb-pbc-collator"
    "edsc-pbc-collator"
    "xrp-pbc-collator"
    "trx-pbc-collator"
    "ada-pbc-collator"
    "matic-pbc-collator"
    "link-pbc-collator"
    "sc-usdt-pbc-collator"
    "doge-pbc-collator"
    "xlm-pbc-collator"
)

BUILT=()
FAILED=()
ALREADY_BUILT=()

# Check which collators are already built
echo -e "${YELLOW}Checking for existing binaries...${NC}"
for collator in "${COLLATORS[@]}"; do
    if [ -f "target/release/$collator" ]; then
        echo -e "${GREEN}✓${NC} $collator already exists"
        ALREADY_BUILT+=("$collator")
    fi
done
echo ""

# Build each collator
for collator in "${COLLATORS[@]}"; do
    # Skip if already built
    if [[ " ${ALREADY_BUILT[@]} " =~ " ${collator} " ]]; then
        echo -e "${YELLOW}⊙${NC} Skipping $collator (already built)"
        BUILT+=("$collator")
        continue
    fi

    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Building: $collator${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    START_TIME=$(date +%s)

    if cargo build --release -p "$collator" 2>&1 | tee "/tmp/${collator}_build.log"; then
        END_TIME=$(date +%s)
        DURATION=$((END_TIME - START_TIME))
        echo -e "${GREEN}✓ Successfully built $collator in ${DURATION}s${NC}"
        BUILT+=("$collator")

        # Verify binary exists and show size
        if [ -f "target/release/$collator" ]; then
            SIZE=$(ls -lh "target/release/$collator" | awk '{print $5}')
            echo -e "${GREEN}  Binary size: $SIZE${NC}"
        fi
    else
        echo -e "${RED}✗ Failed to build $collator${NC}"
        FAILED+=("$collator")
        echo -e "${RED}  Build log saved to: /tmp/${collator}_build.log${NC}"
    fi

    echo ""
done

# Final summary
echo ""
echo "======================================"
echo "Build Summary"
echo "======================================"
echo -e "${GREEN}Successfully built: ${#BUILT[@]}${NC}"
for collator in "${BUILT[@]}"; do
    echo -e "  ${GREEN}✓${NC} $collator"
done

if [ ${#FAILED[@]} -gt 0 ]; then
    echo ""
    echo -e "${RED}Failed: ${#FAILED[@]}${NC}"
    for collator in "${FAILED[@]}"; do
        echo -e "  ${RED}✗${NC} $collator"
    done
fi

echo ""
echo "======================================"
echo "All Binaries:"
echo "======================================"
ls -lh target/release/*-pbc-collator 2>/dev/null || echo "No binaries found"

echo ""
echo -e "${YELLOW}Note: eth-pbc-collator excluded due to dependency conflicts (needs stable2506 investigation)${NC}"

exit 0
