#!/usr/bin/env bash

# Comprehensive PBC Health Check
# Checks all running PBC collators across all 22 validators
# Author: Eoj
# Date: 2025-12-03

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${BLUE}============================================================${NC}"
echo -e "${BLUE}ËTRID PBC Network - Comprehensive Health Check${NC}"
echo -e "${BLUE}============================================================${NC}"
echo -e "Date: $(date)"
echo ""

# All 22 validator VMs
declare -a VMS=(
    "vmi2896906"  # ts-val-03
    "vmi2896907"  # ts-val-04
    "vmi2896908"  # ts-val-05
    "vmi2896909"  # ts-val-06
    "vmi2896910"  # ts-val-07
    "vmi2896911"  # ts-val-08
    "vmi2896914"  # ts-val-09
    "vmi2896915"  # ts-val-10
    "vmi2896916"  # ts-val-11
    "vmi2896917"  # ts-val-12
    "vmi2896918"  # ts-val-13
    "vmi2896921"  # ts-val-14
    "vmi2896922"  # ts-val-15
    "vmi2896923"  # ts-val-16
    "vmi2896924"  # ts-val-17
    "vmi2896925"  # ts-val-18
    "vmi2897381"  # ts-val-19
    "vmi2897382"  # ts-val-20
    "vmi2897383"  # ts-val-21
    "vmi2897384"  # ts-val-22
)

# Oracle Cloud VMs (if accessible)
declare -a ORACLE_VMS=(
    "100.96.84.69"    # ts-val-01 (gizzi-io-validator)
    "100.70.242.106"  # ts-val-02 (auditdev)
)

# PBC chains and their port assignments
declare -A PBC_PORTS=(
    ["btc"]=9944
    ["eth"]=9945
    ["sol"]=9946
    ["xrp"]=9947
    ["bnb"]=9948
    ["trx"]=9949
    ["usdt"]=9950
    ["doge"]=9951
    ["ada"]=9952
    ["link"]=9953
    ["xlm"]=9954
    ["edsc"]=9955
)

# Alternative port mapping from production script
declare -A PBC_PORTS_ALT=(
    ["ada"]=9945
    ["btc"]=9947
    ["bnb"]=9960
    ["doge"]=9948
    ["edsc"]=9949
    ["link"]=9951
    ["matic"]=9952
    ["sc-usdt"]=9953
    ["sol"]=9954
    ["trx"]=9955
    ["xlm"]=9956
    ["xrp"]=9957
)

SSH_KEY="$HOME/.ssh/id_rsa"

# Track statistics
declare -A VM_REACHABLE
declare -A VM_PROCESSES
declare -A CHAIN_BLOCKS
declare -A CHAIN_FINALIZED
declare -A CHAIN_PEERS
declare -A CHAIN_STATUS

TOTAL_VMS=0
REACHABLE_VMS=0
TOTAL_RUNNING_COLLATORS=0

# SSH execute function with timeout
ssh_exec() {
    local vm=$1
    local cmd=$2
    local timeout=${3:-10}

    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=$timeout root@${vm}.contaboserver.net "$cmd" 2>/dev/null
}

# SSH execute for Oracle VMs (direct IP)
ssh_exec_oracle() {
    local ip=$1
    local cmd=$2
    local timeout=${3:-10}

    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=$timeout root@${ip} "$cmd" 2>/dev/null
}

# Check VM reachability
check_vm_reachability() {
    local vm=$1
    echo -e "${YELLOW}Checking VM: ${vm}${NC}"

    TOTAL_VMS=$((TOTAL_VMS + 1))

    # Try to connect
    if ssh_exec "$vm" "echo 'ok'" 5 >/dev/null 2>&1; then
        VM_REACHABLE["$vm"]="yes"
        REACHABLE_VMS=$((REACHABLE_VMS + 1))
        echo -e "  ${GREEN}✓ Reachable${NC}"
        return 0
    else
        VM_REACHABLE["$vm"]="no"
        echo -e "  ${RED}✗ Unreachable${NC}"
        return 1
    fi
}

# Get running processes on VM
get_running_processes() {
    local vm=$1

    if [ "${VM_REACHABLE[$vm]}" != "yes" ]; then
        return 1
    fi

    echo -e "${YELLOW}  Checking running processes...${NC}"

    # Get PBC collator processes
    local process_list=$(ssh_exec "$vm" "ps aux | grep -E '(pbc|collator)' | grep -v grep" 20)
    local process_count=$(echo "$process_list" | grep -c "pbc" 2>/dev/null || echo "0")
    process_count=$(echo "$process_count" | tr -d '\n' | tr -d ' ')

    # Ensure it's a valid number
    if ! [[ "$process_count" =~ ^[0-9]+$ ]]; then
        process_count=0
    fi

    VM_PROCESSES["$vm"]="$process_count"
    TOTAL_RUNNING_COLLATORS=$((TOTAL_RUNNING_COLLATORS + process_count))

    echo -e "  ${CYAN}Running collators: ${process_count}${NC}"

    if [ "$process_count" -gt 0 ]; then
        echo "$process_list" | while read -r line; do
            if echo "$line" | grep -q "pbc"; then
                local chain=$(echo "$line" | grep -oP '(btc|eth|sol|xrp|bnb|trx|usdt|doge|ada|link|xlm|edsc|matic|sc-usdt)-pbc' | head -1)
                if [ ! -z "$chain" ]; then
                    echo -e "    ${GREEN}→ ${chain}${NC}"
                fi
            fi
        done
    fi
}

# Check RPC health for a chain
check_rpc_health() {
    local vm=$1
    local chain=$2
    local port=$3

    # Try to connect to RPC
    local health=$(ssh_exec "$vm" "curl -s -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_health\"}' http://127.0.0.1:${port}" 15)

    if echo "$health" | grep -q "result"; then
        local peers=$(echo "$health" | grep -oP '"peers":\K[0-9]+' || echo "0")
        local is_syncing=$(echo "$health" | grep -oP '"isSyncing":\K(true|false)' || echo "unknown")

        CHAIN_PEERS["${chain}_${vm}"]="$peers"
        CHAIN_STATUS["${chain}_${vm}"]="syncing:$is_syncing"

        echo -e "    ${GREEN}✓ RPC responding${NC} | Peers: ${CYAN}${peers}${NC} | Syncing: ${YELLOW}${is_syncing}${NC}"

        # Get block header
        local header=$(ssh_exec "$vm" "curl -s -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\"}' http://127.0.0.1:${port}" 15)

        if echo "$header" | grep -q "result"; then
            local block_num=$(echo "$header" | grep -oP '"number":"0x\K[0-9a-f]+' | head -1)
            if [ ! -z "$block_num" ]; then
                # Convert hex to decimal
                block_num=$((16#$block_num))
                CHAIN_BLOCKS["${chain}_${vm}"]="$block_num"
                echo -e "    ${CYAN}Current block: #${block_num}${NC}"
            fi
        fi

        # Get finalized head
        local finalized=$(ssh_exec "$vm" "curl -s -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getFinalizedHead\"}' http://127.0.0.1:${port}" 15)

        if echo "$finalized" | grep -q "result"; then
            local fin_hash=$(echo "$finalized" | grep -oP '"result":"\K0x[0-9a-f]+' | head -1)
            if [ ! -z "$fin_hash" ]; then
                # Get finalized block header
                local fin_header=$(ssh_exec "$vm" "curl -s -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"'$fin_hash'\"]}' http://127.0.0.1:${port}" 15)

                if echo "$fin_header" | grep -q "result"; then
                    local fin_block=$(echo "$fin_header" | grep -oP '"number":"0x\K[0-9a-f]+' | head -1)
                    if [ ! -z "$fin_block" ]; then
                        fin_block=$((16#$fin_block))
                        CHAIN_FINALIZED["${chain}_${vm}"]="$fin_block"
                        echo -e "    ${CYAN}Finalized block: #${fin_block}${NC}"

                        # Check if finalization is close to current block
                        if [ ! -z "${CHAIN_BLOCKS[${chain}_${vm}]}" ]; then
                            local current_block=${CHAIN_BLOCKS["${chain}_${vm}"]}
                            local diff=$((current_block - fin_block))
                            if [ $diff -le 10 ]; then
                                echo -e "    ${GREEN}✓ Finalization healthy (lag: ${diff} blocks)${NC}"
                            else
                                echo -e "    ${YELLOW}⚠ Finalization lagging (lag: ${diff} blocks)${NC}"
                            fi
                        fi
                    fi
                fi
            fi
        fi

        # Get system peers
        local peers_info=$(ssh_exec "$vm" "curl -s -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_peers\"}' http://127.0.0.1:${port}" 15)

        if echo "$peers_info" | grep -q "result"; then
            local peer_count=$(echo "$peers_info" | grep -o "peerId" | wc -l)
            echo -e "    ${CYAN}Connected peers: ${peer_count}${NC}"
        fi

        return 0
    else
        echo -e "    ${RED}✗ RPC not responding${NC}"
        return 1
    fi
}

# Check all chains on a VM
check_vm_chains() {
    local vm=$1

    if [ "${VM_REACHABLE[$vm]}" != "yes" ]; then
        return 1
    fi

    echo -e "${YELLOW}  Checking chain health...${NC}"

    # Try both port mappings
    for chain in btc eth sol xrp bnb trx usdt doge ada link xlm edsc matic sc-usdt; do
        # Check if process is running first
        local process_check=$(ssh_exec "$vm" "pgrep -f '${chain}.*pbc' || echo 'not_running'" 10)

        if [ "$process_check" != "not_running" ] && [ ! -z "$process_check" ]; then
            echo -e "  ${BLUE}Chain: ${chain}${NC}"

            # Try primary port mapping
            local port=${PBC_PORTS[$chain]}
            if [ ! -z "$port" ]; then
                check_rpc_health "$vm" "$chain" "$port"
            fi

            # If RPC didn't respond, try alternative port
            if [ -z "${CHAIN_BLOCKS[${chain}_${vm}]}" ]; then
                port=${PBC_PORTS_ALT[$chain]}
                if [ ! -z "$port" ]; then
                    check_rpc_health "$vm" "$chain" "$port"
                fi
            fi
        fi
    done
}

# Main execution
main() {
    echo -e "${CYAN}Step 1: Checking Contabo VMs...${NC}\n"

    for vm in "${VMS[@]}"; do
        echo -e "${GREEN}=== ${vm} ===${NC}"
        if check_vm_reachability "$vm"; then
            get_running_processes "$vm"
            check_vm_chains "$vm"
        fi
        echo ""
    done

    echo -e "${CYAN}Step 2: Checking Oracle Cloud VMs...${NC}\n"

    for ip in "${ORACLE_VMS[@]}"; do
        echo -e "${GREEN}=== Oracle VM: ${ip} ===${NC}"
        TOTAL_VMS=$((TOTAL_VMS + 1))

        if ssh_exec_oracle "$ip" "echo 'ok'" 5 >/dev/null 2>&1; then
            REACHABLE_VMS=$((REACHABLE_VMS + 1))
            echo -e "  ${GREEN}✓ Reachable${NC}"

            # Check processes
            local process_list=$(ssh_exec_oracle "$ip" "ps aux | grep -E '(pbc|collator)' | grep -v grep" 20)
            local process_count=$(echo "$process_list" | grep -c "pbc" 2>/dev/null || echo "0")
            process_count=$(echo "$process_count" | tr -d '\n' | tr -d ' ')

            # Ensure it's a valid number
            if ! [[ "$process_count" =~ ^[0-9]+$ ]]; then
                process_count=0
            fi

            VM_PROCESSES["$ip"]="$process_count"
            TOTAL_RUNNING_COLLATORS=$((TOTAL_RUNNING_COLLATORS + process_count))

            echo -e "  ${CYAN}Running collators: ${process_count}${NC}"
        else
            echo -e "  ${RED}✗ Unreachable${NC}"
        fi
        echo ""
    done

    # Generate summary report
    echo -e "${BLUE}============================================================${NC}"
    echo -e "${BLUE}HEALTH CHECK SUMMARY${NC}"
    echo -e "${BLUE}============================================================${NC}"
    echo ""

    echo -e "${YELLOW}Infrastructure Status:${NC}"
    echo -e "  Total VMs: ${TOTAL_VMS}"
    echo -e "  Reachable VMs: ${GREEN}${REACHABLE_VMS}${NC}"
    echo -e "  Unreachable VMs: ${RED}$((TOTAL_VMS - REACHABLE_VMS))${NC}"
    echo -e "  Total Running Collators: ${CYAN}${TOTAL_RUNNING_COLLATORS}${NC}"
    echo ""

    echo -e "${YELLOW}Block Heights by Chain:${NC}"

    # Aggregate block heights per chain
    declare -A CHAIN_MAX_BLOCK
    declare -A CHAIN_MIN_BLOCK
    declare -A CHAIN_NODE_COUNT

    for key in "${!CHAIN_BLOCKS[@]}"; do
        local chain=$(echo "$key" | cut -d'_' -f1)
        local block=${CHAIN_BLOCKS[$key]}

        if [ -z "${CHAIN_MAX_BLOCK[$chain]}" ] || [ $block -gt ${CHAIN_MAX_BLOCK[$chain]} ]; then
            CHAIN_MAX_BLOCK["$chain"]="$block"
        fi

        if [ -z "${CHAIN_MIN_BLOCK[$chain]}" ] || [ $block -lt ${CHAIN_MIN_BLOCK[$chain]} ]; then
            CHAIN_MIN_BLOCK["$chain"]="$block"
        fi

        CHAIN_NODE_COUNT["$chain"]=$((${CHAIN_NODE_COUNT[$chain]:-0} + 1))
    done

    for chain in btc eth sol xrp bnb trx usdt doge ada link xlm edsc matic sc-usdt; do
        if [ ! -z "${CHAIN_MAX_BLOCK[$chain]}" ]; then
            local max_block=${CHAIN_MAX_BLOCK[$chain]}
            local min_block=${CHAIN_MIN_BLOCK[$chain]}
            local node_count=${CHAIN_NODE_COUNT[$chain]}
            local diff=$((max_block - min_block))

            echo -e "  ${CYAN}${chain^^}:${NC} Blocks #${min_block} - #${max_block} (${node_count} nodes)"

            if [ $diff -le 5 ]; then
                echo -e "    ${GREEN}✓ All nodes in sync (max difference: ${diff} blocks)${NC}"
            else
                echo -e "    ${YELLOW}⚠ Nodes out of sync (difference: ${diff} blocks)${NC}"
            fi
        fi
    done

    echo ""
    echo -e "${YELLOW}Peer Connectivity:${NC}"

    # Aggregate peer counts
    declare -A CHAIN_TOTAL_PEERS

    for key in "${!CHAIN_PEERS[@]}"; do
        local chain=$(echo "$key" | cut -d'_' -f1)
        local peers=${CHAIN_PEERS[$key]}
        CHAIN_TOTAL_PEERS["$chain"]=$((${CHAIN_TOTAL_PEERS[$chain]:-0} + peers))
    done

    for chain in btc eth sol xrp bnb trx usdt doge ada link xlm edsc matic sc-usdt; do
        if [ ! -z "${CHAIN_TOTAL_PEERS[$chain]}" ]; then
            local total_peers=${CHAIN_TOTAL_PEERS[$chain]}
            local node_count=${CHAIN_NODE_COUNT[$chain]:-0}
            local avg_peers=0
            if [ $node_count -gt 0 ]; then
                avg_peers=$((total_peers / node_count))
            fi

            echo -e "  ${CYAN}${chain^^}:${NC} ${total_peers} total peers (avg: ${avg_peers} per node)"

            if [ $avg_peers -ge 3 ]; then
                echo -e "    ${GREEN}✓ Good connectivity${NC}"
            elif [ $avg_peers -ge 1 ]; then
                echo -e "    ${YELLOW}⚠ Low connectivity${NC}"
            else
                echo -e "    ${RED}✗ No peers${NC}"
            fi
        fi
    done

    echo ""
    echo -e "${YELLOW}Finalization Status:${NC}"

    for chain in btc eth sol xrp bnb trx usdt doge ada link xlm edsc matic sc-usdt; do
        if [ ! -z "${CHAIN_MAX_BLOCK[$chain]}" ] && [ ! -z "${CHAIN_FINALIZED[${chain}_${VMS[0]}]}" ]; then
            local max_block=${CHAIN_MAX_BLOCK[$chain]}
            local finalized=0
            local fin_count=0

            # Calculate average finalized block
            for key in "${!CHAIN_FINALIZED[@]}"; do
                if [[ "$key" == "${chain}_"* ]]; then
                    finalized=$((finalized + ${CHAIN_FINALIZED[$key]}))
                    fin_count=$((fin_count + 1))
                fi
            done

            if [ $fin_count -gt 0 ]; then
                finalized=$((finalized / fin_count))
                local lag=$((max_block - finalized))

                echo -e "  ${CYAN}${chain^^}:${NC} Block #${finalized} finalized (lag: ${lag} blocks)"

                if [ $lag -le 10 ]; then
                    echo -e "    ${GREEN}✓ Finalization healthy${NC}"
                elif [ $lag -le 50 ]; then
                    echo -e "    ${YELLOW}⚠ Finalization lagging${NC}"
                else
                    echo -e "    ${RED}✗ Finalization stalled${NC}"
                fi
            fi
        fi
    done

    echo ""
    echo -e "${BLUE}============================================================${NC}"
    echo -e "${GREEN}Health check complete!${NC}"
    echo -e "${BLUE}============================================================${NC}"

    # Save report
    REPORT_FILE="/Users/macbook/Desktop/pbc-health-report-$(date +%Y%m%d-%H%M%S).txt"
    {
        echo "ËTRID PBC Network - Health Check Report"
        echo "Date: $(date)"
        echo "============================================================"
        echo ""
        echo "Infrastructure Status:"
        echo "  Total VMs: ${TOTAL_VMS}"
        echo "  Reachable VMs: ${REACHABLE_VMS}"
        echo "  Unreachable VMs: $((TOTAL_VMS - REACHABLE_VMS))"
        echo "  Total Running Collators: ${TOTAL_RUNNING_COLLATORS}"
        echo ""
        echo "Unreachable VMs:"
        for vm in "${VMS[@]}" "${ORACLE_VMS[@]}"; do
            if [ "${VM_REACHABLE[$vm]}" == "no" ]; then
                echo "  - $vm"
            fi
        done
        echo ""
        echo "Running Processes by VM:"
        for vm in "${VMS[@]}" "${ORACLE_VMS[@]}"; do
            if [ ! -z "${VM_PROCESSES[$vm]}" ]; then
                echo "  $vm: ${VM_PROCESSES[$vm]} collators"
            fi
        done
        echo ""
        echo "Block Heights:"
        for chain in btc eth sol xrp bnb trx usdt doge ada link xlm edsc matic sc-usdt; do
            if [ ! -z "${CHAIN_MAX_BLOCK[$chain]}" ]; then
                echo "  ${chain}: #${CHAIN_MIN_BLOCK[$chain]} - #${CHAIN_MAX_BLOCK[$chain]} (${CHAIN_NODE_COUNT[$chain]} nodes)"
            fi
        done
    } > "$REPORT_FILE"

    echo -e "${GREEN}Report saved to: ${REPORT_FILE}${NC}"
}

# Run main function
main
