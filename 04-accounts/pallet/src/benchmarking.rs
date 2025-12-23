//! Benchmarking setup for pallet-accounts
//!
//! This module contains benchmarks for all extrinsics in pallet-accounts.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::pallet::{AccountData, TokenType};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_system::Pallet as System;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec;
use sp_std::vec::Vec;

#[benchmarks]
mod benchmarks {
    use super::*;
    use frame_support::BoundedVec;

    #[benchmark]
    fn transfer() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: T::Balance = 1000u32.into();

        // Setup: Give caller some balance
        Accounts::<T>::insert(&caller, AccountData {
            etr_balance: 10000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller.clone()), recipient.clone(), TokenType::ETR, amount);

        // Verify transfer occurred
        assert!(Accounts::<T>::contains_key(&recipient));
    }

    #[benchmark]
    fn mint_etr() {
        let _caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: T::Balance = 1000u32.into();

        #[extrinsic_call]
        mint_etr(RawOrigin::Root, recipient.clone(), amount);

        // Verify minting occurred
        let account_info = Accounts::<T>::get(&recipient);
        assert_eq!(account_info.etr_balance, amount);
    }

    #[benchmark]
    fn mint_etd() {
        let _caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: T::Balance = 1000u32.into();

        #[extrinsic_call]
        mint_etd(RawOrigin::Signed(_caller.clone()), recipient.clone(), amount);

        // Verify minting occurred
        let account_info = Accounts::<T>::get(&recipient);
        assert_eq!(account_info.etd_balance, amount);
    }

    #[benchmark]
    fn burn() {
        let caller: T::AccountId = whitelisted_caller();
        let amount: T::Balance = 500u32.into();

        // Setup: Give caller some ETR balance
        Accounts::<T>::insert(&caller, AccountData {
            etr_balance: 1000u32.into(),
            etd_balance: 0u32.into(),
            nonce: 0,
            is_validator: false,
            reputation: 0,
        });

        #[extrinsic_call]
        burn(RawOrigin::Signed(caller.clone()), TokenType::ETR, amount);

        // Verify burn occurred
        let account_info = Accounts::<T>::get(&caller);
        assert_eq!(account_info.etr_balance, 500u32.into());
    }

    #[benchmark]
    fn create_recovery() {
        let caller: T::AccountId = whitelisted_caller();
        let guardians: Vec<T::AccountId> = vec![account("guardian", 0, 0)];
        let delay: BlockNumberFor<T> = 10u32.into();

        #[extrinsic_call]
        create_recovery(RawOrigin::Signed(caller.clone()), guardians.clone(), 1, delay);

        assert!(RecoveryConfigs::<T>::contains_key(&caller));
    }

    #[benchmark]
    fn initiate_recovery() {
        let guardian: T::AccountId = whitelisted_caller();
        let lost: T::AccountId = account("lost", 0, 0);
        let new_acct: T::AccountId = account("new", 0, 0);
        let delay: BlockNumberFor<T> = 10u32.into();

        let guardians: BoundedVec<_, _> = vec![guardian.clone()].try_into().unwrap();
        let config = RecoveryConfig { guardians, threshold: 1, delay_period: delay };
        RecoveryConfigs::<T>::insert(&lost, config);

        #[extrinsic_call]
        initiate_recovery(RawOrigin::Signed(guardian.clone()), lost.clone(), new_acct.clone());

        assert!(ActiveRecoveries::<T>::contains_key(&lost));
    }

    #[benchmark]
    fn approve_recovery() {
        let guardian1: T::AccountId = whitelisted_caller();
        let guardian2: T::AccountId = account("guardian2", 0, 0);
        let lost: T::AccountId = account("lost", 0, 0);
        let new_acct: T::AccountId = account("new", 0, 0);
        let delay: BlockNumberFor<T> = 10u32.into();

        let guardians: BoundedVec<_, _> = vec![guardian1.clone(), guardian2.clone()].try_into().unwrap();
        let config = RecoveryConfig { guardians, threshold: 2, delay_period: delay };
        RecoveryConfigs::<T>::insert(&lost, config);

        let approvals: BoundedVec<_, _> = vec![guardian1.clone()].try_into().unwrap();
        ActiveRecoveries::<T>::insert(&lost, ActiveRecovery {
            new_account: new_acct.clone(),
            approvals,
            created_at: System::<T>::block_number(),
            executable_at: System::<T>::block_number(),
        });

        #[extrinsic_call]
        approve_recovery(RawOrigin::Signed(guardian2.clone()), lost.clone());

        let recovery = ActiveRecoveries::<T>::get(&lost).unwrap();
        assert!(recovery.approvals.contains(&guardian2));
    }

    #[benchmark]
    fn execute_recovery() {
        let guardian: T::AccountId = whitelisted_caller();
        let lost: T::AccountId = account("lost", 0, 0);
        let new_acct: T::AccountId = account("new", 0, 0);
        let delay: BlockNumberFor<T> = 10u32.into();

        let guardians: BoundedVec<_, _> = vec![guardian.clone()].try_into().unwrap();
        let config = RecoveryConfig { guardians, threshold: 1, delay_period: delay };
        RecoveryConfigs::<T>::insert(&lost, config);

        let approvals: BoundedVec<_, _> = vec![guardian.clone()].try_into().unwrap();
        ActiveRecoveries::<T>::insert(&lost, ActiveRecovery {
            new_account: new_acct.clone(),
            approvals,
            created_at: System::<T>::block_number(),
            executable_at: System::<T>::block_number(),
        });

        // Ensure delay passed
        System::<T>::set_block_number(System::<T>::block_number() + delay + 1u32.into());

        #[extrinsic_call]
        execute_recovery(RawOrigin::Signed(guardian.clone()), lost.clone());

        assert!(!ActiveRecoveries::<T>::contains_key(&lost));
    }

    #[benchmark]
    fn cancel_recovery() {
        let caller: T::AccountId = whitelisted_caller();

        let approvals: BoundedVec<_, _> = vec![caller.clone()].try_into().unwrap();
        ActiveRecoveries::<T>::insert(&caller, ActiveRecovery {
            new_account: caller.clone(),
            approvals,
            created_at: System::<T>::block_number(),
            executable_at: System::<T>::block_number(),
        });

        #[extrinsic_call]
        cancel_recovery(RawOrigin::Signed(caller.clone()), caller.clone());

        assert!(!ActiveRecoveries::<T>::contains_key(&caller));
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
