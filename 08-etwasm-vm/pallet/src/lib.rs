//! EtwasmVM Pallet - WebAssembly Smart Contract Engine for Ëtrid
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use parity_scale_codec::Decode;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Max size of code in bytes
        type MaxCodeSize: Get<u32>;
    }

    /// Storage: Contract code hash by account
    #[pallet::storage]
    #[pallet::getter(fn contract_code_hash)]
    pub type ContractCodeHash<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Hash>;

    /// Storage: Contract owner by account
    #[pallet::storage]
    #[pallet::getter(fn contract_owner)]
    pub type ContractOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId>;

    /// Storage: Contract key-value storage
    #[pallet::storage]
    #[pallet::getter(fn contract_storage)]
    pub type ContractStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<(BoundedVec<u8, ConstU32<64>>, BoundedVec<u8, ConstU32<256>>), ConstU32<128>>,
    >;

    /// Storage: Actual WASM code by code hash
    #[pallet::storage]
    #[pallet::getter(fn code_storage)]
    pub type CodeStorage<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, BoundedVec<u8, ConstU32<1024>>>;

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
        /// Deploy a new smart contract
        #[pallet::weight(100_000)]
        #[pallet::call_index(0)]
        pub fn deploy_contract(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(
                code.len() <= T::MaxCodeSize::get() as usize,
                Error::<T>::CodeTooLarge
            );

            // Generate code hash
            let hash_bytes = sp_io::hashing::blake2_256(&code);
            let code_hash = T::Hash::decode(&mut &hash_bytes[..])
                .map_err(|_| Error::<T>::CodeTooLarge)?;

            // Store the code
            let bounded_code = BoundedVec::<u8, ConstU32<1024>>::try_from(code)
                .map_err(|_| Error::<T>::CodeTooLarge)?;
            CodeStorage::<T>::insert(code_hash, bounded_code);

            // Store contract metadata
            ContractCodeHash::<T>::insert(&sender, code_hash);
            ContractOwner::<T>::insert(&sender, &sender);
            ContractStorage::<T>::insert(&sender, BoundedVec::default());

            Self::deposit_event(Event::ContractDeployed(sender, code_hash));
            Ok(())
        }

        /// Call a deployed contract
        #[pallet::weight(100_000)]
        #[pallet::call_index(1)]
        pub fn call_contract(
            origin: OriginFor<T>,
            contract_addr: T::AccountId,
            function: Vec<u8>,
            _args: Vec<u8>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Verify contract exists
            ensure!(
                ContractCodeHash::<T>::contains_key(&contract_addr),
                Error::<T>::ContractNotFound
            );

            // Emulated call (WASM runtime TBD)
            Self::deposit_event(Event::ContractCalled(caller, function));
            Ok(())
        }

        /// Get contract storage value
        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn get_storage(
            origin: OriginFor<T>,
            contract_addr: T::AccountId,
            key: Vec<u8>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            let storage = ContractStorage::<T>::get(&contract_addr)
                .ok_or(Error::<T>::ContractNotFound)?;

            ensure!(
                storage.iter().any(|(k, _)| k.as_slice() == key.as_slice()),
                Error::<T>::StorageKeyNotFound
            );
            Ok(())
        }

        /// Execute contract with gas limit
        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn execute_with_gas(
            origin: OriginFor<T>,
            _call_data: Vec<u8>,
            _gas_limit: u64,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            // Placeholder for VM gas system
            Ok(())
        }
    }
}
