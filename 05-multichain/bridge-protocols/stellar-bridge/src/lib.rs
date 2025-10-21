// STELLAR BRIDGE PALLET - Handles XLM and Stellar asset bridging
// Optimized for 5-second ledgers, SCP consensus, and Horizon API
// Stellar-specific: Anchors, SDEX, Soroban smart contracts, SEP-24

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, pallet_prelude::ConstU32};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_runtime::RuntimeDebug;
use sp_core::H256;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

// Use sp_core H256 for 32-byte types (Ed25519 for Stellar)
pub type StellarTxHash = H256;
pub type StellarAccountId = H256;

/// Stellar Asset (asset_code, issuer_account)
/// Asset codes can be 4 or 12 bytes (alphanumeric4 or alphanumeric12)
pub type StellarAsset = (BoundedVec<u8, ConstU32<12>>, StellarAccountId);

/// Stellar Anchor service configuration (SEP-24 support)
/// Format: (domain, deposit_server, withdrawal_server)
pub type AnchorConfig = (
    BoundedVec<u8, ConstU32<256>>,  // domain
    BoundedVec<u8, ConstU32<512>>,  // deposit_server
    BoundedVec<u8, ConstU32<512>>,  // withdrawal_server
);

/// Stellar deposit record
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct StellarDeposit<AccountId, Balance> {
    pub stellar_account: StellarAccountId,
    pub etrid_account: AccountId,
    pub amount: Balance,
    pub tx_hash: StellarTxHash,
    pub ledger: u64, // Stellar uses ledger numbers (5-second confirmations)
    pub confirmations: u32,
    pub asset: Option<StellarAsset>, // None for XLM, Some for custom assets
    pub is_confirmed: bool,
}

/// Stellar withdrawal request
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId, Balance))]
#[codec(dumb_trait_bound)]
pub struct StellarWithdrawal<AccountId, Balance> {
    pub etrid_account: AccountId,
    pub stellar_account: StellarAccountId,
    pub amount: Balance,
    pub asset: Option<StellarAsset>, // None for XLM, Some for custom assets
    pub memo: Option<BoundedVec<u8, ConstU32<28>>>, // Stellar memo (max 28 bytes)
    pub status: WithdrawalStatus,
}

#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[codec(dumb_trait_bound)]
pub enum WithdrawalStatus {
    Pending,
    Processing,
    Completed(StellarTxHash),
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

    /// Configuration trait for Stellar bridge
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// Minimum confirmations required (SCP finality, typically 3 ledgers = 15 seconds)
        type MinConfirmations: Get<u32>;

        /// Bridge fee percentage (e.g., 0.1% = 10)
        type BridgeFeeRate: Get<u32>;

        /// Maximum number of deposits per account
        #[pallet::constant]
        type MaxDepositsPerAccount: Get<u32>;

        /// Maximum number of withdrawals per account
        #[pallet::constant]
        type MaxWithdrawalsPerAccount: Get<u32>;
    }

    /// XLM to ËTR exchange rate (scaled by 1e7 - Stellar uses 7 decimals/stroops)
    #[pallet::storage]
    #[pallet::getter(fn xlm_to_etr_rate)]
    pub type XlmToEtrRate<T> = StorageValue<_, u128, ValueQuery>;

    /// Pending deposits by Stellar transaction hash
    #[pallet::storage]
    #[pallet::getter(fn pending_deposits)]
    pub type PendingDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        StellarTxHash,
        StellarDeposit<T::AccountId, BalanceOf<T>>,
    >;

    /// Confirmed deposits by Etrid account
    #[pallet::storage]
    #[pallet::getter(fn confirmed_deposits)]
    pub type ConfirmedDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<StellarTxHash, T::MaxDepositsPerAccount>,
        ValueQuery,
    >;

    /// Pending withdrawals by Etrid account
    #[pallet::storage]
    #[pallet::getter(fn pending_withdrawals)]
    pub type PendingWithdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<StellarWithdrawal<T::AccountId, BalanceOf<T>>, T::MaxWithdrawalsPerAccount>,
        ValueQuery,
    >;

    /// Total bridged XLM volume
    #[pallet::storage]
    #[pallet::getter(fn total_bridged_volume)]
    pub type TotalBridgedVolume<T> = StorageValue<_, u128, ValueQuery>;

    /// Supported Stellar assets (asset_id => asset)
    #[pallet::storage]
    #[pallet::getter(fn supported_assets)]
    pub type SupportedAssets<T> = StorageMap<_, Blake2_128Concat, u32, StellarAsset>;

    /// Stellar asset exchange rates (scaled by 1e7)
    #[pallet::storage]
    #[pallet::getter(fn asset_rates)]
    pub type AssetRates<T> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

    /// Next asset ID
    #[pallet::storage]
    #[pallet::getter(fn next_asset_id)]
    pub type NextAssetId<T> = StorageValue<_, u32, ValueQuery>;

    /// Registered Stellar Horizon servers (for monitoring and relaying)
    #[pallet::storage]
    #[pallet::getter(fn horizon_servers)]
    pub type HorizonServers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u8, ConstU32<256>>,
    >;

    /// Stellar Anchor configurations for SEP-24 integration
    #[pallet::storage]
    #[pallet::getter(fn anchor_configs)]
    pub type AnchorConfigs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // asset_id
        AnchorConfig,
    >;

    /// Bridge operator account (for admin functions)
    #[pallet::storage]
    #[pallet::getter(fn bridge_operator)]
    pub type BridgeOperator<T: Config> = StorageValue<_, T::AccountId>;

    /// Current Stellar ledger (for tracking)
    #[pallet::storage]
    #[pallet::getter(fn current_ledger)]
    pub type CurrentLedger<T> = StorageValue<_, u64, ValueQuery>;

    /// SDEX integration enabled (Stellar Decentralized Exchange)
    #[pallet::storage]
    #[pallet::getter(fn sdex_enabled)]
    pub type SdexEnabled<T> = StorageValue<_, bool, ValueQuery>;

    /// Soroban smart contract integration enabled
    #[pallet::storage]
    #[pallet::getter(fn soroban_enabled)]
    pub type SorobanEnabled<T> = StorageValue<_, bool, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub xlm_to_etr_rate: u128,
        pub sdex_enabled: bool,
        pub soroban_enabled: bool,
        pub _phantom: sp_std::marker::PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                xlm_to_etr_rate: 10_000_000, // 1:1 default (7 decimals)
                sdex_enabled: false,
                soroban_enabled: false,
                _phantom: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            XlmToEtrRate::<T>::put(self.xlm_to_etr_rate);
            SdexEnabled::<T>::put(self.sdex_enabled);
            SorobanEnabled::<T>::put(self.soroban_enabled);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// XLM deposit initiated
        DepositInitiated {
            etrid_account: T::AccountId,
            stellar_account: StellarAccountId,
            amount: BalanceOf<T>,
            tx_hash: StellarTxHash,
        },

        /// XLM deposit confirmed
        DepositConfirmed {
            etrid_account: T::AccountId,
            amount: BalanceOf<T>,
            tx_hash: StellarTxHash,
        },

        /// Stellar asset deposit confirmed
        AssetDepositConfirmed {
            etrid_account: T::AccountId,
            asset_id: u32,
            amount: BalanceOf<T>,
            tx_hash: StellarTxHash,
        },

        /// Withdrawal requested
        WithdrawalRequested {
            etrid_account: T::AccountId,
            stellar_account: StellarAccountId,
            amount: BalanceOf<T>,
        },

        /// Withdrawal completed
        WithdrawalCompleted {
            etrid_account: T::AccountId,
            stellar_account: StellarAccountId,
            amount: BalanceOf<T>,
            tx_hash: StellarTxHash,
        },

        /// Exchange rate updated
        ExchangeRateUpdated {
            old_rate: u128,
            new_rate: u128,
        },

        /// Asset support added
        AssetAdded {
            asset_id: u32,
            rate: u128,
        },

        /// Stellar ledger updated
        LedgerUpdated {
            new_ledger: u64,
        },

        /// SDEX integration toggled
        SdexToggled {
            enabled: bool,
        },

        /// Soroban integration toggled
        SorobanToggled {
            enabled: bool,
        },

        /// Bridge operator changed
        OperatorChanged {
            new_operator: T::AccountId,
        },

        /// Horizon server registered
        HorizonServerRegistered {
            node: T::AccountId,
            url: BoundedVec<u8, ConstU32<256>>,
        },

        /// Anchor configured
        AnchorConfigured {
            asset_id: u32,
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
        /// Invalid Stellar account ID
        InvalidStellarAccount,
        /// Invalid amount (zero or too large)
        InvalidAmount,
        /// Withdrawal already processing
        WithdrawalAlreadyProcessing,
        /// Asset not supported
        AssetNotSupported,
        /// Not a registered Horizon server
        NotHorizonServer,
        /// Only bridge operator can call this
        NotOperator,
        /// Arithmetic overflow
        Overflow,
        /// Invalid ledger number
        InvalidLedger,
        /// SDEX not enabled
        SdexNotEnabled,
        /// Soroban not enabled
        SorobanNotEnabled,
        /// Too many deposits
        TooManyDeposits,
        /// Too many withdrawals
        TooManyWithdrawals,
        /// Invalid asset code
        InvalidAssetCode,
        /// Invalid anchor configuration
        InvalidAnchorConfig,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initiate XLM deposit (called by relayer with proof)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn initiate_xlm_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            stellar_account: StellarAccountId,
            amount: BalanceOf<T>,
            tx_hash: StellarTxHash,
            ledger: u64,
            confirmations: u32,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Validate inputs
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
            ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);
            ensure!(ledger > 0, Error::<T>::InvalidLedger);

            // Create deposit record
            let deposit = StellarDeposit {
                stellar_account: stellar_account.clone(),
                etrid_account: etrid_account.clone(),
                amount,
                tx_hash: tx_hash.clone(),
                ledger,
                confirmations,
                asset: None, // XLM deposit
                is_confirmed: confirmations >= T::MinConfirmations::get(),
            };

            // Store pending deposit
            PendingDeposits::<T>::insert(&tx_hash, deposit);

            // Update ledger if newer
            if ledger > CurrentLedger::<T>::get() {
                CurrentLedger::<T>::put(ledger);
                Self::deposit_event(Event::<T>::LedgerUpdated { new_ledger: ledger });
            }

            // Emit event
            Self::deposit_event(Event::<T>::DepositInitiated {
                etrid_account,
                stellar_account,
                amount,
                tx_hash,
            });

            Ok(())
        }

        /// Confirm XLM deposit after required confirmations
        #[pallet::call_index(1)]
        #[pallet::weight(15_000)]
        pub fn confirm_xlm_deposit(
            origin: OriginFor<T>,
            tx_hash: StellarTxHash,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Get pending deposit
            let mut deposit = PendingDeposits::<T>::get(&tx_hash)
                .ok_or(Error::<T>::DepositNotFound)?;

            // Check confirmations (SCP finality)
            ensure!(
                deposit.confirmations >= T::MinConfirmations::get(),
                Error::<T>::InsufficientConfirmations
            );

            // Calculate amount after bridge fee
            let fee_rate = T::BridgeFeeRate::get();
            let fee_amount = deposit.amount * fee_rate.into() / 1000u32.into();
            let net_amount = deposit.amount - fee_amount;

            // Convert XLM to ËTR using exchange rate
            let rate = XlmToEtrRate::<T>::get();
            let etr_amount = Self::convert_xlm_to_etr(net_amount, rate)?;

            // Mint ËTR to user
            let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

            // Update deposit status
            deposit.is_confirmed = true;
            PendingDeposits::<T>::insert(&tx_hash, deposit.clone());

            // Add to confirmed deposits
            ConfirmedDeposits::<T>::try_mutate(&deposit.etrid_account, |deposits| {
                deposits.try_push(tx_hash.clone())
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
                tx_hash,
            });

            Ok(())
        }

        /// Initiate Stellar asset deposit (non-XLM)
        #[pallet::call_index(2)]
        #[pallet::weight(12_000)]
        pub fn initiate_asset_deposit(
            origin: OriginFor<T>,
            etrid_account: T::AccountId,
            stellar_account: StellarAccountId,
            asset_id: u32,
            amount: BalanceOf<T>,
            tx_hash: StellarTxHash,
            ledger: u64,
            confirmations: u32,
        ) -> DispatchResult {
            let _relayer = ensure_signed(origin)?;

            // Validate asset is supported
            let asset = SupportedAssets::<T>::get(asset_id)
                .ok_or(Error::<T>::AssetNotSupported)?;

            // Validate inputs
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);
            ensure!(!PendingDeposits::<T>::contains_key(&tx_hash), Error::<T>::DepositAlreadyExists);

            // Create deposit record
            let deposit = StellarDeposit {
                stellar_account: stellar_account.clone(),
                etrid_account: etrid_account.clone(),
                amount,
                tx_hash: tx_hash.clone(),
                ledger,
                confirmations,
                asset: Some(asset.clone()),
                is_confirmed: confirmations >= T::MinConfirmations::get(),
            };

            // Store pending deposit
            PendingDeposits::<T>::insert(&tx_hash, deposit);

            // If confirmed, process immediately
            if confirmations >= T::MinConfirmations::get() {
                Self::process_asset_deposit(tx_hash, asset_id)?;
            }

            Ok(())
        }

        /// Request XLM withdrawal
        #[pallet::call_index(3)]
        #[pallet::weight(20_000)]
        pub fn request_xlm_withdrawal(
            origin: OriginFor<T>,
            stellar_account: StellarAccountId,
            amount: BalanceOf<T>,
            memo: Option<BoundedVec<u8, ConstU32<28>>>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate inputs
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);

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
            let withdrawal = StellarWithdrawal {
                etrid_account: sender.clone(),
                stellar_account: stellar_account.clone(),
                amount,
                asset: None, // XLM withdrawal
                memo,
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
                stellar_account,
                amount,
            });

            Ok(())
        }

        /// Update XLM/ËTR exchange rate (operator only)
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

            let old_rate = XlmToEtrRate::<T>::get();
            XlmToEtrRate::<T>::put(new_rate);

            Self::deposit_event(Event::<T>::ExchangeRateUpdated { old_rate, new_rate });

            Ok(())
        }

        /// Add supported Stellar asset (operator only)
        #[pallet::call_index(5)]
        #[pallet::weight(5_000)]
        pub fn add_supported_asset(
            origin: OriginFor<T>,
            asset: StellarAsset,
            exchange_rate: u128,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Check operator
            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            // Validate asset code (asset is tuple: (code, issuer))
            ensure!(
                asset.0.iter().any(|&b| b != 0),
                Error::<T>::InvalidAssetCode
            );

            // Get next asset ID
            let asset_id = NextAssetId::<T>::get();
            NextAssetId::<T>::put(asset_id.saturating_add(1));

            // Add asset
            SupportedAssets::<T>::insert(asset_id, asset);
            AssetRates::<T>::insert(asset_id, exchange_rate);

            Self::deposit_event(Event::<T>::AssetAdded { asset_id, rate: exchange_rate });

            Ok(())
        }

        /// Register Horizon server (for monitoring)
        #[pallet::call_index(6)]
        #[pallet::weight(5_000)]
        pub fn register_horizon_server(
            origin: OriginFor<T>,
            url: BoundedVec<u8, ConstU32<256>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            HorizonServers::<T>::insert(&who, url.clone());

            Self::deposit_event(Event::<T>::HorizonServerRegistered { node: who, url });

            Ok(())
        }

        /// Configure Anchor service (SEP-24)
        #[pallet::call_index(7)]
        #[pallet::weight(5_000)]
        pub fn configure_anchor(
            origin: OriginFor<T>,
            asset_id: u32,
            config: AnchorConfig,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Check operator
            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            // Verify asset is supported
            ensure!(
                SupportedAssets::<T>::contains_key(asset_id),
                Error::<T>::AssetNotSupported
            );

            AnchorConfigs::<T>::insert(asset_id, config);

            Self::deposit_event(Event::<T>::AnchorConfigured { asset_id });

            Ok(())
        }

        /// Toggle SDEX integration (operator only)
        #[pallet::call_index(8)]
        #[pallet::weight(5_000)]
        pub fn toggle_sdex(
            origin: OriginFor<T>,
            enabled: bool,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            SdexEnabled::<T>::put(enabled);

            Self::deposit_event(Event::<T>::SdexToggled { enabled });

            Ok(())
        }

        /// Toggle Soroban integration (operator only)
        #[pallet::call_index(9)]
        #[pallet::weight(5_000)]
        pub fn toggle_soroban(
            origin: OriginFor<T>,
            enabled: bool,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let operator = BridgeOperator::<T>::get()
                .ok_or(Error::<T>::NotOperator)?;
            ensure!(sender == operator, Error::<T>::NotOperator);

            SorobanEnabled::<T>::put(enabled);

            Self::deposit_event(Event::<T>::SorobanToggled { enabled });

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

            Self::deposit_event(Event::<T>::OperatorChanged { new_operator });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Convert XLM amount to ËTR using exchange rate
        fn convert_xlm_to_etr(xlm_amount: BalanceOf<T>, rate: u128) -> Result<BalanceOf<T>, DispatchError> {
            let xlm_u128: u128 = xlm_amount.saturated_into();
            let etr_u128 = xlm_u128.checked_mul(rate)
                .and_then(|v| v.checked_div(10_000_000)) // Stellar uses 7 decimals (stroops)
                .ok_or(Error::<T>::Overflow)?;

            Ok(etr_u128.saturated_into())
        }

        /// Process confirmed asset deposit
        fn process_asset_deposit(tx_hash: StellarTxHash, asset_id: u32) -> DispatchResult {
            let deposit = PendingDeposits::<T>::get(&tx_hash)
                .ok_or(Error::<T>::DepositNotFound)?;

            // Get asset rate
            let rate = AssetRates::<T>::get(asset_id);

            // Convert asset to ËTR
            let etr_amount = Self::convert_xlm_to_etr(deposit.amount, rate)?;

            // Mint ËTR
            let _ = T::Currency::deposit_creating(&deposit.etrid_account, etr_amount);

            // Emit event
            Self::deposit_event(Event::<T>::AssetDepositConfirmed {
                etrid_account: deposit.etrid_account,
                asset_id,
                amount: etr_amount,
                tx_hash,
            });

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, parameter_types};
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test {
            System: frame_system,
            Balances: pallet_balances,
            XlmBridge: pallet,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<u64>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
        type Nonce = u64;
        type Block = Block;
        type RuntimeTask = ();
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
    }

    parameter_types! {
        pub const ExistentialDeposit: u64 = 1;
    }

    impl pallet_balances::Config for Test {
        type MaxLocks = ();
        type MaxReserves = ();
        type ReserveIdentifier = [u8; 8];
        type Balance = u64;
        type RuntimeEvent = RuntimeEvent;
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = System;
        type WeightInfo = ();
        type FreezeIdentifier = ();
        type MaxFreezes = ();
        type RuntimeHoldReason = ();
        type RuntimeFreezeReason = ();
    }

    parameter_types! {
        pub const BridgeFee: Perbill = Perbill::from_rational(5u32, 1000u32); // 0.5%
        pub const MinBridgeAmount: u64 = 10_000_000; // 1 XLM (7 decimals)
        pub const MaxBridgeAmount: u64 = 1_000_000_000_000; // 100,000 XLM
        pub const XlmBridgePalletId: PalletId = PalletId(*b"xlm/brdg");
        pub const StellarConfirmations: u32 = 3;
        pub const MaxAssets: u32 = 100;
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type BridgeFee = BridgeFee;
        type MinBridgeAmount = MinBridgeAmount;
        type MaxBridgeAmount = MaxBridgeAmount;
        type PalletId = XlmBridgePalletId;
        type StellarConfirmations = StellarConfirmations;
        type MaxAssets = MaxAssets;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        t.into()
    }

    #[test]
    fn test_register_horizon_server() {
        new_test_ext().execute_with(|| {
            let server = 1u64;
            let url = b"https://horizon.stellar.org".to_vec().try_into().unwrap();
            
            assert_ok!(XlmBridge::register_horizon_server(
                RuntimeOrigin::signed(server),
                url
            ));
            
            assert!(pallet::HorizonServers::<Test>::contains_key(server));
        });
    }

    #[test]
    fn test_deposit_xlm() {
        new_test_ext().execute_with(|| {
            let beneficiary = 1u64;
            let amount = 10_000_000u64; // 1 XLM
            let tx_hash = [1u8; 32];
            
            // Register Horizon server
            let server = 2u64;
            let url = b"https://horizon.stellar.org".to_vec().try_into().unwrap();
            assert_ok!(XlmBridge::register_horizon_server(
                RuntimeOrigin::signed(server),
                url
            ));
            
            // Set bridge account
            let bridge_account = [5u8; 32];
            assert_ok!(XlmBridge::update_bridge_account(
                RuntimeOrigin::root(),
                bridge_account
            ));
            
            // Enable bridge
            assert_ok!(XlmBridge::set_bridge_status(RuntimeOrigin::root(), true));
            
            // Deposit
            assert_ok!(XlmBridge::deposit_xlm(
                RuntimeOrigin::signed(server),
                beneficiary,
                amount,
                tx_hash
            ));
            
            // Verify balance
            assert!(Balances::free_balance(beneficiary) > 0);
        });
    }
}
