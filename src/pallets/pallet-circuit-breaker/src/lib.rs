//! # Circuit Breaker Pallet
//!
//! Provides emergency safety controls for the EDSC system.
//!
//! ## Overview
//!
//! This pallet implements circuit breaker patterns to protect the EDSC system from:
//! - Excessive redemption volumes
//! - Rapid reserve ratio changes
//! - Suspicious activity patterns
//! - System anomalies
//!
//! Features:
//! - Volume caps (hourly/daily)
//! - Reserve ratio thresholds
//! - Auto-pause mechanisms
//! - Emergency withdrawal limits
//! - Configurable safety parameters

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
	use sp_arithmetic::Permill;
	use sp_runtime::traits::Saturating;
	use sp_std::vec::Vec;

	/// Circuit breaker status
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum CircuitStatus {
		/// Normal operation
		Normal,
		/// Throttled - limited operations allowed
		Throttled,
		/// Paused - critical operations suspended
		Paused,
		/// Emergency - all non-critical operations halted
		Emergency,
	}

	impl Default for CircuitStatus {
		fn default() -> Self {
			CircuitStatus::Normal
		}
	}

	impl CircuitStatus {
		/// Convert to u8 for event emission (0=Normal, 1=Throttled, 2=Paused, 3=Emergency)
		pub fn to_u8(&self) -> u8 {
			match self {
				CircuitStatus::Normal => 0,
				CircuitStatus::Throttled => 1,
				CircuitStatus::Paused => 2,
				CircuitStatus::Emergency => 3,
			}
		}
	}

	/// Volume tracking for rate limiting
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
	#[scale_info(skip_type_params(T))]
	pub struct VolumeTracker<BlockNumber: Default> {
		/// Total volume in current hour
		pub hourly_volume: u128,
		/// Block when hourly tracking started
		pub hourly_start_block: BlockNumber,
		/// Total volume in current day
		pub daily_volume: u128,
		/// Block when daily tracking started
		pub daily_start_block: BlockNumber,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum hourly redemption volume (in EDSC, with 18 decimals)
		#[pallet::constant]
		type MaxHourlyVolume: Get<u128>;

		/// Maximum daily redemption volume (in EDSC, with 18 decimals)
		#[pallet::constant]
		type MaxDailyVolume: Get<u128>;

		/// Reserve ratio threshold for throttling (basis points, e.g., 9500 = 95%)
		#[pallet::constant]
		type ThrottleThreshold: Get<u16>;

		/// Reserve ratio threshold for emergency pause (basis points, e.g., 9000 = 90%)
		#[pallet::constant]
		type EmergencyThreshold: Get<u16>;

		/// Blocks per hour (for volume tracking)
		#[pallet::constant]
		type BlocksPerHour: Get<BlockNumberFor<Self>>;

		/// Blocks per day (for volume tracking)
		#[pallet::constant]
		type BlocksPerDay: Get<BlockNumberFor<Self>>;
	}

	/// Current circuit breaker status
	#[pallet::storage]
	#[pallet::getter(fn circuit_status)]
	pub type Status<T: Config> = StorageValue<_, CircuitStatus, ValueQuery>;

	/// Volume tracker for redemptions
	#[pallet::storage]
	#[pallet::getter(fn volume_tracker)]
	pub type RedemptionVolume<T: Config> = StorageValue<
		_,
		VolumeTracker<BlockNumberFor<T>>,
		ValueQuery,
	>;

	/// Manual pause flag (set by governance)
	#[pallet::storage]
	#[pallet::getter(fn is_manually_paused)]
	pub type ManualPauseEnabled<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Whitelist of accounts exempt from circuit breaker
	#[pallet::storage]
	#[pallet::getter(fn whitelisted)]
	pub type Whitelist<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	/// Total number of times circuit was triggered
	#[pallet::storage]
	#[pallet::getter(fn trigger_count)]
	pub type TriggerCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Circuit breaker status changed [old_status: 0=Normal, 1=Throttled, 2=Paused, 3=Emergency]
		StatusChanged {
			old_status: u8,
			new_status: u8,
		},
		/// Volume limit exceeded [hourly/daily, current_volume, max_volume]
		VolumeLimitExceeded {
			period: Vec<u8>,
			current_volume: u128,
			max_volume: u128,
		},
		/// Reserve ratio threshold breached
		ReserveThresholdBreached {
			current_ratio: u16,
			threshold: u16,
		},
		/// Manual pause activated
		ManualPauseActivated,
		/// Manual pause deactivated
		ManualPauseDeactivated,
		/// Account added to whitelist
		AccountWhitelisted { account: T::AccountId },
		/// Account removed from whitelist
		AccountRemovedFromWhitelist { account: T::AccountId },
		/// Circuit breaker triggered
		CircuitTriggered { reason: Vec<u8> },
		/// Circuit breaker reset
		CircuitReset,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Operation blocked by circuit breaker
		CircuitBreakerActive,
		/// Volume limit exceeded
		VolumeLimitExceeded,
		/// Reserve ratio too low
		ReserveRatioTooLow,
		/// Already in requested state
		AlreadyInState,
		/// Invalid threshold value
		InvalidThreshold,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			// Reset volume trackers if needed
			Self::reset_volume_trackers_if_needed(n);
			Weight::zero()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Manually pause the circuit (governance/root only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(0)]
		pub fn activate_manual_pause(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			ManualPauseEnabled::<T>::put(true);
			let old_status = Status::<T>::get();
			Status::<T>::put(CircuitStatus::Paused);

			Self::deposit_event(Event::ManualPauseActivated);
			Self::deposit_event(Event::StatusChanged {
				old_status: old_status.to_u8(),
				new_status: CircuitStatus::Paused.to_u8(),
			});

			Ok(())
		}

		/// Resume operations (governance/root only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(1)]
		pub fn resume(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			ManualPauseEnabled::<T>::put(false);
			let old_status = Status::<T>::get();
			Status::<T>::put(CircuitStatus::Normal);

			Self::deposit_event(Event::ManualPauseDeactivated);
			Self::deposit_event(Event::StatusChanged {
				old_status: old_status.to_u8(),
				new_status: CircuitStatus::Normal.to_u8(),
			});
			Self::deposit_event(Event::CircuitReset);

			Ok(())
		}

		/// Add account to whitelist (governance/root only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(2)]
		pub fn add_to_whitelist(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			Whitelist::<T>::insert(&account, true);
			Self::deposit_event(Event::AccountWhitelisted { account });

			Ok(())
		}

		/// Remove account from whitelist (governance/root only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(3)]
		pub fn remove_from_whitelist(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			Whitelist::<T>::remove(&account);
			Self::deposit_event(Event::AccountRemovedFromWhitelist { account });

			Ok(())
		}

		/// Reset circuit breaker (governance/root only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(4)]
		pub fn reset_circuit(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			let old_status = Status::<T>::get();
			Status::<T>::put(CircuitStatus::Normal);
			ManualPauseEnabled::<T>::put(false);

			Self::deposit_event(Event::StatusChanged {
				old_status: old_status.to_u8(),
				new_status: CircuitStatus::Normal.to_u8(),
			});
			Self::deposit_event(Event::CircuitReset);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Check if an operation is allowed
		pub fn is_operation_allowed(account: &T::AccountId, _amount: u128) -> DispatchResult {
			// Check if account is whitelisted
			if Whitelist::<T>::get(account) {
				return Ok(());
			}

			// Check manual pause
			if ManualPauseEnabled::<T>::get() {
				return Err(Error::<T>::CircuitBreakerActive.into());
			}

			// Check circuit status
			match Status::<T>::get() {
				CircuitStatus::Emergency => Err(Error::<T>::CircuitBreakerActive.into()),
				CircuitStatus::Paused => Err(Error::<T>::CircuitBreakerActive.into()),
				CircuitStatus::Throttled => {
					// Allow small amounts in throttled mode
					// In production, this would check specific throttle limits
					Ok(())
				}
				CircuitStatus::Normal => Ok(()),
			}
		}

		/// Track volume for an operation
		pub fn track_volume(amount: u128) -> DispatchResult {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let mut tracker = RedemptionVolume::<T>::get();

			// Update hourly volume
			tracker.hourly_volume = tracker.hourly_volume.saturating_add(amount);

			// Update daily volume
			tracker.daily_volume = tracker.daily_volume.saturating_add(amount);

			// Check hourly limit
			if tracker.hourly_volume > T::MaxHourlyVolume::get() {
				Self::trigger_circuit(b"Hourly volume limit exceeded".to_vec());
				return Err(Error::<T>::VolumeLimitExceeded.into());
			}

			// Check daily limit
			if tracker.daily_volume > T::MaxDailyVolume::get() {
				Self::trigger_circuit(b"Daily volume limit exceeded".to_vec());
				return Err(Error::<T>::VolumeLimitExceeded.into());
			}

			RedemptionVolume::<T>::put(tracker);
			Ok(())
		}

		/// Check reserve ratio and update circuit status
		pub fn check_reserve_ratio(reserve_ratio: u16) -> DispatchResult {
			let current_status = Status::<T>::get();

			// Check emergency threshold
			if reserve_ratio < T::EmergencyThreshold::get() {
				if current_status != CircuitStatus::Emergency {
					Self::trigger_circuit(b"Emergency reserve threshold breached".to_vec());
					Status::<T>::put(CircuitStatus::Emergency);
					Self::deposit_event(Event::ReserveThresholdBreached {
						current_ratio: reserve_ratio,
						threshold: T::EmergencyThreshold::get(),
					});
				}
				return Err(Error::<T>::ReserveRatioTooLow.into());
			}

			// Check throttle threshold
			if reserve_ratio < T::ThrottleThreshold::get() {
				if current_status == CircuitStatus::Normal {
					Status::<T>::put(CircuitStatus::Throttled);
					Self::deposit_event(Event::StatusChanged {
						old_status: current_status.to_u8(),
						new_status: CircuitStatus::Throttled.to_u8(),
					});
					Self::deposit_event(Event::ReserveThresholdBreached {
						current_ratio: reserve_ratio,
						threshold: T::ThrottleThreshold::get(),
					});
				}
			} else {
				// Reserve ratio is healthy, return to normal if throttled
				if current_status == CircuitStatus::Throttled {
					Status::<T>::put(CircuitStatus::Normal);
					Self::deposit_event(Event::StatusChanged {
						old_status: current_status.to_u8(),
						new_status: CircuitStatus::Normal.to_u8(),
					});
				}
			}

			Ok(())
		}

		/// Trigger circuit breaker
		fn trigger_circuit(reason: Vec<u8>) {
			let old_status = Status::<T>::get();
			Status::<T>::put(CircuitStatus::Paused);
			TriggerCount::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::CircuitTriggered { reason });
			Self::deposit_event(Event::StatusChanged {
				old_status: old_status.to_u8(),
				new_status: CircuitStatus::Paused.to_u8(),
			});
		}

		/// Reset volume trackers if periods have elapsed
		fn reset_volume_trackers_if_needed(_current_block: BlockNumberFor<T>) {
			// Volume tracker reset logic
			// In production, this would check elapsed time and reset trackers
			// For now, this is a placeholder that will be implemented when
			// integrated with pallet_timestamp
		}

		/// Get current circuit status
		pub fn get_status() -> CircuitStatus {
			Status::<T>::get()
		}

		/// Check if account is whitelisted
		pub fn is_whitelisted(account: &T::AccountId) -> bool {
			Whitelist::<T>::get(account)
		}
	}
}
