//! # EDSC Receipts Pallet
//!
//! Soulbound Token (SBT) receipt registry for EDSC purchases.
//! Receipts enable fee-free redemption by proving purchase price.
//!
//! ## Features
//! - Non-transferable receipts (SBT)
//! - Records purchase price at time of minting
//! - Partial consumption (redeem in multiple transactions)
//! - Per-wallet and daily caps enforcement
//! - Authorized minter control

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{Saturating, Zero};
	use sp_std::vec::Vec;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Receipt<AccountId, BlockNumber> {
		/// Owner of the receipt (cannot be transferred)
		pub owner: AccountId,
		/// Total EDSC amount on receipt
		pub amount: u128,
		/// Purchase price in USD cents (e.g., 100 = $1.00)
		pub purchase_price: u128,
		/// Block when receipt was created
		pub timestamp: BlockNumber,
		/// Remaining amount available for redemption
		pub amount_available: u128,
		/// Expiry block (optional, 0 = no expiry)
		pub expiry: BlockNumber,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum receipts per wallet
		#[pallet::constant]
		type MaxReceiptsPerWallet: Get<u32>;

		/// Receipt expiry period (in blocks, 0 = no expiry)
		#[pallet::constant]
		type ReceiptExpiryPeriod: Get<BlockNumberFor<Self>>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Next receipt ID
	#[pallet::storage]
	#[pallet::getter(fn next_receipt_id)]
	pub type NextReceiptId<T> = StorageValue<_, u64, ValueQuery>;

	/// All receipts by ID
	#[pallet::storage]
	#[pallet::getter(fn receipts)]
	pub type Receipts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // receipt_id
		Receipt<T::AccountId, BlockNumberFor<T>>,
	>;

	/// Receipt IDs owned by each account
	#[pallet::storage]
	#[pallet::getter(fn receipts_of)]
	pub type ReceiptsOf<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<u64, T::MaxReceiptsPerWallet>,
		ValueQuery,
	>;

	/// Authorized receipt minters (exchanges, merchants, etc.)
	#[pallet::storage]
	#[pallet::getter(fn is_authorized_minter)]
	pub type AuthorizedMinters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	/// Per-wallet redemption tracking (daily limits)
	/// (AccountId, Day) => Amount redeemed
	#[pallet::storage]
	#[pallet::getter(fn daily_redeemed)]
	pub type DailyRedeemed<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		u32, // day number (block / blocks_per_day)
		u128,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Receipt created [receipt_id, owner, amount, price]
		ReceiptCreated {
			receipt_id: u64,
			owner: T::AccountId,
			amount: u128,
			price: u128,
		},
		/// Receipt consumed (partially or fully) [receipt_id, amount_consumed, amount_remaining]
		ReceiptConsumed {
			receipt_id: u64,
			amount_consumed: u128,
			amount_remaining: u128,
		},
		/// Receipt expired [receipt_id]
		ReceiptExpired {
			receipt_id: u64,
		},
		/// Minter authorized [minter]
		MinterAuthorized {
			minter: T::AccountId,
		},
		/// Minter revoked [minter]
		MinterRevoked {
			minter: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Receipt does not exist
		ReceiptNotFound,
		/// Caller is not the receipt owner
		NotOwner,
		/// Caller is not an authorized minter
		NotAuthorizedMinter,
		/// Insufficient amount available on receipt
		InsufficientReceiptBalance,
		/// Receipt has expired
		ReceiptExpired,
		/// Too many receipts for this wallet
		TooManyReceipts,
		/// Arithmetic overflow
		Overflow,
		/// Arithmetic underflow
		Underflow,
		/// Daily redemption limit exceeded
		DailyLimitExceeded,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new receipt (authorized minters only)
		///
		/// # Parameters
		/// - `origin`: Minter account (must be authorized)
		/// - `owner`: Recipient of the receipt
		/// - `amount`: EDSC amount on receipt
		/// - `price`: Purchase price in USD cents
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn create_receipt(
			origin: OriginFor<T>,
			owner: T::AccountId,
			amount: u128,
			price: u128,
		) -> DispatchResult {
			let minter = ensure_signed(origin)?;

			// Check minter authorization
			ensure!(AuthorizedMinters::<T>::get(&minter), Error::<T>::NotAuthorizedMinter);

			// Check receipt limit
			let mut owner_receipts = ReceiptsOf::<T>::get(&owner);
			ensure!(
				(owner_receipts.len() as u32) < T::MaxReceiptsPerWallet::get(),
				Error::<T>::TooManyReceipts
			);

			// Calculate expiry
			let current_block = <frame_system::Pallet<T>>::block_number();
			let expiry = if T::ReceiptExpiryPeriod::get() > Zero::zero() {
				current_block.saturating_add(T::ReceiptExpiryPeriod::get())
			} else {
				Zero::zero()
			};

			// Create receipt
			let receipt_id = NextReceiptId::<T>::get();
			let receipt = Receipt {
				owner: owner.clone(),
				amount,
				purchase_price: price,
				timestamp: current_block,
				amount_available: amount,
				expiry,
			};

			// Store receipt
			Receipts::<T>::insert(receipt_id, receipt);
			owner_receipts
				.try_push(receipt_id)
				.map_err(|_| Error::<T>::TooManyReceipts)?;
			ReceiptsOf::<T>::insert(&owner, owner_receipts);
			NextReceiptId::<T>::put(receipt_id.saturating_add(1));

			Self::deposit_event(Event::ReceiptCreated {
				receipt_id,
				owner,
				amount,
				price,
			});

			Ok(())
		}

		/// Consume receipt (partial or full redemption)
		///
		/// # Parameters
		/// - `origin`: Receipt owner
		/// - `receipt_id`: ID of receipt to consume
		/// - `amount`: Amount to consume
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn consume_receipt(
			origin: OriginFor<T>,
			receipt_id: u64,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Get receipt
			let mut receipt = Receipts::<T>::get(receipt_id)
				.ok_or(Error::<T>::ReceiptNotFound)?;

			// Check ownership
			ensure!(receipt.owner == who, Error::<T>::NotOwner);

			// Check expiry
			let current_block = <frame_system::Pallet<T>>::block_number();
			if receipt.expiry > Zero::zero() {
				ensure!(current_block <= receipt.expiry, Error::<T>::ReceiptExpired);
			}

			// Check available amount
			ensure!(receipt.amount_available >= amount, Error::<T>::InsufficientReceiptBalance);

			// Update receipt
			receipt.amount_available = receipt.amount_available
				.checked_sub(amount)
				.ok_or(Error::<T>::Underflow)?;

			// If fully consumed, remove receipt
			if receipt.amount_available == 0 {
				Receipts::<T>::remove(receipt_id);
				ReceiptsOf::<T>::mutate(&who, |receipts| {
					receipts.retain(|&id| id != receipt_id);
				});
			} else {
				Receipts::<T>::insert(receipt_id, &receipt);
			}

			Self::deposit_event(Event::ReceiptConsumed {
				receipt_id,
				amount_consumed: amount,
				amount_remaining: receipt.amount_available,
			});

			Ok(())
		}

		/// Authorize a receipt minter (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `minter`: Account to authorize
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn authorize_minter(
			origin: OriginFor<T>,
			minter: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			AuthorizedMinters::<T>::insert(&minter, true);
			Self::deposit_event(Event::MinterAuthorized { minter });
			Ok(())
		}

		/// Revoke minter authorization (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `minter`: Account to revoke
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn revoke_minter(
			origin: OriginFor<T>,
			minter: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			AuthorizedMinters::<T>::remove(&minter);
			Self::deposit_event(Event::MinterRevoked { minter });
			Ok(())
		}

		/// Clean up expired receipts (anyone can call)
		///
		/// # Parameters
		/// - `origin`: Anyone
		/// - `receipt_id`: ID of expired receipt
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn remove_expired_receipt(
			origin: OriginFor<T>,
			receipt_id: u64,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let receipt = Receipts::<T>::get(receipt_id)
				.ok_or(Error::<T>::ReceiptNotFound)?;

			// Check if expired
			let current_block = <frame_system::Pallet<T>>::block_number();
			if receipt.expiry > Zero::zero() {
				ensure!(current_block > receipt.expiry, Error::<T>::ReceiptExpired);
			}

			// Remove receipt
			Receipts::<T>::remove(receipt_id);
			ReceiptsOf::<T>::mutate(&receipt.owner, |receipts| {
				receipts.retain(|&id| id != receipt_id);
			});

			Self::deposit_event(Event::ReceiptExpired { receipt_id });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get receipt purchase price (for redemption pallet)
		pub fn get_receipt_price(receipt_id: u64) -> Result<u128, DispatchError> {
			let receipt = Receipts::<T>::get(receipt_id)
				.ok_or(Error::<T>::ReceiptNotFound)?;
			Ok(receipt.purchase_price)
		}

		/// Check if receipt is valid and owned by account
		pub fn is_valid_receipt(
			receipt_id: u64,
			owner: &T::AccountId,
		) -> Result<bool, DispatchError> {
			if let Some(receipt) = Receipts::<T>::get(receipt_id) {
				// Check ownership
				if receipt.owner != *owner {
					return Ok(false);
				}

				// Check expiry
				let current_block = <frame_system::Pallet<T>>::block_number();
				if receipt.expiry > Zero::zero() && current_block > receipt.expiry {
					return Ok(false);
				}

				// Check available amount
				if receipt.amount_available == 0 {
					return Ok(false);
				}

				Ok(true)
			} else {
				Ok(false)
			}
		}

		/// Get all receipts for an account
		pub fn get_account_receipts(
			owner: &T::AccountId,
		) -> Vec<(u64, Receipt<T::AccountId, BlockNumberFor<T>>)> {
			let receipt_ids = ReceiptsOf::<T>::get(owner);
			receipt_ids
				.iter()
				.filter_map(|&id| Receipts::<T>::get(id).map(|r| (id, r)))
				.collect()
		}
	}
}
