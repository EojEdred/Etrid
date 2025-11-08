#!/bin/bash

# ÉTRID Web UI Suite - Build All Applications
# This script builds all web UIs

set -e

echo "=========================================="
echo "ÉTRID Web UI Suite - Building All Apps"
echo "=========================================="
echo ""

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}Project Root: $PROJECT_ROOT${NC}"
echo ""

# Track build status
BUILD_SUCCESS=0
BUILD_FAILED=0
FAILED_APPS=()

# Function to build an app
build_app() {
    local app_name=$1
    local app_path=$2
    local extra_flags=$3

    echo ""
    echo "=========================================="
    echo -e "${YELLOW}Building $app_name...${NC}"
    echo "=========================================="
    cd "$PROJECT_ROOT/$app_path"

    # Install dependencies
    echo "Installing dependencies..."
    if [ -n "$extra_flags" ]; then
        if npm install $extra_flags; then
            echo -e "${GREEN}✓ Dependencies installed${NC}"
        else
            echo -e "${RED}✗ Failed to install dependencies${NC}"
            BUILD_FAILED=$((BUILD_FAILED + 1))
            FAILED_APPS+=("$app_name")
            return 1
        fi
    else
        if npm install; then
            echo -e "${GREEN}✓ Dependencies installed${NC}"
        else
            echo -e "${RED}✗ Failed to install dependencies${NC}"
            BUILD_FAILED=$((BUILD_FAILED + 1))
            FAILED_APPS+=("$app_name")
            return 1
        fi
    fi

    # Build the application
    echo "Building application..."
    if npm run build; then
        echo -e "${GREEN}✓ $app_name built successfully${NC}"
        BUILD_SUCCESS=$((BUILD_SUCCESS + 1))
    else
        echo -e "${RED}✗ $app_name build failed${NC}"
        BUILD_FAILED=$((BUILD_FAILED + 1))
        FAILED_APPS+=("$app_name")
        return 1
    fi
}

# Build all applications
build_app "Lightning Landing" "apps/lightning-landing" ""
build_app "MasterChef Dashboard" "apps/masterchef-dashboard" ""
build_app "Validator Dashboard" "apps/validator-dashboard" ""
build_app "Watchtower Monitor" "apps/watchtower-monitor" ""
build_app "Wallet Web" "apps/wallet-web/etrid-crypto-website" "--legacy-peer-deps"

echo ""
echo "=========================================="
echo "Build Summary"
echo "=========================================="
echo -e "${GREEN}Successful builds: $BUILD_SUCCESS${NC}"
echo -e "${RED}Failed builds: $BUILD_FAILED${NC}"

if [ $BUILD_FAILED -gt 0 ]; then
    echo ""
    echo -e "${RED}Failed applications:${NC}"
    for app in "${FAILED_APPS[@]}"; do
        echo "  - $app"
    done
    echo ""
    exit 1
else
    echo ""
    echo -e "${GREEN}All applications built successfully!${NC}"
    echo ""
    echo "To start all applications, run:"
    echo "  ./scripts/start-all-web-uis.sh"
    echo ""
fi
