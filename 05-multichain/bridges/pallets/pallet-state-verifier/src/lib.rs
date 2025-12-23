#![cfg_attr(not(feature = "std"), no_std)]

//! # State Verifier Pallet
//!
//! Verify external chain state matches internal records
//!
//! ## Overview
//!
//! This pallet performs reconciliation between internal accounting
//! and external chain reality, verifies Merkle proofs of external
//! state, and handles balance discrepancies.

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Reconciliation report
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ReconciliationReport<Balance> {
    /// Report timestamp
    pub timestamp: u64,
    /// Internal balance (what we think is locked)
    pub internal_balance: Balance,
    /// External balance (what blockchain shows)
    pub external_balance: Balance,
    /// Difference (should be 0)
    pub discrepancy: i128,
    /// Total wrapped tokens minted
    pub wrapped_supply: Balance,
    /// true if all matches
    pub balanced: bool,
    /// Status
    pub status: ReconciliationStatus,
}

/// Reconciliation status
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ReconciliationStatus {
    /// All balances match
    Healthy,
    /// Minor discrepancy
    Warning,
    /// Major mismatch
    Critical,
}

/// Balance dispute
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct BalanceDispute<AccountId, Balance> {
    /// Challenger
    pub challenger: AccountId,
    /// Reported balance
    pub reported_balance: Balance,
    /// Actual balance (from proof)
    pub actual_balance: Balance,
    /// Timestamp
    pub timestamp: u64,
    /// Proof data
    pub proof: Vec<u8>,
    /// Dispute resolved
    pub resolved: bool,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::sp_runtime::traits::Zero;
    use frame_system::pallet_prelude::*;
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

        /// Tolerance threshold for discrepancies (in basis points)
        #[pallet::constant]
        type ToleranceThreshold: Get<u32>;

        /// Reconciliation interval (in blocks)
        #[pallet::constant]
        type ReconciliationInterval: Get<BlockNumberFor<Self>>;

        /// Origin that can perform reconciliation
        type ReconciliationOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    /// Storage: Last reconciliation block
    #[pallet::storage]
    #[pallet::getter(fn last_reconciliation_block)]
    pub type LastReconciliationBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    /// Storage: External vault balance (cached from relayers)
    #[pallet::storage]
    #[pallet::getter(fn external_vault_balance)]
    pub type ExternalVaultBalance<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    /// Storage: Wrapped token total supply (cached)
    #[pallet::storage]
    #[pallet::getter(fn wrapped_total_supply)]
    pub type WrappedTotalSupply<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    /// Storage: Balance disputes
    #[pallet::storage]
    #[pallet::getter(fn disputes)]
    pub type Disputes<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, BalanceDispute<T::AccountId, T::Balance>>;

    /// Storage: Dispute counter
    #[pallet::storage]
    #[pallet::getter(fn dispute_counter)]
    pub type DisputeCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Storage: Circuit breaker (halts operations on critical mismatch)
    #[pallet::storage]
    #[pallet::getter(fn circuit_breaker_triggered)]
    pub type CircuitBreakerTriggered<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Reconciliation performed [timestamp, balanced, discrepancy]
        ReconciliationPerformed {
            timestamp: u64,
            balanced: bool,
            discrepancy: i128,
        },
        /// Balance discrepancy detected [discrepancy, severity]
        BalanceDiscrepancy {
            discrepancy: i128,
            severity: ReconciliationStatus,
        },
        /// External balance verified [vault_address, balance]
        ExternalBalanceVerified {
            vault_address: [u8; 32],
            balance: T::Balance,
        },
        /// Merkle proof verified [root, leaf]
        MerkleProofVerified { root: [u8; 32], leaf: [u8; 32] },
        /// Dispute submitted [dispute_id, challenger]
        DisputeSubmitted {
            dispute_id: u64,
            challenger: T::AccountId,
        },
        /// Dispute resolved [dispute_id, valid]
        DisputeResolved { dispute_id: u64, valid: bool },
        /// Circuit breaker triggered
        CircuitBreakerTriggered,
        /// Circuit breaker reset
        CircuitBreakerReset,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Arithmetic overflow
        ArithmeticOverflow,
        /// Arithmetic underflow
        ArithmeticUnderflow,
        /// Invalid Merkle proof
        InvalidMerkleProof,
        /// Circuit breaker is active
        CircuitBreakerActive,
        /// Dispute not found
        DisputeNotFound,
        /// Dispute already resolved
        DisputeAlreadyResolved,
        /// Too soon to reconcile
        TooSoonToReconcile,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Verify external vault balance
        ///
        /// Relayers submit the current balance of the external vault
        /// along with a cryptographic proof.
        ///
        /// # Arguments
        /// * `origin` - Must be signed
        /// * `vault_address` - External vault address
        /// * `claimed_balance` - Balance claimed to exist
        /// * `proof` - Merkle proof or other cryptographic proof
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn verify_external_balance(
            origin: OriginFor<T>,
            vault_address: [u8; 32],
            claimed_balance: T::Balance,
            proof: Vec<u8>,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // In production, verify the proof here
            // For now, we trust the relayer (should be replaced with actual verification)
            let _verified = Self::verify_proof(&proof)?;

            // Update cached external balance
            ExternalVaultBalance::<T>::put(claimed_balance);

            // Emit event
            Self::deposit_event(Event::ExternalBalanceVerified {
                vault_address,
                balance: claimed_balance,
            });

            Ok(())
        }

        /// Perform reconciliation
        ///
        /// Checks if internal records match external reality.
        /// Auto-triggers circuit breaker if critical mismatch detected.
        ///
        /// # Arguments
        /// * `origin` - Must be authorized
        /// * `internal_balance` - Current internal locked balance
        /// * `wrapped_supply` - Current wrapped token supply
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn reconcile(
            origin: OriginFor<T>,
            internal_balance: T::Balance,
            wrapped_supply: T::Balance,
        ) -> DispatchResult {
            T::ReconciliationOrigin::ensure_origin(origin)?;

            // Check if enough time has passed since last reconciliation
            let current_block = <frame_system::Pallet<T>>::block_number();
            let last_block = LastReconciliationBlock::<T>::get();
            ensure!(
                current_block >= last_block + T::ReconciliationInterval::get(),
                Error::<T>::TooSoonToReconcile
            );

            // Get external balance
            let external_balance = ExternalVaultBalance::<T>::get();

            // Calculate discrepancy
            let internal_u128: u128 = internal_balance.try_into().ok().unwrap_or(0);
            let external_u128: u128 = external_balance.try_into().ok().unwrap_or(0);
            let discrepancy = (external_u128 as i128) - (internal_u128 as i128);

            // Determine status
            let abs_discrepancy = discrepancy.abs() as u128;
            let threshold = (internal_u128 * T::ToleranceThreshold::get() as u128) / 10000;

            let (balanced, status) = if discrepancy == 0 {
                (true, ReconciliationStatus::Healthy)
            } else if abs_discrepancy <= threshold {
                (false, ReconciliationStatus::Warning)
            } else {
                (false, ReconciliationStatus::Critical)
            };

            // If critical, trigger circuit breaker
            if matches!(status, ReconciliationStatus::Critical) {
                CircuitBreakerTriggered::<T>::put(true);
                Self::deposit_event(Event::CircuitBreakerTriggered);
            }

            // Update last reconciliation block
            LastReconciliationBlock::<T>::put(current_block);

            // Cache wrapped supply
            WrappedTotalSupply::<T>::put(wrapped_supply);

            // Emit events
            Self::deposit_event(Event::ReconciliationPerformed {
                timestamp: Self::current_timestamp(),
                balanced,
                discrepancy,
            });

            if !balanced {
                Self::deposit_event(Event::BalanceDiscrepancy {
                    discrepancy,
                    severity: status,
                });
            }

            Ok(())
        }

        /// Submit balance dispute
        ///
        /// Anyone can challenge the reported balance with proof.
        ///
        /// # Arguments
        /// * `origin` - Must be signed
        /// * `reported_balance` - Balance that was reported
        /// * `actual_balance` - Actual balance (with proof)
        /// * `proof` - Cryptographic proof
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn submit_balance_dispute(
            origin: OriginFor<T>,
            reported_balance: T::Balance,
            actual_balance: T::Balance,
            proof: Vec<u8>,
        ) -> DispatchResult {
            let challenger = ensure_signed(origin)?;

            // Generate dispute ID
            let dispute_id = DisputeCounter::<T>::get();
            DisputeCounter::<T>::put(dispute_id + 1);

            // Create dispute
            let dispute = BalanceDispute {
                challenger: challenger.clone(),
                reported_balance,
                actual_balance,
                timestamp: Self::current_timestamp(),
                proof,
                resolved: false,
            };

            // Store dispute
            Disputes::<T>::insert(dispute_id, dispute);

            // Emit event
            Self::deposit_event(Event::DisputeSubmitted {
                dispute_id,
                challenger,
            });

            Ok(())
        }

        /// Resolve dispute
        ///
        /// Governance or authorized origin resolves the dispute.
        ///
        /// # Arguments
        /// * `origin` - Must be authorized
        /// * `dispute_id` - The dispute to resolve
        /// * `valid` - Whether the dispute is valid
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn resolve_dispute(
            origin: OriginFor<T>,
            dispute_id: u64,
            valid: bool,
        ) -> DispatchResult {
            T::ReconciliationOrigin::ensure_origin(origin)?;

            // Get dispute
            let mut dispute = Disputes::<T>::get(dispute_id).ok_or(Error::<T>::DisputeNotFound)?;
            ensure!(!dispute.resolved, Error::<T>::DisputeAlreadyResolved);

            // Mark as resolved
            dispute.resolved = true;
            Disputes::<T>::insert(dispute_id, dispute.clone());

            // If valid, update external balance
            if valid {
                ExternalVaultBalance::<T>::put(dispute.actual_balance);
            }

            // Emit event
            Self::deposit_event(Event::DisputeResolved { dispute_id, valid });

            Ok(())
        }

        /// Reset circuit breaker
        ///
        /// After resolving discrepancies, governance can reset the circuit breaker.
        ///
        /// # Arguments
        /// * `origin` - Must be authorized
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn reset_circuit_breaker(origin: OriginFor<T>) -> DispatchResult {
            T::ReconciliationOrigin::ensure_origin(origin)?;

            CircuitBreakerTriggered::<T>::put(false);
            Self::deposit_event(Event::CircuitBreakerReset);

            Ok(())
        }

        /// Verify Merkle proof
        ///
        /// Public function to verify Merkle proofs for external state.
        ///
        /// # Arguments
        /// * `origin` - Must be signed
        /// * `root` - Merkle root
        /// * `leaf` - Leaf to verify
        /// * `proof` - Merkle proof path
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn verify_merkle_proof(
            origin: OriginFor<T>,
            root: [u8; 32],
            leaf: [u8; 32],
            proof: Vec<[u8; 32]>,
        ) -> DispatchResult {
            let _caller = ensure_signed(origin)?;

            // Verify Merkle proof
            let verified = Self::verify_merkle_proof_internal(root, leaf, &proof)?;
            ensure!(verified, Error::<T>::InvalidMerkleProof);

            // Emit event
            Self::deposit_event(Event::MerkleProofVerified { root, leaf });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Verify a proof (placeholder for actual verification logic)
        fn verify_proof(_proof: &[u8]) -> Result<bool, DispatchError> {
            // In production, implement actual proof verification
            // For now, return true
            Ok(true)
        }

        /// Verify Merkle proof internal
        fn verify_merkle_proof_internal(
            root: [u8; 32],
            mut current_hash: [u8; 32],
            proof: &[[u8; 32]],
        ) -> Result<bool, DispatchError> {
            use sp_io::hashing::blake2_256;

            for sibling in proof {
                // Combine hashes (sorted order)
                let combined = if current_hash < *sibling {
                    [current_hash.as_slice(), sibling.as_slice()].concat()
                } else {
                    [sibling.as_slice(), current_hash.as_slice()].concat()
                };

                current_hash = blake2_256(&combined);
            }

            Ok(current_hash == root)
        }

        /// Get current timestamp (placeholder)
        fn current_timestamp() -> u64 {
            // In production, use pallet-timestamp
            <frame_system::Pallet<T>>::block_number().try_into().ok().unwrap_or(0)
        }

        /// Get reconciliation report
        pub fn get_reconciliation_report(
            internal_balance: T::Balance,
        ) -> ReconciliationReport<T::Balance> {
            let external_balance = ExternalVaultBalance::<T>::get();
            let wrapped_supply = WrappedTotalSupply::<T>::get();

            let internal_u128: u128 = internal_balance.try_into().ok().unwrap_or(0);
            let external_u128: u128 = external_balance.try_into().ok().unwrap_or(0);
            let discrepancy = (external_u128 as i128) - (internal_u128 as i128);

            let balanced = discrepancy == 0;
            let status = if balanced {
                ReconciliationStatus::Healthy
            } else if discrepancy.abs() as u128 <= (internal_u128 * 50) / 10000 {
                ReconciliationStatus::Warning
            } else {
                ReconciliationStatus::Critical
            };

            ReconciliationReport {
                timestamp: Self::current_timestamp(),
                internal_balance,
                external_balance,
                discrepancy,
                wrapped_supply,
                balanced,
                status,
            }
        }
    }
}
