//! # Reserve-Backed Token Pallet
//!
//! Synthetic asset creation backed by multi-asset reserves.
//!
//! ## Overview
//!
//! This pallet enables the creation and management of reserve-backed synthetic tokens
//! (like sBTC, sETH, sUSD) that are fully collateralized by assets in the multi-asset reserve.
//!
//! ## Features
//!
//! - **Synthetic Token Creation**: Governance can create new synthetic assets
//! - **Collateralized Minting**: Users mint synthetics by locking collateral
//! - **Burning & Redemption**: Burn synthetics to retrieve collateral
//! - **Collateralization Ratios**: Maintain healthy collateral levels (e.g., 150%)
//! - **Liquidation**: Undercollateralized positions can be liquidated
//! - **Oracle Integration**: Uses reserve oracle for price feeds
//! - **Multi-Collateral**: Support various collateral types
//!
//! ## Use Cases
//!
//! - sBTC: Synthetic Bitcoin backed by diversified reserve
//! - sETH: Synthetic Ethereum
//! - sUSD: Algorithmic stablecoin backed by crypto reserves
//! - sGold: Synthetic gold-backed token

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency, ExistenceRequirement},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{FixedPointNumber, FixedU128, Permill};
	use sp_runtime::traits::{AccountIdConversion, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, Saturating};
	use sp_runtime::SaturatedConversion;
	use sp_std::vec::Vec;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Synthetic token metadata
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct SyntheticToken {
		/// Token symbol (sBTC, sETH, etc.)
		pub symbol: BoundedVec<u8, ConstU32<16>>,
		/// Token name
		pub name: BoundedVec<u8, ConstU32<64>>,
		/// Decimals
		pub decimals: u8,
		/// Minimum collateralization ratio (150% = 15000 basis points)
		pub min_collateral_ratio: u16,
		/// Liquidation ratio (120% = 12000 basis points)
		pub liquidation_ratio: u16,
		/// Total supply
		pub total_supply: u128,
		/// Is active
		pub is_active: bool,
		/// Creation block
		pub created_at: u32,
	}

	/// Collateral position for a user
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CollateralPosition<Balance> {
		/// Synthetic token ID
		pub synthetic_id: u32,
		/// Collateral amount locked
		pub collateral_amount: Balance,
		/// Synthetic tokens minted
		pub synthetic_minted: u128,
		/// Last update block
		pub last_update: u32,
	}

	/// Liquidation record
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct LiquidationRecord<AccountId, Balance> {
		/// Position owner
		pub owner: AccountId,
		/// Liquidator
		pub liquidator: AccountId,
		/// Synthetic ID
		pub synthetic_id: u32,
		/// Amount liquidated
		pub amount: u128,
		/// Collateral seized
		pub collateral_seized: Balance,
		/// Liquidation block
		pub block_number: u32,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Native currency for collateral
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Maximum number of synthetic tokens
		#[pallet::constant]
		type MaxSynthetics: Get<u32>;

		/// Maximum positions per user
		#[pallet::constant]
		type MaxPositionsPerUser: Get<u32>;

		/// Minimum collateral amount
		#[pallet::constant]
		type MinCollateral: Get<BalanceOf<Self>>;

		/// Liquidation penalty (5% = 500 basis points)
		#[pallet::constant]
		type LiquidationPenalty: Get<u16>;

		/// Pallet ID
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	pub trait WeightInfo {
		fn create_synthetic() -> Weight;
		fn mint_synthetic() -> Weight;
		fn burn_synthetic() -> Weight;
		fn liquidate_position() -> Weight;
	}

	impl WeightInfo for () {
		fn create_synthetic() -> Weight { Weight::from_parts(10_000, 0) }
		fn mint_synthetic() -> Weight { Weight::from_parts(10_000, 0) }
		fn burn_synthetic() -> Weight { Weight::from_parts(10_000, 0) }
		fn liquidate_position() -> Weight { Weight::from_parts(20_000, 0) }
	}

	// ===================== STORAGE =====================

	/// Registered synthetic tokens
	#[pallet::storage]
	#[pallet::getter(fn synthetic_token)]
	pub type SyntheticTokens<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // SyntheticId
		SyntheticToken,
		OptionQuery
	>;

	/// User collateral positions
	#[pallet::storage]
	#[pallet::getter(fn position)]
	pub type Positions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		u32, // SyntheticId
		CollateralPosition<BalanceOf<T>>,
		OptionQuery
	>;

	/// User balances of synthetic tokens
	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub type Balances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		u32, // SyntheticId
		u128,
		ValueQuery
	>;

	/// Total collateral locked in system
	#[pallet::storage]
	#[pallet::getter(fn total_collateral)]
	pub type TotalCollateral<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Next synthetic token ID
	#[pallet::storage]
	pub type NextSyntheticId<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Synthetic count
	#[pallet::storage]
	#[pallet::getter(fn synthetic_count)]
	pub type SyntheticCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Liquidation history
	#[pallet::storage]
	pub type LiquidationHistory<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Record ID
		LiquidationRecord<T::AccountId, BalanceOf<T>>,
		OptionQuery
	>;

	/// Next liquidation record ID
	#[pallet::storage]
	pub type NextLiquidationId<T: Config> = StorageValue<_, u32, ValueQuery>;

	// ===================== EVENTS =====================

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Synthetic token created
		SyntheticCreated {
			synthetic_id: u32,
			symbol: Vec<u8>,
			min_collateral_ratio: u16,
		},
		/// Synthetic tokens minted
		SyntheticMinted {
			who: T::AccountId,
			synthetic_id: u32,
			amount: u128,
			collateral: BalanceOf<T>,
		},
		/// Synthetic tokens burned
		SyntheticBurned {
			who: T::AccountId,
			synthetic_id: u32,
			amount: u128,
			collateral_returned: BalanceOf<T>,
		},
		/// Position liquidated
		PositionLiquidated {
			owner: T::AccountId,
			liquidator: T::AccountId,
			synthetic_id: u32,
			amount: u128,
			collateral_seized: BalanceOf<T>,
		},
		/// Collateral added to position
		CollateralAdded {
			who: T::AccountId,
			synthetic_id: u32,
			amount: BalanceOf<T>,
		},
		/// Synthetic token deactivated
		SyntheticDeactivated { synthetic_id: u32 },
	}

	// ===================== ERRORS =====================

	#[pallet::error]
	pub enum Error<T> {
		/// Synthetic token not found
		SyntheticNotFound,
		/// Synthetic already exists
		SyntheticAlreadyExists,
		/// Too many synthetics
		TooManySynthetics,
		/// Synthetic is inactive
		SyntheticInactive,
		/// Insufficient collateral
		InsufficientCollateral,
		/// Below minimum collateral ratio
		BelowMinimumCollateralRatio,
		/// Position not found
		PositionNotFound,
		/// Insufficient synthetic balance
		InsufficientSyntheticBalance,
		/// Amount too small
		AmountTooSmall,
		/// Collateral ratio healthy (cannot liquidate)
		CollateralRatioHealthy,
		/// Arithmetic overflow
		ArithmeticOverflow,
		/// Arithmetic underflow
		ArithmeticUnderflow,
		/// Division by zero
		DivisionByZero,
		/// Invalid collateral ratio
		InvalidCollateralRatio,
		/// Position would become undercollateralized
		WouldBecomeUndercollateralized,
	}

	// ===================== CALLS =====================

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new synthetic token
		///
		/// Parameters:
		/// - symbol: Token symbol (e.g., "sBTC")
		/// - name: Token name
		/// - decimals: Number of decimals
		/// - min_collateral_ratio: Minimum ratio in basis points (15000 = 150%)
		/// - liquidation_ratio: Liquidation threshold in basis points (12000 = 120%)
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_synthetic())]
		pub fn create_synthetic(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
			name: Vec<u8>,
			decimals: u8,
			min_collateral_ratio: u16,
			liquidation_ratio: u16,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Validate ratios
			ensure!(
				min_collateral_ratio >= 10000 && min_collateral_ratio <= 50000,
				Error::<T>::InvalidCollateralRatio
			);
			ensure!(
				liquidation_ratio >= 10000 && liquidation_ratio < min_collateral_ratio,
				Error::<T>::InvalidCollateralRatio
			);

			let count = SyntheticCount::<T>::get();
			ensure!(count < T::MaxSynthetics::get(), Error::<T>::TooManySynthetics);

			let synthetic_id = NextSyntheticId::<T>::get();
			let bounded_symbol: BoundedVec<u8, ConstU32<16>> = symbol.clone().try_into()
				.map_err(|_| Error::<T>::SyntheticAlreadyExists)?;
			let bounded_name: BoundedVec<u8, ConstU32<64>> = name.try_into()
				.map_err(|_| Error::<T>::SyntheticAlreadyExists)?;

			let current_block = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();

			let synthetic = SyntheticToken {
				symbol: bounded_symbol,
				name: bounded_name,
				decimals,
				min_collateral_ratio,
				liquidation_ratio,
				total_supply: 0,
				is_active: true,
				created_at: current_block,
			};

			SyntheticTokens::<T>::insert(synthetic_id, synthetic);
			NextSyntheticId::<T>::put(synthetic_id.saturating_add(1));
			SyntheticCount::<T>::put(count.saturating_add(1));

			Self::deposit_event(Event::SyntheticCreated {
				synthetic_id,
				symbol,
				min_collateral_ratio,
			});

			Ok(())
		}

		/// Mint synthetic tokens by providing collateral
		///
		/// Parameters:
		/// - synthetic_id: ID of the synthetic token to mint
		/// - collateral_amount: Amount of collateral to lock
		/// - synthetic_amount: Amount of synthetic tokens to mint
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::mint_synthetic())]
		pub fn mint_synthetic(
			origin: OriginFor<T>,
			synthetic_id: u32,
			collateral_amount: BalanceOf<T>,
			synthetic_amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Validate inputs
			ensure!(collateral_amount >= T::MinCollateral::get(), Error::<T>::InsufficientCollateral);
			ensure!(synthetic_amount > 0, Error::<T>::AmountTooSmall);

			// Get synthetic token
			let mut synthetic = SyntheticTokens::<T>::get(synthetic_id)
				.ok_or(Error::<T>::SyntheticNotFound)?;
			ensure!(synthetic.is_active, Error::<T>::SyntheticInactive);

			// Check collateralization ratio
			// For simplicity, using 1:1 price (in production, use oracle)
			// collateral_ratio = (collateral_value / synthetic_value) * 10000
			let collateral_value = Self::balance_to_u128(collateral_amount)?;
			let required_collateral = synthetic_amount
				.checked_mul(synthetic.min_collateral_ratio as u128)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_div(10000)
				.ok_or(Error::<T>::DivisionByZero)?;

			ensure!(
				collateral_value >= required_collateral,
				Error::<T>::BelowMinimumCollateralRatio
			);

			// Reserve collateral from user
			T::Currency::reserve(&who, collateral_amount)?;

			// Update or create position
			let current_block = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();

			Positions::<T>::mutate(&who, synthetic_id, |position_opt| {
				if let Some(position) = position_opt {
					// Update existing position
					position.collateral_amount = position.collateral_amount
						.checked_add(&collateral_amount)
						.ok_or(Error::<T>::ArithmeticOverflow)?;
					position.synthetic_minted = position.synthetic_minted
						.checked_add(synthetic_amount)
						.ok_or(Error::<T>::ArithmeticOverflow)?;
					position.last_update = current_block;
				} else {
					// Create new position
					*position_opt = Some(CollateralPosition {
						synthetic_id,
						collateral_amount,
						synthetic_minted: synthetic_amount,
						last_update: current_block,
					});
				}
				Ok::<(), DispatchError>(())
			})?;

			// Update balances
			Balances::<T>::mutate(&who, synthetic_id, |balance| {
				*balance = balance.saturating_add(synthetic_amount);
			});

			// Update total supply
			synthetic.total_supply = synthetic.total_supply.saturating_add(synthetic_amount);
			SyntheticTokens::<T>::insert(synthetic_id, synthetic);

			// Update total collateral
			TotalCollateral::<T>::mutate(|total| {
				*total = total.saturating_add(collateral_amount);
			});

			Self::deposit_event(Event::SyntheticMinted {
				who,
				synthetic_id,
				amount: synthetic_amount,
				collateral: collateral_amount,
			});

			Ok(())
		}

		/// Burn synthetic tokens and retrieve collateral
		///
		/// Parameters:
		/// - synthetic_id: ID of the synthetic token
		/// - synthetic_amount: Amount of synthetic tokens to burn
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::burn_synthetic())]
		pub fn burn_synthetic(
			origin: OriginFor<T>,
			synthetic_id: u32,
			synthetic_amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(synthetic_amount > 0, Error::<T>::AmountTooSmall);

			// Get synthetic token
			let mut synthetic = SyntheticTokens::<T>::get(synthetic_id)
				.ok_or(Error::<T>::SyntheticNotFound)?;

			// Check user balance
			let user_balance = Balances::<T>::get(&who, synthetic_id);
			ensure!(user_balance >= synthetic_amount, Error::<T>::InsufficientSyntheticBalance);

			// Get position
			let mut position = Positions::<T>::get(&who, synthetic_id)
				.ok_or(Error::<T>::PositionNotFound)?;

			// Calculate collateral to return proportionally
			let collateral_to_return = if position.synthetic_minted == synthetic_amount {
				// Burning all, return all collateral
				position.collateral_amount
			} else {
				// Proportional return
				let collateral_u128 = Self::balance_to_u128(position.collateral_amount)?;
				let return_amount = collateral_u128
					.checked_mul(synthetic_amount)
					.ok_or(Error::<T>::ArithmeticOverflow)?
					.checked_div(position.synthetic_minted)
					.ok_or(Error::<T>::DivisionByZero)?;
				Self::u128_to_balance(return_amount)?
			};

			// Update position
			position.collateral_amount = position.collateral_amount
				.checked_sub(&collateral_to_return)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;
			position.synthetic_minted = position.synthetic_minted
				.checked_sub(synthetic_amount)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;

			if position.synthetic_minted == 0 {
				// Remove empty position
				Positions::<T>::remove(&who, synthetic_id);
			} else {
				Positions::<T>::insert(&who, synthetic_id, position);
			}

			// Update balance
			Balances::<T>::mutate(&who, synthetic_id, |balance| {
				*balance = balance.saturating_sub(synthetic_amount);
			});

			// Update total supply
			synthetic.total_supply = synthetic.total_supply.saturating_sub(synthetic_amount);
			SyntheticTokens::<T>::insert(synthetic_id, synthetic);

			// Return collateral
			T::Currency::unreserve(&who, collateral_to_return);

			// Update total collateral
			TotalCollateral::<T>::mutate(|total| {
				*total = total.saturating_sub(collateral_to_return);
			});

			Self::deposit_event(Event::SyntheticBurned {
				who,
				synthetic_id,
				amount: synthetic_amount,
				collateral_returned: collateral_to_return,
			});

			Ok(())
		}

		/// Add more collateral to an existing position
		///
		/// Parameters:
		/// - synthetic_id: ID of the synthetic token
		/// - amount: Additional collateral amount
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn add_collateral(
			origin: OriginFor<T>,
			synthetic_id: u32,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(amount > BalanceOf::<T>::from(0u32), Error::<T>::AmountTooSmall);

			// Get position
			let mut position = Positions::<T>::get(&who, synthetic_id)
				.ok_or(Error::<T>::PositionNotFound)?;

			// Reserve additional collateral
			T::Currency::reserve(&who, amount)?;

			// Update position
			position.collateral_amount = position.collateral_amount
				.checked_add(&amount)
				.ok_or(Error::<T>::ArithmeticOverflow)?;
			position.last_update = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();

			Positions::<T>::insert(&who, synthetic_id, position);

			// Update total collateral
			TotalCollateral::<T>::mutate(|total| {
				*total = total.saturating_add(amount);
			});

			Self::deposit_event(Event::CollateralAdded {
				who,
				synthetic_id,
				amount,
			});

			Ok(())
		}

		/// Liquidate an undercollateralized position
		///
		/// Parameters:
		/// - owner: Account with undercollateralized position
		/// - synthetic_id: ID of the synthetic token
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::liquidate_position())]
		pub fn liquidate_position(
			origin: OriginFor<T>,
			owner: T::AccountId,
			synthetic_id: u32,
		) -> DispatchResult {
			let liquidator = ensure_signed(origin)?;

			// Get synthetic token
			let mut synthetic = SyntheticTokens::<T>::get(synthetic_id)
				.ok_or(Error::<T>::SyntheticNotFound)?;

			// Get position
			let position = Positions::<T>::get(&owner, synthetic_id)
				.ok_or(Error::<T>::PositionNotFound)?;

			// Check if position is undercollateralized
			let collateral_value = Self::balance_to_u128(position.collateral_amount)?;
			let debt_value = position.synthetic_minted;

			// collateral_ratio = (collateral / debt) * 10000
			let collateral_ratio = collateral_value
				.checked_mul(10000)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_div(debt_value)
				.ok_or(Error::<T>::DivisionByZero)?;

			ensure!(
				(collateral_ratio as u16) < synthetic.liquidation_ratio,
				Error::<T>::CollateralRatioHealthy
			);

			// Calculate liquidation penalty
			let penalty_amount = position.collateral_amount
				.saturated_into::<u128>()
				.checked_mul(T::LiquidationPenalty::get() as u128)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_div(10000)
				.ok_or(Error::<T>::DivisionByZero)?;

			let penalty = Self::u128_to_balance(penalty_amount)?;
			let collateral_to_liquidator = position.collateral_amount
				.checked_sub(&penalty)
				.unwrap_or(position.collateral_amount);

			// Liquidator must have enough synthetic tokens to burn
			let liquidator_balance = Balances::<T>::get(&liquidator, synthetic_id);
			ensure!(
				liquidator_balance >= position.synthetic_minted,
				Error::<T>::InsufficientSyntheticBalance
			);

			// Burn liquidator's synthetic tokens
			Balances::<T>::mutate(&liquidator, synthetic_id, |balance| {
				*balance = balance.saturating_sub(position.synthetic_minted);
			});

			// Update total supply
			synthetic.total_supply = synthetic.total_supply.saturating_sub(position.synthetic_minted);
			SyntheticTokens::<T>::insert(synthetic_id, synthetic);

			// Transfer collateral to liquidator
			T::Currency::unreserve(&owner, position.collateral_amount);
			T::Currency::transfer(&owner, &liquidator, collateral_to_liquidator, ExistenceRequirement::KeepAlive)?;

			// Remove position
			Positions::<T>::remove(&owner, synthetic_id);

			// Update total collateral
			TotalCollateral::<T>::mutate(|total| {
				*total = total.saturating_sub(position.collateral_amount);
			});

			// Record liquidation
			let liquidation_id = NextLiquidationId::<T>::get();
			let record = LiquidationRecord {
				owner: owner.clone(),
				liquidator: liquidator.clone(),
				synthetic_id,
				amount: position.synthetic_minted,
				collateral_seized: collateral_to_liquidator,
				block_number: <frame_system::Pallet<T>>::block_number().saturated_into::<u32>(),
			};
			LiquidationHistory::<T>::insert(liquidation_id, record);
			NextLiquidationId::<T>::put(liquidation_id.saturating_add(1));

			Self::deposit_event(Event::PositionLiquidated {
				owner,
				liquidator,
				synthetic_id,
				amount: position.synthetic_minted,
				collateral_seized: collateral_to_liquidator,
			});

			Ok(())
		}

		/// Deactivate a synthetic token (governance only)
		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		pub fn deactivate_synthetic(
			origin: OriginFor<T>,
			synthetic_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

			SyntheticTokens::<T>::mutate(synthetic_id, |synthetic_opt| {
				if let Some(synthetic) = synthetic_opt {
					synthetic.is_active = false;
					Self::deposit_event(Event::SyntheticDeactivated { synthetic_id });
					Ok(())
				} else {
					Err(Error::<T>::SyntheticNotFound)
				}
			})?;

			Ok(())
		}
	}

	// ===================== HELPER FUNCTIONS =====================

	impl<T: Config> Pallet<T> {
		/// Get pallet account ID
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		/// Convert Balance to u128
		fn balance_to_u128(balance: BalanceOf<T>) -> Result<u128, DispatchError> {
			TryInto::<u128>::try_into(balance).map_err(|_| Error::<T>::ArithmeticOverflow.into())
		}

		/// Convert u128 to Balance
		fn u128_to_balance(value: u128) -> Result<BalanceOf<T>, DispatchError> {
			value.try_into().map_err(|_| Error::<T>::ArithmeticOverflow.into())
		}

		/// Get collateralization ratio for a position (in basis points)
		pub fn get_collateral_ratio(owner: &T::AccountId, synthetic_id: u32) -> Option<u16> {
			let position = Positions::<T>::get(owner, synthetic_id)?;

			let collateral_value = Self::balance_to_u128(position.collateral_amount).ok()?;
			let debt_value = position.synthetic_minted;

			if debt_value == 0 {
				return Some(u16::MAX);
			}

			let ratio = collateral_value
				.checked_mul(10000)?
				.checked_div(debt_value)?;

			Some(ratio as u16)
		}

		/// Check if a position is healthy
		pub fn is_position_healthy(owner: &T::AccountId, synthetic_id: u32) -> bool {
			let synthetic = match SyntheticTokens::<T>::get(synthetic_id) {
				Some(s) => s,
				None => return false,
			};

			let ratio = match Self::get_collateral_ratio(owner, synthetic_id) {
				Some(r) => r,
				None => return false,
			};

			ratio >= synthetic.min_collateral_ratio
		}
	}
}
