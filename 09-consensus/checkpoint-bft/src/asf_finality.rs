// ═══════════════════════════════════════════════════════════════════════════
// ASF FINALITY LEVELS - Ascending Scale of Finality Integration
// ═══════════════════════════════════════════════════════════════════════════
//
// Integrates checkpoint certificates with ËTRID's Ascending Scale of Finality.
// Provides graduated finality levels based on signature count and depth.
//
// Finality Levels:
// - None:         No certificate (0-14 signatures)
// - Weak:         15-17 signatures (71-81% quorum)
// - Moderate:     18-20 signatures (86-95% quorum)
// - Strong:       21 signatures (100% quorum)
// - Irreversible: 21 signatures + depth ≥3 checkpoints
//
// Security Properties:
// - Finality can only upgrade (never downgrade)
// - Strong/Irreversible blocks are locked (cannot change)
// - Prevents finality reversion attacks
//
// ═══════════════════════════════════════════════════════════════════════════

use codec::{Decode, Encode};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{CheckpointCertificate, TOTAL_VALIDATORS};

/// ASF Finality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, Serialize, Deserialize)]
pub enum AsfFinalityLevel {
    /// No finality (0-14 signatures)
    None = 0,

    /// Weak finality (15-17 signatures = 71-81%)
    Weak = 1,

    /// Moderate finality (18-20 signatures = 86-95%)
    Moderate = 2,

    /// Strong finality (21 signatures = 100%)
    Strong = 3,

    /// Irreversible finality (21 signatures + depth ≥3)
    Irreversible = 4,
}

impl AsfFinalityLevel {
    /// Get finality level from signature count
    pub fn from_signature_count(count: usize) -> Self {
        match count {
            0..=14 => AsfFinalityLevel::None,
            15..=17 => AsfFinalityLevel::Weak,
            18..=20 => AsfFinalityLevel::Moderate,
            21.. => AsfFinalityLevel::Strong,
        }
    }

    /// Check if level is locked (cannot be changed)
    pub fn is_locked(&self) -> bool {
        matches!(self, AsfFinalityLevel::Strong | AsfFinalityLevel::Irreversible)
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            AsfFinalityLevel::None => "No finality",
            AsfFinalityLevel::Weak => "Weak finality (15-17 signatures)",
            AsfFinalityLevel::Moderate => "Moderate finality (18-20 signatures)",
            AsfFinalityLevel::Strong => "Strong finality (21 signatures)",
            AsfFinalityLevel::Irreversible => "Irreversible finality (21 sigs + depth ≥3)",
        }
    }

    /// Get minimum signatures required for this level
    pub fn min_signatures(&self) -> usize {
        match self {
            AsfFinalityLevel::None => 0,
            AsfFinalityLevel::Weak => 15,
            AsfFinalityLevel::Moderate => 18,
            AsfFinalityLevel::Strong => 21,
            AsfFinalityLevel::Irreversible => 21,
        }
    }
}

impl std::fmt::Display for AsfFinalityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsfFinalityLevel::None => write!(f, "None"),
            AsfFinalityLevel::Weak => write!(f, "Weak"),
            AsfFinalityLevel::Moderate => write!(f, "Moderate"),
            AsfFinalityLevel::Strong => write!(f, "Strong"),
            AsfFinalityLevel::Irreversible => write!(f, "Irreversible"),
        }
    }
}

/// Finality information for a block
#[derive(Debug, Clone, Encode, Decode)]
pub struct FinalityInfo {
    pub block_hash: [u8; 32],
    pub finality_level: AsfFinalityLevel,
    pub signature_count: u32,
    pub finalized_at: u64,
    /// Once Strong/Irreversible, cannot be changed
    pub locked: bool,
}

impl FinalityInfo {
    fn new(block_hash: [u8; 32], signature_count: u32) -> Self {
        let finality_level = AsfFinalityLevel::from_signature_count(signature_count as usize);
        let locked = finality_level.is_locked();

        Self {
            block_hash,
            finality_level,
            signature_count,
            finalized_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|e| {
                    log::error!("System time before UNIX epoch in finality info: {:?}", e);
                    std::time::Duration::from_secs(0)
                })
                .as_millis() as u64,
            locked,
        }
    }
}

/// Finality tracker for ASF
pub struct FinalityTracker {
    /// Finalized blocks: block_number -> FinalityInfo
    finalized_blocks: Arc<RwLock<HashMap<u32, FinalityInfo>>>,

    /// Checkpoint depth tracking (for Irreversible calculation)
    /// block_number -> checkpoint_depth
    checkpoint_depths: Arc<RwLock<HashMap<u32, u32>>>,
}

impl FinalityTracker {
    /// Create new finality tracker
    pub fn new() -> Self {
        Self {
            finalized_blocks: Arc::new(RwLock::new(HashMap::new())),
            checkpoint_depths: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Finalize block with initial level
    pub fn finalize_block(
        &self,
        block_number: u32,
        block_hash: [u8; 32],
        signature_count: u32,
    ) -> Result<AsfFinalityLevel, String> {
        let mut finalized = self.finalized_blocks.write();

        // Check if already finalized
        if let Some(existing) = finalized.get(&block_number) {
            if existing.block_hash != block_hash {
                return Err(format!(
                    "Cannot finalize block #{}: already finalized with different hash \
                     (existing: {:?}, new: {:?})",
                    block_number, existing.block_hash, block_hash
                ));
            }

            // Same block - check if we can upgrade
            if existing.locked {
                // Already locked at Strong/Irreversible - cannot change
                return Ok(existing.finality_level);
            }

            // Can upgrade if more signatures
            if signature_count > existing.signature_count {
                drop(finalized);
                return self.upgrade_finality(block_number, signature_count);
            }

            return Ok(existing.finality_level);
        }

        // New finalization
        let info = FinalityInfo::new(block_hash, signature_count);
        let level = info.finality_level;

        tracing::info!(
            "🔒 Finalized block #{} = {:?} with {} finality ({} signatures)",
            block_number,
            block_hash,
            level,
            signature_count
        );

        finalized.insert(block_number, info);

        Ok(level)
    }

    /// Upgrade finality level (Weak → Moderate → Strong → Irreversible)
    pub fn upgrade_finality(
        &self,
        block_number: u32,
        new_signature_count: u32,
    ) -> Result<AsfFinalityLevel, String> {
        let mut finalized = self.finalized_blocks.write();

        let info = finalized
            .get_mut(&block_number)
            .ok_or_else(|| format!("Block #{} not finalized", block_number))?;

        // Check if locked
        if info.locked {
            return Err(format!(
                "Cannot upgrade block #{}: locked at {} finality",
                block_number, info.finality_level
            ));
        }

        // Check if upgrade is valid (more signatures)
        if new_signature_count <= info.signature_count {
            return Ok(info.finality_level);
        }

        let old_level = info.finality_level;
        let new_level = AsfFinalityLevel::from_signature_count(new_signature_count as usize);

        // Ensure we only upgrade (never downgrade)
        if new_level < old_level {
            return Err(format!(
                "Cannot downgrade finality from {} to {}",
                old_level, new_level
            ));
        }

        // Update
        info.signature_count = new_signature_count;
        info.finality_level = new_level;
        info.locked = new_level.is_locked();

        tracing::info!(
            "⬆️ Upgraded block #{} finality: {} → {} ({} signatures)",
            block_number,
            old_level,
            new_level,
            new_signature_count
        );

        Ok(new_level)
    }

    /// Check if block can be upgraded to Irreversible
    pub fn check_irreversible_upgrade(&self, block_number: u32) -> Result<bool, String> {
        let finalized = self.finalized_blocks.read();
        let info = finalized
            .get(&block_number)
            .ok_or_else(|| format!("Block #{} not finalized", block_number))?;

        // Must have 21 signatures (Strong)
        if info.signature_count < TOTAL_VALIDATORS as u32 {
            return Ok(false);
        }

        // Already Irreversible
        if info.finality_level == AsfFinalityLevel::Irreversible {
            return Ok(true);
        }

        drop(finalized);

        // Check checkpoint depth
        let depth = self.get_checkpoint_depth(block_number);
        if depth >= 3 {
            // Eligible for Irreversible upgrade
            self.upgrade_to_irreversible(block_number)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Upgrade block to Irreversible
    fn upgrade_to_irreversible(&self, block_number: u32) -> Result<(), String> {
        let mut finalized = self.finalized_blocks.write();

        let info = finalized
            .get_mut(&block_number)
            .ok_or_else(|| format!("Block #{} not finalized", block_number))?;

        if info.finality_level == AsfFinalityLevel::Irreversible {
            return Ok(()); // Already Irreversible
        }

        if info.signature_count < TOTAL_VALIDATORS as u32 {
            return Err(format!(
                "Cannot upgrade to Irreversible: only {} signatures",
                info.signature_count
            ));
        }

        let old_level = info.finality_level;
        info.finality_level = AsfFinalityLevel::Irreversible;
        info.locked = true;

        tracing::info!(
            "⬆️🔒 Upgraded block #{} to IRREVERSIBLE finality (was {})",
            block_number,
            old_level
        );

        Ok(())
    }

    /// Update checkpoint depth for block
    pub fn update_checkpoint_depth(&self, block_number: u32, depth: u32) {
        self.checkpoint_depths.write().insert(block_number, depth);

        // Check if can upgrade to Irreversible
        if depth >= 3 {
            self.check_irreversible_upgrade(block_number).ok();
        }
    }

    /// Get checkpoint depth for block
    pub fn get_checkpoint_depth(&self, block_number: u32) -> u32 {
        self.checkpoint_depths
            .read()
            .get(&block_number)
            .cloned()
            .unwrap_or(0)
    }

    /// Check if block finalized with at least given level
    pub fn has_finality_level(
        &self,
        block_number: u32,
        required_level: AsfFinalityLevel,
    ) -> bool {
        self.finalized_blocks
            .read()
            .get(&block_number)
            .map(|info| info.finality_level >= required_level)
            .unwrap_or(false)
    }

    /// Get finality level for block
    pub fn get_finality_level(&self, block_number: u32) -> Option<AsfFinalityLevel> {
        self.finalized_blocks
            .read()
            .get(&block_number)
            .map(|info| info.finality_level)
    }

    /// Get finality info for block
    pub fn get_finality_info(&self, block_number: u32) -> Option<FinalityInfo> {
        self.finalized_blocks
            .read()
            .get(&block_number)
            .cloned()
    }

    /// Check if block is locked (Strong or Irreversible)
    pub fn is_locked(&self, block_number: u32) -> bool {
        self.finalized_blocks
            .read()
            .get(&block_number)
            .map(|info| info.locked)
            .unwrap_or(false)
    }

    /// Get all finalized blocks with at least given level
    pub fn get_blocks_with_finality(&self, min_level: AsfFinalityLevel) -> Vec<u32> {
        self.finalized_blocks
            .read()
            .iter()
            .filter(|(_, info)| info.finality_level >= min_level)
            .map(|(num, _)| *num)
            .collect()
    }

    /// Cleanup old finalized blocks
    pub fn cleanup_old_finalized(&self, current_block: u32, keep_blocks: u32) {
        let cutoff = current_block.saturating_sub(keep_blocks);

        self.finalized_blocks
            .write()
            .retain(|&block_number, _| block_number >= cutoff);

        self.checkpoint_depths
            .write()
            .retain(|&block_number, _| block_number >= cutoff);

        tracing::debug!("🧹 Cleaned finality data older than block #{}", cutoff);
    }

    /// Generate finality report
    pub fn generate_report(&self) -> FinalityReport {
        let finalized = self.finalized_blocks.read();

        let mut blocks_by_level: HashMap<AsfFinalityLevel, Vec<u32>> = HashMap::new();

        for (block_number, info) in finalized.iter() {
            blocks_by_level
                .entry(info.finality_level)
                .or_insert_with(Vec::new)
                .push(*block_number);
        }

        // Get latest blocks for each level
        let latest_irreversible = blocks_by_level
            .get(&AsfFinalityLevel::Irreversible)
            .and_then(|blocks| blocks.iter().max().cloned());

        let latest_strong = blocks_by_level
            .get(&AsfFinalityLevel::Strong)
            .and_then(|blocks| blocks.iter().max().cloned());

        FinalityReport {
            total_finalized: finalized.len(),
            blocks_by_level,
            latest_irreversible,
            latest_strong,
        }
    }
}

impl Default for FinalityTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Finality report
#[derive(Debug, Clone)]
pub struct FinalityReport {
    pub total_finalized: usize,
    pub blocks_by_level: HashMap<AsfFinalityLevel, Vec<u32>>,
    pub latest_irreversible: Option<u32>,
    pub latest_strong: Option<u32>,
}

/// Extension trait for CheckpointCertificate
pub trait CertificateAsfExt {
    /// Calculate ASF finality level
    fn finality_level(&self) -> AsfFinalityLevel;

    /// Check if irreversible (21/21 + depth ≥3)
    fn is_irreversible(&self, checkpoint_depth: u32) -> bool;

    /// Get signature percentage
    fn signature_percentage(&self) -> f64;
}

impl CertificateAsfExt for CheckpointCertificate {
    fn finality_level(&self) -> AsfFinalityLevel {
        AsfFinalityLevel::from_signature_count(self.signatures.len())
    }

    fn is_irreversible(&self, checkpoint_depth: u32) -> bool {
        self.signatures.len() == TOTAL_VALIDATORS && checkpoint_depth >= 3
    }

    fn signature_percentage(&self) -> f64 {
        (self.signatures.len() as f64 / TOTAL_VALIDATORS as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finality_level_from_signatures() {
        assert_eq!(AsfFinalityLevel::from_signature_count(0), AsfFinalityLevel::None);
        assert_eq!(AsfFinalityLevel::from_signature_count(14), AsfFinalityLevel::None);
        assert_eq!(AsfFinalityLevel::from_signature_count(15), AsfFinalityLevel::Weak);
        assert_eq!(AsfFinalityLevel::from_signature_count(17), AsfFinalityLevel::Weak);
        assert_eq!(AsfFinalityLevel::from_signature_count(18), AsfFinalityLevel::Moderate);
        assert_eq!(AsfFinalityLevel::from_signature_count(20), AsfFinalityLevel::Moderate);
        assert_eq!(AsfFinalityLevel::from_signature_count(21), AsfFinalityLevel::Strong);
    }

    #[test]
    fn test_finalize_block() {
        let tracker = FinalityTracker::new();

        let block_hash = [1u8; 32];

        // Finalize with 15 signatures (Weak)
        let level = tracker.finalize_block(16, block_hash, 15).unwrap();
        assert_eq!(level, AsfFinalityLevel::Weak);

        // Upgrade to 18 signatures (Moderate)
        let level = tracker.upgrade_finality(16, 18).unwrap();
        assert_eq!(level, AsfFinalityLevel::Moderate);

        // Upgrade to 21 signatures (Strong)
        let level = tracker.upgrade_finality(16, 21).unwrap();
        assert_eq!(level, AsfFinalityLevel::Strong);

        // Should be locked now
        assert!(tracker.is_locked(16));
    }

    #[test]
    fn test_cannot_downgrade() {
        let tracker = FinalityTracker::new();

        let block_hash = [1u8; 32];

        // Finalize with 21 signatures (Strong)
        tracker.finalize_block(16, block_hash, 21).unwrap();

        // Attempt to downgrade should fail
        let result = tracker.upgrade_finality(16, 15);
        assert!(result.is_err());
    }

    #[test]
    fn test_irreversible_upgrade() {
        let tracker = FinalityTracker::new();

        let block_hash = [1u8; 32];

        // Finalize with 21 signatures (Strong)
        tracker.finalize_block(16, block_hash, 21).unwrap();

        // Set depth < 3 - should not upgrade
        tracker.update_checkpoint_depth(16, 2);
        assert_eq!(
            tracker.get_finality_level(16).unwrap(),
            AsfFinalityLevel::Strong
        );

        // Set depth ≥ 3 - should upgrade to Irreversible
        tracker.update_checkpoint_depth(16, 3);
        assert_eq!(
            tracker.get_finality_level(16).unwrap(),
            AsfFinalityLevel::Irreversible
        );
    }

    #[test]
    fn test_locked_cannot_change() {
        let tracker = FinalityTracker::new();

        let block_hash = [1u8; 32];

        // Finalize with 21 signatures (Strong - locked)
        tracker.finalize_block(16, block_hash, 21).unwrap();

        // Attempt to change should fail (locked)
        let result = tracker.upgrade_finality(16, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_dual_finalization_prevention() {
        let tracker = FinalityTracker::new();

        let block_hash_1 = [1u8; 32];
        let block_hash_2 = [2u8; 32];

        // Finalize block 16 with hash 1
        tracker.finalize_block(16, block_hash_1, 15).unwrap();

        // Attempt to finalize block 16 with different hash - should fail
        let result = tracker.finalize_block(16, block_hash_2, 15);
        assert!(result.is_err());
    }

    #[test]
    fn test_finality_report() {
        let tracker = FinalityTracker::new();

        // Finalize multiple blocks at different levels
        tracker.finalize_block(16, [1u8; 32], 15).unwrap(); // Weak
        tracker.finalize_block(32, [2u8; 32], 18).unwrap(); // Moderate
        tracker.finalize_block(48, [3u8; 32], 21).unwrap(); // Strong

        // Upgrade block 48 to Irreversible
        tracker.update_checkpoint_depth(48, 3);

        let report = tracker.generate_report();

        assert_eq!(report.total_finalized, 3);
        assert_eq!(report.latest_irreversible, Some(48));
        assert_eq!(report.latest_strong, Some(48));
    }

    #[test]
    fn test_has_finality_level() {
        let tracker = FinalityTracker::new();

        tracker.finalize_block(16, [1u8; 32], 18).unwrap(); // Moderate

        // Check various levels
        assert!(tracker.has_finality_level(16, AsfFinalityLevel::None));
        assert!(tracker.has_finality_level(16, AsfFinalityLevel::Weak));
        assert!(tracker.has_finality_level(16, AsfFinalityLevel::Moderate));
        assert!(!tracker.has_finality_level(16, AsfFinalityLevel::Strong));
        assert!(!tracker.has_finality_level(16, AsfFinalityLevel::Irreversible));
    }
}
