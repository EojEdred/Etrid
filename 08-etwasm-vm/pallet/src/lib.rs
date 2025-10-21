//! EtwasmVM Pallet - WebAssembly Smart Contract Engine for Ëtrid
//!
//! This pallet provides EVM-compatible smart contract execution on Ëtrid
//! using the ETWasm VM runtime.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Re-export ETWasm modules
pub use etwasm_gas_metering as gas;
pub use etwasm_opcodes as opcodes;
pub use etwasm_runtime as runtime;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use codec::Decode;
    use sp_std::prelude::*;
    use sp_core::H256;
    use sp_runtime::traits::SaturatedConversion;

    use etwasm_gas_metering::VMw;
    use etwasm_runtime::{
        ExecutionContext, ExecutionResult, Interpreter,
        Storage as StorageBackend, InMemoryStorage
    };

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Max size of contract code in bytes
        #[pallet::constant]
        type MaxCodeSize: Get<u32>;

        /// Default gas limit for contract calls
        #[pallet::constant]
        type DefaultGasLimit: Get<VMw>;

        /// Maximum gas limit allowed per call
        #[pallet::constant]
        type MaxGasLimit: Get<VMw>;
    }

    /// Storage: Contract code hash by account
    #[pallet::storage]
    #[pallet::getter(fn contract_code_hash)]
    pub type ContractCodeHash<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        T::Hash
    >;

    /// Storage: Contract owner by account
    #[pallet::storage]
    #[pallet::getter(fn contract_owner)]
    pub type ContractOwner<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        T::AccountId
    >;

    /// Storage: Contract persistent storage (key-value pairs)
    /// Maps (contract_addr, storage_key) => storage_value
    #[pallet::storage]
    #[pallet::getter(fn contract_storage_value)]
    pub type ContractStorageValue<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, T::AccountId,  // contract address
        Blake2_128Concat, H256,           // storage key
        H256,                              // storage value
    >;

    /// Storage: Actual bytecode by code hash
    #[pallet::storage]
    #[pallet::getter(fn code_storage)]
    pub type CodeStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BoundedVec<u8, ConstU32<1048576>> // 1MB max
    >;

    /// Storage: Gas used by contract in current block
    #[pallet::storage]
    #[pallet::getter(fn gas_used)]
    pub type GasUsed<T: Config> = StorageValue<_, VMw, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Contract deployed successfully
        ContractDeployed {
            deployer: T::AccountId,
            contract_address: T::AccountId,
            code_hash: T::Hash
        },
        /// Contract called successfully
        ContractCalled {
            caller: T::AccountId,
            contract: T::AccountId,
            gas_used: VMw,
        },
        /// Contract execution completed
        ContractExecuted {
            contract: T::AccountId,
            gas_used: VMw,
            success: bool,
        },
        /// Contract reverted
        ContractReverted {
            contract: T::AccountId,
            reason: Vec<u8>,
            gas_used: VMw,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Contract code exceeds maximum size
        CodeTooLarge,
        /// Contract not found at specified address
        ContractNotFound,
        /// Caller is not the contract owner
        NotContractOwner,
        /// Contract execution failed
        ExecutionFailed,
        /// Storage key not found
        StorageKeyNotFound,
        /// Gas limit exceeded
        GasLimitExceeded,
        /// Out of gas during execution
        OutOfGas,
        /// Invalid bytecode
        InvalidBytecode,
        /// Stack overflow/underflow
        StackError,
        /// Invalid opcode
        InvalidOpcode,
        /// Invalid jump destination
        InvalidJump,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Reset gas counter at the start of each block
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            GasUsed::<T>::put(0);
            Weight::from_parts(1_000, 0)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deploy a new smart contract
        ///
        /// Stores the bytecode and initializes contract storage.
        /// The sender becomes the contract owner.
        #[pallet::weight(100_000)]
        #[pallet::call_index(0)]
        pub fn deploy_contract(
            origin: OriginFor<T>,
            code: Vec<u8>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate code size
            ensure!(
                (code.len() as u32) <= T::MaxCodeSize::get(),
                Error::<T>::CodeTooLarge
            );

            // Generate code hash
            let hash_bytes = sp_io::hashing::blake2_256(&code);
            let code_hash = T::Hash::decode(&mut &hash_bytes[..])
                .map_err(|_| Error::<T>::InvalidBytecode)?;

            // Store the code
            let bounded_code = BoundedVec::<u8, ConstU32<1048576>>::try_from(code.clone())
                .map_err(|_| Error::<T>::CodeTooLarge)?;
            CodeStorage::<T>::insert(code_hash, bounded_code);

            // Use sender as contract address (simplified)
            let contract_address = sender.clone();

            // Store contract metadata
            ContractCodeHash::<T>::insert(&contract_address, code_hash);
            ContractOwner::<T>::insert(&contract_address, &sender);

            Self::deposit_event(Event::ContractDeployed {
                deployer: sender,
                contract_address,
                code_hash,
            });

            Ok(())
        }

        /// Call a deployed contract
        ///
        /// Executes the contract bytecode with the ETWasm interpreter.
        #[pallet::weight(100_000)]
        #[pallet::call_index(1)]
        pub fn call_contract(
            origin: OriginFor<T>,
            contract_addr: T::AccountId,
            input_data: Vec<u8>,
            gas_limit: Option<VMw>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Verify contract exists
            let code_hash = ContractCodeHash::<T>::get(&contract_addr)
                .ok_or(Error::<T>::ContractNotFound)?;

            // Load bytecode
            let code = CodeStorage::<T>::get(code_hash)
                .ok_or(Error::<T>::ContractNotFound)?;

            // Validate gas limit
            let gas_limit = gas_limit.unwrap_or_else(T::DefaultGasLimit::get);
            ensure!(
                gas_limit <= T::MaxGasLimit::get(),
                Error::<T>::GasLimitExceeded
            );

            // Create execution context
            let context = ExecutionContext {
                caller: Self::account_to_bytes32(&caller),
                address: Self::account_to_bytes32(&contract_addr),
                value: 0, // No value transfer for now
                gas_limit,
                gas_price: 1,
                block_number: frame_system::Pallet::<T>::block_number().saturated_into(),
                timestamp: 0, // TODO: Get actual timestamp
                chain_id: 2, // Ëtrid chain ID
            };

            // Create storage backend
            let mut storage = PalletStorage::<T> {
                contract_addr: contract_addr.clone(),
                _phantom: Default::default(),
            };

            // Execute bytecode
            let interpreter = Interpreter::new(context, code.to_vec(), storage);
            let result = interpreter.execute();

            // Handle execution result
            match result {
                ExecutionResult::Success { gas_used, return_data } => {
                    Self::charge_gas(gas_used)?;
                    Self::deposit_event(Event::ContractExecuted {
                        contract: contract_addr,
                        gas_used,
                        success: true,
                    });
                    Ok(())
                }
                ExecutionResult::Revert { gas_used, reason } => {
                    Self::charge_gas(gas_used)?;
                    Self::deposit_event(Event::ContractReverted {
                        contract: contract_addr,
                        reason,
                        gas_used,
                    });
                    Err(Error::<T>::ExecutionFailed.into())
                }
                ExecutionResult::OutOfGas { gas_used } => {
                    Self::charge_gas(gas_used)?;
                    Err(Error::<T>::OutOfGas.into())
                }
                ExecutionResult::StackError => {
                    Err(Error::<T>::StackError.into())
                }
                ExecutionResult::InvalidOpcode(_) => {
                    Err(Error::<T>::InvalidOpcode.into())
                }
                ExecutionResult::InvalidJump => {
                    Err(Error::<T>::InvalidJump.into())
                }
                ExecutionResult::Error(_) => {
                    Err(Error::<T>::ExecutionFailed.into())
                }
            }
        }

        /// Execute contract bytecode directly with gas limit
        ///
        /// Useful for testing and direct contract execution.
        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn execute_bytecode(
            origin: OriginFor<T>,
            bytecode: Vec<u8>,
            gas_limit: VMw,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Validate gas limit
            ensure!(
                gas_limit <= T::MaxGasLimit::get(),
                Error::<T>::GasLimitExceeded
            );

            // Create execution context
            let context = ExecutionContext {
                caller: Self::account_to_bytes32(&caller),
                address: [0u8; 32], // No specific contract address
                value: 0,
                gas_limit,
                gas_price: 1,
                block_number: frame_system::Pallet::<T>::block_number().saturated_into(),
                timestamp: 0,
                chain_id: 2,
            };

            // Use in-memory storage for direct execution
            let storage = InMemoryStorage::default();

            // Execute
            let interpreter = Interpreter::new(context, bytecode, storage);
            let result = interpreter.execute();

            // Handle result
            match result {
                ExecutionResult::Success { gas_used, .. } => {
                    Self::charge_gas(gas_used)?;
                    Ok(())
                }
                ExecutionResult::OutOfGas { gas_used } => {
                    Self::charge_gas(gas_used)?;
                    Err(Error::<T>::OutOfGas.into())
                }
                _ => Err(Error::<T>::ExecutionFailed.into()),
            }
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Convert AccountId to 32-byte array for EVM compatibility
        fn account_to_bytes32(account: &T::AccountId) -> [u8; 32] {
            let encoded = account.encode();
            let mut result = [0u8; 32];
            let len = core::cmp::min(encoded.len(), 32);
            result[32 - len..].copy_from_slice(&encoded[..len]);
            result
        }

        /// Charge gas for execution
        fn charge_gas(amount: VMw) -> DispatchResult {
            let current = GasUsed::<T>::get();
            let new_total = current.saturating_add(amount);

            // Check block gas limit
            ensure!(
                new_total <= etwasm_gas_metering::VMW_BLOCK_LIMIT,
                Error::<T>::GasLimitExceeded
            );

            GasUsed::<T>::put(new_total);
            Ok(())
        }
    }

    /// Storage backend implementation for pallet storage
    pub struct PalletStorage<T: Config> {
        contract_addr: T::AccountId,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> StorageBackend for PalletStorage<T> {
        fn read(&self, key: &H256) -> Option<H256> {
            ContractStorageValue::<T>::get(&self.contract_addr, key)
        }

        fn write(&mut self, key: H256, value: H256) {
            ContractStorageValue::<T>::insert(&self.contract_addr, key, value);
        }
    }
}
