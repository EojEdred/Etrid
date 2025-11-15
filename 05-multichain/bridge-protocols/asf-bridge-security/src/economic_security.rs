//! # Economic Security
//!
//! Economic deposits and security calculations for bridge operations.

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use asf_collator::{CollatorId, Balance, BlockNumber};

/// Economic deposit for bridge security
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct EconomicDeposit {
    /// Depositor (collator)
    pub depositor: CollatorId,
    /// Deposit amount
    pub amount: Balance,
    /// When deposit was made
    pub deposited_at: BlockNumber,
    /// Lock period (blocks)
    pub lock_period: BlockNumber,
    /// Whether deposit is locked
    pub locked: bool,
}

impl EconomicDeposit {
    /// Create new deposit
    pub fn new(
        depositor: CollatorId,
        amount: Balance,
        deposited_at: BlockNumber,
        lock_period: BlockNumber,
    ) -> Self {
        Self {
            depositor,
            amount,
            deposited_at,
            lock_period,
            locked: true,
        }
    }

    /// Check if can withdraw
    pub fn can_withdraw(&self, current_block: BlockNumber) -> bool {
        !self.locked && current_block >= self.deposited_at + self.lock_period
    }

    /// Unlock deposit
    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

/// Security level calculator
pub struct SecurityCalculator {
    /// Minimum security ratio (basis points, e.g., 20000 = 2x)
    min_security_ratio: u32,
}

impl SecurityCalculator {
    /// Create new calculator
    pub fn new(min_security_ratio: u32) -> Self {
        Self { min_security_ratio }
    }

    /// Calculate required security for transfer value
    pub fn required_security(&self, transfer_value: Balance) -> Balance {
        transfer_value.saturating_mul(self.min_security_ratio as u128) / 10_000
    }

    /// Check if deposits provide sufficient security
    pub fn is_sufficient(
        &self,
        deposits: &[EconomicDeposit],
        transfer_value: Balance,
    ) -> bool {
        let total: Balance = deposits.iter().map(|d| d.amount).sum();
        let required = self.required_security(transfer_value);
        total >= required
    }

    /// Calculate total available security
    pub fn total_security(&self, deposits: &[EconomicDeposit]) -> Balance {
        deposits.iter().map(|d| d.amount).sum()
    }
}

impl Default for SecurityCalculator {
    fn default() -> Self {
        Self::new(20_000) // 2x transfer value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    #[test]
    fn test_economic_deposit() {
        let collator = AccountId32::new([1u8; 32]);
        let mut deposit = EconomicDeposit::new(collator, 1_000_000, 100, 1000);

        assert!(deposit.locked);
        assert!(!deposit.can_withdraw(500));

        deposit.unlock();
        assert!(!deposit.locked);
        assert!(deposit.can_withdraw(1101));
    }

    #[test]
    fn test_security_calculator() {
        let calc = SecurityCalculator::default();

        // For 1M transfer, requires 2M security
        let required = calc.required_security(1_000_000);
        assert_eq!(required, 2_000_000);
    }

    #[test]
    fn test_sufficient_security() {
        let calc = SecurityCalculator::default();
        let collator = AccountId32::new([1u8; 32]);

        let deposits = vec![
            EconomicDeposit::new(collator.clone(), 1_500_000, 100, 1000),
            EconomicDeposit::new(collator, 1_000_000, 100, 1000),
        ];

        // Total deposits = 2.5M, required for 1M transfer = 2M
        assert!(calc.is_sufficient(&deposits, 1_000_000));

        // Required for 2M transfer = 4M, insufficient
        assert!(!calc.is_sufficient(&deposits, 2_000_000));
    }

    #[test]
    fn test_total_security() {
        let calc = SecurityCalculator::default();
        let collator = AccountId32::new([1u8; 32]);

        let deposits = vec![
            EconomicDeposit::new(collator.clone(), 1_000_000, 100, 1000),
            EconomicDeposit::new(collator.clone(), 2_000_000, 100, 1000),
            EconomicDeposit::new(collator, 500_000, 100, 1000),
        ];

        assert_eq!(calc.total_security(&deposits), 3_500_000);
    }
}
