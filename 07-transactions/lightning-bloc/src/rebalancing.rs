//! Automated Channel Rebalancing
//!
//! Maintains optimal liquidity distribution across Lightning channels
//! through automated circular rebalancing.
//!
//! Features:
//! - Automatic imbalance detection
//! - Circular rebalancing (A → B → C → A)
//! - Fee optimization
//! - Scheduled rebalancing
//! - Manual override options
//! - Rebalancing recommendations

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    vec::Vec,
    string::{String, ToString},
    collections::BTreeMap as HashMap,
    format,
};

#[cfg(feature = "std")]
use std::{
    vec::Vec,
    string::String,
    collections::HashMap,
};

use crate::routing::{Route, Router};

/// Target balance ratio (local:total)
pub const DEFAULT_TARGET_RATIO: f64 = 0.5; // 50/50 split

/// Rebalance threshold (deviation from target)
pub const REBALANCE_THRESHOLD: f64 = 0.2; // 20% deviation

/// Maximum fee for rebalancing (as percentage of amount)
pub const MAX_REBALANCE_FEE_PERCENT: f64 = 0.5; // 0.5%

/// Channel balance information
#[derive(Clone, Debug)]
pub struct ChannelBalance {
    pub channel_id: String,
    pub local_balance: u128,
    pub remote_balance: u128,
    pub capacity: u128,
}

impl ChannelBalance {
    pub fn new(channel_id: String, local_balance: u128, remote_balance: u128) -> Self {
        let capacity = local_balance + remote_balance;
        Self {
            channel_id,
            local_balance,
            remote_balance,
            capacity,
        }
    }

    /// Get local balance ratio
    pub fn local_ratio(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.local_balance as f64 / self.capacity as f64
    }

    /// Check if channel needs rebalancing
    pub fn needs_rebalancing(&self, target_ratio: f64) -> bool {
        let current_ratio = self.local_ratio();
        (current_ratio - target_ratio).abs() > REBALANCE_THRESHOLD
    }

    /// Calculate amount needed to reach target ratio
    pub fn rebalance_amount(&self, target_ratio: f64) -> i128 {
        let target_local = (self.capacity as f64 * target_ratio) as u128;
        target_local as i128 - self.local_balance as i128
    }
}

/// Rebalancing recommendation
#[derive(Clone, Debug)]
pub struct RebalanceRecommendation {
    pub channel_id: String,
    pub current_ratio: f64,
    pub target_ratio: f64,
    pub amount: u128,
    pub direction: RebalanceDirection,
    pub estimated_fee: u128,
    pub priority: Priority,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RebalanceDirection {
    /// Need to increase local balance (receive payment)
    Receive,
    /// Need to decrease local balance (send payment)
    Send,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Circular rebalancing route
#[derive(Clone, Debug)]
pub struct CircularRoute {
    pub channels: Vec<String>,
    pub amount: u128,
    pub total_fee: u128,
}

/// Channel Rebalancer
pub struct ChannelRebalancer {
    /// Target balance ratio
    target_ratio: f64,
    /// Maximum fee percentage
    max_fee_percent: f64,
    /// Router for finding paths
    router: Option<Router>,
    /// Channel balances
    channels: HashMap<String, ChannelBalance>,
}

impl ChannelRebalancer {
    pub fn new(target_ratio: f64, max_fee_percent: f64) -> Self {
        Self {
            target_ratio,
            max_fee_percent,
            router: None,
            channels: HashMap::new(),
        }
    }

    /// Set router
    pub fn set_router(&mut self, router: Router) {
        self.router = Some(router);
    }

    /// Add channel to monitor
    pub fn add_channel(&mut self, balance: ChannelBalance) {
        self.channels.insert(balance.channel_id.clone(), balance);
    }

    /// Analyze all channels and get recommendations
    pub fn analyze_channels(&self) -> Vec<RebalanceRecommendation> {
        let mut recommendations = Vec::new();

        for balance in self.channels.values() {
            if !balance.needs_rebalancing(self.target_ratio) {
                continue;
            }

            let current_ratio = balance.local_ratio();
            let rebalance_amt = balance.rebalance_amount(self.target_ratio);

            let (amount, direction) = if rebalance_amt > 0 {
                (rebalance_amt as u128, RebalanceDirection::Receive)
            } else {
                ((-rebalance_amt) as u128, RebalanceDirection::Send)
            };

            // Estimate fee (simplified)
            let estimated_fee = (amount as f64 * self.max_fee_percent / 100.0) as u128;

            // Determine priority based on deviation
            let deviation = (current_ratio - self.target_ratio).abs();
            let priority = if deviation > 0.4 {
                Priority::Critical
            } else if deviation > 0.3 {
                Priority::High
            } else if deviation > 0.2 {
                Priority::Medium
            } else {
                Priority::Low
            };

            recommendations.push(RebalanceRecommendation {
                channel_id: balance.channel_id.clone(),
                current_ratio,
                target_ratio: self.target_ratio,
                amount,
                direction,
                estimated_fee,
                priority,
            });
        }

        // Sort by priority (highest first)
        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        recommendations
    }

    /// Find circular rebalancing route
    pub fn find_circular_route(
        &self,
        from_channel: &str,
        to_channel: &str,
        amount: u128,
    ) -> Result<CircularRoute, RebalanceError> {
        // In a real implementation, this would use the router to find
        // a path from 'from_channel' back to itself via other channels

        // Simplified version
        if !self.channels.contains_key(from_channel) {
            return Err(RebalanceError::ChannelNotFound);
        }

        if !self.channels.contains_key(to_channel) {
            return Err(RebalanceError::ChannelNotFound);
        }

        // Calculate estimated fee
        let estimated_fee = (amount as f64 * self.max_fee_percent / 100.0) as u128;

        Ok(CircularRoute {
            channels: vec![from_channel.to_string(), to_channel.to_string()],
            amount,
            total_fee: estimated_fee,
        })
    }

    /// Execute rebalancing
    pub fn execute_rebalancing(
        &mut self,
        recommendation: &RebalanceRecommendation,
    ) -> Result<RebalanceResult, RebalanceError> {
        let channel = self.channels.get_mut(&recommendation.channel_id)
            .ok_or(RebalanceError::ChannelNotFound)?;

        // Simulate rebalancing
        match recommendation.direction {
            RebalanceDirection::Receive => {
                channel.local_balance += recommendation.amount;
                channel.remote_balance -= recommendation.amount;
            }
            RebalanceDirection::Send => {
                if channel.local_balance < recommendation.amount {
                    return Err(RebalanceError::InsufficientBalance);
                }
                channel.local_balance -= recommendation.amount;
                channel.remote_balance += recommendation.amount;
            }
        }

        Ok(RebalanceResult {
            channel_id: recommendation.channel_id.clone(),
            amount_moved: recommendation.amount,
            fee_paid: recommendation.estimated_fee,
            new_ratio: channel.local_ratio(),
        })
    }

    /// Auto-rebalance all channels
    pub fn auto_rebalance(&mut self) -> Result<Vec<RebalanceResult>, RebalanceError> {
        let recommendations = self.analyze_channels();
        let mut results = Vec::new();

        for recommendation in recommendations {
            if recommendation.priority >= Priority::High {
                match self.execute_rebalancing(&recommendation) {
                    Ok(result) => results.push(result),
                    Err(_) => continue, // Skip failed rebalancings
                }
            }
        }

        Ok(results)
    }

    /// Get channel balance
    pub fn get_channel(&self, channel_id: &str) -> Option<&ChannelBalance> {
        self.channels.get(channel_id)
    }

    /// Get all channels
    pub fn get_all_channels(&self) -> Vec<&ChannelBalance> {
        self.channels.values().collect()
    }

    /// Get statistics
    pub fn statistics(&self) -> RebalanceStatistics {
        let total_channels = self.channels.len();
        let balanced = self.channels.values()
            .filter(|ch| !ch.needs_rebalancing(self.target_ratio))
            .count();
        let needs_rebalancing = total_channels - balanced;

        let avg_ratio = if total_channels > 0 {
            self.channels.values().map(|ch| ch.local_ratio()).sum::<f64>() / total_channels as f64
        } else {
            0.0
        };

        RebalanceStatistics {
            total_channels,
            balanced_channels: balanced,
            needs_rebalancing,
            average_ratio: avg_ratio,
            target_ratio: self.target_ratio,
        }
    }
}

/// Rebalancing result
#[derive(Clone, Debug)]
pub struct RebalanceResult {
    pub channel_id: String,
    pub amount_moved: u128,
    pub fee_paid: u128,
    pub new_ratio: f64,
}

/// Rebalancing statistics
#[derive(Clone, Debug)]
pub struct RebalanceStatistics {
    pub total_channels: usize,
    pub balanced_channels: usize,
    pub needs_rebalancing: usize,
    pub average_ratio: f64,
    pub target_ratio: f64,
}

/// Rebalancing errors
#[derive(Clone, Debug, PartialEq)]
pub enum RebalanceError {
    ChannelNotFound,
    InsufficientBalance,
    RouteNotFound,
    FeeTooHigh,
    NoChannelsNeedRebalancing,
}

impl core::fmt::Display for RebalanceError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            RebalanceError::ChannelNotFound => write!(f, "Channel not found"),
            RebalanceError::InsufficientBalance => write!(f, "Insufficient balance"),
            RebalanceError::RouteNotFound => write!(f, "Route not found"),
            RebalanceError::FeeTooHigh => write!(f, "Fee too high"),
            RebalanceError::NoChannelsNeedRebalancing => write!(f, "No channels need rebalancing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_balance_creation() {
        let balance = ChannelBalance::new("ch1".to_string(), 5000, 5000);
        assert_eq!(balance.capacity, 10000);
        assert_eq!(balance.local_ratio(), 0.5);
    }

    #[test]
    fn test_needs_rebalancing() {
        let balanced = ChannelBalance::new("ch1".to_string(), 5000, 5000);
        assert!(!balanced.needs_rebalancing(0.5));

        let imbalanced = ChannelBalance::new("ch2".to_string(), 8000, 2000);
        assert!(imbalanced.needs_rebalancing(0.5));
    }

    #[test]
    fn test_rebalance_amount() {
        let balance = ChannelBalance::new("ch1".to_string(), 8000, 2000);
        let amount = balance.rebalance_amount(0.5);
        // Target: 5000, Current: 8000, Need to send 3000
        assert_eq!(amount, -3000);
    }

    #[test]
    fn test_rebalancer_creation() {
        let rebalancer = ChannelRebalancer::new(0.5, 0.5);
        assert_eq!(rebalancer.target_ratio, 0.5);
        assert_eq!(rebalancer.max_fee_percent, 0.5);
    }

    #[test]
    fn test_add_channel() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);
        let balance = ChannelBalance::new("ch1".to_string(), 5000, 5000);

        rebalancer.add_channel(balance);
        assert_eq!(rebalancer.get_all_channels().len(), 1);
    }

    #[test]
    fn test_analyze_channels() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);

        // Balanced channel - should not appear in recommendations
        let balanced = ChannelBalance::new("ch1".to_string(), 5000, 5000);
        rebalancer.add_channel(balanced);

        // Imbalanced channel - should appear
        let imbalanced = ChannelBalance::new("ch2".to_string(), 8000, 2000);
        rebalancer.add_channel(imbalanced);

        let recommendations = rebalancer.analyze_channels();
        assert_eq!(recommendations.len(), 1);
        assert_eq!(recommendations[0].channel_id, "ch2");
    }

    #[test]
    fn test_recommendation_priority() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);

        // Critical imbalance (90% local)
        let critical = ChannelBalance::new("ch1".to_string(), 9000, 1000);
        rebalancer.add_channel(critical);

        // Medium imbalance (70% local)
        let medium = ChannelBalance::new("ch2".to_string(), 7000, 3000);
        rebalancer.add_channel(medium);

        let recommendations = rebalancer.analyze_channels();
        assert_eq!(recommendations.len(), 2);
        // Should be sorted by priority (critical first)
        assert_eq!(recommendations[0].priority, Priority::Critical);
    }

    #[test]
    fn test_execute_rebalancing_receive() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);
        let balance = ChannelBalance::new("ch1".to_string(), 3000, 7000);
        rebalancer.add_channel(balance);

        let recommendation = RebalanceRecommendation {
            channel_id: "ch1".to_string(),
            current_ratio: 0.3,
            target_ratio: 0.5,
            amount: 2000,
            direction: RebalanceDirection::Receive,
            estimated_fee: 10,
            priority: Priority::High,
        };

        let result = rebalancer.execute_rebalancing(&recommendation);
        assert!(result.is_ok());

        let channel = rebalancer.get_channel("ch1").unwrap();
        assert_eq!(channel.local_balance, 5000);
        assert_eq!(channel.remote_balance, 5000);
    }

    #[test]
    fn test_execute_rebalancing_send() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);
        let balance = ChannelBalance::new("ch1".to_string(), 7000, 3000);
        rebalancer.add_channel(balance);

        let recommendation = RebalanceRecommendation {
            channel_id: "ch1".to_string(),
            current_ratio: 0.7,
            target_ratio: 0.5,
            amount: 2000,
            direction: RebalanceDirection::Send,
            estimated_fee: 10,
            priority: Priority::High,
        };

        let result = rebalancer.execute_rebalancing(&recommendation);
        assert!(result.is_ok());

        let channel = rebalancer.get_channel("ch1").unwrap();
        assert_eq!(channel.local_balance, 5000);
        assert_eq!(channel.remote_balance, 5000);
    }

    #[test]
    fn test_auto_rebalance() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);

        // Add critical imbalance
        let critical = ChannelBalance::new("ch1".to_string(), 9000, 1000);
        rebalancer.add_channel(critical);

        // Add low priority imbalance (should be skipped)
        let low = ChannelBalance::new("ch2".to_string(), 6000, 4000);
        rebalancer.add_channel(low);

        let results = rebalancer.auto_rebalance().unwrap();
        // Only critical priority should be rebalanced
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);

        rebalancer.add_channel(ChannelBalance::new("ch1".to_string(), 5000, 5000));
        rebalancer.add_channel(ChannelBalance::new("ch2".to_string(), 8000, 2000));
        rebalancer.add_channel(ChannelBalance::new("ch3".to_string(), 4800, 5200));

        let stats = rebalancer.statistics();
        assert_eq!(stats.total_channels, 3);
        assert_eq!(stats.balanced_channels, 2); // ch1 and ch3 are balanced
        assert_eq!(stats.needs_rebalancing, 1);
    }

    #[test]
    fn test_circular_route_finding() {
        let mut rebalancer = ChannelRebalancer::new(0.5, 0.5);

        rebalancer.add_channel(ChannelBalance::new("ch1".to_string(), 5000, 5000));
        rebalancer.add_channel(ChannelBalance::new("ch2".to_string(), 5000, 5000));

        let route = rebalancer.find_circular_route("ch1", "ch2", 1000);
        assert!(route.is_ok());

        let r = route.unwrap();
        assert_eq!(r.amount, 1000);
        assert!(r.total_fee > 0);
    }
}
