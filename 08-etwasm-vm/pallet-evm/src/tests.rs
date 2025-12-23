use crate::{self as pallet_evm, mock::*, pallet::Error};
use frame_support::{assert_ok, assert_noop};
use sp_core::H160;

#[test]
fn dispatch_succeeds_for_simple_program() {
    new_test_ext().execute_with(|| {
        let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3, 0x00];
        let calldata = vec![0xde, 0xad, 0xbe, 0xef];
        let who = sp_core::crypto::AccountId32::new([1u8; 32]);
        assert_ok!(EVMPallet::call_evm(RuntimeOrigin::signed(who.clone()), bytecode, calldata, 50_000));
        assert_eq!(EVMPallet::known_accounts(H160::repeat_byte(0x00)), None);
    });
}

#[test]
fn fails_on_malformed_calldata() {
    new_test_ext().execute_with(|| {
        let bytecode = vec![0x00];
        let calldata = vec![0x01, 0x02];
        let who = sp_core::crypto::AccountId32::new([2u8; 32]);
        assert_noop!(EVMPallet::call_evm(RuntimeOrigin::signed(who), bytecode, calldata, 1_000), Error::<Test>::MalformedCalldata);
    });
}
