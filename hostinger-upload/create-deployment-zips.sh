#!/bin/bash
set -e

echo "========================================="
echo "Creating Deployment Packages (ZIPs)"
echo "========================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

cd /Users/macbook/Desktop/etrid/hostinger-upload

# 1. Validator Dashboard
echo -e "${BLUE}[1/6] Creating validator-dual-node.zip...${NC}"
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/validator-dual-node.zip . > /dev/null 2>&1
echo -e "${GREEN}✅ validator-dual-node.zip created${NC}"

# 2. Governance UI
echo -e "${BLUE}[2/6] Creating governance-dual-node.zip...${NC}"
cd /Users/macbook/Desktop/etrid/apps/governance-ui/etrid-snapshot/dist
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/governance-dual-node.zip . > /dev/null 2>&1
echo -e "${GREEN}✅ governance-dual-node.zip created${NC}"

# 3. Wallet
echo -e "${BLUE}[3/6] Creating wallet-dual-node.zip...${NC}"
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/wallet-dual-node.zip . > /dev/null 2>&1
echo -e "${GREEN}✅ wallet-dual-node.zip created${NC}"

# 4. Watchtower
echo -e "${BLUE}[4/6] Creating watchtower-dual-node.zip...${NC}"
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/watchtower-dual-node.zip . > /dev/null 2>&1
echo -e "${GREEN}✅ watchtower-dual-node.zip created${NC}"

# 5. MasterChef
echo -e "${BLUE}[5/6] Creating masterchef-dual-node.zip...${NC}"
cd /Users/macbook/Desktop/etrid/apps/masterchef-dashboard/out
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/masterchef-dual-node.zip . > /dev/null 2>&1
echo -e "${GREEN}✅ masterchef-dual-node.zip created${NC}"

# 6. Documentation Portal
echo -e "${BLUE}[6/6] Creating docs-portal.zip...${NC}"
cd /Users/macbook/Desktop/etrid/hostinger-upload/docs-portal
zip -r /Users/macbook/Desktop/etrid/hostinger-upload/docs-portal.zip . > /dev/null 2>&1
echo -e "${GREEN}✅ docs-portal.zip created${NC}"

echo ""
echo "========================================="
echo -e "${GREEN}✅ ALL DEPLOYMENT PACKAGES CREATED!${NC}"
echo "========================================="
echo ""
echo "Created ZIPs:"
echo "  1. validator-dual-node.zip"
echo "  2. governance-dual-node.zip"
echo "  3. wallet-dual-node.zip"
echo "  4. watchtower-dual-node.zip (fake data removed!)"
echo "  5. masterchef-dual-node.zip"
echo "  6. docs-portal.zip"
echo ""
echo "All ZIPs are in: /Users/macbook/Desktop/etrid/hostinger-upload/"
echo ""
echo "Next step: Upload to Hostinger!"
echo ""
