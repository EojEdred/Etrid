//! # ASF Pallet Configuration for LINK-PBC Runtime
//!
//! This module configures the ASF (Adaptive Scale of Finality) consensus
//! for the Chainlink Partition Burst Chain.

use crate::Balance;
use crate::UNITS;
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
    pub const AsfMinValidatorStake: Balance = 64 * UNITS;

    /// Maximum validator stake (1,000,000 ÉTR)
    pub const AsfMaxValidatorStake: Balance = 1_000_000 * UNITS;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asf_parameters() {
        assert_eq!(AsfMaxCommitteeSize::get(), 21);
        assert_eq!(AsfEpochDuration::get(), 2400);
        assert_eq!(AsfMinValidatorStake::get(), 64 * UNITS);
    }
}
