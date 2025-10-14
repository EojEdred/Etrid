//! Runtime — provides runtime API definitions for external query interfaces
//! (mobile wallet, explorer, governance dashboards)

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use consensus_day_proposal_system::{ProposalStatus};
use crate::queries;

/// Runtime API trait exposing read-only functions.
pub trait VotingRuntimeApi<AccountId> {
    fn get_vote_tally(proposal_id: u64) -> (u32, u32);
    fn get_proposal_status(proposal_id: u64) -> Option<ProposalStatus>;
    fn get_all_voters(proposal_id: u64) -> Vec<AccountId>;
}

/// Default implementation pulling from queries.rs
pub struct VotingRuntime<T>(sp_std::marker::PhantomData<T>);

impl<T: crate::Config> VotingRuntimeApi<T::AccountId> for VotingRuntime<T> {
    fn get_vote_tally(proposal_id: u64) -> (u32, u32) {
        queries::get_vote_tally::<T>(proposal_id)
    }

    fn get_proposal_status(proposal_id: u64) -> Option<ProposalStatus> {
        queries::get_proposal_status::<T>(proposal_id)
    }

    fn get_all_voters(proposal_id: u64) -> Vec<T::AccountId> {
        queries::get_votes_by_proposal::<T>(proposal_id)
            .into_iter()
            .map(|(a, _)| a)
            .collect()
    }
}