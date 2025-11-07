#!/bin/bash
# Ëtrid FlareChain - Start All Azure Validators
# Starts all stopped/deallocated Azure VMs for mainnet validators
# Date: November 7, 2025

set -e

OUTPUT_FILE="vm_startup_$(date +%Y%m%d_%H%M%S).log"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Starting All Azure Validators           ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "AZURE VALIDATOR STARTUP PROCEDURE"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Start Time: $(date)"
    echo ""

    # Get all stopped/deallocated VMs
    echo "Checking VM states..."
    echo ""

    vms=$(az vm list -d --query "[?powerState!='VM running'].{Name:name, RG:resourceGroup, State:powerState}" --output tsv)

    if [ -z "$vms" ]; then
        echo "✓ All VMs are already running!"
        exit 0
    fi

    vm_count=$(echo "$vms" | wc -l)
    echo "Found $vm_count VMs that need to be started"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "STARTING VMs"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Start VMs in parallel by resource group for faster startup
    declare -A rg_vms

    while IFS=$'\t' read -r name rg state; do
        if [ -z "$name" ]; then continue; fi
        echo "Queuing: $name ($rg) - Current state: $state"
        rg_vms["$rg"]+="$name "
    done <<< "$vms"

    echo ""
    echo "Starting VMs grouped by resource group (parallel)..."
    echo ""

    # Start VMs by resource group
    for rg in "${!rg_vms[@]}"; do
        vm_list="${rg_vms[$rg]}"
        echo "Starting VMs in $rg:"

        for vm in $vm_list; do
            echo "  → $vm"
            az vm start --resource-group "$rg" --name "$vm" --no-wait &
        done
        echo ""
    done

    echo "All startup commands issued. Waiting for VMs to start..."
    echo "(This typically takes 2-5 minutes)"
    echo ""

    # Wait for all VMs to start
    sleep 30  # Give Azure time to process requests

    echo "Checking VM states..."
    attempts=0
    max_attempts=20

    while [ $attempts -lt $max_attempts ]; do
        running_count=$(az vm list -d --query "[?powerState=='VM running'] | length(@)" --output tsv)
        total_count=$(az vm list --query "length([])" --output tsv)

        echo "Attempt $((attempts + 1))/$max_attempts: $running_count/$total_count VMs running"

        if [ "$running_count" == "$total_count" ]; then
            echo ""
            echo "✓ All VMs are now running!"
            break
        fi

        sleep 15
        ((attempts++))
    done

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "FINAL VM STATUS"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    az vm list -d --query "[].{Name:name, State:powerState, IP:publicIps, Location:location}" --output table

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "VALIDATOR SERVICE STATUS"
    echo "─────────────────────────────────────────────────────────────"
    echo ""
    echo "Waiting 30 seconds for VMs to fully boot and services to start..."
    sleep 30

    echo ""
    echo "Testing FlareChain validator services..."
    echo ""

    # Test a few key validators
    test_ips=("98.71.91.84" "20.224.104.239" "108.142.205.177" "4.180.238.67")
    responding=0

    for ip in "${test_ips[@]}"; do
        echo "Testing $ip:9944..."
        if curl -s -m 5 -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null | grep -q "result"; then
            echo "  ✓ Validator responding"
            ((responding++))
        else
            echo "  ✗ Validator not responding yet"
        fi
    done

    echo ""
    if [ $responding -gt 0 ]; then
        echo "✓ Some validators are responding!"
        echo ""
        echo "Run the health check to see full status:"
        echo "  bash check-validators-simple.sh"
    else
        echo "⚠ Validators not responding yet. This may be normal if:"
        echo "  1. Validators are still booting (wait 2-3 minutes)"
        echo "  2. Systemd services are not enabled (need manual start)"
        echo "  3. Firewall rules need to be checked"
        echo ""
        echo "To check validator service status on a VM:"
        echo "  SSH into VM and run: sudo systemctl status flarechain-validator"
        echo ""
        echo "To manually start validator service:"
        echo "  sudo systemctl start flarechain-validator"
        echo "  sudo systemctl enable flarechain-validator"
    fi

    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "Startup process complete: $(date)"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

} | tee "$OUTPUT_FILE"

echo ""
echo "Startup log saved to: $OUTPUT_FILE"
echo ""
