#!/bin/bash
# FlareChain Binary Verification Script
# Verifies both macOS and Linux binaries are correctly built

set -e

echo "================================================================"
echo "FlareChain Binary Verification"
echo "================================================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if file exists
check_file() {
    local file=$1
    if [ -f "$file" ]; then
        echo -e "${GREEN}✅ Found: $file${NC}"
        return 0
    else
        echo -e "${RED}❌ Missing: $file${NC}"
        return 1
    fi
}

# Function to check file size
check_size() {
    local file=$1
    local min_size=$2  # in MB
    local size=$(du -m "$file" | cut -f1)

    if [ $size -gt $min_size ]; then
        echo -e "${GREEN}✅ Size: ${size}MB (> ${min_size}MB)${NC}"
        return 0
    else
        echo -e "${RED}❌ Size: ${size}MB (expected > ${min_size}MB)${NC}"
        return 1
    fi
}

# Check macOS binary
echo "=== macOS ARM64 Binary ==="
if check_file "macos-arm64/flarechain-node"; then
    check_size "macos-arm64/flarechain-node" 50

    # Check architecture
    arch=$(file macos-arm64/flarechain-node | grep -o "arm64\|x86_64\|Mach-O")
    if [[ "$arch" == *"arm64"* ]] || [[ "$arch" == *"Mach-O"* ]]; then
        echo -e "${GREEN}✅ Architecture: $arch${NC}"
    else
        echo -e "${RED}❌ Wrong architecture: $arch (expected arm64)${NC}"
    fi

    # Check if executable
    if [ -x "macos-arm64/flarechain-node" ]; then
        echo -e "${GREEN}✅ Executable permissions set${NC}"
    else
        echo -e "${YELLOW}⚠️  Not executable, fixing...${NC}"
        chmod +x macos-arm64/flarechain-node
    fi

    # Try to run version check (only on macOS)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo -n "Testing execution: "
        if version=$(./macos-arm64/flarechain-node --version 2>&1 | head -1); then
            echo -e "${GREEN}✅ $version${NC}"
        else
            echo -e "${RED}❌ Failed to execute${NC}"
        fi
    else
        echo -e "${YELLOW}⚠️  Skipping execution test (not on macOS)${NC}"
    fi
fi
echo ""

# Check Linux binary
echo "=== Linux x86_64 Binary ==="
if check_file "linux-x86_64/flarechain-node"; then
    check_size "linux-x86_64/flarechain-node" 50

    # Check architecture
    arch=$(file linux-x86_64/flarechain-node)
    if [[ "$arch" == *"x86-64"* ]] || [[ "$arch" == *"x86_64"* ]]; then
        echo -e "${GREEN}✅ Architecture: x86-64 Linux${NC}"
    else
        echo -e "${RED}❌ Wrong architecture: $arch (expected x86-64)${NC}"
    fi

    # Check if executable
    if [ -x "linux-x86_64/flarechain-node" ]; then
        echo -e "${GREEN}✅ Executable permissions set${NC}"
    else
        echo -e "${YELLOW}⚠️  Not executable, fixing...${NC}"
        chmod +x linux-x86_64/flarechain-node
    fi

    # Can't execute Linux binary on macOS, but can check ELF format
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if file linux-x86_64/flarechain-node | grep -q "ELF"; then
            echo -e "${GREEN}✅ Valid ELF executable${NC}"
        else
            echo -e "${RED}❌ Not a valid ELF executable${NC}"
        fi
    else
        # On Linux, try to run it
        echo -n "Testing execution: "
        if version=$(./linux-x86_64/flarechain-node --version 2>&1 | head -1); then
            echo -e "${GREEN}✅ $version${NC}"
        else
            echo -e "${RED}❌ Failed to execute${NC}"
        fi
    fi
fi
echo ""

# Check README exists
echo "=== Documentation ==="
check_file "README.md"
check_file "DEPLOYMENT_GUIDE.md"
echo ""

# Summary
echo "================================================================"
echo "Verification Complete"
echo "================================================================"
echo ""
echo "Next Steps:"
echo "1. Transfer Linux binary to your VMs:"
echo "   scp linux-x86_64/flarechain-node ubuntu@98.71.91.84:~/"
echo ""
echo "2. Load session keys:"
echo "   source ../secrets/.env.mainnet"
echo ""
echo "3. Start validators and insert keys"
echo "   See DEPLOYMENT_GUIDE.md for complete instructions"
echo ""
echo "================================================================"
