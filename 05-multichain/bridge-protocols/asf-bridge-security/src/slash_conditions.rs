//! # Slash Conditions
//!
//! Defines slashing conditions for bridge misbehavior.

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use asf_collator::{Hash, BlockNumber, CollatorId, Balance};

/// Slash reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum SlashReason {
    /// Attested to false finality
    FalseFinality,
    /// Double spending across chains
    DoubleSpend,
    /// Invalid merkle proof
    InvalidProof,
    /// Insufficient security deposit
    InsufficientDeposit,
    /// Failed to relay verified transfer
    RelayFailure,
}

/// Slash record
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct SlashRecord {
    /// Slashed collator
    pub collator: CollatorId,
    /// Slash reason
    pub reason: SlashReason,
    /// Amount slashed
    pub amount: Balance,
    /// Block where slash occurred
    pub slash_block: BlockNumber,
    /// Evidence hash
    pub evidence: Hash,
}

impl SlashRecord {
    /// Create new slash record
    pub fn new(
        collator: CollatorId,
        reason: SlashReason,
        amount: Balance,
        slash_block: BlockNumber,
        evidence: Hash,
    ) -> Self {
        Self {
            collator,
            reason,
            amount,
            slash_block,
            evidence,
        }
    }
}

/// Slash condition checker
pub struct SlashChecker {
    /// Maximum tolerable error rate (basis points)
    max_error_rate: u32,
    /// Slash multiplier (e.g., 10x deposit)
    slash_multiplier: u32,
}

impl SlashChecker {
    /// Create new checker
    pub fn new(max_error_rate: u32, slash_multiplier: u32) -> Self {
        Self {
            max_error_rate,
            slash_multiplier,
        }
    }

    /// Check if should slash for false finality
    pub fn should_slash_false_finality(
        &self,
        claimed_finalized: bool,
        actually_finalized: bool,
    ) -> bool {
        claimed_finalized && !actually_finalized
    }

    /// Calculate slash amount
    pub fn calculate_slash_amount(&self, deposit: Balance) -> Balance {
        deposit.saturating_mul(self.slash_multiplier as u128)
    }
}

impl Default for SlashChecker {
    fn default() -> Self {
        Self::new(
            100, // 1% error rate
            10,  // 10x deposit
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::H256;
    use sp_core::crypto::AccountId32;

    #[test]
    fn test_slash_false_finality() {
        let checker = SlashChecker::default();

        assert!(checker.should_slash_false_finality(true, false));
        assert!(!checker.should_slash_false_finality(false, false));
        assert!(!checker.should_slash_false_finality(true, true));
    }

    #[test]
    fn test_slash_amount() {
        let checker = SlashChecker::default();
        let deposit = 1_000_000u128;

        let slash = checker.calculate_slash_amount(deposit);
        assert_eq!(slash, 10_000_000); // 10x
    }

    #[test]
    fn test_slash_record() {
        let collator = AccountId32::new([1u8; 32]);
        let record = SlashRecord::new(
            collator,
            SlashReason::FalseFinality,
            10_000_000,
            1000,
            H256::random(),
        );

        assert_eq!(record.reason, SlashReason::FalseFinality);
        assert_eq!(record.amount, 10_000_000);
    }
}
