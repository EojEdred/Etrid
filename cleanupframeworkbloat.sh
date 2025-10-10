R#!/bin/bash
# cleanup-framework-bloat.sh
# Removes unnecessary files from downloaded framework repos

set -e

echo "================================================"
echo "ETRID FRAMEWORK CLEANUP"
echo "Removing bloat from downloaded repos"
echo "================================================"
echo ""

cd /Users/macbook/Desktop/Etrid-Clean/etrid-reorganized

# Get initial size
INITIAL_SIZE=$(du -sh . 2>/dev/null | cut -f1)
echo "Initial size: $INITIAL_SIZE"
echo ""

# ================================================
# PHASE 1: Clean Substrate
# ================================================
echo "PHASE 1: Cleaning Substrate bloat..."

if [ -d "pallets/substrate-pallets" ]; then
    cd pallets/substrate-pallets
    
    # Delete full substrate-core if copied
    if [ -d "substrate-core-full" ]; then
        echo "  Removing substrate-core-full (huge!)..."
        rm -rf substrate-core-full/
    fi
    
    if [ -d "substrate-core" ]; then
        echo "  Removing substrate-core (huge!)..."
        rm -rf substrate-core/
    fi
    
    # Keep only the 6 essential pallets
    echo "  Keeping only essential pallets..."
    KEEP_PALLETS="balances timestamp system session staking transaction-payment"
    
    for dir in */; do
        dirname="${dir%/}"
        if [[ ! " $KEEP_PALLETS " =~ " $dirname " ]]; then
            echo "    Removing: $dirname"
            rm -rf "$dirname"
        else
            echo "    ✓ Keeping: $dirname"
        fi
    done
    
    cd ../..
    echo "  ✓ Substrate cleaned"
else
    echo "  ⚠ pallets/substrate-pallets not found"
fi

echo ""

# ================================================
# PHASE 2: Clean Cosmos
# ================================================
echo "PHASE 2: Cleaning Cosmos bloat..."

if [ -d "pallets/cosmos-modules" ]; then
    cd pallets/cosmos-modules
    
    # Delete full cosmos-core if copied
    if [ -d "cosmos-core-full" ]; then
        echo "  Removing cosmos-core-full (huge!)..."
        rm -rf cosmos-core-full/
    fi
    
    if [ -d "cosmos-core" ]; then
        echo "  Removing cosmos-core (huge!)..."
        rm -rf cosmos-core/
    fi
    
    # Keep only the 4 essential modules
    echo "  Keeping only essential modules..."
    KEEP_MODULES="distribution gov staking bank"
    
    for dir in */; do
        dirname="${dir%/}"
        if [[ ! " $KEEP_MODULES " =~ " $dirname " ]]; then
            echo "    Removing: $dirname"
            rm -rf "$dirname"
        else
            echo "    ✓ Keeping: $dirname"
        fi
    done
    
    cd ../..
    echo "  ✓ Cosmos cleaned"
else
    echo "  ⚠ pallets/cosmos-modules not found"
fi

echo ""

# ================================================
# PHASE 3: Clean Tests & Examples
# ================================================
echo "PHASE 3: Removing tests and examples..."

# Remove test directories
find . -type d -name "tests" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "test" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "__tests__" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "test-utils" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "testing" -prune -exec rm -rf {} \; 2>/dev/null

# Remove example directories (but keep in client SDKs)
find pallets/ -type d -name "examples" -prune -exec rm -rf {} \; 2>/dev/null
find runtime/ -type d -name "examples" -prune -exec rm -rf {} \; 2>/dev/null

# Remove benchmark directories
find . -type d -name "benches" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "benchmarking" -prune -exec rm -rf {} \; 2>/dev/null

echo "  ✓ Tests and examples removed"
echo ""

# ================================================
# PHASE 4: Clean CI/CD & Docs
# ================================================
echo "PHASE 4: Removing CI/CD and extra docs..."

# Remove .github from subfolders (keep root .github)
find client/ -type d -name ".github" -prune -exec rm -rf {} \; 2>/dev/null
find apps/ -type d -name ".github" -prune -exec rm -rf {} \; 2>/dev/null
find pallets/ -type d -name ".github" -prune -exec rm -rf {} \; 2>/dev/null

# Remove docs folders from subfolders (keep root docs)
find client/ -type d -name "docs" -prune -exec rm -rf {} \; 2>/dev/null
find pallets/ -type d -name "docs" -prune -exec rm -rf {} \; 2>/dev/null

# Remove .circleci, .travis, .gitlab
find . -type d -name ".circleci" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name ".travis" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name ".gitlab" -prune -exec rm -rf {} \; 2>/dev/null

echo "  ✓ CI/CD removed"
echo ""

# ================================================
# PHASE 5: Clean Build Artifacts
# ================================================
echo "PHASE 5: Removing build artifacts..."

# Remove build directories
find . -type d -name "target" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "node_modules" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "dist" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name "build" -prune -exec rm -rf {} \; 2>/dev/null
find . -type d -name ".next" -prune -exec rm -rf {} \; 2>/dev/null

# Remove lock files (we'll regenerate them)
find . -type f -name "Cargo.lock" -delete 2>/dev/null
find . -type f -name "package-lock.json" -delete 2>/dev/null
find . -type f -name "yarn.lock" -delete 2>/dev/null

echo "  ✓ Build artifacts removed"
echo ""

# ================================================
# PHASE 6: Clean Ethereum-specific code
# ================================================
echo "PHASE 6: Removing Ethereum-specific files..."

# Remove Ethereum addresses and ENS references
find client/ apps/ -type f -name "*ethereum*" -delete 2>/dev/null
find client/ apps/ -type f -name "*ens*" -delete 2>/dev/null
find client/ apps/ -type f -name "*metamask*" -delete 2>/dev/null

echo "  ✓ Ethereum-specific files removed"
echo ""

# ================================================
# FINAL REPORT
# ================================================
echo "================================================"
echo "CLEANUP COMPLETE!"
echo "================================================"
echo ""

FINAL_SIZE=$(du -sh . 2>/dev/null | cut -f1)
echo "Before: $INITIAL_SIZE"
echo "After:  $FINAL_SIZE"
echo ""

echo "What's left:"
echo ""
echo "Pallets:"
if [ -d "pallets/substrate-pallets" ]; then
    echo "  Substrate pallets:"
    ls -1 pallets/substrate-pallets/ 2>/dev/null | head -10
fi
if [ -d "pallets/cosmos-modules" ]; then
    echo "  Cosmos modules:"
    ls -1 pallets/cosmos-modules/ 2>/dev/null | head -10
fi

echo ""
echo "Client SDKs:"
echo "  etrid-js:     $(find client/etrid-js -type f 2>/dev/null | wc -l) files"
echo "  etrid-swift:  $(find client/etrid-swift -type f 2>/dev/null | wc -l) files"

echo ""
echo "Apps:"
echo "  governance-ui: $(find apps/governance-ui -type f 2>/dev/null | wc -l) files"
echo "  wallet-web:    $(find apps/wallet-web -type f 2>/dev/null | wc -l) files"
echo "  wallet-mobile: $(find apps/wallet-mobile -type f 2>/dev/null | wc -l) files"

echo ""
echo "================================================"
echo "✅ Your project is now lean and ready!"
echo "================================================"
echo ""
echo "Next steps:"
echo "  1. Review what's left: ls -la pallets/"
echo "  2. Read INTEGRATION_GUIDE.md for next steps"
echo "  3. Start building custom runtime pallets"
echo ""
