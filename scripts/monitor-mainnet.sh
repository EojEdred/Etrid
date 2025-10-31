#!/bin/bash
# Monitor Mainnet Launch and Validator Status
# Provides real-time overview of all 21 validators

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Check validator list
if [ ! -f "validator-vms-numbered.txt" ]; then
    echo -e "${RED}❌ Error: validator-vms-numbered.txt not found${NC}"
    exit 1
fi

REFRESH_INTERVAL=5  # seconds between updates

clear
echo "╔════════════════════════════════════════════════════════════╗"
echo "║        Ëtrid FlareChain Mainnet Status Monitor            ║"
echo "║           Press Ctrl+C to exit                             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

while true; do
    # Move cursor to top
    tput home 2>/dev/null || clear

    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║        Ëtrid FlareChain Mainnet Status Monitor            ║"
    echo "║           Press Ctrl+C to exit                             ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "📊 Updated: $(date '+%Y-%m-%d %H:%M:%S UTC')"
    echo ""

    # Network overview
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${CYAN}NETWORK OVERVIEW${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    online_count=0
    total_peers=0
    max_block=0
    min_block=999999
    finalized_block=""

    # Collect data from all validators
    while read num vm; do
        # Check if validator is responding
        health=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=2 -o StrictHostKeyChecking=no "$vm" "curl -s -m 2 -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:9944" 2>/dev/null)

        if [ -n "$health" ]; then
            online_count=$((online_count+1))

            # Extract peer count
            peers=$(echo "$health" | grep -o '"peers":[0-9]*' | cut -d':' -f2)
            [ -n "$peers" ] && total_peers=$((total_peers + peers))

            # Get current block
            block_hex=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=2 -o StrictHostKeyChecking=no "$vm" "curl -s -m 2 -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}' http://localhost:9944" 2>/dev/null | grep -o '"number":"0x[^"]*' | cut -d'"' -f4)

            if [ -n "$block_hex" ]; then
                block_num=$((16#${block_hex#0x}))
                [ $block_num -gt $max_block ] && max_block=$block_num
                [ $block_num -lt $min_block ] && min_block=$block_num
            fi

            # Get finalized block (only from first validator)
            if [ -z "$finalized_block" ]; then
                finalized_hex=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=2 -o StrictHostKeyChecking=no "$vm" "curl -s -m 2 -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\", \"params\": [null]}' http://localhost:9944" 2>/dev/null | grep -o '"number":"0x[^"]*' | cut -d'"' -f4)
                [ -n "$finalized_hex" ] && finalized_block=$((16#${finalized_hex#0x}))
            fi
        fi
    done < validator-vms-numbered.txt

    # Calculate average peers
    if [ $online_count -gt 0 ]; then
        avg_peers=$((total_peers / online_count))
    else
        avg_peers=0
    fi

    # Display network stats
    total_validators=$(wc -l < validator-vms-numbered.txt)

    if [ $online_count -ge 15 ]; then
        status_color=$GREEN
        status_icon="✅"
        status_text="HEALTHY"
    elif [ $online_count -ge 10 ]; then
        status_color=$YELLOW
        status_icon="⚠️ "
        status_text="DEGRADED"
    else
        status_color=$RED
        status_icon="❌"
        status_text="CRITICAL"
    fi

    echo -e "Network Status:     ${status_color}${status_icon} ${status_text}${NC}"
    echo -e "Validators Online:  ${status_color}$online_count${NC} / $total_validators"
    echo -e "Consensus:          $([ $online_count -ge 15 ] && echo -e "${GREEN}Active (>2/3)${NC}" || echo -e "${RED}Waiting for validators${NC}")"
    echo ""
    echo -e "Current Block:      ${BLUE}#$max_block${NC}"
    echo -e "Finalized Block:    ${CYAN}#${finalized_block:-0}${NC}"
    echo -e "Avg Peers/Node:     $avg_peers"

    if [ $max_block -ne $min_block ]; then
        echo -e "Block Sync Delta:   ${YELLOW}$((max_block - min_block)) blocks${NC}"
    fi

    echo ""

    # Individual validator status
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${CYAN}VALIDATOR STATUS${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    printf "%-4s %-20s %-8s %-10s %-6s\n" "#" "Name" "Status" "Block" "Peers"
    echo "────────────────────────────────────────────────────────────"

    while read num vm; do
        VALIDATOR_NAME=$(jq -r ".validators[$((num-1))].name" mainnet-deployment-package/validator-keys-complete.json 2>/dev/null || echo "Validator-$num")

        # Truncate name if too long
        if [ ${#VALIDATOR_NAME} -gt 18 ]; then
            VALIDATOR_NAME="${VALIDATOR_NAME:0:15}..."
        fi

        # Check status
        health=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=2 -o StrictHostKeyChecking=no "$vm" "curl -s -m 2 -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:9944" 2>/dev/null)

        if [ -n "$health" ]; then
            peers=$(echo "$health" | grep -o '"peers":[0-9]*' | cut -d':' -f2)

            block_hex=$(ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=2 -o StrictHostKeyChecking=no "$vm" "curl -s -m 2 -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}' http://localhost:9944" 2>/dev/null | grep -o '"number":"0x[^"]*' | cut -d'"' -f4)

            if [ -n "$block_hex" ]; then
                block_num=$((16#${block_hex#0x}))
                printf "${GREEN}%-4s${NC} %-20s ${GREEN}%-8s${NC} ${BLUE}%-10s${NC} %-6s\n" \
                    "$num" "$VALIDATOR_NAME" "Online" "#$block_num" "$peers"
            else
                printf "${GREEN}%-4s${NC} %-20s ${GREEN}%-8s${NC} ${YELLOW}%-10s${NC} %-6s\n" \
                    "$num" "$VALIDATOR_NAME" "Online" "Syncing" "$peers"
            fi
        else
            printf "${RED}%-4s${NC} %-20s ${RED}%-8s${NC} ${RED}%-10s${NC} ${RED}%-6s${NC}\n" \
                "$num" "$VALIDATOR_NAME" "Offline" "N/A" "N/A"
        fi
    done < validator-vms-numbered.txt

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Refreshing in $REFRESH_INTERVAL seconds... (Ctrl+C to exit)"

    sleep $REFRESH_INTERVAL
done
