//! # Block Validation
//!
//! This module validates blocks before they are proposed to the network.

use crate::{
    Block, BlockProductionError, BlockProductionResult, BlockType, Hash, MAX_BLOCK_SIZE,
    MAX_TRANSACTIONS_PER_BLOCK,
};

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK VALIDATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Validates blocks before proposal
#[derive(Debug, Clone)]
pub struct BlockValidator {
    /// Maximum block size (bytes)
    max_block_size: usize,
    
    /// Maximum transactions per block
    max_transactions: usize,
    
    /// Strict validation mode
    strict: bool,
}

impl BlockValidator {
    /// Create a new block validator
    pub fn new(max_block_size: usize, max_transactions: usize, strict: bool) -> Self {
        Self {
            max_block_size,
            max_transactions,
            strict,
        }
    }

    /// Validate a block completely
    pub fn validate_block(&self, block: &Block) -> BlockProductionResult<()> {
        self.validate_header(block)?;
        self.validate_body(block)?;
        self.validate_size(block)?;
        self.validate_consistency(block)?;
        
        Ok(())
    }

    /// Validate block header
    fn validate_header(&self, block: &Block) -> BlockProductionResult<()> {
        let header = &block.header;
        
        // Check block number is positive
        if header.number == 0 {
            return Err(BlockProductionError::ValidationFailed(
                "Block number must be > 0",
            ));
        }

        // Check parent hash is not zero (except genesis)
        if header.number > 1 && header.parent_hash == Hash::default() {
            return Err(BlockProductionError::ValidationFailed(
                "Invalid parent hash",
            ));
        }

        // Check timestamp is reasonable (not in far future)
        // In production, would check against current time
        if header.timestamp == 0 && self.strict {
            return Err(BlockProductionError::ValidationFailed(
                "Invalid timestamp",
            ));
        }

        Ok(())
    }

    /// Validate block body
    fn validate_body(&self, block: &Block) -> BlockProductionResult<()> {
        let body = &block.body;
        
        // Check transaction count
        if body.transactions.len() > self.max_transactions {
            return Err(BlockProductionError::ValidationFailed(
                "Too many transactions",
            ));
        }

        // Validate each transaction
        for (i, tx) in body.transactions.iter().enumerate() {
            if tx.is_empty() {
                return Err(BlockProductionError::ValidationFailed(
                    "Empty transaction",
                ));
            }

            // Check individual transaction size (max 1MB)
            if tx.len() > 1_000_000 {
                return Err(BlockProductionError::ValidationFailed(
                    "Transaction too large",
                ));
            }
        }

        Ok(())
    }

    /// Validate block size
    fn validate_size(&self, block: &Block) -> BlockProductionResult<()> {
        use codec::Encode;
        
        let encoded_size = block.encode().len();
        
        if encoded_size > self.max_block_size {
            return Err(BlockProductionError::ValidationFailed(
                "Block too large",
            ));
        }

        Ok(())
    }

    /// Validate block consistency
    fn validate_consistency(&self, block: &Block) -> BlockProductionResult<()> {
        // Verify extrinsics root matches body
        let calculated_root = self.calculate_extrinsics_root(block);
        
        if self.strict && calculated_root != block.header.extrinsics_root {
            return Err(BlockProductionError::ValidationFailed(
                "Extrinsics root mismatch",
            ));
        }

        // Verify block type consistency
        match block.header.block_type {
            BlockType::Queen => {
                // Queen blocks should be from PPFA validators
                // Would check committee membership in production
            }
            BlockType::Ant => {
                // Ant blocks can be from any validator
                // but should have timeout evidence
            }
        }

        Ok(())
    }

    /// Calculate extrinsics root
    fn calculate_extrinsics_root(&self, block: &Block) -> Hash {
        use sp_core::hashing::blake2_256;
        
        if block.body.transactions.is_empty() {
            return Hash::default();
        }

        let mut data = Vec::new();
        for tx in &block.body.transactions {
            data.extend_from_slice(tx);
        }

        Hash::from(blake2_256(&data))
    }
}

impl Default for BlockValidator {
    fn default() -> Self {
        Self::new(MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK, false)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATION RULES
// ═══════════════════════════════════════════════════════════════════════════════

/// Validation rule for blocks
pub trait ValidationRule {
    /// Check if block passes this rule
    fn validate(&self, block: &Block) -> BlockProductionResult<()>;
    
    /// Get rule name
    fn name(&self) -> &'static str;
}

/// Rule: Block number must be sequential
pub struct SequentialNumberRule {
    expected_number: u64,
}

impl SequentialNumberRule {
    pub fn new(expected_number: u64) -> Self {
        Self { expected_number }
    }
}

impl ValidationRule for SequentialNumberRule {
    fn validate(&self, block: &Block) -> BlockProductionResult<()> {
        if block.number() != self.expected_number {
            return Err(BlockProductionError::ValidationFailed(
                "Block number not sequential",
            ));
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "SequentialNumber"
    }
}

/// Rule: Block must have minimum transactions (for Ants)
pub struct MinimumTransactionsRule {
    min_count: usize,
}

impl MinimumTransactionsRule {
    pub fn new(min_count: usize) -> Self {
        Self { min_count }
    }
}

impl ValidationRule for MinimumTransactionsRule {
    fn validate(&self, block: &Block) -> BlockProductionResult<()> {
        if block.transaction_count() < self.min_count {
            return Err(BlockProductionError::ValidationFailed(
                "Insufficient transactions",
            ));
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "MinimumTransactions"
    }
}

/// Rule: Parent hash must match
pub struct ParentHashRule {
    expected_parent: Hash,
}

impl ParentHashRule {
    pub fn new(expected_parent: Hash) -> Self {
        Self { expected_parent }
    }
}

impl ValidationRule for ParentHashRule {
    fn validate(&self, block: &Block) -> BlockProductionResult<()> {
        if block.parent_hash() != self.expected_parent {
            return Err(BlockProductionError::ValidationFailed(
                "Parent hash mismatch",
            ));
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "ParentHash"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATION CHAIN
// ═══════════════════════════════════════════════════════════════════════════════

/// Chain of validation rules
pub struct ValidationChain {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl ValidationChain {
    /// Create a new validation chain
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the chain
    pub fn add_rule<R: ValidationRule + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Validate block against all rules
    pub fn validate(&self, block: &Block) -> BlockProductionResult<()> {
        for rule in &self.rules {
            rule.validate(block)?;
        }
        Ok(())
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for ValidationChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BlockBody, BlockHeader, ValidatorId};

    fn create_test_block(number: u64, tx_count: usize) -> Block {
        let mut block = Block {
            header: BlockHeader {
                number,
                parent_hash: Hash::from([1u8; 32]),
                state_root: Hash::default(),
                extrinsics_root: Hash::default(),
                block_type: BlockType::Queen,
                proposer: ValidatorId::from([1u8; 32]),
                ppfa_index: 0,
                timestamp: 1000,
                epoch: 1,
            },
            body: BlockBody {
                transactions: Vec::new(),
            },
        };

        // Add transactions
        for i in 0..tx_count {
            block.body.transactions.push(vec![i as u8]);
        }

        block
    }

    #[test]
    fn test_validator_creation() {
        let validator = BlockValidator::default();
        assert_eq!(validator.max_block_size, MAX_BLOCK_SIZE);
        assert_eq!(validator.max_transactions, MAX_TRANSACTIONS_PER_BLOCK);
    }

    #[test]
    fn test_valid_block() {
        let validator = BlockValidator::default();
        let block = create_test_block(1, 10);
        
        assert!(validator.validate_block(&block).is_ok());
    }

    #[test]
    fn test_invalid_block_number() {
        let validator = BlockValidator::new(MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK, true);
        let block = create_test_block(0, 10); // Block 0 is invalid
        
        assert!(validator.validate_header(&block).is_err());
    }

    #[test]
    fn test_too_many_transactions() {
        let validator = BlockValidator::new(MAX_BLOCK_SIZE, 5, false); // Max 5 transactions
        let block = create_test_block(1, 10); // Has 10 transactions
        
        assert!(validator.validate_body(&block).is_err());
    }

    #[test]
    fn test_empty_transaction() {
        let validator = BlockValidator::default();
        let mut block = create_test_block(1, 0);
        block.body.transactions.push(Vec::new()); // Add empty transaction
        
        assert!(validator.validate_body(&block).is_err());
    }

    #[test]
    fn test_sequential_number_rule() {
        let rule = SequentialNumberRule::new(5);
        
        let valid_block = create_test_block(5, 10);
        assert!(rule.validate(&valid_block).is_ok());
        
        let invalid_block = create_test_block(6, 10);
        assert!(rule.validate(&invalid_block).is_err());
    }

    #[test]
    fn test_minimum_transactions_rule() {
        let rule = MinimumTransactionsRule::new(5);
        
        let valid_block = create_test_block(1, 10);
        assert!(rule.validate(&valid_block).is_ok());
        
        let invalid_block = create_test_block(1, 3);
        assert!(rule.validate(&invalid_block).is_err());
    }

    #[test]
    fn test_parent_hash_rule() {
        let expected = Hash::from([1u8; 32]);
        let rule = ParentHashRule::new(expected);
        
        let valid_block = create_test_block(1, 10);
        assert!(rule.validate(&valid_block).is_ok());
        
        let mut invalid_block = create_test_block(1, 10);
        invalid_block.header.parent_hash = Hash::from([2u8; 32]);
        assert!(rule.validate(&invalid_block).is_err());
    }

    #[test]
    fn test_validation_chain() {
        let chain = ValidationChain::new()
            .add_rule(SequentialNumberRule::new(1))
            .add_rule(MinimumTransactionsRule::new(5));
        
        assert_eq!(chain.rule_count(), 2);
        
        let valid_block = create_test_block(1, 10);
        assert!(chain.validate(&valid_block).is_ok());
        
        let invalid_block = create_test_block(2, 10); // Wrong number
        assert!(chain.validate(&invalid_block).is_err());
    }

    #[test]
    fn test_validation_chain_multiple_failures() {
        let chain = ValidationChain::new()
            .add_rule(SequentialNumberRule::new(1))
            .add_rule(MinimumTransactionsRule::new(5));
        
        let block = create_test_block(2, 3); // Both rules fail
        assert!(chain.validate(&block).is_err());
    }

    #[test]
    fn test_block_size_validation() {
        let validator = BlockValidator::new(100, MAX_TRANSACTIONS_PER_BLOCK, false); // Small max size
        
        // Create block with many large transactions
        let mut block = create_test_block(1, 0);
        for _ in 0..100 {
            block.body.transactions.push(vec![0u8; 100]);
        }
        
        assert!(validator.validate_size(&block).is_err());
    }

    #[test]
    fn test_strict_mode_timestamp() {
        let strict_validator = BlockValidator::new(MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK, true);
        let non_strict_validator = BlockValidator::new(MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK, false);
        
        let mut block = create_test_block(1, 10);
        block.header.timestamp = 0;
        
        assert!(strict_validator.validate_header(&block).is_err());
        assert!(non_strict_validator.validate_header(&block).is_ok());
    }

    #[test]
    fn test_extrinsics_root_calculation() {
        let validator = BlockValidator::default();
        let block = create_test_block(1, 10);
        
        let root = validator.calculate_extrinsics_root(&block);
        assert_ne!(root, Hash::default());
    }

    #[test]
    fn test_empty_block_root() {
        let validator = BlockValidator::default();
        let block = create_test_block(1, 0); // No transactions
        
        let root = validator.calculate_extrinsics_root(&block);
        assert_eq!(root, Hash::default());
    }
}
