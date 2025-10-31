#!/usr/bin/env bash
# Activate AI Monitoring System on Monitoring Server
# This makes the 3-tier AI system live

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
MONITORING_SERVER="compiler-dev01@98.71.91.84"
AI_DIR="/opt/ai-monitoring"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "🤖 Activating AI Monitoring System"
echo "===================================="
echo ""

# Step 1: Deploy AI monitoring code
echo -e "${YELLOW}1️⃣  Deploying AI monitoring code to server...${NC}"
scp -i "$SSH_KEY" -o StrictHostKeyChecking=no -r \
    ~/Desktop/etrid/ai-monitoring \
    "$MONITORING_SERVER:/tmp/"

ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" \
    "sudo mkdir -p $AI_DIR && \
     sudo cp -r /tmp/ai-monitoring/* $AI_DIR/ && \
     sudo chmod -R 755 $AI_DIR"

echo -e "${GREEN}✅ Code deployed${NC}"
echo ""

# Step 2: Install dependencies
echo -e "${YELLOW}2️⃣  Installing Python dependencies...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'INSTALL'
set -e

# Update package list
sudo apt update -qq

# Install pip if needed
if ! command -v pip3 &> /dev/null; then
    sudo apt install -y python3-pip
fi

# Install required packages
pip3 install --user anthropic openai requests python-dotenv 2>/dev/null || \
    sudo pip3 install anthropic openai requests python-dotenv

echo "✓ Dependencies installed"
INSTALL

echo -e "${GREEN}✅ Dependencies installed${NC}"
echo ""

# Step 3: Create GLOBAL_MEMORY file
echo -e "${YELLOW}3️⃣  Creating GLOBAL_MEMORY file...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'MEMORY'
set -e

sudo mkdir -p /opt/ai-monitoring
sudo touch /opt/ai-monitoring/GLOBAL_MEMORY.md
sudo chmod 666 /opt/ai-monitoring/GLOBAL_MEMORY.md

# Initialize with header
sudo tee /opt/ai-monitoring/GLOBAL_MEMORY.md > /dev/null <<'EOF'
# Ëtrid FlareChain - AI Monitoring Global Memory
**Network:** Ëtrid FlareChain Mainnet
**AI System:** 3-Tier (Ollama → GPT-4 → Claude)
**Started:** $(date -u +"%Y-%m-%d %H:%M UTC")
**Validators:** 21 total

This file logs all AI monitoring decisions and actions.

---

EOF

echo "✓ GLOBAL_MEMORY created"
MEMORY

echo -e "${GREEN}✅ GLOBAL_MEMORY ready${NC}"
echo ""

# Step 4: Start AI monitoring
echo -e "${YELLOW}4️⃣  Starting AI monitoring system...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'START'
set -e

cd /opt/ai-monitoring

# Kill any existing orchestrator
pkill -f orchestrator.py 2>/dev/null || true

# Start new orchestrator in background
nohup python3 orchestrator.py > /var/log/ai-monitoring.log 2>&1 &

# Wait a moment for it to start
sleep 3

# Check if running
if pgrep -f orchestrator.py > /dev/null; then
    echo "✓ AI monitoring started (PID: $(pgrep -f orchestrator.py))"
else
    echo "✗ Failed to start AI monitoring"
    exit 1
fi
START

echo -e "${GREEN}✅ AI monitoring active${NC}"
echo ""

# Step 5: Verify
echo -e "${YELLOW}5️⃣  Verifying system...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'VERIFY'
echo "Process status:"
ps aux | grep orchestrator | grep -v grep

echo ""
echo "Recent logs:"
tail -10 /var/log/ai-monitoring.log 2>/dev/null || echo "No logs yet (starting up...)"
VERIFY

echo ""
echo "===================================="
echo -e "${GREEN}🎉 AI Monitoring System ACTIVATED!${NC}"
echo "===================================="
echo ""
echo "Monitor AI decisions:"
echo "  ssh -i ~/.ssh/gizzi-validator $MONITORING_SERVER 'tail -f $AI_DIR/GLOBAL_MEMORY.md'"
echo ""
echo "View logs:"
echo "  ssh -i ~/.ssh/gizzi-validator $MONITORING_SERVER 'tail -f /var/log/ai-monitoring.log'"
echo ""
echo "Check status:"
echo "  ssh -i ~/.ssh/gizzi-validator $MONITORING_SERVER 'ps aux | grep orchestrator'"
echo ""
echo "Stop monitoring:"
echo "  ssh -i ~/.ssh/gizzi-validator $MONITORING_SERVER 'pkill -f orchestrator.py'"
echo ""
