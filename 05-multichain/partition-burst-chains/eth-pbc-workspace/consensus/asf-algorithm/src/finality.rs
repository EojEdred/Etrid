//! # Finality Level Calculation
//!
//! This module implements the Ascending Scale of Finality as specified
//! in the Ëtrid Ivory Papers. It tracks finality progression and
//! determines when blocks reach different levels of irreversibility.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::{BlockNumber, FinalityLevel, Hash};

// ═══════════════════════════════════════════════════════════════════════════════
// FINALITY TRACKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Tracks finality progression for blocks
#[derive(Debug, Clone)]
pub struct FinalityTracker {
    /// Map of block hash to finality info
    blocks: BTreeMap<Hash, BlockFinalityInfo>,
    
    /// Highest finalized block number
    highest_finalized: BlockNumber,
}

impl FinalityTracker {
    /// Create a new finality tracker
    pub fn new() -> Self {
        Self {
            blocks: BTreeMap::new(),
            highest_finalized: 0,
        }
    }

    /// Track a new block
    pub fn track_block(&mut self, block_hash: Hash, block_number: BlockNumber) {
        let info = BlockFinalityInfo::new(block_hash, block_number);
        self.blocks.insert(block_hash, info);
    }

    /// Update certificate count for a block
    pub fn update_certificates(&mut self, block_hash: &Hash, count: u32) {
        if let Some(info) = self.blocks.get_mut(block_hash) {
            info.certificate_count = count;
            info.finality_level = FinalityLevel::from(count);
            
            // Update highest finalized if this block is finalized
            if info.finality_level.is_finalized() && info.block_number > self.highest_finalized {
                self.highest_finalized = info.block_number;
            }
        }
    }

    /// Get finality info for a block
    pub fn get_info(&self, block_hash: &Hash) -> Option<&BlockFinalityInfo> {
        self.blocks.get(block_hash)
    }

    /// Get finality level for a block
    pub fn get_level(&self, block_hash: &Hash) -> FinalityLevel {
        self.blocks
            .get(block_hash)
            .map(|info| info.finality_level)
            .unwrap_or(FinalityLevel::None)
    }

    /// Check if a block is finalized
    pub fn is_finalized(&self, block_hash: &Hash) -> bool {
        self.get_level(block_hash).is_finalized()
    }

    /// Get highest finalized block number
    pub fn highest_finalized(&self) -> BlockNumber {
        self.highest_finalized
    }

    /// Prune old blocks (keep only recent ones)
    pub fn prune(&mut self, keep_blocks: usize) {
        if self.blocks.len() <= keep_blocks {
            return;
        }

        // Keep the most recent blocks
        let to_remove = self.blocks.len() - keep_blocks;
        let keys_to_remove: Vec<Hash> = self
            .blocks
            .iter()
            .take(to_remove)
            .map(|(k, _)| *k)
            .collect();

        for key in keys_to_remove {
            self.blocks.remove(&key);
        }
    }

    /// Get all blocks at a specific finality level
    pub fn blocks_at_level(&self, level: FinalityLevel) -> Vec<Hash> {
        self.blocks
            .iter()
            .filter(|(_, info)| info.finality_level == level)
            .map(|(hash, _)| *hash)
            .collect()
    }

    /// Get statistics
    pub fn stats(&self) -> FinalityStats {
        let mut stats = FinalityStats::default();
        
        for info in self.blocks.values() {
            match info.finality_level {
                FinalityLevel::None => stats.none += 1,
                FinalityLevel::Weak => stats.weak += 1,
                FinalityLevel::Moderate => stats.moderate += 1,
                FinalityLevel::Strong => stats.strong += 1,
                FinalityLevel::Irreversible => stats.irreversible += 1,
            }
        }
        
        stats.total = self.blocks.len();
        stats.highest_finalized = self.highest_finalized;
        
        stats
    }
}

impl Default for FinalityTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK FINALITY INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Information about a block's finality status
#[derive(Debug, Clone)]
pub struct BlockFinalityInfo {
    /// Block hash
    pub block_hash: Hash,
    
    /// Block number
    pub block_number: BlockNumber,
    
    /// Number of validity certificates
    pub certificate_count: u32,
    
    /// Current finality level
    pub finality_level: FinalityLevel,
    
    /// Timestamp when first tracked
    pub first_seen: u64,
    
    /// Timestamp when finalized (if finalized)
    pub finalized_at: Option<u64>,
}

impl BlockFinalityInfo {
    /// Create new finality info for a block
    pub fn new(block_hash: Hash, block_number: BlockNumber) -> Self {
        Self {
            block_hash,
            block_number,
            certificate_count: 0,
            finality_level: FinalityLevel::None,
            first_seen: 0, // Should be set to actual timestamp
            finalized_at: None,
        }
    }

    /// Update finality level based on certificate count
    pub fn update(&mut self, certificate_count: u32, timestamp: u64) {
        self.certificate_count = certificate_count;
        self.finality_level = FinalityLevel::from(certificate_count);
        
        if self.finality_level.is_finalized() && self.finalized_at.is_none() {
            self.finalized_at = Some(timestamp);
        }
    }

    /// Time to finalization (if finalized)
    pub fn time_to_finalization(&self) -> Option<u64> {
        self.finalized_at.map(|fin| fin.saturating_sub(self.first_seen))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FINALITY STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Statistics about finality levels across all tracked blocks
#[derive(Debug, Clone, Default)]
pub struct FinalityStats {
    /// Total blocks tracked
    pub total: usize,
    
    /// Blocks with no finality
    pub none: usize,
    
    /// Blocks with weak finality
    pub weak: usize,
    
    /// Blocks with moderate finality
    pub moderate: usize,
    
    /// Blocks with strong finality
    pub strong: usize,
    
    /// Blocks with irreversible finality
    pub irreversible: usize,
    
    /// Highest finalized block number
    pub highest_finalized: BlockNumber,
}

impl FinalityStats {
    /// Calculate percentage at each level
    pub fn percentages(&self) -> FinalityPercentages {
        let total = self.total as f64;
        
        FinalityPercentages {
            none: (self.none as f64 / total) * 100.0,
            weak: (self.weak as f64 / total) * 100.0,
            moderate: (self.moderate as f64 / total) * 100.0,
            strong: (self.strong as f64 / total) * 100.0,
            irreversible: (self.irreversible as f64 / total) * 100.0,
        }
    }

    /// Get finalized percentage (all levels except None)
    pub fn finalized_percentage(&self) -> f64 {
        let finalized = self.weak + self.moderate + self.strong + self.irreversible;
        (finalized as f64 / self.total as f64) * 100.0
    }
}

/// Percentages at each finality level
#[derive(Debug, Clone)]
pub struct FinalityPercentages {
    pub none: f64,
    pub weak: f64,
    pub moderate: f64,
    pub strong: f64,
    pub irreversible: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// FINALITY PROGRESSION ANALYZER
// ═══════════════════════════════════════════════════════════════════════════════

/// Analyzes how quickly blocks progress through finality levels
#[derive(Debug, Clone)]
pub struct FinalityProgressionAnalyzer {
    /// Historical progression times
    progression_times: Vec<ProgressionRecord>,
}

impl FinalityProgressionAnalyzer {
    /// Create a new analyzer
    pub fn new() -> Self {
        Self {
            progression_times: Vec::new(),
        }
    }

    /// Record a finality progression
    pub fn record_progression(
        &mut self,
        from_level: FinalityLevel,
        to_level: FinalityLevel,
        time_ms: u64,
    ) {
        self.progression_times.push(ProgressionRecord {
            from_level,
            to_level,
            time_ms,
        });

        // Keep only last 1000 records
        if self.progression_times.len() > 1000 {
            self.progression_times.remove(0);
        }
    }

    /// Get average time to reach a finality level
    pub fn average_time_to_level(&self, level: FinalityLevel) -> Option<u64> {
        let times: Vec<u64> = self
            .progression_times
            .iter()
            .filter(|r| r.to_level == level)
            .map(|r| r.time_ms)
            .collect();

        if times.is_empty() {
            return None;
        }

        let sum: u64 = times.iter().sum();
        Some(sum / times.len() as u64)
    }

    /// Get median time to reach a finality level
    pub fn median_time_to_level(&self, level: FinalityLevel) -> Option<u64> {
        let mut times: Vec<u64> = self
            .progression_times
            .iter()
            .filter(|r| r.to_level == level)
            .map(|r| r.time_ms)
            .collect();

        if times.is_empty() {
            return None;
        }

        times.sort_unstable();
        Some(times[times.len() / 2])
    }
}

impl Default for FinalityProgressionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Record of finality progression
#[derive(Debug, Clone)]
struct ProgressionRecord {
    from_level: FinalityLevel,
    to_level: FinalityLevel,
    time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finality_tracker_creation() {
        let tracker = FinalityTracker::new();
        assert_eq!(tracker.highest_finalized(), 0);
    }

    #[test]
    fn test_track_block() {
        let mut tracker = FinalityTracker::new();
        let block_hash = Hash::default();
        
        tracker.track_block(block_hash, 1);
        assert!(tracker.get_info(&block_hash).is_some());
        assert_eq!(tracker.get_level(&block_hash), FinalityLevel::None);
    }

    #[test]
    fn test_update_certificates() {
        let mut tracker = FinalityTracker::new();
        let block_hash = Hash::default();
        
        tracker.track_block(block_hash, 1);
        tracker.update_certificates(&block_hash, 15);
        
        assert_eq!(tracker.get_level(&block_hash), FinalityLevel::Weak);
        assert!(tracker.is_finalized(&block_hash));
        assert_eq!(tracker.highest_finalized(), 1);
    }

    #[test]
    fn test_finality_levels() {
        let mut tracker = FinalityTracker::new();
        let block_hash = Hash::default();
        
        tracker.track_block(block_hash, 1);
        
        // Progress through finality levels
        tracker.update_certificates(&block_hash, 0);
        assert_eq!(tracker.get_level(&block_hash), FinalityLevel::None);
        
        tracker.update_certificates(&block_hash, 15);
        assert_eq!(tracker.get_level(&block_hash), FinalityLevel::Weak);
        
        tracker.update_certificates(&block_hash, 35);
        assert_eq!(tracker.get_level(&block_hash), FinalityLevel::Moderate);
        
        tracker.update_certificates(&block_hash, 75);
        assert_eq!(tracker.get_level(&block_hash), FinalityLevel::Strong);
        
        tracker.update_certificates(&block_hash, 150);
        assert_eq!(tracker.get_level(&block_hash), FinalityLevel::Irreversible);
    }

    #[test]
    fn test_finality_stats() {
        let mut tracker = FinalityTracker::new();
        
        // Add blocks at different finality levels
        for i in 0..5 {
            let mut hash_bytes = [0u8; 32];
            hash_bytes[0] = i;
            let hash = Hash::from(hash_bytes);
            
            tracker.track_block(hash, i as u64);
            tracker.update_certificates(&hash, (i as u32) * 30); // 0, 30, 60, 90, 120
        }
        
        let stats = tracker.stats();
        assert_eq!(stats.total, 5);
        assert_eq!(stats.none, 1); // 0 certificates
        assert_eq!(stats.moderate, 2); // 30 and 60 certificates
        assert_eq!(stats.strong, 1); // 90 certificates
        assert_eq!(stats.irreversible, 1); // 120 certificates
    }

    #[test]
    fn test_blocks_at_level() {
        let mut tracker = FinalityTracker::new();
        
        for i in 0..3 {
            let mut hash_bytes = [0u8; 32];
            hash_bytes[0] = i;
            let hash = Hash::from(hash_bytes);
            
            tracker.track_block(hash, i as u64);
            tracker.update_certificates(&hash, 15); // All at Weak level
        }
        
        let weak_blocks = tracker.blocks_at_level(FinalityLevel::Weak);
        assert_eq!(weak_blocks.len(), 3);
    }

    #[test]
    fn test_progression_analyzer() {
        let mut analyzer = FinalityProgressionAnalyzer::new();
        
        analyzer.record_progression(FinalityLevel::None, FinalityLevel::Weak, 1000);
        analyzer.record_progression(FinalityLevel::None, FinalityLevel::Weak, 1500);
        analyzer.record_progression(FinalityLevel::None, FinalityLevel::Weak, 2000);
        
        let avg = analyzer.average_time_to_level(FinalityLevel::Weak);
        assert_eq!(avg, Some(1500)); // (1000 + 1500 + 2000) / 3
    }

    #[test]
    fn test_block_finality_info() {
        let mut info = BlockFinalityInfo::new(Hash::default(), 1);
        assert_eq!(info.finality_level, FinalityLevel::None);
        
        info.update(20, 5000);
        assert_eq!(info.finality_level, FinalityLevel::Moderate);
        assert_eq!(info.finalized_at, Some(5000));
    }
}