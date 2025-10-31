#!/usr/bin/env bash
# Fix validators #14-17 that failed due to "Text file busy"

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔧 Fixing Failed Validators (#14-17)"
echo "======================================"
echo ""

fix_validator() {
    local validator_num=$1
    local ssh_user=$2
    local ssh_host=$3
    local name=$4

    echo -e "${YELLOW}Fixing #$validator_num ($name)...${NC}"

    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" bash <<'FIX_SCRIPT'
set -e

# Stop the service if running
sudo systemctl stop node_exporter 2>/dev/null || true

# Wait a moment for the file to be released
sleep 2

# Download and replace
cd /tmp
rm -rf node_exporter-1.7.0.linux-amd64*
wget -q https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz
tar xzf node_exporter-1.7.0.linux-amd64.tar.gz
sudo cp node_exporter-1.7.0.linux-amd64/node_exporter /usr/local/bin/
rm -rf node_exporter-1.7.0.linux-amd64*

# Ensure systemd service exists
if [ ! -f /etc/systemd/system/node_exporter.service ]; then
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
fi

# Start service
sudo systemctl daemon-reload
sudo systemctl enable node_exporter
sudo systemctl start node_exporter

echo "  ✓ Node exporter fixed and restarted"
FIX_SCRIPT

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}  ✅ #$validator_num fixed${NC}"
        return 0
    else
        echo -e "${RED}  ❌ #$validator_num still failed${NC}"
        return 1
    fi
}

# Fix the 4 failed validators
successful=0
failed=0

fix_validator 14 "audit-dev01" "51.142.203.160" "EDSC Dev" && ((successful++)) || ((failed++))
fix_validator 15 "flarenode15" "172.166.164.19" "Economics Dev Primary" && ((successful++)) || ((failed++))
fix_validator 16 "flarenode16" "172.166.187.180" "Economics Dev Secondary" && ((successful++)) || ((failed++))
fix_validator 17 "flarenode17" "172.166.210.244" "Ethics Dev Primary" && ((successful++)) || ((failed++))

echo ""
echo "======================================"
echo "Fix Summary"
echo "======================================"
echo -e "${GREEN}✅ Fixed: $successful/4${NC}"
echo -e "${RED}❌ Still Failed: $failed/4${NC}"
echo ""

if [ $successful -eq 4 ]; then
    echo -e "${GREEN}🎉 All validators fixed!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some validators still need attention${NC}"
    exit 1
fi
