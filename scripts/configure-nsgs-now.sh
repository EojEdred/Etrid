#!/usr/bin/env bash
# Auto-configure NSG rules for all Ëtrid validators
set -e

MYIP="206.188.236.130"
MONITORING_IP="98.71.91.84"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔥 Configuring NSG Rules for All Validators"
echo "============================================="
echo ""
echo "Your IP: $MYIP"
echo "Monitoring Server: $MONITORING_IP"
echo ""

# Function to configure NSG for a VM
configure_vm_nsg() {
    local vm_name=$1
    local rg=$2

    echo -e "${YELLOW}Configuring $vm_name...${NC}"

    # Get NSG attached to VM's NIC
    local nsg_name=$(az vm show --name "$vm_name" --resource-group "$rg" \
        --query 'networkProfile.networkInterfaces[0].id' -o tsv | \
        xargs az network nic show --ids | \
        jq -r '.networkSecurityGroup.id' | \
        awk -F'/' '{print $NF}')

    if [ -z "$nsg_name" ] || [ "$nsg_name" == "null" ]; then
        echo "  ⚠️  No NSG found, skipping"
        return 0
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
        --name "Allow-Prometheus" \
        --priority 110 \
        --source-address-prefixes "$MONITORING_IP/32" \
        --destination-port-ranges 9615 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --output none 2>/dev/null || true

    # Node Exporter from monitoring server
    az network nsg rule create \
        --resource-group "$rg" \
        --nsg-name "$nsg_name" \
        --name "Allow-NodeExporter" \
        --priority 120 \
        --source-address-prefixes "$MONITORING_IP/32" \
        --destination-port-ranges 9100 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --output none 2>/dev/null || true

    # P2P networking
    az network nsg rule create \
        --resource-group "$rg" \
        --nsg-name "$nsg_name" \
        --name "Allow-P2P" \
        --priority 130 \
        --source-address-prefixes "*" \
        --destination-port-ranges 30333 \
        --access Allow \
        --protocol Tcp \
        --direction Inbound \
        --output none 2>/dev/null || true

    echo -e "${GREEN}  ✅ Done${NC}"
}

# Configure all VMs
configure_vm_nsg "etrid-compiler-dev-secondary" "ETRID-VALIDATORS-EU-NORTH"
configure_vm_nsg "etrid-multichain-dev-primary" "ETRID-VALIDATORS-EU-NORTH"
configure_vm_nsg "etrid-compiler-dev-primary" "ETRID-VALIDATORS-EU-WEST"
configure_vm_nsg "etrid-consensus-dev-secondary" "ETRID-VALIDATORS-EU-WEST"
configure_vm_nsg "etrid-multichain-dev-secondary" "ETRID-VALIDATORS-EU-WEST"
configure_vm_nsg "etrid-runtime-dev-primary" "ETRID-VALIDATORS-EU-WEST"
configure_vm_nsg "etrid-runtime-dev-secondary" "ETRID-VALIDATORS-EU-WEST"
configure_vm_nsg "etrid-audit-dev-secondary" "ETRID-VALIDATORS-UK-SOUTH"
configure_vm_nsg "etrid-flarenode-15" "ETRID-VALIDATORS-UK-SOUTH"
configure_vm_nsg "etrid-flarenode-16" "ETRID-VALIDATORS-UK-SOUTH"
configure_vm_nsg "etrid-flarenode-17" "ETRID-VALIDATORS-UK-SOUTH"
configure_vm_nsg "etrid-oracle-dev" "ETRID-VALIDATORS-UK-SOUTH"
configure_vm_nsg "etrid-flarenode-18" "ETRID-VALIDATORS-EU-FR"
configure_vm_nsg "etrid-flarenode-19" "ETRID-VALIDATORS-EU-FR"
configure_vm_nsg "etrid-flarenode-20" "ETRID-VALIDATORS-EU-FR"
configure_vm_nsg "etrid-flarenode-21" "ETRID-VALIDATORS-EU-FR"

echo ""
echo "============================================="
echo -e "${GREEN}🎉 All NSG rules configured!${NC}"
echo "============================================="
echo ""
echo "Rules added:"
echo "  ✅ SSH (22) from $MYIP"
echo "  ✅ Prometheus (9615) from $MONITORING_IP"
echo "  ✅ Node Exporter (9100) from $MONITORING_IP"
echo "  ✅ P2P (30333) from anywhere"
echo ""
