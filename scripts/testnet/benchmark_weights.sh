#!/bin/bash
# Ëtrid Protocol Weight Benchmarking Suite
# Generates production-ready weight values for all pallets
# Addresses AUDIT_PACKAGE.md requirement (DoS via placeholder weights)

set -e

# ============================================================================
# Configuration
# ============================================================================

ETRID_ROOT="${ETRID_ROOT:-/Users/macbook/Desktop/etrid}"
BIN_DIR="$ETRID_ROOT/target/release"
WEIGHTS_OUTPUT="$ETRID_ROOT/runtime-weights"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Benchmark configuration
BENCHMARK_STEPS=50
BENCHMARK_REPEAT=20
BENCHMARK_RUNS=10

# Pallets to benchmark
PALLETS=(
    "pallet-validator-committee"
    "pallet-edsc-token"
    "pallet-edsc-redemption"
    "pallet-edsc-checkpoint"
    "pallet-etwasm-vm"
)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Progress tracking
BENCHMARKS_RUN=0
BENCHMARKS_SUCCESS=0
BENCHMARKS_FAILED=0

# ============================================================================
# Helper Functions
# ============================================================================

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; BENCHMARKS_SUCCESS=$((BENCHMARKS_SUCCESS + 1)); }
log_failure() { echo -e "${RED}[FAILURE]${NC} $1"; BENCHMARKS_FAILED=$((BENCHMARKS_FAILED + 1)); }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_section() {
    echo ""
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${PURPLE}  $1${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# ============================================================================
# Pre-Flight Checks
# ============================================================================

preflight_checks() {
    log_section "Pre-Flight Checks"

    # Check if runtime-benchmarks feature is available
    if [ ! -f "$BIN_DIR/flarechain-node" ]; then
        log_failure "FlareChain node binary not found"
        log_info "Build with runtime-benchmarks feature:"
        log_info "  cd $ETRID_ROOT"
        log_info "  cargo build --release --features runtime-benchmarks -p flarechain-node"
        exit 1
    fi

    # Check if binary supports benchmarking
    if ! $BIN_DIR/flarechain-node benchmark pallet --help &>/dev/null; then
        log_failure "Binary does not support benchmarking"
        log_info "Rebuild with runtime-benchmarks feature:"
        log_info "  cargo build --release --features runtime-benchmarks -p flarechain-node"
        exit 1
    fi

    log_success "Binary supports benchmarking"

    # Create output directory
    mkdir -p "$WEIGHTS_OUTPUT"
    mkdir -p "$WEIGHTS_OUTPUT/backup-$TIMESTAMP"

    log_success "Output directory: $WEIGHTS_OUTPUT"
}

# ============================================================================
# Benchmark Individual Pallet
# ============================================================================

benchmark_pallet() {
    local pallet="$1"
    local pallet_name=$(echo "$pallet" | sed 's/pallet-//')

    log_section "Benchmarking: $pallet"

    BENCHMARKS_RUN=$((BENCHMARKS_RUN + 1))

    local output_file="$WEIGHTS_OUTPUT/${pallet_name}.rs"
    local backup_file="$WEIGHTS_OUTPUT/backup-$TIMESTAMP/${pallet_name}.rs"

    # Backup existing weights if they exist
    if [ -f "$output_file" ]; then
        log_info "Backing up existing weights..."
        cp "$output_file" "$backup_file"
    fi

    log_info "Running benchmark..."
    log_info "  Steps: $BENCHMARK_STEPS"
    log_info "  Repeat: $BENCHMARK_REPEAT"
    log_info "  Runs: $BENCHMARK_RUNS"

    # Run benchmark
    local benchmark_start=$(date +%s)

    if $BIN_DIR/flarechain-node benchmark pallet \
        --chain dev \
        --pallet "$pallet" \
        --extrinsic "*" \
        --steps $BENCHMARK_STEPS \
        --repeat $BENCHMARK_REPEAT \
        --wasm-execution=compiled \
        --output "$output_file" \
        --template "$ETRID_ROOT/.maintain/frame-weight-template.hbs" \
        2>&1 | tee "$WEIGHTS_OUTPUT/${pallet_name}-benchmark.log"; then

        local benchmark_end=$(date +%s)
        local duration=$((benchmark_end - benchmark_start))

        log_success "Benchmark completed in ${duration}s"
        log_info "  Output: $output_file"
        log_info "  Log: $WEIGHTS_OUTPUT/${pallet_name}-benchmark.log"

        # Verify output file
        if [ -f "$output_file" ] && [ -s "$output_file" ]; then
            local extrinsics=$(grep -c "fn " "$output_file" || echo "0")
            log_info "  Extrinsics benchmarked: $extrinsics"
            return 0
        else
            log_failure "Output file is empty or missing"
            return 1
        fi
    else
        log_failure "Benchmark failed for $pallet"
        log_info "  Check log: $WEIGHTS_OUTPUT/${pallet_name}-benchmark.log"
        return 1
    fi
}

# ============================================================================
# Benchmark FlareChain Runtime
# ============================================================================

benchmark_flarechain_runtime() {
    log_section "Benchmarking FlareChain Runtime (All Pallets)"

    # FlareChain-specific pallets
    local flarechain_pallets=(
        "frame_system"
        "pallet_balances"
        "pallet_timestamp"
        "pallet_transaction_payment"
        "pallet_validator_committee"
    )

    for pallet in "${flarechain_pallets[@]}"; do
        benchmark_pallet "$pallet"
        echo ""
    done
}

# ============================================================================
# Benchmark EDSC Bridge Pallets
# ============================================================================

benchmark_edsc_pallets() {
    log_section "Benchmarking EDSC Bridge Pallets"

    local edsc_pallets=(
        "pallet_edsc_token"
        "pallet_edsc_redemption"
        "pallet_edsc_checkpoint"
    )

    for pallet in "${edsc_pallets[@]}"; do
        # Check if pallet exists
        if $BIN_DIR/flarechain-node benchmark pallet --list | grep -q "^$pallet$"; then
            benchmark_pallet "$pallet"
        else
            log_warning "Pallet $pallet not found in runtime"
        fi
        echo ""
    done
}

# ============================================================================
# Benchmark EtwasmVM Pallet
# ============================================================================

benchmark_etwasm() {
    log_section "Benchmarking ËtwasmVM Smart Contract Pallet"

    local pallet="pallet_etwasm_vm"

    if $BIN_DIR/flarechain-node benchmark pallet --list | grep -q "^$pallet$"; then
        benchmark_pallet "$pallet"
    else
        log_warning "ËtwasmVM pallet not found in runtime"
        log_info "  This is expected if EtwasmVM is not yet integrated"
    fi
}

# ============================================================================
# Analyze Weight Changes
# ============================================================================

analyze_weight_changes() {
    log_section "Analyzing Weight Changes"

    if [ ! -d "$WEIGHTS_OUTPUT/backup-$TIMESTAMP" ] || [ -z "$(ls -A "$WEIGHTS_OUTPUT/backup-$TIMESTAMP")" ]; then
        log_info "No previous weights to compare"
        return 0
    fi

    log_info "Comparing with previous weights..."

    for new_file in "$WEIGHTS_OUTPUT"/*.rs; do
        if [ ! -f "$new_file" ]; then
            continue
        fi

        local filename=$(basename "$new_file")
        local old_file="$WEIGHTS_OUTPUT/backup-$TIMESTAMP/$filename"

        if [ ! -f "$old_file" ]; then
            log_info "  $filename: NEW"
            continue
        fi

        # Compare file sizes as rough indicator
        local old_size=$(wc -c < "$old_file")
        local new_size=$(wc -c < "$new_file")
        local size_diff=$((new_size - old_size))

        if [ $size_diff -gt 0 ]; then
            log_info "  $filename: INCREASED (+${size_diff} bytes)"
        elif [ $size_diff -lt 0 ]; then
            log_info "  $filename: DECREASED (${size_diff} bytes)"
        else
            log_info "  $filename: UNCHANGED"
        fi
    done
}

# ============================================================================
# Generate Integration Instructions
# ============================================================================

generate_integration_instructions() {
    log_section "Integration Instructions"

    cat > "$WEIGHTS_OUTPUT/INTEGRATION.md" <<EOF
# Weight Integration Instructions

Generated: $(date)

## Overview

Production weight values have been generated for all Ëtrid Protocol pallets.
These weights replace placeholder values to prevent DoS attacks via cheap operations.

## Generated Weights

The following weight files have been generated:

EOF

    for file in "$WEIGHTS_OUTPUT"/*.rs; do
        if [ -f "$file" ]; then
            local filename=$(basename "$file")
            echo "- $filename" >> "$WEIGHTS_OUTPUT/INTEGRATION.md"
        fi
    done

    cat >> "$WEIGHTS_OUTPUT/INTEGRATION.md" <<'EOF'

## Integration Steps

### 1. Review Generated Weights

Before integration, review the generated weight files:

```bash
# Check for any anomalies
for file in runtime-weights/*.rs; do
    echo "Reviewing: $file"
    grep "Weight::from_parts" "$file" | head -5
done
```

### 2. Copy to Runtime

Copy weight files to the runtime directory:

```bash
# FlareChain runtime
cp runtime-weights/*.rs 05-multichain/flare-chain/runtime/src/weights/

# Or for specific pallets
cp runtime-weights/validator_committee.rs \
   pallets/pallet-validator-committee/src/weights.rs
```

### 3. Update Runtime Configuration

In your runtime `lib.rs`, import the weights:

```rust
// Import weight modules
pub mod weights;

// Use in pallet configuration
impl pallet_validator_committee::Config for Runtime {
    type WeightInfo = weights::validator_committee::WeightInfo<Runtime>;
    // ... other config
}
```

### 4. Verify Integration

Test that weights are properly integrated:

```bash
# Build runtime
cargo build --release -p flare-chain-runtime

# Run tests
cargo test -p pallet-validator-committee

# Check that weights are no longer placeholder
grep -r "Weight::from_parts(10_000" runtime/src/weights/
# Should return no results
```

### 5. Transaction Fee Testing

Test that transaction fees are now accurate:

```bash
# Submit test transactions and verify fees
# Expected: Fees vary based on actual computation
# Previous: All operations cost similar amounts (placeholder weights)
```

## Validation Checklist

- [ ] All weight files generated successfully
- [ ] No compilation errors after integration
- [ ] Transaction fees vary appropriately by operation
- [ ] Heavy operations (like `add_validator`) cost more than light operations
- [ ] No operations have suspiciously low weights (<1000)
- [ ] Runtime benchmarks pass: `cargo test --features runtime-benchmarks`

## Troubleshooting

### Weight Overflow Errors

If you see weight overflow errors:
- Review the generated weights for anomalies
- Re-run benchmarks with `--repeat 50` for more samples
- Check that `Weight::from_parts()` values are reasonable

### Performance Regression

If blocks are taking too long:
- Verify weights are correctly applied
- Check for any 10x or 100x anomalies in weight values
- Consider re-benchmarking on production hardware

## Mainnet Preparation

Before mainnet:
1. Run benchmarks on production-grade hardware
2. Perform weight audits for critical extrinsics
3. Test under high load (1000+ tx/s)
4. Verify that weight limits prevent block stuffing

## Support

For issues or questions:
- Discord: https://discord.gg/etrid
- GitHub Issues: https://github.com/etrid-protocol/etrid/issues

EOF

    log_success "Integration guide: $WEIGHTS_OUTPUT/INTEGRATION.md"
}

# ============================================================================
# Generate Summary Report
# ============================================================================

generate_report() {
    log_section "Benchmark Summary"

    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "                  WEIGHT BENCHMARKING SUMMARY                   "
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Date: $(date)"
    echo "Configuration:"
    echo "  Steps: $BENCHMARK_STEPS"
    echo "  Repeat: $BENCHMARK_REPEAT"
    echo "  Runs: $BENCHMARK_RUNS"
    echo ""
    echo "Results:"
    echo "  Benchmarks Run:     $BENCHMARKS_RUN"
    echo -e "  ${GREEN}Successful:         $BENCHMARKS_SUCCESS${NC}"
    echo -e "  ${RED}Failed:             $BENCHMARKS_FAILED${NC}"
    echo ""
    echo "Output Directory: $WEIGHTS_OUTPUT"
    echo "Backup Directory: $WEIGHTS_OUTPUT/backup-$TIMESTAMP"
    echo ""

    if [ $BENCHMARKS_FAILED -eq 0 ]; then
        echo -e "${GREEN}✅ ALL BENCHMARKS COMPLETED SUCCESSFULLY${NC}"
        echo ""
        echo "Next Steps:"
        echo "  1. Review generated weights in: $WEIGHTS_OUTPUT"
        echo "  2. Follow integration instructions: $WEIGHTS_OUTPUT/INTEGRATION.md"
        echo "  3. Update AUDIT_PACKAGE.md to reflect production weights"
        echo "  4. Test transaction fees with real weights"
    else
        echo -e "${RED}⚠️  SOME BENCHMARKS FAILED${NC}"
        echo ""
        echo "Action Required:"
        echo "  1. Review failed benchmark logs in: $WEIGHTS_OUTPUT"
        echo "  2. Fix any missing extrinsics or runtime issues"
        echo "  3. Re-run benchmarks for failed pallets"
    fi

    echo ""
    echo "═══════════════════════════════════════════════════════════════"
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    clear

    echo -e "${PURPLE}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                                                              ║"
    echo "║         ËTRID PROTOCOL WEIGHT BENCHMARKING SUITE            ║"
    echo "║         Production-Ready Weight Generation                  ║"
    echo "║                                                              ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"

    # Pre-flight checks
    preflight_checks

    # Run benchmarks
    benchmark_flarechain_runtime
    benchmark_edsc_pallets
    benchmark_etwasm

    # Analyze changes
    analyze_weight_changes

    # Generate integration instructions
    generate_integration_instructions

    # Generate summary report
    generate_report

    # Exit code
    if [ $BENCHMARKS_FAILED -eq 0 ]; then
        exit 0
    else
        exit 1
    fi
}

# Run main
main
