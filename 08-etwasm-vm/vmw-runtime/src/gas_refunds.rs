//! Gas refund mechanisms for storage cleanup and optimization incentives

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use crate::VMw;

/// ============================================================================
/// REFUND CONSTANTS (EIP-3529)
/// ============================================================================

/// Refund for clearing storage (SSTORE from non-zero to zero)
pub const REFUND_SSTORE_CLEARS: VMw = 15_000;

/// Maximum refund as percentage of gas used (50% per EIP-3529)
pub const MAX_REFUND_QUOTIENT: u64 = 2;

/// Refund for resetting storage to original value
pub const REFUND_SSTORE_RESET: VMw = 4_800;

/// Refund for SELFDESTRUCT operation
pub const REFUND_SELFDESTRUCT: VMw = 24_000;

/// ============================================================================
/// GAS REFUND MANAGER
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GasRefundManager {
    /// Total refund accumulated for storage clears
    storage_clear_refunds: VMw,

    /// Total refund accumulated for storage resets
    storage_reset_refunds: VMw,

    /// Total refund accumulated for self-destruct
    selfdestruct_refunds: VMw,

    /// Number of storage slots cleared
    slots_cleared: u32,

    /// Number of storage slots reset to original value
    slots_reset: u32,

    /// Number of contracts self-destructed
    contracts_destructed: u32,

    /// Refund history for analysis
    refund_history: Vec<RefundEntry>,
}

impl GasRefundManager {
    /// Create new refund manager
    pub fn new() -> Self {
        Self {
            storage_clear_refunds: 0,
            storage_reset_refunds: 0,
            selfdestruct_refunds: 0,
            slots_cleared: 0,
            slots_reset: 0,
            contracts_destructed: 0,
            refund_history: Vec::new(),
        }
    }

    /// Add refund for clearing storage slot
    pub fn add_storage_clear_refund(&mut self) {
        self.storage_clear_refunds = self.storage_clear_refunds.saturating_add(REFUND_SSTORE_CLEARS);
        self.slots_cleared = self.slots_cleared.saturating_add(1);

        self.refund_history.push(RefundEntry {
            refund_type: RefundType::StorageClear,
            amount: REFUND_SSTORE_CLEARS,
        });
    }

    /// Add refund for resetting storage slot to original value
    pub fn add_storage_reset_refund(&mut self) {
        self.storage_reset_refunds = self.storage_reset_refunds.saturating_add(REFUND_SSTORE_RESET);
        self.slots_reset = self.slots_reset.saturating_add(1);

        self.refund_history.push(RefundEntry {
            refund_type: RefundType::StorageReset,
            amount: REFUND_SSTORE_RESET,
        });
    }

    /// Add refund for self-destruct operation
    pub fn add_selfdestruct_refund(&mut self) {
        self.selfdestruct_refunds = self.selfdestruct_refunds.saturating_add(REFUND_SELFDESTRUCT);
        self.contracts_destructed = self.contracts_destructed.saturating_add(1);

        self.refund_history.push(RefundEntry {
            refund_type: RefundType::SelfDestruct,
            amount: REFUND_SELFDESTRUCT,
        });
    }

    /// Add custom refund amount
    pub fn add_refund(&mut self, amount: VMw) {
        self.storage_clear_refunds = self.storage_clear_refunds.saturating_add(amount);

        self.refund_history.push(RefundEntry {
            refund_type: RefundType::Custom,
            amount,
        });
    }

    /// Get total refund accumulated
    pub fn get_total_refund(&self) -> VMw {
        self.storage_clear_refunds
            .saturating_add(self.storage_reset_refunds)
            .saturating_add(self.selfdestruct_refunds)
    }

    /// Calculate actual refund based on gas used (capped at 50%)
    pub fn calculate_actual_refund(&self, gas_used: VMw) -> VMw {
        let total_refund = self.get_total_refund();
        let max_refund = gas_used / MAX_REFUND_QUOTIENT;
        total_refund.min(max_refund)
    }

    /// Get refund breakdown
    pub fn get_refund_breakdown(&self) -> RefundBreakdown {
        RefundBreakdown {
            storage_clear_refunds: self.storage_clear_refunds,
            storage_reset_refunds: self.storage_reset_refunds,
            selfdestruct_refunds: self.selfdestruct_refunds,
            total_refund: self.get_total_refund(),
            slots_cleared: self.slots_cleared,
            slots_reset: self.slots_reset,
            contracts_destructed: self.contracts_destructed,
        }
    }

    /// Get refund statistics
    pub fn get_refund_stats(&self) -> RefundStats {
        let total_refund = self.get_total_refund();
        let avg_refund_per_entry = if self.refund_history.is_empty() {
            0
        } else {
            total_refund / self.refund_history.len() as u64
        };

        RefundStats {
            total_entries: self.refund_history.len() as u32,
            total_refund,
            avg_refund_per_entry,
            storage_clear_count: self.slots_cleared,
            storage_reset_count: self.slots_reset,
            selfdestruct_count: self.contracts_destructed,
        }
    }

    /// Check if any refunds are available
    pub fn has_refunds(&self) -> bool {
        self.get_total_refund() > 0
    }

    /// Clear all refunds
    pub fn clear_refunds(&mut self) {
        self.storage_clear_refunds = 0;
        self.storage_reset_refunds = 0;
        self.selfdestruct_refunds = 0;
        self.slots_cleared = 0;
        self.slots_reset = 0;
        self.contracts_destructed = 0;
        self.refund_history.clear();
    }

    /// Get refund efficiency (percentage of potential refunds claimed)
    pub fn get_refund_efficiency(&self, gas_used: VMw) -> u8 {
        let total_refund = self.get_total_refund();
        if total_refund == 0 {
            return 0;
        }

        let actual_refund = self.calculate_actual_refund(gas_used);
        ((actual_refund * 100) / total_refund) as u8
    }

    /// Reset for new execution
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for GasRefundManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// REFUND TYPES
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum RefundType {
    /// Storage slot cleared (non-zero to zero)
    StorageClear,
    /// Storage slot reset to original value
    StorageReset,
    /// Contract self-destructed
    SelfDestruct,
    /// Custom refund
    Custom,
}

impl RefundType {
    pub fn name(&self) -> &'static str {
        match self {
            RefundType::StorageClear => "Storage Clear",
            RefundType::StorageReset => "Storage Reset",
            RefundType::SelfDestruct => "Self Destruct",
            RefundType::Custom => "Custom",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            RefundType::StorageClear => "Storage slot cleared from non-zero to zero",
            RefundType::StorageReset => "Storage slot reset to original transaction value",
            RefundType::SelfDestruct => "Contract self-destructed",
            RefundType::Custom => "Custom refund amount",
        }
    }
}

/// ============================================================================
/// REFUND ENTRY
/// ============================================================================

#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct RefundEntry {
    pub refund_type: RefundType,
    pub amount: VMw,
}

/// ============================================================================
/// REFUND BREAKDOWN
/// ============================================================================

#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct RefundBreakdown {
    pub storage_clear_refunds: VMw,
    pub storage_reset_refunds: VMw,
    pub selfdestruct_refunds: VMw,
    pub total_refund: VMw,
    pub slots_cleared: u32,
    pub slots_reset: u32,
    pub contracts_destructed: u32,
}

impl RefundBreakdown {
    /// Get largest refund category
    pub fn get_largest_category(&self) -> RefundType {
        let max = self.storage_clear_refunds
            .max(self.storage_reset_refunds)
            .max(self.selfdestruct_refunds);

        if max == self.storage_clear_refunds {
            RefundType::StorageClear
        } else if max == self.storage_reset_refunds {
            RefundType::StorageReset
        } else {
            RefundType::SelfDestruct
        }
    }

    /// Check if refunds are substantial (>10% of typical transaction)
    pub fn is_substantial(&self) -> bool {
        self.total_refund > 100_000 // > 0.1 ÉTR worth of gas
    }
}

/// ============================================================================
/// REFUND STATISTICS
/// ============================================================================

#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct RefundStats {
    pub total_entries: u32,
    pub total_refund: VMw,
    pub avg_refund_per_entry: VMw,
    pub storage_clear_count: u32,
    pub storage_reset_count: u32,
    pub selfdestruct_count: u32,
}

impl RefundStats {
    /// Get dominant refund operation
    pub fn get_dominant_operation(&self) -> RefundType {
        let max = self.storage_clear_count
            .max(self.storage_reset_count)
            .max(self.selfdestruct_count);

        if max == self.storage_clear_count {
            RefundType::StorageClear
        } else if max == self.storage_reset_count {
            RefundType::StorageReset
        } else {
            RefundType::SelfDestruct
        }
    }
}

/// ============================================================================
/// STORAGE VALUE TRACKER (for refund calculation)
/// ============================================================================

/// Tracks original and current storage values for refund calculation
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StorageValueTracker {
    /// Original value at transaction start
    original_values: Vec<([u8; 32], [u8; 32])>, // (key, value)

    /// Current values during execution
    current_values: Vec<([u8; 32], [u8; 32])>, // (key, value)
}

impl StorageValueTracker {
    pub fn new() -> Self {
        Self {
            original_values: Vec::new(),
            current_values: Vec::new(),
        }
    }

    /// Record original value at transaction start
    pub fn record_original(&mut self, key: [u8; 32], value: [u8; 32]) {
        self.original_values.push((key, value));
    }

    /// Update current value
    pub fn update_current(&mut self, key: [u8; 32], value: [u8; 32]) {
        // Remove old entry if exists
        self.current_values.retain(|(k, _)| k != &key);
        self.current_values.push((key, value));
    }

    /// Get original value
    pub fn get_original(&self, key: &[u8; 32]) -> Option<[u8; 32]> {
        self.original_values
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| *v)
    }

    /// Get current value
    pub fn get_current(&self, key: &[u8; 32]) -> Option<[u8; 32]> {
        self.current_values
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| *v)
    }

    /// Check if value was reset to original
    pub fn is_reset_to_original(&self, key: &[u8; 32]) -> bool {
        if let (Some(original), Some(current)) = (self.get_original(key), self.get_current(key)) {
            original == current
        } else {
            false
        }
    }

    /// Check if value was cleared (set to zero)
    pub fn is_cleared(&self, key: &[u8; 32]) -> bool {
        if let Some(current) = self.get_current(key) {
            current.iter().all(|&b| b == 0)
        } else {
            false
        }
    }

    /// Clear tracker for new transaction
    pub fn clear(&mut self) {
        self.original_values.clear();
        self.current_values.clear();
    }
}

impl Default for StorageValueTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refund_manager_new() {
        let manager = GasRefundManager::new();
        assert_eq!(manager.get_total_refund(), 0);
        assert!(!manager.has_refunds());
    }

    #[test]
    fn test_storage_clear_refund() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();
        assert_eq!(manager.storage_clear_refunds, REFUND_SSTORE_CLEARS);
        assert_eq!(manager.slots_cleared, 1);
        assert!(manager.has_refunds());
    }

    #[test]
    fn test_storage_reset_refund() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_reset_refund();
        assert_eq!(manager.storage_reset_refunds, REFUND_SSTORE_RESET);
        assert_eq!(manager.slots_reset, 1);
    }

    #[test]
    fn test_selfdestruct_refund() {
        let mut manager = GasRefundManager::new();

        manager.add_selfdestruct_refund();
        assert_eq!(manager.selfdestruct_refunds, REFUND_SELFDESTRUCT);
        assert_eq!(manager.contracts_destructed, 1);
    }

    #[test]
    fn test_total_refund() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();
        manager.add_storage_reset_refund();
        manager.add_selfdestruct_refund();

        let expected = REFUND_SSTORE_CLEARS + REFUND_SSTORE_RESET + REFUND_SELFDESTRUCT;
        assert_eq!(manager.get_total_refund(), expected);
    }

    #[test]
    fn test_actual_refund_cap() {
        let mut manager = GasRefundManager::new();

        // Add large refund
        manager.add_storage_clear_refund();
        manager.add_storage_clear_refund();
        manager.add_storage_clear_refund();

        let gas_used = 10_000;
        let actual_refund = manager.calculate_actual_refund(gas_used);

        // Should be capped at 50% of gas used
        assert_eq!(actual_refund, gas_used / 2);
    }

    #[test]
    fn test_actual_refund_no_cap() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();

        let gas_used = 100_000;
        let actual_refund = manager.calculate_actual_refund(gas_used);

        // Refund is less than 50% cap, so full refund
        assert_eq!(actual_refund, REFUND_SSTORE_CLEARS);
    }

    #[test]
    fn test_refund_breakdown() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();
        manager.add_storage_reset_refund();

        let breakdown = manager.get_refund_breakdown();
        assert_eq!(breakdown.slots_cleared, 1);
        assert_eq!(breakdown.slots_reset, 1);
        assert_eq!(breakdown.contracts_destructed, 0);
    }

    #[test]
    fn test_refund_stats() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();
        manager.add_storage_clear_refund();
        manager.add_storage_reset_refund();

        let stats = manager.get_refund_stats();
        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.storage_clear_count, 2);
        assert_eq!(stats.storage_reset_count, 1);
    }

    #[test]
    fn test_refund_efficiency() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();

        // With enough gas, efficiency is 100%
        let efficiency1 = manager.get_refund_efficiency(100_000);
        assert_eq!(efficiency1, 100);

        // With limited gas, efficiency is lower
        let efficiency2 = manager.get_refund_efficiency(10_000);
        assert!(efficiency2 < 100);
    }

    #[test]
    fn test_custom_refund() {
        let mut manager = GasRefundManager::new();

        manager.add_refund(5000);
        assert_eq!(manager.storage_clear_refunds, 5000);
        assert_eq!(manager.refund_history.len(), 1);
    }

    #[test]
    fn test_clear_refunds() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();
        manager.add_storage_reset_refund();

        manager.clear_refunds();

        assert_eq!(manager.get_total_refund(), 0);
        assert!(!manager.has_refunds());
        assert_eq!(manager.refund_history.len(), 0);
    }

    #[test]
    fn test_storage_value_tracker() {
        let mut tracker = StorageValueTracker::new();

        let key = [1u8; 32];
        let original = [2u8; 32];
        let modified = [3u8; 32];

        tracker.record_original(key, original);
        tracker.update_current(key, modified);

        assert_eq!(tracker.get_original(&key), Some(original));
        assert_eq!(tracker.get_current(&key), Some(modified));
        assert!(!tracker.is_reset_to_original(&key));
    }

    #[test]
    fn test_storage_value_reset() {
        let mut tracker = StorageValueTracker::new();

        let key = [1u8; 32];
        let value = [2u8; 32];

        tracker.record_original(key, value);
        tracker.update_current(key, [3u8; 32]); // Modify
        tracker.update_current(key, value); // Reset to original

        assert!(tracker.is_reset_to_original(&key));
    }

    #[test]
    fn test_storage_value_cleared() {
        let mut tracker = StorageValueTracker::new();

        let key = [1u8; 32];
        let zero = [0u8; 32];

        tracker.record_original(key, [5u8; 32]);
        tracker.update_current(key, zero);

        assert!(tracker.is_cleared(&key));
    }

    #[test]
    fn test_refund_breakdown_largest_category() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();
        manager.add_storage_clear_refund();
        manager.add_storage_reset_refund();

        let breakdown = manager.get_refund_breakdown();
        assert_eq!(breakdown.get_largest_category(), RefundType::StorageClear);
    }

    #[test]
    fn test_refund_types() {
        assert_eq!(RefundType::StorageClear.name(), "Storage Clear");
        assert_eq!(RefundType::StorageReset.name(), "Storage Reset");
        assert_eq!(RefundType::SelfDestruct.name(), "Self Destruct");
    }

    #[test]
    fn test_reset() {
        let mut manager = GasRefundManager::new();

        manager.add_storage_clear_refund();
        manager.add_storage_reset_refund();

        manager.reset();

        assert_eq!(manager.get_total_refund(), 0);
        assert!(!manager.has_refunds());
    }
}
