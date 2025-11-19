//! # Checkpoint Finality Persistence Extension
//!
//! This module extends CheckpointFinality with persistent storage capabilities.
//!
//! ## Features
//!
//! - Automatic persistence of checkpoint signatures and certificates
//! - Recovery from disk on node restart
//! - Periodic auto-flush to prevent data loss
//! - Configurable retention and pruning
//!
//! ## Usage
//!
//! ```rust,ignore
//! let checkpoint_db = Arc::new(CheckpointStorageAdapter::new(db_path)?);
//! let mut checkpoint_finality = CheckpointFinality::new(...);
//! checkpoint_finality.set_storage(checkpoint_db.clone());
//! checkpoint_finality.restore_from_storage().await?;
//! ```

use crate::*;
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// CHECKPOINT STORAGE TRAIT
// ============================================================================

/// Trait for checkpoint persistent storage
///
/// Implementations should provide atomic operations and handle errors gracefully.
#[async_trait::async_trait]
pub trait CheckpointStorage: Send + Sync {
    /// Store a checkpoint signature (append to existing list)
    async fn store_signature(
        &self,
        checkpoint: CheckpointNumber,
        signature: &CheckpointSignature,
    ) -> Result<(), String>;

    /// Store a checkpoint certificate (overwrites existing)
    async fn store_certificate(
        &self,
        checkpoint: CheckpointNumber,
        certificate: &CheckpointCertificate,
    ) -> Result<(), String>;

    /// Get all signatures for a checkpoint
    async fn get_signatures(
        &self,
        checkpoint: CheckpointNumber,
    ) -> Result<Vec<CheckpointSignature>, String>;

    /// Get certificate for a checkpoint
    async fn get_certificate(
        &self,
        checkpoint: CheckpointNumber,
    ) -> Result<Option<CheckpointCertificate>, String>;

    /// Get the last finalized checkpoint number
    async fn get_last_finalized(&self) -> Result<u64, String>;

    /// Set the last finalized checkpoint number
    async fn set_last_finalized(&self, checkpoint: u64) -> Result<(), String>;

    /// Prune old checkpoint data, keeping last N checkpoints
    async fn prune_old_checkpoints(&self, keep_last_n: u64) -> Result<(), String>;

    /// Flush all pending writes to disk
    async fn flush(&self) -> Result<(), String>;
}

// ============================================================================
// CHECKPOINT FINALITY PERSISTENCE EXTENSION
// ============================================================================

/// Configuration for checkpoint persistence
#[derive(Clone, Debug)]
pub struct CheckpointPersistenceConfig {
    /// Enable persistence (default: true)
    pub enabled: bool,

    /// Number of checkpoints to retain in storage (default: 1000)
    pub retention_checkpoints: u64,

    /// Auto-flush interval in seconds (default: 10)
    pub flush_interval_secs: u64,

    /// Prune interval in seconds (default: 300 = 5 minutes)
    pub prune_interval_secs: u64,
}

impl Default for CheckpointPersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_checkpoints: 1000,
            flush_interval_secs: 10,
            prune_interval_secs: 300,
        }
    }
}

/// Extension methods for CheckpointFinality to add persistence
pub struct CheckpointFinalityPersistence {
    storage: Arc<dyn CheckpointStorage>,
    config: CheckpointPersistenceConfig,
    last_flush_time: Instant,
    last_prune_time: Instant,
}

impl CheckpointFinalityPersistence {
    /// Create a new persistence manager
    pub fn new(
        storage: Arc<dyn CheckpointStorage>,
        config: CheckpointPersistenceConfig,
    ) -> Self {
        Self {
            storage,
            config,
            last_flush_time: Instant::now(),
            last_prune_time: Instant::now(),
        }
    }

    /// Check if auto-flush is due
    pub fn should_flush(&self) -> bool {
        self.last_flush_time.elapsed() >= Duration::from_secs(self.config.flush_interval_secs)
    }

    /// Check if pruning is due
    pub fn should_prune(&self) -> bool {
        self.last_prune_time.elapsed() >= Duration::from_secs(self.config.prune_interval_secs)
    }

    /// Perform auto-flush if needed
    pub async fn auto_flush(&mut self) -> Result<(), String> {
        if !self.config.enabled || !self.should_flush() {
            return Ok(());
        }

        tracing::debug!("💾 Auto-flushing checkpoint database");
        self.storage.flush().await?;
        self.last_flush_time = Instant::now();

        Ok(())
    }

    /// Perform auto-pruning if needed
    pub async fn auto_prune(&mut self, last_finalized: u64) -> Result<(), String> {
        if !self.config.enabled || !self.should_prune() {
            return Ok(());
        }

        tracing::debug!("🧹 Auto-pruning old checkpoint data");
        self.storage.prune_old_checkpoints(self.config.retention_checkpoints).await?;
        self.last_prune_time = Instant::now();

        tracing::info!(
            "✅ Pruning complete: keeping last {} checkpoints from finalized checkpoint {}",
            self.config.retention_checkpoints,
            last_finalized
        );

        Ok(())
    }

    /// Store a signature with automatic persistence
    pub async fn persist_signature(
        &self,
        checkpoint: CheckpointNumber,
        signature: &CheckpointSignature,
    ) -> Result<(), String> {
        if !self.config.enabled {
            return Ok(());
        }

        self.storage.store_signature(checkpoint, signature).await
    }

    /// Store a certificate with automatic persistence
    pub async fn persist_certificate(
        &self,
        checkpoint: CheckpointNumber,
        certificate: &CheckpointCertificate,
    ) -> Result<(), String> {
        if !self.config.enabled {
            return Ok(());
        }

        self.storage.store_certificate(checkpoint, certificate).await
    }

    /// Update last finalized checkpoint in storage
    pub async fn persist_last_finalized(&self, checkpoint: u64) -> Result<(), String> {
        if !self.config.enabled {
            return Ok(());
        }

        self.storage.set_last_finalized(checkpoint).await
    }

    /// Restore checkpoint state from storage
    pub async fn restore_state(
        &self,
    ) -> Result<RestoredCheckpointState, String> {
        if !self.config.enabled {
            return Ok(RestoredCheckpointState::default());
        }

        tracing::info!("🔄 Restoring checkpoint state from storage");

        // Load last finalized checkpoint
        let last_finalized = self.storage.get_last_finalized().await?;

        tracing::info!("📍 Last finalized checkpoint: {}", last_finalized);

        // Load recent signatures (last 100 checkpoints)
        let mut all_signatures = HashMap::new();
        let start_checkpoint = last_finalized.saturating_sub(100);

        for checkpoint_num in start_checkpoint..=last_finalized + 10 {
            let checkpoint = CheckpointNumber(checkpoint_num);
            let signatures = self.storage.get_signatures(checkpoint).await?;

            if !signatures.is_empty() {
                tracing::debug!(
                    "📝 Restored {} signatures for checkpoint {}",
                    signatures.len(),
                    checkpoint_num
                );
                all_signatures.insert(checkpoint, signatures);
            }
        }

        // Load recent certificates (last 100 checkpoints)
        let mut all_certificates = HashMap::new();

        for checkpoint_num in start_checkpoint..=last_finalized {
            let checkpoint = CheckpointNumber(checkpoint_num);
            if let Some(cert) = self.storage.get_certificate(checkpoint).await? {
                tracing::debug!(
                    "📜 Restored certificate for checkpoint {} with {} signatures",
                    checkpoint_num,
                    cert.signatures.len()
                );
                all_certificates.insert(checkpoint, cert);
            }
        }

        tracing::info!(
            "✅ Restored checkpoint state: last_finalized={}, signatures={} checkpoints, certificates={}",
            last_finalized,
            all_signatures.len(),
            all_certificates.len()
        );

        Ok(RestoredCheckpointState {
            last_finalized: CheckpointNumber(last_finalized),
            signatures: all_signatures,
            certificates: all_certificates,
        })
    }
}

/// Restored checkpoint state from storage
#[derive(Debug, Default)]
pub struct RestoredCheckpointState {
    pub last_finalized: CheckpointNumber,
    pub signatures: HashMap<CheckpointNumber, Vec<CheckpointSignature>>,
    pub certificates: HashMap<CheckpointNumber, CheckpointCertificate>,
}

// ============================================================================
// CHECKPOINT FINALITY PERSISTENCE METHODS
// ============================================================================

impl CheckpointFinality {
    /// Set persistent storage for checkpoints
    ///
    /// This enables automatic persistence of all checkpoint operations.
    pub fn set_storage(
        &mut self,
        storage: Arc<dyn CheckpointStorage>,
        config: CheckpointPersistenceConfig,
    ) {
        self.persistence = Some(CheckpointFinalityPersistence::new(storage, config));

        tracing::info!(
            "💾 Checkpoint persistence enabled: retention={} checkpoints, flush_interval={}s",
            config.retention_checkpoints,
            config.flush_interval_secs
        );
    }

    /// Restore checkpoint state from persistent storage
    ///
    /// Call this immediately after setting storage on node startup.
    pub async fn restore_from_storage(&mut self) -> Result<(), String> {
        let persistence = match &self.persistence {
            Some(p) => p,
            None => {
                tracing::warn!("⚠️ No persistence configured, skipping restore");
                return Ok(());
            }
        };

        let state = persistence.restore_state().await?;

        // Restore last finalized checkpoint
        self.last_finalized_checkpoint = state.last_finalized;

        // Restore signatures
        self.checkpoint_signatures = state.signatures.clone();

        // Restore certificates
        self.checkpoint_certificates = state.certificates;

        // Rebuild validator_signatures map for double-sign detection
        for (checkpoint, signatures) in state.signatures {
            for sig in signatures {
                let key = (sig.validator_id, checkpoint);
                self.validator_signatures
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push((sig.block_hash, sig.authority_set_id));
            }
        }

        tracing::info!(
            "🎉 Checkpoint state restored successfully: last_finalized={}",
            self.last_finalized_checkpoint.0
        );

        Ok(())
    }

    /// Persist a checkpoint signature (called automatically when signature is added)
    async fn persist_signature(&self, signature: &CheckpointSignature) -> Result<(), String> {
        if let Some(ref persistence) = self.persistence {
            persistence.persist_signature(signature.checkpoint_number, signature).await?;
        }
        Ok(())
    }

    /// Persist a checkpoint certificate (called automatically when certificate is created)
    async fn persist_certificate(&self, certificate: &CheckpointCertificate) -> Result<(), String> {
        if let Some(ref persistence) = self.persistence {
            persistence.persist_certificate(certificate.checkpoint_number, certificate).await?;
        }
        Ok(())
    }

    /// Persist last finalized checkpoint (called automatically when finality advances)
    async fn persist_last_finalized(&self) -> Result<(), String> {
        if let Some(ref persistence) = self.persistence {
            persistence.persist_last_finalized(self.last_finalized_checkpoint.0).await?;
        }
        Ok(())
    }

    /// Perform periodic maintenance (flush, prune)
    ///
    /// Call this periodically (e.g., every 10 seconds) from the worker loop.
    pub async fn perform_periodic_maintenance(&mut self) -> Result<(), String> {
        if let Some(ref mut persistence) = self.persistence {
            // Auto-flush
            persistence.auto_flush().await?;

            // Auto-prune
            persistence.auto_prune(self.last_finalized_checkpoint.0).await?;
        }

        Ok(())
    }
}

// ============================================================================
// EXTEND EXISTING METHODS WITH PERSISTENCE HOOKS
// ============================================================================

// Note: The following are integration points where persistence should be called.
// These need to be integrated into the existing CheckpointFinality implementation:

// 1. In add_checkpoint_signature() - after adding signature:
//    self.persist_signature(&signature).await?;

// 2. In create_checkpoint_certificate() - after creating certificate:
//    self.persist_certificate(&certificate).await?;

// 3. In advance_finality() - after advancing finality:
//    self.persist_last_finalized().await?;

// 4. In run_worker() or similar periodic task - every 10 seconds:
//    self.perform_periodic_maintenance().await?;
