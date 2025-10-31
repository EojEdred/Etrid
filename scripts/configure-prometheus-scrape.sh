#!/usr/bin/env bash
# Configure Prometheus to scrape all 8 accessible validators
# Monitoring Server: compiler-dev01@98.71.91.84

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
MONITORING_SERVER="compiler-dev01@98.71.91.84"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "📊 Configuring Prometheus to Scrape Validators"
echo "==============================================="
echo ""

# Create Prometheus configuration
cat > /tmp/prometheus-validators.yml << 'PROM_CONFIG'
# Prometheus configuration for Ëtrid FlareChain validators
global:
  scrape_interval: 15s
  evaluation_interval: 15s

# Scrape configurations
scrape_configs:
  # Prometheus self-monitoring
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  # FlareChain validator metrics (blockchain-specific)
  - job_name: 'flarechain_validators'
    scrape_interval: 15s
    static_configs:
      - targets:
        # Validators #14-21 (accessible)
        - '51.142.203.160:9615'   # Validator #14 - audit-dev01
        - '172.166.164.19:9615'   # Validator #15 - flarenode15
        - '172.166.187.180:9615'  # Validator #16 - flarenode16
        - '172.166.210.244:9615'  # Validator #17 - flarenode17
        - '4.251.115.186:9615'    # Validator #18 - flarenode18
        - '52.143.191.232:9615'   # Validator #19 - flarenode19
        - '4.211.206.210:9615'    # Validator #20 - flarenode20
        - '4.178.181.122:9615'    # Validator #21 - flarenode21

        # Add validators #1-13 when firewall rules are configured
        # - '20.186.91.207:9615'    # Validator #1 - Gizzi (Bootstrap)
        # - '172.177.44.73:9615'    # Validator #2 - EojEdred (Bootstrap)
        # - '20.186.91.207:9616'    # Validator #3 - Governance (port 9616 - shares VM with #1)
        # - '52.252.142.146:9615'   # Validator #4 - Security Dev
        # - '132.145.145.135:9615'  # Validator #5 - Audit Dev (Oracle)
        # - '20.224.104.239:9615'   # Validator #6 - Consensus Dev
        # - '108.142.205.177:9615'  # Validator #7 - Runtime Dev
        # - '4.180.238.67:9615'     # Validator #8 - Runtime Dev
        # - '4.180.59.25:9615'      # Validator #9 - Compiler Dev
        # - '98.71.91.84:9615'      # Validator #10 - Compiler Dev (this server)
        # - '68.219.230.63:9615'    # Validator #11 - Multichain Dev
        # - '98.71.219.106:9615'    # Validator #12 - Multichain Dev
        # - '172.167.8.217:9615'    # Validator #13 - Oracle Dev

        labels:
          network: 'flarechain_mainnet'
          cluster: 'validators'

  # Node Exporter metrics (system-level: CPU, RAM, disk, network)
  - job_name: 'node_exporters'
    scrape_interval: 15s
    static_configs:
      - targets:
        # Validators #14-21 (accessible)
        - '51.142.203.160:9100'   # Validator #14
        - '172.166.164.19:9100'   # Validator #15
        - '172.166.187.180:9100'  # Validator #16
        - '172.166.210.244:9100'  # Validator #17
        - '4.251.115.186:9100'    # Validator #18
        - '52.143.191.232:9100'   # Validator #19
        - '4.211.206.210:9100'    # Validator #20
        - '4.178.181.122:9100'    # Validator #21

        # Monitoring server itself
        - 'localhost:9100'        # VM #10 - Monitoring Server

        labels:
          network: 'flarechain_mainnet'
          cluster: 'validators'

  # Grafana monitoring
  - job_name: 'grafana'
    static_configs:
      - targets: ['localhost:3000']
PROM_CONFIG

echo -e "${YELLOW}1️⃣  Uploading Prometheus configuration...${NC}"
scp -i "$SSH_KEY" -o StrictHostKeyChecking=no \
    /tmp/prometheus-validators.yml \
    "$MONITORING_SERVER:/tmp/prometheus.yml.new"

echo -e "${GREEN}✅ Configuration uploaded${NC}"
echo ""

echo -e "${YELLOW}2️⃣  Backing up existing config and installing new one...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" bash <<'INSTALL_CONFIG'
set -e

# Backup existing config
sudo cp /etc/prometheus/prometheus.yml /etc/prometheus/prometheus.yml.backup-$(date +%Y%m%d-%H%M%S)

# Install new config
sudo cp /tmp/prometheus.yml.new /etc/prometheus/prometheus.yml

# Validate config
if sudo promtool check config /etc/prometheus/prometheus.yml; then
    echo "✓ Prometheus config is valid"
else
    echo "✗ Config validation failed, restoring backup"
    sudo cp /etc/prometheus/prometheus.yml.backup-* /etc/prometheus/prometheus.yml
    exit 1
fi

# Restart Prometheus
echo "Restarting Prometheus..."
sudo systemctl restart prometheus

# Wait for Prometheus to start
sleep 5

# Check status
if systemctl is-active --quiet prometheus; then
    echo "✓ Prometheus restarted successfully"
else
    echo "✗ Prometheus failed to start"
    sudo journalctl -u prometheus -n 20 --no-pager
    exit 1
fi
INSTALL_CONFIG

echo -e "${GREEN}✅ Prometheus configured and restarted${NC}"
echo ""

echo -e "${YELLOW}3️⃣  Verifying scrape targets...${NC}"
sleep 5

# Check Prometheus targets
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" \
    'curl -s http://localhost:9090/api/v1/targets | python3 -m json.tool | grep -A 3 "health"' || true

echo -e "${GREEN}✅ Scrape targets configured${NC}"
echo ""

echo "==============================================="
echo -e "${GREEN}🎉 Prometheus Configuration Complete!${NC}"
echo "==============================================="
echo ""
echo "Configured to scrape:"
echo "  • 8 FlareChain validators (blockchain metrics)"
echo "  • 9 Node Exporters (system metrics: 8 validators + monitoring server)"
echo "  • Grafana dashboard"
echo "  • Prometheus self-monitoring"
echo ""
echo "View targets: http://98.71.91.84:9090/targets"
echo "View metrics: http://98.71.91.84:9090/graph"
echo ""
echo "Next steps:"
echo "  1. Open Grafana: http://98.71.91.84:3000"
echo "  2. Add Prometheus as data source"
echo "  3. Create dashboards for validator monitoring"
echo ""
