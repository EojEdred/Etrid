//! Common Bridge Protocol Components for Ëtrid
//!
//! This crate provides shared functionality for all bridge protocols in Component 05.
//!
//! ## Modules
//! - `multisig`: Multi-signature custodian functionality for bridge security
//! - `treasury`: Treasury integration traits for cross-chain fee routing

#![cfg_attr(not(feature = "std"), no_std)]

pub mod multisig;
pub mod treasury;

// Re-export commonly used types
pub use multisig::{MultiSigCustodian, PendingApproval};
pub use treasury::TreasuryInterface;
