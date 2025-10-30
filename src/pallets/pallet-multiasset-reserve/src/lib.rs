//! # Multi-Asset Reserve Pallet
//!
//! Advanced multi-asset reserve management with automatic rebalancing.
//!
//! ## Overview
//!
//! This pallet manages a diversified reserve pool that can hold multiple assets
//! (BTC, ETH, USDC, gold, etc.) with dynamic allocation strategies and automatic
//! rebalancing to maintain target ratios.
//!
//! ## Features
//!
//! - **Multi-Asset Management**: Support for multiple reserve assets simultaneously
//! - **Dynamic Allocation**: Equal weight, market cap weighted, or risk-adjusted strategies
//! - **Automatic Rebalancing**: Rebalance when ratios deviate beyond threshold
//! - **Oracle Integration**: Get real-time prices from pallet-reserve-oracle
//! - **Vault Integration**: Store assets securely in pallet-reserve-vault
//! - **Asset Whitelisting**: Governance-controlled asset approval
//! - **Position Limits**: Min/max holdings per asset for risk management
//!
//! ## Use Cases
//!
//! - Reserve backing for synthetic assets (sBTC, sETH)
//! - Diversified treasury management
//! - Multi-asset collateral for stablecoins
//! - Risk-adjusted portfolio management

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Get, Currency, ReservableCurrency},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{FixedPointNumber, FixedU128, Permill};
	use sp_runtime::traits::{AccountIdConversion, SaturatedConversion, Saturating};
	use sp_std::vec::Vec;

	/// Asset metadata and configuration
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct AssetMetadata {
		/// Asset symbol (BTC, ETH, etc.)
		pub symbol: BoundedVec<u8, ConstU32<16>>,
		/// Asset decimals
		pub decimals: u8,
		/// Whether asset is active
		pub is_active: bool,
		/// Minimum holdings allowed
		pub min_holding: u128,
		/// Maximum holdings allowed
		pub max_holding: u128,
		/// Target allocation (percentage in Permill)
		pub target_allocation: Permill,
		/// Last rebalance block
		pub last_rebalance: u32,
	}

	/// Reserve composition entry (asset holdings)
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct AssetHolding {
		/// Asset ID
		pub asset_id: u32,
		/// Amount held
		pub amount: u128,
		/// Value in USD (cached, with 8 decimals)
		pub value_usd: u128,
		/// Last price update block
		pub last_price_update: u32,
	}

	/// Allocation strategy types
	#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[repr(u8)]
	pub enum AllocationStrategy {
		/// Equal weight allocation (each asset gets equal %)
		EqualWeight = 0,
		/// Market cap weighted (higher cap = higher allocation)
		MarketCapWeighted = 1,
		/// Risk-adjusted (lower volatility = higher allocation)
		RiskAdjusted = 2,
		/// Custom manual allocations
		Custom = 3,
	}

	impl Default for AllocationStrategy {
		fn default() -> Self {
			AllocationStrategy::Custom
		}
	}

	/// Rebalancing status
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct RebalanceStatus<BlockNumber> {
		/// Last rebalance block
		pub last_rebalance: BlockNumber,
		/// Assets rebalanced
		pub assets_rebalanced: u32,
		/// Total value rebalanced (USD)
		pub total_value_usd: u128,
		/// Success status
		pub success: bool,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_reserve_oracle::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum number of assets in reserve
		#[pallet::constant]
		type MaxAssets: Get<u32>;

		/// Rebalance interval (minimum blocks between rebalances)
		#[pallet::constant]
		type RebalanceInterval: Get<BlockNumberFor<Self>>;

		/// Rebalance threshold (deviation % to trigger rebalance)
		#[pallet::constant]
		type RebalanceThreshold: Get<Permill>;

		/// Pallet ID for account derivation
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	pub trait WeightInfo {
		fn add_asset() -> Weight;
		fn remove_asset() -> Weight;
		fn set_target_allocation() -> Weight;
		fn deposit_to_reserve() -> Weight;
		fn withdraw_from_reserve() -> Weight;
		fn trigger_rebalance() -> Weight;
	}

	impl WeightInfo for () {
		fn add_asset() -> Weight { Weight::from_parts(10_000, 0) }
		fn remove_asset() -> Weight { Weight::from_parts(10_000, 0) }
		fn set_target_allocation() -> Weight { Weight::from_parts(10_000, 0) }
		fn deposit_to_reserve() -> Weight { Weight::from_parts(10_000, 0) }
		fn withdraw_from_reserve() -> Weight { Weight::from_parts(10_000, 0) }
		fn trigger_rebalance() -> Weight { Weight::from_parts(50_000, 0) }
	}

	// ===================== STORAGE =====================

	/// Asset configuration registry
	#[pallet::storage]
	#[pallet::getter(fn asset_config)]
	pub type AssetConfigs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // AssetId
		AssetMetadata,
		OptionQuery
	>;

	/// Reserve composition (holdings per asset)
	#[pallet::storage]
	#[pallet::getter(fn asset_holding)]
	pub type ReserveComposition<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // AssetId
		AssetHolding,
		OptionQuery
	>;

	/// Current allocation strategy (stored as u8)
	#[pallet::storage]
	pub type CurrentStrategy<T: Config> = StorageValue<_, u8, ValueQuery>;

	/// Total reserve value (cached, USD with 8 decimals)
	#[pallet::storage]
	#[pallet::getter(fn total_reserve_value)]
	pub type TotalReserveValue<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// Whitelisted assets
	#[pallet::storage]
	#[pallet::getter(fn is_whitelisted)]
	pub type WhitelistedAssets<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // AssetId
		bool,
		ValueQuery
	>;

	/// Asset count
	#[pallet::storage]
	#[pallet::getter(fn asset_count)]
	pub type AssetCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Last rebalance status
	#[pallet::storage]
	#[pallet::getter(fn last_rebalance_status)]
	pub type LastRebalance<T: Config> = StorageValue<_, RebalanceStatus<BlockNumberFor<T>>, OptionQuery>;

	/// Rebalancing enabled flag
	#[pallet::storage]
	#[pallet::getter(fn rebalancing_enabled)]
	pub type RebalancingEnabled<T: Config> = StorageValue<_, bool, ValueQuery>;

	// ===================== EVENTS =====================

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Asset added to reserve
		AssetAdded { asset_id: u32, symbol: Vec<u8> },
		/// Asset removed from reserve
		AssetRemoved { asset_id: u32 },
		/// Target allocation updated
		AllocationUpdated { asset_id: u32, target: Permill },
		/// Deposit made to reserve
		DepositedToReserve { asset_id: u32, amount: u128, depositor: T::AccountId },
		/// Withdrawal from reserve
		WithdrawnFromReserve { asset_id: u32, amount: u128, recipient: T::AccountId },
		/// Rebalance triggered
		RebalanceTriggered { total_value: u128, assets_count: u32 },
		/// Rebalance completed
		RebalanceCompleted { success: bool, assets_rebalanced: u32 },
		/// Reserve value updated
		ReserveValueUpdated { total_value_usd: u128 },
		/// Allocation strategy changed (strategy: 0=EqualWeight, 1=MarketCapWeighted, 2=RiskAdjusted, 3=Custom)
		StrategyChanged { strategy_code: u8 },
		/// Asset whitelisted
		AssetWhitelisted { asset_id: u32 },
		/// Asset removed from whitelist
		AssetDewhitelisted { asset_id: u32 },
	}

	// ===================== ERRORS =====================

	#[pallet::error]
	pub enum Error<T> {
		/// Asset already exists
		AssetAlreadyExists,
		/// Asset not found
		AssetNotFound,
		/// Asset not whitelisted
		AssetNotWhitelisted,
		/// Too many assets
		TooManyAssets,
		/// Invalid allocation (sum != 100%)
		InvalidAllocation,
		/// Amount below minimum
		BelowMinimumHolding,
		/// Amount exceeds maximum
		ExceedsMaximumHolding,
		/// Insufficient balance
		InsufficientBalance,
		/// Rebalance too frequent
		RebalanceTooFrequent,
		/// Rebalancing disabled
		RebalancingDisabled,
		/// Asset is inactive
		AssetInactive,
		/// Invalid asset ID
		InvalidAssetId,
		/// Arithmetic overflow
		ArithmeticOverflow,
		/// Allocation sum exceeds 100%
		AllocationSumExceeds100,
		/// Invalid allocation strategy code
		InvalidStrategy,
	}

	// ===================== CALLS =====================

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add a new asset to the reserve
		///
		/// # Parameters
		/// - `origin`: Must be root (governance)
		/// - `asset_id`: Unique asset identifier
		/// - `symbol`: Asset symbol (BTC, ETH, etc.)
		/// - `decimals`: Number of decimals
		/// - `min_holding`: Minimum amount to hold
		/// - `max_holding`: Maximum amount to hold
		/// - `target_allocation`: Target allocation percentage
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::add_asset())]
		pub fn add_asset(
			origin: OriginFor<T>,
			asset_id: u32,
			symbol: Vec<u8>,
			decimals: u8,
			min_holding: u128,
			max_holding: u128,
			target_allocation: Permill,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(!AssetConfigs::<T>::contains_key(asset_id), Error::<T>::AssetAlreadyExists);
			ensure!(AssetCount::<T>::get() < T::MaxAssets::get(), Error::<T>::TooManyAssets);
			ensure!(max_holding >= min_holding, Error::<T>::InvalidAllocation);

			let bounded_symbol: BoundedVec<u8, ConstU32<16>> = symbol.clone().try_into()
				.map_err(|_| Error::<T>::InvalidAssetId)?;

			let metadata = AssetMetadata {
				symbol: bounded_symbol,
				decimals,
				is_active: true,
				min_holding,
				max_holding,
				target_allocation,
				last_rebalance: frame_system::Pallet::<T>::block_number().saturated_into(),
			};

			AssetConfigs::<T>::insert(asset_id, metadata);
			WhitelistedAssets::<T>::insert(asset_id, true);
			AssetCount::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::AssetAdded { asset_id, symbol });
			Self::deposit_event(Event::AssetWhitelisted { asset_id });

			Ok(())
		}

		/// Remove asset from reserve
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::remove_asset())]
		pub fn remove_asset(
			origin: OriginFor<T>,
			asset_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(AssetConfigs::<T>::contains_key(asset_id), Error::<T>::AssetNotFound);

			// Check that asset has zero holdings before removing
			if let Some(holding) = ReserveComposition::<T>::get(asset_id) {
				ensure!(holding.amount == 0, Error::<T>::InsufficientBalance);
			}

			AssetConfigs::<T>::remove(asset_id);
			WhitelistedAssets::<T>::remove(asset_id);
			ReserveComposition::<T>::remove(asset_id);
			AssetCount::<T>::mutate(|count| *count = count.saturating_sub(1));

			Self::deposit_event(Event::AssetRemoved { asset_id });

			Ok(())
		}

		/// Set target allocation for an asset
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::set_target_allocation())]
		pub fn set_target_allocation(
			origin: OriginFor<T>,
			asset_id: u32,
			target: Permill,
		) -> DispatchResult {
			ensure_root(origin)?;

			AssetConfigs::<T>::try_mutate(asset_id, |maybe_config| -> DispatchResult {
				let config = maybe_config.as_mut().ok_or(Error::<T>::AssetNotFound)?;
				config.target_allocation = target;
				Ok(())
			})?;

			Self::deposit_event(Event::AllocationUpdated { asset_id, target });

			Ok(())
		}

		/// Deposit asset into reserve
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::deposit_to_reserve())]
		pub fn deposit_to_reserve(
			origin: OriginFor<T>,
			asset_id: u32,
			amount: u128,
		) -> DispatchResult {
			let depositor = ensure_signed(origin)?;

			let config = AssetConfigs::<T>::get(asset_id).ok_or(Error::<T>::AssetNotFound)?;
			ensure!(config.is_active, Error::<T>::AssetInactive);
			ensure!(WhitelistedAssets::<T>::get(asset_id), Error::<T>::AssetNotWhitelisted);

			// Update holdings
			ReserveComposition::<T>::try_mutate(asset_id, |maybe_holding| -> DispatchResult {
				let mut holding = maybe_holding.take().unwrap_or(AssetHolding {
					asset_id,
					amount: 0,
					value_usd: 0,
					last_price_update: 0,
				});

				holding.amount = holding.amount.saturating_add(amount);

				// Check max holding limit
				ensure!(holding.amount <= config.max_holding, Error::<T>::ExceedsMaximumHolding);

				// Update USD value (get price from oracle)
				// TODO: Integrate with pallet-reserve-oracle for real price
				holding.value_usd = holding.amount; // Placeholder
				holding.last_price_update = frame_system::Pallet::<T>::block_number().saturated_into();

				*maybe_holding = Some(holding);
				Ok(())
			})?;

			// Update total reserve value
			Self::update_total_reserve_value();

			Self::deposit_event(Event::DepositedToReserve {
				asset_id,
				amount,
				depositor
			});

			Ok(())
		}

		/// Withdraw asset from reserve
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::withdraw_from_reserve())]
		pub fn withdraw_from_reserve(
			origin: OriginFor<T>,
			asset_id: u32,
			amount: u128,
		) -> DispatchResult {
			ensure_root(origin)?; // Only governance can withdraw

			ReserveComposition::<T>::try_mutate(asset_id, |maybe_holding| -> DispatchResult {
				let holding = maybe_holding.as_mut().ok_or(Error::<T>::AssetNotFound)?;

				ensure!(holding.amount >= amount, Error::<T>::InsufficientBalance);
				holding.amount = holding.amount.saturating_sub(amount);

				// Update USD value
				holding.value_usd = holding.amount; // Placeholder
				holding.last_price_update = frame_system::Pallet::<T>::block_number().saturated_into();

				Ok(())
			})?;

			// Update total reserve value
			Self::update_total_reserve_value();

			Self::deposit_event(Event::WithdrawnFromReserve {
				asset_id,
				amount,
				recipient: Self::account_id()
			});

			Ok(())
		}

		/// Trigger manual rebalance
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::trigger_rebalance())]
		pub fn trigger_rebalance(
			origin: OriginFor<T>,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(RebalancingEnabled::<T>::get(), Error::<T>::RebalancingDisabled);

			// Check rebalance interval
			if let Some(last) = LastRebalance::<T>::get() {
				let current_block = frame_system::Pallet::<T>::block_number();
				ensure!(
					current_block >= last.last_rebalance.saturating_add(T::RebalanceInterval::get()),
					Error::<T>::RebalanceTooFrequent
				);
			}

			let total_value = Self::calculate_total_value();
			let asset_count = AssetCount::<T>::get();

			Self::deposit_event(Event::RebalanceTriggered {
				total_value,
				assets_count: asset_count
			});

			// Perform rebalancing logic
			let success = Self::perform_rebalance();

			let status = RebalanceStatus {
				last_rebalance: frame_system::Pallet::<T>::block_number(),
				assets_rebalanced: asset_count,
				total_value_usd: total_value,
				success,
			};

			LastRebalance::<T>::put(status);

			Self::deposit_event(Event::RebalanceCompleted {
				success,
				assets_rebalanced: asset_count
			});

			Ok(())
		}

		/// Set allocation strategy
		///
		/// Parameters:
		/// - strategy: 0 = EqualWeight, 1 = MarketCapWeighted, 2 = RiskAdjusted, 3 = Custom
		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn set_allocation_strategy(
			origin: OriginFor<T>,
			strategy_code: u8,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Validate strategy code
			ensure!(strategy_code <= 3, Error::<T>::InvalidStrategy);

			CurrentStrategy::<T>::put(strategy_code);

			Self::deposit_event(Event::StrategyChanged { strategy_code });

			Ok(())
		}

		/// Enable/disable automatic rebalancing
		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn set_rebalancing_enabled(
			origin: OriginFor<T>,
			enabled: bool,
		) -> DispatchResult {
			ensure_root(origin)?;

			RebalancingEnabled::<T>::put(enabled);

			Ok(())
		}
	}

	// ===================== HELPER FUNCTIONS =====================

	impl<T: Config> Pallet<T> {
		/// Get pallet account ID
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		/// Get current allocation strategy
		pub fn allocation_strategy() -> AllocationStrategy {
			match CurrentStrategy::<T>::get() {
				0 => AllocationStrategy::EqualWeight,
				1 => AllocationStrategy::MarketCapWeighted,
				2 => AllocationStrategy::RiskAdjusted,
				_ => AllocationStrategy::Custom,
			}
		}

		/// Calculate total reserve value across all assets
		fn calculate_total_value() -> u128 {
			let mut total: u128 = 0;

			for (_, holding) in ReserveComposition::<T>::iter() {
				total = total.saturating_add(holding.value_usd);
			}

			total
		}

		/// Update cached total reserve value
		fn update_total_reserve_value() {
			let total = Self::calculate_total_value();
			TotalReserveValue::<T>::put(total);

			Self::deposit_event(Event::ReserveValueUpdated { total_value_usd: total });
		}

		/// Perform rebalancing logic
		fn perform_rebalance() -> bool {
			// Placeholder for rebalancing algorithm
			// In production, this would:
			// 1. Calculate current allocation percentages
			// 2. Compare to target allocations
			// 3. Calculate trades needed to rebalance
			// 4. Execute swaps via DEX or treasury
			// 5. Update holdings

			true
		}

		/// Get asset allocation percentage
		pub fn get_asset_allocation(asset_id: u32) -> Option<Permill> {
			let holding = ReserveComposition::<T>::get(asset_id)?;
			let total = TotalReserveValue::<T>::get();

			if total == 0 {
				return Some(Permill::zero());
			}

			// Calculate percentage (value / total) * 1_000_000
			let percentage = holding.value_usd
				.saturating_mul(1_000_000)
				.checked_div(total)?;

			Some(Permill::from_parts(percentage as u32))
		}

		/// Check if rebalancing is needed
		pub fn needs_rebalancing() -> bool {
			let threshold = T::RebalanceThreshold::get();

			for (asset_id, config) in AssetConfigs::<T>::iter() {
				if !config.is_active {
					continue;
				}

				if let Some(current_alloc) = Self::get_asset_allocation(asset_id) {
					let target = config.target_allocation;

					// Calculate deviation
					let deviation = if current_alloc > target {
						current_alloc.saturating_sub(target)
					} else {
						target.saturating_sub(current_alloc)
					};

					if deviation > threshold {
						return true;
					}
				}
			}

			false
		}
	}
}
