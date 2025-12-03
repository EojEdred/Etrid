#!/usr/bin/env bash

# Quick PBC Health Check
# Fast check of all PBC collators across 22 validators
# Author: Eoj
# Date: 2025-12-03

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${BLUE}================================================================${NC}"
echo -e "${BLUE}ËTRID PBC Network - Quick Health Check${NC}"
echo -e "${BLUE}================================================================${NC}"
echo "Date: $(date)"
echo ""

# All 22 validator VMs (Contabo)
VMS=(
    "vmi2896906" "vmi2896907" "vmi2896908" "vmi2896909" "vmi2896910"
    "vmi2896911" "vmi2896914" "vmi2896915" "vmi2896916" "vmi2896917"
    "vmi2896918" "vmi2896921" "vmi2896922" "vmi2896923" "vmi2896924"
    "vmi2896925" "vmi2897381" "vmi2897382" "vmi2897383" "vmi2897384"
)

SSH_KEY="$HOME/.ssh/id_rsa"
TIMEOUT=5

# Output file
REPORT="/Users/macbook/Desktop/pbc-quick-health-$(date +%Y%m%d-%H%M%S).txt"

# Stats
TOTAL_VMS=0
REACHABLE_VMS=0
TOTAL_PROCESSES=0

# Quick SSH exec
ssh_exec() {
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=$TIMEOUT root@${1}.contaboserver.net "$2" 2>/dev/null
}

{
    echo "ËTRID PBC NETWORK - QUICK HEALTH CHECK"
    echo "Date: $(date)"
    echo "================================================================"
    echo ""
} > "$REPORT"

for vm in "${VMS[@]}"; do
    TOTAL_VMS=$((TOTAL_VMS + 1))

    printf "%-15s " "$vm"

    # Test reachability
    if ! ssh_exec "$vm" "echo 'ok'" >/dev/null 2>&1; then
        echo -e "${RED}UNREACHABLE${NC}"
        echo "$vm: UNREACHABLE" >> "$REPORT"
        continue
    fi

    REACHABLE_VMS=$((REACHABLE_VMS + 1))

    # Count running PBC processes
    proc_count=$(ssh_exec "$vm" "pgrep -f 'pbc' | wc -l" 2>/dev/null | tr -d ' ')

    # Validate it's a number
    if ! [[ "$proc_count" =~ ^[0-9]+$ ]]; then
        proc_count=0
    fi

    TOTAL_PROCESSES=$((TOTAL_PROCESSES + proc_count))

    if [ "$proc_count" -gt 0 ]; then
        echo -e "${GREEN}OK${NC} - ${CYAN}${proc_count} collators${NC}"
        echo "$vm: OK - $proc_count collators running" >> "$REPORT"

        # List which PBCs are running
        chains=$(ssh_exec "$vm" "ps aux | grep 'pbc' | grep -v grep" 2>/dev/null | awk '{for(i=1;i<=NF;i++){if($i~/-pbc/){print $i}}}' | sort -u | tr '\n' ' ')

        if [ ! -z "$chains" ]; then
            echo "  Chains: $chains" >> "$REPORT"
        fi
    else
        echo -e "${YELLOW}OK${NC} - ${YELLOW}0 collators${NC}"
        echo "$vm: OK - 0 collators running" >> "$REPORT"
    fi
done

echo ""
echo -e "${BLUE}================================================================${NC}"
echo -e "${BLUE}SUMMARY${NC}"
echo -e "${BLUE}================================================================${NC}"
echo -e "Total VMs: ${TOTAL_VMS}"
echo -e "Reachable: ${GREEN}${REACHABLE_VMS}${NC}"
echo -e "Unreachable: ${RED}$((TOTAL_VMS - REACHABLE_VMS))${NC}"
echo -e "Total Running Collators: ${CYAN}${TOTAL_PROCESSES}${NC}"

{
    echo ""
    echo "================================================================"
    echo "SUMMARY"
    echo "================================================================"
    echo "Total VMs: ${TOTAL_VMS}"
    echo "Reachable: ${REACHABLE_VMS}"
    echo "Unreachable: $((TOTAL_VMS - REACHABLE_VMS))"
    echo "Total Running Collators: ${TOTAL_PROCESSES}"
} >> "$REPORT"

echo ""
echo -e "${GREEN}Report saved to: $REPORT${NC}"
echo ""

# Now do RPC checks for chains that are running
echo -e "${BLUE}================================================================${NC}"
echo -e "${BLUE}RPC HEALTH CHECK (Sample)${NC}"
echo -e "${BLUE}================================================================${NC}"
echo ""

{
    echo ""
    echo "================================================================"
    echo "RPC HEALTH CHECK"
    echo "================================================================"
    echo ""
} >> "$REPORT"

# Sample 3 VMs for RPC check
SAMPLE_VMS=("vmi2896907" "vmi2896915" "vmi2896925")

for vm in "${SAMPLE_VMS[@]}"; do
    echo -e "${YELLOW}Checking RPC on ${vm}...${NC}"
    echo "VM: $vm" >> "$REPORT"

    # Find which ports have PBC processes
    ports=$(ssh_exec "$vm" "netstat -tlnp 2>/dev/null | grep pbc | awk '{print \$4}' | awk -F: '{print \$NF}' | sort -u" 2>/dev/null)

    if [ -z "$ports" ]; then
        echo -e "  ${YELLOW}No PBC ports found${NC}"
        echo "  No PBC ports listening" >> "$REPORT"
        continue
    fi

    for port in $ports; do
        # Quick health check
        result=$(ssh_exec "$vm" "curl -s -m 2 -X POST -H 'Content-Type: application/json' --data '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_health\"}' http://127.0.0.1:${port}" 2>/dev/null)

        if echo "$result" | grep -q "result"; then
            peers=$(echo "$result" | grep -o '"peers":[0-9]*' | grep -o '[0-9]*')
            echo -e "  Port ${port}: ${GREEN}OK${NC} - ${peers} peers"
            echo "  Port $port: OK - $peers peers" >> "$REPORT"
        else
            echo -e "  Port ${port}: ${RED}NO RESPONSE${NC}"
            echo "  Port $port: NO RESPONSE" >> "$REPORT"
        fi
    done

    echo "" >> "$REPORT"
done

echo ""
echo -e "${GREEN}Full health check complete!${NC}"
echo -e "${GREEN}Report: $REPORT${NC}"
