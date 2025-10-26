#!/bin/bash
#
# Ëtrid Ember Testnet Deployment Script
# Automates deployment of testnet infrastructure
#
# Usage: ./deploy-testnet.sh [step]
# Steps: check, base, validators, monitoring, verify, all

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ANSIBLE_DIR="$(dirname "$SCRIPT_DIR")"
ENVIRONMENT="testnet"
INVENTORY="$ANSIBLE_DIR/environments/$ENVIRONMENT/inventory/hosts.yml"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check Ansible
    if ! command -v ansible &> /dev/null; then
        log_error "Ansible is not installed. Please install Ansible first."
        echo "  macOS: brew install ansible"
        echo "  Ubuntu: sudo apt install ansible"
        exit 1
    fi
    log_success "Ansible found: $(ansible --version | head -1)"

    # Check Python
    if ! command -v python3 &> /dev/null; then
        log_error "Python 3 is not installed."
        exit 1
    fi
    log_success "Python 3 found: $(python3 --version)"

    # Check inventory file
    if [ ! -f "$INVENTORY" ]; then
        log_error "Inventory file not found: $INVENTORY"
        exit 1
    fi
    log_success "Inventory file found"

    # Check SSH key
    if [ ! -f "$HOME/.ssh/etrid_ember" ]; then
        log_warning "SSH key not found at ~/.ssh/etrid_ember"
        log_info "Generating SSH key..."
        ssh-keygen -t ed25519 -C "etrid-ember-testnet" -f ~/.ssh/etrid_ember -N ""
        log_success "SSH key generated"
    else
        log_success "SSH key found"
    fi
}

check_inventory_ips() {
    log_info "Checking inventory for server IPs..."

    if grep -q "0.0.0.0" "$INVENTORY"; then
        log_error "Inventory contains placeholder IPs (0.0.0.0)"
        log_error "Please update $INVENTORY with actual server IPs"
        log_error ""
        log_error "Steps to get server IPs:"
        log_error "1. Provision servers on Hetzner/OVH"
        log_error "2. Run: hcloud server list"
        log_error "3. Update inventory file with real IPs"
        exit 1
    fi
    log_success "Inventory has been updated with server IPs"
}

test_connectivity() {
    log_info "Testing SSH connectivity to all servers..."

    if ansible all -i "$INVENTORY" -m ping &> /dev/null; then
        log_success "All servers are reachable"
    else
        log_error "Some servers are not reachable"
        log_info "Running connectivity test with verbose output..."
        ansible all -i "$INVENTORY" -m ping
        exit 1
    fi
}

provision_base() {
    log_info "Provisioning base infrastructure..."
    log_info "This will take 10-15 minutes..."

    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/01-provision-base.yml" \
        --diff

    log_success "Base provisioning complete"
}

deploy_validators() {
    log_info "Deploying validators..."
    log_info "This will take 5-10 minutes per validator..."

    # Deploy first validator
    log_info "Deploying validator1..."
    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/02-deploy-validator.yml" \
        --limit validator1 \
        --diff

    log_success "Validator1 deployed"
    log_warning "IMPORTANT: Save the session keys displayed above!"

    read -p "Press Enter to continue with remaining validators..."

    # Deploy remaining validators
    log_info "Deploying validators 2-3..."
    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/02-deploy-validator.yml" \
        --limit validators \
        --diff

    log_success "All validators deployed"
}

deploy_monitoring() {
    log_info "Deploying monitoring infrastructure..."
    log_info "This will take 5-10 minutes..."

    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/03-setup-monitoring.yml" \
        --diff

    log_success "Monitoring infrastructure deployed"
    log_info ""
    MONITORING_IP=$(grep -A1 "monitoring1:" "$INVENTORY" | grep "ansible_host:" | awk '{print $2}')
    log_info "Grafana URL: http://$MONITORING_IP:3000"
    log_info "Default login: admin / CHANGE_ME_IMMEDIATELY_TESTNET"
    log_warning "CRITICAL: Change Grafana admin password immediately!"
}

verify_deployment() {
    log_info "Verifying deployment..."

    # Check services
    log_info "Checking service status..."
    ansible all -i "$INVENTORY" -a "systemctl is-active etrid" || true

    # Check validator peers
    log_info "Checking validator peer connections..."
    ansible validators -i "$INVENTORY" -m shell \
        -a "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}' http://localhost:9933 | jq .result" \
        || true

    # Check block production
    log_info "Checking block production..."
    ansible validators -i "$INVENTORY" -m shell \
        -a "curl -s -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}' http://localhost:9933 | jq .result.number" \
        --limit validator1 \
        || true

    log_success "Verification complete"
}

show_status() {
    log_info "Ëtrid Ember Testnet Status"
    log_info "=============================="

    # Validator status
    echo ""
    log_info "Validators:"
    ansible validators -i "$INVENTORY" -m shell \
        -a "systemctl is-active etrid" \
        --one-line \
        || true

    # Monitoring status
    echo ""
    log_info "Monitoring:"
    ansible monitoring -i "$INVENTORY" -m shell \
        -a "systemctl is-active prometheus grafana-server" \
        --one-line \
        || true

    echo ""
    log_info "To view logs: ansible validators -i $INVENTORY -m shell -a 'journalctl -u etrid -n 20'"
}

print_next_steps() {
    log_info ""
    log_info "================================================================"
    log_success "Ember Testnet Deployment Complete!"
    log_info "================================================================"
    log_info ""
    log_info "Next Steps:"
    log_info ""
    log_info "1. Change Grafana password:"
    MONITORING_IP=$(grep -A1 "monitoring1:" "$INVENTORY" | grep "ansible_host:" | awk '{print $2}')
    log_info "   - Open: http://$MONITORING_IP:3000"
    log_info "   - Login: admin / CHANGE_ME_IMMEDIATELY_TESTNET"
    log_info "   - Settings → Change Password"
    log_info ""
    log_info "2. Save validator session keys securely"
    log_info "   - Keys are backed up in: $ANSIBLE_DIR/backups/keys/testnet/"
    log_info "   - Store in 1Password, Vault, or encrypted file"
    log_info ""
    log_info "3. Monitor network health:"
    log_info "   - Grafana dashboards"
    log_info "   - Validator logs: ssh validator1 journalctl -u etrid -f"
    log_info ""
    log_info "4. Get bootnode address:"
    log_info "   - ssh validator1"
    log_info "   - curl -H 'Content-Type: application/json' \\"
    log_info "         -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_localPeerId\"}' \\"
    log_info "         http://localhost:9933"
    log_info ""
    log_info "Documentation:"
    log_info "   - Game Plan: $ANSIBLE_DIR/../ai-devs/EMBER_TESTNET_INTEGRATION_GAMEPLAN.md"
    log_info ""
    log_info "================================================================"
}

show_usage() {
    echo "Usage: $0 [step]"
    echo ""
    echo "Steps:"
    echo "  check       - Check prerequisites and connectivity"
    echo "  base        - Provision base infrastructure"
    echo "  validators  - Deploy validator nodes"
    echo "  monitoring  - Deploy monitoring infrastructure"
    echo "  verify      - Verify deployment"
    echo "  status      - Show current status"
    echo "  all         - Run complete deployment"
    echo ""
    echo "Examples:"
    echo "  $0 check              # Check prerequisites"
    echo "  $0 all                # Full deployment"
    echo "  $0 validators         # Deploy only validators"
    echo ""
}

# Main
main() {
    case "${1:-help}" in
        check)
            check_prerequisites
            check_inventory_ips
            test_connectivity
            log_success "All checks passed!"
            ;;
        base)
            check_prerequisites
            check_inventory_ips
            test_connectivity
            provision_base
            ;;
        validators)
            check_prerequisites
            check_inventory_ips
            test_connectivity
            deploy_validators
            ;;
        monitoring)
            check_prerequisites
            check_inventory_ips
            test_connectivity
            deploy_monitoring
            ;;
        verify)
            check_prerequisites
            check_inventory_ips
            verify_deployment
            ;;
        status)
            check_prerequisites
            show_status
            ;;
        all)
            log_info "Starting complete Ember Testnet deployment..."
            check_prerequisites
            check_inventory_ips
            test_connectivity

            provision_base
            log_info "Waiting 30 seconds for services to settle..."
            sleep 30

            deploy_validators
            log_info "Waiting 30 seconds for validators to sync..."
            sleep 30

            deploy_monitoring
            log_info "Waiting 30 seconds for monitoring to start..."
            sleep 30

            verify_deployment
            print_next_steps
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            log_error "Unknown step: $1"
            show_usage
            exit 1
            ;;
    esac
}

main "$@"
