//! # Pallet DID Registry
//!
//! ## Overview
//!
//! This pallet provides on-chain DID (Decentralized Identifier) registration, management,
//! and resolution for the Ëtrid blockchain, following the W3C DID specification with
//! Ëtrid-specific extensions.
//!
//! ## Features
//!
//! - W3C DID-compliant decentralized identity management
//! - On-chain DID document storage and retrieval
//! - Flexible access control with multiple permission levels
//! - Ownership transfer and delegation capabilities
//! - DID expiration and revocation mechanisms
//! - Support for controller and owner separation
//!
//! ## Extrinsics
//!
//! - `register_did` - Register a new DID with document hash
//! - `update_did` - Update DID document hash (owner or controller)
//! - `revoke_did` - Revoke a DID permanently (owner only)
//! - `transfer_ownership` - Transfer DID ownership to another account
//! - `set_expiration` - Set expiration block for a DID
//! - `grant_access` - Grant access permissions to an agent
//! - `revoke_access` - Revoke access permissions from an agent
//!
//! ## Usage Example
//!
//! ```ignore
//! // Register a new DID
//! DidRegistry::register_did(
//!     Origin::signed(alice),
//!     b"did:etrid:alice".to_vec(),
//!     bob, // controller
//!     b"QmHash123...".to_vec(), // document hash
//! )?;
//!
//! // Update DID document
//! DidRegistry::update_did(
//!     Origin::signed(alice),
//!     did_hash,
//!     b"QmNewHash456...".to_vec(),
//! )?;
//!
//! // Grant read access to charlie
//! DidRegistry::grant_access(
//!     Origin::signed(alice),
//!     did_hash,
//!     charlie,
//!     AccessLevel::Reader,
//! )?;
//! ```
//!
//! ## Storage Items
//!
//! - `Registrations` - Maps DID hash to registration record
//! - `OwnerDids` - Maps owner account to list of owned DIDs
//! - `AccessControlList` - Maps DID and agent to access permissions
//! - `TotalDids` - Total number of registered DIDs
//! - `Nonce` - Nonce counter for unique operations
//!
//! ## Events
//!
//! - `DidRegistered` - When a new DID is registered
//! - `DidUpdated` - When a DID document is updated
//! - `DidRevoked` - When a DID is revoked
//! - `OwnershipTransferred` - When DID ownership changes
//! - `ExpirationSet` - When DID expiration is configured
//! - `AccessGranted` - When access is granted to an agent
//! - `AccessRevoked` - When access is revoked from an agent
//!
//! ## Errors
//!
//! - `DidAlreadyExists` - Attempting to register an existing DID
//! - `DidNotFound` - DID does not exist
//! - `NotAuthorized` - Caller lacks required permissions
//! - `DidRevoked` - DID has been revoked
//! - `DidExpired` - DID has expired
//!
//! ## W3C DID Format
//!
//! DIDs follow the format: `did:etrid:{identifier}`
//! where identifier is a unique string hashed for storage

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{Decode, Encode, MaxEncodedLen, DecodeWithMemTracking};
    use frame_support::pallet_prelude::*;
    use frame_support::BoundedVec;
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_core::H256;
    use sp_runtime::RuntimeDebug;
    use sp_std::vec::Vec;

    /// Maximum DID identifier length
    pub const MAX_DID_IDENTIFIER_LENGTH: u32 = 64;
    /// Maximum document hash length
    pub const MAX_DOCUMENT_HASH_LENGTH: u32 = 64;
    /// Maximum controller identifier length
    pub const MAX_CONTROLLER_LENGTH: u32 = 64;

    /// DID Registration entry
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct DidRegistration<T: Config> {
        /// DID identifier
        pub did: BoundedVec<u8, ConstU32<MAX_DID_IDENTIFIER_LENGTH>>,
        /// Owner account
        pub owner: T::AccountId,
        /// Controller (can be different from owner)
        pub controller: T::AccountId,
        /// Document hash (hash of the full DID document stored off-chain)
        pub document_hash: BoundedVec<u8, ConstU32<MAX_DOCUMENT_HASH_LENGTH>>,
        /// Registration block
        pub registered_at: BlockNumberFor<T>,
        /// Last update block
        pub updated_at: BlockNumberFor<T>,
        /// Expiration block (if set)
        pub expires_at: Option<BlockNumberFor<T>>,
        /// Revoked status
        pub revoked: bool,
    }

    impl<T: Config> DidRegistration<T> {
        pub fn is_active(&self, current_block: BlockNumberFor<T>) -> bool {
            !self.revoked && (self.expires_at.is_none() || self.expires_at.unwrap() > current_block)
        }

        pub fn is_expired(&self, current_block: BlockNumberFor<T>) -> bool {
            self.expires_at.map_or(false, |exp| exp <= current_block)
        }
    }

    /// Access control level
    #[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
    pub enum AccessLevel {
        None,
        Reader,
        Writer,
        Admin,
    }

    impl DecodeWithMemTracking for AccessLevel {}

    /// Access control entry
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct AccessControl<T: Config> {
        pub did_hash: H256,
        pub agent: T::AccountId,
        pub level: AccessLevel,
        pub granted_at: BlockNumberFor<T>,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum number of access control entries per DID
        #[pallet::constant]
        type MaxAccessControlEntries: Get<u32>;
    }

    /// DID registrations (DID hash => Registration)
    #[pallet::storage]
    #[pallet::getter(fn registrations)]
    pub type Registrations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // Hash of DID identifier
        DidRegistration<T>,
        OptionQuery,
    >;

    /// Owner to DIDs mapping (Owner => Vec<DID Hash>)
    #[pallet::storage]
    #[pallet::getter(fn owner_dids)]
    pub type OwnerDids<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<H256, T::MaxAccessControlEntries>,
        ValueQuery,
    >;

    /// Access control list (DID hash => Agent => AccessControl)
    #[pallet::storage]
    #[pallet::getter(fn access_control)]
    pub type AccessControlList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, H256,         // DID hash
        Blake2_128Concat, T::AccountId, // Agent
        AccessControl<T>,
        OptionQuery,
    >;

    /// Total number of registered DIDs
    #[pallet::storage]
    #[pallet::getter(fn total_dids)]
    pub type TotalDids<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Nonce counter for unique operations
    #[pallet::storage]
    #[pallet::getter(fn nonce)]
    pub type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// DID registered
        DidRegistered {
            did_hash: H256,
            owner: T::AccountId,
        },
        /// DID document updated
        DidUpdated {
            did_hash: H256,
            updated_by: T::AccountId,
        },
        /// DID revoked
        DidRevoked {
            did_hash: H256,
            revoked_by: T::AccountId,
        },
        /// DID ownership transferred
        OwnershipTransferred {
            did_hash: H256,
            old_owner: T::AccountId,
            new_owner: T::AccountId,
        },
        /// DID expiration set
        ExpirationSet {
            did_hash: H256,
            expires_at: BlockNumberFor<T>,
        },
        /// Access granted
        AccessGranted {
            did_hash: H256,
            agent: T::AccountId,
            level: AccessLevel,
        },
        /// Access revoked
        AccessRevoked {
            did_hash: H256,
            agent: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// DID already registered
        DidAlreadyExists,
        /// DID not found
        DidNotFound,
        /// Not authorized to perform this operation
        NotAuthorized,
        /// DID identifier too long
        DidIdentifierTooLong,
        /// Document hash too long
        DocumentHashTooLong,
        /// DID is revoked
        DidRevoked,
        /// DID is expired
        DidExpired,
        /// Invalid expiration time
        InvalidExpiration,
        /// Too many access control entries
        TooManyAccessEntries,
        /// Cannot transfer to same owner
        SameOwner,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new DID
        #[pallet::call_index(0)]
        #[pallet::weight(100_000)]
        pub fn register_did(
            origin: OriginFor<T>,
            did_identifier: Vec<u8>,
            controller: T::AccountId,
            document_hash: Vec<u8>,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // Validate lengths
            let bounded_did = BoundedVec::<u8, ConstU32<MAX_DID_IDENTIFIER_LENGTH>>::try_from(did_identifier.clone())
                .map_err(|_| Error::<T>::DidIdentifierTooLong)?;

            let bounded_hash = BoundedVec::<u8, ConstU32<MAX_DOCUMENT_HASH_LENGTH>>::try_from(document_hash)
                .map_err(|_| Error::<T>::DocumentHashTooLong)?;

            // Hash DID for storage key
            let did_hash = Self::hash_did(&did_identifier);

            // Ensure DID doesn't already exist
            ensure!(!Registrations::<T>::contains_key(did_hash), Error::<T>::DidAlreadyExists);

            let current_block = <frame_system::Pallet<T>>::block_number();

            // Create registration
            let registration = DidRegistration {
                did: bounded_did,
                owner: owner.clone(),
                controller,
                document_hash: bounded_hash,
                registered_at: current_block,
                updated_at: current_block,
                expires_at: None,
                revoked: false,
            };

            // Store registration
            Registrations::<T>::insert(did_hash, registration);

            // Add to owner's DID list
            OwnerDids::<T>::try_mutate(&owner, |dids| -> Result<(), DispatchError> {
                dids.try_push(did_hash).map_err(|_| Error::<T>::TooManyAccessEntries)?;
                Ok(())
            })?;

            // Increment counters
            TotalDids::<T>::mutate(|n| *n = n.saturating_add(1));
            Nonce::<T>::mutate(|n| *n = n.saturating_add(1));

            Self::deposit_event(Event::DidRegistered {
                did_hash,
                owner,
            });

            Ok(())
        }

        /// Update DID document hash
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn update_did(
            origin: OriginFor<T>,
            did_hash: H256,
            new_document_hash: Vec<u8>,
        ) -> DispatchResult {
            let updater = ensure_signed(origin)?;

            // Get registration
            let mut registration = Registrations::<T>::get(did_hash)
                .ok_or(Error::<T>::DidNotFound)?;

            // Check authorization (owner or controller)
            ensure!(
                registration.owner == updater || registration.controller == updater,
                Error::<T>::NotAuthorized
            );

            // Check if revoked
            ensure!(!registration.revoked, Error::<T>::DidRevoked);

            // Validate new hash
            let bounded_hash = BoundedVec::<u8, ConstU32<MAX_DOCUMENT_HASH_LENGTH>>::try_from(new_document_hash)
                .map_err(|_| Error::<T>::DocumentHashTooLong)?;

            // Update
            let current_block = <frame_system::Pallet<T>>::block_number();
            registration.document_hash = bounded_hash;
            registration.updated_at = current_block;

            Registrations::<T>::insert(did_hash, registration);

            Self::deposit_event(Event::DidUpdated {
                did_hash,
                updated_by: updater,
            });

            Ok(())
        }

        /// Revoke a DID
        #[pallet::call_index(2)]
        #[pallet::weight(50_000)]
        pub fn revoke_did(
            origin: OriginFor<T>,
            did_hash: H256,
        ) -> DispatchResult {
            let revoker = ensure_signed(origin)?;

            // Get registration
            let mut registration = Registrations::<T>::get(did_hash)
                .ok_or(Error::<T>::DidNotFound)?;

            // Only owner can revoke
            ensure!(registration.owner == revoker, Error::<T>::NotAuthorized);

            // Revoke
            registration.revoked = true;
            registration.updated_at = <frame_system::Pallet<T>>::block_number();

            Registrations::<T>::insert(did_hash, registration);

            Self::deposit_event(Event::DidRevoked {
                did_hash,
                revoked_by: revoker,
            });

            Ok(())
        }

        /// Transfer DID ownership
        #[pallet::call_index(3)]
        #[pallet::weight(50_000)]
        pub fn transfer_ownership(
            origin: OriginFor<T>,
            did_hash: H256,
            new_owner: T::AccountId,
        ) -> DispatchResult {
            let current_owner = ensure_signed(origin)?;

            // Get registration
            let mut registration = Registrations::<T>::get(did_hash)
                .ok_or(Error::<T>::DidNotFound)?;

            // Only owner can transfer
            ensure!(registration.owner == current_owner, Error::<T>::NotAuthorized);

            // Check not same owner
            ensure!(current_owner != new_owner, Error::<T>::SameOwner);

            // Update owner lists
            OwnerDids::<T>::mutate(&current_owner, |dids| {
                dids.retain(|&h| h != did_hash);
            });

            OwnerDids::<T>::try_mutate(&new_owner, |dids| -> Result<(), DispatchError> {
                dids.try_push(did_hash).map_err(|_| Error::<T>::TooManyAccessEntries)?;
                Ok(())
            })?;

            // Update registration
            registration.owner = new_owner.clone();
            registration.updated_at = <frame_system::Pallet<T>>::block_number();

            Registrations::<T>::insert(did_hash, registration);

            Self::deposit_event(Event::OwnershipTransferred {
                did_hash,
                old_owner: current_owner,
                new_owner,
            });

            Ok(())
        }

        /// Set DID expiration
        #[pallet::call_index(4)]
        #[pallet::weight(25_000)]
        pub fn set_expiration(
            origin: OriginFor<T>,
            did_hash: H256,
            expires_at: BlockNumberFor<T>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Get registration
            let mut registration = Registrations::<T>::get(did_hash)
                .ok_or(Error::<T>::DidNotFound)?;

            // Check authorization
            ensure!(
                registration.owner == caller || registration.controller == caller,
                Error::<T>::NotAuthorized
            );

            // Validate expiration is in future
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(expires_at > current_block, Error::<T>::InvalidExpiration);

            // Set expiration
            registration.expires_at = Some(expires_at);
            registration.updated_at = current_block;

            Registrations::<T>::insert(did_hash, registration);

            Self::deposit_event(Event::ExpirationSet {
                did_hash,
                expires_at,
            });

            Ok(())
        }

        /// Grant access to an agent
        #[pallet::call_index(5)]
        #[pallet::weight(25_000)]
        pub fn grant_access(
            origin: OriginFor<T>,
            did_hash: H256,
            agent: T::AccountId,
            level: AccessLevel,
        ) -> DispatchResult {
            let granter = ensure_signed(origin)?;

            // Get registration
            let registration = Registrations::<T>::get(did_hash)
                .ok_or(Error::<T>::DidNotFound)?;

            // Only owner can grant access
            ensure!(registration.owner == granter, Error::<T>::NotAuthorized);

            let current_block = <frame_system::Pallet<T>>::block_number();

            // Create access control entry
            let access = AccessControl {
                did_hash,
                agent: agent.clone(),
                level: level.clone(),
                granted_at: current_block,
            };

            // Store access control
            AccessControlList::<T>::insert(did_hash, &agent, access);

            Self::deposit_event(Event::AccessGranted {
                did_hash,
                agent,
                level,
            });

            Ok(())
        }

        /// Revoke access from an agent
        #[pallet::call_index(6)]
        #[pallet::weight(25_000)]
        pub fn revoke_access(
            origin: OriginFor<T>,
            did_hash: H256,
            agent: T::AccountId,
        ) -> DispatchResult {
            let revoker = ensure_signed(origin)?;

            // Get registration
            let registration = Registrations::<T>::get(did_hash)
                .ok_or(Error::<T>::DidNotFound)?;

            // Only owner can revoke access
            ensure!(registration.owner == revoker, Error::<T>::NotAuthorized);

            // Remove access control
            AccessControlList::<T>::remove(did_hash, &agent);

            Self::deposit_event(Event::AccessRevoked {
                did_hash,
                agent,
            });

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Hash DID identifier for storage key
        pub fn hash_did(identifier: &[u8]) -> H256 {
            H256::from(sp_io::hashing::blake2_256(identifier))
        }

        /// Resolve DID (get registration)
        pub fn resolve_did(did_hash: H256) -> Option<DidRegistration<T>> {
            Registrations::<T>::get(did_hash)
        }

        /// Check if DID is active
        pub fn is_did_active(did_hash: H256) -> bool {
            if let Some(registration) = Registrations::<T>::get(did_hash) {
                let current_block = <frame_system::Pallet<T>>::block_number();
                registration.is_active(current_block)
            } else {
                false
            }
        }

        /// Check access level for agent
        pub fn check_access(did_hash: H256, agent: &T::AccountId) -> AccessLevel {
            AccessControlList::<T>::get(did_hash, agent)
                .map(|ac| ac.level)
                .unwrap_or(AccessLevel::None)
        }

        /// Get DIDs owned by account
        pub fn get_owner_dids(owner: &T::AccountId) -> Vec<H256> {
            OwnerDids::<T>::get(owner).into_iter().collect()
        }

        /// Get active DIDs count
        pub fn active_dids_count() -> u64 {
            let current_block = <frame_system::Pallet<T>>::block_number();
            let mut count = 0u64;

            for (_, registration) in Registrations::<T>::iter() {
                if registration.is_active(current_block) {
                    count = count.saturating_add(1);
                }
            }

            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_err, assert_ok, parameter_types,
        traits::{ConstU32, ConstU64},
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
            DidRegistry: crate,
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
        type MaxAccessControlEntries = ConstU32<100>;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    #[test]
    fn test_register_did() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);
            assert!(DidRegistry::registrations(did_hash).is_some());
            assert_eq!(DidRegistry::total_dids(), 1);
        });
    }

    #[test]
    fn test_register_duplicate_did() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash.clone()
            ));

            assert_err!(
                DidRegistry::register_did(
                    RuntimeOrigin::signed(owner),
                    did,
                    controller,
                    doc_hash
                ),
                Error::<Test>::DidAlreadyExists
            );
        });
    }

    #[test]
    fn test_update_did() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);
            let new_hash = b"newhash456".to_vec();

            assert_ok!(DidRegistry::update_did(
                RuntimeOrigin::signed(owner),
                did_hash,
                new_hash
            ));
        });
    }

    #[test]
    fn test_update_unauthorized() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let other = 3u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);
            let new_hash = b"newhash456".to_vec();

            assert_err!(
                DidRegistry::update_did(
                    RuntimeOrigin::signed(other),
                    did_hash,
                    new_hash
                ),
                Error::<Test>::NotAuthorized
            );
        });
    }

    #[test]
    fn test_revoke_did() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);

            assert_ok!(DidRegistry::revoke_did(
                RuntimeOrigin::signed(owner),
                did_hash
            ));

            let registration = DidRegistry::registrations(did_hash).unwrap();
            assert!(registration.revoked);
        });
    }

    #[test]
    fn test_transfer_ownership() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let new_owner = 3u64;
            let controller = 2u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);

            assert_ok!(DidRegistry::transfer_ownership(
                RuntimeOrigin::signed(owner),
                did_hash,
                new_owner
            ));

            let registration = DidRegistry::registrations(did_hash).unwrap();
            assert_eq!(registration.owner, new_owner);
        });
    }

    #[test]
    fn test_grant_access() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let agent = 3u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);

            assert_ok!(DidRegistry::grant_access(
                RuntimeOrigin::signed(owner),
                did_hash,
                agent,
                AccessLevel::Reader
            ));

            let access_level = DidRegistry::check_access(did_hash, &agent);
            assert_eq!(access_level, AccessLevel::Reader);
        });
    }

    #[test]
    fn test_revoke_access() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let agent = 3u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);

            assert_ok!(DidRegistry::grant_access(
                RuntimeOrigin::signed(owner),
                did_hash,
                agent,
                AccessLevel::Writer
            ));

            assert_ok!(DidRegistry::revoke_access(
                RuntimeOrigin::signed(owner),
                did_hash,
                agent
            ));

            let access_level = DidRegistry::check_access(did_hash, &agent);
            assert_eq!(access_level, AccessLevel::None);
        });
    }

    #[test]
    fn test_is_did_active() {
        new_test_ext().execute_with(|| {
            let owner = 1u64;
            let controller = 2u64;
            let did = b"did:etrid:user1".to_vec();
            let doc_hash = b"hash123".to_vec();

            assert_ok!(DidRegistry::register_did(
                RuntimeOrigin::signed(owner),
                did.clone(),
                controller,
                doc_hash
            ));

            let did_hash = DidRegistry::hash_did(&did);
            assert!(DidRegistry::is_did_active(did_hash));

            // Revoke and check
            assert_ok!(DidRegistry::revoke_did(
                RuntimeOrigin::signed(owner),
                did_hash
            ));

            assert!(!DidRegistry::is_did_active(did_hash));
        });
    }
}
