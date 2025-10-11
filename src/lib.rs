//! Etrid Runtime
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet_accounts;
pub use pallet_governance;
pub use pallet_consensus;
pub use pallet_multichain;
pub use pallet_vm;

// Re-export pallet items
pub use pallet_accounts as accounts;
pub use pallet_governance as governance;
pub use pallet_consensus as consensus;
pub use pallet_multichain as multichain;
pub use pallet_vm as vm;
