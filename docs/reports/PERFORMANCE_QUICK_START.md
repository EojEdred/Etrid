# Performance Optimization Quick Start Guide

**Last Updated:** October 22, 2025
**Purpose:** Execute critical performance optimization tasks
**Estimated Time:** 4-8 hours for critical items

---

## Prerequisites

```bash
# Ensure you're in the Etrid root directory
cd /Users/macbook/Desktop/etrid

# Check Rust toolchain
rustc --version  # Should be 1.70+

# Install required tools
cargo install cargo-flamegraph  # For profiling
cargo install subxt-cli          # For transaction submission

# macOS: Install profiling tools
brew install valgrind heaptrack

# Verify node configuration
ls -la 05-multichain/flare-chain/node/
```

---

## Part 1: Generate Runtime Weights (CRITICAL - 2-4 hours)

### Step 1: Build Node with Benchmarks

```bash
# Clean previous builds (optional but recommended)
cargo clean -p flarechain-node

# Build with runtime-benchmarks feature
# This will take 30-60 minutes on first build
cargo build --release --features runtime-benchmarks -p flarechain-node

# Verify the binary supports benchmarking
./target/release/flarechain-node benchmark pallet --help
```

**Expected Output:**
```
Benchmark the extrinsic weight of FRAME pallets

Usage: flarechain-node benchmark pallet [OPTIONS]

Options:
  --chain <CHAIN>
  --pallet <PALLET>
  --extrinsic <EXTRINSIC>
  ...
```

### Step 2: Run Benchmark Suite

```bash
# Make benchmark script executable
chmod +x scripts/testnet/benchmark_weights.sh

# Set environment variables (optional)
export ETRID_ROOT=/Users/macbook/Desktop/etrid
export BENCHMARK_STEPS=50     # Default: 50
export BENCHMARK_REPEAT=20    # Default: 20

# Run the complete benchmark suite
./scripts/testnet/benchmark_weights.sh
```

**What This Does:**
- Benchmarks all pallets (validator-committee, reserve-vault, etc.)
- Generates production-ready weight files
- Creates integration instructions
- Takes 1-2 hours depending on hardware

**Expected Output:**
```
╔══════════════════════════════════════════════════════════════╗
║         ËTRID PROTOCOL WEIGHT BENCHMARKING SUITE            ║
╚══════════════════════════════════════════════════════════════╝

[INFO] Binary supports benchmarking
[INFO] Output directory: runtime-weights

[TEST] Benchmarking: pallet-validator-committee
[INFO] Running benchmark...
[SUCCESS] Benchmark completed in 243s
[INFO]   Output: runtime-weights/validator_committee.rs
[INFO]   Extrinsics benchmarked: 8

...

═══════════════════════════════════════════════════════════════
✅ ALL BENCHMARKS COMPLETED SUCCESSFULLY
═══════════════════════════════════════════════════════════════
```

### Step 3: Verify Generated Weights

```bash
# Check generated weight files
ls -lh runtime-weights/

# Expected files:
# - validator_committee.rs
# - reserve_vault.rs
# - reserve_oracle.rs
# - did_registry.rs
# - circuit_breaker.rs
# - custodian_registry.rs
# - xcm_bridge.rs
# - aidid.rs
# - INTEGRATION.md

# Inspect a weight file
head -50 runtime-weights/validator_committee.rs

# Should see Weight::from_parts(...) not placeholder values
grep -c "Weight::from_parts" runtime-weights/validator_committee.rs
```

### Step 4: Integrate Weights into Runtime

```bash
# Create weights directory in runtime
mkdir -p 05-multichain/flare-chain/runtime/src/weights

# Copy generated weights
cp runtime-weights/*.rs 05-multichain/flare-chain/runtime/src/weights/

# Update runtime to use new weights
# (Manual step - follow runtime-weights/INTEGRATION.md)
```

**Validation:**
```bash
# Rebuild runtime with new weights
cargo build --release -p flare-chain-runtime

# Run tests to ensure weights are correct
cargo test -p pallet-validator-committee
cargo test -p pallet-reserve-vault

# Verify no placeholder weights remain
grep -r "Weight::from_parts(10_000" 05-multichain/flare-chain/runtime/src/weights/
# Should return no results
```

---

## Part 2: Database Optimization (CRITICAL - 2 hours)

### Step 1: Create Database Configuration

We'll create a production-ready database config:

```bash
# Create config directory
mkdir -p config/production

# The config file is created below
```

**File:** `config/production/database.toml`
```toml
[database]
# Database backend
type = "rocksdb"
path = "chains/flarechain/db"

# Cache configuration (adjust based on available RAM)
# Validator: 2-4GB, Archive: 8-16GB, RPC: 4-8GB
cache_size_mb = 4096

[database.rocksdb]
# Write buffer settings
write_buffer_size_mb = 64
max_write_buffer_number = 3
min_write_buffer_number_to_merge = 1

# Block cache for reads
block_cache_size_mb = 2048

# Compression (saves ~30% disk space)
compression_type = "lz4"           # Fast compression for recent data
bottommost_compression_type = "zstd"  # Better compression for old data

# Compaction settings
max_background_jobs = 4
level0_file_num_compaction_trigger = 4
level0_slowdown_writes_trigger = 20
level0_stop_writes_trigger = 36

# Bloom filters (improves read performance)
bloom_filter_bits_per_key = 10
use_ribbon_filter = true

# Performance tuning
enable_pipelined_write = true
allow_concurrent_memtable_write = true
```

### Step 2: Create Node Startup Scripts

**File:** `scripts/start-validator-optimized.sh`
```bash
#!/bin/bash
# Optimized validator node startup

./target/release/flarechain-node \
  --chain flare-mainnet \
  --name "etrid-validator-01" \
  --validator \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  \
  # Database optimization
  --db-cache 4096 \
  --pruning 256 \
  --state-cache-size 1073741824 \
  \
  # Network optimization
  --max-parallel-downloads 8 \
  --in-peers 25 \
  --out-peers 25 \
  --kademlia-disjoint-query-paths \
  \
  # Performance
  --wasm-execution compiled \
  --execution native-else-wasm \
  \
  # Monitoring
  --prometheus-external \
  --prometheus-port 9615
```

**File:** `scripts/start-archive-optimized.sh`
```bash
#!/bin/bash
# Optimized archive node startup

./target/release/flarechain-node \
  --chain flare-mainnet \
  --name "etrid-archive-01" \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  --rpc-cors all \
  \
  # Archive configuration
  --pruning archive \
  --db-cache 8192 \
  --state-cache-size 4294967296 \
  \
  # Network
  --max-parallel-downloads 16 \
  --in-peers 50 \
  --out-peers 50 \
  \
  # RPC limits
  --rpc-max-connections 1000 \
  --rpc-max-request-size 15 \
  --rpc-max-response-size 15 \
  \
  # Performance
  --wasm-execution compiled \
  --execution native-else-wasm \
  \
  # Monitoring
  --prometheus-external \
  --prometheus-port 9615
```

### Step 3: Performance Validation

```bash
# Start optimized node
./scripts/start-validator-optimized.sh &

# Wait for startup (30 seconds)
sleep 30

# Check RocksDB stats
curl -s http://localhost:9615/metrics | grep rocksdb

# Expected metrics:
# substrate_state_cache_bytes (should be ~1GB)
# substrate_state_db_cache_bytes (should be ~2GB)
# rocksdb_block_cache_hit_rate (should be >0.8)

# Monitor database performance
watch -n 5 'curl -s http://localhost:9615/metrics | grep -E "rocksdb_(read|write)_latency"'
```

---

## Part 3: Load Testing (CRITICAL - 2-4 hours)

### Step 1: Install Transaction Tools

```bash
# Option 1: Rust-based (recommended)
cargo install subxt-cli

# Option 2: JavaScript-based
npm install -g @polkadot/api-cli

# Verify installation
subxt --version
# or
polkadot-js-api --version
```

### Step 2: Start Testnet

```bash
# Option A: Use existing testnet script
./scripts/start-testnet.sh

# Option B: Manual dev node
./target/release/flarechain-node \
  --dev \
  --tmp \
  --rpc-cors all \
  --alice \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945

# Wait for node to start
sleep 10

# Verify node is running
curl -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
  http://127.0.0.1:9944
```

### Step 3: Run Stress Tests

```bash
# Quick test (5 minutes, 1000 TPS)
TEST_MODE=quick TARGET_TPS=1000 ./scripts/testnet/stress_test_harness.sh

# Full test (5 minutes, 2000 TPS)
TARGET_TPS=2000 TEST_DURATION=300 ./scripts/testnet/stress_test_harness.sh

# Long-running test (72 hours - run on dedicated infrastructure)
RUN_LONG_TEST=true ./scripts/testnet/stress_test_harness.sh
```

**Expected Output:**
```
╔══════════════════════════════════════════════════════════════╗
║         ËTRID PROTOCOL STRESS TEST HARNESS                  ║
║         Target: 1000 tx/s for 300s                         ║
╚══════════════════════════════════════════════════════════════╝

[TEST] Node Connection & Health Check
[INFO]   Peers: 0
[INFO]   Syncing: false
[PASS] Node is healthy and synced

[TEST] Block Production Rate (30 second sample)
[INFO]   Starting block: 42
[INFO]   Ending block: 47
[INFO]   Blocks produced: 5 in 30 seconds
[INFO]   Average block time: 6.00s
[PASS] Block production rate is acceptable (6.00s per block)

[TEST] High Transaction Volume (1000 tx/s for 300s)
[INFO]   Target: 1000 transactions/second
[INFO]   Duration: 300 seconds
[INFO]   Total transactions: 300000
  Progress: 300/300s | Submitted: 300000 | Rate: 1000 tx/s
[INFO]   Test completed:
[INFO]     Duration: 300s
[INFO]     Transactions submitted: 300000
[INFO]     Actual rate: 1000.00 tx/s
[PASS] Node survived high transaction volume

═══════════════════════════════════════════════════════════════
✅ ALL STRESS TESTS PASSED
Status: READY FOR AUDIT
═══════════════════════════════════════════════════════════════
```

### Step 4: Analyze Results

```bash
# View detailed results
cat stress-test-results/stress-test-*.log

# Key metrics to check:
# - TPS achieved vs target
# - Success rate (should be >99%)
# - Block production (should be consistent)
# - Memory growth (should be <50%)

# Check node metrics during test
curl -s http://localhost:9615/metrics | grep -E "(transaction|block|finality)"
```

---

## Part 4: Profiling (HIGH PRIORITY - 2-3 hours)

### Step 1: CPU Profiling with Flamegraph

```bash
# Profile a running node
# First, start node in background
./target/release/flarechain-node --dev --tmp &
NODE_PID=$!

# Let it run for 60 seconds to gather data
sleep 60

# Generate flamegraph
sudo cargo flamegraph --pid $NODE_PID -o flamegraph-flarechain.svg

# View flamegraph
open flamegraph-flarechain.svg

# Look for functions consuming >5% CPU time
# Common hotspots:
# - Block import
# - State transitions
# - Consensus (ASF)
# - Transaction validation
```

### Step 2: Memory Profiling

```bash
# Profile with heaptrack
heaptrack ./target/release/flarechain-node --dev --tmp &
HEAP_PID=$!

# Run for 5 minutes
sleep 300

# Stop node
kill $HEAP_PID

# Analyze heap usage
heaptrack_gui heaptrack.flarechain-node.*.gz

# Look for:
# - Memory leaks (increasing allocations)
# - Large allocations
# - Allocation hot spots
```

### Step 3: Continuous Profiling During Load Test

```bash
# Start node with profiling
cargo flamegraph --bin flarechain-node -- --dev --tmp &

# In another terminal, run stress test
sleep 30  # Wait for node to start
TARGET_TPS=1000 TEST_DURATION=300 ./scripts/testnet/stress_test_harness.sh

# Flamegraph will be generated when node stops
# Compare baseline flamegraph vs load test flamegraph
```

---

## Part 5: Monitoring Setup (HIGH PRIORITY - 1-2 hours)

### Step 1: Set Up Prometheus

```bash
# Install Prometheus (macOS)
brew install prometheus

# Or use Docker
docker run -d \
  --name prometheus \
  -p 9090:9090 \
  -v $(pwd)/scripts/testnet/prometheus.yml:/etc/prometheus/prometheus.yml \
  prom/prometheus

# Verify Prometheus is scraping metrics
open http://localhost:9090/targets
```

**Prometheus Config:** `scripts/testnet/prometheus.yml` (already exists)

### Step 2: Set Up Grafana

```bash
# Install Grafana (macOS)
brew install grafana
brew services start grafana

# Or use Docker
docker run -d \
  --name grafana \
  -p 3000:3000 \
  -v $(pwd)/scripts/testnet/grafana-dashboard.json:/var/lib/grafana/dashboards/ \
  grafana/grafana

# Access Grafana
open http://localhost:3000
# Default login: admin/admin

# Add Prometheus data source
# Configuration > Data Sources > Add Prometheus
# URL: http://localhost:9090

# Import dashboard
# Dashboards > Import > Upload JSON
# Use: scripts/testnet/grafana-dashboard.json
```

### Step 3: Monitor Key Metrics

**Critical Metrics Dashboard:**

1. **Transactions**
   - `rate(substrate_proposer_block_constructed_count[1m])` - Block production rate
   - `substrate_sub_txpool_validations_scheduled` - Transaction pool

2. **Performance**
   - `substrate_block_height` - Current block
   - `substrate_block_height{status="finalized"}` - Finalized block
   - Finality lag: `substrate_block_height{status="best"} - substrate_block_height{status="finalized"}`

3. **Resources**
   - `process_resident_memory_bytes` - Memory usage
   - `process_cpu_seconds_total` - CPU usage
   - `substrate_state_cache_bytes` - State cache

4. **Network**
   - `substrate_network_peers_count` - Peer count
   - `substrate_network_bandwidth_bytes_total` - Bandwidth

---

## Part 6: Performance Validation Checklist

### Pre-Launch Validation

```bash
# 1. Runtime weights generated and integrated
ls -lh 05-multichain/flare-chain/runtime/src/weights/
# Should have 8+ .rs files

# 2. Database optimized
grep -E "(cache_size|pruning|state-cache)" scripts/start-validator-optimized.sh
# Should show optimized values

# 3. Load tests passed
ls -lh stress-test-results/
# Should have recent test results

# 4. 1000+ TPS achieved
grep "Actual rate" stress-test-results/stress-test-*.log
# Should show >=1000 tx/s

# 5. Monitoring active
curl -s http://localhost:9615/metrics | wc -l
# Should show 100+ metrics

# 6. Profiling complete
ls -lh flamegraph*.svg
# Should have flamegraph file

# 7. No memory leaks
grep "Memory growth" stress-test-results/stress-test-*.log
# Should show <50% growth
```

### Performance Targets

| Metric | Target | How to Check |
|--------|--------|--------------|
| TPS | 1,000+ | Stress test results |
| Block Time | 6s | Prometheus: `rate(substrate_block_height[1m])` |
| Finality Lag | <100 blocks | Prometheus: finality lag query |
| Memory Growth | <50%/hour | Stress test memory monitoring |
| Database Hit Rate | >80% | Prometheus: `rocksdb_block_cache_hit_rate` |
| Peer Count | 10-50 | Prometheus: `substrate_network_peers_count` |

---

## Troubleshooting

### Build Failures

```bash
# Clear cache and retry
cargo clean
cargo build --release --features runtime-benchmarks -p flarechain-node

# Check for dependency issues
cargo tree -p flarechain-node

# Update dependencies
cargo update
```

### Benchmark Failures

```bash
# Check individual pallet
./target/release/flarechain-node benchmark pallet \
  --chain dev \
  --pallet pallet_validator_committee \
  --extrinsic add_validator \
  --steps 10 \
  --repeat 5

# Increase verbosity
RUST_LOG=debug ./scripts/testnet/benchmark_weights.sh
```

### Load Test Failures

```bash
# Check node is running
curl http://localhost:9944

# Check node logs
tail -f /tmp/flarechain.log

# Reduce load
TARGET_TPS=100 TEST_DURATION=60 ./scripts/testnet/stress_test_harness.sh
```

### Profiling Issues

```bash
# macOS: May need sudo for flamegraph
sudo cargo flamegraph --bin flarechain-node -- --dev --tmp

# Linux: Install perf
sudo apt-get install linux-tools-common linux-tools-generic

# Check permissions
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

---

## Next Steps

After completing all critical items:

1. **Document Results**
   - Save all benchmark outputs
   - Archive stress test results
   - Export flamegraphs and analysis

2. **Prepare for Audit**
   - Update AUDIT_PACKAGE.md with performance data
   - Document any issues found
   - Create performance baseline report

3. **Production Deployment**
   - Use optimized node scripts
   - Configure monitoring alerts
   - Set up log aggregation
   - Schedule regular benchmarking

4. **Continuous Optimization**
   - Re-benchmark after major changes
   - Monitor production metrics
   - Track performance regressions
   - Optimize based on real-world usage

---

## Quick Command Reference

```bash
# Build with benchmarks
cargo build --release --features runtime-benchmarks -p flarechain-node

# Run benchmarks
./scripts/testnet/benchmark_weights.sh

# Start optimized node
./scripts/start-validator-optimized.sh

# Run load tests
TARGET_TPS=1000 ./scripts/testnet/stress_test_harness.sh

# Profile CPU
cargo flamegraph --bin flarechain-node -- --dev --tmp

# Check metrics
curl http://localhost:9615/metrics | grep substrate
```

---

**Estimated Total Time:**
- Part 1 (Weights): 2-4 hours
- Part 2 (Database): 2 hours
- Part 3 (Load Testing): 2-4 hours
- Part 4 (Profiling): 2-3 hours
- Part 5 (Monitoring): 1-2 hours

**Total: 9-15 hours for all critical and high-priority items**

**Priority Order:**
1. Generate runtime weights (CRITICAL - blocks DoS)
2. Run initial load tests (CRITICAL - validate TPS)
3. Optimize database (CRITICAL - performance)
4. Set up monitoring (HIGH - observability)
5. Profile and optimize (HIGH - identify bottlenecks)
