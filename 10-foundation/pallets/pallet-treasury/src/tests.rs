use crate::{mock::*, Error, Event, *};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::traits::AccountIdConversion;

mod test_funding;
mod test_disbursement;
mod test_multisig;
mod test_emergency;
mod test_categories;
mod test_integration;

// Re-export common test utilities
pub use test_funding::*;
pub use test_disbursement::*;
pub use test_multisig::*;
pub use test_emergency::*;
pub use test_categories::*;
pub use test_integration::*;

/// Basic sanity tests
#[test]
fn treasury_account_id_works() {
	new_test_ext().execute_with(|| {
		let treasury_account = Treasury::account_id();
		assert_eq!(treasury_account, AccountIdConversion::<u64>::into_account_truncating(&TreasuryPalletId::get()));
	});
}

#[test]
fn genesis_config_works() {
	new_test_ext().execute_with(|| {
		// Check directors were initialized
		let directors = Treasury::directors();
		assert_eq!(directors.len(), 9);
		assert_eq!(directors.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

		// Check budget allocations
		let allocations = Treasury::budget_allocations();
		assert_eq!(allocations.development_bps, 4000); // 40%
		assert_eq!(allocations.marketing_bps, 2000); // 20%
		assert_eq!(allocations.operations_bps, 1500); // 15%
		assert_eq!(allocations.grants_bps, 1500); // 15%
		assert_eq!(allocations.emergency_reserve_bps, 1000); // 10%
		assert!(allocations.is_valid());

		// Check treasury balance was initialized
		let balance = Treasury::treasury_balance();
		assert_eq!(balance, 500_000_000_000_000u128);
	});
}

#[test]
fn is_director_works() {
	new_test_ext().execute_with(|| {
		assert!(Treasury::is_director(&1));
		assert!(Treasury::is_director(&9));
		assert!(!Treasury::is_director(&10));
		assert!(!Treasury::is_director(&100));
	});
}

#[test]
fn budget_allocations_validation_works() {
	let valid = BudgetAllocations {
		development_bps: 4000,
		marketing_bps: 2000,
		operations_bps: 1500,
		grants_bps: 1500,
		emergency_reserve_bps: 1000,
	};
	assert!(valid.is_valid());

	let invalid = BudgetAllocations {
		development_bps: 4000,
		marketing_bps: 2000,
		operations_bps: 2000, // Total = 10500 (105%)
		grants_bps: 1500,
		emergency_reserve_bps: 1000,
	};
	assert!(!invalid.is_valid());

	let under = BudgetAllocations {
		development_bps: 4000,
		marketing_bps: 2000,
		operations_bps: 1000, // Total = 9500 (95%)
		grants_bps: 1500,
		emergency_reserve_bps: 1000,
	};
	assert!(!under.is_valid());
}

#[test]
fn calculate_allocation_works() {
	new_test_ext().execute_with(|| {
		let total = 100_000 * ETR;

		// 40% = 40,000 ETR
		let dev = Treasury::calculate_allocation(total, 4000);
		assert_eq!(dev, 40_000 * ETR);

		// 20% = 20,000 ETR
		let marketing = Treasury::calculate_allocation(total, 2000);
		assert_eq!(marketing, 20_000 * ETR);

		// 10% = 10,000 ETR
		let emergency = Treasury::calculate_allocation(total, 1000);
		assert_eq!(emergency, 10_000 * ETR);

		// Edge case: 100% = 100,000 ETR
		let all = Treasury::calculate_allocation(total, 10000);
		assert_eq!(all, total);

		// Edge case: 0% = 0 ETR
		let none = Treasury::calculate_allocation(total, 0);
		assert_eq!(none, 0);
	});
}

#[test]
fn category_allocation_by_enum_works() {
	let allocations = BudgetAllocations::default_allocations();

	assert_eq!(
		allocations.get_allocation_bps(&BudgetCategory::Development),
		4000
	);
	assert_eq!(
		allocations.get_allocation_bps(&BudgetCategory::Marketing),
		2000
	);
	assert_eq!(
		allocations.get_allocation_bps(&BudgetCategory::Operations),
		1500
	);
	assert_eq!(
		allocations.get_allocation_bps(&BudgetCategory::Grants),
		1500
	);
	assert_eq!(
		allocations.get_allocation_bps(&BudgetCategory::EmergencyReserve),
		1000
	);
}

#[test]
fn set_budget_allocations_requires_root() {
	new_test_ext().execute_with(|| {
		let new_allocations = BudgetAllocations {
			development_bps: 5000,
			marketing_bps: 2000,
			operations_bps: 1000,
			grants_bps: 1000,
			emergency_reserve_bps: 1000,
		};

		// Non-root should fail
		assert_noop!(
			Treasury::set_budget_allocations(RuntimeOrigin::signed(1), new_allocations.clone()),
			sp_runtime::DispatchError::BadOrigin
		);

		// Root should succeed
		assert_ok!(Treasury::set_budget_allocations(
			RuntimeOrigin::root(),
			new_allocations.clone()
		));

		assert_eq!(Treasury::budget_allocations(), new_allocations);
	});
}

#[test]
fn set_budget_allocations_validates_sum() {
	new_test_ext().execute_with(|| {
		let invalid = BudgetAllocations {
			development_bps: 5000,
			marketing_bps: 2000,
			operations_bps: 2000, // Total = 11000 (110%)
			grants_bps: 1000,
			emergency_reserve_bps: 1000,
		};

		assert_noop!(
			Treasury::set_budget_allocations(RuntimeOrigin::root(), invalid),
			Error::<Test>::InvalidBudgetAllocations
		);
	});
}

#[test]
fn allocate_to_categories_requires_root() {
	new_test_ext().execute_with(|| {
		let amount = 100_000 * ETR;

		// Non-root should fail
		assert_noop!(
			Treasury::allocate_to_categories(RuntimeOrigin::signed(1), amount),
			sp_runtime::DispatchError::BadOrigin
		);

		// Root should succeed
		assert_ok!(Treasury::allocate_to_categories(RuntimeOrigin::root(), amount));
	});
}

#[test]
fn allocate_to_categories_distributes_correctly() {
	new_test_ext().execute_with(|| {
		let total = 100_000 * ETR;

		assert_ok!(Treasury::allocate_to_categories(RuntimeOrigin::root(), total));

		// Check category allocations (40/20/15/15%)
		assert_eq!(
			Treasury::category_allocation(BudgetCategory::Development),
			40_000 * ETR
		);
		assert_eq!(
			Treasury::category_allocation(BudgetCategory::Marketing),
			20_000 * ETR
		);
		assert_eq!(
			Treasury::category_allocation(BudgetCategory::Operations),
			15_000 * ETR
		);
		assert_eq!(
			Treasury::category_allocation(BudgetCategory::Grants),
			15_000 * ETR
		);

		// Emergency reserve is locked separately (10%)
		assert_eq!(Treasury::emergency_reserve(), 10_000 * ETR);
	});
}

#[test]
fn add_director_requires_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Treasury::add_director(RuntimeOrigin::signed(1), 10),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_ok!(Treasury::add_director(RuntimeOrigin::root(), 10));
	});
}

#[test]
fn add_director_works() {
	new_test_ext().execute_with(|| {
		// Directors 1-9 already exist from genesis
		assert_eq!(Treasury::directors().len(), 9);

		// Cannot add existing director
		assert_noop!(
			Treasury::add_director(RuntimeOrigin::root(), 1),
			Error::<Test>::DirectorAlreadyExists
		);

		// Cannot exceed 9 directors (already at max)
		assert_noop!(
			Treasury::add_director(RuntimeOrigin::root(), 10),
			Error::<Test>::MaxDirectorsReached
		);

		// Remove one director first
		assert_ok!(Treasury::remove_director(RuntimeOrigin::root(), 9));
		assert_eq!(Treasury::directors().len(), 8);

		// Now can add new director
		assert_ok!(Treasury::add_director(RuntimeOrigin::root(), 10));
		assert_eq!(Treasury::directors().len(), 9);
		assert!(Treasury::is_director(&10));
	});
}

#[test]
fn remove_director_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(Treasury::directors().len(), 9);

		assert_ok!(Treasury::remove_director(RuntimeOrigin::root(), 9));
		assert_eq!(Treasury::directors().len(), 8);
		assert!(!Treasury::is_director(&9));

		// Cannot remove non-existent director
		assert_noop!(
			Treasury::remove_director(RuntimeOrigin::root(), 100),
			Error::<Test>::DirectorNotFound
		);
	});
}

#[test]
fn funding_source_totals_tracked() {
	new_test_ext().execute_with(|| {
		let amount = 1000 * ETR;

		// Fund from different sources
		assert_ok!(Treasury::fund_treasury(
			RuntimeOrigin::root(),
			FundingSource::TransactionFees,
			amount
		));

		assert_ok!(Treasury::fund_treasury(
			RuntimeOrigin::root(),
			FundingSource::ConsensusDayMinting,
			amount * 2
		));

		assert_ok!(Treasury::fund_treasury(
			RuntimeOrigin::root(),
			FundingSource::ValidatorSlashing,
			amount * 3
		));

		// Check totals
		assert_eq!(
			Treasury::funding_source_totals(FundingSource::TransactionFees),
			amount
		);
		assert_eq!(
			Treasury::funding_source_totals(FundingSource::ConsensusDayMinting),
			amount * 2
		);
		assert_eq!(
			Treasury::funding_source_totals(FundingSource::ValidatorSlashing),
			amount * 3
		);
	});
}

#[test]
fn helper_methods_work() {
	new_test_ext().execute_with(|| {
		// Test receive_transaction_fees
		let fee_amount = 1000 * ETR;
		let initial_balance = Treasury::treasury_balance();

		assert_ok!(Treasury::receive_transaction_fees(fee_amount));

		assert_eq!(
			Treasury::treasury_balance(),
			initial_balance + fee_amount
		);
		assert_eq!(
			Treasury::funding_source_totals(FundingSource::TransactionFees),
			fee_amount
		);

		// Test receive_slashing_proceeds
		let slash_amount = 500 * ETR;
		assert_ok!(Treasury::receive_slashing_proceeds(slash_amount));

		assert_eq!(
			Treasury::treasury_balance(),
			initial_balance + fee_amount + slash_amount
		);
		assert_eq!(
			Treasury::funding_source_totals(FundingSource::ValidatorSlashing),
			slash_amount
		);

		// Test receive_consensus_day_minting
		let mint_amount = 200_000_000 * ETR;
		assert_ok!(Treasury::receive_consensus_day_minting(mint_amount));

		assert_eq!(
			Treasury::treasury_balance(),
			initial_balance + fee_amount + slash_amount + mint_amount
		);
		assert_eq!(
			Treasury::funding_source_totals(FundingSource::ConsensusDayMinting),
			mint_amount
		);

		// Test receive_cross_chain_fees
		let bridge_fee = 100 * ETR;
		assert_ok!(Treasury::receive_cross_chain_fees(bridge_fee));

		assert_eq!(
			Treasury::funding_source_totals(FundingSource::CrossChainFees),
			bridge_fee
		);

		// Test receive_stability_fees
		let stability_fee = 50 * ETR;
		assert_ok!(Treasury::receive_stability_fees(stability_fee));

		assert_eq!(
			Treasury::funding_source_totals(FundingSource::StabilityFees),
			stability_fee
		);
	});
}
