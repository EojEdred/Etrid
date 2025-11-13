//! # Model Registry Pallet
//!
//! ## Overview
//! The Model Registry pallet manages AI model registration with AIDID integration.
//!
//! ## Features
//! - Register AI models with unique AIDID (did:ai:model-name:version)
//! - Model versioning (like Git for AI models)
//! - Royalty management (model creators earn % of inference fees)
//! - Model verification (cryptographic proof of model weights)
//! - Public/private models
//!
//! ## AIDID Integration
//! Every model gets a W3C-compliant Decentralized Identifier:
//! - did:ai:gpt4:v20250101
//! - did:ai:stable-diffusion:v3.0
//! - did:ai:whisper:v2.5
//!
//! This provides:
//! - Verifiable model identity (prove you're running real GPT-4)
//! - Cross-chain reputation (use AIDID on any blockchain)
//! - Audit trail (all model versions immutably recorded)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, Hash};
    use sp_std::vec::Vec;
    use aidid::AiDid;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Model visibility
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ModelVisibility {
        /// Anyone can use (paid)
        Public,
        /// Only owner can use
        Private,
        /// Whitelisted accounts only
        Restricted,
    }

    /// Model license type
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum LicenseType {
        /// MIT license
        MIT,
        /// Apache 2.0
        Apache2,
        /// GPL v3
        GPLv3,
        /// Proprietary (custom terms)
        Proprietary,
        /// Creative Commons
        CreativeCommons,
    }

    /// AI model metadata
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct ModelMetadata<T: Config> {
        /// Model creator/owner
        pub owner: T::AccountId,
        /// Model name (e.g., "gpt-4", "stable-diffusion")
        pub name: BoundedVec<u8, ConstU32<64>>,
        /// Version string (e.g., "v1.0.0", "v20250101")
        pub version: BoundedVec<u8, ConstU32<32>>,
        /// Model description
        pub description: BoundedVec<u8, ConstU32<256>>,
        /// Model weights hash (cryptographic proof)
        pub weights_hash: T::Hash,
        /// AIDID (W3C Decentralized Identifier)
        pub aidid: BoundedVec<u8, ConstU32<128>>, // e.g., "did:ai:gpt4:v20250101"
        /// Model file size in bytes
        pub size_bytes: u64,
        /// Visibility
        pub visibility: ModelVisibility,
        /// License type
        pub license: LicenseType,
        /// Royalty percentage (in basis points, e.g., 500 = 5%)
        pub royalty_bps: u16,
        /// Model category/type
        pub category: BoundedVec<u8, ConstU32<32>>,
        /// Registration timestamp
        pub registered_at: u64,
        /// Total inference count (usage tracking)
        pub inference_count: u64,
        /// Total royalties earned
        pub total_royalties: BalanceOf<T>,
    }

    /// Model version lineage (for Git-like versioning)
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ModelLineage {
        /// Parent model ID (if this is a version/fork)
        pub parent_id: Option<u64>,
        /// All version IDs in this lineage
        pub version_ids: BoundedVec<u64, ConstU32<100>>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency type (ËDSC)
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Registration fee (spam prevention)
        #[pallet::constant]
        type RegistrationFee: Get<BalanceOf<Self>>;

        /// Maximum royalty percentage (in basis points)
        #[pallet::constant]
        type MaxRoyaltyBps: Get<u16>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Models by ID
    #[pallet::storage]
    #[pallet::getter(fn models)]
    pub type Models<T: Config> = StorageMap<_, Blake2_128Concat, u64, ModelMetadata<T>>;

    /// Model ID counter
    #[pallet::storage]
    #[pallet::getter(fn next_model_id)]
    pub type NextModelId<T> = StorageValue<_, u64, ValueQuery>;

    /// Owner's model IDs
    #[pallet::storage]
    #[pallet::getter(fn owner_models)]
    pub type OwnerModels<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u64, ConstU32<100>>,
        ValueQuery,
    >;

    /// Model name → Model ID mapping (for lookups)
    #[pallet::storage]
    #[pallet::getter(fn model_by_name)]
    pub type ModelByName<T> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, ConstU32<64>>, // model name
        BoundedVec<u64, ConstU32<100>>, // all version IDs
        ValueQuery,
    >;

    /// AIDID → Model ID mapping
    #[pallet::storage]
    #[pallet::getter(fn model_by_aidid)]
    pub type ModelByAidid<T> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, ConstU32<128>>, // AIDID
        u64, // model ID
    >;

    /// Model lineage (version history)
    #[pallet::storage]
    #[pallet::getter(fn model_lineage)]
    pub type ModelLineages<T> = StorageMap<_, Blake2_128Concat, u64, ModelLineage>;

    /// Total models registered
    #[pallet::storage]
    #[pallet::getter(fn total_models)]
    pub type TotalModels<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Model registered [model_id, owner, aidid]
        ModelRegistered {
            model_id: u64,
            owner: T::AccountId,
            aidid: BoundedVec<u8, ConstU32<128>>,
        },
        /// Model updated [model_id]
        ModelUpdated { model_id: u64 },
        /// New version published [model_id, parent_id]
        VersionPublished { model_id: u64, parent_id: u64 },
        /// Royalty paid [model_id, owner, amount]
        RoyaltyPaid { model_id: u64, owner: T::AccountId, amount: BalanceOf<T> },
        /// Inference counted [model_id]
        InferenceCounted { model_id: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Model not found
        ModelNotFound,
        /// Not the model owner
        NotModelOwner,
        /// Model name already exists
        ModelNameExists,
        /// AIDID already exists
        AididExists,
        /// Invalid royalty percentage
        InvalidRoyalty,
        /// Insufficient balance
        InsufficientBalance,
        /// Overflow
        Overflow,
        /// Too many models
        TooManyModels,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new AI model
        ///
        /// # Arguments
        /// * `origin` - Model owner
        /// * `name` - Model name (e.g., "gpt-4")
        /// * `version` - Version string (e.g., "v1.0.0")
        /// * `description` - Model description
        /// * `weights_hash` - Cryptographic hash of model weights
        /// * `size_bytes` - Model file size
        /// * `visibility` - Public/Private/Restricted
        /// * `license` - License type
        /// * `royalty_bps` - Royalty % in basis points (e.g., 500 = 5%)
        /// * `category` - Model category (LLM, ImageGen, etc.)
        #[pallet::call_index(0)]
        #[pallet::weight(15_000)]
        pub fn register_model(
            origin: OriginFor<T>,
            name: BoundedVec<u8, ConstU32<64>>,
            version: BoundedVec<u8, ConstU32<32>>,
            description: BoundedVec<u8, ConstU32<256>>,
            weights_hash: T::Hash,
            size_bytes: u64,
            visibility: ModelVisibility,
            license: LicenseType,
            royalty_bps: u16,
            category: BoundedVec<u8, ConstU32<32>>,
            parent_id: Option<u64>, // For versioning (None = new model, Some = new version)
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // Validate royalty
            ensure!(royalty_bps <= T::MaxRoyaltyBps::get(), Error::<T>::InvalidRoyalty);

            // Charge registration fee
            T::Currency::withdraw(
                &owner,
                T::RegistrationFee::get(),
                frame_support::traits::WithdrawReasons::FEE,
                frame_support::traits::ExistenceRequirement::KeepAlive,
            )?;

            // Generate AIDID (did:ai:{name}:{version})
            let aidid = Self::generate_aidid(&name, &version);

            // Ensure AIDID unique
            ensure!(!ModelByAidid::<T>::contains_key(&aidid), Error::<T>::AididExists);

            // Get next model ID
            let model_id = NextModelId::<T>::get();
            let next_id = model_id.checked_add(1).ok_or(Error::<T>::Overflow)?;
            NextModelId::<T>::put(next_id);

            // Create model metadata
            let metadata = ModelMetadata {
                owner: owner.clone(),
                name: name.clone(),
                version,
                description,
                weights_hash,
                aidid: aidid.clone(),
                size_bytes,
                visibility,
                license,
                royalty_bps,
                category,
                registered_at: Self::current_timestamp(),
                inference_count: 0,
                total_royalties: 0_u32.into(),
            };

            // Store model
            Models::<T>::insert(model_id, metadata);

            // Add to owner's models
            OwnerModels::<T>::try_mutate(&owner, |models| {
                models.try_push(model_id).map_err(|_| Error::<T>::TooManyModels)?;
                Ok::<(), DispatchError>(())
            })?;

            // Add to name index
            ModelByName::<T>::try_mutate(&name, |ids| {
                ids.try_push(model_id).map_err(|_| Error::<T>::Overflow)?;
                Ok::<(), DispatchError>(())
            })?;

            // Add to AIDID index
            ModelByAidid::<T>::insert(&aidid, model_id);

            // Handle versioning
            if let Some(parent) = parent_id {
                // This is a new version of existing model
                ModelLineages::<T>::try_mutate(parent, |maybe_lineage| {
                    if let Some(lineage) = maybe_lineage {
                        lineage.version_ids.try_push(model_id)
                            .map_err(|_| Error::<T>::Overflow)?;
                    } else {
                        // Create new lineage
                        *maybe_lineage = Some(ModelLineage {
                            parent_id: None,
                            version_ids: BoundedVec::try_from(vec![parent, model_id])
                                .map_err(|_| Error::<T>::Overflow)?,
                        });
                    }
                    Ok::<(), DispatchError>(())
                })?;

                Self::deposit_event(Event::VersionPublished { model_id, parent_id: parent });
            }

            // Update metrics
            TotalModels::<T>::mutate(|total| *total = total.saturating_add(1));

            Self::deposit_event(Event::ModelRegistered { model_id, owner, aidid });
            Ok(())
        }

        /// Update model visibility
        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn update_visibility(
            origin: OriginFor<T>,
            model_id: u64,
            visibility: ModelVisibility,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            Models::<T>::try_mutate(model_id, |maybe_model| {
                let model = maybe_model.as_mut().ok_or(Error::<T>::ModelNotFound)?;
                ensure!(model.owner == owner, Error::<T>::NotModelOwner);

                model.visibility = visibility;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::ModelUpdated { model_id });
            Ok(())
        }

        /// Record inference (called by job-marketplace)
        #[pallet::call_index(2)]
        #[pallet::weight(3_000)]
        pub fn record_inference(origin: OriginFor<T>, model_id: u64) -> DispatchResult {
            ensure_root(origin)?; // Only callable by other pallets

            Models::<T>::try_mutate(model_id, |maybe_model| {
                let model = maybe_model.as_mut().ok_or(Error::<T>::ModelNotFound)?;
                model.inference_count = model.inference_count.saturating_add(1);
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::InferenceCounted { model_id });
            Ok(())
        }

        /// Pay royalty to model creator
        #[pallet::call_index(3)]
        #[pallet::weight(5_000)]
        pub fn pay_royalty(
            origin: OriginFor<T>,
            model_id: u64,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?; // Only callable by other pallets

            Models::<T>::try_mutate(model_id, |maybe_model| {
                let model = maybe_model.as_mut().ok_or(Error::<T>::ModelNotFound)?;
                model.total_royalties = model.total_royalties.saturating_add(amount);

                // TODO: Transfer royalty to model owner
                // T::Currency::transfer(...);

                Self::deposit_event(Event::RoyaltyPaid {
                    model_id,
                    owner: model.owner.clone(),
                    amount,
                });

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Generate AIDID from model name and version
        /// Format: did:ai:{name}:{version}
        fn generate_aidid(
            name: &BoundedVec<u8, ConstU32<64>>,
            version: &BoundedVec<u8, ConstU32<32>>,
        ) -> BoundedVec<u8, ConstU32<128>> {
            let mut aidid = Vec::new();
            aidid.extend_from_slice(b"did:ai:");
            aidid.extend_from_slice(name.as_slice());
            aidid.extend_from_slice(b":");
            aidid.extend_from_slice(version.as_slice());

            BoundedVec::try_from(aidid).unwrap_or_default()
        }

        /// Get current timestamp
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number()
                .try_into()
                .unwrap_or(0u64)
                .saturating_mul(6)
        }

        /// Get model royalty percentage
        pub fn get_royalty_bps(model_id: u64) -> Option<u16> {
            Models::<T>::get(model_id).map(|m| m.royalty_bps)
        }

        /// Verify model weights hash (cryptographic proof)
        pub fn verify_model_weights(model_id: u64, claimed_hash: T::Hash) -> bool {
            if let Some(model) = Models::<T>::get(model_id) {
                model.weights_hash == claimed_hash
            } else {
                false
            }
        }
    }
}
