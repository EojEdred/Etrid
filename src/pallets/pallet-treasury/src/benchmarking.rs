//! Benchmarking setup for pallet-treasury
//!
//! This module contains benchmarks for all extrinsics in pallet-treasury.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{BudgetCategory, FundingSource, DisbursementStatus};
use crate::pallet::{Disbursement, BalanceOf};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::traits::Currency;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn fund_treasury() {
        let caller: T::AccountId = whitelisted_caller();
        let amount: BalanceOf<T> = 10000u32.into();

        // Setup: Give caller some balance to deposit
        T::Currency::make_free_balance_be(&caller, amount * 2u32.into());

        #[extrinsic_call]
        fund_treasury(
            RawOrigin::Signed(caller.clone()),
            FundingSource::Other,
            amount
        );

        // Verify treasury balance increased
        let treasury_balance = TreasuryBalance::<T>::get();
        assert!(treasury_balance >= amount);
    }

    #[benchmark]
    fn propose_disbursement() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 1000u32.into();
        let description = vec![1u8; 100];

        // Setup: Make caller a director
        let mut directors = BoundedVec::new();
        directors.try_push(caller.clone()).ok();
        Directors::<T>::put(directors);

        // Fund treasury first
        let treasury_account = Pallet::<T>::account_id();
        T::Currency::make_free_balance_be(&treasury_account, amount * 10u32.into());
        TreasuryBalance::<T>::put(amount * 10u32.into());

        #[extrinsic_call]
        propose_disbursement(
            RawOrigin::Signed(caller.clone()),
            BudgetCategory::Development,
            recipient.clone(),
            amount,
            description
        );

        // Verify proposal was created (check next ID incremented)
        assert!(DisbursementCount::<T>::get() > 0);
    }

    #[benchmark]
    fn approve_disbursement() {
        let director1: T::AccountId = whitelisted_caller();
        let director2: T::AccountId = account("director2", 0, 0);
        let director3: T::AccountId = account("director3", 0, 0);
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 1000u32.into();

        // Setup: Create directors
        let mut directors = BoundedVec::new();
        directors.try_push(director1.clone()).ok();
        directors.try_push(director2.clone()).ok();
        directors.try_push(director3).ok();
        Directors::<T>::put(directors);

        // Fund treasury
        let treasury_account = Pallet::<T>::account_id();
        T::Currency::make_free_balance_be(&treasury_account, amount * 10u32.into());
        TreasuryBalance::<T>::put(amount * 10u32.into());

        // Create a disbursement manually
        let disbursement_id = 0u64;
        let disbursement = Disbursement {
            id: disbursement_id,
            category: BudgetCategory::Development,
            recipient: recipient.clone(),
            amount,
            description: BoundedVec::try_from(vec![1u8; 100]).unwrap_or_default(),
            proposer: director2.clone(),
            status: DisbursementStatus::Pending,
            proposed_at: frame_system::Pallet::<T>::block_number(),
            approval_count: 0,
            is_emergency: false,
        };
        Disbursements::<T>::insert(disbursement_id, disbursement);
        DisbursementCount::<T>::put(1u64);

        #[extrinsic_call]
        approve_disbursement(RawOrigin::Signed(director1.clone()), disbursement_id);

        // Verify approval was recorded
        let approvals = DirectorApprovals::<T>::get(disbursement_id);
        assert!(approvals.len() > 0);
    }

    #[benchmark]
    fn emergency_withdrawal() {
        let director1: T::AccountId = whitelisted_caller();
        let director2: T::AccountId = account("director2", 0, 0);
        let director3: T::AccountId = account("director3", 0, 0);
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 5000u32.into();
        let description = vec![1u8; 100];

        // Setup: Create enough directors
        let mut directors = BoundedVec::new();
        directors.try_push(director1.clone()).ok();
        directors.try_push(director2).ok();
        directors.try_push(director3).ok();
        Directors::<T>::put(directors);

        // Fund emergency reserve
        let treasury_account = Pallet::<T>::account_id();
        T::Currency::make_free_balance_be(&treasury_account, amount * 10u32.into());
        EmergencyReserve::<T>::put(amount * 5u32.into());

        #[extrinsic_call]
        emergency_withdrawal(
            RawOrigin::Signed(director1.clone()),
            recipient.clone(),
            amount,
            description
        );

        // Verify emergency withdrawal proposal was created
        assert!(DisbursementCount::<T>::get() > 0);
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
