#!/bin/bash
# Ëtrid Protocol - Automated Profiling Suite
# CPU, Memory, and Performance Profiling Automation

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

ETRID_ROOT="${ETRID_ROOT:-$(pwd)}"
PROFILE_DIR="$ETRID_ROOT/profiling-results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo -e "${PURPLE}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║     ËTRID PROTOCOL PROFILING SUITE                          ║"
echo "║     CPU, Memory & Performance Analysis                      ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Configuration
PROFILE_DURATION="${PROFILE_DURATION:-60}"  # seconds
PROFILE_MODE="${PROFILE_MODE:-all}"         # all, cpu, memory, baseline

# Create output directory
mkdir -p "$PROFILE_DIR"

# ============================================================================
# Check Dependencies
# ============================================================================

check_dependencies() {
    log_info "Checking dependencies..."

    MISSING_DEPS=()

    # Check for cargo-flamegraph
    if ! cargo flamegraph --help &>/dev/null; then
        MISSING_DEPS+=("cargo-flamegraph")
    fi

    # Check for perf (Linux)
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if ! command -v perf &> /dev/null; then
            log_warning "perf not found (optional for Linux profiling)"
        fi
    fi

    # Check for heaptrack (optional)
    if ! command -v heaptrack &> /dev/null; then
        log_warning "heaptrack not found (optional for heap profiling)"
    fi

    # Check for valgrind (optional)
    if ! command -v valgrind &> /dev/null; then
        log_warning "valgrind not found (optional for memory leak detection)"
    fi

    if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
        log_error "Missing required dependencies: ${MISSING_DEPS[*]}"
        log_info "Install with:"
        for dep in "${MISSING_DEPS[@]}"; do
            case $dep in
                cargo-flamegraph)
                    echo "  cargo install flamegraph"
                    ;;
            esac
        done
        exit 1
    fi

    log_success "All required dependencies found"
}

# ============================================================================
# CPU Profiling with Flamegraph
# ============================================================================

profile_cpu_flamegraph() {
    log_info "Running CPU profiling with flamegraph..."
    log_info "  Duration: ${PROFILE_DURATION}s"

    OUTPUT_FILE="$PROFILE_DIR/flamegraph-cpu-$TIMESTAMP.svg"

    log_info "Starting FlareChain node with profiling..."

    # Profile the node binary
    timeout $PROFILE_DURATION cargo flamegraph \
        --bin flarechain-node \
        --output "$OUTPUT_FILE" \
        -- --dev --tmp --rpc-cors all 2>&1 | tee "$PROFILE_DIR/flamegraph-cpu-$TIMESTAMP.log" &

    PROFILE_PID=$!

    log_info "  Profiling PID: $PROFILE_PID"
    log_info "  Collecting samples for ${PROFILE_DURATION}s..."

    # Wait for profiling to complete
    wait $PROFILE_PID || true

    if [ -f "$OUTPUT_FILE" ]; then
        FILE_SIZE=$(ls -lh "$OUTPUT_FILE" | awk '{print $5}')
        log_success "CPU flamegraph created: $OUTPUT_FILE ($FILE_SIZE)"
        log_info "  View with: open $OUTPUT_FILE"

        # Analyze flamegraph for hotspots
        log_info "Analyzing CPU hotspots..."
        analyze_flamegraph "$OUTPUT_FILE"
    else
        log_error "Flamegraph generation failed"
        return 1
    fi
}

# ============================================================================
# Analyze Flamegraph
# ============================================================================

analyze_flamegraph() {
    local flamegraph="$1"

    # Extract function names and estimate % time
    # (This is a simple heuristic - actual % would need perf data parsing)

    log_info "Top potential hotspots:"
    log_info "  (Functions that may consume significant CPU time)"

    # Common hotspots in blockchain nodes
    cat <<EOF | tee "$PROFILE_DIR/flamegraph-analysis-$TIMESTAMP.txt"

Common Hotspot Categories:
  1. Block Import Pipeline
     - look for: import_block, execute_block, apply_extrinsic
  2. State Transitions
     - look for: state::*, storage::*, trie::*
  3. Consensus
     - look for: consensus::*, asf::*, finality::*
  4. Transaction Validation
     - look for: validate::*, check_transaction::*
  5. Networking
     - look for: network::*, p2p::*, libp2p::*
  6. WASM Execution
     - look for: wasmi::*, runtime::*, execute_call::*

Action Items:
  1. Open flamegraph: $flamegraph
  2. Look for wide bars (high % time)
  3. Identify functions consuming >5% CPU
  4. Focus optimization on those areas

EOF

    log_success "Analysis saved to: $PROFILE_DIR/flamegraph-analysis-$TIMESTAMP.txt"
}

# ============================================================================
# Memory Profiling with Heaptrack
# ============================================================================

profile_memory_heaptrack() {
    if ! command -v heaptrack &> /dev/null; then
        log_warning "heaptrack not installed, skipping memory profiling"
        log_info "  Install with: brew install heaptrack (macOS) or apt install heaptrack (Linux)"
        return 0
    fi

    log_info "Running memory profiling with heaptrack..."
    log_info "  Duration: ${PROFILE_DURATION}s"

    cd "$PROFILE_DIR"

    # Run heaptrack
    heaptrack --output heaptrack-$TIMESTAMP \
        "$ETRID_ROOT/target/release/flarechain-node" --dev --tmp &

    HEAP_PID=$!

    log_info "  Heaptrack PID: $HEAP_PID"
    log_info "  Collecting memory allocations for ${PROFILE_DURATION}s..."

    sleep $PROFILE_DURATION

    # Stop node gracefully
    kill -TERM $HEAP_PID 2>/dev/null || true
    sleep 5
    kill -KILL $HEAP_PID 2>/dev/null || true

    # Find heaptrack output
    HEAP_FILE=$(ls -t heaptrack.flarechain-node.*.gz 2>/dev/null | head -1)

    if [ -n "$HEAP_FILE" ]; then
        log_success "Memory profile created: $HEAP_FILE"
        log_info "  Analyze with: heaptrack_gui $HEAP_FILE"

        # Generate text report
        heaptrack --analyze "$HEAP_FILE" > "heaptrack-analysis-$TIMESTAMP.txt" 2>&1 || true
        log_success "Text analysis: heaptrack-analysis-$TIMESTAMP.txt"
    else
        log_error "Heaptrack profiling failed"
    fi

    cd "$ETRID_ROOT"
}

# ============================================================================
# Memory Leak Detection with Valgrind
# ============================================================================

profile_memory_valgrind() {
    if ! command -v valgrind &> /dev/null; then
        log_warning "valgrind not installed, skipping leak detection"
        return 0
    fi

    log_info "Running memory leak detection with valgrind..."
    log_warning "This is SLOW - will run for ${PROFILE_DURATION}s"

    OUTPUT_FILE="$PROFILE_DIR/valgrind-memcheck-$TIMESTAMP.log"

    timeout $PROFILE_DURATION valgrind \
        --leak-check=full \
        --show-leak-kinds=all \
        --track-origins=yes \
        --verbose \
        --log-file="$OUTPUT_FILE" \
        "$ETRID_ROOT/target/release/flarechain-node" --dev --tmp &

    VALGRIND_PID=$!

    log_info "  Valgrind PID: $VALGRIND_PID"

    wait $VALGRIND_PID || true

    if [ -f "$OUTPUT_FILE" ]; then
        log_success "Valgrind report created: $OUTPUT_FILE"

        # Check for leaks
        LEAKS=$(grep "definitely lost:" "$OUTPUT_FILE" || echo "0 bytes")
        log_info "  Memory leaks: $LEAKS"

        if grep -q "0 bytes in 0 blocks" "$OUTPUT_FILE"; then
            log_success "No memory leaks detected!"
        else
            log_warning "Potential memory leaks found - review $OUTPUT_FILE"
        fi
    fi
}

# ============================================================================
# Baseline Performance Test
# ============================================================================

run_baseline_performance() {
    log_info "Running baseline performance test..."

    OUTPUT_FILE="$PROFILE_DIR/baseline-performance-$TIMESTAMP.log"

    {
        echo "Ëtrid Protocol - Baseline Performance Test"
        echo "==========================================="
        echo "Timestamp: $(date)"
        echo "Duration: ${PROFILE_DURATION}s"
        echo ""

        # Start node and collect metrics
        "$ETRID_ROOT/target/release/flarechain-node" --dev --tmp &
        NODE_PID=$!

        log_info "  Node PID: $NODE_PID"

        sleep 10  # Wait for startup

        echo "=== Initial Metrics ==="
        INITIAL_MEM=$(ps -o rss= -p $NODE_PID 2>/dev/null || echo "0")
        INITIAL_TIME=$(date +%s)
        echo "Memory: $((INITIAL_MEM / 1024)) MB"
        echo ""

        # Monitor for duration
        for i in $(seq 1 $((PROFILE_DURATION / 10))); do
            sleep 10

            CURRENT_MEM=$(ps -o rss= -p $NODE_PID 2>/dev/null || echo "0")
            CURRENT_MEM_MB=$((CURRENT_MEM / 1024))

            echo "[$((i * 10))s] Memory: ${CURRENT_MEM_MB} MB"

            # Get block height if node is running
            if curl -s http://localhost:9944 &>/dev/null; then
                BLOCK_HEIGHT=$(curl -s -H "Content-Type: application/json" \
                    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
                    http://localhost:9944 2>/dev/null | grep -o '"number":"0x[^"]*"' | head -1 || echo "")

                if [ -n "$BLOCK_HEIGHT" ]; then
                    echo "         Block: $BLOCK_HEIGHT"
                fi
            fi
        done

        echo ""
        echo "=== Final Metrics ==="
        FINAL_MEM=$(ps -o rss= -p $NODE_PID 2>/dev/null || echo "0")
        FINAL_TIME=$(date +%s)
        FINAL_MEM_MB=$((FINAL_MEM / 1024))
        DURATION=$((FINAL_TIME - INITIAL_TIME))

        echo "Memory: ${FINAL_MEM_MB} MB"
        echo "Memory growth: $((FINAL_MEM - INITIAL_MEM)) KB ($((FINAL_MEM_MB - INITIAL_MEM / 1024)) MB)"
        echo "Duration: ${DURATION}s"
        echo ""

        # Calculate memory growth rate
        if [ $DURATION -gt 0 ]; then
            GROWTH_RATE=$(( (FINAL_MEM - INITIAL_MEM) * 3600 / DURATION / 1024 ))
            echo "Growth rate: ${GROWTH_RATE} MB/hour"

            if [ $GROWTH_RATE -lt 100 ]; then
                echo "Status: ✓ PASS (acceptable growth)"
            else
                echo "Status: ⚠ WARNING (high growth rate)"
            fi
        fi

        # Stop node
        kill -TERM $NODE_PID 2>/dev/null || true
        sleep 2
        kill -KILL $NODE_PID 2>/dev/null || true

    } | tee "$OUTPUT_FILE"

    log_success "Baseline performance logged: $OUTPUT_FILE"
}

# ============================================================================
# Generate Profiling Report
# ============================================================================

generate_report() {
    log_info "Generating profiling summary report..."

    REPORT_FILE="$PROFILE_DIR/PROFILING_REPORT_$TIMESTAMP.md"

    cat > "$REPORT_FILE" <<EOF
# Ëtrid Protocol - Profiling Report

**Date:** $(date)
**Duration:** ${PROFILE_DURATION}s
**Mode:** $PROFILE_MODE

---

## Profiling Results

### Files Generated

EOF

    # List all generated files
    find "$PROFILE_DIR" -name "*$TIMESTAMP*" -type f | while read file; do
        FILENAME=$(basename "$file")
        FILESIZE=$(ls -lh "$file" | awk '{print $5}')
        echo "- \`$FILENAME\` ($FILESIZE)" >> "$REPORT_FILE"
    done

    cat >> "$REPORT_FILE" <<EOF

---

## Analysis Summary

### CPU Profiling

EOF

    if [ -f "$PROFILE_DIR/flamegraph-cpu-$TIMESTAMP.svg" ]; then
        echo "✅ **Flamegraph generated**" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "**View:** \`open profiling-results/flamegraph-cpu-$TIMESTAMP.svg\`" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "**Key areas to examine:**" >> "$REPORT_FILE"
        echo "1. Functions with >5% CPU time" >> "$REPORT_FILE"
        echo "2. Wide bars in flamegraph" >> "$REPORT_FILE"
        echo "3. Block import pipeline" >> "$REPORT_FILE"
        echo "4. State transitions" >> "$REPORT_FILE"
        echo "5. Consensus operations" >> "$REPORT_FILE"
    else
        echo "❌ CPU profiling not performed" >> "$REPORT_FILE"
    fi

    cat >> "$REPORT_FILE" <<EOF

### Memory Profiling

EOF

    if ls "$PROFILE_DIR"/heaptrack*.gz &>/dev/null; then
        echo "✅ **Heap analysis complete**" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "**View:** \`heaptrack_gui profiling-results/heaptrack*.gz\`" >> "$REPORT_FILE"
    else
        echo "⚠️  Heap profiling not performed (heaptrack not available)" >> "$REPORT_FILE"
    fi

    echo "" >> "$REPORT_FILE"

    if [ -f "$PROFILE_DIR/valgrind-memcheck-$TIMESTAMP.log" ]; then
        echo "✅ **Memory leak check complete**" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        LEAKS=$(grep "definitely lost:" "$PROFILE_DIR/valgrind-memcheck-$TIMESTAMP.log" | head -1)
        echo "**Result:** $LEAKS" >> "$REPORT_FILE"
    else
        echo "⚠️  Memory leak detection not performed (valgrind not available)" >> "$REPORT_FILE"
    fi

    cat >> "$REPORT_FILE" <<EOF

### Baseline Performance

EOF

    if [ -f "$PROFILE_DIR/baseline-performance-$TIMESTAMP.log" ]; then
        echo "✅ **Baseline test complete**" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "\`\`\`" >> "$REPORT_FILE"
        tail -10 "$PROFILE_DIR/baseline-performance-$TIMESTAMP.log" >> "$REPORT_FILE"
        echo "\`\`\`" >> "$REPORT_FILE"
    else
        echo "⚠️  Baseline test not performed" >> "$REPORT_FILE"
    fi

    cat >> "$REPORT_FILE" <<EOF

---

## Recommendations

### Immediate Actions

1. **Review CPU Flamegraph**
   - Identify functions consuming >5% CPU time
   - Focus optimization efforts on hot paths
   - Look for unnecessary allocations

2. **Check Memory Growth**
   - Baseline test shows memory growth rate
   - Target: <50 MB/hour growth
   - Investigate if growth >100 MB/hour

3. **Analyze Allocations**
   - Use heaptrack to find allocation hotspots
   - Look for large allocations
   - Check for memory pooling opportunities

### Optimization Targets

Based on common blockchain node patterns:

1. **Block Import** - Often 20-30% of CPU time
   - Optimize state transitions
   - Batch storage operations
   - Parallelize where possible

2. **Transaction Validation** - Often 15-25% of CPU time
   - Cache validation results
   - Optimize signature verification
   - Use batch verification

3. **Networking** - Often 10-20% of CPU time
   - Optimize message serialization
   - Use connection pooling
   - Implement message batching

4. **Storage I/O** - Critical for performance
   - Optimize RocksDB configuration
   - Batch write operations
   - Use appropriate cache sizes

---

## Next Steps

1. Review all generated profiling files
2. Identify top 3 optimization opportunities
3. Implement optimizations
4. Re-run profiling to measure improvements
5. Compare before/after flamegraphs

---

**Generated by:** Ëtrid Protocol Profiling Suite
**Location:** $PROFILE_DIR
EOF

    log_success "Profiling report generated: $REPORT_FILE"
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    log_info "Profile mode: $PROFILE_MODE"
    log_info "Profile duration: ${PROFILE_DURATION}s"
    log_info "Output directory: $PROFILE_DIR"
    echo ""

    check_dependencies
    echo ""

    case $PROFILE_MODE in
        cpu)
            profile_cpu_flamegraph
            ;;
        memory)
            profile_memory_heaptrack
            profile_memory_valgrind
            ;;
        baseline)
            run_baseline_performance
            ;;
        all)
            profile_cpu_flamegraph
            echo ""
            run_baseline_performance
            echo ""
            profile_memory_heaptrack
            ;;
        *)
            log_error "Unknown profile mode: $PROFILE_MODE"
            log_info "Valid modes: all, cpu, memory, baseline"
            exit 1
            ;;
    esac

    echo ""
    generate_report

    echo ""
    echo -e "${GREEN}"
    echo "═══════════════════════════════════════════════════════════════"
    echo "              PROFILING SUITE COMPLETE                         "
    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo ""
    echo "Results saved to: $PROFILE_DIR"
    echo "Summary report: $REPORT_FILE"
    echo ""
    echo "Quick commands:"
    echo "  View flamegraph:  open $PROFILE_DIR/flamegraph-cpu-$TIMESTAMP.svg"
    echo "  View report:      cat $REPORT_FILE"
    echo "  List results:     ls -lh $PROFILE_DIR"
    echo ""
}

main "$@"
