#![cfg_attr(not(feature = "std"), no_std)]

//! # Polygon (MATIC) Bridge Pallet
//!
//! ## Overview
//! 
//! The Polygon Bridge pallet enables cross-chain communication between the Ëtrid FlareChain
//! and the Polygon network (MATIC). This bridge supports:
//! - EVM-compatible smart contract interactions
//! - Plasma Bridge protocol
//! - PoS (Proof of Stake) Bridge support
//! - ERC-20 token transfers (including MATIC and wrapped tokens)
//! - Low gas fees and fast finality (2-3 second block times)
//! - 128 block confirmations for security
//!
//! ## Polygon-Specific Features
//! 
//! - **Block Time:** 2-3 seconds (vs Ethereum's 12 seconds)
//! - **Confirmations:** 128 blocks for finality (~5-6 minutes)
//! - **Gas Fees:** ~100x cheaper than Ethereum mainnet
//! - **EVM Compatibility:** Full Ethereum tooling support
//! - **Token Standard:** ERC-20 (18 decimals for MATIC)
//! - **Bridge Types:** Plasma Bridge (faster) & PoS Bridge (more secure)

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, ReservableCurrency},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_core::{H160, H256};
    use sp_runtime::traits::{AccountIdConversion, SaturatedConversion, Saturating};
    use sp_std::vec::Vec;

    type BalanceOf<T> =
        <<T as pallet_etr_lock::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Polygon address type (EVM-compatible, 20 bytes)
    pub type PolygonAddress = H160;
    
    /// Polygon transaction hash (32 bytes)
    pub type PolygonTxHash = H256;

    /// Polygon bridge deposit record
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[codec(dumb_trait_bound)]
    pub struct PolygonDeposit<AccountId, Balance> {
        pub account: AccountId,
        pub polygon_address: PolygonAddress,
        pub amount: Balance,
        pub tx_hash: PolygonTxHash,
        pub block_number: u64,
        pub confirmations: u32,
        pub bridge_type: BridgeType,
    }

    /// Polygon bridge withdrawal record
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[codec(dumb_trait_bound)]
    pub struct PolygonWithdrawal<AccountId, Balance> {
        pub account: AccountId,
        pub polygon_address: PolygonAddress,
        pub amount: Balance,
        pub nonce: u64,
        pub status: WithdrawalStatus,
        pub bridge_type: BridgeType,
    }

    /// Bridge type selection
    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[repr(u8)]
    pub enum BridgeType {
        /// Plasma Bridge - Faster, uses fraud proofs
        Plasma = 0,
        /// PoS Bridge - More secure, uses validator checkpoints
        PoS = 1,
    }

    impl BridgeType {
        /// Convert to u8 for events and extrinsics
        pub fn to_u8(&self) -> u8 {
            match self {
                BridgeType::Plasma => 0,
                BridgeType::PoS => 1,
            }
        }

        /// Convert from u8
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(BridgeType::Plasma),
                1 => Some(BridgeType::PoS),
                _ => None,
            }
        }
    }

    /// Withdrawal status
    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[repr(u8)]
    pub enum WithdrawalStatus {
        Pending = 0,
        Confirmed = 1,
        Completed = 2,
        Failed = 3,
    }

    impl WithdrawalStatus {
        /// Convert to u8 for events
        pub fn to_u8(&self) -> u8 {
            match self {
                WithdrawalStatus::Pending => 0,
                WithdrawalStatus::Confirmed => 1,
                WithdrawalStatus::Completed => 2,
                WithdrawalStatus::Failed => 3,
            }
        }
    }

    /// ERC-20 token on Polygon
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, serde::Serialize, serde::Deserialize)]
    pub struct PolygonToken {
        pub contract_address: PolygonAddress,
        pub decimals: u8,
        pub symbol: BoundedVec<u8, ConstU32<10>>,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_etr_lock::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Note: Currency type is inherited from pallet_etr_lock::Config to avoid ambiguity

        /// Minimum number of Polygon block confirmations required (default: 128)
        #[pallet::constant]
        type MinConfirmations: Get<u32>;

        /// Bridge fee rate in basis points (e.g., 10 = 0.1%)
        #[pallet::constant]
        type BridgeFeeRate: Get<u32>;

        /// Maximum gas limit for Polygon transactions
        #[pallet::constant]
        type MaxGasLimit: Get<u64>;

        /// Minimum bridge amount (prevents dust attacks)
        #[pallet::constant]
        type MinBridgeAmount: Get<BalanceOf<Self>>;

        /// Maximum number of deposits per account
        #[pallet::constant]
        type MaxDepositsPerAccount: Get<u32>;

        /// Maximum number of withdrawals per account
        #[pallet::constant]
        type MaxWithdrawalsPerAccount: Get<u32>;

        /// Pallet ID for generating the bridge account
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    // ==================== STORAGE ====================

    /// Pending deposits from Polygon to Ëtrid
    #[pallet::storage]
    #[pallet::getter(fn pending_deposits)]
    pub type PendingDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<PolygonDeposit<T::AccountId, BalanceOf<T>>, T::MaxDepositsPerAccount>,
        ValueQuery,
    >;

    /// Pending withdrawals from Ëtrid to Polygon
    #[pallet::storage]
    #[pallet::getter(fn pending_withdrawals)]
    pub type PendingWithdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<PolygonWithdrawal<T::AccountId, BalanceOf<T>>, T::MaxWithdrawalsPerAccount>,
        ValueQuery,
    >;

    /// Total value locked in the bridge
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Withdrawal nonce counter
    #[pallet::storage]
    #[pallet::getter(fn withdrawal_nonce)]
    pub type WithdrawalNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Registered ERC-20 tokens on Polygon
    #[pallet::storage]
    #[pallet::getter(fn registered_tokens)]
    pub type RegisteredTokens<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PolygonAddress,
        PolygonToken,
        OptionQuery,
    >;

    /// Polygon bridge contract addresses
    #[pallet::storage]
    #[pallet::getter(fn bridge_contract)]
    pub type BridgeContract<T: Config> = StorageValue<_, PolygonAddress, OptionQuery>;

    /// PoS Bridge checkpoint manager address
    #[pallet::storage]
    #[pallet::getter(fn checkpoint_manager)]
    pub type CheckpointManager<T: Config> = StorageValue<_, PolygonAddress, OptionQuery>;

    /// Processed Polygon burn transactions (to prevent replay attacks)
    #[pallet::storage]
    #[pallet::getter(fn processed_polygon_burns)]
    pub type ProcessedPolygonBurns<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PolygonTxHash,
        bool,
        ValueQuery,
    >;

    // ==================== GENESIS ====================

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub bridge_contract: Option<PolygonAddress>,
        pub checkpoint_manager: Option<PolygonAddress>,
        pub registered_tokens: Vec<(PolygonAddress, PolygonToken)>,
        pub _phantom: sp_std::marker::PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                bridge_contract: None,
                checkpoint_manager: None,
                registered_tokens: Vec::new(),
                _phantom: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            if let Some(contract) = self.bridge_contract {
                BridgeContract::<T>::put(contract);
            }
            if let Some(manager) = self.checkpoint_manager {
                CheckpointManager::<T>::put(manager);
            }
            for (address, token) in &self.registered_tokens {
                RegisteredTokens::<T>::insert(address, token.clone());
            }
        }
    }

    // ==================== EVENTS ====================

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Deposit initiated from Polygon to Ëtrid
        /// bridge_type: 0 = Plasma, 1 = PoS
        DepositInitiated {
            account: T::AccountId,
            polygon_address: PolygonAddress,
            amount: BalanceOf<T>,
            tx_hash: PolygonTxHash,
            bridge_type: u8,
        },
        /// Deposit confirmed after required confirmations
        DepositConfirmed {
            account: T::AccountId,
            amount: BalanceOf<T>,
            confirmations: u32,
        },
        /// Deposit completed and funds released
        DepositCompleted {
            account: T::AccountId,
            amount: BalanceOf<T>,
            fee: BalanceOf<T>,
        },
        /// Withdrawal initiated from Ëtrid to Polygon
        /// bridge_type: 0 = Plasma, 1 = PoS
        WithdrawalInitiated {
            account: T::AccountId,
            polygon_address: PolygonAddress,
            amount: BalanceOf<T>,
            nonce: u64,
            bridge_type: u8,
        },
        /// Withdrawal confirmed on Polygon
        WithdrawalConfirmed {
            account: T::AccountId,
            nonce: u64,
            tx_hash: PolygonTxHash,
        },
        /// Withdrawal completed
        WithdrawalCompleted {
            account: T::AccountId,
            amount: BalanceOf<T>,
            nonce: u64,
        },
        /// Bridge contract address updated
        BridgeContractUpdated {
            old_address: Option<PolygonAddress>,
            new_address: PolygonAddress,
        },
        /// ERC-20 token registered
        TokenRegistered {
            contract_address: PolygonAddress,
            symbol: Vec<u8>,
            decimals: u8,
        },
        /// Bridge paused for emergency
        BridgePaused,
        /// Bridge unpaused
        BridgeUnpaused,
        /// ETR bridged to Polygon [from, amount, polygon_address]
        EtrBridgedToPolygon {
            from: T::AccountId,
            amount: BalanceOf<T>,
            polygon_address: PolygonAddress,
        },
        /// ETR unlocked from Polygon burn [to, amount, polygon_burn_tx]
        EtrUnlockedFromPolygon {
            to: T::AccountId,
            amount: BalanceOf<T>,
            polygon_burn_tx: PolygonTxHash,
        },
    }

    // ==================== ERRORS ====================

    #[pallet::error]
    pub enum Error<T> {
        /// Polygon address is invalid
        InvalidPolygonAddress,
        /// Amount is below minimum bridge amount
        AmountTooLow,
        /// Amount exceeds maximum allowed
        AmountTooHigh,
        /// Insufficient balance for bridging
        InsufficientBalance,
        /// Deposit not found
        DepositNotFound,
        /// Withdrawal not found
        WithdrawalNotFound,
        /// Not enough confirmations yet
        InsufficientConfirmations,
        /// Transaction hash already processed
        DuplicateTransaction,
        /// Bridge contract not set
        BridgeContractNotSet,
        /// Gas limit too high
        GasLimitTooHigh,
        /// Token not registered
        TokenNotRegistered,
        /// Too many pending deposits
        TooManyDeposits,
        /// Too many pending withdrawals
        TooManyWithdrawals,
        /// Bridge is paused
        BridgePaused,
        /// Invalid bridge type
        InvalidBridgeType,
        /// Checkpoint not verified (PoS Bridge)
        CheckpointNotVerified,
        /// Arithmetic overflow
        Overflow,
        /// Burn transaction already processed
        BurnAlreadyProcessed,
        /// Lock account not configured
        LockAccountNotSet,
    }

    // ==================== CALLS ====================

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initiate a deposit from Polygon to Ëtrid
        ///
        /// This is called by relayers monitoring the Polygon chain
        /// bridge_type_raw: 0 = Plasma, 1 = PoS
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn initiate_deposit(
            origin: OriginFor<T>,
            account: T::AccountId,
            polygon_address: PolygonAddress,
            amount: BalanceOf<T>,
            tx_hash: PolygonTxHash,
            block_number: u64,
            bridge_type_raw: u8,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Convert and validate bridge type
            let bridge_type = BridgeType::from_u8(bridge_type_raw)
                .ok_or(Error::<T>::InvalidBridgeType)?;

            // Validate amount
            ensure!(
                amount >= T::MinBridgeAmount::get(),
                Error::<T>::AmountTooLow
            );

            // Create deposit record
            let deposit = PolygonDeposit {
                account: account.clone(),
                polygon_address,
                amount,
                tx_hash,
                block_number,
                confirmations: 0,
                bridge_type,
            };

            // Add to pending deposits
            PendingDeposits::<T>::try_mutate(&account, |deposits| {
                deposits.try_push(deposit).map_err(|_| Error::<T>::TooManyDeposits)
            })?;

            Self::deposit_event(Event::DepositInitiated {
                account,
                polygon_address,
                amount,
                tx_hash,
                bridge_type: bridge_type_raw,
            });

            Ok(())
        }

        /// Confirm deposit after sufficient Polygon confirmations
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn confirm_deposit(
            origin: OriginFor<T>,
            account: T::AccountId,
            tx_hash: PolygonTxHash,
            confirmations: u32,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Verify minimum confirmations (128 blocks for Polygon)
            ensure!(
                confirmations >= T::MinConfirmations::get(),
                Error::<T>::InsufficientConfirmations
            );

            // Update deposit confirmations
            PendingDeposits::<T>::try_mutate(&account, |deposits| {
                let deposit = deposits
                    .iter_mut()
                    .find(|d| d.tx_hash == tx_hash)
                    .ok_or(Error::<T>::DepositNotFound)?;

                deposit.confirmations = confirmations;

                Self::deposit_event(Event::DepositConfirmed {
                    account: account.clone(),
                    amount: deposit.amount,
                    confirmations,
                });

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }

        /// Complete deposit and mint tokens to user
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn complete_deposit(
            origin: OriginFor<T>,
            account: T::AccountId,
            tx_hash: PolygonTxHash,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Get and remove deposit
            let deposit = PendingDeposits::<T>::try_mutate(&account, |deposits| {
                let pos = deposits
                    .iter()
                    .position(|d| d.tx_hash == tx_hash)
                    .ok_or(Error::<T>::DepositNotFound)?;

                let deposit = deposits.remove(pos);

                // Verify confirmations
                ensure!(
                    deposit.confirmations >= T::MinConfirmations::get(),
                    Error::<T>::InsufficientConfirmations
                );

                Ok::<_, DispatchError>(deposit)
            })?;

            // Calculate fee (e.g., 0.1% = 10 basis points)
            let fee_rate = T::BridgeFeeRate::get();
            let fee = deposit.amount * fee_rate.into() / 10000u32.into();
            let amount_after_fee = deposit.amount.saturating_sub(fee);

            // Mint tokens to user (in production, this would be from bridge reserve)
            let bridge_account = Self::bridge_account();
            <T as pallet_etr_lock::Config>::Currency::transfer(
                &bridge_account,
                &account,
                amount_after_fee,
                ExistenceRequirement::KeepAlive,
            )?;

            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_add(deposit.amount);
            });

            Self::deposit_event(Event::DepositCompleted {
                account,
                amount: amount_after_fee,
                fee,
            });

            Ok(())
        }

        /// Initiate withdrawal from Ëtrid to Polygon
        /// bridge_type_raw: 0 = Plasma, 1 = PoS
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn initiate_withdrawal(
            origin: OriginFor<T>,
            polygon_address: PolygonAddress,
            amount: BalanceOf<T>,
            bridge_type_raw: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Convert and validate bridge type
            let bridge_type = BridgeType::from_u8(bridge_type_raw)
                .ok_or(Error::<T>::InvalidBridgeType)?;

            // Validate amount
            ensure!(
                amount >= T::MinBridgeAmount::get(),
                Error::<T>::AmountTooLow
            );

            // Ensure user has sufficient balance
            ensure!(
                <T as pallet_etr_lock::Config>::Currency::free_balance(&who) >= amount,
                Error::<T>::InsufficientBalance
            );

            // Lock user's tokens
            <T as pallet_etr_lock::Config>::Currency::transfer(
                &who,
                &Self::bridge_account(),
                amount,
                ExistenceRequirement::AllowDeath,
            )?;

            // Generate nonce
            let nonce = WithdrawalNonce::<T>::get();
            WithdrawalNonce::<T>::put(nonce + 1);

            // Create withdrawal record
            let withdrawal = PolygonWithdrawal {
                account: who.clone(),
                polygon_address,
                amount,
                nonce,
                status: WithdrawalStatus::Pending,
                bridge_type,
            };

            // Add to pending withdrawals
            PendingWithdrawals::<T>::try_mutate(&who, |withdrawals| {
                withdrawals
                    .try_push(withdrawal)
                    .map_err(|_| Error::<T>::TooManyWithdrawals)
            })?;

            Self::deposit_event(Event::WithdrawalInitiated {
                account: who,
                polygon_address,
                amount,
                nonce,
                bridge_type: bridge_type_raw,
            });

            Ok(())
        }

        /// Confirm withdrawal was processed on Polygon
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn confirm_withdrawal(
            origin: OriginFor<T>,
            account: T::AccountId,
            nonce: u64,
            tx_hash: PolygonTxHash,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Update withdrawal status
            PendingWithdrawals::<T>::try_mutate(&account, |withdrawals| {
                let withdrawal = withdrawals
                    .iter_mut()
                    .find(|w| w.nonce == nonce)
                    .ok_or(Error::<T>::WithdrawalNotFound)?;

                withdrawal.status = WithdrawalStatus::Confirmed;

                Self::deposit_event(Event::WithdrawalConfirmed {
                    account: account.clone(),
                    nonce,
                    tx_hash,
                });

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }

        /// Complete withdrawal and update records
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn complete_withdrawal(
            origin: OriginFor<T>,
            account: T::AccountId,
            nonce: u64,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Remove withdrawal
            let withdrawal = PendingWithdrawals::<T>::try_mutate(&account, |withdrawals| {
                let pos = withdrawals
                    .iter()
                    .position(|w| w.nonce == nonce)
                    .ok_or(Error::<T>::WithdrawalNotFound)?;

                let withdrawal = withdrawals.remove(pos);

                // Verify status
                ensure!(
                    withdrawal.status == WithdrawalStatus::Confirmed,
                    Error::<T>::WithdrawalNotFound
                );

                Ok::<_, DispatchError>(withdrawal)
            })?;

            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_sub(withdrawal.amount);
            });

            Self::deposit_event(Event::WithdrawalCompleted {
                account,
                amount: withdrawal.amount,
                nonce,
            });

            Ok(())
        }

        /// Register a new ERC-20 token for bridging
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn register_token(
            origin: OriginFor<T>,
            contract_address: PolygonAddress,
            decimals: u8,
            symbol: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let bounded_symbol: BoundedVec<u8, ConstU32<10>> =
                symbol.clone().try_into().map_err(|_| Error::<T>::Overflow)?;

            let token = PolygonToken {
                contract_address,
                decimals,
                symbol: bounded_symbol,
            };

            RegisteredTokens::<T>::insert(contract_address, token);

            Self::deposit_event(Event::TokenRegistered {
                contract_address,
                symbol,
                decimals,
            });

            Ok(())
        }

        /// Update bridge contract address
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn update_bridge_contract(
            origin: OriginFor<T>,
            new_address: PolygonAddress,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let old_address = BridgeContract::<T>::get();
            BridgeContract::<T>::put(new_address);

            Self::deposit_event(Event::BridgeContractUpdated {
                old_address,
                new_address,
            });

            Ok(())
        }

        /// Update checkpoint manager address (PoS Bridge)
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn update_checkpoint_manager(
            origin: OriginFor<T>,
            new_address: PolygonAddress,
        ) -> DispatchResult {
            ensure_root(origin)?;

            CheckpointManager::<T>::put(new_address);

            Ok(())
        }

        /// Bridge ETR tokens to Polygon
        ///
        /// Locks ETR on FlareChain and emits event for relayer to mint on Polygon
        #[pallet::call_index(9)]
        #[pallet::weight(150_000)]
        pub fn bridge_etr_to_polygon(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            polygon_destination: PolygonAddress,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Convert Polygon address to bytes
            let destination_bytes = polygon_destination.as_bytes().to_vec();

            // Lock ETR using shared locking pallet
            pallet_etr_lock::Pallet::<T>::lock_for_bridge(
                frame_system::RawOrigin::Signed(who.clone()).into(),
                pallet_etr_lock::ChainId::Polygon,
                amount,
                destination_bytes.clone(),
            )?;

            // Emit event for relayer
            Self::deposit_event(Event::<T>::EtrBridgedToPolygon {
                from: who,
                amount,
                polygon_address: polygon_destination,
            });

            Ok(())
        }

        /// Process ETR burn from Polygon (called by relayer)
        ///
        /// Unlocks ETR on FlareChain when wrapped ETR is burned on Polygon
        #[pallet::call_index(10)]
        #[pallet::weight(150_000)]
        pub fn process_etr_burn_from_polygon(
            origin: OriginFor<T>,
            etrid_recipient: T::AccountId,
            amount: BalanceOf<T>,
            polygon_burn_tx: PolygonTxHash,
        ) -> DispatchResult {
            // Should be called by authorized relayer/oracle
            let _relayer = ensure_signed(origin)?;
            // TODO: Add relayer authorization check

            // Verify burn hasn't been processed
            ensure!(
                !ProcessedPolygonBurns::<T>::contains_key(&polygon_burn_tx),
                Error::<T>::BurnAlreadyProcessed
            );

            // Unlock ETR
            pallet_etr_lock::Pallet::<T>::unlock_from_bridge(
                frame_system::RawOrigin::Root.into(),
                pallet_etr_lock::ChainId::Polygon,
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
            ProcessedPolygonBurns::<T>::insert(&polygon_burn_tx, true);

            // Emit event
            Self::deposit_event(Event::<T>::EtrUnlockedFromPolygon {
                to: etrid_recipient,
                amount,
                polygon_burn_tx,
            });

            Ok(())
        }
    }

    // ==================== HELPERS ====================

    impl<T: Config> Pallet<T> {
        /// Get the bridge account ID
        pub fn bridge_account() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Convert Polygon wei (18 decimals) to Ëtrid native balance
        pub fn wei_to_native(wei: u128) -> BalanceOf<T> {
            // MATIC uses 18 decimals like ETH
            // Conversion factor: 1 MATIC = 1e18 wei = 1 ÉTR (assumed 1:1 peg)
            wei.saturated_into()
        }

        /// Convert Ëtrid native balance to Polygon wei
        pub fn native_to_wei(native: BalanceOf<T>) -> u128 {
            native.saturated_into()
        }
    }
}