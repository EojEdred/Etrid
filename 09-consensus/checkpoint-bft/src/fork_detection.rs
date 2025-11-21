// ═══════════════════════════════════════════════════════════════════════════
// FORK DETECTION - Fork-Aware Signature Collection with Partition Safety
// ═══════════════════════════════════════════════════════════════════════════
//
// Prevents signature replay attacks across competing forks and detects
// network partitions where multiple blocks at the same height achieve quorum.
//
// Security Properties:
// - Signatures only accepted for blocks on canonical chain
// - Detects competing forks at same height
// - Delays finalization if partition suspected
// - Emergency halt if dual finalization detected
//
// ═══════════════════════════════════════════════════════════════════════════

use parking_lot::RwLock;
use sp_core::H256;
use std::collections::HashMap;
use std::sync::Arc;

use crate::{CheckpointCertificate, CheckpointSignature, QUORUM_THRESHOLD};

/// Fork-aware signature collector with partition safety
pub struct ForkAwareCollector {
    /// Current canonical chain tip
    canonical_tip: Arc<RwLock<H256>>,

    /// Signatures by block hash -> validator_id -> signature
    /// Tracks ALL competing blocks at each height
    signatures: Arc<RwLock<HashMap<H256, HashMap<u32, CheckpointSignature>>>>,

    /// Fork detection: block number -> all known hashes at that height
    fork_tracker: Arc<RwLock<HashMap<u32, Vec<H256>>>>,

    /// Competing checkpoints at same height (multiple blocks with quorum)
    competing_checkpoints: Arc<RwLock<HashMap<u32, Vec<CheckpointCertificate>>>>,

    /// Finalized blocks (locked - cannot be changed)
    finalized: Arc<RwLock<HashMap<u32, H256>>>,

    /// Canonical chain cache (for quick ancestry checks)
    /// Hash -> (parent_hash, block_number)
    canonical_chain: Arc<RwLock<HashMap<H256, (H256, u32)>>>,
}

impl ForkAwareCollector {
    /// Create new fork-aware collector
    pub fn new(canonical_tip: H256) -> Self {
        Self {
            canonical_tip: Arc::new(RwLock::new(canonical_tip)),
            signatures: Arc::new(RwLock::new(HashMap::new())),
            fork_tracker: Arc::new(RwLock::new(HashMap::new())),
            competing_checkpoints: Arc::new(RwLock::new(HashMap::new())),
            finalized: Arc::new(RwLock::new(HashMap::new())),
            canonical_chain: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Update canonical chain tip
    pub fn update_canonical_tip(&self, new_tip: H256, block_number: u32, parent_hash: H256) {
        *self.canonical_tip.write() = new_tip;

        // Add to canonical chain cache
        self.canonical_chain
            .write()
            .insert(new_tip, (parent_hash, block_number));

        tracing::debug!(
            "🔄 Updated canonical tip to {:?} (block #{})",
            new_tip,
            block_number
        );
    }

    /// Add signature with fork protection
    pub fn add_signature(
        &self,
        signature: CheckpointSignature,
        canonical_chain: &[H256],
    ) -> Result<Option<CheckpointCertificate>, String> {
        let block_hash = H256::from_slice(&signature.block_hash);
        let block_number = signature.block_number;
        let validator_id = signature.validator_id;

        // Step 1: Verify block is on canonical chain
        if !self.is_on_canonical_chain(&block_hash, canonical_chain) {
            tracing::warn!(
                "⚠️ Rejecting signature from validator {} for non-canonical block {:?} at height #{}",
                validator_id,
                block_hash,
                block_number
            );
            return Err(format!(
                "Block {:?} is not on canonical chain",
                block_hash
            ));
        }

        // Step 2: Track fork (all blocks seen at this height)
        self.track_fork(block_number, block_hash);

        // Step 3: Add signature to collection
        let mut signatures = self.signatures.write();
        let block_sigs = signatures.entry(block_hash).or_insert_with(HashMap::new);

        // Check for duplicate
        if block_sigs.contains_key(&validator_id) {
            tracing::debug!(
                "Duplicate signature from validator {} for block {:?}",
                validator_id,
                block_hash
            );
            return Ok(None);
        }

        block_sigs.insert(validator_id, signature.clone());

        tracing::debug!(
            "Checkpoint #{}: {}/{} signatures for block {:?}",
            block_number,
            block_sigs.len(),
            QUORUM_THRESHOLD,
            block_hash
        );

        // Step 4: Check if quorum reached
        let sig_count = block_sigs.len();
        if sig_count >= QUORUM_THRESHOLD {
            drop(signatures); // Release lock

            tracing::info!(
                "✅ Quorum reached for checkpoint #{}: {}/{} signatures for block {:?}",
                block_number,
                sig_count,
                crate::TOTAL_VALIDATORS,
                block_hash
            );

            // Create certificate
            let sigs: Vec<CheckpointSignature> = self
                .signatures
                .read()
                .get(&block_hash)
                .unwrap()
                .values()
                .cloned()
                .collect();

            if let Some(cert) = CheckpointCertificate::new(
                block_number,
                signature.block_hash,
                signature.authority_set_id,
                sigs,
            ) {
                // Step 5: Check partition safety before finalizing
                match self.check_partition_safety(&cert) {
                    Ok(()) => {
                        // Safe to finalize
                        self.finalize_block(&cert)?;
                        return Ok(Some(cert));
                    }
                    Err(e) => {
                        // Partition detected - delay finalization
                        tracing::error!(
                            "🚨 PARTITION SAFETY VIOLATION: {}",
                            e
                        );
                        self.add_competing_checkpoint(cert.clone());
                        return Err(format!("Partition safety violation: {}", e));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Verify block is on canonical chain
    fn is_on_canonical_chain(&self, block_hash: &H256, canonical_chain: &[H256]) -> bool {
        canonical_chain.contains(block_hash)
    }

    /// Track all blocks seen at each height (fork detection)
    fn track_fork(&self, block_number: u32, block_hash: H256) {
        let mut fork_tracker = self.fork_tracker.write();
        let blocks_at_height = fork_tracker.entry(block_number).or_insert_with(Vec::new);

        if !blocks_at_height.contains(&block_hash) {
            blocks_at_height.push(block_hash);

            if blocks_at_height.len() > 1 {
                tracing::warn!(
                    "🚨 FORK DETECTED at height #{}: {} competing blocks",
                    block_number,
                    blocks_at_height.len()
                );
            }
        }
    }

    /// Check for network partition (multiple blocks with quorum at same height)
    pub fn check_partition_safety(
        &self,
        certificate: &CheckpointCertificate,
    ) -> Result<(), String> {
        let block_number = certificate.block_number;
        let block_hash = H256::from_slice(&certificate.block_hash);

        // Check if already finalized different block at this height
        if let Some(finalized_hash) = self.finalized.read().get(&block_number) {
            if *finalized_hash != block_hash {
                // CRITICAL: Dual finalization detected!
                tracing::error!(
                    "🚨🚨🚨 CRITICAL: DUAL FINALIZATION DETECTED at height #{}: \
                     Already finalized {:?}, attempting to finalize {:?}",
                    block_number,
                    finalized_hash,
                    block_hash
                );
                return Err(format!(
                    "DUAL FINALIZATION: Block #{} already finalized as {:?}, cannot finalize {:?}",
                    block_number, finalized_hash, block_hash
                ));
            }
        }

        // Check for competing checkpoints (other blocks at same height with quorum)
        let fork_tracker = self.fork_tracker.read();
        if let Some(blocks_at_height) = fork_tracker.get(&block_number) {
            for competing_hash in blocks_at_height {
                if *competing_hash == block_hash {
                    continue; // Skip self
                }

                // Check if competing block has quorum
                let sig_count = self
                    .signatures
                    .read()
                    .get(competing_hash)
                    .map(|sigs| sigs.len())
                    .unwrap_or(0);

                if sig_count >= QUORUM_THRESHOLD {
                    // PARTITION DETECTED: Multiple blocks have quorum
                    tracing::error!(
                        "🚨 NETWORK PARTITION SUSPECTED at height #{}: \
                         Both {:?} and {:?} have quorum ({} vs {} signatures)",
                        block_number,
                        block_hash,
                        competing_hash,
                        certificate.signatures.len(),
                        sig_count
                    );

                    return Err(format!(
                        "Network partition: Multiple blocks at height #{} have quorum",
                        block_number
                    ));
                }
            }
        }

        Ok(())
    }

    /// Finalize block (lock in place)
    fn finalize_block(&self, certificate: &CheckpointCertificate) -> Result<(), String> {
        let block_number = certificate.block_number;
        let block_hash = H256::from_slice(&certificate.block_hash);

        let mut finalized = self.finalized.write();

        // Double-check not already finalized (race condition protection)
        if let Some(existing) = finalized.get(&block_number) {
            if *existing != block_hash {
                return Err(format!(
                    "Cannot finalize {:?}: block #{} already finalized as {:?}",
                    block_hash, block_number, existing
                ));
            }
            // Same block, already finalized - ok
            return Ok(());
        }

        finalized.insert(block_number, block_hash);

        tracing::info!(
            "🔒 Finalized block #{} = {:?} with {} signatures",
            block_number,
            block_hash,
            certificate.signatures.len()
        );

        Ok(())
    }

    /// Add competing checkpoint for monitoring
    fn add_competing_checkpoint(&self, certificate: CheckpointCertificate) {
        let block_number = certificate.block_number;
        let mut competing = self.competing_checkpoints.write();

        competing
            .entry(block_number)
            .or_insert_with(Vec::new)
            .push(certificate);

        tracing::warn!(
            "⚠️ Added competing checkpoint at height #{}: now {} competing",
            block_number,
            competing.get(&block_number).unwrap().len()
        );
    }

    /// Get finalized block hash at height
    pub fn get_finalized(&self, block_number: u32) -> Option<H256> {
        self.finalized.read().get(&block_number).cloned()
    }

    /// Get competing checkpoints at height
    pub fn get_competing_checkpoints(&self, block_number: u32) -> Vec<CheckpointCertificate> {
        self.competing_checkpoints
            .read()
            .get(&block_number)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all blocks seen at height
    pub fn get_blocks_at_height(&self, block_number: u32) -> Vec<H256> {
        self.fork_tracker
            .read()
            .get(&block_number)
            .cloned()
            .unwrap_or_default()
    }

    /// Get signature count for specific block
    pub fn get_signature_count(&self, block_hash: &H256) -> usize {
        self.signatures
            .read()
            .get(block_hash)
            .map(|sigs| sigs.len())
            .unwrap_or(0)
    }

    /// Cleanup old data (prevent memory bloat)
    pub fn cleanup_old_data(&self, current_best_block: u32) {
        const KEEP_BLOCKS: u32 = 100;
        let cutoff = current_best_block.saturating_sub(KEEP_BLOCKS);

        // Clean fork tracker
        self.fork_tracker
            .write()
            .retain(|&block_number, _| block_number >= cutoff);

        // Clean competing checkpoints
        self.competing_checkpoints
            .write()
            .retain(|&block_number, _| block_number >= cutoff);

        // Keep finalized blocks (needed for safety checks)
        // Only clean very old finalized blocks
        let finalize_cutoff = current_best_block.saturating_sub(1000);
        self.finalized
            .write()
            .retain(|&block_number, _| block_number >= finalize_cutoff);

        tracing::debug!("🧹 Cleaned fork detection data older than block #{}", cutoff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::ed25519::Pair;
    use sp_core::Pair as PairT;

    fn create_test_signature(
        validator_id: u32,
        block_number: u32,
        block_hash: [u8; 32],
        _pair: &Pair,
    ) -> CheckpointSignature {
        CheckpointSignature {
            chain_id: [0u8; 32],
            block_number,
            block_hash,
            validator_id,
            validator_pubkey: [0u8; 32],
            authority_set_id: 1,
            authority_set_hash: [0u8; 32],
            checkpoint_type: crate::vrf::CheckpointType::Guaranteed,
            signature_nonce: 1,
            signature: vec![0u8; 64],
            timestamp_ms: 0,
        }
    }

    #[test]
    fn test_fork_detection() {
        let collector = ForkAwareCollector::new(H256::random());

        let block_number = 16;
        let block_hash_a = [1u8; 32];
        let block_hash_b = [2u8; 32];

        let pair = Pair::from_string("//Alice", None).unwrap();

        // Add signature for block A
        let sig_a = create_test_signature(0, block_number, block_hash_a, &pair);
        let canonical_chain = vec![H256::from(block_hash_a)];
        collector.add_signature(sig_a, &canonical_chain).ok();

        // Add signature for block B (fork)
        let sig_b = create_test_signature(1, block_number, block_hash_b, &pair);
        let canonical_chain_b = vec![H256::from(block_hash_b)];
        collector.add_signature(sig_b, &canonical_chain_b).ok();

        // Should detect fork
        let blocks_at_height = collector.get_blocks_at_height(block_number);
        assert_eq!(blocks_at_height.len(), 2);
    }

    #[test]
    fn test_partition_safety() {
        let collector = ForkAwareCollector::new(H256::random());
        let block_number = 16;
        let block_hash = [3u8; 32];

        // Create certificate
        let pairs: Vec<_> = (0..15)
            .map(|i| Pair::from_string(&format!("//Validator{}", i), None).unwrap())
            .collect();

        let sigs: Vec<_> = pairs
            .iter()
            .enumerate()
            .map(|(i, pair)| create_test_signature(i as u32, block_number, block_hash, pair))
            .collect();

        let cert = CheckpointCertificate::new(block_number, block_hash, 1, sigs).unwrap();

        // Should pass partition safety (no competing blocks)
        assert!(collector.check_partition_safety(&cert).is_ok());

        // Finalize
        collector.finalize_block(&cert).unwrap();

        // Attempting to finalize different block should fail
        let block_hash_2 = [4u8; 32];
        let sigs2: Vec<_> = pairs
            .iter()
            .enumerate()
            .map(|(i, pair)| create_test_signature(i as u32, block_number, block_hash_2, pair))
            .collect();

        let cert2 = CheckpointCertificate::new(block_number, block_hash_2, 1, sigs2).unwrap();

        // Should fail partition safety (dual finalization)
        assert!(collector.check_partition_safety(&cert2).is_err());
    }

    #[test]
    fn test_non_canonical_rejection() {
        let collector = ForkAwareCollector::new(H256::random());

        let block_hash = [5u8; 32];
        let block_number = 16;
        let pair = Pair::from_string("//Alice", None).unwrap();

        let sig = create_test_signature(0, block_number, block_hash, &pair);

        // Empty canonical chain (block not on it)
        let canonical_chain = vec![];

        // Should reject non-canonical block
        let result = collector.add_signature(sig, &canonical_chain);
        assert!(result.is_err());
    }
}
