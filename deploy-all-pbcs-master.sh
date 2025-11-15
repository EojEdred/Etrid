#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# MASTER PBC DEPLOYMENT ORCHESTRATOR
# ═══════════════════════════════════════════════════════════════════════════════
# Orchestrates PBC builds across all VMs respecting architecture differences
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_section() {
    echo ""
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${MAGENTA} $*${NC}"
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Architecture groups
ARM_VMS=("gizzi" "auditdev")
PRIMARY_CONTABO="${PRIMARY_BUILD_VM:-eojedred-validator}"

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN ORCHESTRATION
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    log_section "ËTRID PBC MASTER DEPLOYMENT ORCHESTRATOR"

    cat <<EOF
${CYAN}
┌───────────────────────────────────────────────────────────────────┐
│                   ËTRID PBC Build Strategy                         │
├───────────────────────────────────────────────────────────────────┤
│                                                                    │
│  Architecture Group 1: ARM (Oracle Cloud)                         │
│  ├─ gizzi (Gizzi)      → Build locally                            │
│  └─ auditdev (Audit)   → Build locally                            │
│                                                                    │
│  Architecture Group 2: x86_64 (Azure/Contabo)                     │
│  ├─ ${PRIMARY_CONTABO} → Build once (primary)                     │
│  └─ 6+ other VMs → Rsync from primary                             │
│                                                                    │
│  PBC Collators: 12 total (ETH excluded - uses Frontier)           │
│  ├─ btc-pbc-collator    ├─ sol-pbc-collator    ├─ ada-pbc-collator│
│  ├─ doge-pbc-collator   ├─ xlm-pbc-collator    ├─ link-pbc-collator│
│  ├─ eth-pbc-collator*   ├─ xrp-pbc-collator    ├─ matic-pbc-collator│
│  └─ (*excluded)         ├─ bnb-pbc-collator    ├─ sc-usdt-pbc-collator│
│                         └─ trx-pbc-collator    └─ edsc-pbc-collator│
└───────────────────────────────────────────────────────────────────┘
${NC}
EOF

    echo "Select deployment mode:"
    echo "  1) Deploy to ARM VMs only (d1, d5)"
    echo "  2) Deploy to Contabo VMs only (x86_64)"
    echo "  3) Deploy to ALL VMs (ARM + Contabo) - RECOMMENDED"
    echo "  4) Verify deployment status"
    echo "  5) Exit"
    echo ""

    read -p "Enter choice [1-5]: " choice

    case "$choice" in
        1)
            deploy_arm_vms
            ;;
        2)
            deploy_contabo_vms
            ;;
        3)
            deploy_all_vms
            ;;
        4)
            verify_all_deployments
            ;;
        5)
            log_info "Exiting..."
            exit 0
            ;;
        *)
            log_error "Invalid choice"
            exit 1
            ;;
    esac
}

deploy_arm_vms() {
    log_section "DEPLOYING TO ARM VMS"

    if [ ! -f "${SCRIPT_DIR}/deploy-pbcs-arm-vms.sh" ]; then
        log_error "ARM deployment script not found!"
        exit 1
    fi

    log_info "Starting ARM VM deployment (d1, d5)..."
    log_info "This will build PBCs locally on each ARM VM concurrently"

    "${SCRIPT_DIR}/deploy-pbcs-arm-vms.sh"

    log_success "ARM VM deployment completed"
}

deploy_contabo_vms() {
    log_section "DEPLOYING TO CONTABO VMS"

    if [ ! -f "${SCRIPT_DIR}/deploy-pbcs-contabo-vms.sh" ]; then
        log_error "Contabo deployment script not found!"
        exit 1
    fi

    log_info "Starting Contabo VM deployment..."
    log_info "Build will run on ${PRIMARY_CONTABO}, then distribute to all VMs"

    "${SCRIPT_DIR}/deploy-pbcs-contabo-vms.sh" deploy

    log_success "Contabo VM deployment completed"
}

deploy_all_vms() {
    log_section "DEPLOYING TO ALL VMS (ARM + CONTABO)"

    local start_time=$(date +%s)

    log_info "Starting parallel deployment to all VM groups..."

    # Deploy to ARM VMs in background
    {
        log_info "Starting ARM VM deployment..."
        deploy_arm_vms
        log_success "ARM VM deployment finished"
    } &
    local arm_pid=$!

    # Deploy to Contabo VMs in background
    {
        log_info "Starting Contabo VM deployment..."
        deploy_contabo_vms
        log_success "Contabo VM deployment finished"
    } &
    local contabo_pid=$!

    # Wait for both deployments
    log_info "Waiting for ARM and Contabo deployments to complete..."

    wait $arm_pid
    local arm_status=$?

    wait $contabo_pid
    local contabo_status=$?

    local end_time=$(date +%s)
    local duration=$((end_time - start_time))

    log_section "DEPLOYMENT SUMMARY"

    if [ $arm_status -eq 0 ]; then
        log_success "✓ ARM VMs (d1, d5): SUCCESS"
    else
        log_error "✗ ARM VMs (d1, d5): FAILED"
    fi

    if [ $contabo_status -eq 0 ]; then
        log_success "✓ Contabo VMs: SUCCESS"
    else
        log_error "✗ Contabo VMs: FAILED"
    fi

    log_info "Total deployment time: ${duration}s"

    if [ $arm_status -eq 0 ] && [ $contabo_status -eq 0 ]; then
        log_success "All deployments completed successfully!"
        return 0
    else
        log_error "Some deployments failed"
        return 1
    fi
}

verify_all_deployments() {
    log_section "VERIFYING ALL DEPLOYMENTS"

    echo ""
    echo "ARM VMs:"
    echo "─────────"
    for vm in "${ARM_VMS[@]}"; do
        if ssh -o ConnectTimeout=5 "${vm}" "echo 'OK' &>/dev/null" 2>/dev/null; then
            local count=$(ssh "${vm}" "ls ~/Desktop/etrid/target/release/*-pbc-collator 2>/dev/null | wc -l" 2>/dev/null || echo "0")
            if [ "$count" -gt 0 ]; then
                log_success "${vm}: ${count} binaries found"
            else
                log_warn "${vm}: No binaries found"
            fi
        else
            log_error "${vm}: Not accessible"
        fi
    done

    echo ""
    echo "Contabo VMs:"
    echo "─────────────"
    "${SCRIPT_DIR}/deploy-pbcs-contabo-vms.sh" verify
}

# ═══════════════════════════════════════════════════════════════════════════════
# EXECUTION
# ═══════════════════════════════════════════════════════════════════════════════

case "${1:-interactive}" in
    interactive)
        main
        ;;
    arm)
        deploy_arm_vms
        ;;
    contabo)
        deploy_contabo_vms
        ;;
    all)
        deploy_all_vms
        ;;
    verify)
        verify_all_deployments
        ;;
    help|--help|-h)
        cat <<EOF
ËTRID PBC Master Deployment Orchestrator

Usage: $0 [COMMAND]

Commands:
  interactive    Interactive menu (default)
  arm            Deploy to ARM VMs only (d1, d5)
  contabo        Deploy to Contabo VMs only (x86_64)
  all            Deploy to ALL VMs (ARM + Contabo) - RECOMMENDED
  verify         Verify deployment status across all VMs
  help           Show this help message

Environment Variables:
  PRIMARY_BUILD_VM    Primary Contabo VM for building (default: etrid-mainnet)

Architecture-Aware Deployment:
  - ARM VMs (d1, d5): Builds run locally on each VM
  - x86_64 VMs: Build once on primary, distribute to all others

Examples:
  # Interactive mode (recommended for first-time users)
  $0

  # Non-interactive: Deploy to all VMs
  $0 all

  # Deploy only to ARM VMs
  $0 arm

  # Deploy only to Contabo VMs with custom primary
  PRIMARY_BUILD_VM=etrid-val-01 $0 contabo

  # Verify deployment status
  $0 verify

Files:
  build-all-pbcs-expert.sh       Core build script
  deploy-pbcs-arm-vms.sh         ARM VM deployment
  deploy-pbcs-contabo-vms.sh     Contabo VM deployment
  deploy-all-pbcs-master.sh      This orchestrator (you are here)

EOF
        ;;
    *)
        log_error "Unknown command: $1"
        echo "Run '$0 help' for usage"
        exit 1
        ;;
esac
