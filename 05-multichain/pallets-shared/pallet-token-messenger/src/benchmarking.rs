//! Benchmarking setup for pallet-token-messenger

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn deposit_for_burn() {
		let caller: T::AccountId = whitelisted_caller();
		let destination_domain = 0u32; // Ethereum
		let amount = 100_000_000_000_000_000_000_u128; // 100 tokens
		let recipient = vec![0x42u8; 20];

		// Setup: Configure domain
		assert_ok!(Pallet::<T>::configure_domain(
			RawOrigin::Root.into(),
			destination_domain,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), amount, destination_domain, recipient);

		assert_eq!(Nonce::<T>::get(destination_domain), 1);
	}

	#[benchmark]
	fn receive_message() {
		let caller: T::AccountId = whitelisted_caller();
		let source_domain = 0u32; // Ethereum
		let local_domain = T::LocalDomain::get();
		let recipient: T::AccountId = whitelisted_caller();

		// Setup: Configure source domain
		assert_ok!(Pallet::<T>::configure_domain(
			RawOrigin::Root.into(),
			source_domain,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		// Create message
		use codec::Encode;
		let burn_msg = BurnMessage {
			version: 1,
			burn_token: b"GENERIC".to_vec().try_into().unwrap(),
			mint_recipient: recipient.encode().try_into().unwrap(),
			amount: 100_000_000_000_000_000_000_u128,
			memo: Default::default(),
		};

		let message = CrossChainMessage {
			version: 1,
			source_domain,
			destination_domain: local_domain,
			nonce: 42,
			sender: vec![0x11u8; 20].try_into().unwrap(),
			recipient: recipient.encode().try_into().unwrap(),
			message_body: burn_msg.encode().try_into().unwrap(),
		};

		let encoded_message = message.encode();
		let attestation = vec![];

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), encoded_message, attestation);

		assert!(UsedNonces::<T>::get(source_domain, 42));
	}

	#[benchmark]
	fn configure_domain() {
		let domain = 0u32;

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			domain,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		);

		assert!(DomainConfigs::<T>::contains_key(domain));
	}

	#[benchmark]
	fn remove_domain() {
		let domain = 0u32;

		// Setup: Configure domain first
		assert_ok!(Pallet::<T>::configure_domain(
			RawOrigin::Root.into(),
			domain,
			true,
			1_000_000_000_000_000_000_000_u128,
			10_000_000_000_000_000_000_000_u128,
			1_000_000_000_000_000_u128,
		));

		#[extrinsic_call]
		_(RawOrigin::Root, domain);

		assert!(!DomainConfigs::<T>::contains_key(domain));
	}

	#[benchmark]
	fn pause_bridge() {
		#[extrinsic_call]
		_(RawOrigin::Root);

		assert!(IsPaused::<T>::get());
	}

	#[benchmark]
	fn unpause_bridge() {
		// Setup: Pause first
		assert_ok!(Pallet::<T>::pause_bridge(RawOrigin::Root.into()));

		#[extrinsic_call]
		_(RawOrigin::Root);

		assert!(!IsPaused::<T>::get());
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
