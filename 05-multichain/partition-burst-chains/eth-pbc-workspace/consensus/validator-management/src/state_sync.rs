//! State Synchronization for Validator Management
//!
//! SECURITY: This module provides READ-ONLY caching of on-chain validator state.
//! It NEVER overrides chain state - all synced data must be validated against on-chain storage.
//!
//! # Security Guarantees
//!
//! 1. **Read-Only**: This module ONLY caches data, never writes to chain state
//! 2. **Validation Required**: All cached data must be validated against on-chain pallet state
//! 3. **No Authority**: Cached state has no authoritative power over consensus
//! 4. **Performance Only**: Exists solely to reduce redundant chain queries
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │   On-Chain Pallet State (SOURCE)    │
//! │   ✓ Authoritative                   │
//! │   ✓ Consensus-validated             │
//! └──────────────┬──────────────────────┘
//!                │
//!                │ Read & Validate
//!                ▼
//! ┌─────────────────────────────────────┐
//! │   StateSyncManager (CACHE)          │
//! │   ✓ Read-only optimization          │
//! │   ✓ Must validate against chain     │
//! │   ✓ No authoritative state changes  │
//! └─────────────────────────────────────┘
//! ```

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::{ValidatorInfo, ValidatorId, ValidatorResult, ValidatorError};

// ═══════════════════════════════════════════════════════════════════════════════
// STATE SYNC MANAGER - READ-ONLY CACHE
// ═══════════════════════════════════════════════════════════════════════════════

/// State sync manager - READ-ONLY cache of on-chain validator state
///
/// # Security Model
///
/// This struct provides a performance optimization by caching validator data
/// from the on-chain pallet state. It has NO authority to modify chain state.
///
/// ## Usage Pattern
///
/// ```ignore
/// // 1. Query chain state (AUTHORITATIVE)
/// let chain_validator = pallet::Validators::get(&validator_id)?;
///
/// // 2. Update cache for future reads (OPTIMIZATION)
/// state_sync.update_validator(chain_validator.clone());
///
/// // 3. For subsequent reads, try cache first
/// if let Some(cached) = state_sync.get_cached(&validator_id) {
///     // Use cached data for display/queries
/// } else {
///     // Fallback to chain query
///     let chain_data = pallet::Validators::get(&validator_id)?;
/// }
///
/// // 4. CRITICAL: Always validate cached data before consensus decisions
/// let chain_validator = pallet::Validators::get(&validator_id)?;
/// if chain_validator.stake != cached.stake {
///     // Chain state is authoritative, update cache
///     state_sync.update_validator(chain_validator);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct StateSyncManager {
    /// Cached validator info (MUST be validated against chain)
    ///
    /// SECURITY: This is a READ-ONLY cache. Never use this for authoritative
    /// consensus decisions without first validating against on-chain state.
    cached_validators: BTreeMap<ValidatorId, ValidatorInfo>,

    /// Last block number when sync occurred
    last_sync_block: u64,

    /// Sync interval in blocks
    sync_interval: u32,

    /// Total cache hits (for monitoring)
    cache_hits: u64,

    /// Total cache misses (for monitoring)
    cache_misses: u64,
}

impl StateSyncManager {
    /// Create new state sync manager
    ///
    /// # Arguments
    ///
    /// * `sync_interval` - Number of blocks between full sync operations
    ///
    /// # Example
    ///
    /// ```ignore
    /// let manager = StateSyncManager::new(100); // Sync every 100 blocks
    /// ```
    pub fn new(sync_interval: u32) -> Self {
        Self {
            cached_validators: BTreeMap::new(),
            last_sync_block: 0,
            sync_interval,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Check if sync is needed based on current block number
    ///
    /// # Arguments
    ///
    /// * `current_block` - Current blockchain block number
    ///
    /// # Returns
    ///
    /// `true` if enough blocks have passed to warrant a full sync
    pub fn needs_sync(&self, current_block: u64) -> bool {
        current_block >= self.last_sync_block + self.sync_interval as u64
    }

    /// Update cached validator info
    ///
    /// # Security Warning
    ///
    /// This function MUST only be called with data that has been read from
    /// and validated against the on-chain pallet state. Never call this with
    /// data from untrusted sources.
    ///
    /// # Arguments
    ///
    /// * `info` - Validator info that has been validated against chain state
    ///
    /// # Example
    ///
    /// ```ignore
    /// // CORRECT: Validate against chain first
    /// let chain_data = pallet::Validators::get(&validator_id)?;
    /// state_sync.update_validator(chain_data);
    ///
    /// // INCORRECT: Never cache unvalidated data
    /// // state_sync.update_validator(untrusted_data); // ❌ DANGEROUS
    /// ```
    pub fn update_validator(&mut self, info: ValidatorInfo) {
        self.cached_validators.insert(info.id.clone(), info);
    }

    /// Update multiple validators in batch
    ///
    /// # Security Warning
    ///
    /// All validator info MUST be validated against on-chain state first.
    ///
    /// # Arguments
    ///
    /// * `validators` - Vector of validated validator info
    pub fn update_validators(&mut self, validators: Vec<ValidatorInfo>) {
        for validator in validators {
            self.cached_validators.insert(validator.id.clone(), validator);
        }
    }

    /// Get cached validator info (READ-ONLY)
    ///
    /// # Security Warning
    ///
    /// Cached data should only be used for:
    /// - Display purposes
    /// - Non-critical queries
    /// - Performance optimization
    ///
    /// For consensus-critical decisions, ALWAYS validate against chain state.
    ///
    /// # Arguments
    ///
    /// * `id` - Validator ID to lookup
    ///
    /// # Returns
    ///
    /// Cached validator info if available, None otherwise
    pub fn get_cached(&mut self, id: &ValidatorId) -> Option<&ValidatorInfo> {
        match self.cached_validators.get(id) {
            Some(info) => {
                self.cache_hits += 1;
                Some(info)
            }
            None => {
                self.cache_misses += 1;
                None
            }
        }
    }

    /// Get cached validator info without updating metrics (for testing)
    pub fn get_cached_readonly(&self, id: &ValidatorId) -> Option<&ValidatorInfo> {
        self.cached_validators.get(id)
    }

    /// Remove validator from cache
    ///
    /// Used when a validator is removed from chain state.
    ///
    /// # Arguments
    ///
    /// * `id` - Validator ID to remove
    pub fn remove_validator(&mut self, id: &ValidatorId) -> Option<ValidatorInfo> {
        self.cached_validators.remove(id)
    }

    /// Mark sync complete for the current block
    ///
    /// # Arguments
    ///
    /// * `block_number` - Block number when sync completed
    pub fn mark_synced(&mut self, block_number: u64) {
        self.last_sync_block = block_number;
    }

    /// Clear entire cache
    ///
    /// Used when cache becomes stale or during reset operations.
    pub fn clear(&mut self) {
        self.cached_validators.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }

    /// Get all cached validators
    ///
    /// # Security Warning
    ///
    /// This returns cached data. For authoritative validator lists,
    /// query the on-chain pallet state directly.
    ///
    /// # Returns
    ///
    /// Vector of all cached validator info
    pub fn all_cached(&self) -> Vec<ValidatorInfo> {
        self.cached_validators.values().cloned().collect()
    }

    /// Get number of validators in cache
    pub fn cached_count(&self) -> usize {
        self.cached_validators.len()
    }

    /// Get cache efficiency metrics
    ///
    /// # Returns
    ///
    /// Tuple of (hits, misses, hit_rate_percentage)
    pub fn cache_metrics(&self) -> (u64, u64, f64) {
        let total = self.cache_hits + self.cache_misses;
        let hit_rate = if total > 0 {
            (self.cache_hits as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        (self.cache_hits, self.cache_misses, hit_rate)
    }

    /// Check if validator exists in cache
    pub fn has_cached(&self, id: &ValidatorId) -> bool {
        self.cached_validators.contains_key(id)
    }

    /// Get last sync block number
    pub fn last_sync_block(&self) -> u64 {
        self.last_sync_block
    }

    /// Get sync interval
    pub fn sync_interval(&self) -> u32 {
        self.sync_interval
    }

    /// Set sync interval
    pub fn set_sync_interval(&mut self, interval: u32) {
        self.sync_interval = interval;
    }

    /// Validate cached data against on-chain state
    ///
    /// This should be called periodically to ensure cache coherence.
    ///
    /// # Arguments
    ///
    /// * `id` - Validator to validate
    /// * `chain_validator` - Authoritative on-chain validator info
    ///
    /// # Returns
    ///
    /// `Ok(true)` if cache matches chain, `Ok(false)` if mismatch (cache updated),
    /// `Err` if validator not in cache
    pub fn validate_cached(
        &mut self,
        id: &ValidatorId,
        chain_validator: &ValidatorInfo,
    ) -> ValidatorResult<bool> {
        match self.cached_validators.get(id) {
            Some(cached) => {
                if cached != chain_validator {
                    // Cache is stale, update it
                    self.update_validator(chain_validator.clone());
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            None => Err(ValidatorError::NotFound),
        }
    }
}

impl Default for StateSyncManager {
    fn default() -> Self {
        Self::new(100) // Sync every 100 blocks by default
    }
}

/// Type alias for use in coordinator
pub type StateSync = StateSyncManager;

// ═══════════════════════════════════════════════════════════════════════════════
// SYNC STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// State synchronization statistics
#[derive(Debug, Clone, Default)]
pub struct SyncStats {
    /// Number of validators in cache
    pub cached_validators: usize,

    /// Last sync block number
    pub last_sync_block: u64,

    /// Next sync block number
    pub next_sync_block: u64,

    /// Cache hit count
    pub cache_hits: u64,

    /// Cache miss count
    pub cache_misses: u64,

    /// Cache hit rate (0.0 - 1.0)
    pub cache_hit_rate: f64,
}

impl StateSyncManager {
    /// Get comprehensive sync statistics
    pub fn get_stats(&self) -> SyncStats {
        let total_queries = self.cache_hits + self.cache_misses;
        let hit_rate = if total_queries > 0 {
            self.cache_hits as f64 / total_queries as f64
        } else {
            0.0
        };

        SyncStats {
            cached_validators: self.cached_validators.len(),
            last_sync_block: self.last_sync_block,
            next_sync_block: self.last_sync_block + self.sync_interval as u64,
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            cache_hit_rate: hit_rate,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PeerType;

    fn create_test_validator(id: u8, stake: u128) -> ValidatorInfo {
        ValidatorInfo {
            id: ValidatorId::from([id; 32]),
            stake,
            reputation: 100,
            peer_type: PeerType::ValidityNode,
            last_block: 0,
            active: true,
            last_epoch: 0,
            blocks_produced: 0,
            certificates_issued: 0,
            network_address: None,
        }
    }

    #[test]
    fn test_state_sync_creation() {
        let manager = StateSyncManager::new(100);
        assert_eq!(manager.sync_interval, 100);
        assert_eq!(manager.last_sync_block, 0);
        assert_eq!(manager.cached_count(), 0);
    }

    #[test]
    fn test_default_sync_manager() {
        let manager = StateSyncManager::default();
        assert_eq!(manager.sync_interval, 100);
    }

    #[test]
    fn test_needs_sync() {
        let manager = StateSyncManager::new(100);

        // At block 0, should need sync at 100
        assert!(!manager.needs_sync(50));
        assert!(!manager.needs_sync(99));
        assert!(manager.needs_sync(100));
        assert!(manager.needs_sync(150));
    }

    #[test]
    fn test_needs_sync_after_mark_synced() {
        let mut manager = StateSyncManager::new(100);
        manager.mark_synced(100);

        assert!(!manager.needs_sync(150));
        assert!(manager.needs_sync(200));
        assert!(manager.needs_sync(250));
    }

    #[test]
    fn test_cache_validator() {
        let mut manager = StateSyncManager::new(100);
        let validator = create_test_validator(1, 1000);

        manager.update_validator(validator.clone());
        assert_eq!(manager.cached_count(), 1);

        let cached = manager.get_cached(&validator.id).unwrap();
        assert_eq!(cached.id, validator.id);
        assert_eq!(cached.stake, 1000);
    }

    #[test]
    fn test_cache_multiple_validators() {
        let mut manager = StateSyncManager::new(100);

        let validators = vec![
            create_test_validator(1, 1000),
            create_test_validator(2, 2000),
            create_test_validator(3, 3000),
        ];

        manager.update_validators(validators.clone());
        assert_eq!(manager.cached_count(), 3);

        for validator in validators {
            let cached = manager.get_cached(&validator.id).unwrap();
            assert_eq!(cached.stake, validator.stake);
        }
    }

    #[test]
    fn test_cache_metrics() {
        let mut manager = StateSyncManager::new(100);
        let v1 = create_test_validator(1, 1000);
        let v2 = create_test_validator(2, 2000);

        manager.update_validator(v1.clone());

        // Hit
        assert!(manager.get_cached(&v1.id).is_some());

        // Miss
        assert!(manager.get_cached(&v2.id).is_none());

        // Hit again
        assert!(manager.get_cached(&v1.id).is_some());

        let (hits, misses, hit_rate) = manager.cache_metrics();
        assert_eq!(hits, 2);
        assert_eq!(misses, 1);
        assert!((hit_rate - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_remove_validator() {
        let mut manager = StateSyncManager::new(100);
        let validator = create_test_validator(1, 1000);

        manager.update_validator(validator.clone());
        assert_eq!(manager.cached_count(), 1);

        let removed = manager.remove_validator(&validator.id);
        assert!(removed.is_some());
        assert_eq!(manager.cached_count(), 0);
    }

    #[test]
    fn test_clear_cache() {
        let mut manager = StateSyncManager::new(100);

        manager.update_validators(vec![
            create_test_validator(1, 1000),
            create_test_validator(2, 2000),
        ]);

        assert_eq!(manager.cached_count(), 2);

        manager.clear();
        assert_eq!(manager.cached_count(), 0);
        assert_eq!(manager.cache_hits, 0);
        assert_eq!(manager.cache_misses, 0);
    }

    #[test]
    fn test_all_cached() {
        let mut manager = StateSyncManager::new(100);

        let validators = vec![
            create_test_validator(1, 1000),
            create_test_validator(2, 2000),
            create_test_validator(3, 3000),
        ];

        manager.update_validators(validators.clone());

        let all = manager.all_cached();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_has_cached() {
        let mut manager = StateSyncManager::new(100);
        let validator = create_test_validator(1, 1000);

        assert!(!manager.has_cached(&validator.id));

        manager.update_validator(validator.clone());
        assert!(manager.has_cached(&validator.id));
    }

    #[test]
    fn test_validate_cached_match() {
        let mut manager = StateSyncManager::new(100);
        let validator = create_test_validator(1, 1000);

        manager.update_validator(validator.clone());

        let result = manager.validate_cached(&validator.id, &validator);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should match
    }

    #[test]
    fn test_validate_cached_mismatch() {
        let mut manager = StateSyncManager::new(100);
        let mut validator = create_test_validator(1, 1000);

        manager.update_validator(validator.clone());

        // Simulate chain state change
        validator.stake = 2000;

        let result = manager.validate_cached(&validator.id, &validator);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should NOT match

        // Cache should be updated
        let cached = manager.get_cached_readonly(&validator.id).unwrap();
        assert_eq!(cached.stake, 2000);
    }

    #[test]
    fn test_validate_cached_not_found() {
        let mut manager = StateSyncManager::new(100);
        let validator = create_test_validator(1, 1000);

        let result = manager.validate_cached(&validator.id, &validator);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidatorError::NotFound);
    }

    #[test]
    fn test_get_stats() {
        let mut manager = StateSyncManager::new(100);

        manager.update_validators(vec![
            create_test_validator(1, 1000),
            create_test_validator(2, 2000),
        ]);

        manager.mark_synced(50);

        let stats = manager.get_stats();
        assert_eq!(stats.cached_validators, 2);
        assert_eq!(stats.last_sync_block, 50);
        assert_eq!(stats.next_sync_block, 150);
    }

    #[test]
    fn test_set_sync_interval() {
        let mut manager = StateSyncManager::new(100);
        assert_eq!(manager.sync_interval(), 100);

        manager.set_sync_interval(200);
        assert_eq!(manager.sync_interval(), 200);
    }

    #[test]
    fn test_read_only_guarantee() {
        // This test demonstrates the read-only nature
        let mut manager = StateSyncManager::new(100);
        let validator = create_test_validator(1, 1000);

        // 1. Cache validator
        manager.update_validator(validator.clone());

        // 2. Get cached (read-only)
        let cached = manager.get_cached(&validator.id).unwrap();

        // 3. Cached data is a reference, cannot modify
        // The following would not compile:
        // cached.stake = 2000; // ❌ Cannot modify through shared reference

        assert_eq!(cached.stake, 1000);
    }

    #[test]
    fn test_security_cache_only_stores_validated_data() {
        // This test demonstrates that the cache should only store
        // data that has been validated against chain state

        let mut manager = StateSyncManager::new(100);

        // Simulate: Data read from chain (authoritative)
        let chain_validator = create_test_validator(1, 64_000_000_000_000_000_000_000);

        // ✓ CORRECT: Cache chain-validated data
        manager.update_validator(chain_validator.clone());

        let cached = manager.get_cached(&chain_validator.id).unwrap();
        assert_eq!(cached.stake, chain_validator.stake);

        // The cache now contains validated data
        // In real usage, this would be checked against chain state regularly
    }
}
