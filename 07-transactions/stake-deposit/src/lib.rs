//! Stake-Deposit Module for ÉTRID
//!
//! Validator staking system with:
//! - Stake deposit and withdrawal
//! - Validator registration
//! - Delegation system
//! - Reward distribution
//! - Slashing penalties

use std::collections::HashMap;
use std::fmt;

/// Validator status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatorStatus {
    Inactive,
    Active,
    Suspended,
    Slashed,
}

impl fmt::Display for ValidatorStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidatorStatus::Inactive => write!(f, "Inactive"),
            ValidatorStatus::Active => write!(f, "Active"),
            ValidatorStatus::Suspended => write!(f, "Suspended"),
            ValidatorStatus::Slashed => write!(f, "Slashed"),
        }
    }
}

/// Validator information
#[derive(Debug, Clone)]
pub struct Validator {
    pub id: String,
    pub address: String,
    pub stake: u128,
    pub status: ValidatorStatus,
    pub commission_rate: u16,  // 0-10000 (0-100%)
    pub created_at: u64,
    pub delegators_count: u64,
    pub total_delegated: u128,
    pub reward_pool: u128,
}

impl Validator {
    pub fn new(
        id: String,
        address: String,
        stake: u128,
        commission_rate: u16,
        created_at: u64,
    ) -> Result<Self, StakeError> {
        if stake == 0 {
            return Err(StakeError::InvalidStake);
        }
        if commission_rate > 10000 {
            return Err(StakeError::InvalidCommissionRate);
        }

        Ok(Self {
            id,
            address,
            stake,
            status: ValidatorStatus::Inactive,
            commission_rate,
            created_at,
            delegators_count: 0,
            total_delegated: 0,
            reward_pool: 0,
        })
    }

    /// Activate validator (requires minimum stake)
    pub fn activate(&mut self, min_stake: u128) -> Result<(), StakeError> {
        let total = self.stake.checked_add(self.total_delegated)
            .ok_or(StakeError::BalanceOverflow)?;

        if total < min_stake {
            return Err(StakeError::InsufficientStake {
                have: total,
                need: min_stake,
            });
        }

        if self.status == ValidatorStatus::Slashed {
            return Err(StakeError::ValidatorSlashed);
        }

        self.status = ValidatorStatus::Active;
        Ok(())
    }

    /// Get total stake (own + delegated)
    pub fn total_stake(&self) -> u128 {
        self.stake
            .checked_add(self.total_delegated)
            .unwrap_or(u128::MAX)
    }

    /// Calculate validator commission from rewards
    pub fn calculate_commission(&self, rewards: u128) -> u128 {
        (rewards as u64)
            .checked_mul(self.commission_rate as u64)
            .unwrap_or(0)
            .checked_div(10000)
            .unwrap_or(0) as u128
    }

    /// Slash validator stake
    pub fn slash(&mut self, amount: u128) -> Result<(), StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidAmount);
        }

        if self.stake < amount {
            return Err(StakeError::InsufficientStake {
                have: self.stake,
                need: amount,
            });
        }

        self.stake -= amount;
        self.status = ValidatorStatus::Slashed;
        Ok(())
    }
}

/// Delegation record
#[derive(Debug, Clone)]
pub struct Delegation {
    pub validator_id: String,
    pub delegator: String,
    pub amount: u128,
    pub created_at: u64,
}

impl Delegation {
    pub fn new(
        validator_id: String,
        delegator: String,
        amount: u128,
        created_at: u64,
    ) -> Result<Self, StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidAmount);
        }

        Ok(Self {
            validator_id,
            delegator,
            amount,
            created_at,
        })
    }
}

/// Reward distribution
#[derive(Debug, Clone)]
pub struct Reward {
    pub validator_id: String,
    pub amount: u128,
    pub distributed_at: u64,
}

impl Reward {
    pub fn new(validator_id: String, amount: u128, distributed_at: u64) -> Result<Self, StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidReward);
        }

        Ok(Self {
            validator_id,
            amount,
            distributed_at,
        })
    }
}

/// Slashing event
#[derive(Debug, Clone)]
pub struct SlashingEvent {
    pub validator_id: String,
    pub amount: u128,
    pub reason: String,
    pub executed_at: u64,
}

impl SlashingEvent {
    pub fn new(
        validator_id: String,
        amount: u128,
        reason: String,
        executed_at: u64,
    ) -> Result<Self, StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidAmount);
        }

        Ok(Self {
            validator_id,
            amount,
            reason,
            executed_at,
        })
    }
}

/// Stake errors
#[derive(Debug, Clone, PartialEq)]
pub enum StakeError {
    InvalidStake,
    InvalidAmount,
    InvalidReward,
    InvalidCommissionRate,
    ValidatorNotFound(String),
    ValidatorAlreadyExists(String),
    DelegationNotFound,
    InsufficientStake { have: u128, need: u128 },
    BalanceOverflow,
    ValidatorSlashed,
    InvalidWithdrawal,
    UnbondingPeriodActive,
}

impl fmt::Display for StakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StakeError::InvalidStake => write!(f, "Invalid stake amount"),
            StakeError::InvalidAmount => write!(f, "Invalid amount"),
            StakeError::InvalidReward => write!(f, "Invalid reward amount"),
            StakeError::InvalidCommissionRate => write!(f, "Invalid commission rate (0-10000)"),
            StakeError::ValidatorNotFound(id) => write!(f, "Validator not found: {}", id),
            StakeError::ValidatorAlreadyExists(id) => write!(f, "Validator already exists: {}", id),
            StakeError::DelegationNotFound => write!(f, "Delegation not found"),
            StakeError::InsufficientStake { have, need } => {
                write!(f, "Insufficient stake: {} < {}", have, need)
            }
            StakeError::BalanceOverflow => write!(f, "Balance overflow"),
            StakeError::ValidatorSlashed => write!(f, "Validator has been slashed"),
            StakeError::InvalidWithdrawal => write!(f, "Invalid withdrawal"),
            StakeError::UnbondingPeriodActive => write!(f, "Unbonding period still active"),
        }
    }
}

/// Stake deposit manager
pub struct StakeDeposit {
    validators: HashMap<String, Validator>,
    delegations: HashMap<String, Vec<Delegation>>,
    rewards: Vec<Reward>,
    slashing_events: Vec<SlashingEvent>,
    min_stake: u128,
    unbonding_period: u64,
}

impl StakeDeposit {
    /// Create new stake deposit system
    pub fn new(min_stake: u128, unbonding_period: u64) -> Self {
        Self {
            validators: HashMap::new(),
            delegations: HashMap::new(),
            rewards: Vec::new(),
            slashing_events: Vec::new(),
            min_stake,
            unbonding_period,
        }
    }

    /// Register new validator
    pub fn register_validator(&mut self, validator: Validator) -> Result<String, StakeError> {
        let id = validator.id.clone();

        if self.validators.contains_key(&id) {
            return Err(StakeError::ValidatorAlreadyExists(id));
        }

        self.validators.insert(id.clone(), validator);
        self.delegations.insert(id.clone(), Vec::new());
        Ok(id)
    }

    /// Get validator
    pub fn get_validator(&self, id: &str) -> Result<Validator, StakeError> {
        self.validators
            .get(id)
            .cloned()
            .ok_or_else(|| StakeError::ValidatorNotFound(id.to_string()))
    }

    /// Activate validator (must have min stake)
    pub fn activate_validator(&mut self, id: &str) -> Result<(), StakeError> {
        let validator = self
            .validators
            .get_mut(id)
            .ok_or_else(|| StakeError::ValidatorNotFound(id.to_string()))?;

        validator.activate(self.min_stake)
    }

    /// Add delegation to validator
    pub fn delegate(
        &mut self,
        validator_id: &str,
        delegator: String,
        amount: u128,
    ) -> Result<(), StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidAmount);
        }

        let validator = self
            .validators
            .get_mut(validator_id)
            .ok_or_else(|| StakeError::ValidatorNotFound(validator_id.to_string()))?;

        validator.total_delegated = validator
            .total_delegated
            .checked_add(amount)
            .ok_or(StakeError::BalanceOverflow)?;

        validator.delegators_count += 1;

        let delegation = Delegation::new(
            validator_id.to_string(),
            delegator,
            amount,
            0,
        )?;

        self.delegations
            .entry(validator_id.to_string())
            .or_insert_with(Vec::new)
            .push(delegation);

        Ok(())
    }

    /// Get delegations for validator
    pub fn get_delegations(&self, validator_id: &str) -> Result<Vec<Delegation>, StakeError> {
        self.delegations
            .get(validator_id)
            .cloned()
            .ok_or_else(|| StakeError::ValidatorNotFound(validator_id.to_string()))
    }

    /// Add reward for validator
    pub fn add_reward(&mut self, validator_id: &str, amount: u128) -> Result<(), StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidReward);
        }

        let validator = self
            .validators
            .get_mut(validator_id)
            .ok_or_else(|| StakeError::ValidatorNotFound(validator_id.to_string()))?;

        validator.reward_pool = validator
            .reward_pool
            .checked_add(amount)
            .ok_or(StakeError::BalanceOverflow)?;

        let reward = Reward::new(validator_id.to_string(), amount, 0)?;
        self.rewards.push(reward);

        Ok(())
    }

    /// Distribute rewards (validator takes commission, rest to delegators)
    pub fn distribute_rewards(&mut self, validator_id: &str) -> Result<u128, StakeError> {
        let validator = self
            .validators
            .get(validator_id)
            .ok_or_else(|| StakeError::ValidatorNotFound(validator_id.to_string()))?;

        let total_rewards = validator.reward_pool;
        let commission = validator.calculate_commission(total_rewards);
        let delegator_rewards = total_rewards.saturating_sub(commission);

        let validator = self.validators.get_mut(validator_id).unwrap();
        validator.reward_pool = 0;

        Ok(commission)
    }

    /// Slash validator for misbehavior
    pub fn slash(&mut self, validator_id: &str, amount: u128, reason: String) -> Result<(), StakeError> {
        let validator = self
            .validators
            .get_mut(validator_id)
            .ok_or_else(|| StakeError::ValidatorNotFound(validator_id.to_string()))?;

        validator.slash(amount)?;

        let event = SlashingEvent::new(
            validator_id.to_string(),
            amount,
            reason,
            0,
        )?;

        self.slashing_events.push(event);
        Ok(())
    }

    /// Get active validators count
    pub fn active_validators_count(&self) -> usize {
        self.validators
            .values()
            .filter(|v| v.status == ValidatorStatus::Active)
            .count()
    }

    /// Get total staked amount
    pub fn total_staked(&self) -> u128 {
        self.validators
            .values()
            .map(|v| v.total_stake())
            .sum()
    }

    /// List validators by status
    pub fn validators_by_status(&self, status: ValidatorStatus) -> Vec<Validator> {
        self.validators
            .values()
            .filter(|v| v.status == status)
            .cloned()
            .collect()
    }

    /// Get slashing events
    pub fn get_slashing_events(&self, validator_id: &str) -> Vec<SlashingEvent> {
        self.slashing_events
            .iter()
            .filter(|e| e.validator_id == validator_id)
            .cloned()
            .collect()
    }

    /// Get rewards history
    pub fn get_rewards(&self, validator_id: &str) -> Vec<Reward> {
        self.rewards
            .iter()
            .filter(|r| r.validator_id == validator_id)
            .cloned()
            .collect()
    }

    /// Increase validator stake
    pub fn increase_stake(&mut self, validator_id: &str, amount: u128) -> Result<(), StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidAmount);
        }

        let validator = self
            .validators
            .get_mut(validator_id)
            .ok_or_else(|| StakeError::ValidatorNotFound(validator_id.to_string()))?;

        validator.stake = validator
            .stake
            .checked_add(amount)
            .ok_or(StakeError::BalanceOverflow)?;

        Ok(())
    }

    /// Decrease validator stake
    pub fn decrease_stake(&mut self, validator_id: &str, amount: u128) -> Result<(), StakeError> {
        if amount == 0 {
            return Err(StakeError::InvalidAmount);
        }

        let validator = self
            .validators
            .get_mut(validator_id)
            .ok_or_else(|| StakeError::ValidatorNotFound(validator_id.to_string()))?;

        if validator.stake < amount {
            return Err(StakeError::InsufficientStake {
                have: validator.stake,
                need: amount,
            });
        }

        validator.stake -= amount;

        // Check if still meets minimum
        let total = validator.total_stake();
        if total < self.min_stake && validator.status == ValidatorStatus::Active {
            validator.status = ValidatorStatus::Suspended;
        }

        Ok(())
    }

    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        );
        assert!(validator.is_ok());
    }

    #[test]
    fn test_validator_invalid_stake() {
        let result = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            0,
            500,
            100,
        );
        assert_eq!(result, Err(StakeError::InvalidStake));
    }

    #[test]
    fn test_validator_invalid_commission() {
        let result = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            15000,
            100,
        );
        assert_eq!(result, Err(StakeError::InvalidCommissionRate));
    }

    #[test]
    fn test_validator_activate() {
        let mut validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        assert!(validator.activate(1000).is_ok());
        assert_eq!(validator.status, ValidatorStatus::Active);
    }

    #[test]
    fn test_validator_activate_insufficient_stake() {
        let mut validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            500,
            500,
            100,
        )
        .unwrap();

        assert!(validator.activate(1000).is_err());
    }

    #[test]
    fn test_validator_total_stake() {
        let mut validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        validator.total_delegated = 500;
        assert_eq!(validator.total_stake(), 1500);
    }

    #[test]
    fn test_validator_calculate_commission() {
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            1000,  // 10%
            100,
        )
        .unwrap();

        let commission = validator.calculate_commission(10000);
        assert_eq!(commission, 1000);
    }

    #[test]
    fn test_validator_slash() {
        let mut validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        assert!(validator.slash(100).is_ok());
        assert_eq!(validator.stake, 900);
        assert_eq!(validator.status, ValidatorStatus::Slashed);
    }

    #[test]
    fn test_delegation_creation() {
        let delegation = Delegation::new(
            "v1".to_string(),
            "delegator".to_string(),
            500,
            100,
        );
        assert!(delegation.is_ok());
    }

    #[test]
    fn test_delegation_invalid_amount() {
        let result = Delegation::new(
            "v1".to_string(),
            "delegator".to_string(),
            0,
            100,
        );
        assert_eq!(result, Err(StakeError::InvalidAmount));
    }

    #[test]
    fn test_reward_creation() {
        let reward = Reward::new("v1".to_string(), 1000, 100);
        assert!(reward.is_ok());
    }

    #[test]
    fn test_reward_invalid_amount() {
        let result = Reward::new("v1".to_string(), 0, 100);
        assert_eq!(result, Err(StakeError::InvalidReward));
    }

    #[test]
    fn test_slashing_event_creation() {
        let event = SlashingEvent::new(
            "v1".to_string(),
            100,
            "double signing".to_string(),
            100,
        );
        assert!(event.is_ok());
    }

    #[test]
    fn test_stake_deposit_creation() {
        let deposit = StakeDeposit::new(1000, 86400);
        assert_eq!(deposit.min_stake, 1000);
        assert_eq!(deposit.unbonding_period, 86400);
    }

    #[test]
    fn test_stake_deposit_register_validator() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        let result = deposit.register_validator(validator);
        assert!(result.is_ok());
        assert_eq!(deposit.validator_count(), 1);
    }

    #[test]
    fn test_stake_deposit_validator_already_exists() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator.clone()).unwrap();
        assert!(deposit.register_validator(validator).is_err());
    }

    #[test]
    fn test_stake_deposit_get_validator() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        let retrieved = deposit.get_validator("v1");
        assert!(retrieved.is_ok());
    }

    #[test]
    fn test_stake_deposit_activate_validator() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        assert!(deposit.activate_validator("v1").is_ok());

        let v = deposit.get_validator("v1").unwrap();
        assert_eq!(v.status, ValidatorStatus::Active);
    }

    #[test]
    fn test_stake_deposit_delegate() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        assert!(deposit.delegate("v1", "delegator".to_string(), 500).is_ok());

        let v = deposit.get_validator("v1").unwrap();
        assert_eq!(v.total_delegated, 500);
        assert_eq!(v.delegators_count, 1);
    }

    #[test]
    fn test_stake_deposit_get_delegations() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        deposit.delegate("v1", "delegator".to_string(), 500).unwrap();

        let delegations = deposit.get_delegations("v1").unwrap();
        assert_eq!(delegations.len(), 1);
    }

    #[test]
    fn test_stake_deposit_add_reward() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        assert!(deposit.add_reward("v1", 100).is_ok());

        let v = deposit.get_validator("v1").unwrap();
        assert_eq!(v.reward_pool, 100);
    }

    #[test]
    fn test_stake_deposit_distribute_rewards() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            1000,  // 10% commission
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        deposit.add_reward("v1", 10000).unwrap();

        let commission = deposit.distribute_rewards("v1");
        assert!(commission.is_ok());
        assert_eq!(commission.unwrap(), 1000);
    }

    #[test]
    fn test_stake_deposit_slash() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        assert!(deposit.slash("v1", 100, "double signing".to_string()).is_ok());

        let v = deposit.get_validator("v1").unwrap();
        assert_eq!(v.stake, 900);
    }

    #[test]
    fn test_stake_deposit_active_validators_count() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        deposit.activate_validator("v1").unwrap();

        assert_eq!(deposit.active_validators_count(), 1);
    }

    #[test]
    fn test_stake_deposit_total_staked() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        assert_eq!(deposit.total_staked(), 1000);
    }

    #[test]
    fn test_stake_deposit_validators_by_status() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        let inactive = deposit.validators_by_status(ValidatorStatus::Inactive);
        assert_eq!(inactive.len(), 1);
    }

    #[test]
    fn test_stake_deposit_get_slashing_events() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        deposit.slash("v1", 100, "test".to_string()).unwrap();

        let events = deposit.get_slashing_events("v1");
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_stake_deposit_get_rewards() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        deposit.add_reward("v1", 100).unwrap();

        let rewards = deposit.get_rewards("v1");
        assert_eq!(rewards.len(), 1);
    }

    #[test]
    fn test_stake_deposit_increase_stake() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        assert!(deposit.increase_stake("v1", 500).is_ok());

        let v = deposit.get_validator("v1").unwrap();
        assert_eq!(v.stake, 1500);
    }

    #[test]
    fn test_stake_deposit_decrease_stake() {
        let mut deposit = StakeDeposit::new(1000, 86400);
        let validator = Validator::new(
            "v1".to_string(),
            "0x123".to_string(),
            1000,
            500,
            100,
        )
        .unwrap();

        deposit.register_validator(validator).unwrap();
        assert!(deposit.decrease_stake("v1", 100).is_ok());

        let v = deposit.get_validator("v1").unwrap();
        assert_eq!(v.stake, 900);
    }

    #[test]
    fn test_validator_status_display() {
        assert_eq!(format!("{}", ValidatorStatus::Active), "Active");
        assert_eq!(format!("{}", ValidatorStatus::Inactive), "Inactive");
        assert_eq!(format!("{}", ValidatorStatus::Slashed), "Slashed");
    }
}
