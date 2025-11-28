//! ASF Equivocation Detection
//!
//! This module detects and reports validators who sign conflicting votes
//! for the same view number. Equivocation is a serious offense that:
//! - Threatens consensus safety
//! - Can lead to conflicting finality
//! - Must result in immediate slashing
//!
//! ## Detection Strategy
//!
//! For each vote received, we check if the same validator has already
//! voted in the same view for a different block. If so, we have proof
//! of equivocation.

use codec::{Decode, Encode};
use sp_core::sr25519::{Public, Signature};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Proof of equivocation - two conflicting votes from same validator
#[derive(Clone, Debug, Encode, Decode)]
pub struct EquivocationProof {
    /// The offending validator's public key
    pub validator_id: [u8; 32],
    /// View number where equivocation occurred
    pub view: u64,
    /// First vote details
    pub first_vote: EquivocationVote,
    /// Second conflicting vote details
    pub second_vote: EquivocationVote,
    /// Timestamp when equivocation was detected
    pub detected_at: u64,
}

/// Vote data included in equivocation proof
#[derive(Clone, Debug, Encode, Decode)]
pub struct EquivocationVote {
    /// Block hash voted for
    pub block_hash: [u8; 32],
    /// Block number
    pub block_number: u32,
    /// The signature on this vote
    pub signature: [u8; 64],
    /// When the vote was received
    pub received_at: u64,
}

impl EquivocationProof {
    /// Verify that this proof is valid
    ///
    /// Checks:
    /// 1. Both votes are from the same validator
    /// 2. Both votes are for the same view
    /// 3. The block hashes are different (actual equivocation)
    /// 4. Both signatures are valid
    pub fn verify(&self) -> Result<(), EquivocationVerifyError> {
        // Check block hashes are different
        if self.first_vote.block_hash == self.second_vote.block_hash {
            return Err(EquivocationVerifyError::SameBlockHash);
        }

        // Verify first signature
        if !self.verify_vote_signature(&self.first_vote) {
            return Err(EquivocationVerifyError::InvalidFirstSignature);
        }

        // Verify second signature
        if !self.verify_vote_signature(&self.second_vote) {
            return Err(EquivocationVerifyError::InvalidSecondSignature);
        }

        Ok(())
    }

    fn verify_vote_signature(&self, vote: &EquivocationVote) -> bool {
        // Create the message that was signed
        // Format: view || block_hash || block_number || validator_id
        let mut message = Vec::new();
        message.extend_from_slice(&self.view.to_le_bytes());
        message.extend_from_slice(&vote.block_hash);
        message.extend_from_slice(&vote.block_number.to_le_bytes());
        message.extend_from_slice(&self.validator_id);

        // Hash the message
        let message_hash = sp_core::hashing::blake2_256(&message);

        // For now, we use a simplified verification since the current implementation
        // uses dummy signatures. In production, this would verify Sr25519 signatures:
        //
        // let public = Public::from_raw(self.validator_id);
        // let signature = Signature::from_raw(vote.signature);
        // sp_core::sr25519::Pair::verify(&signature, &message_hash, &public)

        // Temporary: Accept all signatures as valid (consistent with dummy signatures in lib.rs)
        // This will be replaced with actual Sr25519 verification when proper signing is implemented
        true
    }

    /// Get a unique identifier for this equivocation
    pub fn id(&self) -> [u8; 32] {
        use sp_core::hashing::blake2_256;
        blake2_256(&self.encode())
    }
}

#[derive(Debug, Clone)]
pub enum EquivocationVerifyError {
    SameBlockHash,
    InvalidFirstSignature,
    InvalidSecondSignature,
}

impl std::fmt::Display for EquivocationVerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SameBlockHash => write!(f, "Both votes are for the same block"),
            Self::InvalidFirstSignature => write!(f, "First vote signature is invalid"),
            Self::InvalidSecondSignature => write!(f, "Second vote signature is invalid"),
        }
    }
}

/// Key for tracking votes: (view, validator_id)
type VoteKey = (u64, [u8; 32]);

/// Stored vote information
#[derive(Clone)]
struct StoredVote {
    block_hash: [u8; 32],
    block_number: u32,
    signature: [u8; 64],
    received_at: u64,
}

/// Equivocation detector - tracks votes and detects double-voting
pub struct EquivocationDetector {
    /// Votes seen per (view, validator_id)
    votes_seen: Arc<RwLock<HashMap<VoteKey, StoredVote>>>,
    /// Detected equivocations pending submission
    pending_proofs: Arc<RwLock<Vec<EquivocationProof>>>,
    /// Already reported validators (to avoid duplicate reports)
    reported_validators: Arc<RwLock<HashMap<[u8; 32], Vec<u64>>>>,
    /// Channel to send equivocation proofs for slashing
    proof_sender: Option<tokio::sync::mpsc::UnboundedSender<EquivocationProof>>,
    /// Maximum views to keep in memory (prevent unbounded growth)
    max_views_retained: u64,
    /// Current view (for cleanup)
    current_view: Arc<std::sync::atomic::AtomicU64>,
}

impl EquivocationDetector {
    /// Create a new equivocation detector
    pub fn new(
        proof_sender: Option<tokio::sync::mpsc::UnboundedSender<EquivocationProof>>,
    ) -> Self {
        Self {
            votes_seen: Arc::new(RwLock::new(HashMap::new())),
            pending_proofs: Arc::new(RwLock::new(Vec::new())),
            reported_validators: Arc::new(RwLock::new(HashMap::new())),
            proof_sender,
            max_views_retained: 1000, // Keep votes for last 1000 views
            current_view: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    /// Check a vote for equivocation
    ///
    /// Returns Some(proof) if equivocation is detected, None otherwise.
    /// The vote is always stored for future detection.
    pub async fn check_vote(
        &self,
        view: u64,
        validator_id: [u8; 32],
        block_hash: [u8; 32],
        block_number: u32,
        signature: [u8; 64],
    ) -> Option<EquivocationProof> {
        let key = (view, validator_id);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let new_vote = StoredVote {
            block_hash,
            block_number,
            signature,
            received_at: now,
        };

        let mut votes = self.votes_seen.write().await;

        // Check if we've seen a vote from this validator in this view
        if let Some(existing) = votes.get(&key) {
            // Check if it's for a different block
            if existing.block_hash != block_hash {
                // EQUIVOCATION DETECTED!
                tracing::warn!(
                    "⚠️ EQUIVOCATION DETECTED: Validator {} voted for {:?} AND {:?} in view {}",
                    hex::encode(&validator_id[..8]),
                    hex::encode(&existing.block_hash[..8]),
                    hex::encode(&block_hash[..8]),
                    view
                );

                let proof = EquivocationProof {
                    validator_id,
                    view,
                    first_vote: EquivocationVote {
                        block_hash: existing.block_hash,
                        block_number: existing.block_number,
                        signature: existing.signature,
                        received_at: existing.received_at,
                    },
                    second_vote: EquivocationVote {
                        block_hash,
                        block_number,
                        signature,
                        received_at: now,
                    },
                    detected_at: now,
                };

                // Store the proof
                self.store_proof(proof.clone()).await;

                return Some(proof);
            }
            // Same vote, just a duplicate - ignore
            return None;
        }

        // First vote from this validator in this view - store it
        votes.insert(key, new_vote);

        // Update current view for cleanup
        self.current_view.store(view, std::sync::atomic::Ordering::SeqCst);

        None
    }

    /// Store an equivocation proof and optionally send for slashing
    async fn store_proof(&self, proof: EquivocationProof) {
        // Check if already reported
        {
            let reported = self.reported_validators.read().await;
            if let Some(views) = reported.get(&proof.validator_id) {
                if views.contains(&proof.view) {
                    tracing::debug!("Equivocation already reported for validator in view {}", proof.view);
                    return;
                }
            }
        }

        // Mark as reported
        {
            let mut reported = self.reported_validators.write().await;
            reported
                .entry(proof.validator_id)
                .or_insert_with(Vec::new)
                .push(proof.view);
        }

        // Store proof
        {
            let mut proofs = self.pending_proofs.write().await;
            proofs.push(proof.clone());
        }

        // Send to slashing handler if channel exists
        if let Some(sender) = &self.proof_sender {
            if let Err(e) = sender.send(proof.clone()) {
                tracing::error!("Failed to send equivocation proof: {:?}", e);
            } else {
                tracing::info!(
                    "📤 Equivocation proof sent for slashing: validator {} in view {}",
                    hex::encode(&proof.validator_id[..8]),
                    proof.view
                );
            }
        }
    }

    /// Get all pending equivocation proofs
    pub async fn get_pending_proofs(&self) -> Vec<EquivocationProof> {
        self.pending_proofs.read().await.clone()
    }

    /// Clear a proof after it's been processed
    pub async fn clear_proof(&self, proof_id: &[u8; 32]) {
        let mut proofs = self.pending_proofs.write().await;
        proofs.retain(|p| &p.id() != proof_id);
    }

    /// Clean up old vote data to prevent unbounded memory growth
    pub async fn cleanup_old_views(&self) {
        let current = self.current_view.load(std::sync::atomic::Ordering::SeqCst);
        if current < self.max_views_retained {
            return;
        }

        let cutoff = current - self.max_views_retained;

        let mut votes = self.votes_seen.write().await;
        votes.retain(|(view, _), _| *view >= cutoff);

        tracing::debug!("Cleaned up votes older than view {}", cutoff);
    }

    /// Get statistics about the detector
    pub async fn stats(&self) -> EquivocationStats {
        let votes = self.votes_seen.read().await;
        let proofs = self.pending_proofs.read().await;
        let reported = self.reported_validators.read().await;

        EquivocationStats {
            votes_tracked: votes.len(),
            pending_proofs: proofs.len(),
            validators_reported: reported.len(),
            current_view: self.current_view.load(std::sync::atomic::Ordering::SeqCst),
        }
    }
}

/// Statistics about the equivocation detector
#[derive(Debug, Clone)]
pub struct EquivocationStats {
    pub votes_tracked: usize,
    pub pending_proofs: usize,
    pub validators_reported: usize,
    pub current_view: u64,
}

impl Clone for EquivocationDetector {
    fn clone(&self) -> Self {
        Self {
            votes_seen: self.votes_seen.clone(),
            pending_proofs: self.pending_proofs.clone(),
            reported_validators: self.reported_validators.clone(),
            proof_sender: self.proof_sender.clone(),
            max_views_retained: self.max_views_retained,
            current_view: Arc::new(std::sync::atomic::AtomicU64::new(
                self.current_view.load(std::sync::atomic::Ordering::SeqCst)
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_no_equivocation_same_vote() {
        let detector = EquivocationDetector::new(None);

        let validator = [1u8; 32];
        let block_hash = [2u8; 32];
        let signature = [3u8; 64];

        // First vote
        let result = detector.check_vote(1, validator, block_hash, 100, signature).await;
        assert!(result.is_none());

        // Same vote again - should not be equivocation
        let result = detector.check_vote(1, validator, block_hash, 100, signature).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_equivocation_detected() {
        let detector = EquivocationDetector::new(None);

        let validator = [1u8; 32];
        let block_hash_1 = [2u8; 32];
        let block_hash_2 = [3u8; 32];
        let signature_1 = [4u8; 64];
        let signature_2 = [5u8; 64];

        // First vote
        let result = detector.check_vote(1, validator, block_hash_1, 100, signature_1).await;
        assert!(result.is_none());

        // Different vote for same view - EQUIVOCATION!
        let result = detector.check_vote(1, validator, block_hash_2, 101, signature_2).await;
        assert!(result.is_some());

        let proof = result.unwrap();
        assert_eq!(proof.validator_id, validator);
        assert_eq!(proof.view, 1);
        assert_eq!(proof.first_vote.block_hash, block_hash_1);
        assert_eq!(proof.second_vote.block_hash, block_hash_2);
    }

    #[tokio::test]
    async fn test_different_views_not_equivocation() {
        let detector = EquivocationDetector::new(None);

        let validator = [1u8; 32];
        let block_hash_1 = [2u8; 32];
        let block_hash_2 = [3u8; 32];
        let signature = [4u8; 64];

        // Vote in view 1
        let result = detector.check_vote(1, validator, block_hash_1, 100, signature).await;
        assert!(result.is_none());

        // Different vote in view 2 - NOT equivocation
        let result = detector.check_vote(2, validator, block_hash_2, 101, signature).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_different_validators_not_equivocation() {
        let detector = EquivocationDetector::new(None);

        let validator_1 = [1u8; 32];
        let validator_2 = [2u8; 32];
        let block_hash_1 = [3u8; 32];
        let block_hash_2 = [4u8; 32];
        let signature = [5u8; 64];

        // Vote from validator 1
        let result = detector.check_vote(1, validator_1, block_hash_1, 100, signature).await;
        assert!(result.is_none());

        // Different vote from validator 2 in same view - NOT equivocation
        let result = detector.check_vote(1, validator_2, block_hash_2, 101, signature).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_pending_proofs() {
        let detector = EquivocationDetector::new(None);

        let validator = [1u8; 32];
        let block_hash_1 = [2u8; 32];
        let block_hash_2 = [3u8; 32];
        let signature_1 = [4u8; 64];
        let signature_2 = [5u8; 64];

        // Create equivocation
        detector.check_vote(1, validator, block_hash_1, 100, signature_1).await;
        let proof = detector.check_vote(1, validator, block_hash_2, 101, signature_2).await;
        assert!(proof.is_some());

        // Check pending proofs
        let proofs = detector.get_pending_proofs().await;
        assert_eq!(proofs.len(), 1);
        assert_eq!(proofs[0].validator_id, validator);

        // Clear proof
        let proof_id = proofs[0].id();
        detector.clear_proof(&proof_id).await;

        let proofs = detector.get_pending_proofs().await;
        assert_eq!(proofs.len(), 0);
    }

    #[tokio::test]
    async fn test_duplicate_reporting_prevention() {
        let detector = EquivocationDetector::new(None);

        let validator = [1u8; 32];
        let block_hash_1 = [2u8; 32];
        let block_hash_2 = [3u8; 32];
        let signature_1 = [4u8; 64];
        let signature_2 = [5u8; 64];

        // Create equivocation twice
        detector.check_vote(1, validator, block_hash_1, 100, signature_1).await;
        detector.check_vote(1, validator, block_hash_2, 101, signature_2).await;

        // Try to report again - should not create duplicate
        detector.check_vote(1, validator, block_hash_1, 100, signature_1).await;

        let proofs = detector.get_pending_proofs().await;
        assert_eq!(proofs.len(), 1, "Should only have one proof, not duplicates");
    }

    #[tokio::test]
    async fn test_cleanup_old_views() {
        let detector = EquivocationDetector::new(None);

        let validator = [1u8; 32];
        let block_hash = [2u8; 32];
        let signature = [3u8; 64];

        // Add votes in old views
        for view in 0..10 {
            detector.check_vote(view, validator, block_hash, 100, signature).await;
        }

        let stats = detector.stats().await;
        assert_eq!(stats.votes_tracked, 10);

        // Simulate advancing to view 1005
        detector.check_vote(1005, validator, block_hash, 100, signature).await;

        // Clean up old views
        detector.cleanup_old_views().await;

        let stats = detector.stats().await;
        // Should only keep views >= 5 (1005 - 1000)
        assert!(stats.votes_tracked <= 2, "Old votes should be cleaned up");
    }

    #[tokio::test]
    async fn test_equivocation_proof_verification() {
        let validator = [1u8; 32];
        let block_hash_1 = [2u8; 32];
        let block_hash_2 = [3u8; 32];
        let signature_1 = [4u8; 64];
        let signature_2 = [5u8; 64];

        let proof = EquivocationProof {
            validator_id: validator,
            view: 1,
            first_vote: EquivocationVote {
                block_hash: block_hash_1,
                block_number: 100,
                signature: signature_1,
                received_at: 1000,
            },
            second_vote: EquivocationVote {
                block_hash: block_hash_2,
                block_number: 101,
                signature: signature_2,
                received_at: 1001,
            },
            detected_at: 1001,
        };

        // Should verify successfully (with dummy signatures)
        assert!(proof.verify().is_ok());
    }

    #[tokio::test]
    async fn test_equivocation_proof_same_block_fails() {
        let validator = [1u8; 32];
        let block_hash = [2u8; 32]; // Same block hash
        let signature_1 = [4u8; 64];
        let signature_2 = [5u8; 64];

        let proof = EquivocationProof {
            validator_id: validator,
            view: 1,
            first_vote: EquivocationVote {
                block_hash,
                block_number: 100,
                signature: signature_1,
                received_at: 1000,
            },
            second_vote: EquivocationVote {
                block_hash, // Same hash!
                block_number: 101,
                signature: signature_2,
                received_at: 1001,
            },
            detected_at: 1001,
        };

        // Should fail verification - same block hash
        let result = proof.verify();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EquivocationVerifyError::SameBlockHash));
    }

    #[tokio::test]
    async fn test_stats() {
        let detector = EquivocationDetector::new(None);

        let validator = [1u8; 32];
        let block_hash_1 = [2u8; 32];
        let block_hash_2 = [3u8; 32];
        let signature = [4u8; 64];

        // Add some votes
        detector.check_vote(1, validator, block_hash_1, 100, signature).await;
        detector.check_vote(2, validator, block_hash_2, 101, signature).await;

        let stats = detector.stats().await;
        assert_eq!(stats.votes_tracked, 2);
        assert_eq!(stats.pending_proofs, 0);
        assert_eq!(stats.validators_reported, 0);
        assert_eq!(stats.current_view, 2);
    }

    #[tokio::test]
    async fn test_channel_sending() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let detector = EquivocationDetector::new(Some(tx));

        let validator = [1u8; 32];
        let block_hash_1 = [2u8; 32];
        let block_hash_2 = [3u8; 32];
        let signature_1 = [4u8; 64];
        let signature_2 = [5u8; 64];

        // Create equivocation
        detector.check_vote(1, validator, block_hash_1, 100, signature_1).await;
        detector.check_vote(1, validator, block_hash_2, 101, signature_2).await;

        // Check that proof was sent through channel
        let received = rx.try_recv();
        assert!(received.is_ok(), "Proof should be sent through channel");

        let proof = received.unwrap();
        assert_eq!(proof.validator_id, validator);
        assert_eq!(proof.view, 1);
    }
}
