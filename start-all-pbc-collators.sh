#!/usr/bin/env bash

# PBC Collator Startup Script
# Starts all PBC collators across validator VMs with proper configuration

set -e

# Ensure bash 4+ for associative arrays
if [ "${BASH_VERSINFO[0]}" -lt 4 ]; then
    echo "This script requires bash 4 or higher"
    exit 1
fi

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}PBC Collator Network Startup${NC}"
echo -e "${GREEN}========================================${NC}"

# VM list
VMS=(
    "vmi2896906"
    "vmi2896907"
    "vmi2896908"
    "vmi2896909"
    "vmi2896910"
    "vmi2896911"
    "vmi2896914"
    "vmi2896915"
    "vmi2896916"
    "vmi2896917"
    "vmi2896918"
    "vmi2896921"
    "vmi2896922"
    "vmi2896923"
    "vmi2896924"
    "vmi2896925"
)

# PBC chains with port assignments
declare -A PBC_PORTS=(
    ["btc-pbc"]=9944
    ["eth-pbc"]=9945
    ["sol-pbc"]=9946
    ["xrp-pbc"]=9947
    ["bnb-pbc"]=9948
    ["trx-pbc"]=9949
    ["matic-pbc"]=9950
    ["xlm-pbc"]=9951
    ["ada-pbc"]=9952
    ["doge-pbc"]=9953
    ["link-pbc"]=9954
    ["edsc-pbc"]=9955
    ["sc-usdt-pbc"]=9956
)

# PBC chain distribution across VMs (round-robin for load balancing)
declare -A VM_PBCS=(
    ["vmi2896906"]="btc-pbc eth-pbc"
    ["vmi2896907"]="sol-pbc xrp-pbc"  # Boot node VM
    ["vmi2896908"]="bnb-pbc trx-pbc"
    ["vmi2896909"]="matic-pbc xlm-pbc"
    ["vmi2896910"]="ada-pbc doge-pbc"
    ["vmi2896911"]="link-pbc edsc-pbc"
    ["vmi2896914"]="sc-usdt-pbc btc-pbc"  # Secondary boot node
    ["vmi2896915"]="eth-pbc sol-pbc"
    ["vmi2896916"]="xrp-pbc bnb-pbc"
    ["vmi2896917"]="trx-pbc matic-pbc"
    ["vmi2896918"]="xlm-pbc ada-pbc"
    ["vmi2896921"]="doge-pbc link-pbc"
    ["vmi2896922"]="edsc-pbc sc-usdt-pbc"
    ["vmi2896923"]="btc-pbc eth-pbc"
    ["vmi2896924"]="sol-pbc xrp-pbc"
    ["vmi2896925"]="bnb-pbc trx-pbc"
)

# SSH key
SSH_KEY="$HOME/.ssh/id_rsa"

# Function to execute command on VM
ssh_exec() {
    local vm=$1
    local cmd=$2
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 root@${vm}.contaboserver.net "$cmd"
}

# Function to create directories on VM
setup_directories() {
    local vm=$1
    echo -e "${YELLOW}Setting up directories on $vm...${NC}"
    ssh_exec "$vm" "mkdir -p /root/pbc-data /root/pbc-logs /root/pbc-collators"
}

# Function to check if binary exists
check_binary() {
    local vm=$1
    local chain=$2
    ssh_exec "$vm" "[ -f /root/pbc-collators/${chain}-collator ] && echo 'exists' || echo 'missing'"
}

# Function to start collator
start_collator() {
    local vm=$1
    local chain=$2
    local port=${PBC_PORTS[$chain]}
    local vm_index=$3

    echo -e "${YELLOW}Starting $chain on $vm (port $port)...${NC}"

    # Check if binary exists
    local binary_check=$(check_binary "$vm" "$chain")
    if [ "$binary_check" != "exists" ]; then
        echo -e "${RED}Binary ${chain}-collator not found on $vm${NC}"
        return 1
    fi

    # Stop existing process if running
    ssh_exec "$vm" "pkill -f '${chain}-collator' || true"
    sleep 2

    # Start collator
    local start_cmd="cd /root/pbc-collators && nohup ./${chain}-collator \
        --chain production \
        --base-path /root/pbc-data/${chain} \
        --rpc-port $port \
        --rpc-cors all \
        --rpc-methods unsafe \
        --name '${chain}-validator-${vm_index}' \
        --validator \
        > /root/pbc-logs/${chain}.log 2>&1 &"

    ssh_exec "$vm" "$start_cmd"

    # Wait a moment for startup
    sleep 3

    # Check if process started
    local pid=$(ssh_exec "$vm" "pgrep -f '${chain}-collator' || echo 'not_running'")
    if [ "$pid" != "not_running" ]; then
        echo -e "${GREEN}✓ $chain started on $vm (PID: $pid)${NC}"
        return 0
    else
        echo -e "${RED}✗ Failed to start $chain on $vm${NC}"
        return 1
    fi
}

# Function to get peer ID from logs
get_peer_id() {
    local vm=$1
    local chain=$2

    echo -e "${YELLOW}Retrieving peer ID for $chain on $vm...${NC}"

    # Wait for node to initialize
    sleep 5

    # Extract peer ID from logs
    local peer_id=$(ssh_exec "$vm" "grep -oP 'Local node identity is: \K[A-Za-z0-9]+' /root/pbc-logs/${chain}.log | head -1 || echo 'not_found'")

    if [ "$peer_id" != "not_found" ] && [ ! -z "$peer_id" ]; then
        echo -e "${GREEN}Peer ID for $chain on $vm: $peer_id${NC}"
        echo "$peer_id"
    else
        echo -e "${RED}Could not retrieve peer ID for $chain on $vm${NC}"
        echo "not_found"
    fi
}

# Main execution
echo -e "\n${GREEN}Step 1: Setting up directories on all VMs${NC}"
for vm in "${VMS[@]}"; do
    setup_directories "$vm" &
done
wait

echo -e "\n${GREEN}Step 2: Starting PBC collators${NC}"

# Track started collators
declare -A STARTED_COLLATORS

vm_index=0
for vm in "${VMS[@]}"; do
    vm_index=$((vm_index + 1))
    pbcs=${VM_PBCS[$vm]}

    if [ -z "$pbcs" ]; then
        echo -e "${YELLOW}No PBCs assigned to $vm${NC}"
        continue
    fi

    echo -e "\n${GREEN}=== Processing $vm ===${NC}"

    for chain in $pbcs; do
        if start_collator "$vm" "$chain" "$vm_index"; then
            STARTED_COLLATORS["${chain}_${vm}"]="$vm"
        fi
    done
done

echo -e "\n${GREEN}Step 3: Collecting boot node information${NC}"

# Boot node VMs
BOOT_NODE_VMS=("vmi2896907" "vmi2896914")

echo -e "\n${YELLOW}Boot Node Configuration:${NC}"
for boot_vm in "${BOOT_NODE_VMS[@]}"; do
    echo -e "\n${GREEN}Boot Node: $boot_vm${NC}"
    pbcs=${VM_PBCS[$boot_vm]}

    for chain in $pbcs; do
        peer_id=$(get_peer_id "$boot_vm" "$chain")
        if [ "$peer_id" != "not_found" ]; then
            # Get VM IP
            vm_ip=$(host ${boot_vm}.contaboserver.net | grep "has address" | awk '{print $4}' || echo "unknown")
            port=${PBC_PORTS[$chain]}
            echo -e "${GREEN}$chain boot node: /ip4/$vm_ip/tcp/$port/p2p/$peer_id${NC}"
        fi
    done
done

echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}Deployment Summary${NC}"
echo -e "${GREEN}========================================${NC}"

# Generate summary
echo -e "\n${YELLOW}Started Collators:${NC}"
for key in "${!STARTED_COLLATORS[@]}"; do
    echo "  - $key"
done | sort

echo -e "\n${GREEN}Step 4: Health check${NC}"
for vm in "${VMS[@]}"; do
    echo -e "\n${YELLOW}Checking processes on $vm:${NC}"
    ssh_exec "$vm" "ps aux | grep -E '(btc|eth|sol|xrp|bnb|trx|matic|xlm|ada|doge|link|edsc|sc-usdt)-pbc-collator' | grep -v grep || echo 'No collators running'"
done

echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}PBC Network Startup Complete!${NC}"
echo -e "${GREEN}========================================${NC}"

# Save deployment report
REPORT_FILE="/Users/macbook/Desktop/pbc-startup-report-$(date +%Y%m%d-%H%M%S).txt"
echo "PBC Collator Startup Report - $(date)" > "$REPORT_FILE"
echo "========================================" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "Started Collators:" >> "$REPORT_FILE"
for key in "${!STARTED_COLLATORS[@]}"; do
    echo "  - $key" >> "$REPORT_FILE"
done | sort >> "$REPORT_FILE"

echo -e "\n${GREEN}Report saved to: $REPORT_FILE${NC}"
