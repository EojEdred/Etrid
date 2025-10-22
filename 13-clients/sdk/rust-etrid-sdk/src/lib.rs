//! # Ëtrid SDK for Rust
//!
//! This library provides a Rust interface to interact with the Ëtrid Protocol blockchain.
//!
//! ## Features
//!
//! - Account management and key generation
//! - RPC client for blockchain queries
//! - Transaction building and signing
//! - Type-safe Substrate integration
//!
//! ## Example
//!
//! ```no_run
//! use etrid_sdk::{Client, Account};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Ëtrid node
//!     let client = Client::new("ws://localhost:9944").await?;
//!
//!     // Create account from mnemonic
//!     let account = Account::from_mnemonic("word1 word2 ... word12")?;
//!
//!     // Query balance
//!     let balance = client.get_balance(&account.address()).await?;
//!     println!("Balance: {} ETR", balance);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod account;
pub mod types;
pub mod error;

pub use client::Client;
pub use account::Account;
pub use types::*;
pub use error::{Error, Result};

/// SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
