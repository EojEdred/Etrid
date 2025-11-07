#!/bin/bash
# Script to fix NSG rules for the 3 Azure VMs
# Run this in Azure Cloud Shell or after logging into the correct subscription

echo "=== Azure NSG Fix Script ==="
echo "This script will open SSH (port 22) on all 3 Azure VMs"
echo ""

# Get your current public IP
MY_IP=$(curl -s https://api.ipify.org)
echo "Your public IP: $MY_IP"
echo ""

# VM 1: eojedred-validator (20.69.26.209)
echo "=== Fixing NSG for eojedred-validator ==="
NIC1=$(az vm show --name eojedred-validator --resource-group ETRID-MAINNET --query "networkProfile.networkInterfaces[0].id" -o tsv)
NSG1=$(az network nic show --ids $NIC1 --query "networkSecurityGroup.id" -o tsv)
NSG1_NAME=$(basename $NSG1)
NSG1_RG=$(echo $NSG1 | cut -d'/' -f5)

az network nsg rule create \
  --resource-group $NSG1_RG \
  --nsg-name $NSG1_NAME \
  --name Allow-SSH-Inbound \
  --priority 100 \
  --source-address-prefixes $MY_IP/32 \
  --destination-port-ranges 22 \
  --access Allow \
  --protocol Tcp \
  --description "Allow SSH from my IP"

echo "✅ NSG rule added for eojedred-validator"
echo ""

# VM 2: etrid-mainnet_127500e4 (20.186.91.207)
echo "=== Fixing NSG for etrid-mainnet_127500e4 ==="
NIC2=$(az vm show --name etrid-mainnet_127500e4 --resource-group etrid-mainnet_group --query "networkProfile.networkInterfaces[0].id" -o tsv)
NSG2=$(az network nic show --ids $NIC2 --query "networkSecurityGroup.id" -o tsv)
NSG2_NAME=$(basename $NSG2)
NSG2_RG=$(echo $NSG2 | cut -d'/' -f5)

az network nsg rule create \
  --resource-group $NSG2_RG \
  --nsg-name $NSG2_NAME \
  --name Allow-SSH-Inbound \
  --priority 100 \
  --source-address-prefixes $MY_IP/32 \
  --destination-port-ranges 22 \
  --access Allow \
  --protocol Tcp \
  --description "Allow SSH from my IP"

echo "✅ NSG rule added for etrid-mainnet_127500e4"
echo ""

# VM 3: SecurityDev (52.252.142.146)
echo "=== Fixing NSG for SecurityDev ==="
NIC3=$(az vm show --name SecurityDev --resource-group etrid-mainnet_group --query "networkProfile.networkInterfaces[0].id" -o tsv)
NSG3=$(az network nic show --ids $NIC3 --query "networkSecurityGroup.id" -o tsv)
NSG3_NAME=$(basename $NSG3)
NSG3_RG=$(echo $NSG3 | cut -d'/' -f5)

az network nsg rule create \
  --resource-group $NSG3_RG \
  --nsg-name $NSG3_NAME \
  --name Allow-SSH-Inbound \
  --priority 100 \
  --source-address-prefixes $MY_IP/32 \
  --destination-port-ranges 22 \
  --access Allow \
  --protocol Tcp \
  --description "Allow SSH from my IP"

echo "✅ NSG rule added for SecurityDev"
echo ""

echo "=== Testing SSH access ==="
echo "Testing eojedred-validator (20.69.26.209)..."
ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 azureuser@20.69.26.209 "hostname" && echo "✅ SSH works!" || echo "❌ Still blocked"

echo "Testing etrid-mainnet_127500e4 (20.186.91.207)..."
ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 azureuser@20.186.91.207 "hostname" && echo "✅ SSH works!" || echo "❌ Still blocked"

echo "Testing SecurityDev (52.252.142.146)..."
ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 azureuser@52.252.142.146 "hostname" && echo "✅ SSH works!" || echo "❌ Still blocked"

echo ""
echo "=== Done! ==="
