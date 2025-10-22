//! Multi-Signature Custodian Module for Bridge Security
//!
//! Provides M-of-N multi-signature functionality for bridge operations.
//! This module is CRITICAL for preventing single points of failure in
//! cross-chain bridge operations.
//!
//! ## Security Features
//! - Threshold validation (1 ≤ M ≤ N)
//! - Duplicate approval prevention
//! - Execution only after threshold reached
//! - Custodian authorization checks
//!
//! ## Usage
//! ```rust,ignore
//! use etrid_bridge_common::multisig::{MultiSigCustodian, PendingApproval};
//!
//! // Create 2-of-3 multisig
//! let custodians = vec![alice, bob, charlie];
//! let multisig = MultiSigCustodian::new(custodians, 2)?;
//!
//! // Verify custodian
//! assert!(multisig.is_custodian(&alice));
//!
//! // Check if threshold reached
//! let approvals = vec![alice, bob];
//! assert!(multisig.has_threshold(&approvals));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// Multi-signature custodian set with M-of-N threshold
///
/// # Type Parameters
/// - `AccountId`: The account identifier type
///
/// # Fields
/// - `custodians`: Vector of authorized custodian accounts
/// - `threshold`: Number of approvals required (M in M-of-N)
///
/// # Invariants
/// - `threshold > 0`
/// - `threshold <= custodians.len()`
///
/// # Note on MaxEncodedLen
/// This struct uses Vec which doesn't implement MaxEncodedLen.
/// For storage, use BoundedVec wrapper or store separately.
#[derive(Encode, Decode, TypeInfo, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct MultiSigCustodian<AccountId> {
    pub custodians: Vec<AccountId>,
    pub threshold: u32,
}

/// Pending approval state for a multi-sig operation
///
/// # Type Parameters
/// - `AccountId`: The account identifier type
/// - `Hash`: The hash type for operation identification
///
/// # Fields
/// - `operation_hash`: Unique identifier for the operation
/// - `approvals`: List of custodians who have approved
/// - `required_approvals`: Number of approvals needed (M in M-of-N)
/// - `executed`: Whether the operation has been executed
///
/// # State Transitions
/// 1. Created with `executed = false`, `approvals = []`
/// 2. Custodians call approve, adding to `approvals`
/// 3. When `approvals.len() >= required_approvals`, operation executes
/// 4. After execution, `executed = true` (prevents re-execution)
#[derive(Encode, Decode, TypeInfo, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct PendingApproval<AccountId, Hash> {
    pub operation_hash: Hash,
    pub approvals: Vec<AccountId>,
    pub required_approvals: u32,
    pub executed: bool,
}

impl<AccountId: Clone + PartialEq> MultiSigCustodian<AccountId> {
    /// Create a new multi-sig custodian set
    ///
    /// # Parameters
    /// - `custodians`: Vector of custodian accounts
    /// - `threshold`: Number of approvals required (M in M-of-N)
    ///
    /// # Returns
    /// - `Ok(MultiSigCustodian)` if valid
    /// - `Err(&'static str)` if invalid threshold
    ///
    /// # Errors
    /// - "Invalid threshold: must be > 0" if threshold is 0
    /// - "Invalid threshold: exceeds custodian count" if threshold > N
    /// - "Invalid custodians: empty set" if custodians is empty
    ///
    /// # Examples
    /// ```rust,ignore
    /// // Valid 2-of-3 multisig
    /// let multisig = MultiSigCustodian::new(vec![alice, bob, charlie], 2)?;
    /// assert_eq!(multisig.threshold, 2);
    ///
    /// // Invalid: threshold = 0
    /// assert!(MultiSigCustodian::new(vec![alice], 0).is_err());
    ///
    /// // Invalid: threshold > N
    /// assert!(MultiSigCustodian::new(vec![alice], 2).is_err());
    /// ```
    pub fn new(custodians: Vec<AccountId>, threshold: u32) -> Result<Self, &'static str> {
        if custodians.is_empty() {
            return Err("Invalid custodians: empty set");
        }
        if threshold == 0 {
            return Err("Invalid threshold: must be > 0");
        }
        if threshold as usize > custodians.len() {
            return Err("Invalid threshold: exceeds custodian count");
        }
        Ok(Self { custodians, threshold })
    }

    /// Check if an account is a custodian
    ///
    /// # Parameters
    /// - `who`: Account to check
    ///
    /// # Returns
    /// - `true` if `who` is in the custodian set
    /// - `false` otherwise
    ///
    /// # Examples
    /// ```rust,ignore
    /// let multisig = MultiSigCustodian::new(vec![alice, bob], 2)?;
    /// assert!(multisig.is_custodian(&alice));
    /// assert!(multisig.is_custodian(&bob));
    /// assert!(!multisig.is_custodian(&charlie));
    /// ```
    pub fn is_custodian(&self, who: &AccountId) -> bool {
        self.custodians.contains(who)
    }

    /// Check if approval threshold has been reached
    ///
    /// # Parameters
    /// - `approvals`: Slice of accounts that have approved
    ///
    /// # Returns
    /// - `true` if `approvals.len() >= threshold`
    /// - `false` otherwise
    ///
    /// # Examples
    /// ```rust,ignore
    /// let multisig = MultiSigCustodian::new(vec![alice, bob, charlie], 2)?;
    ///
    /// // Not enough approvals
    /// assert!(!multisig.has_threshold(&[alice]));
    ///
    /// // Exactly threshold
    /// assert!(multisig.has_threshold(&[alice, bob]));
    ///
    /// // More than threshold
    /// assert!(multisig.has_threshold(&[alice, bob, charlie]));
    /// ```
    pub fn has_threshold(&self, approvals: &[AccountId]) -> bool {
        approvals.len() >= self.threshold as usize
    }

    /// Validate that all approvals are from authorized custodians
    ///
    /// # Parameters
    /// - `approvals`: Slice of accounts to validate
    ///
    /// # Returns
    /// - `Ok(())` if all approvals are from custodians
    /// - `Err(&'static str)` if any approval is not from a custodian
    ///
    /// # Examples
    /// ```rust,ignore
    /// let multisig = MultiSigCustodian::new(vec![alice, bob], 2)?;
    ///
    /// // All valid
    /// assert!(multisig.validate_approvals(&[alice, bob]).is_ok());
    ///
    /// // Contains invalid custodian
    /// assert!(multisig.validate_approvals(&[alice, charlie]).is_err());
    /// ```
    pub fn validate_approvals(&self, approvals: &[AccountId]) -> Result<(), &'static str> {
        for approval in approvals {
            if !self.is_custodian(approval) {
                return Err("Invalid approval: not a custodian");
            }
        }
        Ok(())
    }

    /// Get the number of custodians
    ///
    /// # Returns
    /// - Number of custodians in the set
    pub fn custodian_count(&self) -> u32 {
        self.custodians.len() as u32
    }

    /// Check if this is a valid multisig configuration
    ///
    /// # Returns
    /// - `true` if configuration is valid
    /// - `false` otherwise
    ///
    /// # Examples
    /// ```rust,ignore
    /// let multisig = MultiSigCustodian {
    ///     custodians: vec![alice, bob, charlie],
    ///     threshold: 2,
    /// };
    /// assert!(multisig.is_valid());
    ///
    /// let invalid = MultiSigCustodian {
    ///     custodians: vec![alice],
    ///     threshold: 2, // threshold > count
    /// };
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.custodians.is_empty()
            && self.threshold > 0
            && (self.threshold as usize) <= self.custodians.len()
    }
}

impl<AccountId: Clone + PartialEq, Hash: Clone + PartialEq> PendingApproval<AccountId, Hash> {
    /// Create a new pending approval
    ///
    /// # Parameters
    /// - `operation_hash`: Unique identifier for the operation
    /// - `required_approvals`: Number of approvals needed
    ///
    /// # Returns
    /// - New `PendingApproval` with no approvals and not executed
    ///
    /// # Examples
    /// ```rust,ignore
    /// let pending = PendingApproval::new(hash, 2);
    /// assert_eq!(pending.approvals.len(), 0);
    /// assert!(!pending.executed);
    /// ```
    pub fn new(operation_hash: Hash, required_approvals: u32) -> Self {
        Self {
            operation_hash,
            approvals: Vec::new(),
            required_approvals,
            executed: false,
        }
    }

    /// Add an approval from a custodian
    ///
    /// # Parameters
    /// - `who`: Custodian adding approval
    ///
    /// # Returns
    /// - `Ok(())` if approval added successfully
    /// - `Err(&'static str)` if duplicate or already executed
    ///
    /// # Errors
    /// - "Already executed" if operation has been executed
    /// - "Duplicate approval" if custodian has already approved
    ///
    /// # Examples
    /// ```rust,ignore
    /// let mut pending = PendingApproval::new(hash, 2);
    ///
    /// // First approval
    /// assert!(pending.add_approval(alice).is_ok());
    /// assert_eq!(pending.approvals.len(), 1);
    ///
    /// // Duplicate approval
    /// assert!(pending.add_approval(alice).is_err());
    ///
    /// // Mark executed
    /// pending.executed = true;
    /// assert!(pending.add_approval(bob).is_err());
    /// ```
    pub fn add_approval(&mut self, who: AccountId) -> Result<(), &'static str> {
        if self.executed {
            return Err("Already executed");
        }
        if self.approvals.contains(&who) {
            return Err("Duplicate approval");
        }
        self.approvals.push(who);
        Ok(())
    }

    /// Check if threshold has been reached
    ///
    /// # Returns
    /// - `true` if `approvals.len() >= required_approvals`
    /// - `false` otherwise
    pub fn has_threshold(&self) -> bool {
        self.approvals.len() >= self.required_approvals as usize
    }

    /// Mark the operation as executed
    ///
    /// # Returns
    /// - `Ok(())` if marked successfully
    /// - `Err(&'static str)` if already executed or threshold not reached
    ///
    /// # Errors
    /// - "Already executed" if already marked as executed
    /// - "Threshold not reached" if not enough approvals
    pub fn mark_executed(&mut self) -> Result<(), &'static str> {
        if self.executed {
            return Err("Already executed");
        }
        if !self.has_threshold() {
            return Err("Threshold not reached");
        }
        self.executed = true;
        Ok(())
    }

    /// Get current approval count
    ///
    /// # Returns
    /// - Number of approvals received so far
    pub fn approval_count(&self) -> u32 {
        self.approvals.len() as u32
    }

    /// Check if a specific custodian has approved
    ///
    /// # Parameters
    /// - `who`: Custodian to check
    ///
    /// # Returns
    /// - `true` if custodian has approved
    /// - `false` otherwise
    pub fn has_approved(&self, who: &AccountId) -> bool {
        self.approvals.contains(who)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type AccountId = u64;
    type Hash = [u8; 32];

    #[test]
    fn multisig_creation_valid() {
        let custodians = vec![1u64, 2u64, 3u64];

        // Valid 2-of-3
        let multisig = MultiSigCustodian::new(custodians.clone(), 2);
        assert!(multisig.is_ok());
        let multisig = multisig.unwrap();
        assert_eq!(multisig.threshold, 2);
        assert_eq!(multisig.custodians.len(), 3);

        // Valid 1-of-3
        let multisig = MultiSigCustodian::new(custodians.clone(), 1);
        assert!(multisig.is_ok());

        // Valid 3-of-3 (unanimous)
        let multisig = MultiSigCustodian::new(custodians, 3);
        assert!(multisig.is_ok());
    }

    #[test]
    fn multisig_creation_invalid_threshold_zero() {
        let custodians = vec![1u64, 2u64, 3u64];
        let result = MultiSigCustodian::new(custodians, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid threshold: must be > 0");
    }

    #[test]
    fn multisig_creation_invalid_threshold_exceeds() {
        let custodians = vec![1u64, 2u64];
        let result = MultiSigCustodian::new(custodians, 3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid threshold: exceeds custodian count");
    }

    #[test]
    fn multisig_creation_invalid_empty_custodians() {
        let custodians: Vec<u64> = vec![];
        let result = MultiSigCustodian::new(custodians, 1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid custodians: empty set");
    }

    #[test]
    fn multisig_is_custodian() {
        let custodians = vec![1u64, 2u64, 3u64];
        let multisig = MultiSigCustodian::new(custodians, 2).unwrap();

        assert!(multisig.is_custodian(&1));
        assert!(multisig.is_custodian(&2));
        assert!(multisig.is_custodian(&3));
        assert!(!multisig.is_custodian(&4));
        assert!(!multisig.is_custodian(&999));
    }

    #[test]
    fn multisig_has_threshold() {
        let custodians = vec![1u64, 2u64, 3u64];
        let multisig = MultiSigCustodian::new(custodians, 2).unwrap();

        // Less than threshold
        assert!(!multisig.has_threshold(&[1]));
        assert!(!multisig.has_threshold(&[]));

        // Exactly threshold
        assert!(multisig.has_threshold(&[1, 2]));
        assert!(multisig.has_threshold(&[2, 3]));

        // More than threshold
        assert!(multisig.has_threshold(&[1, 2, 3]));
    }

    #[test]
    fn multisig_validate_approvals() {
        let custodians = vec![1u64, 2u64, 3u64];
        let multisig = MultiSigCustodian::new(custodians, 2).unwrap();

        // All valid
        assert!(multisig.validate_approvals(&[1, 2]).is_ok());
        assert!(multisig.validate_approvals(&[1, 2, 3]).is_ok());
        assert!(multisig.validate_approvals(&[]).is_ok());

        // Contains invalid custodian
        assert!(multisig.validate_approvals(&[1, 4]).is_err());
        assert!(multisig.validate_approvals(&[999]).is_err());
    }

    #[test]
    fn multisig_custodian_count() {
        let multisig = MultiSigCustodian::new(vec![1u64, 2u64, 3u64], 2).unwrap();
        assert_eq!(multisig.custodian_count(), 3);

        let multisig = MultiSigCustodian::new(vec![1u64], 1).unwrap();
        assert_eq!(multisig.custodian_count(), 1);
    }

    #[test]
    fn multisig_is_valid() {
        let valid = MultiSigCustodian {
            custodians: vec![1u64, 2u64, 3u64],
            threshold: 2,
        };
        assert!(valid.is_valid());

        let invalid_threshold_zero = MultiSigCustodian {
            custodians: vec![1u64, 2u64],
            threshold: 0,
        };
        assert!(!invalid_threshold_zero.is_valid());

        let invalid_threshold_exceeds = MultiSigCustodian {
            custodians: vec![1u64],
            threshold: 2,
        };
        assert!(!invalid_threshold_exceeds.is_valid());

        let invalid_empty = MultiSigCustodian::<u64> {
            custodians: vec![],
            threshold: 1,
        };
        assert!(!invalid_empty.is_valid());
    }

    #[test]
    fn pending_approval_creation() {
        let hash = [1u8; 32];
        let pending = PendingApproval::<AccountId, Hash>::new(hash, 2);

        assert_eq!(pending.operation_hash, hash);
        assert_eq!(pending.required_approvals, 2);
        assert_eq!(pending.approvals.len(), 0);
        assert!(!pending.executed);
    }

    #[test]
    fn pending_approval_add_approval() {
        let hash = [1u8; 32];
        let mut pending = PendingApproval::<AccountId, Hash>::new(hash, 2);

        // First approval
        assert!(pending.add_approval(1).is_ok());
        assert_eq!(pending.approvals.len(), 1);
        assert!(pending.has_approved(&1));

        // Second approval
        assert!(pending.add_approval(2).is_ok());
        assert_eq!(pending.approvals.len(), 2);
        assert!(pending.has_approved(&2));

        // Duplicate approval
        assert!(pending.add_approval(1).is_err());
        assert_eq!(pending.add_approval(1).unwrap_err(), "Duplicate approval");
    }

    #[test]
    fn pending_approval_add_after_executed() {
        let hash = [1u8; 32];
        let mut pending = PendingApproval::<AccountId, Hash>::new(hash, 2);

        pending.add_approval(1).unwrap();
        pending.add_approval(2).unwrap();
        pending.executed = true;

        // Try to add approval after execution
        let result = pending.add_approval(3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Already executed");
    }

    #[test]
    fn pending_approval_has_threshold() {
        let hash = [1u8; 32];
        let mut pending = PendingApproval::<AccountId, Hash>::new(hash, 2);

        assert!(!pending.has_threshold());

        pending.add_approval(1).unwrap();
        assert!(!pending.has_threshold());

        pending.add_approval(2).unwrap();
        assert!(pending.has_threshold());

        pending.add_approval(3).unwrap();
        assert!(pending.has_threshold());
    }

    #[test]
    fn pending_approval_mark_executed() {
        let hash = [1u8; 32];
        let mut pending = PendingApproval::<AccountId, Hash>::new(hash, 2);

        // Try to execute without threshold
        let result = pending.mark_executed();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Threshold not reached");

        // Add approvals
        pending.add_approval(1).unwrap();
        pending.add_approval(2).unwrap();

        // Execute
        assert!(pending.mark_executed().is_ok());
        assert!(pending.executed);

        // Try to execute again
        let result = pending.mark_executed();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Already executed");
    }

    #[test]
    fn pending_approval_approval_count() {
        let hash = [1u8; 32];
        let mut pending = PendingApproval::<AccountId, Hash>::new(hash, 3);

        assert_eq!(pending.approval_count(), 0);

        pending.add_approval(1).unwrap();
        assert_eq!(pending.approval_count(), 1);

        pending.add_approval(2).unwrap();
        assert_eq!(pending.approval_count(), 2);

        pending.add_approval(3).unwrap();
        assert_eq!(pending.approval_count(), 3);
    }

    #[test]
    fn pending_approval_has_approved() {
        let hash = [1u8; 32];
        let mut pending = PendingApproval::<AccountId, Hash>::new(hash, 2);

        assert!(!pending.has_approved(&1));
        assert!(!pending.has_approved(&2));

        pending.add_approval(1).unwrap();
        assert!(pending.has_approved(&1));
        assert!(!pending.has_approved(&2));

        pending.add_approval(2).unwrap();
        assert!(pending.has_approved(&1));
        assert!(pending.has_approved(&2));
    }

    #[test]
    fn integration_multisig_workflow() {
        // Setup: 3 custodians, 2-of-3 threshold
        let custodians = vec![1u64, 2u64, 3u64];
        let multisig = MultiSigCustodian::new(custodians, 2).unwrap();

        // Create pending operation
        let operation_hash = [42u8; 32];
        let mut pending = PendingApproval::new(operation_hash, multisig.threshold);

        // Custodian 1 approves
        let custodian1 = 1u64;
        assert!(multisig.is_custodian(&custodian1));
        assert!(pending.add_approval(custodian1).is_ok());
        assert!(!pending.has_threshold());

        // Custodian 2 approves (reaches threshold)
        let custodian2 = 2u64;
        assert!(multisig.is_custodian(&custodian2));
        assert!(pending.add_approval(custodian2).is_ok());
        assert!(pending.has_threshold());

        // Execute operation
        assert!(pending.mark_executed().is_ok());

        // Custodian 3 tries to approve after execution (should fail)
        let custodian3 = 3u64;
        assert!(multisig.is_custodian(&custodian3));
        assert!(pending.add_approval(custodian3).is_err());

        // Verify final state
        assert!(pending.executed);
        assert_eq!(pending.approval_count(), 2);
        assert!(multisig.validate_approvals(&pending.approvals).is_ok());
    }

    #[test]
    fn integration_non_custodian_cannot_approve() {
        let custodians = vec![1u64, 2u64, 3u64];
        let multisig = MultiSigCustodian::new(custodians, 2).unwrap();

        let operation_hash = [42u8; 32];
        let mut pending = PendingApproval::new(operation_hash, multisig.threshold);

        // Add approvals from custodians
        pending.add_approval(1).unwrap();
        pending.add_approval(2).unwrap();

        // Try to add approval from non-custodian (at application level)
        let non_custodian = 999u64;
        assert!(!multisig.is_custodian(&non_custodian));

        // The multisig validation would catch this
        pending.approvals.push(non_custodian);
        assert!(multisig.validate_approvals(&pending.approvals).is_err());
    }
}
