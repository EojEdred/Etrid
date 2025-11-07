//! # EDSC Token Pallet
//!
//! Ëtrid Dollar Stablecoin (EDSC) token implementation with:
//! - Controlled minting (governance/authorized minters only)
//! - Public burning (redemption mechanism)
//! - Standard ERC20-like transfers
//! - Supply tracking and caps

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Placeholder WeightInfo trait
pub trait WeightInfo {}
impl WeightInfo for () {}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;

	type BalanceOf = u128;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Maximum supply cap (prevents infinite minting)
		#[pallet::constant]
		type MaxSupply: Get<u128>;

		/// Minimum balance to prevent dust accounts
		#[pallet::constant]
		type MinBalance: Get<u128>;

		/// Weight information for extrinsics
		type WeightInfo: crate::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Total supply of EDSC tokens
	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub type TotalSupply<T> = StorageValue<_, u128, ValueQuery>;

	/// Token balances for each account
	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;

	/// Allowances for delegated transfers (spender => amount)
	#[pallet::storage]
	#[pallet::getter(fn allowance)]
	pub type Allowances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, // owner
		Blake2_128Concat,
		T::AccountId, // spender
		u128,
		ValueQuery,
	>;

	/// Authorized minter addresses (can mint EDSC)
	/// Only governance can add/remove minters
	#[pallet::storage]
	#[pallet::getter(fn is_minter)]
	pub type AuthorizedMinters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	/// Minting paused flag (emergency control)
	#[pallet::storage]
	#[pallet::getter(fn minting_paused)]
	pub type MintingPaused<T> = StorageValue<_, bool, ValueQuery>;

	/// Burning paused flag (emergency control)
	#[pallet::storage]
	#[pallet::getter(fn burning_paused)]
	pub type BurningPaused<T> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// EDSC tokens transferred [from, to, amount]
		Transfer { from: T::AccountId, to: T::AccountId, amount: u128 },
		/// EDSC tokens minted [to, amount]
		Minted { to: T::AccountId, amount: u128 },
		/// EDSC tokens burned [from, amount]
		Burned { from: T::AccountId, amount: u128 },
		/// Approval granted [owner, spender, amount]
		Approval { owner: T::AccountId, spender: T::AccountId, amount: u128 },
		/// Minter authorized [minter]
		MinterAuthorized { minter: T::AccountId },
		/// Minter revoked [minter]
		MinterRevoked { minter: T::AccountId },
		/// Minting paused
		MintingPaused,
		/// Minting unpaused
		MintingUnpaused,
		/// Burning paused
		BurningPaused,
		/// Burning unpaused
		BurningUnpaused,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Insufficient balance for transfer/burn
		InsufficientBalance,
		/// Transfer would create dust account
		BelowMinimumBalance,
		/// Caller is not an authorized minter
		NotAuthorizedMinter,
		/// Minting would exceed max supply
		MaxSupplyExceeded,
		/// Minting is currently paused
		MintingPaused,
		/// Burning is currently paused
		BurningPaused,
		/// Insufficient allowance for delegated transfer
		InsufficientAllowance,
		/// Arithmetic overflow
		Overflow,
		/// Arithmetic underflow
		Underflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfer EDSC tokens to another account
		///
		/// # Parameters
		/// - `origin`: Sender account
		/// - `to`: Recipient account
		/// - `amount`: Amount to transfer (in smallest units)
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_transfer(&from, &to, amount)?;
			Ok(())
		}

		/// Approve spender to transfer tokens on behalf of owner
		///
		/// # Parameters
		/// - `origin`: Owner account
		/// - `spender`: Account authorized to spend
		/// - `amount`: Maximum amount spender can transfer
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn approve(
			origin: OriginFor<T>,
			spender: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			Allowances::<T>::insert(&owner, &spender, amount);
			Self::deposit_event(Event::Approval { owner, spender, amount });
			Ok(())
		}

		/// Transfer tokens using allowance (delegated transfer)
		///
		/// # Parameters
		/// - `origin`: Spender account
		/// - `from`: Owner account
		/// - `to`: Recipient account
		/// - `amount`: Amount to transfer
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;

			// Check allowance
			let allowance = Allowances::<T>::get(&from, &spender);
			ensure!(allowance >= amount, Error::<T>::InsufficientAllowance);

			// Perform transfer
			Self::do_transfer(&from, &to, amount)?;

			// Decrease allowance
			let new_allowance = allowance.checked_sub(amount).ok_or(Error::<T>::Underflow)?;
			Allowances::<T>::insert(&from, &spender, new_allowance);

			Ok(())
		}

		/// Mint new EDSC tokens (authorized minters only)
		///
		/// # Parameters
		/// - `origin`: Minter account (must be authorized)
		/// - `to`: Recipient account
		/// - `amount`: Amount to mint
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn mint(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let minter = ensure_signed(origin)?;

			// Check minter authorization
			ensure!(AuthorizedMinters::<T>::get(&minter), Error::<T>::NotAuthorizedMinter);

			// Check minting not paused
			ensure!(!MintingPaused::<T>::get(), Error::<T>::MintingPaused);

			// Check max supply
			let new_supply = TotalSupply::<T>::get()
				.checked_add(amount)
				.ok_or(Error::<T>::Overflow)?;
			ensure!(new_supply <= T::MaxSupply::get(), Error::<T>::MaxSupplyExceeded);

			// Mint tokens
			Self::do_mint(&to, amount)?;

			Ok(())
		}

		/// Burn EDSC tokens (anyone can burn their own tokens)
		///
		/// # Parameters
		/// - `origin`: Token holder account
		/// - `amount`: Amount to burn
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn burn(
			origin: OriginFor<T>,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check burning not paused
			ensure!(!BurningPaused::<T>::get(), Error::<T>::BurningPaused);

			// Burn tokens
			Self::do_burn(&who, amount)?;

			Ok(())
		}

		/// Authorize a minter (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `minter`: Account to authorize
		#[pallet::call_index(5)]
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
		#[pallet::call_index(6)]
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

		/// Pause minting (emergency control, governance only)
		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn pause_minting(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			MintingPaused::<T>::put(true);
			Self::deposit_event(Event::MintingPaused);
			Ok(())
		}

		/// Unpause minting (governance only)
		#[pallet::call_index(8)]
		#[pallet::weight(10_000)]
		pub fn unpause_minting(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			MintingPaused::<T>::put(false);
			Self::deposit_event(Event::MintingUnpaused);
			Ok(())
		}

		/// Pause burning (emergency control, governance only)
		#[pallet::call_index(9)]
		#[pallet::weight(10_000)]
		pub fn pause_burning(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			BurningPaused::<T>::put(true);
			Self::deposit_event(Event::BurningPaused);
			Ok(())
		}

		/// Unpause burning (governance only)
		#[pallet::call_index(10)]
		#[pallet::weight(10_000)]
		pub fn unpause_burning(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			BurningPaused::<T>::put(false);
			Self::deposit_event(Event::BurningUnpaused);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Internal transfer logic
		pub fn do_transfer(
			from: &T::AccountId,
			to: &T::AccountId,
			amount: u128,
		) -> DispatchResult {
			// Check sender balance
			let from_balance = Balances::<T>::get(from);
			ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

			// Check minimum balance constraint
			if from_balance - amount > 0 {
				ensure!(
					from_balance - amount >= T::MinBalance::get(),
					Error::<T>::BelowMinimumBalance
				);
			}

			// Update balances
			let new_from_balance = from_balance.checked_sub(amount).ok_or(Error::<T>::Underflow)?;
			let to_balance = Balances::<T>::get(to);
			let new_to_balance = to_balance.checked_add(amount).ok_or(Error::<T>::Overflow)?;

			Balances::<T>::insert(from, new_from_balance);
			Balances::<T>::insert(to, new_to_balance);

			Self::deposit_event(Event::Transfer { from: from.clone(), to: to.clone(), amount });

			Ok(())
		}

		/// Internal mint logic
		pub fn do_mint(to: &T::AccountId, amount: u128) -> DispatchResult {
			// Update recipient balance
			let balance = Balances::<T>::get(to);
			let new_balance = balance.checked_add(amount).ok_or(Error::<T>::Overflow)?;
			Balances::<T>::insert(to, new_balance);

			// Update total supply
			let total = TotalSupply::<T>::get();
			let new_total = total.checked_add(amount).ok_or(Error::<T>::Overflow)?;
			TotalSupply::<T>::put(new_total);

			Self::deposit_event(Event::Minted { to: to.clone(), amount });

			Ok(())
		}

		/// Internal burn logic
		pub fn do_burn(from: &T::AccountId, amount: u128) -> DispatchResult {
			// Check balance
			let balance = Balances::<T>::get(from);
			ensure!(balance >= amount, Error::<T>::InsufficientBalance);

			// Update balance
			let new_balance = balance.checked_sub(amount).ok_or(Error::<T>::Underflow)?;
			Balances::<T>::insert(from, new_balance);

			// Update total supply
			let total = TotalSupply::<T>::get();
			let new_total = total.checked_sub(amount).ok_or(Error::<T>::Underflow)?;
			TotalSupply::<T>::put(new_total);

			Self::deposit_event(Event::Burned { from: from.clone(), amount });

			Ok(())
		}
	}
}
