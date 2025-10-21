//! # Pallet Validator Committee
//!
//! This pallet provides runtime storage and queries for the ASF validator committee.
//! It integrates the `validator-management` library into the Substrate runtime,
//! exposing Runtime APIs for the node service layer to query committee state.
//!
//! ## Overview
//!
//! The pallet manages:
//! - Active validator committee members
//! - Validator registration and deregistration
//! - Committee rotation and epoch transitions
//! - Validator health and performance metrics
//!
//! ## Integration with ASF Consensus
//!
//! This pallet is used by:
//! - `asf_service.rs` - Queries active committee for PPFA proposer selection
//! - `finality-gadget` - Validates finality votes from committee members
//! - `block-production` - Determines block authoring permissions
//!
//! ## Runtime API
//!
//! The pallet exposes a Runtime API (defined in runtime/src/lib.rs) that allows
//! the node service to query committee state without directly accessing storage.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::BoundedVec;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    // Re-export types from validator-management
    pub use asf_algorithm::{ValidatorId, Balance};
    pub use validator_management::{ValidatorInfo, PeerType, CommitteeManager};

    /// Current epoch number
    pub type Epoch = u64;

    /// Simplified validator data for storage (codec-compatible)
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, sp_core::RuntimeDebug)]
    pub struct StoredValidatorInfo {
        pub validator_id: ValidatorId,
        pub stake: Balance,
        // Store peer_type as u8: 0=ValidityNode, 1=PerformanceNode, etc.
        pub peer_type: u8,
    }

    impl StoredValidatorInfo {
        pub fn new(validator_id: ValidatorId, stake: Balance, peer_type: u8) -> Self {
            Self {
                validator_id,
                stake,
                peer_type,
            }
        }

        /// Convert to ValidatorInfo for use with consensus modules
        pub fn to_validator_info(&self) -> ValidatorInfo {
            let peer_type = match self.peer_type {
                0 => PeerType::ValidityNode,
                1 => PeerType::PerformanceNode,
                2 => PeerType::ArchiveNode,
                _ => PeerType::ValidityNode, // Default
            };
            ValidatorInfo::new(self.validator_id.clone(), self.stake, peer_type)
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum committee size
        #[pallet::constant]
        type MaxCommitteeSize: Get<u32>;

        /// Minimum validator stake required
        #[pallet::constant]
        type MinValidatorStake: Get<Balance>;
    }

    /// Active committee members (ValidatorId => StoredValidatorInfo)
    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ValidatorId,
        StoredValidatorInfo,
        OptionQuery,
    >;

    /// Current active committee members (ordered list of ValidatorIds)
    #[pallet::storage]
    #[pallet::getter(fn committee)]
    pub type Committee<T: Config> = StorageValue<_, BoundedVec<ValidatorId, T::MaxCommitteeSize>, ValueQuery>;

    /// Current epoch number
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, Epoch, ValueQuery>;

    /// Committee size limit
    #[pallet::storage]
    #[pallet::getter(fn committee_size_limit)]
    pub type CommitteeSizeLimit<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Initial validators (validator_id, stake, peer_type_u8)
        pub validators: Vec<(ValidatorId, Balance, u8)>,
        /// Committee size limit
        pub committee_size: u32,
        #[serde(skip)]
        pub _phantom: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Set committee size limit
            CommitteeSizeLimit::<T>::put(self.committee_size);

            // Initialize validators
            let mut committee_members: BoundedVec<ValidatorId, T::MaxCommitteeSize> = BoundedVec::default();
            for (validator_id, stake, peer_type) in &self.validators {
                if *stake >= T::MinValidatorStake::get() {
                    let stored_info = StoredValidatorInfo::new(validator_id.clone(), *stake, *peer_type);
                    Validators::<T>::insert(validator_id, stored_info);
                    let _ = committee_members.try_push(validator_id.clone());
                }
            }

            // Store initial committee
            Committee::<T>::put(committee_members);

            // Initialize epoch
            CurrentEpoch::<T>::put(0);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A validator was added to the committee
        ValidatorAdded { validator_id: ValidatorId },
        /// A validator was removed from the committee
        ValidatorRemoved { validator_id: ValidatorId },
        /// Committee was rotated for a new epoch
        CommitteeRotated { epoch: Epoch, committee_size: u32 },
        /// Validator stake was updated
        ValidatorStakeUpdated { validator_id: ValidatorId, new_stake: Balance },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Committee is at maximum capacity
        CommitteeFull,
        /// Validator not found
        ValidatorNotFound,
        /// Insufficient stake
        InsufficientStake,
        /// Validator already exists
        ValidatorAlreadyExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a validator to the committee
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn add_validator(
            origin: OriginFor<T>,
            validator_id: ValidatorId,
            stake: Balance,
            peer_type: u8, // 0=ValidityNode, 1=PerformanceNode, 2=ArchiveNode
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Check stake requirement
            ensure!(stake >= T::MinValidatorStake::get(), Error::<T>::InsufficientStake);

            // Check if validator already exists
            ensure!(!Validators::<T>::contains_key(&validator_id), Error::<T>::ValidatorAlreadyExists);

            // Check committee size limit
            let current_committee = Committee::<T>::get();
            let max_size = T::MaxCommitteeSize::get();
            ensure!(current_committee.len() < max_size as usize, Error::<T>::CommitteeFull);

            // Add validator
            let stored_info = StoredValidatorInfo::new(validator_id.clone(), stake, peer_type);
            Validators::<T>::insert(&validator_id, stored_info);

            // Add to committee
            let validator_id_clone = validator_id.clone();
            Committee::<T>::try_mutate(|committee| -> Result<(), DispatchError> {
                committee.try_push(validator_id_clone)
                    .map_err(|_| Error::<T>::CommitteeFull)?;
                Ok(())
            })?;

            Self::deposit_event(Event::ValidatorAdded { validator_id });

            Ok(())
        }

        /// Remove a validator from the committee
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn remove_validator(
            origin: OriginFor<T>,
            validator_id: ValidatorId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Check if validator exists
            ensure!(Validators::<T>::contains_key(&validator_id), Error::<T>::ValidatorNotFound);

            // Remove from storage
            Validators::<T>::remove(&validator_id);

            // Remove from committee
            let validator_id_clone = validator_id.clone();
            Committee::<T>::mutate(|committee| {
                committee.retain(|v| v != &validator_id_clone);
            });

            Self::deposit_event(Event::ValidatorRemoved { validator_id });

            Ok(())
        }

        /// Rotate committee for new epoch
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn rotate_committee(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            // Increment epoch
            let new_epoch = CurrentEpoch::<T>::mutate(|epoch| {
                *epoch += 1;
                *epoch
            });

            // Get current committee size
            let committee = Committee::<T>::get();
            let committee_size = committee.len() as u32;

            Self::deposit_event(Event::CommitteeRotated {
                epoch: new_epoch,
                committee_size,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get all active committee members
        pub fn get_committee() -> Vec<ValidatorInfo> {
            Committee::<T>::get()
                .into_iter()
                .filter_map(|validator_id| {
                    Validators::<T>::get(&validator_id).map(|stored| stored.to_validator_info())
                })
                .collect()
        }

        /// Get specific validator info
        pub fn get_validator(validator_id: &ValidatorId) -> Option<ValidatorInfo> {
            Validators::<T>::get(validator_id).map(|stored| stored.to_validator_info())
        }

        /// Check if validator is in active committee
        pub fn is_validator_active(validator_id: &ValidatorId) -> bool {
            Committee::<T>::get().contains(validator_id)
        }

        /// Get current epoch
        pub fn get_current_epoch() -> Epoch {
            CurrentEpoch::<T>::get()
        }
    }
}
