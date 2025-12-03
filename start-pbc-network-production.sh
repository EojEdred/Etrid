#!/usr/bin/env bash

# Production PBC Network Startup Script
# Starts all PBC collators with proper production configuration

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}ETRID PBC Network - Production Startup${NC}"
echo -e "${GREEN}========================================${NC}"

# All validator VMs
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

# Available PBC chains (no eth-pbc as binary doesn't exist)
PBC_CHAINS=(
    "ada-pbc"
    "btc-pbc"
    "bnb-pbc"
    "doge-pbc"
    "edsc-pbc"
    "link-pbc"
    "matic-pbc"
    "sc-usdt-pbc"
    "sol-pbc"
    "trx-pbc"
    "xlm-pbc"
    "xrp-pbc"
)

# Port mapping for each chain
declare -A CHAIN_PORTS=(
    ["ada-pbc"]=9945
    ["btc-pbc"]=9947
    ["bnb-pbc"]=9960
    ["doge-pbc"]=9948
    ["edsc-pbc"]=9949
    ["link-pbc"]=9951
    ["matic-pbc"]=9952
    ["sc-usdt-pbc"]=9953
    ["sol-pbc"]=9954
    ["trx-pbc"]=9955
    ["xlm-pbc"]=9956
    ["xrp-pbc"]=9957
)

# P2P port mapping
declare -A CHAIN_P2P_PORTS=(
    ["ada-pbc"]=30334
    ["btc-pbc"]=30336
    ["bnb-pbc"]=30350
    ["doge-pbc"]=30337
    ["edsc-pbc"]=30338
    ["link-pbc"]=30340
    ["matic-pbc"]=30341
    ["sc-usdt-pbc"]=30342
    ["sol-pbc"]=30343
    ["trx-pbc"]=30344
    ["xlm-pbc"]=30345
    ["xrp-pbc"]=30346
)

# PBC IDs
declare -A CHAIN_IDS=(
    ["ada-pbc"]=0
    ["btc-pbc"]=2
    ["bnb-pbc"]=1
    ["doge-pbc"]=3
    ["edsc-pbc"]=4
    ["link-pbc"]=6
    ["matic-pbc"]=7
    ["sc-usdt-pbc"]=8
    ["sol-pbc"]=9
    ["trx-pbc"]=10
    ["xlm-pbc"]=11
    ["xrp-pbc"]=12
)

# Boot nodes (using existing production boot nodes)
declare -A BOOT_NODES=(
    ["ada-pbc"]="/ip4/100.102.128.51/tcp/30334/p2p/12D3KooWAFR6HZiKRcbikY18Xa6TzFwyykKNXSExWecPb62CgexF /ip4/100.89.102.75/tcp/30334/p2p/12D3KooWMLFC7uwhZASTbsSQe2SVUmSXXEVssFXpjzsD1PWoehe2 /ip4/100.80.84.82/tcp/30334/p2p/12D3KooWSeSzPszkevHEqs4CAhW3nmf7At2vo1oWyZL49sHkP7Fr"
    ["btc-pbc"]="/ip4/100.102.128.51/tcp/30336/p2p/12D3KooWDXvELsRboeRacUdu6vQe1i3gug5MKd7Z5gj3cLJ4WJJo /ip4/100.89.102.75/tcp/30336/p2p/12D3KooWDNbKfweJQtXZTEsJk3maGo5SAhYeb9TvnCF55VNGqn3W /ip4/100.80.84.82/tcp/30336/p2p/12D3KooWQ1Mz8QkkdgCxwUvmU1YYqsbifCK4fZBq7tKpEzW2qHxX"
    ["bnb-pbc"]="/ip4/100.102.128.51/tcp/30335/p2p/12D3KooWHfQm2kYQ5xEK7N9vF8vqKGt7CZnCmZLBJGMvNd4mA8bM /ip4/100.89.102.75/tcp/30335/p2p/12D3KooWGWGqXRPmWdFMUEkxnLVZHsE6FqVh7s82B1P3JNKhZLqK /ip4/100.80.84.82/tcp/30335/p2p/12D3KooWN5s5pdfQHT9B6Y9VtJQAzN7E7d2QvEsLwXHZvJUXmqFS"
    ["doge-pbc"]="/ip4/100.102.128.51/tcp/30337/p2p/12D3KooWCD8ZF46mNLtbwe7bDS9aAfZibbbyzVKU9D3W6VoQt46u /ip4/100.89.102.75/tcp/30337/p2p/12D3KooWRyrqU3CzbjnATYs3M1d6cS66wzhb98Vt1hXVSUZMdrUb /ip4/100.80.84.82/tcp/30337/p2p/12D3KooWCHWfRQFNKancbJuCxgygWjDd1Vy4kzv6ReVXHPHUi9QM"
    ["edsc-pbc"]="/ip4/100.102.128.51/tcp/30338/p2p/12D3KooWFNqsuJyBqChKpQxnuiwXaVcpqHSE7tHtGoQXsEYztarP /ip4/100.89.102.75/tcp/30338/p2p/12D3KooWE4m3Hf9n7MKVvFZ1p7TqpVeq9KPNmZ2zMLBH1CEi4rhS /ip4/100.80.84.82/tcp/30338/p2p/12D3KooWQfVhSig9RdZAQQMy8UdsNzjF8NfKmExd5ZJriE8d6qz1"
    ["link-pbc"]="/ip4/100.102.128.51/tcp/30340/p2p/12D3KooWRpnWFa2x72opp4tP3y6ACqM5pCTqteJn9v99aoctqhSF /ip4/100.89.102.75/tcp/30340/p2p/12D3KooWMVvHLGsLwep9fGnZPMZrUwjYQQihRQvptJhBH8njr8Qi /ip4/100.80.84.82/tcp/30340/p2p/12D3KooWDrDGoigHJyjyi8mN1uRtqpErQzJK5HPQnvUUKPfhSx2T"
    ["matic-pbc"]="/ip4/100.102.128.51/tcp/30341/p2p/12D3KooWJLxWSvYVfDXtTW5bPEiW69tXYxyk5T4DGPVH1pExNTzu /ip4/100.89.102.75/tcp/30341/p2p/12D3KooWDkbKKGGzHEJVTL2wVv1qs59yJJesvVFKuzx2zQGgP5dv /ip4/100.80.84.82/tcp/30341/p2p/12D3KooWQhferoer3ZgjWSs5TNNExnyn9MgRJ3uew1NWGtRp6Khq"
    ["sc-usdt-pbc"]="/ip4/100.102.128.51/tcp/30342/p2p/12D3KooWLqmMBjXYaP896wenxXakybbUCV2xL42J5D2jocPKb5g5 /ip4/100.89.102.75/tcp/30342/p2p/12D3KooWMghLGWxQf365bFQC5BVAYkW7NPfA9dzhgyEvxaHGU5Gx /ip4/100.80.84.82/tcp/30342/p2p/12D3KooWPYbGBm2SuXtNe5mx5FhaurJ1N3H9QZF2BpfLAmabHuy2"
    ["sol-pbc"]="/ip4/100.102.128.51/tcp/30343/p2p/12D3KooWRuoA6BRNhGPF4raJpDicHvSq9bqomuGiS4Dcjqesz7uT /ip4/100.89.102.75/tcp/30343/p2p/12D3KooWJ1GmZmaXxsUjttABsEiegmPVeM4VGwhhxtQU5san2J1S /ip4/100.80.84.82/tcp/30343/p2p/12D3KooWHbnkZU3TWZJcfCnSuH2uyjqmW7b6tfoJTXGz3J26TRz2"
    ["trx-pbc"]="/ip4/100.102.128.51/tcp/30344/p2p/12D3KooWKWnSjJFtPHjA5uMUwmsj1K69JWEQ2ACoBk4wqg3iY4eS /ip4/100.89.102.75/tcp/30344/p2p/12D3KooWBRoSDPPDCVqRQU1noAHNRo7JXNH2NAYQYgi7ECnEyrm5 /ip4/100.80.84.82/tcp/30344/p2p/12D3KooWNbNiqmWuHXypSakLw7AhxovPvBwRHJuyUeeDg3BocqyH"
    ["xlm-pbc"]="/ip4/100.102.128.51/tcp/30345/p2p/12D3KooWHkdUXykD7srEkWa1Upmdy9n2kUz2oEY6pb7ugN3cDYqV /ip4/100.89.102.75/tcp/30345/p2p/12D3KooWQbpz8nTAwCHDjFQq36YUEwymGdLji9RUzQSkaVhLXeaj /ip4/100.80.84.82/tcp/30345/p2p/12D3KooWGAeV9FbRdH2VUYi5ksA7dVLJtEFjcgmfJjkCRbyiMKGV"
    ["xrp-pbc"]="/ip4/100.102.128.51/tcp/30346/p2p/12D3KooWM1DYFF33zLkzVm5RrwWJeR91BRWTPbfDrkkBeLGoh9Gn /ip4/100.89.102.75/tcp/30346/p2p/12D3KooWGrZrdEnhX4HHSBJxuBgYX6jF9jFKwjMcrbXniZVN23dk /ip4/100.80.84.82/tcp/30346/p2p/12D3KooWQq2RtcAdSVwispQeRpV91UJECpEaz8iRqxDDEkXvprgX"
)

SSH_KEY="$HOME/.ssh/id_rsa"

# SSH execute function
ssh_exec() {
    local vm=$1
    local cmd=$2
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 root@${vm}.contaboserver.net "$cmd"
}

# Start collator function
start_collator() {
    local vm=$1
    local chain=$2
    local vm_number=$3

    local chain_name=$(echo $chain | sed 's/-pbc$//')
    local rpc_port=${CHAIN_PORTS[$chain]}
    local p2p_port=${CHAIN_P2P_PORTS[$chain]}
    local pbc_id=${CHAIN_IDS[$chain]}
    local boot_nodes=${BOOT_NODES[$chain]}

    echo -e "${YELLOW}Starting $chain on $vm (RPC: $rpc_port, P2P: $p2p_port)...${NC}"

    # Check if binary exists
    local binary_check=$(ssh_exec "$vm" "[ -f /root/pbc-collators/${chain}-collator ] && echo 'exists' || echo 'missing'")
    if [ "$binary_check" != "exists" ]; then
        echo -e "${RED}  ✗ Binary ${chain}-collator not found on $vm${NC}"
        return 1
    fi

    # Build bootnode arguments
    local bootnode_args=""
    for bn in $boot_nodes; do
        bootnode_args="$bootnode_args --bootnodes $bn"
    done

    # Start the collator
    local start_cmd="cd /root/pbc-collators && nohup ./${chain}-collator \
        --chain /var/lib/etrid/chainspecs/${chain}-spec.json \
        --base-path /var/lib/etrid/${chain}-collator-${vm_number} \
        --pbc-id $pbc_id \
        --rpc-port $rpc_port \
        --rpc-cors all \
        --rpc-methods Safe \
        --rpc-external \
        --port $p2p_port \
        --name ${chain^^}-VALIDATOR-${vm_number} \
        --validator \
        --telemetry-url 'wss://telemetry.etrid.org/submit/ 0' \
        $bootnode_args \
        > /root/pbc-logs/${chain}-${vm_number}.log 2>&1 &"

    ssh_exec "$vm" "$start_cmd"

    sleep 2

    # Verify process started
    local pid=$(ssh_exec "$vm" "pgrep -f '${chain}-collator' | tail -1" || echo "not_running")
    if [ "$pid" != "not_running" ] && [ ! -z "$pid" ]; then
        echo -e "${GREEN}  ✓ Started (PID: $pid)${NC}"
        return 0
    else
        echo -e "${RED}  ✗ Failed to start${NC}"
        # Check log for errors
        ssh_exec "$vm" "tail -20 /root/pbc-logs/${chain}-${vm_number}.log 2>/dev/null" || true
        return 1
    fi
}

# Main execution
echo -e "\n${BLUE}Distributing PBC collators across ${#VMS[@]} validator VMs...${NC}\n"

# Track statistics
declare -A STARTED_COLLATORS
TOTAL_STARTED=0
TOTAL_FAILED=0

# Distribute chains across VMs (2 chains per VM for load balancing)
vm_index=0
chain_index=0
chains_per_vm=2

for vm in "${VMS[@]}"; do
    vm_index=$((vm_index + 1))
    echo -e "${GREEN}=== VM: $vm (Validator $vm_index) ===${NC}"

    # Assign 2 chains to this VM (round-robin)
    for i in $(seq 0 $((chains_per_vm - 1))); do
        chain_idx=$(( (vm_index - 1) * chains_per_vm + i ))
        chain_idx_mod=$((chain_idx % ${#PBC_CHAINS[@]}))
        chain=${PBC_CHAINS[$chain_idx_mod]}

        if start_collator "$vm" "$chain" "$vm_index"; then
            STARTED_COLLATORS["${chain}_${vm}"]="running"
            TOTAL_STARTED=$((TOTAL_STARTED + 1))
        else
            TOTAL_FAILED=$((TOTAL_FAILED + 1))
        fi
    done

    echo ""
done

# Summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Deployment Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "Total VMs: ${BLUE}${#VMS[@]}${NC}"
echo -e "Successfully started: ${GREEN}$TOTAL_STARTED${NC}"
echo -e "Failed: ${RED}$TOTAL_FAILED${NC}"
echo ""

# Generate deployment report
REPORT_FILE="/Users/macbook/Desktop/pbc-production-deployment-$(date +%Y%m%d-%H%M%S).txt"
{
    echo "ETRID PBC Network - Production Deployment Report"
    echo "Date: $(date)"
    echo "========================================"
    echo ""
    echo "Deployment Statistics:"
    echo "  Total VMs: ${#VMS[@]}"
    echo "  Successfully started: $TOTAL_STARTED"
    echo "  Failed: $TOTAL_FAILED"
    echo ""
    echo "Started Collators:"
    for key in "${!STARTED_COLLATORS[@]}"; do
        echo "  - $key"
    done | sort
    echo ""
} > "$REPORT_FILE"

echo -e "${GREEN}Report saved to: $REPORT_FILE${NC}"

# Health check
echo -e "\n${BLUE}Performing health check (waiting 10s for initialization)...${NC}"
sleep 10

echo -e "\n${YELLOW}Active Collators by VM:${NC}"
for vm in "${VMS[@]}"; do
    count=$(ssh_exec "$vm" "pgrep -f 'pbc-collator' | wc -l" || echo "0")
    if [ "$count" -gt 0 ]; then
        echo -e "  ${vm}: ${GREEN}$count process(es)${NC}"
    fi
done

echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}PBC Network Deployment Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
