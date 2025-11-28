//! Benchmarking setup for pallet-accounts
//!
//! This module contains benchmarks for all extrinsics in pallet-accounts.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn transfer() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 1000u32.into();

        // Setup: Give caller some balance
        Accounts::<T>::insert(&caller, AccountInfo {
            etr_balance: 10000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            recovery: None,
        });

        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller.clone()), recipient.clone(), amount);

        // Verify transfer occurred
        assert!(Accounts::<T>::contains_key(&recipient));
    }

    #[benchmark]
    fn mint_etr() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 1000u32.into();

        #[extrinsic_call]
        mint_etr(RawOrigin::Root, recipient.clone(), amount);

        // Verify minting occurred
        let account_info = Accounts::<T>::get(&recipient).unwrap();
        assert_eq!(account_info.etr_balance, amount);
    }

    #[benchmark]
    fn mint_etd() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 1000u32.into();

        #[extrinsic_call]
        mint_etd(RawOrigin::Root, recipient.clone(), amount);

        // Verify minting occurred
        let account_info = Accounts::<T>::get(&recipient).unwrap();
        assert_eq!(account_info.etd_balance, amount);
    }

    #[benchmark]
    fn burn() {
        let caller: T::AccountId = whitelisted_caller();
        let amount: BalanceOf<T> = 500u32.into();

        // Setup: Give caller some ETR balance
        Accounts::<T>::insert(&caller, AccountInfo {
            etr_balance: 1000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            recovery: None,
        });

        #[extrinsic_call]
        burn(RawOrigin::Signed(caller.clone()), amount);

        // Verify burn occurred
        let account_info = Accounts::<T>::get(&caller).unwrap();
        assert_eq!(account_info.etr_balance, 500u32.into());
    }

    #[benchmark]
    fn create_recovery() {
        let caller: T::AccountId = whitelisted_caller();
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = account("friend2", 0, 0);
        let friend3: T::AccountId = account("friend3", 0, 0);
        let friends = vec![friend1, friend2, friend3];
        let threshold = 2u32;

        // Setup: Give caller an account
        Accounts::<T>::insert(&caller, AccountInfo {
            etr_balance: 1000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            recovery: None,
        });

        #[extrinsic_call]
        create_recovery(RawOrigin::Signed(caller.clone()), friends, threshold);

        // Verify recovery was created
        let account_info = Accounts::<T>::get(&caller).unwrap();
        assert!(account_info.recovery.is_some());
    }

    #[benchmark]
    fn initiate_recovery() {
        let lost_account: T::AccountId = account("lost", 0, 0);
        let friend1: T::AccountId = whitelisted_caller();
        let friend2: T::AccountId = account("friend2", 0, 0);
        let new_account: T::AccountId = account("new", 0, 0);

        // Setup: Create account with recovery
        Accounts::<T>::insert(&lost_account, AccountInfo {
            etr_balance: 1000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            recovery: Some(RecoveryConfig {
                friends: vec![friend1.clone(), friend2],
                threshold: 2,
                approvals: vec![],
                new_account_id: None,
            }),
        });

        #[extrinsic_call]
        initiate_recovery(RawOrigin::Signed(friend1.clone()), lost_account.clone(), new_account);

        // Verify recovery was initiated
        let account_info = Accounts::<T>::get(&lost_account).unwrap();
        assert!(account_info.recovery.unwrap().new_account_id.is_some());
    }

    #[benchmark]
    fn approve_recovery() {
        let lost_account: T::AccountId = account("lost", 0, 0);
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = whitelisted_caller();
        let new_account: T::AccountId = account("new", 0, 0);

        // Setup: Create account with recovery in progress
        Accounts::<T>::insert(&lost_account, AccountInfo {
            etr_balance: 1000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            recovery: Some(RecoveryConfig {
                friends: vec![friend1.clone(), friend2.clone()],
                threshold: 2,
                approvals: vec![friend1.clone()],
                new_account_id: Some(new_account.clone()),
            }),
        });

        #[extrinsic_call]
        approve_recovery(RawOrigin::Signed(friend2.clone()), lost_account.clone());

        // Verify approval was recorded
        let account_info = Accounts::<T>::get(&lost_account).unwrap();
        let recovery = account_info.recovery.unwrap();
        assert_eq!(recovery.approvals.len(), 2);
    }

    #[benchmark]
    fn execute_recovery() {
        let lost_account: T::AccountId = account("lost", 0, 0);
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = account("friend2", 0, 0);
        let new_account: T::AccountId = whitelisted_caller();

        // Setup: Create account with recovery ready to execute
        Accounts::<T>::insert(&lost_account, AccountInfo {
            etr_balance: 1000u32.into(),
            etd_balance: 500u32.into(),
            nonce: 0,
            recovery: Some(RecoveryConfig {
                friends: vec![friend1.clone(), friend2.clone()],
                threshold: 2,
                approvals: vec![friend1, friend2],
                new_account_id: Some(new_account.clone()),
            }),
        });

        #[extrinsic_call]
        execute_recovery(RawOrigin::Signed(new_account.clone()), lost_account.clone());

        // Verify recovery was executed (new account should have the balance)
        assert!(Accounts::<T>::contains_key(&new_account));
    }

    #[benchmark]
    fn cancel_recovery() {
        let caller: T::AccountId = whitelisted_caller();
        let friend1: T::AccountId = account("friend1", 0, 0);
        let friend2: T::AccountId = account("friend2", 0, 0);

        // Setup: Create account with recovery
        Accounts::<T>::insert(&caller, AccountInfo {
            etr_balance: 1000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            recovery: Some(RecoveryConfig {
                friends: vec![friend1, friend2],
                threshold: 2,
                approvals: vec![],
                new_account_id: None,
            }),
        });

        #[extrinsic_call]
        cancel_recovery(RawOrigin::Signed(caller.clone()));

        // Verify recovery was cancelled
        let account_info = Accounts::<T>::get(&caller).unwrap();
        assert!(account_info.recovery.is_none());
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
