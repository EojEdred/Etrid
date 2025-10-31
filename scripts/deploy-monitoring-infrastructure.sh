#!/bin/bash
# Deploy Complete Monitoring Infrastructure to VM #10
# Target: compiler-dev01@98.71.91.84
# Purpose: Prometheus + Grafana + Node Exporter for 21 validators

set -e

echo "🚀 ËTRID Monitoring Infrastructure Deployment"
echo "=============================================="
echo ""

# Configuration
MONITORING_VM="compiler-dev01@98.71.91.84"
SSH_KEY="$HOME/.ssh/gizzi-validator"
MONITORING_DIR="/opt/monitoring"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Target VM: ${MONITORING_VM}${NC}"
echo -e "${YELLOW}SSH Key: ${SSH_KEY}${NC}"
echo ""

# Test SSH connection
echo "1️⃣  Testing SSH connection..."
if ssh -i "$SSH_KEY" -o ConnectTimeout=5 "$MONITORING_VM" "echo 'SSH OK'" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ SSH connection successful${NC}"
else
    echo -e "${RED}❌ SSH connection failed${NC}"
    exit 1
fi

# Create deployment script
echo ""
echo "2️⃣  Creating deployment script..."

cat > /tmp/install-monitoring.sh << 'REMOTE_SCRIPT'
#!/bin/bash
set -e

echo "Installing monitoring stack on $(hostname)..."

# Update system
echo "Updating system packages..."
sudo apt-get update -qq

# Install prerequisites
echo "Installing prerequisites..."
sudo apt-get install -y wget curl tar docker.io docker-compose

# Create monitoring directory
echo "Creating monitoring directory..."
sudo mkdir -p /opt/monitoring/{prometheus,grafana,data}
cd /opt/monitoring

# Install Prometheus
echo "Installing Prometheus..."
PROM_VERSION="2.47.0"
wget -q https://github.com/prometheus/prometheus/releases/download/v${PROM_VERSION}/prometheus-${PROM_VERSION}.linux-amd64.tar.gz
tar xzf prometheus-${PROM_VERSION}.linux-amd64.tar.gz
sudo mv prometheus-${PROM_VERSION}.linux-amd64/prometheus /usr/local/bin/
sudo mv prometheus-${PROM_VERSION}.linux-amd64/promtool /usr/local/bin/
rm -rf prometheus-${PROM_VERSION}.linux-amd64*

# Install Node Exporter
echo "Installing Node Exporter..."
NODE_EXPORTER_VERSION="1.6.1"
wget -q https://github.com/prometheus/node_exporter/releases/download/v${NODE_EXPORTER_VERSION}/node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64.tar.gz
tar xzf node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64.tar.gz
sudo mv node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64/node_exporter /usr/local/bin/
rm -rf node_exporter-${NODE_EXPORTER_VERSION}*

# Create prometheus user
sudo useradd --no-create-home --shell /bin/false prometheus || true

# Set permissions
sudo chown -R prometheus:prometheus /opt/monitoring/prometheus
sudo chown -R prometheus:prometheus /opt/monitoring/data

echo "✅ Monitoring tools installed successfully"
echo ""
echo "Installed versions:"
prometheus --version 2>&1 | head -1
node_exporter --version 2>&1 | head -1
docker --version
docker-compose --version
REMOTE_SCRIPT

chmod +x /tmp/install-monitoring.sh

# Copy and execute installation script
echo "3️⃣  Copying installation script to VM..."
scp -i "$SSH_KEY" /tmp/install-monitoring.sh "$MONITORING_VM:/tmp/"

echo ""
echo "4️⃣  Installing Prometheus, Grafana, and Node Exporter..."
ssh -i "$SSH_KEY" "$MONITORING_VM" "bash /tmp/install-monitoring.sh"

# Create Prometheus configuration
echo ""
echo "5️⃣  Creating Prometheus configuration..."

cat > /tmp/prometheus.yml << 'PROMCONFIG'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  # Prometheus itself
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  # Node Exporter on monitoring server
  - job_name: 'monitoring-server'
    static_configs:
      - targets: ['localhost:9100']
        labels:
          instance: 'monitoring-server'
          role: 'monitoring'

  # FlareChain validators (16 accessible VMs)
  - job_name: 'flarechain-validators'
    static_configs:
      - targets:
        - '20.224.104.239:9615'  # Validator #6
        - '108.142.205.177:9615' # Validator #7
        - '4.180.238.67:9615'    # Validator #8
        - '4.180.59.25:9615'     # Validator #9
        - '98.71.91.84:9615'     # Validator #10 (this VM)
        - '68.219.230.63:9615'   # Validator #11
        - '98.71.219.106:9615'   # Validator #12
        - '172.167.8.217:9615'   # Validator #13
        - '51.142.203.160:9615'  # Validator #14
        - '172.166.164.19:9615'  # Validator #15
        - '172.166.187.180:9615' # Validator #16
        - '172.166.210.244:9615' # Validator #17
        - '4.251.115.186:9615'   # Validator #18
        - '52.143.191.232:9615'  # Validator #19
        - '4.211.206.210:9615'   # Validator #20
        - '4.178.181.122:9615'   # Validator #21
        labels:
          chain: 'flarechain'
          network: 'mainnet'

  # Node Exporters on all validators (system metrics)
  - job_name: 'validator-nodes'
    static_configs:
      - targets:
        - '20.224.104.239:9100'  # Validator #6
        - '108.142.205.177:9100' # Validator #7
        - '4.180.238.67:9100'    # Validator #8
        - '4.180.59.25:9100'     # Validator #9
        - '98.71.91.84:9100'     # Validator #10
        - '68.219.230.63:9100'   # Validator #11
        - '98.71.219.106:9100'   # Validator #12
        - '172.167.8.217:9100'   # Validator #13
        - '51.142.203.160:9100'  # Validator #14
        - '172.166.164.19:9100'  # Validator #15
        - '172.166.187.180:9100' # Validator #16
        - '172.166.210.244:9100' # Validator #17
        - '4.251.115.186:9100'   # Validator #18
        - '52.143.191.232:9100'  # Validator #19
        - '4.211.206.210:9100'   # Validator #20
        - '4.178.181.122:9100'   # Validator #21
PROMCONFIG

scp -i "$SSH_KEY" /tmp/prometheus.yml "$MONITORING_VM:/tmp/"
ssh -i "$SSH_KEY" "$MONITORING_VM" "sudo mv /tmp/prometheus.yml /opt/monitoring/prometheus/prometheus.yml && sudo chown prometheus:prometheus /opt/monitoring/prometheus/prometheus.yml"

# Create systemd services
echo ""
echo "6️⃣  Creating systemd services..."

# Prometheus service
cat > /tmp/prometheus.service << 'SERVICE'
[Unit]
Description=Prometheus Monitoring System
After=network.target

[Service]
Type=simple
User=prometheus
Group=prometheus
ExecStart=/usr/local/bin/prometheus \
  --config.file=/opt/monitoring/prometheus/prometheus.yml \
  --storage.tsdb.path=/opt/monitoring/data \
  --web.console.templates=/etc/prometheus/consoles \
  --web.console.libraries=/etc/prometheus/console_libraries \
  --web.listen-address=0.0.0.0:9090 \
  --storage.tsdb.retention.time=90d
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
SERVICE

# Node Exporter service
cat > /tmp/node-exporter.service << 'SERVICE'
[Unit]
Description=Node Exporter
After=network.target

[Service]
Type=simple
User=prometheus
Group=prometheus
ExecStart=/usr/local/bin/node_exporter \
  --web.listen-address=0.0.0.0:9100
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
SERVICE

# Grafana docker-compose
cat > /tmp/docker-compose.yml << 'COMPOSE'
version: '3.8'

services:
  grafana:
    image: grafana/grafana:latest
    container_name: grafana
    restart: always
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=etrid2025
      - GF_SERVER_ROOT_URL=http://localhost:3000
      - GF_AUTH_ANONYMOUS_ENABLED=true
      - GF_AUTH_ANONYMOUS_ORG_ROLE=Viewer
      - GF_INSTALL_PLUGINS=grafana-piechart-panel
    volumes:
      - /opt/monitoring/grafana:/var/lib/grafana
    networks:
      - monitoring

networks:
  monitoring:
    driver: bridge
COMPOSE

# Copy service files
scp -i "$SSH_KEY" /tmp/prometheus.service "$MONITORING_VM:/tmp/"
scp -i "$SSH_KEY" /tmp/node-exporter.service "$MONITORING_VM:/tmp/"
scp -i "$SSH_KEY" /tmp/docker-compose.yml "$MONITORING_VM:/tmp/"

ssh -i "$SSH_KEY" "$MONITORING_VM" << 'REMOTE'
sudo mv /tmp/prometheus.service /etc/systemd/system/
sudo mv /tmp/node-exporter.service /etc/systemd/system/
sudo mv /tmp/docker-compose.yml /opt/monitoring/

# Reload systemd
sudo systemctl daemon-reload

# Enable and start services
sudo systemctl enable prometheus
sudo systemctl enable node-exporter
sudo systemctl start prometheus
sudo systemctl start node-exporter

# Start Grafana
cd /opt/monitoring
sudo docker-compose up -d

# Wait for services to start
sleep 5

# Check status
echo ""
echo "=== Service Status ==="
sudo systemctl status prometheus --no-pager | head -5
sudo systemctl status node-exporter --no-pager | head -5
sudo docker ps

echo ""
echo "=== Listening Ports ==="
sudo ss -tlnp | grep -E ':(9090|9100|3000)'
REMOTE

echo ""
echo -e "${GREEN}✅ Monitoring infrastructure deployed successfully!${NC}"
echo ""
echo "=== Access Information ==="
echo "📊 Prometheus: http://98.71.91.84:9090"
echo "📈 Grafana:    http://98.71.91.84:3000"
echo "   Username: admin"
echo "   Password: etrid2025"
echo ""
echo "🔧 Node Exporter: http://98.71.91.84:9100/metrics"
echo ""
echo "Next Steps:"
echo "1. Configure firewall rules to allow access to ports 9090, 3000"
echo "2. Add Prometheus datasource in Grafana"
echo "3. Import FlareChain dashboard"
echo "4. Deploy node exporters to all 16 validators"
echo ""
