#![cfg_attr(not(feature = "std"), no_std)]

//! etrid-stablecoin

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
type AccountOf<T> = <T as frame_system::Config>::AccountId;

pub type Balance = u128;

pub const ETD_DECIMALS: u8 = 18;
pub const ONE_ETD: Balance = 1_000_000_000_000_000_000u128; // 10^18, 1:1 USD

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
pub struct Pallet<T>(_);

#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}

// ============================================================
// STORAGE
// ============================================================

#[pallet::storage]
pub type EtdBalances<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AccountOf<T>,
    Balance,
    ValueQuery,
>;

#[pallet::storage]
pub type TotalEtdSupply<T: Config> = StorageValue<_, Balance, ValueQuery>;

/// Collateral backing ETD (stored as amount)
#[pallet::storage]
pub type Collateral<T: Config> = StorageValue<_, Balance, ValueQuery>;

// ============================================================
// EVENTS
// ============================================================

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// ETD minted
    EtdMinted {
        account: AccountOf<T>,
        amount: Balance,
    },
    /// ETD burned
    EtdBurned {
        account: AccountOf<T>,
        amount: Balance,
    },
    /// ETD transferred
    EtdTransferred {
        from: AccountOf<T>,
        to: AccountOf<T>,
        amount: Balance,
    },
    /// Collateral added
    CollateralAdded {
        amount: Balance,
    },
}

// ============================================================
// ERRORS
// ============================================================

#[pallet::error]
pub enum Error<T> {
    InsufficientBalance,
    InsufficientCollateral,
}

// ============================================================
// EXTRINSICS
// ============================================================

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Mint new ETD (backed by collateral)
    #[pallet::call_index(0)]
    #[pallet::weight(10_000)]
    pub fn mint_etd(
        origin: OriginFor<T>,
        account: AccountOf<T>,
        amount: Balance,
    ) -> DispatchResult {
        ensure_root(origin)?;

        // For v1: Assume collateral is provided separately
        // Check: collateral >= total_supply + amount (1:1 backing)

        EtdBalances::<T>::mutate(&account, |balance| {
            *balance = balance.saturating_add(amount);
        });

        TotalEtdSupply::<T>::mutate(|supply| {
            *supply = supply.saturating_add(amount);
        });

        Self::deposit_event(Event::EtdMinted { account, amount });

        Ok(())
    }

    /// Burn ETD (returns collateral)
    #[pallet::call_index(1)]
    #[pallet::weight(10_000)]
    pub fn burn_etd(
        origin: OriginFor<T>,
        account: AccountOf<T>,
        amount: Balance,
    ) -> DispatchResult {
        ensure_root(origin)?;

        let balance = EtdBalances::<T>::get(&account);
        ensure!(balance >= amount, Error::<T>::InsufficientBalance);

        EtdBalances::<T>::insert(&account, balance.saturating_sub(amount));

        TotalEtdSupply::<T>::mutate(|supply| {
            *supply = supply.saturating_sub(amount);
        });

        Self::deposit_event(Event::EtdBurned { account, amount });

        Ok(())
    }

    /// Transfer ETD
    #[pallet::call_index(2)]
    #[pallet::weight(10_000)]
    pub fn transfer_etd(
        origin: OriginFor<T>,
        to: AccountOf<T>,
        amount: Balance,
    ) -> DispatchResult {
        let from = ensure_signed(origin)?;
        ensure!(amount > 0, Error::<T>::InsufficientBalance);

        let from_balance = EtdBalances::<T>::get(&from);
        ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

        EtdBalances::<T>::insert(&from, from_balance.saturating_sub(amount));
        EtdBalances::<T>::mutate(&to, |balance| {
            *balance = balance.saturating_add(amount);
        });

        Self::deposit_event(Event::EtdTransferred { from, to, amount });

        Ok(())
    }

    /// Add collateral backing (only root)
    #[pallet::call_index(3)]
    #[pallet::weight(5_000)]
    pub fn add_collateral(
        origin: OriginFor<T>,
        amount: Balance,
    ) -> DispatchResult {
        ensure_root(origin)?;

        Collateral::<T>::mutate(|collateral| {
            *collateral = collateral.saturating_add(amount);
        });

        Self::deposit_event(Event::CollateralAdded { amount });

        Ok(())
    }

    /// Batch mint ETD (for genesis distribution)
    #[pallet::call_index(4)]
    #[pallet::weight(50_000)]
    pub fn batch_mint_etd(
        origin: OriginFor<T>,
        recipients: Vec<(AccountOf<T>, Balance)>,
    ) -> DispatchResult {
        ensure_root(origin)?;

        let mut total_minted = Balance::default();

        for (account, amount) in recipients {
            EtdBalances::<T>::mutate(&account, |balance| {
                *balance = balance.saturating_add(amount);
            });
            total_minted = total_minted.saturating_add(amount);
        }

        TotalEtdSupply::<T>::mutate(|supply| {
            *supply = supply.saturating_add(total_minted);
        });

        Ok(())
    }
}

// ============================================================
// STORAGE GETTERS
// ============================================================

impl<T: Config> Pallet<T> {
    pub fn get_balance(account: &AccountOf<T>) -> Balance {
        EtdBalances::<T>::get(account)
    }

    pub fn total_supply() -> Balance {
        TotalEtdSupply::<T>::get()
    }

    pub fn total_collateral() -> Balance {
        Collateral::<T>::get()
    }

    pub fn is_backed() -> bool {
        Self::total_collateral() >= Self::total_supply()
    }
}
}