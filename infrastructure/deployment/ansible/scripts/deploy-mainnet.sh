#!/bin/bash
#
# Ëtrid Mainnet Deployment Script
# Automates deployment of mainnet infrastructure
#
# Usage: ./deploy-mainnet.sh [step]
# Steps: check, base, validators, monitoring, verify, all

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ANSIBLE_DIR="$(dirname "$SCRIPT_DIR")"
ENVIRONMENT="mainnet"
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

log_critical() {
    echo -e "${MAGENTA}[CRITICAL]${NC} $1"
}

check_prerequisites() {
    log_info "Checking prerequisites for MAINNET deployment..."

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
    if [ ! -f "$HOME/.ssh/etrid_mainnet" ]; then
        log_critical "SSH key not found at ~/.ssh/etrid_mainnet"
        log_critical "For mainnet, you MUST generate keys securely:"
        log_error "  ssh-keygen -t ed25519 -C 'etrid-mainnet' -f ~/.ssh/etrid_mainnet"
        log_error "  Use a STRONG passphrase!"
        exit 1
    else
        log_success "SSH key found"
    fi

    # MAINNET SAFETY CHECK
    log_critical "================================================================"
    log_critical "⚠️  MAINNET DEPLOYMENT - PRODUCTION ENVIRONMENT"
    log_critical "================================================================"
    log_critical "You are about to deploy to MAINNET (PRODUCTION)."
    log_critical "This deployment will:"
    log_critical "  - Deploy 10 Foundation validators"
    log_critical "  - Deploy multi-region RPC infrastructure"
    log_critical "  - Configure production monitoring"
    log_critical "  - Use REAL tokens with REAL value"
    log_critical ""
    log_critical "Have you completed:"
    log_critical "  [  ] Testnet deployment and testing"
    log_critical "  [  ] Security audit"
    log_critical "  [  ] Key generation ceremonies"
    log_critical "  [  ] Multi-signature setup"
    log_critical "  [  ] Backup procedures tested"
    log_critical "  [  ] Team training completed"
    log_critical "  [  ] Legal compliance verified"
    log_critical "================================================================"
    echo ""
    read -p "Are you ABSOLUTELY SURE you want to continue? (type 'YES' in capitals): " confirmation

    if [ "$confirmation" != "YES" ]; then
        log_error "Deployment cancelled. Good call - safety first!"
        exit 1
    fi

    log_warning "Proceeding with mainnet deployment..."
}

check_inventory_ips() {
    log_info "Checking inventory for server IPs..."

    if grep -q "0.0.0.0" "$INVENTORY"; then
        log_error "Inventory contains placeholder IPs (0.0.0.0)"
        log_error "Please update $INVENTORY with actual server IPs"
        log_error ""
        log_error "For mainnet, ensure:"
        log_error "  - Multi-region distribution (US, EU, Asia, LatAm)"
        log_error "  - High-availability servers"
        log_error "  - Geographic redundancy"
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
    log_info "Provisioning base infrastructure for MAINNET..."
    log_info "This will take 15-20 minutes for 10+ servers..."

    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/01-provision-base.yml" \
        --diff

    log_success "Base provisioning complete"
}

deploy_validators() {
    log_critical "Deploying MAINNET validators..."
    log_critical "This is a PRODUCTION deployment with REAL tokens!"

    # Deploy first validator
    log_info "Deploying validator1..."
    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/02-deploy-validator.yml" \
        --limit validator1 \
        --diff

    log_success "Validator1 deployed"
    log_critical "CRITICAL: Save the session keys to hardware security module!"
    log_critical "DO NOT proceed until keys are securely backed up!"

    read -p "Have you saved validator1 keys to HSM? (type 'YES'): " confirm1
    if [ "$confirm1" != "YES" ]; then
        log_error "Deployment halted. Save keys first!"
        exit 1
    fi

    # Deploy remaining Foundation validators
    log_info "Deploying validators 2-10..."
    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/02-deploy-validator.yml" \
        --limit validators \
        --diff

    log_success "All Foundation validators deployed"
    log_critical "Ensure ALL validator keys are backed up to HSM and offline storage!"
}

deploy_monitoring() {
    log_info "Deploying monitoring infrastructure for MAINNET..."
    log_info "This will take 10-15 minutes..."

    ansible-playbook -i "$INVENTORY" \
        "$ANSIBLE_DIR/playbooks/03-setup-monitoring.yml" \
        --diff

    log_success "Monitoring infrastructure deployed"
    log_info ""
    MONITORING_IP=$(grep -A2 "monitoring_primary:" "$INVENTORY" | grep "ansible_host:" | awk '{print $2}')
    log_info "Grafana URL: http://$MONITORING_IP:3000"
    log_info "Default login: admin / CHANGE_ME_IMMEDIATELY_MAINNET"
    log_critical "CRITICAL: Change Grafana admin password IMMEDIATELY!"
    log_critical "CRITICAL: Enable 2FA on Grafana account!"
}

verify_deployment() {
    log_info "Verifying MAINNET deployment..."

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
    log_info "Ëtrid Mainnet Status"
    log_info "====================="

    # Validator status
    echo ""
    log_info "Foundation Validators:"
    ansible validators -i "$INVENTORY" -m shell \
        -a "systemctl is-active etrid" \
        --one-line \
        || true

    # RPC status
    echo ""
    log_info "RPC Nodes:"
    ansible rpc_nodes -i "$INVENTORY" -m shell \
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
    log_success "Ëtrid Mainnet Deployment Complete!"
    log_info "================================================================"
    log_info ""
    log_critical "CRITICAL POST-DEPLOYMENT ACTIONS:"
    log_info ""
    log_info "1. Security:"
    log_critical "   ⚠️  Change ALL default passwords"
    log_critical "   ⚠️  Enable 2FA on Grafana"
    log_critical "   ⚠️  Verify HSM key backups"
    log_critical "   ⚠️  Test disaster recovery procedures"
    log_info ""
    log_info "2. Monitoring:"
    MONITORING_IP=$(grep -A2 "monitoring_primary:" "$INVENTORY" | grep "ansible_host:" | awk '{print $2}')
    log_info "   - Grafana: http://$MONITORING_IP:3000"
    log_info "   - Configure PagerDuty alerts"
    log_info "   - Set up 24/7 on-call rotation"
    log_info ""
    log_info "3. Validator Keys:"
    log_info "   - Keys are backed up in: $ANSIBLE_DIR/backups/keys/mainnet/"
    log_critical "   ⚠️  Move to hardware security modules"
    log_critical "   ⚠️  Delete plaintext backups after HSM transfer"
    log_critical "   ⚠️  Test key recovery procedures"
    log_info ""
    log_info "4. Network Health:"
    log_info "   - Monitor block production (first 24 hours)"
    log_info "   - Verify finality"
    log_info "   - Check peer connections (target: 20+ peers per validator)"
    log_info ""
    log_info "5. Community:"
    log_info "   - Announce mainnet launch"
    log_info "   - Update documentation"
    log_info "   - Enable community validator onboarding"
    log_info ""
    log_info "Documentation:"
    log_info "   - Mainnet Reusability: $ANSIBLE_DIR/../ai-devs/MAINNET_REUSABILITY_ASSESSMENT.md"
    log_info ""
    log_info "================================================================"
    log_critical "REMEMBER: This is MAINNET. Double-check everything!"
    log_info "================================================================"
}

show_usage() {
    echo "Usage: $0 [step]"
    echo ""
    echo "Steps:"
    echo "  check       - Check prerequisites and connectivity (with safety prompts)"
    echo "  base        - Provision base infrastructure"
    echo "  validators  - Deploy validator nodes (10 Foundation validators)"
    echo "  monitoring  - Deploy monitoring infrastructure"
    echo "  verify      - Verify deployment"
    echo "  status      - Show current status"
    echo "  all         - Run complete deployment (with safety checks)"
    echo ""
    echo "⚠️  WARNING: This script deploys to MAINNET (PRODUCTION)"
    echo "⚠️  Ensure testnet deployment completed successfully first"
    echo "⚠️  Ensure security audit passed"
    echo "⚠️  Ensure key ceremonies completed"
    echo ""
    echo "Examples:"
    echo "  $0 check              # Check prerequisites (with safety prompts)"
    echo "  $0 all                # Full mainnet deployment"
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
            log_critical "Starting complete MAINNET deployment..."
            check_prerequisites
            check_inventory_ips
            test_connectivity

            provision_base
            log_info "Waiting 30 seconds for services to settle..."
            sleep 30

            deploy_validators
            log_info "Waiting 60 seconds for validators to sync..."
            sleep 60

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
