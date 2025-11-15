//! # Federated Learning Pallet
//!
//! Enables privacy-preserving distributed AI training where data never leaves local GPUs.
//!
//! ## Use Cases
//! - Healthcare: Train cancer detection AI on hospital data without sharing patient records
//! - Finance: Fraud detection across banks without exposing transactions
//! - Privacy-focused AI: Personal AI assistants trained on user data locally
//!
//! ## How It Works
//! 1. Coordinator creates federated learning round
//! 2. GPU nodes train model on local data
//! 3. Nodes submit gradients (NOT raw data)
//! 4. Coordinator aggregates gradients → new global model
//! 5. Repeat for N rounds until convergence
//!
//! ## Value Proposition vs Cocoon
//! - Cocoon: Doesn't support federated learning (data must be uploaded)
//! - Ëtrid: Worth $10B+ in healthcare/finance alone

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    /// Federated learning round status
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum RoundStatus {
        /// Accepting participant registrations
        Open,
        /// Training in progress
        Training,
        /// Aggregating gradients
        Aggregating,
        /// Round completed
        Completed,
    }

    /// Federated learning round
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct FederatedRound<T: Config> {
        /// Round coordinator
        pub coordinator: T::AccountId,
        /// Base model ID (from model-registry)
        pub model_id: u64,
        /// Target participants
        pub min_participants: u32,
        pub max_participants: u32,
        /// Current participants
        pub participants: BoundedVec<T::AccountId, ConstU32<1000>>,
        /// Gradient submissions received
        pub gradients_received: u32,
        /// Round status
        pub status: RoundStatus,
        /// Payment per participant (in ËDSC)
        pub payment_per_participant: u128,
        /// Round number
        pub round_number: u32,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Federated rounds by ID
    #[pallet::storage]
    pub type FederatedRounds<T: Config> = StorageMap<_, Blake2_128Concat, u64, FederatedRound<T>>;

    /// Next round ID
    #[pallet::storage]
    pub type NextRoundId<T> = StorageValue<_, u64, ValueQuery>;

    /// Gradient hashes (round_id, participant) → gradient_hash
    #[pallet::storage]
    pub type Gradients<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u64, // round_id
        Blake2_128Concat,
        T::AccountId, // participant
        BoundedVec<u8, ConstU32<32>>, // gradient hash
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Federated round created [round_id, coordinator]
        RoundCreated { round_id: u64, coordinator: T::AccountId },
        /// Participant joined [round_id, participant]
        ParticipantJoined { round_id: u64, participant: T::AccountId },
        /// Gradient submitted [round_id, participant]
        GradientSubmitted { round_id: u64, participant: T::AccountId },
        /// Round completed [round_id, new_model_id]
        RoundCompleted { round_id: u64, new_model_id: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        RoundNotFound,
        RoundClosed,
        AlreadyParticipant,
        MaxParticipantsReached,
        NotParticipant,
        GradientAlreadySubmitted,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create federated learning round
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_round(
            origin: OriginFor<T>,
            model_id: u64,
            min_participants: u32,
            max_participants: u32,
            payment_per_participant: u128,
        ) -> DispatchResult {
            let coordinator = ensure_signed(origin)?;

            let round_id = NextRoundId::<T>::get();
            NextRoundId::<T>::put(round_id + 1);

            let round = FederatedRound {
                coordinator: coordinator.clone(),
                model_id,
                min_participants,
                max_participants,
                participants: BoundedVec::default(),
                gradients_received: 0,
                status: RoundStatus::Open,
                payment_per_participant,
                round_number: 1,
            };

            FederatedRounds::<T>::insert(round_id, round);
            Self::deposit_event(Event::RoundCreated { round_id, coordinator });
            Ok(())
        }

        /// Join federated round as GPU provider
        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn join_round(origin: OriginFor<T>, round_id: u64) -> DispatchResult {
            let participant = ensure_signed(origin)?;

            FederatedRounds::<T>::try_mutate(round_id, |maybe_round| {
                let round = maybe_round.as_mut().ok_or(Error::<T>::RoundNotFound)?;
                ensure!(round.status == RoundStatus::Open, Error::<T>::RoundClosed);
                ensure!(
                    round.participants.len() < round.max_participants as usize,
                    Error::<T>::MaxParticipantsReached
                );

                round
                    .participants
                    .try_push(participant.clone())
                    .map_err(|_| Error::<T>::MaxParticipantsReached)?;

                Self::deposit_event(Event::ParticipantJoined { round_id, participant });
                Ok(())
            })
        }

        /// Submit gradient after local training
        #[pallet::call_index(2)]
        #[pallet::weight(8_000)]
        pub fn submit_gradient(
            origin: OriginFor<T>,
            round_id: u64,
            gradient_hash: BoundedVec<u8, ConstU32<32>>,
        ) -> DispatchResult {
            let participant = ensure_signed(origin)?;

            FederatedRounds::<T>::try_mutate(round_id, |maybe_round| {
                let round = maybe_round.as_mut().ok_or(Error::<T>::RoundNotFound)?;
                ensure!(
                    round.participants.contains(&participant),
                    Error::<T>::NotParticipant
                );

                // Store gradient hash
                ensure!(
                    !Gradients::<T>::contains_key(round_id, &participant),
                    Error::<T>::GradientAlreadySubmitted
                );
                Gradients::<T>::insert(round_id, &participant, gradient_hash);

                round.gradients_received += 1;

                // If all gradients received, move to aggregating
                if round.gradients_received == round.participants.len() as u32 {
                    round.status = RoundStatus::Aggregating;
                }

                Self::deposit_event(Event::GradientSubmitted { round_id, participant });
                Ok(())
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// Calculate gradient aggregation (to be implemented by off-chain worker)
        /// Uses Federated Averaging (FedAvg) algorithm
        pub fn aggregate_gradients(round_id: u64) -> Option<Vec<u8>> {
            // TODO: Implement gradient aggregation
            // 1. Fetch all gradients from off-chain storage (IPFS)
            // 2. Average gradients: avg_gradient = sum(gradients) / n
            // 3. Apply to base model → new global model
            // 4. Upload new model to IPFS → return hash
            None
        }
    }
}
