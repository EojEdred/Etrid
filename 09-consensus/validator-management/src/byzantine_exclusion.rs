//! # Byzantine Validator Exclusion
//!
//! This module manages active exclusion of Byzantine validators from consensus
//! participation. It integrates with the ByzantineDetector from asf-algorithm
//! to enforce security policies.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::{ValidatorError, ValidatorId, ValidatorResult};
use asf_algorithm::ByzantineDetector;

// ═══════════════════════════════════════════════════════════════════════════════
// EXCLUSION MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manages exclusion of Byzantine validators
#[derive(Debug, Clone)]
pub struct ByzantineExclusionManager {
    /// Validators currently excluded from consensus
    excluded_validators: BTreeMap<ValidatorId, ExclusionRecord>,

    /// Automatic exclusion threshold (incidents)
    auto_exclude_threshold: u32,

    /// Exclusion duration in blocks
    exclusion_duration: u32,

    /// Permanent exclusion threshold (repeated offenses)
    permanent_ban_threshold: u32,
}

/// Record of validator exclusion
#[derive(Debug, Clone)]
pub struct ExclusionRecord {
    /// Validator ID
    pub validator: ValidatorId,

    /// Reason for exclusion
    pub reason: ExclusionReason,

    /// Block number when excluded
    pub excluded_at: u64,

    /// Block number when exclusion expires (None = permanent)
    pub expires_at: Option<u64>,

    /// Number of times this validator has been excluded
    pub offense_count: u32,
}

/// Reasons for excluding a validator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExclusionReason {
    /// Too many Byzantine suspicions
    ByzantineBehavior { incident_count: u32 },

    /// Failed signature verification
    InvalidSignatures { count: u32 },

    /// Conflicting votes detected
    Equivocation,

    /// Repeated slashing offenses
    RepeatedSlashing { times: u32 },

    /// Manual exclusion (governance decision)
    ManualBan,

    /// Insufficient stake (below minimum)
    InsufficientStake,
}

impl ByzantineExclusionManager {
    /// Create a new exclusion manager
    pub fn new(
        auto_exclude_threshold: u32,
        exclusion_duration: u32,
        permanent_ban_threshold: u32,
    ) -> Self {
        Self {
            excluded_validators: BTreeMap::new(),
            auto_exclude_threshold,
            exclusion_duration,
            permanent_ban_threshold,
        }
    }

    /// Process Byzantine detections and exclude validators automatically
    pub fn process_detections(
        &mut self,
        detector: &ByzantineDetector,
        current_block: u64,
    ) -> Vec<ValidatorId> {
        let mut newly_excluded = Vec::new();

        for validator in detector.get_suspected() {
            if let Some(record) = detector.get_record(&validator) {
                // Auto-exclude if threshold exceeded
                if record.incident_count >= self.auto_exclude_threshold
                    && !self.is_excluded(&validator)
                {
                    let reason = ExclusionReason::ByzantineBehavior {
                        incident_count: record.incident_count,
                    };

                    self.exclude_validator(validator.clone(), reason, current_block);

                    log::warn!(
                        "🚫 Auto-excluded Byzantine validator {:?} (incidents: {})",
                        validator,
                        record.incident_count
                    );

                    newly_excluded.push(validator);
                }
            }
        }

        newly_excluded
    }

    /// Exclude a validator from consensus
    pub fn exclude_validator(
        &mut self,
        validator: ValidatorId,
        reason: ExclusionReason,
        current_block: u64,
    ) {
        let offense_count = self
            .excluded_validators
            .get(&validator)
            .map(|r| r.offense_count + 1)
            .unwrap_or(1);

        // Permanent ban if too many offenses
        let expires_at = if offense_count >= self.permanent_ban_threshold {
            None // Permanent
        } else {
            Some(current_block + self.exclusion_duration as u64)
        };

        let record = ExclusionRecord {
            validator: validator.clone(),
            reason,
            excluded_at: current_block,
            expires_at,
            offense_count,
        };

        self.excluded_validators.insert(validator, record);
    }

    /// Check if a validator is currently excluded
    pub fn is_excluded(&self, validator: &ValidatorId) -> bool {
        self.excluded_validators.contains_key(validator)
    }

    /// Get exclusion record for a validator
    pub fn get_exclusion(&self, validator: &ValidatorId) -> Option<&ExclusionRecord> {
        self.excluded_validators.get(validator)
    }

    /// Check if a validator can participate in consensus
    pub fn can_participate(&self, validator: &ValidatorId, current_block: u64) -> bool {
        if let Some(record) = self.excluded_validators.get(validator) {
            // Check if exclusion has expired
            if let Some(expires_at) = record.expires_at {
                if current_block >= expires_at {
                    // Exclusion expired, can participate
                    return true;
                }
            }
            // Still excluded or permanent ban
            false
        } else {
            // Not excluded, can participate
            true
        }
    }

    /// Remove expired exclusions
    pub fn cleanup_expired(&mut self, current_block: u64) -> Vec<ValidatorId> {
        let mut restored = Vec::new();

        self.excluded_validators.retain(|validator, record| {
            if let Some(expires_at) = record.expires_at {
                if current_block >= expires_at {
                    // Exclusion expired
                    restored.push(validator.clone());
                    log::info!(
                        "✅ Restored validator {:?} after exclusion period",
                        validator
                    );
                    return false; // Remove from excluded list
                }
            }
            true // Keep in excluded list
        });

        restored
    }

    /// Get list of all excluded validators
    pub fn excluded_list(&self) -> Vec<ValidatorId> {
        self.excluded_validators.keys().cloned().collect()
    }

    /// Get number of excluded validators
    pub fn exclusion_count(&self) -> usize {
        self.excluded_validators.len()
    }

    /// Manually reinstate a validator (requires governance approval)
    pub fn reinstate_validator(&mut self, validator: &ValidatorId) -> ValidatorResult<()> {
        if self.excluded_validators.remove(validator).is_some() {
            log::info!("🔓 Manually reinstated validator {:?}", validator);
            Ok(())
        } else {
            Err(ValidatorError::NotFound)
        }
    }

    /// Get statistics on exclusions
    pub fn exclusion_stats(&self) -> ExclusionStats {
        let mut by_reason: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut permanent_bans = 0;
        let mut temporary_exclusions = 0;

        for record in self.excluded_validators.values() {
            let reason_str = match &record.reason {
                ExclusionReason::ByzantineBehavior { .. } => "Byzantine Behavior",
                ExclusionReason::InvalidSignatures { .. } => "Invalid Signatures",
                ExclusionReason::Equivocation => "Equivocation",
                ExclusionReason::RepeatedSlashing { .. } => "Repeated Slashing",
                ExclusionReason::ManualBan => "Manual Ban",
                ExclusionReason::InsufficientStake => "Insufficient Stake",
            };

            *by_reason.entry(reason_str).or_insert(0) += 1;

            if record.expires_at.is_none() {
                permanent_bans += 1;
            } else {
                temporary_exclusions += 1;
            }
        }

        ExclusionStats {
            total_excluded: self.excluded_validators.len(),
            permanent_bans,
            temporary_exclusions,
            by_reason,
        }
    }
}

impl Default for ByzantineExclusionManager {
    fn default() -> Self {
        Self::new(
            5,      // Auto-exclude after 5 incidents
            14400,  // Exclusion for 14400 blocks (~24 hours at 6s)
            3,      // Permanent ban after 3 offenses
        )
    }
}

/// Statistics on validator exclusions
#[derive(Debug, Clone)]
pub struct ExclusionStats {
    /// Total validators currently excluded
    pub total_excluded: usize,

    /// Permanent bans
    pub permanent_bans: usize,

    /// Temporary exclusions
    pub temporary_exclusions: usize,

    /// Exclusions by reason
    pub by_reason: BTreeMap<&'static str, u32>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTEGRATION WITH CONSENSUS
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if validator is allowed to participate in consensus
pub fn is_validator_allowed(
    validator: &ValidatorId,
    exclusion_manager: &ByzantineExclusionManager,
    current_block: u64,
) -> bool {
    exclusion_manager.can_participate(validator, current_block)
}

/// Filter out excluded validators from a list
pub fn filter_excluded_validators(
    validators: Vec<ValidatorId>,
    exclusion_manager: &ByzantineExclusionManager,
    current_block: u64,
) -> Vec<ValidatorId> {
    validators
        .into_iter()
        .filter(|v| exclusion_manager.can_participate(v, current_block))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_validator(id: u8) -> ValidatorId {
        ValidatorId::from([id; 32])
    }

    #[test]
    fn test_exclusion_manager_creation() {
        let manager = ByzantineExclusionManager::new(5, 100, 3);
        assert_eq!(manager.auto_exclude_threshold, 5);
        assert_eq!(manager.exclusion_count(), 0);
    }

    #[test]
    fn test_exclude_validator() {
        let mut manager = ByzantineExclusionManager::new(5, 100, 3);
        let validator = create_test_validator(1);

        let reason = ExclusionReason::ByzantineBehavior { incident_count: 5 };
        manager.exclude_validator(validator.clone(), reason, 1000);

        assert!(manager.is_excluded(&validator));
        assert!(!manager.can_participate(&validator, 1000));
    }

    #[test]
    fn test_temporary_exclusion_expires() {
        let mut manager = ByzantineExclusionManager::new(5, 100, 3);
        let validator = create_test_validator(1);

        let reason = ExclusionReason::Equivocation;
        manager.exclude_validator(validator.clone(), reason, 1000);

        // Should be excluded at block 1000
        assert!(!manager.can_participate(&validator, 1000));

        // Should still be excluded at block 1050
        assert!(!manager.can_participate(&validator, 1050));

        // Should be restored at block 1100 (1000 + 100 duration)
        assert!(manager.can_participate(&validator, 1100));
    }

    #[test]
    fn test_permanent_ban_after_repeated_offenses() {
        let mut manager = ByzantineExclusionManager::new(5, 100, 3);
        let validator = create_test_validator(1);

        // First offense
        manager.exclude_validator(
            validator.clone(),
            ExclusionReason::Equivocation,
            1000,
        );
        let record = manager.get_exclusion(&validator).unwrap();
        assert_eq!(record.offense_count, 1);
        assert!(record.expires_at.is_some());

        // Second offense
        manager.exclude_validator(
            validator.clone(),
            ExclusionReason::Equivocation,
            2000,
        );
        let record = manager.get_exclusion(&validator).unwrap();
        assert_eq!(record.offense_count, 2);
        assert!(record.expires_at.is_some());

        // Third offense - permanent ban
        manager.exclude_validator(
            validator.clone(),
            ExclusionReason::Equivocation,
            3000,
        );
        let record = manager.get_exclusion(&validator).unwrap();
        assert_eq!(record.offense_count, 3);
        assert!(record.expires_at.is_none()); // Permanent

        // Should never be allowed again
        assert!(!manager.can_participate(&validator, 10000));
    }

    #[test]
    fn test_cleanup_expired_exclusions() {
        let mut manager = ByzantineExclusionManager::new(5, 100, 3);

        let v1 = create_test_validator(1);
        let v2 = create_test_validator(2);

        manager.exclude_validator(v1.clone(), ExclusionReason::Equivocation, 1000);
        manager.exclude_validator(v2.clone(), ExclusionReason::Equivocation, 1050);

        assert_eq!(manager.exclusion_count(), 2);

        // Cleanup at block 1100 - v1 should be restored
        let restored = manager.cleanup_expired(1100);
        assert_eq!(restored.len(), 1);
        assert!(restored.contains(&v1));
        assert_eq!(manager.exclusion_count(), 1);

        // Cleanup at block 1150 - v2 should be restored
        let restored = manager.cleanup_expired(1150);
        assert_eq!(restored.len(), 1);
        assert!(restored.contains(&v2));
        assert_eq!(manager.exclusion_count(), 0);
    }

    #[test]
    fn test_manual_reinstatement() {
        let mut manager = ByzantineExclusionManager::new(5, 100, 3);
        let validator = create_test_validator(1);

        manager.exclude_validator(
            validator.clone(),
            ExclusionReason::ManualBan,
            1000,
        );
        assert!(manager.is_excluded(&validator));

        // Manually reinstate
        assert!(manager.reinstate_validator(&validator).is_ok());
        assert!(!manager.is_excluded(&validator));
    }

    #[test]
    fn test_exclusion_stats() {
        let mut manager = ByzantineExclusionManager::new(5, 100, 3);

        // Add various exclusions
        manager.exclude_validator(
            create_test_validator(1),
            ExclusionReason::ByzantineBehavior { incident_count: 5 },
            1000,
        );
        manager.exclude_validator(
            create_test_validator(2),
            ExclusionReason::Equivocation,
            1000,
        );

        // Third offense - permanent
        let v3 = create_test_validator(3);
        manager.exclude_validator(v3.clone(), ExclusionReason::Equivocation, 1000);
        manager.exclude_validator(v3.clone(), ExclusionReason::Equivocation, 2000);
        manager.exclude_validator(v3.clone(), ExclusionReason::Equivocation, 3000);

        let stats = manager.exclusion_stats();
        assert_eq!(stats.total_excluded, 3);
        assert_eq!(stats.permanent_bans, 1);
        assert_eq!(stats.temporary_exclusions, 2);
    }

    #[test]
    fn test_filter_excluded_validators() {
        let mut manager = ByzantineExclusionManager::new(5, 100, 3);

        let v1 = create_test_validator(1);
        let v2 = create_test_validator(2);
        let v3 = create_test_validator(3);

        // Exclude v2
        manager.exclude_validator(v2.clone(), ExclusionReason::Equivocation, 1000);

        let validators = vec![v1.clone(), v2.clone(), v3.clone()];
        let filtered = filter_excluded_validators(validators, &manager, 1000);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&v1));
        assert!(filtered.contains(&v3));
        assert!(!filtered.contains(&v2));
    }
}
