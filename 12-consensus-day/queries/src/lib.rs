//! Ëtrid Consensus-Day — Unified Queries
//!
//! Exposes read-only helper functions for proposals, votes, mint events,
//! and distributions.  Intended for dashboards, explorers, off-chain workers,
//! or runtime APIs that need aggregated governance data.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use sp_runtime::RuntimeDebug;

use consensus_day_proposal_system::{Proposals, ProposalRecord, ProposalStatus};
use consensus_day_voting_protocol::{VoteCount, Votes, Ballot, VoteRecord};
use consensus_day_minting_logic::{MintEvents, MintRecord};
use consensus_day_distribution::{Distributions, DistributionRecord};

/// -----------------------------
/// High-Level Query Structures
/// -----------------------------

#[derive(Clone, Eq, PartialEq, RuntimeDebug)]
pub struct ProposalSummary<AccountId> {
    pub id: u64,
    pub proposer: AccountId,
    pub status: ProposalStatus,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug)]
pub struct GovernanceSnapshot<AccountId, Balance> {
    pub proposals: Vec<ProposalSummary<AccountId>>,
    pub mints: Vec<MintRecord<AccountId, Balance>>,
    pub distributions: Vec<DistributionRecord<AccountId, Balance>>,
}

/// -----------------------------
/// Query Functions
/// -----------------------------

/// Return all proposals and their current status.
pub fn get_all_proposals<T: consensus_day_proposal_system::Config>() -> Vec<(u64, ProposalRecord<T::AccountId>)> {
    Proposals::<T>::iter().collect()
}

/// Return votes_for / votes_against for a given proposal.
pub fn get_vote_results<T: consensus_day_voting_protocol::Config>(proposal_id: u64) -> (u32, u32) {
    VoteCount::<T>::get(proposal_id)
}

/// Return every VoteRecord for a given proposal (for analytics).
pub fn get_vote_records<T: consensus_day_voting_protocol::Config>(proposal_id: u64) -> Vec<VoteRecord<T::AccountId>> {
    Votes::<T>::iter_prefix(proposal_id).map(|(_, v)| v).collect()
}

/// Return all minting events on-chain.
pub fn get_mint_history<T: consensus_day_minting_logic::Config>() -> Vec<(u64, MintRecord<T::AccountId, <<T as consensus_day_minting_logic::Config>::Currency as frame_support::traits::Currency<T::AccountId>>::Balance>)> {
    MintEvents::<T>::iter().collect()
}

/// Return a distribution record by mint ID, if any.
pub fn get_distribution_record<T: consensus_day_distribution::Config>(mint_id: u64)
    -> Option<DistributionRecord<T::AccountId, <<T as consensus_day_distribution::Config>::Currency as frame_support::traits::Currency<T::AccountId>>::Balance>> {
    Distributions::<T>::get(mint_id)
}

/// Aggregate everything into one snapshot for explorers or UI.
pub fn get_full_consensus_snapshot<
    TProp: consensus_day_proposal_system::Config,
    TVote: consensus_day_voting_protocol::Config<AccountId = <TProp as frame_system::Config>::AccountId>,
    TMint: consensus_day_minting_logic::Config<AccountId = <TProp as frame_system::Config>::AccountId>,
    TDist: consensus_day_distribution::Config<AccountId = <TProp as frame_system::Config>::AccountId>,
>(
) -> GovernanceSnapshot<
    TProp::AccountId,
    <<TMint as consensus_day_minting_logic::Config>::Currency as frame_support::traits::Currency<TMint::AccountId>>::Balance,
> {
    // Collect proposal summaries
    let proposals = Proposals::<TProp>::iter()
        .map(|(id, p)| ProposalSummary {
            id,
            proposer: p.proposer,
            status: p.status,
            votes_for: p.votes_for,
            votes_against: p.votes_against,
        })
        .collect::<Vec<_>>();

    // Collect all mint records
    let mints: Vec<MintRecord<_, _>> = MintEvents::<TMint>::iter().map(|(_, r)| r).collect();

    // Collect all distributions
    let distributions: Vec<DistributionRecord<_, _>> =
        Distributions::<TDist>::iter().map(|(_, r)| r).collect();

    GovernanceSnapshot {
        proposals,
        mints,
        distributions,
    }
}