#!/bin/bash
echo "=========================================="
echo "COPYING 6 DOWNLOADED REPOS TO NEW STRUCTURE"
echo "=========================================="
echo ""

NEW_DIR="$(pwd)/etrid-reorganized"

if [ ! -d "$NEW_DIR" ]; then
    echo "ERROR: etrid-reorganized folder not found!"
    exit 1
fi

echo "Copying framework repos..."
echo ""

# FRAMEWORK REPOS (2 repos)
if [ -d "framework/substrate-core" ]; then
    echo "1. Extracting Substrate Core..."
    
    # Copy entire substrate-core for reference
    echo "   - Copying full substrate-core to pallets/substrate-pallets/"
    sudo cp -r framework/substrate-core "$NEW_DIR/pallets/substrate-pallets/substrate-core-full" 2>/dev/null || cp -r framework/substrate-core "$NEW_DIR/pallets/substrate-pallets/substrate-core-full"
    
    # Extract key pallets
    if [ -d "framework/substrate-core/frame" ]; then
        for pallet in balances timestamp system session staking transaction-payment; do
            if [ -d "framework/substrate-core/frame/$pallet" ]; then
                echo "   - Extracting pallet: $pallet"
                sudo cp -r "framework/substrate-core/frame/$pallet" "$NEW_DIR/pallets/substrate-pallets/" 2>/dev/null || cp -r "framework/substrate-core/frame/$pallet" "$NEW_DIR/pallets/substrate-pallets/"
            fi
        done
    fi
    
    # Copy primitives
    if [ -d "framework/substrate-core/primitives" ]; then
        echo "   - Copying primitives"
        mkdir -p "$NEW_DIR/runtime/primitives/substrate-ref"
        sudo cp -r framework/substrate-core/primitives/* "$NEW_DIR/runtime/primitives/substrate-ref/" 2>/dev/null || cp -r framework/substrate-core/primitives/* "$NEW_DIR/runtime/primitives/substrate-ref/"
    fi
    
    echo "   ✓ Substrate Core copied ($(find "$NEW_DIR/pallets/substrate-pallets" -type f | wc -l) files)"
else
    echo "   ✗ framework/substrate-core NOT FOUND!"
fi

echo ""

if [ -d "framework/cosmos-core" ]; then
    echo "2. Extracting Cosmos Core..."
    
    # Copy entire cosmos-core for reference
    echo "   - Copying full cosmos-core to pallets/cosmos-modules/"
    sudo cp -r framework/cosmos-core "$NEW_DIR/pallets/cosmos-modules/cosmos-core-full" 2>/dev/null || cp -r framework/cosmos-core "$NEW_DIR/pallets/cosmos-modules/cosmos-core-full"
    
    # Extract key modules
    if [ -d "framework/cosmos-core/x" ]; then
        for module in distribution gov staking bank; do
            if [ -d "framework/cosmos-core/x/$module" ]; then
                echo "   - Extracting module: $module"
                sudo cp -r "framework/cosmos-core/x/$module" "$NEW_DIR/pallets/cosmos-modules/" 2>/dev/null || cp -r "framework/cosmos-core/x/$module" "$NEW_DIR/pallets/cosmos-modules/"
            fi
        done
    fi
    
    # Copy types
    if [ -d "framework/cosmos-core/types" ]; then
        echo "   - Copying types"
        mkdir -p "$NEW_DIR/runtime/primitives/cosmos-ref"
        sudo cp -r framework/cosmos-core/types/* "$NEW_DIR/runtime/primitives/cosmos-ref/" 2>/dev/null || cp -r framework/cosmos-core/types/* "$NEW_DIR/runtime/primitives/cosmos-ref/"
    fi
    
    echo "   ✓ Cosmos Core copied ($(find "$NEW_DIR/pallets/cosmos-modules" -type f | wc -l) files)"
else
    echo "   ✗ framework/cosmos-core NOT FOUND!"
fi

echo ""
echo "Copying frontend repos..."
echo ""

# FRONTEND REPOS (4 repos)
if [ -d "frontend/et-ethers" ]; then
    echo "3. Copying et-ethers → client/etrid-js..."
    sudo cp -r frontend/et-ethers/* "$NEW_DIR/client/etrid-js/" 2>/dev/null || cp -r frontend/et-ethers/* "$NEW_DIR/client/etrid-js/"
    
    # Update package name
    if [ -f "$NEW_DIR/client/etrid-js/package.json" ]; then
        sudo sed -i.bak 's/"name": "[^"]*"/"name": "@etrid\/etrid-js"/g' "$NEW_DIR/client/etrid-js/package.json" 2>/dev/null || sed -i.bak 's/"name": "[^"]*"/"name": "@etrid\/etrid-js"/g' "$NEW_DIR/client/etrid-js/package.json"
        sudo rm "$NEW_DIR/client/etrid-js/package.json.bak" 2>/dev/null || rm "$NEW_DIR/client/etrid-js/package.json.bak"
    fi
    
    echo "   ✓ et-ethers copied ($(find "$NEW_DIR/client/etrid-js" -type f | wc -l) files)"
else
    echo "   ✗ frontend/et-ethers NOT FOUND!"
fi

echo ""

if [ -d "frontend/et-voting-ui" ]; then
    echo "4. Copying et-voting-ui → apps/governance-ui..."
    sudo cp -r frontend/et-voting-ui/* "$NEW_DIR/apps/governance-ui/" 2>/dev/null || cp -r frontend/et-voting-ui/* "$NEW_DIR/apps/governance-ui/"
    
    # Create .env.example
    cat > "$NEW_DIR/apps/governance-ui/.env.example" << 'EOF'
VITE_APP_NAME=Ëtrid Governance
VITE_RPC_URL=ws://localhost:9944
VITE_CHAIN_ID=etrid-testnet
VITE_TOKEN_SYMBOL=ÉTR
VITE_TOKEN_DECIMALS=12
VITE_CONSENSUS_DAY_DATE=2026-01-01T00:00:00Z
EOF
    
    echo "   ✓ et-voting-ui copied ($(find "$NEW_DIR/apps/governance-ui" -type f | wc -l) files)"
else
    echo "   ✗ frontend/et-voting-ui NOT FOUND!"
fi

echo ""

if [ -d "frontend/et-wallet-connector" ]; then
    echo "5. Copying et-wallet-connector → apps/wallet-web/src/services..."
    mkdir -p "$NEW_DIR/apps/wallet-web/src/services"
    sudo cp -r frontend/et-wallet-connector/* "$NEW_DIR/apps/wallet-web/src/services/" 2>/dev/null || cp -r frontend/et-wallet-connector/* "$NEW_DIR/apps/wallet-web/src/services/"
    echo "   ✓ et-wallet-connector copied ($(find "$NEW_DIR/apps/wallet-web/src/services" -type f | wc -l) files)"
else
    echo "   ✗ frontend/et-wallet-connector NOT FOUND!"
fi

echo ""

if [ -d "frontend/et-wallet-ios" ]; then
    echo "6. Copying et-wallet-ios → client/etrid-swift..."
    sudo cp -r frontend/et-wallet-ios/* "$NEW_DIR/client/etrid-swift/" 2>/dev/null || cp -r frontend/et-wallet-ios/* "$NEW_DIR/client/etrid-swift/"
    
    # Also copy iOS app if exists
    if [ -d "frontend/et-wallet-ios/app" ]; then
        echo "   - Copying iOS app to apps/wallet-mobile/ios"
        mkdir -p "$NEW_DIR/apps/wallet-mobile/ios"
        sudo cp -r frontend/et-wallet-ios/app/* "$NEW_DIR/apps/wallet-mobile/ios/" 2>/dev/null || cp -r frontend/et-wallet-ios/app/* "$NEW_DIR/apps/wallet-mobile/ios/"
    fi
    
    echo "   ✓ et-wallet-ios copied ($(find "$NEW_DIR/client/etrid-swift" -type f | wc -l) files)"
else
    echo "   ✗ frontend/et-wallet-ios NOT FOUND!"
fi

echo ""
echo "=========================================="
echo "FINAL VERIFICATION"
echo "=========================================="
echo ""
echo "6 Downloaded Repos Status:"
echo ""
echo "FRAMEWORK (2 repos):"
echo "  1. substrate-core: $(find "$NEW_DIR/pallets/substrate-pallets" -type f | wc -l) files"
echo "  2. cosmos-core:    $(find "$NEW_DIR/pallets/cosmos-modules" -type f | wc -l) files"
echo ""
echo "FRONTEND (4 repos):"
echo "  3. et-ethers:      $(find "$NEW_DIR/client/etrid-js" -type f | wc -l) files"
echo "  4. et-voting-ui:   $(find "$NEW_DIR/apps/governance-ui" -type f | wc -l) files"
echo "  5. et-wallet-conn: $(find "$NEW_DIR/apps/wallet-web/src/services" -type f 2>/dev/null | wc -l) files"
echo "  6. et-wallet-ios:  $(find "$NEW_DIR/client/etrid-swift" -type f | wc -l) files"
echo ""
echo "=========================================="
echo "✅ DONE!"
echo "=========================================="
echo ""
echo "Next: cd etrid-reorganized && ls -la"
echo ""
