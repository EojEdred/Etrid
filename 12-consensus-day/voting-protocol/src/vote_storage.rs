//! Vote storage and management functions

/// Reset all votes for a given proposal
pub fn reset_votes<T: crate::pallet::Config>(proposal_id: u64) {
    // Remove all votes for this proposal
    let _ = crate::pallet::Votes::<T>::clear_prefix(proposal_id, u32::MAX, None);

    // Reset vote count
    crate::pallet::VoteCount::<T>::remove(proposal_id);
}
