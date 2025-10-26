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
}
