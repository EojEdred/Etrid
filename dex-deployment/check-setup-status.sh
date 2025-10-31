#!/bin/bash
################################################################################
# SETUP STATUS CHECKER
#
# Checks the status of all chain configurations
# Shows what's ready and what needs to be done
################################################################################

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}"
cat << 'EOF'
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║     ËTRID DEX DEPLOYMENT - SETUP STATUS CHECK            ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}\n"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

check_chain() {
    local chain=$1
    local chain_name=$2

    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}$chain_name${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Check if directory exists
    if [ ! -d "$chain" ]; then
        echo -e "  ${RED}✗${NC} Directory not found"
        return
    fi

    # Check .env file
    if [ -f "$chain/.env" ]; then
        echo -e "  ${GREEN}✓${NC} .env file exists"

        # Check if PRIVATE_KEY is set
        if grep -q "PRIVATE_KEY=your_private_key_here" "$chain/.env" 2>/dev/null; then
            echo -e "  ${YELLOW}⚠${NC}  PRIVATE_KEY not set (still placeholder)"
        elif grep -q "PRIVATE_KEY=0x" "$chain/.env" 2>/dev/null; then
            echo -e "  ${GREEN}✓${NC} PRIVATE_KEY configured"
        else
            echo -e "  ${YELLOW}⚠${NC}  PRIVATE_KEY may not be set correctly"
        fi
    else
        echo -e "  ${RED}✗${NC} .env file missing"
    fi

    # Check node_modules
    if [ -d "$chain/node_modules" ]; then
        echo -e "  ${GREEN}✓${NC} npm dependencies installed"
    else
        echo -e "  ${RED}✗${NC} npm dependencies not installed"
    fi

    # Check contract file
    if [ -f "$chain/Etrid"*".sol" ]; then
        echo -e "  ${GREEN}✓${NC} Contract file exists"
    else
        echo -e "  ${RED}✗${NC} Contract file missing"
    fi

    # Check hardhat config
    if [ -f "$chain/hardhat.config.js" ]; then
        echo -e "  ${GREEN}✓${NC} Hardhat config exists"
    else
        echo -e "  ${RED}✗${NC} Hardhat config missing"
    fi

    echo ""
}

# Check all chains
check_chain "solana" "SOLANA"
check_chain "bsc" "BSC (BINANCE SMART CHAIN)"
check_chain "polygon" "POLYGON"
check_chain "ethereum" "ETHEREUM"
check_chain "base" "BASE"
check_chain "arbitrum" "ARBITRUM"
check_chain "hyperliquid" "HYPERLIQUID"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  SUMMARY                                                   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

# Count ready chains
READY_COUNT=0
TOTAL_COUNT=7

for chain in solana bsc polygon ethereum base arbitrum hyperliquid; do
    if [ -f "$chain/.env" ] && [ -d "$chain/node_modules" ]; then
        # Check if private key is set
        if ! grep -q "PRIVATE_KEY=your_private_key_here" "$chain/.env" 2>/dev/null; then
            ((READY_COUNT++))
        fi
    fi
done

echo -e "Chains fully configured: ${GREEN}$READY_COUNT${NC} / $TOTAL_COUNT"
echo ""

if [ $READY_COUNT -eq 0 ]; then
    echo -e "${YELLOW}⚠️  No chains are fully configured yet${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Add your PRIVATE_KEY to .env files"
    echo "2. Get gas tokens for each chain"
    echo "3. Run deployment scripts"
    echo ""
    echo -e "Read: ${BLUE}ENV_SETUP_COMPLETE_GUIDE.md${NC}"
elif [ $READY_COUNT -lt $TOTAL_COUNT ]; then
    echo -e "${YELLOW}⚠️  Some chains need configuration${NC}"
    echo ""
    echo "Chains that need PRIVATE_KEY:"
    for chain in solana bsc polygon ethereum base arbitrum hyperliquid; do
        if [ -f "$chain/.env" ]; then
            if grep -q "PRIVATE_KEY=your_private_key_here" "$chain/.env" 2>/dev/null; then
                echo "  - $chain"
            fi
        fi
    done
    echo ""
    echo -e "Read: ${BLUE}ENV_SETUP_COMPLETE_GUIDE.md${NC}"
else
    echo -e "${GREEN}✅ All chains are configured!${NC}"
    echo ""
    echo "You're ready to deploy!"
    echo ""
    echo "Next steps:"
    echo "1. Make sure you have gas tokens on each chain"
    echo "2. Run deployment scripts"
    echo "3. Create pools on DEXes"
    echo ""
    echo -e "Deploy with: ${GREEN}./DEPLOY_BULLX_HYPERLIQUID.sh${NC}"
fi

echo ""
echo -e "${BLUE}💡 Tip:${NC} Run this script anytime to check setup status"
echo ""
