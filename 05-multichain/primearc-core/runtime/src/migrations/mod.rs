//! # FlareChain Runtime Migrations
//!
//! This module contains all runtime upgrade migrations for FlareChain.
//!
//! ## Migration History
//!
//! - **v106**: Fixed GRANDPA committee formation (10 validators)
//! - **v107**: Transitioned to ASF primary finality (GRANDPA fallback)
//!
//! ## Adding New Migrations
//!
//! 1. Create new module: `vXXX.rs`
//! 2. Add public export here
//! 3. Update `Executive` in `lib.rs` to use new migration
//! 4. Bump `spec_version` in `RuntimeVersion`

pub mod v107;

pub use v107::MigrateToAsfPrimary;
