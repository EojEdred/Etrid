//! Runtime Config — contains configurable constants and helper traits
//! for quorum, majority, and other Consensus Day parameters.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Get;

/// Static runtime configuration parameters for voting.
pub struct DefaultConfig;

impl DefaultConfig {
    pub const DEFAULT_QUORUM_PERCENT: u8 = 40;     // Minimum % of total voters to reach quorum
    pub const DEFAULT_MAJORITY_PERCENT: u8 = 51;   // Majority threshold for approval
    pub const MAX_PROPOSAL_DESCRIPTION: usize = 4096;
}

/// Trait that can be implemented for runtime-level dynamic configuration (future DAO votes).
pub trait VotingParameters {
    fn quorum_percent() -> u8;
    fn majority_percent() -> u8;
}

impl VotingParameters for DefaultConfig {
    fn quorum_percent() -> u8 { Self::DEFAULT_QUORUM_PERCENT }
    fn majority_percent() -> u8 { Self::DEFAULT_MAJORITY_PERCENT }
}