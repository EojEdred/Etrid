#!/usr/bin/env bash
set -e

# Ëtrid FlareChain - Update All Validators with Bootnode Configuration
# This script configures all accessible validators to connect to the 3 Azure bootstrap nodes

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SSH_KEY="$HOME/.ssh/gizzi-validator"
VALIDATOR_CONFIG="/Users/macbook/Desktop/etrid/config/validator-ips.json"

# Azure Bootstrap Nodes (Already Running)
BOOTNODE_VM1="/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm"
BOOTNODE_VM2="/ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb"
BOOTNODE_VM3="/ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6"

# Combined bootnode string
BOOTNODES="${BOOTNODE_VM1},${BOOTNODE_VM2},${BOOTNODE_VM3}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

# Validators to update (accessible validators, excluding the 3 Azure bootstrap nodes)
# Using simple arrays to avoid bash version compatibility issues
VALIDATOR_NAMES=(
    "Runtime-Dev"
    "Compiler-Dev"
    "Network-Dev"
    "SDK-Dev"
    "DevTools-Dev"
    "API-Dev"
    "Docs-Dev"
    "QA-Dev"
    "Perf-Dev"
    "Community-Dev"
    "Analytics-Dev"
    "Ethics-Dev"
    "FlareNode-16"
    "FlareNode-19"
    "FlareNode-20"
    "FlareNode-21"
)

VALIDATOR_TARGETS=(
    "ubuntu@20.224.104.239"
    "ubuntu@98.71.91.84"
    "ubuntu@20.169.114.25"
    "ubuntu@20.75.92.203"
    "ubuntu@20.55.31.30"
    "ubuntu@20.73.34.17"
    "ubuntu@20.109.102.30"
    "ubuntu@52.250.61.132"
    "ubuntu@20.218.66.251"
    "ubuntu@20.109.219.185"
    "ubuntu@20.83.208.17"
    "ubuntu@172.177.175.132"
    "ubuntu@20.84.231.225"
    "ubuntu@4.175.83.133"
    "ubuntu@52.184.47.99"
    "ubuntu@4.178.181.122"
)

# Check prerequisites
check_prereqs() {
    log_info "Checking prerequisites..."

    if [[ ! -f "$SSH_KEY" ]]; then
        log_error "SSH key not found: $SSH_KEY"
        exit 1
    fi

    log_success "Prerequisites OK"
}

# Update a single validator
update_validator() {
    local name=$1
    local ssh_target=$2

    log_info "Updating $name ($ssh_target)..."

    # Check if validator is running
    local running=$(ssh -i "$SSH_KEY" -o ConnectTimeout=5 -o StrictHostKeyChecking=no $ssh_target \
        "pgrep -f flarechain-node" 2>/dev/null || echo "")

    if [[ -z "$running" ]]; then
        log_warn "$name - flarechain-node not running, skipping"
        return 1
    fi

    # Get the systemd service file path
    local service_exists=$(ssh -i "$SSH_KEY" $ssh_target \
        "test -f /etc/systemd/system/flarechain-validator.service && echo 'yes' || echo 'no'")

    if [[ "$service_exists" == "yes" ]]; then
        # Update systemd service file
        log_info "$name - Updating systemd service with bootnodes..."

        ssh -i "$SSH_KEY" $ssh_target << 'SSHEOF'
            # Backup existing service
            sudo cp /etc/systemd/system/flarechain-validator.service \
                /etc/systemd/system/flarechain-validator.service.backup

            # Create updated service file
            sudo sed -i '/--bootnodes/d' /etc/systemd/system/flarechain-validator.service

            # Add bootnode line after --rpc-cors or --unsafe-rpc-external
            sudo sed -i '/--rpc-cors\|--unsafe-rpc-external/a\  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb,/ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6 \\' \
                /etc/systemd/system/flarechain-validator.service

            # Reload and restart
            sudo systemctl daemon-reload
            sudo systemctl restart flarechain-validator
SSHEOF

        log_success "$name - Service updated and restarted"
        return 0
    else
        # No systemd service, check for manual process
        log_warn "$name - No systemd service found, requires manual restart with --bootnodes flag"
        log_info "$name - Use: --bootnodes $BOOTNODES"
        return 1
    fi
}

# Main execution
main() {
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  Ëtrid FlareChain - Bootnode Configuration Update        ║"
    echo "║  Connecting Validators to Azure Bootstrap Nodes          ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""

    check_prereqs

    echo ""
    log_info "Bootnode Configuration:"
    echo "  VM1 (EojEdred):     $BOOTNODE_VM1"
    echo "  VM2 (Governance):   $BOOTNODE_VM2"
    echo "  VM3 (Security):     $BOOTNODE_VM3"
    echo ""

    log_info "Updating ${#VALIDATOR_NAMES[@]} validators..."
    echo ""

    local success_count=0
    local fail_count=0

    for i in "${!VALIDATOR_NAMES[@]}"; do
        if update_validator "${VALIDATOR_NAMES[$i]}" "${VALIDATOR_TARGETS[$i]}"; then
            ((success_count++))
        else
            ((fail_count++))
        fi
        echo ""
    done

    # Summary
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  Update Complete!                                         ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""
    log_info "Successfully updated: $success_count validators"
    if [[ $fail_count -gt 0 ]]; then
        log_warn "Failed/Skipped: $fail_count validators"
    fi
    echo ""

    # Verification instructions
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "VERIFICATION:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Check validator connectivity:"
    echo "  ssh -i $SSH_KEY <validator> 'sudo journalctl -u flarechain-validator -f'"
    echo ""
    echo "Look for:"
    echo "  - Discovered new external address"
    echo "  - Connection established with peer"
    echo "  - Peer count increasing (look for '2 peers', '3 peers', etc.)"
    echo ""
    echo "Check all validators at once:"
    echo "  for ip in \$(jq -r '.validators[].ip' $VALIDATOR_CONFIG); do"
    echo "    echo \"Checking \$ip:\""
    echo "    ssh -i $SSH_KEY ubuntu@\$ip 'pgrep -c flarechain-node || echo 0'"
    echo "  done"
    echo ""
}

main "$@"
