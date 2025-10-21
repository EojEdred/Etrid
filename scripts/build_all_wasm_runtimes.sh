#!/bin/bash
# Build all 14 WASM runtimes for Ëtrid Protocol
# FlareChain (relay) + 13 PBCs

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔════════════════════════════════════════════════════════════╗"
echo "║     ËTRID PROTOCOL - WASM RUNTIME BUILD SUITE             ║"
echo "║     Building 14 Runtimes (FlareChain + 13 PBCs)           ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Build tracking
TOTAL_RUNTIMES=14
SUCCESSFUL_BUILDS=0
FAILED_BUILDS=0

BUILD_LOG="/tmp/wasm_build_$(date +%Y%m%d_%H%M%S).log"

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$BUILD_LOG"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1" | tee -a "$BUILD_LOG"
    SUCCESSFUL_BUILDS=$((SUCCESSFUL_BUILDS + 1))
}

log_failure() {
    echo -e "${RED}[✗]${NC} $1" | tee -a "$BUILD_LOG"
    FAILED_BUILDS=$((FAILED_BUILDS + 1))
}

log_building() {
    echo -e "${YELLOW}[BUILD]${NC} $1" | tee -a "$BUILD_LOG"
}

# Check we're in the right directory
if [ ! -d "05-multichain" ]; then
    log_failure "Not in etrid project root directory"
    exit 1
fi

log_info "Starting WASM runtime builds..."
log_info "Build log: $BUILD_LOG"
echo ""

# 1. Build FlareChain Relay Chain Runtime
log_building "Building FlareChain relay chain runtime..."
cd 05-multichain/flare-chain/runtime

if cargo build --release --features=runtime-benchmarks >> "$BUILD_LOG" 2>&1; then
    if [ -f "../../../target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm" ]; then
        WASM_SIZE=$(ls -lh ../../../target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm | awk '{print $5}')
        log_success "FlareChain runtime built successfully (size: $WASM_SIZE)"
    else
        log_failure "FlareChain runtime build succeeded but WASM file not found"
    fi
else
    log_failure "FlareChain runtime build failed"
fi

cd ../../..

# 2. Build all 13 PBC runtimes
PBC_CHAINS=("btc" "eth" "doge" "sol" "xlm" "xrp" "bnb" "trx" "ada" "link" "matic" "sc-usdt" "edsc")

for PBC in "${PBC_CHAINS[@]}"; do
    log_building "Building ${PBC}-pbc runtime..."

    PBC_DIR="05-multichain/partition-burst-chains/pbc-chains/${PBC}-pbc/runtime"

    if [ ! -d "$PBC_DIR" ]; then
        log_failure "${PBC}-pbc runtime directory not found at $PBC_DIR"
        continue
    fi

    cd "$PBC_DIR"

    if cargo build --release >> "$BUILD_LOG" 2>&1; then
        # Find the WASM file
        WASM_PATH=$(find ../../../../target/release/wbuild -name "${PBC}_pbc_runtime.wasm" 2>/dev/null | head -1)
        if [ -n "$WASM_PATH" ]; then
            WASM_SIZE=$(ls -lh "$WASM_PATH" | awk '{print $5}')
            log_success "${PBC}-pbc runtime built successfully (size: $WASM_SIZE)"
        else
            log_failure "${PBC}-pbc runtime build succeeded but WASM file not found"
        fi
    else
        log_failure "${PBC}-pbc runtime build failed"
    fi

    cd ../../../../..
done

# Summary
echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}                    BUILD SUMMARY                            ${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Total Runtimes: $TOTAL_RUNTIMES"
echo -e "Successful: ${GREEN}$SUCCESSFUL_BUILDS${NC}"
echo -e "Failed: ${RED}$FAILED_BUILDS${NC}"
echo ""

# List all WASM files built
log_info "WASM files generated:"
find target/release/wbuild -name "*.wasm" -type f | while read WASM_FILE; do
    SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
    echo "  - $(basename $WASM_FILE) ($SIZE)"
done

echo ""
echo "Detailed build log: $BUILD_LOG"
echo ""

if [ $FAILED_BUILDS -eq 0 ]; then
    echo -e "${GREEN}✅ ALL WASM RUNTIMES BUILT SUCCESSFULLY!${NC}"
    exit 0
else
    echo -e "${RED}❌ $FAILED_BUILDS RUNTIME BUILD(S) FAILED${NC}"
    echo "Check log for details: $BUILD_LOG"
    exit 1
fi
