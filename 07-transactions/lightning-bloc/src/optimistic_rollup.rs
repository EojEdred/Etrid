//! Optimistic Rollup Integration for Lightning-Bloc Layer 2
//!
//! Provides optimistic execution with fraud proofs, state commitments,
//! and efficient settlement to the main chain.

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

/// Challenge period for optimistic execution (7 days)
pub const OPTIMISTIC_CHALLENGE_PERIOD: u64 = 7 * 24 * 60 * 60;

/// Maximum state root size
pub const MAX_STATE_ROOT_SIZE: usize = 32; // 32 bytes for hash

/// State commitment representing L2 state at a specific block
#[derive(Clone, Debug, PartialEq)]
pub struct StateCommitment {
    pub commitment_id: String,
    pub state_root: Vec<u8>,
    pub block_number: u64,
    pub transaction_count: u64,
    pub timestamp: u64,
    pub challenge_deadline: u64,
    pub status: CommitmentStatus,
}

impl StateCommitment {
    /// Create new state commitment
    pub fn new(
        state_root: Vec<u8>,
        block_number: u64,
        transaction_count: u64,
        timestamp: u64,
    ) -> Result<Self, RollupError> {
        if state_root.len() > MAX_STATE_ROOT_SIZE {
            return Err(RollupError::InvalidStateRoot {
                size: state_root.len(),
                max: MAX_STATE_ROOT_SIZE,
            });
        }

        if state_root.is_empty() {
            return Err(RollupError::EmptyStateRoot);
        }

        let commitment_id = format!("commitment_{}_{}", block_number, timestamp);
        let challenge_deadline = timestamp + OPTIMISTIC_CHALLENGE_PERIOD;

        Ok(Self {
            commitment_id,
            state_root,
            block_number,
            transaction_count,
            timestamp,
            challenge_deadline,
            status: CommitmentStatus::Pending,
        })
    }

    /// Check if challenge period has expired
    pub fn is_challenge_expired(&self, current_time: u64) -> bool {
        current_time > self.challenge_deadline
    }

    /// Check if commitment can be finalized
    pub fn can_finalize(&self, current_time: u64) -> bool {
        matches!(self.status, CommitmentStatus::Pending) && self.is_challenge_expired(current_time)
    }

    /// Finalize commitment after challenge period
    pub fn finalize(&mut self, timestamp: u64) -> Result<(), RollupError> {
        if !matches!(self.status, CommitmentStatus::Pending) {
            return Err(RollupError::InvalidCommitmentStatus {
                current: self.status.clone(),
            });
        }

        self.status = CommitmentStatus::Finalized { finalized_at: timestamp };
        Ok(())
    }

    /// Challenge commitment with fraud proof
    pub fn challenge(&mut self, challenger: String, timestamp: u64) -> Result<(), RollupError> {
        if !matches!(self.status, CommitmentStatus::Pending) {
            return Err(RollupError::InvalidCommitmentStatus {
                current: self.status.clone(),
            });
        }

        if self.is_challenge_expired(timestamp) {
            return Err(RollupError::ChallengePeriodExpired {
                deadline: self.challenge_deadline,
            });
        }

        self.status = CommitmentStatus::Challenged {
            challenger,
            challenged_at: timestamp,
        };
        Ok(())
    }

    /// Reject commitment after successful challenge
    pub fn reject(&mut self, timestamp: u64) -> Result<(), RollupError> {
        if !matches!(self.status, CommitmentStatus::Challenged { .. }) {
            return Err(RollupError::InvalidCommitmentStatus {
                current: self.status.clone(),
            });
        }

        self.status = CommitmentStatus::Rejected { rejected_at: timestamp };
        Ok(())
    }
}

/// Commitment status
#[derive(Clone, Debug, PartialEq)]
pub enum CommitmentStatus {
    Pending,
    Challenged { challenger: String, challenged_at: u64 },
    Finalized { finalized_at: u64 },
    Rejected { rejected_at: u64 },
}

impl fmt::Display for CommitmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommitmentStatus::Pending => write!(f, "Pending"),
            CommitmentStatus::Challenged { challenger, .. } => {
                write!(f, "Challenged by {}", challenger)
            }
            CommitmentStatus::Finalized { .. } => write!(f, "Finalized"),
            CommitmentStatus::Rejected { .. } => write!(f, "Rejected"),
        }
    }
}

/// L2 block containing transactions
#[derive(Clone, Debug, PartialEq)]
pub struct L2Block {
    pub block_number: u64,
    pub parent_hash: Vec<u8>,
    pub state_root: Vec<u8>,
    pub transactions_root: Vec<u8>,
    pub transaction_count: u64,
    pub timestamp: u64,
    pub sequencer: String,
}

impl L2Block {
    /// Create new L2 block
    pub fn new(
        block_number: u64,
        parent_hash: Vec<u8>,
        state_root: Vec<u8>,
        transactions_root: Vec<u8>,
        transaction_count: u64,
        timestamp: u64,
        sequencer: String,
    ) -> Self {
        Self {
            block_number,
            parent_hash,
            state_root,
            transactions_root,
            transaction_count,
            timestamp,
            sequencer,
        }
    }

    /// Calculate block hash
    pub fn hash(&self) -> Vec<u8> {
        // Simplified hash - in production use proper cryptographic hash
        let mut data = Vec::new();
        data.extend_from_slice(&self.block_number.to_le_bytes());
        data.extend_from_slice(&self.parent_hash);
        data.extend_from_slice(&self.state_root);
        data.extend_from_slice(&self.transactions_root);
        data
    }
}

/// Rollup sequencer producing L2 blocks
#[derive(Clone, Debug, PartialEq)]
pub struct Sequencer {
    pub address: String,
    pub stake: u128,
    pub blocks_produced: u64,
    pub challenges_faced: u64,
    pub active: bool,
}

impl Sequencer {
    pub fn new(address: String, stake: u128) -> Self {
        Self {
            address,
            stake,
            blocks_produced: 0,
            challenges_faced: 0,
            active: true,
        }
    }

    /// Record block production
    pub fn record_block(&mut self) {
        self.blocks_produced += 1;
    }

    /// Record challenge
    pub fn record_challenge(&mut self) {
        self.challenges_faced += 1;
    }
}

/// Optimistic rollup manager
pub struct OptimisticRollup {
    commitments: HashMap<String, StateCommitment>,
    blocks: HashMap<u64, L2Block>,
    sequencers: HashMap<String, Sequencer>,
    latest_block_number: u64,
    latest_finalized_block: u64,
}

impl OptimisticRollup {
    /// Create new optimistic rollup
    pub fn new() -> Self {
        Self {
            commitments: HashMap::new(),
            blocks: HashMap::new(),
            sequencers: HashMap::new(),
            latest_block_number: 0,
            latest_finalized_block: 0,
        }
    }

    /// Register sequencer
    pub fn register_sequencer(&mut self, sequencer: Sequencer) -> Result<(), RollupError> {
        if self.sequencers.contains_key(&sequencer.address) {
            return Err(RollupError::SequencerAlreadyRegistered(sequencer.address.clone()));
        }

        self.sequencers.insert(sequencer.address.clone(), sequencer);
        Ok(())
    }

    /// Submit L2 block
    pub fn submit_block(&mut self, block: L2Block) -> Result<(), RollupError> {
        // Verify sequencer is registered and active
        let sequencer = self.sequencers
            .get_mut(&block.sequencer)
            .ok_or_else(|| RollupError::SequencerNotFound(block.sequencer.clone()))?;

        if !sequencer.active {
            return Err(RollupError::SequencerInactive(block.sequencer.clone()));
        }

        // Verify block number is next in sequence
        if block.block_number != self.latest_block_number + 1 {
            return Err(RollupError::InvalidBlockNumber {
                expected: self.latest_block_number + 1,
                got: block.block_number,
            });
        }

        // Verify parent hash
        if block.block_number > 0 {
            if let Some(parent) = self.blocks.get(&(block.block_number - 1)) {
                let parent_hash = parent.hash();
                if parent_hash != block.parent_hash {
                    return Err(RollupError::InvalidParentHash);
                }
            }
        }

        // Store block
        self.blocks.insert(block.block_number, block.clone());
        self.latest_block_number = block.block_number;

        // Record block production
        sequencer.record_block();

        // Create state commitment
        let commitment = StateCommitment::new(
            block.state_root.clone(),
            block.block_number,
            block.transaction_count,
            block.timestamp,
        )?;

        self.commitments.insert(commitment.commitment_id.clone(), commitment);

        Ok(())
    }

    /// Challenge state commitment
    pub fn challenge_commitment(
        &mut self,
        commitment_id: &str,
        challenger: String,
        timestamp: u64,
    ) -> Result<(), RollupError> {
        let commitment = self.commitments
            .get_mut(commitment_id)
            .ok_or_else(|| RollupError::CommitmentNotFound(commitment_id.to_string()))?;

        commitment.challenge(challenger.clone(), timestamp)?;

        // Record challenge for sequencer
        let block = self.blocks.get(&commitment.block_number)
            .ok_or_else(|| RollupError::BlockNotFound(commitment.block_number))?;

        if let Some(sequencer) = self.sequencers.get_mut(&block.sequencer) {
            sequencer.record_challenge();
        }

        Ok(())
    }

    /// Finalize commitments after challenge period
    pub fn finalize_commitments(&mut self, current_time: u64) -> Vec<String> {
        let mut finalized = Vec::new();

        for (id, commitment) in self.commitments.iter_mut() {
            if commitment.can_finalize(current_time) {
                if commitment.finalize(current_time).is_ok() {
                    self.latest_finalized_block = self.latest_finalized_block.max(commitment.block_number);
                    finalized.push(id.clone());
                }
            }
        }

        finalized
    }

    /// Get commitment
    pub fn get_commitment(&self, commitment_id: &str) -> Option<&StateCommitment> {
        self.commitments.get(commitment_id)
    }

    /// Get block
    pub fn get_block(&self, block_number: u64) -> Option<&L2Block> {
        self.blocks.get(&block_number)
    }

    /// Get sequencer
    pub fn get_sequencer(&self, address: &str) -> Option<&Sequencer> {
        self.sequencers.get(address)
    }

    /// Get pending commitments count
    pub fn pending_commitments_count(&self) -> usize {
        self.commitments
            .values()
            .filter(|c| matches!(c.status, CommitmentStatus::Pending))
            .count()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> RollupStatistics {
        let total_blocks = self.blocks.len();
        let total_commitments = self.commitments.len();
        let pending_commitments = self.pending_commitments_count();
        let total_sequencers = self.sequencers.len();
        let active_sequencers = self.sequencers.values().filter(|s| s.active).count();

        RollupStatistics {
            total_blocks,
            latest_block_number: self.latest_block_number,
            latest_finalized_block: self.latest_finalized_block,
            total_commitments,
            pending_commitments,
            total_sequencers,
            active_sequencers,
        }
    }
}

impl Default for OptimisticRollup {
    fn default() -> Self {
        Self::new()
    }
}

/// Rollup statistics
#[derive(Clone, Debug, PartialEq)]
pub struct RollupStatistics {
    pub total_blocks: usize,
    pub latest_block_number: u64,
    pub latest_finalized_block: u64,
    pub total_commitments: usize,
    pub pending_commitments: usize,
    pub total_sequencers: usize,
    pub active_sequencers: usize,
}

/// Rollup errors
#[derive(Clone, Debug, PartialEq)]
pub enum RollupError {
    InvalidStateRoot { size: usize, max: usize },
    EmptyStateRoot,
    InvalidCommitmentStatus { current: CommitmentStatus },
    ChallengePeriodExpired { deadline: u64 },
    CommitmentNotFound(String),
    BlockNotFound(u64),
    SequencerNotFound(String),
    SequencerInactive(String),
    SequencerAlreadyRegistered(String),
    InvalidBlockNumber { expected: u64, got: u64 },
    InvalidParentHash,
}

impl fmt::Display for RollupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollupError::InvalidStateRoot { size, max } => {
                write!(f, "Invalid state root: size {} > max {}", size, max)
            }
            RollupError::EmptyStateRoot => write!(f, "Empty state root"),
            RollupError::InvalidCommitmentStatus { current } => {
                write!(f, "Invalid commitment status: {}", current)
            }
            RollupError::ChallengePeriodExpired { deadline } => {
                write!(f, "Challenge period expired: deadline {}", deadline)
            }
            RollupError::CommitmentNotFound(id) => write!(f, "Commitment not found: {}", id),
            RollupError::BlockNotFound(num) => write!(f, "Block not found: {}", num),
            RollupError::SequencerNotFound(addr) => write!(f, "Sequencer not found: {}", addr),
            RollupError::SequencerInactive(addr) => write!(f, "Sequencer inactive: {}", addr),
            RollupError::SequencerAlreadyRegistered(addr) => {
                write!(f, "Sequencer already registered: {}", addr)
            }
            RollupError::InvalidBlockNumber { expected, got } => {
                write!(f, "Invalid block number: expected {}, got {}", expected, got)
            }
            RollupError::InvalidParentHash => write!(f, "Invalid parent hash"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_commitment(block_number: u64, timestamp: u64) -> StateCommitment {
        StateCommitment::new(
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            block_number,
            100,
            timestamp,
        )
        .unwrap()
    }

    fn create_test_block(block_number: u64, sequencer: &str, timestamp: u64) -> L2Block {
        L2Block::new(
            block_number,
            vec![0; 32],
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            10,
            timestamp,
            sequencer.to_string(),
        )
    }

    #[test]
    fn test_state_commitment_creation() {
        let commitment = create_test_commitment(1, 1000);
        assert_eq!(commitment.block_number, 1);
        assert!(matches!(commitment.status, CommitmentStatus::Pending));
    }

    #[test]
    fn test_state_commitment_invalid_root() {
        let large_root = vec![0u8; MAX_STATE_ROOT_SIZE + 1];
        let result = StateCommitment::new(large_root, 1, 100, 1000);
        assert!(result.is_err());
    }

    #[test]
    fn test_challenge_period_expiration() {
        let commitment = create_test_commitment(1, 1000);
        assert!(!commitment.is_challenge_expired(1000 + OPTIMISTIC_CHALLENGE_PERIOD - 1));
        assert!(commitment.is_challenge_expired(1000 + OPTIMISTIC_CHALLENGE_PERIOD + 1));
    }

    #[test]
    fn test_commitment_finalization() {
        let mut commitment = create_test_commitment(1, 1000);
        let finalize_time = 1000 + OPTIMISTIC_CHALLENGE_PERIOD + 1;

        assert!(commitment.can_finalize(finalize_time));
        assert!(commitment.finalize(finalize_time).is_ok());
        assert!(matches!(commitment.status, CommitmentStatus::Finalized { .. }));
    }

    #[test]
    fn test_commitment_challenge() {
        let mut commitment = create_test_commitment(1, 1000);
        let result = commitment.challenge("alice".to_string(), 1100);
        assert!(result.is_ok());
        assert!(matches!(commitment.status, CommitmentStatus::Challenged { .. }));
    }

    #[test]
    fn test_challenge_after_expiration() {
        let mut commitment = create_test_commitment(1, 1000);
        let expired_time = 1000 + OPTIMISTIC_CHALLENGE_PERIOD + 1;
        let result = commitment.challenge("alice".to_string(), expired_time);
        assert!(result.is_err());
    }

    #[test]
    fn test_l2_block_creation() {
        let block = create_test_block(1, "sequencer_1", 1000);
        assert_eq!(block.block_number, 1);
        assert_eq!(block.sequencer, "sequencer_1");
    }

    #[test]
    fn test_l2_block_hash() {
        let block = create_test_block(1, "sequencer_1", 1000);
        let hash = block.hash();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_sequencer_creation() {
        let sequencer = Sequencer::new("alice".to_string(), 1000);
        assert_eq!(sequencer.address, "alice");
        assert!(sequencer.active);
        assert_eq!(sequencer.blocks_produced, 0);
    }

    #[test]
    fn test_optimistic_rollup_creation() {
        let rollup = OptimisticRollup::new();
        assert_eq!(rollup.latest_block_number, 0);
    }

    #[test]
    fn test_register_sequencer() {
        let mut rollup = OptimisticRollup::new();
        let sequencer = Sequencer::new("alice".to_string(), 1000);

        assert!(rollup.register_sequencer(sequencer).is_ok());
        assert!(rollup.get_sequencer("alice").is_some());
    }

    #[test]
    fn test_submit_block() {
        let mut rollup = OptimisticRollup::new();
        let sequencer = Sequencer::new("alice".to_string(), 1000);
        rollup.register_sequencer(sequencer).unwrap();

        let block = create_test_block(1, "alice", 1000);
        assert!(rollup.submit_block(block).is_ok());
        assert_eq!(rollup.latest_block_number, 1);
    }

    #[test]
    fn test_submit_block_invalid_sequencer() {
        let mut rollup = OptimisticRollup::new();
        let block = create_test_block(1, "unknown", 1000);
        assert!(rollup.submit_block(block).is_err());
    }

    #[test]
    fn test_submit_block_invalid_number() {
        let mut rollup = OptimisticRollup::new();
        let sequencer = Sequencer::new("alice".to_string(), 1000);
        rollup.register_sequencer(sequencer).unwrap();

        let block = create_test_block(5, "alice", 1000); // Skip to block 5
        assert!(rollup.submit_block(block).is_err());
    }

    #[test]
    fn test_challenge_commitment() {
        let mut rollup = OptimisticRollup::new();
        let sequencer = Sequencer::new("alice".to_string(), 1000);
        rollup.register_sequencer(sequencer).unwrap();

        let block = create_test_block(1, "alice", 1000);
        rollup.submit_block(block).unwrap();

        // Get commitment ID
        let commitment_id = format!("commitment_1_1000");

        let result = rollup.challenge_commitment(&commitment_id, "bob".to_string(), 1100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_finalize_commitments() {
        let mut rollup = OptimisticRollup::new();
        let sequencer = Sequencer::new("alice".to_string(), 1000);
        rollup.register_sequencer(sequencer).unwrap();

        let block = create_test_block(1, "alice", 1000);
        rollup.submit_block(block).unwrap();

        // Fast forward past challenge period
        let finalize_time = 1000 + OPTIMISTIC_CHALLENGE_PERIOD + 1;
        let finalized = rollup.finalize_commitments(finalize_time);

        assert_eq!(finalized.len(), 1);
        assert_eq!(rollup.latest_finalized_block, 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut rollup = OptimisticRollup::new();
        let sequencer = Sequencer::new("alice".to_string(), 1000);
        rollup.register_sequencer(sequencer).unwrap();

        let block = create_test_block(1, "alice", 1000);
        rollup.submit_block(block).unwrap();

        let stats = rollup.get_statistics();
        assert_eq!(stats.total_blocks, 1);
        assert_eq!(stats.latest_block_number, 1);
        assert_eq!(stats.total_sequencers, 1);
        assert_eq!(stats.pending_commitments, 1);
    }
}
