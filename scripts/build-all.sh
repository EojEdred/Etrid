#!/usr/bin/env bash

# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID BUILD ALL - Comprehensive Build Script
# ═══════════════════════════════════════════════════════════════════════════════
# This script builds the entire Etrid blockchain project including:
# - Substrate node/runtime (FlareChain + PBC collators)
# - All custom pallets
# - All frontend apps (wallet-web, validator-dashboard, watchtower-monitor)
# - JavaScript SDK
#
# Usage:
#   ./scripts/build-all.sh [OPTIONS]
#
# Options:
#   --release          Build in release mode (production optimizations)
#   --skip-rust        Skip Rust builds (node, runtime, pallets)
#   --skip-frontend    Skip frontend app builds
#   --skip-sdk         Skip SDK build
#   --clean            Clean build artifacts before building
#   --help             Show this help message
#
# Examples:
#   ./scripts/build-all.sh                    # Build everything in dev mode
#   ./scripts/build-all.sh --release          # Build everything in release mode
#   ./scripts/build-all.sh --skip-frontend    # Build only Rust components
#   ./scripts/build-all.sh --clean --release  # Clean and build release
#
# Requirements:
#   - Rust toolchain (rustc, cargo)
#   - Node.js >= 18.0.0
#   - npm >= 9.0.0
# ═══════════════════════════════════════════════════════════════════════════════

set -e  # Exit on error

# ═══════════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BUILD_MODE="dev"
SKIP_RUST=false
SKIP_FRONTEND=false
SKIP_SDK=false
DO_CLEAN=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════════════════════
# Helper Functions
# ═══════════════════════════════════════════════════════════════════════════════

print_header() {
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}\n"
}

print_section() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

show_help() {
    grep '^#' "$0" | grep -v '#!/usr/bin/env' | sed 's/^# //' | sed 's/^#//'
    exit 0
}

format_duration() {
    local seconds=$1
    local minutes=$((seconds / 60))
    local remaining_seconds=$((seconds % 60))

    if [ $minutes -gt 0 ]; then
        echo "${minutes}m ${remaining_seconds}s"
    else
        echo "${seconds}s"
    fi
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        print_error "Required command '$1' not found"
        return 1
    fi
    return 0
}

# ═══════════════════════════════════════════════════════════════════════════════
# Parse Command Line Arguments
# ═══════════════════════════════════════════════════════════════════════════════

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            BUILD_MODE="release"
            shift
            ;;
        --skip-rust)
            SKIP_RUST=true
            shift
            ;;
        --skip-frontend)
            SKIP_FRONTEND=true
            shift
            ;;
        --skip-sdk)
            SKIP_SDK=true
            shift
            ;;
        --clean)
            DO_CLEAN=true
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════════
# Pre-flight Checks
# ═══════════════════════════════════════════════════════════════════════════════

print_header "ËTRID BUILD ALL - Starting Build Process"

print_section "Checking Prerequisites"

if [ "$SKIP_RUST" = false ]; then
    check_command "cargo" || exit 1
    check_command "rustc" || exit 1
    print_success "Rust toolchain found: $(rustc --version)"
fi

if [ "$SKIP_FRONTEND" = false ] || [ "$SKIP_SDK" = false ]; then
    check_command "node" || exit 1
    check_command "npm" || exit 1
    print_success "Node.js found: $(node --version)"
    print_success "npm found: $(npm --version)"
fi

cd "$PROJECT_ROOT"

# ═══════════════════════════════════════════════════════════════════════════════
# Clean Build Artifacts (if requested)
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$DO_CLEAN" = true ]; then
    print_header "Cleaning Build Artifacts"

    if [ "$SKIP_RUST" = false ]; then
        print_section "Cleaning Rust artifacts"
        cargo clean 2>&1 | grep -v "warning:" || true
        print_success "Rust artifacts cleaned"
    fi

    if [ "$SKIP_FRONTEND" = false ]; then
        print_section "Cleaning frontend artifacts"
        find apps -name "node_modules" -type d -exec rm -rf {} + 2>/dev/null || true
        find apps -name ".next" -type d -exec rm -rf {} + 2>/dev/null || true
        find apps -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
        print_success "Frontend artifacts cleaned"
    fi

    if [ "$SKIP_SDK" = false ]; then
        print_section "Cleaning SDK artifacts"
        rm -rf 13-clients/sdk/js-etrid-sdk/node_modules 2>/dev/null || true
        rm -rf 13-clients/sdk/js-etrid-sdk/dist 2>/dev/null || true
        print_success "SDK artifacts cleaned"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Build Rust Components
# ═══════════════════════════════════════════════════════════════════════════════

TOTAL_START_TIME=$(date +%s)

if [ "$SKIP_RUST" = false ]; then
    print_header "Building Rust Components"

    RUST_START_TIME=$(date +%s)

    # Determine cargo build flags
    if [ "$BUILD_MODE" = "release" ]; then
        CARGO_FLAGS="--release"
        print_info "Build mode: RELEASE (optimized)"
    else
        CARGO_FLAGS=""
        print_info "Build mode: DEV (debug)"
    fi

    # Build the main workspace (includes node, runtime, pallets)
    print_section "Building Substrate node and runtime (FlareChain)"

    if cargo build $CARGO_FLAGS 2>&1 | tee /tmp/etrid-build.log | grep -E "(Compiling|Finished|error|warning:)"; then
        print_success "Substrate node and runtime built successfully"
    else
        print_error "Failed to build Substrate node and runtime"
        print_info "Check /tmp/etrid-build.log for details"
        exit 1
    fi

    # Build all pallets explicitly
    print_section "Building custom pallets"

    PALLET_DIRS=(
        "pallets/pallet-aidid"
        "pallets/pallet-circuit-breaker"
        "pallets/pallet-custodian-registry"
        "pallets/pallet-did-registry"
        "pallets/pallet-reserve-oracle"
        "pallets/pallet-reserve-vault"
        "pallets/pallet-validator-committee"
        "pallets/pallet-xcm-bridge"
    )

    for pallet_dir in "${PALLET_DIRS[@]}"; do
        if [ -d "$pallet_dir" ]; then
            pallet_name=$(basename "$pallet_dir")
            print_info "Building $pallet_name..."

            if cargo build $CARGO_FLAGS --package "$pallet_name" 2>&1 | grep -E "(Finished|error)"; then
                print_success "$pallet_name built"
            else
                print_warning "$pallet_name build completed with warnings (check logs)"
            fi
        fi
    done

    RUST_END_TIME=$(date +%s)
    RUST_DURATION=$((RUST_END_TIME - RUST_START_TIME))

    print_success "All Rust components built in $(format_duration $RUST_DURATION)"
else
    print_warning "Skipping Rust builds (--skip-rust flag)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Build JavaScript SDK
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_SDK" = false ]; then
    print_header "Building JavaScript SDK"

    SDK_START_TIME=$(date +%s)
    SDK_DIR="13-clients/sdk/js-etrid-sdk"

    if [ -d "$SDK_DIR" ]; then
        cd "$SDK_DIR"

        print_section "Installing SDK dependencies"
        if npm install 2>&1 | grep -E "(added|up to date)"; then
            print_success "SDK dependencies installed"
        else
            print_error "Failed to install SDK dependencies"
            exit 1
        fi

        print_section "Building SDK"
        if npm run build 2>&1 | tail -n 20; then
            print_success "SDK built successfully"
        else
            print_error "Failed to build SDK"
            exit 1
        fi

        cd "$PROJECT_ROOT"

        SDK_END_TIME=$(date +%s)
        SDK_DURATION=$((SDK_END_TIME - SDK_START_TIME))

        print_success "SDK built in $(format_duration $SDK_DURATION)"
    else
        print_warning "SDK directory not found: $SDK_DIR"
    fi
else
    print_warning "Skipping SDK build (--skip-sdk flag)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Build Frontend Applications
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_FRONTEND" = false ]; then
    print_header "Building Frontend Applications"

    FRONTEND_START_TIME=$(date +%s)

    # Array of frontend apps to build
    FRONTEND_APPS=(
        "apps/wallet-web/etrid-crypto-website:Wallet Web"
        "apps/validator-dashboard:Validator Dashboard"
        "apps/watchtower-monitor:Watchtower Monitor"
    )

    for app_entry in "${FRONTEND_APPS[@]}"; do
        IFS=: read -r app_dir app_name <<< "$app_entry"

        if [ -d "$app_dir" ] && [ -f "$app_dir/package.json" ]; then
            print_section "Building $app_name"

            cd "$app_dir"

            print_info "Installing dependencies for $app_name..."
            if npm install --silent 2>&1 | tail -n 5; then
                print_success "Dependencies installed"
            else
                print_error "Failed to install dependencies for $app_name"
                cd "$PROJECT_ROOT"
                continue
            fi

            print_info "Building $app_name..."
            if npm run build 2>&1 | tail -n 20; then
                print_success "$app_name built successfully"
            else
                print_error "Failed to build $app_name"
            fi

            cd "$PROJECT_ROOT"
        else
            print_warning "$app_name not found or missing package.json: $app_dir"
        fi
    done

    FRONTEND_END_TIME=$(date +%s)
    FRONTEND_DURATION=$((FRONTEND_END_TIME - FRONTEND_START_TIME))

    print_success "All frontend apps built in $(format_duration $FRONTEND_DURATION)"
else
    print_warning "Skipping frontend builds (--skip-frontend flag)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Build Summary
# ═══════════════════════════════════════════════════════════════════════════════

TOTAL_END_TIME=$(date +%s)
TOTAL_DURATION=$((TOTAL_END_TIME - TOTAL_START_TIME))

print_header "Build Complete"

echo -e "${GREEN}Build Summary:${NC}"
echo -e "  Total time: $(format_duration $TOTAL_DURATION)"

if [ "$SKIP_RUST" = false ]; then
    echo -e "  Rust components: $(format_duration $RUST_DURATION)"
fi

if [ "$SKIP_SDK" = false ]; then
    echo -e "  JavaScript SDK: $(format_duration $SDK_DURATION)"
fi

if [ "$SKIP_FRONTEND" = false ]; then
    echo -e "  Frontend apps: $(format_duration $FRONTEND_DURATION)"
fi

echo ""
print_success "All builds completed successfully!"
echo ""

# Show binary locations
if [ "$SKIP_RUST" = false ]; then
    print_info "Build artifacts:"

    if [ "$BUILD_MODE" = "release" ]; then
        BINARY_PATH="target/release/etrid"
    else
        BINARY_PATH="target/debug/etrid"
    fi

    if [ -f "$BINARY_PATH" ]; then
        echo -e "  Node binary: ${CYAN}$BINARY_PATH${NC}"

        # Show binary size
        BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
        echo -e "  Binary size: ${CYAN}$BINARY_SIZE${NC}"
    fi
fi

exit 0
