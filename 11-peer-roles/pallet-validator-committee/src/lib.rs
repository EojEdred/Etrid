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
    use codec::Decode;
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
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, sp_core::RuntimeDebug, MaxEncodedLen)]
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
                0 => PeerType::Common,
                1 => PeerType::StakingCommon,
                2 => PeerType::ValidityNode,
                3 => PeerType::FlareNode,
                4 => PeerType::DecentralizedDirector,
                _ => PeerType::Common, // Default to Common
            };
            ValidatorInfo::new(self.validator_id.clone(), self.stake, peer_type)
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            // Extract PPFA authorization from block digest and record it
            // This ensures block imports store PPFA authorizations for validation

            let digests = frame_system::Pallet::<T>::digest();

            // Look for PPFA PreRuntime digest items
            // PPFA_ENGINE_ID is *b"ppfa" = [112, 112, 102, 97]
            const PPFA_ENGINE_ID: [u8; 4] = *b"ppfa";

            for digest_item in digests.logs.iter() {
                if let sp_runtime::DigestItem::PreRuntime(engine_id, data) = digest_item {
                    if engine_id == &PPFA_ENGINE_ID {
                        // Decode PPFA seal data: (ppfa_index: u32, proposer: ValidatorId)
                        if let Ok((ppfa_index, proposer_id)) = <(u32, ValidatorId)>::decode(&mut &data[..]) {
                            // Record PPFA authorization in storage for future validation
                            Self::record_ppfa_authorization(block_number, ppfa_index, proposer_id);
                        }
                        // Note: Failed decodes are silently ignored to avoid runtime panics
                    }
                }
            }

            // Return weight for digest processing
            Weight::from_parts(10_000, 0)
        }
    }

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

    /// Next epoch validators (pre-selected for next rotation)
    #[pallet::storage]
    #[pallet::getter(fn next_epoch_validators)]
    pub type NextEpochValidators<T: Config> = StorageValue<_, BoundedVec<ValidatorId, T::MaxCommitteeSize>, ValueQuery>;

    /// PPFA authorization history: (block_number, ppfa_index) => validator_id
    /// Stores which validator was authorized to propose at specific slots
    #[pallet::storage]
    #[pallet::getter(fn ppfa_history)]
    pub type PPFAHistory<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (BlockNumberFor<T>, u32), // (block_number, ppfa_index)
        ValidatorId,
        OptionQuery,
    >;

    /// Epoch duration in blocks
    #[pallet::storage]
    #[pallet::getter(fn epoch_duration)]
    pub type EpochDuration<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
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

        /// Record PPFA authorization for a block
        pub fn record_ppfa_authorization(
            block_number: BlockNumberFor<T>,
            ppfa_index: u32,
            proposer_id: ValidatorId,
        ) {
            PPFAHistory::<T>::insert((block_number, ppfa_index), proposer_id);
        }

        /// Check if proposer was authorized for specific block/ppfa_index
        pub fn is_proposer_authorized(
            block_number: BlockNumberFor<T>,
            ppfa_index: u32,
            proposer_id: &ValidatorId,
        ) -> bool {
            PPFAHistory::<T>::get((block_number, ppfa_index))
                .map(|authorized| &authorized == proposer_id)
                .unwrap_or(false)
        }

        /// Get next epoch start block (based on current epoch and duration)
        pub fn next_epoch_start() -> BlockNumberFor<T> {
            let current_epoch: u64 = CurrentEpoch::<T>::get() as u64;
            let epoch_duration = EpochDuration::<T>::get();
            let next_epoch_number = current_epoch + 1;

            // Convert u64 to BlockNumberFor<T>
            let next_epoch_u64 = next_epoch_number * TryInto::<u64>::try_into(epoch_duration).unwrap_or(0);
            next_epoch_u64.try_into().unwrap_or_else(|_| epoch_duration)
        }

        /// Get validators for next epoch
        pub fn get_next_epoch_validators() -> Vec<ValidatorInfo> {
            NextEpochValidators::<T>::get()
                .into_iter()
                .filter_map(|validator_id| {
                    Validators::<T>::get(&validator_id).map(|stored| stored.to_validator_info())
                })
                .collect()
        }

        /// Set epoch duration (can be called during genesis or governance)
        pub fn set_epoch_duration(duration: BlockNumberFor<T>) {
            EpochDuration::<T>::put(duration);
        }

        /// Get epoch duration
        pub fn get_epoch_duration() -> BlockNumberFor<T> {
            EpochDuration::<T>::get()
        }

        // ═══════════════════════════════════════════════════════════════════════════
        // RUNTIME API METHODS FOR ASF CONSENSUS
        // ═══════════════════════════════════════════════════════════════════════════

        /// Get the proposer for a given slot using PPFA rotation
        ///
        /// This implements the PPFA rotation algorithm where validators take turns
        /// proposing blocks in a round-robin fashion based on the slot number.
        pub fn get_proposer_for_slot(slot: u64) -> Option<ValidatorId> {
            let committee = Self::get_committee();
            if committee.is_empty() {
                return None;
            }

            // PPFA rotation: slot % committee_size
            let index = (slot as usize) % committee.len();
            Some(committee[index].id.clone())
        }

        /// Get total stake across all active validators
        ///
        /// This is used for calculating stake-weighted voting thresholds in ASF consensus.
        pub fn get_total_stake() -> Balance {
            Self::get_committee()
                .iter()
                .map(|validator| validator.stake)
                .sum()
        }

        /// Get the PPFA index for a given block number
        ///
        /// The PPFA index determines which validator should propose at a given block height.
        /// This is used for both block production and validation.
        pub fn get_ppfa_index(block_number: u32) -> u32 {
            let committee = Committee::<T>::get();
            if committee.is_empty() {
                return 0;
            }

            // PPFA index is block_number mod committee_size
            (block_number % committee.len() as u32)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION MANAGER IMPLEMENTATION FOR GRANDPA FINALITY COORDINATION
// ═══════════════════════════════════════════════════════════════════════════════

/// SessionManager implementation for ValidatorCommittee pallet
///
/// This integrates with pallet_session to coordinate GRANDPA finality:
/// - Provides validator set for each session (every 600 blocks)
/// - Notifies GRANDPA when validator set changes
/// - Enables session key rotation and validator set updates
///
/// ËTRID-specific: Uses ValidatorId (AccountId32) from ASF consensus
impl<T: Config> pallet_session::SessionManager<T::AccountId> for Pallet<T>
where
    T::AccountId: From<ValidatorId> + Into<ValidatorId>,
{
    /// Called at the start of each new session to get the validator set
    ///
    /// Returns Some(validators) if committee is not empty, None otherwise
    /// This list is used by GRANDPA to determine finality voters
    fn new_session(_new_index: u32) -> Option<sp_std::vec::Vec<T::AccountId>> {
        // Get current committee members
        let committee = Committee::<T>::get();

        if committee.is_empty() {
            return None;
        }

        // Convert ValidatorId to T::AccountId
        let validators: sp_std::vec::Vec<T::AccountId> = committee
            .into_iter()
            .map(|validator_id| validator_id.into())
            .collect();

        Some(validators)
    }

    /// Called when a session ends
    ///
    /// ËTRID uses this to track session transitions
    /// Future: Can add validator performance tracking here
    fn end_session(_end_index: u32) {
        // Track session history for analytics
        // Future: Record validator performance metrics
    }

    /// Called when a session starts (after new_session)
    ///
    /// ËTRID uses this for epoch transitions and committee rotation
    fn start_session(start_index: u32) {
        // Check if we need to rotate committee at epoch boundary
        // Epoch duration is tracked separately in EpochDuration storage
        let epoch_duration = EpochDuration::<T>::get();

        if epoch_duration > 0u32.into() {
            // Calculate if this session starts a new epoch
            let epoch_duration_u32: u32 = epoch_duration.try_into()
                .unwrap_or(2400); // Default to 2400 blocks if conversion fails

            // Session period is 600 blocks (Period constant)
            // Check if session index is a multiple of (epoch_duration / session_period)
            let sessions_per_epoch = epoch_duration_u32 / 600;

            if sessions_per_epoch > 0 && start_index % sessions_per_epoch == 0 {
                let new_epoch = (start_index / sessions_per_epoch) as u64;

                // Update current epoch
                CurrentEpoch::<T>::put(new_epoch);

                // Emit epoch transition event
                Self::deposit_event(Event::CommitteeRotated {
                    epoch: new_epoch,
                    committee_size: Committee::<T>::get().len() as u32,
                });
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATOR ID CONVERSION FOR SESSION MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Type alias for ValidatorIdOf trait required by pallet_session
///
/// This converts from T::AccountId to ValidatorId for committee lookups
/// ËTRID uses AccountId32 for both, so this is an identity conversion
pub struct ValidatorIdOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> sp_runtime::traits::Convert<T::AccountId, Option<T::AccountId>> for ValidatorIdOf<T>
where
    T::AccountId: From<ValidatorId> + Into<ValidatorId>,
{
    fn convert(account: T::AccountId) -> Option<T::AccountId> {
        // Identity conversion - AccountId is already ValidatorId in ËTRID
        Some(account)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RUNTIME API DEFINITION
// ═══════════════════════════════════════════════════════════════════════════════

sp_api::decl_runtime_apis! {
    /// Runtime API for validator committee management
    ///
    /// This API allows the node service layer (especially asf_service.rs) to query
    /// validator committee state without directly accessing runtime storage.
    pub trait ValidatorCommitteeApi<ValidatorId, BlockNumber>
    where
        ValidatorId: codec::Codec,
        BlockNumber: codec::Codec,
    {
        /// Get all active committee members
        fn get_committee() -> sp_std::vec::Vec<ValidatorInfo>;

        /// Get specific validator info by ID
        fn get_validator(validator_id: ValidatorId) -> Option<ValidatorInfo>;

        /// Check if validator is in active committee
        fn is_in_committee(validator_id: ValidatorId) -> bool;

        /// Get current epoch number
        fn current_epoch() -> Epoch;

        /// Get next epoch start block
        fn next_epoch_start() -> BlockNumber;

        /// Get validators for next epoch (pre-computed)
        fn get_next_epoch_validators() -> sp_std::vec::Vec<ValidatorInfo>;

        /// Check if proposer was authorized for specific block/ppfa_index
        fn is_proposer_authorized(
            block_number: BlockNumber,
            ppfa_index: u32,
            proposer_id: ValidatorId,
        ) -> bool;

        /// Get epoch duration in blocks
        fn epoch_duration() -> BlockNumber;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_err, assert_ok, parameter_types,
        traits::{BuildGenesisConfig, ConstU32, ConstU128},
    };
    use sp_core::{H256, crypto::AccountId32};
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    // Configure a mock runtime to test the pallet
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test
        {
            System: frame_system,
            ValidatorCommittee: crate,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Nonce = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Block = Block;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ConstU32<16>;
        type RuntimeTask = ();
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
        type ExtensionsWeightInfo = ();
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type MaxCommitteeSize = ConstU32<100>;
        type MinValidatorStake = ConstU128<1000>;
    }

    // Build genesis storage according to the mock runtime
    fn new_test_ext() -> sp_io::TestExternalities {
        let genesis_config = crate::GenesisConfig::<Test> {
            validators: vec![
                // (validator_id, stake, peer_type)
                (AccountId32::from([1u8; 32]), 5000, 0), // ValidityNode with 5000 stake
                (AccountId32::from([2u8; 32]), 3000, 1), // FlareNode with 3000 stake
                (AccountId32::from([3u8; 32]), 2000, 0), // ValidityNode with 2000 stake
            ],
            committee_size: 10,
            _phantom: Default::default(),
        };

        let storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .into();

        let mut ext = sp_io::TestExternalities::new(storage);
        ext.execute_with(|| {
            // Initialize genesis manually since we're not using RuntimeGenesisConfig
            genesis_config.build();
            System::set_block_number(1);
        });
        ext
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ADD VALIDATOR TESTS
    // ═════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_add_validator_success() {
        new_test_ext().execute_with(|| {
            let new_validator = AccountId32::from([4u8; 32]);
            let stake = 2500;
            let peer_type = 0;

            assert_ok!(ValidatorCommittee::add_validator(
                RuntimeOrigin::root(),
                new_validator.clone(),
                stake,
                peer_type
            ));

            // Verify validator was added to storage
            assert!(Validators::<Test>::contains_key(&new_validator));

            // Verify validator is in committee
            let committee = Committee::<Test>::get();
            assert!(committee.contains(&new_validator));
            assert_eq!(committee.len(), 4); // 3 genesis + 1 new

            // Verify event was emitted
            System::assert_has_event(
                Event::ValidatorAdded {
                    validator_id: new_validator,
                }
                .into(),
            );
        });
    }

    #[test]
    fn test_add_validator_insufficient_stake() {
        new_test_ext().execute_with(|| {
            let new_validator = AccountId32::from([4u8; 32]);
            let stake = 500; // Below MinValidatorStake (1000)
            let peer_type = 0;

            assert_err!(
                ValidatorCommittee::add_validator(
                    RuntimeOrigin::root(),
                    new_validator,
                    stake,
                    peer_type
                ),
                Error::<Test>::InsufficientStake
            );
        });
    }

    #[test]
    fn test_add_validator_already_exists() {
        new_test_ext().execute_with(|| {
            let existing_validator = AccountId32::from([1u8; 32]); // Already in genesis
            let stake = 2000;
            let peer_type = 0;

            assert_err!(
                ValidatorCommittee::add_validator(
                    RuntimeOrigin::root(),
                    existing_validator,
                    stake,
                    peer_type
                ),
                Error::<Test>::ValidatorAlreadyExists
            );
        });
    }

    #[test]
    fn test_add_validator_committee_full() {
        new_test_ext().execute_with(|| {
            // Genesis has 3 validators, MaxCommitteeSize is 100
            // Add 97 more validators to reach max size (3 + 97 = 100)
            for i in 10..107 {
                let validator_id = AccountId32::from([i as u8; 32]);
                assert_ok!(ValidatorCommittee::add_validator(
                    RuntimeOrigin::root(),
                    validator_id,
                    1000,
                    0
                ));
            }

            // Now committee is full (100), try to add one more (should fail)
            let overflow_validator = AccountId32::from([200u8; 32]);
            assert_err!(
                ValidatorCommittee::add_validator(
                    RuntimeOrigin::root(),
                    overflow_validator,
                    1000,
                    0
                ),
                Error::<Test>::CommitteeFull
            );
        });
    }

    #[test]
    fn test_add_validator_requires_root() {
        new_test_ext().execute_with(|| {
            let new_validator = AccountId32::from([4u8; 32]);

            // Try to add validator with non-root origin (should fail)
            assert!(ValidatorCommittee::add_validator(
                RuntimeOrigin::signed(1),
                new_validator,
                1000,
                0
            )
            .is_err());
        });
    }

    // ═════════════════════════════════════════════════════════════════════════
    // REMOVE VALIDATOR TESTS
    // ═════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_remove_validator_success() {
        new_test_ext().execute_with(|| {
            let validator_to_remove = AccountId32::from([1u8; 32]); // Exists in genesis

            assert_ok!(ValidatorCommittee::remove_validator(
                RuntimeOrigin::root(),
                validator_to_remove.clone()
            ));

            // Verify validator was removed from storage
            assert!(!Validators::<Test>::contains_key(&validator_to_remove));

            // Verify validator is not in committee
            let committee = Committee::<Test>::get();
            assert!(!committee.contains(&validator_to_remove));
            assert_eq!(committee.len(), 2); // 3 genesis - 1 removed

            // Verify event was emitted
            System::assert_has_event(
                Event::ValidatorRemoved {
                    validator_id: validator_to_remove,
                }
                .into(),
            );
        });
    }

    #[test]
    fn test_remove_validator_not_found() {
        new_test_ext().execute_with(|| {
            let non_existent_validator = AccountId32::from([99u8; 32]);

            assert_err!(
                ValidatorCommittee::remove_validator(
                    RuntimeOrigin::root(),
                    non_existent_validator
                ),
                Error::<Test>::ValidatorNotFound
            );
        });
    }

    #[test]
    fn test_remove_validator_requires_root() {
        new_test_ext().execute_with(|| {
            let validator_to_remove = AccountId32::from([1u8; 32]);

            // Try to remove validator with non-root origin (should fail)
            assert!(ValidatorCommittee::remove_validator(
                RuntimeOrigin::signed(1),
                validator_to_remove
            )
            .is_err());
        });
    }

    // ═════════════════════════════════════════════════════════════════════════
    // ROTATE COMMITTEE TESTS
    // ═════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_rotate_committee_success() {
        new_test_ext().execute_with(|| {
            let initial_epoch = CurrentEpoch::<Test>::get();
            assert_eq!(initial_epoch, 0);

            assert_ok!(ValidatorCommittee::rotate_committee(RuntimeOrigin::root()));

            // Verify epoch was incremented
            let new_epoch = CurrentEpoch::<Test>::get();
            assert_eq!(new_epoch, 1);

            // Verify event was emitted
            System::assert_has_event(
                Event::CommitteeRotated {
                    epoch: 1,
                    committee_size: 3, // 3 genesis validators
                }
                .into(),
            );
        });
    }

    #[test]
    fn test_rotate_committee_multiple_times() {
        new_test_ext().execute_with(|| {
            for expected_epoch in 1..=5 {
                assert_ok!(ValidatorCommittee::rotate_committee(RuntimeOrigin::root()));
                assert_eq!(CurrentEpoch::<Test>::get(), expected_epoch);
            }
        });
    }

    #[test]
    fn test_rotate_committee_requires_root() {
        new_test_ext().execute_with(|| {
            // Try to rotate committee with non-root origin (should fail)
            assert!(ValidatorCommittee::rotate_committee(RuntimeOrigin::signed(1)).is_err());
        });
    }

    // ═════════════════════════════════════════════════════════════════════════
    // QUERY FUNCTION TESTS
    // ═════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_get_committee() {
        new_test_ext().execute_with(|| {
            let committee = ValidatorCommittee::get_committee();
            assert_eq!(committee.len(), 3);

            // Verify all genesis validators are present
            assert!(committee.iter().any(|v| v.id == AccountId32::from([1u8; 32])));
            assert!(committee.iter().any(|v| v.id == AccountId32::from([2u8; 32])));
            assert!(committee.iter().any(|v| v.id == AccountId32::from([3u8; 32])));

            // Verify stakes
            let validator1 = committee.iter().find(|v| v.id == AccountId32::from([1u8; 32])).unwrap();
            assert_eq!(validator1.stake, 5000);
        });
    }

    #[test]
    fn test_get_validator() {
        new_test_ext().execute_with(|| {
            let validator_id = AccountId32::from([1u8; 32]);
            let info = ValidatorCommittee::get_validator(&validator_id);

            assert!(info.is_some());
            let info = info.unwrap();
            assert_eq!(info.id, validator_id);
            assert_eq!(info.stake, 5000);
        });
    }

    #[test]
    fn test_get_validator_not_found() {
        new_test_ext().execute_with(|| {
            let non_existent = AccountId32::from([99u8; 32]);
            let info = ValidatorCommittee::get_validator(&non_existent);
            assert!(info.is_none());
        });
    }

    #[test]
    fn test_is_validator_active() {
        new_test_ext().execute_with(|| {
            assert!(ValidatorCommittee::is_validator_active(&AccountId32::from([1u8; 32])));
            assert!(ValidatorCommittee::is_validator_active(&AccountId32::from([2u8; 32])));
            assert!(!ValidatorCommittee::is_validator_active(&AccountId32::from([99u8; 32])));
        });
    }

    #[test]
    fn test_get_current_epoch() {
        new_test_ext().execute_with(|| {
            assert_eq!(ValidatorCommittee::get_current_epoch(), 0);

            ValidatorCommittee::rotate_committee(RuntimeOrigin::root()).unwrap();
            assert_eq!(ValidatorCommittee::get_current_epoch(), 1);
        });
    }

    // ═════════════════════════════════════════════════════════════════════════
    // PPFA AUTHORIZATION TESTS
    // ═════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_record_ppfa_authorization() {
        new_test_ext().execute_with(|| {
            let block_number = 10;
            let ppfa_index = 2;
            let validator_id = AccountId32::from([1u8; 32]);

            ValidatorCommittee::record_ppfa_authorization(
                block_number,
                ppfa_index,
                validator_id.clone(),
            );

            // Verify authorization was recorded
            assert!(ValidatorCommittee::is_proposer_authorized(
                block_number,
                ppfa_index,
                &validator_id
            ));
        });
    }

    #[test]
    fn test_is_proposer_authorized_false() {
        new_test_ext().execute_with(|| {
            let block_number = 10;
            let ppfa_index = 2;
            let validator_id = AccountId32::from([1u8; 32]);

            // No authorization recorded
            assert!(!ValidatorCommittee::is_proposer_authorized(
                block_number,
                ppfa_index,
                &validator_id
            ));
        });
    }

    #[test]
    fn test_ppfa_authorization_different_slots() {
        new_test_ext().execute_with(|| {
            let validator1 = AccountId32::from([1u8; 32]);
            let validator2 = AccountId32::from([2u8; 32]);

            // Authorize different validators for different slots
            ValidatorCommittee::record_ppfa_authorization(10, 0, validator1.clone());
            ValidatorCommittee::record_ppfa_authorization(10, 1, validator2.clone());
            ValidatorCommittee::record_ppfa_authorization(11, 0, validator2.clone());

            // Verify correct authorizations
            assert!(ValidatorCommittee::is_proposer_authorized(10, 0, &validator1));
            assert!(ValidatorCommittee::is_proposer_authorized(10, 1, &validator2));
            assert!(ValidatorCommittee::is_proposer_authorized(11, 0, &validator2));

            // Verify incorrect authorizations
            assert!(!ValidatorCommittee::is_proposer_authorized(10, 0, &validator2));
            assert!(!ValidatorCommittee::is_proposer_authorized(10, 1, &validator1));
        });
    }

    // ═════════════════════════════════════════════════════════════════════════
    // EPOCH DURATION TESTS
    // ═════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_set_and_get_epoch_duration() {
        new_test_ext().execute_with(|| {
            let duration = 100u64;
            ValidatorCommittee::set_epoch_duration(duration);
            assert_eq!(ValidatorCommittee::get_epoch_duration(), duration);
        });
    }

    #[test]
    fn test_next_epoch_start() {
        new_test_ext().execute_with(|| {
            // Set epoch duration
            ValidatorCommittee::set_epoch_duration(100);

            // Current epoch is 0, next epoch is 1
            // Epoch 0: blocks 0-99, Epoch 1: blocks 100-199
            let next_start = ValidatorCommittee::next_epoch_start();
            assert_eq!(next_start, 100); // Epoch 1 starts at block 100
        });
    }

    // ═════════════════════════════════════════════════════════════════════════
    // INTEGRATION TESTS
    // ═════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_complete_lifecycle() {
        new_test_ext().execute_with(|| {
            // 1. Start with 3 genesis validators
            assert_eq!(Committee::<Test>::get().len(), 3);
            assert_eq!(CurrentEpoch::<Test>::get(), 0);

            // 2. Add a new validator
            let new_validator = AccountId32::from([4u8; 32]);
            assert_ok!(ValidatorCommittee::add_validator(
                RuntimeOrigin::root(),
                new_validator.clone(),
                1500,
                1
            ));
            assert_eq!(Committee::<Test>::get().len(), 4);

            // 3. Rotate committee to new epoch
            assert_ok!(ValidatorCommittee::rotate_committee(RuntimeOrigin::root()));
            assert_eq!(CurrentEpoch::<Test>::get(), 1);

            // 4. Record PPFA authorization
            ValidatorCommittee::record_ppfa_authorization(10, 0, new_validator.clone());
            assert!(ValidatorCommittee::is_proposer_authorized(10, 0, &new_validator));

            // 5. Remove a validator
            let validator_to_remove = AccountId32::from([1u8; 32]);
            assert_ok!(ValidatorCommittee::remove_validator(
                RuntimeOrigin::root(),
                validator_to_remove
            ));
            assert_eq!(Committee::<Test>::get().len(), 3);

            // 6. Rotate again
            assert_ok!(ValidatorCommittee::rotate_committee(RuntimeOrigin::root()));
            assert_eq!(CurrentEpoch::<Test>::get(), 2);
        });
    }

    #[test]
    fn test_validator_info_conversion() {
        new_test_ext().execute_with(|| {
            let validator_id = AccountId32::from([1u8; 32]);
            let stake = 5000;

            // Get validator info
            let info = ValidatorCommittee::get_validator(&validator_id).unwrap();

            // Verify conversion from StoredValidatorInfo to ValidatorInfo
            assert_eq!(info.id, validator_id);
            assert_eq!(info.stake, stake);
            // peer_type 0 should convert to ValidityNode
            assert_eq!(info.peer_type, PeerType::ValidityNode);
        });
    }

    #[test]
    fn test_committee_size_limit() {
        new_test_ext().execute_with(|| {
            // Genesis set committee_size to 10 but MaxCommitteeSize is 100
            // We should be able to add validators up to MaxCommitteeSize
            assert_eq!(CommitteeSizeLimit::<Test>::get(), 10);

            // Try adding validators beyond genesis committee_size but within MaxCommitteeSize
            for i in 4..15 {
                let validator_id = AccountId32::from([i as u8; 32]);
                assert_ok!(ValidatorCommittee::add_validator(
                    RuntimeOrigin::root(),
                    validator_id,
                    1000,
                    0
                ));
            }

            // Committee should now have 14 validators (3 genesis + 11 new)
            assert_eq!(Committee::<Test>::get().len(), 14);
        });
    }
}
