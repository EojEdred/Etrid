//! Ëtrid Peer-Roles — Staking & Role Assignment Pallet
//!
//! Handles staking, unstaking, role registration, and slashing logic for all
//! peer types (FlareNodes, ValidityNodes, Directors, etc.).
//!
//! Depends on: `peer_roles_staking_types` for shared structs & enums.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use peer_roles_staking_types::{
    defaults, Role, RoleEvent, RoleInterface, RoleRecord, StakeRequirement,
};
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency, Get},
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Zero;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Primary balance type bound.
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency used for staking (ËTR).
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Minimum bond period (in blocks) before unstake is allowed.
        #[pallet::constant]
        type UnbondPeriod: Get<u32>;
    }

    #[pallet::storage]
    #[pallet::getter(fn role_of)]
    /// Mapping of account → current role record.
    pub type Roles<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, RoleRecord<T::AccountId, BalanceOf<T>>>;

    #[pallet::storage]
    #[pallet::getter(fn role_index)]
    /// List of all active roles.
    pub type RoleIndex<T: Config> = StorageValue<_, Vec<RoleRecord<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RoleAssigned(T::AccountId, Role),
        RoleRevoked(T::AccountId, Role),
        StakeIncreased(T::AccountId, BalanceOf<T>),
        StakeDecreased(T::AccountId, BalanceOf<T>),
        StakeSlashed(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Attempted to assign an already active role.
        RoleAlreadyAssigned,
        /// Insufficient stake for requested role.
        InsufficientStake,
        /// Account has no active role.
        NoActiveRole,
        /// Unstake period has not elapsed.
        BondNotMature,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Assign a network role to an account with the required stake.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn assign_role(
            origin: OriginFor<T>,
            role: Role,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!Roles::<T>::contains_key(&who), Error::<T>::RoleAlreadyAssigned);

            // Verify stake requirement
            let required = match role {
                Role::DecentralizedDirector => BalanceOf::<T>::from(defaults::DIRECTOR_STAKE),
                Role::ValidityNode => BalanceOf::<T>::from(defaults::VALIDITY_STAKE),
                Role::CommonStakePeer => BalanceOf::<T>::from(defaults::COMMON_STAKE),
                _ => BalanceOf::<T>::zero(),
            };
            ensure!(stake >= required, Error::<T>::InsufficientStake);

            // Reserve stake
            T::Currency::reserve(&who, stake)?;

            let record = RoleRecord {
                account: who.clone(),
                role,
                stake,
                last_update: <frame_system::Pallet<T>>::block_number().saturated_into::<u64>(),
                active: true,
            };
            Roles::<T>::insert(&who, &record);
            RoleIndex::<T>::mutate(|v| v.push(record));

            Self::deposit_event(Event::<T>::RoleAssigned(who, role));
            Ok(())
        }

        /// Increase the reserved stake for an account.
        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn increase_stake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Roles::<T>::try_mutate(&who, |maybe_record| -> DispatchResult {
                let mut record = maybe_record.as_mut().ok_or(Error::<T>::NoActiveRole)?;
                T::Currency::reserve(&who, amount)?;
                record.stake += amount;
                record.last_update =
                    <frame_system::Pallet<T>>::block_number().saturated_into::<u64>();
                Self::deposit_event(Event::<T>::StakeIncreased(who.clone(), amount));
                Ok(())
            })
        }

        /// Unstake part or all of the bonded amount (after unbond period).
        #[pallet::call_index(2)]
        #[pallet::weight(5_000)]
        pub fn unstake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Roles::<T>::try_mutate(&who, |maybe_record| -> DispatchResult {
                let record = maybe_record.as_mut().ok_or(Error::<T>::NoActiveRole)?;
                // simple placeholder time check; ASF can later replace with epoch-based logic
                ensure!(
                    <frame_system::Pallet<T>>::block_number().saturated_into::<u32>()
                        > T::UnbondPeriod::get(),
                    Error::<T>::BondNotMature
                );

                T::Currency::unreserve(&who, amount);
                record.stake -= amount;
                record.last_update =
                    <frame_system::Pallet<T>>::block_number().saturated_into::<u64>();
                Self::deposit_event(Event::<T>::StakeDecreased(who.clone(), amount));
                Ok(())
            })
        }

        /// Revoke a role completely (used by governance or misconduct handling).
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn revoke_role(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            let _who = ensure_root(origin)?;
            if let Some(record) = Roles::<T>::take(&account) {
                T::Currency::unreserve(&account, record.stake);
                RoleIndex::<T>::mutate(|v| v.retain(|r| r.account != account));
                Self::deposit_event(Event::<T>::RoleRevoked(account, record.role));
            }
            Ok(())
        }

        /// Slash a misbehaving validator or director by an amount.
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn slash(origin: OriginFor<T>, account: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let _who = ensure_root(origin)?;
            Roles::<T>::try_mutate(&account, |maybe_record| -> DispatchResult {
                let record = maybe_record.as_mut().ok_or(Error::<T>::NoActiveRole)?;
                let slash_amount = amount.min(record.stake);
                T::Currency::slash_reserved(&account, slash_amount);
                record.stake -= slash_amount;
                record.active = record.stake > BalanceOf::<T>::zero();
                record.last_update =
                    <frame_system::Pallet<T>>::block_number().saturated_into::<u64>();
                Self::deposit_event(Event::<T>::StakeSlashed(account.clone(), slash_amount));
                Ok(())
            })
        }
    }

    // -------- Trait Implementation --------
    impl<T: Config> RoleInterface<T::AccountId, BalanceOf<T>> for Pallet<T> {
        fn get_role(account: &T::AccountId) -> Option<Role> {
            Roles::<T>::get(account).map(|r| r.role)
        }

        fn get_stake(account: &T::AccountId) -> Option<BalanceOf<T>> {
            Roles::<T>::get(account).map(|r| r.stake)
        }

        fn stake_requirement(role: &Role) -> StakeRequirement {
            match role {
                Role::DecentralizedDirector => StakeRequirement::Director,
                Role::ValidityNode => StakeRequirement::Validity,
                Role::CommonStakePeer => StakeRequirement::CommonStake,
                _ => StakeRequirement::None,
            }
        }
    }
}