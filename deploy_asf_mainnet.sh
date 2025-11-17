#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FlareChain ASF Mainnet Deployment Script
# ═══════════════════════════════════════════════════════════════════════════════
#
# This script deploys the Pure ASF mainnet configuration to all 20 validators
#
# Prerequisites:
# 1. etrid binary built (./target/release/etrid)
# 2. ASF raw chainspec generated (./flarechain_production_raw.json)
# 3. SSH access to all validator servers
#
# Usage:
#   ./deploy_asf_mainnet.sh
#
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BINARY_PATH="./target/release/etrid"
CHAINSPEC_PATH="./flarechain_production_raw.json"
REMOTE_USER="ubuntu"
REMOTE_BINARY_PATH="/usr/local/bin/etrid"
REMOTE_CHAINSPEC_PATH="/etc/etrid/flarechain_production.json"
REMOTE_BASE_PATH="/var/lib/etrid"

# Validator server IPs (update these with your actual IPs)
VALIDATORS=(
    "146.190.136.56"   # val-1
    "143.198.151.42"   # val-2
    "164.90.218.73"    # val-3
    "167.172.35.194"   # val-4
    "159.223.209.145"  # val-5
    "157.230.52.181"   # val-6
    "142.93.201.67"    # val-7
    "147.182.255.38"   # val-8
    "167.99.238.102"   # val-9
    "159.223.195.221"  # val-10
    "165.232.141.89"   # val-11
    "138.197.197.45"   # val-12
    "159.89.50.178"    # val-13
    "174.138.70.231"   # val-14
    "143.198.115.67"   # val-15
    "138.197.201.134"  # val-16
    "165.232.137.92"   # val-17
    "157.245.108.156"  # val-18
    "134.209.81.203"   # val-19
    "146.190.80.142"   # val-20
)

echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}    FlareChain Pure ASF Mainnet Deployment${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}✗ Binary not found: $BINARY_PATH${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Binary found: $BINARY_PATH ($(du -h $BINARY_PATH | cut -f1))${NC}"

if [ ! -f "$CHAINSPEC_PATH" ]; then
    echo -e "${RED}✗ Chainspec not found: $CHAINSPEC_PATH${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Chainspec found: $CHAINSPEC_PATH ($(wc -l < $CHAINSPEC_PATH) lines)${NC}"

echo -e "${GREEN}✓ Ready to deploy to ${#VALIDATORS[@]} validators${NC}"
echo ""

# Confirmation prompt
read -p "Deploy to ALL ${#VALIDATORS[@]} validators? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    echo -e "${YELLOW}Deployment cancelled${NC}"
    exit 0
fi

echo ""
echo -e "${BLUE}Starting deployment...${NC}"
echo ""

# Deployment function
deploy_to_validator() {
    local ip=$1
    local index=$2
    local validator_name="val-$index"

    echo -e "${BLUE}[$validator_name] Deploying to $ip...${NC}"

    # 1. Upload binary
    echo -e "  Uploading etrid binary..."
    if scp -o StrictHostKeyChecking=no "$BINARY_PATH" "$REMOTE_USER@$ip:/tmp/etrid" 2>&1 | grep -q "100%"; then
        echo -e "  ${GREEN}✓ Binary uploaded${NC}"
    else
        echo -e "  ${RED}✗ Binary upload failed${NC}"
        return 1
    fi

    # 2. Upload chainspec
    echo -e "  Uploading chainspec..."
    if scp -o StrictHostKeyChecking=no "$CHAINSPEC_PATH" "$REMOTE_USER@$ip:/tmp/chainspec.json" 2>&1 | grep -q "100%"; then
        echo -e "  ${GREEN}✓ Chainspec uploaded${NC}"
    else
        echo -e "  ${RED}✗ Chainspec upload failed${NC}"
        return 1
    fi

    # 3. Execute remote deployment
    ssh -o StrictHostKeyChecking=no "$REMOTE_USER@$ip" << 'ENDSSH'
        # Stop old node
        echo "  Stopping old flarechain-node..."
        sudo systemctl stop flarechain-node 2>/dev/null || true
        sudo pkill -9 flarechain-node 2>/dev/null || true

        # Install new binary
        echo "  Installing etrid binary..."
        sudo mv /tmp/etrid /usr/local/bin/etrid
        sudo chmod +x /usr/local/bin/etrid

        # Install chainspec
        echo "  Installing chainspec..."
        sudo mkdir -p /etc/etrid
        sudo mv /tmp/chainspec.json /etc/etrid/flarechain_production.json

        # Backup old database (optional)
        echo "  Backing up old database..."
        sudo mv /var/lib/etrid/chains /var/lib/etrid/chains.backup.$(date +%Y%m%d_%H%M%S) 2>/dev/null || true

        # Generate node key if not exists
        if [ ! -f /etc/etrid/node-key.secret ]; then
            echo "  Generating node key..."
            openssl rand -hex 32 | sudo tee /etc/etrid/node-key.secret > /dev/null
            sudo chmod 600 /etc/etrid/node-key.secret
        fi

        # Update systemd service
        echo "  Updating systemd service..."
        sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<EOF
[Unit]
Description=Etrid FlareChain Validator (Pure ASF)
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/var/lib/etrid
ExecStart=/usr/local/bin/etrid \\
  --chain /etc/etrid/flarechain_production.json \\
  --validator \\
  --base-path /var/lib/etrid \\
  --port 30333 \\
  --rpc-port 9944 \\
  --prometheus-port 9615 \\
  --prometheus-external \\
  --node-key \$(cat /etc/etrid/node-key.secret) \\
  --name "FlareChain-Validator-\$(hostname)" \\
  --rpc-cors all \\
  --unsafe-rpc-external

Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

        # Reload systemd and start
        echo "  Starting etrid validator..."
        sudo systemctl daemon-reload
        sudo systemctl enable etrid-validator
        sudo systemctl start etrid-validator

        # Wait for startup
        sleep 5

        # Check status
        if sudo systemctl is-active --quiet etrid-validator; then
            echo "  ✓ Validator started successfully"
            exit 0
        else
            echo "  ✗ Validator failed to start"
            exit 1
        fi
ENDSSH

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}[$validator_name] ✓ Deployment successful${NC}"
        return 0
    else
        echo -e "${RED}[$validator_name] ✗ Deployment failed${NC}"
        return 1
    fi
}

# Deploy to all validators
successful=0
failed=0

for i in "${!VALIDATORS[@]}"; do
    ip="${VALIDATORS[$i]}"
    index=$((i + 1))

    if deploy_to_validator "$ip" "$index"; then
        ((successful++))
    else
        ((failed++))
    fi

    echo ""
done

# Summary
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}    Deployment Summary${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}Successful: $successful${NC}"
echo -e "${RED}Failed: $failed${NC}"
echo -e "${BLUE}Total: ${#VALIDATORS[@]}${NC}"
echo ""

if [ $failed -eq 0 ]; then
    echo -e "${GREEN}✓ All validators deployed successfully!${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo "1. Monitor block production: watch 'curl -s http://146.190.136.56:9944 | jq .'"
    echo "2. Check validator logs: ssh ubuntu@146.190.136.56 'sudo journalctl -u etrid-validator -f'"
    echo "3. Verify finality is working across all validators"
else
    echo -e "${RED}⚠ Some deployments failed. Please check the failed validators manually.${NC}"
fi

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
