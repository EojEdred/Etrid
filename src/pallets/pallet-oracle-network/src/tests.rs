//! Tests for oracle network pallet

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::StaticLookup;

#[test]
fn register_oracle_works() {
	new_test_ext().execute_with(|| {
		// Register oracle with sufficient stake
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000));

		// Check oracle is registered
		let operator = OracleNetwork::oracle_operators(1).unwrap();
		assert_eq!(operator.stake, 1000);
		assert_eq!(operator.reputation, 100);
		assert_eq!(operator.active, true);

		// Check event
		System::assert_last_event(Event::OracleRegistered {
			operator: 1,
			stake: 1000,
		}.into());
	});
}

#[test]
fn register_oracle_fails_insufficient_stake() {
	new_test_ext().execute_with(|| {
		// Try to register with stake below minimum
		assert_noop!(
			OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 500),
			Error::<Test>::InsufficientStake
		);
	});
}

#[test]
fn register_oracle_fails_already_registered() {
	new_test_ext().execute_with(|| {
		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000));

		// Try to register again
		assert_noop!(
			OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000),
			Error::<Test>::OracleAlreadyRegistered
		);
	});
}

#[test]
fn deregister_oracle_works() {
	new_test_ext().execute_with(|| {
		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000));

		// Deregister oracle
		assert_ok!(OracleNetwork::deregister_oracle(RuntimeOrigin::signed(1)));

		// Check oracle is removed
		assert!(OracleNetwork::oracle_operators(1).is_none());

		// Check event
		System::assert_last_event(Event::OracleDeregistered {
			operator: 1,
		}.into());
	});
}

#[test]
fn increase_stake_works() {
	new_test_ext().execute_with(|| {
		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000));

		// Increase stake
		assert_ok!(OracleNetwork::increase_stake(RuntimeOrigin::signed(1), 500));

		// Check new stake
		let operator = OracleNetwork::oracle_operators(1).unwrap();
		assert_eq!(operator.stake, 1500);

		// Check event
		System::assert_last_event(Event::StakeIncreased {
			operator: 1,
			additional: 500,
			new_total: 1500,
		}.into());
	});
}

#[test]
fn decrease_stake_works() {
	new_test_ext().execute_with(|| {
		// Register oracle with higher stake
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 2000));

		// Decrease stake
		assert_ok!(OracleNetwork::decrease_stake(RuntimeOrigin::signed(1), 500));

		// Check new stake
		let operator = OracleNetwork::oracle_operators(1).unwrap();
		assert_eq!(operator.stake, 1500);

		// Check event
		System::assert_last_event(Event::StakeDecreased {
			operator: 1,
			amount: 500,
			new_total: 1500,
		}.into());
	});
}

#[test]
fn decrease_stake_fails_below_minimum() {
	new_test_ext().execute_with(|| {
		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000));

		// Try to decrease below minimum
		assert_noop!(
			OracleNetwork::decrease_stake(RuntimeOrigin::signed(1), 500),
			Error::<Test>::CannotUnstakeBelowMinimum
		);
	});
}

#[test]
fn create_data_request_works() {
	new_test_ext().execute_with(|| {
		let data_spec = b"BTC/USD".to_vec();
		let payment = 100;
		let min_oracles = 3;
		let expiration = 1000;

		// Create data request
		assert_ok!(OracleNetwork::create_data_request(
			RuntimeOrigin::signed(1),
			data_spec.clone(),
			payment,
			min_oracles,
			expiration
		));

		// Check request is created
		let request = OracleNetwork::data_requests(0).unwrap();
		assert_eq!(request.requester, 1);
		assert_eq!(request.payment_per_oracle, payment);
		assert_eq!(request.min_oracles, min_oracles);

		// Check event
		System::assert_last_event(Event::DataRequestCreated {
			request_id: 0,
			requester: 1,
			data_spec,
		}.into());
	});
}

#[test]
fn submit_oracle_response_works() {
	new_test_ext().execute_with(|| {
		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(2), 1000));

		// Create data request
		let data_spec = b"BTC/USD".to_vec();
		assert_ok!(OracleNetwork::create_data_request(
			RuntimeOrigin::signed(1),
			data_spec,
			100,
			2,
			1000
		));

		// Submit oracle response
		let response_data = b"50000".to_vec(); // $50,000
		assert_ok!(OracleNetwork::submit_oracle_response(
			RuntimeOrigin::signed(2),
			0,
			response_data
		));

		// Check event
		System::assert_last_event(Event::DataResponseSubmitted {
			request_id: 0,
			oracle: 2,
		}.into());
	});
}

#[test]
fn submit_oracle_response_fails_not_registered() {
	new_test_ext().execute_with(|| {
		// Create data request
		let data_spec = b"BTC/USD".to_vec();
		assert_ok!(OracleNetwork::create_data_request(
			RuntimeOrigin::signed(1),
			data_spec,
			100,
			2,
			1000
		));

		// Try to submit response without being registered
		let response_data = b"50000".to_vec();
		assert_noop!(
			OracleNetwork::submit_oracle_response(
				RuntimeOrigin::signed(2),
				0,
				response_data
			),
			Error::<Test>::OracleNotFound
		);
	});
}

#[test]
fn slash_oracle_works() {
	new_test_ext().execute_with(|| {
		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(2), 2000));

		let reason = b"Bad data submission".to_vec();

		// Slash oracle (governance)
		assert_ok!(OracleNetwork::slash_oracle(
			RuntimeOrigin::root(),
			<Test as frame_system::Config>::Lookup::unlookup(2),
			reason.clone()
		));

		// Check stake reduced (5% slash)
		let operator = OracleNetwork::oracle_operators(2).unwrap();
		assert_eq!(operator.stake, 1900); // 2000 - 5% = 1900

		// Check reputation decreased
		assert_eq!(operator.reputation, 90); // 100 - 10 = 90

		// Check event
		System::assert_has_event(Event::OracleSlashed {
			operator: 2,
			slashed_amount: 100,
			reason,
		}.into());
	});
}

#[test]
fn oracle_reputation_system_works() {
	new_test_ext().execute_with(|| {
		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(2), 1000));

		// Create data request
		assert_ok!(OracleNetwork::create_data_request(
			RuntimeOrigin::signed(1),
			b"ETH/USD".to_vec(),
			100,
			1,
			1000
		));

		// Submit successful response
		assert_ok!(OracleNetwork::submit_oracle_response(
			RuntimeOrigin::signed(2),
			0,
			b"3000".to_vec()
		));

		// Check reputation stayed at 100 (already max)
		let operator = OracleNetwork::oracle_operators(2).unwrap();
		assert_eq!(operator.reputation, 100);
		assert_eq!(operator.successful_submissions, 1);
	});
}

#[test]
fn is_oracle_active_works() {
	new_test_ext().execute_with(|| {
		// Not registered
		assert_eq!(OracleNetwork::is_oracle_active(&1), false);

		// Register oracle
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000));

		// Active
		assert_eq!(OracleNetwork::is_oracle_active(&1), true);
	});
}

#[test]
fn get_active_oracles_works() {
	new_test_ext().execute_with(|| {
		// Register multiple oracles
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(1), 1000));
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(2), 1000));
		assert_ok!(OracleNetwork::register_oracle(RuntimeOrigin::signed(3), 1000));

		// Get active oracles
		let active = OracleNetwork::get_active_oracles();
		assert_eq!(active.len(), 3);
		assert!(active.contains(&1));
		assert!(active.contains(&2));
		assert!(active.contains(&3));
	});
}
