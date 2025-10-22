//! Common Bridge Protocol Components for Ëtrid
//!
//! This crate provides shared functionality for all bridge protocols in Component 05.
//!
//! ## Modules
//! - `multisig`: Multi-signature custodian functionality for bridge security

#![cfg_attr(not(feature = "std"), no_std)]

pub mod multisig;

// Re-export commonly used types
pub use multisig::{MultiSigCustodian, PendingApproval};
