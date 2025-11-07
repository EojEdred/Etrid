//! Common Bridge Protocol Components for Ëtrid
//!
//! This crate provides shared functionality for all bridge protocols in Component 05.
//!
//! ## Modules
//! - `multisig`: Multi-signature custodian functionality for bridge security
//! - `treasury`: Treasury integration traits for cross-chain fee routing

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod multisig;
pub mod treasury;
pub mod oracle_adapter;

// Re-export commonly used types
pub use multisig::{MultiSigCustodian, PendingApproval};
pub use treasury::TreasuryInterface;
pub use oracle_adapter::{PriceOracle, ExchangeRate, OracleAggregator, StaticRateOracle, OracleError};
