#!/bin/bash
#
# FlareChain Validator Deployment Script
# Automates deployment of a single validator to the mainnet
#
# Usage: ./deploy-validator.sh <validator_index> <server_ip> <validator_name>
# Example: ./deploy-validator.sh 3 1.2.3.4 "Validator-3"
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
VALIDATOR_INDEX=$1
SERVER_IP=$2
VALIDATOR_NAME=$3
BINARY_PATH="/Users/macbook/Desktop/etrid/target/release/flarechain-node"
CHAINSPEC_PATH="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
KEYS_FILE="/Users/macbook/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json"
BOOTNODES="/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/129.80.122.34/tcp/30333/p2p/12D3KooWGjGCJzexrJct6nGCSDnj7vaJtMohpagFUPBhPgpZqvpd"

# Validate arguments
if [ -z "$VALIDATOR_INDEX" ] || [ -z "$SERVER_IP" ] || [ -z "$VALIDATOR_NAME" ]; then
    echo -e "${RED}Error: Missing required arguments${NC}"
    echo "Usage: $0 <validator_index> <server_ip> <validator_name>"
    echo "Example: $0 3 1.2.3.4 \"Validator-3\""
    exit 1
fi

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  FlareChain Mainnet - Validator Deployment Script            ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Deploying:${NC} Validator #$VALIDATOR_INDEX"
echo -e "${GREEN}Server IP:${NC} $SERVER_IP"
echo -e "${GREEN}Name:${NC} $VALIDATOR_NAME"
echo ""

# Extract session keys from JSON
echo -e "${YELLOW}[1/8] Extracting session keys...${NC}"
MNEMONIC=$(jq -r ".validators[$((VALIDATOR_INDEX-1))].sessionKeys.phrase" "$KEYS_FILE")
ACCOUNT_ID=$(jq -r ".validators[$((VALIDATOR_INDEX-1))].sessionKeys.accountId" "$KEYS_FILE")

if [ "$MNEMONIC" == "null" ] || [ -z "$MNEMONIC" ]; then
    echo -e "${RED}Error: Could not extract mnemonic for validator $VALIDATOR_INDEX${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Account ID: $ACCOUNT_ID${NC}"
echo ""

# Upload binary and chainspec
echo -e "${YELLOW}[2/8] Uploading files to server...${NC}"
scp "$BINARY_PATH" "ubuntu@$SERVER_IP:/tmp/flarechain-node"
scp "$CHAINSPEC_PATH" "ubuntu@$SERVER_IP:/tmp/chainspec-mainnet-raw-FIXED.json"
echo -e "${GREEN}✓ Files uploaded${NC}"
echo ""

# Setup server
echo -e "${YELLOW}[3/8] Setting up server...${NC}"
ssh "ubuntu@$SERVER_IP" bash << 'EOSSH'
# Move files
sudo mv /tmp/flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node
sudo mkdir -p /home/ubuntu/flarechain-data
sudo mv /tmp/chainspec-mainnet-raw-FIXED.json /home/ubuntu/
sudo chown -R ubuntu:ubuntu /home/ubuntu/flarechain-data

# Create network directory
sudo mkdir -p /home/ubuntu/flarechain-data/chains/flarechain_mainnet/network
sudo chown -R ubuntu:ubuntu /home/ubuntu/flarechain-data
EOSSH
echo -e "${GREEN}✓ Server setup complete${NC}"
echo ""

# Insert session keys
echo -e "${YELLOW}[4/8] Inserting session keys...${NC}"

# AURA key
ssh "ubuntu@$SERVER_IP" /usr/local/bin/flarechain-node key insert \
    --base-path /home/ubuntu/flarechain-data \
    --chain /home/ubuntu/chainspec-mainnet-raw-FIXED.json \
    --key-type aura \
    --scheme sr25519 \
    --suri "\"$MNEMONIC\""

# GRANDPA key
ssh "ubuntu@$SERVER_IP" /usr/local/bin/flarechain-node key insert \
    --base-path /home/ubuntu/flarechain-data \
    --chain /home/ubuntu/chainspec-mainnet-raw-FIXED.json \
    --key-type gran \
    --scheme ed25519 \
    --suri "\"$MNEMONIC\""

# ASF key
ssh "ubuntu@$SERVER_IP" /usr/local/bin/flarechain-node key insert \
    --base-path /home/ubuntu/flarechain-data \
    --chain /home/ubuntu/chainspec-mainnet-raw-FIXED.json \
    --key-type asfk \
    --scheme sr25519 \
    --suri "\"$MNEMONIC\""

echo -e "${GREEN}✓ Session keys inserted (AURA, GRANDPA, ASF)${NC}"
echo ""

# Generate network key
echo -e "${YELLOW}[5/8] Generating network key...${NC}"
ssh "ubuntu@$SERVER_IP" /usr/local/bin/flarechain-node key generate-node-key \
    --file /home/ubuntu/flarechain-data/chains/flarechain_mainnet/network/secret_ed25519

ssh "ubuntu@$SERVER_IP" sudo chown -R ubuntu:ubuntu /home/ubuntu/flarechain-data
echo -e "${GREEN}✓ Network key generated${NC}"
echo ""

# Create systemd service
echo -e "${YELLOW}[6/8] Creating systemd service...${NC}"
ssh "ubuntu@$SERVER_IP" sudo tee /etc/systemd/system/flarechain-validator.service > /dev/null << EOSERVICE
[Unit]
Description=FlareChain Validator Node - $VALIDATOR_NAME
Documentation=https://docs.etrid.com
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=ubuntu
Group=ubuntu
WorkingDirectory=/home/ubuntu

ExecStart=/usr/local/bin/flarechain-node \\
  --chain=/home/ubuntu/chainspec-mainnet-raw-FIXED.json \\
  --base-path=/home/ubuntu/flarechain-data \\
  --validator \\
  --name="$VALIDATOR_NAME" \\
  --port=30333 \\
  --rpc-port=9944 \\
  --prometheus-port=9615 \\
  --prometheus-external \\
  --rpc-cors=all \\
  --rpc-methods=Unsafe \\
  --rpc-external \\
  --unsafe-rpc-external \\
  --public-addr=/ip4/$SERVER_IP/tcp/30333 \\
  --bootnodes=$BOOTNODES

LimitNOFILE=65536
LimitNPROC=4096

Restart=always
RestartSec=10
StartLimitInterval=600
StartLimitBurst=5

StandardOutput=journal
StandardError=journal
SyslogIdentifier=flarechain-validator

[Install]
WantedBy=multi-user.target
EOSERVICE

ssh "ubuntu@$SERVER_IP" sudo systemctl daemon-reload
ssh "ubuntu@$SERVER_IP" sudo systemctl enable flarechain-validator
echo -e "${GREEN}✓ Systemd service created${NC}"
echo ""

# Configure firewall
echo -e "${YELLOW}[7/8] Configuring firewall...${NC}"
ssh "ubuntu@$SERVER_IP" bash << 'EOSSH'
sudo ufw allow 30333/tcp comment "Substrate P2P"
sudo ufw allow 30334/tcp comment "DETR P2P TCP"
sudo ufw allow 30334/udp comment "DETR P2P UDP"
sudo ufw allow 9944/tcp comment "RPC"
sudo ufw allow 9615/tcp comment "Prometheus"
sudo ufw --force enable
EOSSH
echo -e "${GREEN}✓ Firewall configured${NC}"
echo ""

# Start validator
echo -e "${YELLOW}[8/8] Starting validator...${NC}"
ssh "ubuntu@$SERVER_IP" sudo systemctl start flarechain-validator
sleep 5

# Check status
echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Validator deployed successfully!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Get peer ID
echo -e "${YELLOW}Retrieving Peer ID...${NC}"
sleep 10
PEER_ID=$(ssh "ubuntu@$SERVER_IP" sudo journalctl -u flarechain-validator -n 200 --no-pager | grep "Local node identity" | tail -1 | awk '{print $NF}')

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              VALIDATOR DEPLOYMENT SUMMARY                      ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Validator:${NC}      $VALIDATOR_NAME (Validator #$VALIDATOR_INDEX)"
echo -e "${BLUE}Account ID:${NC}     $ACCOUNT_ID"
echo -e "${BLUE}Server IP:${NC}      $SERVER_IP"
echo -e "${BLUE}Peer ID:${NC}        $PEER_ID"
echo -e "${BLUE}Bootnode:${NC}       /ip4/$SERVER_IP/tcp/30333/p2p/$PEER_ID"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Monitor logs:    ssh ubuntu@$SERVER_IP 'sudo journalctl -u flarechain-validator -f'"
echo "2. Check peers:     curl -s http://$SERVER_IP:9944 -d '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_health\"}' | jq"
echo "3. Check sync:      curl -s http://$SERVER_IP:9944 -d '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_syncState\"}' | jq"
echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Save deployment info
echo "Validator #$VALIDATOR_INDEX deployed at $(date)" >> /tmp/validator-deployments.log
echo "  Name: $VALIDATOR_NAME" >> /tmp/validator-deployments.log
echo "  IP: $SERVER_IP" >> /tmp/validator-deployments.log
echo "  Peer ID: $PEER_ID" >> /tmp/validator-deployments.log
echo "  Bootnode: /ip4/$SERVER_IP/tcp/30333/p2p/$PEER_ID" >> /tmp/validator-deployments.log
echo "" >> /tmp/validator-deployments.log

echo -e "${GREEN}✓ Deployment logged to /tmp/validator-deployments.log${NC}"
