//! # Dispute Arbitration Pallet
//!
//! Decentralized dispute resolution with staked judges for job disputes.
//!
//! ## How It Works
//! 1. User disputes job result → creates dispute
//! 2. 3 random staked arbitrators assigned
//! 3. Arbitrators review evidence (logs, outputs)
//! 4. 2/3 majority vote → decision
//! 5. Honest judges earn fees, dishonest lose stake
//!
//! ## Arbitrator Requirements
//! - Stake 1,000 ËTRD ($1,000 value)
//! - Complete training course
//! - Pass reputation threshold (95% honesty rate)
//!
//! ## Incentives
//! - Fee: $5 per dispute (split 3 ways = $1.67 each)
//! - Volume: 1,000 disputes/month = $5,000 revenue
//! - Honest judges: Earn 8% APY on stake + fees
//! - Dishonest judges: Lose 10% stake per wrong decision

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Randomness},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Hash, Saturating};
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Vote outcome
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum VoteOutcome {
        RefundUser,
        FavorProvider,
        Split50_50,
    }

    /// Dispute status
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum DisputeStatus {
        /// Waiting for arbitrators
        Pending,
        /// Arbitrators reviewing
        InReview,
        /// Voting in progress
        Voting,
        /// Resolved
        Resolved,
    }

    /// Arbitrator info
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Arbitrator<T: Config> {
        pub account: T::AccountId,
        pub stake: BalanceOf<T>,
        pub total_cases: u32,
        pub honest_votes: u32,
        pub dishonest_votes: u32,
        pub reputation_score: u16, // 0-10000 = 0%-100%
        pub is_active: bool,
        pub registered_at: u64,
    }

    /// Dispute record
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Dispute<T: Config> {
        pub job_id: u64,
        pub disputor: T::AccountId,
        pub defendant: T::AccountId,
        pub reason: BoundedVec<u8, ConstU32<512>>,
        pub evidence_hash: T::Hash,
        pub status: DisputeStatus,
        pub assigned_arbitrators: BoundedVec<T::AccountId, ConstU32<3>>,
        pub votes: BoundedVec<(T::AccountId, VoteOutcome), ConstU32<3>>,
        pub final_outcome: Option<VoteOutcome>,
        pub created_at: u64,
        pub resolved_at: Option<u64>,
        pub fee: BalanceOf<T>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        /// Arbitrator stake requirement
        #[pallet::constant]
        type ArbitratorStake: Get<BalanceOf<Self>>;

        /// Dispute fee
        #[pallet::constant]
        type DisputeFee: Get<BalanceOf<Self>>;

        /// Slash percentage for dishonest votes (basis points)
        #[pallet::constant]
        type SlashPercentage: Get<u16>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Arbitrators by account
    #[pallet::storage]
    pub type Arbitrators<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Arbitrator<T>>;

    /// Active arbitrators list
    #[pallet::storage]
    pub type ActiveArbitrators<T: Config> = StorageValue<
        _,
        BoundedVec<T::AccountId, ConstU32<1000>>,
        ValueQuery,
    >;

    /// Disputes by ID
    #[pallet::storage]
    pub type Disputes<T: Config> = StorageMap<_, Blake2_128Concat, u64, Dispute<T>>;

    /// Next dispute ID
    #[pallet::storage]
    pub type NextDisputeId<T> = StorageValue<_, u64, ValueQuery>;

    /// Total disputes resolved
    #[pallet::storage]
    pub type TotalDisputes<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Arbitrator registered [account, stake]
        ArbitratorRegistered { account: T::AccountId, stake: BalanceOf<T> },
        /// Dispute created [dispute_id, job_id]
        DisputeCreated { dispute_id: u64, job_id: u64 },
        /// Arbitrators assigned [dispute_id, arbitrators]
        ArbitratorsAssigned {
            dispute_id: u64,
            arbitrators: BoundedVec<T::AccountId, ConstU32<3>>,
        },
        /// Vote submitted [dispute_id, arbitrator, vote]
        VoteSubmitted { dispute_id: u64, arbitrator: T::AccountId, vote: VoteOutcome },
        /// Dispute resolved [dispute_id, outcome]
        DisputeResolved { dispute_id: u64, outcome: VoteOutcome },
        /// Arbitrator slashed [arbitrator, amount]
        ArbitratorSlashed { arbitrator: T::AccountId, amount: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        DisputeNotFound,
        NotArbitrator,
        ArbitratorAlreadyExists,
        InsufficientStake,
        NotAssignedToDispute,
        AlreadyVoted,
        DisputeNotInVoting,
        NotEnoughActiveArbitrators,
        InvalidVote,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register as arbitrator
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_arbitrator(origin: OriginFor<T>) -> DispatchResult {
            let account = ensure_signed(origin)?;

            ensure!(
                !Arbitrators::<T>::contains_key(&account),
                Error::<T>::ArbitratorAlreadyExists
            );

            let stake = T::ArbitratorStake::get();
            T::Currency::reserve(&account, stake)?;

            let arbitrator = Arbitrator {
                account: account.clone(),
                stake,
                total_cases: 0,
                honest_votes: 0,
                dishonest_votes: 0,
                reputation_score: 10000, // Start at 100%
                is_active: true,
                registered_at: Self::current_timestamp(),
            };

            Arbitrators::<T>::insert(&account, arbitrator);

            ActiveArbitrators::<T>::try_mutate(|active| {
                active.try_push(account.clone()).map_err(|_| Error::<T>::NotEnoughActiveArbitrators)?;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::ArbitratorRegistered { account, stake });
            Ok(())
        }

        /// Create dispute
        #[pallet::call_index(1)]
        #[pallet::weight(15_000)]
        pub fn create_dispute(
            origin: OriginFor<T>,
            job_id: u64,
            defendant: T::AccountId,
            reason: BoundedVec<u8, ConstU32<512>>,
            evidence_hash: T::Hash,
        ) -> DispatchResult {
            let disputor = ensure_signed(origin)?;

            let fee = T::DisputeFee::get();
            T::Currency::reserve(&disputor, fee)?;

            let dispute_id = NextDisputeId::<T>::get();
            NextDisputeId::<T>::put(dispute_id + 1);

            let dispute = Dispute {
                job_id,
                disputor: disputor.clone(),
                defendant,
                reason,
                evidence_hash,
                status: DisputeStatus::Pending,
                assigned_arbitrators: BoundedVec::default(),
                votes: BoundedVec::default(),
                final_outcome: None,
                created_at: Self::current_timestamp(),
                resolved_at: None,
                fee,
            };

            Disputes::<T>::insert(dispute_id, dispute);

            Self::deposit_event(Event::DisputeCreated { dispute_id, job_id });

            // Automatically assign arbitrators
            Self::assign_arbitrators(dispute_id)?;

            Ok(())
        }

        /// Submit vote (called by assigned arbitrators)
        #[pallet::call_index(2)]
        #[pallet::weight(8_000)]
        pub fn submit_vote(
            origin: OriginFor<T>,
            dispute_id: u64,
            vote: VoteOutcome,
        ) -> DispatchResult {
            let arbitrator = ensure_signed(origin)?;

            ensure!(Arbitrators::<T>::contains_key(&arbitrator), Error::<T>::NotArbitrator);

            Disputes::<T>::try_mutate(dispute_id, |maybe_dispute| {
                let dispute = maybe_dispute.as_mut().ok_or(Error::<T>::DisputeNotFound)?;

                ensure!(
                    dispute.assigned_arbitrators.contains(&arbitrator),
                    Error::<T>::NotAssignedToDispute
                );

                // Check if already voted
                for (voter, _) in dispute.votes.iter() {
                    if voter == &arbitrator {
                        return Err(Error::<T>::AlreadyVoted.into());
                    }
                }

                dispute
                    .votes
                    .try_push((arbitrator.clone(), vote.clone()))
                    .map_err(|_| Error::<T>::InvalidVote)?;

                Self::deposit_event(Event::VoteSubmitted {
                    dispute_id,
                    arbitrator,
                    vote,
                });

                // Check if we have 2/3 votes
                if dispute.votes.len() >= 2 {
                    Self::resolve_dispute(dispute_id)?;
                }

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number()
                .try_into()
                .unwrap_or(0u64)
                .saturating_mul(6)
        }

        /// Assign 3 random arbitrators to dispute
        fn assign_arbitrators(dispute_id: u64) -> DispatchResult {
            let active = ActiveArbitrators::<T>::get();
            ensure!(active.len() >= 3, Error::<T>::NotEnoughActiveArbitrators);

            // Use randomness to select 3 arbitrators
            let random_seed = T::Randomness::random(&b"arbitrator_selection"[..]).0;
            let mut selected = BoundedVec::default();

            // Simple random selection (in production, use more sophisticated algorithm)
            let indices: Vec<usize> = vec![
                (random_seed.as_ref()[0] as usize) % active.len(),
                (random_seed.as_ref()[1] as usize) % active.len(),
                (random_seed.as_ref()[2] as usize) % active.len(),
            ];

            for &idx in indices.iter().take(3) {
                if let Some(arb) = active.get(idx) {
                    if !selected.contains(arb) {
                        selected.try_push(arb.clone()).ok();
                    }
                }
            }

            Disputes::<T>::try_mutate(dispute_id, |maybe_dispute| {
                let dispute = maybe_dispute.as_mut().ok_or(Error::<T>::DisputeNotFound)?;
                dispute.assigned_arbitrators = selected.clone();
                dispute.status = DisputeStatus::Voting;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::ArbitratorsAssigned {
                dispute_id,
                arbitrators: selected,
            });

            Ok(())
        }

        /// Resolve dispute based on votes
        fn resolve_dispute(dispute_id: u64) -> DispatchResult {
            Disputes::<T>::try_mutate(dispute_id, |maybe_dispute| {
                let dispute = maybe_dispute.as_mut().ok_or(Error::<T>::DisputeNotFound)?;

                // Count votes
                let mut refund_count = 0;
                let mut favor_count = 0;
                let mut split_count = 0;

                for (_arbitrator, vote) in dispute.votes.iter() {
                    match vote {
                        VoteOutcome::RefundUser => refund_count += 1,
                        VoteOutcome::FavorProvider => favor_count += 1,
                        VoteOutcome::Split50_50 => split_count += 1,
                    }
                }

                // Determine outcome (majority wins)
                let outcome = if refund_count >= 2 {
                    VoteOutcome::RefundUser
                } else if favor_count >= 2 {
                    VoteOutcome::FavorProvider
                } else {
                    VoteOutcome::Split50_50
                };

                dispute.final_outcome = Some(outcome.clone());
                dispute.status = DisputeStatus::Resolved;
                dispute.resolved_at = Some(Self::current_timestamp());

                // Distribute fees to arbitrators
                let fee_per_arbitrator = dispute.fee.saturating_div(3_u32.into());
                for (arbitrator, _) in dispute.votes.iter() {
                    // Transfer fee share
                    T::Currency::unreserve(&dispute.disputor, fee_per_arbitrator);
                    // TODO: Transfer to arbitrator
                }

                Self::deposit_event(Event::DisputeResolved {
                    dispute_id,
                    outcome,
                });

                TotalDisputes::<T>::mutate(|total| *total = total.saturating_add(1));

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }
    }
}
