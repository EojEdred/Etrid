//! Benchmarking setup for pallet-treasury
//!
//! This module contains benchmarks for all extrinsics in pallet-treasury.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::traits::Currency;
use frame_support::BoundedVec;

#[benchmarks]
mod benchmarks {
    use super::*;
    type BalanceOf<T> =
        <<T as crate::pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[benchmark]
    fn fund_treasury() {
        let caller: T::AccountId = whitelisted_caller();
        let amount: BalanceOf<T> = 10_000u32.into();

        #[extrinsic_call]
        fund_treasury(
            RawOrigin::Signed(caller.clone()),
            FundingSource::Other,
            amount
        );

        // Verify treasury balance increased (tracked in storage)
        assert!(TreasuryBalance::<T>::get() >= amount);
    }

    #[benchmark]
    fn propose_disbursement() {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 1000u32.into();

        // Setup: Make caller a director
        let mut directors = Directors::<T>::get();
        if !directors.contains(&caller) {
            directors.try_push(caller.clone()).ok();
            Directors::<T>::put(directors);
        }

        // Seed treasury balance
        TreasuryBalance::<T>::put(amount * 2u32.into());

        #[extrinsic_call]
        propose_disbursement(
            RawOrigin::Signed(caller.clone()),
            BudgetCategory::Development,
            recipient.clone(),
            amount,
            b"dev funds".to_vec()
        );

        // Verify disbursement was created
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

        TreasuryBalance::<T>::put(amount * 2u32.into());

        // Create a proposal
        let disbursement_id = DisbursementCount::<T>::get();
        let disbursement = Disbursement {
            id: disbursement_id,
            category: BudgetCategory::Development,
            recipient: recipient.clone(),
            amount,
            description: BoundedVec::default(),
            proposer: director1.clone(),
            status: DisbursementStatus::Pending,
            proposed_at: frame_system::Pallet::<T>::block_number(),
            approval_count: 0,
            is_emergency: false,
        };
        Disbursements::<T>::insert(disbursement_id, disbursement);

        #[extrinsic_call]
        approve_disbursement(RawOrigin::Signed(director1.clone()), disbursement_id);

        // Verify approval was recorded
        let approvals = DirectorApprovals::<T>::get(disbursement_id);
        assert!(!approvals.is_empty());
    }

    #[benchmark]
    fn emergency_withdrawal() {
        let director: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 1_000u32.into();

        let mut directors = BoundedVec::new();
        let _ = directors.try_push(director.clone());
        Directors::<T>::put(directors);

        EmergencyReserve::<T>::put(amount * 2u32.into());

        #[extrinsic_call]
        emergency_withdrawal(
            RawOrigin::Signed(director.clone()),
            recipient.clone(),
            amount,
            b"emergency".to_vec()
        );

        let last_id = DisbursementCount::<T>::get().saturating_sub(1);
        assert!(Disbursements::<T>::contains_key(last_id));
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
