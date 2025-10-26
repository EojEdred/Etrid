//! # Pallet AIDID - AI Decentralized Identity
//!
//! **World's First AI DID Standard**
//!
//! ## Overview
//!
//! This pallet provides decentralized identities specifically designed for Artificial
//! Intelligence agents, models, and systems on the Ëtrid blockchain. AIDID extends the
//! W3C DID specification with AI-specific features including capability declarations,
//! model attestation, reputation tracking, and comprehensive safety profiles.
//!
//! ## Features
//!
//! - Unique DIDs for AI agents (format: `did:etrid:ai:{type}:{id}`)
//! - Capability declarations (tasks, modalities, languages, context limits)
//! - Cryptographic model attestation and provenance tracking
//! - Reputation system with inference tracking and user ratings
//! - Permission-based authorization for AI actions
//! - Safety profiles (alignment methods, content filtering, bias evaluation)
//! - Pricing model configuration for AI services
//! - AI activation/deactivation controls
//!
//! ## Extrinsics
//!
//! - `register_ai` - Register a new AI identity with profile
//! - `update_profile` - Update AI capabilities and configuration
//! - `attest_model` - Cryptographically attest AI model provenance
//! - `grant_permission` - Grant specific permissions to an AI
//! - `revoke_permission` - Revoke permissions from an AI
//! - `record_inference` - Record AI inference execution (success/failure)
//! - `submit_rating` - Submit user rating for AI performance
//! - `deactivate_ai` - Temporarily deactivate an AI
//! - `reactivate_ai` - Reactivate a deactivated AI
//! - `update_pricing` - Update AI service pricing model
//!
//! ## Usage Example
//!
//! ```ignore
//! // Register a new LLM AI
//! AIDID::register_ai(
//!     Origin::signed(controller),
//!     b"gpt-etrid-v1".to_vec(),
//!     AIType::LLM,
//!     ai_profile, // Includes capabilities, safety, version
//! )?;
//!
//! // Attest the model
//! AIDID::attest_model(
//!     Origin::signed(controller),
//!     ai_did_hash,
//!     model_attestation, // Includes model hash, training data hash, benchmarks
//! )?;
//!
//! // Grant permission for data access
//! AIDID::grant_permission(
//!     Origin::signed(controller),
//!     ai_did_hash,
//!     permission, // Action, resource, conditions
//! )?;
//!
//! // Record successful inference
//! AIDID::record_inference(
//!     Origin::signed(user),
//!     ai_did_hash,
//!     true, // success
//! )?;
//! ```
//!
//! ## Storage Items
//!
//! - `AIIdentities` - Maps DID hash to AI identity record
//! - `ControllerAIs` - Maps controller account to owned AIs
//! - `AIReputation` - Tracks AI performance metrics and ratings
//! - `AIPermissions` - Maps AI DID and permission hash to permission details
//! - `TotalAIs` - Total number of registered AI identities
//!
//! ## Events
//!
//! - `AIRegistered` - When a new AI identity is registered
//! - `AIUpdated` - When AI profile is updated
//! - `ModelAttested` - When AI model is cryptographically attested
//! - `PermissionGranted` - When permission is granted to AI
//! - `PermissionRevoked` - When permission is revoked from AI
//! - `InferenceRecorded` - When AI inference execution is logged
//! - `RatingSubmitted` - When user submits AI rating
//! - `AIDeactivated` - When AI is deactivated
//! - `AIReactivated` - When AI is reactivated
//! - `PricingUpdated` - When AI pricing model is updated
//!
//! ## Errors
//!
//! - `AIAlreadyExists` - AI identity already registered
//! - `AINotFound` - AI identity does not exist
//! - `NotController` - Caller is not the AI controller
//! - `AIDeactivated` - AI is currently deactivated
//! - `InvalidRating` - Rating value out of valid range (0-10000)
//! - `InvalidAttestation` - Model attestation data is invalid
//! - `NoCapabilities` - AI profile missing required capabilities
//! - `NoModalities` - AI profile missing input/output modalities

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;
use types::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::BoundedVec;
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use sp_std::vec::Vec;

    /// Maximum identifier length
    pub const MAX_IDENTIFIER_LENGTH: u32 = 64;
    /// Maximum version length
    pub const MAX_VERSION_LENGTH: u32 = 32;
    /// Maximum architecture description length
    pub const MAX_ARCHITECTURE_LENGTH: u32 = 128;
    /// Maximum parameters string length
    pub const MAX_PARAMETERS_LENGTH: u32 = 32;
    /// Maximum alignment method length
    pub const MAX_ALIGNMENT_METHOD_LENGTH: u32 = 64;
    /// Maximum number of tasks
    pub const MAX_TASKS: u32 = 32;
    /// Maximum number of modalities
    pub const MAX_MODALITIES: u32 = 16;
    /// Maximum number of languages
    pub const MAX_LANGUAGES: u32 = 100;
    /// Maximum language code length
    pub const MAX_LANGUAGE_LENGTH: u32 = 8;
    /// Maximum number of benchmarks
    pub const MAX_BENCHMARKS: u32 = 50;
    /// Maximum benchmark name length
    pub const MAX_BENCHMARK_NAME_LENGTH: u32 = 64;
    /// Maximum training data hash length
    pub const MAX_TRAINING_DATA_HASH_LENGTH: u32 = 128;
    /// Maximum number of prohibited tasks
    pub const MAX_PROHIBITED_TASKS: u32 = 32;
    /// Maximum number of permissions
    pub const MAX_PERMISSIONS: u32 = 100;
    /// Maximum action length
    pub const MAX_ACTION_LENGTH: u32 = 64;
    /// Maximum resource length
    pub const MAX_RESOURCE_LENGTH: u32 = 128;
    /// Maximum number of conditions
    pub const MAX_CONDITIONS: u32 = 10;
    /// Maximum condition length
    pub const MAX_CONDITION_LENGTH: u32 = 128;

    /// AI Identity structure
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct AIIdentity<T: Config> {
        /// DID identifier (e.g., "gpt4-turbo")
        pub identifier: BoundedVec<u8, ConstU32<MAX_IDENTIFIER_LENGTH>>,
        /// AI type
        pub ai_type: AIType,
        /// Controller account
        pub controller: T::AccountId,
        /// AI profile
        pub profile: AIProfile,
        /// Model attestation (optional)
        pub attestation: Option<ModelAttestation>,
        /// Pricing model (optional)
        pub pricing: Option<PricingModel>,
        /// Registration block
        pub registered_at: BlockNumberFor<T>,
        /// Last update block
        pub updated_at: BlockNumberFor<T>,
        /// Active status
        pub active: bool,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum number of AIs per controller
        #[pallet::constant]
        type MaxAIsPerController: Get<u32>;
    }

    /// AI Identity Registry (DID hash => AIIdentity)
    #[pallet::storage]
    #[pallet::getter(fn ai_identity)]
    pub type AIIdentities<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // Hash of DID identifier
        AIIdentity<T>,
        OptionQuery,
    >;

    /// Controller to AI DIDs mapping
    #[pallet::storage]
    #[pallet::getter(fn controller_ais)]
    pub type ControllerAIs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<H256, T::MaxAIsPerController>,
        ValueQuery,
    >;

    /// AI Reputation tracking
    #[pallet::storage]
    #[pallet::getter(fn ai_reputation)]
    pub type AIReputation<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // DID hash
        Reputation,
        ValueQuery,
    >;

    /// AI Permissions (DID hash => Permission hash => Permission)
    #[pallet::storage]
    #[pallet::getter(fn ai_permissions)]
    pub type AIPermissions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, H256, // DID hash
        Blake2_128Concat, H256, // Permission hash
        Permission,
        OptionQuery,
    >;

    /// Total number of registered AIs
    #[pallet::storage]
    #[pallet::getter(fn total_ais)]
    pub type TotalAIs<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// AI identity registered
        AIRegistered {
            did_hash: H256,
            controller: T::AccountId,
            ai_type: AIType,
        },
        /// AI profile updated
        AIUpdated {
            did_hash: H256,
            updated_by: T::AccountId,
        },
        /// Model attested
        ModelAttested {
            did_hash: H256,
            model_hash: [u8; 32],
        },
        /// Permission granted
        PermissionGranted {
            did_hash: H256,
            permission_hash: H256,
        },
        /// Permission revoked
        PermissionRevoked {
            did_hash: H256,
            permission_hash: H256,
        },
        /// Inference recorded
        InferenceRecorded {
            did_hash: H256,
            success: bool,
        },
        /// Rating submitted
        RatingSubmitted {
            did_hash: H256,
            rating: u32,
        },
        /// AI deactivated
        AIDeactivated {
            did_hash: H256,
        },
        /// AI reactivated
        AIReactivated {
            did_hash: H256,
        },
        /// Pricing updated
        PricingUpdated {
            did_hash: H256,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// AI identity already exists
        AIAlreadyExists,
        /// AI identity not found
        AINotFound,
        /// Not the controller
        NotController,
        /// Identifier too long
        IdentifierTooLong,
        /// Invalid attestation
        InvalidAttestation,
        /// AI is deactivated
        AIDeactivated,
        /// Invalid rating (must be 0-10000)
        InvalidRating,
        /// Too many AIs for controller
        TooManyAIs,
        /// Invalid benchmark score
        InvalidBenchmarkScore,
        /// Invalid toxicity score
        InvalidToxicityScore,
        /// No capabilities specified
        NoCapabilities,
        /// No modalities specified
        NoModalities,
        /// Invalid context size
        InvalidContextSize,
        /// Version required
        VersionRequired,
        /// Alignment method required
        AlignmentMethodRequired,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new AI identity
        #[pallet::call_index(0)]
        #[pallet::weight(100_000)]
        pub fn register_ai(
            origin: OriginFor<T>,
            identifier: Vec<u8>,
            ai_type: AIType,
            profile: AIProfile,
        ) -> DispatchResult {
            let controller = ensure_signed(origin)?;

            // Validate profile
            Self::validate_profile(&profile)?;

            // Create bounded identifier
            let bounded_identifier = BoundedVec::<u8, ConstU32<MAX_IDENTIFIER_LENGTH>>::try_from(identifier.clone())
                .map_err(|_| Error::<T>::IdentifierTooLong)?;

            // Hash the identifier for storage key
            let did_hash = Self::hash_identifier(&identifier, &ai_type);

            // Ensure doesn't already exist
            ensure!(!AIIdentities::<T>::contains_key(did_hash), Error::<T>::AIAlreadyExists);

            let current_block = <frame_system::Pallet<T>>::block_number();

            // Create AI identity
            let identity = AIIdentity {
                identifier: bounded_identifier,
                ai_type,
                controller: controller.clone(),
                profile,
                attestation: None,
                pricing: None,
                registered_at: current_block,
                updated_at: current_block,
                active: true,
            };

            // Store identity
            AIIdentities::<T>::insert(did_hash, identity);

            // Add to controller's AI list
            ControllerAIs::<T>::try_mutate(&controller, |ais| -> Result<(), DispatchError> {
                ais.try_push(did_hash).map_err(|_| Error::<T>::TooManyAIs)?;
                Ok(())
            })?;

            // Increment counter
            TotalAIs::<T>::mutate(|n| *n = n.saturating_add(1));

            Self::deposit_event(Event::AIRegistered {
                did_hash,
                controller,
                ai_type,
            });

            Ok(())
        }

        /// Update AI profile
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn update_profile(
            origin: OriginFor<T>,
            did_hash: H256,
            new_profile: AIProfile,
        ) -> DispatchResult {
            let updater = ensure_signed(origin)?;

            // Validate profile
            Self::validate_profile(&new_profile)?;

            // Get and verify identity
            let mut identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.controller == updater, Error::<T>::NotController);

            // Update profile
            identity.profile = new_profile;
            identity.updated_at = <frame_system::Pallet<T>>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::AIUpdated {
                did_hash,
                updated_by: updater,
            });

            Ok(())
        }

        /// Attest model
        #[pallet::call_index(2)]
        #[pallet::weight(75_000)]
        pub fn attest_model(
            origin: OriginFor<T>,
            did_hash: H256,
            attestation: ModelAttestation,
        ) -> DispatchResult {
            let attester = ensure_signed(origin)?;

            // Validate attestation
            Self::validate_attestation(&attestation)?;

            // Get and verify identity
            let mut identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.controller == attester, Error::<T>::NotController);

            let model_hash = attestation.model_hash;

            // Store attestation
            identity.attestation = Some(attestation);
            identity.updated_at = <frame_system::Pallet<T>>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::ModelAttested {
                did_hash,
                model_hash,
            });

            Ok(())
        }

        /// Grant permission to AI
        #[pallet::call_index(3)]
        #[pallet::weight(25_000)]
        pub fn grant_permission(
            origin: OriginFor<T>,
            did_hash: H256,
            permission: Permission,
        ) -> DispatchResult {
            let granter = ensure_signed(origin)?;

            // Verify controller
            let identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;
            ensure!(identity.controller == granter, Error::<T>::NotController);

            // Hash permission for storage
            let permission_hash = Self::hash_permission(&permission);

            // Store permission
            AIPermissions::<T>::insert(did_hash, permission_hash, permission);

            Self::deposit_event(Event::PermissionGranted {
                did_hash,
                permission_hash,
            });

            Ok(())
        }

        /// Revoke permission from AI
        #[pallet::call_index(4)]
        #[pallet::weight(25_000)]
        pub fn revoke_permission(
            origin: OriginFor<T>,
            did_hash: H256,
            permission_hash: H256,
        ) -> DispatchResult {
            let revoker = ensure_signed(origin)?;

            // Verify controller
            let identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;
            ensure!(identity.controller == revoker, Error::<T>::NotController);

            // Remove permission
            AIPermissions::<T>::remove(did_hash, permission_hash);

            Self::deposit_event(Event::PermissionRevoked {
                did_hash,
                permission_hash,
            });

            Ok(())
        }

        /// Record inference execution
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn record_inference(
            origin: OriginFor<T>,
            did_hash: H256,
            success: bool,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            // Verify AI exists and is active
            let identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.active, Error::<T>::AIDeactivated);

            // Update reputation
            AIReputation::<T>::mutate(did_hash, |rep| {
                rep.record_inference(success);
            });

            Self::deposit_event(Event::InferenceRecorded {
                did_hash,
                success,
            });

            Ok(())
        }

        /// Submit rating for AI
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn submit_rating(
            origin: OriginFor<T>,
            did_hash: H256,
            rating: u32,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            // Validate rating (0-10000)
            ensure!(rating <= 10000, Error::<T>::InvalidRating);

            // Verify AI exists
            ensure!(AIIdentities::<T>::contains_key(did_hash), Error::<T>::AINotFound);

            // Update reputation
            AIReputation::<T>::mutate(did_hash, |rep| {
                rep.add_rating(rating);
            });

            Self::deposit_event(Event::RatingSubmitted {
                did_hash,
                rating,
            });

            Ok(())
        }

        /// Deactivate AI
        #[pallet::call_index(7)]
        #[pallet::weight(25_000)]
        pub fn deactivate_ai(
            origin: OriginFor<T>,
            did_hash: H256,
        ) -> DispatchResult {
            let deactivator = ensure_signed(origin)?;

            // Get and verify identity
            let mut identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.controller == deactivator, Error::<T>::NotController);

            // Deactivate
            identity.active = false;
            identity.updated_at = <frame_system::Pallet<T>>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::AIDeactivated {
                did_hash,
            });

            Ok(())
        }

        /// Reactivate AI
        #[pallet::call_index(8)]
        #[pallet::weight(25_000)]
        pub fn reactivate_ai(
            origin: OriginFor<T>,
            did_hash: H256,
        ) -> DispatchResult {
            let activator = ensure_signed(origin)?;

            // Get and verify identity
            let mut identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.controller == activator, Error::<T>::NotController);

            // Reactivate
            identity.active = true;
            identity.updated_at = <frame_system::Pallet<T>>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::AIReactivated {
                did_hash,
            });

            Ok(())
        }

        /// Update pricing model
        #[pallet::call_index(9)]
        #[pallet::weight(25_000)]
        pub fn update_pricing(
            origin: OriginFor<T>,
            did_hash: H256,
            pricing: PricingModel,
        ) -> DispatchResult {
            let updater = ensure_signed(origin)?;

            // Get and verify identity
            let mut identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.controller == updater, Error::<T>::NotController);

            // Update pricing
            identity.pricing = Some(pricing);
            identity.updated_at = <frame_system::Pallet<T>>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::PricingUpdated {
                did_hash,
            });

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Hash DID identifier for storage key
        pub fn hash_identifier(identifier: &[u8], ai_type: &AIType) -> H256 {
            let type_byte = ai_type.to_u8();
            let combined = [&[type_byte], identifier].concat();
            H256::from(sp_io::hashing::blake2_256(&combined))
        }

        /// Hash permission for storage
        pub fn hash_permission(permission: &Permission) -> H256 {
            let data = [&permission.action[..], &permission.resource[..]].concat();
            H256::from(sp_io::hashing::blake2_256(&data))
        }

        /// Validate AI profile
        fn validate_profile(profile: &AIProfile) -> Result<(), DispatchError> {
            // Check capabilities
            ensure!(!profile.capabilities.tasks.is_empty(), Error::<T>::NoCapabilities);
            ensure!(!profile.capabilities.input_modalities.is_empty(), Error::<T>::NoModalities);
            ensure!(!profile.capabilities.output_modalities.is_empty(), Error::<T>::NoModalities);

            // Validate context size if set
            if let Some(max_context) = profile.capabilities.max_context {
                ensure!(max_context > 0, Error::<T>::InvalidContextSize);
            }

            // Validate safety profile
            ensure!(profile.safety.toxicity_score <= 10000, Error::<T>::InvalidToxicityScore);
            ensure!(!profile.safety.alignment_method.is_empty(), Error::<T>::AlignmentMethodRequired);

            // Validate version
            ensure!(!profile.version.is_empty(), Error::<T>::VersionRequired);

            Ok(())
        }

        /// Validate model attestation
        fn validate_attestation(attestation: &ModelAttestation) -> Result<(), DispatchError> {
            // Model hash cannot be zero
            ensure!(attestation.model_hash != [0u8; 32], Error::<T>::InvalidAttestation);

            // Training data hash cannot be empty
            ensure!(!attestation.training_data_hash.is_empty(), Error::<T>::InvalidAttestation);

            // Version cannot be empty
            ensure!(!attestation.version.is_empty(), Error::<T>::InvalidAttestation);

            // Validate benchmark scores
            for benchmark in attestation.benchmarks.iter() {
                ensure!(benchmark.score <= 10000, Error::<T>::InvalidBenchmarkScore);
            }

            Ok(())
        }

        /// Get AI identity by hash
        pub fn get_ai_identity(did_hash: H256) -> Option<AIIdentity<T>> {
            AIIdentities::<T>::get(did_hash)
        }

        /// Check if AI has permission
        pub fn has_permission(did_hash: H256, action: &[u8], resource: &[u8]) -> bool {
            let data = [action, resource].concat();
            let permission_hash = H256::from(sp_io::hashing::blake2_256(&data));
            AIPermissions::<T>::contains_key(did_hash, permission_hash)
        }

        /// Get AI reputation
        pub fn get_reputation(did_hash: H256) -> Reputation {
            AIReputation::<T>::get(did_hash)
        }

        /// Get controller's AIs
        pub fn get_controller_ais(controller: &T::AccountId) -> Vec<H256> {
            ControllerAIs::<T>::get(controller).into_iter().collect()
        }

        /// Check if AI is active
        pub fn is_active(did_hash: H256) -> bool {
            AIIdentities::<T>::get(did_hash)
                .map(|identity| identity.active)
                .unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_err, assert_ok, parameter_types,
        traits::ConstU32,
        BoundedVec,
    };
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test
        {
            System: frame_system,
            AIDID: crate,
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

    impl crate::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type MaxAIsPerController = ConstU32<100>;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    fn create_test_profile() -> AIProfile {
        let mut tasks = BoundedVec::default();
        tasks.try_push(Task::TextGeneration).unwrap();

        let mut input_modalities = BoundedVec::default();
        input_modalities.try_push(Modality::Text).unwrap();

        let mut output_modalities = BoundedVec::default();
        output_modalities.try_push(Modality::Text).unwrap();

        let capabilities = Capabilities {
            tasks,
            input_modalities,
            output_modalities,
            languages: BoundedVec::default(),
            max_context: Some(4096),
            max_output: Some(2048),
        };

        let mut alignment_method = BoundedVec::default();
        alignment_method.extend_from_slice(b"RLHF");

        let safety = SafetyProfile {
            alignment_method,
            content_filtering: true,
            bias_evaluated: true,
            toxicity_score: 100,
        };

        let mut version = BoundedVec::default();
        version.extend_from_slice(b"v1.0.0");

        let mut architecture = BoundedVec::default();
        architecture.extend_from_slice(b"transformer");

        let mut parameters = BoundedVec::default();
        parameters.extend_from_slice(b"175B");

        AIProfile {
            ai_type: AIType::LLM,
            version,
            architecture,
            parameters,
            capabilities,
            restrictions: Restrictions::default(),
            safety,
        }
    }

    #[test]
    fn test_register_ai() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile
            ));

            let did_hash = AIDID::hash_identifier(&identifier, &AIType::LLM);
            assert!(AIDID::ai_identity(did_hash).is_some());
            assert_eq!(AIDID::total_ais(), 1);
        });
    }

    #[test]
    fn test_register_duplicate_ai() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile.clone()
            ));

            assert_err!(
                AIDID::register_ai(
                    RuntimeOrigin::signed(controller),
                    identifier,
                    AIType::LLM,
                    profile
                ),
                Error::<Test>::AIAlreadyExists
            );
        });
    }

    #[test]
    fn test_update_profile() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile.clone()
            ));

            let did_hash = AIDID::hash_identifier(&identifier, &AIType::LLM);
            let new_profile = create_test_profile();

            assert_ok!(AIDID::update_profile(
                RuntimeOrigin::signed(controller),
                did_hash,
                new_profile
            ));
        });
    }

    #[test]
    fn test_attest_model() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile
            ));

            let did_hash = AIDID::hash_identifier(&identifier, &AIType::LLM);

            let mut version = BoundedVec::default();
            version.extend_from_slice(b"v1.0.0");

            let mut training_data_hash = BoundedVec::default();
            training_data_hash.extend_from_slice(b"QmXyz...");

            let mut benchmarks = BoundedVec::default();
            let mut benchmark_name = BoundedVec::default();
            benchmark_name.extend_from_slice(b"MMLU");
            benchmarks.try_push(Benchmark {
                name: benchmark_name,
                score: 8670,
            }).unwrap();

            let attestation = ModelAttestation {
                model_hash: [1u8; 32],
                training_data_hash,
                version,
                training_date: 1634567890,
                reproducible: true,
                benchmarks,
            };

            assert_ok!(AIDID::attest_model(
                RuntimeOrigin::signed(controller),
                did_hash,
                attestation
            ));
        });
    }

    #[test]
    fn test_record_inference() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile
            ));

            let did_hash = AIDID::hash_identifier(&identifier, &AIType::LLM);

            assert_ok!(AIDID::record_inference(
                RuntimeOrigin::signed(controller),
                did_hash,
                true
            ));

            let reputation = AIDID::get_reputation(did_hash);
            assert_eq!(reputation.total_inferences, 1);
            assert_eq!(reputation.successful_inferences, 1);
        });
    }

    #[test]
    fn test_submit_rating() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile
            ));

            let did_hash = AIDID::hash_identifier(&identifier, &AIType::LLM);

            assert_ok!(AIDID::submit_rating(
                RuntimeOrigin::signed(controller),
                did_hash,
                9000
            ));

            let reputation = AIDID::get_reputation(did_hash);
            assert_eq!(reputation.rating_count, 1);
            assert_eq!(reputation.user_rating, 9000);
        });
    }

    #[test]
    fn test_deactivate_and_reactivate() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile
            ));

            let did_hash = AIDID::hash_identifier(&identifier, &AIType::LLM);

            assert!(AIDID::is_active(did_hash));

            assert_ok!(AIDID::deactivate_ai(
                RuntimeOrigin::signed(controller),
                did_hash
            ));

            assert!(!AIDID::is_active(did_hash));

            assert_ok!(AIDID::reactivate_ai(
                RuntimeOrigin::signed(controller),
                did_hash
            ));

            assert!(AIDID::is_active(did_hash));
        });
    }

    #[test]
    fn test_grant_permission() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"gpt-4".to_vec();
            let profile = create_test_profile();

            assert_ok!(AIDID::register_ai(
                RuntimeOrigin::signed(controller),
                identifier.clone(),
                AIType::LLM,
                profile
            ));

            let did_hash = AIDID::hash_identifier(&identifier, &AIType::LLM);

            let mut action = BoundedVec::default();
            action.extend_from_slice(b"read");

            let mut resource = BoundedVec::default();
            resource.extend_from_slice(b"/data/users");

            let permission = Permission {
                action,
                resource,
                conditions: BoundedVec::default(),
            };

            assert_ok!(AIDID::grant_permission(
                RuntimeOrigin::signed(controller),
                did_hash,
                permission
            ));
        });
    }

    #[test]
    fn test_reputation_calculation() {
        let mut rep = Reputation::default();

        // Record 90 successful and 10 failed inferences
        for _ in 0..90 {
            rep.record_inference(true);
        }
        for _ in 0..10 {
            rep.record_inference(false);
        }

        assert_eq!(rep.total_inferences, 100);
        assert_eq!(rep.successful_inferences, 90);
        assert_eq!(rep.success_rate(), 9000); // 90%
    }

    #[test]
    fn test_invalid_profile_no_capabilities() {
        new_test_ext().execute_with(|| {
            let controller = 1u64;
            let identifier = b"invalid-ai".to_vec();

            let mut version = BoundedVec::default();
            version.extend_from_slice(b"v1.0.0");

            let mut alignment_method = BoundedVec::default();
            alignment_method.extend_from_slice(b"RLHF");

            let profile = AIProfile {
                ai_type: AIType::LLM,
                version,
                architecture: BoundedVec::default(),
                parameters: BoundedVec::default(),
                capabilities: Capabilities::default(), // Empty capabilities
                restrictions: Restrictions::default(),
                safety: SafetyProfile {
                    alignment_method,
                    content_filtering: true,
                    bias_evaluated: true,
                    toxicity_score: 100,
                },
            };

            assert_err!(
                AIDID::register_ai(
                    RuntimeOrigin::signed(controller),
                    identifier,
                    AIType::LLM,
                    profile
                ),
                Error::<Test>::NoCapabilities
            );
        });
    }
}
