//! # ASF Pallet Configuration for FlareChain Runtime
//!
//! This module configures the ASF (Adaptive Scale of Finality) consensus
//! for the FlareChain relay chain. ASF provides:
//!
//! - HotStuff Byzantine Fault Tolerant consensus
//! - Ascending scale of finality (5 levels: 0-4)
//! - PPFA (Probabilistic Permissioned Finality Authorities) rotation
//! - Stake-weighted voting with slashing
//!
//! ## Phase 2 Hybrid Mode
//!
//! During Phase 2, ASF runs in hybrid mode alongside GRANDPA:
//! - GRANDPA provides immediate BFT finality (2/3 threshold)
//! - ASF accumulates validity certificates for ascending finality
//! - Both systems share the same validator set via `pallet_validator_committee`
//!
//! ## Configuration Values (from Ivory Papers)
//!
//! - **MaxCommitteeSize**: 21 validators (optimal for BFT)
//! - **EpochDuration**: 2400 blocks (~4 hours at 6s blocks)
//! - **MinValidatorStake**: 64 ETR (Validity Node minimum)
//! - **SlashingInterface**: EtridStaking pallet

use crate::{Runtime, EtridStaking, Balances, RuntimeEvent, AccountId, Balance, UNITS};
use frame_support::parameter_types;
use sp_runtime::Perbill;

// ═══════════════════════════════════════════════════════════════════════════════
// ASF COMMITTEE PARAMETERS
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    /// Maximum committee size (21 validators as per Ivory Papers)
    ///
    /// This is the optimal size for BFT consensus:
    /// - Large enough for decentralization
    /// - Small enough for fast consensus rounds
    /// - 2/3 threshold = 15 validators needed for finality
    pub const AsfMaxCommitteeSize: u32 = 21;

    /// Epoch duration in blocks (2400 blocks = ~4 hours at 6s blocks)
    ///
    /// An epoch defines:
    /// - Validator set rotation period
    /// - PPFA authority rotation
    /// - Reward distribution cycle
    pub const AsfEpochDuration: u32 = 2400;

    /// Minimum stake required to be a validator (64 ETR)
    ///
    /// This is the Validity Node minimum stake as defined in the Ivory Papers.
    /// Validators must maintain at least this amount to participate in consensus.
    pub const AsfMinValidatorStake: Balance = 64 * UNITS; // 64 ETR

    /// Maximum validator stake (1,000,000 ETR)
    ///
    /// Prevents single validators from dominating stake-weighted voting.
    pub const AsfMaxValidatorStake: Balance = 1_000_000 * UNITS;

    /// Slash percentage for equivocation (50%)
    ///
    /// Validators who sign conflicting blocks (equivocation) lose 50% of stake.
    /// This is a severe penalty to discourage Byzantine behavior.
    pub const AsfEquivocationSlashPercent: Perbill = Perbill::from_percent(50);

    /// Slash percentage for offline/unresponsive validators (5%)
    ///
    /// Validators who miss too many blocks get a smaller slash.
    pub const AsfOfflineSlashPercent: Perbill = Perbill::from_percent(5);

    /// Maximum offline blocks before slashing (100 blocks)
    ///
    /// Validators can miss up to 100 consecutive blocks (~10 minutes)
    /// before being considered offline and slashed.
    pub const AsfMaxOfflineBlocks: u32 = 100;

    /// Minimum reputation score to remain active (70/100)
    ///
    /// Validators below this score are excluded from consensus.
    pub const AsfMinReputationScore: u32 = 70;

    /// Blocks per PPFA rotation (200 blocks)
    ///
    /// PPFA rotates which validators are active finality authorities
    /// every 200 blocks (~20 minutes) to ensure all validators participate.
    pub const AsfPpfaRotationBlocks: u32 = 200;

    /// Finality certificate expiry (14400 blocks = 24 hours)
    ///
    /// Validity certificates older than this are discarded.
    pub const AsfCertificateExpiry: u32 = 14400;
}

// ═══════════════════════════════════════════════════════════════════════════════
// PHASE 2 HYBRID MODE CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════
//
// During Phase 2, ASF and GRANDPA run in parallel:
//
// 1. Block Production:
//    - ASF HotStuff consensus determines block authors
//    - PPFA rotation selects proposer for each block
//
// 2. Finality:
//    - GRANDPA provides immediate BFT finality (existing system)
//    - ASF accumulates validity certificates in parallel
//    - Blocks get ascending finality levels as certificates accumulate
//
// 3. Validator Set:
//    - `pallet_validator_committee` manages single validator set
//    - Both GRANDPA and ASF use same validators
//    - Session keys include both `grandpa` and `asf` keys
//
// 4. Slashing:
//    - ASF slashing for equivocation/offline
//    - GRANDPA equivocation handled by existing system
//    - `pallet_etrid_staking` is the slashing interface
//
// 5. Rewards:
//    - `pallet_validator_rewards` distributes rewards for both systems
//
// ═══════════════════════════════════════════════════════════════════════════════

/// ASF Slashing Interface - integrates with Ëtrid Staking
///
/// This interface allows ASF consensus to slash validators for:
/// - Equivocation (double-signing blocks)
/// - Offline/unresponsive behavior
/// - Invalid certificate generation
pub struct AsfSlashingInterface;

impl asf_algorithm::SlashingInterface<AccountId, Balance> for AsfSlashingInterface {
    fn slash_validator(
        validator: &AccountId,
        amount: Balance,
        reason: asf_algorithm::SlashReason,
    ) -> Result<(), sp_runtime::DispatchError> {
        use frame_support::traits::Currency;

        // Log the slashing event
        log::warn!(
            "ASF: Slashing validator {:?} for {:?}, amount: {}",
            validator,
            reason,
            amount
        );

        // Slash via EtridStaking pallet
        // Note: This is a simplified implementation
        // The actual slashing logic should integrate with pallet_etrid_staking
        let _ = Balances::slash(validator, amount);

        Ok(())
    }

    fn is_validator_active(validator: &AccountId) -> bool {
        // Check if validator is in the active set
        // This integrates with pallet_validator_committee
        crate::ValidatorCommittee::is_validator_active(validator)
    }

    fn get_validator_stake(validator: &AccountId) -> Balance {
        // Get validator stake from staking pallet
        EtridStaking::get_validator_stake(validator)
    }
}

/// ASF Randomness Source - provides randomness for PPFA rotation
///
/// Uses the same randomness source as other pallets for consistency.
pub struct AsfRandomnessSource;

impl asf_algorithm::RandomnessSource for AsfRandomnessSource {
    fn random_seed(block_number: u64) -> [u8; 32] {
        use sp_runtime::traits::Hash;
        use frame_support::traits::Randomness;

        // Use collective flip randomness (same as pallet_consensus)
        let subject = b"asf-ppfa-rotation";
        let random = crate::RandomnessCollectiveFlip::random(subject);

        // Mix in block number for determinism
        let mut seed = random.0;
        let block_bytes = block_number.to_le_bytes();
        for (i, byte) in block_bytes.iter().enumerate() {
            seed[i] ^= byte;
        }

        seed
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUTURE: PALLET ASF CONFIGURATION (Phase 3+)
// ═══════════════════════════════════════════════════════════════════════════════
//
// When a full `pallet-asf` is created, it will use this configuration:
//
// ```rust
// impl pallet_asf::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type MaxCommitteeSize = AsfMaxCommitteeSize;
//     type EpochDuration = AsfEpochDuration;
//     type MinValidatorStake = AsfMinValidatorStake;
//     type MaxValidatorStake = AsfMaxValidatorStake;
//     type SlashingInterface = AsfSlashingInterface;
//     type RandomnessSource = AsfRandomnessSource;
//     type EquivocationSlash = AsfEquivocationSlashPercent;
//     type OfflineSlash = AsfOfflineSlashPercent;
//     type MaxOfflineBlocks = AsfMaxOfflineBlocks;
//     type MinReputationScore = AsfMinReputationScore;
//     type PpfaRotationBlocks = AsfPpfaRotationBlocks;
//     type CertificateExpiry = AsfCertificateExpiry;
//     type WeightInfo = ();
// }
// ```
//
// For now, ASF consensus logic is handled by:
// - `asf-algorithm` crate (core consensus math)
// - `pallet_validator_committee` (validator set management)
// - `client/consensus-asf` (block production)
// - `finality-gadget` (finality certificate aggregation)
//
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asf_parameters() {
        // Verify committee size is 21
        assert_eq!(AsfMaxCommitteeSize::get(), 21);

        // Verify epoch duration is 2400 blocks (~4 hours)
        assert_eq!(AsfEpochDuration::get(), 2400);

        // Verify minimum stake is 64 ETR
        assert_eq!(AsfMinValidatorStake::get(), 64 * UNITS);

        // Verify BFT threshold for 21 validators
        let threshold = asf_algorithm::bft_threshold(AsfMaxCommitteeSize::get());
        assert_eq!(threshold, 15); // 2/3 of 21 = 14, +1 = 15
    }

    #[test]
    fn test_slashing_percentages() {
        // Equivocation should be 50%
        assert_eq!(
            AsfEquivocationSlashPercent::get(),
            Perbill::from_percent(50)
        );

        // Offline should be 5%
        assert_eq!(AsfOfflineSlashPercent::get(), Perbill::from_percent(5));
    }

    #[test]
    fn test_ppfa_rotation() {
        // PPFA should rotate every 200 blocks
        assert_eq!(AsfPpfaRotationBlocks::get(), 200);

        // Verify rotation frequency relative to epoch
        let rotations_per_epoch = AsfEpochDuration::get() / AsfPpfaRotationBlocks::get();
        assert_eq!(rotations_per_epoch, 12); // 2400 / 200 = 12 rotations per epoch
    }
}
