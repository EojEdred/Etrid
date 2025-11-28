//! Transaction Batching and Compression for Lightning-Bloc Layer 2
//!
//! Provides mechanisms to batch multiple off-chain transactions and compress them
//! for efficient on-chain settlement, reducing gas costs and improving throughput.

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    string::{String, ToString},
    vec,
    vec::Vec,
    format,
};

#[cfg(not(feature = "std"))]
use core::{
    fmt,
    default::Default,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
};

#[cfg(feature = "std")]
use std::{
    collections::HashMap,
    fmt,
    vec::Vec,
    string::String,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
    default::Default,
};

/// Maximum transactions in a batch
pub const MAX_BATCH_SIZE: usize = 1000;

/// Maximum batch age in seconds before forced submission
pub const MAX_BATCH_AGE: u64 = 300; // 5 minutes

/// Compression ratio threshold for efficiency (1-100, higher = more compressed)
pub const MIN_COMPRESSION_RATIO: u8 = 20; // 20% size reduction minimum

/// Off-chain transaction record
#[derive(Clone, Debug, PartialEq)]
pub struct OffChainTransaction {
    pub tx_id: String,
    pub channel_id: String,
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub nonce: u64,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

impl OffChainTransaction {
    /// Create new off-chain transaction
    pub fn new(
        channel_id: String,
        from: String,
        to: String,
        amount: u128,
        nonce: u64,
        signature: Vec<u8>,
        timestamp: u64,
    ) -> Self {
        let tx_id = format!("tx_{}_{}_{}_{}", channel_id, from, nonce, timestamp);
        Self {
            tx_id,
            channel_id,
            from,
            to,
            amount,
            nonce,
            signature,
            timestamp,
        }
    }

    /// Serialize transaction to bytes (for batching)
    pub fn serialize(&self) -> Vec<u8> {
        // Simplified serialization - in production use proper encoding
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.channel_id.as_bytes());
        bytes.extend_from_slice(self.from.as_bytes());
        bytes.extend_from_slice(self.to.as_bytes());
        bytes.extend_from_slice(&self.amount.to_le_bytes());
        bytes.extend_from_slice(&self.nonce.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes.extend_from_slice(&self.signature);
        bytes
    }

    /// Get transaction size in bytes
    pub fn size(&self) -> usize {
        self.serialize().len()
    }
}

/// Transaction batch for efficient settlement
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionBatch {
    pub batch_id: String,
    pub transactions: Vec<OffChainTransaction>,
    pub created_at: u64,
    pub compressed_data: Option<Vec<u8>>,
    pub merkle_root: Vec<u8>,
}

impl TransactionBatch {
    /// Create new transaction batch
    pub fn new(batch_id: String, created_at: u64) -> Self {
        Self {
            batch_id,
            transactions: Vec::new(),
            created_at,
            compressed_data: None,
            merkle_root: Vec::new(),
        }
    }

    /// Add transaction to batch
    pub fn add_transaction(&mut self, tx: OffChainTransaction) -> Result<(), BatchingError> {
        if self.transactions.len() >= MAX_BATCH_SIZE {
            return Err(BatchingError::BatchFull {
                current: self.transactions.len(),
                max: MAX_BATCH_SIZE,
            });
        }

        self.transactions.push(tx);
        Ok(())
    }

    /// Check if batch should be submitted (age or size threshold)
    pub fn should_submit(&self, current_time: u64) -> bool {
        let age = current_time.saturating_sub(self.created_at);
        age >= MAX_BATCH_AGE || self.transactions.len() >= MAX_BATCH_SIZE
    }

    /// Get uncompressed size
    pub fn uncompressed_size(&self) -> usize {
        self.transactions.iter().map(|tx| tx.size()).sum()
    }

    /// Compress batch data
    pub fn compress(&mut self) -> Result<CompressionResult, BatchingError> {
        if self.transactions.is_empty() {
            return Err(BatchingError::EmptyBatch);
        }

        // Serialize all transactions
        let mut uncompressed = Vec::new();
        for tx in &self.transactions {
            uncompressed.extend_from_slice(&tx.serialize());
        }

        let original_size = uncompressed.len();

        // Simple compression: Remove redundant data
        // In production: use proper compression algorithms (zstd, lz4, etc.)
        let compressed = self.simple_compress(&uncompressed);
        let compressed_size = compressed.len();

        // Calculate compression ratio
        let ratio = if original_size > 0 {
            ((original_size - compressed_size) * 100) / original_size
        } else {
            0
        };

        // Calculate merkle root for verification
        self.merkle_root = self.calculate_merkle_root();
        self.compressed_data = Some(compressed);

        Ok(CompressionResult {
            original_size,
            compressed_size,
            ratio: ratio as u8,
            merkle_root: self.merkle_root.clone(),
        })
    }

    /// Simple compression algorithm (placeholder)
    fn simple_compress(&self, data: &[u8]) -> Vec<u8> {
        // In production: implement proper compression
        // For now, just remove trailing zeros as an example
        let mut result = data.to_vec();
        while result.last() == Some(&0) {
            result.pop();
        }
        result
    }

    /// Calculate merkle root of transactions
    fn calculate_merkle_root(&self) -> Vec<u8> {
        // Simplified merkle tree - in production use proper cryptographic hash
        let mut hashes: Vec<Vec<u8>> = self.transactions
            .iter()
            .map(|tx| tx.tx_id.as_bytes().to_vec())
            .collect();

        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let mut combined = chunk[0].clone();
                if chunk.len() > 1 {
                    combined.extend_from_slice(&chunk[1]);
                }
                next_level.push(combined);
            }
            hashes = next_level;
        }

        hashes.first().cloned().unwrap_or_default()
    }

    /// Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Get batch age in seconds
    pub fn age(&self, current_time: u64) -> u64 {
        current_time.saturating_sub(self.created_at)
    }
}

/// Compression result
#[derive(Clone, Debug, PartialEq)]
pub struct CompressionResult {
    pub original_size: usize,
    pub compressed_size: usize,
    pub ratio: u8, // Percentage reduction
    pub merkle_root: Vec<u8>,
}

impl CompressionResult {
    /// Check if compression was effective
    pub fn is_effective(&self) -> bool {
        self.ratio >= MIN_COMPRESSION_RATIO
    }
}

/// Batch settlement record
#[derive(Clone, Debug, PartialEq)]
pub struct BatchSettlement {
    pub settlement_id: String,
    pub batch_id: String,
    pub transaction_count: usize,
    pub merkle_root: Vec<u8>,
    pub settled_at: u64,
    pub on_chain_tx_hash: Vec<u8>,
}

impl BatchSettlement {
    pub fn new(
        batch_id: String,
        transaction_count: usize,
        merkle_root: Vec<u8>,
        settled_at: u64,
        on_chain_tx_hash: Vec<u8>,
    ) -> Self {
        let settlement_id = format!("settlement_{}_{}", batch_id, settled_at);
        Self {
            settlement_id,
            batch_id,
            transaction_count,
            merkle_root,
            settled_at,
            on_chain_tx_hash,
        }
    }
}

/// Transaction batching manager
pub struct BatchingManager {
    active_batches: HashMap<String, TransactionBatch>,
    settled_batches: HashMap<String, BatchSettlement>,
    next_batch_id: u64,
}

impl BatchingManager {
    /// Create new batching manager
    pub fn new() -> Self {
        Self {
            active_batches: HashMap::new(),
            settled_batches: HashMap::new(),
            next_batch_id: 1,
        }
    }

    /// Create new batch
    pub fn create_batch(&mut self, timestamp: u64) -> String {
        let batch_id = format!("batch_{}", self.next_batch_id);
        self.next_batch_id += 1;

        let batch = TransactionBatch::new(batch_id.clone(), timestamp);
        self.active_batches.insert(batch_id.clone(), batch);

        batch_id
    }

    /// Add transaction to active batch
    pub fn add_to_batch(
        &mut self,
        batch_id: &str,
        tx: OffChainTransaction,
    ) -> Result<(), BatchingError> {
        let batch = self.active_batches
            .get_mut(batch_id)
            .ok_or_else(|| BatchingError::BatchNotFound(batch_id.to_string()))?;

        batch.add_transaction(tx)
    }

    /// Get or create active batch for auto-batching
    pub fn get_or_create_active_batch(&mut self, timestamp: u64) -> String {
        // Find an active batch that's not full
        for (id, batch) in &self.active_batches {
            if batch.transactions.len() < MAX_BATCH_SIZE {
                return id.clone();
            }
        }

        // Create new batch if none available
        self.create_batch(timestamp)
    }

    /// Compress and prepare batch for settlement
    pub fn prepare_batch(&mut self, batch_id: &str) -> Result<CompressionResult, BatchingError> {
        let batch = self.active_batches
            .get_mut(batch_id)
            .ok_or_else(|| BatchingError::BatchNotFound(batch_id.to_string()))?;

        batch.compress()
    }

    /// Settle batch on-chain
    pub fn settle_batch(
        &mut self,
        batch_id: &str,
        on_chain_tx_hash: Vec<u8>,
        timestamp: u64,
    ) -> Result<String, BatchingError> {
        // Remove from active batches
        let batch = self.active_batches
            .remove(batch_id)
            .ok_or_else(|| BatchingError::BatchNotFound(batch_id.to_string()))?;

        if batch.compressed_data.is_none() {
            return Err(BatchingError::BatchNotCompressed);
        }

        // Create settlement record
        let settlement = BatchSettlement::new(
            batch_id.to_string(),
            batch.transaction_count(),
            batch.merkle_root.clone(),
            timestamp,
            on_chain_tx_hash,
        );

        let settlement_id = settlement.settlement_id.clone();
        self.settled_batches.insert(batch_id.to_string(), settlement);

        Ok(settlement_id)
    }

    /// Get batches ready for submission
    pub fn get_ready_batches(&self, current_time: u64) -> Vec<String> {
        self.active_batches
            .iter()
            .filter(|(_, batch)| batch.should_submit(current_time))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get active batch
    pub fn get_batch(&self, batch_id: &str) -> Option<&TransactionBatch> {
        self.active_batches.get(batch_id)
    }

    /// Get settlement record
    pub fn get_settlement(&self, batch_id: &str) -> Option<&BatchSettlement> {
        self.settled_batches.get(batch_id)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> BatchingStatistics {
        let active_batches = self.active_batches.len();
        let settled_batches = self.settled_batches.len();
        let pending_transactions: usize = self.active_batches
            .values()
            .map(|b| b.transaction_count())
            .sum();
        let total_settled_transactions: usize = self.settled_batches
            .values()
            .map(|s| s.transaction_count)
            .sum();

        BatchingStatistics {
            active_batches,
            settled_batches,
            pending_transactions,
            total_settled_transactions,
        }
    }
}

impl Default for BatchingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Batching statistics
#[derive(Clone, Debug, PartialEq)]
pub struct BatchingStatistics {
    pub active_batches: usize,
    pub settled_batches: usize,
    pub pending_transactions: usize,
    pub total_settled_transactions: usize,
}

/// Batching errors
#[derive(Clone, Debug, PartialEq)]
pub enum BatchingError {
    BatchFull { current: usize, max: usize },
    BatchNotFound(String),
    EmptyBatch,
    BatchNotCompressed,
    CompressionFailed(String),
    InvalidBatchData(String),
}

impl fmt::Display for BatchingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BatchingError::BatchFull { current, max } => {
                write!(f, "Batch full: {} >= {}", current, max)
            }
            BatchingError::BatchNotFound(id) => write!(f, "Batch not found: {}", id),
            BatchingError::EmptyBatch => write!(f, "Empty batch"),
            BatchingError::BatchNotCompressed => write!(f, "Batch not compressed"),
            BatchingError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
            BatchingError::InvalidBatchData(msg) => write!(f, "Invalid batch data: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tx(nonce: u64, timestamp: u64) -> OffChainTransaction {
        OffChainTransaction::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            nonce,
            vec![1, 2, 3, 4],
            timestamp,
        )
    }

    #[test]
    fn test_offchain_transaction_creation() {
        let tx = create_test_tx(1, 1000);
        assert_eq!(tx.channel_id, "ch1");
        assert_eq!(tx.amount, 100);
    }

    #[test]
    fn test_transaction_serialization() {
        let tx = create_test_tx(1, 1000);
        let bytes = tx.serialize();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_batch_creation() {
        let batch = TransactionBatch::new("batch_1".to_string(), 1000);
        assert_eq!(batch.batch_id, "batch_1");
        assert_eq!(batch.transaction_count(), 0);
    }

    #[test]
    fn test_add_transaction_to_batch() {
        let mut batch = TransactionBatch::new("batch_1".to_string(), 1000);
        let tx = create_test_tx(1, 1000);
        assert!(batch.add_transaction(tx).is_ok());
        assert_eq!(batch.transaction_count(), 1);
    }

    #[test]
    fn test_batch_should_submit_age() {
        let batch = TransactionBatch::new("batch_1".to_string(), 1000);
        assert!(!batch.should_submit(1000 + MAX_BATCH_AGE - 1));
        assert!(batch.should_submit(1000 + MAX_BATCH_AGE + 1));
    }

    #[test]
    fn test_batch_should_submit_size() {
        let mut batch = TransactionBatch::new("batch_1".to_string(), 1000);

        // Add MAX_BATCH_SIZE transactions
        for i in 0..MAX_BATCH_SIZE {
            let tx = create_test_tx(i as u64, 1000);
            batch.add_transaction(tx).unwrap();
        }

        assert!(batch.should_submit(1001)); // Should submit due to size
    }

    #[test]
    fn test_batch_full_error() {
        let mut batch = TransactionBatch::new("batch_1".to_string(), 1000);

        // Fill the batch
        for i in 0..MAX_BATCH_SIZE {
            let tx = create_test_tx(i as u64, 1000);
            batch.add_transaction(tx).unwrap();
        }

        // Try to add one more
        let tx = create_test_tx(MAX_BATCH_SIZE as u64, 1000);
        let result = batch.add_transaction(tx);
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_compression() {
        let mut batch = TransactionBatch::new("batch_1".to_string(), 1000);

        // Add some transactions
        for i in 0..10 {
            let tx = create_test_tx(i, 1000);
            batch.add_transaction(tx).unwrap();
        }

        let result = batch.compress();
        assert!(result.is_ok());

        let compression_result = result.unwrap();
        assert!(compression_result.compressed_size <= compression_result.original_size);
    }

    #[test]
    fn test_empty_batch_compression() {
        let mut batch = TransactionBatch::new("batch_1".to_string(), 1000);
        let result = batch.compress();
        assert!(result.is_err());
    }

    #[test]
    fn test_batching_manager_create_batch() {
        let mut manager = BatchingManager::new();
        let batch_id = manager.create_batch(1000);
        assert!(manager.get_batch(&batch_id).is_some());
    }

    #[test]
    fn test_batching_manager_add_to_batch() {
        let mut manager = BatchingManager::new();
        let batch_id = manager.create_batch(1000);
        let tx = create_test_tx(1, 1000);

        let result = manager.add_to_batch(&batch_id, tx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_batching_manager_prepare_batch() {
        let mut manager = BatchingManager::new();
        let batch_id = manager.create_batch(1000);

        // Add transactions
        for i in 0..10 {
            let tx = create_test_tx(i, 1000);
            manager.add_to_batch(&batch_id, tx).unwrap();
        }

        let result = manager.prepare_batch(&batch_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_batching_manager_settle_batch() {
        let mut manager = BatchingManager::new();
        let batch_id = manager.create_batch(1000);

        // Add and compress
        for i in 0..10 {
            let tx = create_test_tx(i, 1000);
            manager.add_to_batch(&batch_id, tx).unwrap();
        }
        manager.prepare_batch(&batch_id).unwrap();

        // Settle
        let result = manager.settle_batch(&batch_id, vec![1, 2, 3, 4], 2000);
        assert!(result.is_ok());
        assert!(manager.get_settlement(&batch_id).is_some());
    }

    #[test]
    fn test_get_ready_batches() {
        let mut manager = BatchingManager::new();
        let batch_id = manager.create_batch(1000);

        // Add transaction
        let tx = create_test_tx(1, 1000);
        manager.add_to_batch(&batch_id, tx).unwrap();

        // Should not be ready yet
        assert_eq!(manager.get_ready_batches(1000 + MAX_BATCH_AGE - 1).len(), 0);

        // Should be ready after MAX_BATCH_AGE
        assert_eq!(manager.get_ready_batches(1000 + MAX_BATCH_AGE + 1).len(), 1);
    }

    #[test]
    fn test_batching_statistics() {
        let mut manager = BatchingManager::new();
        let batch_id = manager.create_batch(1000);

        for i in 0..5 {
            let tx = create_test_tx(i, 1000);
            manager.add_to_batch(&batch_id, tx).unwrap();
        }

        let stats = manager.get_statistics();
        assert_eq!(stats.active_batches, 1);
        assert_eq!(stats.pending_transactions, 5);
    }

    #[test]
    fn test_get_or_create_active_batch() {
        let mut manager = BatchingManager::new();

        // First call creates a batch
        let batch_id_1 = manager.get_or_create_active_batch(1000);

        // Second call returns the same batch
        let batch_id_2 = manager.get_or_create_active_batch(1000);
        assert_eq!(batch_id_1, batch_id_2);
    }
}
