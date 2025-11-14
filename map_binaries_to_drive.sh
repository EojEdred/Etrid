#!/bin/bash
# Map Etrid Built Binaries to External Drive
# This script moves build outputs to your external drive using symlinks
# Your laptop stays clean, builds go to external storage

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
EXTERNAL_DRIVE="/Volumes/NO NAME"
PROJECT_DIR="$HOME/Desktop/etrid"
BUILD_DIR="$EXTERNAL_DRIVE/etrid-builds"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Map Etrid Binaries to External Drive${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if external drive is mounted
if [ ! -d "$EXTERNAL_DRIVE" ]; then
    echo -e "${RED}ERROR: External drive not found at $EXTERNAL_DRIVE${NC}"
    echo "Please mount your external drive and try again."
    exit 1
fi

echo -e "${GREEN}✓ External drive found${NC}"
df -h "$EXTERNAL_DRIVE" | tail -1

echo ""
echo -e "${YELLOW}This script will:${NC}"
echo "  1. Create build directories on external drive"
echo "  2. Move existing binaries to external drive"
echo "  3. Create symlinks so cargo builds to external drive"
echo "  4. Move cargo cache to external drive"
echo "  5. Move npm cache to external drive"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Create build directory structure on external drive
echo ""
echo -e "${GREEN}=== Setting up external drive structure ===${NC}"

mkdir -p "$BUILD_DIR/cargo-home"
mkdir -p "$BUILD_DIR/etrid-target"
mkdir -p "$BUILD_DIR/npm-cache"
mkdir -p "$BUILD_DIR/binaries"
mkdir -p "$BUILD_DIR/multichain-targets"

echo -e "${GREEN}✓ Directory structure created${NC}"

# Move existing binaries to external drive
echo ""
echo -e "${GREEN}=== Moving existing binaries ===${NC}"

if [ -d "$HOME/Desktop/etrid-binaries" ]; then
    echo "Moving etrid-binaries to external drive..."
    mv "$HOME/Desktop/etrid-binaries/"* "$BUILD_DIR/binaries/" 2>/dev/null || true
    rmdir "$HOME/Desktop/etrid-binaries" 2>/dev/null || true
    ln -s "$BUILD_DIR/binaries" "$HOME/Desktop/etrid-binaries"
    echo -e "${GREEN}✓ Binaries moved and symlinked${NC}"
else
    echo -e "${BLUE}No existing binaries directory found${NC}"
fi

# Setup Cargo home on external drive
echo ""
echo -e "${GREEN}=== Setting up Cargo cache on external drive ===${NC}"

if [ -d "$HOME/.cargo/registry" ]; then
    echo "Moving cargo registry to external drive..."
    mv "$HOME/.cargo/registry" "$BUILD_DIR/cargo-home/" 2>/dev/null || true
    ln -s "$BUILD_DIR/cargo-home/registry" "$HOME/.cargo/registry"
    echo -e "${GREEN}✓ Cargo registry moved${NC}"
fi

if [ -d "$HOME/.cargo/git" ]; then
    echo "Moving cargo git cache to external drive..."
    mv "$HOME/.cargo/git" "$BUILD_DIR/cargo-home/" 2>/dev/null || true
    ln -s "$BUILD_DIR/cargo-home/git" "$HOME/.cargo/git"
    echo -e "${GREEN}✓ Cargo git cache moved${NC}"
fi

# Setup main target directory
echo ""
echo -e "${GREEN}=== Setting up main target directory ===${NC}"

if [ -d "$PROJECT_DIR/target" ]; then
    echo "Moving main target directory..."
    rm -rf "$PROJECT_DIR/target"
fi

ln -s "$BUILD_DIR/etrid-target" "$PROJECT_DIR/target"
echo -e "${GREEN}✓ Main target → external drive${NC}"

# Setup multichain target directories
echo ""
echo -e "${GREEN}=== Setting up multichain target directories ===${NC}"

# List of multichain modules that have Cargo projects
MULTICHAIN_MODULES=(
    "05-multichain/bridge-protocols/btc-bridge"
    "05-multichain/bridge-protocols/eth-bridge"
    "05-multichain/bridge-protocols/sol-bridge"
    "05-multichain/bridge-protocols/xrp-bridge"
    "05-multichain/bridge-protocols/bnb-bridge"
    "05-multichain/bridge-protocols/trx-bridge"
    "05-multichain/bridge-protocols/sc-usdt-bridge"
    "05-multichain/pbc-collators/btc-pbc-collator"
    "05-multichain/pbc-collators/eth-pbc-collator"
    "05-multichain/pbc-collators/sol-pbc-collator"
    "05-multichain/pbc-collators/xrp-pbc-collator"
    "05-multichain/pbc-collators/bnb-pbc-collator"
    "05-multichain/pbc-collators/trx-pbc-collator"
    "05-multichain/pbc-collators/sc-usdt-pbc-collator"
    "05-multichain/pbc-runtimes/btc-pbc-runtime"
    "05-multichain/pbc-runtimes/eth-pbc-runtime"
    "05-multichain/pbc-runtimes/sol-pbc-runtime"
    "05-multichain/pbc-runtimes/xrp-pbc-runtime"
    "05-multichain/pbc-runtimes/bnb-pbc-runtime"
    "05-multichain/pbc-runtimes/trx-pbc-runtime"
    "05-multichain/pbc-runtimes/sc-usdt-pbc-runtime"
)

for module in "${MULTICHAIN_MODULES[@]}"; do
    MODULE_PATH="$PROJECT_DIR/$module"
    if [ -d "$MODULE_PATH" ]; then
        MODULE_NAME=$(basename "$module")

        # Create external target directory for this module
        mkdir -p "$BUILD_DIR/multichain-targets/$MODULE_NAME"

        # Remove existing target if present
        if [ -d "$MODULE_PATH/target" ]; then
            rm -rf "$MODULE_PATH/target"
        fi

        # Create symlink
        ln -s "$BUILD_DIR/multichain-targets/$MODULE_NAME" "$MODULE_PATH/target"
        echo -e "${GREEN}✓ $MODULE_NAME/target → external drive${NC}"
    fi
done

# Setup npm cache on external drive
echo ""
echo -e "${GREEN}=== Setting up npm cache ===${NC}"

# Configure npm to use external cache (preserves node_modules in place)
npm config set cache "$BUILD_DIR/npm-cache"
echo -e "${GREEN}✓ npm cache → external drive${NC}"

# Create a restore script
echo ""
echo -e "${GREEN}=== Creating restore script ===${NC}"

cat > "$HOME/Desktop/etrid/restore_from_drive.sh" << 'EOF'
#!/bin/bash
# Restore script - if you need to work without external drive
# This removes symlinks and restores local directories

set -e

echo "Restoring local build directories..."

# Remove symlinks
rm -f "$HOME/Desktop/etrid-binaries"
rm -f "$HOME/.cargo/registry"
rm -f "$HOME/.cargo/git"
rm -f "$HOME/Desktop/etrid/target"

# Restore directories
mkdir -p "$HOME/Desktop/etrid-binaries"
mkdir -p "$HOME/.cargo/registry"
mkdir -p "$HOME/.cargo/git"
mkdir -p "$HOME/Desktop/etrid/target"

# Reset npm cache
npm config delete cache

echo "✓ Restored to local directories"
echo "Note: You'll need to rebuild (cargo build)"
EOF

chmod +x "$HOME/Desktop/etrid/restore_from_drive.sh"
echo -e "${GREEN}✓ Restore script created${NC}"

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Setup Complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${GREEN}Build directories now mapped:${NC}"
echo "  • Main target/     → $BUILD_DIR/etrid-target"
echo "  • Cargo cache      → $BUILD_DIR/cargo-home"
echo "  • npm cache        → $BUILD_DIR/npm-cache"
echo "  • Binaries         → $BUILD_DIR/binaries"
echo "  • Multichain builds → $BUILD_DIR/multichain-targets"
echo ""
echo -e "${YELLOW}Important notes:${NC}"
echo "  • External drive MUST be mounted to build"
echo "  • Builds will be slightly slower (external USB)"
echo "  • Your laptop SSD stays clean"
echo "  • node_modules stay in project (fast access)"
echo ""
echo -e "${GREEN}Next steps:${NC}"
echo "  1. Build a project: cd $PROJECT_DIR && cargo build --release"
echo "  2. Binaries will appear in: $BUILD_DIR/binaries"
echo "  3. To restore local builds: bash restore_from_drive.sh"
echo ""
echo -e "${BLUE}Disk usage on external drive:${NC}"
df -h "$EXTERNAL_DRIVE"
echo ""
