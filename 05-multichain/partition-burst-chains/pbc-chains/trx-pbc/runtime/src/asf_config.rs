//! # ASF Pallet Configuration for TRX-PBC Runtime
//!
//! This module configures the ASF (Adaptive Scale of Finality) consensus
//! for the TRX Partition Burst Chain. ASF provides:
//!
//! - HotStuff Byzantine Fault Tolerant consensus
//! - Ascending scale of finality (5 levels: 0-4)
//! - PPFA (Probabilistic Permissioned Finality Authorities) rotation
//! - Stake-weighted voting with slashing
//!
//! ## Configuration Values (from Ivory Papers)
//!
//! - **MaxCommitteeSize**: 21 validators (optimal for BFT)
//! - **EpochDuration**: 2400 blocks (~4 hours at 6s blocks)
//! - **MinValidatorStake**: 64 ETR (Validity Node minimum)

use crate::constants::UNITS;
use crate::Balance;
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

/// ASF Slashing Interface - integrates with Ëtrid Staking
///
/// This interface allows ASF consensus to slash validators for:
/// - Equivocation (double-signing blocks)
/// - Offline/unresponsive behavior
/// - Invalid certificate generation

/// ASF Randomness Source - provides randomness for PPFA rotation
///
/// Uses the same randomness source as other pallets for consistency.
pub struct AsfRandomnessSource;
