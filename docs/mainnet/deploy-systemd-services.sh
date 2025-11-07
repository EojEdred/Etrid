#!/bin/bash
# FlareChain Mainnet - Systemd Service Deployment
# Deploys systemd service files to all 21 validator VMs

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Configuration
TEMPLATE_FILE="flarechain-validator.service.template"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE_PATH="$SCRIPT_DIR/$TEMPLATE_FILE"

# Bootnode addresses (update these with actual values after genesis)
BOOTNODES="/ip4/BOOTNODE_IP_1/tcp/30333/p2p/PEER_ID_1,/ip4/BOOTNODE_IP_2/tcp/30333/p2p/PEER_ID_2"

# Validator names (5 Directors + 16 ValidityNodes)
declare -A VALIDATORS=(
    # Directors (Decentralized Directors)
    ["oracle_vm1"]="GizziDirector"
    ["oracle_vm2"]="EojDirector"
    ["azure_vm1"]="AzureDirector1"
    ["azure_vm2"]="AzureDirector2"
    ["aws_vm1"]="AWSDirector"

    # ValidityNodes
    ["oracle_vm3"]="OracleValidator1"
    ["oracle_vm4"]="OracleValidator2"
    ["oracle_vm5"]="OracleValidator3"
    ["oracle_vm6"]="OracleValidator4"
    ["azure_vm3"]="AzureValidator1"
    ["azure_vm4"]="AzureValidator2"
    ["azure_vm5"]="AzureValidator3"
    ["azure_vm6"]="AzureValidator4"
    ["azure_vm7"]="AzureValidator5"
    ["azure_vm8"]="AzureValidator6"
    ["azure_vm9"]="AzureValidator7"
    ["azure_vm10"]="AzureValidator8"
    ["aws_vm2"]="AWSValidator1"
    ["aws_vm3"]="AWSValidator2"
    ["aws_vm4"]="AWSValidator3"
    ["local_vm1"]="LocalValidator"
)

# VM SSH access (update with actual IPs)
declare -A VM_IPS=(
    # Oracle Cloud (6 VMs)
    ["oracle_vm1"]="129.xxx.xxx.xxx"
    ["oracle_vm2"]="129.xxx.xxx.xxx"
    ["oracle_vm3"]="129.xxx.xxx.xxx"
    ["oracle_vm4"]="129.xxx.xxx.xxx"
    ["oracle_vm5"]="129.xxx.xxx.xxx"
    ["oracle_vm6"]="129.xxx.xxx.xxx"

    # Azure (10 VMs)
    ["azure_vm1"]="20.xxx.xxx.xxx"
    ["azure_vm2"]="20.xxx.xxx.xxx"
    ["azure_vm3"]="20.xxx.xxx.xxx"
    ["azure_vm4"]="20.xxx.xxx.xxx"
    ["azure_vm5"]="20.xxx.xxx.xxx"
    ["azure_vm6"]="20.xxx.xxx.xxx"
    ["azure_vm7"]="20.xxx.xxx.xxx"
    ["azure_vm8"]="20.xxx.xxx.xxx"
    ["azure_vm9"]="20.xxx.xxx.xxx"
    ["azure_vm10"]="20.xxx.xxx.xxx"

    # AWS (4 VMs)
    ["aws_vm1"]="3.xxx.xxx.xxx"
    ["aws_vm2"]="3.xxx.xxx.xxx"
    ["aws_vm3"]="3.xxx.xxx.xxx"
    ["aws_vm4"]="3.xxx.xxx.xxx"

    # Local (1 VM)
    ["local_vm1"]="192.168.x.x"
)

# SSH users per provider
declare -A SSH_USERS=(
    ["oracle"]="ubuntu"
    ["azure"]="azureuser"
    ["aws"]="ec2-user"
    ["local"]="validator"
)

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}FlareChain Systemd Service Deployment${NC}"
echo -e "${BLUE}========================================${NC}"
echo

# Check template exists
if [ ! -f "$TEMPLATE_PATH" ]; then
    echo -e "${RED}❌ Template file not found: $TEMPLATE_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Template found: $TEMPLATE_FILE"
echo

# Function to deploy to a single VM
deploy_to_vm() {
    local vm_id=$1
    local validator_name=${VALIDATORS[$vm_id]}
    local vm_ip=${VM_IPS[$vm_id]}

    # Determine SSH user based on VM prefix
    local ssh_user
    if [[ $vm_id == oracle_* ]]; then
        ssh_user="${SSH_USERS[oracle]}"
    elif [[ $vm_id == azure_* ]]; then
        ssh_user="${SSH_USERS[azure]}"
    elif [[ $vm_id == aws_* ]]; then
        ssh_user="${SSH_USERS[aws]}"
    else
        ssh_user="${SSH_USERS[local]}"
    fi

    echo -e "${YELLOW}Deploying to $vm_id ($validator_name)...${NC}"

    # Create customized service file
    local service_file="/tmp/flarechain-validator-$vm_id.service"
    sed -e "s|%VALIDATOR_NAME%|$validator_name|g" \
        -e "s|%BOOTNODES%|$BOOTNODES|g" \
        "$TEMPLATE_PATH" > "$service_file"

    # Deploy to VM
    scp -o StrictHostKeyChecking=no "$service_file" "${ssh_user}@${vm_ip}:/tmp/flarechain-validator.service" && \
    ssh -o StrictHostKeyChecking=no "${ssh_user}@${vm_ip}" "
        sudo mv /tmp/flarechain-validator.service /etc/systemd/system/ && \
        sudo systemctl daemon-reload && \
        echo 'Service installed successfully on $validator_name'
    " && \
    echo -e "${GREEN}✓${NC} $validator_name deployed" || \
    echo -e "${RED}✗${NC} $validator_name failed"

    # Cleanup temp file
    rm -f "$service_file"
}

# Deploy to all VMs in parallel (4 at a time)
echo -e "${BLUE}Deploying systemd services to all 21 validators...${NC}"
echo

# Export functions and variables for parallel execution
export -f deploy_to_vm
export TEMPLATE_PATH BOOTNODES
export -A VALIDATORS VM_IPS SSH_USERS

# Deploy in parallel batches
for vm_id in "${!VALIDATORS[@]}"; do
    deploy_to_vm "$vm_id" &

    # Limit to 4 parallel deployments
    if [[ $(jobs -r -p | wc -l) -ge 4 ]]; then
        wait -n
    fi
done

# Wait for all remaining deployments
wait

echo
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✓ Systemd service deployment complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Verify services are installed:"
echo "   ssh <vm> 'sudo systemctl status flarechain-validator'"
echo
echo "2. Start validators:"
echo "   ssh <vm> 'sudo systemctl start flarechain-validator'"
echo
echo "3. Enable auto-start on boot:"
echo "   ssh <vm> 'sudo systemctl enable flarechain-validator'"
echo
echo "4. View logs:"
echo "   ssh <vm> 'sudo journalctl -u flarechain-validator -f'"
echo
