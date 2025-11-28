//! # Ant Handler
//!
//! This module manages Ant blocks (secondary blocks) which are produced
//! when the primary PPFA proposer fails to produce a Queen block in time.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::{
    Block, BlockNumber, BlockProductionError, BlockProductionResult, BlockType, Hash, ValidatorId,
    MAX_ANTS_PER_SLOT,
};

// ═══════════════════════════════════════════════════════════════════════════════
// ANT BLOCK MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manages Ant block production and tracking
#[derive(Debug, Clone)]
pub struct AntManager {
    /// Ant blocks by slot
    ants_per_slot: BTreeMap<u64, Vec<AntBlock>>,
    
    /// Maximum Ants per slot
    max_ants_per_slot: u32,
    
    /// Ant production timeout (milliseconds)
    ant_timeout: u64,
}

impl AntManager {
    /// Create a new Ant manager
    pub fn new(max_ants_per_slot: u32, ant_timeout: u64) -> Self {
        Self {
            ants_per_slot: BTreeMap::new(),
            max_ants_per_slot,
            ant_timeout,
        }
    }

    /// Check if Ant production is allowed for this slot
    pub fn can_produce_ant(&self, slot: u64) -> bool {
        let count = self.ant_count_for_slot(slot);
        count < self.max_ants_per_slot
    }

    /// Get Ant count for a slot
    pub fn ant_count_for_slot(&self, slot: u64) -> u32 {
        self.ants_per_slot
            .get(&slot)
            .map(|ants| ants.len() as u32)
            .unwrap_or(0)
    }

    /// Register an Ant block
    pub fn register_ant(&mut self, slot: u64, ant: AntBlock) -> BlockProductionResult<()> {
        if !self.can_produce_ant(slot) {
            return Err(BlockProductionError::TooManyAnts);
        }

        self.ants_per_slot
            .entry(slot)
            .or_insert_with(Vec::new)
            .push(ant);

        Ok(())
    }

    /// Get all Ants for a slot
    pub fn get_ants_for_slot(&self, slot: u64) -> Vec<&AntBlock> {
        self.ants_per_slot
            .get(&slot)
            .map(|ants| ants.iter().collect())
            .unwrap_or_default()
    }

    /// Check if Queen block exists for slot
    pub fn has_queen_for_slot(&self, slot: u64) -> bool {
        // Would check block storage in production
        // For now, we check if we have fewer than max Ants
        // (implies Queen succeeded or we're still producing)
        false
    }

    /// Select best Ant for slot (if Queen failed)
    pub fn select_best_ant(&self, slot: u64) -> Option<&AntBlock> {
        let ants = self.ants_per_slot.get(&slot)?;
        
        if ants.is_empty() {
            return None;
        }

        // Select Ant with:
        // 1. Most transactions
        // 2. Earliest timestamp (tiebreaker)
        ants.iter()
            .max_by(|a, b| {
                a.block
                    .transaction_count()
                    .cmp(&b.block.transaction_count())
                    .then_with(|| b.timestamp.cmp(&a.timestamp)) // Earlier is better
            })
    }

    /// Clear old Ants (keep last N slots)
    pub fn prune_old_ants(&mut self, current_slot: u64, keep_slots: u64) {
        if current_slot < keep_slots {
            return;
        }

        let cutoff = current_slot - keep_slots;
        self.ants_per_slot.retain(|slot, _| *slot > cutoff);
    }

    /// Get total Ant count
    pub fn total_ant_count(&self) -> usize {
        self.ants_per_slot.values().map(|ants| ants.len()).sum()
    }

    /// Get Ant statistics
    pub fn get_stats(&self) -> AntStats {
        let total_ants = self.total_ant_count();
        let slots_with_ants = self.ants_per_slot.len();
        
        let avg_ants_per_slot = if slots_with_ants > 0 {
            total_ants as f64 / slots_with_ants as f64
        } else {
            0.0
        };

        AntStats {
            total_ants,
            slots_with_ants,
            avg_ants_per_slot,
        }
    }

    /// Get timeout value
    pub fn ant_timeout(&self) -> u64 {
        self.ant_timeout
    }
}

impl Default for AntManager {
    fn default() -> Self {
        Self::new(MAX_ANTS_PER_SLOT, 3000) // 3 second timeout
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ANT BLOCK
// ═══════════════════════════════════════════════════════════════════════════════

/// Represents an Ant block (secondary block)
#[derive(Debug, Clone)]
pub struct AntBlock {
    /// The Ant block
    pub block: Block,
    
    /// Proposer who created this Ant
    pub proposer: ValidatorId,
    
    /// Timestamp when produced
    pub timestamp: u64,
    
    /// Slot this Ant was produced for
    pub slot: u64,
}

impl AntBlock {
    /// Create a new Ant block
    pub fn new(block: Block, proposer: ValidatorId, timestamp: u64, slot: u64) -> Self {
        Self {
            block,
            proposer,
            timestamp,
            slot,
        }
    }

    /// Get block hash
    pub fn hash(&self) -> Hash {
        self.block.hash()
    }

    /// Get block number
    pub fn number(&self) -> BlockNumber {
        self.block.number()
    }

    /// Verify this is actually an Ant block
    pub fn is_valid_ant(&self) -> bool {
        self.block.header.block_type == BlockType::Ant
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ANT PRODUCTION STRATEGY
// ═══════════════════════════════════════════════════════════════════════════════

/// Strategy for when to produce Ant blocks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntStrategy {
    /// Produce Ant immediately after timeout
    Immediate,
    
    /// Wait for a random delay after timeout (reduces collisions)
    RandomDelay,
    
    /// Only produce Ant if we have many transactions
    HighTxOnly,
}

/// Ant production decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntDecision {
    /// Produce an Ant block
    Produce,
    
    /// Wait (Queen might still arrive)
    Wait,
    
    /// Don't produce (max Ants reached or Queen arrived)
    Skip,
}

/// Determines when to produce Ant blocks
pub struct AntProducer {
    /// Production strategy
    strategy: AntStrategy,
    
    /// Minimum transactions for HighTxOnly strategy
    min_transactions: usize,
}

impl AntProducer {
    /// Create a new Ant producer
    pub fn new(strategy: AntStrategy, min_transactions: usize) -> Self {
        Self {
            strategy,
            min_transactions,
        }
    }

    /// Decide whether to produce an Ant
    pub fn should_produce_ant(
        &self,
        manager: &AntManager,
        slot: u64,
        time_since_slot_start: u64,
        available_transactions: usize,
    ) -> AntDecision {
        // Check if we can produce more Ants
        if !manager.can_produce_ant(slot) {
            return AntDecision::Skip;
        }

        // Check if timeout has passed
        if time_since_slot_start < manager.ant_timeout() {
            return AntDecision::Wait;
        }

        // Apply strategy
        match self.strategy {
            AntStrategy::Immediate => AntDecision::Produce,
            
            AntStrategy::RandomDelay => {
                // In production, would use random delay
                // For now, produce after timeout
                AntDecision::Produce
            }
            
            AntStrategy::HighTxOnly => {
                if available_transactions >= self.min_transactions {
                    AntDecision::Produce
                } else {
                    AntDecision::Wait
                }
            }
        }
    }
}

impl Default for AntProducer {
    fn default() -> Self {
        Self::new(AntStrategy::RandomDelay, 10)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Ant block statistics
#[derive(Debug, Clone, Default)]
pub struct AntStats {
    /// Total Ant blocks produced
    pub total_ants: usize,
    
    /// Number of slots with Ant blocks
    pub slots_with_ants: usize,
    
    /// Average Ants per slot (where Ants exist)
    pub avg_ants_per_slot: f64,
}

impl AntStats {
    /// Calculate Ant rate (slots with Ants / total slots)
    pub fn ant_rate(&self, total_slots: usize) -> f64 {
        if total_slots == 0 {
            return 0.0;
        }
        self.slots_with_ants as f64 / total_slots as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BlockBody, BlockHeader};

    fn create_test_ant_block(number: u64, tx_count: usize) -> AntBlock {
        let mut block = Block {
            header: BlockHeader {
                number,
                parent_hash: Hash::default(),
                state_root: Hash::default(),
                extrinsics_root: Hash::default(),
                block_type: BlockType::Ant,
                proposer: ValidatorId::from([1u8; 32]),
                ppfa_index: 0,
                timestamp: 0,
                epoch: 0,
            },
            body: BlockBody {
                transactions: Vec::new(),
            },
        };

        // Add transactions
        for _ in 0..tx_count {
            block.body.transactions.push(vec![1, 2, 3]);
        }

        AntBlock::new(block, ValidatorId::from([1u8; 32]), 1000, 0)
    }

    #[test]
    fn test_ant_manager_creation() {
        let manager = AntManager::default();
        assert_eq!(manager.max_ants_per_slot, MAX_ANTS_PER_SLOT);
    }

    #[test]
    fn test_can_produce_ant() {
        let manager = AntManager::default();
        assert!(manager.can_produce_ant(0));
    }

    #[test]
    fn test_register_ant() {
        let mut manager = AntManager::default();
        let ant = create_test_ant_block(1, 5);
        
        assert!(manager.register_ant(0, ant).is_ok());
        assert_eq!(manager.ant_count_for_slot(0), 1);
    }

    #[test]
    fn test_max_ants_per_slot() {
        let mut manager = AntManager::new(2, 3000); // Max 2 Ants
        
        let ant1 = create_test_ant_block(1, 5);
        let ant2 = create_test_ant_block(1, 5);
        let ant3 = create_test_ant_block(1, 5);
        
        assert!(manager.register_ant(0, ant1).is_ok());
        assert!(manager.register_ant(0, ant2).is_ok());
        assert!(manager.register_ant(0, ant3).is_err()); // Should fail
    }

    #[test]
    fn test_get_ants_for_slot() {
        let mut manager = AntManager::default();
        
        let ant1 = create_test_ant_block(1, 5);
        let ant2 = create_test_ant_block(1, 3);
        
        manager.register_ant(0, ant1).unwrap();
        manager.register_ant(0, ant2).unwrap();
        
        let ants = manager.get_ants_for_slot(0);
        assert_eq!(ants.len(), 2);
    }

    #[test]
    fn test_select_best_ant() {
        let mut manager = AntManager::default();
        
        let ant1 = create_test_ant_block(1, 3); // 3 transactions
        let ant2 = create_test_ant_block(1, 7); // 7 transactions (best)
        let ant3 = create_test_ant_block(1, 5); // 5 transactions
        
        manager.register_ant(0, ant1).unwrap();
        manager.register_ant(0, ant2).unwrap();
        manager.register_ant(0, ant3).unwrap();
        
        let best = manager.select_best_ant(0).unwrap();
        assert_eq!(best.block.transaction_count(), 7);
    }

    #[test]
    fn test_prune_old_ants() {
        let mut manager = AntManager::default();
        
        manager.register_ant(0, create_test_ant_block(1, 1)).unwrap();
        manager.register_ant(1, create_test_ant_block(2, 1)).unwrap();
        manager.register_ant(5, create_test_ant_block(3, 1)).unwrap();
        
        assert_eq!(manager.total_ant_count(), 3);
        
        // Prune: keep last 3 slots from slot 5
        manager.prune_old_ants(5, 3);
        
        // Should keep slots 3, 4, 5 (only slot 5 has Ants)
        assert_eq!(manager.total_ant_count(), 1);
    }

    #[test]
    fn test_ant_stats() {
        let mut manager = AntManager::default();
        
        manager.register_ant(0, create_test_ant_block(1, 1)).unwrap();
        manager.register_ant(0, create_test_ant_block(1, 1)).unwrap();
        manager.register_ant(2, create_test_ant_block(2, 1)).unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_ants, 3);
        assert_eq!(stats.slots_with_ants, 2);
        assert_eq!(stats.avg_ants_per_slot, 1.5); // 3 Ants / 2 slots
    }

    #[test]
    fn test_ant_block_creation() {
        let ant = create_test_ant_block(1, 5);
        assert_eq!(ant.number(), 1);
        assert!(ant.is_valid_ant());
        assert_eq!(ant.block.transaction_count(), 5);
    }

    #[test]
    fn test_ant_producer_immediate() {
        let producer = AntProducer::new(AntStrategy::Immediate, 0);
        let manager = AntManager::default();
        
        // Before timeout
        let decision = producer.should_produce_ant(&manager, 0, 2000, 5);
        assert_eq!(decision, AntDecision::Wait);
        
        // After timeout
        let decision = producer.should_produce_ant(&manager, 0, 4000, 5);
        assert_eq!(decision, AntDecision::Produce);
    }

    #[test]
    fn test_ant_producer_high_tx_only() {
        let producer = AntProducer::new(AntStrategy::HighTxOnly, 10);
        let manager = AntManager::default();
        
        // After timeout, but not enough transactions
        let decision = producer.should_produce_ant(&manager, 0, 4000, 5);
        assert_eq!(decision, AntDecision::Wait);
        
        // After timeout, with enough transactions
        let decision = producer.should_produce_ant(&manager, 0, 4000, 15);
        assert_eq!(decision, AntDecision::Produce);
    }

    #[test]
    fn test_ant_producer_max_reached() {
        let producer = AntProducer::new(AntStrategy::Immediate, 0);
        let mut manager = AntManager::new(1, 3000); // Max 1 Ant
        
        // Register one Ant
        manager.register_ant(0, create_test_ant_block(1, 1)).unwrap();
        
        // Should skip (max reached)
        let decision = producer.should_produce_ant(&manager, 0, 4000, 5);
        assert_eq!(decision, AntDecision::Skip);
    }

    #[test]
    fn test_ant_rate_calculation() {
        let mut stats = AntStats::default();
        stats.slots_with_ants = 3;
        
        assert_eq!(stats.ant_rate(10), 0.3); // 3/10 = 30%
        assert_eq!(stats.ant_rate(0), 0.0); // Division by zero
    }
}