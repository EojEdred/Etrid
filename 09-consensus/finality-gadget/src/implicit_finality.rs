//! Implicit Finality Tracker
//!
//! Provides probabilistic finality based on chain depth, similar to Bitcoin's
//! 6-confirmation rule. This allows fast confirmations for low-value transactions
//! without waiting for explicit BFT finality.
//!
//! ## Finality Levels
//!
//! - 1 block deep: ~50% confidence
//! - 3 blocks deep: ~94% confidence
//! - 6 blocks deep: ~99% confidence
//! - K blocks deep: Implicit finality threshold
//!
//! ## Use Cases
//!
//! - Low-value transactions (coffee purchase): 1-2 confirmations
//! - Medium-value transactions: 3-6 confirmations
//! - High-value transactions: Wait for explicit finality

use std::sync::atomic::{AtomicU32, Ordering};

/// Configuration for implicit finality
#[derive(Clone, Debug)]
pub struct ImplicitFinalityConfig {
    /// Number of confirmations for implicit finality (default: 6)
    pub confirmation_depth: u32,
    /// Whether implicit finality is enabled
    pub enabled: bool,
}

impl Default for ImplicitFinalityConfig {
    fn default() -> Self {
        Self {
            confirmation_depth: 6,
            enabled: true,
        }
    }
}

/// Implicit finality tracker
pub struct ImplicitFinalityTracker {
    /// Configuration
    config: ImplicitFinalityConfig,
    /// Current best block number
    best_block: AtomicU32,
    /// Last explicitly finalized block number
    last_explicit_finalized: AtomicU32,
    /// Last implicitly finalized block number
    last_implicit_finalized: AtomicU32,
}

impl ImplicitFinalityTracker {
    /// Create a new implicit finality tracker
    pub fn new(config: ImplicitFinalityConfig) -> Self {
        Self {
            config,
            best_block: AtomicU32::new(0),
            last_explicit_finalized: AtomicU32::new(0),
            last_implicit_finalized: AtomicU32::new(0),
        }
    }

    /// Update the best block number
    pub fn set_best_block(&self, number: u32) {
        let old = self.best_block.swap(number, Ordering::SeqCst);
        if number > old {
            self.update_implicit_finality();
        }
    }

    /// Update the explicit finality marker
    pub fn set_explicit_finalized(&self, number: u32) {
        self.last_explicit_finalized.store(number, Ordering::SeqCst);
        // Implicit finality is always at least as high as explicit
        let implicit = self.last_implicit_finalized.load(Ordering::SeqCst);
        if number > implicit {
            self.last_implicit_finalized.store(number, Ordering::SeqCst);
        }
    }

    /// Update implicit finality based on current best block
    fn update_implicit_finality(&self) {
        if !self.config.enabled {
            return;
        }

        let best = self.best_block.load(Ordering::SeqCst);
        let new_implicit = best.saturating_sub(self.config.confirmation_depth);
        let explicit = self.last_explicit_finalized.load(Ordering::SeqCst);

        // Implicit finality cannot be lower than explicit finality
        let new_implicit = new_implicit.max(explicit);

        let old_implicit = self.last_implicit_finalized.swap(new_implicit, Ordering::SeqCst);

        if new_implicit > old_implicit {
            log::debug!(
                "📊 Implicit finality advanced: #{} -> #{} (best: #{})",
                old_implicit,
                new_implicit,
                best
            );
        }
    }

    /// Check if a block has implicit finality
    pub fn is_implicitly_final(&self, block_number: u32) -> bool {
        if !self.config.enabled {
            return false;
        }
        block_number <= self.last_implicit_finalized.load(Ordering::SeqCst)
    }

    /// Check if a block has explicit finality
    pub fn is_explicitly_final(&self, block_number: u32) -> bool {
        block_number <= self.last_explicit_finalized.load(Ordering::SeqCst)
    }

    /// Get the finality status of a block
    pub fn get_finality_status(&self, block_number: u32) -> FinalityStatus {
        let explicit = self.last_explicit_finalized.load(Ordering::SeqCst);
        let implicit = self.last_implicit_finalized.load(Ordering::SeqCst);
        let best = self.best_block.load(Ordering::SeqCst);

        if block_number <= explicit {
            FinalityStatus::ExplicitlyFinalized
        } else if block_number <= implicit {
            FinalityStatus::ImplicitlyFinalized
        } else if block_number <= best {
            let depth = best.saturating_sub(block_number);
            FinalityStatus::Pending {
                depth,
                confidence: self.calculate_confidence(depth),
            }
        } else {
            FinalityStatus::Unknown
        }
    }

    /// Calculate confidence percentage based on depth
    pub fn calculate_confidence(&self, depth: u32) -> f64 {
        // With BFT consensus and honest majority, probability of revert
        // decreases exponentially with depth
        match depth {
            0 => 0.50,       // Just imported
            1 => 0.75,       // 1 confirmation
            2 => 0.875,      // 2 confirmations
            3 => 0.9375,     // 3 confirmations
            4 => 0.96875,    // 4 confirmations
            5 => 0.984375,   // 5 confirmations
            _ => 0.99,       // 6+ confirmations
        }
    }

    /// Get current state
    pub fn state(&self) -> ImplicitFinalityState {
        ImplicitFinalityState {
            best_block: self.best_block.load(Ordering::SeqCst),
            explicit_finalized: self.last_explicit_finalized.load(Ordering::SeqCst),
            implicit_finalized: self.last_implicit_finalized.load(Ordering::SeqCst),
            confirmation_depth: self.config.confirmation_depth,
        }
    }
}

/// Finality status of a block
#[derive(Debug, Clone, PartialEq)]
pub enum FinalityStatus {
    /// Block has explicit BFT finality (3 certificates)
    ExplicitlyFinalized,
    /// Block has implicit finality (K confirmations)
    ImplicitlyFinalized,
    /// Block is pending finality
    Pending { depth: u32, confidence: f64 },
    /// Block number is unknown/future
    Unknown,
}

/// Current state of implicit finality tracker
#[derive(Debug, Clone)]
pub struct ImplicitFinalityState {
    pub best_block: u32,
    pub explicit_finalized: u32,
    pub implicit_finalized: u32,
    pub confirmation_depth: u32,
}

impl Clone for ImplicitFinalityTracker {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            best_block: AtomicU32::new(self.best_block.load(Ordering::SeqCst)),
            last_explicit_finalized: AtomicU32::new(self.last_explicit_finalized.load(Ordering::SeqCst)),
            last_implicit_finalized: AtomicU32::new(self.last_implicit_finalized.load(Ordering::SeqCst)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_finality_progression() {
        let tracker = ImplicitFinalityTracker::new(ImplicitFinalityConfig {
            confirmation_depth: 6,
            enabled: true,
        });

        // Set best block to 10
        tracker.set_best_block(10);

        // Block 4 should be implicitly finalized (10 - 6 = 4)
        assert!(tracker.is_implicitly_final(4));
        assert!(!tracker.is_implicitly_final(5));
    }

    #[test]
    fn test_explicit_overrides_implicit() {
        let tracker = ImplicitFinalityTracker::new(ImplicitFinalityConfig {
            confirmation_depth: 6,
            enabled: true,
        });

        tracker.set_best_block(5);
        // Implicit would be 0 (5 - 6 = -1 -> 0)

        // Set explicit finality to 3
        tracker.set_explicit_finalized(3);

        // Implicit should be at least explicit
        assert!(tracker.is_implicitly_final(3));
    }

    #[test]
    fn test_confidence_calculation() {
        let tracker = ImplicitFinalityTracker::new(ImplicitFinalityConfig::default());

        assert_eq!(tracker.calculate_confidence(0), 0.50);
        assert_eq!(tracker.calculate_confidence(1), 0.75);
        assert_eq!(tracker.calculate_confidence(3), 0.9375);
        assert_eq!(tracker.calculate_confidence(6), 0.99);
        assert_eq!(tracker.calculate_confidence(10), 0.99);
    }

    #[test]
    fn test_finality_status() {
        let tracker = ImplicitFinalityTracker::new(ImplicitFinalityConfig {
            confirmation_depth: 6,
            enabled: true,
        });

        tracker.set_best_block(20);
        tracker.set_explicit_finalized(10);

        // Block 8 should be explicitly finalized
        assert_eq!(
            tracker.get_finality_status(8),
            FinalityStatus::ExplicitlyFinalized
        );

        // Block 12 should be implicitly finalized (20 - 6 = 14, and 12 < 14)
        assert_eq!(
            tracker.get_finality_status(12),
            FinalityStatus::ImplicitlyFinalized
        );

        // Block 16 should be pending
        match tracker.get_finality_status(16) {
            FinalityStatus::Pending { depth, confidence } => {
                assert_eq!(depth, 4);
                assert_eq!(confidence, 0.96875);
            }
            _ => panic!("Expected Pending status"),
        }

        // Block 25 should be unknown (future block)
        assert_eq!(
            tracker.get_finality_status(25),
            FinalityStatus::Unknown
        );
    }

    #[test]
    fn test_disabled_implicit_finality() {
        let tracker = ImplicitFinalityTracker::new(ImplicitFinalityConfig {
            confirmation_depth: 6,
            enabled: false,
        });

        tracker.set_best_block(20);

        // With implicit finality disabled, no block should be implicitly final
        assert!(!tracker.is_implicitly_final(10));

        // Only explicit finality works
        tracker.set_explicit_finalized(10);
        assert!(tracker.is_explicitly_final(10));
    }

    #[test]
    fn test_state_snapshot() {
        let tracker = ImplicitFinalityTracker::new(ImplicitFinalityConfig {
            confirmation_depth: 6,
            enabled: true,
        });

        tracker.set_best_block(20);
        tracker.set_explicit_finalized(10);

        let state = tracker.state();
        assert_eq!(state.best_block, 20);
        assert_eq!(state.explicit_finalized, 10);
        assert_eq!(state.implicit_finalized, 14); // 20 - 6 = 14
        assert_eq!(state.confirmation_depth, 6);
    }

    #[test]
    fn test_implicit_never_below_explicit() {
        let tracker = ImplicitFinalityTracker::new(ImplicitFinalityConfig {
            confirmation_depth: 6,
            enabled: true,
        });

        // Set best block to 5
        tracker.set_best_block(5);
        // Implicit would be -1 (saturates to 0)

        // Set explicit finality to 3
        tracker.set_explicit_finalized(3);

        let state = tracker.state();
        // Implicit should be at least 3 (explicit level)
        assert!(state.implicit_finalized >= state.explicit_finalized);
    }
}
