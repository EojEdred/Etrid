//! Validation — reusable functions for checking voter eligibility,
//! quorum reach, and proposal state consistency.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::ensure;
use crate::{Error, Config, VoteCount};
use consensus_day_proposal_system::{ProposalStatus, Proposals, Participants};

/// Ensure a proposal exists and is currently active.
pub fn ensure_active_proposal<T: Config>(proposal_id: u64) -> Result<(), Error<T>> {
    let proposal = Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
    ensure!(proposal.status == ProposalStatus::Active, Error::<T>::ProposalNotActive);
    Ok(())
}

/// Ensure a voter is registered for Consensus Day.
pub fn ensure_registered_voter<T: Config>(who: &T::AccountId) -> Result<(), Error<T>> {
    ensure!(Participants::<T>::get(who), Error::<T>::NotRegistered);
    Ok(())
}

/// Check whether quorum has been reached.
pub fn has_quorum<T: Config>(proposal_id: u64, quorum_percent: u8) -> bool {
    let (yes, no) = VoteCount::<T>::get(proposal_id);
    let total_voters = Participants::<T>::iter().count() as u32;
    let total_votes = yes + no;
    total_voters > 0 && (total_votes * 100 / total_voters) >= quorum_percent as u32
}