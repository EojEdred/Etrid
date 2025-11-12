#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# DEPLOY PBC COLLATORS TO CONTABO VMS (x86_64)
# ═══════════════════════════════════════════════════════════════════════════════
# Strategy: Build once on primary VM, distribute to all others via rsync
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

# Contabo VMs - adjust these to match your actual SSH host aliases
# Based on your VM reference doc, you have 13+ Contabo/Azure x86_64 VMs
PRIMARY_BUILD_VM="${PRIMARY_BUILD_VM:-etrid-mainnet}"  # The VM to build on

# List all Contabo VMs to distribute to (excluding the build VM)
# You can customize this list based on your actual SSH host aliases
CONTABO_VMS=(
    "etrid-val-01"
    "etrid-val-02"
    "etrid-azure-val-01"
    "etrid-azure-val-02"
    "eojedred-validator"
    "securitydev"
    "auditdev"
    # Add more VMs as needed based on your deployment
)

# ═══════════════════════════════════════════════════════════════════════════════
# BUILD ON PRIMARY VM
# ═══════════════════════════════════════════════════════════════════════════════

build_on_primary() {
    log_section "BUILDING PBC COLLATORS ON PRIMARY VM: ${PRIMARY_BUILD_VM}"

    log_info "Uploading build script..."
    scp build-all-pbcs-expert.sh "${PRIMARY_BUILD_VM}:~/"

    log_info "Starting build process..."
    ssh "${PRIMARY_BUILD_VM}" bash <<'REMOTE_SCRIPT'
        set -euo pipefail

        echo "=== Contabo x86_64 Build Session ==="
        echo "VM: $(hostname)"
        echo "Architecture: $(uname -m)"
        echo "CPU: $(nproc) cores"
        echo "RAM: $(free -h | grep Mem | awk '{print $2}')"
        echo "Date: $(date)"
        echo ""

        # Navigate to etrid directory
        cd ~/Desktop/etrid || { echo "ERROR: etrid directory not found"; exit 1; }

        # Update repository
        echo "Pulling latest changes..."
        git pull origin main || echo "WARNING: git pull failed"

        # Move and prepare build script
        if [ -f ~/build-all-pbcs-expert.sh ]; then
            mv ~/build-all-pbcs-expert.sh .
            chmod +x build-all-pbcs-expert.sh
        fi

        # Detect RAM and choose build strategy
        ram_gb=$(free -g | grep Mem | awk '{print $2}')
        echo "Detected RAM: ${ram_gb}GB"

        if [ "${ram_gb}" -ge 24 ]; then
            echo "Using PARALLEL build mode (sufficient RAM)"
            ./build-all-pbcs-expert.sh parallel
        else
            echo "Using SEQUENTIAL build mode (limited RAM)"
            ./build-all-pbcs-expert.sh sequential
        fi

        echo ""
        echo "=== Build Complete ==="
        echo "Creating distribution directory..."
        mkdir -p ~/pbc-binaries

        # Copy all PBC binaries
        cp target/release/*-pbc-collator ~/pbc-binaries/ 2>/dev/null || true

        echo "Binaries ready for distribution:"
        ls -lh ~/pbc-binaries/
REMOTE_SCRIPT

    if [ $? -eq 0 ]; then
        log_success "Build completed on ${PRIMARY_BUILD_VM}"
        return 0
    else
        log_error "Build failed on ${PRIMARY_BUILD_VM}"
        return 1
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# DISTRIBUTE TO OTHER VMS
# ═══════════════════════════════════════════════════════════════════════════════

distribute_to_vm() {
    local vm="$1"

    log_info "Distributing to ${CYAN}${vm}${NC}..."

    # Create remote directory
    if ! ssh "${vm}" "mkdir -p ~/pbc-binaries" 2>/dev/null; then
        log_warn "Cannot connect to ${vm}, skipping..."
        return 1
    fi

    # Rsync binaries from primary VM to target VM
    if ssh "${PRIMARY_BUILD_VM}" "rsync -avz --progress ~/pbc-binaries/ ${vm}:~/pbc-binaries/" 2>&1 | grep -v "Warning"; then
        log_success "✓ ${vm}"
        return 0
    else
        log_error "✗ ${vm} - distribution failed"
        return 1
    fi
}

distribute_to_all() {
    log_section "DISTRIBUTING TO CONTABO VMS"

    local total=${#CONTABO_VMS[@]}
    local success=0

    log_info "Target VMs: ${total}"

    # Distribute to all VMs in parallel
    for vm in "${CONTABO_VMS[@]}"; do
        if [ "${vm}" != "${PRIMARY_BUILD_VM}" ]; then
            distribute_to_vm "${vm}" &
        fi
    done

    # Wait for all distributions
    wait

    # Count successes
    for vm in "${CONTABO_VMS[@]}"; do
        if ssh "${vm}" "ls ~/pbc-binaries/*-pbc-collator &>/dev/null" 2>/dev/null; then
            ((success++))
        fi
    done

    log_section "DISTRIBUTION SUMMARY"
    log_success "Successfully distributed to ${success}/${total} VMs"
}

# ═══════════════════════════════════════════════════════════════════════════════
# VERIFICATION
# ═══════════════════════════════════════════════════════════════════════════════

verify_deployment() {
    log_section "VERIFYING DEPLOYMENT"

    echo ""
    echo "VM                        | Status    | Binary Count"
    echo "--------------------------|-----------|-------------"

    # Check primary build VM
    local count=$(ssh "${PRIMARY_BUILD_VM}" "ls ~/pbc-binaries/*-pbc-collator 2>/dev/null | wc -l" 2>/dev/null || echo "0")
    printf "%-25s | %-9s | %d\n" "${PRIMARY_BUILD_VM}" "PRIMARY" "${count}"

    # Check distribution VMs
    for vm in "${CONTABO_VMS[@]}"; do
        if [ "${vm}" != "${PRIMARY_BUILD_VM}" ]; then
            local count=$(ssh "${vm}" "ls ~/pbc-binaries/*-pbc-collator 2>/dev/null | wc -l" 2>/dev/null || echo "0")
            local status="FAIL"
            if [ "${count}" -gt 0 ]; then
                status="OK"
            fi
            printf "%-25s | %-9s | %d\n" "${vm}" "${status}" "${count}"
        fi
    done
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    log_section "CONTABO VM PBC DEPLOYMENT (x86_64)"

    log_info "Primary build VM: ${CYAN}${PRIMARY_BUILD_VM}${NC}"
    log_info "Distribution targets: ${CYAN}${#CONTABO_VMS[@]}${NC} VMs"

    # Check if primary VM is accessible
    if ! ssh -o ConnectTimeout=5 "${PRIMARY_BUILD_VM}" "echo 'Connected' &>/dev/null"; then
        log_error "Cannot connect to primary build VM: ${PRIMARY_BUILD_VM}"
        log_info "Please set PRIMARY_BUILD_VM environment variable to an accessible VM"
        exit 1
    fi

    # Step 1: Build on primary VM
    if ! build_on_primary; then
        log_error "Build failed on primary VM"
        exit 1
    fi

    # Step 2: Distribute to all VMs
    distribute_to_all

    # Step 3: Verify deployment
    verify_deployment

    log_section "DEPLOYMENT COMPLETE"

    echo ""
    echo "To run a PBC collator on any VM:"
    echo "  ssh <vm-name>"
    echo "  cd ~/pbc-binaries"
    echo "  ./btc-pbc-collator --help"
}

# Parse command line arguments
case "${1:-deploy}" in
    deploy)
        main
        ;;
    verify)
        verify_deployment
        ;;
    help|--help|-h)
        cat <<EOF
ËTRID PBC Contabo Deployment Script

Usage: $0 [COMMAND]

Commands:
  deploy    Build on primary VM and distribute to all (default)
  verify    Verify deployment status across all VMs
  help      Show this help message

Environment Variables:
  PRIMARY_BUILD_VM    VM to build on (default: etrid-mainnet)

Workflow:
  1. Build all 12 PBC collators on PRIMARY_BUILD_VM
  2. Rsync binaries from primary to all Contabo VMs
  3. Verify deployment

Example:
  # Use custom build VM
  PRIMARY_BUILD_VM=etrid-val-01 $0 deploy

  # Just verify existing deployment
  $0 verify

EOF
        ;;
    *)
        log_error "Unknown command: $1"
        echo "Run '$0 help' for usage"
        exit 1
        ;;
esac
