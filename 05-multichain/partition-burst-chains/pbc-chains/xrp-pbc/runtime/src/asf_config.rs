//! # ASF Pallet Configuration for XRP-PBC Runtime
//!
//! This module configures the ASF (Adaptive Scale of Finality) consensus
//! for the XRP Ledger Partition Burst Chain.

use crate::{Runtime, Balances, RuntimeEvent, AccountId, Balance, EtridStaking};
use frame_support::parameter_types;
use frame_support::traits::ReservableCurrency;
use sp_runtime::Perbill;
use sp_std::vec::Vec;

// ═══════════════════════════════════════════════════════════════════════════════
// ASF COMMITTEE PARAMETERS (Standard across all PBCs)
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    /// Maximum committee size (21 validators)
    pub const AsfMaxCommitteeSize: u32 = 21;

    /// Epoch duration (2400 blocks = ~4 hours at 6s blocks)
    pub const AsfEpochDuration: u32 = 2400;

    /// Minimum validator stake (64 ÉTR)
    pub const AsfMinValidatorStake: Balance = 64_000_000_000_000_000_000_000; // 64 ÉTR with 18 decimals

    /// Maximum validator stake (1,000,000 ÉTR)
    pub const AsfMaxValidatorStake: Balance = 1_000_000_000_000_000_000_000_000_000; // 1M ÉTR with 18 decimals

    /// Slash percentage for equivocation (50%)
    pub const AsfEquivocationSlashPercent: Perbill = Perbill::from_percent(50);

    /// Slash percentage for offline validators (5%)
    pub const AsfOfflineSlashPercent: Perbill = Perbill::from_percent(5);

    /// Maximum offline blocks before slashing (100 blocks)
    pub const AsfMaxOfflineBlocks: u32 = 100;

    /// Minimum reputation score (70/100)
    pub const AsfMinReputationScore: u32 = 70;

    /// PPFA rotation frequency (200 blocks)
    pub const AsfPpfaRotationBlocks: u32 = 200;

    /// Certificate expiry (14400 blocks = 24 hours)
    pub const AsfCertificateExpiry: u32 = 14400;
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLASHING INTERFACE (Active - not commented)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AsfSlashingInterface;

impl AsfSlashingInterface {
    pub fn slash_validator(
        validator: &AccountId,
        amount: Balance,
    ) -> Result<(), sp_runtime::DispatchError> {
        use frame_support::traits::Currency;

        // Skip logging to avoid complex formatting dependencies

        let _ = <Balances as ReservableCurrency<AccountId>>::slash_reserved(validator, amount);
        Ok(())
    }

    pub fn is_validator_active(validator: &AccountId) -> bool {
        crate::ValidatorCommittee::is_validator_active(validator)
    }

    pub fn get_validator_stake(_validator: &AccountId) -> Balance {
        // Return a default stake value as a placeholder
        100_000_000_000_000_000_000 // 100 ETR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asf_parameters() {
        assert_eq!(AsfMaxCommitteeSize::get(), 21);
        assert_eq!(AsfEpochDuration::get(), 2400);
        assert_eq!(AsfMinValidatorStake::get(), 64_000_000_000_000_000_000_000);
    }
}