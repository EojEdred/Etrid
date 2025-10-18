#![cfg_attr(not(feature = "std"), no_std)]

//! # Account Types
//!
//! Defines account types for the Ëtrid multichain system

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_core::crypto::AccountId32;
use sp_std::vec::Vec;
use sp_runtime::traits::Hash;
use core::marker::PhantomData;
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BlockNumberOf<T> = BlockNumberFor<T>;
type HashOf<T> = <T as frame_system::Config>::Hash;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
#[pallet::without_storage_info]
pub struct Pallet<T>(_);

#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}

// ============================================================
// ACCOUNT TYPES (Per Ivory Paper)
// ============================================================

#[derive(Clone, Encode, Decode, scale_info::TypeInfo, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum AccountType {
    /// EBCA - External Blockchain Account (non-Ëtrid key pairs)
    ExternalBlockchainAccount {
        key_type: Vec<u8>, // e.g., "Bitcoin", "Ethereum"
        external_address: Vec<u8>,
    },
    /// RCA - Root Chain Account (FlareChain main account)
    RootChainAccount,
    /// RCWA - Root Chain Withdrawal Account (for withdrawals from RCA)
    RootChainWithdrawalAccount {
        linked_rca: AccountId32,
    },
    /// SCA - Side Chain Account (PBC partition chain account)
    SideChainAccount {
        chain_id: u32, // PBC chain ID (1-12)
    },
    /// SSCA - Smart Side Chain Account (smart contract account)
    SmartSideChainAccount {
        chain_id: u32,
        contract_hash: [u8; 32],
    },
}

// ============================================================
// ACCOUNT STATE (Per Ivory Paper)
// ============================================================

#[derive(Clone, Encode, Decode, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct AccountState<AccountId, Hash, BlockNumber> {
    /// Transaction count (replay protection)
    pub nonce: u64,
    /// ÉTR balance
    pub balance: u128,
    /// Storage root (for smart contracts)
    pub storage_root: Option<Hash>,
    /// Code hash (for smart contracts)
    pub code_hash: Option<Hash>,
    /// Account type
    pub account_type: AccountType,
    /// Created block
    pub created_at: BlockNumber,
    /// Account is active
    pub is_active: bool,
    /// Phantom data for AccountId
    #[codec(skip)]
    pub _phantom: PhantomData<AccountId>,
}

/// Merkle Patricia Tree for account storage
#[derive(Clone, Encode, Decode, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct AccountStorageTree {
    pub root: [u8; 32],
    pub depth: u32,
}

// ============================================================
// STORAGE
// ============================================================

#[pallet::storage]
pub type Accounts<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AccountOf<T>,
    AccountState<AccountOf<T>, HashOf<T>, BlockNumberOf<T>>,
    OptionQuery,
>;

#[pallet::storage]
pub type AccountsByType<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    Vec<u8>, // Account type name
    Vec<AccountOf<T>>,
    ValueQuery,
>;

#[pallet::storage]
pub type StorageTrees<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AccountOf<T>,
    AccountStorageTree,
    OptionQuery,
>;

#[pallet::storage]
pub type AccountCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

// ============================================================
// EVENTS
// ============================================================

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Account created
    AccountCreated {
        account: AccountOf<T>,
        account_type: Vec<u8>,
    },
    /// Account activated
    AccountActivated {
        account: AccountOf<T>,
    },
    /// Account deactivated
    AccountDeactivated {
        account: AccountOf<T>,
    },
    /// Nonce incremented
    NonceIncremented {
        account: AccountOf<T>,
        nonce: u64,
    },
}

// ============================================================
// ERRORS
// ============================================================

#[pallet::error]
pub enum Error<T> {
    AccountAlreadyExists,
    AccountNotFound,
    AccountNotActive,
    InvalidAccountType,
    StorageRootNotFound,
}

// ============================================================
// EXTRINSICS
// ============================================================

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Create Root Chain Account (RCA)
    #[pallet::call_index(0)]
    #[pallet::weight(20_000)]
    pub fn create_root_chain_account(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        ensure!(!Accounts::<T>::contains_key(&who), Error::<T>::AccountAlreadyExists);

        let block_number = <frame_system::Pallet<T>>::block_number();

        let account_state: AccountState<AccountOf<T>, HashOf<T>, BlockNumberOf<T>> = AccountState {
            nonce: 0,
            balance: 0,
            storage_root: None,
            code_hash: None,
            account_type: AccountType::RootChainAccount,
            created_at: block_number,
            is_active: true,
            _phantom: PhantomData,
        };

        Accounts::<T>::insert(who.clone(), account_state);

        let account_type_name = "RCA".as_bytes().to_vec();
        AccountsByType::<T>::mutate(account_type_name.clone(), |accounts| {
            accounts.push(who.clone());
        });

        AccountCounter::<T>::mutate(|count| *count = count.saturating_add(1));

        Self::deposit_event(Event::AccountCreated {
            account: who,
            account_type: account_type_name,
        });

        Ok(())
    }

    /// Create Side Chain Account (SCA)
    #[pallet::call_index(1)]
    #[pallet::weight(20_000)]
    pub fn create_side_chain_account(
        origin: OriginFor<T>,
        chain_id: u32,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        ensure!(!Accounts::<T>::contains_key(&who), Error::<T>::AccountAlreadyExists);
        ensure!(chain_id >= 1 && chain_id <= 12, Error::<T>::InvalidAccountType);

        let block_number = <frame_system::Pallet<T>>::block_number();

        let account_state: AccountState<AccountOf<T>, HashOf<T>, BlockNumberOf<T>> = AccountState {
            nonce: 0,
            balance: 0,
            storage_root: None,
            code_hash: None,
            account_type: AccountType::SideChainAccount { chain_id },
            created_at: block_number,
            is_active: true,
            _phantom: PhantomData,
        };

        Accounts::<T>::insert(who.clone(), account_state);

        let account_type_name = format!("SCA_{}", chain_id).into_bytes();
        AccountsByType::<T>::mutate(account_type_name.clone(), |accounts| {
            accounts.push(who.clone());
        });

        AccountCounter::<T>::mutate(|count| *count = count.saturating_add(1));

        Self::deposit_event(Event::AccountCreated {
            account: who,
            account_type: account_type_name,
        });

        Ok(())
    }

    /// Create Smart Side Chain Account (SSCA)
    #[pallet::call_index(2)]
    #[pallet::weight(50_000)]
    pub fn create_smart_account(
        origin: OriginFor<T>,
        chain_id: u32,
        contract_hash: [u8; 32],
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        ensure!(!Accounts::<T>::contains_key(&who), Error::<T>::AccountAlreadyExists);
        ensure!(chain_id >= 1 && chain_id <= 12, Error::<T>::InvalidAccountType);

        let block_number = <frame_system::Pallet<T>>::block_number();

        let account_state: AccountState<AccountOf<T>, HashOf<T>, BlockNumberOf<T>> = AccountState {
            nonce: 0,
            balance: 0,
            storage_root: Some(T::Hashing::hash(&contract_hash)),
            code_hash: Some(T::Hashing::hash(&contract_hash)),
            account_type: AccountType::SmartSideChainAccount {
                chain_id,
                contract_hash,
            },
            created_at: block_number,
            is_active: true,
            _phantom: PhantomData,
        };

        Accounts::<T>::insert(who.clone(), account_state);

        let account_type_name = format!("SSCA_{}_{}", chain_id, hex::encode(&contract_hash[..8]))
            .into_bytes();
        AccountsByType::<T>::mutate(account_type_name.clone(), |accounts| {
            accounts.push(who.clone());
        });

        AccountCounter::<T>::mutate(|count| *count = count.saturating_add(1));

        Self::deposit_event(Event::AccountCreated {
            account: who,
            account_type: account_type_name,
        });

        Ok(())
    }

    /// Create Withdrawal Account (RCWA - linked to RCA)
    #[pallet::call_index(3)]
    #[pallet::weight(20_000)]
    pub fn create_withdrawal_account(
        origin: OriginFor<T>,
        linked_rca: AccountId32,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        ensure!(!Accounts::<T>::contains_key(&who), Error::<T>::AccountAlreadyExists);

        let block_number = <frame_system::Pallet<T>>::block_number();

        let account_state: AccountState<AccountOf<T>, HashOf<T>, BlockNumberOf<T>> = AccountState {
            nonce: 0,
            balance: 0,
            storage_root: None,
            code_hash: None,
            account_type: AccountType::RootChainWithdrawalAccount {
                linked_rca: linked_rca.clone(),
            },
            created_at: block_number,
            is_active: true,
            _phantom: PhantomData,
        };

        Accounts::<T>::insert(who.clone(), account_state);

        let account_type_name = "RCWA".as_bytes().to_vec();
        AccountsByType::<T>::mutate(account_type_name.clone(), |accounts| {
            accounts.push(who.clone());
        });

        AccountCounter::<T>::mutate(|count| *count = count.saturating_add(1));

        Self::deposit_event(Event::AccountCreated {
            account: who,
            account_type: account_type_name,
        });

        Ok(())
    }

    /// Increment nonce (after each transaction)
    #[pallet::call_index(4)]
    #[pallet::weight(5_000)]
    pub fn increment_nonce(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        Accounts::<T>::try_mutate(&who, |maybe_acc| {
            match maybe_acc {
                Some(ref mut acc) => {
                    ensure!(acc.is_active, Error::<T>::AccountNotActive);
                    acc.nonce = acc.nonce.saturating_add(1);
                    Self::deposit_event(Event::NonceIncremented {
                        account: who.clone(),
                        nonce: acc.nonce,
                    });
                    Ok(())
                }
                None => Err(Error::<T>::AccountNotFound),
            }
        })?;

        Ok(())
    }

    /// Deactivate account
    #[pallet::call_index(5)]
    #[pallet::weight(10_000)]
    pub fn deactivate_account(
        origin: OriginFor<T>,
        account: AccountOf<T>,
    ) -> DispatchResult {
        ensure_root(origin)?;

        Accounts::<T>::try_mutate(&account, |maybe_acc| {
            match maybe_acc {
                Some(ref mut acc) => {
                    acc.is_active = false;
                    Self::deposit_event(Event::AccountDeactivated {
                        account: account.clone(),
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
    pub fn get_account(
        account: &AccountOf<T>,
    ) -> Option<AccountState<AccountOf<T>, HashOf<T>, BlockNumberOf<T>>> {
        Accounts::<T>::get(account)
    }

    pub fn get_nonce(account: &AccountOf<T>) -> u64 {
        Accounts::<T>::get(account)
            .map(|acc| acc.nonce)
            .unwrap_or(0)
    }

    pub fn account_exists(account: &AccountOf<T>) -> bool {
        Accounts::<T>::contains_key(account)
    }

    pub fn is_active(account: &AccountOf<T>) -> bool {
        Accounts::<T>::get(account)
            .map(|acc| acc.is_active)
            .unwrap_or(false)
    }

    pub fn get_account_type(account: &AccountOf<T>) -> Option<AccountType> {
        Accounts::<T>::get(account).map(|acc| acc.account_type)
    }

    pub fn total_accounts() -> u64 {
        AccountCounter::<T>::get()
    }

    pub fn accounts_by_type(type_name: &[u8]) -> Vec<AccountOf<T>> {
        AccountsByType::<T>::get(type_name.to_vec())
    }
}
}