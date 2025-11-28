//! AIDID Registry
//!
//! On-chain registry for AI identities

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use frame_support::{
    pallet_prelude::*,
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_core::H256;

use crate::types::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum length of DID identifier
        #[pallet::constant]
        type MaxIdentifierLength: Get<u32>;

        /// Maximum number of capabilities
        #[pallet::constant]
        type MaxCapabilities: Get<u32>;

        /// Maximum number of benchmarks
        #[pallet::constant]
        type MaxBenchmarks: Get<u32>;
    }

    /// AI Identity Registry
    /// Maps DID identifier hash -> AI Identity
    #[pallet::storage]
    #[pallet::getter(fn ai_identity)]
    pub type AIIdentities<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // Hash of DID identifier
        AIIdentity<T>,
    >;

    /// Controller of AI identity (who can update it)
    #[pallet::storage]
    #[pallet::getter(fn ai_controller)]
    pub type AIController<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // DID hash
        T::AccountId,
    >;

    /// Reputation tracking
    #[pallet::storage]
    #[pallet::getter(fn ai_reputation)]
    pub type AIReputation<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // DID hash
        Reputation,
        ValueQuery,
    >;

    /// Authorization permissions
    #[pallet::storage]
    #[pallet::getter(fn ai_permissions)]
    pub type AIPermissions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, H256,  // DID hash
        Blake2_128Concat, H256,  // Permission hash
        Permission,
    >;

    /// AI Identity structure
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct AIIdentity<T: Config> {
        /// DID identifier (e.g., "gpt4-turbo")
        pub identifier: BoundedVec<u8, T::MaxIdentifierLength>,
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
        /// Registration timestamp
        pub registered_at: BlockNumberFor<T>,
        /// Last update timestamp
        pub updated_at: BlockNumberFor<T>,
        /// Active status
        pub active: bool,
    }

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
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new AI identity
        #[pallet::weight(100_000)]
        #[pallet::call_index(0)]
        pub fn register_ai(
            origin: OriginFor<T>,
            identifier: Vec<u8>,
            ai_type: AIType,
            profile: AIProfile,
        ) -> DispatchResult {
            let controller = ensure_signed(origin)?;

            // Create bounded identifier
            let bounded_identifier = BoundedVec::<u8, T::MaxIdentifierLength>::try_from(identifier.clone())
                .map_err(|_| Error::<T>::IdentifierTooLong)?;

            // Hash the identifier for storage key
            let did_hash = Self::hash_identifier(&identifier, &ai_type);

            // Ensure doesn't already exist
            ensure!(!AIIdentities::<T>::contains_key(did_hash), Error::<T>::AIAlreadyExists);

            // Create AI identity
            let identity = AIIdentity {
                identifier: bounded_identifier,
                ai_type,
                controller: controller.clone(),
                profile,
                attestation: None,
                pricing: None,
                registered_at: frame_system::Pallet::<T>::block_number(),
                updated_at: frame_system::Pallet::<T>::block_number(),
                active: true,
            };

            // Store identity
            AIIdentities::<T>::insert(did_hash, identity);
            AIController::<T>::insert(did_hash, controller.clone());

            Self::deposit_event(Event::AIRegistered {
                did_hash,
                controller,
                ai_type,
            });

            Ok(())
        }

        /// Update AI profile
        #[pallet::weight(50_000)]
        #[pallet::call_index(1)]
        pub fn update_profile(
            origin: OriginFor<T>,
            did_hash: H256,
            new_profile: AIProfile,
        ) -> DispatchResult {
            let updater = ensure_signed(origin)?;

            // Get and verify identity
            let mut identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.controller == updater, Error::<T>::NotController);

            // Update profile
            identity.profile = new_profile;
            identity.updated_at = frame_system::Pallet::<T>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::AIUpdated {
                did_hash,
                updated_by: updater,
            });

            Ok(())
        }

        /// Attest model
        #[pallet::weight(75_000)]
        #[pallet::call_index(2)]
        pub fn attest_model(
            origin: OriginFor<T>,
            did_hash: H256,
            attestation: ModelAttestation,
        ) -> DispatchResult {
            let attester = ensure_signed(origin)?;

            // Get and verify identity
            let mut identity = AIIdentities::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;

            ensure!(identity.controller == attester, Error::<T>::NotController);

            let model_hash = attestation.model_hash;

            // Store attestation
            identity.attestation = Some(attestation);
            identity.updated_at = frame_system::Pallet::<T>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::ModelAttested {
                did_hash,
                model_hash,
            });

            Ok(())
        }

        /// Grant permission
        #[pallet::weight(25_000)]
        #[pallet::call_index(3)]
        pub fn grant_permission(
            origin: OriginFor<T>,
            did_hash: H256,
            permission: Permission,
        ) -> DispatchResult {
            let granter = ensure_signed(origin)?;

            // Verify controller
            let controller = AIController::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;
            ensure!(controller == granter, Error::<T>::NotController);

            // Hash permission for storage
            let permission_hash = sp_io::hashing::blake2_256(
                &[&permission.action[..], &permission.resource[..]].concat()
            );
            let permission_hash = H256::from(permission_hash);

            // Store permission
            AIPermissions::<T>::insert(did_hash, permission_hash, permission);

            Self::deposit_event(Event::PermissionGranted {
                did_hash,
                permission_hash,
            });

            Ok(())
        }

        /// Revoke permission
        #[pallet::weight(25_000)]
        #[pallet::call_index(4)]
        pub fn revoke_permission(
            origin: OriginFor<T>,
            did_hash: H256,
            permission_hash: H256,
        ) -> DispatchResult {
            let revoker = ensure_signed(origin)?;

            // Verify controller
            let controller = AIController::<T>::get(did_hash)
                .ok_or(Error::<T>::AINotFound)?;
            ensure!(controller == revoker, Error::<T>::NotController);

            // Remove permission
            AIPermissions::<T>::remove(did_hash, permission_hash);

            Self::deposit_event(Event::PermissionRevoked {
                did_hash,
                permission_hash,
            });

            Ok(())
        }

        /// Record inference
        #[pallet::weight(10_000)]
        #[pallet::call_index(5)]
        pub fn record_inference(
            origin: OriginFor<T>,
            did_hash: H256,
            success: bool,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            // Verify AI exists
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

        /// Submit rating
        #[pallet::weight(10_000)]
        #[pallet::call_index(6)]
        pub fn submit_rating(
            origin: OriginFor<T>,
            did_hash: H256,
            rating: u32,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            // Validate rating
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
        #[pallet::weight(25_000)]
        #[pallet::call_index(7)]
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
            identity.updated_at = frame_system::Pallet::<T>::block_number();

            AIIdentities::<T>::insert(did_hash, identity);

            Self::deposit_event(Event::AIDeactivated {
                did_hash,
            });

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Hash DID identifier for storage key
        fn hash_identifier(identifier: &[u8], ai_type: &AIType) -> H256 {
            let type_str = ai_type.to_str().as_bytes();
            let combined = [type_str, identifier].concat();
            H256::from(sp_io::hashing::blake2_256(&combined))
        }

        /// Get AI identity by hash
        pub fn get_ai_identity(did_hash: H256) -> Option<AIIdentity<T>> {
            AIIdentities::<T>::get(did_hash)
        }

        /// Check if AI has permission
        pub fn has_permission(did_hash: H256, action: &[u8], resource: &[u8]) -> bool {
            let permission_hash = sp_io::hashing::blake2_256(&[action, resource].concat());
            let permission_hash = H256::from(permission_hash);
            AIPermissions::<T>::contains_key(did_hash, permission_hash)
        }
    }
}
