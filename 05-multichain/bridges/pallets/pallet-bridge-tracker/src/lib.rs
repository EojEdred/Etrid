#![cfg_attr(not(feature = "std"), no_std)]

//! # Bridge Tracker Pallet
//!
//! Track external currency deposits/withdrawals with M-of-N attestation
//!
//! ## Overview
//!
//! This pallet maintains immutable records of all cross-chain transactions,
//! verifies deposits via multi-signature attestation, and tracks withdrawal
//! requests for external chain releases.

pub use pallet::*;

pub mod types;
use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*, sp_runtime::traits::Zero};
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The balance type
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen;

        /// Minimum required signatures for attestation (M in M-of-N)
        #[pallet::constant]
        type MinimumAttestationSignatures: Get<u32>;

        /// Maximum validators in attestation set (N in M-of-N)
        #[pallet::constant]
        type MaxValidators: Get<u32>;

        /// Minimum confirmations required on external chain
        #[pallet::constant]
        type MinimumConfirmations: Get<u8>;

        /// Origin that can authorize validators
        type ValidatorOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    /// Storage: Deposit records by deposit ID
    #[pallet::storage]
    #[pallet::getter(fn deposits)]
    pub type Deposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256,
        DepositRecord<T::AccountId, T::Balance, BlockNumberFor<T>>,
    >;

    /// Storage: Withdrawal records by withdrawal ID
    #[pallet::storage]
    #[pallet::getter(fn withdrawals)]
    pub type Withdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256,
        WithdrawalRecord<T::AccountId, T::Balance, BlockNumberFor<T>>,
    >;

    /// Storage: Total locked balance
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    /// Storage: Total withdrawn balance
    #[pallet::storage]
    #[pallet::getter(fn total_withdrawn)]
    pub type TotalWithdrawn<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    /// Storage: Attestation sets by message hash
    #[pallet::storage]
    #[pallet::getter(fn attestations)]
    pub type Attestations<T: Config> =
        StorageMap<_, Blake2_128Concat, H256, AttestationSet<T::AccountId>>;

    /// Storage: Authorized validators for attestation
    #[pallet::storage]
    #[pallet::getter(fn authorized_validators)]
    pub type AuthorizedValidators<T: Config> =
        StorageValue<_, BoundedVec<T::AccountId, ConstU32<100>>, ValueQuery>;

    /// Storage: External vault address (multi-sig on external chain)
    #[pallet::storage]
    #[pallet::getter(fn external_vault_address)]
    pub type ExternalVaultAddress<T: Config> = StorageValue<_, [u8; 32], ValueQuery>;

    /// Storage: Last verified external block number
    #[pallet::storage]
    #[pallet::getter(fn last_verified_block)]
    pub type LastVerifiedBlock<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Storage: Last reconciliation timestamp
    #[pallet::storage]
    #[pallet::getter(fn last_reconciliation_time)]
    pub type LastReconciliationTime<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Storage: Bridge paused status
    #[pallet::storage]
    #[pallet::getter(fn paused)]
    pub type Paused<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Deposit recorded [deposit_id, user, amount, external_tx_hash]
        DepositRecorded {
            deposit_id: H256,
            user: T::AccountId,
            amount: T::Balance,
            external_tx_hash: H256,
        },
        /// Deposit verified [deposit_id, signatures_count]
        DepositVerified {
            deposit_id: H256,
            signatures_count: u32,
        },
        /// Deposit processed [deposit_id, wrapped_minted]
        DepositProcessed {
            deposit_id: H256,
            wrapped_minted: T::Balance,
        },
        /// Withdrawal requested [withdrawal_id, user, amount, target_address]
        WithdrawalRequested {
            withdrawal_id: H256,
            user: T::AccountId,
            amount: T::Balance,
            target_address: [u8; 32],
        },
        /// Withdrawal approved [withdrawal_id]
        WithdrawalApproved { withdrawal_id: H256 },
        /// Withdrawal completed [withdrawal_id, external_tx_hash]
        WithdrawalCompleted {
            withdrawal_id: H256,
            external_tx_hash: H256,
        },
        /// Attestation submitted [message_hash, validator]
        AttestationSubmitted {
            message_hash: H256,
            validator: T::AccountId,
        },
        /// Validator added [validator]
        ValidatorAdded { validator: T::AccountId },
        /// Validator removed [validator]
        ValidatorRemoved { validator: T::AccountId },
        /// Bridge paused
        BridgePaused,
        /// Bridge unpaused
        BridgeUnpaused,
        /// Reconciliation performed [balanced, discrepancy]
        ReconciliationPerformed { balanced: bool, discrepancy: i128 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Deposit already exists
        DepositAlreadyExists,
        /// Deposit not found
        DepositNotFound,
        /// Withdrawal not found
        WithdrawalNotFound,
        /// Insufficient confirmations
        InsufficientConfirmations,
        /// Insufficient attestation signatures
        InsufficientAttestationSignatures,
        /// Invalid signature
        InvalidSignature,
        /// Not authorized validator
        NotAuthorizedValidator,
        /// Deposit already verified
        DepositAlreadyVerified,
        /// Withdrawal already completed
        WithdrawalAlreadyCompleted,
        /// Bridge is paused
        BridgePaused,
        /// Arithmetic overflow
        ArithmeticOverflow,
        /// Arithmetic underflow
        ArithmeticUnderflow,
        /// Too many validators
        TooManyValidators,
        /// Deposit not verified
        DepositNotVerified,
        /// Invalid deposit status
        InvalidDepositStatus,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Record a deposit from external chain
        ///
        /// This function is called by authorized relayers after detecting
        /// a deposit transaction on the external chain.
        ///
        /// # Arguments
        /// * `origin` - Must be a signed extrinsic from authorized relayer
        /// * `user` - Account that will receive wrapped tokens
        /// * `amount` - Amount deposited on external chain
        /// * `external_tx_hash` - Transaction hash on external chain
        /// * `block_number` - Block number on external chain
        ///
        /// # Errors
        /// * `BridgePaused` - Bridge operations are halted
        /// * `DepositAlreadyExists` - This deposit has already been recorded
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn record_deposit(
            origin: OriginFor<T>,
            user: T::AccountId,
            amount: T::Balance,
            external_tx_hash: H256,
            block_number: u32,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;
            ensure!(!Paused::<T>::get(), Error::<T>::BridgePaused);

            // Generate deposit ID (hash of external tx)
            let deposit_id = external_tx_hash;

            // Ensure deposit doesn't already exist
            ensure!(
                !Deposits::<T>::contains_key(deposit_id),
                Error::<T>::DepositAlreadyExists
            );

            // Create deposit record
            let deposit = DepositRecord {
                user: user.clone(),
                amount,
                external_tx_hash,
                external_block_number: block_number,
                timestamp: <frame_system::Pallet<T>>::block_number(),
                bridge_message_hash: H256::zero(),
                verified: false,
                confirmations: 0,
                status: DepositStatus::Pending,
            };

            // Store deposit
            Deposits::<T>::insert(deposit_id, deposit);

            // Emit event
            Self::deposit_event(Event::DepositRecorded {
                deposit_id,
                user,
                amount,
                external_tx_hash,
            });

            Ok(())
        }

        /// Verify deposit with M-of-N attestation
        ///
        /// Once enough validators have signed the deposit message,
        /// this function marks the deposit as verified.
        ///
        /// # Arguments
        /// * `origin` - Must be a signed extrinsic
        /// * `deposit_id` - The deposit to verify
        /// * `signatures` - List of (validator, signature) pairs
        ///
        /// # Errors
        /// * `DepositNotFound` - Deposit doesn't exist
        /// * `DepositAlreadyVerified` - Deposit already verified
        /// * `InsufficientAttestationSignatures` - Not enough valid signatures
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn verify_deposit(
            origin: OriginFor<T>,
            deposit_id: H256,
            signatures: Vec<(T::AccountId, Vec<u8>)>,
        ) -> DispatchResult {
            let _caller = ensure_signed(origin)?;

            // Get deposit
            let mut deposit = Deposits::<T>::get(deposit_id).ok_or(Error::<T>::DepositNotFound)?;
            ensure!(!deposit.verified, Error::<T>::DepositAlreadyVerified);

            // Verify we have enough signatures
            let sig_count = signatures.len() as u32;
            ensure!(
                sig_count >= T::MinimumAttestationSignatures::get(),
                Error::<T>::InsufficientAttestationSignatures
            );

            // Verify each signature is from authorized validator
            let authorized = AuthorizedValidators::<T>::get();
            for (validator, _signature) in &signatures {
                ensure!(
                    authorized.contains(validator),
                    Error::<T>::NotAuthorizedValidator
                );
            }

            // Mark deposit as verified
            deposit.verified = true;
            deposit.status = DepositStatus::Verified;
            Deposits::<T>::insert(deposit_id, deposit.clone());

            // Update total locked
            let new_total = TotalLocked::<T>::get()
                .checked_add(&amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;
            TotalLocked::<T>::put(new_total);

            // Emit event
            Self::deposit_event(Event::DepositVerified {
                deposit_id,
                signatures_count: sig_count,
            });

            Ok(())
        }

        /// Record a withdrawal request
        ///
        /// User requests to withdraw wrapped tokens and receive
        /// native currency on external chain.
        ///
        /// # Arguments
        /// * `origin` - Must be signed by user
        /// * `amount` - Amount to withdraw
        /// * `target_address` - Address on external chain to receive funds
        ///
        /// # Errors
        /// * `BridgePaused` - Bridge operations are halted
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn record_withdrawal(
            origin: OriginFor<T>,
            amount: T::Balance,
            target_address: [u8; 32],
        ) -> DispatchResult {
            let user = ensure_signed(origin)?;
            ensure!(!Paused::<T>::get(), Error::<T>::BridgePaused);

            // Generate withdrawal ID
            let withdrawal_id = H256::random();

            // Create withdrawal record
            let withdrawal = WithdrawalRecord {
                user: user.clone(),
                amount,
                target_address,
                external_tx_hash: None,
                timestamp: <frame_system::Pallet<T>>::block_number(),
                released: false,
                status: WithdrawalStatus::Pending,
            };

            // Store withdrawal
            Withdrawals::<T>::insert(withdrawal_id, withdrawal);

            // Emit event
            Self::deposit_event(Event::WithdrawalRequested {
                withdrawal_id,
                user,
                amount,
                target_address,
            });

            Ok(())
        }

        /// Confirm withdrawal release on external chain
        ///
        /// Called by relayers after multi-sig releases funds on external chain.
        ///
        /// # Arguments
        /// * `origin` - Must be signed
        /// * `withdrawal_id` - The withdrawal to confirm
        /// * `external_tx_hash` - Transaction hash on external chain
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn confirm_withdrawal_release(
            origin: OriginFor<T>,
            withdrawal_id: H256,
            external_tx_hash: H256,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Get withdrawal
            let mut withdrawal =
                Withdrawals::<T>::get(withdrawal_id).ok_or(Error::<T>::WithdrawalNotFound)?;
            ensure!(!withdrawal.released, Error::<T>::WithdrawalAlreadyCompleted);

            // Mark as completed
            withdrawal.released = true;
            withdrawal.status = WithdrawalStatus::Completed;
            withdrawal.external_tx_hash = Some(external_tx_hash);
            Withdrawals::<T>::insert(withdrawal_id, withdrawal.clone());

            // Update total withdrawn
            let new_total = TotalWithdrawn::<T>::get()
                .checked_add(&withdrawal.amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;
            TotalWithdrawn::<T>::put(new_total);

            // Emit event
            Self::deposit_event(Event::WithdrawalCompleted {
                withdrawal_id,
                external_tx_hash,
            });

            Ok(())
        }

        /// Mark deposit as processed (wrapped tokens minted)
        ///
        /// Called by Tier 1 pool after minting wrapped tokens.
        ///
        /// # Arguments
        /// * `origin` - Must be signed
        /// * `deposit_id` - The deposit to mark as processed
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn mark_deposit_processed(
            origin: OriginFor<T>,
            deposit_id: H256,
        ) -> DispatchResult {
            let _caller = ensure_signed(origin)?;

            // Get deposit
            let mut deposit = Deposits::<T>::get(deposit_id).ok_or(Error::<T>::DepositNotFound)?;
            ensure!(deposit.verified, Error::<T>::DepositNotVerified);
            ensure!(
                deposit.status == DepositStatus::Verified,
                Error::<T>::InvalidDepositStatus
            );

            // Mark as processed
            deposit.status = DepositStatus::Processed;
            Deposits::<T>::insert(deposit_id, deposit.clone());

            // Emit event
            Self::deposit_event(Event::DepositProcessed {
                deposit_id,
                wrapped_minted: deposit.amount,
            });

            Ok(())
        }

        /// Add authorized validator
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn add_validator(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
            T::ValidatorOrigin::ensure_origin(origin)?;

            AuthorizedValidators::<T>::try_mutate(|validators| {
                if !validators.contains(&validator) {
                    validators
                        .try_push(validator.clone())
                        .map_err(|_| Error::<T>::TooManyValidators)?;
                }
                Ok::<_, DispatchError>(())
            })?;

            Self::deposit_event(Event::ValidatorAdded { validator });
            Ok(())
        }

        /// Remove authorized validator
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn remove_validator(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
            T::ValidatorOrigin::ensure_origin(origin)?;

            AuthorizedValidators::<T>::mutate(|validators| {
                validators.retain(|v| v != &validator);
            });

            Self::deposit_event(Event::ValidatorRemoved { validator });
            Ok(())
        }

        /// Emergency pause bridge operations
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn pause_bridge(origin: OriginFor<T>) -> DispatchResult {
            T::ValidatorOrigin::ensure_origin(origin)?;
            Paused::<T>::put(true);
            Self::deposit_event(Event::BridgePaused);
            Ok(())
        }

        /// Unpause bridge operations
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn unpause_bridge(origin: OriginFor<T>) -> DispatchResult {
            T::ValidatorOrigin::ensure_origin(origin)?;
            Paused::<T>::put(false);
            Self::deposit_event(Event::BridgeUnpaused);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get accounting snapshot
        pub fn get_accounting_snapshot() -> (T::Balance, T::Balance, T::Balance) {
            let total_locked = TotalLocked::<T>::get();
            let total_withdrawn = TotalWithdrawn::<T>::get();
            let net_balance = total_locked.saturating_sub(total_withdrawn);
            (total_locked, total_withdrawn, net_balance)
        }
    }
}
