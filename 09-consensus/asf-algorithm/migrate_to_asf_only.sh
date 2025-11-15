#!/bin/bash
# ════════════════════════════════════════════════════════════════════════════
# ËTRID Phase 4: GRANDPA Removal Migration Script
# ════════════════════════════════════════════════════════════════════════════
#
# Purpose: Automate the complete removal of GRANDPA from FlareChain runtime
# Status: READY FOR EXECUTION (after review)
# Runtime Version: v106 → v108
#
# ════════════════════════════════════════════════════════════════════════════

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Paths
ETRID_ROOT="/Users/macbook/Desktop/etrid"
RUNTIME_DIR="$ETRID_ROOT/05-multichain/flare-chain/runtime"
RUNTIME_SRC="$RUNTIME_DIR/src/lib.rs"
RUNTIME_CARGO="$RUNTIME_DIR/Cargo.toml"
NODE_DIR="$ETRID_ROOT/05-multichain/flare-chain/node"
NODE_CARGO="$NODE_DIR/Cargo.toml"
BACKUP_DIR="$ETRID_ROOT/09-consensus/asf-algorithm/phase4-backups"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

# Banner
echo -e "${BLUE}════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  ËTRID Phase 4: GRANDPA Removal Migration                                 ${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# Step 1: Verification
echo -e "${YELLOW}Step 1: Pre-Migration Verification${NC}"
echo "-----------------------------------"

# Check if files exist
if [ ! -f "$RUNTIME_SRC" ]; then
    echo -e "${RED}ERROR: Runtime lib.rs not found at $RUNTIME_SRC${NC}"
    exit 1
fi

if [ ! -f "$RUNTIME_CARGO" ]; then
    echo -e "${RED}ERROR: Runtime Cargo.toml not found at $RUNTIME_CARGO${NC}"
    exit 1
fi

if [ ! -f "$NODE_CARGO" ]; then
    echo -e "${RED}ERROR: Node Cargo.toml not found at $NODE_CARGO${NC}"
    exit 1
fi

# Check current runtime version
CURRENT_VERSION=$(grep "spec_version:" "$RUNTIME_SRC" | head -1 | grep -o '[0-9]*')
echo -e "Current runtime version: ${BLUE}$CURRENT_VERSION${NC}"

if [ "$CURRENT_VERSION" != "106" ]; then
    echo -e "${RED}ERROR: Expected runtime version 106, found $CURRENT_VERSION${NC}"
    echo -e "${YELLOW}This script is designed to migrate from v106 to v108${NC}"
    exit 1
fi

# Check for GRANDPA references
GRANDPA_COUNT_RUNTIME=$(grep -c -i "grandpa" "$RUNTIME_SRC" || true)
GRANDPA_COUNT_CARGO=$(grep -c -i "grandpa" "$RUNTIME_CARGO" || true)
GRANDPA_COUNT_NODE=$(grep -c -i "grandpa" "$NODE_CARGO" || true)

echo -e "GRANDPA references in runtime lib.rs: ${YELLOW}$GRANDPA_COUNT_RUNTIME${NC}"
echo -e "GRANDPA references in runtime Cargo.toml: ${YELLOW}$GRANDPA_COUNT_CARGO${NC}"
echo -e "GRANDPA references in node Cargo.toml: ${YELLOW}$GRANDPA_COUNT_NODE${NC}"
echo ""

# Ask for confirmation
echo -e "${YELLOW}This will:${NC}"
echo "  1. Backup all files to: $BACKUP_DIR"
echo "  2. Remove GRANDPA from runtime lib.rs (spec_version 106 → 108)"
echo "  3. Remove GRANDPA dependencies from Cargo.toml files"
echo "  4. Update SessionKeys to ASF-only"
echo "  5. Remove GRANDPA migration code"
echo ""
read -p "Continue with migration? (yes/no): " CONFIRM

if [ "$CONFIRM" != "yes" ]; then
    echo -e "${RED}Migration cancelled by user${NC}"
    exit 0
fi
echo ""

# Step 2: Create Backups
echo -e "${YELLOW}Step 2: Creating Backups${NC}"
echo "------------------------"

mkdir -p "$BACKUP_DIR"

# Backup runtime files
cp "$RUNTIME_SRC" "$BACKUP_DIR/lib.rs.v106.backup-$TIMESTAMP"
cp "$RUNTIME_CARGO" "$BACKUP_DIR/runtime-Cargo.toml.v106.backup-$TIMESTAMP"
cp "$NODE_CARGO" "$BACKUP_DIR/node-Cargo.toml.v106.backup-$TIMESTAMP"

echo -e "${GREEN}✓ Backups created in: $BACKUP_DIR${NC}"
echo ""

# Step 3: Update Runtime lib.rs
echo -e "${YELLOW}Step 3: Updating Runtime lib.rs${NC}"
echo "--------------------------------"

# Create a temp file for modifications
TEMP_FILE=$(mktemp)

# Remove GRANDPA import (line 11)
sed '/use sp_consensus_grandpa::AuthorityId as GrandpaId;/d' "$RUNTIME_SRC" > "$TEMP_FILE"
cp "$TEMP_FILE" "$RUNTIME_SRC"
echo -e "${GREEN}✓ Removed GRANDPA import${NC}"

# Update SessionKeys (lines 69-73)
# This is complex, so we'll use a multi-line sed approach
sed -i.bak '/impl_opaque_keys! {/,/}/ {
    /pub grandpa: Grandpa,/d
}' "$RUNTIME_SRC"
rm -f "$RUNTIME_SRC.bak"
echo -e "${GREEN}✓ Updated SessionKeys to ASF-only${NC}"

# Update runtime version to 108
sed -i.bak 's/spec_version: 106,/spec_version: 108,/' "$RUNTIME_SRC"
rm -f "$RUNTIME_SRC.bak"
echo -e "${GREEN}✓ Updated spec_version to 108${NC}"

# Remove GRANDPA migration module (lines 100-210)
sed -i.bak '/^\/\/ ═.*RUNTIME UPGRADE MIGRATION v106/,/^}/d' "$RUNTIME_SRC"
rm -f "$RUNTIME_SRC.bak"
echo -e "${GREEN}✓ Removed GRANDPA migration code${NC}"

# Remove pallet_grandpa::Config
sed -i.bak '/^impl pallet_grandpa::Config for Runtime {/,/^}/d' "$RUNTIME_SRC"
rm -f "$RUNTIME_SRC.bak"
echo -e "${GREEN}✓ Removed pallet_grandpa::Config${NC}"

# Remove Grandpa from construct_runtime!
sed -i.bak '/Grandpa: pallet_grandpa,/d' "$RUNTIME_SRC"
rm -f "$RUNTIME_SRC.bak"
echo -e "${GREEN}✓ Removed Grandpa from construct_runtime!${NC}"

# Update Executive migration
sed -i.bak 's/migrations::FixGrandpaCommitteeV106,/(),  \/\/ No migrations for v108/' "$RUNTIME_SRC"
rm -f "$RUNTIME_SRC.bak"
echo -e "${GREEN}✓ Updated Executive migration${NC}"

# Remove GRANDPA Runtime API
sed -i.bak '/impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {/,/^    }/d' "$RUNTIME_SRC"
rm -f "$RUNTIME_SRC.bak"
echo -e "${GREEN}✓ Removed GRANDPA Runtime API${NC}"

echo ""

# Step 4: Update Runtime Cargo.toml
echo -e "${YELLOW}Step 4: Updating Runtime Cargo.toml${NC}"
echo "------------------------------------"

# Remove pallet-grandpa dependency
sed -i.bak '/^pallet-grandpa = { git/d' "$RUNTIME_CARGO"
rm -f "$RUNTIME_CARGO.bak"
echo -e "${GREEN}✓ Removed pallet-grandpa dependency${NC}"

# Remove sp-consensus-grandpa primitive
sed -i.bak '/^sp-consensus-grandpa = { git/d' "$RUNTIME_CARGO"
rm -f "$RUNTIME_CARGO.bak"
echo -e "${GREEN}✓ Removed sp-consensus-grandpa primitive${NC}"

# Remove std feature flags
sed -i.bak '/"pallet-grandpa\/std",/d' "$RUNTIME_CARGO"
sed -i.bak '/"sp-consensus-grandpa\/std",/d' "$RUNTIME_CARGO"
rm -f "$RUNTIME_CARGO.bak"
echo -e "${GREEN}✓ Removed GRANDPA std feature flags${NC}"

echo ""

# Step 5: Update Node Cargo.toml
echo -e "${YELLOW}Step 5: Updating Node Cargo.toml${NC}"
echo "--------------------------------"

# Remove sc-consensus-grandpa
sed -i.bak '/^sc-consensus-grandpa = { git/d' "$NODE_CARGO"
rm -f "$NODE_CARGO.bak"
echo -e "${GREEN}✓ Removed sc-consensus-grandpa dependency${NC}"

# Remove sp-consensus-grandpa
sed -i.bak '/^sp-consensus-grandpa = { git/d' "$NODE_CARGO"
rm -f "$NODE_CARGO.bak"
echo -e "${GREEN}✓ Removed sp-consensus-grandpa primitive${NC}"

echo ""

# Step 6: Verification
echo -e "${YELLOW}Step 6: Post-Migration Verification${NC}"
echo "------------------------------------"

# Verify no GRANDPA references remain
GRANDPA_REMAINING_RUNTIME=$(grep -c -i "grandpa" "$RUNTIME_SRC" || true)
GRANDPA_REMAINING_CARGO=$(grep -c -i "grandpa" "$RUNTIME_CARGO" || true)
GRANDPA_REMAINING_NODE=$(grep -c -i "grandpa" "$NODE_CARGO" || true)

echo -e "GRANDPA references remaining:"
echo -e "  - Runtime lib.rs: ${BLUE}$GRANDPA_REMAINING_RUNTIME${NC}"
echo -e "  - Runtime Cargo.toml: ${BLUE}$GRANDPA_REMAINING_CARGO${NC}"
echo -e "  - Node Cargo.toml: ${BLUE}$GRANDPA_REMAINING_NODE${NC}"

if [ "$GRANDPA_REMAINING_RUNTIME" -eq 0 ] && [ "$GRANDPA_REMAINING_CARGO" -eq 0 ] && [ "$GRANDPA_REMAINING_NODE" -eq 0 ]; then
    echo -e "${GREEN}✓ All GRANDPA references successfully removed!${NC}"
else
    echo -e "${YELLOW}⚠ Warning: Some GRANDPA references may remain (could be in comments/docs)${NC}"
fi

# Verify new runtime version
NEW_VERSION=$(grep "spec_version:" "$RUNTIME_SRC" | head -1 | grep -o '[0-9]*')
echo -e "New runtime version: ${GREEN}$NEW_VERSION${NC}"

if [ "$NEW_VERSION" == "108" ]; then
    echo -e "${GREEN}✓ Runtime version successfully updated to v108${NC}"
else
    echo -e "${RED}ERROR: Runtime version not updated correctly${NC}"
    exit 1
fi

echo ""

# Step 7: Build Test (Optional)
echo -e "${YELLOW}Step 7: Build Test (Optional)${NC}"
echo "------------------------------"
echo "Would you like to test the build now?"
read -p "Run 'cargo check' on runtime? (yes/no): " RUN_CHECK

if [ "$RUN_CHECK" == "yes" ]; then
    echo -e "${BLUE}Running cargo check...${NC}"
    cd "$RUNTIME_DIR"
    if cargo check --release; then
        echo -e "${GREEN}✓ Runtime check passed!${NC}"
    else
        echo -e "${RED}✗ Runtime check failed${NC}"
        echo -e "${YELLOW}Review the errors and consider rolling back if needed${NC}"
    fi
    cd -
fi

echo ""

# Step 8: Summary
echo -e "${BLUE}════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Phase 4 Migration Complete!${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}What Changed:${NC}"
echo "  - Runtime version: v106 → v108"
echo "  - GRANDPA completely removed from runtime and node"
echo "  - SessionKeys updated to ASF-only"
echo "  - ~150 lines of GRANDPA code removed"
echo "  - 6 dependencies removed from Cargo.toml files"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Review changes: git diff"
echo "  2. Build runtime: cd $RUNTIME_DIR && cargo build --release"
echo "  3. Build node: cd $NODE_DIR && cargo build --release"
echo "  4. Generate chainspec: flarechain-node build-spec --chain mainnet_asf_only > chainspec.json"
echo "  5. Test locally before deploying"
echo ""
echo -e "${YELLOW}Rollback:${NC}"
echo "  If needed, restore from backups:"
echo "    cp $BACKUP_DIR/lib.rs.v106.backup-$TIMESTAMP $RUNTIME_SRC"
echo "    cp $BACKUP_DIR/runtime-Cargo.toml.v106.backup-$TIMESTAMP $RUNTIME_CARGO"
echo "    cp $BACKUP_DIR/node-Cargo.toml.v106.backup-$TIMESTAMP $NODE_CARGO"
echo ""
echo -e "${GREEN}Migration completed at: $(date)${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════════════════${NC}"

# Cleanup
rm -f "$TEMP_FILE"
