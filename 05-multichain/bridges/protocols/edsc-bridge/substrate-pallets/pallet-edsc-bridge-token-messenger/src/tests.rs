//! Unit tests for pallet-edsc-bridge-token-messenger

#![cfg(test)]

use crate::{mock::*, *};
use frame_support::{assert_ok, assert_noop};

#[test]
fn test_domain_conversion() {
	assert_eq!(Domain::Ethereum.to_u32(), 0);
	assert_eq!(Domain::Solana.to_u32(), 1);
	assert_eq!(Domain::Etrid.to_u32(), 2);

	assert_eq!(Domain::from_u32(0), Some(Domain::Ethereum));
	assert_eq!(Domain::from_u32(2), Some(Domain::Etrid));
	assert_eq!(Domain::from_u32(99), None);
}

#[test]
fn configure_domain_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(TokenMessenger::configure_domain(
			RuntimeOrigin::root(),
			0, // Ethereum
			true,
			1_000_000_000_000_000_000_000_000, // 1M EDSC
			10_000_000_000_000_000_000_000_000, // 10M EDSC daily
		));

		let config = TokenMessenger::domain_config(0).unwrap();
		assert_eq!(config.enabled, true);
		assert_eq!(config.max_burn_amount, 1_000_000_000_000_000_000_000_000);
	});
}

#[test]
fn pause_unpause_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(TokenMessenger::is_paused(), false);

		assert_ok!(TokenMessenger::pause_bridge(RuntimeOrigin::root()));
		assert_eq!(TokenMessenger::is_paused(), true);

		assert_ok!(TokenMessenger::unpause_bridge(RuntimeOrigin::root()));
		assert_eq!(TokenMessenger::is_paused(), false);
	});
}
