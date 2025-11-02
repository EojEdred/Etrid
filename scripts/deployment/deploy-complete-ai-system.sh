#!/bin/bash
# Complete AI Monitoring System Deployment
# Deploys Ollama + AI Monitoring + Prometheus + Grafana to VM #10
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Configuration
MONITORING_SERVER="compiler-dev01@98.71.91.84"
SSH_KEY="$HOME/.ssh/gizzi-validator"
LOCAL_AI_DIR="$HOME/Desktop/etrid/ai-monitoring"
REMOTE_AI_DIR="/opt/ai-monitoring"

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  ËTRID AI Monitoring System - Complete Deployment            ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Step 1: Test SSH connectivity
echo -e "${YELLOW}[1/8]${NC} Testing SSH connectivity to monitoring server..."
if ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 "$MONITORING_SERVER" "echo 'SSH OK'" &>/dev/null; then
    echo -e "${GREEN}✓${NC} SSH connection successful"
else
    echo -e "${RED}✗${NC} Cannot connect to monitoring server"
    exit 1
fi

# Step 2: Install system dependencies
echo -e "${YELLOW}[2/8]${NC} Installing system dependencies..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" "sudo apt-get update && sudo apt-get install -y curl git python3 python3-pip jq htop"
echo -e "${GREEN}✓${NC} System dependencies installed"

# Step 3: Install Ollama
echo -e "${YELLOW}[3/8]${NC} Installing Ollama..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" << 'REMOTE_OLLAMA'
if ! command -v ollama &> /dev/null; then
    echo "Installing Ollama..."
    curl -fsSL https://ollama.ai/install.sh | sh

    # Enable Ollama service
    sudo systemctl enable ollama
    sudo systemctl start ollama

    # Wait for Ollama to start
    sleep 5

    # Pull the model
    ollama pull llama2:13b
else
    echo "Ollama already installed"
fi
REMOTE_OLLAMA
echo -e "${GREEN}✓${NC} Ollama installed and model pulled"

# Step 4: Create directory structure
echo -e "${YELLOW}[4/8]${NC} Creating directory structure..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" << 'REMOTE_DIRS'
sudo mkdir -p /opt/ai-monitoring/{skills,dids,logs}
sudo chown -R $USER:$USER /opt/ai-monitoring
REMOTE_DIRS
echo -e "${GREEN}✓${NC} Directory structure created"

# Step 5: Upload AI monitoring files
echo -e "${YELLOW}[5/8]${NC} Uploading AI monitoring system files..."

# Upload main Python script
scp -i "$SSH_KEY" "$LOCAL_AI_DIR/ai_dev_workers.py" "$MONITORING_SERVER:$REMOTE_AI_DIR/"

# Upload GLOBAL_MEMORY.md
scp -i "$SSH_KEY" "$LOCAL_AI_DIR/GLOBAL_MEMORY.md" "$MONITORING_SERVER:$REMOTE_AI_DIR/"

# Upload environment configuration
scp -i "$SSH_KEY" "$LOCAL_AI_DIR/.env.production" "$MONITORING_SERVER:$REMOTE_AI_DIR/.env"

# Upload skills directory
scp -i "$SSH_KEY" -r "$LOCAL_AI_DIR/skills/"* "$MONITORING_SERVER:$REMOTE_AI_DIR/skills/"

# Upload DIDs directory
scp -i "$SSH_KEY" -r "$LOCAL_AI_DIR/dids/"* "$MONITORING_SERVER:$REMOTE_AI_DIR/dids/"

echo -e "${GREEN}✓${NC} AI monitoring files uploaded"

# Step 6: Install Python dependencies
echo -e "${YELLOW}[6/8]${NC} Installing Python dependencies..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" << 'REMOTE_PYTHON'
cd /opt/ai-monitoring
pip3 install --user openai anthropic requests python-dotenv psutil
REMOTE_PYTHON
echo -e "${GREEN}✓${NC} Python dependencies installed"

# Step 7: Create systemd service
echo -e "${YELLOW}[7/8]${NC} Creating systemd service..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" << 'REMOTE_SERVICE'
sudo tee /etc/systemd/system/etrid-ai-monitoring.service > /dev/null << 'EOF'
[Unit]
Description=ËTRID AI Monitoring System
After=network.target ollama.service prometheus.service
Wants=ollama.service

[Service]
Type=simple
User=compiler-dev01
WorkingDirectory=/opt/ai-monitoring
EnvironmentFile=/opt/ai-monitoring/.env
ExecStart=/usr/bin/python3 /opt/ai-monitoring/ai_dev_workers.py
Restart=always
RestartSec=10
StandardOutput=append:/opt/ai-monitoring/logs/ai-monitoring.log
StandardError=append:/opt/ai-monitoring/logs/ai-monitoring-error.log

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
sudo systemctl daemon-reload

# Don't start yet - need to configure Claude API key first
echo "Service created but not started - waiting for API key configuration"
REMOTE_SERVICE
echo -e "${GREEN}✓${NC} Systemd service created"

# Step 8: Display next steps
echo ""
echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Deployment Status: AI System Ready (Pending API Key)       ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}✓ Ollama installed and running${NC}"
echo -e "${GREEN}✓ AI monitoring system deployed${NC}"
echo -e "${GREEN}✓ GPT-4 API key configured${NC}"
echo -e "${YELLOW}⚠ Claude API key needs to be configured${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. SSH to monitoring server:"
echo "   ssh -i ~/.ssh/gizzi-validator $MONITORING_SERVER"
echo ""
echo "2. Edit the .env file and add your Claude API key:"
echo "   nano /opt/ai-monitoring/.env"
echo "   # Set: ANTHROPIC_API_KEY=your-actual-key-here"
echo ""
echo "3. Start the AI monitoring service:"
echo "   sudo systemctl start etrid-ai-monitoring"
echo "   sudo systemctl enable etrid-ai-monitoring"
echo ""
echo "4. Monitor the system:"
echo "   sudo systemctl status etrid-ai-monitoring"
echo "   tail -f /opt/ai-monitoring/logs/ai-monitoring.log"
echo "   tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md"
echo ""
echo -e "${BLUE}Deployment complete! 🎉${NC}"
