//! # Reserve Vault Pallet
//!
//! Multi-asset collateral vault for EDSC backing with haircut calculations.
//!
//! ## Features
//! - Multi-asset support (BTC, ETH, ÉTR, USDC, etc.)
//! - Risk-adjusted valuations (haircuts)
//! - Reserve ratio calculation and enforcement
//! - Automatic circuit breaker triggers
//! - Governance-controlled withdrawals
//!
//! ## Reserve Ratio Formula
//! ```
//! RR = (Vault Value + Custodian Attested Value) / Total EDSC Supply
//!
//! Targets:
//! - Optimal: 110-130%
//! - Throttle: 105% (slow redemptions)
//! - Critical: 100% (emergency pause)
//! ```
//!
//! ## Haircuts (Risk Adjustments)
//! - ÉTR: 40% (volatile)
//! - BTC: 10%
//! - ETH: 15%
//! - USDC: 5%

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
	use sp_arithmetic::{FixedPointNumber, FixedU128, Permill, traits::{SaturatedConversion, Saturating}};
	use sp_runtime::traits::CheckedSub;
	use sp_std::vec::Vec;

	/// Supported collateral asset types
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[derive(serde::Serialize, serde::Deserialize)]
	#[codec(dumb_trait_bound)]
	pub enum AssetType {
		/// Native token (ÉTR)
		ETR,
		/// Bitcoin
		BTC,
		/// Ethereum
		ETH,
		/// USD Coin
		USDC,
		/// Other stablecoins
		USDT,
		DAI,
	}

	impl AssetType {
		/// Convert from u8 to AssetType
		pub fn from_u8(val: u8) -> Option<Self> {
			match val {
				0 => Some(AssetType::ETR),
				1 => Some(AssetType::BTC),
				2 => Some(AssetType::ETH),
				3 => Some(AssetType::USDC),
				4 => Some(AssetType::USDT),
				5 => Some(AssetType::DAI),
				_ => None,
			}
		}
	}

	/// Vault entry for each asset type
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[codec(dumb_trait_bound)]
	pub struct VaultEntry {
		/// Raw balance of asset
		pub raw_balance: u128,
		/// Haircut percentage (risk adjustment, in Permill)
		pub haircut: Permill,
		/// USD value (in cents, updated by oracle)
		pub usd_value: u128,
		/// Risk-adjusted USD value (after haircut)
		pub adjusted_value: u128,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_edsc_token::Config + pallet_edsc_redemption::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Optimal reserve ratio minimum (1.10 = 110%)
		#[pallet::constant]
		type OptimalReserveMin: Get<FixedU128>;

		/// Optimal reserve ratio maximum (1.30 = 130%)
		#[pallet::constant]
		type OptimalReserveMax: Get<FixedU128>;

		/// Throttle reserve ratio (1.05 = 105%)
		#[pallet::constant]
		type ThrottleReserveRatio: Get<FixedU128>;

		/// Emergency reserve ratio (1.00 = 100%)
		#[pallet::constant]
		type EmergencyReserveRatio: Get<FixedU128>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Vault balances by asset type
	#[pallet::storage]
	#[pallet::getter(fn vault)]
	pub type Vault<T: Config> = StorageMap<_, Blake2_128Concat, AssetType, VaultEntry, OptionQuery>;

	/// Custodian-attested off-chain reserve value (USD cents)
	/// This includes fiat reserves, T-Bills, etc.
	#[pallet::storage]
	#[pallet::getter(fn custodian_value)]
	pub type CustodianAttestedValue<T> = StorageValue<_, u128, ValueQuery>;

	/// Current reserve ratio (cached)
	#[pallet::storage]
	#[pallet::getter(fn reserve_ratio)]
	pub type ReserveRatio<T> = StorageValue<_, FixedU128, ValueQuery>;

	/// Haircut configurations for each asset
	#[pallet::storage]
	#[pallet::getter(fn haircut)]
	pub type Haircuts<T: Config> = StorageMap<_, Blake2_128Concat, AssetType, Permill, ValueQuery>;

	/// USD price for each asset (in cents, e.g., BTC = 6000000 = $60,000)
	/// Updated by oracle or governance
	#[pallet::storage]
	#[pallet::getter(fn asset_price)]
	pub type AssetPrices<T: Config> = StorageMap<_, Blake2_128Concat, AssetType, u128, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collateral deposited [asset_type, amount, depositor]
		/// asset_type: 0=ETR, 1=BTC, 2=ETH, 3=USDC, 4=USDT, 5=DAI
		CollateralDeposited {
			asset_type: u8,
			amount: u128,
			depositor: T::AccountId,
		},
		/// Collateral withdrawn [asset_type, amount, recipient]
		CollateralWithdrawn {
			asset_type: u8,
			amount: u128,
			recipient: T::AccountId,
		},
		/// Reserve ratio updated [new_ratio]
		ReserveRatioUpdated { ratio: FixedU128 },
		/// Haircut updated [asset_type, new_haircut]
		HaircutUpdated { asset_type: u8, haircut: Permill },
		/// Asset price updated [asset_type, new_price]
		AssetPriceUpdated { asset_type: u8, price: u128 },
		/// Custodian value updated [new_value]
		CustodianValueUpdated { value: u128 },
		/// Reserve ratio entered critical zone
		ReserveCritical { ratio: FixedU128 },
		/// Reserve ratio entered throttle zone
		ReserveThrottled { ratio: FixedU128 },
		/// Reserve ratio returned to optimal
		ReserveOptimal { ratio: FixedU128 },
		/// Payout executed [recipient, usd_amount, assets_paid]
		PayoutExecuted {
			recipient: T::AccountId,
			usd_amount: u128,
			assets_paid: Vec<(u8, u128)>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Insufficient vault balance
		InsufficientVaultBalance,
		/// Asset not supported
		AssetNotSupported,
		/// Reserve ratio too low for withdrawal
		ReserveRatioTooLow,
		/// Arithmetic overflow
		Overflow,
		/// Arithmetic underflow
		Underflow,
	}

	/// Genesis configuration
	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub initial_haircuts: Vec<(AssetType, Permill)>,
		pub initial_prices: Vec<(AssetType, u128)>,
		#[serde(skip)]
		pub _phantom: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Initialize haircuts
			for (asset, haircut) in &self.initial_haircuts {
				Haircuts::<T>::insert(asset, haircut);
			}

			// Initialize prices
			for (asset, price) in &self.initial_prices {
				AssetPrices::<T>::insert(asset, price);
			}

			// Initialize reserve ratio to 0
			ReserveRatio::<T>::put(FixedU128::zero());
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Deposit collateral into vault (anyone can deposit)
		///
		/// # Parameters
		/// - `origin`: Depositor account
		/// - `asset_type`: Asset type as u8 (0=ETR, 1=BTC, 2=ETH, 3=USDC, 4=USDT, 5=DAI)
		/// - `amount`: Amount to deposit
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn deposit_collateral(
			origin: OriginFor<T>,
			asset_type: u8,
			amount: u128,
		) -> DispatchResult {
			let depositor = ensure_signed(origin)?;
			let asset = AssetType::from_u8(asset_type).ok_or(Error::<T>::AssetNotSupported)?;

			// TODO: Transfer asset from depositor to vault
			// For now, just update vault balance

			// Get or create vault entry
			let mut entry = Vault::<T>::get(&asset).unwrap_or(VaultEntry {
				raw_balance: 0,
				haircut: Haircuts::<T>::get(&asset),
				usd_value: 0,
				adjusted_value: 0,
			});

			// Update balance
			entry.raw_balance = entry.raw_balance
				.checked_add(amount)
				.ok_or(Error::<T>::Overflow)?;

			// Recalculate USD values
			Self::update_vault_entry_value(&mut entry, &asset)?;

			// Store updated entry
			Vault::<T>::insert(&asset, entry);

			Self::deposit_event(Event::CollateralDeposited {
				asset_type,
				amount,
				depositor,
			});

			// Recalculate reserve ratio
			Self::calculate_and_update_reserve_ratio()?;

			Ok(())
		}

		/// Withdraw collateral from vault (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `asset_type`: Asset type as u8
		/// - `amount`: Amount to withdraw
		/// - `recipient`: Recipient account
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn withdraw_collateral(
			origin: OriginFor<T>,
			asset_type: u8,
			amount: u128,
			recipient: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			let asset = AssetType::from_u8(asset_type).ok_or(Error::<T>::AssetNotSupported)?;

			// Get vault entry
			let mut entry = Vault::<T>::get(&asset)
				.ok_or(Error::<T>::AssetNotSupported)?;

			// Check balance
			ensure!(entry.raw_balance >= amount, Error::<T>::InsufficientVaultBalance);

			// Check if withdrawal would break reserve ratio
			let new_balance = entry.raw_balance.saturating_sub(amount);
			let reserve_ratio = Self::calculate_reserve_ratio_after_withdrawal(&asset, new_balance)?;
			ensure!(
				reserve_ratio >= <T as Config>::EmergencyReserveRatio::get(),
				Error::<T>::ReserveRatioTooLow
			);

			// Update balance
			entry.raw_balance = new_balance;

			// Recalculate USD values
			Self::update_vault_entry_value(&mut entry, &asset)?;

			// Store updated entry
			if entry.raw_balance > 0 {
				Vault::<T>::insert(&asset, entry);
			} else {
				Vault::<T>::remove(&asset);
			}

			// TODO: Transfer asset to recipient

			Self::deposit_event(Event::CollateralWithdrawn {
				asset_type,
				amount,
				recipient,
			});

			// Recalculate reserve ratio
			Self::calculate_and_update_reserve_ratio()?;

			Ok(())
		}

		/// Update asset price (governance or oracle)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `asset_type`: Asset type as u8
		/// - `price`: New price in USD cents
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn update_asset_price(
			origin: OriginFor<T>,
			asset_type: u8,
			price: u128,
		) -> DispatchResult {
			ensure_root(origin)?; // TODO: Allow oracle pallet
			let asset = AssetType::from_u8(asset_type).ok_or(Error::<T>::AssetNotSupported)?;

			AssetPrices::<T>::insert(&asset, price);

			// Update vault entry if exists
			if let Some(mut entry) = Vault::<T>::get(&asset) {
				Self::update_vault_entry_value(&mut entry, &asset)?;
				Vault::<T>::insert(&asset, entry);
			}

			Self::deposit_event(Event::AssetPriceUpdated { asset_type, price });

			// Recalculate reserve ratio
			Self::calculate_and_update_reserve_ratio()?;

			Ok(())
		}

		/// Update haircut for asset (governance only)
		///
		/// # Parameters
		/// - `origin`: Root/governance
		/// - `asset_type`: Asset type as u8
		/// - `haircut`: New haircut percentage (Permill)
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn update_haircut(
			origin: OriginFor<T>,
			asset_type: u8,
			haircut: Permill,
		) -> DispatchResult {
			ensure_root(origin)?;
			let asset = AssetType::from_u8(asset_type).ok_or(Error::<T>::AssetNotSupported)?;

			Haircuts::<T>::insert(&asset, haircut);

			// Update vault entry if exists
			if let Some(mut entry) = Vault::<T>::get(&asset) {
				entry.haircut = haircut;
				Self::update_vault_entry_value(&mut entry, &asset)?;
				Vault::<T>::insert(&asset, entry);
			}

			Self::deposit_event(Event::HaircutUpdated { asset_type, haircut });

			// Recalculate reserve ratio
			Self::calculate_and_update_reserve_ratio()?;

			Ok(())
		}

		/// Update custodian-attested value (custodian registry pallet only)
		///
		/// # Parameters
		/// - `origin`: Root/governance (TODO: custodian pallet)
		/// - `value`: New attested value in USD cents
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn update_custodian_value(
			origin: OriginFor<T>,
			value: u128,
		) -> DispatchResult {
			ensure_root(origin)?; // TODO: Allow custodian pallet

			CustodianAttestedValue::<T>::put(value);

			Self::deposit_event(Event::CustodianValueUpdated { value });

			// Recalculate reserve ratio
			Self::calculate_and_update_reserve_ratio()?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Internal helper to update custodian value (called by custodian registry pallet)
		pub fn do_update_custodian_value(value: u128) -> DispatchResult {
			CustodianAttestedValue::<T>::put(value);
			Self::deposit_event(Event::CustodianValueUpdated { value });
			// Recalculate reserve ratio
			Self::calculate_and_update_reserve_ratio()?;
			Ok(())
		}

		/// Internal helper to execute payout (called by redemption pallet)
		///
		/// Pays out USD value from vault reserves proportionally across assets.
		/// Applies haircuts in reverse - assets are withdrawn at risk-adjusted values.
		///
		/// # Parameters
		/// - `recipient`: Account receiving the payout
		/// - `usd_amount`: Amount to pay in USD cents
		///
		/// # Returns
		/// - Ok(()) if payout successful
		/// - Err if insufficient reserves or arithmetic errors
		pub fn do_payout(recipient: &T::AccountId, usd_amount: u128) -> DispatchResult {
			// Check if we have sufficient vault value
			let total_vault_value = Self::calculate_total_vault_value()?;
			ensure!(total_vault_value >= usd_amount, Error::<T>::InsufficientVaultBalance);

			// Calculate reserve ratio after payout
			let custodian_value = CustodianAttestedValue::<T>::get();
			let total_reserves_after = total_vault_value
				.checked_sub(usd_amount)
				.ok_or(Error::<T>::Underflow)?
				.checked_add(custodian_value)
				.ok_or(Error::<T>::Overflow)?;

			let total_supply = pallet_edsc_token::Pallet::<T>::total_supply();
			if total_supply > 0 {
				let reserve_ratio_after = FixedU128::saturating_from_rational(total_reserves_after, total_supply);
				// Ensure we don't go below emergency ratio
				ensure!(
					reserve_ratio_after >= <T as Config>::EmergencyReserveRatio::get(),
					Error::<T>::ReserveRatioTooLow
				);
			}

			// Collect all assets and their proportions
			let mut asset_withdrawals: Vec<(AssetType, u8, u128)> = Vec::new();
			let mut remaining_usd = usd_amount;

			// Withdraw proportionally from each asset
			for (asset, mut entry) in Vault::<T>::iter() {
				if remaining_usd == 0 {
					break;
				}

				// Calculate this asset's proportion of total vault value
				let asset_proportion = if total_vault_value > 0 {
					FixedU128::saturating_from_rational(entry.adjusted_value, total_vault_value)
				} else {
					FixedU128::zero()
				};

				// Calculate USD amount to withdraw from this asset
				let usd_from_asset = asset_proportion
					.saturating_mul_int(usd_amount)
					.min(entry.adjusted_value)
					.min(remaining_usd);

				if usd_from_asset == 0 {
					continue;
				}

				// Calculate raw asset amount needed
				// adjusted_value = usd_value * (1 - haircut)
				// usd_value = raw_balance * price / decimals
				// So: raw_amount = (usd_from_asset / (1 - haircut)) * decimals / price

				let haircut_multiplier = Permill::one().saturating_sub(entry.haircut);
				let usd_before_haircut = if haircut_multiplier.is_zero() {
					usd_from_asset
				} else {
					// Reverse haircut: usd_from_asset / (1 - haircut)
					FixedU128::from_u32(1)
						.checked_div(&FixedU128::saturating_from_rational(
							haircut_multiplier.deconstruct(),
							Permill::one().deconstruct(),
						))
						.unwrap_or(FixedU128::one())
						.saturating_mul_int(usd_from_asset)
				};

				let price = AssetPrices::<T>::get(&asset);
				ensure!(price > 0, Error::<T>::Overflow); // Prevent division by zero

				let raw_amount = usd_before_haircut
					.checked_mul(1_000_000_000_000)
					.ok_or(Error::<T>::Overflow)?
					.checked_div(price)
					.ok_or(Error::<T>::Overflow)?
					.min(entry.raw_balance);

				if raw_amount == 0 {
					continue;
				}

				// Update vault entry
				entry.raw_balance = entry.raw_balance.saturating_sub(raw_amount);
				Self::update_vault_entry_value(&mut entry, &asset)?;

				if entry.raw_balance > 0 {
					Vault::<T>::insert(&asset, entry);
				} else {
					Vault::<T>::remove(&asset);
				}

				// Record withdrawal
				let asset_type_u8 = match asset {
					AssetType::ETR => 0,
					AssetType::BTC => 1,
					AssetType::ETH => 2,
					AssetType::USDC => 3,
					AssetType::USDT => 4,
					AssetType::DAI => 5,
				};
				asset_withdrawals.push((asset.clone(), asset_type_u8, raw_amount));

				remaining_usd = remaining_usd.saturating_sub(usd_from_asset);
			}

			// TODO: Actual asset transfers to recipient
			// For now, this is a placeholder - in production, this would:
			// 1. Transfer on-chain assets (ETR, USDC, etc.) directly
			// 2. Coordinate with custodian for off-chain asset delivery (BTC, ETH)

			// Emit payout event
			let assets_paid: Vec<(u8, u128)> = asset_withdrawals
				.iter()
				.map(|(_, asset_type, amount)| (*asset_type, *amount))
				.collect();

			Self::deposit_event(Event::PayoutExecuted {
				recipient: recipient.clone(),
				usd_amount,
				assets_paid,
			});

			// Recalculate reserve ratio
			Self::calculate_and_update_reserve_ratio()?;

			Ok(())
		}

		/// Update vault entry USD values
		fn update_vault_entry_value(entry: &mut VaultEntry, asset: &AssetType) -> DispatchResult {
			let price = AssetPrices::<T>::get(asset);

			// Calculate USD value (raw_balance * price)
			// Note: Assumes price is per unit, adjust decimals as needed
			entry.usd_value = entry.raw_balance
				.checked_mul(price)
				.ok_or(Error::<T>::Overflow)?
				.checked_div(1_000_000_000_000) // Adjust for decimals
				.unwrap_or(0);

			// Apply haircut
			let haircut_amount = entry.haircut.mul_floor(entry.usd_value);
			entry.adjusted_value = entry.usd_value.saturating_sub(haircut_amount);

			Ok(())
		}

		/// Calculate total vault value (sum of all adjusted values)
		fn calculate_total_vault_value() -> Result<u128, DispatchError> {
			let mut total: u128 = 0;

			for (_asset, entry) in Vault::<T>::iter() {
				total = total.checked_add(entry.adjusted_value).ok_or(Error::<T>::Overflow)?;
			}

			Ok(total)
		}

		/// Calculate reserve ratio
		/// RR = (Vault Value + Custodian Value) / Total EDSC Supply
		pub fn calculate_reserve_ratio() -> Result<FixedU128, DispatchError> {
			// Get total vault value
			let vault_value = Self::calculate_total_vault_value()?;

			// Get custodian value
			let custodian_value = CustodianAttestedValue::<T>::get();

			// Total reserves
			let total_reserves = vault_value
				.checked_add(custodian_value)
				.ok_or(Error::<T>::Overflow)?;

			// Get total EDSC supply
			let total_supply = pallet_edsc_token::Pallet::<T>::total_supply();

			// Avoid division by zero
			if total_supply == 0 {
				return Ok(FixedU128::from_u32(1)); // Default to 100%
			}

			// Calculate ratio: reserves / supply
			let ratio = FixedU128::saturating_from_rational(total_reserves, total_supply);

			Ok(ratio)
		}

		/// Calculate reserve ratio after hypothetical withdrawal
		fn calculate_reserve_ratio_after_withdrawal(
			asset: &AssetType,
			new_balance: u128,
		) -> Result<FixedU128, DispatchError> {
			// Calculate what the vault value would be
			let mut total: u128 = 0;

			for (asset_type, entry) in Vault::<T>::iter() {
				if asset_type == *asset {
					// Use hypothetical balance
					let price = AssetPrices::<T>::get(&asset_type);
					let usd_value = new_balance
						.checked_mul(price)
						.ok_or(Error::<T>::Overflow)?
						.checked_div(1_000_000_000_000)
						.unwrap_or(0);
					let haircut_amount = entry.haircut.mul_floor(usd_value);
					let adjusted = usd_value.saturating_sub(haircut_amount);
					total = total.checked_add(adjusted).ok_or(Error::<T>::Overflow)?;
				} else {
					total = total.checked_add(entry.adjusted_value).ok_or(Error::<T>::Overflow)?;
				}
			}

			let custodian_value = CustodianAttestedValue::<T>::get();
			let total_reserves = total.checked_add(custodian_value).ok_or(Error::<T>::Overflow)?;
			let total_supply = pallet_edsc_token::Pallet::<T>::total_supply();

			if total_supply == 0 {
				return Ok(FixedU128::from_u32(1));
			}

			let ratio = FixedU128::saturating_from_rational(total_reserves, total_supply);
			Ok(ratio)
		}

		/// Calculate and update reserve ratio, trigger circuit breakers
		pub fn calculate_and_update_reserve_ratio() -> DispatchResult {
			let ratio = Self::calculate_reserve_ratio()?;
			let previous = ReserveRatio::<T>::get();

			// Store new ratio
			ReserveRatio::<T>::put(ratio);

			Self::deposit_event(Event::ReserveRatioUpdated { ratio });

			// Update redemption pallet
			let _ = pallet_edsc_redemption::Pallet::<T>::do_update_reserve_ratio(ratio);

			// Check circuit breakers
			if ratio < <T as Config>::EmergencyReserveRatio::get() {
				Self::deposit_event(Event::ReserveCritical { ratio });
			} else if ratio < <T as Config>::ThrottleReserveRatio::get() {
				Self::deposit_event(Event::ReserveThrottled { ratio });
			} else if ratio >= <T as Config>::OptimalReserveMin::get() && ratio <= <T as Config>::OptimalReserveMax::get() {
				// Only emit if recovering from non-optimal
				if previous < <T as Config>::OptimalReserveMin::get() || previous > <T as Config>::OptimalReserveMax::get() {
					Self::deposit_event(Event::ReserveOptimal { ratio });
				}
			}

			Ok(())
		}

		/// Get current reserve ratio (for external pallets)
		pub fn get_reserve_ratio() -> FixedU128 {
			ReserveRatio::<T>::get()
		}

		/// Get total reserves value (for queries)
		pub fn get_total_reserves() -> Result<u128, DispatchError> {
			let vault_value = Self::calculate_total_vault_value()?;
			let custodian_value = CustodianAttestedValue::<T>::get();
			vault_value.checked_add(custodian_value).ok_or(Error::<T>::Overflow.into())
		}
	}

	/// Hooks for automatic reserve ratio updates
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// On finalize, recalculate reserve ratio periodically
		fn on_finalize(_n: BlockNumberFor<T>) {
			// Auto-recalculate every 100 blocks (~10 minutes)
			let current_block = <frame_system::Pallet<T>>::block_number();

			if current_block.saturated_into::<u32>() % 100 == 0 {
				// Attempt recalculation (ignore errors)
				let _ = Self::calculate_and_update_reserve_ratio();
			}
		}
	}
}
