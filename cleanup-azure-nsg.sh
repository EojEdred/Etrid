#!/bin/bash
# Azure NSG SSH Rules Cleanup Script
# Execute this in Azure Cloud Shell after logging into eojedredbitepubkey1@proton.me

set -e

NSG_NAME="eojedred-validator-nsg"
RG_NAME="ETRID-MAINNET"
CURRENT_IP="206.188.236.130"
PERMANENT_IP="73.185.170.6"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Azure NSG SSH Rules Cleanup                                 ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Show current SSH rules
echo "Current SSH rules:"
az network nsg rule list \
  --resource-group $RG_NAME \
  --nsg-name $NSG_NAME \
  --query "[?destinationPortRange=='22'].{Priority:priority, Name:name, Source:sourceAddressPrefix, Access:access}" \
  --output table

echo ""
echo "Press Enter to continue with cleanup, or Ctrl+C to cancel..."
read

# Option 1: Delete temporary rules
echo ""
echo "Deleting temporary rules..."

echo "  Deleting: Temp-SSH-Claude (Priority 102)..."
az network nsg rule delete \
  --resource-group $RG_NAME \
  --nsg-name $NSG_NAME \
  --name "Temp-SSH-Claude" 2>/dev/null || echo "    (already deleted)"

echo "  Deleting: Allow-SSH-From-New-IP (Priority 310)..."
az network nsg rule delete \
  --resource-group $RG_NAME \
  --nsg-name $NSG_NAME \
  --name "Allow-SSH-From-New-IP" 2>/dev/null || echo "    (already deleted)"

# Ask about 172.56.11.73 IP
echo ""
echo "Current SSH-Access rule has IP: 172.56.11.73"
echo "Do you want to keep this IP? (y/n)"
read -r KEEP_OLD_IP

# Option 2: Create consolidated rule
echo ""
echo "Creating consolidated SSH rule..."

# Delete all old SSH rules
echo "  Deleting old rules..."
az network nsg rule delete --resource-group $RG_NAME --nsg-name $NSG_NAME --name "SSH-Access" 2>/dev/null || true
az network nsg rule delete --resource-group $RG_NAME --nsg-name $NSG_NAME --name "SSH-IP-73-185-170-6" 2>/dev/null || true

# Build source IP list
if [[ "$KEEP_OLD_IP" == "y" ]]; then
    SOURCE_IPS="$PERMANENT_IP $CURRENT_IP 172.56.11.73"
    echo "  Including IPs: $PERMANENT_IP, $CURRENT_IP, 172.56.11.73"
else
    SOURCE_IPS="$PERMANENT_IP $CURRENT_IP"
    echo "  Including IPs: $PERMANENT_IP, $CURRENT_IP"
fi

# Create consolidated rule
echo "  Creating: SSH-Consolidated (Priority 100)..."
az network nsg rule create \
  --resource-group $RG_NAME \
  --nsg-name $NSG_NAME \
  --name "SSH-Consolidated" \
  --priority 100 \
  --source-address-prefixes $SOURCE_IPS \
  --source-port-ranges '*' \
  --destination-address-prefixes '*' \
  --destination-port-ranges 22 \
  --access Allow \
  --protocol Tcp \
  --description "Consolidated SSH access for admin IPs"

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Cleanup Complete!                                           ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Show final SSH rules
echo "Final SSH rules:"
az network nsg rule list \
  --resource-group $RG_NAME \
  --nsg-name $NSG_NAME \
  --query "[?destinationPortRange=='22'].{Priority:priority, Name:name, Source:sourceAddressPrefix, Access:access}" \
  --output table

echo ""
echo "Testing SSH access..."
echo ""

echo "Testing V0B-EojEdred (20.69.26.209)..."
ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 -o StrictHostKeyChecking=no azureuser@20.69.26.209 "echo 'SSH OK'" 2>&1 | head -2

echo ""
echo "Testing SecurityDev (52.252.142.146)..."
ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 -o StrictHostKeyChecking=no azureuser@52.252.142.146 "echo 'SSH OK'" 2>&1 | head -2

echo ""
echo "Done! SSH rules consolidated."
