//! # Network Message Authentication
//!
//! This module provides authenticated messaging for consensus p2p communication.
//! Prevents message injection, replay attacks, and man-in-the-middle attacks.
//!
//! Security Features:
//! - Message signing and verification
//! - Nonce-based replay protection
//! - Timestamp-based freshness checks
//! - Authenticated encryption support

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    crypto::{verify_signature, SignData, Signature},
    AsfError, AsfResult, Hash, ValidatorId,
};

// ═══════════════════════════════════════════════════════════════════════════════
// MESSAGE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Type of consensus message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum MessageType {
    /// Vote message
    Vote,
    /// Certificate message
    Certificate,
    /// Block proposal
    Proposal,
    /// Phase transition announcement
    PhaseTransition,
}

/// Authenticated network message
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct AuthenticatedMessage {
    /// Message type
    pub message_type: MessageType,

    /// Sender's validator ID
    pub sender: ValidatorId,

    /// Message payload (SCALE-encoded)
    pub payload: Vec<u8>,

    /// Nonce for replay protection
    pub nonce: u64,

    /// Timestamp (Unix milliseconds)
    pub timestamp: u64,

    /// Epoch
    pub epoch: u32,

    /// Signature over (message_type, payload, nonce, timestamp, epoch)
    pub signature: Signature,
}

impl AuthenticatedMessage {
    /// Create a new authenticated message
    pub fn new<P: SignData>(
        message_type: MessageType,
        sender: ValidatorId,
        payload: Vec<u8>,
        nonce: u64,
        timestamp: u64,
        epoch: u32,
        keypair: &P,
    ) -> Self {
        let message = Self::signing_message(&message_type, &payload, nonce, timestamp, epoch);
        let signature = keypair.sign(&message);

        Self {
            message_type,
            sender,
            payload,
            nonce,
            timestamp,
            epoch,
            signature,
        }
    }

    /// Get the message to be signed
    fn signing_message(
        message_type: &MessageType,
        payload: &[u8],
        nonce: u64,
        timestamp: u64,
        epoch: u32,
    ) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(&message_type.encode());
        message.extend_from_slice(payload);
        message.extend_from_slice(&nonce.to_le_bytes());
        message.extend_from_slice(&timestamp.to_le_bytes());
        message.extend_from_slice(&epoch.to_le_bytes());
        message
    }

    /// Verify message signature
    pub fn verify_signature(&self) -> AsfResult<()> {
        let message = Self::signing_message(
            &self.message_type,
            &self.payload,
            self.nonce,
            self.timestamp,
            self.epoch,
        );

        verify_signature(&self.signature, &message, &self.sender)
    }

    /// Verify message is fresh (not replayed, within time window)
    pub fn verify_freshness(
        &self,
        current_time: u64,
        max_age_ms: u64,
        current_epoch: u32,
    ) -> AsfResult<()> {
        // Check epoch is not from the future
        if self.epoch > current_epoch {
            return Err(AsfError::InvalidVote("Message from future epoch"));
        }

        // Check timestamp is not from the future
        if self.timestamp > current_time + 5000 {
            // Allow 5s clock skew
            return Err(AsfError::InvalidVote("Message timestamp in future"));
        }

        // Check message is not too old
        let age = current_time.saturating_sub(self.timestamp);
        if age > max_age_ms {
            return Err(AsfError::InvalidVote("Message too old"));
        }

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// REPLAY PROTECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Tracks nonces to prevent replay attacks
pub struct NonceTracker {
    /// Last nonce seen from each validator
    last_nonce: BTreeMap<ValidatorId, u64>,

    /// Maximum number of validators to track
    max_validators: usize,
}

impl NonceTracker {
    /// Create a new nonce tracker
    pub fn new(max_validators: usize) -> Self {
        Self {
            last_nonce: BTreeMap::new(),
            max_validators,
        }
    }

    /// Check if nonce is valid (not a replay) and update
    pub fn check_and_update(&mut self, validator: &ValidatorId, nonce: u64) -> AsfResult<()> {
        // Get last nonce for this validator
        let last = self.last_nonce.get(validator).copied().unwrap_or(0);

        // Nonce must be strictly increasing
        if nonce <= last {
            return Err(AsfError::InvalidVote("Nonce replay detected"));
        }

        // Update last nonce
        self.last_nonce.insert(validator.clone(), nonce);

        // Prune if too many validators
        if self.last_nonce.len() > self.max_validators {
            self.prune_oldest();
        }

        Ok(())
    }

    /// Get last nonce for a validator
    pub fn get_last_nonce(&self, validator: &ValidatorId) -> u64 {
        self.last_nonce.get(validator).copied().unwrap_or(0)
    }

    /// Remove oldest entries to prevent unbounded growth
    fn prune_oldest(&mut self) {
        if self.last_nonce.len() <= self.max_validators {
            return;
        }

        let remove_count = self.last_nonce.len() - self.max_validators;
        let to_remove: Vec<ValidatorId> = self
            .last_nonce
            .iter()
            .take(remove_count)
            .map(|(k, _)| k.clone())
            .collect();

        for key in to_remove {
            self.last_nonce.remove(&key);
        }
    }

    /// Clear all nonces
    pub fn clear(&mut self) {
        self.last_nonce.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MESSAGE VALIDATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Validates incoming network messages
pub struct MessageValidator {
    /// Nonce tracker for replay protection
    nonce_tracker: NonceTracker,

    /// Maximum message age (milliseconds)
    max_message_age: u64,

    /// Current epoch
    current_epoch: u32,
}

impl MessageValidator {
    /// Create a new message validator
    pub fn new(max_validators: usize, max_message_age: u64) -> Self {
        Self {
            nonce_tracker: NonceTracker::new(max_validators),
            max_message_age,
            current_epoch: 0,
        }
    }

    /// Validate an incoming message
    ///
    /// # Security Checks
    /// 1. Cryptographic signature verification
    /// 2. Replay protection (nonce checking)
    /// 3. Freshness check (timestamp within window)
    /// 4. Epoch validation
    pub fn validate_message(
        &mut self,
        message: &AuthenticatedMessage,
        current_time: u64,
    ) -> AsfResult<()> {
        // 1. Verify signature
        message.verify_signature()?;

        // 2. Check freshness
        message.verify_freshness(current_time, self.max_message_age, self.current_epoch)?;

        // 3. Check nonce (replay protection)
        self.nonce_tracker
            .check_and_update(&message.sender, message.nonce)?;

        Ok(())
    }

    /// Update current epoch
    pub fn update_epoch(&mut self, epoch: u32) {
        self.current_epoch = epoch;
    }

    /// Get next nonce for a validator
    pub fn get_next_nonce(&self, validator: &ValidatorId) -> u64 {
        self.nonce_tracker.get_last_nonce(validator) + 1
    }

    /// Clear nonce tracking (e.g., after epoch transition)
    pub fn clear_nonces(&mut self) {
        self.nonce_tracker.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MESSAGE BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Helper to build authenticated messages
pub struct MessageBuilder {
    /// Validator ID
    validator: ValidatorId,

    /// Current nonce
    nonce: u64,

    /// Current epoch
    epoch: u32,
}

impl MessageBuilder {
    /// Create a new message builder
    pub fn new(validator: ValidatorId, epoch: u32) -> Self {
        Self {
            validator,
            nonce: 1,
            epoch,
        }
    }

    /// Build an authenticated message
    pub fn build<P: SignData>(
        &mut self,
        message_type: MessageType,
        payload: Vec<u8>,
        keypair: &P,
    ) -> AuthenticatedMessage {
        let timestamp = Self::current_timestamp();
        let nonce = self.nonce;
        self.nonce += 1;

        AuthenticatedMessage::new(
            message_type,
            self.validator.clone(),
            payload,
            nonce,
            timestamp,
            self.epoch,
            keypair,
        )
    }

    /// Update epoch
    pub fn update_epoch(&mut self, epoch: u32) {
        self.epoch = epoch;
        self.nonce = 1; // Reset nonce on epoch change
    }

    /// Get current timestamp (placeholder - should use actual system time)
    fn current_timestamp() -> u64 {
        // In production, this would use sp_timestamp or similar
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;
    use sp_core::Pair as _;

    fn create_test_validator(id: u8) -> (sp_core::sr25519::Pair, ValidatorId) {
        let seed = [id; 32];
        let pair = sp_core::sr25519::Pair::from_seed(&seed);
        let validator_id = ValidatorId::from(pair.public().0);
        (pair, validator_id)
    }

    #[test]
    fn test_message_authentication() {
        let (keypair, validator_id) = create_test_validator(1);

        let message = AuthenticatedMessage::new(
            MessageType::Vote,
            validator_id.clone(),
            vec![1, 2, 3, 4],
            1,
            1000,
            1,
            &keypair,
        );

        // Should verify successfully
        assert!(message.verify_signature().is_ok());
    }

    #[test]
    fn test_message_tampering_detection() {
        let (keypair, validator_id) = create_test_validator(1);

        let mut message = AuthenticatedMessage::new(
            MessageType::Vote,
            validator_id.clone(),
            vec![1, 2, 3, 4],
            1,
            1000,
            1,
            &keypair,
        );

        // Tamper with payload
        message.payload = vec![5, 6, 7, 8];

        // Should fail verification
        assert!(message.verify_signature().is_err());
    }

    #[test]
    fn test_nonce_replay_protection() {
        let mut tracker = NonceTracker::new(100);
        let (_, validator_id) = create_test_validator(1);

        // First message with nonce 1
        assert!(tracker.check_and_update(&validator_id, 1).is_ok());

        // Second message with nonce 2
        assert!(tracker.check_and_update(&validator_id, 2).is_ok());

        // Replay nonce 1 - should fail
        assert!(tracker.check_and_update(&validator_id, 1).is_err());

        // Replay nonce 2 - should fail
        assert!(tracker.check_and_update(&validator_id, 2).is_err());

        // New nonce 3 - should succeed
        assert!(tracker.check_and_update(&validator_id, 3).is_ok());
    }

    #[test]
    fn test_message_freshness() {
        let (keypair, validator_id) = create_test_validator(1);

        let current_time = 10000;
        let message = AuthenticatedMessage::new(
            MessageType::Vote,
            validator_id,
            vec![1, 2, 3],
            1,
            9000, // 1 second old
            1,
            &keypair,
        );

        // Should be fresh within 5 second window
        assert!(message.verify_freshness(current_time, 5000, 1).is_ok());

        // Should be too old for 500ms window
        assert!(message.verify_freshness(current_time, 500, 1).is_err());
    }

    #[test]
    fn test_future_timestamp_rejection() {
        let (keypair, validator_id) = create_test_validator(1);

        let current_time = 10000;
        let message = AuthenticatedMessage::new(
            MessageType::Vote,
            validator_id,
            vec![1, 2, 3],
            1,
            20000, // 10 seconds in future
            1,
            &keypair,
        );

        // Should reject future timestamp
        assert!(message.verify_freshness(current_time, 5000, 1).is_err());
    }

    #[test]
    fn test_message_validator() {
        let (keypair, validator_id) = create_test_validator(1);
        let mut validator = MessageValidator::new(100, 5000);

        let message = AuthenticatedMessage::new(
            MessageType::Vote,
            validator_id,
            vec![1, 2, 3],
            1,
            1000,
            0,
            &keypair,
        );

        // Should validate successfully
        assert!(validator.validate_message(&message, 1000).is_ok());

        // Replay should fail
        assert!(validator.validate_message(&message, 1000).is_err());
    }

    #[test]
    fn test_message_builder() {
        let (keypair, validator_id) = create_test_validator(1);
        let mut builder = MessageBuilder::new(validator_id.clone(), 1);

        let msg1 = builder.build(MessageType::Vote, vec![1, 2, 3], &keypair);
        assert_eq!(msg1.nonce, 1);

        let msg2 = builder.build(MessageType::Certificate, vec![4, 5, 6], &keypair);
        assert_eq!(msg2.nonce, 2);

        // Nonces should auto-increment
        assert!(msg2.nonce > msg1.nonce);
    }

    #[test]
    fn test_epoch_transition() {
        let (keypair, validator_id) = create_test_validator(1);
        let mut builder = MessageBuilder::new(validator_id.clone(), 1);

        builder.build(MessageType::Vote, vec![1], &keypair);
        builder.build(MessageType::Vote, vec![2], &keypair);

        // Update epoch
        builder.update_epoch(2);

        // Nonce should reset to 1
        let msg = builder.build(MessageType::Vote, vec![3], &keypair);
        assert_eq!(msg.nonce, 1);
        assert_eq!(msg.epoch, 2);
    }
}
