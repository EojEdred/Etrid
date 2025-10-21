#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Encode, Decode};
use scale_info::TypeInfo;

pub trait BridgeTrait {
    type AccountId;
    type Balance;
    fn total_locked() -> u64;
    fn total_minted() -> Self::Balance;
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
pub struct BridgeStats<Balance> {
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub total_locked: u64,
    pub total_minted: Balance,
}
