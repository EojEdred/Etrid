#!/usr/bin/env bash
set -e

# Ëtrid FlareChain - Install Systemd Services (Manual Start)
# This script only INSTALLS services, does NOT start them

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

# Install service to a VM (without starting)
install_service() {
    local vm_key=$1
    local vm_ip=$2
    local vm_name=$3
    local bootnode_flag=$4

    log_info "Installing service on $vm_key ($vm_ip)..."

    # Create service file from template
    local service_content=$(cat "$TEMPLATE_FILE" | \
        sed "s|{{VALIDATOR_NAME}}|$vm_name|g" | \
        sed "s|{{BOOTNODE_FLAG}}|$bootnode_flag|g")

    # Deploy to VM
    ssh -i "$SSH_KEY" azureuser@"$vm_ip" "cat > /tmp/flarechain-validator.service" <<< "$service_content"

    # Install service (but don't start or enable)
    ssh -i "$SSH_KEY" azureuser@"$vm_ip" "sudo mv /tmp/flarechain-validator.service /etc/systemd/system/ && \
        sudo chmod 644 /etc/systemd/system/flarechain-validator.service && \
        sudo systemctl daemon-reload"

    log_success "Service installed on $vm_key (not started)"
}

# Main installation
main() {
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  Ëtrid FlareChain - Systemd Service Installation         ║"
    echo "║  (Services will NOT be started automatically)             ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""

    check_prereqs

    # Install on all 3 VMs (VM1 without bootnode, VM2/VM3 with placeholder)
    echo ""
    log_info "Installing services on all 3 VMs..."
    echo ""

    # VM1 - Bootstrap (no bootnode flag)
    install_service "VM1-Bootstrap" "$VM1_IP" "$VM1_NAME" ""

    # VM2 & VM3 - Will need bootnode flag updated before starting
    log_warn "VM2 and VM3 need bootnode address before starting!"
    log_info "You must update the service files with VM1's bootnode address"
    install_service "VM2-Governance" "$VM2_IP" "$VM2_NAME" "# --bootnodes <UPDATE_AFTER_VM1_STARTS>"
    install_service "VM3-Security" "$VM3_IP" "$VM3_NAME" "# --bootnodes <UPDATE_AFTER_VM1_STARTS>"

    # Summary
    echo ""
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  Installation Complete! ✅                                ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""
    log_success "Systemd services installed on all 3 VMs"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "MANUAL DEPLOYMENT STEPS:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "1. Start VM1 (Bootstrap Node):"
    echo "   ssh -i $SSH_KEY azureuser@$VM1_IP 'sudo systemctl start flarechain-validator'"
    echo ""
    echo "2. Get VM1 Peer ID (wait 10 seconds after start):"
    echo "   ssh -i $SSH_KEY azureuser@$VM1_IP \"sudo journalctl -u flarechain-validator | grep 'Local node identity'\""
    echo ""
    echo "3. Update VM2 and VM3 services with bootnode address:"
    echo "   # On VM2:"
    echo "   ssh -i $SSH_KEY azureuser@$VM2_IP"
    echo "   sudo nano /etc/systemd/system/flarechain-validator.service"
    echo "   # Replace: # --bootnodes <UPDATE_AFTER_VM1_STARTS>"
    echo "   # With:    --bootnodes /ip4/$VM1_IP/tcp/30333/p2p/<PEER_ID>"
    echo "   sudo systemctl daemon-reload"
    echo "   exit"
    echo ""
    echo "   # Repeat for VM3"
    echo ""
    echo "4. Start VM2 and VM3:"
    echo "   ssh -i $SSH_KEY azureuser@$VM2_IP 'sudo systemctl start flarechain-validator'"
    echo "   ssh -i $SSH_KEY azureuser@$VM3_IP 'sudo systemctl start flarechain-validator'"
    echo ""
    echo "5. Enable auto-start on reboot (optional, on each VM):"
    echo "   sudo systemctl enable flarechain-validator"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "MONITORING COMMANDS:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Check service status:"
    echo "  sudo systemctl status flarechain-validator"
    echo ""
    echo "View live logs:"
    echo "  sudo journalctl -u flarechain-validator -f"
    echo ""
    echo "Stop service:"
    echo "  sudo systemctl stop flarechain-validator"
    echo ""
}

main "$@"
