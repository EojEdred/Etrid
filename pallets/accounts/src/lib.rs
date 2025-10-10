#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::RuntimeDebug;

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum TokenType {
        ETR,
        ETD,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
    pub struct AccountData<Balance> {
        pub etr_balance: Balance,
        pub etd_balance: Balance,
        pub nonce: u32,
        pub is_validator: bool,
        pub reputation: u64,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Balance: Parameter + From<u64> + Into<u64> + AtLeast32BitUnsigned + Default + Copy;
        type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub(super) type Accounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AccountData<T::Balance>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Transferred(T::AccountId, T::AccountId, TokenType, T::Balance),
        Minted(T::AccountId, TokenType, T::Balance),
        Burned(T::AccountId, TokenType, T::Balance),
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        InvalidTokenType,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            token_type: TokenType,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::do_transfer(&sender, &to, token_type, amount)?;
            Ok(())
        }

        pub fn mint_etr(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            T::GovernanceOrigin::ensure_origin(origin)?;
            Accounts::<T>::mutate(&to, |acct| {
                acct.etr_balance += amount;
            });
            Self::deposit_event(Event::Minted(to, TokenType::ETR, amount));
            Ok(())
        }

        pub fn mint_etd(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            Accounts::<T>::mutate(&to, |acct| {
                acct.etd_balance += amount;
            });
            Self::deposit_event(Event::Minted(to, TokenType::ETD, amount));
            Ok(())
        }

        pub fn burn(
            origin: OriginFor<T>,
            token_type: TokenType,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Accounts::<T>::mutate(&sender, |acct| match token_type {
                TokenType::ETR => {
                    ensure!(acct.etr_balance >= amount, Error::<T>::InsufficientBalance);
                    acct.etr_balance -= amount;
                },
                TokenType::ETD => {
                    ensure!(acct.etd_balance >= amount, Error::<T>::InsufficientBalance);
                    acct.etd_balance -= amount;
                },
            });
            Self::deposit_event(Event::Burned(sender, token_type, amount));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn do_transfer(
            from: &T::AccountId,
            to: &T::AccountId,
            token_type: TokenType,
            amount: T::Balance,
        ) -> Result<(), DispatchError> {
            Accounts::<T>::try_mutate(from, |from_acct| {
                Accounts::<T>::mutate(to, |to_acct| {
                    match token_type {
                        TokenType::ETR => {
                            ensure!(from_acct.etr_balance >= amount, Error::<T>::InsufficientBalance);
                            from_acct.etr_balance -= amount;
                            to_acct.etr_balance += amount;
                        },
                        TokenType::ETD => {
                            ensure!(from_acct.etd_balance >= amount, Error::<T>::InsufficientBalance);
                            from_acct.etd_balance -= amount;
                            to_acct.etd_balance += amount;
                        },
                    }
                    from_acct.nonce += 1;
                    Ok(())
                })
            })?;
            Self::deposit_event(Event::Transferred(from.clone(), to.clone(), token_type, amount));
            Ok(())
        }
    }
}
