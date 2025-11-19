#!/bin/bash
# ËTRID Checkpoint BFT Finality - Real-time Monitoring Dashboard
# Usage: ./monitor-checkpoint-finality.sh [rpc-port]

RPC_PORT=${1:-9944}
PROMETHEUS_PORT=${2:-9615}
RPC_URL="http://localhost:$RPC_PORT"
METRICS_URL="http://localhost:$PROMETHEUS_PORT/metrics"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

clear

echo -e "${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║        ËTRID CHECKPOINT BFT FINALITY MONITOR                   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"

# Function to get metric value
get_metric() {
    curl -s "$METRICS_URL" 2>/dev/null | grep "^$1" | head -1 | awk '{print $2}' || echo "N/A"
}

# Function to get RPC data
get_rpc() {
    curl -s -X POST -H "Content-Type: application/json" \
        --data "{\"jsonrpc\":\"2.0\",\"method\":\"$1\",\"params\":$2,\"id\":1}" \
        "$RPC_URL" 2>/dev/null | jq -r '.result' 2>/dev/null || echo "N/A"
}

# Main monitoring loop
while true; do
    # Move cursor to top
    tput cup 3 0

    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

    # Get blockchain metrics
    BEST_BLOCK=$(get_rpc "chain_getHeader" "[]" | jq -r '.number' 2>/dev/null | xargs printf "%d" 2>/dev/null || echo "N/A")
    FINALIZED_BLOCK=$(get_rpc "chain_getFinalizedHead" "[]")
    PEER_COUNT=$(get_metric "substrate_sub_libp2p_peers_count")

    # Get checkpoint metrics
    CHECKPOINT_LAST=$(get_metric "checkpoint_last_finalized")
    CHECKPOINT_PENDING=$(get_metric "checkpoint_signatures_pending")
    CHECKPOINT_CERTS=$(get_metric "checkpoint_certificates_created")
    CHECKPOINT_QUORUM_TIME=$(get_metric "checkpoint_time_to_quorum")
    CHECKPOINT_LAG=$(get_metric "checkpoint_behind_best")
    STUCK_CHECKPOINTS=$(get_metric "stuck_checkpoints")

    # Calculate checkpoint number from best block
    if [[ "$BEST_BLOCK" != "N/A" ]]; then
        EXPECTED_CHECKPOINT=$((BEST_BLOCK / 16))
    else
        EXPECTED_CHECKPOINT="N/A"
    fi

    # Display dashboard
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  NETWORK STATUS${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "  Timestamp:           ${YELLOW}$TIMESTAMP${NC}"
    echo -e "  Best Block:          ${GREEN}#$BEST_BLOCK${NC}"
    echo -e "  Finalized Block:     ${GREEN}$FINALIZED_BLOCK${NC}"
    echo -e "  Peer Count:          ${CYAN}$PEER_COUNT${NC}"
    echo ""

    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  CHECKPOINT FINALITY${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "  Last Finalized:      ${GREEN}Checkpoint #$CHECKPOINT_LAST${NC}"
    echo -e "  Expected Checkpoint: ${YELLOW}#$EXPECTED_CHECKPOINT${NC}"

    # Calculate lag and show health status
    if [[ "$CHECKPOINT_LAST" != "N/A" && "$EXPECTED_CHECKPOINT" != "N/A" ]]; then
        CHECKPOINT_LAG_CALC=$((EXPECTED_CHECKPOINT - CHECKPOINT_LAST))

        if [ "$CHECKPOINT_LAG_CALC" -le 1 ]; then
            STATUS="${GREEN}✓ HEALTHY${NC}"
        elif [ "$CHECKPOINT_LAG_CALC" -le 3 ]; then
            STATUS="${YELLOW}⚠ DEGRADED${NC}"
        else
            STATUS="${RED}✗ CRITICAL${NC}"
        fi

        echo -e "  Checkpoint Lag:      ${YELLOW}$CHECKPOINT_LAG_CALC checkpoints${NC}"
        echo -e "  Health Status:       $STATUS"
    else
        echo -e "  Checkpoint Lag:      ${YELLOW}N/A${NC}"
        echo -e "  Health Status:       ${YELLOW}⏳ INITIALIZING${NC}"
    fi

    echo ""
    echo -e "  Signatures Pending:  ${CYAN}$CHECKPOINT_PENDING${NC}"
    echo -e "  Certificates Created: ${GREEN}$CHECKPOINT_CERTS${NC}"
    echo -e "  Avg Quorum Time:     ${CYAN}${CHECKPOINT_QUORUM_TIME}s${NC}"

    # Stuck checkpoint warning
    if [[ "$STUCK_CHECKPOINTS" != "N/A" && "$STUCK_CHECKPOINTS" != "0" ]]; then
        echo -e "  ${RED}⚠ STUCK CHECKPOINTS:  $STUCK_CHECKPOINTS${NC}"
    fi

    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  EXPECTED BEHAVIOR${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "  Checkpoint Interval: ${CYAN}16 blocks (~96 seconds)${NC}"
    echo -e "  Quorum Requirement:  ${CYAN}15/21 validators${NC}"
    echo -e "  Target Quorum Time:  ${CYAN}<15 seconds${NC}"
    echo -e "  Finality Lag:        ${CYAN}<120 seconds (2 checkpoints)${NC}"
    echo ""

    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "  ${YELLOW}Press Ctrl+C to exit${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"

    # Refresh every 5 seconds
    sleep 5
done
