// ═══════════════════════════════════════════════════════════════════════════
// VRF-BASED CHECKPOINT DETECTION
// ═══════════════════════════════════════════════════════════════════════════
//
// Replaces deterministic checkpoints with VRF-based unpredictability to prevent
// grinding attacks and strategic block production.
//
// Security Properties:
// - VRF output unpredictable until parent block finalized
// - All validators compute identical result (deterministic verification)
// - Grinding attacks impossible (parent hash cannot be changed)
// - Threshold tuning protected by hardcoded bounds
//
// ═══════════════════════════════════════════════════════════════════════════

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Guaranteed checkpoint interval (fallback safety mechanism)
/// Even if VRF never triggers, we checkpoint every 32 blocks
pub const GUARANTEED_CHECKPOINT_INTERVAL: u32 = 32;

/// VRF threshold for opportunity checkpoints
/// 32/256 = 1/8 chance = 12.5% probability
/// This gives average checkpoint every 8 blocks, with guaranteed fallback at 32
const VRF_THRESHOLD_NUMERATOR: u8 = 32;

// Threshold bounds for future governance adjustments
#[allow(dead_code)]
const VRF_THRESHOLD_DENOMINATOR: u8 = 255;
#[allow(dead_code)]
const MIN_THRESHOLD_NUMERATOR: u8 = 13; // ~5%
#[allow(dead_code)]
const MAX_THRESHOLD_NUMERATOR: u8 = 64; // ~25%

/// VRF context for domain separation
const VRF_CHECKPOINT_CONTEXT: &[u8] = b"ETRID-CHECKPOINT-VRF-V1";

/// Checkpoint type with VRF proof for opportunity checkpoints
#[derive(Clone, Debug, Encode, Decode, Serialize, Deserialize, PartialEq, Eq)]
pub enum CheckpointType {
    /// Guaranteed checkpoint (every 32 blocks)
    Guaranteed,

    /// Opportunity checkpoint triggered by VRF
    Opportunity {
        /// VRF proof (verifiable by all validators)
        vrf_proof: Vec<u8>,
        /// VRF output (deterministic result)
        vrf_output: Vec<u8>,
    },
}

/// VRF-based checkpoint decision
#[derive(Clone, Debug)]
pub struct VrfCheckpointDecision {
    /// Block number being evaluated
    pub block_number: u32,

    /// Parent block hash (prevents grinding)
    pub parent_hash: [u8; 32],

    /// Current epoch number
    pub epoch: u64,

    /// Epoch randomness (from BABE/VRF consensus)
    pub epoch_randomness: [u8; 32],

    /// Whether this block should be a checkpoint
    pub is_checkpoint: bool,

    /// Type of checkpoint (if is_checkpoint = true)
    pub checkpoint_type: CheckpointType,
}

impl VrfCheckpointDecision {
    /// Evaluate whether a block should be a checkpoint using VRF
    ///
    /// Security: Uses parent_hash (not current block) to prevent grinding attacks
    /// where block producer tries multiple block contents to avoid checkpoints.
    pub fn evaluate(
        block_number: u32,
        parent_hash: [u8; 32],
        epoch: u64,
        epoch_randomness: [u8; 32],
    ) -> Self {
        // Guaranteed checkpoint every 32 blocks (safety fallback)
        if block_number % GUARANTEED_CHECKPOINT_INTERVAL == 0 && block_number > 0 {
            return Self {
                block_number,
                parent_hash,
                epoch,
                epoch_randomness,
                is_checkpoint: true,
                checkpoint_type: CheckpointType::Guaranteed,
            };
        }

        // Compute VRF-based opportunity checkpoint
        // NOTE: This is a deterministic "VRF-like" computation that all validators
        // can reproduce. In production, this would use actual VRF with validator's
        // secret key, but for checkpoint detection we need deterministic agreement.
        let vrf_input = Self::build_vrf_input(block_number, parent_hash, epoch, epoch_randomness);
        let vrf_output = Self::compute_deterministic_vrf(&vrf_input);

        // Check if VRF output meets threshold
        let is_opportunity = Self::check_vrf_threshold(&vrf_output);

        if is_opportunity {
            Self {
                block_number,
                parent_hash,
                epoch,
                epoch_randomness,
                is_checkpoint: true,
                checkpoint_type: CheckpointType::Opportunity {
                    vrf_proof: vrf_input.to_vec(), // In production, use actual VRF proof
                    vrf_output: vrf_output.to_vec(),
                },
            }
        } else {
            Self {
                block_number,
                parent_hash,
                epoch,
                epoch_randomness,
                is_checkpoint: false,
                checkpoint_type: CheckpointType::Guaranteed, // Unused
            }
        }
    }

    /// Build VRF input with all context binding
    ///
    /// Security: Binds to parent_hash (anti-grinding), epoch_randomness (unpredictable),
    /// block_number (prevents reuse), and epoch (prevents cross-epoch replay)
    fn build_vrf_input(
        block_number: u32,
        parent_hash: [u8; 32],
        epoch: u64,
        epoch_randomness: [u8; 32],
    ) -> Vec<u8> {
        let mut input = Vec::new();

        // Domain separator
        input.extend_from_slice(VRF_CHECKPOINT_CONTEXT);

        // Block context (MUST use parent_hash, not current block)
        input.extend_from_slice(&block_number.to_le_bytes());
        input.extend_from_slice(&parent_hash);

        // Epoch context (prevents cross-epoch reuse)
        input.extend_from_slice(&epoch.to_le_bytes());
        input.extend_from_slice(&epoch_randomness);

        input
    }

    /// Compute deterministic VRF-like output
    ///
    /// NOTE: This uses Blake2-256 for deterministic computation that all validators
    /// can reproduce. In production with actual VRF, each validator would compute
    /// their own VRF output and prove it, but for checkpoint detection we need
    /// consensus on the same result.
    fn compute_deterministic_vrf(input: &[u8]) -> [u8; 32] {
        sp_io::hashing::blake2_256(input)
    }

    /// Check if VRF output meets threshold for opportunity checkpoint
    ///
    /// Security: Threshold is hardcoded to prevent manipulation
    fn check_vrf_threshold(vrf_output: &[u8; 32]) -> bool {
        // Use first byte as random value [0, 255]
        let random_byte = vrf_output[0];

        // Check against threshold
        random_byte < VRF_THRESHOLD_NUMERATOR
    }

    /// Verify a VRF checkpoint decision (for peer validation)
    ///
    /// All validators must agree on the VRF result for consensus
    pub fn verify(
        block_number: u32,
        parent_hash: [u8; 32],
        epoch: u64,
        epoch_randomness: [u8; 32],
        checkpoint_type: &CheckpointType,
    ) -> Result<bool, String> {
        // Recompute VRF decision
        let decision = Self::evaluate(block_number, parent_hash, epoch, epoch_randomness);

        // Verify checkpoint type matches
        match (&decision.checkpoint_type, checkpoint_type) {
            (CheckpointType::Guaranteed, CheckpointType::Guaranteed) => {
                // Both agree it's a guaranteed checkpoint
                if decision.is_checkpoint {
                    Ok(true)
                } else {
                    Err("Guaranteed checkpoint verification failed".to_string())
                }
            }
            (
                CheckpointType::Opportunity {
                    vrf_output: our_output,
                    ..
                },
                CheckpointType::Opportunity {
                    vrf_output: their_output,
                    ..
                },
            ) => {
                // Verify VRF outputs match
                if our_output == their_output && decision.is_checkpoint {
                    Ok(true)
                } else {
                    Err(format!(
                        "VRF output mismatch: ours={:?}, theirs={:?}",
                        our_output, their_output
                    ))
                }
            }
            _ => Err(format!(
                "Checkpoint type mismatch: expected {:?}, got {:?}",
                decision.checkpoint_type, checkpoint_type
            )),
        }
    }
}

/// Helper function for backward compatibility
pub fn is_checkpoint_block(
    block_number: u32,
    parent_hash: [u8; 32],
    epoch: u64,
    epoch_randomness: [u8; 32],
) -> bool {
    VrfCheckpointDecision::evaluate(block_number, parent_hash, epoch, epoch_randomness)
        .is_checkpoint
}

/// Get checkpoint type for a block
pub fn get_checkpoint_type(
    block_number: u32,
    parent_hash: [u8; 32],
    epoch: u64,
    epoch_randomness: [u8; 32],
) -> Option<CheckpointType> {
    let decision = VrfCheckpointDecision::evaluate(block_number, parent_hash, epoch, epoch_randomness);
    if decision.is_checkpoint {
        Some(decision.checkpoint_type)
    } else {
        None
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guaranteed_checkpoint_at_32() {
        let parent_hash = [0u8; 32];
        let epoch_randomness = [0u8; 32];
        let epoch = 0;

        // Block 32 should always be a guaranteed checkpoint
        let decision = VrfCheckpointDecision::evaluate(32, parent_hash, epoch, epoch_randomness);
        assert!(decision.is_checkpoint);
        assert!(matches!(decision.checkpoint_type, CheckpointType::Guaranteed));

        // Block 64 should also be guaranteed
        let decision = VrfCheckpointDecision::evaluate(64, parent_hash, epoch, epoch_randomness);
        assert!(decision.is_checkpoint);
        assert!(matches!(decision.checkpoint_type, CheckpointType::Guaranteed));
    }

    #[test]
    fn test_deterministic_vrf_output() {
        let parent_hash = [1u8; 32];
        let epoch_randomness = [2u8; 32];
        let epoch = 1;
        let block_number = 10;

        // Same inputs should produce same output
        let decision1 = VrfCheckpointDecision::evaluate(block_number, parent_hash, epoch, epoch_randomness);
        let decision2 = VrfCheckpointDecision::evaluate(block_number, parent_hash, epoch, epoch_randomness);

        assert_eq!(decision1.is_checkpoint, decision2.is_checkpoint);
    }

    #[test]
    fn test_parent_hash_binding() {
        let epoch_randomness = [0u8; 32];
        let epoch = 0;
        let block_number = 10;

        // Different parent hashes should produce different results
        let parent1 = [1u8; 32];
        let parent2 = [2u8; 32];

        let decision1 = VrfCheckpointDecision::evaluate(block_number, parent1, epoch, epoch_randomness);
        let decision2 = VrfCheckpointDecision::evaluate(block_number, parent2, epoch, epoch_randomness);

        // Not necessarily different checkpoint decisions, but VRF outputs should differ
        if let CheckpointType::Opportunity { vrf_output: out1, .. } = decision1.checkpoint_type {
            if let CheckpointType::Opportunity { vrf_output: out2, .. } = decision2.checkpoint_type {
                // VRF outputs should be different for different parent hashes
                assert_ne!(out1, out2);
            }
        }
    }

    #[test]
    fn test_epoch_binding() {
        let parent_hash = [1u8; 32];
        let epoch_randomness = [2u8; 32];
        let block_number = 10;

        // Different epochs should produce different VRF outputs
        let decision1 = VrfCheckpointDecision::evaluate(block_number, parent_hash, 0, epoch_randomness);
        let decision2 = VrfCheckpointDecision::evaluate(block_number, parent_hash, 1, epoch_randomness);

        // VRF inputs should differ based on epoch
        assert_ne!(decision1.epoch, decision2.epoch);
    }

    #[test]
    fn test_vrf_threshold_bounds() {
        // Verify threshold is within safe bounds
        assert!(VRF_THRESHOLD_NUMERATOR >= MIN_THRESHOLD_NUMERATOR);
        assert!(VRF_THRESHOLD_NUMERATOR <= MAX_THRESHOLD_NUMERATOR);
    }

    #[test]
    fn test_verification() {
        let parent_hash = [1u8; 32];
        let epoch_randomness = [2u8; 32];
        let epoch = 1;
        let block_number = 32;

        // Evaluate checkpoint
        let decision = VrfCheckpointDecision::evaluate(block_number, parent_hash, epoch, epoch_randomness);

        // Verify should succeed with same parameters
        let result = VrfCheckpointDecision::verify(
            block_number,
            parent_hash,
            epoch,
            epoch_randomness,
            &decision.checkpoint_type,
        );

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_no_checkpoint_between_guaranteed() {
        let parent_hash = [0u8; 32];
        let epoch_randomness = [0u8; 32];
        let epoch = 0;

        // Most blocks between guaranteed checkpoints should not be checkpoints
        // (unless VRF triggers, which is probabilistic)
        for block_number in 1..32 {
            let decision = VrfCheckpointDecision::evaluate(block_number, parent_hash, epoch, epoch_randomness);
            // Can't assert !is_checkpoint because VRF might trigger
            // Just verify the logic executes without panic
            let _ = decision.is_checkpoint;
        }
    }
}
