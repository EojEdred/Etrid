#!/usr/bin/env bash
# Configure Azure NSG rules for all validators
# This script provides the commands to run - Azure CLI must be authenticated

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "🔥 Azure NSG Configuration for Ëtrid Validators"
echo "================================================"
echo ""

# Check if Azure CLI is installed
if ! command -v az &> /dev/null; then
    echo -e "${RED}❌ Azure CLI not installed${NC}"
    echo ""
    echo "Install Azure CLI:"
    echo "  macOS: brew install azure-cli"
    echo "  Linux: curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash"
    echo ""
    exit 1
fi

# Check if logged in
if ! az account show &> /dev/null; then
    echo -e "${YELLOW}⚠️  Not logged into Azure${NC}"
    echo ""
    echo "Login to Azure:"
    echo "  az login"
    echo ""
    exit 1
fi

echo -e "${GREEN}✅ Azure CLI ready${NC}"
echo ""

# Get current public IP for SSH access
MYIP=$(curl -s https://api.ipify.org)
echo -e "${BLUE}Your public IP: $MYIP${NC}"
echo ""

# Monitoring server IP
MONITORING_IP="98.71.91.84"

# Required ports
echo "Required firewall rules for each validator:"
echo "  1. SSH (22) from $MYIP"
echo "  2. Prometheus metrics (9615) from $MONITORING_IP"
echo "  3. Node Exporter (9100) from $MONITORING_IP"
echo "  4. P2P networking (30333) from 0.0.0.0/0"
echo ""

# Function to configure NSG for a VM
configure_nsg() {
    local vm_name=$1
    local resource_group=$2

    echo -e "${YELLOW}Configuring NSG for $vm_name...${NC}"

    # Get NSG name
    local nsg_name=$(az network nsg list --resource-group "$resource_group" --query "[?contains(id, '$vm_name')].name" -o tsv)

    if [ -z "$nsg_name" ]; then
        echo -e "${RED}❌ Could not find NSG for $vm_name${NC}"
        return 1
    fi

    echo "  NSG: $nsg_name"

    # Add SSH rule
    echo "  Adding SSH rule..."
    az network nsg rule create \
        --resource-group "$resource_group" \
        --nsg-name "$nsg_name" \
        --name "Allow-SSH-MyIP" \
        --priority 100 \
        --source-address-prefixes "$MYIP/32" \
        --destination-port-ranges 22 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --description "SSH from admin IP" \
        --output none 2>/dev/null || echo "    (Rule may already exist)"

    # Add Prometheus rule
    echo "  Adding Prometheus rule..."
    az network nsg rule create \
        --resource-group "$resource_group" \
        --nsg-name "$nsg_name" \
        --name "Allow-Prometheus-Monitoring" \
        --priority 110 \
        --source-address-prefixes "$MONITORING_IP/32" \
        --destination-port-ranges 9615 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --description "Prometheus metrics from monitoring server" \
        --output none 2>/dev/null || echo "    (Rule may already exist)"

    # Add Node Exporter rule
    echo "  Adding Node Exporter rule..."
    az network nsg rule create \
        --resource-group "$resource_group" \
        --nsg-name "$nsg_name" \
        --name "Allow-NodeExporter-Monitoring" \
        --priority 120 \
        --source-address-prefixes "$MONITORING_IP/32" \
        --destination-port-ranges 9100 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --description "Node Exporter metrics from monitoring server" \
        --output none 2>/dev/null || echo "    (Rule may already exist)"

    # Add P2P rule
    echo "  Adding P2P networking rule..."
    az network nsg rule create \
        --resource-group "$resource_group" \
        --nsg-name "$nsg_name" \
        --name "Allow-P2P-Blockchain" \
        --priority 130 \
        --source-address-prefixes "*" \
        --destination-port-ranges 30333 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --description "P2P blockchain networking" \
        --output none 2>/dev/null || echo "    (Rule may already exist)"

    echo -e "${GREEN}  ✅ $vm_name configured${NC}"
    echo ""
}

# List of Azure VMs to configure (validators #1-13)
# Note: You'll need to update these with your actual resource group names

echo -e "${YELLOW}Configuring validators #1-13...${NC}"
echo ""

# Example commands - UPDATE WITH YOUR ACTUAL RESOURCE GROUPS
# Get resource groups:
echo "First, find your resource groups:"
echo "  az vm list --query '[].{Name:name, ResourceGroup:resourceGroup}' -o table"
echo ""

# You can run these manually or update the script with your resource groups
cat << 'MANUAL_COMMANDS'
# MANUAL CONFIGURATION COMMANDS
# Copy and run these after updating resource group names:

# Validator #1 - Gizzi (20.186.91.207)
# configure_nsg "etrid-validator-01" "YOUR_RESOURCE_GROUP"

# Validator #2 - EojEdred (172.177.44.73)
# configure_nsg "etrid-validator-02" "YOUR_RESOURCE_GROUP"

# Validator #3 - Governance Dev (20.186.91.207 - shares with #1)
# Same VM as #1, no separate NSG

# Validator #4 - Security Dev (52.252.142.146)
# configure_nsg "etrid-security-dev" "YOUR_RESOURCE_GROUP"

# Validator #5 - Audit Dev (132.145.145.135) - ORACLE CLOUD
# Use Oracle Cloud Console to configure Security Lists

# Validators #6-13 - Various
# configure_nsg "etrid-consensus-dev-secondary" "YOUR_RESOURCE_GROUP"
# configure_nsg "etrid-runtime-dev-primary" "YOUR_RESOURCE_GROUP"
# configure_nsg "etrid-runtime-dev-secondary" "YOUR_RESOURCE_GROUP"
# configure_nsg "etrid-compiler-dev-primary" "YOUR_RESOURCE_GROUP"
# configure_nsg "etrid-compiler-dev-secondary" "YOUR_RESOURCE_GROUP"
# configure_nsg "etrid-multichain-dev-primary" "YOUR_RESOURCE_GROUP"
# configure_nsg "etrid-multichain-dev-secondary" "YOUR_RESOURCE_GROUP"
# configure_nsg "etrid-oracle-dev" "YOUR_RESOURCE_GROUP"

MANUAL_COMMANDS

echo ""
echo "================================================"
echo "AUTOMATED CONFIGURATION"
echo "================================================"
echo ""
echo "To find your VMs and resource groups:"
az vm list --query '[].{Name:name, ResourceGroup:resourceGroup, Location:location}' -o table

echo ""
echo "================================================"
echo "QUICK FIX - Allow all traffic temporarily"
echo "================================================"
echo ""
echo "If you want to quickly allow access, run:"
echo ""
cat << 'QUICKFIX'
# Get all NSGs
for nsg in $(az network nsg list --query '[].name' -o tsv); do
  rg=$(az network nsg list --query "[?name=='$nsg'].resourceGroup" -o tsv | head -1)

  # Allow SSH from your IP
  az network nsg rule create \
    --resource-group "$rg" \
    --nsg-name "$nsg" \
    --name Allow-All-Admin \
    --priority 100 \
    --source-address-prefixes YOUR_IP_HERE/32 \
    --destination-port-ranges '*' \
    --access Allow \
    --protocol '*' \
    --direction Inbound \
    --output none 2>/dev/null
done
QUICKFIX

echo ""
echo "Replace YOUR_IP_HERE with: $MYIP"
echo ""
