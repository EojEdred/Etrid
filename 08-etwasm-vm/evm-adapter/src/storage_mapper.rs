use alloc::collections::BTreeMap;
use sp_core::{crypto::AccountId32, H160};

use crate::error::{AdapterError, AdapterResult};

/// Trait describing how we map an EVM address to a Substrate account and back.
pub trait StorageMapper {
    fn evm_to_account(&self, address: &H160) -> AdapterResult<AccountId32>;
    fn account_to_evm(&self, account: &AccountId32) -> AdapterResult<H160>;
}

/// In-memory bidirectional mapping used by the adapter. In production we will
/// replace this with persistent storage inside the pallet, but having the
/// helper here keeps the adapter stateless and easy to test.
#[derive(Default, Clone)]
pub struct AccountMapping {
    forward: BTreeMap<H160, AccountId32>,
    reverse: BTreeMap<AccountId32, H160>,
}

impl AccountMapping {
    pub fn insert(&mut self, address: H160, account: AccountId32) {
        self.reverse.insert(account.clone(), address);
        self.forward.insert(address, account);
    }
}

impl StorageMapper for AccountMapping {
    fn evm_to_account(&self, address: &H160) -> AdapterResult<AccountId32> {
        self.forward
            .get(address)
            .cloned()
            .ok_or(AdapterError::StorageMappingFailed)
    }

    fn account_to_evm(&self, account: &AccountId32) -> AdapterResult<H160> {
        self.reverse
            .get(account)
            .cloned()
            .ok_or(AdapterError::StorageMappingFailed)
    }
}
