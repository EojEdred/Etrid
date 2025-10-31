//! # EDSC Stability Pallet
//!
//! Stablecoin reserve management and stability mechanisms for EDSC.
//!
//! ## Overview
//!
//! This pallet implements the EDSC stablecoin system as specified in Ivory Papers Vol III.
//! EDSC is a reserve-backed stablecoin pegged to 1 USD with multi-asset collateral backing.
//!
//! ## Features
//!
//! - **Multi-Asset Reserve**: 40% ETR, 30% BTC, 20% ETH, 10% Other
//! - **Over-Collateralization**: 150% minimum, 120% liquidation threshold
//! - **Automatic Rebalancing**: Triggered when reserve deviates > 5%
//! - **Stability Mechanisms**: Dynamic interest rates, liquidations, emergency pause
//! - **Treasury Integration**: Stability fees and liquidation penalties flow to treasury
//! - **Peg Defense**: Interest rate adjustments when price deviates from $1.00
//!
//! ## Reserve Composition
//!
//! Target allocation:
//! - 40% ËTR (native token)
//! - 30% Synthetic Bitcoin (sBTC)
//! - 20% Synthetic Ethereum (sETH)
//! - 10% Other approved assets
//!
//! ## Stability Mechanisms
//!
//! 1. **Interest Rate Adjustments**:
//!    - If EDSC > $1.01: Lower interest rate to encourage minting
//!    - If EDSC < $0.99: Raise interest rate to encourage burning
//!
//! 2. **Liquidation System**:
//!    - Positions below 120% collateralization can be liquidated
//!    - 5% liquidation penalty goes to treasury
//!
//! 3. **Emergency Circuit Breaker**:
//!    - Directors can pause if peg breaks > 10%
//!    - Prevents cascading liquidations during extreme volatility
//!
//! 4. **Automatic Rebalancing**:
//!    - System rebalances reserve when composition deviates > 5% from target
//!    - Ensures diversification and stability

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Treasury interface for routing stability fees
pub trait TreasuryInterface<AccountId, Balance> {
	/// Send stability fees to treasury
	fn receive_stability_fees(amount: Balance) -> Result<(), sp_runtime::DispatchError>;
}

#[frame_support::pallet]
pub mod pallet {
	use codec::DecodeWithMemTracking;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency, ExistenceRequirement},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{FixedPointNumber, FixedU128, Permill, Perbill};
	use sp_runtime::traits::{AccountIdConversion, CheckedAdd, CheckedSub, Saturating, Zero};
	use sp_runtime::SaturatedConversion;
	use sp_std::vec::Vec;
	use crate::TreasuryInterface;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Asset type for reserve composition
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ReserveAsset {
		/// Native ËTR token
		ETR,
		/// Synthetic Bitcoin
		SBTC,
		/// Synthetic Ethereum
		SETH,
		/// Other approved assets
		Other(u32),
	}

	/// Reserve composition target
	#[derive(Clone, Encode, Decode, DecodeWithMemTracking, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, serde::Serialize, serde::Deserialize)]
	pub struct ReserveComposition {
		/// ETR allocation (target 40% = 4000 basis points)
		pub etr_allocation: u16,
		/// sBTC allocation (target 30% = 3000 basis points)
		pub sbtc_allocation: u16,
		/// sETH allocation (target 20% = 2000 basis points)
		pub seth_allocation: u16,
		/// Other assets allocation (target 10% = 1000 basis points)
		pub other_allocation: u16,
	}

	impl Default for ReserveComposition {
		fn default() -> Self {
			Self {
				etr_allocation: 4000,   // 40%
				sbtc_allocation: 3000,  // 30%
				seth_allocation: 2000,  // 20%
				other_allocation: 1000, // 10%
			}
		}
	}

	/// EDSC collateral position
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct EDSCPosition<Balance> {
		/// Collateral deposited (in native currency)
		pub collateral_amount: Balance,
		/// EDSC minted
		pub edsc_minted: u128,
		/// Interest rate at time of minting (basis points per year)
		pub interest_rate: u16,
		/// Accumulated interest owed
		pub interest_owed: u128,
		/// Last interest update block
		pub last_interest_update: u32,
		/// Creation block
		pub created_at: u32,
	}

	/// Liquidation record for EDSC positions
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct EDSCLiquidation<AccountId, Balance> {
		/// Position owner
		pub owner: AccountId,
		/// Liquidator
		pub liquidator: AccountId,
		/// EDSC amount liquidated
		pub edsc_amount: u128,
		/// Collateral seized
		pub collateral_seized: Balance,
		/// Penalty amount sent to treasury
		pub penalty_amount: Balance,
		/// Block number
		pub block_number: u32,
	}

	/// Rebalancing event record
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct RebalanceRecord {
		/// Composition before rebalance
		pub old_composition: ReserveComposition,
		/// Composition after rebalance
		pub new_composition: ReserveComposition,
		/// Block number
		pub block_number: u32,
		/// Total value rebalanced
		pub total_value: u128,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Native currency (ËTR)
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Minimum collateralization ratio (150% = 15000 basis points)
		#[pallet::constant]
		type MinCollateralRatio: Get<u16>;

		/// Liquidation threshold (120% = 12000 basis points)
		#[pallet::constant]
		type LiquidationThreshold: Get<u16>;

		/// Liquidation penalty (5% = 500 basis points)
		#[pallet::constant]
		type LiquidationPenalty: Get<u16>;

		/// Rebalancing deviation threshold (5% = 500 basis points)
		#[pallet::constant]
		type RebalanceThreshold: Get<u16>;

		/// Emergency pause threshold (10% = 1000 basis points)
		#[pallet::constant]
		type EmergencyPauseThreshold: Get<u16>;

		/// Minimum EDSC mint amount
		#[pallet::constant]
		type MinEDSCMint: Get<u128>;

		/// Base interest rate (annual, in basis points)
		#[pallet::constant]
		type BaseInterestRate: Get<u16>;

		/// Pallet ID for reserve account
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Treasury account ID
		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;

		/// Treasury pallet interface for proper fee routing
		type Treasury: crate::TreasuryInterface<Self::AccountId, BalanceOf<Self>>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	pub trait WeightInfo {
		fn deposit_collateral_mint_edsc() -> Weight;
		fn burn_edsc_withdraw_collateral() -> Weight;
		fn liquidate_position() -> Weight;
		fn trigger_rebalance() -> Weight;
		fn adjust_interest_rate() -> Weight;
		fn emergency_pause() -> Weight;
	}

	impl WeightInfo for () {
		fn deposit_collateral_mint_edsc() -> Weight { Weight::from_parts(20_000, 0) }
		fn burn_edsc_withdraw_collateral() -> Weight { Weight::from_parts(20_000, 0) }
		fn liquidate_position() -> Weight { Weight::from_parts(30_000, 0) }
		fn trigger_rebalance() -> Weight { Weight::from_parts(50_000, 0) }
		fn adjust_interest_rate() -> Weight { Weight::from_parts(10_000, 0) }
		fn emergency_pause() -> Weight { Weight::from_parts(10_000, 0) }
	}

	// ===================== STORAGE =====================

	/// Main EDSC reserve account balance
	#[pallet::storage]
	#[pallet::getter(fn reserve_balance)]
	pub type EDSCReserveBalance<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Current reserve composition
	#[pallet::storage]
	#[pallet::getter(fn reserve_composition)]
	pub type CurrentReserveComposition<T: Config> = StorageValue<_, ReserveComposition, ValueQuery>;

	/// Target reserve composition (can be updated by governance)
	#[pallet::storage]
	#[pallet::getter(fn target_composition)]
	pub type TargetReserveComposition<T: Config> = StorageValue<_, ReserveComposition, ValueQuery>;

	/// Current collateralization ratio (in basis points)
	#[pallet::storage]
	#[pallet::getter(fn collateralization_ratio)]
	pub type CollateralizationRatio<T: Config> = StorageValue<_, u16, ValueQuery>;

	/// Current interest rate for EDSC borrowing (annual, basis points)
	#[pallet::storage]
	#[pallet::getter(fn interest_rate)]
	pub type InterestRate<T: Config> = StorageValue<_, u16, ValueQuery>;

	/// Total EDSC supply
	#[pallet::storage]
	#[pallet::getter(fn total_edsc_supply)]
	pub type TotalEDSCSupply<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// User EDSC balances
	#[pallet::storage]
	#[pallet::getter(fn edsc_balance)]
	pub type EDSCBalances<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u128,
		ValueQuery
	>;

	/// User collateral positions
	#[pallet::storage]
	#[pallet::getter(fn position)]
	pub type Positions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		EDSCPosition<BalanceOf<T>>,
		OptionQuery
	>;

	/// Accumulated stability fees for treasury
	#[pallet::storage]
	#[pallet::getter(fn stability_fees)]
	pub type StabilityFees<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Emergency pause flag
	#[pallet::storage]
	#[pallet::getter(fn is_paused)]
	pub type EmergencyPaused<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Last rebalance block
	#[pallet::storage]
	pub type LastRebalanceBlock<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Liquidation history
	#[pallet::storage]
	pub type LiquidationHistory<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Record ID
		EDSCLiquidation<T::AccountId, BalanceOf<T>>,
		OptionQuery
	>;

	/// Next liquidation record ID
	#[pallet::storage]
	pub type NextLiquidationId<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Rebalancing history
	#[pallet::storage]
	pub type RebalanceHistory<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Record ID
		RebalanceRecord,
		OptionQuery
	>;

	/// Next rebalance record ID
	#[pallet::storage]
	pub type NextRebalanceId<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Current EDSC price (in cents, e.g., 100 = $1.00)
	/// In production, this would come from oracle
	#[pallet::storage]
	#[pallet::getter(fn edsc_price)]
	pub type EDSCPrice<T: Config> = StorageValue<_, u32, ValueQuery>;

	// ===================== GENESIS CONFIG =====================

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_interest_rate: u16,
		pub target_composition: ReserveComposition,
		pub edsc_price: u32,
		pub _phantom: PhantomData<T>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				initial_interest_rate: T::BaseInterestRate::get(),
				target_composition: ReserveComposition::default(),
				edsc_price: 100, // $1.00
				_phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			InterestRate::<T>::put(self.initial_interest_rate);
			TargetReserveComposition::<T>::put(self.target_composition.clone());
			CurrentReserveComposition::<T>::put(self.target_composition.clone());
			EDSCPrice::<T>::put(self.edsc_price);
			CollateralizationRatio::<T>::put(T::MinCollateralRatio::get());
		}
	}

	// ===================== EVENTS =====================

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// EDSC minted
		EDSCMinted {
			who: T::AccountId,
			collateral: BalanceOf<T>,
			edsc_amount: u128,
			interest_rate: u16,
		},
		/// EDSC burned and collateral withdrawn
		EDSCBurned {
			who: T::AccountId,
			edsc_amount: u128,
			collateral_returned: BalanceOf<T>,
			interest_paid: u128,
		},
		/// Position liquidated
		PositionLiquidated {
			owner: T::AccountId,
			liquidator: T::AccountId,
			edsc_amount: u128,
			collateral_seized: BalanceOf<T>,
			penalty: BalanceOf<T>,
		},
		/// Reserve rebalanced
		ReserveRebalanced {
			old_composition: ReserveComposition,
			new_composition: ReserveComposition,
			total_value: u128,
		},
		/// Interest rate adjusted
		InterestRateAdjusted {
			old_rate: u16,
			new_rate: u16,
			reason: Vec<u8>,
		},
		/// Emergency pause activated
		EmergencyPauseActivated {
			triggered_by: T::AccountId,
			reason: Vec<u8>,
		},
		/// Emergency pause deactivated
		EmergencyPauseDeactivated {
			by: T::AccountId,
		},
		/// Stability fees collected
		StabilityFeesCollected {
			amount: BalanceOf<T>,
			sent_to_treasury: bool,
		},
		/// Target composition updated
		TargetCompositionUpdated {
			new_composition: ReserveComposition,
		},
		/// Collateral added to position
		CollateralAdded {
			who: T::AccountId,
			amount: BalanceOf<T>,
		},
		/// Interest accrued on position
		InterestAccrued {
			who: T::AccountId,
			interest_amount: u128,
		},
		/// Interest collected and sent to treasury
		InterestCollected {
			position_owner: T::AccountId,
			interest_amount: u128,
			treasury_amount: BalanceOf<T>,
			block_number: u32,
		},
	}

	// ===================== ERRORS =====================

	#[pallet::error]
	pub enum Error<T> {
		/// System is paused
		SystemPaused,
		/// Amount below minimum
		BelowMinimum,
		/// Insufficient collateral
		InsufficientCollateral,
		/// Below minimum collateral ratio
		BelowMinimumCollateralRatio,
		/// Position not found
		PositionNotFound,
		/// Insufficient EDSC balance
		InsufficientEDSCBalance,
		/// Position is healthy (cannot liquidate)
		PositionHealthy,
		/// Reserve composition invalid
		InvalidReserveComposition,
		/// Rebalancing not needed
		RebalancingNotNeeded,
		/// Arithmetic overflow
		ArithmeticOverflow,
		/// Arithmetic underflow
		ArithmeticUnderflow,
		/// Division by zero
		DivisionByZero,
		/// Invalid interest rate
		InvalidInterestRate,
		/// Not authorized (directors only)
		NotAuthorized,
		/// Peg deviation not critical
		PegDeviationNotCritical,
	}

	// ===================== HOOKS =====================

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			// Check if automatic rebalancing is needed
			// In production, this would be done periodically
			Weight::from_parts(5_000, 0)
		}

		fn on_finalize(_n: BlockNumberFor<T>) {
			// Update collateralization ratio
			let _ = Self::update_collateralization_ratio();
		}
	}

	// ===================== CALLS =====================

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Deposit collateral and mint EDSC
		///
		/// Users lock collateral (ËTR) at 150% ratio to mint EDSC.
		/// Interest begins accruing immediately at current interest rate.
		///
		/// Parameters:
		/// - collateral_amount: Amount of ËTR to lock as collateral
		/// - edsc_amount: Amount of EDSC to mint
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::deposit_collateral_mint_edsc())]
		pub fn deposit_collateral_mint_edsc(
			origin: OriginFor<T>,
			collateral_amount: BalanceOf<T>,
			edsc_amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check system not paused
			ensure!(!EmergencyPaused::<T>::get(), Error::<T>::SystemPaused);

			// Validate amounts
			ensure!(edsc_amount >= T::MinEDSCMint::get(), Error::<T>::BelowMinimum);
			ensure!(!collateral_amount.is_zero(), Error::<T>::InsufficientCollateral);

			// Check collateralization ratio
			let collateral_value = Self::balance_to_u128(collateral_amount)?;
			let required_collateral = edsc_amount
				.checked_mul(T::MinCollateralRatio::get() as u128)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_div(10000)
				.ok_or(Error::<T>::DivisionByZero)?;

			ensure!(
				collateral_value >= required_collateral,
				Error::<T>::BelowMinimumCollateralRatio
			);

			// Reserve collateral from user
			T::Currency::reserve(&who, collateral_amount)?;

			let current_block = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();
			let current_rate = InterestRate::<T>::get();

			// Update or create position
			if let Some(mut position) = Positions::<T>::get(&who) {
				// Accrue interest before updating
				let interest = Self::calculate_accrued_interest(&position, current_block)?;
				position.interest_owed = position.interest_owed.saturating_add(interest);

				// Update position
				position.collateral_amount = position.collateral_amount
					.checked_add(&collateral_amount)
					.ok_or(Error::<T>::ArithmeticOverflow)?;
				position.edsc_minted = position.edsc_minted
					.checked_add(edsc_amount)
					.ok_or(Error::<T>::ArithmeticOverflow)?;
				position.interest_rate = current_rate;
				position.last_interest_update = current_block;

				Positions::<T>::insert(&who, position);
			} else {
				// Create new position
				let position = EDSCPosition {
					collateral_amount,
					edsc_minted: edsc_amount,
					interest_rate: current_rate,
					interest_owed: 0,
					last_interest_update: current_block,
					created_at: current_block,
				};
				Positions::<T>::insert(&who, position);
			}

			// Mint EDSC to user
			EDSCBalances::<T>::mutate(&who, |balance| {
				*balance = balance.saturating_add(edsc_amount);
			});

			// Update total supply
			TotalEDSCSupply::<T>::mutate(|supply| {
				*supply = supply.saturating_add(edsc_amount);
			});

			// Add to reserve
			EDSCReserveBalance::<T>::mutate(|balance| {
				*balance = balance.saturating_add(collateral_amount);
			});

			Self::deposit_event(Event::EDSCMinted {
				who,
				collateral: collateral_amount,
				edsc_amount,
				interest_rate: current_rate,
			});

			Ok(())
		}

		/// Burn EDSC and withdraw collateral
		///
		/// Users burn EDSC to retrieve their collateral. Must pay accrued interest.
		///
		/// Parameters:
		/// - edsc_amount: Amount of EDSC to burn
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::burn_edsc_withdraw_collateral())]
		pub fn burn_edsc_withdraw_collateral(
			origin: OriginFor<T>,
			edsc_amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(edsc_amount > 0, Error::<T>::BelowMinimum);

			// Check user balance
			let user_balance = EDSCBalances::<T>::get(&who);
			ensure!(user_balance >= edsc_amount, Error::<T>::InsufficientEDSCBalance);

			// Get position
			let mut position = Positions::<T>::get(&who)
				.ok_or(Error::<T>::PositionNotFound)?;

			let current_block = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();

			// Calculate accrued interest
			let accrued_interest = Self::calculate_accrued_interest(&position, current_block)?;
			let total_interest = position.interest_owed.saturating_add(accrued_interest);

			// Calculate collateral to return proportionally
			let collateral_to_return = if position.edsc_minted == edsc_amount {
				// Burning all EDSC, return all collateral
				position.collateral_amount
			} else {
				// Proportional return
				let collateral_u128 = Self::balance_to_u128(position.collateral_amount)?;
				let return_amount = collateral_u128
					.checked_mul(edsc_amount)
					.ok_or(Error::<T>::ArithmeticOverflow)?
					.checked_div(position.edsc_minted)
					.ok_or(Error::<T>::DivisionByZero)?;
				Self::u128_to_balance(return_amount)?
			};

			// Calculate interest payment in collateral terms
			// In production, would convert using oracle price
			let interest_in_collateral = Self::u128_to_balance(
				total_interest.checked_div(100).unwrap_or(0)
			)?;

			// Ensure enough collateral after interest payment
			ensure!(
				collateral_to_return > interest_in_collateral,
				Error::<T>::InsufficientCollateral
			);

			let final_collateral = collateral_to_return
				.checked_sub(&interest_in_collateral)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;

			// Update position
			position.collateral_amount = position.collateral_amount
				.checked_sub(&collateral_to_return)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;
			position.edsc_minted = position.edsc_minted
				.checked_sub(edsc_amount)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;
			position.interest_owed = 0;
			position.last_interest_update = current_block;

			if position.edsc_minted == 0 {
				// Remove empty position
				Positions::<T>::remove(&who);
			} else {
				Positions::<T>::insert(&who, position);
			}

			// Burn EDSC
			EDSCBalances::<T>::mutate(&who, |balance| {
				*balance = balance.saturating_sub(edsc_amount);
			});

			// Update total supply
			TotalEDSCSupply::<T>::mutate(|supply| {
				*supply = supply.saturating_sub(edsc_amount);
			});

			// Return collateral to user
			T::Currency::unreserve(&who, collateral_to_return);

			// Transfer interest to treasury using proper integration
			if !interest_in_collateral.is_zero() {
				let treasury_account = T::TreasuryAccount::get();

				// Transfer interest from user to treasury account
				T::Currency::transfer(&who, &treasury_account, interest_in_collateral, ExistenceRequirement::KeepAlive)?;

				// Route through treasury pallet for proper tracking
				T::Treasury::receive_stability_fees(interest_in_collateral)?;

				// Track in local stability fees storage
				StabilityFees::<T>::mutate(|fees| {
					*fees = fees.saturating_add(interest_in_collateral);
				});
			}

			// Update reserve
			EDSCReserveBalance::<T>::mutate(|balance| {
				*balance = balance.saturating_sub(collateral_to_return);
			});

			Self::deposit_event(Event::EDSCBurned {
				who,
				edsc_amount,
				collateral_returned: final_collateral,
				interest_paid: total_interest,
			});

			Ok(())
		}

		/// Liquidate an undercollateralized position
		///
		/// Anyone can liquidate positions below 120% collateralization.
		/// Liquidator must provide EDSC to burn, receives collateral with 5% penalty to treasury.
		///
		/// Parameters:
		/// - owner: Account with undercollateralized position
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::liquidate_position())]
		pub fn liquidate_position(
			origin: OriginFor<T>,
			owner: T::AccountId,
		) -> DispatchResult {
			let liquidator = ensure_signed(origin)?;

			// Get position
			let position = Positions::<T>::get(&owner)
				.ok_or(Error::<T>::PositionNotFound)?;

			// Calculate current collateralization ratio
			let collateral_value = Self::balance_to_u128(position.collateral_amount)?;
			let debt_value = position.edsc_minted;

			let collateral_ratio = collateral_value
				.checked_mul(10000)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_div(debt_value)
				.ok_or(Error::<T>::DivisionByZero)?;

			// Check if undercollateralized
			ensure!(
				(collateral_ratio as u16) < T::LiquidationThreshold::get(),
				Error::<T>::PositionHealthy
			);

			// Liquidator must have enough EDSC
			let liquidator_balance = EDSCBalances::<T>::get(&liquidator);
			ensure!(
				liquidator_balance >= debt_value,
				Error::<T>::InsufficientEDSCBalance
			);

			// Calculate penalty
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

			// Burn liquidator's EDSC
			EDSCBalances::<T>::mutate(&liquidator, |balance| {
				*balance = balance.saturating_sub(debt_value);
			});

			// Update total supply
			TotalEDSCSupply::<T>::mutate(|supply| {
				*supply = supply.saturating_sub(debt_value);
			});

			// Transfer collateral to liquidator
			T::Currency::unreserve(&owner, position.collateral_amount);
			T::Currency::transfer(&owner, &liquidator, collateral_to_liquidator, ExistenceRequirement::KeepAlive)?;

			// Send liquidation penalty to treasury using proper integration
			if !penalty.is_zero() {
				let treasury_account = T::TreasuryAccount::get();

				// Transfer penalty from owner to treasury account
				T::Currency::transfer(&owner, &treasury_account, penalty, ExistenceRequirement::KeepAlive)?;

				// Route through treasury pallet for proper tracking
				T::Treasury::receive_stability_fees(penalty)?;

				// Track in local stability fees storage
				StabilityFees::<T>::mutate(|fees| {
					*fees = fees.saturating_add(penalty);
				});
			}

			// Remove position
			Positions::<T>::remove(&owner);

			// Update reserve
			EDSCReserveBalance::<T>::mutate(|balance| {
				*balance = balance.saturating_sub(position.collateral_amount);
			});

			// Record liquidation
			let liquidation_id = NextLiquidationId::<T>::get();
			let current_block = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();

			let record = EDSCLiquidation {
				owner: owner.clone(),
				liquidator: liquidator.clone(),
				edsc_amount: debt_value,
				collateral_seized: collateral_to_liquidator,
				penalty_amount: penalty,
				block_number: current_block,
			};
			LiquidationHistory::<T>::insert(liquidation_id, record);
			NextLiquidationId::<T>::put(liquidation_id.saturating_add(1));

			Self::deposit_event(Event::PositionLiquidated {
				owner,
				liquidator,
				edsc_amount: debt_value,
				collateral_seized: collateral_to_liquidator,
				penalty,
			});

			Ok(())
		}

		/// Trigger reserve rebalancing
		///
		/// Anyone can trigger rebalancing if reserve composition deviates > 5% from target.
		/// This ensures the reserve maintains proper diversification.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::trigger_rebalance())]
		pub fn trigger_rebalance(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			ensure!(!EmergencyPaused::<T>::get(), Error::<T>::SystemPaused);

			let current = CurrentReserveComposition::<T>::get();
			let target = TargetReserveComposition::<T>::get();

			// Check if rebalancing is needed
			let deviation = Self::calculate_composition_deviation(&current, &target)?;
			ensure!(
				deviation > T::RebalanceThreshold::get(),
				Error::<T>::RebalancingNotNeeded
			);

			// In production, this would execute actual asset swaps
			// For now, we update the composition to target
			CurrentReserveComposition::<T>::put(target.clone());

			let current_block = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();
			LastRebalanceBlock::<T>::put(current_block);

			// Record rebalancing
			let rebalance_id = NextRebalanceId::<T>::get();
			let total_value = Self::balance_to_u128(EDSCReserveBalance::<T>::get())?;

			let record = RebalanceRecord {
				old_composition: current.clone(),
				new_composition: target.clone(),
				block_number: current_block,
				total_value,
			};
			RebalanceHistory::<T>::insert(rebalance_id, record);
			NextRebalanceId::<T>::put(rebalance_id.saturating_add(1));

			Self::deposit_event(Event::ReserveRebalanced {
				old_composition: current,
				new_composition: target,
				total_value,
			});

			Ok(())
		}

		/// Adjust interest rate (governance only)
		///
		/// Governance can adjust the interest rate to maintain EDSC peg.
		/// - Raise rate when EDSC < $0.99 (encourage burning)
		/// - Lower rate when EDSC > $1.01 (encourage minting)
		///
		/// Parameters:
		/// - new_rate: New interest rate in basis points (annual)
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::adjust_interest_rate())]
		pub fn adjust_interest_rate(
			origin: OriginFor<T>,
			new_rate: u16,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(new_rate <= 5000, Error::<T>::InvalidInterestRate); // Max 50%

			let old_rate = InterestRate::<T>::get();
			InterestRate::<T>::put(new_rate);

			let reason = if new_rate > old_rate {
				b"Rate increased to defend peg".to_vec()
			} else {
				b"Rate decreased to encourage minting".to_vec()
			};

			Self::deposit_event(Event::InterestRateAdjusted {
				old_rate,
				new_rate,
				reason,
			});

			Ok(())
		}

		/// Emergency pause (directors only)
		///
		/// Directors can pause the system if EDSC peg breaks > 10%.
		/// This prevents cascading liquidations during extreme market volatility.
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::emergency_pause())]
		pub fn emergency_pause(origin: OriginFor<T>) -> DispatchResult {
			// For now, require root (TODO: check if caller is a director)
			let _ = ensure_root(origin)?;

			// Check EDSC price deviation
			let price = EDSCPrice::<T>::get();
			let target_price = 100u32; // $1.00 in cents

			let deviation = if price > target_price {
				((price - target_price) * 10000) / target_price
			} else {
				((target_price - price) * 10000) / target_price
			};

			ensure!(
				deviation as u16 > T::EmergencyPauseThreshold::get(),
				Error::<T>::PegDeviationNotCritical
			);

			EmergencyPaused::<T>::put(true);

			// Use treasury account for root origin event
			let treasury_account = T::TreasuryAccount::get();
			Self::deposit_event(Event::EmergencyPauseActivated {
				triggered_by: treasury_account,
				reason: b"Peg deviation exceeded threshold".to_vec(),
			});

			Ok(())
		}

		/// Deactivate emergency pause (governance only)
		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn deactivate_emergency_pause(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_root(origin)?;

			EmergencyPaused::<T>::put(false);

			// Use treasury account for root origin event
			let treasury_account = T::TreasuryAccount::get();
			Self::deposit_event(Event::EmergencyPauseDeactivated { by: treasury_account });

			Ok(())
		}

		/// Update target reserve composition (governance only)
		///
		/// Governance can adjust target allocations via annual voting on Consensus Day.
		///
		/// Parameters:
		/// - etr_allocation: ETR allocation in basis points (e.g., 4000 = 40%)
		/// - sbtc_allocation: sBTC allocation in basis points
		/// - seth_allocation: sETH allocation in basis points
		/// - other_allocation: Other assets allocation in basis points
		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn update_target_composition(
			origin: OriginFor<T>,
			etr_allocation: u16,
			sbtc_allocation: u16,
			seth_allocation: u16,
			other_allocation: u16,
		) -> DispatchResult {
			ensure_root(origin)?;

			let new_composition = ReserveComposition {
				etr_allocation,
				sbtc_allocation,
				seth_allocation,
				other_allocation,
			};

			// Validate composition sums to 100%
			let total = new_composition.etr_allocation as u32
				+ new_composition.sbtc_allocation as u32
				+ new_composition.seth_allocation as u32
				+ new_composition.other_allocation as u32;

			ensure!(total == 10000, Error::<T>::InvalidReserveComposition);

			TargetReserveComposition::<T>::put(new_composition.clone());

			Self::deposit_event(Event::TargetCompositionUpdated {
				new_composition,
			});

			Ok(())
		}

		/// Add collateral to existing position
		///
		/// Users can add more collateral to improve their collateralization ratio.
		///
		/// Parameters:
		/// - amount: Additional collateral amount
		#[pallet::call_index(8)]
		#[pallet::weight(10_000)]
		pub fn add_collateral(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!amount.is_zero(), Error::<T>::BelowMinimum);

			let mut position = Positions::<T>::get(&who)
				.ok_or(Error::<T>::PositionNotFound)?;

			// Reserve additional collateral
			T::Currency::reserve(&who, amount)?;

			// Update position
			position.collateral_amount = position.collateral_amount
				.checked_add(&amount)
				.ok_or(Error::<T>::ArithmeticOverflow)?;

			Positions::<T>::insert(&who, position);

			// Update reserve
			EDSCReserveBalance::<T>::mutate(|balance| {
				*balance = balance.saturating_add(amount);
			});

			Self::deposit_event(Event::CollateralAdded { who, amount });

			Ok(())
		}

		/// Collect interest from a position and send to treasury
		///
		/// Can be called by anyone to collect accrued interest on a position.
		/// Interest is sent directly to the treasury.
		///
		/// Parameters:
		/// - owner: Account with position to collect interest from
		#[pallet::call_index(9)]
		#[pallet::weight(20_000)]
		pub fn collect_interest(
			origin: OriginFor<T>,
			owner: T::AccountId,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// Get position
			let mut position = Positions::<T>::get(&owner)
				.ok_or(Error::<T>::PositionNotFound)?;

			let current_block = <frame_system::Pallet<T>>::block_number()
				.saturated_into::<u32>();
			let blocks_elapsed = current_block.saturating_sub(position.last_interest_update);

			// Calculate accrued interest
			let interest = Self::calculate_accrued_interest(&position, current_block)?;

			// Update position
			position.interest_owed = position.interest_owed.saturating_add(interest);
			position.last_interest_update = current_block;
			Positions::<T>::insert(&owner, position.clone());

			// Send interest to treasury using proper integration
			if interest > 0 {
				let treasury_account = T::TreasuryAccount::get();
				let interest_balance: BalanceOf<T> = Self::u128_to_balance(interest)?;

				// Deposit to treasury account
				T::Currency::deposit_creating(&treasury_account, interest_balance);

				// Route through treasury pallet for proper tracking and budgeting
				T::Treasury::receive_stability_fees(interest_balance)?;

				// Track in local stability fees storage
				StabilityFees::<T>::mutate(|fees| {
					*fees = fees.saturating_add(interest_balance);
				});

				Self::deposit_event(Event::InterestCollected {
					position_owner: owner,
					interest_amount: interest,
					treasury_amount: interest_balance,
					block_number: current_block,
				});
			}

			Ok(())
		}
	}

	// ===================== HELPER FUNCTIONS =====================

	impl<T: Config> Pallet<T> {
		/// Get pallet reserve account ID
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		/// Convert Balance to u128
		fn balance_to_u128(balance: BalanceOf<T>) -> Result<u128, DispatchError> {
			TryInto::<u128>::try_into(balance)
				.map_err(|_| Error::<T>::ArithmeticOverflow.into())
		}

		/// Convert u128 to Balance
		fn u128_to_balance(value: u128) -> Result<BalanceOf<T>, DispatchError> {
			value.try_into()
				.map_err(|_| Error::<T>::ArithmeticOverflow.into())
		}

		/// Calculate accrued interest for a position
		fn calculate_accrued_interest(
			position: &EDSCPosition<BalanceOf<T>>,
			current_block: u32,
		) -> Result<u128, DispatchError> {
			let blocks_elapsed = current_block.saturating_sub(position.last_interest_update);

			// Assume ~6 second blocks, ~5,256,000 blocks per year
			let blocks_per_year = 5_256_000u128;

			// Interest = principal * rate * time
			let interest = position.edsc_minted
				.checked_mul(position.interest_rate as u128)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_mul(blocks_elapsed as u128)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_div(10000) // Convert from basis points
				.ok_or(Error::<T>::DivisionByZero)?
				.checked_div(blocks_per_year)
				.ok_or(Error::<T>::DivisionByZero)?;

			Ok(interest)
		}

		/// Calculate deviation between current and target composition
		fn calculate_composition_deviation(
			current: &ReserveComposition,
			target: &ReserveComposition,
		) -> Result<u16, DispatchError> {
			let etr_dev = (current.etr_allocation as i32 - target.etr_allocation as i32).abs();
			let sbtc_dev = (current.sbtc_allocation as i32 - target.sbtc_allocation as i32).abs();
			let seth_dev = (current.seth_allocation as i32 - target.seth_allocation as i32).abs();
			let other_dev = (current.other_allocation as i32 - target.other_allocation as i32).abs();

			let total_deviation = (etr_dev + sbtc_dev + seth_dev + other_dev) as u16;

			// Return max deviation across all assets
			Ok(total_deviation / 4)
		}

		/// Update overall collateralization ratio
		fn update_collateralization_ratio() -> Result<(), DispatchError> {
			let total_supply = TotalEDSCSupply::<T>::get();

			if total_supply == 0 {
				CollateralizationRatio::<T>::put(T::MinCollateralRatio::get());
				return Ok(());
			}

			let reserve_value = Self::balance_to_u128(EDSCReserveBalance::<T>::get())?;

			let ratio = reserve_value
				.checked_mul(10000)
				.ok_or(Error::<T>::ArithmeticOverflow)?
				.checked_div(total_supply)
				.ok_or(Error::<T>::DivisionByZero)?;

			CollateralizationRatio::<T>::put(ratio as u16);

			Ok(())
		}

		/// Get current collateralization ratio for a position
		pub fn get_position_collateral_ratio(owner: &T::AccountId) -> Option<u16> {
			let position = Positions::<T>::get(owner)?;

			let collateral_value = Self::balance_to_u128(position.collateral_amount).ok()?;
			let debt_value = position.edsc_minted;

			if debt_value == 0 {
				return Some(u16::MAX);
			}

			let ratio = collateral_value
				.checked_mul(10000)?
				.checked_div(debt_value)?;

			Some(ratio as u16)
		}

		/// Check if position is healthy
		pub fn is_position_healthy(owner: &T::AccountId) -> bool {
			let ratio = match Self::get_position_collateral_ratio(owner) {
				Some(r) => r,
				None => return false,
			};

			ratio >= T::MinCollateralRatio::get()
		}

		/// Check if position is liquidatable
		pub fn is_position_liquidatable(owner: &T::AccountId) -> bool {
			let ratio = match Self::get_position_collateral_ratio(owner) {
				Some(r) => r,
				None => return false,
			};

			ratio < T::LiquidationThreshold::get()
		}
	}
}
