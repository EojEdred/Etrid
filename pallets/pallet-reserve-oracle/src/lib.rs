//! # Reserve Oracle Pallet
//!
//! Aggregates reserve data from vault and custodians for the EDSC system.
//!
//! ## Overview
//!
//! This pallet serves as the central oracle for EDSC reserve data on FlareChain.
//! It aggregates:
//! - On-chain collateral from pallet-reserve-vault
//! - Off-chain reserves from pallet-custodian-registry
//! - Total EDSC supply from pallet-edsc-token
//!
//! And provides:
//! - Total reserve value (USD)
//! - Reserve ratio (reserves / supply)
//! - Asset price feeds
//! - Historical reserve data
//! - Threshold alerts
//!
//! This data is then published to PBC-EDSC via checkpoint synchronization.

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
		traits::Get,
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{FixedPointNumber, FixedU128, Permill};
	use sp_runtime::traits::SaturatedConversion;
	use sp_std::vec::Vec;

	/// Reserve snapshot data structure
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct ReserveSnapshot<BlockNumber> {
		/// Block number when snapshot was taken
		pub block_number: BlockNumber,
		/// Total on-chain vault value (USD cents)
		pub vault_value: u128,
		/// Total custodian attested value (USD cents)
		pub custodian_value: u128,
		/// Total reserve value (vault + custodian, USD cents)
		pub total_reserves: u128,
		/// Total EDSC supply (with 18 decimals)
		pub total_supply: u128,
		/// Reserve ratio (basis points, 10000 = 100%)
		pub reserve_ratio: u16,
		/// Timestamp of snapshot
		pub timestamp: u64,
	}

	/// Asset price data
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct AssetPrice {
		/// Asset symbol (ETR, BTC, ETH, etc.)
		pub symbol: BoundedVec<u8, ConstU32<16>>,
		/// Price in USD cents (with 8 decimals: 100_000_000 = $1.00)
		pub price_usd_cents: u128,
		/// Last update block
		pub last_update: u32,
		/// Data source (exchange, oracle, etc.)
		pub source: BoundedVec<u8, ConstU32<32>>,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Update interval for reserve snapshots (in blocks)
		#[pallet::constant]
		type SnapshotInterval: Get<BlockNumberFor<Self>>;

		/// Maximum number of snapshots to store
		#[pallet::constant]
		type MaxSnapshots: Get<u32>;

		/// Optimal reserve ratio minimum (basis points: 11000 = 110%)
		#[pallet::constant]
		type ReserveOptimalMin: Get<u16>;

		/// Optimal reserve ratio maximum (basis points: 13000 = 130%)
		#[pallet::constant]
		type ReserveOptimalMax: Get<u16>;

		/// Throttle threshold (basis points: 10500 = 105%)
		#[pallet::constant]
		type ReserveThrottleThreshold: Get<u16>;

		/// Critical threshold (basis points: 10000 = 100%)
		#[pallet::constant]
		type ReserveCriticalThreshold: Get<u16>;

		/// Maximum price staleness (blocks before price is considered stale)
		#[pallet::constant]
		type MaxPriceStaleness: Get<BlockNumberFor<Self>>;
	}

	/// Latest reserve snapshot
	#[pallet::storage]
	#[pallet::getter(fn latest_snapshot)]
	pub type LatestSnapshot<T: Config> = StorageValue<
		_,
		ReserveSnapshot<BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Historical reserve snapshots
	#[pallet::storage]
	#[pallet::getter(fn snapshots)]
	pub type Snapshots<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		ReserveSnapshot<BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Total number of snapshots created
	#[pallet::storage]
	#[pallet::getter(fn snapshot_count)]
	pub type SnapshotCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Asset price feeds
	#[pallet::storage]
	#[pallet::getter(fn asset_prices)]
	pub type AssetPrices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, ConstU32<16>>, // Asset symbol
		AssetPrice,
		OptionQuery,
	>;

	/// Reserve ratio alert flag
	#[pallet::storage]
	#[pallet::getter(fn alert_active)]
	pub type AlertActive<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Last checkpoint sent to PBC-EDSC
	#[pallet::storage]
	#[pallet::getter(fn last_checkpoint)]
	pub type LastCheckpoint<T: Config> = StorageValue<_, BlockNumberFor<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Reserve snapshot created [block_number, total_reserves, reserve_ratio]
		SnapshotCreated {
			block_number: BlockNumberFor<T>,
			total_reserves: u128,
			reserve_ratio: u16,
		},
		/// Reserve ratio alert triggered [current_ratio, threshold]
		ReserveRatioAlert {
			current_ratio: u16,
			threshold: u16,
			alert_level: u8, // 0=Normal, 1=Warning, 2=Throttle, 3=Critical
		},
		/// Asset price updated [symbol, price, source]
		AssetPriceUpdated {
			symbol: Vec<u8>,
			price_usd_cents: u128,
			source: Vec<u8>,
		},
		/// Checkpoint published to PBC-EDSC
		CheckpointPublished {
			block_number: BlockNumberFor<T>,
			reserve_ratio: u16,
		},
		/// Reserve data aggregated successfully
		ReserveDataAggregated {
			vault_value: u128,
			custodian_value: u128,
			total_value: u128,
		},
		/// Stale price detected
		StalePriceDetected {
			symbol: Vec<u8>,
			last_update: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Maximum snapshots reached
		MaxSnapshotsReached,
		/// Snapshot not found
		SnapshotNotFound,
		/// Invalid price data
		InvalidPriceData,
		/// Price too stale
		PriceTooStale,
		/// Reserve calculation overflow
		ReserveCalculationOverflow,
		/// Invalid reserve ratio
		InvalidReserveRatio,
		/// No supply data available
		NoSupplyData,
		/// Asset not found
		AssetNotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: BlockNumberFor<T>) {
			// Check if we need to create a snapshot
			if Self::should_create_snapshot(n) {
				let _ = Self::create_reserve_snapshot(n);
			}

			// Check for stale prices
			Self::check_price_staleness(n);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Update asset price (oracle/governance only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(0)]
		pub fn update_asset_price(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
			price_usd_cents: u128,
			source: Vec<u8>,
		) -> DispatchResult {
			ensure_root(origin)?;

			let bounded_symbol: BoundedVec<u8, ConstU32<16>> = symbol.clone().try_into()
				.map_err(|_| Error::<T>::InvalidPriceData)?;
			let bounded_source: BoundedVec<u8, ConstU32<32>> = source.clone().try_into()
				.map_err(|_| Error::<T>::InvalidPriceData)?;

			let current_block = <frame_system::Pallet<T>>::block_number();
			let block_u32: u32 = current_block.saturated_into();

			let price_data = AssetPrice {
				symbol: bounded_symbol.clone(),
				price_usd_cents,
				last_update: block_u32,
				source: bounded_source,
			};

			AssetPrices::<T>::insert(&bounded_symbol, price_data);

			Self::deposit_event(Event::AssetPriceUpdated {
				symbol,
				price_usd_cents,
				source,
			});

			Ok(())
		}

		/// Force create reserve snapshot (governance only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(1)]
		pub fn force_snapshot(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			let block_number = <frame_system::Pallet<T>>::block_number();
			Self::create_reserve_snapshot(block_number)?;

			Ok(())
		}

		/// Publish checkpoint to PBC-EDSC (governance/automated)
		#[pallet::weight(10_000)]
		#[pallet::call_index(2)]
		pub fn publish_checkpoint(
			origin: OriginFor<T>,
			block_number: BlockNumberFor<T>,
		) -> DispatchResult {
			ensure_root(origin)?;

			let snapshot = Snapshots::<T>::get(block_number)
				.ok_or(Error::<T>::SnapshotNotFound)?;

			// In production, this would send XCM/DETRP2P message to PBC-EDSC
			// For now, we just record that we published it
			LastCheckpoint::<T>::put(block_number);

			Self::deposit_event(Event::CheckpointPublished {
				block_number,
				reserve_ratio: snapshot.reserve_ratio,
			});

			Ok(())
		}

		/// Clear reserve ratio alert (governance only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(3)]
		pub fn clear_alert(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			AlertActive::<T>::put(false);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Check if a snapshot should be created
		fn should_create_snapshot(n: BlockNumberFor<T>) -> bool {
			let interval = T::SnapshotInterval::get();
			n % interval == 0u32.into()
		}

		/// Create a reserve snapshot
		fn create_reserve_snapshot(block_number: BlockNumberFor<T>) -> DispatchResult {
			// Check max snapshots
			let count = SnapshotCount::<T>::get();
			ensure!(count < T::MaxSnapshots::get(), Error::<T>::MaxSnapshotsReached);

			// Get vault value from reserve vault
			let vault_value = Self::get_vault_total_value();

			// Get custodian attested value from custodian registry
			let custodian_value = Self::get_custodian_total_value();

			// Calculate total reserves
			let total_reserves = vault_value.saturating_add(custodian_value);

			// Get total EDSC supply
			let total_supply = Self::get_total_supply();

			// Calculate reserve ratio (basis points)
			let reserve_ratio = Self::calculate_reserve_ratio(total_reserves, total_supply)?;

			// Get timestamp
			let timestamp = Self::get_timestamp();

			// Create snapshot
			let snapshot = ReserveSnapshot {
				block_number,
				vault_value,
				custodian_value,
				total_reserves,
				total_supply,
				reserve_ratio,
				timestamp,
			};

			// Store snapshot
			Snapshots::<T>::insert(block_number, snapshot.clone());
			LatestSnapshot::<T>::put(snapshot.clone());
			SnapshotCount::<T>::put(count.saturating_add(1));

			// Emit event
			Self::deposit_event(Event::SnapshotCreated {
				block_number,
				total_reserves,
				reserve_ratio,
			});

			Self::deposit_event(Event::ReserveDataAggregated {
				vault_value,
				custodian_value,
				total_value: total_reserves,
			});

			// Check reserve ratio thresholds
			Self::check_reserve_ratio_thresholds(reserve_ratio);

			Ok(())
		}

		/// Get total value from reserve vault
		fn get_vault_total_value() -> u128 {
			// In production, this would query pallet-reserve-vault
			// For now, return placeholder
			// TODO: Implement vault value aggregation
			0u128
		}

		/// Get total attested value from custodian registry
		fn get_custodian_total_value() -> u128 {
			// In production, this would query pallet-custodian-registry
			// For now, return placeholder
			// TODO: Implement custodian value aggregation
			0u128
		}

		/// Get total EDSC supply
		fn get_total_supply() -> u128 {
			// In production, this would query pallet-edsc-token
			// For now, return placeholder
			// TODO: Implement supply query
			50_000_000_000_000_000_000_000_000_000u128 // 50 billion EDSC with 18 decimals
		}

		/// Calculate reserve ratio in basis points
		fn calculate_reserve_ratio(reserves: u128, supply: u128) -> Result<u16, DispatchError> {
			if supply == 0 {
				return Ok(0);
			}

			// Calculate ratio: (reserves / supply) * 10000
			// Convert to basis points (10000 = 100%)
			let ratio = reserves
				.checked_mul(10000)
				.ok_or(Error::<T>::ReserveCalculationOverflow)?
				.checked_div(supply)
				.ok_or(Error::<T>::ReserveCalculationOverflow)?;

			// Clamp to u16 range
			let ratio_u16 = if ratio > u16::MAX as u128 {
				u16::MAX
			} else {
				ratio as u16
			};

			Ok(ratio_u16)
		}

		/// Check reserve ratio against thresholds
		fn check_reserve_ratio_thresholds(ratio: u16) {
			let critical = T::ReserveCriticalThreshold::get();
			let throttle = T::ReserveThrottleThreshold::get();
			let optimal_min = T::ReserveOptimalMin::get();
			let optimal_max = T::ReserveOptimalMax::get();

			// Critical alert
			if ratio < critical {
				AlertActive::<T>::put(true);
				Self::deposit_event(Event::ReserveRatioAlert {
					current_ratio: ratio,
					threshold: critical,
					alert_level: 3, // Critical
				});
			}
			// Throttle alert
			else if ratio < throttle {
				AlertActive::<T>::put(true);
				Self::deposit_event(Event::ReserveRatioAlert {
					current_ratio: ratio,
					threshold: throttle,
					alert_level: 2, // Throttle
				});
			}
			// Below optimal
			else if ratio < optimal_min {
				Self::deposit_event(Event::ReserveRatioAlert {
					current_ratio: ratio,
					threshold: optimal_min,
					alert_level: 1, // Warning
				});
			}
			// Above optimal
			else if ratio > optimal_max {
				Self::deposit_event(Event::ReserveRatioAlert {
					current_ratio: ratio,
					threshold: optimal_max,
					alert_level: 1, // Warning (over-collateralized)
				});
			}
			// Normal
			else {
				AlertActive::<T>::put(false);
			}
		}

		/// Check for stale asset prices
		fn check_price_staleness(current_block: BlockNumberFor<T>) {
			let max_staleness = T::MaxPriceStaleness::get();
			let current_u32: u32 = current_block.saturated_into();

			// Iterate over asset prices and check staleness
			// In production, this would check all stored prices
			// For now, this is a placeholder
		}

		/// Get current timestamp
		fn get_timestamp() -> u64 {
			// In production, this would use pallet_timestamp
			// For now, return placeholder
			0u64
		}

		/// Get asset price in USD cents
		pub fn get_asset_price(symbol: &[u8]) -> Option<u128> {
			let bounded_symbol: BoundedVec<u8, ConstU32<16>> = symbol.to_vec().try_into().ok()?;
			AssetPrices::<T>::get(&bounded_symbol).map(|price| price.price_usd_cents)
		}

		/// Get latest reserve ratio
		pub fn get_reserve_ratio() -> Option<u16> {
			LatestSnapshot::<T>::get().map(|snapshot| snapshot.reserve_ratio)
		}

		/// Get total reserve value
		pub fn get_total_reserves() -> Option<u128> {
			LatestSnapshot::<T>::get().map(|snapshot| snapshot.total_reserves)
		}

		/// Check if reserve ratio is healthy
		pub fn is_reserve_ratio_healthy() -> bool {
			if let Some(ratio) = Self::get_reserve_ratio() {
				ratio >= T::ReserveCriticalThreshold::get()
			} else {
				false
			}
		}
	}
}
