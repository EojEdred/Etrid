//! Common types used throughout the SDK

use serde::{Deserialize, Serialize};

/// Account balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Free balance
    pub free: u128,
    /// Reserved balance
    pub reserved: u128,
    /// Frozen balance
    pub frozen: u128,
}

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block number
    pub number: u64,
    /// Block hash
    pub hash: String,
    /// Parent hash
    pub parent_hash: String,
    /// State root
    pub state_root: String,
    /// Extrinsics root
    pub extrinsics_root: String,
}

/// Transaction hash
pub type TxHash = String;

/// Account address (SS58 encoded)
pub type Address = String;
