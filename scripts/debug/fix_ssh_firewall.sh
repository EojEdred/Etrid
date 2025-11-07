#!/bin/bash

# Fix Firewall Rules for SSH on Azure VMs
# This script opens port 22 in the OS firewall (iptables/ufw/firewalld)

set -e

echo "========================================="
echo "SSH Firewall Fix Script"
echo "========================================="
echo ""

# List of VMs to fix (update based on diagnostic results)
declare -a VMS=(
  "etrid-compiler-dev-secondary:ETRID-VALIDATORS-EU-NORTH"
  "etrid-multichain-dev-primary:ETRID-VALIDATORS-EU-NORTH"
  "etrid-compiler-dev-primary:ETRID-VALIDATORS-EU-WEST"
  "etrid-consensus-dev-secondary:ETRID-VALIDATORS-EU-WEST"
  "etrid-multichain-dev-secondary:ETRID-VALIDATORS-EU-WEST"
  "etrid-runtime-dev-primary:ETRID-VALIDATORS-EU-WEST"
  "etrid-runtime-dev-secondary:ETRID-VALIDATORS-EU-WEST"
  "etrid-flarenode-15:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-flarenode-16:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-flarenode-17:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-oracle-dev:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-flarenode-18:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-19:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-20:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-21:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-22:ETRID-VALIDATORS-EU-FR"
)

# Function to fix firewall
fix_firewall() {
  local VM_NAME=$1
  local RG=$2

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Fixing firewall on: $VM_NAME ($RG)"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  RESULT=$(az vm run-command invoke \
    -g "$RG" \
    -n "$VM_NAME" \
    --command-id RunShellScript \
    --scripts "
      echo '=== Configuring Firewall Rules for SSH ==='

      # Check which firewall is in use
      FIREWALL_TYPE='none'

      if command -v ufw >/dev/null 2>&1; then
        FIREWALL_TYPE='ufw'
      elif command -v firewall-cmd >/dev/null 2>&1; then
        FIREWALL_TYPE='firewalld'
      elif command -v iptables >/dev/null 2>&1; then
        FIREWALL_TYPE='iptables'
      fi

      echo \"Detected firewall: \$FIREWALL_TYPE\"
      echo ''

      # Configure UFW (Ubuntu/Debian)
      if [ \"\$FIREWALL_TYPE\" = 'ufw' ]; then
        echo '=== Configuring UFW ==='
        ufw allow 22/tcp
        ufw status
      fi

      # Configure firewalld (CentOS/RHEL/Fedora)
      if [ \"\$FIREWALL_TYPE\" = 'firewalld' ]; then
        echo '=== Configuring firewalld ==='
        firewall-cmd --permanent --add-service=ssh
        firewall-cmd --permanent --add-port=22/tcp
        firewall-cmd --reload
        firewall-cmd --list-all
      fi

      # Configure iptables
      if [ \"\$FIREWALL_TYPE\" = 'iptables' ]; then
        echo '=== Configuring iptables ==='
        # Check if rule already exists
        if ! iptables -C INPUT -p tcp --dport 22 -j ACCEPT 2>/dev/null; then
          iptables -I INPUT -p tcp --dport 22 -j ACCEPT
          echo 'Added iptables rule for SSH'
        else
          echo 'SSH rule already exists in iptables'
        fi

        # Try to save rules
        if command -v iptables-save >/dev/null 2>&1; then
          iptables-save > /etc/iptables/rules.v4 2>/dev/null || iptables-save > /etc/sysconfig/iptables 2>/dev/null || echo 'Could not save iptables rules'
        fi

        echo ''
        echo 'Current iptables rules:'
        iptables -L INPUT -n --line-numbers | grep -E 'Chain|22|ACCEPT|DROP'
      fi

      echo ''
      echo '=== Verifying Port 22 ==='
      netstat -tuln | grep ':22' || ss -tuln | grep ':22'
    " \
    --query "value[0].message" \
    -o tsv 2>&1)

  echo "$RESULT"
  echo "✅ Firewall configuration completed"
  echo ""
  echo ""
}

# Confirmation prompt
echo "This script will configure OS-level firewall rules to allow SSH on:"
echo ""
for vm_info in "${VMS[@]}"; do
  IFS=':' read -r VM_NAME RG <<< "$vm_info"
  echo "  - $VM_NAME ($RG)"
done
echo ""
read -p "Continue? (y/n): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "Aborted."
  exit 0
fi

echo ""
echo "Starting firewall fix process..."
echo ""

for vm_info in "${VMS[@]}"; do
  IFS=':' read -r VM_NAME RG <<< "$vm_info"
  fix_firewall "$VM_NAME" "$RG"

  # Small delay to avoid throttling
  sleep 2
done

echo "========================================="
echo "Firewall Fix Complete"
echo "========================================="
echo ""
echo "Test SSH connections with:"
echo "ssh azureuser@<PUBLIC_IP>"
