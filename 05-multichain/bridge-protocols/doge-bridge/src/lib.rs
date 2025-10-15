//! # DOGE Bridge Pallet (Dogecoin)
//!
//! Bridge pallet for Dogecoin blockchain integration with Ëtrid Multichain.
//! Supports native DOGE transfers via UTXO-based transactions.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, ReservableCurrency},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{AccountIdConversion, Saturating},
        ArithmeticError, Perbill,
    };
    use sp_std::prelude::*;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Dogecoin transaction hash
    pub type DogeTxHash = [u8; 32];

    /// Dogecoin address (P2PKH or P2SH) - max 35 bytes
    pub type DogeAddress = BoundedVec<u8, ConstU32<35>>;

    /// Dogecoin UTXO reference
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct DogeUtxo {
        pub amount: u64, // in Koinus (1 DOGE = 100,000,000 Koinus)
        pub tx_hash: DogeTxHash,
        pub vout: u32,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        
        /// Bridge fee percentage (e.g., 1%)
        #[pallet::constant]
        type BridgeFee: Get<Perbill>;
        
        /// Minimum bridge amount (in Koinus)
        #[pallet::constant]
        type MinBridgeAmount: Get<BalanceOf<Self>>;
        
        /// Maximum bridge amount per transaction
        #[pallet::constant]
        type MaxBridgeAmount: Get<BalanceOf<Self>>;
        
        /// Bridge pallet account ID
        #[pallet::constant]
        type PalletId: Get<PalletId>;
        
        /// Number of Dogecoin confirmations required (typically 6)
        #[pallet::constant]
        type DogeConfirmations: Get<u32>;
        
        /// Dogecoin to Ëtrid conversion rate multiplier
        #[pallet::constant]
        type DogeConversionRate: Get<u64>;
    }

    /// DOGE deposits from Dogecoin to Ëtrid
    #[pallet::storage]
    pub type DogeDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        DogeTxHash,
        (T::AccountId, BalanceOf<T>, BlockNumberFor<T>),
        OptionQuery,
    >;

    /// DOGE withdrawals from Ëtrid to Dogecoin
    #[pallet::storage]
    pub type DogeWithdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        (DogeAddress, BalanceOf<T>, BlockNumberFor<T>),
        OptionQuery,
    >;

    /// Registered Dogecoin relay nodes (for bridge monitoring)
    #[pallet::storage]
    pub type DogeRelayNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        DogeAddress,
        OptionQuery,
    >;

    /// Multi-sig Dogecoin address controlled by bridge
    #[pallet::storage]
    pub type DogeBridgeAddress<T: Config> = StorageValue<_, DogeAddress, OptionQuery>;

    /// Bridge status (active/paused)
    #[pallet::storage]
    pub type BridgeActive<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Total DOGE locked in bridge
    #[pallet::storage]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Dogecoin block height last processed
    #[pallet::storage]
    pub type LastProcessedBlock<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Pending UTXOs to be spent for withdrawals
    #[pallet::storage]
    pub type PendingUtxos<T: Config> = StorageValue<_, BoundedVec<DogeUtxo, ConstU32<100>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// DOGE deposited from Dogecoin [who, amount, doge_tx_hash]
        DogeDeposited {
            who: T::AccountId,
            amount: BalanceOf<T>,
            doge_tx_hash: DogeTxHash,
        },
        /// DOGE withdrawn to Dogecoin [who, amount, doge_address]
        DogeWithdrawn {
            who: T::AccountId,
            amount: BalanceOf<T>,
            doge_address: Vec<u8>,
        },
        /// Dogecoin relay node registered [node, doge_address]
        RelayNodeRegistered {
            node: T::AccountId,
            doge_address: Vec<u8>,
        },
        /// Bridge address updated [new_address]
        BridgeAddressUpdated { address: Vec<u8> },
        /// Bridge status changed [is_active]
        BridgeStatusChanged { is_active: bool },
        /// Dogecoin block processed [block_height]
        DogeBlockProcessed { block_height: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Bridge is currently paused
        BridgeInactive,
        /// Amount below minimum threshold
        AmountTooLow,
        /// Amount exceeds maximum threshold
        AmountTooHigh,
        /// Insufficient balance for bridge
        InsufficientBalance,
        /// Invalid Dogecoin address format
        InvalidDogeAddress,
        /// Deposit already processed
        DuplicateDeposit,
        /// Withdrawal not found
        WithdrawalNotFound,
        /// Not enough Dogecoin confirmations
        InsufficientConfirmations,
        /// Not a registered relay node
        NotRelayNode,
        /// Bridge address not set
        BridgeAddressNotSet,
        /// Invalid UTXO reference
        InvalidUtxo,
        /// Arithmetic overflow
        Overflow,
        /// UTXO list full
        UtxoListFull,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deposit DOGE from Dogecoin blockchain
        /// 
        /// Called by bridge relay nodes after detecting deposit on Dogecoin
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn deposit_doge(
            origin: OriginFor<T>,
            beneficiary: T::AccountId,
            amount: BalanceOf<T>,
            doge_tx_hash: DogeTxHash,
            confirmations: u32,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;
            
            // Verify bridge is active
            ensure!(BridgeActive::<T>::get(), Error::<T>::BridgeInactive);
            
            // Verify relayer is registered relay node
            ensure!(
                DogeRelayNodes::<T>::contains_key(&relayer),
                Error::<T>::NotRelayNode
            );
            
            // Verify sufficient confirmations
            ensure!(
                confirmations >= T::DogeConfirmations::get(),
                Error::<T>::InsufficientConfirmations
            );
            
            // Check minimum and maximum amounts
            ensure!(amount >= T::MinBridgeAmount::get(), Error::<T>::AmountTooLow);
            ensure!(amount <= T::MaxBridgeAmount::get(), Error::<T>::AmountTooHigh);
            
            // Prevent duplicate deposits
            ensure!(
                !DogeDeposits::<T>::contains_key(&doge_tx_hash),
                Error::<T>::DuplicateDeposit
            );
            
            // Calculate bridge fee
            let fee = T::BridgeFee::get() * amount;
            let net_amount = amount.saturating_sub(fee);
            
            // Mint wrapped DOGE to beneficiary
            T::Currency::deposit_creating(&beneficiary, net_amount);
            
            // Record deposit
            let current_block = frame_system::Pallet::<T>::block_number();
            DogeDeposits::<T>::insert(
                doge_tx_hash,
                (beneficiary.clone(), amount, current_block),
            );
            
            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_add(amount);
            });
            
            Self::deposit_event(Event::DogeDeposited {
                who: beneficiary,
                amount: net_amount,
                doge_tx_hash,
            });
            
            Ok(())
        }

        /// Withdraw DOGE to Dogecoin blockchain
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn withdraw_doge(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            doge_address: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            // Verify bridge is active
            ensure!(BridgeActive::<T>::get(), Error::<T>::BridgeInactive);
            
            // Verify bridge address is set
            ensure!(
                DogeBridgeAddress::<T>::get().is_some(),
                Error::<T>::BridgeAddressNotSet
            );
            
            // Check minimum and maximum amounts
            ensure!(amount >= T::MinBridgeAmount::get(), Error::<T>::AmountTooLow);
            ensure!(amount <= T::MaxBridgeAmount::get(), Error::<T>::AmountTooHigh);
            
            // Verify user has sufficient balance
            ensure!(
                T::Currency::free_balance(&who) >= amount,
                Error::<T>::InsufficientBalance
            );
            
            // Validate Dogecoin address format
            ensure!(
                Self::validate_doge_address(&doge_address),
                Error::<T>::InvalidDogeAddress
            );

            let doge_address_bounded: DogeAddress = doge_address.clone().try_into()
                .map_err(|_| Error::<T>::InvalidDogeAddress)?;

            // Calculate bridge fee
            let fee = T::BridgeFee::get() * amount;
            let net_amount = amount.saturating_sub(fee);

            // Burn wrapped DOGE from user
            T::Currency::withdraw(
                &who,
                amount,
                frame_support::traits::WithdrawReasons::TRANSFER,
                ExistenceRequirement::KeepAlive,
            )?;

            // Record withdrawal for relay node processing
            let current_block = frame_system::Pallet::<T>::block_number();
            DogeWithdrawals::<T>::insert(
                &who,
                (doge_address_bounded.clone(), net_amount, current_block),
            );

            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_sub(amount);
            });

            Self::deposit_event(Event::DogeWithdrawn {
                who,
                amount: net_amount,
                doge_address: doge_address_bounded.into(),
            });

            Ok(())
        }

        /// Register as Dogecoin bridge relay node
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn register_relay_node(
            origin: OriginFor<T>,
            doge_address: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            // Validate address
            ensure!(
                Self::validate_doge_address(&doge_address),
                Error::<T>::InvalidDogeAddress
            );

            let doge_address_bounded: DogeAddress = doge_address.clone().try_into()
                .map_err(|_| Error::<T>::InvalidDogeAddress)?;

            DogeRelayNodes::<T>::insert(&who, doge_address_bounded.clone());

            Self::deposit_event(Event::RelayNodeRegistered {
                node: who,
                doge_address: doge_address_bounded.into(),
            });

            Ok(())
        }

        /// Set bridge active status (governance only)
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn set_bridge_status(origin: OriginFor<T>, is_active: bool) -> DispatchResult {
            ensure_root(origin)?;
            
            BridgeActive::<T>::put(is_active);
            
            Self::deposit_event(Event::BridgeStatusChanged { is_active });
            
            Ok(())
        }

        /// Update Dogecoin bridge multi-sig address
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn update_bridge_address(
            origin: OriginFor<T>,
            address: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                Self::validate_doge_address(&address),
                Error::<T>::InvalidDogeAddress
            );

            let address_bounded: DogeAddress = address.clone().try_into()
                .map_err(|_| Error::<T>::InvalidDogeAddress)?;

            DogeBridgeAddress::<T>::put(address_bounded.clone());

            Self::deposit_event(Event::BridgeAddressUpdated { address: address_bounded.into() });

            Ok(())
        }

        /// Update last processed Dogecoin block height
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn update_processed_block(
            origin: OriginFor<T>,
            block_height: u64,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;
            
            // Verify relayer is registered
            ensure!(
                DogeRelayNodes::<T>::contains_key(&relayer),
                Error::<T>::NotRelayNode
            );
            
            LastProcessedBlock::<T>::put(block_height);
            
            Self::deposit_event(Event::DogeBlockProcessed { block_height });
            
            Ok(())
        }

        /// Add UTXO to pending list for withdrawal processing
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn add_pending_utxo(
            origin: OriginFor<T>,
            amount: u64,
            tx_hash: DogeTxHash,
            vout: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let utxo = DogeUtxo {
                amount,
                tx_hash,
                vout,
            };

            PendingUtxos::<T>::try_mutate(|utxos| {
                utxos.try_push(utxo).map_err(|_| Error::<T>::UtxoListFull)?;
                Ok::<(), Error<T>>(())
            })?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get bridge pallet account
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Validate Dogecoin address format
        /// Supports P2PKH (starts with 'D') and P2SH (starts with '9' or 'A')
        pub fn validate_doge_address(address: &[u8]) -> bool {
            if address.is_empty() || address.len() < 26 || address.len() > 35 {
                return false;
            }

            // Mainnet: P2PKH starts with 'D', P2SH starts with '9' or 'A'
            // Testnet: P2PKH starts with 'n', P2SH starts with '2'
            let first_char = address[0];
            matches!(first_char, b'D' | b'9' | b'A' | b'n' | b'2')
        }

        /// Convert Koinus (1e-8 DOGE) to Ëtrid balance
        pub fn koinus_to_etrid(koinus: u64) -> BalanceOf<T> {
            use sp_runtime::traits::SaturatedConversion;
            let conversion_rate = T::DogeConversionRate::get();
            (koinus.saturating_mul(conversion_rate) as u128).saturated_into()
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
            DogeBridge: pallet,
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
        pub const BridgeFee: Perbill = Perbill::from_percent(1);
        pub const MinBridgeAmount: u64 = 100_000_000; // 1 DOGE
        pub const MaxBridgeAmount: u64 = 1_000_000_000_000; // 10,000 DOGE
        pub const DogeBridgePalletId: PalletId = PalletId(*b"dge/brdg");
        pub const DogeConfirmations: u32 = 6;
        pub const DogeConversionRate: u64 = 1;
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type BridgeFee = BridgeFee;
        type MinBridgeAmount = MinBridgeAmount;
        type MaxBridgeAmount = MaxBridgeAmount;
        type PalletId = DogeBridgePalletId;
        type DogeConfirmations = DogeConfirmations;
        type DogeConversionRate = DogeConversionRate;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        t.into()
    }

    #[test]
    fn test_validate_doge_address() {
        assert!(pallet::Pallet::<Test>::validate_doge_address(
            &b"DGtE4FzGVWmw7J5oEFLJSLh9jYy".to_vec()
        ));
        assert!(!pallet::Pallet::<Test>::validate_doge_address(&b"invalid".to_vec()));
    }

    #[test]
    fn test_deposit_doge() {
        new_test_ext().execute_with(|| {
            let beneficiary = 1u64;
            let amount = 100_000_000u64; // 1 DOGE
            let tx_hash = [1u8; 32];
            
            // Register relay node
            let relayer = 2u64;
            let doge_addr = b"DGtE4FzGVWmw7J5oEFLJSLh9jYy".to_vec();
            assert_ok!(DogeBridge::register_relay_node(
                RuntimeOrigin::signed(relayer),
                doge_addr
            ));
            
            // Enable bridge
            assert_ok!(DogeBridge::set_bridge_status(RuntimeOrigin::root(), true));
            
            // Deposit with sufficient confirmations
            assert_ok!(DogeBridge::deposit_doge(
                RuntimeOrigin::signed(relayer),
                beneficiary,
                amount,
                tx_hash,
                6
            ));
            
            // Verify balance
            assert!(Balances::free_balance(beneficiary) > 0);
        });
    }
}
