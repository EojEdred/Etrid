# Checkpoint Database

Persistent storage for ËTRID checkpoint finality state to survive node restarts.

## Overview

The checkpoint-db crate provides RocksDB-backed persistent storage for checkpoint signatures, certificates, and finality state. This ensures that validators can recover their consensus state after a node restart without having to resync from genesis.

## Features

- **Persistent Storage**: Stores checkpoint signatures and certificates to disk using RocksDB
- **Fast Recovery**: Restores consensus state in milliseconds on node startup
- **Automatic Pruning**: Configurable retention policy to prevent unbounded growth
- **Atomic Operations**: Thread-safe writes with RocksDB guarantees
- **Auto-Flush**: Periodic flushing to ensure data durability
- **Comprehensive Testing**: 100+ unit and integration tests

## Architecture

### Database Schema

The checkpoint database uses RocksDB with three column families:

#### 1. Signatures Column Family
- **Key**: `checkpoint_number` (u64, 8 bytes)
- **Value**: `Vec<CheckpointSignature>` (SCALE encoded)
- **Purpose**: Stores all signatures for each checkpoint

#### 2. Certificates Column Family
- **Key**: `checkpoint_number` (u64, 8 bytes)
- **Value**: `CheckpointCertificate` (SCALE encoded)
- **Purpose**: Stores finalized checkpoint certificates

#### 3. Metadata Column Family
- **Key**: `"last_finalized"` (static string)
- **Value**: `checkpoint_number` (u64, 8 bytes)
- **Purpose**: Tracks the last finalized checkpoint

### Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    CheckpointFinality                       │
│  (In-memory consensus state + persistence hooks)            │
└─────────────────┬──────────────────────────┬────────────────┘
                  │                          │
                  │ CheckpointStorage        │ Recovery
                  │ Trait (async)            │ on startup
                  ▼                          ▼
┌─────────────────────────────────────────────────────────────┐
│               CheckpointStorageAdapter                      │
│  (Async wrapper for sync RocksDB operations)                │
└─────────────────┬───────────────────────────────────────────┘
                  │
                  │ Sync operations
                  ▼
┌─────────────────────────────────────────────────────────────┐
│                    CheckpointDB                             │
│  (RocksDB backend with column families)                     │
└─────────────────────────────────────────────────────────────┘
                  │
                  │ Persistent storage
                  ▼
┌─────────────────────────────────────────────────────────────┐
│                   RocksDB Files                             │
│  (*.sst, *.log, MANIFEST, etc.)                             │
└─────────────────────────────────────────────────────────────┘
```

## Usage

### Basic Example

```rust
use checkpoint_db::{CheckpointDB, CheckpointStorageAdapter};
use std::sync::Arc;
use std::path::Path;

// Create database
let db_path = Path::new("/var/lib/etrid/checkpoints");
let checkpoint_db = Arc::new(CheckpointDB::new(db_path)?);

// Create adapter for async trait
let storage = Arc::new(CheckpointStorageAdapter::new(checkpoint_db));

// Store checkpoint signature
storage.store_signature(checkpoint_number, &signature).await?;

// Store checkpoint certificate
storage.store_certificate(checkpoint_number, &certificate).await?;

// Update last finalized
storage.set_last_finalized(checkpoint_number).await?;

// Flush to disk
storage.flush().await?;
```

### Integration with CheckpointFinality

```rust
use finality_gadget::{CheckpointFinality, CheckpointPersistenceConfig};
use checkpoint_db::{CheckpointDB, CheckpointStorageAdapter};

// Create checkpoint finality instance
let mut checkpoint_finality = CheckpointFinality::new(
    validator_id,
    max_validators,
    canonical_chain_client,
    network_bridge,
);

// Setup persistent storage
let checkpoint_db_path = config.database.path().join("checkpoints");
let checkpoint_db = Arc::new(CheckpointDB::new(&checkpoint_db_path)?);
let storage_adapter = Arc::new(CheckpointStorageAdapter::new(checkpoint_db));

// Configure persistence
let persistence_config = CheckpointPersistenceConfig {
    enabled: true,
    retention_checkpoints: 1000,  // Keep last 1000 checkpoints
    flush_interval_secs: 10,      // Flush every 10 seconds
    prune_interval_secs: 300,     // Prune every 5 minutes
};

// Attach storage to checkpoint finality
checkpoint_finality.set_storage(storage_adapter, persistence_config);

// Restore state from disk on startup
checkpoint_finality.restore_from_storage().await?;

// Spawn maintenance task for periodic flush/prune
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        checkpoint_finality.perform_periodic_maintenance().await?;
    }
});
```

## Configuration

### CheckpointPersistenceConfig

```rust
pub struct CheckpointPersistenceConfig {
    /// Enable persistence (default: true)
    pub enabled: bool,

    /// Number of checkpoints to retain in storage (default: 1000)
    pub retention_checkpoints: u64,

    /// Auto-flush interval in seconds (default: 10)
    pub flush_interval_secs: u64,

    /// Auto-prune interval in seconds (default: 300 = 5 minutes)
    pub prune_interval_secs: u64,
}
```

### Recommended Settings

| Scenario | Retention | Flush Interval | Prune Interval |
|----------|-----------|----------------|----------------|
| **Testnet** | 100 | 5s | 60s (1 min) |
| **Mainnet** | 1000 | 10s | 300s (5 min) |
| **Archive Node** | 10000 | 30s | 3600s (1 hour) |
| **Resource Constrained** | 50 | 15s | 60s (1 min) |

## Performance

### Benchmarks

Tested on MacBook Pro M1, SSD storage:

| Operation | Throughput | Latency (p50) | Latency (p99) |
|-----------|------------|---------------|---------------|
| **Store Signature** | ~100k/s | 10 µs | 50 µs |
| **Store Certificate** | ~50k/s | 20 µs | 100 µs |
| **Get Signatures** | ~200k/s | 5 µs | 25 µs |
| **Get Certificate** | ~200k/s | 5 µs | 25 µs |
| **Flush** | - | 1-5 ms | 20 ms |
| **Prune** | ~10k checkpoints/s | - | - |

### Storage Requirements

Assuming:
- 21 validators
- 64-byte signatures
- 10 blocks/checkpoint

| Time Period | Checkpoints | Signatures | Storage (approx) |
|-------------|-------------|------------|------------------|
| **1 hour** | 60 | 1,260 | 100 KB |
| **1 day** | 1,440 | 30,240 | 2.4 MB |
| **1 week** | 10,080 | 211,680 | 17 MB |
| **1 month** | 43,200 | 907,200 | 72 MB |
| **1 year** | 518,400 | 10,886,400 | 870 MB |

With pruning (retention = 1000 checkpoints): **~8 MB steady state**

## Recovery Strategy

### On Node Startup

1. **Open Database**: Load RocksDB from disk
2. **Read Metadata**: Get last finalized checkpoint number
3. **Load Recent State**: Restore last 100 checkpoints worth of signatures/certificates
4. **Rebuild Index**: Reconstruct validator signature tracking for double-sign detection
5. **Resume Consensus**: Continue from restored state

### Recovery Time

| State Size | Recovery Time |
|------------|---------------|
| 100 checkpoints | ~10 ms |
| 1000 checkpoints | ~50 ms |
| 10000 checkpoints | ~200 ms |

## Data Integrity

### Guarantees

- **Atomicity**: Individual writes are atomic (RocksDB guarantee)
- **Durability**: Data is durable after `flush()` (WAL + SST files)
- **Consistency**: SCALE encoding ensures deterministic serialization
- **Crash Recovery**: RocksDB WAL enables recovery from crashes

### Error Handling

The crate provides comprehensive error types:

```rust
pub enum CheckpointDbError {
    RocksDb(rocksdb::Error),
    Decode(codec::Error),
    Bincode(bincode::Error),
    InvalidData(String),
    ColumnFamilyNotFound(String),
}
```

All operations return `Result<T, CheckpointDbError>` for proper error propagation.

## Monitoring

### Log Messages

```
🗄️  Opening checkpoint database at /var/lib/etrid/checkpoints
✅ Checkpoint database opened successfully
📝 Stored signature: checkpoint=42, validator=5, total=12
📜 Stored certificate: checkpoint=42, signatures=14
🏁 Updated last finalized checkpoint: 42
💾 Auto-flushing checkpoint database
🧹 Auto-pruning old checkpoint data
✅ Pruned 10 old checkpoints
```

### Metrics

Call `get_stats()` to get database statistics:

```rust
let stats = checkpoint_db.get_stats()?;
println!("{}", stats);

// Output:
// CheckpointDB Stats: last_finalized=1000, certificates=1000,
//   signatures=14000, range=1-1000
```

## Testing

### Run Unit Tests

```bash
cd 09-consensus/checkpoint-db
cargo test
```

### Run Integration Tests

```bash
cargo test --test integration_tests
```

### Test Coverage

- ✅ Signature storage and retrieval
- ✅ Certificate storage and retrieval
- ✅ Last finalized tracking
- ✅ Pruning old data
- ✅ Batch operations
- ✅ Persistence across restarts
- ✅ Concurrent writes
- ✅ Empty database initialization
- ✅ Recovery from unclean shutdown
- ✅ Database statistics

## Troubleshooting

### Issue: Database won't open

**Symptoms**: Error on `CheckpointDB::new()`

**Solutions**:
1. Check directory permissions
2. Ensure path is valid
3. Verify disk space available
4. Check for file locks (another process using DB)

### Issue: Signatures not persisting

**Symptoms**: Signatures disappear after restart

**Solutions**:
1. Ensure `flush()` is called periodically
2. Check `persistence_config.enabled = true`
3. Verify no errors in logs during storage
4. Check disk space

### Issue: Excessive disk usage

**Symptoms**: Database grows unbounded

**Solutions**:
1. Reduce `retention_checkpoints` value
2. Decrease `prune_interval_secs` for more frequent pruning
3. Manually call `prune_old_checkpoints()`
4. Verify pruning task is running

### Issue: Slow startup

**Symptoms**: Node takes long time to restore state

**Solutions**:
1. Reduce retention (fewer checkpoints to load)
2. Use SSD instead of HDD
3. Optimize RocksDB settings
4. Consider incremental restoration

## Advanced Usage

### Custom RocksDB Options

For production deployments, you may want to customize RocksDB options:

```rust
// Note: This requires modifying CheckpointDB::new() to accept Options

let mut db_opts = Options::default();
db_opts.create_if_missing(true);
db_opts.create_missing_column_families(true);
db_opts.set_max_background_jobs(4);
db_opts.set_bytes_per_sync(1048576);
db_opts.set_keep_log_file_num(10);

let db = CheckpointDB::new_with_options(path, db_opts)?;
```

### Manual Pruning

```rust
// Prune checkpoints older than 1000 from current
checkpoint_db.prune_old_checkpoints(1000)?;
```

### Batch Operations

For better performance when storing many signatures:

```rust
let signatures = vec![
    (CheckpointNumber(1), sig1),
    (CheckpointNumber(1), sig2),
    (CheckpointNumber(1), sig3),
];

checkpoint_db.store_signatures_batch(&signatures)?;
```

## Migration & Upgrades

### Database Schema Changes

If the schema changes in future versions, migration will be needed:

1. Backup existing database: `cp -r checkpoints checkpoints.backup`
2. Run migration tool: `etrid-checkpoint-migrate --from v1 --to v2`
3. Verify migration: `etrid-checkpoint-verify`

### Backward Compatibility

Current version: **v0.1.0**

Breaking changes will be communicated with major version bumps.

## License

Apache-2.0

## Authors

Ëtrid Foundation

## Related Crates

- `etrid-finality-gadget`: Consensus finality implementation
- `sp-core`: Substrate primitives for cryptography
- `codec`: SCALE encoding/decoding
- `rocksdb`: RocksDB bindings for Rust
