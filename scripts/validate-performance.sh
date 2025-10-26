#!/bin/bash
# Ëtrid Protocol - Performance Validation Script
# Validates that all critical performance optimizations are in place

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

ETRID_ROOT="${ETRID_ROOT:-$(pwd)}"
CHECKS_PASSED=0
CHECKS_FAILED=0
CHECKS_WARNING=0

# Header
clear
echo -e "${PURPLE}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║     ËTRID PROTOCOL PERFORMANCE VALIDATION                   ║"
echo "║     Production Readiness Check                              ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

# Helper functions
check_pass() {
    echo -e "${GREEN}✓${NC} $1"
    CHECKS_PASSED=$((CHECKS_PASSED + 1))
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    CHECKS_FAILED=$((CHECKS_FAILED + 1))
}

check_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    CHECKS_WARNING=$((CHECKS_WARNING + 1))
}

section() {
    echo ""
    echo -e "${BLUE}━━━ $1 ━━━${NC}"
}

# ============================================================================
# 1. Build Artifacts
# ============================================================================

section "1. Build Artifacts"

# Check if node binary exists
if [ -f "$ETRID_ROOT/target/release/flarechain-node" ]; then
    check_pass "FlareChain node binary exists"

    # Check if it supports benchmarking
    if $ETRID_ROOT/target/release/flarechain-node benchmark pallet --help &>/dev/null; then
        check_pass "Node supports runtime benchmarking"
    else
        check_fail "Node does not support benchmarking (rebuild with --features runtime-benchmarks)"
    fi
else
    check_fail "FlareChain node binary not found (run: cargo build --release -p flarechain-node)"
fi

# Check runtime binary
if [ -f "$ETRID_ROOT/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm" ]; then
    check_pass "Runtime WASM binary exists"
else
    check_warn "Runtime WASM binary not found (expected after build)"
fi

# ============================================================================
# 2. Runtime Weights
# ============================================================================

section "2. Runtime Weights"

# Check if runtime-weights directory exists
if [ -d "$ETRID_ROOT/runtime-weights" ]; then
    weight_files=$(find "$ETRID_ROOT/runtime-weights" -name "*.rs" 2>/dev/null | wc -l)
    if [ $weight_files -gt 0 ]; then
        check_pass "Runtime weights generated ($weight_files pallet weight files)"

        # Check if INTEGRATION.md exists
        if [ -f "$ETRID_ROOT/runtime-weights/INTEGRATION.md" ]; then
            check_pass "Integration guide exists"
        else
            check_warn "Integration guide not found"
        fi

        # Check for placeholder weights
        placeholder_count=$(grep -r "Weight::from_parts(10_000" "$ETRID_ROOT/runtime-weights" 2>/dev/null | wc -l || echo "0")
        if [ $placeholder_count -eq 0 ]; then
            check_pass "No placeholder weights detected"
        else
            check_warn "Found $placeholder_count placeholder weights (should be production values)"
        fi
    else
        check_fail "No weight files generated (run: ./scripts/testnet/benchmark_weights.sh)"
    fi
else
    check_fail "runtime-weights directory not found (run benchmarks to create)"
fi

# Check if weights are integrated into runtime
if [ -d "$ETRID_ROOT/05-multichain/flare-chain/runtime/src/weights" ]; then
    integrated_weights=$(find "$ETRID_ROOT/05-multichain/flare-chain/runtime/src/weights" -name "*.rs" 2>/dev/null | wc -l)
    if [ $integrated_weights -gt 0 ]; then
        check_pass "Weights integrated into runtime ($integrated_weights files)"
    else
        check_warn "No weights integrated into runtime yet"
    fi
else
    check_warn "Runtime weights directory not created"
fi

# ============================================================================
# 3. Database Configuration
# ============================================================================

section "3. Database Configuration"

# Check if database config exists
if [ -f "$ETRID_ROOT/config/production/database.toml" ]; then
    check_pass "Database configuration file exists"

    # Check for key optimizations
    if grep -q "cache_size_mb.*[0-9][0-9][0-9][0-9]" "$ETRID_ROOT/config/production/database.toml"; then
        check_pass "Database cache configured (>1GB)"
    else
        check_warn "Database cache may not be optimized"
    fi

    if grep -q "compression_type.*=.*\"lz4\"" "$ETRID_ROOT/config/production/database.toml"; then
        check_pass "Database compression enabled"
    else
        check_warn "Database compression not configured"
    fi
else
    check_fail "Database configuration not found"
fi

# Check optimized startup scripts
if [ -f "$ETRID_ROOT/scripts/start-validator-optimized.sh" ]; then
    check_pass "Optimized validator startup script exists"

    if grep -q "db-cache.*[0-9][0-9][0-9][0-9]" "$ETRID_ROOT/scripts/start-validator-optimized.sh"; then
        check_pass "Validator script uses optimized cache settings"
    else
        check_warn "Validator script may not use optimized settings"
    fi
else
    check_fail "Optimized validator script not found"
fi

if [ -f "$ETRID_ROOT/scripts/start-archive-optimized.sh" ]; then
    check_pass "Optimized archive startup script exists"
else
    check_warn "Optimized archive script not found (optional)"
fi

# ============================================================================
# 4. Load Testing Infrastructure
# ============================================================================

section "4. Load Testing"

# Check if stress test harness exists
if [ -f "$ETRID_ROOT/scripts/testnet/stress_test_harness.sh" ]; then
    check_pass "Stress test harness exists"

    if [ -x "$ETRID_ROOT/scripts/testnet/stress_test_harness.sh" ]; then
        check_pass "Stress test harness is executable"
    else
        check_warn "Stress test harness not executable (run: chmod +x scripts/testnet/stress_test_harness.sh)"
    fi
else
    check_fail "Stress test harness not found"
fi

# Check if stress test results exist
if [ -d "$ETRID_ROOT/stress-test-results" ]; then
    result_count=$(find "$ETRID_ROOT/stress-test-results" -name "*.log" 2>/dev/null | wc -l)
    if [ $result_count -gt 0 ]; then
        check_pass "Stress test results found ($result_count test runs)"

        # Check latest results for TPS
        latest_result=$(ls -t "$ETRID_ROOT/stress-test-results"/*.log 2>/dev/null | head -1)
        if [ -f "$latest_result" ]; then
            if grep -q "Actual rate:.*[0-9][0-9][0-9][0-9]" "$latest_result"; then
                check_pass "Latest test achieved 1000+ TPS"
            else
                check_warn "Latest test may not have achieved target TPS"
            fi

            if grep -q "ALL.*TESTS.*PASSED" "$latest_result"; then
                check_pass "Latest stress test passed"
            else
                check_warn "Latest stress test may have failures"
            fi
        fi
    else
        check_warn "No stress test results found (run tests to generate)"
    fi
else
    check_warn "Stress test results directory not found (created on first test run)"
fi

# Check transaction submission tools
if command -v subxt &> /dev/null; then
    check_pass "subxt-cli installed for transaction submission"
elif command -v polkadot-js-api &> /dev/null; then
    check_pass "polkadot-js-api installed for transaction submission"
else
    check_warn "No transaction submission tool found (install subxt-cli or polkadot-js-api)"
fi

# ============================================================================
# 5. Profiling Tools
# ============================================================================

section "5. Profiling"

# Check for flamegraph
if command -v flamegraph &> /dev/null || cargo flamegraph --help &>/dev/null; then
    check_pass "cargo-flamegraph installed"
else
    check_warn "cargo-flamegraph not installed (run: cargo install flamegraph)"
fi

# Check for profiling results
flamegraph_count=$(find "$ETRID_ROOT" -maxdepth 1 -name "flamegraph*.svg" 2>/dev/null | wc -l)
if [ $flamegraph_count -gt 0 ]; then
    check_pass "Flamegraph profiling results found ($flamegraph_count files)"
else
    check_warn "No flamegraph results found (run profiling to generate)"
fi

# Check for heaptrack (optional)
if command -v heaptrack &> /dev/null; then
    check_pass "heaptrack installed for memory profiling"
else
    check_warn "heaptrack not installed (optional - run: brew install heaptrack)"
fi

# ============================================================================
# 6. Monitoring Infrastructure
# ============================================================================

section "6. Monitoring"

# Check Prometheus config
if [ -f "$ETRID_ROOT/scripts/testnet/prometheus.yml" ]; then
    check_pass "Prometheus configuration exists"
else
    check_warn "Prometheus configuration not found"
fi

# Check Grafana dashboard
if [ -f "$ETRID_ROOT/scripts/testnet/grafana-dashboard.json" ]; then
    check_pass "Grafana dashboard configuration exists"
else
    check_warn "Grafana dashboard not found"
fi

# Check if Prometheus is installed
if command -v prometheus &> /dev/null; then
    check_pass "Prometheus installed"
else
    check_warn "Prometheus not installed (run: brew install prometheus)"
fi

# Check if Grafana is installed
if command -v grafana-server &> /dev/null; then
    check_pass "Grafana installed"
else
    check_warn "Grafana not installed (run: brew install grafana)"
fi

# Check if Prometheus is running
if curl -s http://localhost:9090/-/healthy &>/dev/null; then
    check_pass "Prometheus is running"
else
    check_warn "Prometheus not running (start with: brew services start prometheus)"
fi

# Check if Grafana is running
if curl -s http://localhost:3000/api/health &>/dev/null; then
    check_pass "Grafana is running"
else
    check_warn "Grafana not running (start with: brew services start grafana)"
fi

# ============================================================================
# 7. Documentation
# ============================================================================

section "7. Documentation"

# Check for performance report
if [ -f "$ETRID_ROOT/PERFORMANCE_ANALYSIS_REPORT.md" ]; then
    check_pass "Performance analysis report exists"

    lines=$(wc -l < "$ETRID_ROOT/PERFORMANCE_ANALYSIS_REPORT.md")
    if [ $lines -gt 500 ]; then
        check_pass "Performance report is comprehensive ($lines lines)"
    else
        check_warn "Performance report may be incomplete"
    fi
else
    check_warn "Performance analysis report not found"
fi

# Check for quick start guide
if [ -f "$ETRID_ROOT/PERFORMANCE_QUICK_START.md" ]; then
    check_pass "Performance quick start guide exists"
else
    check_warn "Quick start guide not found"
fi

# ============================================================================
# Summary
# ============================================================================

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "                    VALIDATION SUMMARY                          "
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo -e "${GREEN}Passed:  $CHECKS_PASSED${NC}"
echo -e "${YELLOW}Warnings: $CHECKS_WARNING${NC}"
echo -e "${RED}Failed:  $CHECKS_FAILED${NC}"
echo ""

TOTAL_CHECKS=$((CHECKS_PASSED + CHECKS_WARNING + CHECKS_FAILED))
PASS_PERCENTAGE=$((CHECKS_PASSED * 100 / TOTAL_CHECKS))

echo "Pass Rate: $PASS_PERCENTAGE%"
echo ""

# Production readiness assessment
if [ $CHECKS_FAILED -eq 0 ] && [ $PASS_PERCENTAGE -ge 80 ]; then
    echo -e "${GREEN}✅ PRODUCTION READY${NC}"
    echo "All critical checks passed. System is ready for production deployment."
elif [ $CHECKS_FAILED -le 3 ] && [ $PASS_PERCENTAGE -ge 60 ]; then
    echo -e "${YELLOW}⚠️  NEEDS ATTENTION${NC}"
    echo "Some critical items missing. Address failures before production."
else
    echo -e "${RED}❌ NOT READY${NC}"
    echo "Critical issues detected. Complete missing items before proceeding."
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"

# Exit code
if [ $CHECKS_FAILED -eq 0 ]; then
    exit 0
else
    exit 1
fi
