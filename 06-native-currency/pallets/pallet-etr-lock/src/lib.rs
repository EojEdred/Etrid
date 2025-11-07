#![cfg_attr(not(feature = "std"), no_std)]

//! # ETR Lock Pallet
//!
//! Manages ETR token locking for cross-chain PBC bridges.

pub use codec::{Encode, Decode, DecodeWithMemTracking};
pub use scale_info::TypeInfo;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Encode, Decode, DecodeWithMemTracking};
    use scale_info::TypeInfo;
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, LockableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, CheckedSub};
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Chain identifier for PBC/external chains
    #[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[codec(dumb_trait_bound)]
    pub enum ChainId {
        // Layer 2s
        Base = 0,
        Arbitrum = 1,
        Optimism = 2,
        Polygon = 3,
        // Layer 1s
        Ethereum = 10,
        BnbChain = 11,
        Avalanche = 12,
        Solana = 13,
        // Other chains
        Bitcoin = 20,
        Cardano = 21,
        Stellar = 22,
        Ripple = 23,
        Dogecoin = 24,
        Tron = 25,
        Chainlink = 26,
        // Stablecoins
        UsdtBridge = 30,
    }

    impl ChainId {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(ChainId::Base),
                1 => Some(ChainId::Arbitrum),
                2 => Some(ChainId::Optimism),
                3 => Some(ChainId::Polygon),
                10 => Some(ChainId::Ethereum),
                11 => Some(ChainId::BnbChain),
                12 => Some(ChainId::Avalanche),
                13 => Some(ChainId::Solana),
                20 => Some(ChainId::Bitcoin),
                21 => Some(ChainId::Cardano),
                22 => Some(ChainId::Stellar),
                23 => Some(ChainId::Ripple),
                24 => Some(ChainId::Dogecoin),
                25 => Some(ChainId::Tron),
                26 => Some(ChainId::Chainlink),
                30 => Some(ChainId::UsdtBridge),
                _ => None,
            }
        }
    }

    /// Lock event record
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    #[codec(mel_bound())]
    pub struct LockEvent<T: Config> {
        pub chain_id: ChainId,
        pub amount: BalanceOf<T>,
        pub locker: T::AccountId,
        pub block_number: BlockNumberFor<T>,
        pub destination_address: BoundedVec<u8, ConstU32<128>>,
    }

    // Manual implementations of DecodeWithMemTracking
    // Required for polkadot-stable2509 compatibility
    impl DecodeWithMemTracking for ChainId {}
    impl<T: Config> DecodeWithMemTracking for LockEvent<T> where
        T::AccountId: Encode + Decode,
        BalanceOf<T>: Encode + Decode,
        BlockNumberFor<T>: Encode + Decode,
    {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: LockableCurrency<Self::AccountId>;

        /// Origin that can lock/unlock (Foundation multisig)
        type BridgeOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Maximum amount that can be locked per transaction
        #[pallet::constant]
        type MaxLockAmount: Get<BalanceOf<Self>>;

        /// Lock identifier for currency locking
        #[pallet::constant]
        type LockIdentifier: Get<[u8; 8]>;
    }

    /// Total amount locked across all chains
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Amount locked for each specific chain
    #[pallet::storage]
    #[pallet::getter(fn locked_for_chain)]
    pub type LockedForChain<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ChainId,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// History of recent lock events (last 1000)
    #[pallet::storage]
    #[pallet::getter(fn lock_events)]
    pub type LockEvents<T: Config> = StorageValue<
        _,
        BoundedVec<LockEvent<T>, ConstU32<1000>>,
        ValueQuery,
    >;

    /// Account that holds all locked funds (Community LP Pool or Treasury)
    #[pallet::storage]
    #[pallet::getter(fn lock_account)]
    pub type LockAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// ETR locked for bridge [chain_id, amount, destination_address]
        EtrLocked {
            chain_id: ChainId,
            amount: BalanceOf<T>,
            destination_address: BoundedVec<u8, ConstU32<128>>
        },

        /// ETR unlocked from bridge [chain_id, amount]
        EtrUnlocked {
            chain_id: ChainId,
            amount: BalanceOf<T>
        },

        /// Lock account set [account]
        LockAccountSet {
            account: T::AccountId
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Lock account not set
        LockAccountNotSet,

        /// Insufficient balance to lock
        InsufficientBalance,

        /// Amount exceeds maximum
        ExceedsMaxLockAmount,

        /// Insufficient locked amount to unlock
        InsufficientLockedAmount,

        /// Arithmetic overflow
        ArithmeticOverflow,

        /// Arithmetic underflow
        ArithmeticUnderflow,

        /// Invalid chain ID
        InvalidChainId,

        /// Destination address too long
        DestinationAddressTooLong,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the account that holds locked funds
        ///
        /// Typically: Community LP Pool or Foundation Treasury
        ///
        /// Requires: Foundation multisig
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn set_lock_account(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> DispatchResult {
            T::BridgeOrigin::ensure_origin(origin)?;

            LockAccount::<T>::put(account.clone());
            Self::deposit_event(Event::LockAccountSet { account });

            Ok(())
        }

        /// Lock ETR for bridging to external chain
        ///
        /// This locks ÉTR on FlareChain to back wrapped ÉTR on external chains
        ///
        /// Arguments:
        /// - `chain_id`: Which chain (Base, Ethereum, etc.)
        /// - `amount`: How much ÉTR to lock
        /// - `destination_address`: Address on target chain
        ///
        /// Callable by: Bridge pallets or authorized origin
        #[pallet::call_index(1)]
        #[pallet::weight(100_000)]
        pub fn lock_for_bridge(
            origin: OriginFor<T>,
            chain_id: ChainId,
            amount: BalanceOf<T>,
            destination_address: Vec<u8>,
        ) -> DispatchResult {
            // Can be called by bridges or authorized origin
            let who = ensure_signed_or_root(origin)?;

            let lock_account = LockAccount::<T>::get()
                .ok_or(Error::<T>::LockAccountNotSet)?;

            // Check balance
            let balance = T::Currency::free_balance(&lock_account);
            ensure!(balance >= amount, Error::<T>::InsufficientBalance);

            // Check max amount
            ensure!(amount <= T::MaxLockAmount::get(), Error::<T>::ExceedsMaxLockAmount);

            // Convert destination address to BoundedVec
            let bounded_dest: BoundedVec<u8, ConstU32<128>> = destination_address
                .try_into()
                .map_err(|_| Error::<T>::DestinationAddressTooLong)?;

            // Update storage
            let current_total = TotalLocked::<T>::get();
            let new_total = current_total
                .checked_add(&amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;
            TotalLocked::<T>::put(new_total);

            let current_chain = LockedForChain::<T>::get(&chain_id);
            let new_chain_amount = current_chain
                .checked_add(&amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;
            LockedForChain::<T>::insert(&chain_id, new_chain_amount);

            // Record event in history
            let lock_event = LockEvent {
                chain_id,
                amount,
                locker: who.unwrap_or(lock_account.clone()),
                block_number: <frame_system::Pallet<T>>::block_number(),
                destination_address: bounded_dest.clone(),
            };

            LockEvents::<T>::try_mutate(|events| {
                // Keep only last 1000 events
                if events.len() >= 1000 {
                    events.remove(0);
                }
                events.try_push(lock_event)
            }).map_err(|_| Error::<T>::ArithmeticOverflow)?;

            // Lock the currency (prevents spending)
            T::Currency::set_lock(
                T::LockIdentifier::get(),
                &lock_account,
                new_total,
                frame_support::traits::WithdrawReasons::all(),
            );

            Self::deposit_event(Event::EtrLocked {
                chain_id,
                amount,
                destination_address: bounded_dest,
            });

            Ok(())
        }

        /// Unlock ETR from bridge (when wrapped tokens burned on external chain)
        ///
        /// Arguments:
        /// - `chain_id`: Which chain
        /// - `amount`: How much to unlock
        ///
        /// Callable by: Bridge pallets or authorized origin
        #[pallet::call_index(2)]
        #[pallet::weight(100_000)]
        pub fn unlock_from_bridge(
            origin: OriginFor<T>,
            chain_id: ChainId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            // Can be called by bridges or authorized origin
            let _who = ensure_signed_or_root(origin)?;

            let lock_account = LockAccount::<T>::get()
                .ok_or(Error::<T>::LockAccountNotSet)?;

            // Check locked amount
            let current_chain = LockedForChain::<T>::get(&chain_id);
            ensure!(current_chain >= amount, Error::<T>::InsufficientLockedAmount);

            let current_total = TotalLocked::<T>::get();
            ensure!(current_total >= amount, Error::<T>::InsufficientLockedAmount);

            // Update storage
            let new_total = current_total
                .checked_sub(&amount)
                .ok_or(Error::<T>::ArithmeticUnderflow)?;
            TotalLocked::<T>::put(new_total);

            let new_chain_amount = current_chain
                .checked_sub(&amount)
                .ok_or(Error::<T>::ArithmeticUnderflow)?;
            LockedForChain::<T>::insert(&chain_id, new_chain_amount);

            // Update lock
            if new_total.is_zero() {
                T::Currency::remove_lock(T::LockIdentifier::get(), &lock_account);
            } else {
                T::Currency::set_lock(
                    T::LockIdentifier::get(),
                    &lock_account,
                    new_total,
                    frame_support::traits::WithdrawReasons::all(),
                );
            }

            Self::deposit_event(Event::EtrUnlocked { chain_id, amount });

            Ok(())
        }
    }

    // Helper function for bridges to check if they can lock
    impl<T: Config> Pallet<T> {
        /// Check if enough ETR is available to lock
        pub fn can_lock(chain_id: ChainId, amount: BalanceOf<T>) -> bool {
            if let Some(lock_account) = LockAccount::<T>::get() {
                let balance = T::Currency::free_balance(&lock_account);
                balance >= amount && amount <= T::MaxLockAmount::get()
            } else {
                false
            }
        }

        /// Get total locked for a specific chain
        pub fn get_locked_amount(chain_id: ChainId) -> BalanceOf<T> {
            LockedForChain::<T>::get(chain_id)
        }

        /// Get total locked across all chains
        pub fn get_total_locked() -> BalanceOf<T> {
            TotalLocked::<T>::get()
        }
    }
}
