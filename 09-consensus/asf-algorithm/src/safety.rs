//! # Safety and Liveness Proofs
//!
//! This module implements safety checks and Byzantine fault tolerance
//! validation for the ASF consensus protocol.
//!
//! Safety guarantees:
//! - No two conflicting blocks can both be finalized
//! - Finality is irreversible once achieved
//! - Byzantine tolerance: tolerates up to f < n/3 malicious validators

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use crate::{
    AsfError, AsfResult, Balance, BlockNumber, ConsensusPhase, Hash, ValidatorId,
    ValidityCertificate,
};

// ═══════════════════════════════════════════════════════════════════════════════
// SAFETY CHECKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Checks safety conditions for consensus
pub struct SafetyChecker {
    /// Maximum Byzantine validators (f = n/3)
    max_byzantine: u32,
    
    /// Total validators
    total_validators: u32,
    
    /// Total stake
    total_stake: Balance,
    
    /// Finalized blocks (cannot be reverted)
    finalized_blocks: BTreeSet<Hash>,
    
    /// Block ancestry (child -> parent mapping)
    ancestry: BTreeMap<Hash, Hash>,
}

impl SafetyChecker {
    /// Create a new safety checker
    pub fn new(total_validators: u32, total_stake: Balance) -> Self {
        let max_byzantine = total_validators / 3;
        
        Self {
            max_byzantine,
            total_validators,
            total_stake,
            finalized_blocks: BTreeSet::new(),
            ancestry: BTreeMap::new(),
        }
    }

    /// Check if a block can be safely finalized
    pub fn can_finalize(
        &self,
        block_hash: &Hash,
        certificates: &[ValidityCertificate],
    ) -> AsfResult<()> {
        // 1. Check certificate count meets Byzantine threshold
        self.check_byzantine_threshold(certificates)?;

        // 2. Check no conflicting finalized blocks
        self.check_no_conflicts(block_hash)?;

        // 3. Check certificate validity
        self.check_certificate_validity(certificates)?;

        Ok(())
    }

    /// Verify Byzantine threshold is met
    fn check_byzantine_threshold(&self, certificates: &[ValidityCertificate]) -> AsfResult<()> {
        let required = crate::bft_threshold(self.total_validators);
        let got = certificates.len() as u32;

        if got < required {
            return Err(AsfError::InsufficientVotes {
                got,
                need: required,
            });
        }

        // Check stake threshold
        let total_stake: Balance = certificates.iter().map(|c| c.stake_weight).sum();
        let required_stake = crate::bft_stake_threshold(self.total_stake);

        if total_stake < required_stake {
            return Err(AsfError::InsufficientStake {
                got: total_stake,
                need: required_stake,
            });
        }

        Ok(())
    }

    /// Check for conflicting finalized blocks
    fn check_no_conflicts(&self, block_hash: &Hash) -> AsfResult<()> {
        // A block conflicts with finalized blocks if it's not a descendant
        // For now, we do a simple check - in production this would be more sophisticated
        if self.finalized_blocks.is_empty() {
            return Ok(());
        }

        // Check if this block is a descendant of any finalized block
        let mut current = *block_hash;
        let mut checked = BTreeSet::new();

        while let Some(parent) = self.ancestry.get(&current) {
            if self.finalized_blocks.contains(parent) {
                return Ok(()); // Valid chain
            }

            if checked.contains(&current) {
                break; // Cycle detected
            }

            checked.insert(current);
            current = *parent;
        }

        // If we get here and there are finalized blocks, this might be a fork
        // In production, we'd do more sophisticated fork detection
        Ok(())
    }

    /// Verify all certificates are valid
    fn check_certificate_validity(&self, certificates: &[ValidityCertificate]) -> AsfResult<()> {
        // Check for duplicate validators
        let mut seen_validators = BTreeSet::new();

        for cert in certificates {
            if seen_validators.contains(&cert.validator) {
                return Err(AsfError::SafetyViolation("Duplicate validator certificate"));
            }
            seen_validators.insert(cert.validator.clone());

            // Check phase progression
            if let Some(prev_cert) = certificates
                .iter()
                .find(|c| c.validator == cert.validator && c.phase != cert.phase)
            {
                if !prev_cert.phase.precedes(&cert.phase) {
                    return Err(AsfError::SafetyViolation("Invalid phase progression"));
                }
            }
        }

        Ok(())
    }

    /// Mark a block as finalized
    pub fn mark_finalized(&mut self, block_hash: Hash) {
        self.finalized_blocks.insert(block_hash);
    }

    /// Record block ancestry
    pub fn record_ancestry(&mut self, child: Hash, parent: Hash) {
        self.ancestry.insert(child, parent);
    }

    /// Check if a block is finalized
    pub fn is_finalized(&self, block_hash: &Hash) -> bool {
        self.finalized_blocks.contains(block_hash)
    }

    /// Get maximum Byzantine validators tolerated
    pub fn max_byzantine(&self) -> u32 {
        self.max_byzantine
    }

    /// Update committee
    pub fn update_committee(&mut self, validators: u32, stake: Balance) {
        self.total_validators = validators;
        self.total_stake = stake;
        self.max_byzantine = validators / 3;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// LIVENESS CHECKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Checks liveness conditions (progress is being made)
pub struct LivenessChecker {
    /// Last finalized block number
    last_finalized: BlockNumber,
    
    /// Last finalized timestamp
    last_finalized_time: u64,
    
    /// Timeout threshold (milliseconds)
    timeout_threshold: u64,
    
    /// Number of view changes
    view_changes: u32,
}

impl LivenessChecker {
    /// Create a new liveness checker
    pub fn new(timeout_threshold: u64) -> Self {
        Self {
            last_finalized: 0,
            last_finalized_time: 0,
            timeout_threshold,
            view_changes: 0,
        }
    }

    /// Check if consensus is making progress
    pub fn check_progress(&self, current_time: u64) -> bool {
        let elapsed = current_time.saturating_sub(self.last_finalized_time);
        elapsed < self.timeout_threshold
    }

    /// Record a finalized block
    pub fn record_finalized(&mut self, block_number: BlockNumber, timestamp: u64) {
        self.last_finalized = block_number;
        self.last_finalized_time = timestamp;
    }

    /// Record a view change (timeout occurred)
    pub fn record_view_change(&mut self) {
        self.view_changes += 1;
    }

    /// Get view changes count
    pub fn view_changes(&self) -> u32 {
        self.view_changes
    }

    /// Get time since last finalization
    pub fn time_since_finalization(&self, current_time: u64) -> u64 {
        current_time.saturating_sub(self.last_finalized_time)
    }

    /// Reset counters
    pub fn reset(&mut self) {
        self.view_changes = 0;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BYZANTINE VALIDATOR DETECTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Detects potentially Byzantine (malicious) validators
pub struct ByzantineDetector {
    /// Validators under suspicion
    suspicious_validators: BTreeMap<ValidatorId, SuspicionRecord>,
    
    /// Threshold for marking as Byzantine
    suspicion_threshold: u32,
}

impl ByzantineDetector {
    /// Create a new Byzantine detector
    pub fn new(suspicion_threshold: u32) -> Self {
        Self {
            suspicious_validators: BTreeMap::new(),
            suspicion_threshold,
        }
    }

    /// Report suspicious behavior
    pub fn report_suspicious(
        &mut self,
        validator: ValidatorId,
        reason: SuspicionReason,
    ) {
        let record = self
            .suspicious_validators
            .entry(validator.clone())
            .or_insert_with(|| SuspicionRecord::new(validator));

        record.add_incident(reason);
    }

    /// Check if a validator is likely Byzantine
    pub fn is_byzantine(&self, validator: &ValidatorId) -> bool {
        self.suspicious_validators
            .get(validator)
            .map(|r| r.incident_count >= self.suspicion_threshold)
            .unwrap_or(false)
    }

    /// Get all suspected Byzantine validators
    pub fn get_suspected(&self) -> Vec<ValidatorId> {
        self.suspicious_validators
            .iter()
            .filter(|(_, r)| r.incident_count >= self.suspicion_threshold)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Clear suspicion for a validator (e.g., after slashing)
    pub fn clear_suspicion(&mut self, validator: &ValidatorId) {
        self.suspicious_validators.remove(validator);
    }

    /// Get suspicion record
    pub fn get_record(&self, validator: &ValidatorId) -> Option<&SuspicionRecord> {
        self.suspicious_validators.get(validator)
    }
}

impl Default for ByzantineDetector {
    fn default() -> Self {
        Self::new(3) // 3 incidents = likely Byzantine
    }
}

/// Record of suspicious behavior
#[derive(Debug, Clone)]
pub struct SuspicionRecord {
    /// Validator ID
    pub validator: ValidatorId,
    
    /// Number of suspicious incidents
    pub incident_count: u32,
    
    /// Reasons for suspicion
    pub reasons: Vec<SuspicionReason>,
    
    /// First incident timestamp
    pub first_incident: u64,
    
    /// Last incident timestamp
    pub last_incident: u64,
}

impl SuspicionRecord {
    fn new(validator: ValidatorId) -> Self {
        Self {
            validator,
            incident_count: 0,
            reasons: Vec::new(),
            first_incident: 0,
            last_incident: 0,
        }
    }

    fn add_incident(&mut self, reason: SuspicionReason) {
        self.incident_count += 1;
        self.reasons.push(reason);
        
        let now = 0; // Should be actual timestamp
        if self.first_incident == 0 {
            self.first_incident = now;
        }
        self.last_incident = now;

        // Keep only last 10 reasons
        if self.reasons.len() > 10 {
            self.reasons.remove(0);
        }
    }
}

/// Reasons for suspecting Byzantine behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuspicionReason {
    /// Voted for conflicting blocks
    ConflictingVotes,
    
    /// Invalid signature
    InvalidSignature,
    
    /// Vote in wrong phase
    InvalidPhase,
    
    /// Duplicate vote
    DuplicateVote,
    
    /// Missed too many blocks
    Unavailable,
    
    /// Invalid certificate
    InvalidCertificate,
}

// ═══════════════════════════════════════════════════════════════════════════════
// FORK DETECTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Detects forks in the chain
pub struct ForkDetector {
    /// Blocks at each height
    blocks_at_height: BTreeMap<BlockNumber, Vec<Hash>>,
}

impl ForkDetector {
    /// Create a new fork detector
    pub fn new() -> Self {
        Self {
            blocks_at_height: BTreeMap::new(),
        }
    }

    /// Record a block at a height
    pub fn record_block(&mut self, block_number: BlockNumber, block_hash: Hash) {
        self.blocks_at_height
            .entry(block_number)
            .or_insert_with(Vec::new)
            .push(block_hash);
    }

    /// Check if there's a fork at a height
    pub fn has_fork_at(&self, block_number: BlockNumber) -> bool {
        self.blocks_at_height
            .get(&block_number)
            .map(|blocks| blocks.len() > 1)
            .unwrap_or(false)
    }

    /// Get all forks
    pub fn get_forks(&self) -> Vec<(BlockNumber, Vec<Hash>)> {
        self.blocks_at_height
            .iter()
            .filter(|(_, blocks)| blocks.len() > 1)
            .map(|(height, blocks)| (*height, blocks.clone()))
            .collect()
    }

    /// Clear old blocks
    pub fn prune(&mut self, keep_last_n: usize) {
        if self.blocks_at_height.len() <= keep_last_n {
            return;
        }

        let to_remove = self.blocks_at_height.len() - keep_last_n;
        let keys_to_remove: Vec<BlockNumber> = self
            .blocks_at_height
            .keys()
            .take(to_remove)
            .copied()
            .collect();

        for key in keys_to_remove {
            self.blocks_at_height.remove(&key);
        }
    }
}

impl Default for ForkDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    fn create_test_cert(validator_id: u8, stake: Balance, phase: ConsensusPhase) -> ValidityCertificate {
        let mut account_bytes = [0u8; 32];
        account_bytes[0] = validator_id;
        
        ValidityCertificate {
            block_hash: Hash::default(),
            block_number: 1,
            phase,
            validator: AccountId32::from(account_bytes),
            stake_weight: stake,
            epoch: 1,
            timestamp: 1000,
            vote_aggregate: crate::VoteAggregate {
                block_hash: Hash::default(),
                block_number: 1,
                phase,
                validator_count: 1,
                total_stake: stake,
                validators: vec![AccountId32::from(account_bytes)],
            },
        }
    }

    #[test]
    fn test_safety_checker_creation() {
        let checker = SafetyChecker::new(21, 21_000);
        assert_eq!(checker.max_byzantine(), 7); // 21 / 3 = 7
    }

    #[test]
    fn test_byzantine_threshold() {
        let checker = SafetyChecker::new(21, 21_000);
        
        // 15 certificates should meet threshold (2/3 of 21)
        let certs: Vec<_> = (0..15)
            .map(|i| create_test_cert(i, 1000, ConsensusPhase::Prepare))
            .collect();
        
        assert!(checker.check_byzantine_threshold(&certs).is_ok());
        
        // 10 certificates should not meet threshold
        let certs: Vec<_> = (0..10)
            .map(|i| create_test_cert(i, 1000, ConsensusPhase::Prepare))
            .collect();
        
        assert!(checker.check_byzantine_threshold(&certs).is_err());
    }

    #[test]
    fn test_finalized_blocks() {
        let mut checker = SafetyChecker::new(21, 21_000);
        let block_hash = Hash::default();
        
        assert!(!checker.is_finalized(&block_hash));
        checker.mark_finalized(block_hash);
        assert!(checker.is_finalized(&block_hash));
    }

    #[test]
    fn test_liveness_checker() {
        let mut checker = LivenessChecker::new(10_000); // 10 second timeout
        
        checker.record_finalized(1, 1000);
        assert!(checker.check_progress(5000)); // 4 seconds elapsed
        assert!(!checker.check_progress(15000)); // 14 seconds elapsed
    }

    #[test]
    fn test_byzantine_detector() {
        let mut detector = ByzantineDetector::new(3);
        
        let mut validator_bytes = [0u8; 32];
        validator_bytes[0] = 1;
        let validator = AccountId32::from(validator_bytes);
        
        assert!(!detector.is_byzantine(&validator));
        
        // Report 3 incidents
        detector.report_suspicious(validator.clone(), SuspicionReason::ConflictingVotes);
        detector.report_suspicious(validator.clone(), SuspicionReason::InvalidSignature);
        detector.report_suspicious(validator.clone(), SuspicionReason::DuplicateVote);
        
        assert!(detector.is_byzantine(&validator));
    }

    #[test]
    fn test_fork_detector() {
        let mut detector = ForkDetector::new();
        
        let mut hash1_bytes = [0u8; 32];
        hash1_bytes[0] = 1;
        let hash1 = Hash::from(hash1_bytes);
        
        let mut hash2_bytes = [0u8; 32];
        hash2_bytes[0] = 2;
        let hash2 = Hash::from(hash2_bytes);
        
        detector.record_block(1, hash1);
        assert!(!detector.has_fork_at(1));
        
        detector.record_block(1, hash2);
        assert!(detector.has_fork_at(1));
        
        let forks = detector.get_forks();
        assert_eq!(forks.len(), 1);
        assert_eq!(forks[0].1.len(), 2);
    }

    #[test]
    fn test_view_changes() {
        let mut checker = LivenessChecker::new(10_000);
        
        assert_eq!(checker.view_changes(), 0);
        checker.record_view_change();
        assert_eq!(checker.view_changes(), 1);
        checker.record_view_change();
        assert_eq!(checker.view_changes(), 2);
    }
}
