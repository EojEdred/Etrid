//! # EDSC Redemption Pallet
//!
//! 3-path redemption engine with dynamic fees and circuit breakers.
//! Maintains EDSC peg through arbitrage-resistant redemption.
//!
//! ## Redemption Paths
//! - **Path 1 (SBT)**: Fee-free redemption with on-chain receipt proof
//! - **Path 2 (Attestation)**: Signed off-chain proof, dynamic fee
//! - **Path 3 (TWAP Fallback)**: No proof required, highest fee
//!
//! ## Dynamic Fee Formula
//! `fee = max(MIN_FEE, SAFETY_MULTIPLIER × (1 - market_price))`
//!
//! Example: If EDSC = $0.98
//! - fee = max(0.25%, 1.2 × 0.02) = 2.4%
//! - Effective payout = $1.00 × (1 - 0.024) = $0.976
//!
//! ## Circuit Breakers
//! - Reserve ratio enforcement (pause if < 100%)
//! - Per-path daily limits
//! - Volume caps (hourly/daily)
//! - Redemption queue when throttled

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement},
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{FixedPointNumber, FixedU128, Permill};
	use sp_runtime::traits::{CheckedSub, SaturatedConversion};
	use sp_std::vec::Vec;

	/// Signature wrapper with fixed size
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[codec(dumb_trait_bound)]
	pub struct Signature {
		pub data: [u8; 256],
		pub len: u8, // actual length of data used
	}

	/// Redemption proof types (3 paths)
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[codec(dumb_trait_bound)]
	pub enum RedemptionProof {
		/// Path 1: On-chain SBT receipt (NO FEE)
		SBT(u64), // receipt_id
		/// Path 2: Signed attestation from custodian (DYNAMIC FEE)
		SignedAttestation(Signature),
		/// Path 3: TWAP fallback, no proof (HIGHEST FEE)
		FallbackTWAP,
	}

	/// Redemption request (for queue system)
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct RedemptionRequest<AccountId, BlockNumber> {
		pub requester: AccountId,
		pub amount: u128,
		pub proof: RedemptionProof,
		pub requested_at: BlockNumber,
		pub status: RequestStatus,
	}

	/// Request status
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum RequestStatus {
		Pending,
		Processing,
		Completed,
		Failed,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_edsc_token::Config + pallet_edsc_receipts::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Minimum redemption fee (0.25% = 2500 basis points out of 1,000,000)
		#[pallet::constant]
		type MinRedemptionFee: Get<Permill>;

		/// Safety multiplier for dynamic fees (1.2 = 120%)
		#[pallet::constant]
		type SafetyMultiplier: Get<FixedU128>;

		/// Path 1 (SBT) daily limit per wallet (USD cents)
		#[pallet::constant]
		type Path1DailyLimit: Get<u128>;

		/// Path 2 (Attestation) daily limit per wallet (USD cents)
		#[pallet::constant]
		type Path2DailyLimit: Get<u128>;

		/// Path 3 (TWAP) daily limit per wallet (USD cents)
		#[pallet::constant]
		type Path3DailyLimit: Get<u128>;

		/// Hourly redemption cap (% of total supply, in Permill)
		#[pallet::constant]
		type HourlyRedemptionCap: Get<Permill>;

		/// Daily redemption cap (% of total supply, in Permill)
		#[pallet::constant]
		type DailyRedemptionCap: Get<Permill>;

		/// Reserve ratio threshold for throttling (105% = 1.05)
		#[pallet::constant]
		type ThrottleReserveRatio: Get<FixedU128>;

		/// Reserve ratio threshold for emergency pause (100% = 1.00)
		#[pallet::constant]
		type EmergencyReserveRatio: Get<FixedU128>;

		/// Maximum queued redemptions
		#[pallet::constant]
		type MaxQueueSize: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Next redemption request ID
	#[pallet::storage]
	#[pallet::getter(fn next_request_id)]
	pub type NextRequestId<T> = StorageValue<_, u64, ValueQuery>;

	/// Redemption requests (queued or processing)
	#[pallet::storage]
	#[pallet::getter(fn redemption_requests)]
	pub type RedemptionRequests<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // request_id
		RedemptionRequest<T::AccountId, BlockNumberFor<T>>,
	>;

	/// Per-wallet, per-path daily redemption tracking
	/// (AccountId, Path, Day) => Amount redeemed (USD cents)
	#[pallet::storage]
	#[pallet::getter(fn daily_redeemed)]
	pub type DailyRedeemed<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Blake2_128Concat, u8>, // path: 1, 2, or 3
			NMapKey<Blake2_128Concat, u32>, // day number
		),
		u128,
		ValueQuery,
	>;

	/// Hourly redemption volume (hour => amount in EDSC)
	#[pallet::storage]
	#[pallet::getter(fn hourly_volume)]
	pub type HourlyVolume<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

	/// Daily redemption volume (day => amount in EDSC)
	#[pallet::storage]
	#[pallet::getter(fn daily_volume)]
	pub type DailyVolume<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

	/// Redemptions paused flag (circuit breaker)
	#[pallet::storage]
	#[pallet::getter(fn redemptions_paused)]
	pub type RedemptionsPaused<T> = StorageValue<_, bool, ValueQuery>;

	/// Redemptions throttled flag (queue mode)
	#[pallet::storage]
	#[pallet::getter(fn redemptions_throttled)]
	pub type RedemptionsThrottled<T> = StorageValue<_, bool, ValueQuery>;

	/// Current oracle price (USD cents per EDSC, e.g., 100 = $1.00)
	/// TODO: Will be populated by pallet-edsc-oracle
	#[pallet::storage]
	#[pallet::getter(fn oracle_price)]
	pub type OraclePrice<T> = StorageValue<_, u128, ValueQuery>;

	/// Reserve ratio (as FixedU128, e.g., 1.15 = 115%)
	/// TODO: Will be calculated by pallet-reserve-vault
	#[pallet::storage]
	#[pallet::getter(fn reserve_ratio)]
	pub type ReserveRatio<T> = StorageValue<_, FixedU128, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Redemption executed [request_id, account, amount, fee, net_payout, path]
		RedemptionExecuted {
			request_id: u64,
			account: T::AccountId,
			amount: u128,
			fee: u128,
			net_payout: u128,
			path: u8,
		},
		/// Redemption queued [request_id, account, amount]
		RedemptionQueued {
			request_id: u64,
			account: T::AccountId,
			amount: u128,
		},
		/// Redemptions paused (circuit breaker triggered)
		RedemptionsPaused,
		/// Redemptions unpaused
		RedemptionsUnpaused,
		/// Redemptions throttled (queue mode enabled)
		RedemptionsThrottled,
		/// Throttle removed
		ThrottleRemoved,
		/// Oracle price updated [new_price]
		OraclePriceUpdated { price: u128 },
		/// Reserve ratio updated [new_ratio]
		ReserveRatioUpdated { ratio: FixedU128 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Redemptions are currently paused
		RedemptionsPaused,
		/// Invalid redemption proof
		InvalidProof,
		/// Receipt not found or invalid
		InvalidReceipt,
		/// Insufficient EDSC balance
		InsufficientBalance,
		/// Daily limit exceeded for this path
		DailyLimitExceeded,
		/// Hourly volume cap exceeded
		HourlyCapExceeded,
		/// Daily volume cap exceeded
		DailyCapExceeded,
		/// Reserve ratio too low (emergency pause)
		ReserveRatioTooLow,
		/// Queue is full
		QueueFull,
		/// Request not found
		RequestNotFound,
		/// Arithmetic overflow
		Overflow,
		/// Arithmetic underflow
		Underflow,
		/// Oracle price stale or invalid
		OracleInvalid,
	}

	/// Genesis configuration
	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub initial_reserve_ratio: FixedU128,
		pub initial_oracle_price: u128,
		#[serde(skip)]
		pub _phantom: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			ReserveRatio::<T>::put(self.initial_reserve_ratio);
			OraclePrice::<T>::put(self.initial_oracle_price);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Redeem EDSC for USD (3-path system)
		///
		/// # Parameters
		/// - `origin`: Account redeeming EDSC
		/// - `amount`: Amount of EDSC to redeem
		/// - `receipt_id_option`: Optional receipt ID for SBT path (None = TWAP path)
		/// - `attestation_sig_bytes`: Optional attestation signature bytes (if provided, uses attestation path)
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn redeem(
			origin: OriginFor<T>,
			amount: u128,
			receipt_id_option: Option<u64>,
			attestation_sig_bytes: Option<Vec<u8>>,
		) -> DispatchResult {
			// Build proof from parameters
			let proof = if let Some(sig_bytes) = attestation_sig_bytes {
				// Convert to Signature struct
				let mut data = [0u8; 256];
				let len = sig_bytes.len().min(256) as u8;
				data[..len as usize].copy_from_slice(&sig_bytes[..len as usize]);
				let sig = Signature { data, len };
				RedemptionProof::SignedAttestation(sig)
			} else if let Some(receipt_id) = receipt_id_option {
				RedemptionProof::SBT(receipt_id)
			} else {
				RedemptionProof::FallbackTWAP
			};
			let who = ensure_signed(origin)?;

			// Check if redemptions paused
			ensure!(!RedemptionsPaused::<T>::get(), Error::<T>::RedemptionsPaused);

			// Check EDSC balance
			let balance = pallet_edsc_token::Pallet::<T>::balance_of(&who);
			ensure!(balance >= amount, Error::<T>::InsufficientBalance);

			// If throttled, queue the request
			if RedemptionsThrottled::<T>::get() {
				return Self::queue_redemption(&who, amount, proof);
			}

			// Execute redemption immediately
			Self::execute_redemption(&who, amount, proof)?;

			Ok(())
		}

		/// Process queued redemption (anyone can call)
		///
		/// # Parameters
		/// - `origin`: Anyone
		/// - `request_id`: ID of queued request
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn process_queue(
			origin: OriginFor<T>,
			request_id: u64,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let mut request = RedemptionRequests::<T>::get(request_id)
				.ok_or(Error::<T>::RequestNotFound)?;

			// Only process pending requests
			ensure!(request.status == RequestStatus::Pending, Error::<T>::RequestNotFound);

			// Check reserve ratio - if recovered, process
			let reserve_ratio = ReserveRatio::<T>::get();
			if reserve_ratio >= T::ThrottleReserveRatio::get() {
				// Update status
				request.status = RequestStatus::Processing;
				RedemptionRequests::<T>::insert(request_id, &request);

				// Execute redemption
				Self::execute_redemption(&request.requester, request.amount, request.proof.clone())?;

				// Mark completed
				request.status = RequestStatus::Completed;
				RedemptionRequests::<T>::insert(request_id, request);
			}

			Ok(())
		}

		/// Pause redemptions (governance only, emergency circuit breaker)
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn pause_redemptions(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			RedemptionsPaused::<T>::put(true);
			Self::deposit_event(Event::RedemptionsPaused);
			Ok(())
		}

		/// Unpause redemptions (governance only)
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn unpause_redemptions(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			RedemptionsPaused::<T>::put(false);
			Self::deposit_event(Event::RedemptionsUnpaused);
			Ok(())
		}

		/// Enable throttle mode (governance only)
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn enable_throttle(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			RedemptionsThrottled::<T>::put(true);
			Self::deposit_event(Event::RedemptionsThrottled);
			Ok(())
		}

		/// Disable throttle mode (governance only)
		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		pub fn disable_throttle(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			RedemptionsThrottled::<T>::put(false);
			Self::deposit_event(Event::ThrottleRemoved);
			Ok(())
		}

		/// Update oracle price (oracle pallet only)
		/// TODO: This will be called by pallet-edsc-oracle
		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn update_oracle_price(
			origin: OriginFor<T>,
			price: u128,
		) -> DispatchResult {
			ensure_root(origin)?; // TODO: Replace with oracle-only permission
			OraclePrice::<T>::put(price);
			Self::deposit_event(Event::OraclePriceUpdated { price });
			Ok(())
		}

		/// Update reserve ratio (reserve vault pallet only)
		/// TODO: This will be called by pallet-reserve-vault
		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn update_reserve_ratio(
			origin: OriginFor<T>,
			ratio: FixedU128,
		) -> DispatchResult {
			ensure_root(origin)?; // TODO: Replace with vault-only permission
			ReserveRatio::<T>::put(ratio);
			Self::deposit_event(Event::ReserveRatioUpdated { ratio });

			// Check circuit breakers
			if ratio < T::EmergencyReserveRatio::get() {
				// Emergency pause
				RedemptionsPaused::<T>::put(true);
				Self::deposit_event(Event::RedemptionsPaused);
			} else if ratio < T::ThrottleReserveRatio::get() {
				// Enable throttle
				RedemptionsThrottled::<T>::put(true);
				Self::deposit_event(Event::RedemptionsThrottled);
			} else {
				// Normal operation
				RedemptionsThrottled::<T>::put(false);
			}

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Internal helper to update oracle price (called by oracle pallet)
		pub fn do_update_oracle_price(price: u128) -> DispatchResult {
			OraclePrice::<T>::put(price);
			Self::deposit_event(Event::OraclePriceUpdated { price });
			Ok(())
		}

		/// Internal helper to update reserve ratio (called by reserve vault pallet)
		pub fn do_update_reserve_ratio(ratio: FixedU128) -> DispatchResult {
			ReserveRatio::<T>::put(ratio);
			Self::deposit_event(Event::ReserveRatioUpdated { ratio });

			// Check circuit breakers
			if ratio < T::EmergencyReserveRatio::get() {
				// Emergency pause
				RedemptionsPaused::<T>::put(true);
				Self::deposit_event(Event::RedemptionsPaused);
			} else if ratio < T::ThrottleReserveRatio::get() {
				// Enable throttle
				RedemptionsThrottled::<T>::put(true);
				Self::deposit_event(Event::RedemptionsThrottled);
			} else {
				// Normal operation
				RedemptionsPaused::<T>::put(false);
				RedemptionsThrottled::<T>::put(false);
			}

			Ok(())
		}

		/// Execute redemption (internal)
		fn execute_redemption(
			who: &T::AccountId,
			amount: u128,
			proof: RedemptionProof,
		) -> DispatchResult {
			// Determine path and verify proof
			let (path, purchase_price) = Self::verify_proof(who, &proof, amount)?;

			// Check daily limits
			Self::check_daily_limit(who, path, amount)?;

			// Check volume caps
			Self::check_volume_caps(amount)?;

			// Calculate fee
			let fee = Self::calculate_fee(path, purchase_price)?;

			// Calculate net payout
			let fee_amount = fee.mul_floor(amount);
			let net_payout = amount.checked_sub(fee_amount).ok_or(Error::<T>::Underflow)?;

			// Burn EDSC
			pallet_edsc_token::Pallet::<T>::do_burn(who, amount)?;

			// Consume receipt if SBT path
			if let RedemptionProof::SBT(receipt_id) = proof {
				pallet_edsc_receipts::Pallet::<T>::consume_receipt(
					frame_system::RawOrigin::Signed(who.clone()).into(),
					receipt_id,
					amount,
				)?;
			}

			// Update tracking
			Self::update_tracking(who, path, amount)?;

			// TODO: Trigger payout from reserve vault
			// pallet_reserve_vault::Pallet::<T>::payout(who, net_payout)?;

			// Generate request ID for event
			let request_id = NextRequestId::<T>::get();
			NextRequestId::<T>::put(request_id.saturating_add(1));

			Self::deposit_event(Event::RedemptionExecuted {
				request_id,
				account: who.clone(),
				amount,
				fee: fee_amount,
				net_payout,
				path,
			});

			Ok(())
		}

		/// Verify redemption proof and return (path, purchase_price)
		fn verify_proof(
			who: &T::AccountId,
			proof: &RedemptionProof,
			amount: u128,
		) -> Result<(u8, u128), DispatchError> {
			match proof {
				// Path 1: SBT receipt (NO FEE)
				RedemptionProof::SBT(receipt_id) => {
					// Verify receipt exists and is valid
					let is_valid = pallet_edsc_receipts::Pallet::<T>::is_valid_receipt(*receipt_id, who)?;
					ensure!(is_valid, Error::<T>::InvalidReceipt);

					// Get purchase price from receipt
					let purchase_price = pallet_edsc_receipts::Pallet::<T>::get_receipt_price(*receipt_id)?;

					Ok((1, purchase_price))
				},

				// Path 2: Signed attestation (DYNAMIC FEE based on market price)
				RedemptionProof::SignedAttestation(signature) => {
					// TODO: Verify signature from authorized custodian
					// For now, use oracle price
					let market_price = OraclePrice::<T>::get();
					ensure!(market_price > 0, Error::<T>::OracleInvalid);

					Ok((2, market_price))
				},

				// Path 3: TWAP fallback (HIGHEST FEE)
				RedemptionProof::FallbackTWAP => {
					let market_price = OraclePrice::<T>::get();
					ensure!(market_price > 0, Error::<T>::OracleInvalid);

					Ok((3, market_price))
				},
			}
		}

		/// Calculate redemption fee
		/// fee = max(MIN_FEE, SAFETY_MULTIPLIER × (1 - market_price/100))
		fn calculate_fee(path: u8, purchase_price: u128) -> Result<Permill, DispatchError> {
			// Path 1 (SBT): NO FEE
			if path == 1 {
				return Ok(Permill::zero());
			}

			// Get market price from oracle
			let market_price = OraclePrice::<T>::get();
			ensure!(market_price > 0, Error::<T>::OracleInvalid);

			// If market price >= $1.00 (100 cents), use minimum fee
			if market_price >= 100 {
				return Ok(T::MinRedemptionFee::get());
			}

			// Calculate depeg amount: (1.00 - market_price)
			// market_price is in cents, so $1.00 = 100
			let depeg = 100u128.checked_sub(market_price).ok_or(Error::<T>::Underflow)?;

			// Convert to FixedU128 (depeg as fraction)
			let depeg_fraction = FixedU128::saturating_from_rational(depeg, 100);

			// Apply safety multiplier
			let dynamic_fee = T::SafetyMultiplier::get()
				.checked_mul(&depeg_fraction)
				.ok_or(Error::<T>::Overflow)?;

			// Convert to Permill (1 millionth)
			let fee_permill = Permill::from_rational(
				dynamic_fee.into_inner(),
				FixedU128::DIV, // DIV is 10^18
			);

			// Take maximum of MIN_FEE and dynamic fee
			let final_fee = if fee_permill > T::MinRedemptionFee::get() {
				fee_permill
			} else {
				T::MinRedemptionFee::get()
			};

			// Path 3 gets additional penalty (2x the dynamic fee)
			if path == 3 {
				let doubled_ppm = final_fee.deconstruct().saturating_mul(2);
				Ok(Permill::from_parts(doubled_ppm))
			} else {
				Ok(final_fee)
			}
		}

		/// Check per-wallet, per-path daily limits
		fn check_daily_limit(who: &T::AccountId, path: u8, amount: u128) -> DispatchResult {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let blocks_per_day = 14400u32; // ~6 sec blocks = 14400 per day
			let day = (current_block.saturated_into::<u32>()) / blocks_per_day;

			let redeemed_today = DailyRedeemed::<T>::get((who, path, day));

			let limit = match path {
				1 => T::Path1DailyLimit::get(),
				2 => T::Path2DailyLimit::get(),
				3 => T::Path3DailyLimit::get(),
				_ => return Err(Error::<T>::InvalidProof.into()),
			};

			let new_total = redeemed_today.checked_add(amount).ok_or(Error::<T>::Overflow)?;
			ensure!(new_total <= limit, Error::<T>::DailyLimitExceeded);

			Ok(())
		}

		/// Check hourly and daily volume caps
		fn check_volume_caps(amount: u128) -> DispatchResult {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let blocks_per_hour = 600u32; // ~6 sec blocks = 600 per hour
			let blocks_per_day = 14400u32;

			let hour = (current_block.saturated_into::<u32>()) / blocks_per_hour;
			let day = (current_block.saturated_into::<u32>()) / blocks_per_day;

			// Check hourly cap
			let hourly_volume = HourlyVolume::<T>::get(hour);
			let total_supply = pallet_edsc_token::Pallet::<T>::total_supply();
			let hourly_cap = T::HourlyRedemptionCap::get().mul_floor(total_supply);
			let new_hourly = hourly_volume.checked_add(amount).ok_or(Error::<T>::Overflow)?;
			ensure!(new_hourly <= hourly_cap, Error::<T>::HourlyCapExceeded);

			// Check daily cap
			let daily_volume = DailyVolume::<T>::get(day);
			let daily_cap = T::DailyRedemptionCap::get().mul_floor(total_supply);
			let new_daily = daily_volume.checked_add(amount).ok_or(Error::<T>::Overflow)?;
			ensure!(new_daily <= daily_cap, Error::<T>::DailyCapExceeded);

			Ok(())
		}

		/// Update tracking (daily limits and volume caps)
		fn update_tracking(who: &T::AccountId, path: u8, amount: u128) -> DispatchResult {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let blocks_per_hour = 600u32;
			let blocks_per_day = 14400u32;

			let hour = (current_block.saturated_into::<u32>()) / blocks_per_hour;
			let day = (current_block.saturated_into::<u32>()) / blocks_per_day;

			// Update daily per-path limit
			DailyRedeemed::<T>::mutate((who, path, day), |total| {
				*total = total.saturating_add(amount);
			});

			// Update hourly volume
			HourlyVolume::<T>::mutate(hour, |volume| {
				*volume = volume.saturating_add(amount);
			});

			// Update daily volume
			DailyVolume::<T>::mutate(day, |volume| {
				*volume = volume.saturating_add(amount);
			});

			Ok(())
		}

		/// Queue redemption request (when throttled)
		fn queue_redemption(
			who: &T::AccountId,
			amount: u128,
			proof: RedemptionProof,
		) -> DispatchResult {
			let request_id = NextRequestId::<T>::get();

			// Check queue size
			let queue_count = RedemptionRequests::<T>::iter().count();
			ensure!(
				(queue_count as u32) < T::MaxQueueSize::get(),
				Error::<T>::QueueFull
			);

			let request = RedemptionRequest {
				requester: who.clone(),
				amount,
				proof,
				requested_at: <frame_system::Pallet<T>>::block_number(),
				status: RequestStatus::Pending,
			};

			RedemptionRequests::<T>::insert(request_id, request);
			NextRequestId::<T>::put(request_id.saturating_add(1));

			Self::deposit_event(Event::RedemptionQueued {
				request_id,
				account: who.clone(),
				amount,
			});

			Ok(())
		}
	}
}
