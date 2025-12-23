//! # ASF Pallet Configuration for DOGE-PBC Runtime
//!
//! This module configures the ASF (Adaptive Scale of Finality) consensus
//! for the Dogecoin Partition Burst Chain.

use crate::{Runtime, Balances, RuntimeEvent, AccountId, Balance};
use frame_support::parameter_types;
use sp_runtime::Perbill;

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

impl asf_algorithm_pbc::SlashingInterface<AccountId, Balance> for AsfSlashingInterface {
    fn slash_validator(
        validator: &AccountId,
        amount: Balance,
        reason: asf_algorithm_pbc::SlashReason,
    ) -> Result<(), sp_runtime::DispatchError> {
        use frame_support::traits::Currency;

        log::warn!(
            "🔴 ASF: Slashing validator {:?} for {:?}, amount: {}",
            validator,
            reason,
            amount
        );

        let _ = Balances::slash_reserved(validator, amount);
        Ok(())
    }

    fn is_validator_active(validator: &AccountId) -> bool {
        crate::ValidatorCommittee::is_validator_active(validator)
    }

    fn get_validator_stake(validator: &AccountId) -> Balance {
        crate::EtridStaking::get_validator_stake(validator)
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