// BNB CHAIN BRIDGE PALLET - Handles BNB and BEP-20 token bridging
// EVM-compatible (easy integration), Binance ecosystem dominance
// Priority #5 - $81.9B market cap, $15B+ stablecoin supply, 140% DAA growth

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_runtime::{traits::SaturatedConversion, RuntimeDebug};
use sp_core::{H160, H256};

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

// Use sp_core types which already have all codec traits implemented
pub type BnbAddress = H160;
pub type Bep20Contract = H160;
pub type BnbTxHash = H256;

/// BNB deposit record
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct BnbDeposit<AccountId, Balance> {
	pub bnb_address: BnbAddress,
	pub etrid_account: AccountId,
	pub amount: Balance,
	pub tx_hash: BnbTxHash,
	pub block_number: u64,
	pub confirmations: u32,
	pub token_contract: Option<Bep20Contract>, // None for BNB, Some for BEP-20
	pub is_confirmed: bool,
}

/// BNB withdrawal request
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct BnbWithdrawal<AccountId, Balance> {
	pub etrid_account: AccountId,
	pub bnb_address: BnbAddress,
	pub amount: Balance,
	pub token_contract: Option<Bep20Contract>, // None for BNB, Some for BEP-20
	pub gas_limit: u64,
	pub gas_price: u128, // In wei
	pub status: WithdrawalStatus,
}

#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[codec(dumb_trait_bound)]
pub enum WithdrawalStatus {
	Pending,
	Processing,
	Completed(BnbTxHash),
	Failed,
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

	type BalanceOf<T> = <<T as pallet_etr_lock::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_etr_lock::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Note: Currency type is inherited from pallet_etr_lock::Config to avoid ambiguity

		/// Minimum confirmations required (15 for BNB Chain - 3 second blocks)
		#[pallet::constant]
		type MinConfirmations: Get<u32>;

		/// Bridge fee percentage (e.g., 0.1% = 10)
		#[pallet::constant]
		type BridgeFeeRate: Get<u32>;

		/// Maximum gas limit for withdrawals
		#[pallet::constant]
		type MaxGasLimit: Get<u64>;

		/// Maximum gas price (in wei)
		#[pallet::constant]
		type MaxGasPrice: Get<u128>;

		/// Maximum number of confirmed deposits per account
		#[pallet::constant]
		type MaxDepositsPerAccount: Get<u32>;

		/// Maximum number of pending withdrawals per account
		#[pallet::constant]
		type MaxWithdrawalsPerAccount: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// BNB to ËTR exchange rate (scaled by 1e18)
	#[pallet::storage]
	#[pallet::getter(fn bnb_to_etr_rate)]
	pub type BnbToEtrRate<T> = StorageValue<_, u128, ValueQuery>;

	/// Pending deposits by BNB tx hash
	#[pallet::storage]
	#[pallet::getter(fn pending_deposits)]
	pub type PendingDeposits<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BnbTxHash,
		BnbDeposit<T::AccountId, BalanceOf<T>>,
	>;

	/// Confirmed deposits by Etrid account
	#[pallet::storage]
	#[pallet::getter(fn confirmed_deposits)]
	pub type ConfirmedDeposits<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<BnbTxHash, T::MaxDepositsPerAccount>,
		ValueQuery,
	>;

	/// Pending withdrawals by Etrid account
	#[pallet::storage]
	#[pallet::getter(fn pending_withdrawals)]
	pub type PendingWithdrawals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<BnbWithdrawal<T::AccountId, BalanceOf<T>>, T::MaxWithdrawalsPerAccount>,
		ValueQuery,
	>;

	/// Total bridged BNB volume
	#[pallet::storage]
	#[pallet::getter(fn total_bridged_volume)]
	pub type TotalBridgedVolume<T> = StorageValue<_, u128, ValueQuery>;

	/// Supported BEP-20 tokens (contract => enabled)
	#[pallet::storage]
	#[pallet::getter(fn supported_tokens)]
	pub type SupportedTokens<T> = StorageMap<_, Blake2_128Concat, Bep20Contract, bool, ValueQuery>;

	/// BEP-20 token exchange rates (scaled by 1e18)
	#[pallet::storage]
	#[pallet::getter(fn token_rates)]
	pub type TokenRates<T> = StorageMap<_, Blake2_128Concat, Bep20Contract, u128, ValueQuery>;

	/// BUSD contract address (Binance USD stablecoin)
	#[pallet::storage]
	#[pallet::getter(fn busd_contract)]
	pub type BusdContract<T> = StorageValue<_, Bep20Contract>;

	/// USDT BEP-20 contract address
	#[pallet::storage]
	#[pallet::getter(fn usdt_contract)]
	pub type UsdtContract<T> = StorageValue<_, Bep20Contract>;

	/// USDC BEP-20 contract address
	#[pallet::storage]
	#[pallet::getter(fn usdc_contract)]
	pub type UsdcContract<T> = StorageValue<_, Bep20Contract>;

	/// Bridge operator account (for admin functions)
	#[pallet::storage]
	#[pallet::getter(fn bridge_operator)]
	pub type BridgeOperator<T: Config> = StorageValue<_, T::AccountId>;

	/// Current BNB Chain block number (for tracking)
	#[pallet::storage]
	#[pallet::getter(fn current_block_number)]
	pub type CurrentBlockNumber<T> = StorageValue<_, u64, ValueQuery>;

	/// Maxwell upgrade enabled (49% faster blocks)
	#[pallet::storage]
	#[pallet::getter(fn maxwell_upgrade_enabled)]
	pub type MaxwellUpgradeEnabled<T> = StorageValue<_, bool, ValueQuery>;

	/// Binance Portal Bridge integration
	#[pallet::storage]
	#[pallet::getter(fn portal_bridge_enabled)]
	pub type PortalBridgeEnabled<T> = StorageValue<_, bool, ValueQuery>;

	/// Processed BNB burn transactions (to prevent replay attacks)
	#[pallet::storage]
	#[pallet::getter(fn processed_bnb_burns)]
	pub type ProcessedBnbBurns<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BnbTxHash,
		bool,
		ValueQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub bnb_to_etr_rate: u128,
		pub maxwell_upgrade_enabled: bool,
		pub portal_bridge_enabled: bool,
		pub _phantom: PhantomData<T>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				bnb_to_etr_rate: 1_000_000_000_000_000_000, // 1:1 default
				maxwell_upgrade_enabled: true,
				portal_bridge_enabled: false,
				_phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			BnbToEtrRate::<T>::put(self.bnb_to_etr_rate);
			MaxwellUpgradeEnabled::<T>::put(self.maxwell_upgrade_enabled);
			PortalBridgeEnabled::<T>::put(self.portal_bridge_enabled);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// BNB deposit initiated [etrid_account, bnb_address, amount, tx_hash]
		DepositInitiated {
			etrid_account: T::AccountId,
			bnb_address: BnbAddress,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
		},
		/// BNB deposit confirmed [etrid_account, amount, tx_hash]
		DepositConfirmed {
			etrid_account: T::AccountId,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
		},
		/// BEP-20 token deposit [etrid_account, contract, amount, tx_hash]
		TokenDepositConfirmed {
			etrid_account: T::AccountId,
			contract: Bep20Contract,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
		},
		/// BUSD deposit (Binance stablecoin) [etrid_account, amount, tx_hash]
		BusdDepositConfirmed {
			etrid_account: T::AccountId,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
		},
		/// Withdrawal requested [etrid_account, bnb_address, amount]
		WithdrawalRequested {
			etrid_account: T::AccountId,
			bnb_address: BnbAddress,
			amount: BalanceOf<T>,
		},
		/// Withdrawal completed [etrid_account, bnb_address, amount, tx_hash]
		WithdrawalCompleted {
			etrid_account: T::AccountId,
			bnb_address: BnbAddress,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
		},
		/// Exchange rate updated [old_rate, new_rate]
		ExchangeRateUpdated {
			old_rate: u128,
			new_rate: u128,
		},
		/// Token support added [contract, rate]
		TokenAdded {
			contract: Bep20Contract,
			rate: u128,
		},
		/// Block number updated [new_block]
		BlockNumberUpdated {
			new_block: u64,
		},
		/// Maxwell upgrade toggled [enabled]
		MaxwellUpgradeToggled {
			enabled: bool,
		},
		/// Portal Bridge toggled [enabled]
		PortalBridgeToggled {
			enabled: bool,
		},
		/// Bridge operator changed [new_operator]
		OperatorChanged {
			new_operator: T::AccountId,
		},
		/// ETR bridged to BNB Chain [from, amount, bnb_address]
		EtrBridgedToBnb {
			from: T::AccountId,
			amount: BalanceOf<T>,
			bnb_address: BnbAddress,
		},
		/// ETR unlocked from BNB Chain burn [to, amount, bnb_burn_tx]
		EtrUnlockedFromBnb {
			to: T::AccountId,
			amount: BalanceOf<T>,
			bnb_burn_tx: BnbTxHash,
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
		/// Invalid BNB address
		InvalidBnbAddress,
		/// Invalid amount (zero or too large)
		InvalidAmount,
		/// Withdrawal already processing
		WithdrawalAlreadyProcessing,
		/// Token not supported
		TokenNotSupported,
		/// Gas limit exceeded
		GasLimitExceeded,
		/// Gas price exceeded
		GasPriceExceeded,
		/// Only bridge operator can call this
		NotOperator,
		/// Arithmetic overflow
		Overflow,
		/// Invalid block number
		InvalidBlockNumber,
		/// Too many deposits for account
		TooManyDeposits,
		/// Too many withdrawals for account
		TooManyWithdrawals,
		/// Burn transaction already processed
		BurnAlreadyProcessed,
		/// Lock account not configured
		LockAccountNotSet,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initiate BNB deposit (called by relayer with proof)
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn initiate_bnb_deposit(
			origin: OriginFor<T>,
			etrid_account: T::AccountId,
			bnb_address: BnbAddress,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
			block_number: u64,
			confirmations: u32,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);
			ensure!(block_number > 0, Error::<T>::InvalidBlockNumber);

			// Create deposit record
			let deposit = BnbDeposit {
				bnb_address: bnb_address.clone(),
				etrid_account: etrid_account.clone(),
				amount,
				tx_hash: tx_hash.clone(),
				block_number,
				confirmations,
				token_contract: None, // BNB deposit
				is_confirmed: confirmations >= T::MinConfirmations::get(),
			};

			// Store pending deposit
			PendingDeposits::<T>::insert(&tx_hash, deposit);

			// Update block number if newer
			if block_number > CurrentBlockNumber::<T>::get() {
				CurrentBlockNumber::<T>::put(block_number);
				Self::deposit_event(Event::<T>::BlockNumberUpdated {
					new_block: block_number,
				});
			}

			// Emit event
			Self::deposit_event(Event::<T>::DepositInitiated {
				etrid_account,
				bnb_address,
				amount,
				tx_hash,
			});

			Ok(())
		}

		/// Confirm BNB deposit after required confirmations
		#[pallet::call_index(1)]
		#[pallet::weight(15_000)]
		pub fn confirm_bnb_deposit(
			origin: OriginFor<T>,
			tx_hash: BnbTxHash,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Get pending deposit
			let mut deposit = PendingDeposits::<T>::get(&tx_hash)
				.ok_or(Error::<T>::DepositNotFound)?;

			// Check confirmations
			ensure!(
				deposit.confirmations >= T::MinConfirmations::get(),
				Error::<T>::InsufficientConfirmations
			);

			// Calculate amount after bridge fee
			let fee_rate = T::BridgeFeeRate::get();
			let fee_amount = deposit.amount * fee_rate.into() / 1000u32.into();
			let net_amount = deposit.amount - fee_amount;

			// Convert BNB to ËTR using exchange rate
			let rate = BnbToEtrRate::<T>::get();
			let etr_amount = Self::convert_bnb_to_etr(net_amount, rate)?;

			// Mint ËTR to user
			let _ = <T as pallet_etr_lock::Config>::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

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

			// Emit event
			Self::deposit_event(Event::<T>::DepositConfirmed {
				etrid_account: deposit.etrid_account,
				amount: etr_amount,
				tx_hash,
			});

			Ok(())
		}

		/// Initiate BEP-20 token deposit
		#[pallet::call_index(2)]
		#[pallet::weight(12_000)]
		pub fn initiate_token_deposit(
			origin: OriginFor<T>,
			etrid_account: T::AccountId,
			bnb_address: BnbAddress,
			token_contract: Bep20Contract,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
			block_number: u64,
			confirmations: u32,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Validate token is supported
			ensure!(
				SupportedTokens::<T>::get(&token_contract),
				Error::<T>::TokenNotSupported
			);

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);

			// Create deposit record
			let deposit = BnbDeposit {
				bnb_address,
				etrid_account,
				amount,
				tx_hash: tx_hash.clone(),
				block_number,
				confirmations,
				token_contract: Some(token_contract.clone()),
				is_confirmed: confirmations >= T::MinConfirmations::get(),
			};

			// Store pending deposit
			PendingDeposits::<T>::insert(&tx_hash, deposit);

			// If confirmed, process immediately
			if confirmations >= T::MinConfirmations::get() {
				Self::process_token_deposit(tx_hash, token_contract)?;
			}

			Ok(())
		}

		/// Fast-track BUSD deposit (Binance stablecoin)
		#[pallet::call_index(3)]
		#[pallet::weight(12_000)]
		pub fn initiate_busd_deposit(
			origin: OriginFor<T>,
			etrid_account: T::AccountId,
			bnb_address: BnbAddress,
			amount: BalanceOf<T>,
			tx_hash: BnbTxHash,
			block_number: u64,
			confirmations: u32,
		) -> DispatchResult {
			let relayer = ensure_signed(origin)?;

			// Get BUSD contract
			let busd_contract = BusdContract::<T>::get()
				.ok_or(Error::<T>::TokenNotSupported)?;

			// Use standard token deposit flow
			Self::initiate_token_deposit(
				frame_system::RawOrigin::Signed(relayer).into(),
				etrid_account.clone(),
				bnb_address,
				busd_contract.clone(),
				amount,
				tx_hash.clone(),
				block_number,
				confirmations,
			)?;

			// Emit special BUSD event if confirmed
			if confirmations >= T::MinConfirmations::get() {
				Self::deposit_event(Event::<T>::BusdDepositConfirmed {
					etrid_account,
					amount,
					tx_hash,
				});
			}

			Ok(())
		}

		/// Request BNB withdrawal
		#[pallet::call_index(4)]
		#[pallet::weight(20_000)]
		pub fn request_bnb_withdrawal(
			origin: OriginFor<T>,
			bnb_address: BnbAddress,
			amount: BalanceOf<T>,
			gas_limit: u64,
			gas_price: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(gas_limit <= T::MaxGasLimit::get(), Error::<T>::GasLimitExceeded);
			ensure!(gas_price <= T::MaxGasPrice::get(), Error::<T>::GasPriceExceeded);

			// Check balance
			let balance = <T as pallet_etr_lock::Config>::Currency::free_balance(&sender);
			ensure!(balance >= amount, Error::<T>::InsufficientBalance);

			// Burn ËTR from user
			<T as pallet_etr_lock::Config>::Currency::withdraw(
				&sender,
				amount,
				frame_support::traits::WithdrawReasons::all(),
				ExistenceRequirement::KeepAlive,
			)?;

			// Create withdrawal request
			let withdrawal = BnbWithdrawal {
				etrid_account: sender.clone(),
				bnb_address: bnb_address.clone(),
				amount,
				token_contract: None, // BNB withdrawal
				gas_limit,
				gas_price,
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
				bnb_address,
				amount,
			});

			Ok(())
		}

		/// Update BNB/ËTR exchange rate (operator only)
		#[pallet::call_index(5)]
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

			let old_rate = BnbToEtrRate::<T>::get();
			BnbToEtrRate::<T>::put(new_rate);

			Self::deposit_event(Event::<T>::ExchangeRateUpdated {
				old_rate,
				new_rate,
			});

			Ok(())
		}

		/// Add supported BEP-20 token (operator only)
		#[pallet::call_index(6)]
		#[pallet::weight(5_000)]
		pub fn add_supported_token(
			origin: OriginFor<T>,
			token_contract: Bep20Contract,
			exchange_rate: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check operator
			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			// Add token
			SupportedTokens::<T>::insert(&token_contract, true);
			TokenRates::<T>::insert(&token_contract, exchange_rate);

			Self::deposit_event(Event::<T>::TokenAdded {
				contract: token_contract,
				rate: exchange_rate,
			});

			Ok(())
		}

		/// Set BUSD contract address (operator only)
		#[pallet::call_index(7)]
		#[pallet::weight(5_000)]
		pub fn set_busd_contract(
			origin: OriginFor<T>,
			contract: Bep20Contract,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			BusdContract::<T>::put(contract);

			Ok(())
		}

		/// Toggle Maxwell upgrade (operator only)
		#[pallet::call_index(8)]
		#[pallet::weight(5_000)]
		pub fn toggle_maxwell_upgrade(
			origin: OriginFor<T>,
			enabled: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			MaxwellUpgradeEnabled::<T>::put(enabled);

			Self::deposit_event(Event::<T>::MaxwellUpgradeToggled {
				enabled,
			});

			Ok(())
		}

		/// Toggle Portal Bridge integration (operator only)
		#[pallet::call_index(9)]
		#[pallet::weight(5_000)]
		pub fn toggle_portal_bridge(
			origin: OriginFor<T>,
			enabled: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			PortalBridgeEnabled::<T>::put(enabled);

			Self::deposit_event(Event::<T>::PortalBridgeToggled {
				enabled,
			});

			Ok(())
		}

		/// Set bridge operator (root only)
		#[pallet::call_index(10)]
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

		/// Bridge ETR tokens to BNB Chain
		///
		/// Locks ETR on FlareChain and emits event for relayer to mint on BNB Chain
		#[pallet::call_index(11)]
		#[pallet::weight(150_000)]
		pub fn bridge_etr_to_bnb(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			bnb_destination: BnbAddress,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert BNB address to bytes
			let destination_bytes = bnb_destination.as_bytes().to_vec();

			// Lock ETR using shared locking pallet
			pallet_etr_lock::Pallet::<T>::lock_for_bridge(
				frame_system::RawOrigin::Signed(who.clone()).into(),
				pallet_etr_lock::ChainId::BnbChain,
				amount,
				destination_bytes.clone(),
			)?;

			// Emit event for relayer
			Self::deposit_event(Event::<T>::EtrBridgedToBnb {
				from: who,
				amount,
				bnb_address: bnb_destination,
			});

			Ok(())
		}

		/// Process ETR burn from BNB Chain (called by relayer)
		///
		/// Unlocks ETR on FlareChain when wrapped ETR is burned on BNB Chain
		#[pallet::call_index(12)]
		#[pallet::weight(150_000)]
		pub fn process_etr_burn_from_bnb(
			origin: OriginFor<T>,
			etrid_recipient: T::AccountId,
			amount: BalanceOf<T>,
			bnb_burn_tx: BnbTxHash,
		) -> DispatchResult {
			// Should be called by authorized relayer/oracle
			let _relayer = ensure_signed(origin)?;
			// TODO: Add relayer authorization check

			// Verify burn hasn't been processed
			ensure!(
				!ProcessedBnbBurns::<T>::contains_key(&bnb_burn_tx),
				Error::<T>::BurnAlreadyProcessed
			);

			// Unlock ETR
			pallet_etr_lock::Pallet::<T>::unlock_from_bridge(
				frame_system::RawOrigin::Root.into(),
				pallet_etr_lock::ChainId::BnbChain,
				amount,
			)?;

			// Get lock account
			let lock_account = pallet_etr_lock::Pallet::<T>::lock_account()
				.ok_or(Error::<T>::LockAccountNotSet)?;

			// Transfer to recipient
			<T as pallet_etr_lock::Config>::Currency::transfer(
				&lock_account,
				&etrid_recipient,
				amount,
				ExistenceRequirement::KeepAlive,
			)?;

			// Mark as processed
			ProcessedBnbBurns::<T>::insert(&bnb_burn_tx, true);

			// Emit event
			Self::deposit_event(Event::<T>::EtrUnlockedFromBnb {
				to: etrid_recipient,
				amount,
				bnb_burn_tx,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Convert BNB amount to ËTR using exchange rate
		fn convert_bnb_to_etr(bnb_amount: BalanceOf<T>, rate: u128) -> Result<BalanceOf<T>, DispatchError> {
			let bnb_u128: u128 = bnb_amount.saturated_into();
			let etr_u128 = bnb_u128.checked_mul(rate)
				.and_then(|v| v.checked_div(1_000_000_000_000_000_000)) // 18 decimals like ETH
				.ok_or(Error::<T>::Overflow)?;

			Ok(etr_u128.saturated_into())
		}

		/// Process confirmed token deposit
		fn process_token_deposit(tx_hash: BnbTxHash, token_contract: Bep20Contract) -> DispatchResult {
			let deposit = PendingDeposits::<T>::get(&tx_hash)
				.ok_or(Error::<T>::DepositNotFound)?;

			// Get token rate
			let rate = TokenRates::<T>::get(&token_contract);

			// Convert token to ËTR
			let etr_amount = Self::convert_bnb_to_etr(deposit.amount, rate)?;

			// Mint ËTR
			let _ = <T as pallet_etr_lock::Config>::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

			// Check if this is BUSD
			let is_busd = BusdContract::<T>::get()
				.map(|busd| busd == token_contract)
				.unwrap_or(false);

			// Emit appropriate event
			if is_busd {
				Self::deposit_event(Event::<T>::BusdDepositConfirmed {
					etrid_account: deposit.etrid_account,
					amount: etr_amount,
					tx_hash,
				});
			} else {
				Self::deposit_event(Event::<T>::TokenDepositConfirmed {
					etrid_account: deposit.etrid_account,
					contract: token_contract,
					amount: etr_amount,
					tx_hash,
				});
			}

			Ok(())
		}
	}
}

/// Test module
#[cfg(test)]
mod tests {
	// Tests will be added here
}
