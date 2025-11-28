#![cfg_attr(not(feature = "std"), no_std)]

//! # Ëtrid Coin
//!
//! Native currency implementation for the Ëtrid multichain system
//!
//! ## Fee Collection & Treasury Routing
//!
//! VMw gas fees are collected from users and routed to the treasury:
//! - VMw consumed is converted to ÉTR using: `fee = (vmw * op_price) / 1_000_000`
//! - 50% of fees go to Treasury for Consensus Day distribution
//! - 50% of fees are burned (deflationary mechanism)

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use frame_support::dispatch::DispatchResult;

pub use pallet::*;

/// Treasury interface for routing VMw fees to the treasury pallet
///
/// This trait connects the native currency pallet to pallet-treasury.
/// Fees collected from VMw consumption are routed to the treasury pool
/// for distribution during Consensus Day.
pub trait TreasuryInterface<Balance> {
    /// Receive transaction fees from VMw gas consumption
    fn receive_transaction_fees(amount: Balance) -> DispatchResult;
}

/// No-op treasury implementation for testing/development
impl<Balance> TreasuryInterface<Balance> for () {
    fn receive_transaction_fees(_amount: Balance) -> DispatchResult {
        Ok(())
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuration trait for pallet-native-currency
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Treasury interface for routing fees
        type Treasury: TreasuryInterface<Balance>;

        /// Treasury fee percentage (default 50%)
        #[pallet::constant]
        type TreasuryFeePercent: Get<u32>;
    }

    // ============================================================
    // CURRENCY DENOMINATIONS (from Ivory Paper)
    // ============================================================
    // Total Supply: 1,000,000,000 ÉTR = 1 billion
    // Smallest Unit: Bitë (0.00001 ÉTR)
    //
    // Unit Hierarchy:
    // - Bitë (bitë)           = 0.00001 ÉTR   (1e-5)   [smallest]
    // - Tribite (tbitë)       = 0.0001 ÉTR    (1e-4)
    // - Quadrite (qbitë)      = 0.001 ÉTR     (1e-3)
    // - Octobite (obitë)      = 0.01 ÉTR      (1e-2)
    // - Sextobite (sbitë)     = 0.1 ÉTR       (1e-1)
    // - ÉTR (Ëtr)             = 1 ÉTR         (1e0)   [base]
    // - KiloÉtrid (kËtr)      = 1,000 ÉTR     (1e3)
    // - MegaÉtrid (mËtr)      = 1,000,000 ÉTR (1e6)
    // - GigaÉtrid (gÉTR)      = 1,000,000,000 ÉTR (1e9) [total supply]
    // ============================================================

    /// ÉTR Token Balance Type (u128 in atomic units, 1 ÉTR = 1e18 atomic units)
    pub type Balance = u128;

    /// Conversion constants for currency denominations
    pub const ONE_BITE: Balance = 1;              // 1 Bitë = 1 atomic unit
    pub const ONE_TRIBITE: Balance = 10;          // 1 Tribite = 10 Bitë
    pub const ONE_QUADRITE: Balance = 100;        // 1 Quadrite = 100 Bitë
    pub const ONE_OCTOBITE: Balance = 1_000;      // 1 Octobite = 1,000 Bitë
    pub const ONE_SEXTOBITE: Balance = 10_000;    // 1 Sextobite = 10,000 Bitë
    pub const ONE_ETRID: Balance = 100_000;       // 1 ÉTR = 100,000 Bitë
    pub const ONE_KILO_ETRID: Balance = 100_000_000;           // 1 kËtr = 100M Bitë
    pub const ONE_MEGA_ETRID: Balance = 100_000_000_000;       // 1 mËtr = 100B Bitë
    pub const ONE_GIGA_ETRID: Balance = 100_000_000_000_000;   // 1 gÉTR = 100T Bitë

    /// Total ÉTR supply: 1 billion ÉTR (in smallest units)
    pub const TOTAL_ETRID_SUPPLY: Balance = 1_000_000_000 * ONE_ETRID; // 1B * 100k = 1e14 atomic

    /// ETD Stablecoin: 1:1 USD peg (max 2.5B)
    pub const TOTAL_ETD_SUPPLY: Balance = 2_500_000_000 * ONE_ETRID; // 2.5B * 100k

    // ============================================================
    // VIRTUAL MACHINE WATTS (VMw) PRICING SCHEDULE (from Ivory Paper)
    // ============================================================
    // VMw is a non-tradable unit representing computational effort
    // Formula: Total Cost (ÉTR) = (VMw_Used * Op_Price) / WATTS_PER_ETR
    //
    // Operation Costs (in VMw):
    // - contract_init:    2,000 VMw (deploy new smart contract)
    // - contract_call:      500 VMw (execute contract function)
    // - storage_read:       100 VMw (read from persistent storage)
    // - storage_write:      300 VMw (write to persistent storage)
    // - state_verify:       150 VMw (verify state transitions)
    // - address_check:       50 VMw (validate address format)
    //
    // Block Limits:
    // - VM Wattage Limit per block: 10,000,000 VMw (10M)
    // - Max VMw per transaction: 1,000,000 VMw (1M)
    // ============================================================

    pub type VMw = u128; // Virtual Machine Watts

    /// VMw operation costs
    pub const VMW_CONTRACT_INIT: VMw = 2_000;
    pub const VMW_CONTRACT_CALL: VMw = 500;
    pub const VMW_STORAGE_READ: VMw = 100;
    pub const VMW_STORAGE_WRITE: VMw = 300;
    pub const VMW_STATE_VERIFY: VMw = 150;
    pub const VMW_ADDRESS_CHECK: VMw = 50;

    /// VMw limits (per block and per transaction)
    pub const VMW_BLOCK_LIMIT: VMw = 10_000_000; // 10M VMw per block
    pub const VMW_TX_LIMIT: VMw = 1_000_000; // 1M VMw per transaction

    /// VMw to ÉTR conversion: 1000 VMw = 0.001 ÉTR
    /// More precisely: 1 VMw = (1 / 1_000_000) ÉTR in atomic units
    pub const WATTS_PER_ETRID: VMw = 1_000_000; // 1M VMw per ÉTR

    // ============================================================
    // STORAGE MAPS
    // ============================================================

    #[pallet::storage]
    #[pallet::getter(fn etrid_balance)]
    pub(super) type EtridBalances<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn etd_balance)]
    pub(super) type EtdBalances<T: Config> =
        StorageMap<_, frame_support::Blake2_128Concat, T::AccountId, Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn vmw_used_this_block)]
    pub(super) type VMwUsedThisBlock<T: Config> = StorageValue<_, VMw, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn etrid_total_supply)]
    pub(super) type EtridTotalSupply<T: Config> = StorageValue<_, Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn etd_total_supply)]
    pub(super) type EtdTotalSupply<T: Config> = StorageValue<_, Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn vmw_op_price)]
    pub(super) type VMwOpPrice<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Total fees collected this block
    #[pallet::storage]
    #[pallet::getter(fn fees_collected_block)]
    pub(super) type FeesCollectedBlock<T: Config> = StorageValue<_, Balance, ValueQuery>;

    /// Total fees sent to treasury this block
    #[pallet::storage]
    #[pallet::getter(fn treasury_fees_block)]
    pub(super) type TreasuryFeesBlock<T: Config> = StorageValue<_, Balance, ValueQuery>;

    /// Cumulative fees collected (all time)
    #[pallet::storage]
    #[pallet::getter(fn total_fees_collected)]
    pub(super) type TotalFeesCollected<T: Config> = StorageValue<_, Balance, ValueQuery>;

    /// Cumulative fees sent to treasury (all time)
    #[pallet::storage]
    #[pallet::getter(fn total_treasury_fees)]
    pub(super) type TotalTreasuryFees<T: Config> = StorageValue<_, Balance, ValueQuery>;

    // ============================================================
    // EVENTS
    // ============================================================

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// ÉTR transfer: from, to, amount
        EtridTransferred { from: T::AccountId, to: T::AccountId, amount: Balance },
        /// ETD transfer: from, to, amount
        EtdTransferred { from: T::AccountId, to: T::AccountId, amount: Balance },
        /// ÉTR minted: account, amount
        EtridMinted { account: T::AccountId, amount: Balance },
        /// ETD minted: account, amount
        EtdMinted { account: T::AccountId, amount: Balance },
        /// ÉTR burned: account, amount
        EtridBurned { account: T::AccountId, amount: Balance },
        /// ETD burned: account, amount
        EtdBurned { account: T::AccountId, amount: Balance },
        /// VMw consumed: tx_id, vmw_used, cost_in_etrid
        VMwConsumed { vmw_used: VMw, cost_in_etrid: Balance },
        /// VMw block limit exceeded
        VMwBlockLimitExceeded,
        /// VMw transaction limit exceeded
        VMwTxLimitExceeded,
        /// Storage rent charged: account, amount
        StorageRentCharged { account: T::AccountId, amount: Balance },
        /// VMw fee collected and routed to treasury
        FeeCollected {
            /// Account that paid the fee
            payer: T::AccountId,
            /// VMw gas consumed
            vmw_used: VMw,
            /// Total fee in ÉTR
            total_fee: Balance,
            /// Amount sent to treasury (50%)
            treasury_amount: Balance,
            /// Amount burned (50%)
            burned_amount: Balance,
        },
        /// Block fee summary
        BlockFeeSummary {
            /// Block number
            block_number: BlockNumberFor<T>,
            /// Total VMw consumed in block
            total_vmw: VMw,
            /// Total fees collected in block
            total_fees: Balance,
            /// Fees sent to treasury
            treasury_fees: Balance,
        },
    }

    // ============================================================
    // ERRORS
    // ============================================================

    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient ÉTR balance
        InsufficientEtridBalance,
        /// Insufficient ETD balance
        InsufficientEtdBalance,
        /// VMw limit exceeded for this block
        VMwBlockLimitExceeded,
        /// VMw limit exceeded for this transaction
        VMwTxLimitExceeded,
        /// Total ÉTR supply would exceed cap
        EtridSupplyCapped,
        /// Total ETD supply would exceed cap
        EtdSupplyCapped,
        /// Invalid transfer amount (zero)
        InvalidAmount,
        /// Account does not exist
        AccountNotFound,
        /// VMw operation price not set
        VMwPriceNotSet,
        /// Invalid VMw operation cost
        InvalidVMwCost,
        /// Treasury fee routing failed
        TreasuryRoutingFailed,
    }

    // ============================================================
    // PALLET IMPLEMENTATION
    // ============================================================

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transfer ÉTR from sender to recipient
        #[pallet::call_index(0)]
        #[pallet::weight(VMW_CONTRACT_CALL as u64)]
        pub fn transfer_etrid(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate amount
            ensure!(amount > 0, Error::<T>::InvalidAmount);

            // Check sender balance
            let sender_balance = EtridBalances::<T>::get(&sender);
            ensure!(sender_balance >= amount, Error::<T>::InsufficientEtridBalance);

            // Perform transfer
            EtridBalances::<T>::insert(&sender, sender_balance - amount);
            let recipient_balance = EtridBalances::<T>::get(&recipient);
            EtridBalances::<T>::insert(&recipient, recipient_balance + amount);

            Self::deposit_event(Event::EtridTransferred {
                from: sender,
                to: recipient,
                amount,
            });

            Ok(())
        }

        /// Transfer ETD from sender to recipient
        #[pallet::call_index(1)]
        #[pallet::weight(VMW_CONTRACT_CALL as u64)]
        pub fn transfer_etd(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate amount
            ensure!(amount > 0, Error::<T>::InvalidAmount);

            // Check sender balance
            let sender_balance = EtdBalances::<T>::get(&sender);
            ensure!(sender_balance >= amount, Error::<T>::InsufficientEtdBalance);

            // Perform transfer
            EtdBalances::<T>::insert(&sender, sender_balance - amount);
            let recipient_balance = EtdBalances::<T>::get(&recipient);
            EtdBalances::<T>::insert(&recipient, recipient_balance + amount);

            Self::deposit_event(Event::EtdTransferred {
                from: sender,
                to: recipient,
                amount,
            });

            Ok(())
        }

        /// Mint ÉTR (governance-controlled, Consensus Day only)
        #[pallet::call_index(2)]
        #[pallet::weight(VMW_CONTRACT_INIT as u64)]
        pub fn mint_etrid(
            _origin: OriginFor<T>,
            account: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            // Check supply cap
            let current_supply = EtridTotalSupply::<T>::get();
            ensure!(
                current_supply + amount <= TOTAL_ETRID_SUPPLY,
                Error::<T>::EtridSupplyCapped
            );

            // Mint
            let balance = EtridBalances::<T>::get(&account);
            EtridBalances::<T>::insert(&account, balance + amount);
            EtridTotalSupply::<T>::set(current_supply + amount);

            Self::deposit_event(Event::EtridMinted { account, amount });

            Ok(())
        }

        /// Mint ETD (governance-controlled, 1:1 USD backing)
        #[pallet::call_index(3)]
        #[pallet::weight(VMW_CONTRACT_INIT as u64)]
        pub fn mint_etd(
            _origin: OriginFor<T>,
            account: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            // Check supply cap
            let current_supply = EtdTotalSupply::<T>::get();
            ensure!(
                current_supply + amount <= TOTAL_ETD_SUPPLY,
                Error::<T>::EtdSupplyCapped
            );

            // Mint
            let balance = EtdBalances::<T>::get(&account);
            EtdBalances::<T>::insert(&account, balance + amount);
            EtdTotalSupply::<T>::set(current_supply + amount);

            Self::deposit_event(Event::EtdMinted { account, amount });

            Ok(())
        }

        /// Burn ÉTR (remove from circulation)
        #[pallet::call_index(4)]
        #[pallet::weight(VMW_CONTRACT_CALL as u64)]
        pub fn burn_etrid(
            origin: OriginFor<T>,
            amount: Balance,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;

            ensure!(amount > 0, Error::<T>::InvalidAmount);

            let balance = EtridBalances::<T>::get(&account);
            ensure!(balance >= amount, Error::<T>::InsufficientEtridBalance);

            EtridBalances::<T>::insert(&account, balance - amount);
            let supply = EtridTotalSupply::<T>::get();
            EtridTotalSupply::<T>::set(supply - amount);

            Self::deposit_event(Event::EtridBurned { account, amount });

            Ok(())
        }

        /// Consume VMw and charge fee - collects fees and routes to treasury
        ///
        /// This is the core fee collection mechanism:
        /// 1. Converts VMw to ÉTR using: fee = (vmw * op_price) / 1,000,000
        /// 2. Deducts fee from caller's ÉTR balance
        /// 3. Routes 50% to Treasury for Consensus Day distribution
        /// 4. Burns 50% (deflationary mechanism)
        #[pallet::call_index(5)]
        #[pallet::weight(VMW_CONTRACT_CALL as u64)]
        pub fn consume_vmw(
            origin: OriginFor<T>,
            vmw_amount: VMw,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;

            // Check VMw limits
            ensure!(vmw_amount <= VMW_TX_LIMIT, Error::<T>::VMwTxLimitExceeded);

            let current_block_vmw = VMwUsedThisBlock::<T>::get();
            ensure!(
                current_block_vmw + vmw_amount <= VMW_BLOCK_LIMIT,
                Error::<T>::VMwBlockLimitExceeded
            );

            // Get op price
            let op_price = VMwOpPrice::<T>::get();
            ensure!(op_price > 0, Error::<T>::VMwPriceNotSet);

            // Calculate cost in ÉTR: (VMw * op_price) / WATTS_PER_ETRID
            let total_fee: Balance = ((vmw_amount as u128) * (op_price as u128)) / WATTS_PER_ETRID;

            // Only collect if fee is non-zero
            if total_fee > 0 {
                // Check caller has sufficient balance
                let caller_balance = EtridBalances::<T>::get(&account);
                ensure!(caller_balance >= total_fee, Error::<T>::InsufficientEtridBalance);

                // Calculate treasury share (default 50%)
                let treasury_percent = T::TreasuryFeePercent::get();
                let treasury_amount = (total_fee * (treasury_percent as u128)) / 100;
                let burn_amount = total_fee - treasury_amount;

                // Deduct total fee from caller's balance
                EtridBalances::<T>::insert(&account, caller_balance - total_fee);

                // Route treasury portion to treasury pallet
                if treasury_amount > 0 {
                    T::Treasury::receive_transaction_fees(treasury_amount)
                        .map_err(|_| Error::<T>::TreasuryRoutingFailed)?;
                }

                // Burn portion is automatically removed (not added anywhere)
                // Reduce total supply by burn amount
                let supply = EtridTotalSupply::<T>::get();
                EtridTotalSupply::<T>::set(supply.saturating_sub(burn_amount));

                // Update block fee tracking
                let current_fees = FeesCollectedBlock::<T>::get();
                FeesCollectedBlock::<T>::set(current_fees + total_fee);
                let current_treasury = TreasuryFeesBlock::<T>::get();
                TreasuryFeesBlock::<T>::set(current_treasury + treasury_amount);

                // Update cumulative fee tracking
                let total_collected = TotalFeesCollected::<T>::get();
                TotalFeesCollected::<T>::set(total_collected + total_fee);
                let total_treasury = TotalTreasuryFees::<T>::get();
                TotalTreasuryFees::<T>::set(total_treasury + treasury_amount);

                // Emit fee collection event
                Self::deposit_event(Event::FeeCollected {
                    payer: account.clone(),
                    vmw_used: vmw_amount,
                    total_fee,
                    treasury_amount,
                    burned_amount: burn_amount,
                });
            }

            // Update block VMw usage
            VMwUsedThisBlock::<T>::set(current_block_vmw + vmw_amount);

            Self::deposit_event(Event::VMwConsumed {
                vmw_used: vmw_amount,
                cost_in_etrid: total_fee,
            });

            Ok(())
        }

        /// Set VMw operation price (governance-controlled)
        #[pallet::call_index(6)]
        #[pallet::weight(1_000u64)]
        pub fn set_vmw_price(
            _origin: OriginFor<T>,
            price: u32,
        ) -> DispatchResult {
            ensure!(price > 0, Error::<T>::InvalidVMwCost);
            VMwOpPrice::<T>::set(price);
            Ok(())
        }
    }

    // ============================================================
    // HELPER FUNCTIONS
    // ============================================================

    impl<T: Config> Pallet<T> {
        /// Get ÉTR balance for an account
        pub fn get_etrid_balance(account: &T::AccountId) -> Balance {
            EtridBalances::<T>::get(account)
        }

        /// Get ETD balance for an account
        pub fn get_etd_balance(account: &T::AccountId) -> Balance {
            EtdBalances::<T>::get(account)
        }

        /// Calculate fee for VMw consumption
        pub fn calculate_vmw_fee(vmw_used: VMw) -> Balance {
            let op_price = VMwOpPrice::<T>::get();
            if op_price == 0 {
                0
            } else {
                ((vmw_used as u128) * (op_price as u128)) / WATTS_PER_ETRID
            }
        }

        /// Convert amount between currency units
        pub fn convert_currency(amount: Balance, from_unit: CurrencyUnit, to_unit: CurrencyUnit) -> Balance {
            let from_multiplier = match from_unit {
                CurrencyUnit::Bite => ONE_BITE,
                CurrencyUnit::Tribite => ONE_TRIBITE,
                CurrencyUnit::Quadrite => ONE_QUADRITE,
                CurrencyUnit::Octobite => ONE_OCTOBITE,
                CurrencyUnit::Sextobite => ONE_SEXTOBITE,
                CurrencyUnit::Etrid => ONE_ETRID,
                CurrencyUnit::KiloEtrid => ONE_KILO_ETRID,
                CurrencyUnit::MegaEtrid => ONE_MEGA_ETRID,
                CurrencyUnit::GigaEtrid => ONE_GIGA_ETRID,
            };

            let to_multiplier = match to_unit {
                CurrencyUnit::Bite => ONE_BITE,
                CurrencyUnit::Tribite => ONE_TRIBITE,
                CurrencyUnit::Quadrite => ONE_QUADRITE,
                CurrencyUnit::Octobite => ONE_OCTOBITE,
                CurrencyUnit::Sextobite => ONE_SEXTOBITE,
                CurrencyUnit::Etrid => ONE_ETRID,
                CurrencyUnit::KiloEtrid => ONE_KILO_ETRID,
                CurrencyUnit::MegaEtrid => ONE_MEGA_ETRID,
                CurrencyUnit::GigaEtrid => ONE_GIGA_ETRID,
            };

            (amount * from_multiplier) / to_multiplier
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Emit block fee summary and reset counters
        fn on_finalize(n: BlockNumberFor<T>) {
            let total_vmw = VMwUsedThisBlock::<T>::get();
            let total_fees = FeesCollectedBlock::<T>::get();
            let treasury_fees = TreasuryFeesBlock::<T>::get();

            // Emit block summary if fees were collected
            if total_fees > 0 {
                Self::deposit_event(Event::BlockFeeSummary {
                    block_number: n,
                    total_vmw,
                    total_fees,
                    treasury_fees,
                });
            }

            // Reset block counters
            VMwUsedThisBlock::<T>::set(0);
            FeesCollectedBlock::<T>::set(0);
            TreasuryFeesBlock::<T>::set(0);
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub balances: Vec<(T::AccountId, Balance)>,
        pub vmw_op_price: u32,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                balances: vec![],
                vmw_op_price: 1,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            let mut total: Balance = 0;
            for (account, balance) in &self.balances {
                EtridBalances::<T>::insert(account, balance);
                total += balance;
            }
            EtridTotalSupply::<T>::set(total);
            VMwOpPrice::<T>::set(self.vmw_op_price);
        }
    }
}

// ============================================================
// CURRENCY UNIT ENUM (for conversions)
// ============================================================

#[derive(Encode, Decode, TypeInfo, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyUnit {
    Bite,
    Tribite,
    Quadrite,
    Octobite,
    Sextobite,
    Etrid,
    KiloEtrid,
    MegaEtrid,
    GigaEtrid,
}
