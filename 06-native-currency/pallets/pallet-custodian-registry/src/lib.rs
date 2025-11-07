//! # Custodian Registry Pallet
//!
//! Registry for off-chain reserve custodians with bonding, attestation, and slashing.
//!
//! ## Features
//! - Bonded custodian registration (slashable security deposit)
//! - Quarterly attestation submissions (proof of reserves)
//! - Regulatory compliance verification (license proofs)
//! - Slashing mechanism for non-compliance
//! - Governance-controlled approval process
//!
//! ## Custodian Requirements
//! - Security bond (slashable deposit)
//! - Regulatory license proof
//! - Quarterly reserve attestations
//! - Third-party audit verification
//!
//! ## Reserve Types
//! - Fiat currency (USD, EUR, etc.)
//! - T-Bills and government bonds
//! - Other stable, auditable assets

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{CheckedAdd, CheckedSub, Saturating, Zero};
	use sp_std::vec::Vec;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Custodian status
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[codec(dumb_trait_bound)]
	pub enum CustodianStatus {
		/// Pending governance approval
		Pending,
		/// Active and in good standing
		Active,
		/// Suspended (investigation or issues)
		Suspended,
		/// Slashed (bond partially or fully lost)
		Slashed,
		/// Removed from registry
		Removed,
	}

	/// Custodian information
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[codec(dumb_trait_bound)]
	pub struct CustodianInfo<AccountId, Balance, BlockNumber> {
		/// Custodian account
		pub account: AccountId,
		/// Security bond amount (slashable)
		pub bond_amount: Balance,
		/// Regulatory license proof (hash or IPFS CID)
		pub license_proof: BoundedVec<u8, ConstU32<256>>,
		/// Last attestation block
		pub last_attestation: BlockNumber,
		/// Current status
		pub status: CustodianStatus,
		/// Current attested reserve value (USD cents)
		pub attested_value: u128,
		/// Number of missed attestations
		pub missed_attestations: u32,
	}

	/// Attestation submission
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[codec(dumb_trait_bound)]
	pub struct Attestation<BlockNumber> {
		/// Reserve value in USD cents
		pub reserve_value: u128,
		/// Proof of reserves (audit report hash, IPFS CID, etc.)
		pub proof: BoundedVec<u8, ConstU32<512>>,
		/// Auditor signature or verification
		pub auditor_signature: BoundedVec<u8, ConstU32<256>>,
		/// Submission block
		pub submitted_at: BlockNumber,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_reserve_vault::Config {
		/// Currency for bonding
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Minimum bond amount for custodians
		#[pallet::constant]
		type MinBondAmount: Get<BalanceOf<Self>>;

		/// Attestation frequency (in blocks, default quarterly = ~2,160,000 blocks @ 6s)
		#[pallet::constant]
		type AttestationFrequency: Get<BlockNumberFor<Self>>;

		/// Maximum missed attestations before suspension
		#[pallet::constant]
		type MaxMissedAttestations: Get<u32>;

		/// Slash amount for non-compliance (percentage of bond, in Permill)
		#[pallet::constant]
		type SlashPercentage: Get<sp_arithmetic::Permill>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Next custodian ID
	#[pallet::storage]
	#[pallet::getter(fn next_custodian_id)]
	pub type NextCustodianId<T> = StorageValue<_, u64, ValueQuery>;

	/// Custodian registry
	#[pallet::storage]
	#[pallet::getter(fn custodians)]
	pub type Custodians<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // custodian_id
		CustodianInfo<T::AccountId, BalanceOf<T>, BlockNumberFor<T>>,
	>;

	/// Account to custodian ID mapping
	#[pallet::storage]
	#[pallet::getter(fn custodian_id_of)]
	pub type CustodianIdOf<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64>;

	/// Attestation history (custodian_id => attestations)
	#[pallet::storage]
	#[pallet::getter(fn attestations)]
	pub type Attestations<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // custodian_id
		BoundedVec<Attestation<BlockNumberFor<T>>, ConstU32<100>>, // Last 100 attestations
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Custodian registered [custodian_id, account, bond_amount]
		CustodianRegistered {
			custodian_id: u64,
			account: T::AccountId,
			bond_amount: BalanceOf<T>,
		},
		/// Custodian approved [custodian_id]
		CustodianApproved { custodian_id: u64 },
		/// Attestation submitted [custodian_id, reserve_value]
		AttestationSubmitted {
			custodian_id: u64,
			reserve_value: u128,
		},
		/// Custodian suspended [custodian_id, reason]
		CustodianSuspended {
			custodian_id: u64,
			reason: BoundedVec<u8, ConstU32<256>>,
		},
		/// Custodian slashed [custodian_id, slash_amount]
		CustodianSlashed {
			custodian_id: u64,
			slash_amount: BalanceOf<T>,
		},
		/// Custodian removed [custodian_id]
		CustodianRemoved { custodian_id: u64 },
		/// Attestation missed [custodian_id, missed_count]
		AttestationMissed {
			custodian_id: u64,
			missed_count: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Custodian not found
		CustodianNotFound,
		/// Custodian already registered
		AlreadyRegistered,
		/// Insufficient bond amount
		InsufficientBond,
		/// Custodian not active
		NotActive,
		/// Attestation too early
		AttestationTooEarly,
		/// Invalid proof format
		InvalidProof,
		/// Not authorized (governance only)
		NotAuthorized,
		/// Arithmetic overflow
		Overflow,
		/// Arithmetic underflow
		Underflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register as custodian (requires bond)
		///
		/// # Parameters
		/// - `origin`: Custodian account
		/// - `bond_amount`: Security deposit amount
		/// - `license_proof`: Regulatory license proof (hash/CID)
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn register_custodian(
			origin: OriginFor<T>,
			bond_amount: BalanceOf<T>,
			license_proof: BoundedVec<u8, ConstU32<256>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check not already registered
			ensure!(!CustodianIdOf::<T>::contains_key(&who), Error::<T>::AlreadyRegistered);

			// Check minimum bond
			ensure!(bond_amount >= T::MinBondAmount::get(), Error::<T>::InsufficientBond);

			// Reserve bond
			T::Currency::reserve(&who, bond_amount)?;

			// Create custodian entry
			let custodian_id = NextCustodianId::<T>::get();
			let current_block = <frame_system::Pallet<T>>::block_number();

			let custodian_info = CustodianInfo {
				account: who.clone(),
				bond_amount,
				license_proof,
				last_attestation: current_block,
				status: CustodianStatus::Pending,
				attested_value: 0,
				missed_attestations: 0,
			};

			// Store custodian
			Custodians::<T>::insert(custodian_id, custodian_info);
			CustodianIdOf::<T>::insert(&who, custodian_id);
			NextCustodianId::<T>::put(custodian_id.saturating_add(1));

			Self::deposit_event(Event::CustodianRegistered {
				custodian_id,
				account: who,
				bond_amount,
			});

			Ok(())
		}

		/// Approve custodian (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `custodian_id`: Custodian to approve
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn approve_custodian(
			origin: OriginFor<T>,
			custodian_id: u64,
		) -> DispatchResult {
			ensure_root(origin)?;

			Custodians::<T>::try_mutate(custodian_id, |maybe_custodian| -> DispatchResult {
				let custodian = maybe_custodian.as_mut().ok_or(Error::<T>::CustodianNotFound)?;

				// Must be pending
				ensure!(custodian.status == CustodianStatus::Pending, Error::<T>::NotActive);

				// Approve
				custodian.status = CustodianStatus::Active;

				Self::deposit_event(Event::CustodianApproved { custodian_id });

				Ok(())
			})
		}

		/// Submit reserve attestation (active custodians only)
		///
		/// # Parameters
		/// - `origin`: Custodian account
		/// - `reserve_value`: Attested reserve value (USD cents)
		/// - `proof`: Proof of reserves (audit report, etc.)
		/// - `auditor_signature`: Third-party auditor signature
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn submit_attestation(
			origin: OriginFor<T>,
			reserve_value: u128,
			proof: BoundedVec<u8, ConstU32<512>>,
			auditor_signature: BoundedVec<u8, ConstU32<256>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get custodian ID
			let custodian_id = CustodianIdOf::<T>::get(&who)
				.ok_or(Error::<T>::CustodianNotFound)?;

			// Get custodian info
			let mut custodian = Custodians::<T>::get(custodian_id)
				.ok_or(Error::<T>::CustodianNotFound)?;

			// Check status
			ensure!(custodian.status == CustodianStatus::Active, Error::<T>::NotActive);

			// Check attestation frequency
			let current_block = <frame_system::Pallet<T>>::block_number();
			let elapsed = current_block.saturating_sub(custodian.last_attestation);
			// Allow attestation if at least 75% of frequency has passed
			let frequency = T::AttestationFrequency::get();
			let min_elapsed = frequency.saturating_mul(3u32.into()) / 4u32.into();
			ensure!(elapsed >= min_elapsed, Error::<T>::AttestationTooEarly);

			// Create attestation
			let attestation = Attestation {
				reserve_value,
				proof,
				auditor_signature,
				submitted_at: current_block,
			};

			// Add to history
			Attestations::<T>::try_mutate(custodian_id, |attestations| -> DispatchResult {
				if attestations.len() >= 100 {
					attestations.remove(0); // Remove oldest
				}
				attestations.try_push(attestation).map_err(|_| Error::<T>::Overflow)?;
				Ok(())
			})?;

			// Update custodian info
			custodian.attested_value = reserve_value;
			custodian.last_attestation = current_block;
			custodian.missed_attestations = 0; // Reset missed count
			Custodians::<T>::insert(custodian_id, custodian);

			// Update reserve vault
			let _ = pallet_reserve_vault::Pallet::<T>::do_update_custodian_value(
				Self::calculate_total_attested_value()
			);

			Self::deposit_event(Event::AttestationSubmitted {
				custodian_id,
				reserve_value,
			});

			Ok(())
		}

		/// Suspend custodian (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `custodian_id`: Custodian to suspend
		/// - `reason`: Reason for suspension
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn suspend_custodian(
			origin: OriginFor<T>,
			custodian_id: u64,
			reason: BoundedVec<u8, ConstU32<256>>,
		) -> DispatchResult {
			ensure_root(origin)?;

			Custodians::<T>::try_mutate(custodian_id, |maybe_custodian| -> DispatchResult {
				let custodian = maybe_custodian.as_mut().ok_or(Error::<T>::CustodianNotFound)?;

				custodian.status = CustodianStatus::Suspended;

				Self::deposit_event(Event::CustodianSuspended {
					custodian_id,
					reason,
				});

				// Recalculate total (suspended custodians don't count)
				let total = Self::calculate_total_attested_value();
				let _ = pallet_reserve_vault::Pallet::<T>::do_update_custodian_value(total);

				Ok(())
			})
		}

		/// Slash custodian bond (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `custodian_id`: Custodian to slash
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn slash_custodian(
			origin: OriginFor<T>,
			custodian_id: u64,
		) -> DispatchResult {
			ensure_root(origin)?;

			Custodians::<T>::try_mutate(custodian_id, |maybe_custodian| -> DispatchResult {
				let custodian = maybe_custodian.as_mut().ok_or(Error::<T>::CustodianNotFound)?;

				// Calculate slash amount
				let slash_amount = T::SlashPercentage::get() * custodian.bond_amount;

				// Slash (unreserve and burn)
				T::Currency::unreserve(&custodian.account, slash_amount);
				// TODO: Actually burn or transfer to treasury

				// Update bond amount
				custodian.bond_amount = custodian.bond_amount.saturating_sub(slash_amount);
				custodian.status = CustodianStatus::Slashed;

				Self::deposit_event(Event::CustodianSlashed {
					custodian_id,
					slash_amount,
				});

				// Recalculate total
				let total = Self::calculate_total_attested_value();
				let _ = pallet_reserve_vault::Pallet::<T>::do_update_custodian_value(total);

				Ok(())
			})
		}

		/// Remove custodian (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `custodian_id`: Custodian to remove
		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		pub fn remove_custodian(
			origin: OriginFor<T>,
			custodian_id: u64,
		) -> DispatchResult {
			ensure_root(origin)?;

			let custodian = Custodians::<T>::get(custodian_id)
				.ok_or(Error::<T>::CustodianNotFound)?;

			// Unreserve remaining bond
			T::Currency::unreserve(&custodian.account, custodian.bond_amount);

			// Remove from storage
			Custodians::<T>::remove(custodian_id);
			CustodianIdOf::<T>::remove(&custodian.account);
			Attestations::<T>::remove(custodian_id);

			Self::deposit_event(Event::CustodianRemoved { custodian_id });

			// Recalculate total
			let total = Self::calculate_total_attested_value();
			let _ = pallet_reserve_vault::Pallet::<T>::do_update_custodian_value(total);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Calculate total attested value from all active custodians
		fn calculate_total_attested_value() -> u128 {
			let mut total: u128 = 0;

			for (_id, custodian) in Custodians::<T>::iter() {
				// Only count active custodians
				if custodian.status == CustodianStatus::Active {
					total = total.saturating_add(custodian.attested_value);
				}
			}

			total
		}

		/// Check for missed attestations and suspend if needed
		fn check_missed_attestations() {
			let current_block = <frame_system::Pallet<T>>::block_number();

			for (custodian_id, mut custodian) in Custodians::<T>::iter() {
				// Only check active custodians
				if custodian.status != CustodianStatus::Active {
					continue;
				}

				let elapsed = current_block.saturating_sub(custodian.last_attestation);

				// If more than attestation frequency has passed, mark as missed
				if elapsed > T::AttestationFrequency::get() {
					custodian.missed_attestations = custodian.missed_attestations.saturating_add(1);

					Self::deposit_event(Event::AttestationMissed {
						custodian_id,
						missed_count: custodian.missed_attestations,
					});

					// Auto-suspend if too many missed
					if custodian.missed_attestations >= T::MaxMissedAttestations::get() {
						custodian.status = CustodianStatus::Suspended;

						let reason: BoundedVec<u8, ConstU32<256>> =
							b"Excessive missed attestations".to_vec().try_into().unwrap_or_default();

						Self::deposit_event(Event::CustodianSuspended {
							custodian_id,
							reason,
						});
					}

					Custodians::<T>::insert(custodian_id, custodian);
				}
			}

			// Recalculate total attested value
			let total = Self::calculate_total_attested_value();
			let _ = pallet_reserve_vault::Pallet::<T>::do_update_custodian_value(total);
		}

		/// Get total attested value (for external queries)
		pub fn get_total_attested_value() -> u128 {
			Self::calculate_total_attested_value()
		}

		/// Get custodian count by status
		pub fn get_custodian_count_by_status(status: CustodianStatus) -> u32 {
			Custodians::<T>::iter()
				.filter(|(_, c)| c.status == status)
				.count() as u32
		}
	}

	/// Hooks for automatic missed attestation checks
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// On finalize, check for missed attestations
		fn on_finalize(n: BlockNumberFor<T>) {
			// Check every 1000 blocks (~100 minutes @ 6s blocks)
			use sp_runtime::traits::Zero;
			let thousand: BlockNumberFor<T> = 1000u32.into();

			if (n % thousand).is_zero() {
				Self::check_missed_attestations();
			}
		}
	}
}
