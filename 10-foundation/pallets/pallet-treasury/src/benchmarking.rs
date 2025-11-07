//! Benchmarking setup for pallet-treasury
//!
//! This module contains benchmarks for all extrinsics in pallet-treasury.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use frame_support::traits::fungible::Mutate;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn fund_treasury() {
        let caller: T::AccountId = whitelisted_caller();
        let amount: BalanceOf<T> = 10000u32.into();

        // Setup: Give caller some balance
        T::Currency::mint_into(&caller, amount * 2u32.into()).unwrap();

        #[extrinsic_call]
        fund_treasury(
            RawOrigin::Signed(caller.clone()),
            FundingSource::ExternalDonation,
            amount
        );

        // Verify treasury balance increased
        let treasury_account = Pallet::<T>::account_id();
        assert!(T::Currency::balance(&treasury_account) >= amount);
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

        // Fund treasury first
        let treasury_account = Pallet::<T>::account_id();
        T::Currency::mint_into(&treasury_account, amount * 10u32.into()).unwrap();

        #[extrinsic_call]
        propose_disbursement(
            RawOrigin::Signed(caller.clone()),
            recipient.clone(),
            amount,
            BudgetCategory::Development
        );

        // Verify proposal was created
        assert!(ProposalCount::<T>::get() > 0);
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
        T::Currency::mint_into(&treasury_account, amount * 10u32.into()).unwrap();

        // Create a proposal
        let proposal_id = ProposalCount::<T>::get();
        let proposal = DisbursementProposal {
            proposer: director1.clone(),
            recipient: recipient.clone(),
            amount,
            category: BudgetCategory::Development,
            approvals: BoundedVec::new(),
            status: ProposalStatus::Pending,
            created_at: frame_system::Pallet::<T>::block_number(),
        };
        Proposals::<T>::insert(proposal_id, proposal);
        ProposalCount::<T>::put(proposal_id + 1);

        #[extrinsic_call]
        approve_disbursement(RawOrigin::Signed(director1.clone()), proposal_id);

        // Verify approval was recorded
        let updated_proposal = Proposals::<T>::get(proposal_id).unwrap();
        assert!(updated_proposal.approvals.len() > 0);
    }

    #[benchmark]
    fn emergency_withdrawal() {
        let director1: T::AccountId = whitelisted_caller();
        let director2: T::AccountId = account("director2", 0, 0);
        let director3: T::AccountId = account("director3", 0, 0);
        let director4: T::AccountId = account("director4", 0, 0);
        let director5: T::AccountId = account("director5", 0, 0);
        let director6: T::AccountId = account("director6", 0, 0);
        let director7: T::AccountId = account("director7", 0, 0);
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount: BalanceOf<T> = 5000u32.into();

        // Setup: Create enough directors for emergency threshold (7-of-9)
        let mut directors = BoundedVec::new();
        directors.try_push(director1.clone()).ok();
        directors.try_push(director2.clone()).ok();
        directors.try_push(director3.clone()).ok();
        directors.try_push(director4.clone()).ok();
        directors.try_push(director5.clone()).ok();
        directors.try_push(director6.clone()).ok();
        directors.try_push(director7.clone()).ok();
        Directors::<T>::put(directors);

        // Fund treasury
        let treasury_account = Pallet::<T>::account_id();
        T::Currency::mint_into(&treasury_account, amount * 10u32.into()).unwrap();

        // Create an emergency proposal
        let proposal_id = ProposalCount::<T>::get();
        let mut approvals = BoundedVec::new();
        // Add 6 approvals (need 7 total with the benchmark call)
        approvals.try_push(director2).ok();
        approvals.try_push(director3).ok();
        approvals.try_push(director4).ok();
        approvals.try_push(director5).ok();
        approvals.try_push(director6).ok();
        approvals.try_push(director7).ok();

        let proposal = DisbursementProposal {
            proposer: director1.clone(),
            recipient: recipient.clone(),
            amount,
            category: BudgetCategory::Emergency,
            approvals,
            status: ProposalStatus::Pending,
            created_at: frame_system::Pallet::<T>::block_number(),
        };
        Proposals::<T>::insert(proposal_id, proposal);
        ProposalCount::<T>::put(proposal_id + 1);

        #[extrinsic_call]
        emergency_withdrawal(RawOrigin::Signed(director1.clone()), proposal_id);

        // Verify emergency withdrawal was executed
        let updated_proposal = Proposals::<T>::get(proposal_id).unwrap();
        assert_eq!(updated_proposal.status, ProposalStatus::Executed);
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
