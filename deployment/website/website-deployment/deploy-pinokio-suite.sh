#!/bin/bash

###############################################################################
# ËTRID Pinokio Suite Deployment Script
# Deploys all web UIs, validator monitoring, and API services
###############################################################################

set -e

echo "🚀 ËTRID Pinokio Suite Deployment"
echo "=================================="
echo ""

# Configuration
PROJECT_ROOT="/Users/macbook/Desktop/etrid"
WEBSITE_DIR="$PROJECT_ROOT/deployment/website/website-deployment/website"
API_DIR="$PROJECT_ROOT/deployment/website/website-deployment/api"
PINOKIO_DIR="$PROJECT_ROOT/pinokio"

# Color output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Install Pinokio dependencies
echo -e "${BLUE}📦 Step 1: Installing Pinokio dependencies...${NC}"
cd "$PINOKIO_DIR"
if [ ! -d "node_modules" ]; then
    npm install
    echo -e "${GREEN}✅ Pinokio dependencies installed${NC}"
else
    echo -e "${GREEN}✅ Pinokio dependencies already installed${NC}"
fi
echo ""

# Step 2: Install API dependencies
echo -e "${BLUE}📦 Step 2: Installing API dependencies...${NC}"
cd "$API_DIR"
if [ ! -d "node_modules" ]; then
    npm install
    echo -e "${GREEN}✅ API dependencies installed${NC}"
else
    echo -e "${GREEN}✅ API dependencies already installed${NC}"
fi
echo ""

# Step 3: Build all web UIs
echo -e "${BLUE}🏗️  Step 3: Building web UIs...${NC}"
cd "$PROJECT_ROOT"
if [ -f "scripts/build-all-web-uis.sh" ]; then
    chmod +x scripts/build-all-web-uis.sh
    ./scripts/build-all-web-uis.sh
    echo -e "${GREEN}✅ Web UIs built successfully${NC}"
else
    echo -e "${YELLOW}⚠️  build-all-web-uis.sh not found, skipping web UI build${NC}"
fi
echo ""

# Step 4: Create reports directory
echo -e "${BLUE}📁 Step 4: Creating reports directory...${NC}"
mkdir -p "$PINOKIO_DIR/reports"
echo -e "${GREEN}✅ Reports directory created${NC}"
echo ""

# Step 5: Check validator configuration
echo -e "${BLUE}🔍 Step 5: Checking validator configuration...${NC}"
VALIDATOR_CONFIG="$PROJECT_ROOT/infrastructure/config/validator-ips.json"
if [ -f "$VALIDATOR_CONFIG" ]; then
    echo -e "${GREEN}✅ Validator configuration found${NC}"
    VALIDATOR_COUNT=$(cat "$VALIDATOR_CONFIG" | grep -o '"id"' | wc -l)
    echo "   Found $VALIDATOR_COUNT validators configured"
else
    echo -e "${YELLOW}⚠️  Validator configuration not found at $VALIDATOR_CONFIG${NC}"
    echo "   Creating sample configuration..."
    mkdir -p "$PROJECT_ROOT/infrastructure/config"
    cat > "$VALIDATOR_CONFIG" << 'EOF'
{
  "validators": [
    {
      "id": 1,
      "name": "Sample Validator",
      "region": "Local",
      "role": "Developer",
      "ip": "127.0.0.1",
      "sshUser": "user",
      "accessible": false
    }
  ]
}
EOF
    echo -e "${GREEN}✅ Sample configuration created${NC}"
fi
echo ""

# Step 6: Check SSH configuration
echo -e "${BLUE}🔐 Step 6: Checking SSH configuration...${NC}"
if [ -n "$SSH_KEY_PATH" ]; then
    if [ -f "$SSH_KEY_PATH" ]; then
        echo -e "${GREEN}✅ SSH key found at $SSH_KEY_PATH${NC}"
    else
        echo -e "${YELLOW}⚠️  SSH key not found at $SSH_KEY_PATH${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  SSH_KEY_PATH not set${NC}"
    echo "   Set it with: export SSH_KEY_PATH=~/.ssh/your-key"
fi
echo ""

# Step 7: Generate initial validator report
echo -e "${BLUE}🤖 Step 7: Generating initial validator report...${NC}"
cd "$PINOKIO_DIR"
if [ -f "$SSH_KEY_PATH" ]; then
    echo "Running AI validator monitoring..."
    timeout 120 node ai-validator-monitor.js monitor || echo -e "${YELLOW}⚠️  Monitoring timed out or failed (normal if no SSH access)${NC}"
else
    echo -e "${YELLOW}⚠️  Skipping monitoring (no SSH key configured)${NC}"
fi
echo ""

# Step 8: Display deployment summary
echo ""
echo "=================================="
echo -e "${GREEN}✅ DEPLOYMENT COMPLETE${NC}"
echo "=================================="
echo ""
echo "📊 Deployment Summary:"
echo ""
echo "🌐 Web Applications (available at etrid.org):"
echo "   • Lightning Landing      → /lightning/"
echo "   • MasterChef Dashboard   → http://localhost:3001"
echo "   • Validator Monitor      → /validator-monitor/"
echo "   • Watchtower Monitor     → http://localhost:3003"
echo "   • Wallet Web             → http://localhost:3004"
echo ""
echo "🔧 Management Tools:"
echo "   • Validator CLI          → cd pinokio && node validator-cli.js"
echo "   • AI Monitor             → cd pinokio && node ai-validator-monitor.js"
echo "   • API Server             → cd deployment/website/website-deployment/api && npm start"
echo ""
echo "📝 Quick Commands:"
echo "   Start Web UIs:   ./scripts/start-all-web-uis.sh"
echo "   Stop Web UIs:    ./scripts/stop-all-web-uis.sh"
echo "   Check Status:    ./scripts/status-web-uis.sh"
echo "   List Validators: cd pinokio && npm run validator:list"
echo "   AI Monitoring:   cd pinokio && npm run validator:monitor"
echo ""
echo "🚀 To start the API server:"
echo "   cd deployment/website/website-deployment/api"
echo "   npm start"
echo ""
echo -e "${BLUE}📖 Documentation: pinokio/README.md${NC}"
echo ""
