#!/bin/bash
# Deploy Complete Monitoring Infrastructure to VM #10 (Prometheus + Grafana)
# Target: compiler-dev01@98.71.91.84
set -e

# Configuration
MONITORING_SERVER="compiler-dev01@98.71.91.84"
SSH_KEY="$HOME/.ssh/gizzi-validator"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  ËTRID Monitoring Infrastructure Deployment                  ║${NC}"
echo -e "${BLUE}║  Prometheus + Grafana on VM #10                              ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Test SSH connectivity
echo -e "${YELLOW}[1/4]${NC} Testing SSH connectivity..."
if ! ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 "$MONITORING_SERVER" "echo 'SSH OK'" &>/dev/null; then
    echo -e "${RED}✗${NC} Cannot connect to monitoring server"
    exit 1
fi
echo -e "${GREEN}✓${NC} SSH connection successful"

# Install Prometheus
echo -e "${YELLOW}[2/4]${NC} Installing Prometheus..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" << 'REMOTE_PROMETHEUS'
# Check if already installed
if systemctl is-active --quiet prometheus; then
    echo "Prometheus already running"
    exit 0
fi

# Download and install Prometheus
cd /tmp
wget -q https://github.com/prometheus/prometheus/releases/download/v2.48.0/prometheus-2.48.0.linux-amd64.tar.gz
tar xzf prometheus-2.48.0.linux-amd64.tar.gz

# Move binaries
sudo mv prometheus-2.48.0.linux-amd64/prometheus /usr/local/bin/
sudo mv prometheus-2.48.0.linux-amd64/promtool /usr/local/bin/

# Create directories
sudo mkdir -p /etc/prometheus /var/lib/prometheus

# Move config files
sudo mv prometheus-2.48.0.linux-amd64/prometheus.yml /etc/prometheus/
sudo mv prometheus-2.48.0.linux-amd64/consoles /etc/prometheus/
sudo mv prometheus-2.48.0.linux-amd64/console_libraries /etc/prometheus/

# Cleanup
rm -rf prometheus-2.48.0.linux-amd64*

# Create Prometheus configuration
sudo tee /etc/prometheus/prometheus.yml > /dev/null << 'PROMCONFIG'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  # FlareChain metrics (all 21 validators)
  - job_name: 'flarechain'
    static_configs:
      - targets:
        # Validators #1-5 (currently inaccessible, will add when firewall fixed)
        # - '20.186.91.207:9615'      # Validator #1 (Gizzi)
        # - '172.177.44.73:9615'      # Validator #2 (EojEdred)
        # - '20.186.91.207:9616'      # Validator #3 (shares VM with #1, port 9616)
        # - '52.252.142.146:9615'     # Validator #4 (Security Dev)
        # - '132.145.145.135:9615'    # Validator #5 (Audit Dev - Oracle)

        # Validators #6-21 (accessible)
        - '20.224.104.239:9615'     # Validator #6 (Runtime Dev)
        - '98.71.91.84:9615'        # Validator #7 (Compiler Dev)
        - '20.169.114.25:9615'      # Validator #8 (Network Dev)
        - '20.75.92.203:9615'       # Validator #9 (SDK Dev)
        - '20.55.31.30:9615'        # Validator #10 (DevTools Dev)
        - '20.73.34.17:9615'        # Validator #11 (API Dev)
        - '20.109.102.30:9615'      # Validator #12 (Docs Dev)
        - '52.250.61.132:9615'      # Validator #13 (QA Dev)
        - '20.218.66.251:9615'      # Validator #14 (Perf Dev)
        - '20.109.219.185:9615'     # Validator #15 (Community Dev)
        - '20.83.208.17:9615'       # Validator #16 (Analytics Dev)
        - '172.177.175.132:9615'    # Validator #17 (Ethics Dev)
        - '20.84.231.225:9615'      # Validator #18 (FlareNode 16)
        - '4.175.83.133:9615'       # Validator #19 (FlareNode 19)
        - '52.184.47.99:9615'       # Validator #20 (FlareNode 20)
        - '4.178.181.122:9615'      # Validator #21 (FlareNode 21)

  # System metrics (node_exporter on accessible validators)
  - job_name: 'node'
    static_configs:
      - targets:
        - '20.224.104.239:9100'     # Validator #6
        - '98.71.91.84:9100'        # Validator #7
        - '20.169.114.25:9100'      # Validator #8
        - '20.75.92.203:9100'       # Validator #9
        - '20.55.31.30:9100'        # Validator #10
        - '20.73.34.17:9100'        # Validator #11
        - '20.109.102.30:9100'      # Validator #12
        - '52.250.61.132:9100'      # Validator #13
        - '20.218.66.251:9100'      # Validator #14
        - '20.109.219.185:9100'     # Validator #15
        - '20.83.208.17:9100'       # Validator #16
        - '172.177.175.132:9100'    # Validator #17
        - '20.84.231.225:9100'      # Validator #18
        - '4.175.83.133:9100'       # Validator #19
        - '52.184.47.99:9100'       # Validator #20
        - '4.178.181.122:9100'      # Validator #21
PROMCONFIG

# Create systemd service
sudo tee /etc/systemd/system/prometheus.service > /dev/null << 'PROMSERVICE'
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
PROMSERVICE

# Start Prometheus
sudo systemctl daemon-reload
sudo systemctl enable prometheus
sudo systemctl start prometheus

echo "✓ Prometheus installed and running"
REMOTE_PROMETHEUS

echo -e "${GREEN}✓${NC} Prometheus installed"

# Install Grafana
echo -e "${YELLOW}[3/4]${NC} Installing Grafana..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" << 'REMOTE_GRAFANA'
# Check if already installed
if systemctl is-active --quiet grafana-server; then
    echo "Grafana already running"
    exit 0
fi

# Install dependencies
sudo apt-get update
sudo apt-get install -y software-properties-common

# Add Grafana repository
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
echo "deb https://packages.grafana.com/oss/deb stable main" | sudo tee /etc/apt/sources.list.d/grafana.list

# Install Grafana
sudo apt-get update
sudo apt-get install -y grafana

# Configure Grafana for public access
sudo tee /etc/grafana/grafana.ini > /dev/null << 'GRAFANACONF'
[server]
http_port = 3000

[auth.anonymous]
enabled = true
org_role = Viewer

[security]
allow_embedding = true
GRAFANACONF

# Start Grafana
sudo systemctl daemon-reload
sudo systemctl enable grafana-server
sudo systemctl start grafana-server

echo "✓ Grafana installed and running"
REMOTE_GRAFANA

echo -e "${GREEN}✓${NC} Grafana installed"

# Verify services
echo -e "${YELLOW}[4/4]${NC} Verifying services..."
ssh -i "$SSH_KEY" "$MONITORING_SERVER" << 'REMOTE_VERIFY'
echo "Checking Prometheus..."
systemctl is-active prometheus && echo "✓ Prometheus running" || echo "✗ Prometheus not running"

echo "Checking Grafana..."
systemctl is-active grafana-server && echo "✓ Grafana running" || echo "✗ Grafana not running"

echo ""
echo "Testing endpoints..."
curl -s http://localhost:9090/-/healthy > /dev/null && echo "✓ Prometheus healthy" || echo "✗ Prometheus not responding"
curl -s http://localhost:3000/api/health | grep -q '"database": "ok"' && echo "✓ Grafana healthy" || echo "✗ Grafana not responding"
REMOTE_VERIFY

echo ""
echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Deployment Complete!                                        ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}✓ Prometheus:${NC} http://98.71.91.84:9090"
echo -e "${GREEN}✓ Grafana:${NC} http://98.71.91.84:3000 (login: admin/admin)"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Open Grafana: http://98.71.91.84:3000"
echo "2. Add Prometheus data source: http://localhost:9090"
echo "3. Create dashboards for network monitoring"
echo "4. Configure DNS (metrics.etrid.io → 98.71.91.84)"
echo "5. Set up SSL certificate (Let's Encrypt)"
echo ""
