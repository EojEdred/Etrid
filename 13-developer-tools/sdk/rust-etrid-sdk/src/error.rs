//! Error types for the SDK

use thiserror::Error;

/// SDK Result type
pub type Result<T> = std::result::Result<T, Error>;

/// SDK Error types
#[derive(Error, Debug)]
pub enum Error {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// RPC error
    #[error("RPC error: {0}")]
    Rpc(String),

    /// Cryptographic error
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Lightning network error
    #[error("Lightning error: {0}")]
    Lightning(String),

    /// Distribution payment error
    #[error("Distribution error: {0}")]
    Distribution(String),

    /// ETWASM VM error
    #[error("ETWASM error: {0}")]
    Etwasm(String),

    /// AI DID error
    #[error("AI DID error: {0}")]
    AiDid(String),

    /// Bridge error
    #[error("Bridge error: {0}")]
    Bridge(String),

    /// Oracle error
    #[error("Oracle error: {0}")]
    Oracle(String),

    /// Reserve vault error
    #[error("Vault error: {0}")]
    Vault(String),

    /// Staking error
    #[error("Staking error: {0}")]
    Staking(String),

    /// Governance error
    #[error("Governance error: {0}")]
    Governance(String),

    /// Account error
    #[error("Account error: {0}")]
    Account(String),

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Insufficient balance
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u128, available: u128 },

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Already exists error
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Unauthorized error
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}
