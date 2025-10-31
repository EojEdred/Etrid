#!/usr/bin/env bash
# Deploy node exporters to validators #1-5

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🚀 Deploying Node Exporters to Validators #1-5"
echo "==============================================="
echo ""

# Function to deploy node exporter
deploy_node_exporter() {
    local validator_num=$1
    local ssh_user=$2
    local ssh_host=$3
    local name=$4

    echo -e "${YELLOW}Deploying to Validator #$validator_num ($name - $ssh_host)...${NC}"

    # Test SSH connection first
    if ! ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" "echo 'SSH OK'" >/dev/null 2>&1; then
        echo -e "${RED}  ❌ SSH connection failed${NC}"
        return 1
    fi

    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" bash <<'DEPLOY_SCRIPT'
set -e

# Check if already installed and running
if systemctl is-active --quiet node_exporter 2>/dev/null; then
    echo "  ✓ Node exporter already running"
    exit 0
fi

# Stop if exists but not running
sudo systemctl stop node_exporter 2>/dev/null || true
sudo pkill -9 node_exporter 2>/dev/null || true
sleep 2

# Download and install
cd /tmp
rm -rf node_exporter-1.7.0.linux-amd64*
wget -q https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz
tar xzf node_exporter-1.7.0.linux-amd64.tar.gz

# Remove old binary if exists
sudo rm -f /usr/local/bin/node_exporter

# Install new binary
sudo cp node_exporter-1.7.0.linux-amd64/node_exporter /usr/local/bin/
sudo chmod +x /usr/local/bin/node_exporter
rm -rf node_exporter-1.7.0.linux-amd64*

# Create systemd service
sudo tee /etc/systemd/system/node_exporter.service > /dev/null <<'EOF'
[Unit]
Description=Node Exporter
After=network.target

[Service]
Type=simple
User=nobody
ExecStart=/usr/local/bin/node_exporter
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl enable node_exporter
sudo systemctl start node_exporter

# Verify it's running
sleep 2
if systemctl is-active --quiet node_exporter; then
    echo "  ✓ Node exporter installed and running"
else
    echo "  ✗ Node exporter failed to start"
    exit 1
fi
DEPLOY_SCRIPT

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}  ✅ Validator #$validator_num deployed${NC}"
        return 0
    else
        echo -e "${RED}  ❌ Validator #$validator_num failed${NC}"
        return 1
    fi
}

# Deploy to all validators #1-5
successful=0
failed=0

echo "Deploying to Oracle Cloud and Azure validators..."
echo ""

deploy_node_exporter 1 "ubuntu" "64.181.215.19" "Gizzi (Oracle Cloud)" && ((successful++)) || ((failed++))
deploy_node_exporter 2 "eojedred" "20.69.26.209" "EojEdred Founder (Azure)" && ((successful++)) || ((failed++))
deploy_node_exporter 3 "governance-dev01" "20.186.91.207" "Governance Dev (Azure)" && ((successful++)) || ((failed++))
deploy_node_exporter 4 "security-dev01" "52.252.142.146" "Security Dev (Azure)" && ((successful++)) || ((failed++))
deploy_node_exporter 5 "ubuntu" "132.145.145.135" "Audit Dev (Oracle/Azure)" && ((successful++)) || ((failed++))

echo ""
echo "==============================================="
echo "Deployment Summary for Validators #1-5"
echo "==============================================="
echo -e "${GREEN}✅ Successful: $successful/5${NC}"
echo -e "${RED}❌ Failed: $failed/5${NC}"
echo ""

if [ $successful -eq 5 ]; then
    echo -e "${GREEN}🎉 All 5 validators deployed successfully!${NC}"
    echo ""
    echo "Next: Update Prometheus to scrape these validators"
    exit 0
elif [ $successful -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Partial success - $successful validators deployed${NC}"
    echo ""
    echo "Accessible validators:"
    [ $successful -gt 0 ] && echo "  - $successful validators with node exporters running"
    [ $failed -gt 0 ] && echo "  - $failed validators need manual configuration"
    exit 2
else
    echo -e "${RED}❌ All deployments failed - check SSH access and firewall rules${NC}"
    exit 1
fi
