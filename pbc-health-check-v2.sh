#!/usr/bin/env bash

# Comprehensive PBC Health Check v2
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
echo -e "${BLUE}ËTRID PBC Network - Comprehensive Health Check v2${NC}"
echo -e "${BLUE}============================================================${NC}"
echo -e "Date: $(date)"
echo ""

# All 22 validator VMs
declare -a VMS=(
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
    "vmi2897381"
    "vmi2897382"
    "vmi2897383"
    "vmi2897384"
)

# PBC chains
declare -a PBC_CHAINS=(
    "btc:9944"
    "eth:9945"
    "sol:9946"
    "xrp:9947"
    "bnb:9948"
    "trx:9949"
    "usdt:9950"
    "doge:9951"
    "ada:9952"
    "link:9953"
    "xlm:9954"
    "edsc:9955"
)

SSH_KEY="$HOME/.ssh/id_rsa"

# Statistics
TOTAL_VMS=0
REACHABLE_VMS=0
TOTAL_COLLATORS=0

# Output file
OUTPUT_FILE="/Users/macbook/Desktop/pbc-health-detailed-$(date +%Y%m%d-%H%M%S).txt"

# SSH function
ssh_exec() {
    local vm=$1
    local cmd=$2
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 root@${vm}.contaboserver.net "$cmd" 2>/dev/null
}

# JSON RPC call
rpc_call() {
    local vm=$1
    local port=$2
    local method=$3
    local params=$4

    if [ -z "$params" ]; then
        ssh_exec "$vm" "curl -s -m 5 -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"$method\"}' http://127.0.0.1:${port}" 2>/dev/null
    else
        ssh_exec "$vm" "curl -s -m 5 -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"$method\",\"params\":[$params]}' http://127.0.0.1:${port}" 2>/dev/null
    fi
}

# Extract JSON value using grep and sed
extract_json_value() {
    local json=$1
    local key=$2

    echo "$json" | grep -o "\"$key\":[^,}]*" | sed 's/.*://g' | tr -d '"' | tr -d ' '
}

# Hex to decimal conversion
hex_to_dec() {
    local hex=$1
    # Remove 0x prefix if present
    hex=${hex#0x}
    # Convert to decimal
    echo $((16#$hex))
}

echo "" >> "$OUTPUT_FILE"
echo "ËTRID PBC Network - Health Check Report" >> "$OUTPUT_FILE"
echo "Date: $(date)" >> "$OUTPUT_FILE"
echo "============================================================" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Process each VM
for vm in "${VMS[@]}"; do
    TOTAL_VMS=$((TOTAL_VMS + 1))

    echo -e "${GREEN}=== VM: ${vm} ===${NC}"
    echo "=== VM: ${vm} ===" >> "$OUTPUT_FILE"

    # Check reachability
    if ! ssh_exec "$vm" "echo ok" >/dev/null 2>&1; then
        echo -e "  ${RED}✗ Unreachable${NC}"
        echo "  Status: UNREACHABLE" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
        continue
    fi

    REACHABLE_VMS=$((REACHABLE_VMS + 1))
    echo -e "  ${GREEN}✓ Reachable${NC}"
    echo "  Status: REACHABLE" >> "$OUTPUT_FILE"

    # Get running processes
    echo -e "  ${YELLOW}Checking processes...${NC}"
    processes=$(ssh_exec "$vm" "ps aux | grep 'pbc' | grep -v grep" 2>/dev/null || echo "")

    if [ -z "$processes" ]; then
        echo -e "  ${YELLOW}No PBC collators running${NC}"
        echo "  Collators: 0" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
        continue
    fi

    # Count processes
    process_count=$(echo "$processes" | wc -l | tr -d ' ')
    TOTAL_COLLATORS=$((TOTAL_COLLATORS + process_count))

    echo -e "  ${CYAN}Running collators: ${process_count}${NC}"
    echo "  Collators: ${process_count}" >> "$OUTPUT_FILE"

    # List running chains
    echo -e "  ${YELLOW}Active chains:${NC}"
    echo "  Active chains:" >> "$OUTPUT_FILE"

    # Check each PBC chain
    for pbc_port in "${PBC_CHAINS[@]}"; do
        IFS=':' read -r chain port <<< "$pbc_port"

        # Check if this chain is running on this VM
        if ! echo "$processes" | grep -q "$chain"; then
            continue
        fi

        echo -e "    ${BLUE}${chain^^}:${NC}"
        echo "    - ${chain^^}:" >> "$OUTPUT_FILE"

        # Get PID
        pid=$(ssh_exec "$vm" "pgrep -f '${chain}.*pbc' | head -1" 2>/dev/null || echo "unknown")
        echo -e "      PID: ${pid}"
        echo "      PID: ${pid}" >> "$OUTPUT_FILE"

        # Try RPC health check
        health=$(rpc_call "$vm" "$port" "system_health" "")

        if echo "$health" | grep -q "result"; then
            echo -e "      ${GREEN}✓ RPC responding${NC}"
            echo "      RPC: OK" >> "$OUTPUT_FILE"

            # Extract peers
            peers=$(echo "$health" | grep -o '"peers":[0-9]*' | grep -o '[0-9]*' || echo "0")
            echo -e "      Peers: ${CYAN}${peers}${NC}"
            echo "      Peers: ${peers}" >> "$OUTPUT_FILE"

            # Extract syncing status
            syncing=$(echo "$health" | grep -o '"isSyncing":[a-z]*' | sed 's/.*://g' || echo "unknown")
            echo -e "      Syncing: ${YELLOW}${syncing}${NC}"
            echo "      Syncing: ${syncing}" >> "$OUTPUT_FILE"

            # Get current block
            header=$(rpc_call "$vm" "$port" "chain_getHeader" "")

            if echo "$header" | grep -q "number"; then
                # Extract hex block number
                block_hex=$(echo "$header" | grep -o '"number":"0x[0-9a-f]*"' | sed 's/.*"0x//g' | sed 's/"//g' | head -1)

                if [ ! -z "$block_hex" ]; then
                    block_num=$(hex_to_dec "$block_hex")
                    echo -e "      Current block: ${CYAN}#${block_num}${NC}"
                    echo "      Current block: #${block_num}" >> "$OUTPUT_FILE"
                fi
            fi

            # Get finalized block
            finalized=$(rpc_call "$vm" "$port" "chain_getFinalizedHead" "")

            if echo "$finalized" | grep -q "result"; then
                fin_hash=$(echo "$finalized" | grep -o '"result":"0x[0-9a-f]*"' | sed 's/.*"0x//g' | sed 's/"//g')

                if [ ! -z "$fin_hash" ]; then
                    # Get finalized block header
                    fin_header=$(rpc_call "$vm" "$port" "chain_getHeader" "\"0x${fin_hash}\"")

                    if echo "$fin_header" | grep -q "number"; then
                        fin_block_hex=$(echo "$fin_header" | grep -o '"number":"0x[0-9a-f]*"' | sed 's/.*"0x//g' | sed 's/"//g' | head -1)

                        if [ ! -z "$fin_block_hex" ]; then
                            fin_block=$(hex_to_dec "$fin_block_hex")
                            echo -e "      Finalized block: ${CYAN}#${fin_block}${NC}"
                            echo "      Finalized block: #${fin_block}" >> "$OUTPUT_FILE"

                            # Calculate finalization lag
                            if [ ! -z "$block_num" ] && [ "$block_num" -gt 0 ]; then
                                lag=$((block_num - fin_block))
                                if [ $lag -le 10 ]; then
                                    echo -e "      ${GREEN}✓ Finalization healthy (lag: ${lag} blocks)${NC}"
                                    echo "      Finalization: HEALTHY (lag: ${lag} blocks)" >> "$OUTPUT_FILE"
                                elif [ $lag -le 50 ]; then
                                    echo -e "      ${YELLOW}⚠ Finalization lagging (lag: ${lag} blocks)${NC}"
                                    echo "      Finalization: LAGGING (lag: ${lag} blocks)" >> "$OUTPUT_FILE"
                                else
                                    echo -e "      ${RED}✗ Finalization stalled (lag: ${lag} blocks)${NC}"
                                    echo "      Finalization: STALLED (lag: ${lag} blocks)" >> "$OUTPUT_FILE"
                                fi
                            fi
                        fi
                    fi
                fi
            fi

            # Get peer list
            peer_list=$(rpc_call "$vm" "$port" "system_peers" "")
            if echo "$peer_list" | grep -q "peerId"; then
                peer_count=$(echo "$peer_list" | grep -o "peerId" | wc -l | tr -d ' ')
                echo -e "      Connected peers: ${CYAN}${peer_count}${NC}"
                echo "      Connected peers: ${peer_count}" >> "$OUTPUT_FILE"
            fi

        else
            echo -e "      ${RED}✗ RPC not responding on port ${port}${NC}"
            echo "      RPC: FAILED (port ${port})" >> "$OUTPUT_FILE"
        fi

        echo "" >> "$OUTPUT_FILE"
    done

    echo "" >> "$OUTPUT_FILE"
done

# Summary
echo -e ""
echo -e "${BLUE}============================================================${NC}"
echo -e "${BLUE}SUMMARY${NC}"
echo -e "${BLUE}============================================================${NC}"
echo -e "Total VMs checked: ${TOTAL_VMS}"
echo -e "Reachable VMs: ${GREEN}${REACHABLE_VMS}${NC}"
echo -e "Unreachable VMs: ${RED}$((TOTAL_VMS - REACHABLE_VMS))${NC}"
echo -e "Total running collators: ${CYAN}${TOTAL_COLLATORS}${NC}"

echo "" >> "$OUTPUT_FILE"
echo "============================================================" >> "$OUTPUT_FILE"
echo "SUMMARY" >> "$OUTPUT_FILE"
echo "============================================================" >> "$OUTPUT_FILE"
echo "Total VMs checked: ${TOTAL_VMS}" >> "$OUTPUT_FILE"
echo "Reachable VMs: ${REACHABLE_VMS}" >> "$OUTPUT_FILE"
echo "Unreachable VMs: $((TOTAL_VMS - REACHABLE_VMS))" >> "$OUTPUT_FILE"
echo "Total running collators: ${TOTAL_COLLATORS}" >> "$OUTPUT_FILE"

echo -e ""
echo -e "${GREEN}Report saved to: ${OUTPUT_FILE}${NC}"
echo ""
