#!/bin/bash
# ËTRID Complete Monitoring Stack - Universal Installer
# Run this on ANY validator VM to install complete monitoring infrastructure
#
# Usage: curl -fsSL https://your-server.com/install-etrid-monitoring.sh | bash
# Or: wget -qO- https://your-server.com/install-etrid-monitoring.sh | bash
#
# What this installs:
# - Prometheus (metrics database)
# - Grafana (visualization dashboards)
# - Ollama + llama2:13b (local AI)
# - Node Exporter (system metrics)
# - AI Monitoring System (12 AI dev workers)
# - FlareChain metrics scraping configuration

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  ËTRID Monitoring Stack - Universal Installer                ║${NC}"
echo -e "${BLUE}║  Installing: Prometheus + Grafana + Ollama + AI System      ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Detect if this is the monitoring server or a validator node
IS_MONITORING_SERVER=false
if [ "$1" == "--monitoring-server" ]; then
    IS_MONITORING_SERVER=true
    echo -e "${YELLOW}Mode: MONITORING SERVER (Full Stack)${NC}"
else
    echo -e "${YELLOW}Mode: VALIDATOR NODE (Node Exporter Only)${NC}"
fi

# Get current user
CURRENT_USER=$(whoami)
echo "Installing as user: $CURRENT_USER"
echo ""

# ============================================================
# Step 1: Install System Dependencies
# ============================================================
echo -e "${YELLOW}[1/7]${NC} Installing system dependencies..."
sudo apt-get update -qq
sudo apt-get install -y curl wget git python3 python3-pip jq htop software-properties-common

# ============================================================
# Step 2: Install Node Exporter (ALL VMs)
# ============================================================
echo -e "${YELLOW}[2/7]${NC} Installing Node Exporter..."
if ! systemctl is-active --quiet node_exporter 2>/dev/null; then
    cd /tmp
    wget -q https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz
    tar xzf node_exporter-1.7.0.linux-amd64.tar.gz
    sudo mv node_exporter-1.7.0.linux-amd64/node_exporter /usr/local/bin/
    rm -rf node_exporter-1.7.0.linux-amd64*

    sudo tee /etc/systemd/system/node_exporter.service > /dev/null << 'EOF'
[Unit]
Description=Node Exporter
After=network.target

[Service]
Type=simple
User=nobody
ExecStart=/usr/local/bin/node_exporter --web.listen-address=:9100
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    sudo systemctl daemon-reload
    sudo systemctl enable node_exporter
    sudo systemctl start node_exporter
    echo -e "${GREEN}✓${NC} Node Exporter installed"
else
    echo -e "${GREEN}✓${NC} Node Exporter already installed"
fi

if [ "$IS_MONITORING_SERVER" = false ]; then
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  Validator Node Installation Complete!                       ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Node Exporter is running on port 9100"
    echo "Metrics available at: http://$(hostname -I | awk '{print $1}'):9100/metrics"
    echo ""
    echo "This VM will now report system metrics to Prometheus!"
    exit 0
fi

# ============================================================
# MONITORING SERVER ONLY - Continue installation
# ============================================================

# Step 3: Install Prometheus
echo -e "${YELLOW}[3/7]${NC} Installing Prometheus..."
if ! systemctl is-active --quiet prometheus 2>/dev/null; then
    cd /tmp
    wget -q https://github.com/prometheus/prometheus/releases/download/v2.48.0/prometheus-2.48.0.linux-amd64.tar.gz
    tar xzf prometheus-2.48.0.linux-amd64.tar.gz

    sudo mv prometheus-2.48.0.linux-amd64/prometheus /usr/local/bin/
    sudo mv prometheus-2.48.0.linux-amd64/promtool /usr/local/bin/
    sudo mkdir -p /etc/prometheus /var/lib/prometheus

    # Download prometheus.yml from monitoring server or use default
    if [ -f "/tmp/prometheus.yml" ]; then
        sudo mv /tmp/prometheus.yml /etc/prometheus/
    else
        sudo tee /etc/prometheus/prometheus.yml > /dev/null << 'EOF'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'node'
    static_configs:
      - targets: ['localhost:9100']
EOF
    fi

    sudo tee /etc/systemd/system/prometheus.service > /dev/null << 'EOF'
[Unit]
Description=Prometheus
Wants=network-online.target
After=network-online.target

[Service]
User=root
ExecStart=/usr/local/bin/prometheus \
  --config.file=/etc/prometheus/prometheus.yml \
  --storage.tsdb.path=/var/lib/prometheus/ \
  --web.console.templates=/etc/prometheus/consoles \
  --web.console.libraries=/etc/prometheus/console_libraries
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    sudo systemctl daemon-reload
    sudo systemctl enable prometheus
    sudo systemctl start prometheus
    echo -e "${GREEN}✓${NC} Prometheus installed"
else
    echo -e "${GREEN}✓${NC} Prometheus already installed"
fi

# Step 4: Install Grafana
echo -e "${YELLOW}[4/7]${NC} Installing Grafana..."
if ! systemctl is-active --quiet grafana-server 2>/dev/null; then
    wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
    echo "deb https://packages.grafana.com/oss/deb stable main" | sudo tee /etc/apt/sources.list.d/grafana.list
    sudo apt-get update -qq
    sudo apt-get install -y grafana

    sudo tee /etc/grafana/grafana.ini > /dev/null << 'EOF'
[server]
http_port = 3000

[auth.anonymous]
enabled = true
org_role = Viewer

[security]
allow_embedding = true
EOF

    sudo systemctl daemon-reload
    sudo systemctl enable grafana-server
    sudo systemctl start grafana-server
    echo -e "${GREEN}✓${NC} Grafana installed"
else
    echo -e "${GREEN}✓${NC} Grafana already installed"
fi

# Step 5: Install Ollama
echo -e "${YELLOW}[5/7]${NC} Installing Ollama + llama2:13b model..."
if ! command -v ollama &> /dev/null; then
    curl -fsSL https://ollama.ai/install.sh | sh
    sudo systemctl enable ollama
    sudo systemctl start ollama
    sleep 5
    ollama pull llama2:13b
    echo -e "${GREEN}✓${NC} Ollama installed with llama2:13b model"
else
    echo -e "${GREEN}✓${NC} Ollama already installed"
    if ! ollama list | grep -q llama2:13b; then
        ollama pull llama2:13b
    fi
fi

# Step 6: Install Python dependencies for AI monitoring
echo -e "${YELLOW}[6/7]${NC} Installing Python dependencies..."
sudo pip3 install --quiet anthropic openai requests python-dotenv psutil paramiko

# Step 7: Download and install AI monitoring system
echo -e "${YELLOW}[7/7]${NC} Installing AI Monitoring System..."
sudo mkdir -p /opt/ai-monitoring/{skills,dids,logs}
sudo chown -R $CURRENT_USER:$CURRENT_USER /opt/ai-monitoring

# Download AI monitoring files (these would be fetched from your server)
echo "Downloading AI monitoring system files..."
echo "Note: Files should be pre-staged or downloaded from your repository"

# Create placeholder .env file
cat > /opt/ai-monitoring/.env << 'EOF'
# ËTRID AI Monitoring Configuration
# IMPORTANT: Add your API keys here!

OPENAI_API_KEY=your-openai-key-here
ANTHROPIC_API_KEY=your-anthropic-key-here

AI_TIER_1=ollama
AI_TIER_2=gpt4
AI_TIER_3=claude

OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=llama2:13b

MONITORING_INTERVAL=300
LOG_LEVEL=INFO
ENABLE_GLOBAL_MEMORY=true
GLOBAL_MEMORY_PATH=/opt/ai-monitoring/GLOBAL_MEMORY.md

NUM_VALIDATORS=21
PROMETHEUS_URL=http://localhost:9090
VALIDATOR_IPS_PATH=/opt/ai-monitoring/validator-ips.json
SSH_KEY_PATH=/home/${CURRENT_USER}/.ssh/gizzi-validator
MONITOR_INTERVAL=300
OPTIMIZED=true

ENABLE_ALERTS=true
EOF

chmod 600 /opt/ai-monitoring/.env

echo ""
echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Monitoring Server Installation Complete!                    ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}✓ Prometheus:${NC} http://$(hostname -I | awk '{print $1}'):9090"
echo -e "${GREEN}✓ Grafana:${NC} http://$(hostname -I | awk '{print $1}'):3000 (admin/admin)"
echo -e "${GREEN}✓ Ollama:${NC} http://$(hostname -I | awk '{print $1}'):11434"
echo -e "${GREEN}✓ Node Exporter:${NC} http://$(hostname -I | awk '{print $1}'):9100"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Edit /opt/ai-monitoring/.env and add your API keys"
echo "2. Copy AI monitoring Python scripts to /opt/ai-monitoring/"
echo "3. Set up systemd service for AI monitoring"
echo "4. Configure Prometheus targets in /etc/prometheus/prometheus.yml"
echo ""
