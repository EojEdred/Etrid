//! # Pallet ASF Registry
//!
//! Dynamic ASF (Ascending Scale of Finality) validator registration pallet.
//!
//! ## Overview
//!
//! This pallet allows validators to register their ASF public keys on-chain,
//! enabling dynamic validator set management without requiring binary upgrades.
//!
//! ## Key Features
//!
//! - **Dynamic Registration**: Validators can register/deregister ASF keys via extrinsics
//! - **Stake Validation**: Only staked validators can register ASF keys
//! - **Runtime API**: Provides `asf_validator_set()` for querying registered keys
//! - **On-chain Verifiable**: All registered keys are stored on-chain and queryable
//!
//! ## Usage
//!
//! 1. Validator generates ASF key locally (sr25519, key_type: asfk)
//! 2. Validator calls `register_asf_key(asf_pubkey)` extrinsic
//! 3. ASF service queries `asf_validator_set()` runtime API for authority set
//! 4. PPFA committee uses same API for proposer selection

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use alloc::vec::Vec;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_core::crypto::KeyTypeId;

    /// ASF key type identifier: "asfk" (0x6173666b)
    pub const ASF_KEY_TYPE: KeyTypeId = KeyTypeId(*b"asfk");

    /// ASF public key type (sr25519, 32 bytes)
    pub type AsfPublicKey = [u8; 32];

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum number of registered ASF validators
        #[pallet::constant]
        type MaxAsfValidators: Get<u32>;

        /// Validator stake checker (optional integration with staking pallet)
        type StakeChecker: ValidatorStakeChecker<Self::AccountId>;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // STORAGE ITEMS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Mapping from AccountId to their registered ASF public key
    #[pallet::storage]
    #[pallet::getter(fn asf_validators)]
    pub type AsfValidators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        AsfPublicKey,
        OptionQuery,
    >;

    /// Ordered list of registered ASF public keys (for quick iteration)
    /// This is the authoritative validator set used by ASF consensus
    #[pallet::storage]
    #[pallet::getter(fn validator_set)]
    pub type ValidatorSet<T: Config> = StorageValue<
        _,
        BoundedVec<AsfPublicKey, T::MaxAsfValidators>,
        ValueQuery,
    >;

    /// Reverse mapping from ASF public key to AccountId
    /// Used for deregistration and validation
    #[pallet::storage]
    #[pallet::getter(fn key_owner)]
    pub type KeyOwner<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        AsfPublicKey,
        T::AccountId,
        OptionQuery,
    >;

    /// Current validator set version (incremented on changes)
    /// Used by ASF service to detect authority set changes
    #[pallet::storage]
    #[pallet::getter(fn validator_set_version)]
    pub type ValidatorSetVersion<T: Config> = StorageValue<_, u64, ValueQuery>;

    // ═══════════════════════════════════════════════════════════════════════════
    // GENESIS CONFIGURATION
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Initial validators (account_id, asf_public_key)
        pub initial_validators: Vec<(T::AccountId, AsfPublicKey)>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            let mut validator_set: BoundedVec<AsfPublicKey, T::MaxAsfValidators> =
                BoundedVec::default();

            for (account_id, asf_key) in &self.initial_validators {
                AsfValidators::<T>::insert(account_id, asf_key);
                KeyOwner::<T>::insert(asf_key, account_id);
                let _ = validator_set.try_push(*asf_key);
            }

            ValidatorSet::<T>::put(validator_set);
            ValidatorSetVersion::<T>::put(1u64);

            log::info!(
                "ASF Registry: Genesis initialized with {} validators",
                self.initial_validators.len()
            );
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EVENTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A validator registered their ASF public key
        AsfKeyRegistered {
            account: T::AccountId,
            asf_key: AsfPublicKey,
            validator_count: u32,
        },
        /// A validator deregistered their ASF public key
        AsfKeyDeregistered {
            account: T::AccountId,
            asf_key: AsfPublicKey,
            validator_count: u32,
        },
        /// Validator set version changed
        ValidatorSetChanged {
            version: u64,
            validator_count: u32,
        },
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ERRORS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::error]
    pub enum Error<T> {
        /// Account is not a staked validator
        NotStakedValidator,
        /// Account already has a registered ASF key
        AlreadyRegistered,
        /// Account does not have a registered ASF key
        NotRegistered,
        /// ASF key is already registered by another account
        KeyAlreadyInUse,
        /// Maximum number of validators reached
        TooManyValidators,
        /// Invalid ASF public key format
        InvalidPublicKey,
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // CALLABLE FUNCTIONS (EXTRINSICS)
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register an ASF public key for the calling validator
        ///
        /// # Parameters
        /// - `origin`: Must be a staked validator
        /// - `asf_pubkey`: The sr25519 public key to register (32 bytes)
        ///
        /// # Errors
        /// - `NotStakedValidator`: If caller is not a staked validator
        /// - `AlreadyRegistered`: If caller already has a registered key
        /// - `KeyAlreadyInUse`: If the key is registered by another account
        /// - `TooManyValidators`: If maximum validator count is reached
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(50_000, 0))]
        pub fn register_asf_key(
            origin: OriginFor<T>,
            asf_pubkey: AsfPublicKey,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify caller is a staked validator
            ensure!(
                T::StakeChecker::is_staked_validator(&who),
                Error::<T>::NotStakedValidator
            );

            // Verify caller doesn't already have a registered key
            ensure!(
                !AsfValidators::<T>::contains_key(&who),
                Error::<T>::AlreadyRegistered
            );

            // Verify key isn't already in use
            ensure!(
                !KeyOwner::<T>::contains_key(&asf_pubkey),
                Error::<T>::KeyAlreadyInUse
            );

            // Add to validator set
            ValidatorSet::<T>::try_mutate(|set| -> DispatchResult {
                set.try_push(asf_pubkey)
                    .map_err(|_| Error::<T>::TooManyValidators)?;
                Ok(())
            })?;

            // Store mappings
            AsfValidators::<T>::insert(&who, asf_pubkey);
            KeyOwner::<T>::insert(&asf_pubkey, &who);

            // Increment version
            let new_version = ValidatorSetVersion::<T>::mutate(|v| {
                *v += 1;
                *v
            });

            let validator_count = ValidatorSet::<T>::get().len() as u32;

            // Emit events
            Self::deposit_event(Event::AsfKeyRegistered {
                account: who,
                asf_key: asf_pubkey,
                validator_count,
            });

            Self::deposit_event(Event::ValidatorSetChanged {
                version: new_version,
                validator_count,
            });

            log::info!(
                "ASF Registry: Registered key {:?}, total validators: {}",
                &asf_pubkey[..4],
                validator_count
            );

            Ok(())
        }

        /// Deregister the ASF public key for the calling validator
        ///
        /// # Parameters
        /// - `origin`: Must have a registered ASF key
        ///
        /// # Errors
        /// - `NotRegistered`: If caller doesn't have a registered key
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(50_000, 0))]
        pub fn deregister_asf_key(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Get the registered key
            let asf_pubkey = AsfValidators::<T>::get(&who)
                .ok_or(Error::<T>::NotRegistered)?;

            // Remove from validator set
            ValidatorSet::<T>::mutate(|set| {
                set.retain(|k| k != &asf_pubkey);
            });

            // Remove mappings
            AsfValidators::<T>::remove(&who);
            KeyOwner::<T>::remove(&asf_pubkey);

            // Increment version
            let new_version = ValidatorSetVersion::<T>::mutate(|v| {
                *v += 1;
                *v
            });

            let validator_count = ValidatorSet::<T>::get().len() as u32;

            // Emit events
            Self::deposit_event(Event::AsfKeyDeregistered {
                account: who,
                asf_key: asf_pubkey,
                validator_count,
            });

            Self::deposit_event(Event::ValidatorSetChanged {
                version: new_version,
                validator_count,
            });

            log::info!(
                "ASF Registry: Deregistered key {:?}, total validators: {}",
                &asf_pubkey[..4],
                validator_count
            );

            Ok(())
        }

        /// Force-register an ASF key (root only, for genesis/recovery)
        ///
        /// # Parameters
        /// - `origin`: Must be root
        /// - `account`: The account to register for
        /// - `asf_pubkey`: The ASF public key to register
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(75_000, 0))]
        pub fn force_register_asf_key(
            origin: OriginFor<T>,
            account: T::AccountId,
            asf_pubkey: AsfPublicKey,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Remove existing key if any
            if let Some(old_key) = AsfValidators::<T>::get(&account) {
                ValidatorSet::<T>::mutate(|set| {
                    set.retain(|k| k != &old_key);
                });
                KeyOwner::<T>::remove(&old_key);
            }

            // Add to validator set
            ValidatorSet::<T>::try_mutate(|set| -> DispatchResult {
                if !set.contains(&asf_pubkey) {
                    set.try_push(asf_pubkey)
                        .map_err(|_| Error::<T>::TooManyValidators)?;
                }
                Ok(())
            })?;

            // Store mappings
            AsfValidators::<T>::insert(&account, asf_pubkey);
            KeyOwner::<T>::insert(&asf_pubkey, &account);

            // Increment version
            let new_version = ValidatorSetVersion::<T>::mutate(|v| {
                *v += 1;
                *v
            });

            let validator_count = ValidatorSet::<T>::get().len() as u32;

            Self::deposit_event(Event::AsfKeyRegistered {
                account,
                asf_key: asf_pubkey,
                validator_count,
            });

            Self::deposit_event(Event::ValidatorSetChanged {
                version: new_version,
                validator_count,
            });

            Ok(())
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PUBLIC HELPER FUNCTIONS (FOR RUNTIME API)
    // ═══════════════════════════════════════════════════════════════════════════

    impl<T: Config> Pallet<T> {
        /// Get the current ASF validator set as a Vec of public keys
        /// This is the main function called by the ASF service via runtime API
        pub fn asf_validator_set() -> Vec<AsfPublicKey> {
            ValidatorSet::<T>::get().into_inner()
        }

        /// Get the current validator set version
        /// Used by ASF service to detect when authority set changes
        pub fn get_validator_set_version() -> u64 {
            ValidatorSetVersion::<T>::get()
        }

        /// Get the number of registered validators
        pub fn validator_count() -> u32 {
            ValidatorSet::<T>::get().len() as u32
        }

        /// Check if an account has a registered ASF key
        pub fn is_registered(account: &T::AccountId) -> bool {
            AsfValidators::<T>::contains_key(account)
        }

        /// Get the ASF key for an account
        pub fn get_asf_key(account: &T::AccountId) -> Option<AsfPublicKey> {
            AsfValidators::<T>::get(account)
        }

        /// Get the account that owns an ASF key
        pub fn get_key_owner(asf_key: &AsfPublicKey) -> Option<T::AccountId> {
            KeyOwner::<T>::get(asf_key)
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TRAIT DEFINITIONS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Trait for checking if an account is a staked validator
    pub trait ValidatorStakeChecker<AccountId> {
        /// Returns true if the account is a staked validator
        fn is_staked_validator(account: &AccountId) -> bool;
    }

    /// Default implementation that allows all accounts (for testing)
    impl<AccountId> ValidatorStakeChecker<AccountId> for () {
        fn is_staked_validator(_account: &AccountId) -> bool {
            // Default: allow all accounts (useful for testing)
            // In production, this should check against pallet-staking
            true
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RUNTIME API DEFINITION
// ═══════════════════════════════════════════════════════════════════════════

sp_api::decl_runtime_apis! {
    /// Runtime API for ASF Registry pallet
    ///
    /// This API is called by the ASF service to query the validator set
    pub trait AsfRegistryApi {
        /// Get the current ASF validator set (public keys)
        fn asf_validator_set() -> sp_std::vec::Vec<[u8; 32]>;

        /// Get the current validator set version
        fn validator_set_version() -> u64;

        /// Get the number of registered validators
        fn validator_count() -> u32;
    }
}
