#!/bin/bash
################################################################################
# ËTRID FLARECHAIN - DEPLOY REPO TO ALL 21 VALIDATORS
# This script pulls the latest repository to all 21 validator VMs
################################################################################

set -e

REPO_URL="https://github.com/EojEdred/Etrid.git"
REPO_DIR="/opt/etrid"
SSH_KEY="${HOME}/.ssh/gizzi-validator"
LOG_FILE="/tmp/deploy-to-validators.log"

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "═══════════════════════════════════════════════════════════════════════"
echo "  ËTRID FLARECHAIN - DEPLOYING TO ALL 21 VALIDATORS"
echo "═══════════════════════════════════════════════════════════════════════"
echo ""
echo "Repository: ${REPO_URL}"
echo "Target Directory: ${REPO_DIR}"
echo "SSH Key: ${SSH_KEY}"
echo "Log File: ${LOG_FILE}"
echo ""
echo "> /dev/null"

# Read validator IPs from JSON file
VALIDATOR_IPS=($(cat infrastructure/config/validator-ips.json | grep '"ip"' | awk -F'"' '{print $4}' | grep -v NEEDS))

echo "Found ${#VALIDATOR_IPS[@]} validators"
echo ""

# Counters
SUCCESS_COUNT=0
FAILED_COUNT=0
declare -a FAILED_VMS

# Function to deploy to a single VM
deploy_to_vm() {
    local IP=$1
    local INDEX=$2
    local TOTAL=$3

    echo "─────────────────────────────────────────────────────────────────────"
    echo "[${INDEX}/${TOTAL}] Deploying to ${IP}..."

    # Try ubuntu user first (Oracle + some Azure)
    local USER="ubuntu"

    # Test SSH connectivity
    if ! ssh -i "${SSH_KEY}" -o ConnectTimeout=5 -o StrictHostKeyChecking=no "${USER}@${IP}" "echo 'SSH OK'" &>/dev/null; then
        # Try azureuser if ubuntu fails
        USER="azureuser"
        if ! ssh -i "${SSH_KEY}" -o ConnectTimeout=5 -o StrictHostKeyChecking=no "${USER}@${IP}" "echo 'SSH OK'" &>/dev/null; then
            echo -e "${RED}✗${NC} Failed to connect to ${IP} (tried both ubuntu and azureuser)"
            echo "[${IP}] SSH connection failed" >> "${LOG_FILE}"
            FAILED_COUNT=$((FAILED_COUNT + 1))
            FAILED_VMS+=("${IP}")
            return 1
        fi
    fi

    echo "  User: ${USER}"

    # Deploy the repository
    ssh -i "${SSH_KEY}" -o StrictHostKeyChecking=no "${USER}@${IP}" bash <<'ENDSSH' 2>>"${LOG_FILE}"
        set -e

        # Create directory if it doesn't exist
        sudo mkdir -p /opt/etrid
        sudo chown -R $(whoami):$(whoami) /opt/etrid

        # Clone or pull repository
        if [ -d "/opt/etrid/.git" ]; then
            echo "  Repository exists, pulling latest changes..."
            cd /opt/etrid
            git fetch origin
            git reset --hard origin/main
            git pull origin main
        else
            echo "  Cloning repository..."
            git clone https://github.com/EojEdred/Etrid.git /opt/etrid
            cd /opt/etrid
        fi

        # Show current commit
        echo "  Current commit: $(git rev-parse --short HEAD) - $(git log -1 --pretty=%B | head -1)"
ENDSSH

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} Successfully deployed to ${IP}"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        echo "[${IP}] Deployment successful" >> "${LOG_FILE}"
        return 0
    else
        echo -e "${RED}✗${NC} Failed to deploy to ${IP}"
        FAILED_COUNT=$((FAILED_COUNT + 1))
        FAILED_VMS+=("${IP}")
        echo "[${IP}] Deployment failed" >> "${LOG_FILE}"
        return 1
    fi
}

# Deploy to all validators
TOTAL=${#VALIDATOR_IPS[@]}
INDEX=1

for IP in "${VALIDATOR_IPS[@]}"; do
    deploy_to_vm "${IP}" "${INDEX}" "${TOTAL}"
    INDEX=$((INDEX + 1))
    echo ""
done

echo "═══════════════════════════════════════════════════════════════════════"
echo "  DEPLOYMENT COMPLETE"
echo "═══════════════════════════════════════════════════════════════════════"
echo ""
echo -e "Total Validators: ${TOTAL}"
echo -e "${GREEN}Successful:${NC} ${SUCCESS_COUNT}"
echo -e "${RED}Failed:${NC} ${FAILED_COUNT}"
echo ""

if [ ${FAILED_COUNT} -gt 0 ]; then
    echo -e "${YELLOW}Failed VMs:${NC}"
    for VM in "${FAILED_VMS[@]}"; do
        echo "  - ${VM}"
    done
    echo ""
    echo "Check log file for details: ${LOG_FILE}"
    exit 1
else
    echo -e "${GREEN}All validators deployed successfully!${NC}"
    exit 0
fi
