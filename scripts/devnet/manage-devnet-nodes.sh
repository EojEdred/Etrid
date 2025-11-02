#!/usr/bin/env bash
# ËTRID DEVNET Quick Node Management
# Provides quick operations on all 16 nodes

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

SSH_KEY="${SSH_KEY:-$HOME/.ssh/etrid_vm1}"

# Color helpers
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }

# Node array
declare -a NODES=(
    "multichain-dev01@68.219.230.63"
    "compiler-dev01@98.71.91.84"
    "compiler-dev01@4.180.59.25"
    "consensus-dev01@20.224.104.239"
    "multichain-dev01@98.71.219.106"
    "runtime-dev01@108.142.205.177"
    "runtime-dev01@4.180.238.67"
    "audit-dev01@51.142.203.160"
    "flarenode15@172.166.164.19"
    "flarenode16@172.166.187.180"
    "flarenode17@172.166.210.244"
    "oracle-dev01@172.167.8.217"
    "flarenode18@4.251.115.186"
    "flarenode19@52.143.191.232"
    "flarenode20@4.211.206.210"
    "flarenode21@4.178.181.122"
)

# Operations
stop_all() {
    log_info "Stopping all nodes..."
    local success=0
    local failed=0

    for i in "${!NODES[@]}"; do
        ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${NODES[$i]}" \
            "sudo systemctl stop flarechain-devnet 2>/dev/null && echo ok" &>/dev/null && \
            { log_success "Node $i stopped"; ((success++)); } || \
            { log_warning "Node $i stop failed"; ((failed++)); }
    done

    echo "Stopped: $success, Failed: $failed"
}

start_all() {
    log_info "Starting all nodes..."
    local success=0
    local failed=0

    for i in "${!NODES[@]}"; do
        ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${NODES[$i]}" \
            "sudo systemctl start flarechain-devnet 2>/dev/null && echo ok" &>/dev/null && \
            { log_success "Node $i started"; ((success++)); } || \
            { log_warning "Node $i start failed"; ((failed++)); }
    done

    echo "Started: $success, Failed: $failed"
}

restart_all() {
    log_info "Restarting all nodes..."
    stop_all
    sleep 5
    start_all
}

restart_all_parallel() {
    log_info "Restarting all nodes (parallel)..."

    for i in "${!NODES[@]}"; do
        ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${NODES[$i]}" \
            "sudo systemctl restart flarechain-devnet 2>/dev/null" &
    done

    wait
    log_success "All restart commands sent"
}

status_all() {
    log_info "Checking status of all nodes..."

    for i in "${!NODES[@]}"; do
        local status=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${NODES[$i]}" \
            "systemctl is-active flarechain-devnet 2>/dev/null" 2>/dev/null || echo "unknown")

        if [ "$status" = "active" ]; then
            log_success "Node $i: RUNNING"
        elif [ "$status" = "inactive" ]; then
            log_warning "Node $i: STOPPED"
        else
            log_error "Node $i: UNREACHABLE"
        fi
    done
}

purge_all() {
    log_warning "WARNING: This will delete all chain data from all nodes!"
    read -p "Type 'yes' to confirm: " confirm

    if [ "$confirm" != "yes" ]; then
        log_info "Cancelled"
        return
    fi

    log_info "Purging all nodes..."

    for i in "${!NODES[@]}"; do
        ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${NODES[$i]}" bash << 'EOF' &
sudo systemctl stop flarechain-devnet 2>/dev/null
sleep 2
sudo rm -rf /data/flarechain/*
sudo systemctl start flarechain-devnet 2>/dev/null
echo "Node purged and restarted"
EOF
    done

    wait
    log_success "All nodes purged and restarted"
}

check_disk() {
    log_info "Checking disk usage on all nodes..."

    for i in "${!NODES[@]}"; do
        local usage=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${NODES[$i]}" \
            "du -sh /data/flarechain 2>/dev/null | awk '{print \$1}'" 2>/dev/null || echo "?")
        echo "Node $i: $usage"
    done
}

tail_logs() {
    if [ -z "$1" ] || [ "$1" -lt 0 ] || [ "$1" -gt 15 ]; then
        log_error "Invalid node number"
        return 1
    fi

    log_info "Tailing logs for node $1..."
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "${NODES[$1]}" \
        "journalctl -u flarechain-devnet -f" 2>/dev/null
}

ssh_node() {
    if [ -z "$1" ] || [ "$1" -lt 0 ] || [ "$1" -gt 15 ]; then
        log_error "Invalid node number"
        return 1
    fi

    log_info "SSH to node $1..."
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "${NODES[$1]}"
}

show_help() {
    cat << 'EOF'
ËTRID DEVNET Node Management

Usage: ./manage-devnet-nodes.sh [COMMAND]

Commands:
  start           Start all nodes
  stop            Stop all nodes
  restart         Restart all nodes (sequential)
  restart-fast    Restart all nodes (parallel)
  status          Check status of all nodes
  purge           Purge chain data and restart all nodes
  disk            Check disk usage on all nodes
  logs <node>     Tail logs for a specific node (0-15)
  ssh <node>      SSH to a specific node (0-15)
  help            Show this help message

Examples:
  ./manage-devnet-nodes.sh start              # Start all nodes
  ./manage-devnet-nodes.sh status             # Check status
  ./manage-devnet-nodes.sh logs 0             # View Node 0 logs
  ./manage-devnet-nodes.sh ssh 3              # SSH to Node 3
  ./manage-devnet-nodes.sh restart-fast       # Parallel restart

Node Mapping:
  0  = multichain-dev01 @ 68.219.230.63
  1  = compiler-dev01 @ 98.71.91.84
  2  = compiler-dev01 @ 4.180.59.25
  3  = consensus-dev01 @ 20.224.104.239
  4  = multichain-dev01 @ 98.71.219.106
  5  = runtime-dev01 @ 108.142.205.177
  6  = runtime-dev01 @ 4.180.238.67
  7  = audit-dev01 @ 51.142.203.160
  8  = flarenode15 @ 172.166.164.19
  9  = flarenode16 @ 172.166.187.180
  10 = flarenode17 @ 172.166.210.244
  11 = oracle-dev01 @ 172.167.8.217
  12 = flarenode18 @ 4.251.115.186
  13 = flarenode19 @ 52.143.191.232
  14 = flarenode20 @ 4.211.206.210
  15 = flarenode21 @ 4.178.181.122

EOF
}

main() {
    local cmd="${1:-help}"
    local arg="$2"

    case "$cmd" in
        start)
            start_all
            ;;
        stop)
            stop_all
            ;;
        restart)
            restart_all
            ;;
        restart-fast)
            restart_all_parallel
            ;;
        status)
            status_all
            ;;
        purge)
            purge_all
            ;;
        disk)
            check_disk
            ;;
        logs)
            tail_logs "$arg"
            ;;
        ssh)
            ssh_node "$arg"
            ;;
        help|-h|--help)
            show_help
            ;;
        *)
            log_error "Unknown command: $cmd"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
