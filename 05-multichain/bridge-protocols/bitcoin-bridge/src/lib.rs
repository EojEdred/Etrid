//! Bitcoin Bridge Pallet for Ëtrid
//! Location: 05-multichain/bridge-protocols/bitcoin-bridge/src/lib.rs
//!
//! Handles BTC <-> ËTR bridging for the BTC Partition Burst Chain
//! Implements the generic Bridge trait from partition-burst-chains/bridge
//!
//! ## Multi-Signature Custodian Security
//! This pallet implements M-of-N multi-signature custodian approval for critical operations:
//! - Withdrawal confirmations require custodian consensus
//! - Prevents single points of failure in bridge security
//! - Configurable threshold (e.g., 2-of-3, 3-of-5)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Saturating, SaturatedConversion, Hash};
    use sp_std::vec::Vec;
    use etrid_bridge_common::multisig::{MultiSigCustodian, PendingApproval};

    // Import the generic Bridge trait
    // use etrid_bridge_interface::BridgeTrait;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        
        /// Minimum BTC confirmations required
        #[pallet::constant]
        type MinConfirmations: Get<u32>;
        
        /// Minimum deposit amount (in satoshis)
        #[pallet::constant]
        type MinDepositAmount: Get<u64>;
        
        /// Maximum deposit amount (in satoshis)
        #[pallet::constant]
        type MaxDepositAmount: Get<u64>;
        
        /// Bridge authority account (multisig)
        type BridgeAuthority: Get<Self::AccountId>;
    }

    /// BTC deposit request
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct DepositRequest<AccountId, Balance> {
        pub depositor: AccountId,
        pub btc_address: BoundedVec<u8, ConstU32<64>>,
        pub btc_txid: BoundedVec<u8, ConstU32<64>>,
        pub amount_satoshi: u64,
        pub amount_etr: Balance,
        pub confirmations: u32,
        pub status: DepositStatus,
        pub block_height: u32,
    }

    /// BTC withdrawal request
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct WithdrawalRequest<AccountId, Balance> {
        pub withdrawer: AccountId,
        pub btc_address: BoundedVec<u8, ConstU32<64>>,
        pub amount_satoshi: u64,
        pub amount_etr: Balance,
        pub status: WithdrawalStatus,
        pub btc_txid: Option<BoundedVec<u8, ConstU32<64>>>,
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

    /// Pending BTC deposits
    #[pallet::storage]
    #[pallet::getter(fn deposits)]
    pub type Deposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, ConstU32<64>>, // BTC txid
        DepositRequest<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// Pending BTC withdrawals
    #[pallet::storage]
    #[pallet::getter(fn withdrawals)]
    pub type Withdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        WithdrawalRequest<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// BTC to ETR exchange rate (satoshi per ETR, scaled by 1e8)
    #[pallet::storage]
    #[pallet::getter(fn exchange_rate)]
    pub type ExchangeRate<T> = StorageValue<_, u64, ValueQuery>;

    /// Total BTC locked in bridge (in satoshis)
    #[pallet::storage]
    #[pallet::getter(fn total_btc_locked)]
    pub type TotalBtcLocked<T> = StorageValue<_, u64, ValueQuery>;

    /// Total ETR minted via bridge
    #[pallet::storage]
    #[pallet::getter(fn total_etr_minted)]
    pub type TotalEtrMinted<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Multi-sig custodian set for bridge operations
    #[pallet::storage]
    #[pallet::getter(fn custodian_set)]
    pub type CustodianSet<T: Config> = StorageValue<_, MultiSigCustodian<T::AccountId>, OptionQuery>;

    /// Pending multi-sig approvals for withdrawals
    /// Maps withdrawal operation hash to pending approval state
    #[pallet::storage]
    #[pallet::getter(fn pending_approvals)]
    pub type PendingApprovals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        PendingApproval<T::AccountId, T::Hash>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// BTC deposit detected [depositor, btc_txid, amount_satoshi]
        DepositDetected(T::AccountId, Vec<u8>, u64),
        /// BTC deposit confirmed [depositor, btc_txid, amount_etr]
        DepositConfirmed(T::AccountId, Vec<u8>, BalanceOf<T>),
        /// ETR minted for BTC deposit [depositor, amount_etr]
        EtrMinted(T::AccountId, BalanceOf<T>),
        /// BTC withdrawal requested [withdrawer, btc_address, amount_satoshi]
        WithdrawalRequested(T::AccountId, Vec<u8>, u64),
        /// ETR burned for BTC withdrawal [withdrawer, amount_etr]
        EtrBurned(T::AccountId, BalanceOf<T>),
        /// BTC withdrawal completed [withdrawer, btc_txid]
        WithdrawalCompleted(T::AccountId, Vec<u8>),
        /// Exchange rate updated [new_rate]
        ExchangeRateUpdated(u64),
        /// Custodian set updated [threshold]
        CustodianSetUpdated(u32),
        /// Withdrawal approval submitted [operation_hash, custodian, approvals_count]
        WithdrawalApprovalSubmitted(T::Hash, T::AccountId, u32),
        /// Withdrawal approved and executed [operation_hash, withdrawer]
        WithdrawalApprovedAndExecuted(T::Hash, T::AccountId),
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
        /// Invalid BTC address
        InvalidBtcAddress,
        /// Invalid BTC transaction ID
        InvalidBtcTxId,
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
        /// No custodian set configured
        NoCustodianSet,
        /// Not a custodian
        NotCustodian,
        /// Unknown operation
        UnknownOperation,
        /// Already executed
        AlreadyExecuted,
        /// Already approved by this custodian
        AlreadyApproved,
        /// Invalid custodian set configuration
        InvalidCustodianSet,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_exchange_rate: u64,
        pub _phantom: PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                initial_exchange_rate: 100_000_000, // 1 BTC = 1 ETR (scaled by 1e8)
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
        /// Register a BTC deposit (called by bridge relayer)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn deposit_btc(
            origin: OriginFor<T>,
            depositor: T::AccountId,
            btc_address: Vec<u8>,
            btc_txid: Vec<u8>,
            amount_satoshi: u64,
            block_height: u32,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller == T::BridgeAuthority::get(), Error::<T>::NotAuthorized);

            // Validate inputs
            ensure!(amount_satoshi >= T::MinDepositAmount::get(), Error::<T>::AmountBelowMinimum);
            ensure!(amount_satoshi <= T::MaxDepositAmount::get(), Error::<T>::AmountAboveMaximum);
            ensure!(btc_address.len() <= 64, Error::<T>::InvalidBtcAddress);
            ensure!(btc_txid.len() <= 64, Error::<T>::InvalidBtcTxId);

            let btc_txid_bounded: BoundedVec<u8, ConstU32<64>> = btc_txid.clone().try_into()
                .map_err(|_| Error::<T>::InvalidBtcTxId)?;
            
            ensure!(!Deposits::<T>::contains_key(&btc_txid_bounded), Error::<T>::DepositAlreadyExists);

            // Calculate ETR amount
            let exchange_rate = Self::exchange_rate();
            ensure!(exchange_rate > 0, Error::<T>::ExchangeRateNotSet);
            
            let amount_etr = Self::satoshi_to_etr(amount_satoshi, exchange_rate)?;

            // Create deposit request
            let btc_address_bounded: BoundedVec<u8, ConstU32<64>> = btc_address.clone().try_into()
                .map_err(|_| Error::<T>::InvalidBtcAddress)?;

            let deposit = DepositRequest {
                depositor: depositor.clone(),
                btc_address: btc_address_bounded,
                btc_txid: btc_txid_bounded.clone(),
                amount_satoshi,
                amount_etr,
                confirmations: 0,
                status: DepositStatus::Pending,
                block_height,
            };

            Deposits::<T>::insert(&btc_txid_bounded, deposit);

            Self::deposit_event(Event::DepositDetected(depositor, btc_txid, amount_satoshi));

            Ok(())
        }

        /// Confirm a BTC deposit with sufficient confirmations
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn confirm_deposit(
            origin: OriginFor<T>,
            btc_txid: Vec<u8>,
            confirmations: u32,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller == T::BridgeAuthority::get(), Error::<T>::NotAuthorized);

            let btc_txid_bounded: BoundedVec<u8, ConstU32<64>> = btc_txid.clone().try_into()
                .map_err(|_| Error::<T>::InvalidBtcTxId)?;

            let mut deposit = Deposits::<T>::get(&btc_txid_bounded)
                .ok_or(Error::<T>::DepositNotFound)?;

            ensure!(deposit.status == DepositStatus::Pending, Error::<T>::InvalidStatusTransition);
            ensure!(confirmations >= T::MinConfirmations::get(), Error::<T>::InsufficientConfirmations);

            deposit.confirmations = confirmations;
            deposit.status = DepositStatus::Confirmed;

            Deposits::<T>::insert(&btc_txid_bounded, deposit.clone());

            // Mint ETR
            T::Currency::deposit_creating(&deposit.depositor, deposit.amount_etr);

            // Update totals
            TotalBtcLocked::<T>::mutate(|total| *total = total.saturating_add(deposit.amount_satoshi));
            TotalEtrMinted::<T>::mutate(|total| *total = total.saturating_add(deposit.amount_etr));

            // Update status
            deposit.status = DepositStatus::Minted;
            Deposits::<T>::insert(&btc_txid_bounded, deposit.clone());

            Self::deposit_event(Event::DepositConfirmed(deposit.depositor.clone(), btc_txid.clone(), deposit.amount_etr));
            Self::deposit_event(Event::EtrMinted(deposit.depositor, deposit.amount_etr));

            Ok(())
        }

        /// Request BTC withdrawal (burn ETR)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn withdraw_btc(
            origin: OriginFor<T>,
            btc_address: Vec<u8>,
            amount_satoshi: u64,
        ) -> DispatchResult {
            let withdrawer = ensure_signed(origin)?;

            // Validate inputs
            ensure!(amount_satoshi >= T::MinDepositAmount::get(), Error::<T>::AmountBelowMinimum);
            ensure!(btc_address.len() <= 64, Error::<T>::InvalidBtcAddress);
            ensure!(!Withdrawals::<T>::contains_key(&withdrawer), Error::<T>::WithdrawalAlreadyExists);

            // Calculate ETR amount to burn
            let exchange_rate = Self::exchange_rate();
            ensure!(exchange_rate > 0, Error::<T>::ExchangeRateNotSet);
            let amount_etr = Self::satoshi_to_etr(amount_satoshi, exchange_rate)?;

            // Burn ETR
            T::Currency::withdraw(
                &withdrawer,
                amount_etr,
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;

            let btc_address_bounded: BoundedVec<u8, ConstU32<64>> = btc_address.clone().try_into()
                .map_err(|_| Error::<T>::InvalidBtcAddress)?;

            // Create withdrawal request
            let withdrawal = WithdrawalRequest {
                withdrawer: withdrawer.clone(),
                btc_address: btc_address_bounded,
                amount_satoshi,
                amount_etr,
                status: WithdrawalStatus::Requested,
                btc_txid: None,
            };

            Withdrawals::<T>::insert(&withdrawer, withdrawal);

            // Update totals
            TotalBtcLocked::<T>::mutate(|total| *total = total.saturating_sub(amount_satoshi));
            TotalEtrMinted::<T>::mutate(|total| *total = total.saturating_sub(amount_etr));

            Self::deposit_event(Event::WithdrawalRequested(withdrawer.clone(), btc_address, amount_satoshi));
            Self::deposit_event(Event::EtrBurned(withdrawer, amount_etr));

            Ok(())
        }

        /// Confirm BTC withdrawal completed (called by bridge relayer)
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn confirm_withdrawal(
            origin: OriginFor<T>,
            withdrawer: T::AccountId,
            btc_txid: Vec<u8>,
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

            let btc_txid_bounded: BoundedVec<u8, ConstU32<64>> = btc_txid.clone().try_into()
                .map_err(|_| Error::<T>::InvalidBtcTxId)?;

            withdrawal.status = WithdrawalStatus::Completed;
            withdrawal.btc_txid = Some(btc_txid_bounded);

            Withdrawals::<T>::insert(&withdrawer, withdrawal);

            Self::deposit_event(Event::WithdrawalCompleted(withdrawer, btc_txid));

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

        /// Set custodian set (governance only)
        ///
        /// # Parameters
        /// - `origin`: Root origin (governance)
        /// - `custodians`: Vector of custodian accounts
        /// - `threshold`: Number of approvals required (M in M-of-N)
        ///
        /// # Example
        /// ```ignore
        /// // Setup 2-of-3 multisig
        /// set_custodians(root, vec![alice, bob, charlie], 2)?;
        /// ```
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn set_custodians(
            origin: OriginFor<T>,
            custodians: Vec<T::AccountId>,
            threshold: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Validate and create custodian set
            let custodian_set = MultiSigCustodian::new(custodians, threshold)
                .map_err(|_| Error::<T>::InvalidCustodianSet)?;

            CustodianSet::<T>::put(custodian_set);

            Self::deposit_event(Event::CustodianSetUpdated(threshold));

            Ok(())
        }

        /// Approve a BTC withdrawal (custodian only)
        ///
        /// # Parameters
        /// - `origin`: Custodian account
        /// - `withdrawer`: Account requesting withdrawal
        /// - `btc_txid`: Bitcoin transaction ID for the withdrawal
        ///
        /// # Security
        /// - Only custodians can approve
        /// - Prevents duplicate approvals
        /// - Executes withdrawal when threshold reached
        /// - Marks operation as executed to prevent re-execution
        ///
        /// # Example
        /// ```ignore
        /// // Custodian 1 approves
        /// approve_withdrawal(custodian1, alice, btc_txid)?;
        /// // Custodian 2 approves (reaches 2-of-3 threshold, executes)
        /// approve_withdrawal(custodian2, alice, btc_txid)?;
        /// ```
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn approve_withdrawal(
            origin: OriginFor<T>,
            withdrawer: T::AccountId,
            btc_txid: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check custodian set exists
            let custodian_set = CustodianSet::<T>::get().ok_or(Error::<T>::NoCustodianSet)?;

            // Check caller is custodian
            ensure!(custodian_set.is_custodian(&who), Error::<T>::NotCustodian);

            // Check withdrawal exists
            let _withdrawal = Withdrawals::<T>::get(&withdrawer)
                .ok_or(Error::<T>::WithdrawalNotFound)?;

            // Create operation hash from withdrawer + btc_txid
            let operation_data = (withdrawer.clone(), btc_txid.clone()).encode();
            let operation_hash = T::Hashing::hash(&operation_data);

            // Get or create pending approval
            let mut pending = PendingApprovals::<T>::get(&operation_hash)
                .unwrap_or_else(|| {
                    PendingApproval::new(operation_hash, custodian_set.threshold)
                });

            // Check not already executed
            ensure!(!pending.executed, Error::<T>::AlreadyExecuted);

            // Check not already approved by this custodian
            ensure!(!pending.approvals.contains(&who), Error::<T>::AlreadyApproved);

            // Add approval
            pending.approvals.push(who.clone());

            let approvals_count = pending.approvals.len() as u32;

            Self::deposit_event(Event::WithdrawalApprovalSubmitted(
                operation_hash,
                who,
                approvals_count,
            ));

            // Check if threshold reached
            if custodian_set.has_threshold(&pending.approvals) {
                // Mark as executed
                pending.executed = true;

                // Execute withdrawal confirmation
                Self::execute_withdrawal_confirmation(withdrawer.clone(), btc_txid.clone())?;

                Self::deposit_event(Event::WithdrawalApprovedAndExecuted(
                    operation_hash,
                    withdrawer,
                ));
            }

            // Store pending approval
            PendingApprovals::<T>::insert(operation_hash, pending);

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Execute withdrawal confirmation (internal helper)
        fn execute_withdrawal_confirmation(
            withdrawer: T::AccountId,
            btc_txid: Vec<u8>,
        ) -> DispatchResult {
            let mut withdrawal = Withdrawals::<T>::get(&withdrawer)
                .ok_or(Error::<T>::WithdrawalNotFound)?;

            ensure!(
                withdrawal.status == WithdrawalStatus::Requested ||
                withdrawal.status == WithdrawalStatus::Processing,
                Error::<T>::InvalidStatusTransition
            );

            let btc_txid_bounded: BoundedVec<u8, ConstU32<64>> = btc_txid.clone().try_into()
                .map_err(|_| Error::<T>::InvalidBtcTxId)?;

            withdrawal.status = WithdrawalStatus::Completed;
            withdrawal.btc_txid = Some(btc_txid_bounded);

            Withdrawals::<T>::insert(&withdrawer, withdrawal);

            Self::deposit_event(Event::WithdrawalCompleted(withdrawer, btc_txid));

            Ok(())
        }

        /// Convert satoshi to ETR using exchange rate
        fn satoshi_to_etr(amount_satoshi: u64, exchange_rate: u64) -> Result<BalanceOf<T>, Error<T>> {
            // exchange_rate is satoshi per ETR, scaled by 1e8
            // amount_etr = (amount_satoshi * 1e8) / exchange_rate
            
            let scaled_amount = (amount_satoshi as u128)
                .checked_mul(100_000_000)
                .ok_or(Error::<T>::ArithmeticOverflow)?;
            
            let amount_etr_u128 = scaled_amount
                .checked_div(exchange_rate as u128)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            let amount_etr = amount_etr_u128.saturated_into();

            Ok(amount_etr)
        }
    }

    // Implement generic Bridge trait
    // impl<T: Config> BridgeTrait for Pallet<T> {
    //     type AccountId = T::AccountId;
    //     type Balance = BalanceOf<T>;

    //     fn total_locked() -> u64 {
    //         Self::total_btc_locked()
    //     }

    //     fn total_minted() -> Self::Balance {
    //         Self::total_etr_minted()
    //     }
    // }
}
