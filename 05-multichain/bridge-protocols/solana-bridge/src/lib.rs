// SOLANA BRIDGE PALLET - Handles SOL and SPL token bridging
// Optimized for high-speed (65,000 TPS) and low fees
// Priority #3 - $10.1B bridge volume (+114% YoY), $59.55B DEX volume

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_runtime::RuntimeDebug;
use sp_core::H256;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

// Use sp_core H256 for 32-byte types (already has all codec traits)
pub type SolanaPublicKey = H256;
pub type SplTokenMint = H256;

/// Solana transaction signature (64 bytes = 2 x H256)
/// Represented as a tuple of two H256 values for codec compatibility
pub type SolanaSignature = (H256, H256);

/// Solana deposit record
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct SolanaDeposit<AccountId, Balance> {
    pub sol_pubkey: SolanaPublicKey,
    pub etrid_account: AccountId,
    pub amount: Balance,
    pub signature: SolanaSignature,
    pub slot: u64, // Solana uses slots instead of blocks
    pub confirmations: u32,
    pub token_mint: Option<SplTokenMint>, // None for SOL, Some for SPL tokens
    pub is_confirmed: bool,
}

/// Solana withdrawal request
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct SolanaWithdrawal<AccountId, Balance> {
    pub etrid_account: AccountId,
    pub sol_pubkey: SolanaPublicKey,
    pub amount: Balance,
    pub token_mint: Option<SplTokenMint>, // None for SOL, Some for SPL tokens
    pub priority_fee: u64, // Solana priority fee for faster processing
    pub compute_units: u32, // Compute budget
    pub status: WithdrawalStatus,
}

#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[codec(dumb_trait_bound)]
pub enum WithdrawalStatus {
    Pending,
    Processing,
    Completed(SolanaSignature),
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
    use sp_runtime::traits::{Zero, SaturatedConversion};

    // Currency type for handling ËTR tokens
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuration trait for Solana bridge
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// Minimum confirmations required (31 for Solana - finalized state)
        type MinConfirmations: Get<u32>;

        /// Bridge fee percentage (e.g., 0.1% = 10)
        type BridgeFeeRate: Get<u32>;

        /// Maximum priority fee (in lamports)
        type MaxPriorityFee: Get<u64>;

        /// Maximum compute units
        type MaxComputeUnits: Get<u32>;

        /// Maximum number of deposits per account
        #[pallet::constant]
        type MaxDepositsPerAccount: Get<u32>;

        /// Maximum number of withdrawals per account
        #[pallet::constant]
        type MaxWithdrawalsPerAccount: Get<u32>;
    }

    /// SOL to ËTR exchange rate (scaled by 1e9 - Solana uses 9 decimals/lamports)
    #[pallet::storage]
    #[pallet::getter(fn sol_to_etr_rate)]
    pub type SolToEtrRate<T> = StorageValue<_, u128, ValueQuery>;

    /// Pending deposits by Solana signature
    #[pallet::storage]
    #[pallet::getter(fn pending_deposits)]
    pub type PendingDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        SolanaSignature,
        SolanaDeposit<T::AccountId, BalanceOf<T>>,
    >;

    /// Confirmed deposits by Etrid account
    #[pallet::storage]
    #[pallet::getter(fn confirmed_deposits)]
    pub type ConfirmedDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<SolanaSignature, T::MaxDepositsPerAccount>,
        ValueQuery,
    >;

    /// Pending withdrawals by Etrid account
    #[pallet::storage]
    #[pallet::getter(fn pending_withdrawals)]
    pub type PendingWithdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<SolanaWithdrawal<T::AccountId, BalanceOf<T>>, T::MaxWithdrawalsPerAccount>,
        ValueQuery,
    >;

    /// Total bridged SOL volume
    #[pallet::storage]
    #[pallet::getter(fn total_bridged_volume)]
    pub type TotalBridgedVolume<T> = StorageValue<_, u128, ValueQuery>;

    /// Supported SPL tokens (mint => enabled)
    #[pallet::storage]
    #[pallet::getter(fn supported_tokens)]
    pub type SupportedTokens<T> = StorageMap<_, Blake2_128Concat, SplTokenMint, bool, ValueQuery>;

    /// SPL token exchange rates (scaled by 1e9)
    #[pallet::storage]
    #[pallet::getter(fn token_rates)]
    pub type TokenRates<T> = StorageMap<_, Blake2_128Concat, SplTokenMint, u128, ValueQuery>;

    /// USDC SPL token mint (73% of Solana stablecoins)
    #[pallet::storage]
    #[pallet::getter(fn usdc_mint)]
    pub type UsdcMint<T> = StorageValue<_, SplTokenMint>;

    /// USDT SPL token mint
    #[pallet::storage]
    #[pallet::getter(fn usdt_mint)]
    pub type UsdtMint<T> = StorageValue<_, SplTokenMint>;

    /// Bridge operator account (for admin functions)
    #[pallet::storage]
    #[pallet::getter(fn bridge_operator)]
    pub type BridgeOperator<T: Config> = StorageValue<_, T::AccountId>;

    /// Current Solana slot (for tracking)
    #[pallet::storage]
    #[pallet::getter(fn current_slot)]
    pub type CurrentSlot<T> = StorageValue<_, u64, ValueQuery>;

    /// Wormhole integration enabled (for cross-chain messaging)
    #[pallet::storage]
    #[pallet::getter(fn wormhole_enabled)]
    pub type WormholeEnabled<T> = StorageValue<_, bool, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub sol_to_etr_rate: u128,
        pub wormhole_enabled: bool,
        pub _phantom: sp_std::marker::PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                sol_to_etr_rate: 1_000_000_000, // 1:1 default
                wormhole_enabled: false,
                _phantom: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            SolToEtrRate::<T>::put(self.sol_to_etr_rate);
            WormholeEnabled::<T>::put(self.wormhole_enabled);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// SOL deposit initiated
        DepositInitiated {
            etrid_account: T::AccountId,
            sol_pubkey: SolanaPublicKey,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
        },

        /// SOL deposit confirmed
        DepositConfirmed {
            etrid_account: T::AccountId,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
        },

        /// SPL token deposit
        TokenDepositConfirmed {
            etrid_account: T::AccountId,
            mint: SplTokenMint,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
        },

        /// USDC deposit (special event - dominant stablecoin on Solana)
        UsdcDepositConfirmed {
            etrid_account: T::AccountId,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
        },

        /// Withdrawal requested
        WithdrawalRequested {
            etrid_account: T::AccountId,
            sol_pubkey: SolanaPublicKey,
            amount: BalanceOf<T>,
        },

        /// Withdrawal completed
        WithdrawalCompleted {
            etrid_account: T::AccountId,
            sol_pubkey: SolanaPublicKey,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
        },

        /// Exchange rate updated
        ExchangeRateUpdated {
            old_rate: u128,
            new_rate: u128,
        },

        /// Token support added
        TokenAdded {
            mint: SplTokenMint,
            rate: u128,
        },

        /// Solana slot updated
        SlotUpdated {
            new_slot: u64,
        },

        /// Wormhole integration toggled
        WormholeToggled {
            enabled: bool,
        },

        /// Bridge operator changed
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
        /// Invalid Solana public key
        InvalidSolanaPublicKey,
        /// Invalid amount (zero or too large)
        InvalidAmount,
        /// Withdrawal already processing
        WithdrawalAlreadyProcessing,
        /// Token not supported
        TokenNotSupported,
        /// Priority fee exceeded
        PriorityFeeExceeded,
        /// Compute units exceeded
        ComputeUnitsExceeded,
        /// Only bridge operator can call this
        NotOperator,
        /// Arithmetic overflow
        Overflow,
        /// Invalid slot number
        InvalidSlot,
        /// Wormhole not enabled
        WormholeNotEnabled,
        /// Too many deposits
        TooManyDeposits,
        /// Too many withdrawals
        TooManyWithdrawals,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initiate SOL deposit (called by relayer with proof)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn initiate_sol_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            sol_pubkey: SolanaPublicKey,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
            slot: u64,
            confirmations: u32,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Validate inputs
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
            ensure!(!PendingDeposits::<T>::contains_key(&signature), Error::<T>::DepositAlreadyExists);
            ensure!(slot > 0, Error::<T>::InvalidSlot);

            // Create deposit record
            let deposit = SolanaDeposit {
                sol_pubkey: sol_pubkey.clone(),
                etrid_account: etrid_account.clone(),
                amount,
                signature: signature.clone(),
                slot,
                confirmations,
                token_mint: None, // SOL deposit
                is_confirmed: confirmations >= T::MinConfirmations::get(),
            };

            // Store pending deposit
            PendingDeposits::<T>::insert(&signature, deposit);

            // Update slot if newer
            if slot > CurrentSlot::<T>::get() {
                CurrentSlot::<T>::put(slot);
                Self::deposit_event(Event::<T>::SlotUpdated { new_slot: slot });
            }

            // Emit event
            Self::deposit_event(Event::<T>::DepositInitiated {
                etrid_account,
                sol_pubkey,
                amount,
                signature,
            });

            Ok(())
        }

        /// Confirm SOL deposit after required confirmations
        #[pallet::call_index(1)]
        #[pallet::weight(15_000)]
        pub fn confirm_sol_deposit(
            origin: OriginFor<T>,
            signature: SolanaSignature,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Get pending deposit
            let mut deposit = PendingDeposits::<T>::get(&signature)
                .ok_or(Error::<T>::DepositNotFound)?;

            // Check confirmations (31 for finalized on Solana)
            ensure!(
                deposit.confirmations >= T::MinConfirmations::get(),
                Error::<T>::InsufficientConfirmations
            );

            // Calculate amount after bridge fee
            let fee_rate = T::BridgeFeeRate::get();
            let fee_amount = deposit.amount * fee_rate.into() / 1000u32.into();
            let net_amount = deposit.amount - fee_amount;

            // Convert SOL to ËTR using exchange rate
            let rate = SolToEtrRate::<T>::get();
            let etr_amount = Self::convert_sol_to_etr(net_amount, rate)?;

            // Mint ËTR to user
            let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

            // Update deposit status
            deposit.is_confirmed = true;
            PendingDeposits::<T>::insert(&signature, deposit.clone());

            // Add to confirmed deposits
            ConfirmedDeposits::<T>::try_mutate(&deposit.etrid_account, |deposits| {
                deposits.try_push(signature.clone())
                    .map_err(|_| Error::<T>::TooManyDeposits)
            })?;

            // Update total volume
            TotalBridgedVolume::<T>::mutate(|total| {
                *total = total.saturating_add(deposit.amount.saturated_into());
            });

            // Emit event
            Self::deposit_event(Event::<T>::DepositConfirmed {
                etrid_account: deposit.etrid_account,
                amount: etr_amount,
                signature,
            });

            Ok(())
        }

        /// Initiate SPL token deposit (optimized for USDC)
        #[pallet::call_index(2)]
        #[pallet::weight(12_000)]
        pub fn initiate_token_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            sol_pubkey: SolanaPublicKey,
            token_mint: SplTokenMint,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
            slot: u64,
            confirmations: u32,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Validate token is supported
            ensure!(
                SupportedTokens::<T>::get(&token_mint),
                Error::<T>::TokenNotSupported
            );

            // Validate inputs
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
            ensure!(!PendingDeposits::<T>::contains_key(&signature), Error::<T>::DepositAlreadyExists);

            // Create deposit record
            let deposit = SolanaDeposit {
                sol_pubkey: sol_pubkey.clone(),
                etrid_account: etrid_account.clone(),
                amount,
                signature: signature.clone(),
                slot,
                confirmations,
                token_mint: Some(token_mint.clone()),
                is_confirmed: confirmations >= T::MinConfirmations::get(),
            };

            // Store pending deposit
            PendingDeposits::<T>::insert(&signature, deposit);

            // If confirmed, process immediately
            if confirmations >= T::MinConfirmations::get() {
                Self::process_token_deposit(signature, token_mint)?;
            }

            Ok(())
        }

        /// Fast-track USDC deposit (73% of Solana stablecoins)
        #[pallet::call_index(3)]
        #[pallet::weight(12_000)]
        pub fn initiate_usdc_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            sol_pubkey: SolanaPublicKey,
            amount: BalanceOf<T>,
            signature: SolanaSignature,
            slot: u64,
            confirmations: u32,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;

            // Get USDC mint
            let usdc_mint = UsdcMint::<T>::get()
                .ok_or(Error::<T>::TokenNotSupported)?;

            // Use standard token deposit flow
            Self::initiate_token_deposit(
                frame_system::RawOrigin::Signed(relayer).into(),
                etrid_account.clone(),
                sol_pubkey,
                usdc_mint.clone(),
                amount,
                signature.clone(),
                slot,
                confirmations,
            )?;

            // Emit special USDC event if confirmed
            if confirmations >= T::MinConfirmations::get() {
                Self::deposit_event(Event::<T>::UsdcDepositConfirmed {
                    etrid_account,
                    amount,
                    signature,
                });
            }

            Ok(())
        }

        /// Request SOL withdrawal
        #[pallet::call_index(4)]
        #[pallet::weight(20_000)]
        pub fn request_sol_withdrawal(
            origin: OriginFor<T>,
            sol_pubkey: SolanaPublicKey,
            amount: BalanceOf<T>,
            priority_fee: u64,
            compute_units: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate inputs
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
            ensure!(priority_fee <= T::MaxPriorityFee::get(), Error::<T>::PriorityFeeExceeded);
            ensure!(compute_units <= T::MaxComputeUnits::get(), Error::<T>::ComputeUnitsExceeded);

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
            let withdrawal = SolanaWithdrawal {
                etrid_account: sender.clone(),
                sol_pubkey: sol_pubkey.clone(),
                amount,
                token_mint: None, // SOL withdrawal
                priority_fee,
                compute_units,
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
                sol_pubkey,
                amount,
            });

            Ok(())
        }

        /// Update SOL/ËTR exchange rate (operator only)
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

            let old_rate = SolToEtrRate::<T>::get();
            SolToEtrRate::<T>::put(new_rate);

            Self::deposit_event(Event::<T>::ExchangeRateUpdated { old_rate, new_rate });

            Ok(())
        }

        /// Add supported SPL token (operator only)
        #[pallet::call_index(6)]
        #[pallet::weight(5_000)]
        pub fn add_supported_token(
            origin: OriginFor<T>,
            token_mint: SplTokenMint,
            exchange_rate: u128,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Check operator
            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            // Add token
            SupportedTokens::<T>::insert(&token_mint, true);
            TokenRates::<T>::insert(&token_mint, exchange_rate);

            Self::deposit_event(Event::<T>::TokenAdded { mint: token_mint, rate: exchange_rate });

            Ok(())
        }

        /// Set USDC mint address (operator only)
        #[pallet::call_index(7)]
        #[pallet::weight(5_000)]
        pub fn set_usdc_mint(
            origin: OriginFor<T>,
            mint: SplTokenMint,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            UsdcMint::<T>::put(mint);

            Ok(())
        }

        /// Toggle Wormhole integration (operator only)
        #[pallet::call_index(8)]
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

            Self::deposit_event(Event::<T>::WormholeToggled { enabled });

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

            Self::deposit_event(Event::<T>::OperatorChanged { new_operator });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Convert SOL amount to ËTR using exchange rate
        fn convert_sol_to_etr(sol_amount: BalanceOf<T>, rate: u128) -> Result<BalanceOf<T>, DispatchError> {
            let sol_u128: u128 = sol_amount.saturated_into();
            let etr_u128 = sol_u128.checked_mul(rate)
                .and_then(|v| v.checked_div(1_000_000_000)) // Solana uses 9 decimals (lamports)
                .ok_or(Error::<T>::Overflow)?;

            Ok(etr_u128.saturated_into())
        }

        /// Process confirmed token deposit
        fn process_token_deposit(signature: SolanaSignature, token_mint: SplTokenMint) -> DispatchResult {
            let deposit = PendingDeposits::<T>::get(&signature)
                .ok_or(Error::<T>::DepositNotFound)?;

            // Get token rate
            let rate = TokenRates::<T>::get(&token_mint);

            // Convert token to ËTR
            let etr_amount = Self::convert_sol_to_etr(deposit.amount, rate)?;

            // Mint ËTR
            let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

            // Check if this is USDC (73% of Solana stablecoins)
            let is_usdc = UsdcMint::<T>::get()
                .map(|usdc| usdc == token_mint)
                .unwrap_or(false);

            // Emit appropriate event
            if is_usdc {
                Self::deposit_event(Event::<T>::UsdcDepositConfirmed {
                    etrid_account: deposit.etrid_account,
                    amount: etr_amount,
                    signature,
                });
            } else {
                Self::deposit_event(Event::<T>::TokenDepositConfirmed {
                    etrid_account: deposit.etrid_account,
                    mint: token_mint,
                    amount: etr_amount,
                    signature,
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
