#!/bin/bash
# Ëtrid Protocol - Phase 1: Immediate Cleanup
# Removes build artifacts and node_modules (saves 16+ GB)
# Safe to run: All deleted files are regenerable

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
echo "║         ËTRID PROTOCOL - PHASE 1 CLEANUP                    ║"
echo "║         Removing Build Artifacts & Node Modules             ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo

# Get initial size
INITIAL_SIZE=$(du -sh . | cut -f1)
echo -e "${BLUE}Initial project size: $INITIAL_SIZE${NC}"
echo

# Step 1: Clean Rust build artifacts
echo -e "${YELLOW}Step 1: Cleaning Rust build artifacts...${NC}"
if command -v cargo &> /dev/null; then
    cargo clean
    echo -e "${GREEN}✓ Rust build artifacts removed${NC}"
else
    echo -e "${RED}✗ Cargo not found, skipping${NC}"
fi
echo

# Step 2: Remove node_modules directories
echo -e "${YELLOW}Step 2: Removing node_modules directories...${NC}"
NODE_MODULES_COUNT=$(find . -name "node_modules" -type d ! -path "*/.git/*" | wc -l)
echo "Found $NODE_MODULES_COUNT node_modules directories"

if [ "$NODE_MODULES_COUNT" -gt 0 ]; then
    find . -name "node_modules" -type d ! -path "*/.git/*" -prune -exec rm -rf {} +
    echo -e "${GREEN}✓ All node_modules removed${NC}"
else
    echo -e "${BLUE}No node_modules found${NC}"
fi
echo

# Step 3: Delete empty test directory
echo -e "${YELLOW}Step 3: Deleting empty/obsolete directories...${NC}"
if [ -d ".edsc-test" ]; then
    rm -rf .edsc-test
    echo -e "${GREEN}✓ Removed .edsc-test${NC}"
fi

# Step 4: Delete audit tarball (already extracted)
if [ -f "etrid-audit-package-2025-10-21.tar.gz" ]; then
    rm -f etrid-audit-package-2025-10-21.tar.gz
    echo -e "${GREEN}✓ Removed audit tarball (already extracted)${NC}"
fi
echo

# Step 5: Clean up contracts build artifacts
echo -e "${YELLOW}Step 4: Cleaning Ethereum contract artifacts...${NC}"
if [ -d "contracts/ethereum/artifacts" ]; then
    rm -rf contracts/ethereum/artifacts
    echo -e "${GREEN}✓ Removed Ethereum artifacts${NC}"
fi
if [ -d "contracts/ethereum/cache" ]; then
    rm -rf contracts/ethereum/cache
    echo -e "${GREEN}✓ Removed Ethereum cache${NC}"
fi
echo

# Get final size
FINAL_SIZE=$(du -sh . | cut -f1)
echo
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}                 CLEANUP COMPLETE                          ${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo
echo -e "${BLUE}Before: $INITIAL_SIZE${NC}"
echo -e "${GREEN}After:  $FINAL_SIZE${NC}"
echo
echo -e "${YELLOW}Note: To reinstall dependencies:${NC}"
echo -e "  Rust: ${BLUE}cargo build${NC}"
echo -e "  Node: ${BLUE}npm install${NC} or ${BLUE}yarn install${NC}"
echo
echo -e "${GREEN}✅ Phase 1 Complete - Estimated savings: 16+ GB${NC}"
echo
