//! EtwasmVM Pallet - WebAssembly Smart Contract Engine for Ëtrid
//!
//! This pallet provides EVM-compatible smart contract execution on Ëtrid
//! using the ETWasm VM runtime.
//!
//! ## Fee Collection & Treasury Routing
//!
//! VMw gas fees are collected from callers and routed to the treasury:
//! - VMw consumed is converted to ÉTR using: `fee = (vmw * op_price) / 1_000_000`
//! - 50% of fees go to Treasury for Consensus Day distribution
//! - 50% of fees are burned (deflationary mechanism)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Re-export ETWasm modules
pub use etwasm_gas_metering as gas;
pub use etwasm_opcodes as opcodes;
pub use etwasm_runtime as runtime;
pub use vmw_runtime as vmw;

use frame_support::dispatch::DispatchResult;

/// Treasury interface for routing VMw fees to the treasury pallet
///
/// This trait connects the ETWasm VM to pallet-treasury for fee collection.
/// Fees collected from contract execution are routed to the treasury pool
/// for distribution during Consensus Day.
pub trait TreasuryInterface<Balance> {
    /// Receive transaction fees from VMw gas consumption
    ///
    /// Called by the ETWasm pallet when gas fees are collected.
    /// The treasury receives 50% of all VMw fees for Consensus Day distribution.
    fn receive_transaction_fees(amount: Balance) -> DispatchResult;
}

/// No-op treasury implementation for testing/development
impl<Balance> TreasuryInterface<Balance> for () {
    fn receive_transaction_fees(_amount: Balance) -> DispatchResult {
        Ok(())
    }
}

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{Time, Currency, ExistenceRequirement, WithdrawReasons},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use codec::Decode;
    use sp_std::prelude::*;
    use sp_std::collections::btree_set::BTreeSet;
    use sp_core::H256;
    use sp_runtime::traits::{SaturatedConversion, Zero, Saturating};

    use etwasm_gas_metering::{VMw, WATTS_PER_ETRID};
    use etwasm_runtime::{
        ExecutionContext, ExecutionResult, Interpreter,
        Storage as StorageBackend, InMemoryStorage
    };
    use vmw_runtime::{VmwMeteringRuntime, MeteringError};

    /// Balance type for fee calculations
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency for fee collection (ÉTR)
        type Currency: Currency<Self::AccountId>;

        /// Treasury interface for routing fees
        type Treasury: TreasuryInterface<BalanceOf<Self>>;

        /// Max size of contract code in bytes
        #[pallet::constant]
        type MaxCodeSize: Get<u32>;

        /// Default gas limit for contract calls
        #[pallet::constant]
        type DefaultGasLimit: Get<VMw>;

        /// Maximum gas limit allowed per call
        #[pallet::constant]
        type MaxGasLimit: Get<VMw>;

        /// VMw operation price (Watts per operation) for metering
        #[pallet::constant]
        type VmwOperationPrice: Get<u32>;

        /// Treasury fee percentage (default 50%)
        #[pallet::constant]
        type TreasuryFeePercent: Get<u32>;
    }

    /// Helper to get current timestamp in milliseconds
    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> u64 {
            pallet_timestamp::Pallet::<T>::now().saturated_into()
        }
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

    /// Storage: Total ÉTR fees collected in current block
    #[pallet::storage]
    #[pallet::getter(fn fees_collected_block)]
    pub type FeesCollectedBlock<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Storage: Total ÉTR fees sent to treasury in current block
    #[pallet::storage]
    #[pallet::getter(fn treasury_fees_block)]
    pub type TreasuryFeesBlock<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Storage: Cumulative fees collected (all time)
    #[pallet::storage]
    #[pallet::getter(fn total_fees_collected)]
    pub type TotalFeesCollected<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Storage: Cumulative fees sent to treasury (all time)
    #[pallet::storage]
    #[pallet::getter(fn total_treasury_fees)]
    pub type TotalTreasuryFees<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

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
        /// VMw fees collected from caller and routed
        /// This event is emitted for every gas charge, enabling fee transparency
        FeeCollected {
            /// Account that paid the fee
            payer: T::AccountId,
            /// VMw gas consumed
            vmw_used: VMw,
            /// Total fee in ÉTR
            total_fee: BalanceOf<T>,
            /// Amount sent to treasury (50%)
            treasury_amount: BalanceOf<T>,
            /// Amount burned (50%)
            burned_amount: BalanceOf<T>,
        },
        /// Block fee summary (emitted at block finalization)
        BlockFeeSummary {
            /// Block number
            block_number: BlockNumberFor<T>,
            /// Total VMw consumed in block
            total_vmw: VMw,
            /// Total fees collected in block
            total_fees: BalanceOf<T>,
            /// Fees sent to treasury
            treasury_fees: BalanceOf<T>,
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
        /// Reentrancy detected during contract execution
        ReentrancyDetected,
        /// Maximum call depth exceeded
        MaxCallDepthExceeded,
        /// Account is locked and cannot execute contracts
        AccountLocked,
        /// Insufficient balance to pay gas fees
        InsufficientBalance,
        /// Fee calculation overflow
        FeeOverflow,
        /// Treasury fee routing failed
        TreasuryRoutingFailed,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Reset gas and fee counters at the start of each block
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            GasUsed::<T>::put(0);
            FeesCollectedBlock::<T>::put(BalanceOf::<T>::zero());
            TreasuryFeesBlock::<T>::put(BalanceOf::<T>::zero());
            Weight::from_parts(3_000, 0)
        }

        /// Emit block fee summary at end of block
        fn on_finalize(n: BlockNumberFor<T>) {
            let total_vmw = GasUsed::<T>::get();
            let total_fees = FeesCollectedBlock::<T>::get();
            let treasury_fees = TreasuryFeesBlock::<T>::get();

            // Only emit if fees were collected
            if !total_fees.is_zero() {
                Self::deposit_event(Event::BlockFeeSummary {
                    block_number: n,
                    total_vmw,
                    total_fees,
                    treasury_fees,
                });
            }
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
                timestamp: Self::current_timestamp(),
                chain_id: 2, // Ëtrid chain ID
                call_stack: BTreeSet::new(),
                reentrancy_depth: 0,
                max_depth: 10, // Max allowed reentrancy depth
            };

            // Create storage backend
            let mut storage = PalletStorage::<T> {
                contract_addr: contract_addr.clone(),
                _phantom: Default::default(),
            };

            // Execute bytecode
            let interpreter = Interpreter::new(context, code.to_vec(), storage);
            let result = interpreter.execute();

            // Handle execution result - collect fees and route to treasury
            match result {
                ExecutionResult::Success { gas_used, return_data } => {
                    // Collect fee from caller and route 50% to treasury
                    Self::charge_gas_with_fee(&caller, gas_used)?;
                    Self::deposit_event(Event::ContractExecuted {
                        contract: contract_addr,
                        gas_used,
                        success: true,
                    });
                    Ok(())
                }
                ExecutionResult::Revert { gas_used, reason } => {
                    // Still collect fees on revert (gas was consumed)
                    Self::charge_gas_with_fee(&caller, gas_used)?;
                    Self::deposit_event(Event::ContractReverted {
                        contract: contract_addr,
                        reason,
                        gas_used,
                    });
                    Err(Error::<T>::ExecutionFailed.into())
                }
                ExecutionResult::OutOfGas { gas_used } => {
                    // Collect fees for gas used before running out
                    Self::charge_gas_with_fee(&caller, gas_used)?;
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
                ExecutionResult::ReentrancyDetected => {
                    Err(Error::<T>::ReentrancyDetected.into())
                }
                ExecutionResult::MaxCallDepthExceeded => {
                    Err(Error::<T>::MaxCallDepthExceeded.into())
                }
                ExecutionResult::AccountLocked => {
                    Err(Error::<T>::AccountLocked.into())
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
                call_stack: BTreeSet::new(),
                reentrancy_depth: 0,
                max_depth: 10, // Max allowed reentrancy depth
            };

            // Use in-memory storage for direct execution
            let storage = InMemoryStorage::default();

            // Execute
            let interpreter = Interpreter::new(context, bytecode, storage);
            let result = interpreter.execute();

            // Handle result - collect fees and route to treasury
            match result {
                ExecutionResult::Success { gas_used, .. } => {
                    Self::charge_gas_with_fee(&caller, gas_used)?;
                    Ok(())
                }
                ExecutionResult::OutOfGas { gas_used } => {
                    Self::charge_gas_with_fee(&caller, gas_used)?;
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

        /// Charge gas for execution - collects fees and routes to treasury
        ///
        /// This is the core fee collection mechanism:
        /// 1. Converts VMw to ÉTR using: fee = (vmw * op_price) / 1,000,000
        /// 2. Withdraws fee from caller's balance
        /// 3. Routes 50% to Treasury for Consensus Day distribution
        /// 4. Burns 50% (deflationary mechanism)
        fn charge_gas_with_fee(payer: &T::AccountId, amount: VMw) -> DispatchResult {
            let current = GasUsed::<T>::get();
            let new_total = current.saturating_add(amount);

            // Check block gas limit
            ensure!(
                new_total <= etwasm_gas_metering::VMW_BLOCK_LIMIT,
                Error::<T>::GasLimitExceeded
            );

            // Calculate fee in ÉTR: (VMw * op_price) / WATTS_PER_ETRID
            let op_price = T::VmwOperationPrice::get();
            let fee_u128 = (amount as u128)
                .saturating_mul(op_price as u128)
                / (WATTS_PER_ETRID as u128);

            // Convert to Balance type
            let total_fee: BalanceOf<T> = fee_u128.saturated_into();

            // Only collect if fee is non-zero
            if !total_fee.is_zero() {
                // Check caller has sufficient balance
                let caller_balance = T::Currency::free_balance(payer);
                ensure!(caller_balance >= total_fee, Error::<T>::InsufficientBalance);

                // Calculate treasury share (default 50%)
                let treasury_percent = T::TreasuryFeePercent::get();
                let treasury_amount = total_fee
                    .saturating_mul(treasury_percent.into())
                    / 100u32.into();
                let burn_amount = total_fee.saturating_sub(treasury_amount);

                // Withdraw total fee from caller (this burns it initially)
                let _imbalance = T::Currency::withdraw(
                    payer,
                    total_fee,
                    WithdrawReasons::FEE,
                    ExistenceRequirement::KeepAlive,
                ).map_err(|_| Error::<T>::InsufficientBalance)?;

                // Route treasury portion to treasury pallet
                if !treasury_amount.is_zero() {
                    T::Treasury::receive_transaction_fees(treasury_amount)
                        .map_err(|_| Error::<T>::TreasuryRoutingFailed)?;
                }

                // Update block fee tracking
                FeesCollectedBlock::<T>::mutate(|f| *f = f.saturating_add(total_fee));
                TreasuryFeesBlock::<T>::mutate(|f| *f = f.saturating_add(treasury_amount));

                // Update cumulative fee tracking
                TotalFeesCollected::<T>::mutate(|f| *f = f.saturating_add(total_fee));
                TotalTreasuryFees::<T>::mutate(|f| *f = f.saturating_add(treasury_amount));

                // Emit fee collection event
                Self::deposit_event(Event::FeeCollected {
                    payer: payer.clone(),
                    vmw_used: amount,
                    total_fee,
                    treasury_amount,
                    burned_amount: burn_amount,
                });
            }

            // Update block VMw usage
            GasUsed::<T>::put(new_total);
            Ok(())
        }

        /// Legacy charge_gas for backwards compatibility (no fee collection)
        /// Use charge_gas_with_fee for production code
        fn charge_gas(amount: VMw) -> DispatchResult {
            let current = GasUsed::<T>::get();
            let new_total = current.saturating_add(amount);

            ensure!(
                new_total <= etwasm_gas_metering::VMW_BLOCK_LIMIT,
                Error::<T>::GasLimitExceeded
            );

            GasUsed::<T>::put(new_total);
            Ok(())
        }

        /// Calculate fee for given VMw amount (read-only)
        pub fn calculate_fee(vmw_amount: VMw) -> BalanceOf<T> {
            let op_price = T::VmwOperationPrice::get();
            let fee_u128 = (vmw_amount as u128)
                .saturating_mul(op_price as u128)
                / (WATTS_PER_ETRID as u128);
            fee_u128.saturated_into()
        }

        /// Create a VMw metering runtime with configured parameters
        /// This provides advanced gas metering with opcode-level cost tracking
        pub fn create_vmw_runtime(gas_limit: VMw) -> VmwMeteringRuntime {
            VmwMeteringRuntime::with_op_price(
                gas_limit,
                T::VmwOperationPrice::get()
            )
        }

        /// Calculate net gas usage accounting for refunds
        /// Returns (gas_consumed, gas_refunded, net_gas)
        pub fn calculate_net_gas(vmw: &VmwMeteringRuntime) -> (VMw, VMw, VMw) {
            let gas_consumed = vmw.gas_consumed;
            let gas_refunded = vmw.gas_refunded;
            let net_gas = gas_consumed.saturating_sub(gas_refunded);
            (gas_consumed, gas_refunded, net_gas)
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
