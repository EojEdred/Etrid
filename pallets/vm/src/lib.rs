//! EtwasmVM Pallet - WebAssembly Smart Contract Engine for Ëtrid
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;
    use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Contract<AccountId, Hash> {
        pub address: AccountId,
        pub code_hash: Hash,
        pub storage: BTreeMap<Vec<u8>, Vec<u8>>,
        pub owner: AccountId,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type MaxCodeSize: Get<u32>;
    }

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::storage]
    #[pallet::getter(fn contracts)]
    pub(super) type Contracts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Contract<T::AccountId, T::Hash>>;

    #[pallet::storage]
    #[pallet::getter(fn code_storage)]
    pub(super) type CodeStorage<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<u8>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ContractDeployed(T::AccountId, T::Hash),
        ContractCalled(T::AccountId, Vec<u8>),
        StorageUpdated(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        CodeTooLarge,
        ContractNotFound,
        NotContractOwner,
        ExecutionFailed,
        StorageKeyNotFound,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100_000)]
        pub fn deploy_contract(
            origin: OriginFor<T>,
            code: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(code.len() <= T::MaxCodeSize::get() as usize, Error::<T>::CodeTooLarge);

            let code_hash = T::Hashing::hash(&code);
            CodeStorage::<T>::insert(code_hash, code);

            let contract = Contract::<T::AccountId, T::Hash> {
                address: sender.clone(),
                code_hash,
                storage: BTreeMap::new(),
                owner: sender.clone(),
            };

            Contracts::<T>::insert(&sender, contract);
            Self::deposit_event(Event::ContractDeployed(sender, code_hash));
            Ok(())
        }

        #[pallet::weight(100_000)]
        pub fn call_contract(
            origin: OriginFor<T>,
            contract_addr: T::AccountId,
            function: Vec<u8>,
            args: Vec<u8>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Contracts::<T>::get(&contract_addr).ok_or(Error::<T>::ContractNotFound)?;

            // Stub: Emulate execution for now
            // TODO: integrate real WASM VM (e.g. wasmi)
            Self::deposit_event(Event::ContractCalled(caller, function));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn get_storage(
            origin: OriginFor<T>,
            contract_addr: T::AccountId,
            key: Vec<u8>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            let contract = Contracts::<T>::get(&contract_addr).ok_or(Error::<T>::ContractNotFound)?;
            ensure!(contract.storage.contains_key(&key), Error::<T>::StorageKeyNotFound);
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn execute_with_gas(
            origin: OriginFor<T>,
            _call_data: Vec<u8>,
            _gas_limit: u64,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            // TODO: WASM execution metered by VMw
            Ok(())
        }
    }
}
