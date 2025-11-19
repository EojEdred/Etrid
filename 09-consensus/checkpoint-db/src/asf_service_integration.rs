//! # ASF Service Integration Example
//!
//! This file shows how to integrate checkpoint persistence into asf_service.rs
//!
//! Copy the relevant sections to `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/asf_service.rs`

use std::sync::Arc;
use std::path::PathBuf;
use checkpoint_db::{CheckpointDB, CheckpointStorageAdapter};
use finality_gadget::{
    CheckpointFinality,
    persistence::{CheckpointPersistenceConfig, CheckpointStorage},
};

/// Configuration for checkpoint storage
#[derive(Clone, Debug)]
pub struct CheckpointStorageConfig {
    /// Enable persistent storage
    pub enabled: bool,

    /// Database path (relative to node base path)
    pub db_path: PathBuf,

    /// Number of checkpoints to retain
    pub retention_checkpoints: u64,

    /// Auto-flush interval in seconds
    pub flush_interval_secs: u64,

    /// Auto-prune interval in seconds
    pub prune_interval_secs: u64,
}

impl Default for CheckpointStorageConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            db_path: PathBuf::from("checkpoints"),
            retention_checkpoints: 1000,
            flush_interval_secs: 10,
            prune_interval_secs: 300,
        }
    }
}

/// Example: Initialize checkpoint storage in new_full()
///
/// Insert this code in the new_full() function after creating the client
pub fn example_initialize_checkpoint_storage(
    base_path: &PathBuf,
    config: &CheckpointStorageConfig,
) -> Result<Arc<dyn CheckpointStorage>, String> {
    if !config.enabled {
        return Err("Checkpoint storage disabled".to_string());
    }

    // Construct full database path
    let checkpoint_db_path = base_path.join(&config.db_path);

    tracing::info!("🗄️  Initializing checkpoint database at {:?}", checkpoint_db_path);

    // Create database directory if it doesn't exist
    std::fs::create_dir_all(&checkpoint_db_path)
        .map_err(|e| format!("Failed to create checkpoint db directory: {}", e))?;

    // Open checkpoint database
    let checkpoint_db = Arc::new(
        CheckpointDB::new(&checkpoint_db_path)
            .map_err(|e| format!("Failed to open checkpoint database: {}", e))?
    );

    // Create storage adapter
    let storage_adapter = Arc::new(CheckpointStorageAdapter::new(checkpoint_db));

    // Print database stats
    tokio::spawn({
        let storage = storage_adapter.clone();
        async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            // Log initial stats
            tracing::info!("✅ Checkpoint database initialized successfully");
        }
    });

    Ok(storage_adapter as Arc<dyn CheckpointStorage>)
}

/// Example: Setup checkpoint finality with persistence
///
/// Insert this code where CheckpointFinality is created
pub async fn example_setup_checkpoint_finality(
    mut checkpoint_finality: CheckpointFinality,
    storage_config: &CheckpointStorageConfig,
    checkpoint_storage: Arc<dyn CheckpointStorage>,
) -> Result<CheckpointFinality, String> {
    // Create persistence configuration
    let persistence_config = CheckpointPersistenceConfig {
        enabled: storage_config.enabled,
        retention_checkpoints: storage_config.retention_checkpoints,
        flush_interval_secs: storage_config.flush_interval_secs,
        prune_interval_secs: storage_config.prune_interval_secs,
    };

    // Attach storage to checkpoint finality
    checkpoint_finality.set_storage(checkpoint_storage, persistence_config);

    tracing::info!("💾 Checkpoint persistence enabled with config: {:?}", persistence_config);

    // Restore state from disk
    tracing::info!("🔄 Restoring checkpoint state from persistent storage...");
    checkpoint_finality.restore_from_storage().await?;

    tracing::info!("✅ Checkpoint finality restored from storage successfully");

    Ok(checkpoint_finality)
}

/// Example: Spawn maintenance task
///
/// Insert this code in new_full() after spawning other essential tasks
pub fn example_spawn_maintenance_task(
    task_manager: &mut sc_service::TaskManager,
    checkpoint_finality: Arc<tokio::sync::Mutex<CheckpointFinality>>,
) {
    task_manager.spawn_essential_handle().spawn(
        "checkpoint-maintenance",
        None,
        async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

            loop {
                interval.tick().await;

                // Acquire lock and perform maintenance
                let mut cf = checkpoint_finality.lock().await;
                if let Err(e) = cf.perform_periodic_maintenance().await {
                    tracing::error!("❌ Checkpoint maintenance failed: {}", e);
                } else {
                    tracing::trace!("✅ Checkpoint maintenance completed");
                }
            }
        }
    );

    tracing::info!("🔧 Checkpoint maintenance task spawned");
}

/// Complete integration example for asf_service.rs
///
/// This shows all the pieces together
#[allow(dead_code)]
pub mod complete_example {
    use super::*;

    /// Add this to the imports section of asf_service.rs:
    /// ```rust,ignore
    /// use checkpoint_db::{CheckpointDB, CheckpointStorageAdapter};
    /// use finality_gadget::persistence::{CheckpointPersistenceConfig, CheckpointStorage};
    /// ```

    /// Add this struct near the configuration types:
    pub struct ExampleConfig {
        /// Base path for node data
        pub base_path: PathBuf,

        /// Checkpoint storage configuration
        pub checkpoint_storage: CheckpointStorageConfig,
    }

    /// Insert this code in new_full() after creating the client:
    /// ```rust,ignore
    /// // =========================================================================
    /// // CHECKPOINT PERSISTENCE SETUP
    /// // =========================================================================
    ///
    /// let checkpoint_storage_config = CheckpointStorageConfig::default();
    ///
    /// let checkpoint_storage = if checkpoint_storage_config.enabled {
    ///     let checkpoint_db_path = config.database.path().join("checkpoints");
    ///
    ///     // Create database directory
    ///     std::fs::create_dir_all(&checkpoint_db_path)
    ///         .map_err(|e| ServiceError::Other(format!("Failed to create checkpoint db: {}", e)))?;
    ///
    ///     // Open checkpoint database
    ///     let checkpoint_db = Arc::new(
    ///         CheckpointDB::new(&checkpoint_db_path)
    ///             .map_err(|e| ServiceError::Other(format!("Failed to open checkpoint db: {}", e)))?
    ///     );
    ///
    ///     // Create adapter
    ///     Some(Arc::new(CheckpointStorageAdapter::new(checkpoint_db)) as Arc<dyn CheckpointStorage>)
    /// } else {
    ///     None
    /// };
    ///
    /// tracing::info!("🗄️  Checkpoint storage initialized: enabled={}", checkpoint_storage.is_some());
    /// ```

    /// Insert this code where CheckpointFinality is created:
    /// ```rust,ignore
    /// // Create checkpoint finality
    /// let mut checkpoint_finality = CheckpointFinality::new(
    ///     validator_id,
    ///     max_validators,
    ///     canonical_chain_client,
    ///     network_bridge,
    /// );
    ///
    /// // Attach persistent storage if enabled
    /// if let Some(storage) = checkpoint_storage {
    ///     let persistence_config = CheckpointPersistenceConfig {
    ///         enabled: true,
    ///         retention_checkpoints: checkpoint_storage_config.retention_checkpoints,
    ///         flush_interval_secs: checkpoint_storage_config.flush_interval_secs,
    ///         prune_interval_secs: checkpoint_storage_config.prune_interval_secs,
    ///     };
    ///
    ///     checkpoint_finality.set_storage(storage, persistence_config);
    ///
    ///     // Restore state from disk
    ///     checkpoint_finality.restore_from_storage().await
    ///         .map_err(|e| ServiceError::Other(format!("Failed to restore checkpoints: {}", e)))?;
    ///
    ///     tracing::info!("✅ Checkpoint finality restored from persistent storage");
    /// }
    ///
    /// // Wrap in Arc<Mutex> for shared access
    /// let checkpoint_finality = Arc::new(tokio::sync::Mutex::new(checkpoint_finality));
    /// ```

    /// Insert this code after spawning other essential tasks:
    /// ```rust,ignore
    /// // Spawn checkpoint maintenance task
    /// task_manager.spawn_essential_handle().spawn(
    ///     "checkpoint-maintenance",
    ///     None,
    ///     {
    ///         let checkpoint_finality = checkpoint_finality.clone();
    ///         async move {
    ///             let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
    ///
    ///             loop {
    ///                 interval.tick().await;
    ///
    ///                 let mut cf = checkpoint_finality.lock().await;
    ///                 if let Err(e) = cf.perform_periodic_maintenance().await {
    ///                     tracing::error!("❌ Checkpoint maintenance failed: {}", e);
    ///                 }
    ///             }
    ///         }
    ///     }
    /// );
    ///
    /// tracing::info!("🔧 Checkpoint maintenance task spawned");
    /// ```
}

/// CLI flag integration
///
/// Add this to cli.rs to enable/disable persistence via command line:
/// ```rust,ignore
/// #[derive(Debug, clap::Parser)]
/// pub struct Cli {
///     // ... existing fields ...
///
///     /// Enable checkpoint persistence (stores checkpoint signatures and certificates to disk)
///     #[arg(long, default_value = "true")]
///     pub enable_checkpoint_persistence: bool,
///
///     /// Checkpoint database path (relative to base path)
///     #[arg(long, default_value = "checkpoints")]
///     pub checkpoint_db_path: String,
///
///     /// Number of checkpoints to retain in storage
///     #[arg(long, default_value = "1000")]
///     pub checkpoint_retention: u64,
/// }
/// ```
