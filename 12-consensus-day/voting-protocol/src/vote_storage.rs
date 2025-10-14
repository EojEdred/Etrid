//! Vote Storage — encapsulates read/write operations on the Votes and VoteCount
//! storage maps for consistency and maintainability.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::storage::{StorageDoubleMap, StorageMap};
use sp_std::vec::Vec;
use crate::{Votes, VoteCount, Ballot, VoteRecord};

/// Count all votes (yes, no, abstain) for a proposal.
pub fn aggregate_votes<T: crate::Config>(proposal_id: u64) -> (u32, u32, u32) {
    let mut yes = 0;
    let mut no = 0;
    let mut abstain = 0;
    for (_voter, record) in Votes::<T>::iter_prefix(proposal_id) {
        match record.ballot {
            Ballot::Yes => yes += 1,
            Ballot::No => no += 1,
            Ballot::Abstain => abstain += 1,
        }
    }
    (yes, no, abstain)
}

/// Reset all vote data for a proposal (after Consensus Day execution).
pub fn reset_votes<T: crate::Config>(proposal_id: u64) {
    Votes::<T>::clear_prefix(proposal_id, u32::MAX, None);
    VoteCount::<T>::remove(proposal_id);
}

/// Return all VoteRecord entries for analytics.
pub fn export_vote_records<T: crate::Config>(proposal_id: u64) -> Vec<VoteRecord<T::AccountId>> {
    Votes::<T>::iter_prefix(proposal_id)
        .map(|(_, record)| record)
        .collect()
}