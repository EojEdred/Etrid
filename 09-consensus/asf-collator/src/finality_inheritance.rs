//! # Finality Inheritance
//!
//! Manages finality inheritance from relay chain to parachains.
//!
//! PBC blocks can achieve irreversible finality through two paths:
//! 1. Accumulating 50+ ASF certificates locally
//! 2. Being finalized by the relay chain (FlareChain)

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    Hash, BlockNumber, ParaId, CollatorFinalityLevel, RelayChainFinalityProof,
    AsfError, AsfResult,
};

use asf_algorithm::FinalityLevel;

/// Finality status for a parachain block
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct FinalityStatus {
    /// Parachain block hash
    pub para_hash: Hash,
    /// Parachain block number
    pub para_block: BlockNumber,
    /// Local finality level (from ASF certificates)
    pub local_finality: CollatorFinalityLevel,
    /// Number of local certificates
    pub certificate_count: u32,
    /// Relay chain finality proof (if available)
    pub relay_finality: Option<RelayChainFinalityProof>,
    /// Combined finality level (max of local and inherited)
    pub combined_finality: CollatorFinalityLevel,
}

impl FinalityStatus {
    /// Create new finality status
    pub fn new(para_hash: Hash, para_block: BlockNumber) -> Self {
        Self {
            para_hash,
            para_block,
            local_finality: CollatorFinalityLevel::None,
            certificate_count: 0,
            relay_finality: None,
            combined_finality: CollatorFinalityLevel::None,
        }
    }

    /// Update with local certificates
    pub fn update_local(&mut self, certificate_count: u32) {
        self.certificate_count = certificate_count;
        self.local_finality = CollatorFinalityLevel::from(certificate_count);
        self.update_combined();
    }

    /// Update with relay chain finality proof
    pub fn update_relay(&mut self, proof: RelayChainFinalityProof) {
        self.relay_finality = Some(proof);
        self.update_combined();
    }

    /// Recalculate combined finality
    fn update_combined(&mut self) {
        self.combined_finality = match &self.relay_finality {
            Some(proof) => {
                let inherited = proof.inherit_finality();
                // Take maximum of local and inherited
                if inherited > self.local_finality {
                    inherited
                } else {
                    self.local_finality
                }
            }
            None => self.local_finality,
        };
    }

    /// Check if block is irreversibly finalized
    pub fn is_irreversible(&self) -> bool {
        self.combined_finality == CollatorFinalityLevel::Irreversible
    }

    /// Check if finalized by any level
    pub fn is_finalized(&self) -> bool {
        self.combined_finality.is_finalized()
    }
}

/// Manages finality tracking for parachain blocks
pub struct FinalityTracker {
    /// Parachain ID
    para_id: ParaId,
    /// Finality status for each block
    block_finality: BTreeMap<BlockNumber, FinalityStatus>,
    /// Highest finalized block
    highest_finalized: Option<BlockNumber>,
}

impl FinalityTracker {
    /// Create new finality tracker
    pub fn new(para_id: ParaId) -> Self {
        Self {
            para_id,
            block_finality: BTreeMap::new(),
            highest_finalized: None,
        }
    }

    /// Update local finality for a block
    pub fn update_local_finality(
        &mut self,
        block_hash: Hash,
        block_number: BlockNumber,
        certificate_count: u32,
    ) {
        let status = self
            .block_finality
            .entry(block_number)
            .or_insert_with(|| FinalityStatus::new(block_hash, block_number));

        status.update_local(certificate_count);

        // Update highest finalized if applicable
        if status.is_finalized() {
            match self.highest_finalized {
                Some(highest) if block_number > highest => {
                    self.highest_finalized = Some(block_number);
                }
                None => {
                    self.highest_finalized = Some(block_number);
                }
                _ => {}
            }
        }
    }

    /// Update relay chain finality
    pub fn update_relay_finality(&mut self, proof: RelayChainFinalityProof) -> AsfResult<()> {
        let block_number = proof.para_block;
        let status = self
            .block_finality
            .entry(block_number)
            .or_insert_with(|| FinalityStatus::new(proof.para_hash, block_number));

        // Verify block hash matches
        if status.para_hash != proof.para_hash {
            return Err(AsfError::InvalidCertificate("Block hash mismatch"));
        }

        status.update_relay(proof);

        // Update highest finalized
        if status.is_irreversible() {
            match self.highest_finalized {
                Some(highest) if block_number > highest => {
                    self.highest_finalized = Some(block_number);
                }
                None => {
                    self.highest_finalized = Some(block_number);
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Get finality status for a block
    pub fn get_status(&self, block_number: BlockNumber) -> Option<&FinalityStatus> {
        self.block_finality.get(&block_number)
    }

    /// Get combined finality level for a block
    pub fn get_finality_level(&self, block_number: BlockNumber) -> CollatorFinalityLevel {
        self.block_finality
            .get(&block_number)
            .map(|s| s.combined_finality)
            .unwrap_or(CollatorFinalityLevel::None)
    }

    /// Check if block is finalized
    pub fn is_finalized(&self, block_number: BlockNumber) -> bool {
        self.block_finality
            .get(&block_number)
            .map(|s| s.is_finalized())
            .unwrap_or(false)
    }

    /// Check if block is irreversibly finalized
    pub fn is_irreversible(&self, block_number: BlockNumber) -> bool {
        self.block_finality
            .get(&block_number)
            .map(|s| s.is_irreversible())
            .unwrap_or(false)
    }

    /// Get highest finalized block number
    pub fn highest_finalized_block(&self) -> Option<BlockNumber> {
        self.highest_finalized
    }

    /// Prune old finality data (keep last N blocks)
    pub fn prune(&mut self, keep_blocks: u64) {
        if let Some(highest) = self.highest_finalized {
            let cutoff = highest.saturating_sub(keep_blocks);
            self.block_finality.retain(|&bn, _| bn > cutoff);
        }
    }

    /// Get all blocks waiting for relay chain finality
    pub fn pending_relay_finality(&self) -> Vec<BlockNumber> {
        self.block_finality
            .iter()
            .filter(|(_, status)| {
                status.relay_finality.is_none()
                    && status.local_finality != CollatorFinalityLevel::Irreversible
            })
            .map(|(bn, _)| *bn)
            .collect()
    }
}

/// Relay chain sync coordinator
pub struct RelayChainSyncCoordinator {
    /// Last synced relay chain block
    last_synced_relay_block: BlockNumber,
    /// Parachain -> relay chain block mapping
    para_to_relay: BTreeMap<BlockNumber, BlockNumber>,
}

impl RelayChainSyncCoordinator {
    /// Create new coordinator
    pub fn new() -> Self {
        Self {
            last_synced_relay_block: 0,
            para_to_relay: BTreeMap::new(),
        }
    }

    /// Record parachain block inclusion in relay chain
    pub fn record_inclusion(
        &mut self,
        para_block: BlockNumber,
        relay_block: BlockNumber,
    ) {
        self.para_to_relay.insert(para_block, relay_block);
        if relay_block > self.last_synced_relay_block {
            self.last_synced_relay_block = relay_block;
        }
    }

    /// Get relay block for parachain block
    pub fn get_relay_block(&self, para_block: BlockNumber) -> Option<BlockNumber> {
        self.para_to_relay.get(&para_block).copied()
    }

    /// Check if parachain block is included in relay chain
    pub fn is_included(&self, para_block: BlockNumber) -> bool {
        self.para_to_relay.contains_key(&para_block)
    }

    /// Get last synced relay block
    pub fn last_synced(&self) -> BlockNumber {
        self.last_synced_relay_block
    }

    /// Prune old mappings
    pub fn prune(&mut self, keep_blocks: u64) {
        let cutoff = self.last_synced_relay_block.saturating_sub(keep_blocks);
        self.para_to_relay.retain(|_, &mut relay_bn| relay_bn > cutoff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::H256;

    #[test]
    fn test_finality_status_local() {
        let hash = H256::random();
        let mut status = FinalityStatus::new(hash, 100);

        // Initially not finalized
        assert!(!status.is_finalized());
        assert_eq!(status.local_finality, CollatorFinalityLevel::None);

        // Update with certificates
        status.update_local(7);
        assert_eq!(status.local_finality, CollatorFinalityLevel::Weak);
        assert!(status.is_finalized());

        status.update_local(15);
        assert_eq!(status.local_finality, CollatorFinalityLevel::Moderate);

        status.update_local(60);
        assert_eq!(status.local_finality, CollatorFinalityLevel::Irreversible);
        assert!(status.is_irreversible());
    }

    #[test]
    fn test_finality_inheritance() {
        let para_hash = H256::random();
        let relay_hash = H256::random();
        let mut status = FinalityStatus::new(para_hash, 100);

        // Start with weak local finality
        status.update_local(7);
        assert_eq!(status.combined_finality, CollatorFinalityLevel::Weak);

        // Relay chain provides strong finality
        let proof = RelayChainFinalityProof {
            relay_block: 1000,
            relay_hash,
            para_block: 100,
            para_hash,
            relay_finality: FinalityLevel::Strong,
        };
        status.update_relay(proof);

        // Should inherit irreversible from relay chain
        assert_eq!(status.combined_finality, CollatorFinalityLevel::Irreversible);
    }

    #[test]
    fn test_finality_tracker() {
        let mut tracker = FinalityTracker::new(1000);
        let hash1 = H256::random();
        let hash2 = H256::random();

        // Add blocks
        tracker.update_local_finality(hash1, 100, 15);
        tracker.update_local_finality(hash2, 101, 25);

        // Check finality levels
        assert_eq!(
            tracker.get_finality_level(100),
            CollatorFinalityLevel::Moderate
        );
        assert_eq!(
            tracker.get_finality_level(101),
            CollatorFinalityLevel::Strong
        );

        // Highest finalized should be 101
        assert_eq!(tracker.highest_finalized_block(), Some(101));
    }

    #[test]
    fn test_relay_chain_sync() {
        let mut coordinator = RelayChainSyncCoordinator::new();

        // Record inclusions
        coordinator.record_inclusion(100, 1000);
        coordinator.record_inclusion(101, 1002);
        coordinator.record_inclusion(102, 1004);

        assert_eq!(coordinator.get_relay_block(100), Some(1000));
        assert_eq!(coordinator.get_relay_block(101), Some(1002));
        assert!(coordinator.is_included(102));
        assert!(!coordinator.is_included(103));
        assert_eq!(coordinator.last_synced(), 1004);
    }

    #[test]
    fn test_tracker_prune() {
        let mut tracker = FinalityTracker::new(1000);

        // Add many blocks
        for i in 0..100 {
            let hash = H256::random();
            tracker.update_local_finality(hash, i, 10);
        }

        // Prune keeping last 50
        tracker.prune(50);

        // Old blocks should be removed
        assert!(tracker.get_status(10).is_none());
        // Recent blocks should remain
        assert!(tracker.get_status(99).is_some());
    }
}
