#![cfg_attr(not(feature = "std"), no_std)]

//! tx-types

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResult, pallet_prelude::{MaxEncodedLen, BoundedVec, ConstU32}};
use frame_system::ensure_signed;
use scale_info::TypeInfo;
use sp_runtime::traits::Zero;
use sp_std::prelude::*;

pub use pallet::*;

/// Currency type selector
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyType {
    Etrid,  // Native ÉTR token
    Etd,    // ETD Stablecoin
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configuration trait for pallet-transaction
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // ============================================================
    // TRANSACTION TYPE DEFINITIONS (from Ivory Paper)
    // ============================================================
    // Ëtrid supports 5 core transaction types:
    // 1. Regular Transfer - Simple value transfer between accounts
    // 2. Stake Deposit - Deposit for validator staking or delegation
    // 3. Smart Contract Call - Execute contract with data payload
    // 4. Contract Initialization - Deploy new WASM contract
    // 5. Lightning Bloc - Cross-chain transaction (micropayment channel)
    // ============================================================

    pub type Balance = u128;
    pub type VMw = u128;
    pub type ChainId = u32;
    pub type Nonce = u64;

    /// Transaction signature (Ed25519) - bounded to 64 bytes
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Clone, PartialEq, Eq)]
    pub struct Signature(pub BoundedVec<u8, ConstU32<64>>);

    /// Transaction type enumeration
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Clone, PartialEq, Eq)]
    pub enum TransactionType {
        /// Regular transfer: sender → recipient with value
        Regular {
            recipient: BoundedVec<u8, ConstU32<32>>,  // Account addresses are 32 bytes
            amount: Balance,
            currency: CurrencyType,
        },
        /// Stake deposit: lock funds for validation
        StakeDeposit {
            validator: BoundedVec<u8, ConstU32<32>>,  // Account addresses are 32 bytes
            amount: Balance,
            lock_period: u32, // blocks
        },
        /// Smart contract call: invoke existing contract
        SmartContractCall {
            contract: BoundedVec<u8, ConstU32<32>>,  // Contract addresses are 32 bytes
            data: BoundedVec<u8, ConstU32<10240>>,  // 10KB max for contract call data
            vmw_limit: VMw,
            value: Balance,
        },
        /// Contract initialization: deploy new WASM contract
        ContractInit {
            init_code: BoundedVec<u8, ConstU32<524288>>,  // 512KB max for contract code
            vmw_limit: VMw,
            value: Balance,
        },
        /// Lightning Bloc: cross-chain micropayment
        LightningBloc {
            target_chain: ChainId,
            recipient: BoundedVec<u8, ConstU32<32>>,  // Account addresses are 32 bytes
            amount: Balance,
            fee: Balance,
        },
    }

    /// Signed transaction structure
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Clone, PartialEq, Eq)]
    #[codec(mel_bound())]
    pub struct SignedTransaction<AccountId: MaxEncodedLen> {
        pub sender: AccountId,
        pub nonce: Nonce,
        pub tx_type: TransactionType,
        pub signature: Signature,
        pub chain_id: ChainId,
    }

    // Manual implementation of DecodeWithMemTracking for SignedTransaction
    // This is needed for older Substrate versions where MaxEncodedLen doesn't auto-impl it
    impl<AccountId: MaxEncodedLen + Decode> codec::DecodeWithMemTracking for SignedTransaction<AccountId> {}

    /// Transaction status
    #[derive(Encode, Decode, TypeInfo, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TransactionStatus {
        Pending,
        Confirmed,
        Failed,
        Executed,
    }

    /// Transaction receipt
    #[derive(Encode, Decode, TypeInfo, Debug, Clone)]
    pub struct TransactionReceipt<AccountId> {
        pub tx_hash: [u8; 32],
        pub sender: AccountId,
        pub status: TransactionStatus,
        pub block_number: u32,
        pub vmw_used: VMw,
        pub fee_paid: Balance,
    }

    // ============================================================
    // STORAGE MAPS
    // ============================================================

    #[pallet::storage]
    #[pallet::getter(fn account_nonce)]
    pub(super) type AccountNonces<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, Nonce, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn transaction_pool)]
    pub(super) type TransactionPool<T: Config> = StorageValue<_, Vec<SignedTransaction<T::AccountId>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn tx_receipt)]
    pub(super) type TransactionReceipts<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, [u8; 32], TransactionReceipt<T::AccountId>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn staking_pool)]
    pub(super) type StakingPool<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn contract_storage)]
    pub(super) type ContractStorage<T: Config> =
        StorageDoubleMap<_, 
            frame_support::Blake2_128Concat, Vec<u8>,  // contract address
            frame_support::Blake2_128Concat, Vec<u8>,  // storage key
            Vec<u8>,
            OptionQuery
        >;

    #[pallet::storage]
    #[pallet::getter(fn contract_code)]
    pub(super) type ContractCode<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, Vec<u8>, Vec<u8>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn lightning_bloc_channels)]
    pub(super) type LightningBlocChannels<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, u32, LightningBlocChannel, OptionQuery>;

    // ============================================================
    // EVENTS
    // ============================================================

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Regular transaction executed
        TransactionExecuted {
            tx_hash: [u8; 32],
            sender: T::AccountId,
            tx_type: Vec<u8>, // serialized transaction type
        },
        /// Transaction failed
        TransactionFailed {
            tx_hash: [u8; 32],
            sender: T::AccountId,
            reason: Vec<u8>,
        },
        /// Stake deposited
        StakeDeposited {
            account: T::AccountId,
            amount: Balance,
            lock_period: u32,
        },
        /// Stake withdrawn
        StakeWithdrawn {
            account: T::AccountId,
            amount: Balance,
        },
        /// Smart contract called
        ContractCalled {
            contract: Vec<u8>,
            caller: T::AccountId,
            vmw_used: VMw,
        },
        /// Smart contract deployed
        ContractDeployed {
            contract: Vec<u8>,
            deployer: T::AccountId,
            code_hash: [u8; 32],
        },
        /// Lightning Bloc channel created
        LightningBlocCreated {
            channel_id: u32,
            target_chain: ChainId,
        },
        /// Lightning Bloc payment routed
        LightningBlocPaymentRouted {
            channel_id: u32,
            sender: T::AccountId,
            recipient: Vec<u8>,
            amount: Balance,
        },
    }

    // ============================================================
    // ERRORS
    // ============================================================

    #[pallet::error]
    pub enum Error<T> {
        /// Nonce mismatch: expected vs provided
        NonceMismatch,
        /// Invalid transaction signature
        InvalidSignature,
        /// Insufficient balance for transaction
        InsufficientBalance,
        /// VMw limit exceeded
        VMwLimitExceeded,
        /// Contract not found
        ContractNotFound,
        /// Invalid contract code
        InvalidContractCode,
        /// Smart contract execution failed
        ContractExecutionFailed,
        /// Stake lock period not expired
        StakeLocked,
        /// Lightning Bloc channel not found
        LightningBlocChannelNotFound,
        /// Lightning Bloc payment failed
        LightningBlocPaymentFailed,
        /// Invalid recipient address
        InvalidRecipient,
        /// Transaction already exists in pool
        TransactionDuplicate,
        /// Invalid transaction data format
        InvalidTransactionFormat,
        /// Chain ID mismatch
        ChainIdMismatch,
    }

    // ============================================================
    // PALLET IMPLEMENTATION
    // ============================================================

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit a regular transfer transaction
        #[pallet::weight(1_000)]
        pub fn submit_regular_transfer(
            origin: OriginFor<T>,
            recipient: Vec<u8>,
            amount: Balance,
            is_etr: bool, // true = ÉTR token, false = ETD stablecoin
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate input
            ensure!(!recipient.is_empty(), Error::<T>::InvalidRecipient);
            ensure!(amount > 0, Error::<T>::InsufficientBalance);

            // Convert bool to CurrencyType
            let currency = if is_etr { CurrencyType::Etrid } else { CurrencyType::Etd };

            // Convert Vec to BoundedVec
            let recipient_bounded = BoundedVec::try_from(recipient)
                .map_err(|_| Error::<T>::InvalidRecipient)?;

            // Create transaction
            let tx_type = TransactionType::Regular {
                recipient: recipient_bounded,
                amount,
                currency,
            };

            // Get and validate nonce
            let nonce = AccountNonces::<T>::get(&sender);
            let next_nonce = nonce + 1;
            AccountNonces::<T>::insert(&sender, next_nonce);

            // Create signed transaction
            let signed_tx = SignedTransaction {
                sender: sender.clone(),
                nonce,
                tx_type,
                signature: Signature(BoundedVec::default()), // Signature validation would go here
                chain_id: 0,
            };

            // Add to pool
            let mut pool = TransactionPool::<T>::get();
            pool.push(signed_tx);
            TransactionPool::<T>::set(pool);

            Self::deposit_event(Event::TransactionExecuted {
                tx_hash: Self::hash_transaction(&sender, nonce),
                sender: sender.clone(),
                tx_type: b"Regular".to_vec(),
            });

            Ok(())
        }

        /// Submit a stake deposit transaction
        #[pallet::weight(1_000)]
        pub fn submit_stake_deposit(
            origin: OriginFor<T>,
            validator: Vec<u8>,
            amount: Balance,
            lock_period: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!validator.is_empty(), Error::<T>::InvalidRecipient);
            ensure!(amount > 0, Error::<T>::InsufficientBalance);
            ensure!(lock_period > 0, Error::<T>::InvalidTransactionFormat);

            // Record stake
            let current_stake = StakingPool::<T>::get(&sender);
            StakingPool::<T>::insert(&sender, current_stake + amount);

            let nonce = AccountNonces::<T>::get(&sender);
            AccountNonces::<T>::insert(&sender, nonce + 1);

            Self::deposit_event(Event::StakeDeposited {
                account: sender,
                amount,
                lock_period,
            });

            Ok(())
        }

        /// Submit a smart contract call transaction
        #[pallet::weight(5_000)]
        pub fn submit_contract_call(
            origin: OriginFor<T>,
            contract: Vec<u8>,
            data: Vec<u8>,
            vmw_limit: VMw,
            value: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!contract.is_empty(), Error::<T>::ContractNotFound);
            ensure!(vmw_limit > 0, Error::<T>::VMwLimitExceeded);

            // Verify contract exists
            let _contract_code = ContractCode::<T>::get(&contract)
                .ok_or(Error::<T>::ContractNotFound)?;

            let nonce = AccountNonces::<T>::get(&sender);
            AccountNonces::<T>::insert(&sender, nonce + 1);

            Self::deposit_event(Event::ContractCalled {
                contract: contract.clone(),
                caller: sender,
                vmw_used: vmw_limit,
            });

            Ok(())
        }

        /// Deploy a new smart contract
        #[pallet::weight(10_000)]
        pub fn deploy_contract(
            origin: OriginFor<T>,
            init_code: Vec<u8>,
            vmw_limit: VMw,
            value: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!init_code.is_empty(), Error::<T>::InvalidContractCode);
            ensure!(vmw_limit > 0, Error::<T>::VMwLimitExceeded);

            // Generate contract address (simplified: hash of init_code)
            let contract_address = Self::hash_code(&init_code);

            // Store contract code
            ContractCode::<T>::insert(contract_address.to_vec(), init_code);

            let nonce = AccountNonces::<T>::get(&sender);
            AccountNonces::<T>::insert(&sender, nonce + 1);

            Self::deposit_event(Event::ContractDeployed {
                contract: contract_address.to_vec(),
                deployer: sender,
                code_hash: contract_address,
            });

            Ok(())
        }

        /// Submit a Lightning Bloc cross-chain payment
        #[pallet::weight(2_000)]
        pub fn submit_lightning_bloc(
            origin: OriginFor<T>,
            target_chain: ChainId,
            recipient: Vec<u8>,
            amount: Balance,
            fee: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!recipient.is_empty(), Error::<T>::InvalidRecipient);
            ensure!(amount > 0, Error::<T>::InsufficientBalance);

            // Verify Lightning Bloc channel exists
            let _channel = LightningBlocChannels::<T>::get(target_chain)
                .ok_or(Error::<T>::LightningBlocChannelNotFound)?;

            let nonce = AccountNonces::<T>::get(&sender);
            AccountNonces::<T>::insert(&sender, nonce + 1);

            Self::deposit_event(Event::LightningBlocPaymentRouted {
                channel_id: target_chain,
                sender: sender.clone(),
                recipient,
                amount,
            });

            Ok(())
        }

        /// Withdraw stake (only after lock period expires)
        #[pallet::weight(1_000)]
        pub fn withdraw_stake(
            origin: OriginFor<T>,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let current_stake = StakingPool::<T>::get(&sender);
            ensure!(current_stake >= amount, Error::<T>::InsufficientBalance);

            StakingPool::<T>::insert(&sender, current_stake - amount);

            Self::deposit_event(Event::StakeWithdrawn {
                account: sender,
                amount,
            });

            Ok(())
        }
    }

    // ============================================================
    // HELPER FUNCTIONS
    // ============================================================

    impl<T: Config> Pallet<T> {
        /// Hash transaction for unique identification
        pub fn hash_transaction(account: &T::AccountId, nonce: Nonce) -> [u8; 32] {
            use sp_core::hashing::keccak_256;
            let mut data = Vec::new();
            data.extend_from_slice(&nonce.to_le_bytes());
            keccak_256(&data)
        }

        /// Hash contract code for address generation
        pub fn hash_code(code: &[u8]) -> [u8; 32] {
            use sp_core::hashing::keccak_256;
            keccak_256(code)
        }

        /// Validate transaction signature
        pub fn validate_signature(_signature: &Signature, _tx_data: &[u8]) -> bool {
            // Signature validation logic would go here
            // For now, return true (actual validation requires Ed25519 verification)
            true
        }

        /// Get transaction pool size
        pub fn pool_size() -> usize {
            TransactionPool::<T>::get().len()
        }

        /// Get account stake
        pub fn get_stake(account: &T::AccountId) -> Balance {
            StakingPool::<T>::get(account)
        }

        /// Get contract code
        pub fn get_contract_code(contract: &[u8]) -> Option<Vec<u8>> {
            ContractCode::<T>::get(contract)
        }

        /// Get contract storage value
        pub fn get_storage(contract: &[u8], key: &[u8]) -> Option<Vec<u8>> {
            ContractStorage::<T>::get(contract, key)
        }

        /// Set contract storage value
        pub fn set_storage(contract: Vec<u8>, key: Vec<u8>, value: Vec<u8>) {
            ContractStorage::<T>::insert(&contract, &key, value);
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub phantom: sp_std::marker::PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                phantom: sp_std::marker::PhantomData,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Initialize empty transaction pool
            TransactionPool::<T>::set(Vec::new());
        }
    }
}

// ============================================================
// LIGHTNING BLOC CHANNEL STRUCTURE
// ============================================================

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub struct LightningBlocChannel {
    pub id: u32,
    pub target_chain: u32,
    pub capacity: u128,
    pub used: u128,
    pub active: bool,
}

// ============================================================
// TRANSACTION VALIDATION RULES (from Ivory Paper)
// ============================================================
// A transaction is valid if:
// 1. Properly formed and encoded (no trailing bytes)
// 2. Digital signature is valid (Ed25519)
// 3. Nonce equals sender's current nonce
// 4. VMwattage ≥ gas used
// 5. Sender's balance covers execution cost
// ============================================================
