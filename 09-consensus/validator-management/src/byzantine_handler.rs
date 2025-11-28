//! Byzantine Behavior Handler
//!
//! Monitors Byzantine detector and executes slashing

use alloc::vec::Vec;
use crate::{
    rewards::{RewardsManager, SlashingReason, StakingInterface},
    ValidatorId, Balance,
};
use asf_algorithm::safety::{ByzantineDetector, SuspicionReason};

/// Handles Byzantine behavior detection and punishment
pub struct ByzantineHandler {
    /// Threshold for automatic slashing
    auto_slash_threshold: u32,
}

impl ByzantineHandler {
    /// Create a new Byzantine handler
    pub fn new(auto_slash_threshold: u32) -> Self {
        Self {
            auto_slash_threshold,
        }
    }

    /// Process Byzantine detections and execute slashing
    pub fn process_detections<S: StakingInterface>(
        &self,
        detector: &ByzantineDetector,
        rewards: &mut RewardsManager,
        staking: &mut S,
    ) -> Vec<(ValidatorId, Balance)> {
        let mut slashed = Vec::new();

        for validator in detector.get_suspected() {
            if let Some(record) = detector.get_record(&validator) {
                // Check if should slash
                if record.incident_count >= self.auto_slash_threshold {
                    // Calculate slash percentage based on severity
                    let slash_pct = self.calculate_slash_percentage(record.incident_count);

                    // Execute slashing
                    if let Ok(amount) = rewards.execute_slashing(
                        validator.clone(),
                        slash_pct,
                        SlashingReason::Byzantine,
                        staking,
                    ) {
                        slashed.push((validator, amount));
                    }
                }
            }
        }

        slashed
    }

    /// Calculate slash percentage based on incident count
    fn calculate_slash_percentage(&self, incident_count: u32) -> u8 {
        match incident_count {
            3..=5 => 10,    // 10% for minor offenses
            6..=10 => 25,   // 25% for moderate offenses
            11..=20 => 50,  // 50% for serious offenses
            _ => 100,       // 100% for severe offenses
        }
    }

    /// Get the auto-slash threshold
    pub fn threshold(&self) -> u32 {
        self.auto_slash_threshold
    }

    /// Set the auto-slash threshold
    pub fn set_threshold(&mut self, threshold: u32) {
        self.auto_slash_threshold = threshold;
    }
}

impl Default for ByzantineHandler {
    fn default() -> Self {
        Self::new(3) // 3 incidents = automatic slashing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    use sp_core::crypto::AccountId32;

    // Mock staking interface for testing
    struct MockStaking {
        stakes: BTreeMap<ValidatorId, Balance>,
    }

    impl MockStaking {
        fn new() -> Self {
            Self {
                stakes: BTreeMap::new(),
            }
        }

        fn set_stake(&mut self, validator: ValidatorId, stake: Balance) {
            self.stakes.insert(validator, stake);
        }
    }

    impl StakingInterface for MockStaking {
        fn get_validator_stake(&self, validator: &ValidatorId) -> Balance {
            self.stakes.get(validator).copied().unwrap_or(0)
        }

        fn slash_validator(&mut self, validator: &ValidatorId, amount: Balance) -> Result<(), &'static str> {
            if let Some(stake) = self.stakes.get_mut(validator) {
                *stake = stake.saturating_sub(amount);
                Ok(())
            } else {
                Err("Validator not found")
            }
        }

        fn is_active(&self, validator: &ValidatorId) -> bool {
            self.stakes.contains_key(validator)
        }
    }

    fn create_test_validator(id: u8) -> ValidatorId {
        ValidatorId::from([id; 32])
    }

    #[test]
    fn test_byzantine_handler_creation() {
        let handler = ByzantineHandler::new(5);
        assert_eq!(handler.threshold(), 5);
    }

    #[test]
    fn test_slash_percentage_calculation() {
        let handler = ByzantineHandler::default();

        assert_eq!(handler.calculate_slash_percentage(3), 10);
        assert_eq!(handler.calculate_slash_percentage(5), 10);
        assert_eq!(handler.calculate_slash_percentage(8), 25);
        assert_eq!(handler.calculate_slash_percentage(15), 50);
        assert_eq!(handler.calculate_slash_percentage(25), 100);
    }

    #[test]
    fn test_process_detections() {
        let handler = ByzantineHandler::new(3);
        let mut detector = ByzantineDetector::default();
        let mut rewards = RewardsManager::default();
        let mut staking = MockStaking::new();

        let validator = create_test_validator(1);
        staking.set_stake(validator.clone(), 100_000);

        // Report 3 incidents (meets threshold)
        detector.report_suspicious(validator.clone(), SuspicionReason::DuplicateVote);
        detector.report_suspicious(validator.clone(), SuspicionReason::ConflictingVotes);
        detector.report_suspicious(validator.clone(), SuspicionReason::InvalidPhase);

        // Process detections
        let slashed = handler.process_detections(&detector, &mut rewards, &mut staking);

        assert_eq!(slashed.len(), 1);
        assert_eq!(slashed[0].0, validator);
        assert_eq!(slashed[0].1, 10_000); // 10% of 100_000

        // Verify stake was actually slashed
        assert_eq!(staking.get_validator_stake(&validator), 90_000);
    }

    #[test]
    fn test_no_slashing_below_threshold() {
        let handler = ByzantineHandler::new(5);
        let mut detector = ByzantineDetector::default();
        let mut rewards = RewardsManager::default();
        let mut staking = MockStaking::new();

        let validator = create_test_validator(1);
        staking.set_stake(validator.clone(), 100_000);

        // Report only 2 incidents (below threshold)
        detector.report_suspicious(validator.clone(), SuspicionReason::DuplicateVote);
        detector.report_suspicious(validator.clone(), SuspicionReason::ConflictingVotes);

        // Process detections
        let slashed = handler.process_detections(&detector, &mut rewards, &mut staking);

        assert_eq!(slashed.len(), 0);
        assert_eq!(staking.get_validator_stake(&validator), 100_000); // No slashing
    }
}
