//! Common test utilities and helpers for bridge integration tests

use frame_support::{
    assert_ok, assert_noop,
    traits::{Currency, OnFinalize, OnInitialize},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

/// Test account IDs
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;
pub const BRIDGE_AUTHORITY: u64 = 100;

/// Test BTC transaction hash
pub fn btc_tx_hash() -> H256 {
    H256::from_low_u64_be(12345)
}

/// Test ETH transaction hash
pub fn eth_tx_hash() -> H256 {
    H256::from_low_u64_be(67890)
}

/// Helper to advance blocks
pub fn run_to_block<T>(n: u32)
where
    T: frame_system::Config,
{
    while frame_system::Pallet::<T>::block_number() < n.into() {
        let block = frame_system::Pallet::<T>::block_number();
        frame_system::Pallet::<T>::on_finalize(block);
        frame_system::Pallet::<T>::set_block_number(block + 1u32.into());
        frame_system::Pallet::<T>::on_initialize(block + 1u32.into());
    }
}

/// Helper to get account balance
pub fn balance_of<T>(account: T::AccountId) -> T::Balance
where
    T: pallet_balances::Config,
{
    pallet_balances::Pallet::<T>::free_balance(&account)
}

/// Test deposit scenario helper
pub struct DepositScenario {
    pub depositor: u64,
    pub amount: u64,
    pub tx_hash: H256,
    pub confirmations: u32,
}

impl Default for DepositScenario {
    fn default() -> Self {
        Self {
            depositor: ALICE,
            amount: 1_000_000,
            tx_hash: btc_tx_hash(),
            confirmations: 6,
        }
    }
}

/// Test withdrawal scenario helper
pub struct WithdrawalScenario {
    pub withdrawer: u64,
    pub amount: u64,
    pub destination: Vec<u8>,
}

impl Default for WithdrawalScenario {
    fn default() -> Self {
        Self {
            withdrawer: ALICE,
            amount: 500_000,
            destination: b"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_vec(),
        }
    }
}

#[macro_export]
macro_rules! assert_bridge_event {
    ($event:expr) => {
        assert!(frame_system::Pallet::<Test>::events()
            .iter()
            .any(|record| record.event == $event));
    };
}

#[macro_export]
macro_rules! last_bridge_event {
    () => {
        frame_system::Pallet::<Test>::events()
            .last()
            .expect("Expected at least one event")
            .event
            .clone()
    };
}
