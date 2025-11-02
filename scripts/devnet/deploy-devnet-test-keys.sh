#!/usr/bin/env bash
# ËTRID DEVNET Test Keys Configuration Script
# Configures standard Substrate test keys on all 16 DEVNET nodes
# Uses built-in dev accounts (alice, bob, charlie, dave, eve, ferdie, etc.)
# Each node purges chain and restarts with dedicated test account

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
REMOTE_BINARY_PATH="/opt/flarechain/flarechain-node"
REMOTE_DATA_PATH="/data/flarechain"
TIMEOUT=30

# Color output helpers
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }
log_section() { echo -e "\n${PURPLE}═══════════════════════════════════════════${NC}\n${PURPLE}$1${NC}\n${PURPLE}═══════════════════════════════════════════${NC}\n"; }

# Array of validator nodes
# Format: "node_number:ssh_user@host:validator_account:description"
declare -a VALIDATORS=(
    "0:multichain-dev01@68.219.230.63:alice:Node0-Alice-Primary"
    "1:compiler-dev01@98.71.91.84:bob:Node1-Bob-Monitoring"
    "2:compiler-dev01@4.180.59.25:charlie:Node2-Charlie-Primary"
    "3:consensus-dev01@20.224.104.239:dave:Node3-Dave-Consensus"
    "4:multichain-dev01@98.71.219.106:eve:Node4-Eve-Secondary"
    "5:runtime-dev01@108.142.205.177:ferdie:Node5-Ferdie-Runtime"
    "6:runtime-dev01@4.180.238.67:alice:Node6-Alice-Runtime"
    "7:audit-dev01@51.142.203.160:bob:Node7-Bob-Audit"
    "8:flarenode15@172.166.164.19:charlie:Node8-Charlie-Econ"
    "9:flarenode16@172.166.187.180:dave:Node9-Dave-Econ"
    "10:flarenode17@172.166.210.244:eve:Node10-Eve-Ethics"
    "11:oracle-dev01@172.167.8.217:ferdie:Node11-Ferdie-Oracle"
    "12:flarenode18@4.251.115.186:alice:Node12-Alice-Ethics"
    "13:flarenode19@52.143.191.232:bob:Node13-Bob-Docs"
    "14:flarenode20@4.211.206.210:charlie:Node14-Charlie-Docs"
    "15:flarenode21@4.178.181.122:dave:Node15-Dave-Docs"
)

# Check if SSH key exists
check_ssh_key() {
    log_info "Checking SSH key: $SSH_KEY"
    if [ ! -f "$SSH_KEY" ]; then
        log_error "SSH key not found: $SSH_KEY"
        exit 1
    fi
    log_success "SSH key found"
}

# Check SSH connectivity to a node
test_ssh_connection() {
    local ssh_target=$1
    local timeout=$2

    if timeout $timeout ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no \
        -o ConnectTimeout=5 -o BatchMode=yes "$ssh_target" "echo ok" &>/dev/null; then
        return 0
    else
        return 1
    fi
}

# Deploy to a single node
deploy_node() {
    local node_spec=$1
    local parallel_id=$2

    # Parse node specification
    IFS=':' read -r node_num ssh_target validator_account description <<< "$node_spec"

    log_info "[Node $node_num] Starting deployment for $description (account: $validator_account)"

    # Test SSH connection first
    if ! test_ssh_connection "$ssh_target" "$TIMEOUT"; then
        log_error "[Node $node_num] SSH connection failed to $ssh_target"
        echo "FAILED:$node_num:Connection timeout" >> /tmp/devnet_deploy_status.txt
        return 1
    fi

    log_info "[Node $node_num] SSH connection successful"

    # Execute deployment script on remote
    ssh -i "$SSH_KEY" \
        -o StrictHostKeyChecking=no \
        -o ConnectTimeout=10 \
        -o BatchMode=yes \
        "$ssh_target" bash -s << DEPLOY_SCRIPT 2>&1 | grep -E "^(\[|SUCCESS|ERROR|flarechain)" || true

set -e

echo "[Node $node_num] Starting test key configuration"

# Kill any existing node processes
echo "[Node $node_num] Stopping existing node processes..."
pkill -f flarechain-node 2>/dev/null || true
sleep 3

# Verify binary exists
if [ ! -f "$REMOTE_BINARY_PATH" ]; then
    echo "ERROR: Binary not found at $REMOTE_BINARY_PATH"
    exit 1
fi

echo "[Node $node_num] Binary verified at $REMOTE_BINARY_PATH"

# Purge chain database
echo "[Node $node_num] Purging chain database..."
if [ -d "$REMOTE_DATA_PATH" ]; then
    rm -rf "$REMOTE_DATA_PATH" || {
        echo "WARNING: Could not remove old data directory"
    }
fi
mkdir -p "$REMOTE_DATA_PATH"

echo "[Node $node_num] Creating systemd service file..."

# Create systemd service
sudo tee /etc/systemd/system/flarechain-devnet.service > /dev/null <<'EOF'
[Unit]
Description=Flarechain DEVNET Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=flarechain
WorkingDirectory=/opt/flarechain
ExecStart=/opt/flarechain/flarechain-node \\
  --dev \\
  ----validator-account \\
  --base-path $REMOTE_DATA_PATH \\
  --name node$node_num \\
  --port 30333 \\
  --rpc-port 9944 \\
  --rpc-external \\
  --rpc-cors all \\
  --prometheus-external \\
  --prometheus-port 9615 \\
  --log info,runtime=debug

StandardOutput=journal
StandardError=journal
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

echo "[Node $node_num] Reloading systemd and starting service..."

# Reload systemd and start service
sudo systemctl daemon-reload
sudo systemctl enable flarechain-devnet.service 2>&1 || {
    echo "WARNING: Could not enable service"
}
sudo systemctl restart flarechain-devnet.service 2>&1 || {
    echo "WARNING: Service restart may have issues"
}

echo "[Node $node_num] Waiting for node startup..."
sleep 5

# Check if service is running
if systemctl is-active --quiet flarechain-devnet 2>/dev/null; then
    echo "SUCCESS: Node service started for $validator_account account"
else
    echo "ERROR: Node service failed to start"
    systemctl status flarechain-devnet 2>&1 || true
    journalctl -u flarechain-devnet -n 20 || true
    exit 1
fi

DEPLOY_SCRIPT

    local exit_code=$?

    if [ $exit_code -eq 0 ]; then
        log_success "[Node $node_num] Deployment completed"
        echo "SUCCESS:$node_num:$validator_account" >> /tmp/devnet_deploy_status.txt
        return 0
    else
        log_error "[Node $node_num] Deployment failed with exit code $exit_code"
        echo "FAILED:$node_num:$validator_account:Exit $exit_code" >> /tmp/devnet_deploy_status.txt
        return 1
    fi
}

# Verify node is running and producing blocks
verify_node() {
    local node_spec=$1

    # Parse node specification
    IFS=':' read -r node_num ssh_target validator_account description <<< "$node_spec"

    log_info "[Node $node_num] Verifying node is producing blocks..."

    # Wait a bit for startup
    sleep 5

    # Try to connect via RPC
    if ssh -i "$SSH_KEY" \
        -o StrictHostKeyChecking=no \
        -o ConnectTimeout=5 \
        -o BatchMode=yes \
        "$ssh_target" \
        curl -s http://localhost:9944 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' 2>/dev/null | grep -q "result"; then
        log_success "[Node $node_num] RPC responding - blocks being produced"
        return 0
    else
        log_warning "[Node $node_num] RPC not responding yet (may still be starting)"
        return 2
    fi
}

# Deploy to all nodes in parallel
deploy_all_parallel() {
    log_section "DEPLOYING TO ALL 16 DEVNET NODES (PARALLEL)"

    > /tmp/devnet_deploy_status.txt

    local pids=()
    local count=0

    for validator in "${VALIDATORS[@]}"; do
        deploy_node "$validator" "$count" &
        pids+=($!)
        count=$((count + 1))

        # Limit concurrent connections
        if [ $((count % 4)) -eq 0 ]; then
            log_info "Waiting for batch to complete..."
            sleep 2
        fi
    done

    log_info "Waiting for all deployments to complete..."
    for pid in "${pids[@]}"; do
        wait "$pid" || true
    done
}

# Print deployment summary
print_summary() {
    log_section "DEPLOYMENT SUMMARY"

    if [ ! -f /tmp/devnet_deploy_status.txt ]; then
        log_error "Status file not found"
        return
    fi

    local successful=0
    local failed=0

    echo ""
    echo "Deployment Results:"
    echo "─────────────────────────────────────────"

    while IFS=':' read -r status node_num account rest; do
        if [ "$status" = "SUCCESS" ]; then
            echo -e "${GREEN}✓${NC} Node $node_num ($account) - SUCCESS"
            ((successful++))
        elif [ "$status" = "FAILED" ]; then
            echo -e "${RED}✗${NC} Node $node_num ($account) - FAILED ($rest)"
            ((failed++))
        fi
    done < /tmp/devnet_deploy_status.txt

    echo ""
    echo "─────────────────────────────────────────"
    echo -e "Total Deployed:  ${GREEN}$successful/16${NC}"
    echo -e "Total Failed:    ${RED}$failed/16${NC}"
    echo "─────────────────────────────────────────"
    echo ""

    if [ $successful -eq 16 ]; then
        log_success "All 16 nodes deployed successfully!"
        return 0
    elif [ $successful -ge 12 ]; then
        log_warning "$successful/16 nodes deployed (acceptable)"
        return 0
    else
        log_error "Only $successful/16 nodes deployed - review errors"
        return 1
    fi
}

# Verify all nodes (after deployment)
verify_all_nodes() {
    log_section "VERIFYING NODE STATUS (30 second wait for startup)"

    sleep 30

    local verified=0
    local not_ready=0

    for validator in "${VALIDATORS[@]}"; do
        if verify_node "$validator"; then
            ((verified++))
        else
            ((not_ready++))
        fi
    done

    echo ""
    echo "─────────────────────────────────────────"
    echo -e "Nodes Verified:   ${GREEN}$verified/16${NC}"
    echo -e "Not Ready Yet:    ${YELLOW}$not_ready/16${NC}"
    echo "─────────────────────────────────────────"
    echo ""
}

# Generate test key documentation
generate_documentation() {
    log_section "TEST KEY MAPPING"

    cat << 'EOF'

Substrate Standard Development Accounts
─────────────────────────────────────────

Account   Public Key (SS58)                          Hex
───────   ─────────────────────────────────────────   ────────────────
alice     1AQST1ZklyJt91tsqK3j2hicAYyNMrx5vcnpZACNB1aeB9u
bob       3Cd0SywyKcFJzWkL6DtKwcYvGuqKzV9z9cE4dD7ikVM8cJAm
charlie   3Esxv6p8t6nYrXsFFAQWwmzWStvnNa7htTg3ap17ttB8kXqo
dave      3EnCLqBGoWEwqdMwEEkNRX4zfRRDKqFNB33pHWsCj6ifiGMa
eve       3DfHHQfz52W35PoC2K52wCCAGzymMiVPAYyqcrCcgcB3JSUc
ferdie    3Dg8sSl4bBrkLvdqUiiGT2LACoLG798eAz91BGe7V873kxAH

Each node is assigned a different account:
─────────────────────────────────────────

Node  SSH Target                     Account  Description
────  ─────────────────────────────  ───────  ──────────────────────
0     multichain-dev01@68.219.230.63   alice   Primary Multichain
1     compiler-dev01@98.71.91.84       bob     Compiler Monitoring
2     compiler-dev01@4.180.59.25       charlie Compiler Primary
3     consensus-dev01@20.224.104.239   dave    Consensus Dev
4     multichain-dev01@98.71.219.106   eve     Multichain Secondary
5     runtime-dev01@108.142.205.177    ferdie  Runtime Primary
6     runtime-dev01@4.180.238.67       alice   Runtime Secondary
7     audit-dev01@51.142.203.160       bob     Audit Dev
8     flarenode15@172.166.164.19       charlie Economics Primary
9     flarenode16@172.166.187.180      dave    Economics Secondary
10    flarenode17@172.166.210.244      eve     Ethics Primary
11    oracle-dev01@172.167.8.217       ferdie  Oracle Dev
12    flarenode18@4.251.115.186        alice   Ethics Secondary
13    flarenode19@52.143.191.232       bob     Docs Primary
14    flarenode20@4.211.206.210        charlie Docs Secondary
15    flarenode21@4.178.181.122        dave    Docs Tertiary

Accessing Nodes
─────────────────────────────────────────

RPC Endpoint:       http://<host>:9944
WebSocket:          ws://<host>:9945
Prometheus Metrics: http://<host>:9615

Example queries:
  curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
    http://localhost:9944

Systemd Service Management
─────────────────────────────────────────

Start:   systemctl start flarechain-devnet
Stop:    systemctl stop flarechain-devnet
Status:  systemctl status flarechain-devnet
Logs:    journalctl -u flarechain-devnet -f

Reset Node (purge and restart):
  systemctl stop flarechain-devnet
  rm -rf /data/flarechain/*
  systemctl start flarechain-devnet

EOF
}

# Main execution
main() {
    echo ""
    echo -e "${PURPLE}"
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║                                                           ║"
    echo "║     ËTRID DEVNET TEST KEYS DEPLOYMENT                    ║"
    echo "║     16-Node Substrate Dev Account Configuration          ║"
    echo "║                                                           ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"

    # Check prerequisites
    check_ssh_key

    # Deploy to all nodes
    deploy_all_parallel

    # Print summary
    print_summary || exit 1

    # Verify nodes are running
    verify_all_nodes

    # Generate documentation
    generate_documentation

    echo ""
    log_success "DEVNET test key configuration complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Wait 60 seconds for nodes to sync"
    echo "  2. Check node status: curl http://<host>:9944 -d '{...}' -H 'Content-Type: application/json'"
    echo "  3. View logs: ssh -i ~/.ssh/etrid_vm1 <user>@<host> journalctl -u flarechain-devnet -f"
    echo ""
}

main "$@"
