#!/usr/bin/env bash
# ËTRID DEVNET Node Verification Script
# Verifies all 16 nodes are running and producing blocks

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
SSH_KEY="${SSH_KEY:-$HOME/.ssh/etrid_vm1}"

# Color output helpers
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }
log_section() { echo -e "\n${PURPLE}═══════════════════════════════════════════${NC}\n${PURPLE}$1${NC}\n${PURPLE}═══════════════════════════════════════════${NC}\n"; }

# Array of validator nodes
declare -a VALIDATORS=(
    "0:multichain-dev01@68.219.230.63:alice:Node0"
    "1:compiler-dev01@98.71.91.84:bob:Node1"
    "2:compiler-dev01@4.180.59.25:charlie:Node2"
    "3:consensus-dev01@20.224.104.239:dave:Node3"
    "4:multichain-dev01@98.71.219.106:eve:Node4"
    "5:runtime-dev01@108.142.205.177:ferdie:Node5"
    "6:runtime-dev01@4.180.238.67:alice:Node6"
    "7:audit-dev01@51.142.203.160:bob:Node7"
    "8:flarenode15@172.166.164.19:charlie:Node8"
    "9:flarenode16@172.166.187.180:dave:Node9"
    "10:flarenode17@172.166.210.244:eve:Node10"
    "11:oracle-dev01@172.167.8.217:ferdie:Node11"
    "12:flarenode18@4.251.115.186:alice:Node12"
    "13:flarenode19@52.143.191.232:bob:Node13"
    "14:flarenode20@4.211.206.210:charlie:Node14"
    "15:flarenode21@4.178.181.122:dave:Node15"
)

# Check node status
check_node() {
    local node_spec=$1

    # Parse node specification
    IFS=':' read -r node_num ssh_target validator_account description <<< "$node_spec"

    # Get status
    status_output=$(ssh -i "$SSH_KEY" \
        -o StrictHostKeyChecking=no \
        -o ConnectTimeout=5 \
        -o BatchMode=yes \
        "$ssh_target" \
        "systemctl is-active flarechain-devnet 2>/dev/null || echo 'inactive'" 2>/dev/null || echo "unreachable")

    # Try RPC query
    rpc_output=$(ssh -i "$SSH_KEY" \
        -o StrictHostKeyChecking=no \
        -o ConnectTimeout=5 \
        -o BatchMode=yes \
        "$ssh_target" \
        curl -s http://localhost:9944 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' 2>/dev/null || echo "{}")

    # Parse block height
    block_hex=$(echo "$rpc_output" | grep -o '"number":"0x[^"]*"' | sed 's/"number":"0x//' | sed 's/"//' | head -1)

    if [ -n "$block_hex" ] && [ "$block_hex" != "" ]; then
        block_num=$((16#$block_hex))
    else
        block_num="?"
    fi

    # Check service
    if [ "$status_output" = "active" ]; then
        if [ "$block_num" != "?" ]; then
            log_success "[Node $node_num] ($validator_account) - RUNNING - Block: $block_num"
            echo "$node_num:running:$block_num" >> /tmp/verify_status.txt
        else
            log_warning "[Node $node_num] ($validator_account) - RUNNING but RPC not responding"
            echo "$node_num:running:?" >> /tmp/verify_status.txt
        fi
    elif [ "$status_output" = "inactive" ]; then
        log_error "[Node $node_num] ($validator_account) - STOPPED"
        echo "$node_num:stopped" >> /tmp/verify_status.txt
    else
        log_error "[Node $node_num] ($validator_account) - UNREACHABLE ($status_output)"
        echo "$node_num:unreachable" >> /tmp/verify_status.txt
    fi
}

# View logs for a specific node
view_logs() {
    local node_num=$1

    if [ -z "$node_num" ] || [ "$node_num" -lt 0 ] || [ "$node_num" -gt 15 ]; then
        log_error "Invalid node number. Use 0-15"
        return 1
    fi

    local ssh_target="${VALIDATORS[$node_num]#*:}"
    ssh_target="${ssh_target%%:*}"

    log_section "LOGS FOR NODE $node_num"

    ssh -i "$SSH_KEY" \
        -o StrictHostKeyChecking=no \
        "$ssh_target" \
        journalctl -u flarechain-devnet -n 50 --no-pager 2>/dev/null || {
        log_error "Could not retrieve logs for node $node_num"
        return 1
    }
}

# Restart a specific node
restart_node() {
    local node_num=$1

    if [ -z "$node_num" ] || [ "$node_num" -lt 0 ] || [ "$node_num" -gt 15 ]; then
        log_error "Invalid node number. Use 0-15"
        return 1
    fi

    local ssh_target="${VALIDATORS[$node_num]#*:}"
    ssh_target="${ssh_target%%:*}"

    log_info "Restarting node $node_num ($ssh_target)..."

    ssh -i "$SSH_KEY" \
        -o StrictHostKeyChecking=no \
        "$ssh_target" \
        "sudo systemctl restart flarechain-devnet" 2>/dev/null || {
        log_error "Could not restart node $node_num"
        return 1
    }

    log_success "Node $node_num restart initiated"
}

# Purge and reset a node
reset_node() {
    local node_num=$1

    if [ -z "$node_num" ] || [ "$node_num" -lt 0 ] || [ "$node_num" -gt 15 ]; then
        log_error "Invalid node number. Use 0-15"
        return 1
    fi

    local ssh_target="${VALIDATORS[$node_num]#*:}"
    ssh_target="${ssh_target%%:*}"

    log_info "Resetting node $node_num ($ssh_target)..."

    ssh -i "$SSH_KEY" \
        -o StrictHostKeyChecking=no \
        "$ssh_target" \
        bash << 'EOF' 2>/dev/null || {
        log_error "Could not reset node $node_num"
        return 1
    }
set -e
echo "Stopping service..."
sudo systemctl stop flarechain-devnet
sleep 3
echo "Purging data..."
sudo rm -rf /data/flarechain/*
echo "Starting service..."
sudo systemctl start flarechain-devnet
sleep 3
echo "Reset complete"
EOF

    log_success "Node $node_num reset initiated"
}

# Print status summary
print_summary() {
    log_section "DEVNET STATUS SUMMARY"

    if [ ! -f /tmp/verify_status.txt ]; then
        log_error "Status file not found"
        return 1
    fi

    local running=0
    local stopped=0
    local unreachable=0
    local block_heights=()

    echo ""
    echo "Node Status Overview:"
    echo "─────────────────────────────────────────"

    while IFS=':' read -r node_num status block; do
        if [ "$status" = "running" ]; then
            if [ "$block" != "?" ]; then
                echo -e "${GREEN}✓${NC} Node $node_num - RUNNING (Block $block)"
                block_heights+=("$block")
            else
                echo -e "${YELLOW}⚠${NC} Node $node_num - RUNNING (RPC timeout)"
            fi
            ((running++))
        elif [ "$status" = "stopped" ]; then
            echo -e "${RED}✗${NC} Node $node_num - STOPPED"
            ((stopped++))
        else
            echo -e "${RED}✗${NC} Node $node_num - UNREACHABLE"
            ((unreachable++))
        fi
    done < /tmp/verify_status.txt

    echo ""
    echo "─────────────────────────────────────────"
    echo -e "Running:     ${GREEN}$running/16${NC}"
    echo -e "Stopped:     ${RED}$stopped/16${NC}"
    echo -e "Unreachable: ${RED}$unreachable/16${NC}"
    echo "─────────────────────────────────────────"
    echo ""

    # Check consensus (all same block height)
    if [ ${#block_heights[@]} -gt 1 ]; then
        min_block="${block_heights[0]}"
        max_block="${block_heights[0]}"

        for block in "${block_heights[@]}"; do
            if [ "$block" -lt "$min_block" ]; then
                min_block="$block"
            fi
            if [ "$block" -gt "$max_block" ]; then
                max_block="$block"
            fi
        done

        block_diff=$((max_block - min_block))
        if [ $block_diff -le 5 ]; then
            log_success "Consensus healthy - block height variance: $block_diff blocks"
        else
            log_warning "Consensus issue - block height variance: $block_diff blocks (min: $min_block, max: $max_block)"
        fi
    fi

    echo ""

    if [ $running -eq 16 ]; then
        log_success "All 16 nodes running!"
        return 0
    elif [ $running -ge 12 ]; then
        log_warning "$running/16 nodes running (acceptable)"
        return 0
    else
        log_error "Only $running/16 nodes running"
        return 1
    fi
}

# Show help
show_help() {
    cat << 'EOF'
ËTRID DEVNET Node Verification Script

Usage: ./verify-devnet-nodes.sh [COMMAND] [OPTIONS]

Commands:
  status              Check status of all nodes (default)
  logs <node_num>     View logs for a specific node (0-15)
  restart <node_num>  Restart a specific node (0-15)
  reset <node_num>    Purge data and restart a specific node (0-15)
  watch               Monitor status every 10 seconds (press Ctrl+C to exit)

Examples:
  ./verify-devnet-nodes.sh                 # Check all nodes
  ./verify-devnet-nodes.sh logs 0          # View Node 0 logs
  ./verify-devnet-nodes.sh restart 3       # Restart Node 3
  ./verify-devnet-nodes.sh reset 5         # Reset Node 5
  ./verify-devnet-nodes.sh watch           # Continuous monitoring

SSH Key Location:
  Default: ~/.ssh/etrid_vm1
  Override: SSH_KEY=/path/to/key ./verify-devnet-nodes.sh

Node Mapping:
  0  - multichain-dev01 @ 68.219.230.63 (alice)
  1  - compiler-dev01 @ 98.71.91.84 (bob)
  2  - compiler-dev01 @ 4.180.59.25 (charlie)
  3  - consensus-dev01 @ 20.224.104.239 (dave)
  4  - multichain-dev01 @ 98.71.219.106 (eve)
  5  - runtime-dev01 @ 108.142.205.177 (ferdie)
  6  - runtime-dev01 @ 4.180.238.67 (alice)
  7  - audit-dev01 @ 51.142.203.160 (bob)
  8  - flarenode15 @ 172.166.164.19 (charlie)
  9  - flarenode16 @ 172.166.187.180 (dave)
  10 - flarenode17 @ 172.166.210.244 (eve)
  11 - oracle-dev01 @ 172.167.8.217 (ferdie)
  12 - flarenode18 @ 4.251.115.186 (alice)
  13 - flarenode19 @ 52.143.191.232 (bob)
  14 - flarenode20 @ 4.211.206.210 (charlie)
  15 - flarenode21 @ 4.178.181.122 (dave)

EOF
}

# Main execution
main() {
    local command="${1:-status}"
    local arg="${2:-}"

    case "$command" in
        status)
            log_section "CHECKING DEVNET NODE STATUS"
            > /tmp/verify_status.txt

            for validator in "${VALIDATORS[@]}"; do
                check_node "$validator" &
            done
            wait

            print_summary
            ;;

        logs)
            if [ -z "$arg" ]; then
                log_error "Node number required: $0 logs <0-15>"
                exit 1
            fi
            view_logs "$arg"
            ;;

        restart)
            if [ -z "$arg" ]; then
                log_error "Node number required: $0 restart <0-15>"
                exit 1
            fi
            restart_node "$arg"
            sleep 3
            log_info "Waiting for restart..."
            sleep 5
            check_node "${VALIDATORS[$arg]}"
            ;;

        reset)
            if [ -z "$arg" ]; then
                log_error "Node number required: $0 reset <0-15>"
                exit 1
            fi
            reset_node "$arg"
            sleep 5
            log_info "Waiting for restart..."
            sleep 10
            check_node "${VALIDATORS[$arg]}"
            ;;

        watch)
            while true; do
                clear
                echo ""
                main status
                echo ""
                echo "Next update in 10 seconds (Ctrl+C to exit)..."
                sleep 10
            done
            ;;

        -h|--help|help)
            show_help
            ;;

        *)
            log_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
