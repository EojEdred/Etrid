// director-node/src/slashing.rs
// SLASHING DETECTOR
// Detects double-signing and equivocation attacks

use detrp2p::PeerId;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use blake2::{Blake2b512, Digest};

// ============================================================================
// SLASHING TYPES
// ============================================================================

pub type CheckpointNumber = u64;
pub type BlockHash = [u8; 32];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointSignature {
    pub validator: PeerId,
    pub checkpoint: CheckpointNumber,
    pub block_hash: BlockHash,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlashingEvidence {
    pub validator: PeerId,
    pub offense_type: SlashingOffense,
    pub proof: SlashingProof,
    pub detected_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlashingOffense {
    /// Validator signed two different blocks at same checkpoint
    DoubleSign,

    /// Validator signed conflicting certificates
    Equivocation,

    /// Validator submitted invalid checkpoint
    InvalidCheckpoint,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlashingProof {
    pub checkpoint: CheckpointNumber,
    pub signature_1: CheckpointSignature,
    pub signature_2: CheckpointSignature,
    pub evidence_hash: [u8; 32],
}

// ============================================================================
// SLASHING DETECTOR
// ============================================================================

pub struct SlashingDetector {
    /// Track all checkpoint signatures per validator
    /// Key: (PeerId, CheckpointNumber) -> Vec<BlockHash>
    checkpoint_signatures: HashMap<(PeerId, CheckpointNumber), Vec<BlockHash>>,

    /// Detected slashing evidence
    evidence: Vec<SlashingEvidence>,
}

impl SlashingDetector {
    pub fn new() -> Self {
        Self {
            checkpoint_signatures: HashMap::new(),
            evidence: Vec::new(),
        }
    }

    /// Check certificate for double-signing
    /// Returns Some(evidence) if double-sign detected
    pub fn check_certificate(
        &mut self,
        validator: &PeerId,
        data: &[u8],
    ) -> Option<SlashingEvidence> {
        // Parse certificate data (simplified - in production use proper ASF types)
        let checkpoint = self.extract_checkpoint(data)?;
        let block_hash = self.extract_block_hash(data)?;

        let key = (*validator, checkpoint);

        // Get existing signatures for this validator+checkpoint
        let existing_hashes = self.checkpoint_signatures.entry(key).or_insert_with(Vec::new);

        // Check if validator already signed a DIFFERENT block at this checkpoint
        for existing_hash in existing_hashes.iter() {
            if *existing_hash != block_hash {
                // DOUBLE SIGN DETECTED!
                log::error!(
                    "🚨 DOUBLE SIGN DETECTED: Validator {:?} signed two different blocks at checkpoint {}",
                    validator,
                    checkpoint
                );

                // Copy values before creating evidence (to avoid borrow checker issues)
                let existing_hash_copy = *existing_hash;
                let validator_copy = *validator;

                // Create evidence hash
                let evidence_hash = self.hash_evidence(validator_copy, checkpoint, existing_hash_copy, block_hash);

                let evidence = SlashingEvidence {
                    validator: validator_copy,
                    offense_type: SlashingOffense::DoubleSign,
                    proof: SlashingProof {
                        checkpoint,
                        signature_1: CheckpointSignature {
                            validator: validator_copy,
                            checkpoint,
                            block_hash: existing_hash_copy,
                            signature: vec![],
                            timestamp: 0,
                        },
                        signature_2: CheckpointSignature {
                            validator: validator_copy,
                            checkpoint,
                            block_hash,
                            signature: vec![],
                            timestamp: 0,
                        },
                        evidence_hash,
                    },
                    detected_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                self.evidence.push(evidence.clone());
                return Some(evidence);
            }
        }

        // No double-sign - record this signature
        existing_hashes.push(block_hash);
        None
    }

    /// Extract checkpoint number from certificate data
    fn extract_checkpoint(&self, data: &[u8]) -> Option<CheckpointNumber> {
        // Simplified extraction - in production parse proper ASF certificate
        if data.len() >= 8 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&data[0..8]);
            Some(u64::from_be_bytes(bytes))
        } else {
            None
        }
    }

    /// Extract block hash from certificate data
    fn extract_block_hash(&self, data: &[u8]) -> Option<BlockHash> {
        // Simplified extraction - in production parse proper ASF certificate
        if data.len() >= 40 {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&data[8..40]);
            Some(hash)
        } else {
            None
        }
    }

    /// Hash slashing evidence for uniqueness
    fn hash_evidence(
        &self,
        validator: PeerId,
        checkpoint: CheckpointNumber,
        hash1: BlockHash,
        hash2: BlockHash,
    ) -> [u8; 32] {
        let mut hasher = Blake2b512::new();
        hasher.update(validator.as_bytes());
        hasher.update(&checkpoint.to_be_bytes());
        hasher.update(&hash1);
        hasher.update(&hash2);

        let result = hasher.finalize();
        let mut evidence_hash = [0u8; 32];
        evidence_hash.copy_from_slice(&result[..32]);
        evidence_hash
    }

    /// Get all detected evidence
    pub fn get_evidence(&self) -> Vec<SlashingEvidence> {
        self.evidence.clone()
    }

    /// Clear evidence (after submitting to on-chain slashing pallet)
    pub fn clear_evidence(&mut self) {
        self.evidence.clear();
    }

    /// Get signature count for validator at checkpoint
    pub fn get_signature_count(&self, validator: &PeerId, checkpoint: CheckpointNumber) -> usize {
        self.checkpoint_signatures
            .get(&(*validator, checkpoint))
            .map(|v| v.len())
            .unwrap_or(0)
    }
}

impl Default for SlashingDetector {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_sign_detection() {
        let mut detector = SlashingDetector::new();
        let validator = PeerId::new([1u8; 32]);

        // Create first certificate (checkpoint 10, block hash A)
        let mut cert1 = vec![0u8; 64];
        cert1[0..8].copy_from_slice(&10u64.to_be_bytes()); // Checkpoint 10
        cert1[8..40].copy_from_slice(&[0xAA; 32]); // Block hash A

        // First signature - should be fine
        let evidence1 = detector.check_certificate(&validator, &cert1);
        assert!(evidence1.is_none());

        // Create second certificate (SAME checkpoint 10, DIFFERENT block hash B)
        let mut cert2 = vec![0u8; 64];
        cert2[0..8].copy_from_slice(&10u64.to_be_bytes()); // Checkpoint 10
        cert2[8..40].copy_from_slice(&[0xBB; 32]); // Block hash B (DIFFERENT!)

        // Second signature - DOUBLE SIGN!
        let evidence2 = detector.check_certificate(&validator, &cert2);
        assert!(evidence2.is_some());

        let evidence = evidence2.unwrap();
        assert_eq!(evidence.offense_type, SlashingOffense::DoubleSign);
        assert_eq!(evidence.validator, validator);
        assert_eq!(evidence.proof.checkpoint, 10);
    }

    #[test]
    fn test_no_double_sign_same_block() {
        let mut detector = SlashingDetector::new();
        let validator = PeerId::new([2u8; 32]);

        // Create certificate (checkpoint 5, block hash A)
        let mut cert = vec![0u8; 64];
        cert[0..8].copy_from_slice(&5u64.to_be_bytes());
        cert[8..40].copy_from_slice(&[0xAA; 32]);

        // Sign same block twice - should be fine
        let evidence1 = detector.check_certificate(&validator, &cert);
        assert!(evidence1.is_none());

        let evidence2 = detector.check_certificate(&validator, &cert);
        assert!(evidence2.is_none());
    }

    #[test]
    fn test_multiple_validators() {
        let mut detector = SlashingDetector::new();
        let validator1 = PeerId::new([1u8; 32]);
        let validator2 = PeerId::new([2u8; 32]);

        // Create certificate for checkpoint 10
        let mut cert = vec![0u8; 64];
        cert[0..8].copy_from_slice(&10u64.to_be_bytes());
        cert[8..40].copy_from_slice(&[0xAA; 32]);

        // Both validators sign - should be fine
        let evidence1 = detector.check_certificate(&validator1, &cert);
        assert!(evidence1.is_none());

        let evidence2 = detector.check_certificate(&validator2, &cert);
        assert!(evidence2.is_none());

        // Validator 1 double-signs
        let mut cert2 = vec![0u8; 64];
        cert2[0..8].copy_from_slice(&10u64.to_be_bytes());
        cert2[8..40].copy_from_slice(&[0xBB; 32]);

        let evidence3 = detector.check_certificate(&validator1, &cert2);
        assert!(evidence3.is_some());

        // Validator 2 should still be clean
        assert_eq!(detector.get_signature_count(&validator2, 10), 1);
    }

    #[test]
    fn test_different_checkpoints() {
        let mut detector = SlashingDetector::new();
        let validator = PeerId::new([3u8; 32]);

        // Sign checkpoint 10 with hash A
        let mut cert1 = vec![0u8; 64];
        cert1[0..8].copy_from_slice(&10u64.to_be_bytes());
        cert1[8..40].copy_from_slice(&[0xAA; 32]);

        detector.check_certificate(&validator, &cert1);

        // Sign checkpoint 11 with hash B - different checkpoint, so fine
        let mut cert2 = vec![0u8; 64];
        cert2[0..8].copy_from_slice(&11u64.to_be_bytes());
        cert2[8..40].copy_from_slice(&[0xBB; 32]);

        let evidence = detector.check_certificate(&validator, &cert2);
        assert!(evidence.is_none());
    }

    #[test]
    fn test_get_evidence() {
        let mut detector = SlashingDetector::new();
        let validator = PeerId::new([4u8; 32]);

        // Trigger double-sign
        let mut cert1 = vec![0u8; 64];
        cert1[0..8].copy_from_slice(&20u64.to_be_bytes());
        cert1[8..40].copy_from_slice(&[0xAA; 32]);
        detector.check_certificate(&validator, &cert1);

        let mut cert2 = vec![0u8; 64];
        cert2[0..8].copy_from_slice(&20u64.to_be_bytes());
        cert2[8..40].copy_from_slice(&[0xBB; 32]);
        detector.check_certificate(&validator, &cert2);

        let evidence = detector.get_evidence();
        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].offense_type, SlashingOffense::DoubleSign);

        // Clear evidence
        detector.clear_evidence();
        assert_eq!(detector.get_evidence().len(), 0);
    }

    #[test]
    fn test_signature_count() {
        let mut detector = SlashingDetector::new();
        let validator = PeerId::new([5u8; 32]);

        assert_eq!(detector.get_signature_count(&validator, 10), 0);

        let mut cert = vec![0u8; 64];
        cert[0..8].copy_from_slice(&10u64.to_be_bytes());
        cert[8..40].copy_from_slice(&[0xAA; 32]);
        detector.check_certificate(&validator, &cert);

        assert_eq!(detector.get_signature_count(&validator, 10), 1);
    }
}
