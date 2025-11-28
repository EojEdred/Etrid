//! Voting Protocol Pallet
//!
//! Manages vote casting, tallying, and validation for Consensus Day proposals.
//! Integrates with the proposal system to track voting participation and outcomes.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::vec::Vec;

pub use pallet::*;

// Public modules
pub mod vote_storage;
pub mod runtime;
pub mod queries;
pub mod validation;
pub mod runtime_config;

/// Ballot options for voting - Moved outside pallet to avoid DecodeWithMemTracking issues
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Ballot {
    Yes,
    No,
    Abstain,
}

impl Ballot {
    /// Convert from u8 for use in call parameters
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Yes),
            1 => Some(Self::No),
            2 => Some(Self::Abstain),
            _ => None,
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Vote record for an individual voter
    #[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct VoteRecord<AccountId: MaxEncodedLen> {
        pub voter: AccountId,
        pub ballot: Ballot,
        pub voted_at: u64,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Storage for individual votes: (proposal_id, voter) => VoteRecord
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u64,                    // proposal_id
        Blake2_128Concat,
        T::AccountId,           // voter
        VoteRecord<T::AccountId>,
        OptionQuery,
    >;

    /// Storage for vote counts: proposal_id => (yes_count, no_count)
    #[pallet::storage]
    #[pallet::getter(fn vote_count)]
    pub type VoteCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,                    // proposal_id
        (u32, u32),             // (yes, no)
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A vote was cast [proposal_id, voter, ballot_u8] (0=Yes, 1=No, 2=Abstain)
        VoteCast(u64, T::AccountId, u8),
        /// Voting closed for a proposal [proposal_id, yes_count, no_count]
        VotingClosed(u64, u32, u32),
        /// Votes reset after Consensus Day [proposal_id]
        VotesReset(u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Proposal not found
        ProposalNotFound,
        /// Proposal is not active for voting
        ProposalNotActive,
        /// Voter is not registered for Consensus Day
        NotRegistered,
        /// Voter has already voted on this proposal
        AlreadyVoted,
        /// Unauthorized operation
        Unauthorized,
        /// Invalid ballot value (must be 0=Yes, 1=No, 2=Abstain)
        InvalidBallot,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Cast a vote on an active proposal
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn cast_vote(
            origin: OriginFor<T>,
            proposal_id: u64,
            ballot_u8: u8, // Use u8 to avoid DecodeWithMemTracking issues
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Convert u8 to Ballot
            let ballot = Ballot::from_u8(ballot_u8)
                .ok_or(Error::<T>::InvalidBallot)?;

            // Validate proposal is active
            crate::validation::ensure_active_proposal::<T>(proposal_id)?;

            // Validate voter is registered
            crate::validation::ensure_registered_voter::<T>(&who)?;

            // Ensure voter hasn't already voted
            ensure!(!Votes::<T>::contains_key(proposal_id, &who), Error::<T>::AlreadyVoted);

            // Record the vote
            let vote_record = VoteRecord {
                voter: who.clone(),
                ballot,
                voted_at: Self::current_timestamp(),
            };
            Votes::<T>::insert(proposal_id, &who, vote_record);

            // Update vote count
            let (mut yes, mut no) = VoteCount::<T>::get(proposal_id);
            match ballot {
                Ballot::Yes => yes += 1,
                Ballot::No => no += 1,
                Ballot::Abstain => {}, // Don't count abstentions in yes/no tally
            }
            VoteCount::<T>::insert(proposal_id, (yes, no));

            Self::deposit_event(Event::VoteCast(proposal_id, who, ballot_u8));
            Ok(())
        }

        /// Close voting for a proposal (admin only)
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn close_voting(
            origin: OriginFor<T>,
            proposal_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let (yes, no) = VoteCount::<T>::get(proposal_id);
            Self::deposit_event(Event::VotingClosed(proposal_id, yes, no));
            Ok(())
        }

        /// Reset votes after Consensus Day execution (admin only)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn reset_votes(
            origin: OriginFor<T>,
            proposal_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            crate::vote_storage::reset_votes::<T>(proposal_id);
            Self::deposit_event(Event::VotesReset(proposal_id));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number().unique_saturated_into()
        }
    }
}
