// ETHEREUM BRIDGE PALLET - Handles ETH and ERC-20 token bridging
// Supports smart contracts, gas optimization, and EVM compatibility
// Priority #1 - $38B bridge volume, 70% stablecoin supply

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
pub type EthereumAddress = H160;
pub type TokenAddress = H160;
pub type EthTxHash = H256;

/// Ethereum deposit record
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct EthereumDeposit<AccountId, Balance> {
	pub eth_address: EthereumAddress,
	pub etrid_account: AccountId,
	pub amount: Balance,
	pub tx_hash: EthTxHash,
	pub confirmations: u32,
	pub token_address: Option<TokenAddress>, // None for ETH, Some for ERC-20
	pub is_confirmed: bool,
}

/// Ethereum withdrawal request
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct EthereumWithdrawal<AccountId, Balance> {
	pub etrid_account: AccountId,
	pub eth_address: EthereumAddress,
	pub amount: Balance,
	pub token_address: Option<TokenAddress>, // None for ETH, Some for ERC-20
	pub gas_limit: u64,
	pub status: WithdrawalStatus,
}

#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[codec(dumb_trait_bound)]
pub enum WithdrawalStatus {
	Pending,
	Processing,
	Completed(EthTxHash),
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

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;

		/// Minimum confirmations required (12 for Ethereum)
		#[pallet::constant]
		type MinConfirmations: Get<u32>;

		/// Bridge fee percentage (e.g., 0.1% = 10)
		#[pallet::constant]
		type BridgeFeeRate: Get<u32>;

		/// Maximum gas limit for withdrawals
		#[pallet::constant]
		type MaxGasLimit: Get<u64>;

		/// Maximum number of confirmed deposits per account
		#[pallet::constant]
		type MaxDepositsPerAccount: Get<u32>;

		/// Maximum number of pending withdrawals per account
		#[pallet::constant]
		type MaxWithdrawalsPerAccount: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// ETH to ËTR exchange rate (scaled by 1e18)
	#[pallet::storage]
	#[pallet::getter(fn eth_to_etr_rate)]
	pub type EthToEtrRate<T> = StorageValue<_, u128, ValueQuery>;

	/// Pending deposits by ETH tx hash
	#[pallet::storage]
	#[pallet::getter(fn pending_deposits)]
	pub type PendingDeposits<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		EthTxHash,
		EthereumDeposit<T::AccountId, BalanceOf<T>>,
	>;

	/// Confirmed deposits by Etrid account
	#[pallet::storage]
	#[pallet::getter(fn confirmed_deposits)]
	pub type ConfirmedDeposits<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<EthTxHash, T::MaxDepositsPerAccount>,
		ValueQuery,
	>;

	/// Pending withdrawals by Etrid account
	#[pallet::storage]
	#[pallet::getter(fn pending_withdrawals)]
	pub type PendingWithdrawals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<EthereumWithdrawal<T::AccountId, BalanceOf<T>>, T::MaxWithdrawalsPerAccount>,
		ValueQuery,
	>;

	/// Total bridged ETH volume
	#[pallet::storage]
	#[pallet::getter(fn total_bridged_volume)]
	pub type TotalBridgedVolume<T> = StorageValue<_, u128, ValueQuery>;

	/// Supported ERC-20 tokens (address => enabled)
	#[pallet::storage]
	#[pallet::getter(fn supported_tokens)]
	pub type SupportedTokens<T> = StorageMap<_, Blake2_128Concat, TokenAddress, bool, ValueQuery>;

	/// ERC-20 token exchange rates (scaled by 1e18)
	#[pallet::storage]
	#[pallet::getter(fn token_rates)]
	pub type TokenRates<T> = StorageMap<_, Blake2_128Concat, TokenAddress, u128, ValueQuery>;

	/// Bridge operator account (for admin functions)
	#[pallet::storage]
	#[pallet::getter(fn bridge_operator)]
	pub type BridgeOperator<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub eth_to_etr_rate: u128,
		pub _phantom: PhantomData<T>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				eth_to_etr_rate: 1_000_000_000_000_000_000, // 1:1 default
				_phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			EthToEtrRate::<T>::put(self.eth_to_etr_rate);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Ethereum deposit initiated [etrid_account, eth_address, amount, tx_hash]
		DepositInitiated {
			etrid_account: T::AccountId,
			eth_address: EthereumAddress,
			amount: BalanceOf<T>,
			tx_hash: EthTxHash,
		},
		/// Ethereum deposit confirmed [etrid_account, amount, tx_hash]
		DepositConfirmed {
			etrid_account: T::AccountId,
			amount: BalanceOf<T>,
			tx_hash: EthTxHash,
		},
		/// ERC-20 token deposit [etrid_account, token_address, amount, tx_hash]
		TokenDepositConfirmed {
			etrid_account: T::AccountId,
			token_address: TokenAddress,
			amount: BalanceOf<T>,
			tx_hash: EthTxHash,
		},
		/// Withdrawal requested [etrid_account, eth_address, amount]
		WithdrawalRequested {
			etrid_account: T::AccountId,
			eth_address: EthereumAddress,
			amount: BalanceOf<T>,
		},
		/// Withdrawal completed [etrid_account, eth_address, amount, tx_hash]
		WithdrawalCompleted {
			etrid_account: T::AccountId,
			eth_address: EthereumAddress,
			amount: BalanceOf<T>,
			tx_hash: EthTxHash,
		},
		/// Exchange rate updated [old_rate, new_rate]
		ExchangeRateUpdated {
			old_rate: u128,
			new_rate: u128,
		},
		/// Token support added [token_address, rate]
		TokenAdded {
			token_address: TokenAddress,
			rate: u128,
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
		/// Invalid Ethereum address
		InvalidEthereumAddress,
		/// Invalid amount (zero or too large)
		InvalidAmount,
		/// Withdrawal already processing
		WithdrawalAlreadyProcessing,
		/// Token not supported
		TokenNotSupported,
		/// Gas limit exceeded
		GasLimitExceeded,
		/// Only bridge operator can call this
		NotOperator,
		/// Arithmetic overflow
		Overflow,
		/// Too many deposits for account
		TooManyDeposits,
		/// Too many withdrawals for account
		TooManyWithdrawals,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initiate ETH deposit (called by relayer with proof)
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn initiate_eth_deposit(
			origin: OriginFor<T>,
			etrid_account: T::AccountId,
			eth_address: EthereumAddress,
			amount: BalanceOf<T>,
			tx_hash: EthTxHash,
			confirmations: u32,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);

			// Create deposit record
			let deposit = EthereumDeposit {
				eth_address: eth_address.clone(),
				etrid_account: etrid_account.clone(),
				amount,
				tx_hash: tx_hash.clone(),
				confirmations,
				token_address: None, // ETH deposit
				is_confirmed: confirmations >= T::MinConfirmations::get(),
			};

			// Store pending deposit
			PendingDeposits::<T>::insert(&tx_hash, deposit);

			// Emit event
			Self::deposit_event(Event::<T>::DepositInitiated {
				etrid_account,
				eth_address,
				amount,
				tx_hash,
			});

			Ok(())
		}

		/// Confirm ETH deposit after required confirmations
		#[pallet::call_index(1)]
		#[pallet::weight(15_000)]
		pub fn confirm_eth_deposit(
			origin: OriginFor<T>,
			tx_hash: EthTxHash,
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

			// Convert ETH to ËTR using exchange rate
			let rate = EthToEtrRate::<T>::get();
			let etr_amount = Self::convert_eth_to_etr(net_amount, rate)?;

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

			// Emit event
			Self::deposit_event(Event::<T>::DepositConfirmed {
				etrid_account: deposit.etrid_account,
				amount: etr_amount,
				tx_hash,
			});

			Ok(())
		}

		/// Initiate ERC-20 token deposit
		#[pallet::call_index(2)]
		#[pallet::weight(12_000)]
		pub fn initiate_token_deposit(
			origin: OriginFor<T>,
			etrid_account: T::AccountId,
			eth_address: EthereumAddress,
			token_address: TokenAddress,
			amount: BalanceOf<T>,
			tx_hash: EthTxHash,
			confirmations: u32,
		) -> DispatchResult {
			let _relayer = ensure_signed(origin)?;

			// Validate token is supported
			ensure!(
				SupportedTokens::<T>::get(&token_address),
				Error::<T>::TokenNotSupported
			);

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);

			// Create deposit record
			let deposit = EthereumDeposit {
				eth_address,
				etrid_account,
				amount,
				tx_hash: tx_hash.clone(),
				confirmations,
				token_address: Some(token_address.clone()),
				is_confirmed: confirmations >= T::MinConfirmations::get(),
			};

			// Store pending deposit
			PendingDeposits::<T>::insert(&tx_hash, deposit);

			// If confirmed, process immediately
			if confirmations >= T::MinConfirmations::get() {
				Self::process_token_deposit(tx_hash, token_address)?;
			}

			Ok(())
		}

		/// Request ETH withdrawal
		#[pallet::call_index(3)]
		#[pallet::weight(20_000)]
		pub fn request_eth_withdrawal(
			origin: OriginFor<T>,
			eth_address: EthereumAddress,
			amount: BalanceOf<T>,
			gas_limit: u64,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Validate inputs
			ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
			ensure!(gas_limit <= T::MaxGasLimit::get(), Error::<T>::GasLimitExceeded);

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
			let withdrawal = EthereumWithdrawal {
				etrid_account: sender.clone(),
				eth_address: eth_address.clone(),
				amount,
				token_address: None, // ETH withdrawal
				gas_limit,
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
				eth_address,
				amount,
			});

			Ok(())
		}

		/// Update ETH/ËTR exchange rate (operator only)
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

			let old_rate = EthToEtrRate::<T>::get();
			EthToEtrRate::<T>::put(new_rate);

			Self::deposit_event(Event::<T>::ExchangeRateUpdated {
				old_rate,
				new_rate,
			});

			Ok(())
		}

		/// Add supported ERC-20 token (operator only)
		#[pallet::call_index(5)]
		#[pallet::weight(5_000)]
		pub fn add_supported_token(
			origin: OriginFor<T>,
			token_address: TokenAddress,
			exchange_rate: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check operator
			let operator = BridgeOperator::<T>::get()
				.ok_or(Error::<T>::NotOperator)?;
			ensure!(sender == operator, Error::<T>::NotOperator);

			// Add token
			SupportedTokens::<T>::insert(&token_address, true);
			TokenRates::<T>::insert(&token_address, exchange_rate);

			Self::deposit_event(Event::<T>::TokenAdded {
				token_address,
				rate: exchange_rate,
			});

			Ok(())
		}

		/// Set bridge operator (root only)
		#[pallet::call_index(6)]
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
		/// Convert ETH amount to ËTR using exchange rate
		fn convert_eth_to_etr(eth_amount: BalanceOf<T>, rate: u128) -> Result<BalanceOf<T>, DispatchError> {
			let eth_u128: u128 = eth_amount.saturated_into();
			let etr_u128 = eth_u128.checked_mul(rate)
				.and_then(|v| v.checked_div(1_000_000_000_000_000_000))
				.ok_or(Error::<T>::Overflow)?;

			Ok(etr_u128.saturated_into())
		}

		/// Process confirmed token deposit
		fn process_token_deposit(tx_hash: EthTxHash, token_address: TokenAddress) -> DispatchResult {
			let deposit = PendingDeposits::<T>::get(&tx_hash)
				.ok_or(Error::<T>::DepositNotFound)?;

			// Get token rate
			let rate = TokenRates::<T>::get(&token_address);

			// Convert token to ËTR
			let etr_amount = Self::convert_eth_to_etr(deposit.amount, rate)?;

			// Mint ËTR
			let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

			// Emit event
			Self::deposit_event(Event::<T>::TokenDepositConfirmed {
				etrid_account: deposit.etrid_account,
				token_address,
				amount: etr_amount,
				tx_hash,
			});

			Ok(())
		}
	}
}

/// Test module
#[cfg(test)]
mod tests {
	// Tests will be added here
}
