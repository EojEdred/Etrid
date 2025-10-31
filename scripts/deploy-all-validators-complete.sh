#!/usr/bin/env bash
# Deploy monitoring and keys to ALL accessible validators
# This script deploys to validators #6-21 (all Azure validators that should be accessible)

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🚀 Deploying to ALL Accessible Validators (#6-21)"
echo "=================================================="
echo ""

# Function to deploy node exporter
deploy_node_exporter() {
    local validator_num=$1
    local ssh_user=$2
    local ssh_host=$3
    local name=$4

    echo -e "${YELLOW}Deploying node exporter to #$validator_num ($name)...${NC}"

    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" bash <<'DEPLOY_SCRIPT'
set -e

# Check if already installed
if systemctl is-active --quiet node_exporter 2>/dev/null; then
    echo "  Node exporter already running"
    exit 0
fi

# Download and install
cd /tmp
wget -q https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz
tar xzf node_exporter-1.7.0.linux-amd64.tar.gz
sudo cp node_exporter-1.7.0.linux-amd64/node_exporter /usr/local/bin/
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

echo "  ✓ Node exporter installed and started"
DEPLOY_SCRIPT

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}  ✅ #$validator_num deployed${NC}"
        return 0
    else
        echo -e "${RED}  ❌ #$validator_num failed${NC}"
        return 1
    fi
}

# Deploy to all validators
successful=0
failed=0

# Validators #6-13
deploy_node_exporter 6 "consensus-dev01" "20.224.104.239" "Consensus Dev" && ((successful++)) || ((failed++))
deploy_node_exporter 7 "runtime-dev01" "108.142.205.177" "Runtime Dev Primary" && ((successful++)) || ((failed++))
deploy_node_exporter 8 "runtime-dev01" "4.180.238.67" "Runtime Dev Secondary" && ((successful++)) || ((failed++))
deploy_node_exporter 9 "compiler-dev01" "4.180.59.25" "Compiler Dev Primary" && ((successful++)) || ((failed++))
deploy_node_exporter 10 "compiler-dev01" "98.71.91.84" "Compiler Dev (Monitoring)" && ((successful++)) || ((failed++))
deploy_node_exporter 11 "multichain-dev01" "68.219.230.63" "Multichain Dev Primary" && ((successful++)) || ((failed++))
deploy_node_exporter 12 "multichain-dev01" "98.71.219.106" "Multichain Dev Secondary" && ((successful++)) || ((failed++))
deploy_node_exporter 13 "oracle-dev01" "172.167.8.217" "Oracle Dev" && ((successful++)) || ((failed++))

# Validators #14-21 (re-run to ensure they're all set)
deploy_node_exporter 14 "audit-dev01" "51.142.203.160" "EDSC Dev" && ((successful++)) || ((failed++))
deploy_node_exporter 15 "flarenode15" "172.166.164.19" "Economics Dev Primary" && ((successful++)) || ((failed++))
deploy_node_exporter 16 "flarenode16" "172.166.187.180" "Economics Dev Secondary" && ((successful++)) || ((failed++))
deploy_node_exporter 17 "flarenode17" "172.166.210.244" "Ethics Dev Primary" && ((successful++)) || ((failed++))
deploy_node_exporter 18 "flarenode18" "4.251.115.186" "Ethics Dev Secondary" && ((successful++)) || ((failed++))
deploy_node_exporter 19 "flarenode19" "52.143.191.232" "Docs Dev Primary" && ((successful++)) || ((failed++))
deploy_node_exporter 20 "flarenode20" "4.211.206.210" "Docs Dev Secondary" && ((successful++)) || ((failed++))
deploy_node_exporter 21 "flarenode21" "4.178.181.122" "Docs Dev Tertiary" && ((successful++)) || ((failed++))

echo ""
echo "=================================================="
echo "Deployment Summary"
echo "=================================================="
echo -e "${GREEN}✅ Successful: $successful/16${NC}"
echo -e "${RED}❌ Failed: $failed/16${NC}"
echo ""

if [ $successful -gt 12 ]; then
    echo -e "${GREEN}🎉 Most validators deployed successfully!${NC}"
    exit 0
elif [ $successful -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Partial deployment - some validators accessible${NC}"
    exit 2
else
    echo -e "${RED}❌ Deployment failed - check firewall rules${NC}"
    exit 1
fi
