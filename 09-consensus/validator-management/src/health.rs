//! # Network Health Monitoring
//!
//! This module monitors network health and adjusts consensus parameters
//! based on network conditions (adaptive slot duration, etc.)

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use codec::{Decode, Encode};

use crate::{BlockNumber, ValidatorId};

// ═══════════════════════════════════════════════════════════════════════════════
// HEALTH METRICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Network health score (0-100)
pub type HealthScore = u8;

/// Health status categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub enum HealthStatus {
    /// Critical (0-29)
    Critical = 0,
    /// Poor (30-49)
    Poor = 1,
    /// Degraded (50-69)
    Degraded = 2,
    /// Normal (70-89)
    Normal = 3,
    /// Optimal (90-100)
    Optimal = 4,
}

impl From<HealthScore> for HealthStatus {
    fn from(score: HealthScore) -> Self {
        match score {
            0..=29 => HealthStatus::Critical,
            30..=49 => HealthStatus::Poor,
            50..=69 => HealthStatus::Degraded,
            70..=89 => HealthStatus::Normal,
            _ => HealthStatus::Optimal,
        }
    }
}

impl HealthStatus {
    /// Get adaptive slot duration multiplier
    pub fn slot_duration_multiplier(&self) -> f64 {
        match self {
            HealthStatus::Critical => 3.0,   // 3x base duration
            HealthStatus::Poor => 2.0,       // 2x base duration
            HealthStatus::Degraded => 1.5,   // 1.5x base duration
            HealthStatus::Normal => 1.2,     // 1.2x base duration
            HealthStatus::Optimal => 1.0,    // 1x base duration
        }
    }

    /// Check if network is healthy enough for consensus
    pub fn can_participate(&self) -> bool {
        *self >= HealthStatus::Degraded
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HEALTH MONITOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Monitors network health and consensus metrics
#[derive(Debug, Clone)]
pub struct HealthMonitor {
    /// Current health score (0-100)
    current_score: HealthScore,
    
    /// Historical health scores
    score_history: VecDeque<HealthScore>,
    
    /// Maximum history size
    max_history: usize,
    
    /// Missed blocks in current window
    missed_blocks: u32,
    
    /// Total blocks in window
    total_blocks: u32,
    
    /// Certificate issuance rate (0-100)
    certificate_rate: u8,
    
    /// Network connectivity score (0-100)
    connectivity_score: u8,
    
    /// Validator participation rate (0-100)
    participation_rate: u8,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(max_history: usize) -> Self {
        Self {
            current_score: 100,
            score_history: VecDeque::with_capacity(max_history),
            max_history,
            missed_blocks: 0,
            total_blocks: 0,
            certificate_rate: 100,
            connectivity_score: 100,
            participation_rate: 100,
        }
    }

    /// Update health score based on metrics
    pub fn update(&mut self) {
        // Calculate component scores
        let block_production_score = self.calculate_block_production_score();
        let certificate_score = self.certificate_rate;
        let connectivity = self.connectivity_score;
        let participation = self.participation_rate;
        
        // Weighted average (can be adjusted)
        let new_score = (
            block_production_score as u32 * 25 +
            certificate_score as u32 * 25 +
            connectivity as u32 * 25 +
            participation as u32 * 25
        ) / 100;
        
        self.current_score = new_score as u8;
        
        // Add to history
        if self.score_history.len() >= self.max_history {
            self.score_history.pop_front();
        }
        self.score_history.push_back(self.current_score);
    }

    /// Calculate block production score
    fn calculate_block_production_score(&self) -> u8 {
        if self.total_blocks == 0 {
            return 100;
        }
        
        let produced = self.total_blocks.saturating_sub(self.missed_blocks);
        ((produced * 100) / self.total_blocks) as u8
    }

    /// Record a missed block
    pub fn record_missed_block(&mut self) {
        self.missed_blocks += 1;
        self.total_blocks += 1;
        self.update();
    }

    /// Record a produced block
    pub fn record_produced_block(&mut self) {
        self.total_blocks += 1;
        self.update();
    }

    /// Record block production result (true = produced, false = missed)
    pub fn record_block_production(&mut self, produced: bool) {
        if produced {
            self.record_produced_block();
        } else {
            self.record_missed_block();
        }
    }

    /// Update certificate issuance rate
    pub fn set_certificate_rate(&mut self, rate: u8) {
        self.certificate_rate = rate.min(100);
        self.update();
    }

    /// Update network connectivity score
    pub fn set_connectivity_score(&mut self, score: u8) {
        self.connectivity_score = score.min(100);
        self.update();
    }

    /// Update validator participation rate
    pub fn set_participation_rate(&mut self, rate: u8) {
        self.participation_rate = rate.min(100);
        self.update();
    }

    /// Get current health score
    pub fn current_score(&self) -> HealthScore {
        self.current_score
    }

    /// Get current health status
    pub fn current_status(&self) -> HealthStatus {
        HealthStatus::from(self.current_score)
    }

    /// Get average health score over history
    pub fn average_score(&self) -> HealthScore {
        if self.score_history.is_empty() {
            return self.current_score;
        }
        
        let sum: u32 = self.score_history.iter().map(|&s| s as u32).sum();
        (sum / self.score_history.len() as u32) as u8
    }

    /// Get health trend (positive = improving, negative = degrading)
    pub fn health_trend(&self) -> i16 {
        if self.score_history.len() < 2 {
            return 0;
        }
        
        let recent_avg = self.recent_average(10);
        let older_avg = self.older_average(10);
        
        recent_avg as i16 - older_avg as i16
    }

    /// Get recent average (last N scores)
    fn recent_average(&self, n: usize) -> u8 {
        let len = self.score_history.len();
        if len == 0 {
            return self.current_score;
        }
        
        let start = len.saturating_sub(n);
        let scores: Vec<u8> = self.score_history.iter().skip(start).copied().collect();
        
        if scores.is_empty() {
            return self.current_score;
        }
        
        let sum: u32 = scores.iter().map(|&s| s as u32).sum();
        (sum / scores.len() as u32) as u8
    }

    /// Get older average (before last N scores)
    fn older_average(&self, n: usize) -> u8 {
        let len = self.score_history.len();
        if len <= n {
            return self.current_score;
        }
        
        let end = len.saturating_sub(n);
        let scores: Vec<u8> = self.score_history.iter().take(end).copied().collect();
        
        if scores.is_empty() {
            return self.current_score;
        }
        
        let sum: u32 = scores.iter().map(|&s| s as u32).sum();
        (sum / scores.len() as u32) as u8
    }

    /// Check if network is healthy
    pub fn is_healthy(&self) -> bool {
        self.current_status().can_participate()
    }

    /// Get adaptive slot duration (milliseconds)
    pub fn adaptive_slot_duration(&self, base_duration: u64) -> u64 {
        let multiplier = self.current_status().slot_duration_multiplier();
        (base_duration as f64 * multiplier) as u64
    }

    /// Reset metrics (for new window)
    pub fn reset_window(&mut self) {
        self.missed_blocks = 0;
        self.total_blocks = 0;
    }

    /// Get health statistics
    pub fn get_stats(&self) -> HealthStats {
        HealthStats {
            current_score: self.current_score,
            current_status: self.current_status(),
            average_score: self.average_score(),
            health_trend: self.health_trend(),
            block_production_rate: self.calculate_block_production_score(),
            certificate_rate: self.certificate_rate,
            connectivity_score: self.connectivity_score,
            participation_rate: self.participation_rate,
        }
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new(100) // Keep last 100 scores
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HEALTH STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Health monitoring statistics
#[derive(Debug, Clone)]
pub struct HealthStats {
    /// Current health score (0-100)
    pub current_score: HealthScore,
    
    /// Current health status
    pub current_status: HealthStatus,
    
    /// Average health score
    pub average_score: HealthScore,
    
    /// Health trend (positive = improving)
    pub health_trend: i16,
    
    /// Block production rate (0-100)
    pub block_production_rate: u8,
    
    /// Certificate issuance rate (0-100)
    pub certificate_rate: u8,
    
    /// Network connectivity score (0-100)
    pub connectivity_score: u8,
    
    /// Validator participation rate (0-100)
    pub participation_rate: u8,
}

impl HealthStats {
    /// Check if all metrics are healthy (>= 70)
    pub fn all_metrics_healthy(&self) -> bool {
        self.block_production_rate >= 70
            && self.certificate_rate >= 70
            && self.connectivity_score >= 70
            && self.participation_rate >= 70
    }

    /// Get lowest metric score
    pub fn lowest_metric(&self) -> u8 {
        *[
            self.block_production_rate,
            self.certificate_rate,
            self.connectivity_score,
            self.participation_rate,
        ]
        .iter()
        .min()
        .unwrap()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATOR UPTIME TRACKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Tracks validator uptime and availability
#[derive(Debug, Clone)]
pub struct UptimeTracker {
    /// Validator uptime records
    records: alloc::collections::BTreeMap<ValidatorId, UptimeRecord>,
    
    /// Check interval (in blocks)
    check_interval: u32,
    
    /// Last check block
    last_check: BlockNumber,
}

impl UptimeTracker {
    /// Create a new uptime tracker
    pub fn new(check_interval: u32) -> Self {
        Self {
            records: alloc::collections::BTreeMap::new(),
            check_interval,
            last_check: 0,
        }
    }

    /// Record validator as online
    pub fn record_online(&mut self, validator: ValidatorId, block_number: BlockNumber) {
        let record = self.records.entry(validator).or_insert_with(UptimeRecord::new);
        record.record_online(block_number);
    }

    /// Record validator as offline
    pub fn record_offline(&mut self, validator: &ValidatorId, block_number: BlockNumber) {
        if let Some(record) = self.records.get_mut(validator) {
            record.record_offline(block_number);
        }
    }

    /// Get uptime percentage for validator
    pub fn uptime_percentage(&self, validator: &ValidatorId) -> u8 {
        self.records
            .get(validator)
            .map(|r| r.uptime_percentage())
            .unwrap_or(0)
    }

    /// Check if uptime check is needed
    pub fn should_check(&self, current_block: BlockNumber) -> bool {
        current_block >= self.last_check + self.check_interval as u64
    }

    /// Update last check block
    pub fn update_check(&mut self, block_number: BlockNumber) {
        self.last_check = block_number;
    }
}

/// Uptime record for a single validator
#[derive(Debug, Clone)]
struct UptimeRecord {
    /// Total online checks
    online_count: u32,
    
    /// Total checks
    total_checks: u32,
    
    /// Last seen block
    last_seen: BlockNumber,
}

impl UptimeRecord {
    fn new() -> Self {
        Self {
            online_count: 0,
            total_checks: 0,
            last_seen: 0,
        }
    }

    fn record_online(&mut self, block_number: BlockNumber) {
        self.online_count += 1;
        self.total_checks += 1;
        self.last_seen = block_number;
    }

    fn record_offline(&mut self, _block_number: BlockNumber) {
        self.total_checks += 1;
    }

    fn uptime_percentage(&self) -> u8 {
        if self.total_checks == 0 {
            return 0;
        }
        ((self.online_count * 100) / self.total_checks) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_from_score() {
        assert_eq!(HealthStatus::from(10), HealthStatus::Critical);
        assert_eq!(HealthStatus::from(40), HealthStatus::Poor);
        assert_eq!(HealthStatus::from(60), HealthStatus::Degraded);
        assert_eq!(HealthStatus::from(80), HealthStatus::Normal);
        assert_eq!(HealthStatus::from(95), HealthStatus::Optimal);
    }

    #[test]
    fn test_health_status_multipliers() {
        assert_eq!(HealthStatus::Optimal.slot_duration_multiplier(), 1.0);
        assert_eq!(HealthStatus::Normal.slot_duration_multiplier(), 1.2);
        assert_eq!(HealthStatus::Critical.slot_duration_multiplier(), 3.0);
    }

    #[test]
    fn test_health_monitor_creation() {
        let monitor = HealthMonitor::new(100);
        assert_eq!(monitor.current_score(), 100);
        assert_eq!(monitor.current_status(), HealthStatus::Optimal);
    }

    #[test]
    fn test_block_production_tracking() {
        let mut monitor = HealthMonitor::new(100);
        
        // Record 8 produced, 2 missed (80%)
        for _ in 0..8 {
            monitor.record_produced_block();
        }
        for _ in 0..2 {
            monitor.record_missed_block();
        }
        
        let score = monitor.current_score();
        assert!(score >= 75 && score <= 85); // Should be around 80%
    }

    #[test]
    fn test_health_score_update() {
        let mut monitor = HealthMonitor::new(100);
        
        monitor.set_certificate_rate(50);
        monitor.set_connectivity_score(60);
        monitor.set_participation_rate(70);
        
        let score = monitor.current_score();
        assert!(score < 100); // Should be lower due to metrics
    }

    #[test]
    fn test_health_trend() {
        let mut monitor = HealthMonitor::new(100);
        
        // Add declining scores
        for i in (0..10).rev() {
            monitor.current_score = i * 10;
            monitor.score_history.push_back(i * 10);
        }
        
        let trend = monitor.health_trend();
        assert!(trend < 0); // Should be negative (degrading)
    }

    #[test]
    fn test_adaptive_slot_duration() {
        let mut monitor = HealthMonitor::new(100);
        let base_duration = 6000; // 6 seconds
        
        monitor.current_score = 95; // Optimal
        assert_eq!(monitor.adaptive_slot_duration(base_duration), 6000);
        
        monitor.current_score = 20; // Critical
        assert_eq!(monitor.adaptive_slot_duration(base_duration), 18000); // 3x
    }

    #[test]
    fn test_is_healthy() {
        let mut monitor = HealthMonitor::new(100);
        
        monitor.current_score = 80;
        assert!(monitor.is_healthy());
        
        monitor.current_score = 40;
        assert!(!monitor.is_healthy()); // Below degraded threshold
    }

    #[test]
    fn test_health_stats() {
        let mut monitor = HealthMonitor::new(100);
        
        monitor.set_certificate_rate(80);
        monitor.set_connectivity_score(90);
        monitor.set_participation_rate(85);
        
        let stats = monitor.get_stats();
        assert_eq!(stats.certificate_rate, 80);
        assert_eq!(stats.connectivity_score, 90);
        assert_eq!(stats.participation_rate, 85);
    }

    #[test]
    fn test_uptime_tracker() {
        let mut tracker = UptimeTracker::new(100);
        let validator = ValidatorId::from([1u8; 32]);
        
        // Record 8 online, 2 offline (80%)
        for i in 0..8 {
            tracker.record_online(validator.clone(), i);
        }
        for i in 8..10 {
            tracker.record_offline(&validator, i);
        }
        
        assert_eq!(tracker.uptime_percentage(&validator), 80);
    }

    #[test]
    fn test_uptime_check_interval() {
        let mut tracker = UptimeTracker::new(100);
        
        assert!(tracker.should_check(0));
        tracker.update_check(0);
        
        assert!(!tracker.should_check(50));
        assert!(tracker.should_check(100));
    }
}
