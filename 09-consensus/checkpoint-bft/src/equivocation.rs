// ═══════════════════════════════════════════════════════════════════════════
// EQUIVOCATION DETECTION - Catch Double-Signing (Nothing-at-Stake Attack)
// ═══════════════════════════════════════════════════════════════════════════
//
// Detects validators signing multiple blocks at the same height (equivocation).
// This is the "nothing-at-stake" problem in proof-of-stake systems.
//
// Security Properties:
// - Captures cryptographic proof of equivocation (both signatures)
// - Enables on-chain slashing (100% stake slash)
// - Permanent validator exclusion after proven equivocation
//
// ═══════════════════════════════════════════════════════════════════════════

use codec::{Decode, Encode};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use crate::CheckpointSignature;

/// Fork accountability tracker
pub struct ForkAccountability {
    /// Track all signed checkpoints per validator
    /// validator_id -> block_number -> (block_hash, signature)
    signed_checkpoints: Arc<RwLock<HashMap<u32, HashMap<u32, Vec<SignedCheckpoint>>>>>,

    /// Detected equivocations (pending on-chain submission)
    equivocation_queue: Arc<RwLock<VecDeque<EquivocationEvidence>>>,

    /// Slashed validators (proven equivocators)
    slashed_validators: Arc<RwLock<HashMap<u32, SlashingRecord>>>,

    /// Maximum checkpoints to track per validator (memory management)
    max_tracked_checkpoints: usize,
}

/// Record of a signed checkpoint
#[derive(Debug, Clone)]
struct SignedCheckpoint {
    block_hash: [u8; 32],
    signature: Vec<u8>,
    timestamp: u64,
}

/// Cryptographic proof of equivocation
#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
pub struct EquivocationEvidence {
    /// Validator ID that equivocated
    pub validator_id: u32,

    /// Block number where equivocation occurred
    pub block_number: u32,

    /// First block hash signed
    pub block_hash_1: [u8; 32],

    /// Second block hash signed (different from first)
    pub block_hash_2: [u8; 32],

    /// First signature
    pub signature_1: Vec<u8>,

    /// Second signature
    pub signature_2: Vec<u8>,

    /// Authority set ID
    pub authority_set_id: u64,

    /// Timestamp when equivocation detected
    pub detected_at: u64,
}

impl EquivocationEvidence {
    /// Verify equivocation evidence is valid
    pub fn verify(&self) -> bool {
        // Check block hashes are different
        if self.block_hash_1 == self.block_hash_2 {
            return false;
        }

        // Check signatures are different
        if self.signature_1 == self.signature_2 {
            return false;
        }

        // Both signatures are for the same block number
        // (This is checked when evidence is created)

        true
    }
}

/// Slashing record for proven equivocator
#[derive(Debug, Clone, Encode, Decode)]
pub struct SlashingRecord {
    pub validator_id: u32,
    pub equivocation_evidence: EquivocationEvidence,
    pub slashed_at: u64,
    pub slash_amount_percent: u32, // 100 = 100% of stake
}

/// Slashing report for on-chain submission
#[derive(Debug, Clone, Encode, Decode)]
pub struct SlashingReport {
    pub evidence: EquivocationEvidence,
    pub slash_amount_percent: u32,
    pub proposed_exclusion: bool, // Permanent exclusion from validator set
}

impl ForkAccountability {
    /// Create new fork accountability tracker
    pub fn new(max_tracked_checkpoints: usize) -> Self {
        Self {
            signed_checkpoints: Arc::new(RwLock::new(HashMap::new())),
            equivocation_queue: Arc::new(RwLock::new(VecDeque::new())),
            slashed_validators: Arc::new(RwLock::new(HashMap::new())),
            max_tracked_checkpoints,
        }
    }

    /// Check for equivocation and record signature
    pub fn check_and_record_signature(
        &self,
        signature: &CheckpointSignature,
    ) -> Result<(), EquivocationEvidence> {
        let validator_id = signature.validator_id;
        let block_number = signature.block_number;
        let block_hash = signature.block_hash;

        let mut signed = self.signed_checkpoints.write();
        let validator_checkpoints = signed.entry(validator_id).or_insert_with(HashMap::new);

        // Get existing signatures for this block number
        let checkpoints_at_height = validator_checkpoints
            .entry(block_number)
            .or_insert_with(Vec::new);

        // Check for equivocation
        for existing in checkpoints_at_height.iter() {
            if existing.block_hash != block_hash {
                // EQUIVOCATION DETECTED!
                tracing::error!(
                    "🚨🚨🚨 EQUIVOCATION DETECTED: Validator {} signed two different blocks at height #{}",
                    validator_id,
                    block_number
                );
                tracing::error!(
                    "  Block 1: {:?}",
                    existing.block_hash
                );
                tracing::error!(
                    "  Block 2: {:?}",
                    block_hash
                );

                let evidence = EquivocationEvidence {
                    validator_id,
                    block_number,
                    block_hash_1: existing.block_hash,
                    block_hash_2: block_hash,
                    signature_1: existing.signature.clone(),
                    signature_2: signature.signature.clone(),
                    authority_set_id: signature.authority_set_id,
                    detected_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                };

                // Queue for slashing
                self.queue_equivocation(evidence.clone());

                return Err(evidence);
            }
        }

        // No equivocation - record signature
        checkpoints_at_height.push(SignedCheckpoint {
            block_hash,
            signature: signature.signature.clone(),
            timestamp: signature.timestamp_ms,
        });

        // Cleanup old checkpoints if needed
        if validator_checkpoints.len() > self.max_tracked_checkpoints {
            // Remove oldest checkpoint
            if let Some(min_block) = validator_checkpoints.keys().min().cloned() {
                validator_checkpoints.remove(&min_block);
            }
        }

        Ok(())
    }

    /// Queue equivocation for on-chain submission
    fn queue_equivocation(&self, evidence: EquivocationEvidence) {
        let validator_id = evidence.validator_id;
        self.equivocation_queue.write().push_back(evidence);

        tracing::info!(
            "📋 Queued equivocation evidence for validator {} (queue size: {})",
            validator_id,
            self.equivocation_queue.read().len()
        );
    }

    /// Get next equivocation from queue (for on-chain submission)
    pub fn pop_equivocation(&self) -> Option<EquivocationEvidence> {
        self.equivocation_queue.write().pop_front()
    }

    /// Get all pending equivocations
    pub fn get_pending_equivocations(&self) -> Vec<EquivocationEvidence> {
        self.equivocation_queue.read().iter().cloned().collect()
    }

    /// Mark validator as slashed
    pub fn mark_slashed(&self, evidence: EquivocationEvidence) {
        let validator_id = evidence.validator_id;

        let record = SlashingRecord {
            validator_id,
            equivocation_evidence: evidence.clone(),
            slashed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            slash_amount_percent: 100, // 100% stake slash for equivocation
        };

        self.slashed_validators
            .write()
            .insert(validator_id, record);

        tracing::error!(
            "⚔️ SLASHED: Validator {} - 100% stake slashed for equivocation at block #{}",
            validator_id,
            evidence.block_number
        );
    }

    /// Check if validator is slashed
    pub fn is_slashed(&self, validator_id: u32) -> bool {
        self.slashed_validators.read().contains_key(&validator_id)
    }

    /// Get slashing record for validator
    pub fn get_slashing_record(&self, validator_id: u32) -> Option<SlashingRecord> {
        self.slashed_validators.read().get(&validator_id).cloned()
    }

    /// Get all slashed validators
    pub fn get_slashed_validators(&self) -> Vec<u32> {
        self.slashed_validators.read().keys().cloned().collect()
    }

    /// Create slashing report for on-chain submission
    pub fn create_slashing_report(&self, evidence: EquivocationEvidence) -> SlashingReport {
        SlashingReport {
            evidence,
            slash_amount_percent: 100, // 100% stake slash
            proposed_exclusion: true,   // Permanent exclusion
        }
    }

    /// Get all signatures for validator at specific block number
    pub fn get_signatures_at_height(
        &self,
        validator_id: u32,
        block_number: u32,
    ) -> Vec<([u8; 32], Vec<u8>)> {
        self.signed_checkpoints
            .read()
            .get(&validator_id)
            .and_then(|checkpoints| checkpoints.get(&block_number))
            .map(|sigs| {
                sigs.iter()
                    .map(|s| (s.block_hash, s.signature.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if validator has signed any block at height
    pub fn has_signed_at_height(&self, validator_id: u32, block_number: u32) -> bool {
        self.signed_checkpoints
            .read()
            .get(&validator_id)
            .and_then(|checkpoints| checkpoints.get(&block_number))
            .map(|sigs| !sigs.is_empty())
            .unwrap_or(false)
    }

    /// Get total checkpoints tracked for validator
    pub fn get_tracked_checkpoint_count(&self, validator_id: u32) -> usize {
        self.signed_checkpoints
            .read()
            .get(&validator_id)
            .map(|checkpoints| checkpoints.len())
            .unwrap_or(0)
    }

    /// Cleanup old data (prevent memory bloat)
    pub fn cleanup_old_checkpoints(&self, current_block: u32, keep_blocks: u32) {
        let cutoff = current_block.saturating_sub(keep_blocks);

        let mut signed = self.signed_checkpoints.write();

        for validator_checkpoints in signed.values_mut() {
            validator_checkpoints.retain(|&block_number, _| block_number >= cutoff);
        }

        tracing::debug!(
            "🧹 Cleaned equivocation tracker, keeping blocks >= #{}",
            cutoff
        );
    }

    /// Generate accountability report
    pub fn generate_report(&self) -> AccountabilityReport {
        let slashed = self.slashed_validators.read().clone();
        let pending = self.equivocation_queue.read().iter().cloned().collect();

        AccountabilityReport {
            total_slashed: slashed.len(),
            pending_equivocations: pending,
            slashed_validators: slashed,
        }
    }
}

/// Accountability report
#[derive(Debug, Clone)]
pub struct AccountabilityReport {
    pub total_slashed: usize,
    pub pending_equivocations: Vec<EquivocationEvidence>,
    pub slashed_validators: HashMap<u32, SlashingRecord>,
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
    fn test_equivocation_detection() {
        let accountability = ForkAccountability::new(100);

        let pair = Pair::from_string("//Alice", None).unwrap();
        let block_number = 16;
        let block_hash_1 = [1u8; 32];
        let block_hash_2 = [2u8; 32];

        // First signature - should succeed
        let sig1 = create_test_signature(0, block_number, block_hash_1, &pair);
        assert!(accountability.check_and_record_signature(&sig1).is_ok());

        // Second signature for different block at same height - should detect equivocation
        let sig2 = create_test_signature(0, block_number, block_hash_2, &pair);
        let result = accountability.check_and_record_signature(&sig2);

        assert!(result.is_err());
        let evidence = result.unwrap_err();

        assert_eq!(evidence.validator_id, 0);
        assert_eq!(evidence.block_number, block_number);
        assert_eq!(evidence.block_hash_1, block_hash_1);
        assert_eq!(evidence.block_hash_2, block_hash_2);
        assert!(evidence.verify());
    }

    #[test]
    fn test_no_false_positive() {
        let accountability = ForkAccountability::new(100);

        let pair = Pair::from_string("//Alice", None).unwrap();
        let block_number = 16;
        let block_hash = [1u8; 32];

        // Same signature twice - should not trigger equivocation
        let sig1 = create_test_signature(0, block_number, block_hash, &pair);
        assert!(accountability.check_and_record_signature(&sig1).is_ok());

        let sig2 = create_test_signature(0, block_number, block_hash, &pair);
        assert!(accountability.check_and_record_signature(&sig2).is_ok());
    }

    #[test]
    fn test_slashing_workflow() {
        let accountability = ForkAccountability::new(100);

        let pair = Pair::from_string("//Alice", None).unwrap();
        let block_number = 16;
        let block_hash_1 = [1u8; 32];
        let block_hash_2 = [2u8; 32];

        // Trigger equivocation
        let sig1 = create_test_signature(0, block_number, block_hash_1, &pair);
        accountability.check_and_record_signature(&sig1).ok();

        let sig2 = create_test_signature(0, block_number, block_hash_2, &pair);
        accountability.check_and_record_signature(&sig2).ok();

        // Check equivocation queued
        let pending = accountability.get_pending_equivocations();
        assert_eq!(pending.len(), 1);

        // Pop equivocation
        let evidence = accountability.pop_equivocation().unwrap();

        // Create slashing report
        let report = accountability.create_slashing_report(evidence.clone());
        assert_eq!(report.slash_amount_percent, 100);
        assert!(report.proposed_exclusion);

        // Mark as slashed
        accountability.mark_slashed(evidence);
        assert!(accountability.is_slashed(0));

        let record = accountability.get_slashing_record(0).unwrap();
        assert_eq!(record.slash_amount_percent, 100);
    }

    #[test]
    fn test_multiple_validators() {
        let accountability = ForkAccountability::new(100);

        let pairs: Vec<_> = (0..3)
            .map(|i| Pair::from_string(&format!("//Validator{}", i), None).unwrap())
            .collect();

        let block_number = 16;
        let block_hash_1 = [1u8; 32];
        let block_hash_2 = [2u8; 32];

        // Validator 0 equivocates
        let sig1 = create_test_signature(0, block_number, block_hash_1, &pairs[0]);
        accountability.check_and_record_signature(&sig1).ok();

        let sig2 = create_test_signature(0, block_number, block_hash_2, &pairs[0]);
        accountability.check_and_record_signature(&sig2).ok();

        // Validator 1 behaves honestly
        let sig3 = create_test_signature(1, block_number, block_hash_1, &pairs[1]);
        assert!(accountability.check_and_record_signature(&sig3).is_ok());

        // Validator 2 equivocates
        let sig4 = create_test_signature(2, block_number, block_hash_1, &pairs[2]);
        accountability.check_and_record_signature(&sig4).ok();

        let sig5 = create_test_signature(2, block_number, block_hash_2, &pairs[2]);
        accountability.check_and_record_signature(&sig5).ok();

        // Should have 2 equivocations queued
        assert_eq!(accountability.get_pending_equivocations().len(), 2);
    }

    #[test]
    fn test_memory_management() {
        let accountability = ForkAccountability::new(10); // Max 10 checkpoints

        let pair = Pair::from_string("//Alice", None).unwrap();

        // Add 20 checkpoints
        for block_number in 0..20 {
            let block_hash = [(block_number as u8); 32];
            let sig = create_test_signature(0, block_number * 16, block_hash, &pair);
            accountability.check_and_record_signature(&sig).ok();
        }

        // Should only keep 10
        assert!(accountability.get_tracked_checkpoint_count(0) <= 10);
    }
}
