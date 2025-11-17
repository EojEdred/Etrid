//! # Adaptive Slot Timing
//!
//! This module implements adaptive slot duration based on network health.
//! When network is healthy, slots are 6 seconds. When network degrades,
//! slots increase to give validators more time.

use crate::{BASE_SLOT_DURATION, HealthMonitor};

// ═══════════════════════════════════════════════════════════════════════════════
// SLOT TIMER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manages slot timing with adaptive duration
#[derive(Debug, Clone)]
pub struct SlotTimer {
    /// Base slot duration (milliseconds)
    base_duration: u64,
    
    /// Current slot duration (adaptive)
    current_duration: u64,
    
    /// Health monitor for adaptation
    health_monitor: HealthMonitor,
    
    /// Last slot timestamp
    last_slot_time: u64,
    
    /// Current slot number
    current_slot: u64,
}

impl SlotTimer {
    /// Create a new slot timer
    pub fn new(base_duration: u64, health_monitor: HealthMonitor) -> Self {
        Self {
            base_duration,
            current_duration: base_duration,
            health_monitor,
            last_slot_time: 0,
            current_slot: 0,
        }
    }

    /// Update slot duration based on network health
    pub fn update_duration(&mut self) {
        self.current_duration = self.health_monitor.adaptive_slot_duration(self.base_duration);
    }

    /// Get current slot duration
    pub fn current_duration(&self) -> u64 {
        self.current_duration
    }

    /// Get base slot duration
    pub fn base_duration(&self) -> u64 {
        self.base_duration
    }

    /// Get duration multiplier (current / base)
    pub fn duration_multiplier(&self) -> f64 {
        self.current_duration as f64 / self.base_duration as f64
    }

    /// Check if it's time for the next slot
    pub fn is_next_slot(&self, current_time: u64) -> bool {
        if self.last_slot_time == 0 {
            return true; // First slot
        }
        
        current_time >= self.last_slot_time + self.current_duration
    }

    /// Advance to next slot
    pub fn advance_slot(&mut self, current_time: u64) {
        self.current_slot += 1;
        self.last_slot_time = current_time;
        self.update_duration(); // Adapt for next slot
    }

    /// Get current slot number
    pub fn current_slot(&self) -> u64 {
        self.current_slot
    }

    /// Get time until next slot (milliseconds)
    pub fn time_until_next_slot(&self, current_time: u64) -> u64 {
        if self.last_slot_time == 0 {
            return 0; // First slot is immediate
        }
        
        let next_slot_time = self.last_slot_time + self.current_duration;
        if current_time >= next_slot_time {
            return 0;
        }
        
        next_slot_time - current_time
    }

    /// Calculate slot number from timestamp
    pub fn slot_from_timestamp(&self, timestamp: u64, genesis_time: u64) -> u64 {
        if timestamp < genesis_time {
            return 0;
        }
        
        let elapsed = timestamp - genesis_time;
        elapsed / self.current_duration
    }

    /// Calculate timestamp for slot
    pub fn timestamp_for_slot(&self, slot: u64, genesis_time: u64) -> u64 {
        genesis_time + (slot * self.current_duration)
    }

    /// Get health monitor
    pub fn health_monitor(&self) -> &HealthMonitor {
        &self.health_monitor
    }

    /// Get mutable health monitor
    pub fn health_monitor_mut(&mut self) -> &mut HealthMonitor {
        &mut self.health_monitor
    }

    /// Reset slot timer
    pub fn reset(&mut self, start_time: u64) {
        self.current_slot = 0;
        self.last_slot_time = 0;  // Allow first slot to trigger immediately
        self.update_duration();
    }
}

impl Default for SlotTimer {
    fn default() -> Self {
        Self::new(BASE_SLOT_DURATION, HealthMonitor::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLOT WINDOW
// ═══════════════════════════════════════════════════════════════════════════════

/// Represents a time window for a slot
#[derive(Debug, Clone, Copy)]
pub struct SlotWindow {
    /// Slot number
    pub slot: u64,
    
    /// Start time (milliseconds)
    pub start_time: u64,
    
    /// End time (milliseconds)
    pub end_time: u64,
    
    /// Duration (milliseconds)
    pub duration: u64,
}

impl SlotWindow {
    /// Create a new slot window
    pub fn new(slot: u64, start_time: u64, duration: u64) -> Self {
        Self {
            slot,
            start_time,
            end_time: start_time + duration,
            duration,
        }
    }

    /// Check if timestamp is within this slot
    pub fn contains(&self, timestamp: u64) -> bool {
        timestamp >= self.start_time && timestamp < self.end_time
    }

    /// Get time remaining in slot
    pub fn time_remaining(&self, current_time: u64) -> u64 {
        if current_time >= self.end_time {
            return 0;
        }
        self.end_time - current_time
    }

    /// Get time elapsed in slot
    pub fn time_elapsed(&self, current_time: u64) -> u64 {
        if current_time <= self.start_time {
            return 0;
        }
        (current_time - self.start_time).min(self.duration)
    }

    /// Get progress percentage (0-100)
    pub fn progress_percentage(&self, current_time: u64) -> u8 {
        let elapsed = self.time_elapsed(current_time);
        ((elapsed * 100) / self.duration).min(100) as u8
    }

    /// Check if slot has expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time >= self.end_time
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLOT SCHEDULER
// ═══════════════════════════════════════════════════════════════════════════════

/// Schedules future slots
#[derive(Debug, Clone)]
pub struct SlotScheduler {
    /// Base duration
    base_duration: u64,
    
    /// Genesis time (network start)
    genesis_time: u64,
}

impl SlotScheduler {
    /// Create a new slot scheduler
    pub fn new(base_duration: u64, genesis_time: u64) -> Self {
        Self {
            base_duration,
            genesis_time,
        }
    }

    /// Get window for a specific slot
    pub fn get_slot_window(&self, slot: u64, adaptive_duration: u64) -> SlotWindow {
        let start_time = self.genesis_time + (slot * adaptive_duration);
        SlotWindow::new(slot, start_time, adaptive_duration)
    }

    /// Get current slot number
    pub fn current_slot(&self, current_time: u64, adaptive_duration: u64) -> u64 {
        if current_time < self.genesis_time {
            return 0;
        }
        
        let elapsed = current_time - self.genesis_time;
        elapsed / adaptive_duration
    }

    /// Get current slot window
    pub fn current_slot_window(&self, current_time: u64, adaptive_duration: u64) -> SlotWindow {
        let slot = self.current_slot(current_time, adaptive_duration);
        self.get_slot_window(slot, adaptive_duration)
    }

    /// Get genesis time
    pub fn genesis_time(&self) -> u64 {
        self.genesis_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_timer_creation() {
        let timer = SlotTimer::default();
        assert_eq!(timer.base_duration(), BASE_SLOT_DURATION);
        assert_eq!(timer.current_duration(), BASE_SLOT_DURATION);
    }

    #[test]
    fn test_duration_multiplier() {
        let mut health = HealthMonitor::default();
        health.set_connectivity_score(30); // Poor health
        
        let mut timer = SlotTimer::new(BASE_SLOT_DURATION, health);
        timer.update_duration();
        
        let multiplier = timer.duration_multiplier();
        assert!(multiplier > 1.0); // Should be increased
    }

    #[test]
    fn test_is_next_slot() {
        let timer = SlotTimer::default();
        assert!(timer.is_next_slot(0)); // First slot always ready
    }

    #[test]
    fn test_advance_slot() {
        let mut timer = SlotTimer::default();
        
        assert_eq!(timer.current_slot(), 0);
        timer.advance_slot(1000);
        assert_eq!(timer.current_slot(), 1);
        timer.advance_slot(7000);
        assert_eq!(timer.current_slot(), 2);
    }

    #[test]
    fn test_time_until_next_slot() {
        let mut timer = SlotTimer::default();
        
        timer.advance_slot(0);
        assert_eq!(timer.time_until_next_slot(0), BASE_SLOT_DURATION);
        assert_eq!(timer.time_until_next_slot(3000), BASE_SLOT_DURATION - 3000);
        assert_eq!(timer.time_until_next_slot(BASE_SLOT_DURATION), 0);
    }

    #[test]
    fn test_slot_from_timestamp() {
        let timer = SlotTimer::default();
        let genesis = 1000;
        
        assert_eq!(timer.slot_from_timestamp(1000, genesis), 0);
        assert_eq!(timer.slot_from_timestamp(7000, genesis), 1); // 6000ms later
        assert_eq!(timer.slot_from_timestamp(13000, genesis), 2); // 12000ms later
    }

    #[test]
    fn test_timestamp_for_slot() {
        let timer = SlotTimer::default();
        let genesis = 1000;
        
        assert_eq!(timer.timestamp_for_slot(0, genesis), 1000);
        assert_eq!(timer.timestamp_for_slot(1, genesis), 1000 + BASE_SLOT_DURATION);
        assert_eq!(timer.timestamp_for_slot(2, genesis), 1000 + (2 * BASE_SLOT_DURATION));
    }

    #[test]
    fn test_slot_window_contains() {
        let window = SlotWindow::new(0, 1000, 6000);
        
        assert!(window.contains(1000));
        assert!(window.contains(3000));
        assert!(window.contains(6999));
        assert!(!window.contains(7000));
    }

    #[test]
    fn test_slot_window_time_remaining() {
        let window = SlotWindow::new(0, 1000, 6000);
        
        assert_eq!(window.time_remaining(1000), 6000);
        assert_eq!(window.time_remaining(4000), 3000);
        assert_eq!(window.time_remaining(7000), 0);
    }

    #[test]
    fn test_slot_window_progress() {
        let window = SlotWindow::new(0, 1000, 6000);
        
        assert_eq!(window.progress_percentage(1000), 0);
        assert_eq!(window.progress_percentage(4000), 50); // 3000/6000 = 50%
        assert_eq!(window.progress_percentage(7000), 100);
    }

    #[test]
    fn test_slot_window_expired() {
        let window = SlotWindow::new(0, 1000, 6000);
        
        assert!(!window.is_expired(1000));
        assert!(!window.is_expired(6999));
        assert!(window.is_expired(7000));
    }

    #[test]
    fn test_slot_scheduler() {
        let scheduler = SlotScheduler::new(BASE_SLOT_DURATION, 1000);
        
        assert_eq!(scheduler.current_slot(1000, BASE_SLOT_DURATION), 0);
        assert_eq!(scheduler.current_slot(7000, BASE_SLOT_DURATION), 1);
    }

    #[test]
    fn test_slot_scheduler_window() {
        let scheduler = SlotScheduler::new(BASE_SLOT_DURATION, 1000);
        
        let window = scheduler.get_slot_window(0, BASE_SLOT_DURATION);
        assert_eq!(window.slot, 0);
        assert_eq!(window.start_time, 1000);
        assert_eq!(window.duration, BASE_SLOT_DURATION);
    }

    #[test]
    fn test_adaptive_duration_update() {
        let mut health = HealthMonitor::default();
        let mut timer = SlotTimer::new(BASE_SLOT_DURATION, health.clone());
        
        // Initially optimal
        assert_eq!(timer.current_duration(), BASE_SLOT_DURATION);
        
        // Degrade health
        health.set_connectivity_score(20); // Critical
        timer.health_monitor = health;
        timer.update_duration();
        
        // Duration should increase
        assert!(timer.current_duration() > BASE_SLOT_DURATION);
    }

    #[test]
    fn test_reset_timer() {
        let mut timer = SlotTimer::default();

        timer.advance_slot(1000);
        timer.advance_slot(7000);
        assert_eq!(timer.current_slot(), 2);

        timer.reset(10000);
        assert_eq!(timer.current_slot(), 0);
        assert_eq!(timer.last_slot_time, 0);  // First slot should trigger immediately
        assert!(timer.is_next_slot(10000));  // Verify first slot is ready
    }
}
