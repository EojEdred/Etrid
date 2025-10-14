//! Queries — provides runtime-facing and offchain helper functions
//! for retrieving voting data, tallies, and results.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::storage::StorageDoubleMap;
use sp_std::vec::Vec;

use crate::{Votes, VoteCount, Ballot};
use consensus_day_proposal_system::{Proposals, ProposalRecord, ProposalStatus};

/// Get all votes cast for a given proposal.
pub fn get_votes_by_proposal<T: crate::Config>(proposal_id: u64) -> Vec<(T::AccountId, Ballot)> {
    Votes::<T>::iter_prefix(proposal_id)
        .map(|(who, record)| (who, record.ballot))
        .collect()
}

/// Return the total "yes" and "no" counts for a given proposal.
pub fn get_vote_tally<T: crate::Config>(proposal_id: u64) -> (u32, u32) {
    VoteCount::<T>::get(proposal_id)
}

/// Return the proposal outcome status (Approved, Rejected, Pending, etc.)
pub fn get_proposal_status<T: crate::Config>(proposal_id: u64) -> Option<ProposalStatus> {
    Proposals::<T>::get(proposal_id).map(|p: ProposalRecord<T::AccountId>| p.status)
}