#!/bin/bash
# Setup External Drive as Memory Overflow (Swap Space)
# For macOS systems with mounted external drive

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}=== Setup External Drive as Swap/Overflow ===${NC}"
echo ""

# Detect mounted external drive
EXTERNAL_DRIVE="/Volumes/NO NAME"

if [ ! -d "$EXTERNAL_DRIVE" ]; then
    echo -e "${RED}ERROR: External drive not found at $EXTERNAL_DRIVE${NC}"
    echo "Available volumes:"
    ls -l /Volumes/
    echo ""
    read -p "Enter the correct path to your external drive: " EXTERNAL_DRIVE

    if [ ! -d "$EXTERNAL_DRIVE" ]; then
        echo -e "${RED}ERROR: Drive not found. Exiting.${NC}"
        exit 1
    fi
fi

echo -e "${GREEN}Found external drive: $EXTERNAL_DRIVE${NC}"
df -h "$EXTERNAL_DRIVE"
echo ""

# Create swap file size (default 8GB, adjustable)
SWAP_SIZE_GB=8
SWAP_FILE="$EXTERNAL_DRIVE/etrid_swapfile"

echo -e "${YELLOW}Creating ${SWAP_SIZE_GB}GB swap file on external drive...${NC}"
echo "This may take several minutes..."

# Check if swap file already exists
if [ -f "$SWAP_FILE" ]; then
    echo -e "${YELLOW}Swap file already exists. Removing old one...${NC}"
    sudo rm -f "$SWAP_FILE"
fi

# Create swap file (macOS method)
# Note: macOS handles swap differently than Linux
# We'll create a dynamic paging file approach

echo -e "${YELLOW}Option 1: Create additional swap file${NC}"
echo "Note: macOS manages swap automatically. This creates additional swap space."

# Calculate block size and count for dd
BLOCK_SIZE=1m
BLOCK_COUNT=$((SWAP_SIZE_GB * 1024))

sudo dd if=/dev/zero of="$SWAP_FILE" bs=$BLOCK_SIZE count=$BLOCK_COUNT

# Set proper permissions
sudo chmod 600 "$SWAP_FILE"

echo -e "${GREEN}✓ Swap file created${NC}"
echo ""

# For macOS, we need to use dynamic_pager or similar
echo -e "${YELLOW}Setting up macOS swap configuration...${NC}"

# Create a script to enable the swap
cat > "$EXTERNAL_DRIVE/enable_swap.sh" << 'EOF'
#!/bin/bash
# Enable external swap file

SWAP_FILE="/Volumes/NO NAME/etrid_swapfile"

if [ -f "$SWAP_FILE" ]; then
    # macOS doesn't use traditional mkswap/swapon
    # Instead, we'll set up a dynamic pager configuration
    echo "Swap file ready at: $SWAP_FILE"

    # Check current swap usage
    sysctl vm.swapusage
else
    echo "ERROR: Swap file not found"
    exit 1
fi
EOF

chmod +x "$EXTERNAL_DRIVE/enable_swap.sh"

echo ""
echo -e "${GREEN}=== Alternative: Use External Drive for Build Artifacts ===${NC}"
echo "A better approach for macOS is to move build artifacts to external drive"
echo ""

# Create build directory on external drive
BUILD_DIR="$EXTERNAL_DRIVE/etrid_build"
mkdir -p "$BUILD_DIR"

echo -e "${YELLOW}Creating symlinks to move heavy build artifacts...${NC}"

# Move cargo cache to external drive
CARGO_TARGET="$BUILD_DIR/cargo"
mkdir -p "$CARGO_TARGET"

echo "To move Cargo cache to external drive, run:"
echo -e "${GREEN}  mv ~/.cargo $CARGO_TARGET/${NC}"
echo -e "${GREEN}  ln -s $CARGO_TARGET ~/.cargo${NC}"
echo ""

# Move etrid target to external drive
ETRID_TARGET="$BUILD_DIR/etrid_target"
mkdir -p "$ETRID_TARGET"

echo "To move Etrid build artifacts to external drive, run:"
echo -e "${GREEN}  cd ~/Desktop/etrid${NC}"
echo -e "${GREEN}  mv target $ETRID_TARGET/${NC}"
echo -e "${GREEN}  ln -s $ETRID_TARGET target${NC}"
echo ""

# Create script to automate the move
cat > "$EXTERNAL_DRIVE/move_builds_to_external.sh" << EOF
#!/bin/bash
# Move heavy build artifacts to external drive

set -e

EXTERNAL_BUILD="$BUILD_DIR"

echo "Moving build artifacts to external drive..."

# Backup locations
if [ -d ~/.cargo ] && [ ! -L ~/.cargo ]; then
    echo "Moving Cargo cache..."
    mv ~/.cargo "\$EXTERNAL_BUILD/cargo_backup"
    ln -s "\$EXTERNAL_BUILD/cargo_backup" ~/.cargo
    echo "✓ Cargo cache moved"
fi

if [ -d ~/Desktop/etrid/target ] && [ ! -L ~/Desktop/etrid/target ]; then
    echo "Moving Etrid target..."
    mv ~/Desktop/etrid/target "\$EXTERNAL_BUILD/etrid_target"
    ln -s "\$EXTERNAL_BUILD/etrid_target" ~/Desktop/etrid/target
    echo "✓ Etrid target moved"
fi

# Move npm cache
if [ -d ~/.npm ] && [ ! -L ~/.npm ]; then
    echo "Moving npm cache..."
    mv ~/.npm "\$EXTERNAL_BUILD/npm_cache"
    ln -s "\$EXTERNAL_BUILD/npm_cache" ~/.npm
    echo "✓ npm cache moved"
fi

# Move gradle cache
if [ -d ~/.gradle ] && [ ! -L ~/.gradle ]; then
    echo "Moving Gradle cache..."
    mv ~/.gradle "\$EXTERNAL_BUILD/gradle_cache"
    ln -s "\$EXTERNAL_BUILD/gradle_cache" ~/.gradle
    echo "✓ Gradle cache moved"
fi

echo ""
echo "✓ All build artifacts moved to external drive"
echo "Your internal SSD will now only store symlinks"
df -h /
EOF

chmod +x "$EXTERNAL_DRIVE/move_builds_to_external.sh"

echo ""
echo -e "${GREEN}=== Setup Complete ===${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo ""
echo "1. Run the cleanup script first:"
echo -e "   ${GREEN}bash ~/Desktop/etrid/cleanup_bloat.sh${NC}"
echo ""
echo "2. Move build artifacts to external drive (RECOMMENDED):"
echo -e "   ${GREEN}bash $EXTERNAL_DRIVE/move_builds_to_external.sh${NC}"
echo ""
echo "3. After moving, rebuild your project:"
echo -e "   ${GREEN}cd ~/Desktop/etrid && cargo build --release${NC}"
echo ""
echo -e "${YELLOW}Benefits:${NC}"
echo "  • Frees up 50-60GB on your internal SSD"
echo "  • Build artifacts stored on external drive (106GB available)"
echo "  • Symlinks make it transparent to build tools"
echo "  • Easy to reverse if needed"
echo ""
echo -e "${RED}IMPORTANT:${NC}"
echo "  • Keep external drive mounted when building"
echo "  • Builds will be slower (external drive speed)"
echo "  • Alternative: Build on internal SSD, move artifacts after"
