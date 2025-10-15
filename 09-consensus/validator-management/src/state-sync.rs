//! # State Synchronization
//!
//! This module handles state synchronization between validators,
//! ensuring all validators have consistent consensus state.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use codec::{Decode, Encode};

use crate::{BlockNumber, Hash, ValidatorError, ValidatorId, ValidatorResult};

// ═══════════════════════════════════════════════════════════════════════════════
// SYNC STATE
// ═══════════════════════════════════════════════════════════════════════════════

/// Synchronization state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum SyncState {
    /// Not syncing
    Idle,
    
    /// Syncing in progress
    Syncing,
    
    /// Sync complete
    Complete,
    
    /// Sync failed
    Failed,
}

/// Sync progress information
#[derive(Debug, Clone, Encode, Decode)]
pub struct SyncProgress {
    /// Current sync state
    pub state: SyncState,
    
    /// Starting block
    pub start_block: BlockNumber,
    
    /// Target block to sync to
    pub target_block: BlockNumber,
    
    /// Current block synced
    pub current_block: BlockNumber,
    
    /// Number of blocks synced
    pub blocks_synced: u64,
    
    /// Sync speed (blocks per second)
    pub sync_speed: u64,
}

impl SyncProgress {
    /// Create new sync progress
    pub fn new(start_block: BlockNumber, target_block: BlockNumber) -> Self {
        Self {
            state: SyncState::Syncing,
            start_block,
            target_block,
            current_block: start_block,
            blocks_synced: 0,
            sync_speed: 0,
        }
    }

    /// Update progress
    pub fn update(&mut self, current_block: BlockNumber, speed: u64) {
        self.current_block = current_block;
        self.blocks_synced = current_block.saturating_sub(self.start_block);
        self.sync_speed = speed;
        
        if current_block >= self.target_block {
            self.state = SyncState::Complete;
        }
    }

    /// Get sync percentage (0-100)
    pub fn percentage(&self) -> u8 {
        if self.target_block <= self.start_block {
            return 100;
        }
        
        let total = self.target_block - self.start_block;
        let done = self.current_block.saturating_sub(self.start_block);
        
        ((done * 100) / total).min(100) as u8
    }

    /// Get estimated time remaining (seconds)
    pub fn estimated_time_remaining(&self) -> Option<u64> {
        if self.sync_speed == 0 {
            return None;
        }
        
        let remaining = self.target_block.saturating_sub(self.current_block);
        Some(remaining / self.sync_speed)
    }

    /// Check if sync is complete
    pub fn is_complete(&self) -> bool {
        self.state == SyncState::Complete || self.current_block >= self.target_block
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// STATE SYNC MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manages state synchronization with peers
#[derive(Debug, Clone)]
pub struct StateSyncManager {
    /// Current sync progress
    sync_progress: Option<SyncProgress>,
    
    /// Peers available for sync
    sync_peers: BTreeMap<ValidatorId, PeerSyncInfo>,
    
    /// Last sync timestamp
    last_sync: u64,
    
    /// Minimum sync interval (seconds)
    min_sync_interval: u64,
}

impl StateSyncManager {
    /// Create a new state sync manager
    pub fn new(min_sync_interval: u64) -> Self {
        Self {
            sync_progress: None,
            sync_peers: BTreeMap::new(),
            last_sync: 0,
            min_sync_interval,
        }
    }

    /// Start syncing from a peer
    pub fn start_sync(
        &mut self,
        peer: ValidatorId,
        start_block: BlockNumber,
        target_block: BlockNumber,
    ) -> ValidatorResult<()> {
        if self.is_syncing() {
            return Err(ValidatorError::NetworkError("Already syncing"));
        }

        let progress = SyncProgress::new(start_block, target_block);
        self.sync_progress = Some(progress);
        
        // Mark peer as sync source
        if let Some(peer_info) = self.sync_peers.get_mut(&peer) {
            peer_info.is_syncing = true;
        }

        Ok(())
    }

    /// Update sync progress
    pub fn update_progress(&mut self, current_block: BlockNumber, speed: u64) {
        if let Some(progress) = &mut self.sync_progress {
            progress.update(current_block, speed);
            
            if progress.is_complete() {
                self.complete_sync();
            }
        }
    }

    /// Complete sync
    fn complete_sync(&mut self) {
        if let Some(progress) = &mut self.sync_progress {
            progress.state = SyncState::Complete;
            self.last_sync = 0; // Should be actual timestamp
        }
        
        // Reset peer sync flags
        for peer_info in self.sync_peers.values_mut() {
            peer_info.is_syncing = false;
        }
    }

    /// Fail sync
    pub fn fail_sync(&mut self) {
        if let Some(progress) = &mut self.sync_progress {
            progress.state = SyncState::Failed;
        }
        
        // Reset peer sync flags
        for peer_info in self.sync_peers.values_mut() {
            peer_info.is_syncing = false;
        }
    }

    /// Check if currently syncing
    pub fn is_syncing(&self) -> bool {
        self.sync_progress
            .as_ref()
            .map(|p| p.state == SyncState::Syncing)
            .unwrap_or(false)
    }

    /// Get sync progress
    pub fn get_progress(&self) -> Option<&SyncProgress> {
        self.sync_progress.as_ref()
    }

    /// Add sync peer
    pub fn add_sync_peer(&mut self, peer: ValidatorId, latest_block: BlockNumber) {
        let info = PeerSyncInfo::new(latest_block);
        self.sync_peers.insert(peer, info);
    }

    /// Remove sync peer
    pub fn remove_sync_peer(&mut self, peer: &ValidatorId) {
        self.sync_peers.remove(peer);
    }

    /// Update peer's latest block
    pub fn update_peer_block(&mut self, peer: &ValidatorId, block: BlockNumber) {
        if let Some(info) = self.sync_peers.get_mut(peer) {
            info.latest_block = block;
        }
    }

    /// Get best sync peer (highest block number)
    pub fn best_sync_peer(&self) -> Option<ValidatorId> {
        self.sync_peers
            .iter()
            .filter(|(_, info)| !info.is_syncing)
            .max_by_key(|(_, info)| info.latest_block)
            .map(|(id, _)| id.clone())
    }

    /// Check if sync is needed
    pub fn needs_sync(&self, local_block: BlockNumber, current_time: u64) -> bool {
        // Check if enough time has passed
        if current_time < self.last_sync + self.min_sync_interval {
            return false;
        }

        // Check if any peer is significantly ahead
        self.sync_peers
            .values()
            .any(|info| info.latest_block > local_block + 10)
    }

    /// Get sync statistics
    pub fn get_stats(&self) -> SyncStats {
        let active_peers = self.sync_peers.len();
        let syncing_peers = self.sync_peers.values().filter(|p| p.is_syncing).count();
        
        let highest_peer_block = self
            .sync_peers
            .values()
            .map(|p| p.latest_block)
            .max()
            .unwrap_or(0);

        SyncStats {
            is_syncing: self.is_syncing(),
            sync_progress: self.sync_progress.as_ref().map(|p| p.percentage()),
            active_peers,
            syncing_peers,
            highest_peer_block,
        }
    }

    /// Clear all sync data
    pub fn clear(&mut self) {
        self.sync_progress = None;
        self.sync_peers.clear();
    }
}

impl Default for StateSyncManager {
    fn default() -> Self {
        Self::new(300) // 5 minutes between syncs
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PEER SYNC INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Sync information for a peer
#[derive(Debug, Clone)]
struct PeerSyncInfo {
    /// Latest block the peer has
    latest_block: BlockNumber,
    
    /// Whether we're currently syncing from this peer
    is_syncing: bool,
    
    /// Last time we synced from this peer
    last_sync_time: u64,
}

impl PeerSyncInfo {
    fn new(latest_block: BlockNumber) -> Self {
        Self {
            latest_block,
            is_syncing: false,
            last_sync_time: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYNC STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// State synchronization statistics
#[derive(Debug, Clone, Default)]
pub struct SyncStats {
    /// Whether currently syncing
    pub is_syncing: bool,
    
    /// Sync progress percentage (if syncing)
    pub sync_progress: Option<u8>,
    
    /// Number of peers available for sync
    pub active_peers: usize,
    
    /// Number of peers currently syncing from
    pub syncing_peers: usize,
    
    /// Highest block number among peers
    pub highest_peer_block: BlockNumber,
}

// ═══════════════════════════════════════════════════════════════════════════════
// STATE SNAPSHOT
// ═══════════════════════════════════════════════════════════════════════════════

/// Consensus state snapshot for syncing
#[derive(Debug, Clone, Encode, Decode)]
pub struct StateSnapshot {
    /// Block number of this snapshot
    pub block_number: BlockNumber,
    
    /// Block hash
    pub block_hash: Hash,
    
    /// Epoch number
    pub epoch: u32,
    
    /// Committee members (encoded)
    pub committee: Vec<u8>,
    
    /// Active validators (encoded)
    pub validators: Vec<u8>,
    
    /// Finalized blocks (encoded)
    pub finalized_blocks: Vec<u8>,
}

impl StateSnapshot {
    /// Create a new state snapshot
    pub fn new(block_number: BlockNumber, block_hash: Hash, epoch: u32) -> Self {
        Self {
            block_number,
            block_hash,
            epoch,
            committee: Vec::new(),
            validators: Vec::new(),
            finalized_blocks: Vec::new(),
        }
    }

    /// Check if snapshot is valid
    pub fn is_valid(&self) -> bool {
        self.block_number > 0 && self.epoch > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_validator(id: u8) -> ValidatorId {
        ValidatorId::from([id; 32])
    }

    #[test]
    fn test_sync_progress_creation() {
        let progress = SyncProgress::new(0, 100);
        assert_eq!(progress.start_block, 0);
        assert_eq!(progress.target_block, 100);
        assert_eq!(progress.state, SyncState::Syncing);
    }

    #[test]
    fn test_sync_progress_update() {
        let mut progress = SyncProgress::new(0, 100);
        
        progress.update(50, 10);
        assert_eq!(progress.current_block, 50);
        assert_eq!(progress.blocks_synced, 50);
        assert_eq!(progress.percentage(), 50);
        
        progress.update(100, 10);
        assert!(progress.is_complete());
        assert_eq!(progress.percentage(), 100);
    }

    #[test]
    fn test_estimated_time_remaining() {
        let mut progress = SyncProgress::new(0, 100);
        progress.update(50, 10); // 10 blocks/sec
        
        let remaining = progress.estimated_time_remaining();
        assert_eq!(remaining, Some(5)); // 50 blocks left / 10 blocks/sec = 5 sec
    }

    #[test]
    fn test_state_sync_manager_creation() {
        let manager = StateSyncManager::new(300);
        assert!(!manager.is_syncing());
    }

    #[test]
    fn test_start_sync() {
        let mut manager = StateSyncManager::new(300);
        let peer = create_test_validator(1);
        
        assert!(manager.start_sync(peer, 0, 100).is_ok());
        assert!(manager.is_syncing());
    }

    #[test]
    fn test_update_progress() {
        let mut manager = StateSyncManager::new(300);
        let peer = create_test_validator(1);
        
        manager.start_sync(peer, 0, 100).unwrap();
        manager.update_progress(50, 10);
        
        let progress = manager.get_progress().unwrap();
        assert_eq!(progress.current_block, 50);
        assert_eq!(progress.percentage(), 50);
    }

    #[test]
    fn test_complete_sync() {
        let mut manager = StateSyncManager::new(300);
        let peer = create_test_validator(1);
        
        manager.start_sync(peer, 0, 100).unwrap();
        manager.update_progress(100, 10);
        
        let progress = manager.get_progress().unwrap();
        assert_eq!(progress.state, SyncState::Complete);
        assert!(!manager.is_syncing());
    }

    #[test]
    fn test_add_sync_peer() {
        let mut manager = StateSyncManager::new(300);
        let peer = create_test_validator(1);
        
        manager.add_sync_peer(peer.clone(), 100);
        assert_eq!(manager.sync_peers.len(), 1);
    }

    #[test]
    fn test_best_sync_peer() {
        let mut manager = StateSyncManager::new(300);
        
        manager.add_sync_peer(create_test_validator(1), 100);
        manager.add_sync_peer(create_test_validator(2), 200);
        manager.add_sync_peer(create_test_validator(3), 150);
        
        let best = manager.best_sync_peer();
        assert_eq!(best, Some(create_test_validator(2))); // Highest block
    }

    #[test]
    fn test_needs_sync() {
        let mut manager = StateSyncManager::new(0); // No interval for testing
        
        manager.add_sync_peer(create_test_validator(1), 100);
        
        assert!(manager.needs_sync(50, 1000)); // Local at 50, peer at 100
        assert!(!manager.needs_sync(95, 1000)); // Local at 95, peer at 100 (< 10 diff)
    }

    #[test]
    fn test_sync_stats() {
        let mut manager = StateSyncManager::new(300);
        
        manager.add_sync_peer(create_test_validator(1), 100);
        manager.add_sync_peer(create_test_validator(2), 200);
        
        let stats = manager.get_stats();
        assert_eq!(stats.active_peers, 2);
        assert_eq!(stats.highest_peer_block, 200);
    }

    #[test]
    fn test_state_snapshot_creation() {
        let snapshot = StateSnapshot::new(100, Hash::default(), 5);
        assert_eq!(snapshot.block_number, 100);
        assert_eq!(snapshot.epoch, 5);
        assert!(snapshot.is_valid());
    }

    #[test]
    fn test_state_snapshot_validation() {
        let mut snapshot = StateSnapshot::new(0, Hash::default(), 0);
        assert!(!snapshot.is_valid());
        
        snapshot.block_number = 1;
        snapshot.epoch = 1;
        assert!(snapshot.is_valid());
    }

    #[test]
    fn test_fail_sync() {
        let mut manager = StateSyncManager::new(300);
        let peer = create_test_validator(1);
        
        manager.start_sync(peer, 0, 100).unwrap();
        manager.fail_sync();
        
        let progress = manager.get_progress().unwrap();
        assert_eq!(progress.state, SyncState::Failed);
    }
}