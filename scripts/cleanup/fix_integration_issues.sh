#!/bin/bash
# Ëtrid Protocol - Fix Integration Issues
# Removes empty stub directories and improves project structure
# Safe to run: Only removes empty directories

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
cd "$ETRID_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║         ËTRID PROTOCOL - INTEGRATION FIXES                  ║"
echo "║         Removing Empty Stubs & Improving Structure          ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo

# Count empty directories before
EMPTY_BEFORE=$(find . -type d -empty ! -path "*/.git/*" ! -path "*/node_modules/*" ! -path "*/target/*" 2>/dev/null | wc -l)
echo -e "${BLUE}Empty directories before: $EMPTY_BEFORE${NC}"
echo

# Step 1: Remove empty client stubs
echo -e "${YELLOW}Step 1: Removing empty client stub directories...${NC}"
REMOVED=0

if [ -d "13-clients/web-wallet" ] && [ -z "$(ls -A 13-clients/web-wallet)" ]; then
    rm -rf 13-clients/web-wallet
    echo "  Removed: 13-clients/web-wallet (empty stub)"
    REMOVED=$((REMOVED + 1))
fi

if [ -d "13-clients/mobile-wallet" ] && [ -z "$(ls -A 13-clients/mobile-wallet)" ]; then
    rm -rf 13-clients/mobile-wallet
    echo "  Removed: 13-clients/mobile-wallet (empty stub)"
    REMOVED=$((REMOVED + 1))
fi

if [ -d "13-clients/ui-generated" ] && [ -z "$(ls -A 13-clients/ui-generated)" ]; then
    rm -rf 13-clients/ui-generated
    echo "  Removed: 13-clients/ui-generated (empty stub)"
    REMOVED=$((REMOVED + 1))
fi

if [ -d "13-clients/cli/etrcpp-console/build" ] && [ -z "$(ls -A 13-clients/cli/etrcpp-console/build)" ]; then
    rm -rf 13-clients/cli/etrcpp-console/build
    echo "  Removed: 13-clients/cli/etrcpp-console/build (empty build dir)"
    REMOVED=$((REMOVED + 1))
fi

echo -e "${GREEN}✓ Removed $REMOVED empty client directories${NC}"
echo

# Step 2: Remove empty foundation directories
echo -e "${YELLOW}Step 2: Removing empty foundation directories...${NC}"
REMOVED=0

if [ -d "10-foundation/legal" ] && [ -z "$(ls -A 10-foundation/legal)" ]; then
    rm -rf 10-foundation/legal
    echo "  Removed: 10-foundation/legal (empty stub)"
    REMOVED=$((REMOVED + 1))
fi

echo -e "${GREEN}✓ Removed $REMOVED empty foundation directories${NC}"
echo

# Step 3: Remove empty infrastructure directories
echo -e "${YELLOW}Step 3: Removing empty infrastructure directories...${NC}"
REMOVED=0

for dir in infra/terraform/digitalocean infra/terraform/gcp infra/terraform/aws; do
    if [ -d "$dir" ] && [ -z "$(ls -A $dir)" ]; then
        rm -rf "$dir"
        echo "  Removed: $dir (empty stub)"
        REMOVED=$((REMOVED + 1))
    fi
done

for dir in infra/monitoring/grafana infra/monitoring/alerts infra/monitoring/prometheus; do
    if [ -d "$dir" ] && [ -z "$(ls -A $dir)" ]; then
        rm -rf "$dir"
        echo "  Removed: $dir (empty stub)"
        REMOVED=$((REMOVED + 1))
    fi
done

echo -e "${GREEN}✓ Removed $REMOVED empty infrastructure directories${NC}"
echo

# Step 4: Remove empty tool directories
echo -e "${YELLOW}Step 4: Removing empty tool directories...${NC}"
REMOVED=0

for dir in tools/key-generator tools/cli/src/commands tools/genesis-builder; do
    if [ -d "$dir" ] && [ -z "$(ls -A $dir)" ]; then
        rm -rf "$dir"
        echo "  Removed: $dir (empty stub)"
        REMOVED=$((REMOVED + 1))
    fi
done

echo -e "${GREEN}✓ Removed $REMOVED empty tool directories${NC}"
echo

# Step 5: Remove empty contract directories
echo -e "${YELLOW}Step 5: Removing empty contract directories...${NC}"
REMOVED=0

for dir in contracts/ethereum/test contracts/ethereum/monitoring/grafana/datasources contracts/ethereum/monitoring/grafana/dashboards; do
    if [ -d "$dir" ] && [ -z "$(ls -A $dir)" ]; then
        rm -rf "$dir"
        echo "  Removed: $dir (empty stub)"
        REMOVED=$((REMOVED + 1))
    fi
done

echo -e "${GREEN}✓ Removed $REMOVED empty contract directories${NC}"
echo

# Step 6: Remove empty reference directories
echo -e "${YELLOW}Step 6: Removing empty reference directories...${NC}"
REMOVED=0

if [ -d "_reference/other-references" ] && [ -z "$(ls -A _reference/other-references)" ]; then
    rm -rf _reference/other-references
    echo "  Removed: _reference/other-references (empty)"
    REMOVED=$((REMOVED + 1))
fi

echo -e "${GREEN}✓ Removed $REMOVED empty reference directories${NC}"
echo

# Step 7: Clean up any remaining empty directories (recursive)
echo -e "${YELLOW}Step 7: Cleaning remaining empty directories...${NC}"
REMOVED=0

# Find and remove empty directories (bottom-up to handle nested empties)
while IFS= read -r dir; do
    if [ -d "$dir" ]; then
        rm -rf "$dir"
        echo "  Removed: $dir"
        REMOVED=$((REMOVED + 1))
    fi
done < <(find . -type d -empty ! -path "*/.git/*" ! -path "*/node_modules/*" ! -path "*/target/*" 2>/dev/null)

echo -e "${GREEN}✓ Removed $REMOVED additional empty directories${NC}"
echo

# Count empty directories after
EMPTY_AFTER=$(find . -type d -empty ! -path "*/.git/*" ! -path "*/node_modules/*" ! -path "*/target/*" 2>/dev/null | wc -l)

echo
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}                 INTEGRATION FIXES COMPLETE                ${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo
echo -e "${BLUE}Empty directories before: $EMPTY_BEFORE${NC}"
echo -e "${GREEN}Empty directories after:  $EMPTY_AFTER${NC}"
echo -e "${YELLOW}Total removed:            $((EMPTY_BEFORE - EMPTY_AFTER))${NC}"
echo
echo -e "${GREEN}✅ Integration issues fixed${NC}"
echo -e "${BLUE}Next: Run 13-clients documentation update${NC}"
echo
