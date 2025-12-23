//! # Primearc Core Chain Runtime Migrations
//!
//! This module contains all runtime upgrade migrations for Primearc Core Chain.
//!
//! ## Migration History
//!
//! - **v106**: Fixed GRANDPA committee formation (10 validators)
//! - **v107**: Transitioned to ASF primary finality (GRANDPA fallback)
//! - **v109**: Sudo key recovery + 9 Director attesters registration (5-of-9 threshold)
//!
//! ## Adding New Migrations
//!
//! 1. Create new module: `vXXX.rs`
//! 2. Add public export here
//! 3. Update `Executive` in `lib.rs` to use new migration
//! 4. Bump `spec_version` in `RuntimeVersion`

pub mod v107;
pub mod v109;

pub use v107::MigrateToAsfPrimary;
pub use v109::MigrateSudoAndEdscMinter;
