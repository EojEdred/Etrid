//! Fraud Proof System for Lightning-Bloc Layer 2
//!
//! Provides challenge/response protocol for state transition validation and dispute resolution
//! with optimistic assumptions and fraud proof submission mechanisms.

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    string::{String, ToString},
    vec,
    vec::Vec,
    format,
};

#[cfg(not(feature = "std"))]
use core::{
    fmt,
    default::Default,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
};

#[cfg(feature = "std")]
use std::{
    collections::HashMap,
    fmt,
    vec::Vec,
    string::String,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
    default::Default,
};

/// Challenge period duration in seconds (e.g., 1 week)
pub const CHALLENGE_PERIOD: u64 = 7 * 24 * 60 * 60;

/// Response period duration in seconds (e.g., 24 hours)
pub const RESPONSE_PERIOD: u64 = 24 * 60 * 60;

/// Maximum fraud proof size in bytes
pub const MAX_FRAUD_PROOF_SIZE: usize = 10_240; // 10KB

/// State transition proof
#[derive(Clone, Debug, PartialEq)]
pub struct StateTransition {
    pub channel_id: String,
    pub from_nonce: u64,
    pub to_nonce: u64,
    pub from_balance_a: u128,
    pub from_balance_b: u128,
    pub to_balance_a: u128,
    pub to_balance_b: u128,
    pub transition_type: TransitionType,
    pub timestamp: u64,
}

impl StateTransition {
    /// Verify state transition validity
    pub fn verify(&self) -> Result<(), FraudProofError> {
        // Check nonce increment
        if self.to_nonce != self.from_nonce + 1 {
            return Err(FraudProofError::InvalidNonceTransition {
                expected: self.from_nonce + 1,
                got: self.to_nonce,
            });
        }

        // Verify balance conservation
        let from_total = self.from_balance_a.checked_add(self.from_balance_b)
            .ok_or(FraudProofError::BalanceOverflow)?;
        let to_total = self.to_balance_a.checked_add(self.to_balance_b)
            .ok_or(FraudProofError::BalanceOverflow)?;

        if from_total != to_total {
            return Err(FraudProofError::BalanceConservationViolated {
                before: from_total,
                after: to_total,
            });
        }

        Ok(())
    }

    /// Calculate state hash for verification
    pub fn state_hash(&self) -> Vec<u8> {
        // In production, use proper cryptographic hash
        let data = format!(
            "{}:{}:{}:{}:{}",
            self.channel_id, self.to_nonce, self.to_balance_a, self.to_balance_b, self.timestamp
        );
        data.as_bytes().to_vec()
    }
}

/// Type of state transition
#[derive(Clone, Debug, PartialEq)]
pub enum TransitionType {
    Payment { from_a_to_b: bool, amount: u128 },
    Deposit { party: String, amount: u128 },
    Withdrawal { party: String, amount: u128 },
    ForceClose,
}

impl fmt::Display for TransitionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransitionType::Payment { from_a_to_b, amount } => {
                write!(f, "Payment: {} -> {} amount {}",
                    if *from_a_to_b { "A" } else { "B" },
                    if *from_a_to_b { "B" } else { "A" },
                    amount)
            }
            TransitionType::Deposit { party, amount } => {
                write!(f, "Deposit: {} deposits {}", party, amount)
            }
            TransitionType::Withdrawal { party, amount } => {
                write!(f, "Withdrawal: {} withdraws {}", party, amount)
            }
            TransitionType::ForceClose => write!(f, "Force Close"),
        }
    }
}

/// Fraud proof containing evidence of invalid state transition
#[derive(Clone, Debug, PartialEq)]
pub struct FraudProof {
    pub proof_id: String,
    pub channel_id: String,
    pub challenger: String,
    pub invalid_transition: StateTransition,
    pub proof_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub submitted_at: u64,
    pub challenge_deadline: u64,
}

impl FraudProof {
    /// Create new fraud proof
    pub fn new(
        channel_id: String,
        challenger: String,
        invalid_transition: StateTransition,
        proof_data: Vec<u8>,
        signature: Vec<u8>,
        submitted_at: u64,
    ) -> Result<Self, FraudProofError> {
        // Validate proof size
        if proof_data.len() > MAX_FRAUD_PROOF_SIZE {
            return Err(FraudProofError::ProofTooLarge {
                size: proof_data.len(),
                max: MAX_FRAUD_PROOF_SIZE,
            });
        }

        if proof_data.is_empty() {
            return Err(FraudProofError::EmptyProof);
        }

        let proof_id = format!("fraud_{}_{}_{}", channel_id, challenger, submitted_at);
        let challenge_deadline = submitted_at + CHALLENGE_PERIOD;

        Ok(Self {
            proof_id,
            channel_id,
            challenger,
            invalid_transition,
            proof_data,
            signature,
            submitted_at,
            challenge_deadline,
        })
    }

    /// Check if challenge period has expired
    pub fn is_challenge_expired(&self, current_time: u64) -> bool {
        current_time > self.challenge_deadline
    }

    /// Verify fraud proof validity
    pub fn verify(&self) -> Result<(), FraudProofError> {
        // Verify the invalid transition
        self.invalid_transition.verify()?;

        // Check proof data integrity
        if self.proof_data.is_empty() {
            return Err(FraudProofError::EmptyProof);
        }

        // In production: verify cryptographic signature
        if self.signature.is_empty() {
            return Err(FraudProofError::InvalidSignature);
        }

        Ok(())
    }
}

/// Challenge response to fraud proof
#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeResponse {
    pub response_id: String,
    pub fraud_proof_id: String,
    pub responder: String,
    pub counter_proof: Vec<u8>,
    pub valid_transition: StateTransition,
    pub signature: Vec<u8>,
    pub submitted_at: u64,
}

impl ChallengeResponse {
    /// Create new challenge response
    pub fn new(
        fraud_proof_id: String,
        responder: String,
        counter_proof: Vec<u8>,
        valid_transition: StateTransition,
        signature: Vec<u8>,
        submitted_at: u64,
    ) -> Result<Self, FraudProofError> {
        if counter_proof.is_empty() {
            return Err(FraudProofError::EmptyProof);
        }

        if counter_proof.len() > MAX_FRAUD_PROOF_SIZE {
            return Err(FraudProofError::ProofTooLarge {
                size: counter_proof.len(),
                max: MAX_FRAUD_PROOF_SIZE,
            });
        }

        let response_id = format!("response_{}_{}_{}", fraud_proof_id, responder, submitted_at);

        Ok(Self {
            response_id,
            fraud_proof_id,
            responder,
            counter_proof,
            valid_transition,
            signature,
            submitted_at,
        })
    }

    /// Verify challenge response validity
    pub fn verify(&self) -> Result<(), FraudProofError> {
        // Verify the valid transition claim
        self.valid_transition.verify()?;

        // Check signature
        if self.signature.is_empty() {
            return Err(FraudProofError::InvalidSignature);
        }

        Ok(())
    }
}

/// Dispute resolution outcome
#[derive(Clone, Debug, PartialEq)]
pub enum DisputeResolution {
    Pending,
    FraudConfirmed {
        malicious_party: String,
        slashed_amount: u128,
        resolved_at: u64,
    },
    FraudRejected {
        reason: String,
        resolved_at: u64,
    },
    ResponseAccepted {
        resolved_at: u64,
    },
}

impl fmt::Display for DisputeResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisputeResolution::Pending => write!(f, "Pending"),
            DisputeResolution::FraudConfirmed { malicious_party, slashed_amount, .. } => {
                write!(f, "Fraud Confirmed: {} slashed {}", malicious_party, slashed_amount)
            }
            DisputeResolution::FraudRejected { reason, .. } => {
                write!(f, "Fraud Rejected: {}", reason)
            }
            DisputeResolution::ResponseAccepted { .. } => {
                write!(f, "Response Accepted")
            }
        }
    }
}

/// Dispute record tracking the full challenge/response lifecycle
#[derive(Clone, Debug, PartialEq)]
pub struct Dispute {
    pub dispute_id: String,
    pub fraud_proof: FraudProof,
    pub response: Option<ChallengeResponse>,
    pub resolution: DisputeResolution,
    pub resolved_at: Option<u64>,
}

impl Dispute {
    /// Create new dispute from fraud proof
    pub fn new(fraud_proof: FraudProof) -> Self {
        let dispute_id = format!("dispute_{}", fraud_proof.proof_id);
        Self {
            dispute_id,
            fraud_proof,
            response: None,
            resolution: DisputeResolution::Pending,
            resolved_at: None,
        }
    }

    /// Add challenge response
    pub fn add_response(&mut self, response: ChallengeResponse) -> Result<(), FraudProofError> {
        if self.response.is_some() {
            return Err(FraudProofError::ResponseAlreadySubmitted);
        }
        self.response = Some(response);
        Ok(())
    }

    /// Resolve dispute with outcome
    pub fn resolve(&mut self, resolution: DisputeResolution, timestamp: u64) -> Result<(), FraudProofError> {
        if !matches!(self.resolution, DisputeResolution::Pending) {
            return Err(FraudProofError::DisputeAlreadyResolved);
        }
        self.resolution = resolution;
        self.resolved_at = Some(timestamp);
        Ok(())
    }

    /// Check if dispute can be automatically resolved due to timeout
    pub fn can_auto_resolve(&self, current_time: u64) -> bool {
        matches!(self.resolution, DisputeResolution::Pending)
            && self.fraud_proof.is_challenge_expired(current_time)
            && self.response.is_none()
    }
}

/// Fraud proof system manager
pub struct FraudProofSystem {
    disputes: HashMap<String, Dispute>,
    channel_disputes: HashMap<String, Vec<String>>, // channel_id -> dispute_ids
}

impl FraudProofSystem {
    /// Create new fraud proof system
    pub fn new() -> Self {
        Self {
            disputes: HashMap::new(),
            channel_disputes: HashMap::new(),
        }
    }

    /// Submit fraud proof and initiate challenge
    pub fn submit_fraud_proof(
        &mut self,
        fraud_proof: FraudProof,
    ) -> Result<String, FraudProofError> {
        // Verify fraud proof
        fraud_proof.verify()?;

        let dispute_id = format!("dispute_{}", fraud_proof.proof_id);
        let channel_id = fraud_proof.channel_id.clone();

        // Check for duplicate
        if self.disputes.contains_key(&dispute_id) {
            return Err(FraudProofError::DuplicateProof(dispute_id));
        }

        // Create dispute
        let dispute = Dispute::new(fraud_proof);
        self.disputes.insert(dispute_id.clone(), dispute);

        // Track by channel
        self.channel_disputes
            .entry(channel_id)
            .or_insert_with(Vec::new)
            .push(dispute_id.clone());

        Ok(dispute_id)
    }

    /// Submit challenge response
    pub fn submit_response(
        &mut self,
        response: ChallengeResponse,
        current_time: u64,
    ) -> Result<(), FraudProofError> {
        // Verify response
        response.verify()?;

        // Get dispute by fraud proof ID
        let dispute_id = format!("dispute_{}", response.fraud_proof_id);
        let dispute = self.disputes.get_mut(&dispute_id)
            .ok_or_else(|| FraudProofError::DisputeNotFound(dispute_id.clone()))?;

        // Check if response period has expired
        let response_deadline = dispute.fraud_proof.submitted_at + RESPONSE_PERIOD;
        if current_time > response_deadline {
            return Err(FraudProofError::ResponsePeriodExpired {
                deadline: response_deadline,
                current: current_time,
            });
        }

        // Add response to dispute
        dispute.add_response(response)?;

        Ok(())
    }

    /// Resolve dispute manually (by arbitrator or automated)
    pub fn resolve_dispute(
        &mut self,
        dispute_id: &str,
        resolution: DisputeResolution,
        timestamp: u64,
    ) -> Result<(), FraudProofError> {
        let dispute = self.disputes.get_mut(dispute_id)
            .ok_or_else(|| FraudProofError::DisputeNotFound(dispute_id.to_string()))?;

        dispute.resolve(resolution, timestamp)
    }

    /// Auto-resolve disputes where challenge period expired without response
    pub fn auto_resolve_expired_disputes(&mut self, current_time: u64) -> Vec<String> {
        let mut resolved_ids = Vec::new();

        for (dispute_id, dispute) in self.disputes.iter_mut() {
            if dispute.can_auto_resolve(current_time) {
                let resolution = DisputeResolution::FraudConfirmed {
                    malicious_party: "unknown".to_string(),
                    slashed_amount: 0,
                    resolved_at: current_time,
                };
                let _ = dispute.resolve(resolution, current_time);
                resolved_ids.push(dispute_id.clone());
            }
        }

        resolved_ids
    }

    /// Get dispute by ID
    pub fn get_dispute(&self, dispute_id: &str) -> Option<&Dispute> {
        self.disputes.get(dispute_id)
    }

    /// Get all disputes for a channel
    pub fn get_channel_disputes(&self, channel_id: &str) -> Vec<&Dispute> {
        self.channel_disputes
            .get(channel_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.disputes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get pending disputes count
    pub fn pending_disputes_count(&self) -> usize {
        self.disputes
            .values()
            .filter(|d| matches!(d.resolution, DisputeResolution::Pending))
            .count()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> FraudProofStatistics {
        let total_disputes = self.disputes.len();
        let pending = self.pending_disputes_count();
        let resolved = total_disputes - pending;

        let confirmed = self.disputes.values()
            .filter(|d| matches!(d.resolution, DisputeResolution::FraudConfirmed { .. }))
            .count();

        let rejected = self.disputes.values()
            .filter(|d| matches!(d.resolution, DisputeResolution::FraudRejected { .. }))
            .count();

        FraudProofStatistics {
            total_disputes,
            pending_disputes: pending,
            resolved_disputes: resolved,
            fraud_confirmed: confirmed,
            fraud_rejected: rejected,
        }
    }
}

impl Default for FraudProofSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Fraud proof statistics
#[derive(Clone, Debug, PartialEq)]
pub struct FraudProofStatistics {
    pub total_disputes: usize,
    pub pending_disputes: usize,
    pub resolved_disputes: usize,
    pub fraud_confirmed: usize,
    pub fraud_rejected: usize,
}

/// Fraud proof errors
#[derive(Clone, Debug, PartialEq)]
pub enum FraudProofError {
    InvalidNonceTransition { expected: u64, got: u64 },
    BalanceOverflow,
    BalanceConservationViolated { before: u128, after: u128 },
    ProofTooLarge { size: usize, max: usize },
    EmptyProof,
    InvalidSignature,
    DuplicateProof(String),
    DisputeNotFound(String),
    DisputeAlreadyResolved,
    ResponseAlreadySubmitted,
    ResponsePeriodExpired { deadline: u64, current: u64 },
    InvalidStateTransition(String),
}

impl fmt::Display for FraudProofError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FraudProofError::InvalidNonceTransition { expected, got } => {
                write!(f, "Invalid nonce transition: expected {}, got {}", expected, got)
            }
            FraudProofError::BalanceOverflow => write!(f, "Balance overflow"),
            FraudProofError::BalanceConservationViolated { before, after } => {
                write!(f, "Balance conservation violated: {} -> {}", before, after)
            }
            FraudProofError::ProofTooLarge { size, max } => {
                write!(f, "Proof too large: {} > {} bytes", size, max)
            }
            FraudProofError::EmptyProof => write!(f, "Empty proof data"),
            FraudProofError::InvalidSignature => write!(f, "Invalid signature"),
            FraudProofError::DuplicateProof(id) => write!(f, "Duplicate proof: {}", id),
            FraudProofError::DisputeNotFound(id) => write!(f, "Dispute not found: {}", id),
            FraudProofError::DisputeAlreadyResolved => write!(f, "Dispute already resolved"),
            FraudProofError::ResponseAlreadySubmitted => write!(f, "Response already submitted"),
            FraudProofError::ResponsePeriodExpired { deadline, current } => {
                write!(f, "Response period expired: deadline {}, current {}", deadline, current)
            }
            FraudProofError::InvalidStateTransition(msg) => {
                write!(f, "Invalid state transition: {}", msg)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_timestamp() -> u64 {
        1234567890
    }

    fn create_test_transition(channel_id: &str, nonce: u64) -> StateTransition {
        StateTransition {
            channel_id: channel_id.to_string(),
            from_nonce: nonce,
            to_nonce: nonce + 1,
            from_balance_a: 1000,
            from_balance_b: 1000,
            to_balance_a: 900,
            to_balance_b: 1100,
            transition_type: TransitionType::Payment {
                from_a_to_b: true,
                amount: 100,
            },
            timestamp: mock_timestamp(),
        }
    }

    #[test]
    fn test_state_transition_verification() {
        let transition = create_test_transition("ch1", 0);
        assert!(transition.verify().is_ok());
    }

    #[test]
    fn test_state_transition_invalid_nonce() {
        let mut transition = create_test_transition("ch1", 0);
        transition.to_nonce = 10; // Invalid jump
        assert!(transition.verify().is_err());
    }

    #[test]
    fn test_state_transition_balance_violation() {
        let mut transition = create_test_transition("ch1", 0);
        transition.to_balance_a = 1500; // Violates conservation
        assert!(transition.verify().is_err());
    }

    #[test]
    fn test_fraud_proof_creation() {
        let transition = create_test_transition("ch1", 0);
        let proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        );
        assert!(proof.is_ok());
    }

    #[test]
    fn test_fraud_proof_too_large() {
        let transition = create_test_transition("ch1", 0);
        let large_data = vec![0u8; MAX_FRAUD_PROOF_SIZE + 1];
        let result = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            large_data,
            vec![5, 6, 7, 8],
            mock_timestamp(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fraud_proof_empty() {
        let transition = create_test_transition("ch1", 0);
        let result = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_challenge_response_creation() {
        let transition = create_test_transition("ch1", 0);
        let response = ChallengeResponse::new(
            "fraud_123".to_string(),
            "bob".to_string(),
            vec![1, 2, 3, 4],
            transition,
            vec![5, 6, 7, 8],
            mock_timestamp(),
        );
        assert!(response.is_ok());
    }

    #[test]
    fn test_dispute_creation() {
        let transition = create_test_transition("ch1", 0);
        let fraud_proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        )
        .unwrap();

        let dispute = Dispute::new(fraud_proof);
        assert!(matches!(dispute.resolution, DisputeResolution::Pending));
    }

    #[test]
    fn test_fraud_proof_system_submit() {
        let mut system = FraudProofSystem::new();
        let transition = create_test_transition("ch1", 0);
        let fraud_proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        )
        .unwrap();

        let result = system.submit_fraud_proof(fraud_proof);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fraud_proof_system_submit_response() {
        let mut system = FraudProofSystem::new();
        let transition = create_test_transition("ch1", 0);
        let fraud_proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition.clone(),
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        )
        .unwrap();

        let fraud_proof_id = fraud_proof.proof_id.clone();
        system.submit_fraud_proof(fraud_proof).unwrap();

        let response = ChallengeResponse::new(
            fraud_proof_id,
            "bob".to_string(),
            vec![9, 10, 11, 12],
            transition,
            vec![13, 14, 15, 16],
            mock_timestamp() + 100,
        )
        .unwrap();

        let result = system.submit_response(response, mock_timestamp() + 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_challenge_period_expiration() {
        let transition = create_test_transition("ch1", 0);
        let fraud_proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        )
        .unwrap();

        assert!(!fraud_proof.is_challenge_expired(mock_timestamp() + 100));
        assert!(fraud_proof.is_challenge_expired(mock_timestamp() + CHALLENGE_PERIOD + 1));
    }

    #[test]
    fn test_auto_resolve_expired_disputes() {
        let mut system = FraudProofSystem::new();
        let transition = create_test_transition("ch1", 0);
        let fraud_proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        )
        .unwrap();

        system.submit_fraud_proof(fraud_proof).unwrap();

        let resolved = system.auto_resolve_expired_disputes(mock_timestamp() + CHALLENGE_PERIOD + 1);
        assert_eq!(resolved.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut system = FraudProofSystem::new();
        let stats = system.get_statistics();
        assert_eq!(stats.total_disputes, 0);
        assert_eq!(stats.pending_disputes, 0);

        // Add a dispute
        let transition = create_test_transition("ch1", 0);
        let fraud_proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        )
        .unwrap();

        system.submit_fraud_proof(fraud_proof).unwrap();

        let stats = system.get_statistics();
        assert_eq!(stats.total_disputes, 1);
        assert_eq!(stats.pending_disputes, 1);
    }

    #[test]
    fn test_get_channel_disputes() {
        let mut system = FraudProofSystem::new();
        let transition = create_test_transition("ch1", 0);
        let fraud_proof = FraudProof::new(
            "ch1".to_string(),
            "alice".to_string(),
            transition,
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            mock_timestamp(),
        )
        .unwrap();

        system.submit_fraud_proof(fraud_proof).unwrap();

        let disputes = system.get_channel_disputes("ch1");
        assert_eq!(disputes.len(), 1);
    }
}
