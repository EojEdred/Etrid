#!/bin/bash
# Deploy Mainnet to All 21 Validators
# This script automates the full deployment process

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     Ëtrid FlareChain Mainnet Deployment Automation        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if validator VMs file exists
if [ ! -f "validator-vms-numbered.txt" ]; then
    echo -e "${RED}❌ Error: validator-vms-numbered.txt not found${NC}"
    echo ""
    echo "Create this file with format:"
    echo "1 ubuntu@64.181.215.19"
    echo "2 ubuntu@<validator-2-ip>"
    echo "..."
    exit 1
fi

# Verify we have 21 validators
VALIDATOR_COUNT=$(wc -l < validator-vms-numbered.txt)
if [ "$VALIDATOR_COUNT" -ne 21 ]; then
    echo -e "${YELLOW}⚠️  Warning: Found $VALIDATOR_COUNT validators, expected 21${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if deployment package exists
if [ ! -f "mainnet-deployment.tar.gz" ]; then
    echo -e "${RED}❌ Error: mainnet-deployment.tar.gz not found${NC}"
    echo "Run the following first:"
    echo "  1. Build mainnet binary"
    echo "  2. Generate chain spec"
    echo "  3. Create deployment package"
    exit 1
fi

echo -e "${BLUE}Deployment Summary:${NC}"
echo "  - Validators: $VALIDATOR_COUNT"
echo "  - Package: $(ls -lh mainnet-deployment.tar.gz | awk '{print $5}')"
echo ""

read -p "Proceed with deployment? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 0
fi

# Phase 1: Upload deployment package
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 1: Uploading deployment package to all VMs...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

upload_count=0
while read num vm; do
    echo -e "${GREEN}[$((upload_count+1))/$VALIDATOR_COUNT]${NC} Uploading to $vm (Validator $num)..."
    scp -i ~/.ssh/gizzi-validator -o StrictHostKeyChecking=no mainnet-deployment.tar.gz "$vm:/home/ubuntu/" 2>/dev/null
    upload_count=$((upload_count+1))
done < validator-vms-numbered.txt

echo -e "${GREEN}✅ Upload complete: $upload_count/$VALIDATOR_COUNT validators${NC}"

# Phase 2: Extract and install
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 2: Installing binaries on all VMs...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

install_count=0
while read num vm; do
    echo -e "${GREEN}[$((install_count+1))/$VALIDATOR_COUNT]${NC} Installing on $vm..."
    ssh -i ~/.ssh/gizzi-validator -o StrictHostKeyChecking=no "$vm" bash << 'EOSSH' 2>/dev/null
        cd /home/ubuntu
        tar -xzf mainnet-deployment.tar.gz
        sudo cp mainnet-deployment-package/flarechain-node /usr/local/bin/
        sudo chmod +x /usr/local/bin/flarechain-node
        sudo mkdir -p /var/lib/etrid
        sudo chown ubuntu:ubuntu /var/lib/etrid
EOSSH
    install_count=$((install_count+1))
done < validator-vms-numbered.txt

echo -e "${GREEN}✅ Installation complete: $install_count/$VALIDATOR_COUNT validators${NC}"

# Phase 3: Insert session keys
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 3: Inserting session keys...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

keys_count=0
while read num vm; do
    echo -e "${GREEN}[$((keys_count+1))/$VALIDATOR_COUNT]${NC} Inserting keys for Validator $num..."

    # Extract keys from local file
    AURA_SEED=$(jq -r ".validators[$((num-1))].sessionKeys.auraSeed" mainnet-deployment-package/validator-keys-complete.json)
    GRANDPA_SEED=$(jq -r ".validators[$((num-1))].sessionKeys.grandpaSeed" mainnet-deployment-package/validator-keys-complete.json)
    ASF_SEED=$(jq -r ".validators[$((num-1))].asfKeys.secretSeed" mainnet-deployment-package/validator-keys-complete.json)

    # Insert keys on remote VM
    ssh -i ~/.ssh/gizzi-validator -o StrictHostKeyChecking=no "$vm" bash << EOSSH 2>/dev/null
        # AURA key
        /usr/local/bin/flarechain-node key insert \
          --base-path /var/lib/etrid \
          --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
          --scheme Sr25519 \
          --suri "$AURA_SEED" \
          --key-type aura

        # GRANDPA key
        /usr/local/bin/flarechain-node key insert \
          --base-path /var/lib/etrid \
          --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
          --scheme Ed25519 \
          --suri "$GRANDPA_SEED" \
          --key-type gran

        # ASF key
        /usr/local/bin/flarechain-node key insert \
          --base-path /var/lib/etrid \
          --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
          --scheme Sr25519 \
          --suri "$ASF_SEED" \
          --key-type afsf
EOSSH
    keys_count=$((keys_count+1))
done < validator-vms-numbered.txt

echo -e "${GREEN}✅ Keys inserted: $keys_count/$VALIDATOR_COUNT validators${NC}"

# Phase 4: Create systemd services
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 4: Creating systemd services...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

service_count=0
while read num vm; do
    VALIDATOR_NAME=$(jq -r ".validators[$((num-1))].name" mainnet-deployment-package/validator-keys-complete.json)
    echo -e "${GREEN}[$((service_count+1))/$VALIDATOR_COUNT]${NC} Creating service for $VALIDATOR_NAME..."

    ssh -i ~/.ssh/gizzi-validator -o StrictHostKeyChecking=no "$vm" bash << EOSSH 2>/dev/null
        sudo tee /etc/systemd/system/flarechain-validator.service > /dev/null << 'EOSERVICE'
[Unit]
Description=Ëtrid FlareChain Mainnet Validator - $VALIDATOR_NAME
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu
ExecStart=/usr/local/bin/flarechain-node \
  --base-path /var/lib/etrid \
  --chain /home/ubuntu/mainnet-deployment-package/flarechain-mainnet-raw.json \
  --name "$VALIDATOR_NAME" \
  --validator \
  --port 30333 \
  --rpc-port 9944 \
  --prometheus-port 9615 \
  --rpc-methods Safe \
  --rpc-cors all \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'

Restart=always
RestartSec=10
LimitNOFILE=10000

[Install]
WantedBy=multi-user.target
EOSERVICE

        sudo systemctl daemon-reload
        sudo systemctl enable flarechain-validator
EOSSH
    service_count=$((service_count+1))
done < validator-vms-numbered.txt

echo -e "${GREEN}✅ Services created: $service_count/$VALIDATOR_COUNT validators${NC}"

# Summary
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║           Deployment Complete!                             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}✅ All validators are ready to start!${NC}"
echo ""
echo "📋 Summary:"
echo "  - Uploaded: $upload_count/$VALIDATOR_COUNT"
echo "  - Installed: $install_count/$VALIDATOR_COUNT"
echo "  - Keys inserted: $keys_count/$VALIDATOR_COUNT"
echo "  - Services created: $service_count/$VALIDATOR_COUNT"
echo ""
echo -e "${YELLOW}⚠️  VALIDATORS ARE NOT STARTED YET${NC}"
echo ""
echo "Next steps:"
echo "  1. Review deployment logs above"
echo "  2. Run: ./start-all-validators.sh"
echo "  3. Monitor with: ./monitor-mainnet.sh"
echo ""
