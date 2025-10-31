//! # Storage Migrations
//!
//! Storage migration utilities for pallet-treasury runtime upgrades.

use frame_support::{traits::StorageVersion, weights::Weight};

/// Current storage version
pub const CURRENT_STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

/// Migration from version 0 to version 1
pub mod v1 {
    use super::*;

    /// Migrate storage from v0 to v1
    pub fn migrate<T: crate::pallet::Config>() -> Weight {
        // Placeholder for future migrations
        Weight::zero()
    }
}
