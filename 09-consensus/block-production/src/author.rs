//! # Block Authoring
//!
//! This module handles the actual creation of blocks, including
//! transaction selection, block building, and signing.

use alloc::vec::Vec;
use sp_core::hashing::blake2_256;

use crate::{
    Block, BlockBody, BlockHeader, BlockNumber, BlockProposal, BlockProductionError,
    BlockProductionResult, BlockType, Hash, TransactionPriority, TransactionStrategy, ValidatorId,
    MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK,
};

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSACTION POOL (SIMPLIFIED)
// ═══════════════════════════════════════════════════════════════════════════════

/// Transaction with priority
#[derive(Debug, Clone)]
pub struct PrioritizedTransaction {
    /// Transaction data (encoded extrinsic)
    pub data: Vec<u8>,
    
    /// Priority score
    pub priority: TransactionPriority,
    
    /// Timestamp when added
    pub timestamp: u64,
    
    /// Fee amount
    pub fee: u64,
}

/// Simple transaction pool
#[derive(Debug, Clone)]
pub struct TransactionPool {
    /// Pending transactions
    transactions: Vec<PrioritizedTransaction>,
    
    /// Selection strategy
    strategy: TransactionStrategy,
}

impl TransactionPool {
    /// Create a new transaction pool
    pub fn new(strategy: TransactionStrategy) -> Self {
        Self {
            transactions: Vec::new(),
            strategy,
        }
    }

    /// Add a transaction to the pool
    pub fn add(&mut self, tx: PrioritizedTransaction) {
        self.transactions.push(tx);
    }

    /// Get transactions for block (sorted by strategy)
    pub fn get_transactions(&mut self, max_count: usize, max_size: usize) -> Vec<Vec<u8>> {
        // Sort by strategy
        self.sort_by_strategy();
        
        let mut selected = Vec::new();
        let mut total_size = 0;
        
        for tx in &self.transactions {
            if selected.len() >= max_count {
                break;
            }
            
            if total_size + tx.data.len() > max_size {
                break;
            }
            
            selected.push(tx.data.clone());
            total_size += tx.data.len();
        }
        
        // Remove selected transactions
        self.transactions.retain(|tx| !selected.contains(&tx.data));
        
        selected
    }

    /// Sort transactions by strategy
    fn sort_by_strategy(&mut self) {
        match self.strategy {
            TransactionStrategy::HighestPriority => {
                self.transactions.sort_by(|a, b| b.priority.cmp(&a.priority));
            }
            TransactionStrategy::OldestFirst => {
                self.transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            }
            TransactionStrategy::HighestFee => {
                self.transactions.sort_by(|a, b| b.fee.cmp(&a.fee));
            }
        }
    }

    /// Get pool size
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Check if pool is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Clear all transactions
    pub fn clear(&mut self) {
        self.transactions.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Builds blocks from transactions
#[derive(Debug)]
pub struct BlockBuilder {
    /// Current block number
    current_number: BlockNumber,
    
    /// Parent block hash
    parent_hash: Hash,
    
    /// Transaction pool
    tx_pool: TransactionPool,
    
    /// Proposer validator ID
    proposer: ValidatorId,
    
    /// PPFA index
    ppfa_index: u32,
    
    /// Current epoch
    epoch: u32,
}

impl BlockBuilder {
    /// Create a new block builder
    pub fn new(
        parent_number: BlockNumber,
        parent_hash: Hash,
        proposer: ValidatorId,
        ppfa_index: u32,
        epoch: u32,
    ) -> Self {
        Self {
            current_number: parent_number + 1,
            parent_hash,
            tx_pool: TransactionPool::new(TransactionStrategy::HighestPriority),
            proposer,
            ppfa_index,
            epoch,
        }
    }

    /// Add transaction to pool
    pub fn add_transaction(&mut self, tx: PrioritizedTransaction) {
        self.tx_pool.add(tx);
    }

    /// Build a Queen block (main block)
    pub fn build_queen(&mut self, timestamp: u64) -> BlockProductionResult<Block> {
        self.build_block(BlockType::Queen, timestamp)
    }

    /// Build an Ant block (secondary block)
    pub fn build_ant(&mut self, timestamp: u64) -> BlockProductionResult<Block> {
        self.build_block(BlockType::Ant, timestamp)
    }

    /// Build a block
    fn build_block(&mut self, block_type: BlockType, timestamp: u64) -> BlockProductionResult<Block> {
        // Get transactions from pool
        let transactions = self
            .tx_pool
            .get_transactions(MAX_TRANSACTIONS_PER_BLOCK, MAX_BLOCK_SIZE);

        if transactions.is_empty() && block_type == BlockType::Queen {
            // Queen blocks can be empty (no transactions)
            // but we log it as a warning in production
        }

        // Create block body
        let body = BlockBody { transactions };

        // Calculate roots
        let extrinsics_root = self.calculate_extrinsics_root(&body);
        let state_root = Hash::default(); // Would be calculated from state

        // Create header
        let header = BlockHeader {
            number: self.current_number,
            parent_hash: self.parent_hash,
            state_root,
            extrinsics_root,
            block_type,
            proposer: self.proposer.clone(),
            ppfa_index: self.ppfa_index,
            timestamp,
            epoch: self.epoch,
        };

        Ok(Block::new(header, body))
    }

    /// Calculate extrinsics root (Merkle root of transactions)
    fn calculate_extrinsics_root(&self, body: &BlockBody) -> Hash {
        if body.transactions.is_empty() {
            return Hash::default();
        }

        // Simple hash of all transactions concatenated
        let mut data = Vec::new();
        for tx in &body.transactions {
            data.extend_from_slice(tx);
        }

        Hash::from(blake2_256(&data))
    }

    /// Get transaction pool
    pub fn transaction_pool(&self) -> &TransactionPool {
        &self.tx_pool
    }

    /// Get mutable transaction pool
    pub fn transaction_pool_mut(&mut self) -> &mut TransactionPool {
        &mut self.tx_pool
    }

    /// Update parent (for next block)
    pub fn update_parent(&mut self, parent_number: BlockNumber, parent_hash: Hash) {
        self.current_number = parent_number + 1;
        self.parent_hash = parent_hash;
    }

    /// Update proposer
    pub fn update_proposer(&mut self, proposer: ValidatorId, ppfa_index: u32) {
        self.proposer = proposer;
        self.ppfa_index = ppfa_index;
    }

    /// Update epoch
    pub fn update_epoch(&mut self, epoch: u32) {
        self.epoch = epoch;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK SIGNER
// ═══════════════════════════════════════════════════════════════════════════════

/// Signs blocks for proposals
pub struct BlockSigner {
    /// Validator ID (public key)
    validator: ValidatorId,
}

impl BlockSigner {
    /// Create a new block signer
    pub fn new(validator: ValidatorId) -> Self {
        Self { validator }
    }

    /// Sign a block (creates proposal)
    pub fn sign_block(&self, block: Block) -> BlockProductionResult<BlockProposal> {
        // In production, this would use the validator's private key
        // For now, we create a dummy signature
        let signature = self.create_signature(&block);
        
        Ok(BlockProposal::new(block, signature))
    }

    /// Create signature (simplified - would use real crypto in production)
    fn create_signature(&self, block: &Block) -> Vec<u8> {
        let block_hash = block.hash();
        
        // Dummy signature: hash(validator_id + block_hash)
        let mut data = self.validator.0.to_vec();
        data.extend_from_slice(&block_hash.0);
        
        blake2_256(&data).to_vec()
    }

    /// Verify a signature
    pub fn verify_signature(&self, proposal: &BlockProposal) -> bool {
        let expected = self.create_signature(&proposal.block);
        expected == proposal.signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_transaction(priority: u64) -> PrioritizedTransaction {
        PrioritizedTransaction {
            data: vec![1, 2, 3],
            priority,
            timestamp: 0,
            fee: priority,
        }
    }

    #[test]
    fn test_transaction_pool_add() {
        let mut pool = TransactionPool::new(TransactionStrategy::HighestPriority);
        
        pool.add(create_test_transaction(100));
        pool.add(create_test_transaction(200));
        
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_transaction_pool_get_by_priority() {
        let mut pool = TransactionPool::new(TransactionStrategy::HighestPriority);
        
        pool.add(create_test_transaction(100));
        pool.add(create_test_transaction(300));
        pool.add(create_test_transaction(200));
        
        let txs = pool.get_transactions(2, 1000);
        assert_eq!(txs.len(), 2);
        
        // Should get highest priority first (300, 200)
        assert_eq!(pool.len(), 1); // One left (100)
    }

    #[test]
    fn test_transaction_pool_max_size() {
        let mut pool = TransactionPool::new(TransactionStrategy::HighestPriority);
        
        // Add transactions totaling 9 bytes
        pool.add(PrioritizedTransaction {
            data: vec![1; 3],
            priority: 300,
            timestamp: 0,
            fee: 0,
        });
        pool.add(PrioritizedTransaction {
            data: vec![1; 3],
            priority: 200,
            timestamp: 0,
            fee: 0,
        });
        pool.add(PrioritizedTransaction {
            data: vec![1; 3],
            priority: 100,
            timestamp: 0,
            fee: 0,
        });
        
        // Get with max size of 7 bytes (should get 2 txs)
        let txs = pool.get_transactions(10, 7);
        assert_eq!(txs.len(), 2);
    }

    #[test]
    fn test_block_builder_creation() {
        let parent_hash = Hash::default();
        let proposer = ValidatorId::from([1u8; 32]);
        
        let builder = BlockBuilder::new(0, parent_hash, proposer, 0, 1);
        assert_eq!(builder.current_number, 1);
    }

    #[test]
    fn test_build_queen_block() {
        let parent_hash = Hash::default();
        let proposer = ValidatorId::from([1u8; 32]);
        
        let mut builder = BlockBuilder::new(0, parent_hash, proposer, 0, 1);
        builder.add_transaction(create_test_transaction(100));
        
        let block = builder.build_queen(1000).unwrap();
        
        assert_eq!(block.number(), 1);
        assert!(block.is_queen());
        assert_eq!(block.transaction_count(), 1);
    }

    #[test]
    fn test_build_ant_block() {
        let parent_hash = Hash::default();
        let proposer = ValidatorId::from([1u8; 32]);
        
        let mut builder = BlockBuilder::new(0, parent_hash, proposer, 0, 1);
        builder.add_transaction(create_test_transaction(100));
        
        let block = builder.build_ant(1000).unwrap();
        
        assert_eq!(block.number(), 1);
        assert!(block.is_ant());
    }

    #[test]
    fn test_extrinsics_root() {
        let parent_hash = Hash::default();
        let proposer = ValidatorId::from([1u8; 32]);
        
        let mut builder1 = BlockBuilder::new(0, parent_hash, proposer.clone(), 0, 1);
        builder1.add_transaction(create_test_transaction(100));
        let block1 = builder1.build_queen(1000).unwrap();
        
        let mut builder2 = BlockBuilder::new(0, parent_hash, proposer, 0, 1);
        builder2.add_transaction(create_test_transaction(100));
        let block2 = builder2.build_queen(1000).unwrap();
        
        // Same transactions should give same root
        assert_eq!(
            block1.header.extrinsics_root,
            block2.header.extrinsics_root
        );
    }

    #[test]
    fn test_block_signer() {
        let validator = ValidatorId::from([1u8; 32]);
        let signer = BlockSigner::new(validator.clone());
        
        let parent_hash = Hash::default();
        let mut builder = BlockBuilder::new(0, parent_hash, validator, 0, 1);
        let block = builder.build_queen(1000).unwrap();
        
        let proposal = signer.sign_block(block).unwrap();
        assert!(signer.verify_signature(&proposal));
    }

    #[test]
    fn test_update_parent() {
        let parent_hash = Hash::default();
        let proposer = ValidatorId::from([1u8; 32]);
        
        let mut builder = BlockBuilder::new(0, parent_hash, proposer, 0, 1);
        assert_eq!(builder.current_number, 1);
        
        let new_hash = Hash::from([1u8; 32]);
        builder.update_parent(5, new_hash);
        
        assert_eq!(builder.current_number, 6);
        assert_eq!(builder.parent_hash, new_hash);
    }

    #[test]
    fn test_empty_block() {
        let parent_hash = Hash::default();
        let proposer = ValidatorId::from([1u8; 32]);
        
        let mut builder = BlockBuilder::new(0, parent_hash, proposer, 0, 1);
        let block = builder.build_queen(1000).unwrap();
        
        assert_eq!(block.transaction_count(), 0);
        assert_eq!(block.header.extrinsics_root, Hash::default());
    }
}
