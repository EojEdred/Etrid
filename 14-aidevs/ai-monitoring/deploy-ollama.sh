#!/usr/bin/env bash
# Deploy Ollama to Monitoring Server (VM #10: 98.71.91.84)
# This enables Tier 1 AI (free, local, fast) for validator monitoring

set -e

MONITORING_SERVER="compiler-dev01@98.71.91.84"
SSH_KEY="$HOME/.ssh/gizzi-validator"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "🤖 Deploying Ollama AI to Monitoring Server"
echo "============================================="
echo ""
echo -e "${BLUE}Target: $MONITORING_SERVER${NC}"
echo ""

# Test SSH connection
echo -e "${YELLOW}1️⃣  Testing SSH connection...${NC}"
if ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
    "$MONITORING_SERVER" "echo 'SSH OK'" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ SSH connection successful${NC}"
else
    echo -e "${RED}❌ SSH connection failed${NC}"
    exit 1
fi
echo ""

# Install Ollama
echo -e "${YELLOW}2️⃣  Installing Ollama...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'INSTALL_SCRIPT'
set -e

# Check if Ollama already installed
if command -v ollama &> /dev/null; then
    echo "✓ Ollama already installed"
    exit 0
fi

# Install Ollama
echo "Downloading and installing Ollama..."
curl -fsSL https://ollama.com/install.sh | sh

# Verify installation
if command -v ollama &> /dev/null; then
    echo "✓ Ollama installed successfully"
else
    echo "✗ Ollama installation failed"
    exit 1
fi
INSTALL_SCRIPT

echo -e "${GREEN}✅ Ollama installed${NC}"
echo ""

# Start Ollama service
echo -e "${YELLOW}3️⃣  Starting Ollama service...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'START_SERVICE'
set -e

# Start Ollama service
sudo systemctl enable ollama 2>/dev/null || echo "Enabling via systemd..."
sudo systemctl start ollama 2>/dev/null || echo "Starting service..."

# Wait for service
sleep 5

# Check if running
if systemctl is-active --quiet ollama 2>/dev/null; then
    echo "✓ Ollama service is running"
else
    echo "⚠ Starting Ollama manually..."
    nohup ollama serve > /var/log/ollama.log 2>&1 &
    sleep 3
fi
START_SERVICE

echo -e "${GREEN}✅ Ollama service started${NC}"
echo ""

# Pull AI model
echo -e "${YELLOW}4️⃣  Pulling Llama 3.2 model (4.7GB - may take 5-10 minutes)...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'PULL_MODEL'
# Pull llama3.2 model
echo "Downloading llama3.2 model..."
ollama pull llama3.2:latest

echo ""
echo "Installed models:"
ollama list
PULL_MODEL

echo -e "${GREEN}✅ Llama 3.2 model downloaded${NC}"
echo ""

# Test Ollama
echo -e "${YELLOW}5️⃣  Testing Ollama...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" \
    'ollama run llama3.2:latest "Say hello in exactly 3 words"' 2>&1 | head -3

echo -e "${GREEN}✅ Ollama working${NC}"
echo ""

echo "============================================="
echo -e "${GREEN}🎉 Ollama Deployment Complete!${NC}"
echo "============================================="
echo ""
echo "API Endpoint: http://98.71.91.84:11434"
echo "Model: llama3.2:latest"
echo "Cost: $0/month (free local AI)"
echo ""

