#!/bin/bash

# 🚀 PrimeSwap Launch Sequence
# Complete deployment automation for launch night
# Usage: ./scripts/launch-sequence.sh <network>

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Banner
echo -e "${BLUE}"
echo "═══════════════════════════════════════════════════════════"
echo "  🚀 ËTRID PRIMESWAP LAUNCH SEQUENCE 🚀"
echo "═══════════════════════════════════════════════════════════"
echo -e "${NC}"

# Check arguments
if [ -z "$1" ]; then
    echo -e "${RED}❌ Error: Network not specified${NC}"
    echo ""
    echo "Usage: ./scripts/launch-sequence.sh <network>"
    echo ""
    echo "Available networks:"
    echo "  - localhost    (for testing)"
    echo "  - goerli       (Ethereum testnet)"
    echo "  - sepolia      (Ethereum testnet)"
    echo "  - mainnet      (Ethereum mainnet)"
    echo "  - bsc          (BSC mainnet)"
    echo "  - bsc_testnet  (BSC testnet)"
    exit 1
fi

NETWORK=$1

echo -e "${YELLOW}📡 Target Network: $NETWORK${NC}"
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo -e "${RED}❌ Error: .env file not found${NC}"
    echo ""
    echo "Please create .env file:"
    echo "  cp .env.example .env"
    echo "  nano .env  # Add your PRIVATE_KEY"
    exit 1
fi

echo -e "${GREEN}✅ .env file found${NC}"

# Check if node_modules exists
if [ ! -d node_modules ]; then
    echo -e "${YELLOW}📦 Installing dependencies...${NC}"
    npm install
fi

echo -e "${GREEN}✅ Dependencies installed${NC}"
echo ""

# Step 1: Compile
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}  STEP 1: Compiling Contracts${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

npx hardhat compile

echo ""
echo -e "${GREEN}✅ Compilation successful${NC}"
echo ""

# Prompt to continue
read -p "Continue with deployment? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}⚠️  Deployment cancelled${NC}"
    exit 0
fi

# Step 2: Deploy
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}  STEP 2: Deploying Contracts to $NETWORK${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

npx hardhat run scripts/deploy-full.js --network $NETWORK

echo ""
echo -e "${GREEN}✅ Deployment successful${NC}"
echo ""

# Check if deployment file was created
if [ ! -f deployments-full.json ]; then
    echo -e "${RED}❌ Error: deployments-full.json not created${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Deployment config saved to deployments-full.json${NC}"
echo ""

# Show contract addresses
echo -e "${YELLOW}📋 Contract Addresses:${NC}"
echo ""
cat deployments-full.json
echo ""

# Prompt for liquidity
read -p "Add initial liquidity now? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}⚠️  Skipping liquidity addition${NC}"
    echo ""
    echo "To add liquidity later, run:"
    echo "  npx hardhat run scripts/add-initial-liquidity.js --network $NETWORK"
    exit 0
fi

# Step 3: Add Liquidity
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}  STEP 3: Adding Initial Liquidity (\$750)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

npx hardhat run scripts/add-initial-liquidity.js --network $NETWORK

echo ""
echo -e "${GREEN}✅ Liquidity added successfully${NC}"
echo ""

# Final Summary
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}  🎉 LAUNCH COMPLETE! 🎉${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${GREEN}✅ Contracts deployed to: $NETWORK${NC}"
echo -e "${GREEN}✅ Initial liquidity: \$750${NC}"
echo -e "${GREEN}✅ Trading pools: Live${NC}"
echo ""
echo -e "${YELLOW}📋 Next Steps:${NC}"
echo "  1. Save deployments-full.json securely"
echo "  2. Verify contracts on block explorer"
echo "  3. Update frontend with contract addresses"
echo "  4. Test a swap to confirm functionality"
echo "  5. Announce launch to community"
echo ""
echo -e "${YELLOW}📄 Deployment file: deployments-full.json${NC}"
echo ""

# Prompt for verification
read -p "Verify contracts on block explorer? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  STEP 4: Verifying Contracts${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    npx hardhat run scripts/verify.js --network $NETWORK
    echo ""
    echo -e "${GREEN}✅ Verification complete${NC}"
fi

echo ""
echo -e "${GREEN}🎊 ALL DONE! Your DEX is live! 🎊${NC}"
echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
