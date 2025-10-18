//! Ëtrid Consensus-Day — Proposal System
//!
//! Handles registration of eligible participants and submission of proposals
//! for the annual Consensus Day event (Dec 1).
//!
//! Future linkages:
//! • Voting Protocol module → cast/tally votes
//! • Distribution module → reward active voters
//! • Minting Logic → fiscal mint post-vote

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency, Get},
    dispatch::DispatchResult,
};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, traits::UniqueSaturatedInto};
use sp_std::vec::Vec;

// -------------------- CORE TYPES --------------------

/// Classification of proposals that can be registered.
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ProposalCategory {
    ProtocolUpgrade = 0,
    EconomicAdjustment = 1,
    DirectorElection = 2,
    TreasuryAllocation = 3,
    General = 4,
}

impl ProposalCategory {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::ProtocolUpgrade),
            1 => Some(Self::EconomicAdjustment),
            2 => Some(Self::DirectorElection),
            3 => Some(Self::TreasuryAllocation),
            4 => Some(Self::General),
            _ => None,
        }
    }
}

/// Current lifecycle state of a proposal.
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ProposalStatus {
    Pending = 0,
    Active = 1,
    Approved = 2,
    Rejected = 3,
    Executed = 4,
}

impl ProposalStatus {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Pending),
            1 => Some(Self::Active),
            2 => Some(Self::Approved),
            3 => Some(Self::Rejected),
            4 => Some(Self::Executed),
            _ => None,
        }
    }
}

/// Core on-chain record for a Consensus Day proposal.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProposalRecord<AccountId> {
    pub proposer: AccountId,
    pub category: ProposalCategory,
    pub title: Vec<u8>,
    pub description: Vec<u8>,
    pub created_at: u64,
    pub status: ProposalStatus,
    pub votes_for: u32,
    pub votes_against: u32,
}

// -------------------- PALLET LOGIC --------------------

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Combined event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Currency used for registration deposits.
        type Currency: ReservableCurrency<Self::AccountId>;
        /// Minimum deposit required to register or submit a proposal.
        #[pallet::constant]
        type RegistrationDeposit: Get<BalanceOf<Self>>;
    }

    /// Registry of all proposals by an auto-incremented ID.
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, ProposalRecord<T::AccountId>>;

    /// Counter for the next proposal ID.
    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Set of registered Consensus Day participants.
    #[pallet::storage]
    #[pallet::getter(fn participants)]
    pub type Participants<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A participant has been registered for Consensus Day
        ParticipantRegistered(T::AccountId),
        /// A new proposal has been submitted (proposal_id, submitter)
        ProposalSubmitted(u64, T::AccountId),
        /// A proposal's status has been updated (proposal_id)
        ProposalStatusChanged(u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyRegistered,
        NotRegistered,
        ProposalNotFound,
        InvalidStatusChange,
        InsufficientDeposit,
        InvalidCategory,
        InvalidStatus,
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register an account for the upcoming Consensus Day.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_participant(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!Participants::<T>::contains_key(&who), Error::<T>::AlreadyRegistered);

            // Reserve deposit to ensure commitment
            let deposit = T::RegistrationDeposit::get();
            ensure!(
                T::Currency::can_reserve(&who, deposit),
                Error::<T>::InsufficientDeposit
            );
            T::Currency::reserve(&who, deposit)?;

            Participants::<T>::insert(&who, true);
            Self::deposit_event(Event::<T>::ParticipantRegistered(who));
            Ok(())
        }

        /// Submit a new proposal to be voted on during Consensus Day.
        #[pallet::call_index(1)]
        #[pallet::weight(20_000)]
        pub fn submit_proposal(
            origin: OriginFor<T>,
            category: u8,
            title: Vec<u8>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Participants::<T>::get(&who), Error::<T>::NotRegistered);

            let category_enum = ProposalCategory::from_u8(category)
                .ok_or(Error::<T>::InvalidCategory)?;

            let id = NextProposalId::<T>::get();
            let now: u64 = <frame_system::Pallet<T>>::block_number().unique_saturated_into();

            let record = ProposalRecord {
                proposer: who.clone(),
                category: category_enum,
                title,
                description,
                created_at: now,
                status: ProposalStatus::Pending,
                votes_for: 0,
                votes_against: 0,
            };

            Proposals::<T>::insert(id, record);
            NextProposalId::<T>::put(id + 1);
            Self::deposit_event(Event::<T>::ProposalSubmitted(id, who));
            Ok(())
        }

        /// Update a proposal's status (root call – Foundation DAO or Governance pallet).
        #[pallet::call_index(2)]
        #[pallet::weight(5_000)]
        pub fn update_status(
            origin: OriginFor<T>,
            proposal_id: u64,
            new_status: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let new_status_enum = ProposalStatus::from_u8(new_status)
                .ok_or(Error::<T>::InvalidStatus)?;

            Proposals::<T>::try_mutate(proposal_id, |maybe_p| -> DispatchResult {
                let proposal = maybe_p.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                // Basic sanity: avoid illegal backward transitions
                ensure!(
                    matches!(
                        (proposal.status, new_status_enum),
                        (ProposalStatus::Pending, ProposalStatus::Active)
                            | (ProposalStatus::Active, ProposalStatus::Approved)
                            | (ProposalStatus::Active, ProposalStatus::Rejected)
                            | (ProposalStatus::Approved, ProposalStatus::Executed)
                    ),
                    Error::<T>::InvalidStatusChange
                );
                proposal.status = new_status_enum;
                Self::deposit_event(Event::<T>::ProposalStatusChanged(proposal_id));
                Ok(())
            })
        }
    }
}