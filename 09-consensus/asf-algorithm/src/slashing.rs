//! # Slashing Enforcement for Byzantine Validators
//!
//! This module connects Byzantine detection to stake penalties (slashing) and
//! validator exclusion from consensus participation.
//!
//! Security Features:
//! - Automatic slashing when Byzantine behavior is confirmed
//! - Graduated penalties based on severity
//! - Validator exclusion from future consensus rounds
//! - Stake confiscation and redistribution

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    safety::{SuspicionReason, SuspicionRecord},
    AsfError, AsfResult, Balance, ValidatorId,
};

// ═══════════════════════════════════════════════════════════════════════════════
// SLASHING SEVERITY LEVELS
// ═══════════════════════════════════════════════════════════════════════════════

/// Severity level for slashing penalties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum SlashingSeverity {
    /// Minor infraction (1-5% stake)
    Minor,
    /// Moderate infraction (10-20% stake)
    Moderate,
    /// Severe infraction (30-50% stake)
    Severe,
    /// Critical Byzantine behavior (100% stake + exclusion)
    Critical,
}

impl SlashingSeverity {
    /// Get the percentage of stake to slash (in basis points: 10000 = 100%)
    pub fn slash_percentage(&self) -> u32 {
        match self {
            SlashingSeverity::Minor => 500,      // 5%
            SlashingSeverity::Moderate => 1500,  // 15%
            SlashingSeverity::Severe => 4000,    // 40%
            SlashingSeverity::Critical => 10000, // 100%
        }
    }

    /// Determine severity from suspicion reason
    pub fn from_suspicion(reason: SuspicionReason) -> Self {
        match reason {
            SuspicionReason::Unavailable => SlashingSeverity::Minor,
            SuspicionReason::DuplicateVote => SlashingSeverity::Moderate,
            SuspicionReason::InvalidPhase => SlashingSeverity::Moderate,
            SuspicionReason::InvalidSignature => SlashingSeverity::Severe,
            SuspicionReason::ConflictingVotes => SlashingSeverity::Critical,
            SuspicionReason::InvalidCertificate => SlashingSeverity::Critical,
        }
    }

    /// Check if this severity level requires permanent exclusion
    pub fn requires_exclusion(&self) -> bool {
        matches!(self, SlashingSeverity::Critical)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLASHING EVENTS
// ═══════════════════════════════════════════════════════════════════════════════

/// A slashing event that occurred
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct SlashingEvent {
    /// Validator being slashed
    pub validator: ValidatorId,

    /// Severity of the slash
    pub severity: SlashingSeverity,

    /// Reason for slashing
    pub reason: SuspicionReason,

    /// Amount slashed
    pub amount_slashed: Balance,

    /// Epoch when slash occurred
    pub epoch: u32,

    /// Timestamp
    pub timestamp: u64,

    /// Whether validator was excluded
    pub excluded: bool,
}

impl SlashingEvent {
    /// Create a new slashing event
    pub fn new(
        validator: ValidatorId,
        severity: SlashingSeverity,
        reason: SuspicionReason,
        amount_slashed: Balance,
        epoch: u32,
        timestamp: u64,
        excluded: bool,
    ) -> Self {
        Self {
            validator,
            severity,
            reason,
            amount_slashed,
            epoch,
            timestamp,
            excluded,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLASHING ENFORCER
// ═══════════════════════════════════════════════════════════════════════════════

/// Enforces slashing penalties for Byzantine validators
pub struct SlashingEnforcer {
    /// Current stakes of all validators
    validator_stakes: BTreeMap<ValidatorId, Balance>,

    /// Validators excluded from consensus
    excluded_validators: BTreeMap<ValidatorId, ExclusionRecord>,

    /// History of slashing events
    slashing_history: Vec<SlashingEvent>,

    /// Treasury to receive slashed funds
    treasury_balance: Balance,

    /// Minimum stake required to be a validator
    minimum_stake: Balance,
}

impl SlashingEnforcer {
    /// Create a new slashing enforcer
    pub fn new(minimum_stake: Balance) -> Self {
        Self {
            validator_stakes: BTreeMap::new(),
            excluded_validators: BTreeMap::new(),
            slashing_history: Vec::new(),
            treasury_balance: 0,
            minimum_stake,
        }
    }

    /// Register a validator with their stake
    pub fn register_validator(&mut self, validator: ValidatorId, stake: Balance) -> AsfResult<()> {
        if stake < self.minimum_stake {
            return Err(AsfError::InsufficientStake {
                got: stake,
                need: self.minimum_stake,
            });
        }

        self.validator_stakes.insert(validator, stake);
        Ok(())
    }

    /// Execute slashing based on suspicion record
    pub fn slash_validator(
        &mut self,
        validator: ValidatorId,
        record: &SuspicionRecord,
        epoch: u32,
        timestamp: u64,
    ) -> AsfResult<SlashingEvent> {
        // Check if validator is already excluded
        if self.is_excluded(&validator) {
            return Err(AsfError::SafetyViolation("Validator already excluded"));
        }

        // Get validator's current stake
        let current_stake = self
            .validator_stakes
            .get(&validator)
            .copied()
            .ok_or(AsfError::InvalidVote("Validator not found"))?;

        // Determine severity based on most severe reason
        let severity = record
            .reasons
            .iter()
            .map(|r| SlashingSeverity::from_suspicion(*r))
            .max_by_key(|s| s.slash_percentage())
            .unwrap_or(SlashingSeverity::Minor);

        // Calculate slash amount
        let slash_percentage = severity.slash_percentage();
        let slash_amount = (current_stake * slash_percentage as u128) / 10000;

        // Execute the slash
        let new_stake = current_stake.saturating_sub(slash_amount);
        self.validator_stakes.insert(validator.clone(), new_stake);

        // Add slashed funds to treasury
        self.treasury_balance = self.treasury_balance.saturating_add(slash_amount);

        // Check if validator should be excluded
        let excluded = severity.requires_exclusion() || new_stake < self.minimum_stake;

        if excluded {
            self.exclude_validator(validator.clone(), severity, epoch, timestamp);
        }

        // Create slashing event
        let event = SlashingEvent::new(
            validator.clone(),
            severity,
            record.reasons.last().copied().unwrap_or(SuspicionReason::InvalidSignature),
            slash_amount,
            epoch,
            timestamp,
            excluded,
        );

        // Record in history
        self.slashing_history.push(event.clone());

        Ok(event)
    }

    /// Manually exclude a validator from consensus
    pub fn exclude_validator(
        &mut self,
        validator: ValidatorId,
        severity: SlashingSeverity,
        epoch: u32,
        timestamp: u64,
    ) {
        let record = ExclusionRecord {
            validator: validator.clone(),
            severity,
            excluded_at_epoch: epoch,
            excluded_at_timestamp: timestamp,
            permanent: severity.requires_exclusion(),
        };

        self.excluded_validators.insert(validator, record);
    }

    /// Check if a validator is excluded
    pub fn is_excluded(&self, validator: &ValidatorId) -> bool {
        self.excluded_validators.contains_key(validator)
    }

    /// Get all excluded validators
    pub fn get_excluded_validators(&self) -> Vec<ValidatorId> {
        self.excluded_validators.keys().cloned().collect()
    }

    /// Get validator's current stake
    pub fn get_stake(&self, validator: &ValidatorId) -> Option<Balance> {
        self.validator_stakes.get(validator).copied()
    }

    /// Get total active stake (excluding excluded validators)
    pub fn get_total_active_stake(&self) -> Balance {
        self.validator_stakes
            .iter()
            .filter(|(v, _)| !self.is_excluded(v))
            .map(|(_, s)| s)
            .sum()
    }

    /// Get total number of active validators
    pub fn get_active_validator_count(&self) -> u32 {
        self.validator_stakes
            .iter()
            .filter(|(v, _)| !self.is_excluded(v))
            .count() as u32
    }

    /// Get treasury balance (slashed funds)
    pub fn treasury_balance(&self) -> Balance {
        self.treasury_balance
    }

    /// Get slashing history
    pub fn get_slashing_history(&self) -> &[SlashingEvent] {
        &self.slashing_history
    }

    /// Get exclusion record for a validator
    pub fn get_exclusion_record(&self, validator: &ValidatorId) -> Option<&ExclusionRecord> {
        self.excluded_validators.get(validator)
    }

    /// Clear old slashing history (keep last N events)
    pub fn prune_history(&mut self, keep_last_n: usize) {
        if self.slashing_history.len() > keep_last_n {
            let remove_count = self.slashing_history.len() - keep_last_n;
            self.slashing_history.drain(0..remove_count);
        }
    }
}

/// Record of a validator exclusion
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ExclusionRecord {
    /// Excluded validator
    pub validator: ValidatorId,

    /// Severity that caused exclusion
    pub severity: SlashingSeverity,

    /// Epoch when excluded
    pub excluded_at_epoch: u32,

    /// Timestamp when excluded
    pub excluded_at_timestamp: u64,

    /// Whether exclusion is permanent
    pub permanent: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTEGRATED BYZANTINE HANDLER
// ═══════════════════════════════════════════════════════════════════════════════

/// Integrates Byzantine detection with slashing enforcement
pub struct ByzantineHandler {
    /// The slashing enforcer
    pub enforcer: SlashingEnforcer,
}

impl ByzantineHandler {
    /// Create a new Byzantine handler
    pub fn new(minimum_stake: Balance) -> Self {
        Self {
            enforcer: SlashingEnforcer::new(minimum_stake),
        }
    }

    /// Handle Byzantine detection and execute slashing
    pub fn handle_byzantine_detection(
        &mut self,
        validator: ValidatorId,
        record: &SuspicionRecord,
        epoch: u32,
        timestamp: u64,
    ) -> AsfResult<SlashingEvent> {
        // Execute slashing
        let event = self.enforcer.slash_validator(validator, record, epoch, timestamp)?;

        Ok(event)
    }

    /// Check if validator should be excluded from consensus
    pub fn should_exclude(&self, validator: &ValidatorId) -> bool {
        self.enforcer.is_excluded(validator)
    }

    /// Get list of validators who can participate in consensus
    pub fn get_eligible_validators(&self) -> Vec<ValidatorId> {
        self.enforcer
            .validator_stakes
            .keys()
            .filter(|v| !self.enforcer.is_excluded(v))
            .cloned()
            .collect()
    }

    /// Register a validator
    pub fn register_validator(&mut self, validator: ValidatorId, stake: Balance) -> AsfResult<()> {
        self.enforcer.register_validator(validator, stake)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    fn create_test_validator(id: u8) -> ValidatorId {
        let mut bytes = [0u8; 32];
        bytes[0] = id;
        AccountId32::from(bytes)
    }

    fn create_test_record(reasons: Vec<SuspicionReason>) -> SuspicionRecord {
        let validator = create_test_validator(1);
        let mut record = SuspicionRecord {
            validator: validator.clone(),
            incident_count: reasons.len() as u32,
            reasons: reasons.clone(),
            first_incident: 1000,
            last_incident: 2000,
        };
        record
    }

    #[test]
    fn test_slash_severity_levels() {
        assert_eq!(SlashingSeverity::Minor.slash_percentage(), 500); // 5%
        assert_eq!(SlashingSeverity::Moderate.slash_percentage(), 1500); // 15%
        assert_eq!(SlashingSeverity::Severe.slash_percentage(), 4000); // 40%
        assert_eq!(SlashingSeverity::Critical.slash_percentage(), 10000); // 100%
    }

    #[test]
    fn test_severity_from_suspicion() {
        assert_eq!(
            SlashingSeverity::from_suspicion(SuspicionReason::Unavailable),
            SlashingSeverity::Minor
        );
        assert_eq!(
            SlashingSeverity::from_suspicion(SuspicionReason::ConflictingVotes),
            SlashingSeverity::Critical
        );
    }

    #[test]
    fn test_slashing_enforcer() {
        let mut enforcer = SlashingEnforcer::new(1000);
        let validator = create_test_validator(1);

        // Register validator with 10,000 stake
        enforcer.register_validator(validator.clone(), 10_000).unwrap();
        assert_eq!(enforcer.get_stake(&validator), Some(10_000));

        // Create a record with moderate suspicion
        let record = create_test_record(vec![SuspicionReason::DuplicateVote]);

        // Execute slash
        let event = enforcer.slash_validator(validator.clone(), &record, 1, 1000).unwrap();

        // Should slash 15% (Moderate severity)
        assert_eq!(event.amount_slashed, 1_500); // 15% of 10,000
        assert_eq!(event.severity, SlashingSeverity::Moderate);
        assert!(!event.excluded); // Not excluded for moderate

        // New stake should be 8,500
        assert_eq!(enforcer.get_stake(&validator), Some(8_500));

        // Treasury should receive slashed funds
        assert_eq!(enforcer.treasury_balance(), 1_500);
    }

    #[test]
    fn test_critical_slash_excludes_validator() {
        let mut enforcer = SlashingEnforcer::new(1000);
        let validator = create_test_validator(1);

        enforcer.register_validator(validator.clone(), 10_000).unwrap();

        // Create critical suspicion record
        let record = create_test_record(vec![SuspicionReason::ConflictingVotes]);

        // Execute slash
        let event = enforcer.slash_validator(validator.clone(), &record, 1, 1000).unwrap();

        // Should slash 100% and exclude
        assert_eq!(event.amount_slashed, 10_000);
        assert_eq!(event.severity, SlashingSeverity::Critical);
        assert!(event.excluded);

        // Validator should be excluded
        assert!(enforcer.is_excluded(&validator));

        // Stake should be 0
        assert_eq!(enforcer.get_stake(&validator), Some(0));
    }

    #[test]
    fn test_below_minimum_stake_exclusion() {
        let mut enforcer = SlashingEnforcer::new(8000); // Minimum 8000
        let validator = create_test_validator(1);

        enforcer.register_validator(validator.clone(), 10_000).unwrap();

        // InvalidSignature = Severe = 40% slash
        // 10,000 - 4,000 = 6,000 < 8,000 minimum → excluded
        let record = create_test_record(vec![SuspicionReason::InvalidSignature]);

        let event = enforcer.slash_validator(validator.clone(), &record, 1, 1000).unwrap();

        // Should be excluded because stake falls below minimum
        assert!(event.excluded);
        assert!(enforcer.is_excluded(&validator));
    }

    #[test]
    fn test_byzantine_handler_integration() {
        let mut handler = ByzantineHandler::new(1000);
        let validator = create_test_validator(1);

        handler.register_validator(validator.clone(), 10_000).unwrap();

        // Handle Byzantine detection
        let record = create_test_record(vec![
            SuspicionReason::DuplicateVote,
            SuspicionReason::InvalidSignature,
        ]);

        let event = handler
            .handle_byzantine_detection(validator.clone(), &record, 1, 1000)
            .unwrap();

        // Should slash at severe level (worst offense)
        assert_eq!(event.severity, SlashingSeverity::Severe);
        assert_eq!(event.amount_slashed, 4_000); // 40% of 10,000
    }

    #[test]
    fn test_active_validator_count() {
        let mut enforcer = SlashingEnforcer::new(1000);

        let val1 = create_test_validator(1);
        let val2 = create_test_validator(2);
        let val3 = create_test_validator(3);

        enforcer.register_validator(val1.clone(), 10_000).unwrap();
        enforcer.register_validator(val2.clone(), 10_000).unwrap();
        enforcer.register_validator(val3.clone(), 10_000).unwrap();

        assert_eq!(enforcer.get_active_validator_count(), 3);
        assert_eq!(enforcer.get_total_active_stake(), 30_000);

        // Exclude one validator
        let record = create_test_record(vec![SuspicionReason::ConflictingVotes]);
        enforcer.slash_validator(val2, &record, 1, 1000).unwrap();

        assert_eq!(enforcer.get_active_validator_count(), 2);
        assert_eq!(enforcer.get_total_active_stake(), 20_000); // Only val1 and val3
    }

    #[test]
    fn test_exclusion_record() {
        let mut enforcer = SlashingEnforcer::new(1000);
        let validator = create_test_validator(1);

        enforcer.exclude_validator(
            validator.clone(),
            SlashingSeverity::Critical,
            5,
            5000,
        );

        let record = enforcer.get_exclusion_record(&validator).unwrap();
        assert_eq!(record.severity, SlashingSeverity::Critical);
        assert_eq!(record.excluded_at_epoch, 5);
        assert!(record.permanent);
    }
}
