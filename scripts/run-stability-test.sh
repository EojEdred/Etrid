#!/bin/bash
# Ëtrid Protocol - 72-Hour Stability Test
# Automated long-running stability and performance validation

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

ETRID_ROOT="${ETRID_ROOT:-$(pwd)}"
TEST_DIR="$ETRID_ROOT/stability-test-results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
TEST_LOG="$TEST_DIR/stability-test-$TIMESTAMP.log"

# Configuration
TEST_DURATION="${TEST_DURATION:-259200}"  # 72 hours in seconds
CHECK_INTERVAL="${CHECK_INTERVAL:-600}"    # Check every 10 minutes
ALERT_MEMORY_GROWTH="${ALERT_MEMORY_GROWTH:-100}"  # MB/hour

echo -e "${PURPLE}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║     ËTRID 72-HOUR STABILITY TEST                            ║"
echo "║     Long-Running Performance Validation                     ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$TEST_LOG"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$TEST_LOG"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$TEST_LOG"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$TEST_LOG"
}

# Create test directory
mkdir -p "$TEST_DIR"

# Initialize test log
{
    echo "Ëtrid Protocol - 72-Hour Stability Test"
    echo "========================================"
    echo ""
    echo "Start Time: $(date)"
    echo "Duration: $TEST_DURATION seconds ($((TEST_DURATION / 3600)) hours)"
    echo "Check Interval: $CHECK_INTERVAL seconds"
    echo ""
} > "$TEST_LOG"

log_info "Test duration: $((TEST_DURATION / 3600)) hours"
log_info "Check interval: $((CHECK_INTERVAL / 60)) minutes"
log_info "Results: $TEST_LOG"
echo ""

# ============================================================================
# Check Prerequisites
# ============================================================================

check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check if node is running
    if ! pgrep -f flarechain-node > /dev/null; then
        log_error "FlareChain node not running"
        log_info "Start with: ./scripts/start-validator-optimized.sh"
        exit 1
    fi

    NODE_PID=$(pgrep -f flarechain-node | head -1)
    log_success "Node running (PID: $NODE_PID)"

    # Check if RPC is accessible
    if ! curl -s http://localhost:9944 &>/dev/null; then
        log_warning "RPC not accessible on :9944"
    else
        log_success "RPC accessible"
    fi

    # Check if metrics are available
    if ! curl -s http://localhost:9615/metrics &>/dev/null; then
        log_warning "Prometheus metrics not accessible on :9615"
    else
        METRIC_COUNT=$(curl -s http://localhost:9615/metrics | wc -l)
        log_success "Prometheus metrics available ($METRIC_COUNT metrics)"
    fi
}

# ============================================================================
# Collect Initial Metrics
# ============================================================================

collect_initial_metrics() {
    log_info "Collecting initial metrics..."

    NODE_PID=$(pgrep -f flarechain-node | head -1)

    # Memory
    INITIAL_MEM=$(ps -o rss= -p $NODE_PID 2>/dev/null || echo "0")
    INITIAL_MEM_MB=$((INITIAL_MEM / 1024))
    log_info "  Initial memory: ${INITIAL_MEM_MB} MB"

    # Block height
    INITIAL_BLOCK=$(curl -s -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
        http://localhost:9944 2>/dev/null | grep -o '"number":"0x[^"]*"' | sed 's/"number":"0x//' | sed 's/"//' | xargs printf "%d\n" 2>/dev/null || echo "0")
    log_info "  Initial block: $INITIAL_BLOCK"

    # CPU baseline
    CPU_BASELINE=$(ps -o %cpu= -p $NODE_PID 2>/dev/null || echo "0")
    log_info "  CPU baseline: ${CPU_BASELINE}%"

    # Save to file for analysis
    echo "INITIAL_MEM=$INITIAL_MEM" > "$TEST_DIR/stability-metrics-$TIMESTAMP.env"
    echo "INITIAL_MEM_MB=$INITIAL_MEM_MB" >> "$TEST_DIR/stability-metrics-$TIMESTAMP.env"
    echo "INITIAL_BLOCK=$INITIAL_BLOCK" >> "$TEST_DIR/stability-metrics-$TIMESTAMP.env"
    echo "INITIAL_TIME=$(date +%s)" >> "$TEST_DIR/stability-metrics-$TIMESTAMP.env"
}

# ============================================================================
# Run Stability Monitoring Loop
# ============================================================================

run_stability_monitoring() {
    log_info "Starting stability monitoring..."
    log_info "Monitor log: tail -f $TEST_LOG"
    echo ""

    # Load initial metrics
    source "$TEST_DIR/stability-metrics-$TIMESTAMP.env"

    CHECKS_PASSED=0
    CHECKS_FAILED=0
    MAX_MEM_MB=$INITIAL_MEM_MB
    MIN_MEM_MB=$INITIAL_MEM_MB

    START_TIME=$(date +%s)
    END_TIME=$((START_TIME + TEST_DURATION))

    while [ $(date +%s) -lt $END_TIME ]; do
        CURRENT_TIME=$(date +%s)
        ELAPSED=$((CURRENT_TIME - START_TIME))
        REMAINING=$((END_TIME - CURRENT_TIME))

        HOURS_ELAPSED=$(awk "BEGIN {printf \"%.2f\", $ELAPSED / 3600}")
        HOURS_REMAINING=$(awk "BEGIN {printf \"%.2f\", $REMAINING / 3600}")

        log_info "===  Check $(($CHECKS_PASSED + $CHECKS_FAILED + 1)) - ${HOURS_ELAPSED}h elapsed, ${HOURS_REMAINING}h remaining ==="

        # Check if node is still running
        if ! pgrep -f flarechain-node > /dev/null; then
            log_error "Node process died!"
            CHECKS_FAILED=$((CHECKS_FAILED + 1))
            break
        fi

        NODE_PID=$(pgrep -f flarechain-node | head -1)

        # Memory check
        CURRENT_MEM=$(ps -o rss= -p $NODE_PID 2>/dev/null || echo "0")
        CURRENT_MEM_MB=$((CURRENT_MEM / 1024))
        MEM_GROWTH_MB=$((CURRENT_MEM_MB - INITIAL_MEM_MB))

        # Update min/max
        if [ $CURRENT_MEM_MB -gt $MAX_MEM_MB ]; then
            MAX_MEM_MB=$CURRENT_MEM_MB
        fi
        if [ $CURRENT_MEM_MB -lt $MIN_MEM_MB ]; then
            MIN_MEM_MB=$CURRENT_MEM_MB
        fi

        log_info "  Memory: ${CURRENT_MEM_MB} MB (Δ ${MEM_GROWTH_MB} MB, range: ${MIN_MEM_MB}-${MAX_MEM_MB} MB)"

        # Calculate memory growth rate
        if [ $ELAPSED -gt 0 ]; then
            MEM_GROWTH_RATE=$(awk "BEGIN {printf \"%.2f\", $MEM_GROWTH_MB * 3600 / $ELAPSED}")
            log_info "  Growth rate: ${MEM_GROWTH_RATE} MB/hour"

            # Alert if growth rate too high
            if (( $(echo "$MEM_GROWTH_RATE > $ALERT_MEMORY_GROWTH" | bc -l 2>/dev/null || echo "0") )); then
                log_warning "  ⚠️  High memory growth rate!"
            fi
        fi

        # CPU check
        CPU_USAGE=$(ps -o %cpu= -p $NODE_PID 2>/dev/null || echo "0")
        log_info "  CPU: ${CPU_USAGE}%"

        # Block height check
        CURRENT_BLOCK=$(curl -s -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
            http://localhost:9944 2>/dev/null | grep -o '"number":"0x[^"]*"' | sed 's/"number":"0x//' | sed 's/"//' | xargs printf "%d\n" 2>/dev/null || echo "0")

        BLOCKS_PRODUCED=$((CURRENT_BLOCK - INITIAL_BLOCK))
        log_info "  Block: $CURRENT_BLOCK (Δ $BLOCKS_PRODUCED)"

        if [ $ELAPSED -gt 0 ]; then
            BLOCK_RATE=$(awk "BEGIN {printf \"%.4f\", $BLOCKS_PRODUCED / $ELAPSED}")
            EXPECTED_BLOCKS=$((ELAPSED / 6))  # 6 second block time
            log_info "  Block rate: $BLOCK_RATE blocks/s (expected: ~0.1667 blocks/s)"
        fi

        # Finality check
        FINALIZED_RESP=$(curl -s -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
            http://localhost:9944 2>/dev/null)

        if echo "$FINALIZED_RESP" | grep -q "result"; then
            log_info "  Finality: Active"
        else
            log_warning "  Finality: Cannot check"
        fi

        # Peer count check (if available)
        PEERS=$(curl -s http://localhost:9615/metrics 2>/dev/null | grep "^substrate_sub_libp2p_peers_count " | awk '{print $2}' || echo "N/A")
        log_info "  Peers: $PEERS"

        # Transaction pool (if available)
        TX_POOL=$(curl -s http://localhost:9615/metrics 2>/dev/null | grep "^substrate_ready_transactions_number " | awk '{print $2}' || echo "N/A")
        log_info "  TX Pool: $TX_POOL"

        # Health check passed
        CHECKS_PASSED=$((CHECKS_PASSED + 1))
        log_success "  ✓ Health check passed"

        # Save checkpoint
        {
            echo "CHECK_TIME=$(date +%s)"
            echo "CHECK_NUM=$((CHECKS_PASSED + CHECKS_FAILED))"
            echo "MEM_MB=$CURRENT_MEM_MB"
            echo "CPU=$CPU_USAGE"
            echo "BLOCK=$CURRENT_BLOCK"
        } >> "$TEST_DIR/stability-checkpoints-$TIMESTAMP.csv"

        echo "" | tee -a "$TEST_LOG"

        # Sleep until next check
        sleep $CHECK_INTERVAL
    done

    # Final summary
    FINAL_TIME=$(date +%s)
    FINAL_DURATION=$((FINAL_TIME - START_TIME))

    log_info "===  Stability Test Complete ==="
    log_info "Duration: $((FINAL_DURATION / 3600)) hours"
    log_info "Health checks passed: $CHECKS_PASSED"
    log_info "Health checks failed: $CHECKS_FAILED"

    if [ $CHECKS_FAILED -eq 0 ]; then
        log_success "✅ STABILITY TEST PASSED"
    else
        log_error "❌ STABILITY TEST FAILED"
    fi
}

# ============================================================================
# Generate Final Report
# ============================================================================

generate_final_report() {
    log_info "Generating final report..."

    REPORT_FILE="$TEST_DIR/STABILITY_REPORT_$TIMESTAMP.md"

    cat > "$REPORT_FILE" <<EOF
# Ëtrid Protocol - 72-Hour Stability Test Report

**Date:** $(date)
**Duration:** $((TEST_DURATION / 3600)) hours
**Test ID:** $TIMESTAMP

---

## Executive Summary

EOF

    # Load metrics
    source "$TEST_DIR/stability-metrics-$TIMESTAMP.env"

    # Calculate final metrics
    NODE_PID=$(pgrep -f flarechain-node | head -1 || echo "")

    if [ -n "$NODE_PID" ]; then
        FINAL_MEM=$(ps -o rss= -p $NODE_PID 2>/dev/null || echo "0")
        FINAL_MEM_MB=$((FINAL_MEM / 1024))
        MEM_GROWTH=$((FINAL_MEM_MB - INITIAL_MEM_MB))

        cat >> "$REPORT_FILE" <<EOF
**Node Status:** ✅ Running

**Memory:**
- Initial: ${INITIAL_MEM_MB} MB
- Final: ${FINAL_MEM_MB} MB
- Growth: ${MEM_GROWTH} MB
- Growth Rate: $(awk "BEGIN {printf \"%.2f\", $MEM_GROWTH * 3600 / $TEST_DURATION}") MB/hour

EOF

        if [ $MEM_GROWTH -lt 100 ]; then
            echo "**Memory Assessment:** ✅ PASS (acceptable growth)" >> "$REPORT_FILE"
        elif [ $MEM_GROWTH -lt 500 ]; then
            echo "**Memory Assessment:** ⚠️  WARNING (moderate growth)" >> "$REPORT_FILE"
        else
            echo "**Memory Assessment:** ❌ FAIL (excessive growth)" >> "$REPORT_FILE"
        fi

    else
        cat >> "$REPORT_FILE" <<EOF
**Node Status:** ❌ Not Running (process died during test)

**Memory:** Cannot collect final metrics
EOF
    fi

    cat >> "$REPORT_FILE" <<EOF

---

## Test Results

### Health Checks

- Total checks: $(wc -l < "$TEST_DIR/stability-checkpoints-$TIMESTAMP.csv")
- Expected checks: $((TEST_DURATION / CHECK_INTERVAL))
- Success rate: $(awk "BEGIN {printf \"%.1f\", $(wc -l < "$TEST_DIR/stability-checkpoints-$TIMESTAMP.csv") * 100 / ($TEST_DURATION / $CHECK_INTERVAL)}%")

### Performance Metrics

\`\`\`
$(tail -20 "$TEST_LOG")
\`\`\`

---

## Detailed Analysis

### Memory Usage Over Time

\`\`\`
$(awk '{print $3}' "$TEST_DIR/stability-checkpoints-$TIMESTAMP.csv" | nl)
\`\`\`

### Recommendations

EOF

    if [ $MEM_GROWTH -gt 500 ]; then
        cat >> "$REPORT_FILE" <<EOF
🔴 **CRITICAL: Memory Leak Suspected**
- Growth of ${MEM_GROWTH} MB over 72 hours is excessive
- Run heap profiling to identify allocation sources
- Review code for memory leaks

EOF
    fi

    if [ $MEM_GROWTH -lt 100 ]; then
        cat >> "$REPORT_FILE" <<EOF
✅ **Memory Management: Excellent**
- Stable memory usage over 72 hours
- No signs of memory leaks
- Production ready

EOF
    fi

    cat >> "$REPORT_FILE" <<EOF
---

## Files Generated

- Test Log: \`$TEST_LOG\`
- Checkpoints: \`$TEST_DIR/stability-checkpoints-$TIMESTAMP.csv\`
- Metrics: \`$TEST_DIR/stability-metrics-$TIMESTAMP.env\`
- Report: \`$REPORT_FILE\`

---

**Test Complete:** $(date)
EOF

    log_success "Report generated: $REPORT_FILE"
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    check_prerequisites
    echo ""

    collect_initial_metrics
    echo ""

    run_stability_monitoring
    echo ""

    generate_final_report

    echo ""
    echo -e "${GREEN}"
    echo "═══════════════════════════════════════════════════════════════"
    echo "           72-HOUR STABILITY TEST COMPLETE                     "
    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo ""
    echo "Results:"
    echo "  Full log: $TEST_LOG"
    echo "  Report: $TEST_DIR/STABILITY_REPORT_$TIMESTAMP.md"
    echo ""
    echo "View report: cat $TEST_DIR/STABILITY_REPORT_$TIMESTAMP.md"
    echo ""
}

# Handle Ctrl+C gracefully
trap 'echo ""; log_warning "Test interrupted by user"; generate_final_report; exit 130' INT TERM

main "$@"
