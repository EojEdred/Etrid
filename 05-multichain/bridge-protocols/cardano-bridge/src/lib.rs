//! Cardano Bridge Pallet for Ëtrid
//! Location: 05-multichain/bridge-protocols/cardano-bridge/src/lib.rs
//!
//! Handles ADA <-> ËTR bridging for the ADA Partition Burst Chain
//! Implements Cardano-specific features:
//! - Extended UTXO (eUTXO) model
//! - Ouroboros Proof-of-Stake consensus
//! - Plutus smart contracts
//! - Native tokens support
//! - Hydra layer 2 compatibility
//! - ADA lovelaces (6 decimals: 1 ADA = 1,000,000 lovelaces)
//! - Catalyst governance integration

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedMul, CheckedDiv, Saturating, SaturatedConversion};
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// Minimum ADA confirmations required (Cardano finality ~15 epochs)
        #[pallet::constant]
        type MinConfirmations: Get<u32>;

        /// Minimum deposit amount (in lovelaces: 1 ADA = 1,000,000 lovelaces)
        #[pallet::constant]
        type MinDepositAmount: Get<u64>;

        /// Maximum deposit amount (in lovelaces)
        #[pallet::constant]
        type MaxDepositAmount: Get<u64>;

        /// Bridge authority account (multisig)
        type BridgeAuthority: Get<Self::AccountId>;
    }

    /// ADA deposit request (eUTXO model)
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct DepositRequest<AccountId, Balance> {
        pub depositor: AccountId,
        pub cardano_address: BoundedVec<u8, ConstU32<128>>, // Shelley addresses up to 128 bytes
        pub cardano_txid: BoundedVec<u8, ConstU32<64>>, // Transaction ID
        pub amount_lovelace: u64, // ADA amount in lovelaces (6 decimals)
        pub amount_etr: Balance,
        pub confirmations: u32,
        pub status: DepositStatus,
        pub epoch: u32, // Cardano epoch number
        pub plutus_datum: Option<BoundedVec<u8, ConstU32<256>>>, // Plutus script datum
    }

    /// ADA withdrawal request (eUTXO model)
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct WithdrawalRequest<AccountId, Balance> {
        pub withdrawer: AccountId,
        pub cardano_address: BoundedVec<u8, ConstU32<128>>,
        pub amount_lovelace: u64,
        pub amount_etr: Balance,
        pub status: WithdrawalStatus,
        pub cardano_txid: Option<BoundedVec<u8, ConstU32<64>>>,
        pub plutus_redeemer: Option<BoundedVec<u8, ConstU32<256>>>, // Plutus script redeemer
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum DepositStatus {
        Pending,
        Confirmed,
        Minted,
        Failed,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum WithdrawalStatus {
        Requested,
        Processing,
        Completed,
        Failed,
    }

    /// Pending ADA deposits
    #[pallet::storage]
    #[pallet::getter(fn deposits)]
    pub type Deposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, ConstU32<64>>, // Cardano txid
        DepositRequest<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// Pending ADA withdrawals
    #[pallet::storage]
    #[pallet::getter(fn withdrawals)]
    pub type Withdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        WithdrawalRequest<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// ADA to ETR exchange rate (lovelaces per ETR, scaled by 1e6)
    #[pallet::storage]
    #[pallet::getter(fn exchange_rate)]
    pub type ExchangeRate<T> = StorageValue<_, u64, ValueQuery>;

    /// Total ADA locked in bridge (in lovelaces)
    #[pallet::storage]
    #[pallet::getter(fn total_ada_locked)]
    pub type TotalAdaLocked<T> = StorageValue<_, u64, ValueQuery>;

    /// Total ETR minted via bridge
    #[pallet::storage]
    #[pallet::getter(fn total_etr_minted)]
    pub type TotalEtrMinted<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Plutus script hash for bridge contract
    #[pallet::storage]
    #[pallet::getter(fn plutus_script_hash)]
    pub type PlutusScriptHash<T> = StorageValue<_, BoundedVec<u8, ConstU32<64>>, OptionQuery>;

    /// Cardano Native Tokens registry (PolicyId -> AssetId mapping)
    #[pallet::storage]
    #[pallet::getter(fn native_tokens)]
    pub type NativeTokens<T> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, ConstU32<28>>, // Policy ID
        u32, // Asset ID
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// ADA deposit detected [depositor, cardano_txid, amount_lovelace]
        DepositDetected(T::AccountId, Vec<u8>, u64),
        /// ADA deposit confirmed [depositor, cardano_txid, amount_etr]
        DepositConfirmed(T::AccountId, Vec<u8>, BalanceOf<T>),
        /// ETR minted for ADA deposit [depositor, amount_etr]
        EtrMinted(T::AccountId, BalanceOf<T>),
        /// ADA withdrawal requested [withdrawer, cardano_address, amount_lovelace]
        WithdrawalRequested(T::AccountId, Vec<u8>, u64),
        /// ETR burned for ADA withdrawal [withdrawer, amount_etr]
        EtrBurned(T::AccountId, BalanceOf<T>),
        /// ADA withdrawal completed [withdrawer, cardano_txid]
        WithdrawalCompleted(T::AccountId, Vec<u8>),
        /// Exchange rate updated [new_rate]
        ExchangeRateUpdated(u64),
        /// Plutus script hash updated [script_hash]
        PlutusScriptUpdated(Vec<u8>),
        /// Native token registered [policy_id, asset_id]
        NativeTokenRegistered(Vec<u8>, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Deposit already exists
        DepositAlreadyExists,
        /// Deposit not found
        DepositNotFound,
        /// Withdrawal already exists
        WithdrawalAlreadyExists,
        /// Withdrawal not found
        WithdrawalNotFound,
        /// Insufficient confirmations
        InsufficientConfirmations,
        /// Amount below minimum
        AmountBelowMinimum,
        /// Amount above maximum
        AmountAboveMaximum,
        /// Invalid Cardano address
        InvalidCardanoAddress,
        /// Invalid Cardano transaction ID
        InvalidCardanoTxId,
        /// Exchange rate not set
        ExchangeRateNotSet,
        /// Arithmetic overflow
        ArithmeticOverflow,
        /// Insufficient balance
        InsufficientBalance,
        /// Not authorized
        NotAuthorized,
        /// Invalid status transition
        InvalidStatusTransition,
        /// Invalid Plutus script
        InvalidPlutusScript,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_exchange_rate: u64,
        pub _phantom: PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                initial_exchange_rate: 1_000_000, // 1 ADA = 1 ETR (scaled by 1e6 lovelaces)
                _phantom: PhantomData,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            ExchangeRate::<T>::put(self.initial_exchange_rate);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register an ADA deposit (called by bridge relayer)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn deposit_ada(
            origin: OriginFor<T>,
            depositor: T::AccountId,
            cardano_address: Vec<u8>,
            cardano_txid: Vec<u8>,
            amount_lovelace: u64,
            epoch: u32,
            plutus_datum: Option<Vec<u8>>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller == T::BridgeAuthority::get(), Error::<T>::NotAuthorized);

            // Validate inputs
            ensure!(amount_lovelace >= T::MinDepositAmount::get(), Error::<T>::AmountBelowMinimum);
            ensure!(amount_lovelace <= T::MaxDepositAmount::get(), Error::<T>::AmountAboveMaximum);
            ensure!(cardano_address.len() <= 128, Error::<T>::InvalidCardanoAddress);
            ensure!(cardano_txid.len() <= 64, Error::<T>::InvalidCardanoTxId);

            let cardano_txid_bounded: BoundedVec<u8, ConstU32<64>> = cardano_txid.clone().try_into()
                .map_err(|_| Error::<T>::InvalidCardanoTxId)?;

            ensure!(!Deposits::<T>::contains_key(&cardano_txid_bounded), Error::<T>::DepositAlreadyExists);

            // Calculate ETR amount
            let exchange_rate = Self::exchange_rate();
            ensure!(exchange_rate > 0, Error::<T>::ExchangeRateNotSet);

            let amount_etr = Self::lovelace_to_etr(amount_lovelace, exchange_rate)?;

            // Create deposit request
            let cardano_address_bounded: BoundedVec<u8, ConstU32<128>> = cardano_address.clone().try_into()
                .map_err(|_| Error::<T>::InvalidCardanoAddress)?;

            let plutus_datum_bounded = if let Some(datum) = plutus_datum {
                Some(datum.try_into().map_err(|_| Error::<T>::InvalidPlutusScript)?)
            } else {
                None
            };

            let deposit = DepositRequest {
                depositor: depositor.clone(),
                cardano_address: cardano_address_bounded,
                cardano_txid: cardano_txid_bounded.clone(),
                amount_lovelace,
                amount_etr,
                confirmations: 0,
                status: DepositStatus::Pending,
                epoch,
                plutus_datum: plutus_datum_bounded,
            };

            Deposits::<T>::insert(&cardano_txid_bounded, deposit);

            Self::deposit_event(Event::DepositDetected(depositor, cardano_txid, amount_lovelace));

            Ok(())
        }

        /// Confirm an ADA deposit with sufficient confirmations
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn confirm_deposit(
            origin: OriginFor<T>,
            cardano_txid: Vec<u8>,
            confirmations: u32,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller == T::BridgeAuthority::get(), Error::<T>::NotAuthorized);

            let cardano_txid_bounded: BoundedVec<u8, ConstU32<64>> = cardano_txid.clone().try_into()
                .map_err(|_| Error::<T>::InvalidCardanoTxId)?;

            let mut deposit = Deposits::<T>::get(&cardano_txid_bounded)
                .ok_or(Error::<T>::DepositNotFound)?;

            ensure!(deposit.status == DepositStatus::Pending, Error::<T>::InvalidStatusTransition);
            ensure!(confirmations >= T::MinConfirmations::get(), Error::<T>::InsufficientConfirmations);

            deposit.confirmations = confirmations;
            deposit.status = DepositStatus::Confirmed;

            Deposits::<T>::insert(&cardano_txid_bounded, deposit.clone());

            // Mint ETR
            T::Currency::deposit_creating(&deposit.depositor, deposit.amount_etr);

            // Update totals
            TotalAdaLocked::<T>::mutate(|total| *total = total.saturating_add(deposit.amount_lovelace));
            TotalEtrMinted::<T>::mutate(|total| *total = total.saturating_add(deposit.amount_etr));

            // Update status
            deposit.status = DepositStatus::Minted;
            Deposits::<T>::insert(&cardano_txid_bounded, deposit.clone());

            Self::deposit_event(Event::DepositConfirmed(deposit.depositor.clone(), cardano_txid.clone(), deposit.amount_etr));
            Self::deposit_event(Event::EtrMinted(deposit.depositor, deposit.amount_etr));

            Ok(())
        }

        /// Request ADA withdrawal (burn ETR)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn withdraw_ada(
            origin: OriginFor<T>,
            cardano_address: Vec<u8>,
            amount_lovelace: u64,
            plutus_redeemer: Option<Vec<u8>>,
        ) -> DispatchResult {
            let withdrawer = ensure_signed(origin)?;

            // Validate inputs
            ensure!(amount_lovelace >= T::MinDepositAmount::get(), Error::<T>::AmountBelowMinimum);
            ensure!(cardano_address.len() <= 128, Error::<T>::InvalidCardanoAddress);
            ensure!(!Withdrawals::<T>::contains_key(&withdrawer), Error::<T>::WithdrawalAlreadyExists);

            // Calculate ETR amount to burn
            let exchange_rate = Self::exchange_rate();
            ensure!(exchange_rate > 0, Error::<T>::ExchangeRateNotSet);
            let amount_etr = Self::lovelace_to_etr(amount_lovelace, exchange_rate)?;

            // Burn ETR
            T::Currency::withdraw(
                &withdrawer,
                amount_etr,
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;

            let cardano_address_bounded: BoundedVec<u8, ConstU32<128>> = cardano_address.clone().try_into()
                .map_err(|_| Error::<T>::InvalidCardanoAddress)?;

            let plutus_redeemer_bounded = if let Some(redeemer) = plutus_redeemer {
                Some(redeemer.try_into().map_err(|_| Error::<T>::InvalidPlutusScript)?)
            } else {
                None
            };

            // Create withdrawal request
            let withdrawal = WithdrawalRequest {
                withdrawer: withdrawer.clone(),
                cardano_address: cardano_address_bounded,
                amount_lovelace,
                amount_etr,
                status: WithdrawalStatus::Requested,
                cardano_txid: None,
                plutus_redeemer: plutus_redeemer_bounded,
            };

            Withdrawals::<T>::insert(&withdrawer, withdrawal);

            // Update totals
            TotalAdaLocked::<T>::mutate(|total| *total = total.saturating_sub(amount_lovelace));
            TotalEtrMinted::<T>::mutate(|total| *total = total.saturating_sub(amount_etr));

            Self::deposit_event(Event::WithdrawalRequested(withdrawer.clone(), cardano_address, amount_lovelace));
            Self::deposit_event(Event::EtrBurned(withdrawer, amount_etr));

            Ok(())
        }

        /// Confirm ADA withdrawal completed (called by bridge relayer)
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn confirm_withdrawal(
            origin: OriginFor<T>,
            withdrawer: T::AccountId,
            cardano_txid: Vec<u8>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller == T::BridgeAuthority::get(), Error::<T>::NotAuthorized);

            let mut withdrawal = Withdrawals::<T>::get(&withdrawer)
                .ok_or(Error::<T>::WithdrawalNotFound)?;

            ensure!(
                withdrawal.status == WithdrawalStatus::Requested ||
                withdrawal.status == WithdrawalStatus::Processing,
                Error::<T>::InvalidStatusTransition
            );

            let cardano_txid_bounded: BoundedVec<u8, ConstU32<64>> = cardano_txid.clone().try_into()
                .map_err(|_| Error::<T>::InvalidCardanoTxId)?;

            withdrawal.status = WithdrawalStatus::Completed;
            withdrawal.cardano_txid = Some(cardano_txid_bounded);

            Withdrawals::<T>::insert(&withdrawer, withdrawal);

            Self::deposit_event(Event::WithdrawalCompleted(withdrawer, cardano_txid));

            Ok(())
        }

        /// Set exchange rate (governance only)
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn set_exchange_rate(
            origin: OriginFor<T>,
            new_rate: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(new_rate > 0, Error::<T>::ExchangeRateNotSet);

            ExchangeRate::<T>::put(new_rate);

            Self::deposit_event(Event::ExchangeRateUpdated(new_rate));

            Ok(())
        }

        /// Update Plutus script hash (governance only)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn set_plutus_script(
            origin: OriginFor<T>,
            script_hash: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let script_hash_bounded: BoundedVec<u8, ConstU32<64>> = script_hash.clone().try_into()
                .map_err(|_| Error::<T>::InvalidPlutusScript)?;

            PlutusScriptHash::<T>::put(script_hash_bounded);

            Self::deposit_event(Event::PlutusScriptUpdated(script_hash));

            Ok(())
        }

        /// Register Cardano Native Token (governance only)
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn register_native_token(
            origin: OriginFor<T>,
            policy_id: Vec<u8>,
            asset_id: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(policy_id.len() == 28, Error::<T>::InvalidCardanoTxId);

            let policy_id_bounded: BoundedVec<u8, ConstU32<28>> = policy_id.clone().try_into()
                .map_err(|_| Error::<T>::InvalidCardanoTxId)?;

            NativeTokens::<T>::insert(&policy_id_bounded, asset_id);

            Self::deposit_event(Event::NativeTokenRegistered(policy_id, asset_id));

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Convert lovelaces to ETR using exchange rate
        fn lovelace_to_etr(amount_lovelace: u64, exchange_rate: u64) -> Result<BalanceOf<T>, Error<T>> {
            // exchange_rate is lovelaces per ETR, scaled by 1e6
            // amount_etr = (amount_lovelace * 1e6) / exchange_rate

            let scaled_amount = (amount_lovelace as u128)
                .checked_mul(1_000_000)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            let amount_etr_u128 = scaled_amount
                .checked_div(exchange_rate as u128)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            let amount_etr = amount_etr_u128.saturated_into();

            Ok(amount_etr)
        }
    }
}
