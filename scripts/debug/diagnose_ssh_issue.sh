#!/bin/bash

# Azure VM SSH Diagnostic Script
# This script checks SSH daemon status and connectivity issues on Azure VMs

set -e

echo "========================================="
echo "Azure VM SSH Connectivity Diagnostic Tool"
echo "========================================="
echo ""

# List of non-working VMs
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

# Function to check SSH daemon on a VM
check_ssh_daemon() {
  local VM_NAME=$1
  local RG=$2

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Checking: $VM_NAME ($RG)"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  # Get public IP
  PUBLIC_IP=$(az vm show -g "$RG" -n "$VM_NAME" -d --query "publicIps" -o tsv 2>/dev/null)
  echo "Public IP: $PUBLIC_IP"

  # Check SSH daemon status
  echo "Checking SSH daemon status..."
  RESULT=$(az vm run-command invoke \
    -g "$RG" \
    -n "$VM_NAME" \
    --command-id RunShellScript \
    --scripts "
      echo '=== SSH Daemon Status ==='
      systemctl status sshd 2>&1 || systemctl status ssh 2>&1 || service ssh status 2>&1

      echo ''
      echo '=== SSH Port Listening ==='
      netstat -tuln | grep ':22' || ss -tuln | grep ':22' || echo 'Port 22 not listening'

      echo ''
      echo '=== iptables Rules ==='
      iptables -L INPUT -n | grep -E 'dpt:22|ACCEPT|DROP|REJECT' || echo 'No iptables rules'

      echo ''
      echo '=== UFW Status ==='
      ufw status 2>&1 || echo 'UFW not installed'

      echo ''
      echo '=== Firewalld Status ==='
      firewall-cmd --list-all 2>&1 || echo 'Firewalld not installed'
    " \
    --query "value[0].message" \
    -o tsv 2>&1)

  echo "$RESULT"
  echo ""
  echo ""
}

# Main execution
echo "Starting diagnostics for 16 VMs..."
echo "This may take several minutes..."
echo ""

for vm_info in "${VMS[@]}"; do
  IFS=':' read -r VM_NAME RG <<< "$vm_info"
  check_ssh_daemon "$VM_NAME" "$RG"

  # Small delay to avoid throttling
  sleep 2
done

echo "========================================="
echo "Diagnostic Complete"
echo "========================================="
echo ""
echo "Review the output above to identify:"
echo "1. VMs where SSH daemon is not running"
echo "2. VMs where port 22 is not listening"
echo "3. VMs with firewall rules blocking SSH"
echo ""
echo "Next steps:"
echo "1. For VMs with SSH not running: Run fix_ssh_daemon.sh"
echo "2. For VMs with firewall issues: Run fix_ssh_firewall.sh"
