#![cfg_attr(not(feature = "std"), no_std)]

//! Transaction Processor Pallet
//!
//! This pallet provides transaction mempool and processing functionality for FlareChain.

extern crate alloc;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use pallet_transaction::SignedTransaction;
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use alloc::vec::Vec;
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BlockNumberOf<T> = BlockNumberFor<T>;
type SignedTx<T> = SignedTransaction<AccountOf<T>>;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
const MAX_POOL_SIZE: usize = 10_000;
const MAX_TX_PER_BLOCK: usize = 1_000;

#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
#[pallet::without_storage_info]
pub struct Pallet<T>(_);

#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}

// ============================================================
// STORAGE
// ============================================================

/// Transaction mempool (pending transactions)
#[pallet::storage]
pub type TransactionPool<T: Config> = StorageValue<
    _,
    BoundedVec<SignedTx<T>, ConstU32<10000>>,
    ValueQuery,
>;

/// Processed transactions (included in blocks)
#[pallet::storage]
pub type ProcessedTransactions<T: Config> = StorageValue<
    _,
    BoundedVec<SignedTx<T>, ConstU32<1000>>,
    ValueQuery,
>;

/// Next nonce for each account (for replay protection)
#[pallet::storage]
pub type NextNonce<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AccountOf<T>,
    u64,
    ValueQuery,
>;

/// Block height when transaction was processed
#[pallet::storage]
pub type TxBlockHeight<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    [u8; 32], // tx hash
    BlockNumberOf<T>,
    OptionQuery,
>;

/// Transaction count stats
#[pallet::storage]
pub type PoolStats<T: Config> = StorageValue<
    _,
    PoolStatistics,
    ValueQuery,
>;

// ============================================================
// DATA STRUCTURES
// ============================================================

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq, Eq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct PoolStatistics {
    pub total_submitted: u64,
    pub total_processed: u64,
    pub total_failed: u64,
    pub current_pool_size: u32,
}

impl Default for PoolStatistics {
    fn default() -> Self {
        PoolStatistics {
            total_submitted: 0,
            total_processed: 0,
            total_failed: 0,
            current_pool_size: 0,
        }
    }
}

// ============================================================
// EVENTS
// ============================================================

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Transaction received
    TransactionReceived {
        from: AccountOf<T>,
        nonce: u64,
    },
    /// Transaction processed successfully
    TransactionProcessed {
        from: AccountOf<T>,
        at_block: BlockNumberOf<T>,
    },
    /// Transaction failed
    TransactionFailed {
        from: AccountOf<T>,
        nonce: u64,
    },
    /// Mempool cleared
    MempoolCleared {
        tx_count: u32,
    },
}

// ============================================================
// ERRORS
// ============================================================

#[pallet::error]
pub enum Error<T> {
    InvalidSignature,
    InvalidNonce,
    InsufficientFunds,
    PoolFull,
    InvalidChainId,
    InvalidTimestamp,
    DuplicateTransaction,
    TransactionTooLarge,
}

// ============================================================
// HOOKS - Process transactions at end of block
// ============================================================

#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_finalize(block_number: BlockNumberFor<T>) {
        // Process pending transactions (simplified for v1)
        // In production: select by gas price, execute, update state
        Self::process_pending_transactions(block_number);
    }
}

// ============================================================
// EXTRINSICS
// ============================================================

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Submit transaction to mempool
    #[pallet::call_index(0)]
    #[pallet::weight(20_000)]
    pub fn submit_transaction(
        origin: OriginFor<T>,
        tx: SignedTx<T>,
    ) -> DispatchResult {
        let _who = ensure_signed(origin)?;

        // Validate transaction
        Self::validate_transaction(&tx)?;

        let mut pool = Self::get_mempool();

        // Check pool size
        ensure!(pool.len() < MAX_POOL_SIZE, Error::<T>::PoolFull);

        // Add to pool
        pool.try_push(tx.clone())
            .map_err(|_| Error::<T>::PoolFull)?;

        TransactionPool::<T>::put(pool.clone());

        // Update stats
        PoolStats::<T>::mutate(|stats| {
            stats.total_submitted = stats.total_submitted.saturating_add(1);
            stats.current_pool_size = pool.len() as u32;
        });

        Self::deposit_event(Event::TransactionReceived {
            from: tx.sender.clone(),
            nonce: tx.nonce,
        });

        Ok(())
    }

    /// Manually clear mempool (admin only)
    #[pallet::call_index(1)]
    #[pallet::weight(100_000)]
    pub fn clear_mempool(origin: OriginFor<T>) -> DispatchResult {
        ensure_root(origin)?;

        let pool = Self::get_mempool();
        let size = pool.len() as u32;

        TransactionPool::<T>::put(BoundedVec::new());

        PoolStats::<T>::mutate(|stats| {
            stats.current_pool_size = 0;
        });

        Self::deposit_event(Event::MempoolCleared { tx_count: size });

        Ok(())
    }

    /// Query current mempool size
    #[pallet::call_index(2)]
    #[pallet::weight(5_000)]
    pub fn query_pool_size(origin: OriginFor<T>) -> DispatchResult {
        let _who = ensure_signed(origin)?;
        // Return via event or storage getter
        Ok(())
    }
}

// ============================================================
// VALIDATION
// ============================================================

impl<T: Config> Pallet<T> {
    fn validate_transaction(tx: &SignedTx<T>) -> DispatchResult {
        // 1. Verify signature (simplified - full Ed25519 verification needed)
        // For now: just check signature is not empty
        ensure!(!tx.signature.0.is_empty(), Error::<T>::InvalidSignature);

        // 2. Check nonce matches expected next nonce
        let expected_nonce = NextNonce::<T>::get(&tx.sender);
        ensure!(tx.nonce == expected_nonce, Error::<T>::InvalidNonce);

        // 3. Verify chain ID (simplified - just check it's set)
        ensure!(tx.chain_id > 0, Error::<T>::InvalidChainId);

        // 4. Check transaction size
        let encoded_size = tx.encode().len();
        ensure!(encoded_size < 1_000_000, Error::<T>::TransactionTooLarge);

        Ok(())
    }
}

// ============================================================
// TRANSACTION PROCESSING
// ============================================================

impl<T: Config> Pallet<T> {
    fn process_pending_transactions(block_number: BlockNumberOf<T>) {
        let pool = Self::get_mempool();

        if pool.is_empty() {
            return;
        }

        // Select top MAX_TX_PER_BLOCK transactions
        let tx_count = pool.len().min(MAX_TX_PER_BLOCK);

        for i in 0..tx_count {
            if let Some(tx) = pool.get(i) {
                // Update nonce
                NextNonce::<T>::mutate(&tx.sender, |nonce| {
                    *nonce = nonce.saturating_add(1);
                });

                // Record block height (using nonce as simple hash)
                let tx_hash = Self::compute_tx_hash(tx);
                TxBlockHeight::<T>::insert(tx_hash, block_number);

                // Update stats
                PoolStats::<T>::mutate(|stats| {
                    stats.total_processed = stats.total_processed.saturating_add(1);
                });

                Self::deposit_event(Event::TransactionProcessed {
                    from: tx.sender.clone(),
                    at_block: block_number,
                });
            }
        }

        // Remove processed transactions from pool
        let remaining: BoundedVec<SignedTx<T>, ConstU32<10000>> =
            BoundedVec::truncate_from(
                pool.iter()
                    .skip(tx_count)
                    .cloned()
                    .collect::<Vec<_>>()
            );

        TransactionPool::<T>::put(remaining.clone());

        PoolStats::<T>::mutate(|stats| {
            stats.current_pool_size = remaining.len() as u32;
        });
    }

    fn compute_tx_hash(tx: &SignedTx<T>) -> [u8; 32] {
        // Simple hash based on nonce (in production use proper hashing)
        let mut hash = [0u8; 32];
        let nonce_bytes = tx.nonce.to_le_bytes();
        hash[0..8].copy_from_slice(&nonce_bytes);
        hash
    }
}

// ============================================================
// STORAGE GETTERS
// ============================================================

impl<T: Config> Pallet<T> {
    pub fn get_mempool() -> BoundedVec<SignedTx<T>, ConstU32<10000>> {
        TransactionPool::<T>::get()
    }

    pub fn get_pool_size() -> usize {
        TransactionPool::<T>::get().len()
    }

    pub fn get_stats() -> PoolStatistics {
        PoolStats::<T>::get()
    }

    pub fn get_next_nonce(account: &AccountOf<T>) -> u64 {
        NextNonce::<T>::get(account)
    }

    pub fn get_tx_block_height(tx_hash: &[u8; 32]) -> Option<BlockNumberOf<T>> {
        TxBlockHeight::<T>::get(tx_hash)
    }
}
}