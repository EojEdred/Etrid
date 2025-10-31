#!/usr/bin/env bash
# Update Prometheus to scrape all 16 validators (#6-21)

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
MONITORING_SERVER="compiler-dev01@98.71.91.84"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "📊 Updating Prometheus Configuration for All 16 Validators"
echo "==========================================================="
echo ""

# Create new prometheus config with all 16 validators
cat > /tmp/prometheus-all-validators.yml <<'EOF'
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  # Prometheus itself
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  # Validator #6 - Consensus Dev
  - job_name: 'validator-6-node'
    static_configs:
      - targets: ['20.224.104.239:9100']
        labels:
          validator: 'validator-6'
          name: 'Consensus Dev'
  - job_name: 'validator-6-substrate'
    static_configs:
      - targets: ['20.224.104.239:9615']
        labels:
          validator: 'validator-6'
          name: 'Consensus Dev'

  # Validator #7 - Runtime Dev Primary
  - job_name: 'validator-7-node'
    static_configs:
      - targets: ['108.142.205.177:9100']
        labels:
          validator: 'validator-7'
          name: 'Runtime Dev Primary'
  - job_name: 'validator-7-substrate'
    static_configs:
      - targets: ['108.142.205.177:9615']
        labels:
          validator: 'validator-7'
          name: 'Runtime Dev Primary'

  # Validator #8 - Runtime Dev Secondary
  - job_name: 'validator-8-node'
    static_configs:
      - targets: ['4.180.238.67:9100']
        labels:
          validator: 'validator-8'
          name: 'Runtime Dev Secondary'
  - job_name: 'validator-8-substrate'
    static_configs:
      - targets: ['4.180.238.67:9615']
        labels:
          validator: 'validator-8'
          name: 'Runtime Dev Secondary'

  # Validator #9 - Compiler Dev Primary
  - job_name: 'validator-9-node'
    static_configs:
      - targets: ['4.180.59.25:9100']
        labels:
          validator: 'validator-9'
          name: 'Compiler Dev Primary'
  - job_name: 'validator-9-substrate'
    static_configs:
      - targets: ['4.180.59.25:9615']
        labels:
          validator: 'validator-9'
          name: 'Compiler Dev Primary'

  # Validator #10 - Compiler Dev (Monitoring)
  - job_name: 'validator-10-node'
    static_configs:
      - targets: ['98.71.91.84:9100']
        labels:
          validator: 'validator-10'
          name: 'Compiler Dev (Monitoring)'
  - job_name: 'validator-10-substrate'
    static_configs:
      - targets: ['98.71.91.84:9615']
        labels:
          validator: 'validator-10'
          name: 'Compiler Dev (Monitoring)'

  # Validator #11 - Multichain Dev Primary
  - job_name: 'validator-11-node'
    static_configs:
      - targets: ['68.219.230.63:9100']
        labels:
          validator: 'validator-11'
          name: 'Multichain Dev Primary'
  - job_name: 'validator-11-substrate'
    static_configs:
      - targets: ['68.219.230.63:9615']
        labels:
          validator: 'validator-11'
          name: 'Multichain Dev Primary'

  # Validator #12 - Multichain Dev Secondary
  - job_name: 'validator-12-node'
    static_configs:
      - targets: ['98.71.219.106:9100']
        labels:
          validator: 'validator-12'
          name: 'Multichain Dev Secondary'
  - job_name: 'validator-12-substrate'
    static_configs:
      - targets: ['98.71.219.106:9615']
        labels:
          validator: 'validator-12'
          name: 'Multichain Dev Secondary'

  # Validator #13 - Oracle Dev
  - job_name: 'validator-13-node'
    static_configs:
      - targets: ['172.167.8.217:9100']
        labels:
          validator: 'validator-13'
          name: 'Oracle Dev'
  - job_name: 'validator-13-substrate'
    static_configs:
      - targets: ['172.167.8.217:9615']
        labels:
          validator: 'validator-13'
          name: 'Oracle Dev'

  # Validator #14 - EDSC Dev
  - job_name: 'validator-14-node'
    static_configs:
      - targets: ['51.142.203.160:9100']
        labels:
          validator: 'validator-14'
          name: 'EDSC Dev'
  - job_name: 'validator-14-substrate'
    static_configs:
      - targets: ['51.142.203.160:9615']
        labels:
          validator: 'validator-14'
          name: 'EDSC Dev'

  # Validator #15 - Economics Dev Primary
  - job_name: 'validator-15-node'
    static_configs:
      - targets: ['172.166.164.19:9100']
        labels:
          validator: 'validator-15'
          name: 'Economics Dev Primary'
  - job_name: 'validator-15-substrate'
    static_configs:
      - targets: ['172.166.164.19:9615']
        labels:
          validator: 'validator-15'
          name: 'Economics Dev Primary'

  # Validator #16 - Economics Dev Secondary
  - job_name: 'validator-16-node'
    static_configs:
      - targets: ['172.166.187.180:9100']
        labels:
          validator: 'validator-16'
          name: 'Economics Dev Secondary'
  - job_name: 'validator-16-substrate'
    static_configs:
      - targets: ['172.166.187.180:9615']
        labels:
          validator: 'validator-16'
          name: 'Economics Dev Secondary'

  # Validator #17 - Ethics Dev Primary
  - job_name: 'validator-17-node'
    static_configs:
      - targets: ['172.166.210.244:9100']
        labels:
          validator: 'validator-17'
          name: 'Ethics Dev Primary'
  - job_name: 'validator-17-substrate'
    static_configs:
      - targets: ['172.166.210.244:9615']
        labels:
          validator: 'validator-17'
          name: 'Ethics Dev Primary'

  # Validator #18 - Ethics Dev Secondary
  - job_name: 'validator-18-node'
    static_configs:
      - targets: ['4.251.115.186:9100']
        labels:
          validator: 'validator-18'
          name: 'Ethics Dev Secondary'
  - job_name: 'validator-18-substrate'
    static_configs:
      - targets: ['4.251.115.186:9615']
        labels:
          validator: 'validator-18'
          name: 'Ethics Dev Secondary'

  # Validator #19 - Docs Dev Primary
  - job_name: 'validator-19-node'
    static_configs:
      - targets: ['52.143.191.232:9100']
        labels:
          validator: 'validator-19'
          name: 'Docs Dev Primary'
  - job_name: 'validator-19-substrate'
    static_configs:
      - targets: ['52.143.191.232:9615']
        labels:
          validator: 'validator-19'
          name: 'Docs Dev Primary'

  # Validator #20 - Docs Dev Secondary
  - job_name: 'validator-20-node'
    static_configs:
      - targets: ['4.211.206.210:9100']
        labels:
          validator: 'validator-20'
          name: 'Docs Dev Secondary'
  - job_name: 'validator-20-substrate'
    static_configs:
      - targets: ['4.211.206.210:9615']
        labels:
          validator: 'validator-20'
          name: 'Docs Dev Secondary'

  # Validator #21 - Docs Dev Tertiary
  - job_name: 'validator-21-node'
    static_configs:
      - targets: ['4.178.181.122:9100']
        labels:
          validator: 'validator-21'
          name: 'Docs Dev Tertiary'
  - job_name: 'validator-21-substrate'
    static_configs:
      - targets: ['4.178.181.122:9615']
        labels:
          validator: 'validator-21'
          name: 'Docs Dev Tertiary'
EOF

echo -e "${YELLOW}📤 Uploading Prometheus configuration...${NC}"
scp -i "$SSH_KEY" -o StrictHostKeyChecking=no /tmp/prometheus-all-validators.yml \
    "$MONITORING_SERVER:/tmp/prometheus.yml"

echo -e "${YELLOW}🔄 Deploying configuration and restarting Prometheus...${NC}"
ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "$MONITORING_SERVER" <<'REMOTE'
set -e

# Backup existing config
sudo cp /etc/prometheus/prometheus.yml /etc/prometheus/prometheus.yml.backup 2>/dev/null || true

# Deploy new config
sudo mv /tmp/prometheus.yml /etc/prometheus/prometheus.yml
sudo chown prometheus:prometheus /etc/prometheus/prometheus.yml

# Restart Prometheus
sudo systemctl restart prometheus

# Wait for Prometheus to start
sleep 3

# Check status
sudo systemctl is-active prometheus

echo "✓ Prometheus restarted with new configuration"
REMOTE

echo ""
echo -e "${GREEN}✅ Prometheus updated successfully!${NC}"
echo ""
echo "📊 Now scraping 16 validators (#6-21):"
echo "   - 16 node exporters (system metrics)"
echo "   - 16 substrate exporters (blockchain metrics)"
echo "   - Total: 33 targets (including Prometheus itself)"
echo ""
echo "🔍 View targets: http://98.71.91.84:9090/targets"
echo ""
