#!/bin/bash
set -e

echo "================================================"
echo "ËTRID Apps - Building All Apps with Dual Nodes"
echo "================================================"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 1. Validator Dashboard
echo -e "${BLUE}[1/5] Building Validator Dashboard...${NC}"
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
npm run build
echo -e "${GREEN}✅ Validator Dashboard built successfully${NC}"
echo ""

# 2. Governance UI
echo -e "${BLUE}[2/5] Building Governance UI...${NC}"
cd /Users/macbook/Desktop/etrid/apps/governance-ui/etrid-snapshot
npm install
npm run build
echo -e "${GREEN}✅ Governance UI built successfully${NC}"
echo ""

# 3. Wallet
echo -e "${BLUE}[3/5] Building Wallet...${NC}"
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
npm install
npm run build
echo -e "${GREEN}✅ Wallet built successfully${NC}"
echo ""

# 4. Watchtower
echo -e "${BLUE}[4/5] Building Watchtower (fake data removed!)...${NC}"
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
npm run build
echo -e "${GREEN}✅ Watchtower built successfully${NC}"
echo ""

# 5. MasterChef
echo -e "${BLUE}[5/5] Building MasterChef...${NC}"
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard
npm run build
echo -e "${GREEN}✅ MasterChef built successfully${NC}"
echo ""

echo "================================================"
echo -e "${GREEN}✅ ALL APPS BUILT SUCCESSFULLY!${NC}"
echo "================================================"
echo ""
echo "Next steps:"
echo "1. Run ./create-deployment-zips.sh to create deployment packages"
echo "2. Upload ZIPs to Hostinger"
echo "3. Open both Azure firewalls (port 9944 to 0.0.0.0/0)"
echo ""
