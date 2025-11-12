#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID PBC COLLATOR BUILD SCRIPT - EXPERT MODE
# ═══════════════════════════════════════════════════════════════════════════════
#
# Purpose: Build all 12 PBC collators respecting architecture differences
#
# Architecture Strategy:
# - ARM (d1, d5): Build locally on each Oracle Cloud VM
# - x86_64 (Contabo): Build once, distribute to all 13 Contabo VMs
#
# PBC Collators (12 total - ETH excluded due to Frontier dependency):
# 1. btc-pbc-collator    2. doge-pbc-collator   3. sol-pbc-collator
# 4. xlm-pbc-collator    5. xrp-pbc-collator    6. bnb-pbc-collator
# 7. trx-pbc-collator    8. ada-pbc-collator    9. link-pbc-collator
# 10. matic-pbc-collator 11. sc-usdt-pbc-collator 12. edsc-pbc-collator
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
ETRID_ROOT="${ETRID_ROOT:-$HOME/Desktop/etrid}"
BUILD_MODE="${BUILD_MODE:-release}"
PARALLEL_JOBS="${PARALLEL_JOBS:-$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)}"

# PBC Collators to build (ETH excluded - uses Frontier from stable2506)
PBC_COLLATORS=(
    "btc-pbc-collator"
    "doge-pbc-collator"
    "sol-pbc-collator"
    "xlm-pbc-collator"
    "xrp-pbc-collator"
    "bnb-pbc-collator"
    "trx-pbc-collator"
    "ada-pbc-collator"
    "link-pbc-collator"
    "matic-pbc-collator"
    "sc-usdt-pbc-collator"
    "edsc-pbc-collator"
)

# ═══════════════════════════════════════════════════════════════════════════════
# HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

log_section() {
    echo ""
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${MAGENTA} $*${NC}"
    echo -e "${MAGENTA}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

detect_architecture() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "arm"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════════════
# BUILD FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

build_single_pbc() {
    local pbc_name="$1"
    local start_time=$(date +%s)

    log_info "Building ${CYAN}${pbc_name}${NC}..."

    if cargo build --${BUILD_MODE} -p "${pbc_name}" -j "${PARALLEL_JOBS}" 2>&1 | tee "/tmp/${pbc_name}-build.log"; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        log_success "${pbc_name} built successfully in ${duration}s"

        # Verify binary exists
        if [ -f "target/${BUILD_MODE}/${pbc_name}" ]; then
            local size=$(du -h "target/${BUILD_MODE}/${pbc_name}" | cut -f1)
            log_info "Binary size: ${size}"
            return 0
        else
            log_error "Binary not found: target/${BUILD_MODE}/${pbc_name}"
            return 1
        fi
    else
        log_error "${pbc_name} build failed! See /tmp/${pbc_name}-build.log for details"
        return 1
    fi
}

build_all_pbcs_sequential() {
    log_section "BUILDING ALL PBC COLLATORS (Sequential Mode)"

    local total=${#PBC_COLLATORS[@]}
    local success=0
    local failed=0
    local failed_builds=()

    for i in "${!PBC_COLLATORS[@]}"; do
        local pbc="${PBC_COLLATORS[$i]}"
        local current=$((i + 1))

        echo ""
        log_info "Progress: ${current}/${total} - Building ${pbc}..."

        if build_single_pbc "${pbc}"; then
            ((success++))
        else
            ((failed++))
            failed_builds+=("${pbc}")
        fi
    done

    log_section "BUILD SUMMARY"
    log_success "Successful builds: ${success}/${total}"

    if [ ${failed} -gt 0 ]; then
        log_error "Failed builds: ${failed}/${total}"
        log_error "Failed: ${failed_builds[*]}"
        return 1
    fi

    return 0
}

build_all_pbcs_parallel() {
    log_section "BUILDING ALL PBC COLLATORS (Parallel Mode - ${PARALLEL_JOBS} jobs)"

    local pids=()
    local results_dir="/tmp/pbc-build-results-$$"
    mkdir -p "${results_dir}"

    # Start all builds in parallel
    for pbc in "${PBC_COLLATORS[@]}"; do
        (
            if build_single_pbc "${pbc}"; then
                echo "SUCCESS" > "${results_dir}/${pbc}.status"
            else
                echo "FAILED" > "${results_dir}/${pbc}.status"
            fi
        ) &
        pids+=($!)
    done

    # Wait for all builds
    log_info "Waiting for ${#pids[@]} parallel builds to complete..."
    for pid in "${pids[@]}"; do
        wait "$pid"
    done

    # Analyze results
    local success=0
    local failed=0
    local failed_builds=()

    for pbc in "${PBC_COLLATORS[@]}"; do
        if [ -f "${results_dir}/${pbc}.status" ]; then
            local status=$(cat "${results_dir}/${pbc}.status")
            if [ "$status" = "SUCCESS" ]; then
                ((success++))
            else
                ((failed++))
                failed_builds+=("${pbc}")
            fi
        else
            ((failed++))
            failed_builds+=("${pbc} (no status)")
        fi
    done

    # Cleanup
    rm -rf "${results_dir}"

    log_section "BUILD SUMMARY"
    log_success "Successful builds: ${success}/${#PBC_COLLATORS[@]}"

    if [ ${failed} -gt 0 ]; then
        log_error "Failed builds: ${failed}/${#PBC_COLLATORS[@]}"
        log_error "Failed: ${failed_builds[*]}"
        return 1
    fi

    return 0
}

# ═══════════════════════════════════════════════════════════════════════════════
# DISTRIBUTION FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

create_distribution_tarball() {
    log_section "CREATING DISTRIBUTION TARBALL"

    local arch=$(detect_architecture)
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local tarball="pbc-collators-${arch}-${timestamp}.tar.gz"

    log_info "Collecting binaries..."

    local temp_dir="/tmp/pbc-dist-$$"
    mkdir -p "${temp_dir}"

    for pbc in "${PBC_COLLATORS[@]}"; do
        if [ -f "target/${BUILD_MODE}/${pbc}" ]; then
            cp "target/${BUILD_MODE}/${pbc}" "${temp_dir}/"
            log_info "✓ ${pbc}"
        else
            log_warn "✗ ${pbc} not found"
        fi
    done

    log_info "Creating tarball: ${tarball}..."
    tar -czf "${tarball}" -C "${temp_dir}" .

    local size=$(du -h "${tarball}" | cut -f1)
    log_success "Created ${tarball} (${size})"

    rm -rf "${temp_dir}"

    echo "${tarball}"
}

distribute_to_vms() {
    local tarball="$1"
    local vms=("$@")

    log_section "DISTRIBUTING TO VMS"

    if [ ${#vms[@]} -lt 2 ]; then
        log_warn "No VMs specified for distribution"
        return 0
    fi

    # Skip first argument (tarball)
    vms=("${@:2}")

    log_info "Distributing ${tarball} to ${#vms[@]} VMs..."

    for vm in "${vms[@]}"; do
        log_info "Uploading to ${vm}..."

        if scp "${tarball}" "${vm}:~/" 2>&1 | grep -v "Warning"; then
            log_info "Extracting on ${vm}..."

            if ssh "${vm}" "mkdir -p ~/etrid-binaries && cd ~/etrid-binaries && tar -xzf ~/${tarball} && rm ~/${tarball}"; then
                log_success "✓ ${vm}"
            else
                log_error "✗ Failed to extract on ${vm}"
            fi
        else
            log_error "✗ Failed to upload to ${vm}"
        fi
    done
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN EXECUTION
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    local arch=$(detect_architecture)

    log_section "ËTRID PBC COLLATOR BUILD - EXPERT MODE"

    log_info "Architecture: ${CYAN}${arch}${NC}"
    log_info "Build mode: ${CYAN}${BUILD_MODE}${NC}"
    log_info "Parallel jobs: ${CYAN}${PARALLEL_JOBS}${NC}"
    log_info "ËTRID root: ${CYAN}${ETRID_ROOT}${NC}"
    log_info "PBC count: ${CYAN}${#PBC_COLLATORS[@]}${NC}"

    # Change to ËTRID root
    cd "${ETRID_ROOT}"

    # Parse command line arguments
    local mode="${1:-sequential}"

    case "${mode}" in
        sequential)
            build_all_pbcs_sequential
            ;;
        parallel)
            build_all_pbcs_parallel
            ;;
        distribute)
            shift
            local tarball=$(create_distribution_tarball)
            distribute_to_vms "${tarball}" "$@"
            ;;
        help|--help|-h)
            cat <<EOF
ËTRID PBC Collator Build Script - Expert Mode

Usage: $0 [MODE] [OPTIONS]

Modes:
  sequential           Build all PBCs one by one (default, safer)
  parallel             Build all PBCs concurrently (faster, needs more RAM)
  distribute [VMs...]  Create tarball and distribute to VMs

Environment Variables:
  ETRID_ROOT          Path to ËTRID repository (default: ~/Desktop/etrid)
  BUILD_MODE          Build mode: release or debug (default: release)
  PARALLEL_JOBS       Number of parallel cargo jobs (default: auto-detect)

Examples:
  # Sequential build (safer for limited RAM)
  $0 sequential

  # Parallel build (faster, needs 32GB+ RAM)
  $0 parallel

  # Build and distribute to Contabo VMs
  $0 sequential
  $0 distribute contabo-01 contabo-02 contabo-03

Architecture Support:
  - ARM (d1, d5): Run this script locally on each VM
  - x86_64 (Contabo): Run once, then use 'distribute' mode

PBC Collators (12 total):
  $(printf '  %s\n' "${PBC_COLLATORS[@]}")

EOF
            exit 0
            ;;
        *)
            log_error "Unknown mode: ${mode}"
            log_info "Run '$0 help' for usage information"
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
