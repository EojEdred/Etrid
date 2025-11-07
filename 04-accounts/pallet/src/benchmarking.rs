//! Benchmarking setup for pallet-accounts
//!
//! This module contains benchmarks for all extrinsics in pallet-accounts.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::pallet::{AccountData, RecoveryConfig, ActiveRecovery, TokenType};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::BoundedVec;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn transfer() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: T::Balance = 1000u64.into();

        // Setup: Give caller some ETR balance
        Accounts::<T>::insert(&caller, AccountData {
            etr_balance: 10000u64.into(),
            etd_balance: 0u64.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller.clone()), recipient.clone(), TokenType::ETR, amount);

        // Verify transfer occurred
        let recipient_data = Accounts::<T>::get(&recipient);
        assert!(recipient_data.etr_balance > 0u64.into());
    }

    #[benchmark]
    fn mint_etr() {
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: T::Balance = 1000u64.into();

        #[extrinsic_call]
        mint_etr(RawOrigin::Root, recipient.clone(), amount);

        // Verify minting occurred
        let account_data = Accounts::<T>::get(&recipient);
        assert_eq!(account_data.etr_balance, amount);
    }

    #[benchmark]
    fn mint_etd() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: T::Balance = 1000u64.into();

        #[extrinsic_call]
        mint_etd(RawOrigin::Signed(caller), recipient.clone(), amount);

        // Verify minting occurred
        let account_data = Accounts::<T>::get(&recipient);
        assert_eq!(account_data.etd_balance, amount);
    }

    #[benchmark]
    fn burn() {
        let caller: T::AccountId = whitelisted_caller();
        let amount: T::Balance = 500u64.into();

        // Setup: Give caller some ETR balance
        Accounts::<T>::insert(&caller, AccountData {
            etr_balance: 1000u64.into(),
            etd_balance: 0u64.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        #[extrinsic_call]
        burn(RawOrigin::Signed(caller.clone()), TokenType::ETR, amount);

        // Verify burn occurred
        let account_data = Accounts::<T>::get(&caller);
        assert_eq!(account_data.etr_balance, 500u64.into());
    }

    #[benchmark]
    fn create_recovery() {
        let caller: T::AccountId = whitelisted_caller();
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = account("friend2", 0, 0);
        let friend3: T::AccountId = account("friend3", 0, 0);
        let guardians = vec![friend1, friend2, friend3];
        let threshold = 2u32;
        let delay_period: BlockNumberFor<T> = 100u32.into();

        // Setup: Give caller an account
        Accounts::<T>::insert(&caller, AccountData {
            etr_balance: 1000u64.into(),
            etd_balance: 0u64.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        #[extrinsic_call]
        create_recovery(RawOrigin::Signed(caller.clone()), guardians, threshold, delay_period);

        // Verify recovery was created
        assert!(RecoveryConfigs::<T>::contains_key(&caller));
    }

    #[benchmark]
    fn initiate_recovery() {
        let lost_account: T::AccountId = account("lost", 0, 0);
        let friend1: T::AccountId = whitelisted_caller();
        let friend2: T::AccountId = account("friend2", 0, 0);
        let new_account: T::AccountId = account("new", 0, 0);

        // Setup: Create account with recovery config
        Accounts::<T>::insert(&lost_account, AccountData {
            etr_balance: 1000u64.into(),
            etd_balance: 0u64.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        let mut guardians = BoundedVec::new();
        guardians.try_push(friend1.clone()).ok();
        guardians.try_push(friend2).ok();

        RecoveryConfigs::<T>::insert(&lost_account, RecoveryConfig {
            guardians,
            threshold: 2,
            delay_period: 100u32.into(),
        });

        #[extrinsic_call]
        initiate_recovery(RawOrigin::Signed(friend1.clone()), lost_account.clone(), new_account.clone());

        // Verify recovery was initiated
        assert!(ActiveRecoveries::<T>::contains_key(&lost_account));
    }

    #[benchmark]
    fn approve_recovery() {
        let lost_account: T::AccountId = account("lost", 0, 0);
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = whitelisted_caller();
        let new_account: T::AccountId = account("new", 0, 0);

        // Setup: Create account with recovery config
        Accounts::<T>::insert(&lost_account, AccountData {
            etr_balance: 1000u64.into(),
            etd_balance: 0u64.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        let mut guardians = BoundedVec::new();
        guardians.try_push(friend1.clone()).ok();
        guardians.try_push(friend2.clone()).ok();

        RecoveryConfigs::<T>::insert(&lost_account, RecoveryConfig {
            guardians,
            threshold: 2,
            delay_period: 100u32.into(),
        });

        // Setup: Create active recovery with one approval
        let mut approvals = BoundedVec::new();
        approvals.try_push(friend1).ok();

        ActiveRecoveries::<T>::insert(&lost_account, ActiveRecovery {
            new_account,
            approvals,
            created_at: 0u32.into(),
            executable_at: 100u32.into(),
        });

        #[extrinsic_call]
        approve_recovery(RawOrigin::Signed(friend2.clone()), lost_account.clone());

        // Verify approval was recorded
        let recovery = ActiveRecoveries::<T>::get(&lost_account).unwrap();
        assert_eq!(recovery.approvals.len(), 2);
    }

    #[benchmark]
    fn execute_recovery() {
        let lost_account: T::AccountId = account("lost", 0, 0);
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = account("friend2", 0, 0);
        let new_account: T::AccountId = whitelisted_caller();

        // Setup: Create account with recovery config
        Accounts::<T>::insert(&lost_account, AccountData {
            etr_balance: 1000u64.into(),
            etd_balance: 500u64.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        let mut guardians = BoundedVec::new();
        guardians.try_push(friend1.clone()).ok();
        guardians.try_push(friend2.clone()).ok();

        RecoveryConfigs::<T>::insert(&lost_account, RecoveryConfig {
            guardians,
            threshold: 2,
            delay_period: 0u32.into(),  // No delay for benchmark
        });

        // Setup: Create active recovery with threshold met
        let mut approvals = BoundedVec::new();
        approvals.try_push(friend1).ok();
        approvals.try_push(friend2).ok();

        ActiveRecoveries::<T>::insert(&lost_account, ActiveRecovery {
            new_account: new_account.clone(),
            approvals,
            created_at: 0u32.into(),
            executable_at: 0u32.into(),  // Already executable
        });

        #[extrinsic_call]
        execute_recovery(RawOrigin::Signed(new_account.clone()), lost_account.clone());

        // Verify recovery was executed (config and active recovery removed)
        assert!(!RecoveryConfigs::<T>::contains_key(&lost_account));
        assert!(!ActiveRecoveries::<T>::contains_key(&lost_account));
    }

    #[benchmark]
    fn cancel_recovery() {
        let caller: T::AccountId = whitelisted_caller();
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = account("friend2", 0, 0);
        let new_account: T::AccountId = account("new", 0, 0);

        // Setup: Create account with active recovery
        Accounts::<T>::insert(&caller, AccountData {
            etr_balance: 1000u64.into(),
            etd_balance: 0u64.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        let mut approvals = BoundedVec::new();
        approvals.try_push(friend1).ok();
        approvals.try_push(friend2).ok();

        ActiveRecoveries::<T>::insert(&caller, ActiveRecovery {
            new_account,
            approvals,
            created_at: 0u32.into(),
            executable_at: 100u32.into(),
        });

        #[extrinsic_call]
        cancel_recovery(RawOrigin::Signed(caller.clone()), caller.clone());

        // Verify recovery was cancelled
        assert!(!ActiveRecoveries::<T>::contains_key(&caller));
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::tests::new_test_ext(),
        crate::tests::Test
    );
}
