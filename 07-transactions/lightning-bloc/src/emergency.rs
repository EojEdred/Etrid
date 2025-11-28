//! Emergency Withdrawal and Timeout Handling for Lightning-Bloc
//!
//! Provides safety mechanisms for users to recover funds in case of unresponsive
//! counterparties, network failures, or other emergency scenarios.

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
    option::Option::{self, Some},
    default::Default,
};

/// Timeout period for counterparty response (24 hours)
pub const COUNTERPARTY_TIMEOUT: u64 = 24 * 60 * 60;

/// Grace period before forced closure (48 hours)
pub const FORCED_CLOSURE_GRACE_PERIOD: u64 = 48 * 60 * 60;

/// Maximum withdrawal queue size per channel
pub const MAX_WITHDRAWAL_QUEUE: usize = 100;

/// Emergency withdrawal request
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawalRequest {
    pub request_id: String,
    pub channel_id: String,
    pub requester: String,
    pub amount: u128,
    pub request_time: u64,
    pub timeout_deadline: u64,
    pub status: WithdrawalStatus,
}

impl WithdrawalRequest {
    /// Create new withdrawal request
    pub fn new(
        channel_id: String,
        requester: String,
        amount: u128,
        request_time: u64,
    ) -> Self {
        let request_id = format!("withdrawal_{}_{}_{}", channel_id, requester, request_time);
        let timeout_deadline = request_time + COUNTERPARTY_TIMEOUT;

        Self {
            request_id,
            channel_id,
            requester,
            amount,
            request_time,
            timeout_deadline,
            status: WithdrawalStatus::Pending,
        }
    }

    /// Check if withdrawal timeout has expired
    pub fn is_timeout_expired(&self, current_time: u64) -> bool {
        current_time > self.timeout_deadline
    }

    /// Approve withdrawal
    pub fn approve(&mut self, approver: String, timestamp: u64) -> Result<(), EmergencyError> {
        if !matches!(self.status, WithdrawalStatus::Pending) {
            return Err(EmergencyError::InvalidWithdrawalStatus {
                current: self.status.clone(),
            });
        }

        self.status = WithdrawalStatus::Approved {
            approver,
            approved_at: timestamp,
        };
        Ok(())
    }

    /// Reject withdrawal
    pub fn reject(&mut self, reason: String, timestamp: u64) -> Result<(), EmergencyError> {
        if !matches!(self.status, WithdrawalStatus::Pending) {
            return Err(EmergencyError::InvalidWithdrawalStatus {
                current: self.status.clone(),
            });
        }

        self.status = WithdrawalStatus::Rejected {
            reason,
            rejected_at: timestamp,
        };
        Ok(())
    }

    /// Force execute withdrawal after timeout
    pub fn force_execute(&mut self, timestamp: u64) -> Result<(), EmergencyError> {
        if !self.is_timeout_expired(timestamp) {
            return Err(EmergencyError::TimeoutNotExpired {
                deadline: self.timeout_deadline,
                current: timestamp,
            });
        }

        if !matches!(self.status, WithdrawalStatus::Pending) {
            return Err(EmergencyError::InvalidWithdrawalStatus {
                current: self.status.clone(),
            });
        }

        self.status = WithdrawalStatus::ForceExecuted {
            executed_at: timestamp,
        };
        Ok(())
    }
}

/// Withdrawal status
#[derive(Clone, Debug, PartialEq)]
pub enum WithdrawalStatus {
    Pending,
    Approved { approver: String, approved_at: u64 },
    Rejected { reason: String, rejected_at: u64 },
    ForceExecuted { executed_at: u64 },
    Completed { completed_at: u64 },
}

impl fmt::Display for WithdrawalStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WithdrawalStatus::Pending => write!(f, "Pending"),
            WithdrawalStatus::Approved { approver, .. } => {
                write!(f, "Approved by {}", approver)
            }
            WithdrawalStatus::Rejected { reason, .. } => write!(f, "Rejected: {}", reason),
            WithdrawalStatus::ForceExecuted { .. } => write!(f, "Force Executed"),
            WithdrawalStatus::Completed { .. } => write!(f, "Completed"),
        }
    }
}

/// Forced channel closure request
#[derive(Clone, Debug, PartialEq)]
pub struct ForcedClosureRequest {
    pub closure_id: String,
    pub channel_id: String,
    pub initiator: String,
    pub last_known_state: ChannelState,
    pub request_time: u64,
    pub grace_deadline: u64,
    pub status: ClosureStatus,
}

impl ForcedClosureRequest {
    /// Create new forced closure request
    pub fn new(
        channel_id: String,
        initiator: String,
        last_known_state: ChannelState,
        request_time: u64,
    ) -> Self {
        let closure_id = format!("closure_{}_{}_{}", channel_id, initiator, request_time);
        let grace_deadline = request_time + FORCED_CLOSURE_GRACE_PERIOD;

        Self {
            closure_id,
            channel_id,
            initiator,
            last_known_state,
            request_time,
            grace_deadline,
            status: ClosureStatus::Pending,
        }
    }

    /// Check if grace period has expired
    pub fn is_grace_expired(&self, current_time: u64) -> bool {
        current_time > self.grace_deadline
    }

    /// Challenge forced closure with newer state
    pub fn challenge(
        &mut self,
        challenger: String,
        newer_state: ChannelState,
        timestamp: u64,
    ) -> Result<(), EmergencyError> {
        if !matches!(self.status, ClosureStatus::Pending) {
            return Err(EmergencyError::InvalidClosureStatus {
                current: self.status.clone(),
            });
        }

        // Verify newer state has higher nonce
        if newer_state.nonce <= self.last_known_state.nonce {
            return Err(EmergencyError::InvalidStateNonce {
                current: self.last_known_state.nonce,
                claimed: newer_state.nonce,
            });
        }

        self.status = ClosureStatus::Challenged {
            challenger,
            newer_state,
            challenged_at: timestamp,
        };
        Ok(())
    }

    /// Finalize forced closure
    pub fn finalize(&mut self, timestamp: u64) -> Result<(), EmergencyError> {
        if !self.is_grace_expired(timestamp) {
            return Err(EmergencyError::GracePeriodNotExpired {
                deadline: self.grace_deadline,
                current: timestamp,
            });
        }

        // Use challenged state if available, otherwise use initiator's state
        let final_state = match &self.status {
            ClosureStatus::Challenged { newer_state, .. } => newer_state.clone(),
            _ => self.last_known_state.clone(),
        };

        self.status = ClosureStatus::Finalized {
            final_state,
            finalized_at: timestamp,
        };
        Ok(())
    }
}

/// Channel state snapshot
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelState {
    pub nonce: u64,
    pub balance_a: u128,
    pub balance_b: u128,
    pub signatures: Vec<Vec<u8>>,
}

/// Closure status
#[derive(Clone, Debug, PartialEq)]
pub enum ClosureStatus {
    Pending,
    Challenged {
        challenger: String,
        newer_state: ChannelState,
        challenged_at: u64,
    },
    Finalized {
        final_state: ChannelState,
        finalized_at: u64,
    },
}

impl fmt::Display for ClosureStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClosureStatus::Pending => write!(f, "Pending"),
            ClosureStatus::Challenged { challenger, .. } => {
                write!(f, "Challenged by {}", challenger)
            }
            ClosureStatus::Finalized { .. } => write!(f, "Finalized"),
        }
    }
}

/// Timeout watchdog for monitoring channel activity
#[derive(Clone, Debug, PartialEq)]
pub struct TimeoutWatchdog {
    pub channel_id: String,
    pub last_activity: u64,
    pub timeout_threshold: u64,
    pub warnings_sent: u32,
}

impl TimeoutWatchdog {
    /// Create new timeout watchdog
    pub fn new(channel_id: String, last_activity: u64, timeout_threshold: u64) -> Self {
        Self {
            channel_id,
            last_activity,
            timeout_threshold,
            warnings_sent: 0,
        }
    }

    /// Check if channel is inactive
    pub fn is_inactive(&self, current_time: u64) -> bool {
        let inactive_duration = current_time.saturating_sub(self.last_activity);
        inactive_duration > self.timeout_threshold
    }

    /// Update activity timestamp
    pub fn update_activity(&mut self, timestamp: u64) {
        self.last_activity = timestamp;
    }

    /// Record warning sent
    pub fn record_warning(&mut self) {
        self.warnings_sent += 1;
    }
}

/// Emergency system manager
pub struct EmergencySystem {
    withdrawal_requests: HashMap<String, WithdrawalRequest>,
    closure_requests: HashMap<String, ForcedClosureRequest>,
    watchdogs: HashMap<String, TimeoutWatchdog>,
}

impl EmergencySystem {
    /// Create new emergency system
    pub fn new() -> Self {
        Self {
            withdrawal_requests: HashMap::new(),
            closure_requests: HashMap::new(),
            watchdogs: HashMap::new(),
        }
    }

    /// Submit emergency withdrawal request
    pub fn request_withdrawal(
        &mut self,
        channel_id: String,
        requester: String,
        amount: u128,
        timestamp: u64,
    ) -> Result<String, EmergencyError> {
        // Check queue size
        let channel_withdrawals = self.withdrawal_requests
            .values()
            .filter(|w| w.channel_id == channel_id && matches!(w.status, WithdrawalStatus::Pending))
            .count();

        if channel_withdrawals >= MAX_WITHDRAWAL_QUEUE {
            return Err(EmergencyError::WithdrawalQueueFull {
                channel_id,
                max: MAX_WITHDRAWAL_QUEUE,
            });
        }

        let request = WithdrawalRequest::new(channel_id, requester, amount, timestamp);
        let request_id = request.request_id.clone();

        self.withdrawal_requests.insert(request_id.clone(), request);
        Ok(request_id)
    }

    /// Approve withdrawal request
    pub fn approve_withdrawal(
        &mut self,
        request_id: &str,
        approver: String,
        timestamp: u64,
    ) -> Result<(), EmergencyError> {
        let request = self.withdrawal_requests
            .get_mut(request_id)
            .ok_or_else(|| EmergencyError::WithdrawalNotFound(request_id.to_string()))?;

        request.approve(approver, timestamp)
    }

    /// Force execute expired withdrawal requests
    pub fn execute_expired_withdrawals(&mut self, current_time: u64) -> Vec<String> {
        let mut executed = Vec::new();

        for (id, request) in self.withdrawal_requests.iter_mut() {
            if request.is_timeout_expired(current_time) && matches!(request.status, WithdrawalStatus::Pending) {
                if request.force_execute(current_time).is_ok() {
                    executed.push(id.clone());
                }
            }
        }

        executed
    }

    /// Request forced channel closure
    pub fn request_forced_closure(
        &mut self,
        channel_id: String,
        initiator: String,
        last_known_state: ChannelState,
        timestamp: u64,
    ) -> Result<String, EmergencyError> {
        let request = ForcedClosureRequest::new(channel_id, initiator, last_known_state, timestamp);
        let closure_id = request.closure_id.clone();

        self.closure_requests.insert(closure_id.clone(), request);
        Ok(closure_id)
    }

    /// Challenge forced closure with newer state
    pub fn challenge_forced_closure(
        &mut self,
        closure_id: &str,
        challenger: String,
        newer_state: ChannelState,
        timestamp: u64,
    ) -> Result<(), EmergencyError> {
        let request = self.closure_requests
            .get_mut(closure_id)
            .ok_or_else(|| EmergencyError::ClosureNotFound(closure_id.to_string()))?;

        request.challenge(challenger, newer_state, timestamp)
    }

    /// Finalize expired forced closures
    pub fn finalize_expired_closures(&mut self, current_time: u64) -> Vec<String> {
        let mut finalized = Vec::new();

        for (id, request) in self.closure_requests.iter_mut() {
            if request.is_grace_expired(current_time) && matches!(request.status, ClosureStatus::Pending | ClosureStatus::Challenged { .. }) {
                if request.finalize(current_time).is_ok() {
                    finalized.push(id.clone());
                }
            }
        }

        finalized
    }

    /// Register timeout watchdog for channel
    pub fn register_watchdog(
        &mut self,
        channel_id: String,
        timeout_threshold: u64,
        timestamp: u64,
    ) {
        let watchdog = TimeoutWatchdog::new(channel_id.clone(), timestamp, timeout_threshold);
        self.watchdogs.insert(channel_id, watchdog);
    }

    /// Update channel activity
    pub fn update_channel_activity(&mut self, channel_id: &str, timestamp: u64) {
        if let Some(watchdog) = self.watchdogs.get_mut(channel_id) {
            watchdog.update_activity(timestamp);
        }
    }

    /// Get inactive channels
    pub fn get_inactive_channels(&self, current_time: u64) -> Vec<String> {
        self.watchdogs
            .iter()
            .filter(|(_, w)| w.is_inactive(current_time))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get withdrawal request
    pub fn get_withdrawal(&self, request_id: &str) -> Option<&WithdrawalRequest> {
        self.withdrawal_requests.get(request_id)
    }

    /// Get closure request
    pub fn get_closure(&self, closure_id: &str) -> Option<&ForcedClosureRequest> {
        self.closure_requests.get(closure_id)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> EmergencyStatistics {
        let total_withdrawals = self.withdrawal_requests.len();
        let pending_withdrawals = self.withdrawal_requests
            .values()
            .filter(|w| matches!(w.status, WithdrawalStatus::Pending))
            .count();

        let total_closures = self.closure_requests.len();
        let pending_closures = self.closure_requests
            .values()
            .filter(|c| matches!(c.status, ClosureStatus::Pending))
            .count();

        let monitored_channels = self.watchdogs.len();

        EmergencyStatistics {
            total_withdrawals,
            pending_withdrawals,
            total_closures,
            pending_closures,
            monitored_channels,
        }
    }
}

impl Default for EmergencySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Emergency system statistics
#[derive(Clone, Debug, PartialEq)]
pub struct EmergencyStatistics {
    pub total_withdrawals: usize,
    pub pending_withdrawals: usize,
    pub total_closures: usize,
    pub pending_closures: usize,
    pub monitored_channels: usize,
}

/// Emergency system errors
#[derive(Clone, Debug, PartialEq)]
pub enum EmergencyError {
    InvalidWithdrawalStatus { current: WithdrawalStatus },
    InvalidClosureStatus { current: ClosureStatus },
    TimeoutNotExpired { deadline: u64, current: u64 },
    GracePeriodNotExpired { deadline: u64, current: u64 },
    WithdrawalNotFound(String),
    ClosureNotFound(String),
    WithdrawalQueueFull { channel_id: String, max: usize },
    InvalidStateNonce { current: u64, claimed: u64 },
}

impl fmt::Display for EmergencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmergencyError::InvalidWithdrawalStatus { current } => {
                write!(f, "Invalid withdrawal status: {}", current)
            }
            EmergencyError::InvalidClosureStatus { current } => {
                write!(f, "Invalid closure status: {}", current)
            }
            EmergencyError::TimeoutNotExpired { deadline, current } => {
                write!(f, "Timeout not expired: deadline {}, current {}", deadline, current)
            }
            EmergencyError::GracePeriodNotExpired { deadline, current } => {
                write!(f, "Grace period not expired: deadline {}, current {}", deadline, current)
            }
            EmergencyError::WithdrawalNotFound(id) => write!(f, "Withdrawal not found: {}", id),
            EmergencyError::ClosureNotFound(id) => write!(f, "Closure not found: {}", id),
            EmergencyError::WithdrawalQueueFull { channel_id, max } => {
                write!(f, "Withdrawal queue full for channel {}: max {}", channel_id, max)
            }
            EmergencyError::InvalidStateNonce { current, claimed } => {
                write!(f, "Invalid state nonce: current {}, claimed {}", current, claimed)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_channel_state(nonce: u64) -> ChannelState {
        ChannelState {
            nonce,
            balance_a: 1000,
            balance_b: 1000,
            signatures: vec![vec![1, 2, 3], vec![4, 5, 6]],
        }
    }

    #[test]
    fn test_withdrawal_request_creation() {
        let request = WithdrawalRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            100,
            1000,
        );
        assert_eq!(request.channel_id, "ch1");
        assert!(matches!(request.status, WithdrawalStatus::Pending));
    }

    #[test]
    fn test_withdrawal_timeout() {
        let request = WithdrawalRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            100,
            1000,
        );

        assert!(!request.is_timeout_expired(1000 + COUNTERPARTY_TIMEOUT - 1));
        assert!(request.is_timeout_expired(1000 + COUNTERPARTY_TIMEOUT + 1));
    }

    #[test]
    fn test_withdrawal_approval() {
        let mut request = WithdrawalRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            100,
            1000,
        );

        assert!(request.approve("bob".to_string(), 1100).is_ok());
        assert!(matches!(request.status, WithdrawalStatus::Approved { .. }));
    }

    #[test]
    fn test_withdrawal_force_execute() {
        let mut request = WithdrawalRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            100,
            1000,
        );

        let force_time = 1000 + COUNTERPARTY_TIMEOUT + 1;
        assert!(request.force_execute(force_time).is_ok());
        assert!(matches!(request.status, WithdrawalStatus::ForceExecuted { .. }));
    }

    #[test]
    fn test_forced_closure_creation() {
        let state = create_test_channel_state(5);
        let request = ForcedClosureRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            state,
            1000,
        );

        assert_eq!(request.channel_id, "ch1");
        assert!(matches!(request.status, ClosureStatus::Pending));
    }

    #[test]
    fn test_forced_closure_grace_period() {
        let state = create_test_channel_state(5);
        let request = ForcedClosureRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            state,
            1000,
        );

        assert!(!request.is_grace_expired(1000 + FORCED_CLOSURE_GRACE_PERIOD - 1));
        assert!(request.is_grace_expired(1000 + FORCED_CLOSURE_GRACE_PERIOD + 1));
    }

    #[test]
    fn test_forced_closure_challenge() {
        let state = create_test_channel_state(5);
        let mut request = ForcedClosureRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            state,
            1000,
        );

        let newer_state = create_test_channel_state(10);
        assert!(request.challenge("bob".to_string(), newer_state, 1100).is_ok());
        assert!(matches!(request.status, ClosureStatus::Challenged { .. }));
    }

    #[test]
    fn test_forced_closure_invalid_challenge() {
        let state = create_test_channel_state(10);
        let mut request = ForcedClosureRequest::new(
            "ch1".to_string(),
            "alice".to_string(),
            state,
            1000,
        );

        let older_state = create_test_channel_state(5);
        assert!(request.challenge("bob".to_string(), older_state, 1100).is_err());
    }

    #[test]
    fn test_timeout_watchdog() {
        let watchdog = TimeoutWatchdog::new("ch1".to_string(), 1000, 3600);

        assert!(!watchdog.is_inactive(1000 + 3599));
        assert!(watchdog.is_inactive(1000 + 3601));
    }

    #[test]
    fn test_emergency_system_request_withdrawal() {
        let mut system = EmergencySystem::new();
        let result = system.request_withdrawal(
            "ch1".to_string(),
            "alice".to_string(),
            100,
            1000,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_emergency_system_approve_withdrawal() {
        let mut system = EmergencySystem::new();
        let request_id = system.request_withdrawal(
            "ch1".to_string(),
            "alice".to_string(),
            100,
            1000,
        )
        .unwrap();

        assert!(system.approve_withdrawal(&request_id, "bob".to_string(), 1100).is_ok());
    }

    #[test]
    fn test_execute_expired_withdrawals() {
        let mut system = EmergencySystem::new();
        system.request_withdrawal(
            "ch1".to_string(),
            "alice".to_string(),
            100,
            1000,
        )
        .unwrap();

        let force_time = 1000 + COUNTERPARTY_TIMEOUT + 1;
        let executed = system.execute_expired_withdrawals(force_time);

        assert_eq!(executed.len(), 1);
    }

    #[test]
    fn test_emergency_system_forced_closure() {
        let mut system = EmergencySystem::new();
        let state = create_test_channel_state(5);

        let result = system.request_forced_closure(
            "ch1".to_string(),
            "alice".to_string(),
            state,
            1000,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_finalize_expired_closures() {
        let mut system = EmergencySystem::new();
        let state = create_test_channel_state(5);

        system.request_forced_closure(
            "ch1".to_string(),
            "alice".to_string(),
            state,
            1000,
        )
        .unwrap();

        let finalize_time = 1000 + FORCED_CLOSURE_GRACE_PERIOD + 1;
        let finalized = system.finalize_expired_closures(finalize_time);

        assert_eq!(finalized.len(), 1);
    }

    #[test]
    fn test_register_watchdog() {
        let mut system = EmergencySystem::new();
        system.register_watchdog("ch1".to_string(), 3600, 1000);

        assert_eq!(system.watchdogs.len(), 1);
    }

    #[test]
    fn test_get_inactive_channels() {
        let mut system = EmergencySystem::new();
        system.register_watchdog("ch1".to_string(), 3600, 1000);

        // Should be inactive after timeout
        let inactive = system.get_inactive_channels(1000 + 3601);
        assert_eq!(inactive.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut system = EmergencySystem::new();

        system.request_withdrawal("ch1".to_string(), "alice".to_string(), 100, 1000).unwrap();
        let state = create_test_channel_state(5);
        system.request_forced_closure("ch1".to_string(), "alice".to_string(), state, 1000).unwrap();
        system.register_watchdog("ch1".to_string(), 3600, 1000);

        let stats = system.get_statistics();
        assert_eq!(stats.total_withdrawals, 1);
        assert_eq!(stats.total_closures, 1);
        assert_eq!(stats.monitored_channels, 1);
    }
}
