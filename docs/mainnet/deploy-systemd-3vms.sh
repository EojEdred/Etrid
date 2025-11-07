#!/usr/bin/env bash
set -e

# Ëtrid FlareChain - Deploy Systemd Services to 3 VMs
# This script deploys systemd service files to all 3 validator VMs

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SSH_KEY="$HOME/.ssh/gizzi-validator"
TEMPLATE_FILE="$SCRIPT_DIR/flarechain-validator-3vm.service.template"

# VM Configuration
VM1_IP="20.69.26.209"
VM1_NAME="EojEdred-Director-02"

VM2_IP="20.186.91.207"
VM2_NAME="Governance-Director-03"

VM3_IP="52.252.142.146"
VM3_NAME="Security-Director-04"

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

# Check prerequisites
check_prereqs() {
    log_info "Checking prerequisites..."

    if [[ ! -f "$SSH_KEY" ]]; then
        log_error "SSH key not found: $SSH_KEY"
        exit 1
    fi

    if [[ ! -f "$TEMPLATE_FILE" ]]; then
        log_error "Template file not found: $TEMPLATE_FILE"
        exit 1
    fi

    log_success "Prerequisites OK"
}

# Deploy service to a VM
deploy_service() {
    local vm_key=$1
    local vm_ip=$2
    local vm_name=$3
    local bootnode_flag=$4

    log_info "Deploying to $vm_key ($vm_ip)..."

    # Create service file from template
    local service_content=$(cat "$TEMPLATE_FILE" | \
        sed "s|{{VALIDATOR_NAME}}|$vm_name|g" | \
        sed "s|{{BOOTNODE_FLAG}}|$bootnode_flag|g")

    # Deploy to VM
    ssh -i "$SSH_KEY" azureuser@"$vm_ip" "cat > /tmp/flarechain-validator.service" <<< "$service_content"

    # Install service
    ssh -i "$SSH_KEY" azureuser@"$vm_ip" "sudo mv /tmp/flarechain-validator.service /etc/systemd/system/ && \
        sudo chmod 644 /etc/systemd/system/flarechain-validator.service && \
        sudo systemctl daemon-reload"

    log_success "Service deployed to $vm_key"
}

# Get peer ID from VM1
get_vm1_peer_id() {
    log_info "Starting VM1 bootstrap node to get peer ID..."

    # Start the service on VM1
    ssh -i "$SSH_KEY" azureuser@"$VM1_IP" "sudo systemctl start flarechain-validator"

    log_info "Waiting for VM1 to generate peer ID (10 seconds)..."
    sleep 10

    # Get peer ID from logs
    local peer_id=$(ssh -i "$SSH_KEY" azureuser@"$VM1_IP" \
        "sudo journalctl -u flarechain-validator --no-pager | grep 'Local node identity' | tail -1 | awk '{print \$NF}'")

    if [[ -z "$peer_id" ]]; then
        log_error "Could not extract peer ID from VM1"
        log_warn "Check VM1 logs: ssh -i $SSH_KEY azureuser@$VM1_IP 'sudo journalctl -u flarechain-validator -f'"
        exit 1
    fi

    echo "$peer_id"
}

# Main deployment
main() {
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  Ëtrid FlareChain - Systemd Service Deployment (3 VMs)   ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""

    check_prereqs

    # Step 1: Deploy VM1 (Bootstrap Node)
    echo ""
    log_info "Step 1/4: Deploying VM1 (Bootstrap Node)..."
    deploy_service "vm1_bootstrap" "$VM1_IP" "$VM1_NAME" ""

    # Step 2: Start VM1 and get peer ID
    echo ""
    log_info "Step 2/4: Getting VM1 bootnode address..."
    PEER_ID=$(get_vm1_peer_id)
    BOOTNODE_ADDR="/ip4/$VM1_IP/tcp/30333/p2p/$PEER_ID"
    log_success "VM1 Peer ID: $PEER_ID"
    log_success "Bootnode address: $BOOTNODE_ADDR"

    # Step 3: Deploy VM2 and VM3 with bootnode
    echo ""
    log_info "Step 3/4: Deploying VM2 and VM3 with bootnode..."

    BOOTNODE_FLAG="--bootnodes $BOOTNODE_ADDR"

    deploy_service "vm2_governance" "$VM2_IP" "$VM2_NAME" "$BOOTNODE_FLAG"
    deploy_service "vm3_security" "$VM3_IP" "$VM3_NAME" "$BOOTNODE_FLAG"

    # Step 4: Start VM2 and VM3
    echo ""
    log_info "Step 4/4: Starting VM2 and VM3..."
    ssh -i "$SSH_KEY" azureuser@"$VM2_IP" "sudo systemctl start flarechain-validator"
    ssh -i "$SSH_KEY" azureuser@"$VM3_IP" "sudo systemctl start flarechain-validator"
    log_success "All validators started"

    # Summary
    echo ""
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  Deployment Complete! ✅                                  ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""
    echo "VM1 (Bootstrap): $VM1_IP"
    echo "  Status: sudo systemctl status flarechain-validator"
    echo "  Logs:   sudo journalctl -u flarechain-validator -f"
    echo ""
    echo "VM2 (Governance): $VM2_IP"
    echo "  Status: sudo systemctl status flarechain-validator"
    echo "  Logs:   sudo journalctl -u flarechain-validator -f"
    echo ""
    echo "VM3 (Security): $VM3_IP"
    echo "  Status: sudo systemctl status flarechain-validator"
    echo "  Logs:   sudo journalctl -u flarechain-validator -f"
    echo ""
    echo "Enable auto-start on reboot (on each VM):"
    echo "  sudo systemctl enable flarechain-validator"
    echo ""
}

main "$@"
