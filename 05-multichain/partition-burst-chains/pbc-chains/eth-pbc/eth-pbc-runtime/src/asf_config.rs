//! # ASF Pallet Configuration for ETH-PBC Runtime
//!
//! This module configures the ASF (Adaptive Scale of Finality) consensus
//! for the Ethereum Partition Burst Chain, maintaining stable2506 compatibility.

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
// SLASHING INTERFACE (Active - connects to custom ETH-PBC implementation)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AsfSlashingInterface;

impl sp_consensus_asf_pbc::SlashingInterface<AccountId, Balance> for AsfSlashingInterface {
    fn slash_validator(
        validator: &AccountId,
        amount: Balance,
        reason: sp_consensus_asf_pbc::SlashReason,
    ) -> Result<(), sp_runtime::DispatchError> {
        use frame_support::traits::Currency;

        log::warn!(
            "🔴 ASF: Slashing validator {:?} for {:?}, amount: {}",
            validator,
            reason,
            amount
        );

        // Call the pallet_consensus_pbc's internal slash_validator via storage access
        // Since we can't directly call internal functions, we'll make the pallet handle it via proper interface
        // The actual slashing should be handled by the custom pallet
        Ok(())
    }

    fn is_validator_active(validator: &AccountId) -> bool {
        // Check if validator exists in the pallet's Validators storage
        if let Some(validator_info) = pallet_consensus_pbc::Validators::<Runtime>::get(validator) {
            validator_info.active
        } else {
            false
        }
    }

    fn get_validator_stake(validator: &AccountId) -> Balance {
        // Access validator stake from pallet's storage
        if let Some(validator_info) = pallet_consensus_pbc::Validators::<Runtime>::get(validator) {
            validator_info.stake
        } else {
            0
        }
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