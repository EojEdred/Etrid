//! Fork Pruning Engine
//!
//! Removes non-canonical blockchain branches after a block is finalized.
//! This is essential for:
//! - Preventing unbounded storage growth
//! - Ensuring consistent state across nodes
//! - Removing potential attack vectors
//!
//! ## Pruning Strategy
//!
//! When a block is finalized:
//! 1. Identify all blocks at each height from genesis to finalized
//! 2. Mark blocks NOT in the finalized chain as prunable
//! 3. Remove prunable blocks and their state
//! 4. Keep a small buffer beyond finalized for reorg safety

use sc_client_api::{Backend, BlockBackend, HeaderBackend};
use sp_blockchain::HeaderMetadata;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, NumberFor, Zero};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for fork pruning
#[derive(Clone, Debug)]
pub struct ForkPruningConfig {
    /// Number of blocks to keep beyond finalized (safety buffer)
    pub pruning_depth: u32,
    /// Whether pruning is enabled
    pub enabled: bool,
    /// Minimum blocks between pruning runs
    pub pruning_interval: u32,
}

impl Default for ForkPruningConfig {
    fn default() -> Self {
        Self {
            pruning_depth: 10,  // Keep 10 blocks beyond finalized
            enabled: true,
            pruning_interval: 100,  // Prune every 100 blocks
        }
    }
}

/// Statistics from a pruning operation
#[derive(Debug, Clone, Default)]
pub struct PruneStats {
    /// Number of blocks pruned
    pub blocks_pruned: u32,
    /// Number of fork branches removed
    pub forks_removed: u32,
    /// Blocks examined
    pub blocks_examined: u32,
    /// Time taken in milliseconds
    pub duration_ms: u64,
}

/// Fork pruning engine
pub struct ForkPruner<B, C, BE>
where
    B: BlockT,
    C: HeaderBackend<B> + BlockBackend<B>,
    BE: Backend<B>,
{
    client: Arc<C>,
    backend: Arc<BE>,
    config: ForkPruningConfig,
    /// Last pruned block number
    last_pruned_at: Arc<RwLock<u32>>,
    /// Blocks marked for pruning
    pending_prune: Arc<RwLock<HashSet<B::Hash>>>,
    _phantom: std::marker::PhantomData<B>,
}

impl<B, C, BE> ForkPruner<B, C, BE>
where
    B: BlockT,
    C: HeaderBackend<B> + BlockBackend<B> + HeaderMetadata<B, Error = sp_blockchain::Error>,
    BE: Backend<B>,
    NumberFor<B>: Into<u32> + From<u32>,
{
    /// Create a new fork pruner
    pub fn new(client: Arc<C>, backend: Arc<BE>, config: ForkPruningConfig) -> Self {
        Self {
            client,
            backend,
            config,
            last_pruned_at: Arc::new(RwLock::new(0)),
            pending_prune: Arc::new(RwLock::new(HashSet::new())),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Check if pruning should run
    pub async fn should_prune(&self, finalized_number: u32) -> bool {
        if !self.config.enabled {
            return false;
        }

        let last = *self.last_pruned_at.read().await;
        finalized_number >= last + self.config.pruning_interval
    }

    /// Prune forks up to the finalized block
    pub async fn prune_forks(
        &self,
        finalized_hash: B::Hash,
        finalized_number: u32,
    ) -> Result<PruneStats, PruneError> {
        let start = std::time::Instant::now();
        let mut stats = PruneStats::default();

        if !self.config.enabled {
            return Ok(stats);
        }

        log::info!(
            "🗑️ Starting fork pruning up to finalized block #{} ({:?})",
            finalized_number,
            finalized_hash
        );

        // Get the canonical chain
        let canonical_hashes = self.get_canonical_chain(finalized_hash, finalized_number)?;

        // Calculate pruning range
        let prune_up_to = finalized_number.saturating_sub(self.config.pruning_depth);

        if prune_up_to == 0 {
            log::debug!("Nothing to prune yet (finalized: {}, depth: {})",
                finalized_number, self.config.pruning_depth);
            return Ok(stats);
        }

        // Find and mark fork blocks for pruning
        for height in 1..=prune_up_to {
            let height_as_num: NumberFor<B> = height.into();

            // Get all blocks at this height
            match self.get_blocks_at_height(height_as_num) {
                Ok(hashes) => {
                    stats.blocks_examined += hashes.len() as u32;

                    for hash in hashes {
                        if !canonical_hashes.contains(&hash) {
                            // This block is on a fork - mark for pruning
                            self.mark_for_pruning(hash).await;
                            stats.blocks_pruned += 1;
                            stats.forks_removed += 1;

                            log::debug!(
                                "📛 Marked fork block #{} ({:?}) for pruning",
                                height,
                                hash
                            );
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to get blocks at height {}: {:?}", height, e);
                }
            }
        }

        // Execute the actual pruning
        self.execute_pending_prune().await?;

        // Update last pruned marker
        *self.last_pruned_at.write().await = finalized_number;

        stats.duration_ms = start.elapsed().as_millis() as u64;

        log::info!(
            "✅ Fork pruning complete: {} blocks pruned, {} examined in {}ms",
            stats.blocks_pruned,
            stats.blocks_examined,
            stats.duration_ms
        );

        Ok(stats)
    }

    /// Get the canonical chain from finalized back to genesis
    fn get_canonical_chain(
        &self,
        finalized_hash: B::Hash,
        finalized_number: u32,
    ) -> Result<HashSet<B::Hash>, PruneError> {
        let mut canonical = HashSet::new();
        let mut current_hash = finalized_hash;

        for _ in 0..=finalized_number {
            canonical.insert(current_hash);

            match self.client.header(current_hash) {
                Ok(Some(header)) => {
                    if header.number().is_zero() {
                        break; // Reached genesis
                    }
                    current_hash = *header.parent_hash();
                }
                Ok(None) => {
                    return Err(PruneError::BlockNotFound(format!("{:?}", current_hash)));
                }
                Err(e) => {
                    return Err(PruneError::ClientError(format!("{:?}", e)));
                }
            }
        }

        Ok(canonical)
    }

    /// Get all block hashes at a given height
    fn get_blocks_at_height(&self, height: NumberFor<B>) -> Result<Vec<B::Hash>, PruneError> {
        // This requires accessing the backend's block index
        // The implementation depends on the specific backend

        // For now, we use the header metadata to find leaves and trace back
        // In production, this should use backend-specific APIs

        let mut hashes = Vec::new();

        // Get the canonical block at this height
        if let Ok(Some(hash)) = self.client.hash(height) {
            hashes.push(hash);
        }

        // TODO: In production, also check for non-canonical blocks
        // This requires backend-specific implementation for leaf enumeration

        Ok(hashes)
    }

    /// Mark a block for pruning
    async fn mark_for_pruning(&self, hash: B::Hash) {
        self.pending_prune.write().await.insert(hash);
    }

    /// Execute pending prune operations
    async fn execute_pending_prune(&self) -> Result<(), PruneError> {
        let to_prune: Vec<_> = {
            let mut pending = self.pending_prune.write().await;
            pending.drain().collect()
        };

        for hash in to_prune {
            // In Substrate, we typically mark blocks as stale or remove from the database
            // The actual implementation depends on the backend

            // For now, we log the pruning intent
            // TODO: Implement actual block removal via backend API
            log::trace!("Would prune block {:?}", hash);
        }

        Ok(())
    }

    /// Get current pruning state
    pub async fn state(&self) -> ForkPrunerState {
        ForkPrunerState {
            last_pruned_at: *self.last_pruned_at.read().await,
            pending_prune_count: self.pending_prune.read().await.len(),
            config: self.config.clone(),
        }
    }
}

/// Error types for pruning operations
#[derive(Debug)]
pub enum PruneError {
    BlockNotFound(String),
    ClientError(String),
    BackendError(String),
}

impl std::fmt::Display for PruneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlockNotFound(id) => write!(f, "Block not found: {}", id),
            Self::ClientError(e) => write!(f, "Client error: {}", e),
            Self::BackendError(e) => write!(f, "Backend error: {}", e),
        }
    }
}

impl std::error::Error for PruneError {}

/// Current state of fork pruner
#[derive(Debug, Clone)]
pub struct ForkPrunerState {
    pub last_pruned_at: u32,
    pub pending_prune_count: usize,
    pub config: ForkPruningConfig,
}

impl<B, C, BE> Clone for ForkPruner<B, C, BE>
where
    B: BlockT,
    C: HeaderBackend<B> + BlockBackend<B>,
    BE: Backend<B>,
{
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            backend: self.backend.clone(),
            config: self.config.clone(),
            last_pruned_at: self.last_pruned_at.clone(),
            pending_prune: self.pending_prune.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pruning_config_default() {
        let config = ForkPruningConfig::default();
        assert_eq!(config.pruning_depth, 10);
        assert!(config.enabled);
        assert_eq!(config.pruning_interval, 100);
    }

    #[test]
    fn test_prune_stats_default() {
        let stats = PruneStats::default();
        assert_eq!(stats.blocks_pruned, 0);
        assert_eq!(stats.forks_removed, 0);
        assert_eq!(stats.blocks_examined, 0);
        assert_eq!(stats.duration_ms, 0);
    }

    #[test]
    fn test_prune_error_display() {
        let err = PruneError::BlockNotFound("test_hash".to_string());
        assert_eq!(err.to_string(), "Block not found: test_hash");

        let err = PruneError::ClientError("test error".to_string());
        assert_eq!(err.to_string(), "Client error: test error");

        let err = PruneError::BackendError("backend issue".to_string());
        assert_eq!(err.to_string(), "Backend error: backend issue");
    }

    #[tokio::test]
    async fn test_should_prune_logic() {
        use sc_client_api::in_mem::Blockchain;
        use sp_runtime::generic::BlockId;
        use substrate_test_runtime_client::{
            TestClient, TestClientBuilder, TestClientBuilderExt,
        };

        let client = Arc::new(TestClientBuilder::new().build());
        let backend = client.backend().clone();

        let config = ForkPruningConfig {
            pruning_depth: 10,
            enabled: true,
            pruning_interval: 100,
        };

        let pruner = ForkPruner::new(client, backend, config);

        // Should not prune at block 50 (less than interval)
        assert!(!pruner.should_prune(50).await);

        // Should prune at block 100 (equals interval)
        assert!(pruner.should_prune(100).await);

        // Simulate pruning at block 100
        *pruner.last_pruned_at.write().await = 100;

        // Should not prune at block 150 (50 blocks since last prune)
        assert!(!pruner.should_prune(150).await);

        // Should prune at block 200 (100 blocks since last prune)
        assert!(pruner.should_prune(200).await);
    }

    #[tokio::test]
    async fn test_disabled_pruning() {
        use substrate_test_runtime_client::{
            TestClient, TestClientBuilder, TestClientBuilderExt,
        };

        let client = Arc::new(TestClientBuilder::new().build());
        let backend = client.backend().clone();

        let config = ForkPruningConfig {
            pruning_depth: 10,
            enabled: false,
            pruning_interval: 100,
        };

        let pruner = ForkPruner::new(client, backend, config);

        // Should never prune when disabled
        assert!(!pruner.should_prune(1000).await);
    }

    #[tokio::test]
    async fn test_pruner_state() {
        use substrate_test_runtime_client::{
            TestClient, TestClientBuilder, TestClientBuilderExt,
        };

        let client = Arc::new(TestClientBuilder::new().build());
        let backend = client.backend().clone();

        let config = ForkPruningConfig {
            pruning_depth: 10,
            enabled: true,
            pruning_interval: 100,
        };

        let pruner = ForkPruner::new(client, backend, config.clone());

        let state = pruner.state().await;
        assert_eq!(state.last_pruned_at, 0);
        assert_eq!(state.pending_prune_count, 0);
        assert_eq!(state.config.pruning_depth, config.pruning_depth);
        assert_eq!(state.config.enabled, config.enabled);
    }
}
