#!/bin/bash

# Deployment script for Vercel

set -e

echo "🚀 Ëtrid Wallet Deployment Script"
echo "=================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Vercel CLI is installed
if ! command -v vercel &> /dev/null; then
    echo -e "${RED}❌ Vercel CLI not found${NC}"
    echo "Install with: npm i -g vercel"
    exit 1
fi

# Function to deploy PWA
deploy_pwa() {
    echo -e "${YELLOW}📱 Deploying PWA...${NC}"
    cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet

    # Check environment variables
    if [ ! -f .env.local ]; then
        echo -e "${YELLOW}⚠️  .env.local not found (OK for production)${NC}"
    fi

    # Build
    echo "Building PWA..."
    npm run build

    # Deploy
    if [ "$1" == "production" ]; then
        echo "Deploying to production..."
        vercel --prod
    else
        echo "Deploying to preview..."
        vercel
    fi

    echo -e "${GREEN}✅ PWA deployed${NC}"
    cd /home/user/Etrid/apps/wallet-mobile
}

# Function to deploy Landing Page
deploy_landing() {
    echo -e "${YELLOW}🎨 Deploying Landing Page...${NC}"
    cd /home/user/Etrid/apps/wallet-mobile/landing-page

    # Build
    echo "Building Landing Page..."
    npm run build

    # Deploy
    if [ "$1" == "production" ]; then
        echo "Deploying to production..."
        vercel --prod
    else
        echo "Deploying to preview..."
        vercel
    fi

    echo -e "${GREEN}✅ Landing Page deployed${NC}"
    cd /home/user/Etrid/apps/wallet-mobile
}

# Main menu
echo "Select deployment target:"
echo "1) PWA (wallet.etrid.com)"
echo "2) Landing Page (www.wallet.etrid.com)"
echo "3) Both"
echo ""
read -p "Choice (1-3): " choice

echo ""
read -p "Deploy to production? (y/N): " prod

if [[ $prod =~ ^[Yy]$ ]]; then
    ENV="production"
    echo -e "${YELLOW}⚠️  Deploying to PRODUCTION${NC}"
else
    ENV="preview"
    echo "Deploying to preview"
fi

echo ""

case $choice in
    1)
        deploy_pwa $ENV
        ;;
    2)
        deploy_landing $ENV
        ;;
    3)
        deploy_pwa $ENV
        deploy_landing $ENV
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}🎉 Deployment complete!${NC}"
