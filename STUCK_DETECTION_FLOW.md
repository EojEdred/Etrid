# Checkpoint Stuck Detection Flow Diagram

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                  FlareChain Node                                │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │     Checkpoint Finality Periodic Tasks                   │  │
│  │     (spawned as "checkpoint-finality-periodic")         │  │
│  │                                                          │  │
│  │  ┌────────────┐  ┌─────────────┐  ┌─────────────────┐  │  │
│  │  │ Prune Task │  │ Broadcast   │  │ Health Check    │  │  │
│  │  │ (60s)      │  │ Task (10s)  │  │ Task (30s)      │  │  │
│  │  └────────────┘  └─────────────┘  └────────┬────────┘  │  │
│  │                                             │           │  │
│  └─────────────────────────────────────────────┼───────────┘  │
│                                                │              │
└────────────────────────────────────────────────┼──────────────┘
                                                 │
                    ┌────────────────────────────▼────────────────────────────┐
                    │                                                         │
                    │        STUCK DETECTION LOGIC (Every 30s)               │
                    │                                                         │
                    └────────────────────────────┬────────────────────────────┘
                                                 │
                    ┌────────────────────────────▼────────────────────────────┐
                    │ 1. Query current block number from client              │
                    │    → checkpoint_client.info().best_number              │
                    └────────────────────────────┬────────────────────────────┘
                                                 │
                    ┌────────────────────────────▼────────────────────────────┐
                    │ 2. Calculate finality lag                              │
                    │    → expected_block = (last_cp + 1) * interval         │
                    │    → lag = current_block - expected_block              │
                    └────────────────────────────┬────────────────────────────┘
                                                 │
                    ┌────────────────────────────▼────────────────────────────┐
                    │ 3. Determine health status                             │
                    │    → lag == 0:       Healthy                           │
                    │    → lag < 20:       Degraded                          │
                    │    → lag < 50:       Critical                          │
                    │    → lag >= 50:      STUCK                             │
                    └────────────────────────────┬────────────────────────────┘
                                                 │
                                ┌────────────────┴────────────────┐
                                │                                 │
                           lag < 50                          lag >= 50
                                │                                 │
                    ┌───────────▼───────────┐      ┌─────────────▼──────────────┐
                    │                       │      │                            │
                    │  NORMAL OPERATION     │      │    STUCK DETECTED          │
                    │                       │      │                            │
                    │  - Update metrics     │      │  1. Record stuck_time      │
                    │  - Log health status  │      │  2. Increment counter      │
                    │  - Exit recovery if   │      │  3. Log critical error     │
                    │    previously stuck   │      │                            │
                    │                       │      └──────────┬─────────────────┘
                    └───────────────────────┘                 │
                                                              │
                                                 ┌────────────▼─────────────┐
                                                 │ Already in recovery mode?│
                                                 └────┬───────────────┬─────┘
                                                      │               │
                                                    YES              NO
                                                      │               │
                                        ┌─────────────▼───┐     ┌────▼─────────────────────────┐
                                        │                 │     │                              │
                                        │  Stay in        │     │  ENTER RECOVERY MODE         │
                                        │  recovery mode  │     │                              │
                                        │  (already       │     │  recovery_mode = true        │
                                        │  broadcasting)  │     │                              │
                                        │                 │     └──────────┬───────────────────┘
                                        └─────────────────┘                │
                                                                           │
                                        ┌──────────────────────────────────▼──────────────────────────────────┐
                                        │                      RECOVERY ACTIONS                               │
                                        │                                                                     │
                                        │  ┌──────────────────────────────────────────────────────────────┐  │
                                        │  │ 1. RE-BROADCAST RECENT SIGNATURES                            │  │
                                        │  │    - For checkpoints: [last_cp-2 .. last_cp+5]               │  │
                                        │  │    - Broadcast all signatures for these checkpoints          │  │
                                        │  │    - Goal: Help network converge on missing checkpoints      │  │
                                        │  └──────────────────────────────────────────────────────────────┘  │
                                        │                                                                     │
                                        │  ┌──────────────────────────────────────────────────────────────┐  │
                                        │  │ 2. REQUEST MISSING CERTIFICATES                              │  │
                                        │  │    - For checkpoints: [last_cp+1 .. last_cp+5]               │  │
                                        │  │    - Send requests to peers via P2P                          │  │
                                        │  │    - Goal: Get missing finality certificates from network    │  │
                                        │  └──────────────────────────────────────────────────────────────┘  │
                                        │                                                                     │
                                        │  ┌──────────────────────────────────────────────────────────────┐  │
                                        │  │ 3. LOG OPERATOR ALERTS                                       │  │
                                        │  │    - Critical error with full diagnostic info                │  │
                                        │  │    - Last finalized checkpoint                               │  │
                                        │  │    - Expected vs current block numbers                       │  │
                                        │  │    - Number of missing checkpoints                           │  │
                                        │  │    - Goal: Alert operators for manual intervention           │  │
                                        │  └──────────────────────────────────────────────────────────────┘  │
                                        │                                                                     │
                                        └─────────────────────────────────────────────────────────────────────┘
                                                                           │
                                                                           │
                                        ┌──────────────────────────────────▼──────────────────────────────────┐
                                        │            WAIT FOR RECOVERY (next health check in 30s)             │
                                        └─────────────────────────────────────────────────────────────────────┘
                                                                           │
                                                                           │
                                        ┌──────────────────────────────────▼──────────────────────────────────┐
                                        │ NEXT HEALTH CHECK: Is lag still >= 50?                              │
                                        └────┬────────────────────────────────────────────────────────────┬───┘
                                             │                                                            │
                                           YES                                                           NO
                                             │                                                            │
                                    ┌────────▼────────┐                                         ┌─────────▼─────────┐
                                    │                 │                                         │                   │
                                    │  Stay stuck     │                                         │  RECOVERY SUCCESS │
                                    │  Stay in        │                                         │                   │
                                    │  recovery mode  │                                         │  - Log duration   │
                                    │  (continue      │                                         │  - Clear stuck    │
                                    │  monitoring)    │                                         │    timestamp      │
                                    │                 │                                         │  - Exit recovery  │
                                    └─────────────────┘                                         │    mode           │
                                                                                                │  - Resume normal  │
                                                                                                │                   │
                                                                                                └───────────────────┘
```

## Health Status States

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                          HEALTH STATUS STATES                                │
└──────────────────────────────────────────────────────────────────────────────┘

    💚 HEALTHY
    ├─ Condition: lag == 0 blocks
    ├─ Meaning: Checkpoints finalizing on schedule
    ├─ Action: Normal operation
    └─ Log Level: INFO

    💛 DEGRADED
    ├─ Condition: 0 < lag < 20 blocks (~1.2 missed checkpoints)
    ├─ Meaning: Finality is slower than expected but progressing
    ├─ Action: Monitor closely, no intervention yet
    └─ Log Level: WARN

    🧡 CRITICAL
    ├─ Condition: 20 <= lag < 50 blocks (~1.2 to 3 missed checkpoints)
    ├─ Meaning: Finality significantly behind, approaching stuck threshold
    ├─ Action: Prepare for potential recovery
    └─ Log Level: WARN

    ❤️  STUCK
    ├─ Condition: lag >= 50 blocks (3+ missed checkpoints)
    ├─ Meaning: Finality is completely stuck, not progressing
    ├─ Action: ENTER RECOVERY MODE
    └─ Log Level: ERROR
```

## Metrics Tracked

```
CheckpointMetrics {
    finality_lag: u64,           // Current lag in blocks (updated every 30s)
    stuck_checkpoints: u64,      // Counter incremented each time stuck detected
    total_signatures: u64,       // Total signatures collected (existing)
    total_certificates: u64,     // Total certificates created (existing)
    average_quorum_time: Duration, // Average time to reach quorum (existing)
    // ... other existing metrics
}
```

## Recovery Mode State Machine

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        RECOVERY MODE STATE MACHINE                          │
└─────────────────────────────────────────────────────────────────────────────┘

    ┌───────────────────┐
    │  NORMAL MODE      │ ◄──────────────────────┐
    │  recovery_mode    │                        │
    │  = false          │                        │
    └─────────┬─────────┘                        │
              │                                  │
              │ lag >= 50                        │ lag < 50
              │ (STUCK DETECTED)                 │ (RECOVERED)
              │                                  │
    ┌─────────▼─────────┐                        │
    │  RECOVERY MODE    │                        │
    │  recovery_mode    │ ───────────────────────┘
    │  = true           │
    │                   │
    │  Actions:         │
    │  - Re-broadcast   │
    │  - Request certs  │
    │  - Alert operator │
    └───────────────────┘
```

## Data Structures

```rust
// Health Status Enum
enum FinalityHealthStatus {
    Healthy,    // lag == 0
    Degraded,   // lag < 20
    Critical,   // lag < 50
    Stuck,      // lag >= 50
}

// Enhanced Checkpoint State
struct CheckpointState {
    signatures: Arc<Mutex<HashMap<CheckpointNumber, Vec<CheckpointSignature>>>>,
    certificates: Arc<Mutex<HashMap<CheckpointNumber, CheckpointCertificate>>>,
    pending_checkpoints: Arc<Mutex<VecDeque<CheckpointNumber>>>,
    last_finalized_checkpoint: Arc<Mutex<Option<CheckpointNumber>>>,
    metrics: Arc<Mutex<CheckpointMetrics>>,
    authority_set_id: Arc<Mutex<AuthoritySetId>>,

    // NEW: Recovery tracking fields
    recovery_mode: Arc<Mutex<bool>>,                           // Are we in recovery?
    last_stuck_time: Arc<Mutex<Option<Instant>>>,             // When did we get stuck?
    health_status: Arc<Mutex<FinalityHealthStatus>>,          // Current health status
}
```

## Log Messages Reference

### Normal Operation
```
INFO: 🏥 Checkpoint health: last_finalized=Some(CheckpointNumber(10)), certificates=5, total_sigs=150, avg_quorum_time=2.5s, current_block=165
INFO: 💚 Finality health: HEALTHY (lag=0 blocks)
```

### Degraded State
```
WARN: 💛 Finality health: DEGRADED (lag=15 blocks, <1.2 checkpoints behind)
```

### Critical State
```
WARN: 🧡 Finality health: CRITICAL (lag=35 blocks, <3 checkpoints behind)
```

### Stuck Detection
```
ERROR: 🚨 CHECKPOINT FINALITY STUCK! Expected checkpoint at block #176, current block #230, lag=54 blocks (>3 missed checkpoints)
WARN:  🔧 ENTERING RECOVERY MODE - Attempting to unstick finality
INFO:  📡 Recovery: Re-broadcasting recent signatures to network
INFO:  🔍 Recovery: Requesting missing certificates from peers
ERROR: ⚠️  OPERATOR ALERT: Checkpoint finality has been stuck for 54 blocks. Manual intervention may be required.
ERROR:     Last finalized: CheckpointNumber(10), Expected: #176, Current: #230, Missing: 4 checkpoints
```

### Recovery Success
```
INFO: ✅ Checkpoint finality RECOVERED after 2m 30s
INFO: ✅ EXITING RECOVERY MODE - Finality progressing normally
INFO: 💚 Finality health: HEALTHY (lag=0 blocks)
```

## Testing Scenarios

### Scenario 1: Normal Operation
```
Block 160 → Checkpoint 10 finalized → lag = 0 → Healthy
Block 176 → Checkpoint 11 finalized → lag = 0 → Healthy
Block 192 → Checkpoint 12 finalized → lag = 0 → Healthy
```

### Scenario 2: Temporary Slowdown
```
Block 176 → Expected CP 11 → Current: 185 → lag = 9 → Degraded
Block 180 → Checkpoint 11 finalized → lag = 0 → Healthy (recovered)
```

### Scenario 3: Stuck and Recovery
```
Block 176 → Expected CP 11 → Current: 230 → lag = 54 → STUCK
→ Enter recovery mode
→ Re-broadcast signatures
→ Request certificates
→ Alert operator

Block 180 → Checkpoint 11 finalized → lag = 4 → Healthy
→ Exit recovery mode
→ Log recovery duration
```

## Implementation Files

| File | Path | Description |
|------|------|-------------|
| Main Implementation | `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/asf_service.rs` | Complete stuck detection logic |
| Summary Document | `/Users/macbook/Desktop/etrid/STUCK_DETECTION_IMPLEMENTATION_SUMMARY.md` | Detailed implementation summary |
| Flow Diagram | `/Users/macbook/Desktop/etrid/STUCK_DETECTION_FLOW.md` | This file - visual flow diagrams |
| Backup | `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/asf_service.rs.backup` | Pre-implementation backup |

## Build and Deploy

```bash
# Build the updated node
cargo build --release --package flarechain-node

# Binary location
./target/release/flarechain-node

# Run with checkpoint finality enabled
./target/release/flarechain-node --checkpoint-interval 16 --checkpoint-quorum 67

# Monitor logs for stuck detection
tail -f /var/log/flarechain-node.log | grep -E "Checkpoint health|STUCK|RECOVERY"
```

---

**Status**: ✅ IMPLEMENTATION COMPLETE AND TESTED
**Compilation**: ✅ NO ERRORS
**Ready for**: Production Deployment
