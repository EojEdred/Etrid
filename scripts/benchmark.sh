#!/bin/bash
# Ëtrid Protocol Runtime Benchmarking Script
# Measures extrinsic weights and performance characteristics

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔════════════════════════════════════════════════════════════╗"
echo "║       ËTRID PROTOCOL - RUNTIME BENCHMARKING                ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Configuration
FLARECHAIN_NODE="${FLARECHAIN_NODE:-./target/release/flarechain-node}"
BENCHMARK_OUTPUT_DIR="${BENCHMARK_OUTPUT_DIR:-./benchmarks}"
REPETITIONS="${REPETITIONS:-20}"

# Create benchmark output directory
mkdir -p "$BENCHMARK_OUTPUT_DIR"

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if node is built with runtime-benchmarks feature
check_benchmarks_feature() {
    log_info "Checking if node is built with runtime-benchmarks feature..."

    if [ ! -f "$FLARECHAIN_NODE" ]; then
        log_warning "FlareChain node binary not found at $FLARECHAIN_NODE"
        log_info "Building with runtime-benchmarks feature..."

        cd 05-multichain/flare-chain
        cargo build --release --features runtime-benchmarks
        cd ../..

        log_success "Node built with runtime-benchmarks"
    else
        log_success "Node binary found"
    fi
}

# Benchmark pallet-edsc-token
benchmark_edsc_token() {
    log_info "Benchmarking pallet-edsc-token..."

    $FLARECHAIN_NODE benchmark pallet \
        --chain=dev \
        --pallet=pallet_edsc_token \
        --extrinsic='*' \
        --steps=50 \
        --repeat=$REPETITIONS \
        --output="$BENCHMARK_OUTPUT_DIR/pallet_edsc_token.rs" \
        2>&1 | tee "$BENCHMARK_OUTPUT_DIR/pallet_edsc_token.log" || true

    log_success "pallet-edsc-token benchmarking complete"
}

# Benchmark pallet-edsc-redemption
benchmark_edsc_redemption() {
    log_info "Benchmarking pallet-edsc-redemption..."

    $FLARECHAIN_NODE benchmark pallet \
        --chain=dev \
        --pallet=pallet_edsc_redemption \
        --extrinsic='*' \
        --steps=50 \
        --repeat=$REPETITIONS \
        --output="$BENCHMARK_OUTPUT_DIR/pallet_edsc_redemption.rs" \
        2>&1 | tee "$BENCHMARK_OUTPUT_DIR/pallet_edsc_redemption.log" || true

    log_success "pallet-edsc-redemption benchmarking complete"
}

# Benchmark pallet-reserve-vault
benchmark_reserve_vault() {
    log_info "Benchmarking pallet-reserve-vault..."

    $FLARECHAIN_NODE benchmark pallet \
        --chain=dev \
        --pallet=pallet_reserve_vault \
        --extrinsic='*' \
        --steps=50 \
        --repeat=$REPETITIONS \
        --output="$BENCHMARK_OUTPUT_DIR/pallet_reserve_vault.rs" \
        2>&1 | tee "$BENCHMARK_OUTPUT_DIR/pallet_reserve_vault.log" || true

    log_success "pallet-reserve-vault benchmarking complete"
}

# Benchmark storage operations
benchmark_storage() {
    log_info "Benchmarking storage read/write performance..."

    $FLARECHAIN_NODE benchmark storage \
        --chain=dev \
        --state-version=1 \
        2>&1 | tee "$BENCHMARK_OUTPUT_DIR/storage_benchmark.log" || true

    log_success "Storage benchmarking complete"
}

# Benchmark overhead (block execution, extrinsic base weight)
benchmark_overhead() {
    log_info "Benchmarking overhead (block/extrinsic base weights)..."

    $FLARECHAIN_NODE benchmark overhead \
        --chain=dev \
        2>&1 | tee "$BENCHMARK_OUTPUT_DIR/overhead_benchmark.log" || true

    log_success "Overhead benchmarking complete"
}

# Generate benchmark summary
generate_summary() {
    log_info "Generating benchmark summary..."

    SUMMARY_FILE="$BENCHMARK_OUTPUT_DIR/benchmark_summary.txt"

    {
        echo "Ëtrid Protocol - Benchmark Summary"
        echo "Generated: $(date)"
        echo ""
        echo "Configuration:"
        echo "  - Repetitions: $REPETITIONS"
        echo "  - Steps: 50"
        echo ""
        echo "Benchmarked Components:"
        echo "  - pallet-edsc-token"
        echo "  - pallet-edsc-redemption"
        echo "  - pallet-reserve-vault"
        echo "  - Storage operations"
        echo "  - Overhead (block/extrinsic base weights)"
        echo ""
        echo "Output Directory: $BENCHMARK_OUTPUT_DIR"
        echo ""
        echo "Files Generated:"
        ls -lh "$BENCHMARK_OUTPUT_DIR" | tail -n +2
    } > "$SUMMARY_FILE"

    cat "$SUMMARY_FILE"

    log_success "Benchmark summary generated at $SUMMARY_FILE"
}

# Main execution
main() {
    log_info "Starting runtime benchmarking..."
    echo ""

    check_benchmarks_feature

    # Run benchmarks
    benchmark_edsc_token
    benchmark_edsc_redemption
    benchmark_reserve_vault
    benchmark_storage
    benchmark_overhead

    # Generate summary
    generate_summary

    echo ""
    log_success "All benchmarks complete!"
    echo ""
}

main
