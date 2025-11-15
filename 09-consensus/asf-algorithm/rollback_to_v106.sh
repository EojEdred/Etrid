#!/bin/bash
# ════════════════════════════════════════════════════════════════════════════
# ËTRID Phase 4: Rollback to v106 (GRANDPA Restoration)
# ════════════════════════════════════════════════════════════════════════════
#
# Purpose: Automatically restore runtime v106 with GRANDPA from backups
# Use when: Phase 4 migration needs to be reverted
#
# ════════════════════════════════════════════════════════════════════════════

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Paths
BACKUP_DIR="/Users/macbook/Desktop/etrid/09-consensus/asf-algorithm/phase4-backups"
RUNTIME_DIR="/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime"
NODE_DIR="/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node"

# Banner
echo -e "${RED}════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${RED}  ËTRID Phase 4: ROLLBACK TO v106                                          ${NC}"
echo -e "${RED}  WARNING: This will restore GRANDPA consensus                             ${NC}"
echo -e "${RED}════════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# Step 1: Find backups
echo -e "${YELLOW}Step 1: Locating Backup Files${NC}"
echo "------------------------------"

# List available backups
if [ ! -d "$BACKUP_DIR" ]; then
    echo -e "${RED}ERROR: Backup directory not found: $BACKUP_DIR${NC}"
    echo -e "${YELLOW}Backups may not have been created yet.${NC}"
    exit 1
fi

echo "Available backups:"
ls -lt "$BACKUP_DIR" | grep "v106"
echo ""

# Find most recent backup
LATEST_LIB=$(ls -t "$BACKUP_DIR"/lib.rs.v106.backup-* 2>/dev/null | head -1)
LATEST_RUNTIME_CARGO=$(ls -t "$BACKUP_DIR"/runtime-Cargo.toml.v106.backup-* 2>/dev/null | head -1)
LATEST_NODE_CARGO=$(ls -t "$BACKUP_DIR"/node-Cargo.toml.v106.backup-* 2>/dev/null | head -1)

if [ -z "$LATEST_LIB" ] || [ -z "$LATEST_RUNTIME_CARGO" ] || [ -z "$LATEST_NODE_CARGO" ]; then
    echo -e "${RED}ERROR: Could not find all required backup files${NC}"
    exit 1
fi

echo -e "${GREEN}Found backups:${NC}"
echo -e "  lib.rs: ${BLUE}$(basename "$LATEST_LIB")${NC}"
echo -e "  Runtime Cargo.toml: ${BLUE}$(basename "$LATEST_RUNTIME_CARGO")${NC}"
echo -e "  Node Cargo.toml: ${BLUE}$(basename "$LATEST_NODE_CARGO")${NC}"
echo ""

# Extract timestamp from backup filename
TIMESTAMP=$(basename "$LATEST_LIB" | sed 's/lib.rs.v106.backup-//')
echo -e "Backup timestamp: ${BLUE}$TIMESTAMP${NC}"
echo ""

# Confirmation
echo -e "${YELLOW}This will:${NC}"
echo "  1. Restore runtime lib.rs from v106 backup"
echo "  2. Restore runtime Cargo.toml with GRANDPA dependencies"
echo "  3. Restore node Cargo.toml with GRANDPA dependencies"
echo "  4. Revert spec_version from 108 → 106"
echo "  5. Restore GRANDPA finality gadget"
echo "  6. Clean and rebuild runtime and node"
echo ""
echo -e "${RED}WARNING: This will undo all Phase 4 changes!${NC}"
echo ""
read -p "Continue with rollback? (yes/no): " CONFIRM

if [ "$CONFIRM" != "yes" ]; then
    echo -e "${RED}Rollback cancelled by user${NC}"
    exit 0
fi
echo ""

# Step 2: Create safety backup of current state
echo -e "${YELLOW}Step 2: Backing Up Current State (v108)${NC}"
echo "----------------------------------------"

SAFETY_DIR="$BACKUP_DIR/pre-rollback-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$SAFETY_DIR"

cp "$RUNTIME_DIR/src/lib.rs" "$SAFETY_DIR/lib.rs.v108"
cp "$RUNTIME_DIR/Cargo.toml" "$SAFETY_DIR/runtime-Cargo.toml.v108"
cp "$NODE_DIR/Cargo.toml" "$SAFETY_DIR/node-Cargo.toml.v108"

echo -e "${GREEN}✓ Current v108 state backed up to: $SAFETY_DIR${NC}"
echo ""

# Step 3: Restore files
echo -e "${YELLOW}Step 3: Restoring v106 Files${NC}"
echo "-----------------------------"

# Restore lib.rs
cp "$LATEST_LIB" "$RUNTIME_DIR/src/lib.rs"
echo -e "${GREEN}✓ Restored lib.rs${NC}"

# Restore runtime Cargo.toml
cp "$LATEST_RUNTIME_CARGO" "$RUNTIME_DIR/Cargo.toml"
echo -e "${GREEN}✓ Restored runtime Cargo.toml${NC}"

# Restore node Cargo.toml
cp "$LATEST_NODE_CARGO" "$NODE_DIR/Cargo.toml"
echo -e "${GREEN}✓ Restored node Cargo.toml${NC}"

echo ""

# Step 4: Verify restoration
echo -e "${YELLOW}Step 4: Verifying Restoration${NC}"
echo "-------------------------------"

# Check runtime version
CURRENT_VERSION=$(grep "spec_version:" "$RUNTIME_DIR/src/lib.rs" | head -1 | grep -o '[0-9]*')
echo -e "Runtime version: ${BLUE}$CURRENT_VERSION${NC}"

if [ "$CURRENT_VERSION" != "106" ]; then
    echo -e "${RED}ERROR: Runtime version is not 106!${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Runtime version is 106${NC}"

# Check GRANDPA in runtime
GRANDPA_RUNTIME=$(grep -c "pallet-grandpa" "$RUNTIME_DIR/Cargo.toml" || true)
echo -e "GRANDPA references in runtime Cargo.toml: ${BLUE}$GRANDPA_RUNTIME${NC}"

if [ "$GRANDPA_RUNTIME" -eq 0 ]; then
    echo -e "${RED}ERROR: GRANDPA not found in runtime Cargo.toml${NC}"
    exit 1
fi
echo -e "${GREEN}✓ GRANDPA dependencies present in runtime${NC}"

# Check GRANDPA in node
GRANDPA_NODE=$(grep -c "grandpa" "$NODE_DIR/Cargo.toml" || true)
echo -e "GRANDPA references in node Cargo.toml: ${BLUE}$GRANDPA_NODE${NC}"

if [ "$GRANDPA_NODE" -eq 0 ]; then
    echo -e "${RED}ERROR: GRANDPA not found in node Cargo.toml${NC}"
    exit 1
fi
echo -e "${GREEN}✓ GRANDPA dependencies present in node${NC}"

# Check SessionKeys
if grep -q "pub grandpa: Grandpa," "$RUNTIME_DIR/src/lib.rs"; then
    echo -e "${GREEN}✓ SessionKeys includes GRANDPA${NC}"
else
    echo -e "${RED}ERROR: SessionKeys missing GRANDPA${NC}"
    exit 1
fi

echo ""

# Step 5: Clean build artifacts
echo -e "${YELLOW}Step 5: Cleaning Build Artifacts${NC}"
echo "---------------------------------"

cd "$RUNTIME_DIR"
cargo clean
echo -e "${GREEN}✓ Runtime build artifacts cleaned${NC}"

cd "$NODE_DIR"
cargo clean
echo -e "${GREEN}✓ Node build artifacts cleaned${NC}"

echo ""

# Step 6: Rebuild
echo -e "${YELLOW}Step 6: Rebuilding Runtime and Node${NC}"
echo "------------------------------------"

echo -e "${BLUE}Building runtime...${NC}"
cd "$RUNTIME_DIR"
if cargo build --release; then
    echo -e "${GREEN}✓ Runtime build successful${NC}"
else
    echo -e "${RED}✗ Runtime build failed${NC}"
    echo -e "${YELLOW}Check errors above. You may need to manually fix issues.${NC}"
    exit 1
fi
echo ""

echo -e "${BLUE}Building node...${NC}"
cd "$NODE_DIR"
if cargo build --release; then
    echo -e "${GREEN}✓ Node build successful${NC}"
else
    echo -e "${RED}✗ Node build failed${NC}"
    echo -e "${YELLOW}Check errors above. You may need to manually fix issues.${NC}"
    exit 1
fi
echo ""

# Step 7: Final verification
echo -e "${YELLOW}Step 7: Final Verification${NC}"
echo "---------------------------"

# Check binary exists
if [ -f "$NODE_DIR/target/release/flarechain-node" ]; then
    echo -e "${GREEN}✓ Node binary exists${NC}"

    # Get runtime version from binary
    BINARY_VERSION=$("$NODE_DIR/target/release/flarechain-node" --version 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' | head -1 || echo "unknown")
    echo -e "Binary version: ${BLUE}$BINARY_VERSION${NC}"
else
    echo -e "${RED}ERROR: Node binary not found${NC}"
    exit 1
fi

echo ""

# Step 8: Summary
echo -e "${GREEN}════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Rollback to v106 Complete!${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Restoration Summary:${NC}"
echo "  - Runtime version: v108 → v106"
echo "  - GRANDPA finality restored"
echo "  - SessionKeys includes GRANDPA"
echo "  - All dependencies restored"
echo "  - Build successful"
echo ""
echo -e "${YELLOW}Backups:${NC}"
echo "  - v106 restored from: $BACKUP_DIR"
echo "  - v108 safety backup: $SAFETY_DIR"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Test the v106 binary:"
echo "     cd $NODE_DIR"
echo "     ./target/release/flarechain-node --dev --tmp"
echo ""
echo "  2. Verify GRANDPA finality:"
echo "     # Check logs for GRANDPA messages"
echo "     # Should see finality reports"
echo ""
echo "  3. If deploying to production:"
echo "     - Copy binary to validators"
echo "     - Restart validator services"
echo "     - Monitor GRANDPA finalization"
echo ""
echo "  4. Investigate why Phase 4 needed rollback"
echo "     - Review error logs"
echo "     - Fix issues before retrying"
echo ""
echo -e "${GREEN}Rollback completed successfully at: $(date)${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════════════════════════════${NC}"
