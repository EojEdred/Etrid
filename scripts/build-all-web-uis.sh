#!/bin/bash

###############################################################################
# Build All ËTRID Web UIs
# Builds production bundles for all applications
###############################################################################

set -e

echo "🏗️  Building ËTRID Web UIs"
echo "=========================="
echo ""

# Project root
PROJECT_ROOT="/Users/macbook/Desktop/etrid"

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Array of applications
declare -a APPS=(
    "lightning-landing"
    "masterchef-dashboard"
    "validator-dashboard"
    "watchtower-monitor"
    "wallet-web/etrid-crypto-website"
)

# Function to build an app
build_app() {
    local app_path=$1
    local app_name=$(basename $(dirname $app_path))

    if [ "$app_name" = "etrid-crypto-website" ]; then
        app_name="wallet-web"
    fi

    echo -e "${BLUE}Building ${app_name}...${NC}"

    cd "$PROJECT_ROOT/apps/$app_path"

    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        echo -e "${YELLOW}   Installing dependencies...${NC}"
        if [[ "$app_path" == *"wallet-web"* ]]; then
            npm install --legacy-peer-deps
        else
            npm install
        fi
    fi

    # Build
    echo "   Building production bundle..."
    if npm run build; then
        echo -e "${GREEN}✅ Build successful${NC}"

        # Show build output location
        if [ -d "out" ]; then
            echo -e "   Output: apps/$app_path/out"
        elif [ -d ".next" ]; then
            echo -e "   Output: apps/$app_path/.next"
        elif [ -d "dist" ]; then
            echo -e "   Output: apps/$app_path/dist"
        fi

        return 0
    else
        echo -e "${RED}❌ Build failed${NC}"
        return 1
    fi

    echo ""
}

# Build all applications
success_count=0
for app_path in "${APPS[@]}"; do
    if build_app "$app_path"; then
        ((success_count++))
    fi
done

echo "=========================="
echo -e "${GREEN}Built $success_count/${#APPS[@]} applications${NC}"
echo ""

if [ $success_count -eq ${#APPS[@]} ]; then
    echo "✨ All builds successful!"
    echo ""
    echo "Next steps:"
    echo "  Start dev:        ./scripts/start-all-web-uis.sh"
    echo "  Check status:     ./scripts/status-web-uis.sh"
else
    echo "⚠️  Some builds failed. Check errors above."
fi
echo ""
