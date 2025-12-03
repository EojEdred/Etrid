//! # Governance Runtime API
//!
//! This crate defines the Runtime API for querying ËTRID governance state.
//! Allows the node RPC layer to query proposals, votes, and Consensus Day phase
//! without directly accessing runtime storage.
//!
//! ## Usage
//!
//! From the node RPC layer:
//!
//! ```rust,ignore
//! use governance_runtime_api::GovernanceApi;
//!
//! // Query all proposals
//! let proposals = client.runtime_api()
//!     .get_proposals(at_hash)?;
//!
//! // Get current Consensus Day phase
//! let phase = client.runtime_api()
//!     .get_consensus_day_phase(at_hash)?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

/// Proposal information for RPC
#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct ProposalInfo<AccountId> {
    pub id: u64,
    pub proposer: AccountId,
    pub title: Vec<u8>,
    pub description: Vec<u8>,
    pub category: u8,
    pub status: u8,
    pub votes_for: u128,
    pub votes_against: u128,
    pub votes_abstain: u128,
    pub created_at: u64,
    pub approved: bool,
    pub executed: bool,
}

/// Consensus Day phase information
#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct ConsensusDayPhaseInfo {
    /// Current phase: 0=Inactive, 1=Registration, 2=Voting, 3=Minting, 4=Distribution
    pub phase: u8,
    /// Block when current phase started
    pub phase_start_block: u32,
    /// Block when Consensus Day event started
    pub event_start_block: u32,
    /// Year/iteration of Consensus Day
    pub year: u32,
    /// Estimated blocks remaining in current phase
    pub blocks_remaining: u32,
}

/// Voting power information for an account
#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct VotingPowerInfo {
    pub staked_amount: u128,
    pub voting_power: u128,
    pub participation_history: u32,
    pub can_vote: bool,
}

/// Vote record for RPC
#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct VoteInfo<AccountId> {
    pub voter: AccountId,
    pub proposal_id: u64,
    pub vote_type: u8,  // 0=Yes, 1=No, 2=Abstain
    pub voting_power: u128,
}

/// Director candidate information
#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct DirectorInfo<AccountId> {
    pub account: AccountId,
    pub stake: u128,
    pub votes: u128,
}

/// Distribution schedule information
#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct DistributionInfo {
    pub total_minted: u128,
    pub foundation_share: u128,
    pub directors_share: u128,
    pub validators_share: u128,
    pub voters_share: u128,
    pub fees_distributed: u128,
}

/// Governance statistics
#[derive(Clone, Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub struct GovernanceStats {
    pub total_proposals: u64,
    pub active_proposals: u64,
    pub approved_proposals: u64,
    pub rejected_proposals: u64,
    pub total_votes_cast: u128,
    pub unique_voters: u32,
    pub circulating_supply: u128,
    pub quorum_threshold: u32,
}

/// Runtime API for governance and Consensus Day queries
sp_api::decl_runtime_apis! {
    /// API for querying ËTRID governance state
    pub trait GovernanceApi<AccountId>
    where
        AccountId: codec::Codec,
    {
        /// Get all proposals
        ///
        /// Returns a list of all governance proposals.
        fn get_proposals() -> Vec<ProposalInfo<AccountId>>;

        /// Get specific proposal by ID
        fn get_proposal(id: u64) -> Option<ProposalInfo<AccountId>>;

        /// Get current Consensus Day phase
        fn get_consensus_day_phase() -> ConsensusDayPhaseInfo;

        /// Check if Consensus Day is active
        fn is_consensus_day_active() -> bool;

        /// Get voting power for an account
        fn get_voting_power(account: AccountId) -> VotingPowerInfo;

        /// Get votes for a proposal
        fn get_proposal_votes(proposal_id: u64) -> Vec<VoteInfo<AccountId>>;

        /// Get vote counts for a proposal (yes, no, abstain)
        fn get_vote_counts(proposal_id: u64) -> (u128, u128, u128);

        /// Get elected directors
        fn get_elected_directors() -> Vec<AccountId>;

        /// Get director candidates
        fn get_director_candidates() -> Vec<DirectorInfo<AccountId>>;

        /// Get distribution info
        fn get_distribution_info() -> DistributionInfo;

        /// Get governance statistics
        fn get_governance_stats() -> GovernanceStats;

        /// Get pending participation reward for account
        fn get_participation_reward(account: AccountId) -> u128;

        /// Get current inflation rate (basis points)
        fn get_inflation_rate() -> u32;
    }
}
