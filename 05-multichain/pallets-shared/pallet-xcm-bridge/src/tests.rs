//! Unit tests for pallet-xcm-bridge

#![cfg(test)]

use crate::{mock::*, *};
use frame_support::{assert_ok, assert_noop};

#[test]
fn test_placeholder() {
	new_test_ext().execute_with(|| {
		// Placeholder test
		assert_eq!(1, 1);
	});
}
