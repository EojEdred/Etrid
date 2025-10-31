#!/usr/bin/env bash
# Configure Azure NSG rules for validators #2-4

set -e

MYIP="206.188.236.130"
MONITORING_IP="98.71.91.84"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔧 Configuring Azure NSG Rules for Validators #2-4"
echo "=================================================="
echo ""

# Function to configure NSG for a VM
configure_vm_nsg() {
    local vm_name=$1
    local rg=$2
    local validator_num=$3

    echo -e "${YELLOW}Configuring #$validator_num ($vm_name in $rg)...${NC}"

    # Get NSG name
    local nsg_name=$(az network nic show \
        --resource-group "$rg" \
        --name "${vm_name}VMNic" \
        --query "networkSecurityGroup.id" \
        --output tsv 2>/dev/null | awk -F/ '{print $NF}')

    if [ -z "$nsg_name" ]; then
        nsg_name="${vm_name}NSG"
    fi

    echo "  NSG: $nsg_name"

    # SSH from your IP
    az network nsg rule create \
        --resource-group "$rg" \
        --nsg-name "$nsg_name" \
        --name "Allow-SSH-Admin" \
        --priority 100 \
        --source-address-prefixes "$MYIP/32" \
        --destination-port-ranges 22 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --output none 2>/dev/null || true

    # Prometheus from monitoring server
    az network nsg rule create \
        --resource-group "$rg" \
        --nsg-name "$nsg_name" \
        --name "Allow-Prometheus-Scrape" \
        --priority 110 \
        --source-address-prefixes "$MONITORING_IP/32" \
        --destination-port-ranges 9615 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --output none 2>/dev/null || true

    # Node exporter from monitoring server
    az network nsg rule create \
        --resource-group "$rg" \
        --nsg-name "$nsg_name" \
        --name "Allow-Node-Exporter" \
        --priority 111 \
        --source-address-prefixes "$MONITORING_IP/32" \
        --destination-port-ranges 9100 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --output none 2>/dev/null || true

    # P2P networking (from anywhere for blockchain)
    az network nsg rule create \
        --resource-group "$rg" \
        --nsg-name "$nsg_name" \
        --name "Allow-P2P-Network" \
        --priority 120 \
        --source-address-prefixes "*" \
        --destination-port-ranges 30333 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --output none 2>/dev/null || true

    echo -e "${GREEN}  ✅ NSG configured${NC}"
}

echo "Configuring Azure validators..."
echo ""

# Need to find the resource groups for these VMs
# Let's search for them
echo "🔍 Finding VMs in Azure..."
echo ""

# Validator #2 - EojEdred Founder (20.69.26.209)
echo "Finding validator #2 (EojEdred)..."
V2_INFO=$(az vm list --query "[?contains(name, 'eoj') || contains(name, 'edred')].{name:name, rg:resourceGroup}" --output tsv 2>/dev/null | head -1)
if [ -n "$V2_INFO" ]; then
    V2_NAME=$(echo "$V2_INFO" | awk '{print $1}')
    V2_RG=$(echo "$V2_INFO" | awk '{print $2}')
    echo "  Found: $V2_NAME in $V2_RG"
    configure_vm_nsg "$V2_NAME" "$V2_RG" 2
else
    echo "  ⚠️  Not found, trying by IP..."
    V2_RG=$(az vm list --query "[?publicIps=='20.69.26.209'].resourceGroup" --output tsv 2>/dev/null)
    if [ -n "$V2_RG" ]; then
        V2_NAME=$(az vm list --query "[?publicIps=='20.69.26.209'].name" --output tsv 2>/dev/null)
        configure_vm_nsg "$V2_NAME" "$V2_RG" 2
    else
        echo "  ❌ Could not find validator #2"
    fi
fi

echo ""

# Validator #3 - Governance Dev (20.186.91.207)
echo "Finding validator #3 (Governance)..."
V3_INFO=$(az vm list --query "[?contains(name, 'governance') || contains(name, 'Governance')].{name:name, rg:resourceGroup}" --output tsv 2>/dev/null | head -1)
if [ -n "$V3_INFO" ]; then
    V3_NAME=$(echo "$V3_INFO" | awk '{print $1}')
    V3_RG=$(echo "$V3_INFO" | awk '{print $2}')
    echo "  Found: $V3_NAME in $V3_RG"
    configure_vm_nsg "$V3_NAME" "$V3_RG" 3
else
    echo "  ⚠️  Not found, searching all VMs..."
    # Try to find by public IP
    V3_INFO=$(az network public-ip list --query "[?ipAddress=='20.186.91.207'].[name, resourceGroup]" --output tsv 2>/dev/null)
    if [ -n "$V3_INFO" ]; then
        V3_RG=$(echo "$V3_INFO" | awk '{print $2}')
        # Get VM from NIC that uses this public IP
        configure_vm_nsg "governance-dev01" "$V3_RG" 3 2>/dev/null || echo "  ❌ Could not configure"
    else
        echo "  ❌ Could not find validator #3"
    fi
fi

echo ""

# Validator #4 - Security Dev (52.252.142.146)
echo "Finding validator #4 (Security)..."
V4_INFO=$(az vm list --query "[?contains(name, 'security') || contains(name, 'Security')].{name:name, rg:resourceGroup}" --output tsv 2>/dev/null | head -1)
if [ -n "$V4_INFO" ]; then
    V4_NAME=$(echo "$V4_INFO" | awk '{print $1}')
    V4_RG=$(echo "$V4_INFO" | awk '{print $2}')
    echo "  Found: $V4_NAME in $V4_RG"
    configure_vm_nsg "$V4_NAME" "$V4_RG" 4
else
    echo "  ⚠️  Not found, searching by IP..."
    V4_INFO=$(az network public-ip list --query "[?ipAddress=='52.252.142.146'].[name, resourceGroup]" --output tsv 2>/dev/null)
    if [ -n "$V4_INFO" ]; then
        V4_RG=$(echo "$V4_INFO" | awk '{print $2}')
        configure_vm_nsg "security-dev01" "$V4_RG" 4 2>/dev/null || echo "  ❌ Could not configure"
    else
        echo "  ❌ Could not find validator #4"
    fi
fi

echo ""
echo "=================================================="
echo "✅ Azure NSG Configuration Complete"
echo "=================================================="
echo ""
echo "Note: Oracle Cloud validators (#1, #5) need manual configuration:"
echo "  1. Login to Oracle Cloud Console"
echo "  2. Go to: Networking → Virtual Cloud Networks → Security Lists"
echo "  3. Add ingress rules:"
echo "     - Port 22 (SSH) from $MYIP/32"
echo "     - Port 9100 (Node Exporter) from $MONITORING_IP/32"
echo "     - Port 9615 (Substrate metrics) from $MONITORING_IP/32"
echo "     - Port 30333 (P2P) from 0.0.0.0/0"
echo ""
