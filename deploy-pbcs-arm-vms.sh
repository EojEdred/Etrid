#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# DEPLOY PBC COLLATORS TO ARM VMS (d1, d5)
# ═══════════════════════════════════════════════════════════════════════════════
# Oracle Cloud ARM VMs must build locally due to architecture differences
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $*"; }
log_section() {
    echo ""
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${MAGENTA} $*${NC}"
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

ARM_VMS=("gizzi" "auditdev")

deploy_to_arm_vm() {
    local vm="$1"

    log_section "DEPLOYING TO ${vm} (ARM)"

    log_info "Uploading build script to ${vm}..."
    scp build-all-pbcs-expert.sh "${vm}:~/"

    log_info "Starting build on ${vm}..."
    ssh "${vm}" bash <<'REMOTE_SCRIPT'
        set -euo pipefail

        echo "=== ARM VM Build Session ==="
        echo "VM: $(hostname)"
        echo "Architecture: $(uname -m)"
        echo "Date: $(date)"
        echo ""

        # Navigate to etrid directory
        cd ~/Desktop/etrid || { echo "ERROR: etrid directory not found"; exit 1; }

        # Update repository
        echo "Pulling latest changes..."
        git pull origin main || echo "WARNING: git pull failed"

        # Make script executable
        if [ -f ~/build-all-pbcs-expert.sh ]; then
            mv ~/build-all-pbcs-expert.sh .
            chmod +x build-all-pbcs-expert.sh
        fi

        # Run build (sequential mode for safety on ARM)
        echo ""
        echo "Starting PBC builds (sequential mode)..."
        ./build-all-pbcs-expert.sh sequential

        echo ""
        echo "=== Build Complete on $(hostname) ==="
        echo "Binaries location: ~/Desktop/etrid/target/release/*-pbc-collator"
        ls -lh target/release/*-pbc-collator 2>/dev/null || echo "No binaries found"
REMOTE_SCRIPT

    if [ $? -eq 0 ]; then
        log_success "Build completed on ${vm}"
    else
        log_error "Build failed on ${vm}"
        return 1
    fi
}

main() {
    log_section "ARM VM PBC DEPLOYMENT"

    log_info "Target VMs: ${ARM_VMS[*]}"
    log_info "Build mode: Sequential (safer for ARM)"

    for vm in "${ARM_VMS[@]}"; do
        if ssh -o ConnectTimeout=5 "${vm}" "echo 'Connected' &>/dev/null"; then
            deploy_to_arm_vm "${vm}" &
        else
            log_warn "${vm} is not accessible, skipping..."
        fi
    done

    log_info "Waiting for all ARM builds to complete..."
    wait

    log_section "ARM VM DEPLOYMENT COMPLETE"
    log_success "All ARM VM builds completed"

    echo ""
    echo "To verify builds on each VM:"
    for vm in "${ARM_VMS[@]}"; do
        echo "  ssh ${vm} 'ls -lh ~/Desktop/etrid/target/release/*-pbc-collator'"
    done
}

main "$@"
