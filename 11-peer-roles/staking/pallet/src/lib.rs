//! # Ëtrid Peer-Roles Staking & Role Assignment Pallet
//!
//! ## Overview
//!
//! This pallet manages staking, role assignment, and slashing for all network peer types
//! in the Ëtrid blockchain, including FlareNodes, ValidityNodes, DecentralizedDirectors,
//! and other peer roles. It provides the economic security layer for the network's
//! consensus and governance systems.
//!
//! ## Features
//!
//! - Role-based staking with minimum stake requirements per role
//! - Flexible stake increase/decrease with unbonding periods
//! - Multi-tier peer roles (FlareNode, ValidityNode, Director, CommonStakePeer, etc.)
//! - Slashing mechanism for misbehavior
//! - Role activation/deactivation based on stake levels
//! - Unbonding queue with configurable lock periods
//! - Governance-controlled role revocation
//!
//! ## Extrinsics
//!
//! - `assign_role` - Assign a network role to an account with required stake
//! - `increase_stake` - Add more stake to an existing role
//! - `unstake` - Initiate unbonding of staked tokens (begins unbonding period)
//! - `revoke_role` - Revoke a role completely (governance only)
//! - `slash` - Slash misbehaving validator or director (governance only)
//! - `withdraw_unbonded` - Withdraw tokens after unbonding period expires
//!
//! ## Usage Example
//!
//! ```ignore
//! // Assign ValidityNode role with 64 ËTR stake
//! Staking::assign_role(
//!     Origin::signed(alice),
//!     1, // ValidityNode role (u8)
//!     64_000_000_000_000_000_000, // 64 ËTR
//! )?;
//!
//! // Increase stake by 10 ËTR
//! Staking::increase_stake(
//!     Origin::signed(alice),
//!     10_000_000_000_000_000_000,
//! )?;
//!
//! // Initiate unbonding of 5 ËTR
//! Staking::unstake(
//!     Origin::signed(alice),
//!     5_000_000_000_000_000_000,
//! )?;
//!
//! // Wait for unbonding period...
//! // Then withdraw unbonded funds
//! Staking::withdraw_unbonded(Origin::signed(alice))?;
//! ```
//!
//! ## Storage Items
//!
//! - `Roles` - Maps account to role record (role type, stake, active status)
//! - `UnbondingQueue` - Maps account to list of unbonding entries (amount, unlock block)
//!
//! ## Events
//!
//! - `RoleAssigned` - When a role is assigned to an account
//! - `RoleRevoked` - When a role is revoked from an account
//! - `StakeIncreased` - When stake is increased
//! - `StakeDecreased` - When stake is decreased (unbonding initiated)
//! - `StakeSlashed` - When stake is slashed for misbehavior
//! - `UnbondingInitiated` - When unbonding process begins
//! - `Withdrawn` - When unbonded funds are withdrawn
//!
//! ## Errors
//!
//! - `RoleAlreadyAssigned` - Account already has an active role
//! - `InsufficientStake` - Stake amount is zero or invalid
//! - `NoActiveRole` - Account has no active role
//! - `BondNotMature` - Unbonding period not yet elapsed
//! - `StakeTooLow` - Stake below minimum for requested role
//! - `InsufficientBalance` - Account lacks free balance to stake
//! - `NoUnbondedFunds` - No funds available to withdraw
//! - `InsufficientBondedStake` - Attempting to unbond more than bonded
//!
//! ## Role Stake Requirements
//!
//! - **DecentralizedDirector**: 128 ËTR minimum
//! - **ValidityNode**: 64 ËTR minimum
//! - **CommonStakePeer**: 1 ËTR minimum
//! - **CommonPeer**: No minimum stake
//! - **CommunityDeveloper**: No minimum stake
//! - **FlareNode**: 64 ËTR minimum (requires special authorization)

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
use sp_runtime::traits::{Zero, UniqueSaturatedInto, Saturating};

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

        /// Maximum number of unbonding entries per account.
        #[pallet::constant]
        type MaxUnbondingEntries: Get<u32>;
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

    #[pallet::storage]
    #[pallet::getter(fn unbonding_queue)]
    /// Mapping of account → (amount, unlock_block) for unbonding stake.
    /// Multiple unbonding entries can exist per account (stored as a bounded list).
    pub type UnbondingQueue<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<(BalanceOf<T>, u32), T::MaxUnbondingEntries>, ValueQuery>;

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
        /// Unbonding initiated [account, amount, unlock_block]
        UnbondingInitiated(T::AccountId, BalanceOf<T>, u32),
        /// Unbonded funds withdrawn [account, amount]
        Withdrawn(T::AccountId, BalanceOf<T>),
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
        /// Stake amount is below minimum requirement for the role.
        StakeTooLow,
        /// Account has insufficient free balance to stake.
        InsufficientBalance,
        /// No unbonded funds available to withdraw.
        NoUnbondedFunds,
        /// Attempting to unstake more than bonded amount.
        InsufficientBondedStake,
        /// Too many unbonding entries for this account.
        TooManyUnbondingEntries,
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

            // Verify stake is not zero
            ensure!(!stake.is_zero(), Error::<T>::InsufficientStake);

            // Validate minimum stake requirement for the role
            let min_stake = Self::get_minimum_stake_for_role(&role);
            ensure!(stake >= min_stake, Error::<T>::StakeTooLow);

            // Check account has sufficient free balance
            let free_balance = T::Currency::free_balance(&who);
            ensure!(free_balance >= stake, Error::<T>::InsufficientBalance);

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

            // Check account has sufficient free balance
            let free_balance = T::Currency::free_balance(&who);
            ensure!(free_balance >= amount, Error::<T>::InsufficientBalance);

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

        /// Initiate unbonding of staked tokens. Tokens become available after UnbondPeriod.
        #[pallet::call_index(2)]
        #[pallet::weight(5_000)]
        pub fn unstake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!amount.is_zero(), Error::<T>::InsufficientStake);

            Roles::<T>::try_mutate(&who, |maybe_record| -> DispatchResult {
                let record = maybe_record.as_mut().ok_or(Error::<T>::NoActiveRole)?;

                // Ensure user has enough bonded stake to unbond
                ensure!(record.stake >= amount, Error::<T>::InsufficientBondedStake);

                // Calculate unlock block number
                let current_block: u32 = <frame_system::Pallet<T>>::block_number().unique_saturated_into();
                let unlock_block = current_block.saturating_add(T::UnbondPeriod::get());

                // Add to unbonding queue (funds remain reserved until withdrawal)
                UnbondingQueue::<T>::try_mutate(&who, |queue| {
                    queue.try_push((amount, unlock_block))
                        .map_err(|_| Error::<T>::TooManyUnbondingEntries)
                })?;

                // Decrease bonded stake immediately but keep reserved
                record.stake = record.stake.saturating_sub(amount);
                record.last_update = current_block as u64;

                // If stake drops to zero, deactivate role
                if record.stake.is_zero() {
                    record.active = false;
                }

                Self::deposit_event(Event::<T>::UnbondingInitiated(who.clone(), amount, unlock_block));
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

        /// Withdraw unbonded tokens that have passed the unbonding period.
        #[pallet::call_index(5)]
        #[pallet::weight(5_000)]
        pub fn withdraw_unbonded(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let current_block: u32 = <frame_system::Pallet<T>>::block_number().unique_saturated_into();
            let mut total_withdrawn = BalanceOf::<T>::zero();

            UnbondingQueue::<T>::mutate(&who, |queue| {
                // Retain only entries that are not yet mature
                queue.retain(|(amount, unlock_block)| {
                    if current_block >= *unlock_block {
                        // Unlock this entry
                        T::Currency::unreserve(&who, *amount);
                        total_withdrawn = total_withdrawn.saturating_add(*amount);
                        false // Remove from queue
                    } else {
                        true // Keep in queue
                    }
                });
            });

            ensure!(!total_withdrawn.is_zero(), Error::<T>::NoUnbondedFunds);

            Self::deposit_event(Event::<T>::Withdrawn(who, total_withdrawn));
            Ok(())
        }
    }

    // -------- Helper Functions --------
    impl<T: Config> Pallet<T> {
        /// Get minimum stake requirement for a given role in BalanceOf<T> units.
        pub fn get_minimum_stake_for_role(role: &Role) -> BalanceOf<T> {
            use sp_runtime::traits::UniqueSaturatedFrom;

            match role {
                Role::DecentralizedDirector => {
                    // 128 ËTR minimum
                    BalanceOf::<T>::unique_saturated_from(defaults::DIRECTOR_STAKE)
                }
                Role::ValidityNode => {
                    // 64 ËTR minimum
                    BalanceOf::<T>::unique_saturated_from(defaults::VALIDITY_STAKE)
                }
                Role::CommonStakePeer => {
                    // 1 ËTR minimum
                    BalanceOf::<T>::unique_saturated_from(defaults::COMMON_STAKE)
                }
                Role::FlareNode => {
                    // FlareNode requires special authorization, but has a stake requirement
                    BalanceOf::<T>::unique_saturated_from(defaults::VALIDITY_STAKE)
                }
                Role::CommonPeer | Role::CommunityDeveloper => {
                    // No minimum stake required
                    BalanceOf::<T>::zero()
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_etrid_staking;
    use frame_support::{
        assert_noop, assert_ok, parameter_types,
        traits::{ConstU32, ConstU64},
    };
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    // Configure a mock runtime to test the pallet.
    frame_support::construct_runtime!(
        pub enum Test
        {
            System: frame_system,
            Balances: pallet_balances,
            Staking: pallet_etrid_staking,
        }
    );

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Nonce = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Block = Block;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = ConstU64<250>;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<u128>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ConstU32<16>;
        type RuntimeTask = ();
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
    }

    impl pallet_balances::Config for Test {
        type MaxLocks = ConstU32<50>;
        type MaxReserves = ();
        type ReserveIdentifier = [u8; 8];
        type Balance = u128;
        type RuntimeEvent = RuntimeEvent;
        type DustRemoval = ();
        type ExistentialDeposit = ConstU128<1>;
        type AccountStore = System;
        type WeightInfo = ();
        type FreezeIdentifier = ();
        type MaxFreezes = ();
        type RuntimeHoldReason = ();
        type RuntimeFreezeReason = ();
    }

    parameter_types! {
        pub const UnbondPeriod: u32 = 100; // 100 blocks for testing
    }

    parameter_types! {
        pub const ConstU128<const N: u128>: u128 = N;
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type UnbondPeriod = UnbondPeriod;
    }

    // Build genesis storage according to the mock runtime.
    pub fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: vec![
                (1, 1000),  // Account 1 with 1000 tokens
                (2, 500),   // Account 2 with 500 tokens
                (3, 50),    // Account 3 with 50 tokens
                (4, 10),    // Account 4 with 10 tokens
            ],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }

    #[test]
    fn test_assign_role_with_minimum_stake() {
        new_test_ext().execute_with(|| {
            // Test Director role (requires 128 ËTR)
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                128
            ));

            assert_eq!(Staking::role_of(1).unwrap().role, Role::DecentralizedDirector);
            assert_eq!(Staking::role_of(1).unwrap().stake, 128);
        });
    }

    #[test]
    fn test_assign_role_fails_with_stake_too_low() {
        new_test_ext().execute_with(|| {
            // Try to assign Director role with insufficient stake
            assert_noop!(
                Staking::assign_role(
                    RuntimeOrigin::signed(1),
                    4, // DecentralizedDirector
                    100 // Less than 128 required
                ),
                Error::<Test>::StakeTooLow
            );
        });
    }

    #[test]
    fn test_assign_role_fails_with_insufficient_balance() {
        new_test_ext().execute_with(|| {
            // Account 4 has only 10 tokens, try to stake 128
            assert_noop!(
                Staking::assign_role(
                    RuntimeOrigin::signed(4),
                    4, // DecentralizedDirector
                    128
                ),
                Error::<Test>::InsufficientBalance
            );
        });
    }

    #[test]
    fn test_increase_stake_with_balance_check() {
        new_test_ext().execute_with(|| {
            // Assign initial role with 128 stake
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                128
            ));

            // Increase stake by 100 (account 1 has 1000 total)
            assert_ok!(Staking::increase_stake(RuntimeOrigin::signed(1), 100));

            assert_eq!(Staking::role_of(1).unwrap().stake, 228);
        });
    }

    #[test]
    fn test_increase_stake_fails_with_insufficient_balance() {
        new_test_ext().execute_with(|| {
            // Assign initial role
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(3),
                1, // ValidityNode (requires 64)
                64
            ));

            // Try to increase stake beyond available balance
            // Account 3 has 50 total, already staked 64 won't work
            // Let's use account 2 with 500 tokens
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(2),
                4, // DecentralizedDirector
                128
            ));

            // Try to increase by 500 (only 372 left free)
            assert_noop!(
                Staking::increase_stake(RuntimeOrigin::signed(2), 500),
                Error::<Test>::InsufficientBalance
            );
        });
    }

    #[test]
    fn test_unbonding_period_enforcement() {
        new_test_ext().execute_with(|| {
            // Assign role
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                200
            ));

            // Initiate unbonding
            assert_ok!(Staking::unstake(RuntimeOrigin::signed(1), 50));

            // Verify unbonding queue entry created
            let queue = Staking::unbonding_queue(1);
            assert_eq!(queue.len(), 1);
            assert_eq!(queue[0].0, 50); // amount
            assert_eq!(queue[0].1, 100); // unlock_block = current(0) + UnbondPeriod(100)

            // Try to withdraw immediately - should fail
            assert_noop!(
                Staking::withdraw_unbonded(RuntimeOrigin::signed(1)),
                Error::<Test>::NoUnbondedFunds
            );

            // Advance blocks to unbonding period
            System::set_block_number(100);

            // Now withdrawal should succeed
            assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(1)));

            // Queue should be empty
            assert_eq!(Staking::unbonding_queue(1).len(), 0);
        });
    }

    #[test]
    fn test_unbond_reduces_stake_immediately() {
        new_test_ext().execute_with(|| {
            // Assign role with 200 stake
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                200
            ));

            // Verify initial stake
            assert_eq!(Staking::role_of(1).unwrap().stake, 200);
            assert_eq!(Staking::role_of(1).unwrap().active, true);

            // Unbond 50
            assert_ok!(Staking::unstake(RuntimeOrigin::signed(1), 50));

            // Stake should be reduced immediately
            assert_eq!(Staking::role_of(1).unwrap().stake, 150);
            assert_eq!(Staking::role_of(1).unwrap().active, true);

            // But funds still reserved
            assert_eq!(Balances::reserved_balance(1), 200);
        });
    }

    #[test]
    fn test_unbond_all_deactivates_role() {
        new_test_ext().execute_with(|| {
            // Assign role
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                200
            ));

            // Unbond all stake
            assert_ok!(Staking::unstake(RuntimeOrigin::signed(1), 200));

            // Role should be deactivated
            assert_eq!(Staking::role_of(1).unwrap().stake, 0);
            assert_eq!(Staking::role_of(1).unwrap().active, false);
        });
    }

    #[test]
    fn test_unbond_fails_with_insufficient_bonded_stake() {
        new_test_ext().execute_with(|| {
            // Assign role
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                128
            ));

            // Try to unbond more than staked
            assert_noop!(
                Staking::unstake(RuntimeOrigin::signed(1), 200),
                Error::<Test>::InsufficientBondedStake
            );
        });
    }

    #[test]
    fn test_multiple_unbonding_entries() {
        new_test_ext().execute_with(|| {
            // Assign role with 300 stake
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                300
            ));

            // First unbond at block 0
            assert_ok!(Staking::unstake(RuntimeOrigin::signed(1), 50));

            // Advance to block 50
            System::set_block_number(50);

            // Second unbond at block 50
            assert_ok!(Staking::unstake(RuntimeOrigin::signed(1), 100));

            // Verify two entries in queue
            let queue = Staking::unbonding_queue(1);
            assert_eq!(queue.len(), 2);
            assert_eq!(queue[0], (50, 100));  // First unbond, unlocks at 100
            assert_eq!(queue[1], (100, 150)); // Second unbond, unlocks at 150

            // At block 100, only first should be withdrawable
            System::set_block_number(100);
            assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(1)));

            // One entry should remain
            let queue = Staking::unbonding_queue(1);
            assert_eq!(queue.len(), 1);
            assert_eq!(queue[0], (100, 150));

            // At block 150, second should be withdrawable
            System::set_block_number(150);
            assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(1)));

            // Queue should be empty
            assert_eq!(Staking::unbonding_queue(1).len(), 0);
        });
    }

    #[test]
    fn test_different_role_stake_requirements() {
        new_test_ext().execute_with(|| {
            // Test ValidityNode (64 minimum)
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                1, // ValidityNode
                64
            ));
            assert_eq!(Staking::role_of(1).unwrap().role, Role::ValidityNode);

            // Test CommonStakePeer (1 minimum)
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(2),
                2, // CommonStakePeer
                1
            ));
            assert_eq!(Staking::role_of(2).unwrap().role, Role::CommonStakePeer);

            // Test CommonPeer (0 minimum)
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(3),
                3, // CommonPeer
                0
            ));
            assert_eq!(Staking::role_of(3).unwrap().role, Role::CommonPeer);
        });
    }

    #[test]
    fn test_reserve_unreserve_balance_tracking() {
        new_test_ext().execute_with(|| {
            let initial_free = Balances::free_balance(1);
            let initial_reserved = Balances::reserved_balance(1);

            // Assign role
            assert_ok!(Staking::assign_role(
                RuntimeOrigin::signed(1),
                4, // DecentralizedDirector
                128
            ));

            // Check balances after staking
            assert_eq!(Balances::free_balance(1), initial_free - 128);
            assert_eq!(Balances::reserved_balance(1), initial_reserved + 128);

            // Unbond half
            assert_ok!(Staking::unstake(RuntimeOrigin::signed(1), 64));

            // Funds still reserved
            assert_eq!(Balances::reserved_balance(1), initial_reserved + 128);

            // Advance past unbonding period and withdraw
            System::set_block_number(100);
            assert_ok!(Staking::withdraw_unbonded(RuntimeOrigin::signed(1)));

            // Check balances after withdrawal
            assert_eq!(Balances::free_balance(1), initial_free - 64);
            assert_eq!(Balances::reserved_balance(1), initial_reserved + 64);
        });
    }
}