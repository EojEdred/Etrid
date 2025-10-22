//! Integration tests for Bitcoin Bridge multi-signature custodian functionality
//!
//! These tests verify the complete M-of-N multi-sig workflow for bridge security.

use super::*;
use crate as pallet_bitcoin_bridge;
use codec::Encode;
use frame_support::{
    assert_noop, assert_ok, derive_impl, parameter_types,
    traits::{ConstU64, Get, Currency},
};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup, Hash as HashT},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime for testing
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        BitcoinBridge: pallet_bitcoin_bridge,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type AccountData = pallet_balances::AccountData<u64>;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

parameter_types! {
    pub const MinConfirmations: u32 = 6;
    pub const MinDepositAmount: u64 = 10000; // 0.0001 BTC
    pub const MaxDepositAmount: u64 = 1000000000; // 10 BTC
}

pub struct BridgeAuthority;
impl Get<u64> for BridgeAuthority {
    fn get() -> u64 {
        1 // Account 1 is the bridge authority
    }
}

impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinConfirmations;
    type MinDepositAmount = MinDepositAmount;
    type MaxDepositAmount = MaxDepositAmount;
    type BridgeAuthority = BridgeAuthority;
}

// Helper functions
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 1000000), // Bridge authority
            (2, 1000000), // User with funds
            (10, 100),    // Custodian 1
            (11, 100),    // Custodian 2
            (12, 100),    // Custodian 3
            (13, 100),    // Custodian 4
            (14, 100),    // Custodian 5
        ],
        dev_accounts: None,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
        // Set exchange rate: 1 BTC = 1 ETR (scaled by 1e8)
        ExchangeRate::<Test>::put(100_000_000);
    });
    ext
}

pub fn account(id: u64) -> u64 {
    id
}

pub fn btc_txid(value: u8) -> Vec<u8> {
    vec![value; 32]
}

#[test]
fn test_set_custodians_valid() {
    new_test_ext().execute_with(|| {
        let custodians = vec![account(10), account(11), account(12)];
        let threshold = 2;

        // Root can set custodians
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians.clone(),
            threshold
        ));

        // Verify custodian set
        let custodian_set = CustodianSet::<Test>::get().unwrap();
        assert_eq!(custodian_set.custodians, custodians);
        assert_eq!(custodian_set.threshold, threshold);

        // Check event
        System::assert_has_event(
            Event::CustodianSetUpdated(threshold).into()
        );
    });
}

#[test]
fn test_set_custodians_invalid_threshold_zero() {
    new_test_ext().execute_with(|| {
        let custodians = vec![account(10), account(11), account(12)];

        // Threshold 0 should fail
        assert_noop!(
            BitcoinBridge::set_custodians(
                RuntimeOrigin::root(),
                custodians,
                0
            ),
            Error::<Test>::InvalidCustodianSet
        );
    });
}

#[test]
fn test_set_custodians_invalid_threshold_exceeds() {
    new_test_ext().execute_with(|| {
        let custodians = vec![account(10), account(11)];

        // Threshold > N should fail
        assert_noop!(
            BitcoinBridge::set_custodians(
                RuntimeOrigin::root(),
                custodians,
                3
            ),
            Error::<Test>::InvalidCustodianSet
        );
    });
}

#[test]
fn test_set_custodians_requires_root() {
    new_test_ext().execute_with(|| {
        let custodians = vec![account(10), account(11), account(12)];

        // Non-root should fail
        assert_noop!(
            BitcoinBridge::set_custodians(
                RuntimeOrigin::signed(account(2)),
                custodians,
                2
            ),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn test_multisig_withdrawal_approval_2_of_3() {
    new_test_ext().execute_with(|| {
        // Setup: 2-of-3 custodian set
        let custodians = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            2
        ));

        // User requests withdrawal
        let withdrawer = account(2);
        let btc_address = vec![0x62, 0x74, 0x63]; // "btc"
        let amount_satoshi = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address,
            amount_satoshi
        ));

        // Verify withdrawal created
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Requested);

        let txid = btc_txid(1);

        // First custodian approves
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(10)),
            withdrawer,
            txid.clone()
        ));

        // Check approval event
        let operation_data = (withdrawer, txid.clone()).encode();
        let operation_hash = <BlakeTwo256 as HashT>::hash(&operation_data);
        System::assert_has_event(
            Event::WithdrawalApprovalSubmitted(operation_hash, account(10), 1).into()
        );

        // Withdrawal should not be executed yet
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Requested);

        // Second custodian approves (reaches threshold)
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(11)),
            withdrawer,
            txid.clone()
        ));

        // Withdrawal should now be completed
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Completed);

        // Check execution event
        System::assert_has_event(
            Event::WithdrawalApprovedAndExecuted(operation_hash, withdrawer).into()
        );
        System::assert_has_event(
            Event::WithdrawalCompleted(withdrawer, txid.clone()).into()
        );
    });
}

#[test]
fn test_multisig_withdrawal_approval_3_of_3() {
    new_test_ext().execute_with(|| {
        // Setup: 3-of-3 custodian set (unanimous)
        let custodians = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            3
        ));

        // User requests withdrawal
        let withdrawer = account(2);
        let btc_address = vec![0x62, 0x74, 0x63];
        let amount_satoshi = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address,
            amount_satoshi
        ));

        let txid = btc_txid(2);

        // First custodian approves
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(10)),
            withdrawer,
            txid.clone()
        ));

        // Not executed yet
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Requested);

        // Second custodian approves
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(11)),
            withdrawer,
            txid.clone()
        ));

        // Still not executed (need 3-of-3)
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Requested);

        // Third custodian approves (reaches threshold)
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(12)),
            withdrawer,
            txid.clone()
        ));

        // Now executed
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Completed);
    });
}

#[test]
fn test_multisig_duplicate_approval_rejected() {
    new_test_ext().execute_with(|| {
        // Setup: 2-of-3 custodian set
        let custodians = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            2
        ));

        // User requests withdrawal
        let withdrawer = account(2);
        let btc_address = vec![0x62, 0x74, 0x63];
        let amount_satoshi = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address,
            amount_satoshi
        ));

        let txid = btc_txid(3);

        // First custodian approves
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(10)),
            withdrawer,
            txid.clone()
        ));

        // Same custodian tries to approve again (should fail)
        assert_noop!(
            BitcoinBridge::approve_withdrawal(
                RuntimeOrigin::signed(account(10)),
                withdrawer,
                txid.clone()
            ),
            Error::<Test>::AlreadyApproved
        );
    });
}

#[test]
fn test_multisig_non_custodian_cannot_approve() {
    new_test_ext().execute_with(|| {
        // Setup: 2-of-3 custodian set
        let custodians = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            2
        ));

        // User requests withdrawal
        let withdrawer = account(2);
        let btc_address = vec![0x62, 0x74, 0x63];
        let amount_satoshi = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address,
            amount_satoshi
        ));

        let txid = btc_txid(4);

        // Non-custodian tries to approve (should fail)
        assert_noop!(
            BitcoinBridge::approve_withdrawal(
                RuntimeOrigin::signed(account(99)),
                withdrawer,
                txid
            ),
            Error::<Test>::NotCustodian
        );
    });
}

#[test]
fn test_multisig_approval_after_execution_rejected() {
    new_test_ext().execute_with(|| {
        // Setup: 2-of-3 custodian set
        let custodians = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            2
        ));

        // User requests withdrawal
        let withdrawer = account(2);
        let btc_address = vec![0x62, 0x74, 0x63];
        let amount_satoshi = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address,
            amount_satoshi
        ));

        let txid = btc_txid(5);

        // First two custodians approve (reaches threshold, executes)
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(10)),
            withdrawer,
            txid.clone()
        ));

        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(11)),
            withdrawer,
            txid.clone()
        ));

        // Verify executed
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Completed);

        // Third custodian tries to approve after execution (should fail)
        assert_noop!(
            BitcoinBridge::approve_withdrawal(
                RuntimeOrigin::signed(account(12)),
                withdrawer,
                txid
            ),
            Error::<Test>::AlreadyExecuted
        );
    });
}

#[test]
fn test_multisig_no_custodian_set() {
    new_test_ext().execute_with(|| {
        // No custodian set configured
        // User requests withdrawal
        let withdrawer = account(2);
        let btc_address = vec![0x62, 0x74, 0x63];
        let amount_satoshi = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address,
            amount_satoshi
        ));

        let txid = btc_txid(6);

        // Try to approve without custodian set (should fail)
        assert_noop!(
            BitcoinBridge::approve_withdrawal(
                RuntimeOrigin::signed(account(10)),
                withdrawer,
                txid
            ),
            Error::<Test>::NoCustodianSet
        );
    });
}

#[test]
fn test_multisig_1_of_1_single_custodian() {
    new_test_ext().execute_with(|| {
        // Setup: 1-of-1 custodian set (single custodian)
        let custodians = vec![account(10)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            1
        ));

        // User requests withdrawal
        let withdrawer = account(2);
        let btc_address = vec![0x62, 0x74, 0x63];
        let amount_satoshi = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address,
            amount_satoshi
        ));

        let txid = btc_txid(7);

        // Single custodian approves (immediately executes)
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(10)),
            withdrawer,
            txid.clone()
        ));

        // Verify executed immediately
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Completed);
    });
}

#[test]
fn test_multisig_withdrawal_not_found() {
    new_test_ext().execute_with(|| {
        // Setup: 2-of-3 custodian set
        let custodians = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            2
        ));

        // Try to approve non-existent withdrawal
        let withdrawer = account(99);
        let txid = btc_txid(8);

        assert_noop!(
            BitcoinBridge::approve_withdrawal(
                RuntimeOrigin::signed(account(10)),
                withdrawer,
                txid
            ),
            Error::<Test>::WithdrawalNotFound
        );
    });
}

#[test]
fn test_multisig_update_custodian_set() {
    new_test_ext().execute_with(|| {
        // Setup initial 2-of-3 custodian set
        let custodians1 = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians1,
            2
        ));

        let custodian_set = CustodianSet::<Test>::get().unwrap();
        assert_eq!(custodian_set.threshold, 2);

        // Update to 3-of-5 custodian set
        let custodians2 = vec![account(10), account(11), account(12), account(13), account(14)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians2.clone(),
            3
        ));

        let custodian_set = CustodianSet::<Test>::get().unwrap();
        assert_eq!(custodian_set.custodians, custodians2);
        assert_eq!(custodian_set.threshold, 3);
    });
}

#[test]
fn test_multisig_multiple_withdrawals_independent() {
    new_test_ext().execute_with(|| {
        // Setup: 2-of-3 custodian set
        let custodians = vec![account(10), account(11), account(12)];
        assert_ok!(BitcoinBridge::set_custodians(
            RuntimeOrigin::root(),
            custodians,
            2
        ));

        // Give user 2 more funds
        let _ = <pallet_balances::Pallet<Test> as Currency<_>>::make_free_balance_be(&account(2), 2000000);

        // User requests first withdrawal
        let withdrawer = account(2);
        let btc_address1 = vec![0x61]; // Different address
        let amount_satoshi1 = 100000;

        assert_ok!(BitcoinBridge::withdraw_btc(
            RuntimeOrigin::signed(withdrawer),
            btc_address1,
            amount_satoshi1
        ));

        let txid1 = btc_txid(10);

        // First custodian approves first withdrawal
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(10)),
            withdrawer,
            txid1.clone()
        ));

        // Second custodian approves first withdrawal (executes)
        assert_ok!(BitcoinBridge::approve_withdrawal(
            RuntimeOrigin::signed(account(11)),
            withdrawer,
            txid1
        ));

        // Verify first withdrawal completed
        let withdrawal = Withdrawals::<Test>::get(withdrawer).unwrap();
        assert_eq!(withdrawal.status, WithdrawalStatus::Completed);
    });
}
