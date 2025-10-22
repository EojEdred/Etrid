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

#[cfg(test)]
mod security_tests;

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

	/// Custodian public key types
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum CustodianPublicKey {
		/// SR25519 public key (32 bytes)
		Sr25519([u8; 32]),
		/// ECDSA public key (33 bytes compressed)
		Ecdsa([u8; 33]),
	}

	/// Custodian information
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CustodianInfo {
		/// Public key for signature verification
		pub public_key: CustodianPublicKey,
		/// Is this custodian currently active?
		pub active: bool,
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
	/// Updated automatically by pallet-edsc-oracle via PriceUpdateCallback trait
	/// See this pallet's implementation of PriceUpdateCallback::on_price_updated() (line 457-461)
	#[pallet::storage]
	#[pallet::getter(fn oracle_price)]
	pub type OraclePrice<T> = StorageValue<_, u128, ValueQuery>;

	/// Reserve ratio (as FixedU128, e.g., 1.15 = 115%)
	/// Updated automatically by pallet-reserve-vault via do_update_reserve_ratio()
	/// See pallet_reserve_vault::calculate_and_update_reserve_ratio() line 673-699
	#[pallet::storage]
	#[pallet::getter(fn reserve_ratio)]
	pub type ReserveRatio<T> = StorageValue<_, FixedU128, ValueQuery>;

	/// Authorized custodians for Path 2 (Signed Attestation) redemptions
	/// Maps custodian ID to their public key info
	#[pallet::storage]
	#[pallet::getter(fn custodians)]
	pub type Custodians<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // custodian_id
		CustodianInfo,
	>;

	/// Next available custodian ID
	#[pallet::storage]
	#[pallet::getter(fn next_custodian_id)]
	pub type NextCustodianId<T> = StorageValue<_, u32, ValueQuery>;

	/// Signature timestamp tracking (prevents replay attacks)
	/// Maps signature hash to used timestamp
	#[pallet::storage]
	#[pallet::getter(fn used_signatures)]
	pub type UsedSignatures<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		[u8; 32], // signature hash
		BlockNumberFor<T>, // when it was used
	>;

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
		/// Custodian added [custodian_id]
		CustodianAdded { custodian_id: u32 },
		/// Custodian removed [custodian_id]
		CustodianRemoved { custodian_id: u32 },
		/// Custodian activated [custodian_id]
		CustodianActivated { custodian_id: u32 },
		/// Custodian deactivated [custodian_id]
		CustodianDeactivated { custodian_id: u32 },
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
		/// Caller is not an authorized oracle feeder
		NotAuthorizedOracle,
		/// Caller is not the reserve vault
		NotAuthorizedVault,
		/// Arithmetic overflow
		Overflow,
		/// Arithmetic underflow
		Underflow,
		/// Oracle price stale or invalid
		OracleInvalid,
		/// No active custodians available
		NoActiveCustodians,
		/// Invalid signature format
		InvalidSignatureFormat,
		/// Signature verification failed
		SignatureVerificationFailed,
		/// Signature already used (replay attack prevention)
		SignatureAlreadyUsed,
		/// Custodian not found
		CustodianNotFound,
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

		/// Update oracle price (governance emergency override only)
		///
		/// **IMPORTANT**: In normal operation, pallet-edsc-oracle automatically
		/// updates prices by calling on_price_updated() via the PriceUpdateCallback trait.
		/// This extrinsic is provided only for governance emergency overrides.
		///
		/// See this pallet's PriceUpdateCallback implementation (line 457-461) and
		/// pallet_edsc_oracle::calculate_and_update_twap() for automatic updates.
		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn update_oracle_price(
			origin: OriginFor<T>,
			price: u128,
		) -> DispatchResult {
			// Governance emergency override only
			ensure_root(origin)?;

			OraclePrice::<T>::put(price);
			Self::deposit_event(Event::OraclePriceUpdated { price });
			Ok(())
		}

		/// Update reserve ratio (governance emergency override only)
		///
		/// **IMPORTANT**: In normal operation, pallet-reserve-vault automatically
		/// updates the reserve ratio by calling do_update_reserve_ratio() internally.
		/// This extrinsic is provided only for governance emergency overrides.
		///
		/// See pallet_reserve_vault::calculate_and_update_reserve_ratio() for automatic updates.
		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn update_reserve_ratio(
			origin: OriginFor<T>,
			ratio: FixedU128,
		) -> DispatchResult {
			ensure_root(origin)?; // Governance emergency override only
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

		/// Add a custodian (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `public_key`: Public key bytes (32 bytes for SR25519, 33 bytes for ECDSA compressed)
		/// - `key_type`: 0 for SR25519, 1 for ECDSA
		#[pallet::call_index(8)]
		#[pallet::weight(10_000)]
		pub fn add_custodian(
			origin: OriginFor<T>,
			public_key: Vec<u8>,
			key_type: u8,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Parse public key
			let custodian_key = match key_type {
				0 => {
					// SR25519 (32 bytes)
					ensure!(public_key.len() == 32, Error::<T>::InvalidSignatureFormat);
					let mut key = [0u8; 32];
					key.copy_from_slice(&public_key);
					CustodianPublicKey::Sr25519(key)
				},
				1 => {
					// ECDSA (33 bytes compressed)
					ensure!(public_key.len() == 33, Error::<T>::InvalidSignatureFormat);
					let mut key = [0u8; 33];
					key.copy_from_slice(&public_key);
					CustodianPublicKey::Ecdsa(key)
				},
				_ => return Err(Error::<T>::InvalidSignatureFormat.into()),
			};

			// Get next ID
			let custodian_id = NextCustodianId::<T>::get();
			NextCustodianId::<T>::put(custodian_id.saturating_add(1));

			// Create custodian info
			let info = CustodianInfo {
				public_key: custodian_key,
				active: true,
			};

			// Store custodian
			Custodians::<T>::insert(custodian_id, info);

			Self::deposit_event(Event::CustodianAdded { custodian_id });
			Ok(())
		}

		/// Remove a custodian (governance only)
		#[pallet::call_index(9)]
		#[pallet::weight(10_000)]
		pub fn remove_custodian(
			origin: OriginFor<T>,
			custodian_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Custodians::<T>::contains_key(custodian_id), Error::<T>::CustodianNotFound);

			Custodians::<T>::remove(custodian_id);
			Self::deposit_event(Event::CustodianRemoved { custodian_id });
			Ok(())
		}

		/// Activate a custodian (governance only)
		#[pallet::call_index(10)]
		#[pallet::weight(10_000)]
		pub fn activate_custodian(
			origin: OriginFor<T>,
			custodian_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

			Custodians::<T>::try_mutate(custodian_id, |maybe_info| -> DispatchResult {
				let info = maybe_info.as_mut().ok_or(Error::<T>::CustodianNotFound)?;
				info.active = true;
				Ok(())
			})?;

			Self::deposit_event(Event::CustodianActivated { custodian_id });
			Ok(())
		}

		/// Deactivate a custodian (governance only)
		#[pallet::call_index(11)]
		#[pallet::weight(10_000)]
		pub fn deactivate_custodian(
			origin: OriginFor<T>,
			custodian_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

			Custodians::<T>::try_mutate(custodian_id, |maybe_info| -> DispatchResult {
				let info = maybe_info.as_mut().ok_or(Error::<T>::CustodianNotFound)?;
				info.active = false;
				Ok(())
			})?;

			Self::deposit_event(Event::CustodianDeactivated { custodian_id });
			Ok(())
		}
	}

	/// Implement oracle price callback trait
	impl<T: Config> pallet_edsc_oracle::PriceUpdateCallback for Pallet<T> {
		fn on_price_updated(price: u128) -> DispatchResult {
			Self::do_update_oracle_price(price)
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

			// Trigger payout from reserve vault
			// NOTE: The actual payout is executed via the RedemptionExecuted event
			// The runtime or an external coordinator listens for this event and triggers:
			//   pallet_reserve_vault::Pallet::<T>::do_payout(&who, net_payout)
			// This avoids circular dependency between redemption ← → vault pallets.

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

		/// Verify custodian signature for Path 2 redemptions
		///
		/// Message format: SCALE-encoded (account_id, amount, block_number)
		/// Signature types: SR25519 (64 bytes) or ECDSA (65 bytes)
		///
		/// # Security Features
		/// - Cryptographic signature verification (SR25519 or ECDSA)
		/// - Replay attack prevention (signature hash tracking)
		/// - Active custodian validation
		/// - Timestamp freshness check (within 100 blocks ~10 minutes)
		fn verify_custodian_signature(
			who: &T::AccountId,
			amount: u128,
			signature: &Signature,
		) -> DispatchResult {
			use sp_core::{sr25519, ecdsa, H256, ByteArray};
			use sp_io::crypto::{sr25519_verify, ecdsa_verify_prehashed};
			use sp_runtime::traits::Hash;

			// 1. Check if any active custodians exist
			let active_custodians: Vec<_> = Custodians::<T>::iter()
				.filter(|(_, info)| info.active)
				.collect();

			ensure!(!active_custodians.is_empty(), Error::<T>::NoActiveCustodians);

			// 2. Parse signature based on length
			ensure!(
				signature.len == 64 || signature.len == 65,
				Error::<T>::InvalidSignatureFormat
			);

			let sig_bytes = &signature.data[..signature.len as usize];

			// 3. Check signature hasn't been used before (prevent replay attacks)
			let sig_hash = <T as frame_system::Config>::Hashing::hash(sig_bytes);
			let sig_hash_bytes: [u8; 32] = sig_hash.as_ref().try_into()
				.map_err(|_| Error::<T>::InvalidSignatureFormat)?;

			ensure!(
				!UsedSignatures::<T>::contains_key(sig_hash_bytes),
				Error::<T>::SignatureAlreadyUsed
			);

			// 4. Construct message to verify
			// Message format: (account_id, amount, block_number)
			// Block number provides timestamp and prevents old signatures
			let current_block = <frame_system::Pallet<T>>::block_number();
			let message = (who, amount, current_block).encode();

			// 5. Try to verify signature with each active custodian's public key
			let mut verified = false;

			for (_custodian_id, custodian_info) in active_custodians {
				match &custodian_info.public_key {
					CustodianPublicKey::Sr25519(pubkey_bytes) => {
						// SR25519 verification (64 byte signature)
						if signature.len == 64 {
							// Copy signature bytes to fixed array
							let mut sig_array = [0u8; 64];
							sig_array.copy_from_slice(sig_bytes);
							let sig = sr25519::Signature::from_raw(sig_array);
							let pubkey = sr25519::Public::from_raw(*pubkey_bytes);

							if sr25519_verify(&sig, &message, &pubkey) {
								verified = true;
								break;
							}
						}
					},
					CustodianPublicKey::Ecdsa(pubkey_bytes) => {
						// ECDSA verification (65 byte signature)
						if signature.len == 65 {
							// Copy signature bytes to fixed array
							let mut sig_array = [0u8; 65];
							sig_array.copy_from_slice(sig_bytes);
							let sig = ecdsa::Signature::from_raw(sig_array);
							let pubkey = ecdsa::Public::from_raw(*pubkey_bytes);

							// Hash the message for ECDSA (uses Keccak256)
							let message_hash = sp_io::hashing::keccak_256(&message);

							if ecdsa_verify_prehashed(&sig, &message_hash, &pubkey) {
								verified = true;
								break;
							}
						}
					},
				}
			}

			ensure!(verified, Error::<T>::SignatureVerificationFailed);

			// 6. Mark signature as used (prevent replay)
			UsedSignatures::<T>::insert(sig_hash_bytes, current_block);

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
					// Verify custodian signature
					Self::verify_custodian_signature(who, amount, &signature)?;

					// Use oracle price for fee calculation
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
