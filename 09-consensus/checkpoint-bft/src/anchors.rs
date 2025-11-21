// ═══════════════════════════════════════════════════════════════════════════
// LONG-RANGE ATTACK PROTECTION
// ═══════════════════════════════════════════════════════════════════════════
//
// Prevents attackers with old validator keys from rewriting blockchain history.
//
// Attack Scenario:
// 1. Attacker compromises validator keys from 6 months ago
// 2. Attacker creates alternative chain starting from old epoch
// 3. Attacker's chain has valid signatures from real validators (at that time)
// 4. New nodes cannot distinguish which chain is "real"
//
// Defense:
// 1. Genesis checkpoint (hard-coded in binary)
// 2. Social consensus checkpoints (updated monthly via governance)
// 3. Expired authority set tracking
// 4. Minimum valid authority_set_id enforcement
//
// ═══════════════════════════════════════════════════════════════════════════

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Checkpoint anchor for long-range attack protection
#[derive(Clone, Debug, Encode, Decode, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckpointAnchor {
    /// Block number of checkpoint
    pub block_number: u32,

    /// Block hash of checkpoint
    pub block_hash: [u8; 32],

    /// Authority set ID at this checkpoint
    pub authority_set_id: u64,

    /// Human-readable description (for governance/auditing)
    pub description: String,

    /// Timestamp when anchor was added (milliseconds since epoch)
    pub timestamp_ms: u64,
}

impl CheckpointAnchor {
    /// Create new checkpoint anchor
    pub fn new(
        block_number: u32,
        block_hash: [u8; 32],
        authority_set_id: u64,
        description: String,
    ) -> Self {
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                log::error!("System time before UNIX epoch in social checkpoint: {:?}", e);
                std::time::Duration::from_secs(0)
            })
            .as_millis() as u64;

        Self {
            block_number,
            block_hash,
            authority_set_id,
            description,
            timestamp_ms,
        }
    }

    /// Create genesis anchor (hard-coded)
    pub fn genesis() -> Self {
        Self {
            block_number: 0,
            block_hash: [0u8; 32], // Will be replaced with actual genesis hash
            authority_set_id: 0,
            description: "Genesis block - hard-coded anchor".to_string(),
            timestamp_ms: 0,
        }
    }
}

/// Long-range attack protection system
#[derive(Clone, Debug)]
pub struct LongRangeProtection {
    /// Genesis checkpoint (hard-coded in binary)
    genesis: CheckpointAnchor,

    /// Social consensus checkpoints (updated monthly via governance)
    /// These are "socially agreed upon" checkpoints that represent
    /// community consensus on the canonical chain.
    social_checkpoints: Vec<CheckpointAnchor>,

    /// Minimum valid authority_set_id (prevents old epoch replay)
    /// Any signatures with authority_set_id < this value are rejected
    min_authority_set_id: u64,

    /// Expired authority set IDs (blacklist)
    /// Authority sets that have been rotated out and should not be trusted
    expired_authority_sets: HashSet<u64>,
}

impl LongRangeProtection {
    /// Create new long-range protection with genesis anchor
    pub fn new(genesis: CheckpointAnchor) -> Self {
        Self {
            genesis,
            social_checkpoints: Vec::new(),
            min_authority_set_id: 0,
            expired_authority_sets: HashSet::new(),
        }
    }

    /// Add social consensus checkpoint (via governance)
    ///
    /// Security: This should only be called via on-chain governance
    /// to prevent unauthorized checkpoint injection
    pub fn add_social_checkpoint(&mut self, anchor: CheckpointAnchor) -> Result<(), String> {
        // Verify anchor is newer than existing checkpoints
        if let Some(last) = self.social_checkpoints.last() {
            if anchor.block_number <= last.block_number {
                return Err(format!(
                    "New checkpoint #{} must be newer than last checkpoint #{}",
                    anchor.block_number, last.block_number
                ));
            }

            if anchor.authority_set_id < last.authority_set_id {
                return Err(format!(
                    "New checkpoint authority_set_id {} cannot be older than {}",
                    anchor.authority_set_id, last.authority_set_id
                ));
            }
        }

        tracing::info!(
            "📌 Adding social consensus checkpoint: #{} ({}) - authority_set_id={}",
            anchor.block_number,
            anchor.description,
            anchor.authority_set_id
        );

        self.social_checkpoints.push(anchor);

        // Limit to last 100 checkpoints (prevent unbounded growth)
        const MAX_SOCIAL_CHECKPOINTS: usize = 100;
        if self.social_checkpoints.len() > MAX_SOCIAL_CHECKPOINTS {
            let excess = self.social_checkpoints.len() - MAX_SOCIAL_CHECKPOINTS;
            self.social_checkpoints.drain(0..excess);
        }

        Ok(())
    }

    /// Get the most recent social consensus checkpoint
    pub fn get_latest_social_checkpoint(&self) -> Option<&CheckpointAnchor> {
        self.social_checkpoints.last()
    }

    /// Get all social consensus checkpoints
    pub fn get_all_social_checkpoints(&self) -> &[CheckpointAnchor] {
        &self.social_checkpoints
    }

    /// Verify chain passes through all checkpoint anchors
    ///
    /// This ensures an alternative chain cannot fork before any social checkpoint
    pub fn verify_chain_history(
        &self,
        chain_blocks: &[(u32, [u8; 32])], // (block_number, block_hash) pairs
    ) -> Result<(), String> {
        // Check genesis
        if let Some((num, hash)) = chain_blocks.first() {
            if *num == self.genesis.block_number && *hash != self.genesis.block_hash {
                return Err(format!(
                    "Genesis mismatch: expected {:?}, got {:?}",
                    self.genesis.block_hash, hash
                ));
            }
        }

        // Check all social checkpoints
        for anchor in &self.social_checkpoints {
            if let Some((_, hash)) = chain_blocks
                .iter()
                .find(|(num, _)| *num == anchor.block_number)
            {
                if *hash != anchor.block_hash {
                    return Err(format!(
                        "Checkpoint #{} mismatch: expected {:?}, got {:?}",
                        anchor.block_number, anchor.block_hash, hash
                    ));
                }
            } else {
                // Chain doesn't include this checkpoint
                tracing::warn!(
                    "⚠️ Chain missing checkpoint #{} ({})",
                    anchor.block_number,
                    anchor.description
                );
            }
        }

        Ok(())
    }

    /// Mark authority set as expired
    ///
    /// Expired authority sets are blacklisted and their signatures rejected
    pub fn expire_authority_set(&mut self, authority_set_id: u64) {
        if self.expired_authority_sets.insert(authority_set_id) {
            tracing::info!(
                "🔒 Authority set {} marked as expired",
                authority_set_id
            );

            // Update minimum valid authority set ID
            if authority_set_id >= self.min_authority_set_id {
                self.min_authority_set_id = authority_set_id + 1;
            }
        }
    }

    /// Check if authority set is expired
    pub fn is_authority_set_expired(&self, authority_set_id: u64) -> bool {
        authority_set_id < self.min_authority_set_id
            || self.expired_authority_sets.contains(&authority_set_id)
    }

    /// Get minimum valid authority_set_id
    pub fn get_min_authority_set_id(&self) -> u64 {
        self.min_authority_set_id
    }

    /// Set minimum valid authority_set_id
    ///
    /// Security: Only call this during authority set rotation
    pub fn set_min_authority_set_id(&mut self, min_id: u64) {
        if min_id > self.min_authority_set_id {
            tracing::info!(
                "🔄 Minimum authority_set_id updated: {} -> {}",
                self.min_authority_set_id,
                min_id
            );
            self.min_authority_set_id = min_id;
        }
    }

    /// Create checkpoint anchor at current block for future protection
    ///
    /// This allows the current state to become a social checkpoint later
    pub fn create_anchor_at_block(
        block_number: u32,
        block_hash: [u8; 32],
        authority_set_id: u64,
    ) -> CheckpointAnchor {
        CheckpointAnchor::new(
            block_number,
            block_hash,
            authority_set_id,
            format!("Social consensus checkpoint at block #{}", block_number),
        )
    }

    /// Cleanup expired authority sets older than cutoff
    ///
    /// Prevents unbounded growth of expired set tracking
    pub fn cleanup_expired_sets(&mut self, cutoff_authority_set_id: u64) {
        let before_count = self.expired_authority_sets.len();

        self.expired_authority_sets
            .retain(|&set_id| set_id >= cutoff_authority_set_id);

        let removed = before_count - self.expired_authority_sets.len();
        if removed > 0 {
            tracing::debug!(
                "Cleaned up {} expired authority sets older than {}",
                removed,
                cutoff_authority_set_id
            );
        }
    }
}

impl Default for LongRangeProtection {
    fn default() -> Self {
        Self::new(CheckpointAnchor::genesis())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_anchor() {
        let genesis = CheckpointAnchor::genesis();
        assert_eq!(genesis.block_number, 0);
        assert_eq!(genesis.authority_set_id, 0);
    }

    #[test]
    fn test_add_social_checkpoint() {
        let mut lrp = LongRangeProtection::default();

        let anchor1 = CheckpointAnchor::new(
            100,
            [1u8; 32],
            1,
            "First checkpoint".to_string(),
        );

        assert!(lrp.add_social_checkpoint(anchor1).is_ok());
        assert_eq!(lrp.social_checkpoints.len(), 1);

        // Add second checkpoint (must be newer)
        let anchor2 = CheckpointAnchor::new(
            200,
            [2u8; 32],
            2,
            "Second checkpoint".to_string(),
        );

        assert!(lrp.add_social_checkpoint(anchor2).is_ok());
        assert_eq!(lrp.social_checkpoints.len(), 2);
    }

    #[test]
    fn test_reject_old_checkpoint() {
        let mut lrp = LongRangeProtection::default();

        let anchor1 = CheckpointAnchor::new(
            100,
            [1u8; 32],
            1,
            "First checkpoint".to_string(),
        );

        lrp.add_social_checkpoint(anchor1).unwrap();

        // Try to add older checkpoint
        let anchor2 = CheckpointAnchor::new(
            50,
            [2u8; 32],
            0,
            "Older checkpoint".to_string(),
        );

        assert!(lrp.add_social_checkpoint(anchor2).is_err());
    }

    #[test]
    fn test_authority_set_expiration() {
        let mut lrp = LongRangeProtection::default();

        assert!(!lrp.is_authority_set_expired(0));
        assert!(!lrp.is_authority_set_expired(1));

        lrp.expire_authority_set(0);

        assert!(lrp.is_authority_set_expired(0));
        assert!(!lrp.is_authority_set_expired(1));
        assert_eq!(lrp.get_min_authority_set_id(), 1);
    }

    #[test]
    fn test_min_authority_set_id_update() {
        let mut lrp = LongRangeProtection::default();

        lrp.set_min_authority_set_id(5);
        assert_eq!(lrp.get_min_authority_set_id(), 5);

        // All sets below 5 should be considered expired
        assert!(lrp.is_authority_set_expired(0));
        assert!(lrp.is_authority_set_expired(4));
        assert!(!lrp.is_authority_set_expired(5));
        assert!(!lrp.is_authority_set_expired(6));
    }

    #[test]
    fn test_chain_history_verification() {
        let mut lrp = LongRangeProtection::default();

        let anchor = CheckpointAnchor::new(
            100,
            [1u8; 32],
            1,
            "Test checkpoint".to_string(),
        );
        lrp.add_social_checkpoint(anchor).unwrap();

        // Valid chain passing through checkpoint
        let valid_chain = vec![
            (50, [0u8; 32]),
            (100, [1u8; 32]), // Matches checkpoint
            (150, [2u8; 32]),
        ];

        assert!(lrp.verify_chain_history(&valid_chain).is_ok());

        // Invalid chain with wrong hash at checkpoint
        let invalid_chain = vec![
            (50, [0u8; 32]),
            (100, [99u8; 32]), // Wrong hash!
            (150, [2u8; 32]),
        ];

        assert!(lrp.verify_chain_history(&invalid_chain).is_err());
    }

    #[test]
    fn test_cleanup_expired_sets() {
        let mut lrp = LongRangeProtection::default();

        // Expire several sets
        lrp.expire_authority_set(1);
        lrp.expire_authority_set(2);
        lrp.expire_authority_set(3);
        lrp.expire_authority_set(10);
        lrp.expire_authority_set(11);

        assert_eq!(lrp.expired_authority_sets.len(), 5);

        // Cleanup sets older than 10
        lrp.cleanup_expired_sets(10);

        assert_eq!(lrp.expired_authority_sets.len(), 2); // Only 10 and 11 remain
        assert!(lrp.expired_authority_sets.contains(&10));
        assert!(lrp.expired_authority_sets.contains(&11));
    }

    #[test]
    fn test_max_social_checkpoints_limit() {
        let mut lrp = LongRangeProtection::default();

        // Add 150 checkpoints (should keep only last 100)
        for i in 1..=150 {
            let anchor = CheckpointAnchor::new(
                i * 100,
                [(i as u8); 32],
                i as u64,
                format!("Checkpoint {}", i),
            );
            lrp.add_social_checkpoint(anchor).unwrap();
        }

        assert_eq!(lrp.social_checkpoints.len(), 100);

        // First checkpoint should be #51 (oldest 50 removed)
        assert_eq!(lrp.social_checkpoints.first().unwrap().block_number, 51 * 100);

        // Last checkpoint should be #150
        assert_eq!(lrp.social_checkpoints.last().unwrap().block_number, 150 * 100);
    }
}
