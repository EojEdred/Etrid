// TRON BRIDGE PALLET - Handles TRX and TRC-20 token bridging
// Optimized for stablecoins (63% of global USDT supply)
// Priority #2 - $21.5B daily transfers, $76B+ stablecoin infrastructure

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_runtime::{traits::SaturatedConversion, RuntimeDebug};
use sp_core::H256;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

/// TRON address (34 bytes base58 encoded, stored as 21 bytes)
#[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode, codec::DecodeWithMemTracking)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TronAddress(pub [u8; 21]);

/// TRC-20 token contract address
#[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode, codec::DecodeWithMemTracking)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenContract(pub [u8; 21]);

/// TRON transaction ID (32 bytes) - using H256 for compatibility
pub type TronTxId = H256;

/// TRON deposit record
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode, codec::DecodeWithMemTracking)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct TronDeposit<AccountId, Balance> {
    pub tron_address: TronAddress,
    pub etrid_account: AccountId,
    pub amount: Balance,
    pub tx_id: TronTxId,
    pub block_height: u64,
    pub confirmations: u32,
    pub token_contract: Option<TokenContract>, // None for TRX, Some for TRC-20
    pub is_confirmed: bool,
}

/// TRON withdrawal request
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode, codec::DecodeWithMemTracking)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct TronWithdrawal<AccountId, Balance> {
    pub etrid_account: AccountId,
    pub tron_address: TronAddress,
    pub amount: Balance,
    pub token_contract: Option<TokenContract>, // None for TRX, Some for TRC-20
    pub energy_limit: u64, // TRON uses energy instead of gas
    pub bandwidth_points: u64,
    pub status: WithdrawalStatus,
}

#[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode, codec::DecodeWithMemTracking)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum WithdrawalStatus {
    Pending,
    Processing,
    Completed(TronTxId),
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

        /// Minimum confirmations required (19 for TRON - super representatives)
        #[pallet::constant]
        type MinConfirmations: Get<u32>;

        /// Bridge fee percentage (e.g., 0.1% = 10)
        #[pallet::constant]
        type BridgeFeeRate: Get<u32>;

        /// Maximum energy limit for withdrawals
        #[pallet::constant]
        type MaxEnergyLimit: Get<u64>;

        /// Maximum bandwidth points
        #[pallet::constant]
        type MaxBandwidth: Get<u64>;

        /// Maximum number of confirmed deposits per account
        #[pallet::constant]
        type MaxDepositsPerAccount: Get<u32>;

        /// Maximum number of pending withdrawals per account
        #[pallet::constant]
        type MaxWithdrawalsPerAccount: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// TRX to ËTR exchange rate (scaled by 1e6 - TRON uses 6 decimals)
    #[pallet::storage]
    #[pallet::getter(fn trx_to_etr_rate)]
    pub type TrxToEtrRate<T> = StorageValue<_, u128, ValueQuery>;

    /// Pending deposits by TRON tx ID
    #[pallet::storage]
    #[pallet::getter(fn pending_deposits)]
    pub type PendingDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        TronTxId,
        TronDeposit<T::AccountId, BalanceOf<T>>,
    >;

    /// Confirmed deposits by Etrid account
    #[pallet::storage]
    #[pallet::getter(fn confirmed_deposits)]
    pub type ConfirmedDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<TronTxId, T::MaxDepositsPerAccount>,
        ValueQuery,
    >;

    /// Pending withdrawals by Etrid account
    #[pallet::storage]
    #[pallet::getter(fn pending_withdrawals)]
    pub type PendingWithdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<TronWithdrawal<T::AccountId, BalanceOf<T>>, T::MaxWithdrawalsPerAccount>,
        ValueQuery,
    >;

    /// Total bridged TRX volume
    #[pallet::storage]
    #[pallet::getter(fn total_bridged_volume)]
    pub type TotalBridgedVolume<T> = StorageValue<_, u128, ValueQuery>;

    /// Supported TRC-20 tokens (contract => enabled)
    #[pallet::storage]
    #[pallet::getter(fn supported_tokens)]
    pub type SupportedTokens<T> = StorageMap<_, Blake2_128Concat, TokenContract, bool, ValueQuery>;

    /// TRC-20 token exchange rates (scaled by 1e6)
    #[pallet::storage]
    #[pallet::getter(fn token_rates)]
    pub type TokenRates<T> = StorageMap<_, Blake2_128Concat, TokenContract, u128, ValueQuery>;

    /// USDT contract address (most important TRC-20)
    #[pallet::storage]
    #[pallet::getter(fn usdt_contract)]
    pub type UsdtContract<T> = StorageValue<_, TokenContract>;

    /// USDC contract address
    #[pallet::storage]
    #[pallet::getter(fn usdc_contract)]
    pub type UsdcContract<T> = StorageValue<_, TokenContract>;

    /// Bridge operator account (for admin functions)
    #[pallet::storage]
    #[pallet::getter(fn bridge_operator)]
    pub type BridgeOperator<T: Config> = StorageValue<_, T::AccountId>;

    /// Current TRON block height (for tracking)
    #[pallet::storage]
    #[pallet::getter(fn current_block_height)]
    pub type CurrentBlockHeight<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub trx_to_etr_rate: u128,
        pub _phantom: PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                trx_to_etr_rate: 1_000_000, // 1:1 default
                _phantom: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            TrxToEtrRate::<T>::put(self.trx_to_etr_rate);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// TRX deposit initiated [etrid_account, tron_address, amount, tx_id]
        DepositInitiated {
            etrid_account: T::AccountId,
            tron_address: TronAddress,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
        },

        /// TRX deposit confirmed [etrid_account, amount, tx_id]
        DepositConfirmed {
            etrid_account: T::AccountId,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
        },

        /// TRC-20 token deposit [etrid_account, contract, amount, tx_id]
        TokenDepositConfirmed {
            etrid_account: T::AccountId,
            token_contract: TokenContract,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
        },

        /// USDT deposit (special event due to volume) [etrid_account, amount, tx_id]
        UsdtDepositConfirmed {
            etrid_account: T::AccountId,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
        },

        /// Withdrawal requested [etrid_account, tron_address, amount]
        WithdrawalRequested {
            etrid_account: T::AccountId,
            tron_address: TronAddress,
            amount: BalanceOf<T>,
        },

        /// Withdrawal completed [etrid_account, tron_address, amount, tx_id]
        WithdrawalCompleted {
            etrid_account: T::AccountId,
            tron_address: TronAddress,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
        },

        /// Exchange rate updated [old_rate, new_rate]
        ExchangeRateUpdated {
            old_rate: u128,
            new_rate: u128,
        },

        /// Token support added [contract, rate]
        TokenAdded {
            token_contract: TokenContract,
            rate: u128,
        },

        /// TRON block height updated [new_height]
        BlockHeightUpdated {
            new_height: u64,
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
        /// Invalid TRON address
        InvalidTronAddress,
        /// Invalid amount (zero or too large)
        InvalidAmount,
        /// Withdrawal already processing
        WithdrawalAlreadyProcessing,
        /// Token not supported
        TokenNotSupported,
        /// Energy limit exceeded
        EnergyLimitExceeded,
        /// Bandwidth limit exceeded
        BandwidthLimitExceeded,
        /// Only bridge operator can call this
        NotOperator,
        /// Arithmetic overflow
        Overflow,
        /// Invalid block height
        InvalidBlockHeight,
        /// Too many deposits for account
        TooManyDeposits,
        /// Too many withdrawals for account
        TooManyWithdrawals,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initiate TRX deposit (called by relayer with proof)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn initiate_trx_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            tron_address: TronAddress,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
            block_height: u64,
            confirmations: u32,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Validate inputs
            ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
            ensure!(!PendingDeposits::<T>::contains_key(&tx_id), Error::<T>::DepositAlreadyExists);
            ensure!(block_height > 0, Error::<T>::InvalidBlockHeight);

            // Create deposit record
            let deposit = TronDeposit {
                tron_address: tron_address.clone(),
                etrid_account: etrid_account.clone(),
                amount,
                tx_id: tx_id.clone(),
                block_height,
                confirmations,
                token_contract: None, // TRX deposit
                is_confirmed: confirmations >= T::MinConfirmations::get(),
            };

            // Store pending deposit
            PendingDeposits::<T>::insert(&tx_id, deposit);

            // Update block height if newer
            if block_height > CurrentBlockHeight::<T>::get() {
                CurrentBlockHeight::<T>::put(block_height);
                Self::deposit_event(Event::<T>::BlockHeightUpdated {
                    new_height: block_height,
                });
            }

            // Emit event
            Self::deposit_event(Event::<T>::DepositInitiated {
                etrid_account,
                tron_address,
                amount,
                tx_id,
            });

            Ok(())
        }

        /// Confirm TRX deposit after required confirmations
        #[pallet::call_index(1)]
        #[pallet::weight(15_000)]
        pub fn confirm_trx_deposit(
            origin: OriginFor<T>,
            tx_id: TronTxId,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Get pending deposit
            let mut deposit = PendingDeposits::<T>::get(&tx_id)
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

            // Convert TRX to ËTR using exchange rate
            let rate = TrxToEtrRate::<T>::get();
            let etr_amount = Self::convert_trx_to_etr(net_amount, rate)?;

            // Mint ËTR to user
            let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

            // Update deposit status
            deposit.is_confirmed = true;
            PendingDeposits::<T>::insert(&tx_id, deposit.clone());

            // Add to confirmed deposits
            ConfirmedDeposits::<T>::try_mutate(&deposit.etrid_account, |deposits| {
                deposits.try_push(tx_id.clone())
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
                tx_id,
            });

            Ok(())
        }

        /// Initiate TRC-20 token deposit (optimized for USDT)
        #[pallet::call_index(2)]
        #[pallet::weight(12_000)]
        pub fn initiate_token_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            tron_address: TronAddress,
            token_contract: TokenContract,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
            block_height: u64,
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
            ensure!(!PendingDeposits::<T>::contains_key(&tx_id), Error::<T>::DepositAlreadyExists);

            // Create deposit record
            let deposit = TronDeposit {
                tron_address: tron_address.clone(),
                etrid_account: etrid_account.clone(),
                amount,
                tx_id: tx_id.clone(),
                block_height,
                confirmations,
                token_contract: Some(token_contract.clone()),
                is_confirmed: confirmations >= T::MinConfirmations::get(),
            };

            // Store pending deposit
            PendingDeposits::<T>::insert(&tx_id, deposit);

            // If confirmed, process immediately
            if confirmations >= T::MinConfirmations::get() {
                Self::process_token_deposit(tx_id, token_contract)?;
            }

            Ok(())
        }

        /// Fast-track USDT deposit (most common TRC-20)
        #[pallet::call_index(3)]
        #[pallet::weight(12_000)]
        pub fn initiate_usdt_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            tron_address: TronAddress,
            amount: BalanceOf<T>,
            tx_id: TronTxId,
            block_height: u64,
            confirmations: u32,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;

            // Get USDT contract
            let usdt_contract = UsdtContract::<T>::get()
                .ok_or(Error::<T>::TokenNotSupported)?;

            // Use standard token deposit flow
            Self::initiate_token_deposit(
                frame_system::RawOrigin::Signed(relayer).into(),
                etrid_account.clone(),
                tron_address,
                usdt_contract.clone(),
                amount,
                tx_id.clone(),
                block_height,
                confirmations,
            )?;

            // Emit special USDT event if confirmed
            if confirmations >= T::MinConfirmations::get() {
                Self::deposit_event(Event::<T>::UsdtDepositConfirmed {
                    etrid_account,
                    amount,
                    tx_id,
                });
            }

            Ok(())
        }

        /// Request TRX withdrawal
        #[pallet::call_index(4)]
        #[pallet::weight(20_000)]
        pub fn request_trx_withdrawal(
            origin: OriginFor<T>,
            tron_address: TronAddress,
            amount: BalanceOf<T>,
            energy_limit: u64,
            bandwidth_points: u64,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate inputs
            ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);
            ensure!(energy_limit <= T::MaxEnergyLimit::get(), Error::<T>::EnergyLimitExceeded);
            ensure!(bandwidth_points <= T::MaxBandwidth::get(), Error::<T>::BandwidthLimitExceeded);

            // Check balance
            let balance = T::Currency::free_balance(&sender);
            ensure!(balance >= amount, Error::<T>::InsufficientBalance);

            // Burn ËTR from user
            let _ = T::Currency::withdraw(
                &sender,
                amount,
                frame_support::traits::WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;

            // Create withdrawal request
            let withdrawal = TronWithdrawal {
                etrid_account: sender.clone(),
                tron_address: tron_address.clone(),
                amount,
                token_contract: None, // TRX withdrawal
                energy_limit,
                bandwidth_points,
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
                tron_address,
                amount,
            });

            Ok(())
        }

        /// Update TRX/ËTR exchange rate (operator only)
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

            let old_rate = TrxToEtrRate::<T>::get();
            TrxToEtrRate::<T>::put(new_rate);

            Self::deposit_event(Event::<T>::ExchangeRateUpdated {
                old_rate,
                new_rate,
            });

            Ok(())
        }

        /// Add supported TRC-20 token (operator only)
        #[pallet::call_index(6)]
        #[pallet::weight(5_000)]
        pub fn add_supported_token(
            origin: OriginFor<T>,
            token_contract: TokenContract,
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
                token_contract,
                rate: exchange_rate,
            });

            Ok(())
        }

        /// Set USDT contract address (operator only)
        #[pallet::call_index(7)]
        #[pallet::weight(5_000)]
        pub fn set_usdt_contract(
            origin: OriginFor<T>,
            contract: TokenContract,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            UsdtContract::<T>::put(contract);

            Ok(())
        }

        /// Set USDC contract address (operator only)
        #[pallet::call_index(8)]
        #[pallet::weight(5_000)]
        pub fn set_usdc_contract(
            origin: OriginFor<T>,
            contract: TokenContract,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            UsdcContract::<T>::put(contract);

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
        /// Convert TRX amount to ËTR using exchange rate
        fn convert_trx_to_etr(trx_amount: BalanceOf<T>, rate: u128) -> Result<BalanceOf<T>, DispatchError> {
            let trx_u128: u128 = trx_amount.saturated_into();
            let etr_u128 = trx_u128.checked_mul(rate)
                .and_then(|v| v.checked_div(1_000_000)) // TRON uses 6 decimals
                .ok_or(Error::<T>::Overflow)?;

            Ok(etr_u128.saturated_into())
        }

        /// Process confirmed token deposit
        fn process_token_deposit(tx_id: TronTxId, token_contract: TokenContract) -> DispatchResult {
            let deposit = PendingDeposits::<T>::get(&tx_id)
                .ok_or(Error::<T>::DepositNotFound)?;

            // Get token rate
            let rate = TokenRates::<T>::get(&token_contract);

            // Convert token to ËTR
            let etr_amount = Self::convert_trx_to_etr(deposit.amount, rate)?;

            // Mint ËTR
            let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

            // Check if this is USDT
            let is_usdt = UsdtContract::<T>::get()
                .map(|usdt| usdt == token_contract)
                .unwrap_or(false);

            // Emit appropriate event
            if is_usdt {
                Self::deposit_event(Event::<T>::UsdtDepositConfirmed {
                    etrid_account: deposit.etrid_account,
                    amount: etr_amount,
                    tx_id,
                });
            } else {
                Self::deposit_event(Event::<T>::TokenDepositConfirmed {
                    etrid_account: deposit.etrid_account,
                    token_contract,
                    amount: etr_amount,
                    tx_id,
                });
            }

            Ok(())
        }
    }
}

/// Test module
#[cfg(test)]
mod tests {
    use super::*;
    // Tests will be added here
}
