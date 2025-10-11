//! Etrid Governance Pallet - E³20
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Time},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    use sp_runtime::traits::Zero;

    pub type ProposalId = u32;
    pub type MomentOf<T> = <<T as Config>::Time as Time>::Moment;

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ProposalStatus {
        Active,
        Passed,
        Rejected,
        Cancelled,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Eq, PartialEq, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct Proposal<T: Config> {
        pub id: ProposalId,
        pub title: BoundedVec<u8, ConstU32<256>>,
        pub description: BoundedVec<u8, ConstU32<1024>>,
        pub proposer: T::AccountId,
        pub created_at: MomentOf<T>,
        pub voting_ends: MomentOf<T>,
        pub votes_for: BalanceOf<T>,
        pub votes_against: BalanceOf<T>,
        pub status: ProposalStatus,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type Time: Time;
        type ProposalDuration: Get<MomentOf<Self>>;
        type MinProposalStake: Get<BalanceOf<Self>>;
    }

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::type_value]
    pub fn DefaultForNextProposalId() -> ProposalId { 0 }

    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T> = StorageValue<_, ProposalId, ValueQuery, DefaultForNextProposalId>;

    #[pallet::type_value]
    pub fn DefaultForLastConsensusDay<T: Config>() -> MomentOf<T> {
        MomentOf::<T>::default()
    }

    #[pallet::storage]
    #[pallet::getter(fn consensus_day)]
    pub type LastConsensusDay<T> = StorageValue<_, MomentOf<T>, ValueQuery, DefaultForLastConsensusDay<T>>;

    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, ProposalId, Proposal<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProposalCreated(ProposalId, T::AccountId),
        Voted(ProposalId, T::AccountId, bool, BalanceOf<T>),
        ProposalPassed(ProposalId),
        ProposalRejected(ProposalId),
        ProposalCancelled(ProposalId),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProposalNotFound,
        VotingClosed,
        AlreadyFinalized,
        NotProposer,
        InsufficientStake,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            let now = T::Time::now();
            let end = now + T::ProposalDuration::get();

            T::Currency::reserve(&proposer, T::MinProposalStake::get())?;

            let id = NextProposalId::<T>::get();
            NextProposalId::<T>::put(id + 1);

            let bounded_title = BoundedVec::<u8, ConstU32<256>>::try_from(title)
                .map_err(|_| Error::<T>::ProposalNotFound)?;
            let bounded_desc = BoundedVec::<u8, ConstU32<1024>>::try_from(description)
                .map_err(|_| Error::<T>::ProposalNotFound)?;

            let proposal = Proposal::<T> {
                id,
                title: bounded_title,
                description: bounded_desc,
                proposer: proposer.clone(),
                created_at: now,
                voting_ends: end,
                votes_for: Zero::zero(),
                votes_against: Zero::zero(),
                status: ProposalStatus::Active,
            };

            Proposals::<T>::insert(id, proposal);
            Self::deposit_event(Event::ProposalCreated(id, proposer));
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            support: bool,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            let now = T::Time::now();
            Proposals::<T>::try_mutate(proposal_id, |maybe_p| {
                let p = maybe_p.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(p.status == ProposalStatus::Active, Error::<T>::AlreadyFinalized);
                ensure!(now < p.voting_ends, Error::<T>::VotingClosed);

                T::Currency::reserve(&voter, amount)?;

                if support {
                    p.votes_for += amount;
                } else {
                    p.votes_against += amount;
                }

                Self::deposit_event(Event::Voted(proposal_id, voter, support, amount));
                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn execute_proposal(origin: OriginFor<T>, proposal_id: ProposalId) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            let now = T::Time::now();
            Proposals::<T>::try_mutate(proposal_id, |maybe_p| {
                let p = maybe_p.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(p.status == ProposalStatus::Active, Error::<T>::AlreadyFinalized);
                ensure!(now >= p.voting_ends, Error::<T>::VotingClosed);

                if p.votes_for > p.votes_against {
                    p.status = ProposalStatus::Passed;
                    Self::deposit_event(Event::ProposalPassed(proposal_id));
                } else {
                    p.status = ProposalStatus::Rejected;
                    Self::deposit_event(Event::ProposalRejected(proposal_id));
                }
                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn cancel_proposal(origin: OriginFor<T>, proposal_id: ProposalId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Proposals::<T>::try_mutate(proposal_id, |maybe_p| {
                let p = maybe_p.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(p.proposer == sender, Error::<T>::NotProposer);
                ensure!(p.status == ProposalStatus::Active, Error::<T>::AlreadyFinalized);
                p.status = ProposalStatus::Cancelled;
                Self::deposit_event(Event::ProposalCancelled(proposal_id));
                Ok(())
            })
        }
    }
}
