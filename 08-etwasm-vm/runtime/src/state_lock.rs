//! State Lock Module - Enforces Checks-Effects-Interactions Pattern
//!
//! This module provides state locking mechanisms to prevent reentrancy attacks
//! by ensuring that contract state cannot be modified during external calls.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::collections::btree_map::BTreeMap;

/// Account identifier type (32-byte array for EVM compatibility)
pub type AccountId = [u8; 32];

/// State lock manager for preventing concurrent state modifications
///
/// The StateLock ensures that when a contract is executing, its state cannot
/// be modified by reentrant calls. This is a critical security feature that
/// prevents reentrancy attacks like the infamous DAO hack.
///
/// # Security Model
///
/// When a contract A calls contract B:
/// 1. Contract A's state is locked
/// 2. Contract B executes with its own state unlocked
/// 3. If B tries to call back to A, the call is rejected (A is locked)
/// 4. When B completes, A's state is unlocked
///
/// This enforces the Checks-Effects-Interactions (CEI) pattern at the VM level.
#[derive(Debug, Clone)]
pub struct StateLock {
    /// Map of locked accounts with their lock count (for nested calls)
    locked_accounts: BTreeMap<AccountId, u32>,
}

impl StateLock {
    /// Create a new StateLock instance
    pub fn new() -> Self {
        Self {
            locked_accounts: BTreeMap::new(),
        }
    }

    /// Lock an account's state
    ///
    /// This should be called before making an external call from a contract.
    /// The lock count is incremented to support nested locking scenarios.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to lock
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut lock = StateLock::new();
    /// let account = [0u8; 32];
    /// lock.lock(&account);
    /// assert!(lock.is_locked(&account));
    /// ```
    pub fn lock(&mut self, account: &AccountId) {
        *self.locked_accounts.entry(*account).or_insert(0) += 1;
    }

    /// Unlock an account's state
    ///
    /// This should be called after an external call completes.
    /// The lock count is decremented, and if it reaches zero, the lock is removed.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to unlock
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut lock = StateLock::new();
    /// let account = [0u8; 32];
    /// lock.lock(&account);
    /// lock.unlock(&account);
    /// assert!(!lock.is_locked(&account));
    /// ```
    pub fn unlock(&mut self, account: &AccountId) {
        if let Some(count) = self.locked_accounts.get_mut(account) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                self.locked_accounts.remove(account);
            }
        }
    }

    /// Check if an account's state is locked
    ///
    /// Returns true if the account is currently locked (has ongoing execution).
    ///
    /// # Arguments
    ///
    /// * `account` - The account to check
    ///
    /// # Returns
    ///
    /// `true` if the account is locked, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut lock = StateLock::new();
    /// let account = [0u8; 32];
    /// assert!(!lock.is_locked(&account));
    /// lock.lock(&account);
    /// assert!(lock.is_locked(&account));
    /// ```
    pub fn is_locked(&self, account: &AccountId) -> bool {
        self.locked_accounts.get(account).map_or(false, |&count| count > 0)
    }

    /// Get the lock count for an account
    ///
    /// Returns the number of times the account has been locked (for nested calls).
    ///
    /// # Arguments
    ///
    /// * `account` - The account to check
    ///
    /// # Returns
    ///
    /// The lock count (0 if not locked)
    pub fn lock_count(&self, account: &AccountId) -> u32 {
        self.locked_accounts.get(account).copied().unwrap_or(0)
    }

    /// Clear all locks
    ///
    /// This is useful for testing or in case of emergency recovery.
    /// Should be used with caution in production code.
    pub fn clear(&mut self) {
        self.locked_accounts.clear();
    }

    /// Get the number of currently locked accounts
    pub fn locked_count(&self) -> usize {
        self.locked_accounts.len()
    }
}

impl Default for StateLock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_unlock() {
        let mut lock = StateLock::new();
        let account = [1u8; 32];

        // Initially not locked
        assert!(!lock.is_locked(&account));
        assert_eq!(lock.lock_count(&account), 0);

        // Lock the account
        lock.lock(&account);
        assert!(lock.is_locked(&account));
        assert_eq!(lock.lock_count(&account), 1);

        // Unlock the account
        lock.unlock(&account);
        assert!(!lock.is_locked(&account));
        assert_eq!(lock.lock_count(&account), 0);
    }

    #[test]
    fn test_nested_locking() {
        let mut lock = StateLock::new();
        let account = [1u8; 32];

        // Lock multiple times
        lock.lock(&account);
        lock.lock(&account);
        lock.lock(&account);
        assert!(lock.is_locked(&account));
        assert_eq!(lock.lock_count(&account), 3);

        // Unlock once - still locked
        lock.unlock(&account);
        assert!(lock.is_locked(&account));
        assert_eq!(lock.lock_count(&account), 2);

        // Unlock all
        lock.unlock(&account);
        lock.unlock(&account);
        assert!(!lock.is_locked(&account));
        assert_eq!(lock.lock_count(&account), 0);
    }

    #[test]
    fn test_multiple_accounts() {
        let mut lock = StateLock::new();
        let account1 = [1u8; 32];
        let account2 = [2u8; 32];

        lock.lock(&account1);
        lock.lock(&account2);

        assert!(lock.is_locked(&account1));
        assert!(lock.is_locked(&account2));
        assert_eq!(lock.locked_count(), 2);

        lock.unlock(&account1);
        assert!(!lock.is_locked(&account1));
        assert!(lock.is_locked(&account2));
        assert_eq!(lock.locked_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut lock = StateLock::new();
        let account1 = [1u8; 32];
        let account2 = [2u8; 32];

        lock.lock(&account1);
        lock.lock(&account2);
        assert_eq!(lock.locked_count(), 2);

        lock.clear();
        assert_eq!(lock.locked_count(), 0);
        assert!(!lock.is_locked(&account1));
        assert!(!lock.is_locked(&account2));
    }

    #[test]
    fn test_unlock_unlocked_account() {
        let mut lock = StateLock::new();
        let account = [1u8; 32];

        // Unlocking an unlocked account should be safe
        lock.unlock(&account);
        assert!(!lock.is_locked(&account));
        assert_eq!(lock.lock_count(&account), 0);
    }
}
