// XRP BRIDGE PALLET - Handles XRP and XRPL EVM Sidechain bridging
// User Priority + Just launched XRPL EVM Sidechain (June 2025)
// Priority #4 - $144B market cap, designed as bridge currency

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_runtime::{traits::SaturatedConversion, RuntimeDebug};
use sp_core::{H160, H256};

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

/// XRPL classic address (20 bytes) - uses H160 for codec compatibility
pub type XrplAddress = H160;

/// XRPL EVM sidechain address (20 bytes - compatible with Ethereum)
pub type XrplEvmAddress = H160;

/// XRP transaction hash (32 bytes)
pub type XrpTxHash = H256;

/// Bridge type - XRPL Classic or EVM Sidechain
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[codec(dumb_trait_bound)]
pub enum BridgeType {
	XrplClassic,
	XrplEvmSidechain,
}

/// XRP deposit record
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct XrpDeposit<AccountId, Balance> {
	pub xrpl_address: XrplAddress,
	pub etrid_account: AccountId,
	pub amount: Balance,
	pub tx_hash: XrpTxHash,
	pub ledger_index: u64, // XRPL uses ledger indices
	pub confirmations: u32,
	pub bridge_type: BridgeType,
	pub destination_tag: Option<u32>, // XRPL destination tags
	pub is_confirmed: bool,
}

/// XRP withdrawal request
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct XrpWithdrawal<AccountId, Balance> {
	pub etrid_account: AccountId,
	pub xrpl_address: XrplAddress,
	pub amount: Balance,
	pub bridge_type: BridgeType,
	pub destination_tag: Option<u32>,
	pub fee_drops: u64, // XRP fees in drops (1 XRP = 1M drops)
	pub status: WithdrawalStatus,
}

#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[codec(dumb_trait_bound)]
pub enum WithdrawalStatus {
	Pending,
	Processing,
	Completed(XrpTxHash),
	Failed,
}

/// Axelar bridge integration status
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[codec(dumb_trait_bound)]
pub struct AxelarBridge {
	pub enabled: bool,
	pub contract_address: Option<XrplEvmAddress>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement},
		BoundedVec,
	};
	use frame_system::pallet_prelude::*;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;

		/// Minimum confirmations required (1 for XRPL - instant finality!)
		#[pallet::constant]
		type MinConfirmations: Get<u32>;

		/// Bridge fee percentage (e.g., 0.1% = 10)
		#[pallet::constant]
		type BridgeFeeRate: Get<u32>;

		/// Maximum fee in drops (1 XRP = 1,000,000 drops)
		#[pallet::constant]
		type MaxFeeDrops: Get<u64>;

		/// Maximum number of confirmed deposits per account
		#[pallet::constant]
		type MaxDepositsPerAccount: Get<u32>;

		/// Maximum number of pending withdrawals per account
		#[pallet::constant]
		type MaxWithdrawalsPerAccount: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// XRP to ËTR exchange rate (scaled by 1e6 - 6 decimals)
	#[pallet::storage]
	#[pallet::getter(fn xrp_to_etr_rate)]
	pub type XrpToEtrRate<T> = StorageValue<_, u128, ValueQuery>;

	/// Pending deposits by XRP tx hash
	#[pallet::storage]
	#[pallet::getter(fn pending_deposits)]
	pub type PendingDeposits<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		XrpTxHash,
		XrpDeposit<T::AccountId, BalanceOf<T>>,
	>;

	/// Confirmed deposits by Etrid account
	#[pallet::storage]
	#[pallet::getter(fn confirmed_deposits)]
	pub type ConfirmedDeposits<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<XrpTxHash, T::MaxDepositsPerAccount>,
		ValueQuery,
	>;

	/// Pending withdrawals by Etrid account
	#[pallet::storage]
	#[pallet::getter(fn pending_withdrawals)]
	pub type PendingWithdrawals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<XrpWithdrawal<T::AccountId, BalanceOf<T>>, T::MaxWithdrawalsPerAccount>,
		ValueQuery,
	>;

	/// Total bridged XRP volume
	#[pallet::storage]
	#[pallet::getter(fn total_bridged_volume)]
	pub type TotalBridgedVolume<T> = StorageValue<_, u128, ValueQuery>;

	/// Bridge operator account (for admin functions)
	#[pallet::storage]
	#[pallet::getter(fn bridge_operator)]
	pub type BridgeOperator<T: Config> = StorageValue<_, T::AccountId>;

	/// Current XRPL ledger index (for tracking)
	#[pallet::storage]
	#[pallet::getter(fn current_ledger_index)]
	pub type CurrentLedgerIndex<T> = StorageValue<_, u64, ValueQuery>;

	/// XRPL EVM Sidechain enabled (launched June 2025)
	#[pallet::storage]
	#[pallet::getter(fn evm_sidechain_enabled)]
	pub type EvmSidechainEnabled<T> = StorageValue<_, bool, ValueQuery>;

	/// Axelar bridge integration
	#[pallet::storage]
	#[pallet::getter(fn axelar_integration)]
	pub type AxelarIntegration<T> = StorageValue<_, AxelarBridge>;

	/// Wormhole integration enabled
	#[pallet::storage]
	#[pallet::getter(fn wormhole_enabled)]
	pub type WormholeEnabled<T> = StorageValue<_, bool, ValueQuery>;

	/// Ripple ODL (On-Demand Liquidity) integration
	#[pallet::storage]
	#[pallet::getter(fn odl_enabled)]
	pub type OdlEnabled<T> = StorageValue<_, bool, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub xrp_to_etr_rate: u128,
		pub evm_sidechain_enabled: bool,
		pub _phantom: PhantomData<T>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				xrp_to_etr_rate: 1_000_000, // 1:1 default
				evm_sidechain_enabled: true,
				_phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			XrpToEtrRate::<T>::put(self.xrp_to_etr_rate);
			EvmSidechainEnabled::<T>::put(self.evm_sidechain_enabled);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// XRP deposit initiated [etrid_account, xrpl_address, amount, tx_hash, is_evm_sidechain]
		DepositInitiated {
			etrid_account: T::AccountId,
			xrpl_address: XrplAddress,
			amount: BalanceOf<T>,
			tx_hash: XrpTxHash,
			is_evm_sidechain: bool,
		},
		/// XRP deposit confirmed [etrid_account, amount, tx_hash]
		DepositConfirmed {
			etrid_account: T::AccountId,
			amount: BalanceOf<T>,
			tx_hash: XrpTxHash,
		},
		/// EVM Sidechain deposit [etrid_account, amount, tx_hash]
		EvmSidechainDepositConfirmed {
			etrid_account: T::AccountId,
			amount: BalanceOf<T>,
			tx_hash: XrpTxHash,
		},
		/// Withdrawal requested [etrid_account, xrpl_address, amount, is_evm_sidechain]
		WithdrawalRequested {
			etrid_account: T::AccountId,
			xrpl_address: XrplAddress,
			amount: BalanceOf<T>,
			is_evm_sidechain: bool,
		},
		/// Withdrawal completed [etrid_account, xrpl_address, amount, tx_hash]
		WithdrawalCompleted {
			etrid_account: T::AccountId,
			xrpl_address: XrplAddress,
			amount: BalanceOf<T>,
			tx_hash: XrpTxHash,
		},
		/// Exchange rate updated [old_rate, new_rate]
		ExchangeRateUpdated {
			old_rate: u128,
			new_rate: u128,
		},
		/// Ledger index updated [new_index]
		LedgerIndexUpdated {
			new_index: u64,
		},
		/// EVM Sidechain toggled [enabled]
		EvmSidechainToggled {
			enabled: bool,
		},
		/// Axelar bridge configured [enabled]
		AxelarConfigured {
			enabled: bool,
		},
		/// Wormhole integration toggled [enabled]
		WormholeToggled {
			enabled: bool,
		},
		/// ODL integration toggled [enabled]
		OdlToggled {
			enabled: bool,
		},
		/// Bridge operator changed [new_operator]
		OperatorChanged {
			new_operator: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Insufficient confirmations
		InsufficientConfirmations,
		/// Deposit already exists
		DepositAlreadyExists,
		/// Deposit not found
		DepositNotFound,
		/// Insufficient balance for withdrawal
		InsufficientBalance,
		/// Invalid XRPL address
		InvalidXrplAddress,
		/// Invalid amount (zero or too large)
		InvalidAmount,
		/// Withdrawal already processing
		WithdrawalAlreadyProcessing,
		/// Fee drops exceeded
		FeeDropsExceeded,
		/// Only bridge operator can call this
		NotOperator,
		/// Arithmetic overflow
		Overflow,
		/// Invalid ledger index
		InvalidLedgerIndex,
		/// EVM Sidechain not enabled
		EvmSidechainNotEnabled,
		/// Axelar not configured
		AxelarNotConfigured,
		/// Wormhole not enabled
		WormholeNotEnabled,
		/// Invalid destination tag
		InvalidDestinationTag,
		/// Too many deposits for account
		TooManyDeposits,
		/// Too many withdrawals for account
		TooManyWithdrawals,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initiate XRP deposit from XRPL Classic
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn initiate_xrp_deposit(
			origin: OriginFor<T>,
			etrid_account: T::AccountId,
			xrpl_address: XrplAddress,
			amount: BalanceOf<T>,
			tx_hash: XrpTxHash,
			ledger_index: u64,
			confirmations: u32,
			destination_tag: Option<u32>,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);
			ensure!(ledger_index > 0, Error::<T>::InvalidLedgerIndex);

			// Create deposit record
			let deposit = XrpDeposit {
				xrpl_address: xrpl_address.clone(),
				etrid_account: etrid_account.clone(),
				amount,
				tx_hash: tx_hash.clone(),
				ledger_index,
				confirmations,
				bridge_type: BridgeType::XrplClassic,
				destination_tag,
				is_confirmed: confirmations >= T::MinConfirmations::get(),
			};

			// Store pending deposit
			PendingDeposits::<T>::insert(&tx_hash, deposit);

			// Update ledger index if newer
			if ledger_index > CurrentLedgerIndex::<T>::get() {
				CurrentLedgerIndex::<T>::put(ledger_index);
				Self::deposit_event(Event::<T>::LedgerIndexUpdated {
					new_index: ledger_index,
				});
			}

			// Emit event
			Self::deposit_event(Event::<T>::DepositInitiated {
				etrid_account,
				xrpl_address,
				amount,
				tx_hash,
				is_evm_sidechain: false,
			});

			Ok(())
		}

		/// Initiate deposit from XRPL EVM Sidechain
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn initiate_evm_sidechain_deposit(
			origin: OriginFor<T>,
			etrid_account: T::AccountId,
			evm_address: XrplEvmAddress,
			amount: BalanceOf<T>,
			tx_hash: XrpTxHash,
			ledger_index: u64,
			confirmations: u32,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Check EVM sidechain is enabled
			ensure!(EvmSidechainEnabled::<T>::get(), Error::<T>::EvmSidechainNotEnabled);

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);

			// Convert EVM address to XRPL address format for storage
			let xrpl_address = evm_address;

			// Create deposit record
			let deposit = XrpDeposit {
				xrpl_address: xrpl_address.clone(),
				etrid_account: etrid_account.clone(),
				amount,
				tx_hash: tx_hash.clone(),
				ledger_index,
				confirmations,
				bridge_type: BridgeType::XrplEvmSidechain,
				destination_tag: None, // EVM doesn't use destination tags
				is_confirmed: confirmations >= T::MinConfirmations::get(),
			};

			// Store pending deposit
			PendingDeposits::<T>::insert(&tx_hash, deposit);

			// Emit event
			Self::deposit_event(Event::<T>::DepositInitiated {
				etrid_account,
				xrpl_address,
				amount,
				tx_hash,
				is_evm_sidechain: true,
			});

			Ok(())
		}

		/// Confirm XRP deposit after required confirmations (usually instant!)
		#[pallet::call_index(2)]
		#[pallet::weight(15_000)]
		pub fn confirm_xrp_deposit(
			origin: OriginFor<T>,
			tx_hash: XrpTxHash,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Get pending deposit
			let mut deposit = PendingDeposits::<T>::get(&tx_hash)
				.ok_or(Error::<T>::DepositNotFound)?;

			// Check confirmations (usually 1 for XRPL - instant finality!)
			ensure!(
				deposit.confirmations >= T::MinConfirmations::get(),
				Error::<T>::InsufficientConfirmations
			);

			// Calculate amount after bridge fee
			let fee_rate = T::BridgeFeeRate::get();
			let fee_amount = deposit.amount * fee_rate.into() / 1000u32.into();
			let net_amount = deposit.amount - fee_amount;

			// Convert XRP to ËTR using exchange rate
			let rate = XrpToEtrRate::<T>::get();
			let etr_amount = Self::convert_xrp_to_etr(net_amount, rate)?;

			// Mint ËTR to user
			let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

			// Update deposit status
			deposit.is_confirmed = true;
			PendingDeposits::<T>::insert(&tx_hash, deposit.clone());

			// Add to confirmed deposits
			ConfirmedDeposits::<T>::try_mutate(&deposit.etrid_account, |deposits| {
				deposits.try_push(tx_hash.clone())
					.map_err(|_| Error::<T>::TooManyDeposits)
			}).ok();

			// Update total volume
			TotalBridgedVolume::<T>::mutate(|total| {
				*total = total.saturating_add(deposit.amount.saturated_into());
			});

			// Emit appropriate event based on bridge type
			match deposit.bridge_type {
				BridgeType::XrplEvmSidechain => {
					Self::deposit_event(Event::<T>::EvmSidechainDepositConfirmed {
						etrid_account: deposit.etrid_account,
						amount: etr_amount,
						tx_hash,
					});
				},
				BridgeType::XrplClassic => {
					Self::deposit_event(Event::<T>::DepositConfirmed {
						etrid_account: deposit.etrid_account,
						amount: etr_amount,
						tx_hash,
					});
				},
			}

			Ok(())
		}

		/// Request XRP withdrawal
		#[pallet::call_index(3)]
		#[pallet::weight(20_000)]
		pub fn request_xrp_withdrawal(
			origin: OriginFor<T>,
			xrpl_address: XrplAddress,
			amount: BalanceOf<T>,
			use_evm_sidechain: bool,
			destination_tag: Option<u32>,
			fee_drops: u64,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(fee_drops <= T::MaxFeeDrops::get(), Error::<T>::FeeDropsExceeded);

			// Determine bridge type from bool
			let bridge_type = if use_evm_sidechain {
				BridgeType::XrplEvmSidechain
			} else {
				BridgeType::XrplClassic
			};

			// Check EVM sidechain if requested
			if use_evm_sidechain {
				ensure!(EvmSidechainEnabled::<T>::get(), Error::<T>::EvmSidechainNotEnabled);
			}

			// Check balance
			let balance = T::Currency::free_balance(&sender);
			ensure!(balance >= amount, Error::<T>::InsufficientBalance);

			// Burn ËTR from user
			T::Currency::withdraw(
				&sender,
				amount,
				frame_support::traits::WithdrawReasons::all(),
				ExistenceRequirement::KeepAlive,
			)?;

			// Create withdrawal request
			let withdrawal = XrpWithdrawal {
				etrid_account: sender.clone(),
				xrpl_address: xrpl_address.clone(),
				amount,
				bridge_type: bridge_type.clone(),
				destination_tag,
				fee_drops,
				status: WithdrawalStatus::Pending,
			};

			// Store withdrawal
			PendingWithdrawals::<T>::try_mutate(&sender, |withdrawals| {
				withdrawals.try_push(withdrawal)
					.map_err(|_| Error::<T>::TooManyWithdrawals)
			})?;

			// Emit event
			Self::deposit_event(Event::<T>::WithdrawalRequested {
				etrid_account: sender,
				xrpl_address,
				amount,
				is_evm_sidechain: use_evm_sidechain,
			});

			Ok(())
		}

		/// Update XRP/ËTR exchange rate (operator only)
		#[pallet::call_index(4)]
		#[pallet::weight(5_000)]
		pub fn update_exchange_rate(
			origin: OriginFor<T>,
			new_rate: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check operator
			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			let old_rate = XrpToEtrRate::<T>::get();
			XrpToEtrRate::<T>::put(new_rate);

			Self::deposit_event(Event::<T>::ExchangeRateUpdated {
				old_rate,
				new_rate,
			});

			Ok(())
		}

		/// Toggle EVM Sidechain support (operator only)
		#[pallet::call_index(5)]
		#[pallet::weight(5_000)]
		pub fn toggle_evm_sidechain(
			origin: OriginFor<T>,
			enabled: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			EvmSidechainEnabled::<T>::put(enabled);

			Self::deposit_event(Event::<T>::EvmSidechainToggled {
				enabled,
			});

			Ok(())
		}

		/// Configure Axelar bridge integration (operator only)
		#[pallet::call_index(6)]
		#[pallet::weight(5_000)]
		pub fn configure_axelar(
			origin: OriginFor<T>,
			enabled: bool,
			contract_address: Option<XrplEvmAddress>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			if enabled {
				let axelar = AxelarBridge {
					enabled: true,
					contract_address,
				};
				AxelarIntegration::<T>::put(axelar);
			} else {
				AxelarIntegration::<T>::take();
			}

			Self::deposit_event(Event::<T>::AxelarConfigured {
				enabled,
			});

			Ok(())
		}

		/// Toggle Wormhole integration (operator only)
		#[pallet::call_index(7)]
		#[pallet::weight(5_000)]
		pub fn toggle_wormhole(
			origin: OriginFor<T>,
			enabled: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			WormholeEnabled::<T>::put(enabled);

			Self::deposit_event(Event::<T>::WormholeToggled {
				enabled,
			});

			Ok(())
		}

		/// Toggle Ripple ODL integration (operator only)
		#[pallet::call_index(8)]
		#[pallet::weight(5_000)]
		pub fn toggle_odl(
			origin: OriginFor<T>,
			enabled: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			OdlEnabled::<T>::put(enabled);

			Self::deposit_event(Event::<T>::OdlToggled {
				enabled,
			});

			Ok(())
		}

		/// Set bridge operator (root only)
		#[pallet::call_index(9)]
		#[pallet::weight(5_000)]
		pub fn set_operator(
			origin: OriginFor<T>,
			new_operator: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			BridgeOperator::<T>::put(new_operator.clone());

			Self::deposit_event(Event::<T>::OperatorChanged {
				new_operator,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Convert XRP amount to ËTR using exchange rate
		fn convert_xrp_to_etr(xrp_amount: BalanceOf<T>, rate: u128) -> Result<BalanceOf<T>, DispatchError> {
			let xrp_u128: u128 = xrp_amount.saturated_into();
			let etr_u128 = xrp_u128.checked_mul(rate)
				.and_then(|v| v.checked_div(1_000_000)) // XRP uses 6 decimals (drops)
				.ok_or(Error::<T>::Overflow)?;

			Ok(etr_u128.saturated_into())
		}
	}
}

/// Test module
#[cfg(test)]
mod tests {
	// Tests will be added here
}
