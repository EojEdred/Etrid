#!/bin/bash

# Deployment script for all Ëtrid Protocol UI apps
# This script deploys all three apps to Vercel

set -e  # Exit on error

echo "======================================"
echo "Ëtrid Protocol - Deploy All UI Apps"
echo "======================================"
echo ""

# Check if logged in to Vercel
if ! vercel whoami > /dev/null 2>&1; then
    echo "❌ Not logged in to Vercel. Please run 'vercel login' first."
    exit 1
fi

echo "✅ Vercel authentication confirmed"
echo ""

# Deploy Wallet-Web (Production)
echo "======================================"
echo "1/3: Deploying Wallet-Web (Production)"
echo "======================================"
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
echo "📍 Directory: $(pwd)"
echo "🚀 Deploying to production..."
vercel --prod
echo "✅ Wallet-Web deployed"
echo ""

# Deploy Watchtower Monitor (Staging)
echo "======================================"
echo "2/3: Deploying Watchtower Monitor (Staging)"
echo "======================================"
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
echo "📍 Directory: $(pwd)"
echo "🚀 Deploying to staging..."
vercel
echo "✅ Watchtower Monitor deployed"
echo ""

# Deploy Validator Dashboard (Staging)
echo "======================================"
echo "3/3: Deploying Validator Dashboard (Staging)"
echo "======================================"
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
echo "📍 Directory: $(pwd)"
echo "🚀 Deploying to staging..."
vercel
echo "✅ Validator Dashboard deployed"
echo ""

echo "======================================"
echo "✅ ALL DEPLOYMENTS COMPLETE"
echo "======================================"
echo ""
echo "Next steps:"
echo "1. Test each deployed URL"
echo "2. Verify dark mode toggle (wallet-web)"
echo "3. Verify WebSocket connections (watchtower & validator)"
echo "4. Check security headers"
echo ""
