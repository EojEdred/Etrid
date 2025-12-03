#!/usr/bin/env bash

# Final PBC Health Check Report
# Comprehensive health check with RPC validation
# Date: 2025-12-03

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo -e "${BLUE}================================================================${NC}"
echo -e "${BLUE}ËTRID PBC NETWORK - FINAL HEALTH CHECK REPORT${NC}"
echo -e "${BLUE}================================================================${NC}"
echo "Date: $(date)"
echo ""

# All 22 validator VMs
VMS=(
    "vmi2896906" "vmi2896907" "vmi2896908" "vmi2896909" "vmi2896910"
    "vmi2896911" "vmi2896914" "vmi2896915" "vmi2896916" "vmi2896917"
    "vmi2896918" "vmi2896921" "vmi2896922" "vmi2896923" "vmi2896924"
    "vmi2896925" "vmi2897381" "vmi2897382" "vmi2897383" "vmi2897384"
)

# PBC chains
PBC_CHAINS=("ada" "btc" "bnb" "doge" "edsc" "eth" "link" "matic" "sc-usdt" "sol" "trx" "xlm" "xrp")

SSH_KEY="$HOME/.ssh/id_rsa"

# Output
REPORT="/Users/macbook/Desktop/PBC-FINAL-HEALTH-REPORT-$(date +%Y%m%d-%H%M%S).txt"

# Stats
declare -A CHAIN_NODES
declare -A CHAIN_BLOCKS
declare -A CHAIN_PEERS

TOTAL_VMS=0
REACHABLE_VMS=0
TOTAL_COLLATORS=0

ssh_exec() {
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 root@${1}.contaboserver.net "$2" 2>/dev/null
}

rpc_call() {
    local vm=$1
    local port=$2
    local method=$3

    ssh_exec "$vm" "curl -s -m 3 -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"$method\"}' http://127.0.0.1:${port}" 2>/dev/null
}

hex_to_dec() {
    local hex=${1#0x}
    if [ ! -z "$hex" ]; then
        echo $((16#$hex))
    else
        echo "0"
    fi
}

{
    echo "================================================================"
    echo "ËTRID PBC NETWORK - COMPREHENSIVE HEALTH CHECK"
    echo "================================================================"
    echo "Date: $(date)"
    echo ""
    echo "================================================================"
    echo "VM-LEVEL HEALTH CHECK"
    echo "================================================================"
    echo ""
} > "$REPORT"

# Check each VM
for vm in "${VMS[@]}"; do
    TOTAL_VMS=$((TOTAL_VMS + 1))

    printf "%-15s " "$vm"

    if ! ssh_exec "$vm" "echo 'ok'" >/dev/null 2>&1; then
        echo -e "${RED}UNREACHABLE${NC}"
        echo "$vm: UNREACHABLE" >> "$REPORT"
        continue
    fi

    REACHABLE_VMS=$((REACHABLE_VMS + 1))

    # Get list of running collators
    collators=$(ssh_exec "$vm" "ps aux | grep 'pbc-collator' | grep -v grep | grep '/usr/local/bin'" 2>/dev/null)

    if [ -z "$collators" ]; then
        echo -e "${YELLOW}OK - 0 collators${NC}"
        echo "$vm: OK - 0 collators" >> "$REPORT"
        continue
    fi

    # Count collators
    count=$(echo "$collators" | wc -l | tr -d ' ')
    TOTAL_COLLATORS=$((TOTAL_COLLATORS + count))

    echo -e "${GREEN}OK - ${count} collators${NC}"
    echo "$vm: OK - $count collators running" >> "$REPORT"

    # Parse which chains
    chains=$(echo "$collators" | grep -o '/usr/local/bin/[a-z-]*-pbc-collator' | sed 's/\/usr\/local\/bin\///g' | sed 's/-pbc-collator//g' | tr '\n' ' ')
    echo "  Chains: $chains" >> "$REPORT"

    # Get RPC ports for each chain
    while read -r line; do
        if echo "$line" | grep -q "pbc-collator"; then
            chain=$(echo "$line" | grep -o '[a-z-]*-pbc-collator' | sed 's/-pbc-collator//g' | head -1)
            port=$(echo "$line" | grep -o '\--rpc-port [0-9]*' | awk '{print $2}')

            if [ ! -z "$chain" ] && [ ! -z "$port" ]; then
                # Track that this chain exists on this VM
                CHAIN_NODES["$chain"]=$((${CHAIN_NODES[$chain]:-0} + 1))

                # Quick RPC check
                health=$(rpc_call "$vm" "$port" "system_health" 2>/dev/null)

                if echo "$health" | grep -q "result"; then
                    peers=$(echo "$health" | grep -o '"peers":[0-9]*' | grep -o '[0-9]*')
                    CHAIN_PEERS["$chain"]=$((${CHAIN_PEERS[$chain]:-0} + ${peers:-0}))

                    # Get block number
                    header=$(rpc_call "$vm" "$port" "chain_getHeader" 2>/dev/null)

                    if echo "$header" | grep -q "number"; then
                        block_hex=$(echo "$header" | grep -o '"number":"0x[0-9a-f]*"' | sed 's/.*"0x//g' | sed 's/"//g' | head -1)
                        if [ ! -z "$block_hex" ]; then
                            block_num=$(hex_to_dec "$block_hex")
                            # Store max block for this chain
                            if [ -z "${CHAIN_BLOCKS[$chain]}" ] || [ "$block_num" -gt "${CHAIN_BLOCKS[$chain]}" ]; then
                                CHAIN_BLOCKS["$chain"]="$block_num"
                            fi
                            echo "    $chain (port $port): Block #$block_num, $peers peers" >> "$REPORT"
                        fi
                    fi
                fi
            fi
        fi
    done <<< "$collators"

    echo "" >> "$REPORT"
done

echo ""
echo -e "${BLUE}================================================================${NC}"
echo -e "${BLUE}INFRASTRUCTURE SUMMARY${NC}"
echo -e "${BLUE}================================================================${NC}"
echo -e "Total VMs: ${TOTAL_VMS}"
echo -e "Reachable: ${GREEN}${REACHABLE_VMS}${NC}"
echo -e "Unreachable: ${RED}$((TOTAL_VMS - REACHABLE_VMS))${NC}"
echo -e "Total Running Collators: ${CYAN}${TOTAL_COLLATORS}${NC}"

{
    echo ""
    echo "================================================================"
    echo "INFRASTRUCTURE SUMMARY"
    echo "================================================================"
    echo "Total VMs: $TOTAL_VMS"
    echo "Reachable VMs: $REACHABLE_VMS"
    echo "Unreachable VMs: $((TOTAL_VMS - REACHABLE_VMS))"
    echo "Total Running Collators: $TOTAL_COLLATORS"
} >> "$REPORT"

echo ""
echo -e "${BLUE}================================================================${NC}"
echo -e "${BLUE}PBC CHAIN STATUS${NC}"
echo -e "${BLUE}================================================================${NC}"

{
    echo ""
    echo "================================================================"
    echo "PBC CHAIN STATUS"
    echo "================================================================"
    echo ""
} >> "$REPORT"

# Summary per chain
for chain in "${PBC_CHAINS[@]}"; do
    node_count=${CHAIN_NODES[$chain]:-0}
    max_block=${CHAIN_BLOCKS[$chain]:-0}
    total_peers=${CHAIN_PEERS[$chain]:-0}

    if [ $node_count -gt 0 ]; then
        avg_peers=$((total_peers / node_count))

        echo -e "${CYAN}${chain^^}-PBC:${NC}"
        echo "  Nodes: ${GREEN}${node_count}${NC}"
        echo "  Max Block Height: ${MAGENTA}#${max_block}${NC}"
        echo "  Total Peers: ${YELLOW}${total_peers}${NC} (avg: ${avg_peers} per node)"

        echo "${chain^^}-PBC:" >> "$REPORT"
        echo "  Active Nodes: $node_count" >> "$REPORT"
        echo "  Max Block Height: #$max_block" >> "$REPORT"
        echo "  Total Peer Connections: $total_peers (avg: $avg_peers per node)" >> "$REPORT"

        # Health assessment
        if [ $avg_peers -ge 3 ]; then
            echo -e "  Status: ${GREEN}HEALTHY${NC} (good peer connectivity)"
            echo "  Status: HEALTHY (good peer connectivity)" >> "$REPORT"
        elif [ $avg_peers -ge 1 ]; then
            echo -e "  Status: ${YELLOW}MODERATE${NC} (low peer connectivity)"
            echo "  Status: MODERATE (low peer connectivity)" >> "$REPORT"
        else
            echo -e "  Status: ${RED}WARNING${NC} (no peer connectivity)"
            echo "  Status: WARNING (no peer connectivity)" >> "$REPORT"
        fi

        echo "" >> "$REPORT"
        echo ""
    else
        echo -e "${YELLOW}${chain^^}-PBC: NOT RUNNING${NC}"
        echo "${chain^^}-PBC: NOT RUNNING" >> "$REPORT"
        echo "" >> "$REPORT"
        echo ""
    fi
done

echo -e "${BLUE}================================================================${NC}"
echo -e "${GREEN}Health check complete!${NC}"
echo -e "${BLUE}================================================================${NC}"
echo ""
echo -e "${GREEN}Full report saved to:${NC}"
echo -e "${CYAN}$REPORT${NC}"
echo ""
