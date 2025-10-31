#!/bin/bash
# Deploy Node Exporter to all 16 accessible validators
# This enables system metrics collection (CPU, RAM, disk, network)

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🚀 Deploying Node Exporter to 16 Validators"
echo "==========================================="
echo ""

# Array of accessible validators
declare -a VALIDATORS=(
  "consensus-dev01@20.224.104.239"  # 6
  "runtime-dev01@108.142.205.177"   # 7
  "runtime-dev01@4.180.238.67"      # 8
  "compiler-dev01@4.180.59.25"      # 9
  "compiler-dev01@98.71.91.84"      # 10
  "multichain-dev01@68.219.230.63"  # 11
  "multichain-dev01@98.71.219.106"  # 12
  "oracle-dev01@172.167.8.217"      # 13
  "audit-dev01@51.142.203.160"      # 14
  "flarenode15@172.166.164.19"      # 15
  "flarenode16@172.166.187.180"     # 16
  "flarenode17@172.166.210.244"     # 17
  "flarenode18@4.251.115.186"       # 18
  "flarenode19@52.143.191.232"      # 19
  "flarenode20@4.211.206.210"       # 20
  "flarenode21@4.178.181.122"       # 21
)

# Create installation script
cat > /tmp/install-node-exporter.sh << 'SCRIPT'
#!/bin/bash
set -e

echo "Installing Node Exporter on $(hostname)..."

# Check if already installed
if [ -f /usr/local/bin/node_exporter ]; then
    echo "Node Exporter already installed, updating..."
fi

# Install Node Exporter
NODE_EXPORTER_VERSION="1.6.1"
cd /tmp
wget -q https://github.com/prometheus/node_exporter/releases/download/v${NODE_EXPORTER_VERSION}/node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64.tar.gz
tar xzf node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64.tar.gz
sudo mv node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64/node_exporter /usr/local/bin/
rm -rf node_exporter-${NODE_EXPORTER_VERSION}*

# Create prometheus user if doesn't exist
sudo useradd --no-create-home --shell /bin/false prometheus || true

# Create systemd service
sudo tee /etc/systemd/system/node-exporter.service > /dev/null << 'SERVICE'
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

# Reload systemd and start service
sudo systemctl daemon-reload
sudo systemctl enable node-exporter
sudo systemctl restart node-exporter

# Check status
sleep 2
if sudo systemctl is-active --quiet node-exporter; then
    echo "✅ Node Exporter installed and running"
    echo "   Access: http://$(hostname -I | awk '{print $1}'):9100/metrics"
else
    echo "❌ Node Exporter failed to start"
    sudo systemctl status node-exporter --no-pager
    exit 1
fi
SCRIPT

chmod +x /tmp/install-node-exporter.sh

# Deploy to each validator
SUCCESS=0
FAILED=0

for validator in "${VALIDATORS[@]}"; do
    validator_num=$((SUCCESS + FAILED + 6))
    echo -e "${YELLOW}Deploying to Validator #${validator_num} (${validator})...${NC}"

    # Copy script
    if scp -i "$SSH_KEY" -o ConnectTimeout=5 /tmp/install-node-exporter.sh "${validator}:/tmp/" > /dev/null 2>&1; then
        # Execute script
        if ssh -i "$SSH_KEY" -o ConnectTimeout=10 "${validator}" "bash /tmp/install-node-exporter.sh" 2>&1 | grep -q "✅"; then
            echo -e "${GREEN}✅ Success${NC}"
            ((SUCCESS++))
        else
            echo -e "${RED}❌ Failed${NC}"
            ((FAILED++))
        fi
    else
        echo -e "${RED}❌ Failed to copy script${NC}"
        ((FAILED++))
    fi
    echo ""
done

echo ""
echo "=== Deployment Summary ==="
echo -e "${GREEN}✅ Successful: ${SUCCESS}/16${NC}"
echo -e "${RED}❌ Failed: ${FAILED}/16${NC}"
echo ""

if [ $SUCCESS -gt 0 ]; then
    echo "Node Exporter metrics available at:"
    for validator in "${VALIDATORS[@]}"; do
        ip=$(echo "$validator" | cut -d'@' -f2)
        echo "  http://${ip}:9100/metrics"
    done
fi

echo ""
echo "Next: Start Prometheus on monitoring server to scrape these exporters"
