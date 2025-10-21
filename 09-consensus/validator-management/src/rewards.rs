//! # Rewards and Slashing
//!
//! This module handles validator reward distribution and slashing
//! for misbehavior in the consensus protocol.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use codec::{Decode, Encode};

use crate::{Balance, BlockNumber, ValidatorError, ValidatorId, ValidatorResult};

// ═══════════════════════════════════════════════════════════════════════════════
// REWARD TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Reward type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum RewardType {
    /// Block production reward
    BlockProduction,
    
    /// Certificate issuance reward
    CertificateIssuance,
    
    /// Committee participation reward
    CommitteeParticipation,
    
    /// Uptime bonus
    UptimeBonus,
}

/// Reward record
#[derive(Debug, Clone, Encode, Decode)]
pub struct RewardRecord {
    /// Validator who earned reward
    pub validator: ValidatorId,
    
    /// Type of reward
    pub reward_type: RewardType,
    
    /// Reward amount
    pub amount: Balance,
    
    /// Block number when earned
    pub block_number: BlockNumber,
    
    /// Epoch when earned
    pub epoch: u32,
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLASHING TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Slashing reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum SlashingReason {
    /// Double voting (voted for two conflicting blocks)
    DoubleVoting,
    
    /// Invalid certificate
    InvalidCertificate,
    
    /// Unavailability (offline too long)
    Unavailability,
    
    /// Invalid block production
    InvalidBlock,
    
    /// Signature verification failure
    InvalidSignature,
    
    /// Malicious behavior detected
    MaliciousBehavior,
}

impl SlashingReason {
    /// Get slash percentage for this reason (0-100)
    pub fn slash_percentage(&self) -> u8 {
        match self {
            SlashingReason::DoubleVoting => 100,        // Full slash
            SlashingReason::InvalidCertificate => 50,   // Half slash
            SlashingReason::Unavailability => 10,       // Minor slash
            SlashingReason::InvalidBlock => 75,         // Major slash
            SlashingReason::InvalidSignature => 25,     // Moderate slash
            SlashingReason::MaliciousBehavior => 100,   // Full slash
        }
    }

    /// Get reputation penalty (0-100 points)
    pub fn reputation_penalty(&self) -> u64 {
        match self {
            SlashingReason::DoubleVoting => 100,
            SlashingReason::InvalidCertificate => 30,
            SlashingReason::Unavailability => 10,
            SlashingReason::InvalidBlock => 50,
            SlashingReason::InvalidSignature => 20,
            SlashingReason::MaliciousBehavior => 100,
        }
    }
}

/// Slashing record
#[derive(Debug, Clone, Encode, Decode)]
pub struct SlashingRecord {
    /// Validator who was slashed
    pub validator: ValidatorId,
    
    /// Reason for slashing
    pub reason: SlashingReason,
    
    /// Amount slashed
    pub amount: Balance,
    
    /// Block number when slashed
    pub block_number: BlockNumber,
    
    /// Epoch when slashed
    pub epoch: u32,
    
    /// Reporter (if any)
    pub reporter: Option<ValidatorId>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// REWARDS MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manages validator rewards and slashing
#[derive(Debug, Clone)]
pub struct RewardsManager {
    /// Reward history
    reward_history: Vec<RewardRecord>,
    
    /// Slashing history
    slashing_history: Vec<SlashingRecord>,
    
    /// Pending rewards (not yet distributed)
    pending_rewards: BTreeMap<ValidatorId, Balance>,
    
    /// Base reward per block
    base_reward: Balance,
    
    /// Certificate reward multiplier
    certificate_multiplier: u8,
}

impl RewardsManager {
    /// Create a new rewards manager
    pub fn new(base_reward: Balance) -> Self {
        Self {
            reward_history: Vec::new(),
            slashing_history: Vec::new(),
            pending_rewards: BTreeMap::new(),
            base_reward,
            certificate_multiplier: 2, // 2x base reward for certificates
        }
    }

    /// Record a reward
    pub fn record_reward(
        &mut self,
        validator: ValidatorId,
        reward_type: RewardType,
        block_number: BlockNumber,
        epoch: u32,
    ) -> Balance {
        let amount = self.calculate_reward(reward_type);
        
        let record = RewardRecord {
            validator: validator.clone(),
            reward_type,
            amount,
            block_number,
            epoch,
        };
        
        self.reward_history.push(record);
        
        // Add to pending rewards
        *self.pending_rewards.entry(validator).or_insert(0) += amount;
        
        amount
    }

    /// Calculate reward amount based on type
    fn calculate_reward(&self, reward_type: RewardType) -> Balance {
        match reward_type {
            RewardType::BlockProduction => self.base_reward,
            RewardType::CertificateIssuance => {
                self.base_reward * self.certificate_multiplier as Balance
            }
            RewardType::CommitteeParticipation => self.base_reward / 2,
            RewardType::UptimeBonus => self.base_reward / 4,
        }
    }

    /// Record slashing
    pub fn record_slashing(
        &mut self,
        validator: ValidatorId,
        stake: Balance,
        reason: SlashingReason,
        block_number: BlockNumber,
        epoch: u32,
        reporter: Option<ValidatorId>,
    ) -> Balance {
        let slash_percentage = reason.slash_percentage();
        let amount = (stake * slash_percentage as Balance) / 100;
        
        let record = SlashingRecord {
            validator: validator.clone(),
            reason,
            amount,
            block_number,
            epoch,
            reporter,
        };
        
        self.slashing_history.push(record);
        
        // Remove any pending rewards
        self.pending_rewards.remove(&validator);
        
        amount
    }

    /// Claim pending rewards for a validator
    pub fn claim_rewards(&mut self, validator: &ValidatorId) -> ValidatorResult<Balance> {
        let amount = self
            .pending_rewards
            .remove(validator)
            .ok_or(ValidatorError::RewardError)?;
        
        if amount == 0 {
            return Err(ValidatorError::RewardError);
        }
        
        Ok(amount)
    }

    /// Get pending rewards for a validator
    pub fn get_pending_rewards(&self, validator: &ValidatorId) -> Balance {
        self.pending_rewards.get(validator).copied().unwrap_or(0)
    }

    /// Get total rewards earned by a validator
    pub fn total_rewards(&self, validator: &ValidatorId) -> Balance {
        self.reward_history
            .iter()
            .filter(|r| &r.validator == validator)
            .map(|r| r.amount)
            .sum()
    }

    /// Get total slashed from a validator
    pub fn total_slashed(&self, validator: &ValidatorId) -> Balance {
        self.slashing_history
            .iter()
            .filter(|s| &s.validator == validator)
            .map(|s| s.amount)
            .sum()
    }

    /// Get validator performance score (0-100)
    pub fn performance_score(&self, validator: &ValidatorId) -> u8 {
        let rewards = self.total_rewards(validator);
        let slashed = self.total_slashed(validator);
        
        if rewards == 0 && slashed == 0 {
            return 50; // Neutral for no activity
        }
        
        // Score based on rewards vs slashed ratio
        if slashed == 0 {
            return 100; // Perfect if no slashing
        }
        
        let ratio = (rewards * 100) / (rewards + slashed);
        ratio.min(100) as u8
    }

    /// Get reward history for validator
    pub fn get_reward_history(&self, validator: &ValidatorId) -> Vec<&RewardRecord> {
        self.reward_history
            .iter()
            .filter(|r| &r.validator == validator)
            .collect()
    }

    /// Get slashing history for validator
    pub fn get_slashing_history(&self, validator: &ValidatorId) -> Vec<&SlashingRecord> {
        self.slashing_history
            .iter()
            .filter(|s| &s.validator == validator)
            .collect()
    }

    /// Get total pending rewards across all validators
    pub fn total_pending(&self) -> Balance {
        self.pending_rewards.values().sum()
    }

    /// Get validator statistics
    pub fn get_stats(&self, validator: &ValidatorId) -> ValidatorRewardStats {
        ValidatorRewardStats {
            total_rewards: self.total_rewards(validator),
            total_slashed: self.total_slashed(validator),
            pending_rewards: self.get_pending_rewards(validator),
            performance_score: self.performance_score(validator),
            reward_count: self.get_reward_history(validator).len() as u32,
            slash_count: self.get_slashing_history(validator).len() as u32,
        }
    }

    /// Clear old history (keep last N records)
    pub fn prune_history(&mut self, keep_last: usize) {
        if self.reward_history.len() > keep_last {
            let remove = self.reward_history.len() - keep_last;
            self.reward_history.drain(0..remove);
        }
        
        if self.slashing_history.len() > keep_last {
            let remove = self.slashing_history.len() - keep_last;
            self.slashing_history.drain(0..remove);
        }
    }
}

impl Default for RewardsManager {
    fn default() -> Self {
        Self::new(1_000_000_000_000_000_000) // 0.001 ËTR per block
    }
}

/// Type alias for use in coordinator
pub type RewardCalculator = RewardsManager;

// ═══════════════════════════════════════════════════════════════════════════════
// STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Validator reward statistics
#[derive(Debug, Clone, Default)]
pub struct ValidatorRewardStats {
    /// Total rewards earned
    pub total_rewards: Balance,
    
    /// Total amount slashed
    pub total_slashed: Balance,
    
    /// Pending (unclaimed) rewards
    pub pending_rewards: Balance,
    
    /// Performance score (0-100)
    pub performance_score: u8,
    
    /// Number of rewards received
    pub reward_count: u32,
    
    /// Number of times slashed
    pub slash_count: u32,
}

impl ValidatorRewardStats {
    /// Get net rewards (total - slashed)
    pub fn net_rewards(&self) -> Balance {
        self.total_rewards.saturating_sub(self.total_slashed)
    }

    /// Check if validator has good performance (>= 70%)
    pub fn is_good_performer(&self) -> bool {
        self.performance_score >= 70
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_validator(id: u8) -> ValidatorId {
        ValidatorId::from([id; 32])
    }

    #[test]
    fn test_slashing_reason_percentages() {
        assert_eq!(SlashingReason::DoubleVoting.slash_percentage(), 100);
        assert_eq!(SlashingReason::InvalidCertificate.slash_percentage(), 50);
        assert_eq!(SlashingReason::Unavailability.slash_percentage(), 10);
    }

    #[test]
    fn test_slashing_reason_reputation() {
        assert_eq!(SlashingReason::DoubleVoting.reputation_penalty(), 100);
        assert_eq!(SlashingReason::Unavailability.reputation_penalty(), 10);
    }

    #[test]
    fn test_rewards_manager_creation() {
        let manager = RewardsManager::new(1000);
        assert_eq!(manager.base_reward, 1000);
    }

    #[test]
    fn test_record_reward() {
        let mut manager = RewardsManager::new(1000);
        let validator = create_test_validator(1);
        
        let amount = manager.record_reward(
            validator.clone(),
            RewardType::BlockProduction,
            1,
            1,
        );
        
        assert_eq!(amount, 1000);
        assert_eq!(manager.get_pending_rewards(&validator), 1000);
    }

    #[test]
    fn test_reward_calculation() {
        let manager = RewardsManager::new(1000);
        
        assert_eq!(
            manager.calculate_reward(RewardType::BlockProduction),
            1000
        );
        assert_eq!(
            manager.calculate_reward(RewardType::CertificateIssuance),
            2000
        ); // 2x multiplier
        assert_eq!(
            manager.calculate_reward(RewardType::CommitteeParticipation),
            500
        ); // Half
    }

    #[test]
    fn test_record_slashing() {
        let mut manager = RewardsManager::new(1000);
        let validator = create_test_validator(1);
        let stake = 100_000;
        
        let slashed = manager.record_slashing(
            validator.clone(),
            stake,
            SlashingReason::DoubleVoting,
            1,
            1,
            None,
        );
        
        assert_eq!(slashed, 100_000); // 100% slash
        assert_eq!(manager.total_slashed(&validator), 100_000);
    }

    #[test]
    fn test_claim_rewards() {
        let mut manager = RewardsManager::new(1000);
        let validator = create_test_validator(1);
        
        manager.record_reward(validator.clone(), RewardType::BlockProduction, 1, 1);
        manager.record_reward(validator.clone(), RewardType::BlockProduction, 2, 1);
        
        let claimed = manager.claim_rewards(&validator).unwrap();
        assert_eq!(claimed, 2000);
        assert_eq!(manager.get_pending_rewards(&validator), 0);
    }

    #[test]
    fn test_total_rewards() {
        let mut manager = RewardsManager::new(1000);
        let validator = create_test_validator(1);
        
        manager.record_reward(validator.clone(), RewardType::BlockProduction, 1, 1);
        manager.record_reward(validator.clone(), RewardType::CertificateIssuance, 2, 1);
        
        assert_eq!(manager.total_rewards(&validator), 3000); // 1000 + 2000
    }

    #[test]
    fn test_performance_score() {
        let mut manager = RewardsManager::new(1000);
        let validator = create_test_validator(1);
        
        // Only rewards, no slashing
        manager.record_reward(validator.clone(), RewardType::BlockProduction, 1, 1);
        assert_eq!(manager.performance_score(&validator), 100);
        
        // Add slashing
        manager.record_slashing(validator.clone(), 10_000, SlashingReason::Unavailability, 2, 1, None);
        let score = manager.performance_score(&validator);
        assert!(score < 100); // Should be lower due to slashing
    }

    #[test]
    fn test_validator_stats() {
        let mut manager = RewardsManager::new(1000);
        let validator = create_test_validator(1);
        
        manager.record_reward(validator.clone(), RewardType::BlockProduction, 1, 1);
        manager.record_reward(validator.clone(), RewardType::BlockProduction, 2, 1);
        manager.record_slashing(validator.clone(), 10_000, SlashingReason::Unavailability, 3, 1, None);
        
        let stats = manager.get_stats(&validator);
        assert_eq!(stats.total_rewards, 2000);
        assert_eq!(stats.total_slashed, 1000); // 10% of 10_000
        assert_eq!(stats.reward_count, 2);
        assert_eq!(stats.slash_count, 1);
    }

    #[test]
    fn test_slashing_removes_pending_rewards() {
        let mut manager = RewardsManager::new(1000);
        let validator = create_test_validator(1);
        
        manager.record_reward(validator.clone(), RewardType::BlockProduction, 1, 1);
        assert_eq!(manager.get_pending_rewards(&validator), 1000);
        
        manager.record_slashing(validator.clone(), 10_000, SlashingReason::DoubleVoting, 2, 1, None);
        assert_eq!(manager.get_pending_rewards(&validator), 0); // Cleared by slashing
    }

    #[test]
    fn test_total_pending() {
        let mut manager = RewardsManager::new(1000);
        
        manager.record_reward(create_test_validator(1), RewardType::BlockProduction, 1, 1);
        manager.record_reward(create_test_validator(2), RewardType::BlockProduction, 2, 1);
        
        assert_eq!(manager.total_pending(), 2000);
    }
}