#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use codec::{Encode, Decode};
    use scale_info::TypeInfo;
    use sp_runtime::{RuntimeDebug, traits::AtLeast32BitUnsigned};
    use sp_std::vec::Vec;

   type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
type HashOf<T> = <T as frame_system::Config>::Hash;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
pub struct Pallet<T>(_);

#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}

// ============================================================
// STORAGE
// ============================================================

#[pallet::storage]
pub type Accounts<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AccountOf<T>,
    AccountInfo<AccountOf<T>, HashOf<T>, BlockNumberOf<T>>,
    OptionQuery,
>;

#[pallet::storage]
pub type DidRegistry<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AccountOf<T>,
    DidDocument<AccountOf<T>, HashOf<T>>,
    OptionQuery,
>;

#[pallet::storage]
pub type AccountCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

// ============================================================
// DATA STRUCTURES
// ============================================================

#[derive(Clone, Encode, Decode, MaxEncodedLen, scale_info::TypeInfo, PartialEq)]
pub struct AccountInfo<AccountId, Hash, BlockNumber> {
    pub nonce: u64,
    pub identity_name: BoundedVec<u8, ConstU32<256>>,
    pub created_at: BlockNumber,
    pub did: Option<Hash>,
}

#[derive(Clone, Encode, Decode, MaxEncodedLen, scale_info::TypeInfo)]
pub struct DidDocument<AccountId, Hash> {
    pub account: AccountId,
    pub public_key: [u8; 32],
    pub root_hash: Hash,
}

// ============================================================
// EVENTS
// ============================================================

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    AccountCreated {
        account: AccountOf<T>,
        identity_name: Vec<u8>,
    },
    DidRegistered {
        account: AccountOf<T>,
        did: HashOf<T>,
    },
    NonceIncremented {
        account: AccountOf<T>,
        new_nonce: u64,
    },
}

// ============================================================
// ERRORS
// ============================================================

#[pallet::error]
pub enum Error<T> {
    AccountAlreadyExists,
    AccountNotFound,
    InvalidIdentityName,
}

// ============================================================
// EXTRINSICS
// ============================================================

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Create new account with DID
    #[pallet::call_index(0)]
    #[pallet::weight(10_000)]
    pub fn create_account(
        origin: OriginFor<T>,
        identity_name: Vec<u8>,
        public_key: [u8; 32],
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Check not already exists
        ensure!(!Accounts::<T>::contains_key(&who), Error::<T>::AccountAlreadyExists);

        // Validate identity name
        ensure!(
            !identity_name.is_empty() && identity_name.len() <= 256,
            Error::<T>::InvalidIdentityName
        );

        let bounded_name = BoundedVec::try_from(identity_name.clone())
            .map_err(|_| Error::<T>::InvalidIdentityName)?;

        let current_block = <frame_system::Pallet<T>>::block_number();

        // Create account info
        let account_info = AccountInfo {
            nonce: 0,
            identity_name: bounded_name,
            created_at: current_block,
            did: None,
        };

        // Create DID document
        let did_doc = DidDocument {
            account: who.clone(),
            public_key,
            root_hash: T::Hashing::hash(&public_key),
        };

        let did = T::Hashing::hash_of(&did_doc);

        // Store
        Accounts::<T>::insert(who.clone(), account_info);
        DidRegistry::<T>::insert(who.clone(), did_doc);

        // Update account with DID
        Accounts::<T>::mutate(&who, |maybe_acc| {
            if let Some(acc) = maybe_acc {
                acc.did = Some(did);
            }
        });

        AccountCounter::<T>::mutate(|count| *count = count.saturating_add(1));

        Self::deposit_event(Event::AccountCreated {
            account: who.clone(),
            identity_name,
        });

        Self::deposit_event(Event::DidRegistered {
            account: who,
            did,
        });

        Ok(())
    }

    /// Increment nonce (called after each transaction)
    #[pallet::call_index(1)]
    #[pallet::weight(1_000)]
    pub fn increment_nonce(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;

        Accounts::<T>::try_mutate(&who, |maybe_acc| {
            match maybe_acc {
                Some(ref mut acc) => {
                    acc.nonce = acc.nonce.saturating_add(1);
                    Self::deposit_event(Event::NonceIncremented {
                        account: who,
                        new_nonce: acc.nonce,
                    });
                    Ok(())
                }
                None => Err(Error::<T>::AccountNotFound),
            }
        })?;

        Ok(())
    }
}

// ============================================================
// STORAGE GETTERS
// ============================================================

impl<T: Config> Pallet<T> {
    pub fn get_nonce(account: &AccountOf<T>) -> u64 {
        Accounts::<T>::get(account)
            .map(|acc| acc.nonce)
            .unwrap_or(0)
    }

    pub fn get_account(
        account: &AccountOf<T>,
    ) -> Option<AccountInfo<AccountOf<T>, HashOf<T>, BlockNumberOf<T>>> {
        Accounts::<T>::get(account)
    }

    pub fn get_did(account: &AccountOf<T>) -> Option<HashOf<T>> {
        Accounts::<T>::get(account).and_then(|acc| acc.did)
    }

    pub fn account_exists(account: &AccountOf<T>) -> bool {
        Accounts::<T>::contains_key(account)
    }

    pub fn total_accounts() -> u64 {
        AccountCounter::<T>::get()
    }
}
}