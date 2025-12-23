use sp_core::{crypto::AccountId32, H160, H256};
use sp_io::hashing::blake2_256;

/// Deterministically derive an `H160` address from an on-chain `AccountId32`.
pub fn account_to_h160(account: &AccountId32) -> H160 {
    let hash = blake2_256(account.as_ref());
    H160::from_slice(&hash[0..20])
}

/// Convert an `H160` to a padded `H256` convenient for SCALE encoding.
pub fn h160_to_h256(address: &H160) -> H256 {
    let mut bytes = [0u8; 32];
    bytes[12..].copy_from_slice(address.as_bytes());
    H256::from(bytes)
}
