//! Watchtower Incentive System for Lightning-Bloc
//!
//! Provides economic incentives for watchtowers to monitor payment channels
//! and protect users from fraudulent channel closures.
//!
//! Features:
//! - Watchtower registration with stake requirements
//! - Channel subscription with fee mechanism
//! - Fraud detection and reporting with rewards
//! - Reputation system
//! - Slashing for misbehavior

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
    error::Error,
    vec::Vec,
    string::String,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some},
    default::Default,
};

/// Minimum stake required to register as a watchtower (1000 tokens)
pub const MIN_WATCHTOWER_STAKE: u128 = 1_000_000_000_000_000_000_000; // 1000 ETR

/// Base reward for successfully reporting fraud (100 tokens)
pub const WATCHTOWER_BASE_REWARD: u128 = 100_000_000_000_000_000_000; // 100 ETR

/// Percentage of disputed amount awarded as reward (10%)
pub const FRAUD_REWARD_PERCENTAGE: u8 = 10;

/// Reputation penalty for false reports
pub const FALSE_REPORT_PENALTY: u32 = 50;

/// Reputation bonus for successful fraud detection
pub const FRAUD_DETECTION_BONUS: u32 = 10;

/// Initial reputation score
pub const INITIAL_REPUTATION: u32 = 100;

/// Watchtower information
#[derive(Clone, Debug, PartialEq)]
pub struct WatchtowerInfo {
    pub operator: String,
    pub stake: u128,
    pub reward_pool: u128,
    pub channels_monitored: u32,
    pub disputes_resolved: u32,
    pub reputation_score: u32,
    pub active: bool,
    pub registered_at: u64,
}

impl WatchtowerInfo {
    /// Create new watchtower info
    pub fn new(operator: String, stake: u128, registered_at: u64) -> Result<Self, WatchtowerError> {
        if stake < MIN_WATCHTOWER_STAKE {
            return Err(WatchtowerError::InsufficientStake {
                have: stake,
                required: MIN_WATCHTOWER_STAKE,
            });
        }

        Ok(Self {
            operator,
            stake,
            reward_pool: 0,
            channels_monitored: 0,
            disputes_resolved: 0,
            reputation_score: INITIAL_REPUTATION,
            active: true,
            registered_at,
        })
    }

    /// Add subscription fee to reward pool
    pub fn add_subscription_fee(&mut self, fee: u128) {
        self.reward_pool = self.reward_pool.saturating_add(fee);
        self.channels_monitored = self.channels_monitored.saturating_add(1);
    }

    /// Record successful fraud detection
    pub fn record_fraud_detection(&mut self, reward: u128) {
        self.disputes_resolved = self.disputes_resolved.saturating_add(1);
        self.reputation_score = self.reputation_score.saturating_add(FRAUD_DETECTION_BONUS);
        self.reward_pool = self.reward_pool.saturating_add(reward);
    }

    /// Slash stake and reputation
    pub fn slash(&mut self, amount: u128) -> u128 {
        let actual_slash = amount.min(self.stake);
        self.stake = self.stake.saturating_sub(actual_slash);
        self.reputation_score = self.reputation_score.saturating_sub(FALSE_REPORT_PENALTY);

        // Deactivate if stake too low
        if self.stake < MIN_WATCHTOWER_STAKE {
            self.active = false;
        }

        actual_slash
    }

    /// Check if watchtower can be slashed
    pub fn can_be_slashed(&self) -> bool {
        self.stake > 0
    }

    /// Get channel monitoring capacity (based on stake and reputation)
    pub fn monitoring_capacity(&self) -> u32 {
        // More stake and reputation = more channels can be monitored
        let stake_factor = (self.stake / MIN_WATCHTOWER_STAKE) as u32;
        let reputation_factor = self.reputation_score / 10;
        stake_factor.saturating_mul(reputation_factor).max(10) // Minimum 10 channels
    }
}

/// Channel subscription record
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelSubscription {
    pub channel_id: String,
    pub watchtower: String,
    pub subscriber: String,
    pub fee_paid: u128,
    pub subscribed_at: u64,
    pub active: bool,
}

impl ChannelSubscription {
    pub fn new(
        channel_id: String,
        watchtower: String,
        subscriber: String,
        fee_paid: u128,
        subscribed_at: u64,
    ) -> Self {
        Self {
            channel_id,
            watchtower,
            subscriber,
            fee_paid,
            subscribed_at,
            active: true,
        }
    }

    /// Deactivate subscription
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

/// Fraud report evidence
#[derive(Clone, Debug, PartialEq)]
pub struct FraudEvidence {
    pub channel_id: String,
    pub reported_by: String,
    pub evidence_data: Vec<u8>,
    pub claimed_nonce: u64,
    pub claimed_balance_a: u128,
    pub claimed_balance_b: u128,
    pub timestamp: u64,
}

impl FraudEvidence {
    pub fn new(
        channel_id: String,
        reported_by: String,
        evidence_data: Vec<u8>,
        claimed_nonce: u64,
        claimed_balance_a: u128,
        claimed_balance_b: u128,
        timestamp: u64,
    ) -> Result<Self, WatchtowerError> {
        if evidence_data.is_empty() {
            return Err(WatchtowerError::InvalidEvidence("Evidence cannot be empty".to_string()));
        }
        if evidence_data.len() > 1024 {
            return Err(WatchtowerError::EvidenceTooLarge {
                size: evidence_data.len(),
                max: 1024,
            });
        }

        Ok(Self {
            channel_id,
            reported_by,
            evidence_data,
            claimed_nonce,
            claimed_balance_a,
            claimed_balance_b,
            timestamp,
        })
    }

    /// Verify evidence integrity (simplified validation)
    pub fn verify(&self) -> bool {
        // In production, this would verify signatures, merkle proofs, etc.
        !self.evidence_data.is_empty() && self.evidence_data.len() <= 1024
    }
}

/// Fraud report record
#[derive(Clone, Debug, PartialEq)]
pub struct FraudReport {
    pub report_id: String,
    pub evidence: FraudEvidence,
    pub reward_paid: u128,
    pub resolution: FraudResolution,
    pub resolved_at: Option<u64>,
}

/// Fraud resolution status
#[derive(Clone, Debug, PartialEq)]
pub enum FraudResolution {
    Pending,
    Confirmed { malicious_party: String },
    Rejected { reason: String },
}

impl fmt::Display for FraudResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FraudResolution::Pending => write!(f, "Pending"),
            FraudResolution::Confirmed { malicious_party } => {
                write!(f, "Confirmed (malicious: {})", malicious_party)
            }
            FraudResolution::Rejected { reason } => write!(f, "Rejected: {}", reason),
        }
    }
}

/// Watchtower errors
#[derive(Clone, Debug, PartialEq)]
pub enum WatchtowerError {
    InsufficientStake { have: u128, required: u128 },
    WatchtowerNotRegistered(String),
    WatchtowerInactive(String),
    AlreadyRegistered(String),
    NotSubscribed { channel_id: String, watchtower: String },
    AlreadySubscribed { channel_id: String, watchtower: String },
    InvalidEvidence(String),
    EvidenceTooLarge { size: usize, max: usize },
    InvalidReward(String),
    CapacityExceeded { current: u32, max: u32 },
    SlashAmountTooLarge { amount: u128, available: u128 },
    ChannelNotDisputed(String),
}

impl fmt::Display for WatchtowerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WatchtowerError::InsufficientStake { have, required } => {
                write!(f, "Insufficient stake: {} < {}", have, required)
            }
            WatchtowerError::WatchtowerNotRegistered(addr) => {
                write!(f, "Watchtower not registered: {}", addr)
            }
            WatchtowerError::WatchtowerInactive(addr) => {
                write!(f, "Watchtower inactive: {}", addr)
            }
            WatchtowerError::AlreadyRegistered(addr) => {
                write!(f, "Watchtower already registered: {}", addr)
            }
            WatchtowerError::NotSubscribed { channel_id, watchtower } => {
                write!(f, "Watchtower {} not subscribed to channel {}", watchtower, channel_id)
            }
            WatchtowerError::AlreadySubscribed { channel_id, watchtower } => {
                write!(f, "Watchtower {} already subscribed to channel {}", watchtower, channel_id)
            }
            WatchtowerError::InvalidEvidence(msg) => write!(f, "Invalid evidence: {}", msg),
            WatchtowerError::EvidenceTooLarge { size, max } => {
                write!(f, "Evidence too large: {} > {} bytes", size, max)
            }
            WatchtowerError::InvalidReward(msg) => write!(f, "Invalid reward: {}", msg),
            WatchtowerError::CapacityExceeded { current, max } => {
                write!(f, "Monitoring capacity exceeded: {} >= {}", current, max)
            }
            WatchtowerError::SlashAmountTooLarge { amount, available } => {
                write!(f, "Slash amount too large: {} > {}", amount, available)
            }
            WatchtowerError::ChannelNotDisputed(id) => {
                write!(f, "Channel not disputed: {}", id)
            }
        }
    }
}

#[cfg(feature = "std")]
impl Error for WatchtowerError {}

/// Watchtower manager
pub struct WatchtowerManager {
    watchtowers: HashMap<String, WatchtowerInfo>,
    subscriptions: HashMap<String, Vec<ChannelSubscription>>,
    fraud_reports: HashMap<String, FraudReport>,
}

impl WatchtowerManager {
    /// Create new watchtower manager
    pub fn new() -> Self {
        Self {
            watchtowers: HashMap::new(),
            subscriptions: HashMap::new(),
            fraud_reports: HashMap::new(),
        }
    }

    /// Register a new watchtower
    pub fn register_watchtower(
        &mut self,
        operator: String,
        stake: u128,
        timestamp: u64,
    ) -> Result<(), WatchtowerError> {
        // Check if already registered
        if self.watchtowers.contains_key(&operator) {
            return Err(WatchtowerError::AlreadyRegistered(operator));
        }

        // Create watchtower info
        let info = WatchtowerInfo::new(operator.clone(), stake, timestamp)?;
        self.watchtowers.insert(operator, info);

        Ok(())
    }

    /// Get watchtower info
    pub fn get_watchtower(&self, operator: &str) -> Result<&WatchtowerInfo, WatchtowerError> {
        self.watchtowers
            .get(operator)
            .ok_or_else(|| WatchtowerError::WatchtowerNotRegistered(operator.to_string()))
    }

    /// Get mutable watchtower info
    fn get_watchtower_mut(&mut self, operator: &str) -> Result<&mut WatchtowerInfo, WatchtowerError> {
        self.watchtowers
            .get_mut(operator)
            .ok_or_else(|| WatchtowerError::WatchtowerNotRegistered(operator.to_string()))
    }

    /// Subscribe watchtower to monitor a channel
    pub fn subscribe_watchtower(
        &mut self,
        channel_id: String,
        watchtower: String,
        subscriber: String,
        fee: u128,
        timestamp: u64,
    ) -> Result<(), WatchtowerError> {
        // Verify watchtower exists and is active
        {
            let info = self.get_watchtower(&watchtower)?;
            if !info.active {
                return Err(WatchtowerError::WatchtowerInactive(watchtower.clone()));
            }

            // Check capacity
            let capacity = info.monitoring_capacity();
            if info.channels_monitored >= capacity {
                return Err(WatchtowerError::CapacityExceeded {
                    current: info.channels_monitored,
                    max: capacity,
                });
            }
        }

        // Check if already subscribed
        if let Some(subs) = self.subscriptions.get(&channel_id) {
            if subs.iter().any(|s| s.watchtower == watchtower && s.active) {
                return Err(WatchtowerError::AlreadySubscribed {
                    channel_id: channel_id.clone(),
                    watchtower: watchtower.clone(),
                });
            }
        }

        // Add subscription fee to reward pool (now we can mutate)
        let info = self.get_watchtower_mut(&watchtower)?;
        info.add_subscription_fee(fee);

        // Create subscription record
        let subscription = ChannelSubscription::new(
            channel_id.clone(),
            watchtower,
            subscriber,
            fee,
            timestamp,
        );

        self.subscriptions
            .entry(channel_id)
            .or_insert_with(Vec::new)
            .push(subscription);

        Ok(())
    }

    /// Check if watchtower is subscribed to channel
    pub fn is_subscribed(&self, channel_id: &str, watchtower: &str) -> bool {
        self.subscriptions
            .get(channel_id)
            .map(|subs| subs.iter().any(|s| s.watchtower == watchtower && s.active))
            .unwrap_or(false)
    }

    /// Calculate fraud detection reward
    pub fn calculate_reward(&self, disputed_amount: u128) -> u128 {
        let percentage_reward = disputed_amount
            .saturating_mul(FRAUD_REWARD_PERCENTAGE as u128)
            .saturating_div(100);
        WATCHTOWER_BASE_REWARD.saturating_add(percentage_reward)
    }

    /// Report fraud with evidence
    pub fn report_fraud(
        &mut self,
        evidence: FraudEvidence,
        disputed_amount: u128,
        malicious_party: String,
        timestamp: u64,
    ) -> Result<String, WatchtowerError> {
        let watchtower = evidence.reported_by.clone();
        let channel_id = evidence.channel_id.clone();

        // Verify watchtower is subscribed
        if !self.is_subscribed(&channel_id, &watchtower) {
            return Err(WatchtowerError::NotSubscribed {
                channel_id,
                watchtower,
            });
        }

        // Verify evidence
        if !evidence.verify() {
            return Err(WatchtowerError::InvalidEvidence(
                "Evidence verification failed".to_string(),
            ));
        }

        // Calculate reward
        let reward = self.calculate_reward(disputed_amount);

        // Update watchtower stats
        let info = self.get_watchtower_mut(&watchtower)?;
        info.record_fraud_detection(reward);

        // Create fraud report
        let report_id = format!("fraud_{}_{}_{}", channel_id, watchtower, timestamp);
        let report = FraudReport {
            report_id: report_id.clone(),
            evidence,
            reward_paid: reward,
            resolution: FraudResolution::Confirmed { malicious_party },
            resolved_at: Some(timestamp),
        };

        self.fraud_reports.insert(report_id.clone(), report);

        Ok(report_id)
    }

    /// Slash watchtower for misbehavior
    pub fn slash_watchtower(
        &mut self,
        operator: &str,
        amount: u128,
    ) -> Result<u128, WatchtowerError> {
        let info = self.get_watchtower_mut(operator)?;

        if amount > info.stake {
            return Err(WatchtowerError::SlashAmountTooLarge {
                amount,
                available: info.stake,
            });
        }

        let slashed = info.slash(amount);
        Ok(slashed)
    }

    /// Get all subscriptions for a channel
    pub fn get_channel_subscriptions(&self, channel_id: &str) -> Vec<&ChannelSubscription> {
        self.subscriptions
            .get(channel_id)
            .map(|subs| subs.iter().filter(|s| s.active).collect())
            .unwrap_or_default()
    }

    /// Get all active watchtowers
    pub fn active_watchtowers(&self) -> Vec<&WatchtowerInfo> {
        self.watchtowers
            .values()
            .filter(|w| w.active)
            .collect()
    }

    /// Get fraud report
    pub fn get_fraud_report(&self, report_id: &str) -> Option<&FraudReport> {
        self.fraud_reports.get(report_id)
    }

    /// Get total staked amount across all watchtowers
    pub fn total_staked(&self) -> u128 {
        self.watchtowers.values().map(|w| w.stake).sum()
    }

    /// Get watchtower statistics
    pub fn get_statistics(&self) -> WatchtowerStatistics {
        let total_watchtowers = self.watchtowers.len();
        let active_watchtowers = self.active_watchtowers().len();
        let total_staked = self.total_staked();
        let total_channels_monitored = self.watchtowers.values().map(|w| w.channels_monitored).sum();
        let total_disputes_resolved = self.watchtowers.values().map(|w| w.disputes_resolved).sum();
        let total_fraud_reports = self.fraud_reports.len();

        WatchtowerStatistics {
            total_watchtowers,
            active_watchtowers,
            total_staked,
            total_channels_monitored,
            total_disputes_resolved,
            total_fraud_reports,
        }
    }
}

impl Default for WatchtowerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Watchtower network statistics
#[derive(Clone, Debug, PartialEq)]
pub struct WatchtowerStatistics {
    pub total_watchtowers: usize,
    pub active_watchtowers: usize,
    pub total_staked: u128,
    pub total_channels_monitored: u32,
    pub total_disputes_resolved: u32,
    pub total_fraud_reports: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_timestamp() -> u64 {
        1234567890
    }

    #[test]
    fn test_watchtower_info_creation() {
        let info = WatchtowerInfo::new("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp());
        assert!(info.is_ok());
        let info = info.unwrap();
        assert_eq!(info.operator, "alice");
        assert_eq!(info.stake, MIN_WATCHTOWER_STAKE);
        assert_eq!(info.reputation_score, INITIAL_REPUTATION);
        assert!(info.active);
    }

    #[test]
    fn test_watchtower_info_insufficient_stake() {
        let result = WatchtowerInfo::new("alice".to_string(), 100, mock_timestamp());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WatchtowerError::InsufficientStake { .. }
        ));
    }

    #[test]
    fn test_watchtower_add_subscription_fee() {
        let mut info = WatchtowerInfo::new("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        info.add_subscription_fee(100);
        assert_eq!(info.reward_pool, 100);
        assert_eq!(info.channels_monitored, 1);
    }

    #[test]
    fn test_watchtower_record_fraud_detection() {
        let mut info = WatchtowerInfo::new("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        let initial_reputation = info.reputation_score;
        info.record_fraud_detection(500);
        assert_eq!(info.disputes_resolved, 1);
        assert_eq!(info.reputation_score, initial_reputation + FRAUD_DETECTION_BONUS);
        assert_eq!(info.reward_pool, 500);
    }

    #[test]
    fn test_watchtower_slash() {
        let mut info = WatchtowerInfo::new("alice".to_string(), MIN_WATCHTOWER_STAKE * 2, mock_timestamp()).unwrap();
        let slashed = info.slash(500);
        assert_eq!(slashed, 500);
        assert_eq!(info.stake, MIN_WATCHTOWER_STAKE * 2 - 500);
        assert!(info.active); // Still active because stake is above minimum
    }

    #[test]
    fn test_watchtower_slash_below_minimum() {
        let mut info = WatchtowerInfo::new("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        let slashed = info.slash(MIN_WATCHTOWER_STAKE / 2);
        assert_eq!(slashed, MIN_WATCHTOWER_STAKE / 2);
        assert!(!info.active); // Deactivated because stake is below minimum
    }

    #[test]
    fn test_watchtower_monitoring_capacity() {
        let info = WatchtowerInfo::new("alice".to_string(), MIN_WATCHTOWER_STAKE * 5, mock_timestamp()).unwrap();
        let capacity = info.monitoring_capacity();
        assert!(capacity >= 10); // At least minimum capacity
    }

    #[test]
    fn test_channel_subscription_creation() {
        let sub = ChannelSubscription::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            mock_timestamp(),
        );
        assert_eq!(sub.channel_id, "ch1");
        assert_eq!(sub.watchtower, "alice");
        assert!(sub.active);
    }

    #[test]
    fn test_fraud_evidence_creation() {
        let evidence = FraudEvidence::new(
            "ch1".to_string(),
            "alice".to_string(),
            vec![1, 2, 3, 4],
            10,
            1000,
            1000,
            mock_timestamp(),
        );
        assert!(evidence.is_ok());
    }

    #[test]
    fn test_fraud_evidence_empty() {
        let result = FraudEvidence::new(
            "ch1".to_string(),
            "alice".to_string(),
            vec![],
            10,
            1000,
            1000,
            mock_timestamp(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fraud_evidence_too_large() {
        let large_data = vec![0u8; 2000];
        let result = FraudEvidence::new(
            "ch1".to_string(),
            "alice".to_string(),
            large_data,
            10,
            1000,
            1000,
            mock_timestamp(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fraud_evidence_verify() {
        let evidence = FraudEvidence::new(
            "ch1".to_string(),
            "alice".to_string(),
            vec![1, 2, 3, 4],
            10,
            1000,
            1000,
            mock_timestamp(),
        )
        .unwrap();
        assert!(evidence.verify());
    }

    #[test]
    fn test_watchtower_manager_creation() {
        let manager = WatchtowerManager::new();
        assert_eq!(manager.watchtowers.len(), 0);
    }

    #[test]
    fn test_register_watchtower() {
        let mut manager = WatchtowerManager::new();
        let result = manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp());
        assert!(result.is_ok());
        assert!(manager.watchtowers.contains_key("alice"));
    }

    #[test]
    fn test_register_watchtower_insufficient_stake() {
        let mut manager = WatchtowerManager::new();
        let result = manager.register_watchtower("alice".to_string(), 100, mock_timestamp());
        assert!(result.is_err());
    }

    #[test]
    fn test_register_watchtower_already_registered() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        let result = manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WatchtowerError::AlreadyRegistered(_)
        ));
    }

    #[test]
    fn test_subscribe_watchtower() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();

        let result = manager.subscribe_watchtower(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            mock_timestamp(),
        );
        assert!(result.is_ok());
        assert!(manager.is_subscribed("ch1", "alice"));
    }

    #[test]
    fn test_subscribe_watchtower_not_registered() {
        let mut manager = WatchtowerManager::new();
        let result = manager.subscribe_watchtower(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            mock_timestamp(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_subscribe_watchtower_inactive() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();

        // Deactivate watchtower
        manager.get_watchtower_mut("alice").unwrap().active = false;

        let result = manager.subscribe_watchtower(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            mock_timestamp(),
        );
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WatchtowerError::WatchtowerInactive(_)
        ));
    }

    #[test]
    fn test_calculate_reward() {
        let manager = WatchtowerManager::new();
        let disputed_amount = 10_000;
        let reward = manager.calculate_reward(disputed_amount);
        let expected = WATCHTOWER_BASE_REWARD + (disputed_amount * FRAUD_REWARD_PERCENTAGE as u128) / 100;
        assert_eq!(reward, expected);
    }

    #[test]
    fn test_report_fraud() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        manager.subscribe_watchtower(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            mock_timestamp(),
        ).unwrap();

        let evidence = FraudEvidence::new(
            "ch1".to_string(),
            "alice".to_string(),
            vec![1, 2, 3, 4],
            10,
            1000,
            1000,
            mock_timestamp(),
        )
        .unwrap();

        let result = manager.report_fraud(evidence, 10_000, "bob".to_string(), mock_timestamp());
        assert!(result.is_ok());

        let info = manager.get_watchtower("alice").unwrap();
        assert_eq!(info.disputes_resolved, 1);
    }

    #[test]
    fn test_report_fraud_not_subscribed() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();

        let evidence = FraudEvidence::new(
            "ch1".to_string(),
            "alice".to_string(),
            vec![1, 2, 3, 4],
            10,
            1000,
            1000,
            mock_timestamp(),
        )
        .unwrap();

        let result = manager.report_fraud(evidence, 10_000, "bob".to_string(), mock_timestamp());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WatchtowerError::NotSubscribed { .. }
        ));
    }

    #[test]
    fn test_slash_watchtower() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE * 2, mock_timestamp()).unwrap();

        let result = manager.slash_watchtower("alice", 500);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 500);

        let info = manager.get_watchtower("alice").unwrap();
        assert_eq!(info.stake, MIN_WATCHTOWER_STAKE * 2 - 500);
    }

    #[test]
    fn test_slash_watchtower_too_much() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();

        let result = manager.slash_watchtower("alice", MIN_WATCHTOWER_STAKE * 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_channel_subscriptions() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        manager.subscribe_watchtower(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            mock_timestamp(),
        ).unwrap();

        let subs = manager.get_channel_subscriptions("ch1");
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].watchtower, "alice");
    }

    #[test]
    fn test_active_watchtowers() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        manager.register_watchtower("bob".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();

        let active = manager.active_watchtowers();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_total_staked() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();
        manager.register_watchtower("bob".to_string(), MIN_WATCHTOWER_STAKE * 2, mock_timestamp()).unwrap();

        let total = manager.total_staked();
        assert_eq!(total, MIN_WATCHTOWER_STAKE * 3);
    }

    #[test]
    fn test_get_statistics() {
        let mut manager = WatchtowerManager::new();
        manager.register_watchtower("alice".to_string(), MIN_WATCHTOWER_STAKE, mock_timestamp()).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_watchtowers, 1);
        assert_eq!(stats.active_watchtowers, 1);
        assert_eq!(stats.total_staked, MIN_WATCHTOWER_STAKE);
    }
}
