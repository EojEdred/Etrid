# Checkpoint Finality Persistence Integration Guide

## Overview

This document describes how to integrate the persistence module into the existing CheckpointFinality implementation.

## Required Changes to `lib.rs`

### 1. Add module declaration (at the top of the file, after use statements)

```rust
pub mod persistence;
pub use persistence::{CheckpointStorage, CheckpointPersistenceConfig, CheckpointFinalityPersistence};
```

### 2. Add persistence field to CheckpointFinality struct

Add this field to the CheckpointFinality struct (around line 1190):

```rust
pub struct CheckpointFinality {
    // ... existing fields ...

    // Persistent storage (optional)
    persistence: Option<CheckpointFinalityPersistence>,
}
```

### 3. Initialize persistence field in `new()` method

In the `CheckpointFinality::new()` method (around line 1210), add:

```rust
impl CheckpointFinality {
    pub fn new(...) -> Self {
        // ... existing initialization ...

        Self {
            // ... existing fields ...
            persistence: None,  // Add this line
        }
    }
}
```

### 4. Add persistence hooks to existing methods

#### In `add_checkpoint_signature()` method (around line 1320):

After successfully adding the signature, before returning:

```rust
// Add signature to collection
checkpoint_sigs.push(signature.clone());
self.metrics.total_signatures += 1;

// PERSISTENCE HOOK: Store signature to disk
if let Err(e) = self.persist_signature(&signature).await {
    tracing::warn!("⚠️ Failed to persist signature: {}", e);
    // Continue even if persistence fails
}

// ... rest of the method
```

#### In `create_checkpoint_certificate()` method (around line 1450):

After creating the certificate, before broadcasting:

```rust
self.checkpoint_certificates.insert(checkpoint_number, certificate.clone());
self.metrics.total_certificates += 1;
self.last_checkpoint_time.insert(checkpoint_number, Instant::now());

// PERSISTENCE HOOK: Store certificate to disk
if let Err(e) = self.persist_certificate(&certificate).await {
    tracing::warn!("⚠️ Failed to persist certificate: {}", e);
}

// ... rest of the method
```

#### In `advance_finality()` method (around line 1587):

After advancing finality:

```rust
if checkpoint_number.0 > self.last_finalized_checkpoint.0 {
    self.last_finalized_checkpoint = checkpoint_number;

    // PERSISTENCE HOOK: Update last finalized in storage
    if let Err(e) = self.persist_last_finalized().await {
        tracing::warn!("⚠️ Failed to persist last finalized: {}", e);
    }

    tracing::info!(
        "🏁 FINALITY ADVANCED: checkpoint={} is now finalized",
        checkpoint_number.0
    );
}
```

## Usage Example

### In `asf_service.rs`:

```rust
use checkpoint_db::CheckpointDB;
use finality_gadget::{CheckpointFinality, CheckpointPersistenceConfig};

// Create checkpoint database
let checkpoint_db_path = config.database.path().join("checkpoints");
let checkpoint_db = Arc::new(CheckpointDB::new(&checkpoint_db_path)?);

// Create storage adapter
let storage_adapter = Arc::new(CheckpointStorageAdapter::new(checkpoint_db));

// Create checkpoint finality instance
let mut checkpoint_finality = CheckpointFinality::new(
    validator_id,
    max_validators,
    canonical_chain_client,
    network_bridge,
);

// Enable persistence
let persistence_config = CheckpointPersistenceConfig {
    enabled: true,
    retention_checkpoints: 1000,
    flush_interval_secs: 10,
    prune_interval_secs: 300,
};

checkpoint_finality.set_storage(storage_adapter, persistence_config);

// Restore state from disk
checkpoint_finality.restore_from_storage().await?;

tracing::info!("✅ Checkpoint finality initialized with persistent storage");
```

### Periodic maintenance task:

Add to the worker loop or spawn a separate task:

```rust
// Spawn maintenance task
let checkpoint_finality_clone = checkpoint_finality.clone();
task_manager.spawn_essential_handle().spawn(
    "checkpoint-maintenance",
    None,
    async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            if let Err(e) = checkpoint_finality_clone.perform_periodic_maintenance().await {
                tracing::error!("❌ Checkpoint maintenance failed: {}", e);
            }
        }
    }
);
```

## Testing

After integration, test the following scenarios:

1. **Normal operation**: Verify signatures and certificates are persisted
2. **Node restart**: Stop node, restart, verify state is restored correctly
3. **Finality recovery**: Verify finalized checkpoints are restored
4. **Pruning**: Verify old data is pruned after retention period
5. **Flush**: Verify data is flushed to disk periodically

## Monitoring

Check logs for these messages:

- `💾 Checkpoint persistence enabled`
- `🔄 Restoring checkpoint state from storage`
- `✅ Checkpoint state restored successfully`
- `📝 Stored signature`
- `📜 Stored certificate`
- `🏁 Updated last finalized checkpoint`
- `💾 Auto-flushing checkpoint database`
- `🧹 Auto-pruning old checkpoint data`

## Troubleshooting

### Issue: Signatures not persisting

- Check that `set_storage()` was called
- Check that `persistence_config.enabled = true`
- Check disk space and permissions

### Issue: Restoration fails on restart

- Check database path is correct
- Check database files exist and are readable
- Check for corruption (delete and resync if needed)

### Issue: Excessive disk usage

- Reduce `retention_checkpoints` value
- Reduce `prune_interval_secs` for more frequent pruning
- Manually call `prune_old_checkpoints()`
