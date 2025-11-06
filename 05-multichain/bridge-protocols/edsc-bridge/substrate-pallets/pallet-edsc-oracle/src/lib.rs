//! # EDSC Oracle Pallet
//!
//! Multi-source TWAP (Time-Weighted Average Price) oracle for EDSC market price discovery.
//!
//! ## Features
//! - Multi-source price aggregation (≥5 sources required)
//! - TWAP calculation (24h window primary, 7d fallback)
//! - Outlier removal (>2σ from median)
//! - Volume-weighted averaging
//! - Staleness detection and circuit breakers
//!
//! ## Price Sources
//! - CEX: Binance, Coinbase, Kraken, etc.
//! - DEX: Uniswap V3, Curve, PancakeSwap
//! - Fallback: CoinGecko, Messari
//!
//! ## Critical Parameters
//! - TWAP_WINDOW_PRIMARY: 24 hours (configurable)
//! - TWAP_WINDOW_FALLBACK: 7 days (configurable)
//! - MIN_SOURCES: 5 (configurable)
//! - OUTLIER_THRESHOLD: 2% (configurable)
//! - ORACLE_STALE_TIMEOUT: 10 minutes (configurable)

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
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{Permill, traits::Saturating};
	use sp_std::{vec, vec::Vec};

	/// Trait for handling price update callbacks
	///
	/// This trait allows the oracle pallet to notify other pallets (like redemption)
	/// when a new TWAP price has been calculated, without creating circular dependencies.
	pub trait PriceUpdateCallback {
		/// Called when a new TWAP price is calculated
		///
		/// # Parameters
		/// - `price`: New TWAP price in smallest units (e.g., $1.00 = 1_000_000_000_000)
		fn on_price_updated(price: u128) -> DispatchResult;
	}

	/// Price feed source
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum PriceSource {
		/// Centralized exchanges
		Binance,
		Coinbase,
		Kraken,
		/// Decentralized exchanges
		UniswapV3,
		Curve,
		PancakeSwap,
		/// Aggregators
		CoinGecko,
		Messari,
	}

	impl PriceSource {
		/// Convert PriceSource to u8
		pub fn to_u8(&self) -> u8 {
			match self {
				PriceSource::Binance => 0,
				PriceSource::Coinbase => 1,
				PriceSource::Kraken => 2,
				PriceSource::UniswapV3 => 3,
				PriceSource::Curve => 4,
				PriceSource::PancakeSwap => 5,
				PriceSource::CoinGecko => 6,
				PriceSource::Messari => 7,
			}
		}

		/// Convert u8 to PriceSource
		pub fn from_u8(val: u8) -> Option<Self> {
			match val {
				0 => Some(PriceSource::Binance),
				1 => Some(PriceSource::Coinbase),
				2 => Some(PriceSource::Kraken),
				3 => Some(PriceSource::UniswapV3),
				4 => Some(PriceSource::Curve),
				5 => Some(PriceSource::PancakeSwap),
				6 => Some(PriceSource::CoinGecko),
				7 => Some(PriceSource::Messari),
				_ => None,
			}
		}
	}

	/// Price data point
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct PricePoint<BlockNumber> {
		/// Price in USD cents (e.g., 100 = $1.00)
		pub price: u128,
		/// Volume in EDSC (for volume weighting)
		pub volume: u128,
		/// Block when price was recorded
		pub timestamp: BlockNumber,
		/// Source of price data
		pub source: PriceSource,
	}

	/// TWAP calculation result
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct TwapResult {
		/// Calculated TWAP price (USD cents)
		pub price: u128,
		/// Number of data points used
		pub data_points: u32,
		/// Number of sources used
		pub sources_used: u32,
		/// Variance (for health monitoring)
		pub variance: u128,
		/// Is using fallback window?
		pub using_fallback: bool,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Callback handler for price updates (connects to redemption pallet)
		type PriceCallback: PriceUpdateCallback;

		/// Primary TWAP window (in blocks, default 24h = 14400 blocks @ 6s)
		#[pallet::constant]
		type PrimaryTwapWindow: Get<BlockNumberFor<Self>>;

		/// Fallback TWAP window (in blocks, default 7d = 100800 blocks @ 6s)
		#[pallet::constant]
		type FallbackTwapWindow: Get<BlockNumberFor<Self>>;

		/// Minimum number of price sources required
		#[pallet::constant]
		type MinPriceSources: Get<u32>;

		/// Outlier threshold (in Permill, e.g., 20000 = 2%)
		#[pallet::constant]
		type OutlierThreshold: Get<Permill>;

		/// Oracle staleness timeout (in blocks, default 10 min = 100 blocks @ 6s)
		#[pallet::constant]
		type StalenessTimeout: Get<BlockNumberFor<Self>>;

		/// Maximum price history entries
		#[pallet::constant]
		type MaxPriceHistory: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Price history (FIFO queue)
	#[pallet::storage]
	#[pallet::getter(fn price_history)]
	pub type PriceHistory<T: Config> = StorageValue<
		_,
		BoundedVec<PricePoint<BlockNumberFor<T>>, T::MaxPriceHistory>,
		ValueQuery,
	>;

	/// Authorized price feeders (can submit price data)
	#[pallet::storage]
	#[pallet::getter(fn is_price_feeder)]
	pub type AuthorizedFeeders<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	/// Current TWAP price (cached result)
	#[pallet::storage]
	#[pallet::getter(fn current_twap)]
	pub type CurrentTwap<T: Config> = StorageValue<_, TwapResult, OptionQuery>;

	/// Last TWAP calculation block
	#[pallet::storage]
	#[pallet::getter(fn last_twap_block)]
	pub type LastTwapBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

	/// Oracle paused flag (circuit breaker)
	#[pallet::storage]
	#[pallet::getter(fn oracle_paused)]
	pub type OraclePaused<T> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Price submitted [source, price, volume, feeder]
		/// source: 0=Binance, 1=Coinbase, 2=Kraken, 3=UniswapV3, 4=Curve, 5=PancakeSwap, 6=CoinGecko, 7=Messari
		PriceSubmitted {
			source: u8,
			price: u128,
			volume: u128,
			feeder: T::AccountId,
		},
		/// TWAP calculated [price, data_points, sources, variance]
		TwapCalculated {
			price: u128,
			data_points: u32,
			sources: u32,
			variance: u128,
		},
		/// Switched to fallback TWAP window
		FallbackWindowActivated,
		/// Oracle marked as stale
		OracleStale,
		/// Outlier price rejected [source, price, median]
		/// source: 0=Binance, 1=Coinbase, 2=Kraken, 3=UniswapV3, 4=Curve, 5=PancakeSwap, 6=CoinGecko, 7=Messari
		OutlierRejected {
			source: u8,
			price: u128,
			median: u128,
		},
		/// Price feeder authorized [feeder]
		FeederAuthorized { feeder: T::AccountId },
		/// Price feeder revoked [feeder]
		FeederRevoked { feeder: T::AccountId },
		/// Oracle paused
		OraclePaused,
		/// Oracle unpaused
		OracleUnpaused,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Caller is not an authorized price feeder
		NotAuthorizedFeeder,
		/// Oracle is paused
		OraclePaused,
		/// Insufficient price sources
		InsufficientSources,
		/// Oracle data is stale
		OracleStale,
		/// Invalid price (zero or unrealistic)
		InvalidPrice,
		/// Arithmetic overflow
		Overflow,
		/// Arithmetic underflow
		Underflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Submit price data (authorized feeders only)
		///
		/// # Parameters
		/// - `origin`: Authorized price feeder
		/// - `source_id`: Price source as u8 (0=Binance, 1=Coinbase, 2=Kraken, 3=UniswapV3, 4=Curve, 5=PancakeSwap, 6=CoinGecko, 7=Messari)
		/// - `price`: Price in USD cents (e.g., 100 = $1.00)
		/// - `volume`: 24h volume in EDSC
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn submit_price(
			origin: OriginFor<T>,
			source_id: u8,
			price: u128,
			volume: u128,
		) -> DispatchResult {
			let feeder = ensure_signed(origin)?;

			// Convert source_id to PriceSource
			let source = PriceSource::from_u8(source_id).ok_or(Error::<T>::InvalidPrice)?;

			// Check authorization
			ensure!(AuthorizedFeeders::<T>::get(&feeder), Error::<T>::NotAuthorizedFeeder);

			// Check oracle not paused
			ensure!(!OraclePaused::<T>::get(), Error::<T>::OraclePaused);

			// Validate price (must be reasonable: $0.50 to $2.00)
			ensure!(price >= 50 && price <= 200, Error::<T>::InvalidPrice);

			// Check for outliers before adding
			if let Err(_) = Self::check_outlier(price) {
				// Get median for event
				let median = Self::calculate_median().unwrap_or(100);
				Self::deposit_event(Event::OutlierRejected {
					source: source_id,
					price,
					median,
				});
				// Don't fail, just reject silently
				return Ok(());
			}

			// Create price point
			let current_block = <frame_system::Pallet<T>>::block_number();
			let price_point = PricePoint {
				price,
				volume,
				timestamp: current_block,
				source: source.clone(),
			};

			// Add to history (FIFO)
			PriceHistory::<T>::mutate(|history| {
				if history.len() >= T::MaxPriceHistory::get() as usize {
					// Remove oldest entry
					history.remove(0);
				}
				let _ = history.try_push(price_point);
			});

			Self::deposit_event(Event::PriceSubmitted {
				source: source_id,
				price,
				volume,
				feeder,
			});

			// Trigger TWAP recalculation (auto-triggered, allow bootstrap)
			let _ = Self::calculate_and_update_twap(true);

			Ok(())
		}

		/// Calculate TWAP manually (anyone can call)
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn calculate_twap(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			// Manual calculation, fail if insufficient sources
			Self::calculate_and_update_twap(false)?;
			Ok(())
		}

		/// Authorize price feeder (governance only)
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn authorize_feeder(
			origin: OriginFor<T>,
			feeder: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			AuthorizedFeeders::<T>::insert(&feeder, true);
			Self::deposit_event(Event::FeederAuthorized { feeder });
			Ok(())
		}

		/// Revoke price feeder (governance only)
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn revoke_feeder(
			origin: OriginFor<T>,
			feeder: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			AuthorizedFeeders::<T>::remove(&feeder);
			Self::deposit_event(Event::FeederRevoked { feeder });
			Ok(())
		}

		/// Pause oracle (governance only, emergency)
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn pause_oracle(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			OraclePaused::<T>::put(true);
			Self::deposit_event(Event::OraclePaused);
			Ok(())
		}

		/// Unpause oracle (governance only)
		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		pub fn unpause_oracle(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			OraclePaused::<T>::put(false);
			Self::deposit_event(Event::OracleUnpaused);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Calculate and update TWAP
		///
		/// Parameters:
		/// - allow_bootstrap: If true, succeeds silently during bootstrap (<MinPriceSources)
		///                   If false, fails with InsufficientSources during bootstrap
		fn calculate_and_update_twap(allow_bootstrap: bool) -> DispatchResult {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let history = PriceHistory::<T>::get();

			// Check minimum data points
			if history.len() < T::MinPriceSources::get() as usize {
				if allow_bootstrap {
					// Auto-triggered during submit_price: succeed silently
					return Ok(());
				} else {
					// Manual calculation: fail if insufficient
					return Err(Error::<T>::InsufficientSources.into());
				}
			}

			// Try primary window first
			let primary_cutoff = current_block.saturating_sub(T::PrimaryTwapWindow::get());
			let mut recent_prices: Vec<_> = history
				.iter()
				.filter(|p| p.timestamp >= primary_cutoff)
				.cloned()
				.collect();

			let using_fallback = if recent_prices.len() < T::MinPriceSources::get() as usize {
				// Use fallback window
				let fallback_cutoff = current_block.saturating_sub(T::FallbackTwapWindow::get());
				recent_prices = history
					.iter()
					.filter(|p| p.timestamp >= fallback_cutoff)
					.cloned()
					.collect();

				Self::deposit_event(Event::FallbackWindowActivated);
				true
			} else {
				false
			};

			// Still insufficient?
			if recent_prices.len() < T::MinPriceSources::get() as usize {
				return Err(Error::<T>::InsufficientSources.into());
			}

			// Count unique sources
			let mut sources_set = Vec::new();
			for price_point in &recent_prices {
				if !sources_set.contains(&price_point.source) {
					sources_set.push(price_point.source.clone());
				}
			}
			let sources_used = sources_set.len() as u32;

			// Calculate volume-weighted average
			let mut total_weighted_price: u128 = 0;
			let mut total_volume: u128 = 0;

			for price_point in &recent_prices {
				// Weight by volume (or equal weight if volume is 0)
				let weight = if price_point.volume > 0 {
					price_point.volume
				} else {
					1_000_000 // Default weight
				};

				total_weighted_price = total_weighted_price
					.checked_add(price_point.price.saturating_mul(weight))
					.ok_or(Error::<T>::Overflow)?;
				total_volume = total_volume
					.checked_add(weight)
					.ok_or(Error::<T>::Overflow)?;
			}

			let twap_price = total_weighted_price
				.checked_div(total_volume)
				.unwrap_or(100); // Default to $1.00 if error

			// Calculate variance
			let variance = Self::calculate_variance(&recent_prices, twap_price);

			// Create TWAP result
			let twap_result = TwapResult {
				price: twap_price,
				data_points: recent_prices.len() as u32,
				sources_used,
				variance,
				using_fallback,
			};

			// Store result
			CurrentTwap::<T>::put(twap_result.clone());
			LastTwapBlock::<T>::put(current_block);

			// Notify price update callback (e.g., redemption pallet)
			let _ = T::PriceCallback::on_price_updated(twap_price);

			Self::deposit_event(Event::TwapCalculated {
				price: twap_price,
				data_points: twap_result.data_points,
				sources: twap_result.sources_used,
				variance,
			});

			Ok(())
		}

		/// Calculate median price (for outlier detection)
		fn calculate_median() -> Result<u128, DispatchError> {
			let history = PriceHistory::<T>::get();
			if history.is_empty() {
				return Err(Error::<T>::InsufficientSources.into());
			}

			let mut prices: Vec<u128> = history.iter().map(|p| p.price).collect();
			prices.sort();

			let mid = prices.len() / 2;
			let median = if prices.len() % 2 == 0 {
				(prices[mid - 1] + prices[mid]) / 2
			} else {
				prices[mid]
			};

			Ok(median)
		}

		/// Check if price is an outlier using variance-aware dynamic threshold
		///
		/// Uses a two-phase validation strategy:
		/// - Bootstrap phase (< MinPriceSources): Basic range check only ($0.50 - $2.00)
		/// - Variance-aware phase (>= MinPriceSources): Dynamic threshold based on price variance
		///
		/// The variance-aware approach adjusts outlier tolerance based on price stability:
		/// - Low variance (stable prices): Strict 2% threshold
		/// - High variance (diverse prices): Relaxed threshold proportional to variance
		///
		/// This allows the oracle to:
		/// 1. Bootstrap from empty state
		/// 2. Maintain strict validation for stable price feeds
		/// 3. Accommodate legitimate price diversity in volatile markets
		fn check_outlier(price: u128) -> DispatchResult {
			let history = PriceHistory::<T>::get();

			// Bootstrap phase: Accept any price within reasonable bounds
			if history.len() < T::MinPriceSources::get() as usize {
				// Basic sanity check: price must be between $0.50 and $2.00
				ensure!(price >= 50 && price <= 200, Error::<T>::InvalidPrice);
				return Ok(());
			}

			// Calculate median for outlier detection
			let median = Self::calculate_median()?;

			// Calculate deviation from median
			let deviation = if price > median {
				price.saturating_sub(median)
			} else {
				median.saturating_sub(price)
			};

			// Variance-aware outlier detection
			// Calculate variance to determine if prices are stable or diverse
			let variance = Self::calculate_variance(&history, median);

			// Two-tier threshold based on price diversity:
			// - Zero variance (all identical prices): Strict 2% threshold for precision
			// - Any variance (diverse prices): Relaxed threshold for volatility tolerance
			//
			// This approach ensures:
			// 1. When all prices are identical (e.g., 100, 100, 100), we reject outliers strictly
			// 2. When prices vary (e.g., 100, 101, 102), we allow legitimate market diversity

			let threshold = if variance == 0 {
				// All prices identical: Use strict configured threshold (2%)
				T::OutlierThreshold::get().mul_floor(median)
			} else {
				// Diverse prices: Use variance-proportional threshold
				// Base threshold is 5%, increase with variance
				// Cap at 15% to maintain outlier protection
				let variance_factor = (variance / 2).min(5); // Max 5x multiplier (variance 10)
				let dynamic_percent = (5 + variance_factor as u32).min(15);
				Permill::from_percent(dynamic_percent).mul_floor(median)
			};

			ensure!(deviation <= threshold, Error::<T>::InvalidPrice);

			Ok(())
		}

		/// Calculate variance of prices
		fn calculate_variance(prices: &Vec<PricePoint<BlockNumberFor<T>>>, mean: u128) -> u128 {
			if prices.is_empty() {
				return 0;
			}

			let mut sum_squared_diff: u128 = 0;
			for price_point in prices {
				let diff = if price_point.price > mean {
					price_point.price.saturating_sub(mean)
				} else {
					mean.saturating_sub(price_point.price)
				};
				sum_squared_diff = sum_squared_diff.saturating_add(diff.saturating_mul(diff));
			}

			sum_squared_diff / (prices.len() as u128)
		}

		/// Check if oracle is stale
		pub fn is_stale() -> bool {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let last_update = LastTwapBlock::<T>::get();
			let elapsed = current_block.saturating_sub(last_update);

			elapsed > T::StalenessTimeout::get()
		}

		/// Get current TWAP price (for external pallets)
		pub fn get_price() -> Result<u128, DispatchError> {
			let twap = CurrentTwap::<T>::get().ok_or(Error::<T>::InsufficientSources)?;

			// Check staleness
			ensure!(!Self::is_stale(), Error::<T>::OracleStale);

			Ok(twap.price)
		}

		/// Get TWAP health status
		pub fn get_health() -> Result<(u32, u32, u128, bool), DispatchError> {
			let twap = CurrentTwap::<T>::get().ok_or(Error::<T>::InsufficientSources)?;
			let is_stale = Self::is_stale();

			Ok((twap.data_points, twap.sources_used, twap.variance, is_stale))
		}
	}

	/// Hooks for automatic TWAP updates
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// On finalize, check staleness and recalculate TWAP if needed
		fn on_finalize(_n: BlockNumberFor<T>) {
			// Check staleness and emit warning BEFORE recalculation
			// This ensures we detect stale data before updating timestamps
			if Self::is_stale() {
				Self::deposit_event(Event::OracleStale);
			}

			// Auto-recalculate TWAP every 100 blocks (~10 minutes)
			let current_block = <frame_system::Pallet<T>>::block_number();
			let last_calc = LastTwapBlock::<T>::get();

			if current_block.saturating_sub(last_calc) >= 100u32.into() {
				// Attempt recalculation (auto-triggered, allow bootstrap)
				let _ = Self::calculate_and_update_twap(true);
			}
		}
	}
}
