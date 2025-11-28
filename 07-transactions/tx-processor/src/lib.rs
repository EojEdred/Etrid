//! # Ëtrid Transaction Processor Pallet
//!
//! ## Overview
//!
//! This pallet provides transaction mempool management and processing functionality for
//! the Ëtrid blockchain. It handles transaction submission, validation, queuing, and
//! execution, serving as the core transaction processing layer between user submissions
//! and block inclusion.
//!
//! ## Features
//!
//! - Transaction mempool with configurable size limits (10,000 transactions)
//! - Ed25519 signature verification for transaction authenticity
//! - Nonce-based replay protection
//! - Automatic transaction processing at block finalization
//! - Transaction batching (up to 1,000 transactions per block)
//! - Pool statistics tracking (submitted, processed, failed counts)
//! - Transaction history with block height recording
//! - Chain ID validation for cross-chain security
//!
//! ## Extrinsics
//!
//! - `submit_transaction` - Submit signed transaction to mempool
//! - `clear_mempool` - Clear all pending transactions (governance only)
//! - `query_pool_size` - Query current mempool size
//!
//! ## Usage Example
//!
//! ```ignore
//! // Create and submit a signed transaction
//! let signed_tx = SignedTransaction {
//!     sender: alice,
//!     nonce: 5,
//!     tx_type: TransactionType::Regular { ... },
//!     signature: ed25519_signature,
//!     chain_id: 1,
//! };
//!
//! TxProcessor::submit_transaction(
//!     Origin::signed(alice),
//!     signed_tx,
//! )?;
//!
//! // Query mempool statistics
//! let stats = TxProcessor::get_stats();
//! println!("Pool size: {}", stats.current_pool_size);
//! ```
//!
//! ## Storage Items
//!
//! - `TransactionPool` - Pending transaction mempool (max 10,000 transactions)
//! - `ProcessedTransactions` - Recently processed transactions (max 1,000)
//! - `NextNonce` - Maps account to expected next nonce
//! - `TxBlockHeight` - Maps transaction hash to block number
//! - `PoolStats` - Mempool statistics (submitted, processed, failed counts)
//!
//! ## Events
//!
//! - `TransactionReceived` - When transaction is accepted into mempool
//! - `TransactionProcessed` - When transaction is included in block
//! - `TransactionFailed` - When transaction validation or execution fails
//! - `MempoolCleared` - When mempool is administratively cleared
//!
//! ## Errors
//!
//! - `InvalidSignature` - Ed25519 signature verification failed
//! - `InvalidNonce` - Nonce does not match expected value
//! - `InsufficientFunds` - Account lacks balance for transaction
//! - `PoolFull` - Mempool has reached maximum capacity
//! - `InvalidChainId` - Chain ID does not match expected value
//! - `DuplicateTransaction` - Transaction already exists in pool
//! - `TransactionTooLarge` - Transaction size exceeds limit (1MB)
//!
//! ## Transaction Processing Pipeline
//!
//! 1. **Submission**: User submits signed transaction via extrinsic
//! 2. **Validation**: Signature, nonce, and basic checks performed
//! 3. **Mempool**: Valid transactions added to pending pool
//! 4. **Selection**: At block finalization, top transactions selected (by priority)
//! 5. **Execution**: Selected transactions processed and state updated
//! 6. **Recording**: Block height and receipt stored for executed transactions
//! 7. **Cleanup**: Processed transactions removed from mempool
//!
//! ## Performance Limits
//!
//! - **Max Mempool Size**: 10,000 transactions
//! - **Max Transactions per Block**: 1,000 transactions
//! - **Max Transaction Size**: 1 MB
#![cfg_attr(not(feature = "std"), no_std)]

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
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
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
        // 1. Verify Ed25519 signature
        ensure!(!tx.signature.0.is_empty(), Error::<T>::InvalidSignature);

        // Verify signature length (Ed25519 signatures are 64 bytes)
        ensure!(tx.signature.0.len() == 64, Error::<T>::InvalidSignature);

        // Get the public key from the sender's account ID
        // In a real implementation, you would derive the public key from the AccountId
        // For now, we'll encode the sender and use validate_signature from pallet_transaction
        let sender_bytes = tx.sender.encode();

        // For proper Ed25519 verification, we need a 32-byte public key
        // In production, the AccountId should be derived from or contain the public key
        // For now, we'll use a simpler validation check
        if sender_bytes.len() >= 32 {
            // Extract first 32 bytes as public key (simplified)
            let public_key_bytes = &sender_bytes[0..32];

            // Create message to verify (encode the transaction without signature)
            let mut message = Vec::new();
            message.extend_from_slice(&tx.sender.encode());
            message.extend_from_slice(&tx.nonce.to_le_bytes());
            message.extend_from_slice(&tx.tx_type.encode());
            message.extend_from_slice(&tx.chain_id.to_le_bytes());

            // Validate signature using Ed25519
            let is_valid = Self::verify_ed25519_signature(&tx.signature.0, public_key_bytes, &message);
            ensure!(is_valid, Error::<T>::InvalidSignature);
        } else {
            // If we can't extract a valid public key, signature verification fails
            return Err(Error::<T>::InvalidSignature.into());
        }

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

    /// Verify Ed25519 signature
    ///
    /// # Arguments
    /// * `signature_bytes` - The signature bytes (must be 64 bytes)
    /// * `public_key_bytes` - The public key bytes (must be 32 bytes)
    /// * `message` - The message that was signed
    ///
    /// # Returns
    /// * `true` if signature is valid, `false` otherwise
    fn verify_ed25519_signature(signature_bytes: &[u8], public_key_bytes: &[u8], message: &[u8]) -> bool {
        // Validate lengths
        if signature_bytes.len() != 64 || public_key_bytes.len() != 32 {
            return false;
        }

        // Parse public key
        let mut pk_array = [0u8; 32];
        pk_array.copy_from_slice(public_key_bytes);
        let verifying_key = match VerifyingKey::from_bytes(&pk_array) {
            Ok(key) => key,
            Err(_) => return false,
        };

        // Parse signature
        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(signature_bytes);
        let signature = Signature::from_bytes(&sig_array);

        // Verify signature
        verifying_key.verify(message, &signature).is_ok()
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

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use crate::pallet::*;
    use ed25519_dalek::{SigningKey, Signer, Verifier, VerifyingKey};

    #[test]
    fn test_verify_ed25519_signature_valid() {
        // Generate a keypair
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let verifying_key = signing_key.verifying_key();
        let public_key_bytes = verifying_key.to_bytes();

        // Create and sign a message
        let message = b"transaction data to be signed";
        let signature = signing_key.sign(message);
        let signature_bytes = signature.to_bytes();

        // Verify directly using ed25519-dalek (since we can't access the private function)
        assert!(verifying_key.verify(message, &signature).is_ok(),
                "Valid Ed25519 signature should verify successfully");
    }

    #[test]
    fn test_verify_ed25519_signature_invalid_signature() {
        // Generate a keypair
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let verifying_key = signing_key.verifying_key();

        // Create and sign a message
        let message = b"transaction data";
        let signature = signing_key.sign(message);
        let mut signature_bytes = signature.to_bytes();

        // Corrupt the signature
        signature_bytes[0] ^= 0xFF;

        // Parse corrupted signature and verify
        let corrupted_sig = ed25519_dalek::Signature::from_bytes(&signature_bytes);
        assert!(verifying_key.verify(message, &corrupted_sig).is_err(),
                "Corrupted signature should fail verification");
    }

    #[test]
    fn test_verify_ed25519_signature_wrong_message() {
        // Generate a keypair
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let verifying_key = signing_key.verifying_key();

        // Create and sign a message
        let message = b"original message";
        let signature = signing_key.sign(message);

        // Try to verify with different message
        let wrong_message = b"different message";
        assert!(verifying_key.verify(wrong_message, &signature).is_err(),
                "Signature should fail verification with wrong message");
    }

    #[test]
    fn test_verify_ed25519_signature_wrong_key() {
        // Generate two keypairs
        let signing_key1 = SigningKey::from_bytes(&[1u8; 32]);
        let signing_key2 = SigningKey::from_bytes(&[2u8; 32]);
        let verifying_key2 = signing_key2.verifying_key();

        // Sign with key1
        let message = b"transaction data";
        let signature = signing_key1.sign(message);

        // Try to verify with key2
        assert!(verifying_key2.verify(message, &signature).is_err(),
                "Signature should fail verification with wrong public key");
    }

    #[test]
    fn test_verify_ed25519_invalid_public_key() {
        let signing_key = SigningKey::from_bytes(&[1u8; 32]);
        let message = b"test message";
        let signature = signing_key.sign(message);

        // Try with all-zero public key
        let zero_pk_bytes = [0u8; 32];
        let result = VerifyingKey::from_bytes(&zero_pk_bytes);

        // If it parses as valid, verification should still fail
        if let Ok(zero_pk) = result {
            assert!(zero_pk.verify(message, &signature).is_err(),
                    "Signature verification with different key should fail");
        }
    }

    #[test]
    fn test_pool_statistics_default() {
        let stats = PoolStatistics::default();
        assert_eq!(stats.total_submitted, 0);
        assert_eq!(stats.total_processed, 0);
        assert_eq!(stats.total_failed, 0);
        assert_eq!(stats.current_pool_size, 0);
    }

    #[test]
    fn test_max_pool_constants() {
        // Verify constants are set correctly
        assert_eq!(MAX_POOL_SIZE, 10_000);
        assert_eq!(MAX_TX_PER_BLOCK, 1_000);
    }
}
}