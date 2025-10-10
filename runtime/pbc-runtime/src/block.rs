use chrono::Utc;
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone)]
pub struct Block {
    pub height: u64,
    pub timestamp: i64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(height: u64, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut hasher = Keccak256::new();
        hasher.update(format!("{height}{timestamp}{data}{prev_hash}"));
        let hash = format!("{:x}", hasher.finalize());
        Block {
            height,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }
}
