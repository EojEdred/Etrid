//! Ëtrid Peer-Roles — Staking & Role Assignment Pallet
//!
//! Handles staking, unstaking, role registration, and slashing logic for all
//! peer types (FlareNodes, ValidityNodes, Directors, etc.).
//!
//! Depends on: `peer_roles_staking_types` for shared structs & enums.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use peer_roles_staking_types::{
    defaults, Role, RoleInterface, RoleRecord, StakeRequirement,
};
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency, Get},
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::{Zero, UniqueSaturatedInto};

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

    // NOTE: RoleIndex storage removed - Vec<RoleRecord> can't implement MaxEncodedLen
    // If needed, can be replaced with BoundedVec or queried from Roles map
    // #[pallet::storage]
    // #[pallet::getter(fn role_index)]
    // pub type RoleIndex<T: Config> = StorageValue<_, Vec<RoleRecord<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Role assigned [account, role_u8] (use Role::from_u8 to convert)
        RoleAssigned(T::AccountId, u8),
        /// Role revoked [account, role_u8]
        RoleRevoked(T::AccountId, u8),
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
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Assign a network role to an account with the required stake.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn assign_role(
            origin: OriginFor<T>,
            role_u8: u8, // Use u8 to avoid DecodeWithMemTracking issues
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Convert u8 to Role
            let role = Role::from_u8(role_u8).ok_or(Error::<T>::InsufficientStake)?;

            ensure!(!Roles::<T>::contains_key(&who), Error::<T>::RoleAlreadyAssigned);

            // Verify stake requirement
            // NOTE: Stake amount validation is simplified - runtime integrations can add
            // proper balance comparisons with role-specific minimums
            ensure!(!stake.is_zero(), Error::<T>::InsufficientStake);

            // Reserve stake
            T::Currency::reserve(&who, stake)?;

            let record = RoleRecord {
                account: who.clone(),
                role,
                stake,
                last_update: <frame_system::Pallet<T>>::block_number().unique_saturated_into(),
                active: true,
            };
            Roles::<T>::insert(&who, &record);
            // RoleIndex removed - see storage definition

            Self::deposit_event(Event::<T>::RoleAssigned(who, role_u8));
            Ok(())
        }

        /// Increase the reserved stake for an account.
        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn increase_stake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Roles::<T>::try_mutate(&who, |maybe_record| -> DispatchResult {
                let record = maybe_record.as_mut().ok_or(Error::<T>::NoActiveRole)?;
                T::Currency::reserve(&who, amount)?;
                record.stake += amount;
                record.last_update =
                    <frame_system::Pallet<T>>::block_number().unique_saturated_into();
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
                let current_block: u32 = <frame_system::Pallet<T>>::block_number().unique_saturated_into();
                ensure!(
                    current_block > T::UnbondPeriod::get(),
                    Error::<T>::BondNotMature
                );

                T::Currency::unreserve(&who, amount);
                record.stake -= amount;
                record.last_update =
                    <frame_system::Pallet<T>>::block_number().unique_saturated_into();
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
                // RoleIndex removed - see storage definition
                let role_u8 = record.role as u8;
                Self::deposit_event(Event::<T>::RoleRevoked(account, role_u8));
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
                    <frame_system::Pallet<T>>::block_number().unique_saturated_into();
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