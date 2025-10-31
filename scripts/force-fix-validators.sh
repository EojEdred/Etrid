#!/usr/bin/env bash
# Force fix validators by killing processes and replacing binary

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔧 Force-Fixing Failed Validators (#14-17)"
echo "============================================"
echo ""

force_fix_validator() {
    local validator_num=$1
    local ssh_user=$2
    local ssh_host=$3
    local name=$4

    echo -e "${YELLOW}Force-fixing #$validator_num ($name)...${NC}"

    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" bash <<'FORCE_FIX_SCRIPT'
set -e

# Stop the service
sudo systemctl stop node_exporter 2>/dev/null || true

# Kill any running node_exporter processes
sudo pkill -9 node_exporter 2>/dev/null || true

# Wait for file to be released
sleep 3

# Remove old binary
sudo rm -f /usr/local/bin/node_exporter

# Download and install fresh
cd /tmp
rm -rf node_exporter-1.7.0.linux-amd64*
wget -q https://github.com/prometheus/node_exporter/releases/download/v1.7.0/node_exporter-1.7.0.linux-amd64.tar.gz
tar xzf node_exporter-1.7.0.linux-amd64.tar.gz
sudo cp node_exporter-1.7.0.linux-amd64/node_exporter /usr/local/bin/
sudo chmod +x /usr/local/bin/node_exporter
rm -rf node_exporter-1.7.0.linux-amd64*

# Recreate service
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

# Start fresh
sudo systemctl daemon-reload
sudo systemctl enable node_exporter
sudo systemctl start node_exporter

# Verify it's running
sleep 2
if systemctl is-active --quiet node_exporter; then
    echo "  ✓ Node exporter running successfully"
else
    echo "  ✗ Node exporter failed to start"
    exit 1
fi
FORCE_FIX_SCRIPT

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}  ✅ #$validator_num fixed and verified${NC}"
        return 0
    else
        echo -e "${RED}  ❌ #$validator_num still failed${NC}"
        return 1
    fi
}

# Force fix the 4 failed validators
successful=0
failed=0

force_fix_validator 14 "audit-dev01" "51.142.203.160" "EDSC Dev" && ((successful++)) || ((failed++))
force_fix_validator 15 "flarenode15" "172.166.164.19" "Economics Dev Primary" && ((successful++)) || ((failed++))
force_fix_validator 16 "flarenode16" "172.166.187.180" "Economics Dev Secondary" && ((successful++)) || ((failed++))
force_fix_validator 17 "flarenode17" "172.166.210.244" "Ethics Dev Primary" && ((successful++)) || ((failed++))

echo ""
echo "============================================"
echo "Force Fix Summary"
echo "============================================"
echo -e "${GREEN}✅ Fixed: $successful/4${NC}"
echo -e "${RED}❌ Still Failed: $failed/4${NC}"
echo ""

if [ $successful -eq 4 ]; then
    echo -e "${GREEN}🎉 All validators force-fixed successfully!${NC}"
    echo ""
    echo "Next: Insert validator keys on validators #6-13"
    exit 0
elif [ $successful -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Partial success - some validators fixed${NC}"
    exit 2
else
    echo -e "${RED}❌ All force-fix attempts failed${NC}"
    exit 1
fi
