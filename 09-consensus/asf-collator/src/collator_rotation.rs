//! # Collator Rotation
//!
//! Implements deterministic collator selection and rotation for PBC consensus.
//!
//! Features:
//! - Round-robin collator selection
//! - Stake-weighted probability
//! - Fast rotation (every 6 blocks)
//! - Graceful collator set updates

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

use crate::{CollatorId, CollatorCommittee, ParaId, BlockNumber, Balance, AsfError, AsfResult};

/// Rotation strategy for collator selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum RotationStrategy {
    /// Round-robin rotation (deterministic, fair)
    RoundRobin,
    /// Stake-weighted random selection
    StakeWeighted,
    /// Hybrid: stake-weighted with minimum rotation
    Hybrid,
}

impl Default for RotationStrategy {
    fn default() -> Self {
        RotationStrategy::Hybrid
    }
}

/// Collator rotation configuration
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct RotationConfig {
    /// Blocks between rotations
    pub rotation_period: BlockNumber,
    /// Rotation strategy
    pub strategy: RotationStrategy,
    /// Minimum blocks before collator can be selected again
    pub cooldown_period: BlockNumber,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            rotation_period: 6, // Fast rotation every 6 blocks
            strategy: RotationStrategy::Hybrid,
            cooldown_period: 12, // 2x rotation period
        }
    }
}

/// Collator selection result
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CollatorSelection {
    /// Selected collator
    pub collator: CollatorId,
    /// Block number for this selection
    pub block_number: BlockNumber,
    /// Rotation round
    pub rotation_round: u64,
    /// Selection reason/priority
    pub priority: u64,
}

/// Collator rotation manager
pub struct RotationManager {
    config: RotationConfig,
    last_selected: Vec<(CollatorId, BlockNumber)>,
}

impl RotationManager {
    /// Create new rotation manager
    pub fn new(config: RotationConfig) -> Self {
        Self {
            config,
            last_selected: Vec::new(),
        }
    }

    /// Check if rotation should occur at block
    pub fn should_rotate(&self, block_number: BlockNumber) -> bool {
        block_number % self.config.rotation_period == 0
    }

    /// Get rotation round for block number
    pub fn get_rotation_round(&self, block_number: BlockNumber) -> u64 {
        (block_number / self.config.rotation_period) as u64
    }

    /// Select next collator using configured strategy
    pub fn select_collator(
        &mut self,
        committee: &CollatorCommittee,
        block_number: BlockNumber,
        randomness_seed: H256,
    ) -> AsfResult<CollatorSelection> {
        if committee.collators.is_empty() {
            return Err(AsfError::InvalidVote("No collators available"));
        }

        let rotation_round = self.get_rotation_round(block_number);

        let collator = match self.config.strategy {
            RotationStrategy::RoundRobin => {
                self.select_round_robin(committee, rotation_round)?
            }
            RotationStrategy::StakeWeighted => {
                self.select_stake_weighted(committee, randomness_seed)?
            }
            RotationStrategy::Hybrid => {
                self.select_hybrid(committee, block_number, randomness_seed)?
            }
        };

        // Update last selected
        self.last_selected.push((collator.clone(), block_number));

        // Keep only recent selections (for cooldown tracking)
        let cutoff = block_number.saturating_sub(self.config.cooldown_period * 2);
        self.last_selected.retain(|(_, bn)| *bn > cutoff);

        Ok(CollatorSelection {
            collator,
            block_number,
            rotation_round,
            priority: rotation_round,
        })
    }

    /// Round-robin selection (deterministic)
    fn select_round_robin(
        &self,
        committee: &CollatorCommittee,
        rotation_round: u64,
    ) -> AsfResult<CollatorId> {
        let index = (rotation_round as usize) % committee.collators.len();
        Ok(committee.collators[index].clone())
    }

    /// Stake-weighted random selection
    fn select_stake_weighted(
        &self,
        committee: &CollatorCommittee,
        randomness_seed: H256,
    ) -> AsfResult<CollatorId> {
        if committee.total_stake == 0 {
            return Err(AsfError::InsufficientStake {
                got: 0,
                need: 1,
            });
        }

        // Generate random number from seed
        let random_value = u128::from_le_bytes([
            randomness_seed[0], randomness_seed[1], randomness_seed[2], randomness_seed[3],
            randomness_seed[4], randomness_seed[5], randomness_seed[6], randomness_seed[7],
            randomness_seed[8], randomness_seed[9], randomness_seed[10], randomness_seed[11],
            randomness_seed[12], randomness_seed[13], randomness_seed[14], randomness_seed[15],
        ]);

        let threshold = random_value % committee.total_stake;

        // NOTE: This is simplified - in production, we'd need actual stake mapping
        // For now, assume equal stake distribution
        let stake_per_collator = committee.total_stake / (committee.collators.len() as u128);
        let mut cumulative = 0u128;

        for collator in &committee.collators {
            cumulative += stake_per_collator;
            if cumulative > threshold {
                return Ok(collator.clone());
            }
        }

        // Fallback to last collator (shouldn't happen)
        Ok(committee.collators.last().unwrap().clone())
    }

    /// Hybrid selection: stake-weighted with cooldown
    fn select_hybrid(
        &self,
        committee: &CollatorCommittee,
        block_number: BlockNumber,
        randomness_seed: H256,
    ) -> AsfResult<CollatorId> {
        // Filter out collators in cooldown
        let cutoff = block_number.saturating_sub(self.config.cooldown_period);
        let available: Vec<_> = committee
            .collators
            .iter()
            .filter(|c| {
                !self
                    .last_selected
                    .iter()
                    .any(|(id, bn)| id == *c && *bn > cutoff)
            })
            .cloned()
            .collect();

        if available.is_empty() {
            // All in cooldown, use stake-weighted
            return self.select_stake_weighted(committee, randomness_seed);
        }

        // Create temporary committee with only available collators
        let mut temp_committee = committee.clone();
        temp_committee.collators = available;

        self.select_stake_weighted(&temp_committee, randomness_seed)
    }

    /// Check if collator is in cooldown period
    pub fn is_in_cooldown(&self, collator: &CollatorId, current_block: BlockNumber) -> bool {
        let cutoff = current_block.saturating_sub(self.config.cooldown_period);
        self.last_selected
            .iter()
            .any(|(id, bn)| id == collator && *bn > cutoff)
    }

    /// Get blocks until collator can be selected again
    pub fn blocks_until_available(
        &self,
        collator: &CollatorId,
        current_block: BlockNumber,
    ) -> Option<BlockNumber> {
        self.last_selected
            .iter()
            .filter(|(id, _)| id == collator)
            .map(|(_, bn)| {
                let available_at = bn + self.config.cooldown_period;
                available_at.saturating_sub(current_block)
            })
            .min()
    }
}

/// Calculate deterministic randomness seed for collator selection
pub fn calculate_randomness_seed(
    para_id: ParaId,
    block_number: BlockNumber,
    relay_parent: H256,
) -> H256 {
    let mut data = Vec::new();
    data.extend_from_slice(&para_id.to_le_bytes());
    data.extend_from_slice(&block_number.to_le_bytes());
    data.extend_from_slice(relay_parent.as_ref());
    BlakeTwo256::hash(&data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    fn create_test_committee(size: usize) -> CollatorCommittee {
        let mut committee = CollatorCommittee::new(1000, 7, 11);
        for i in 0..size {
            let collator = AccountId32::new([i as u8; 32]);
            committee.add_collator(collator, 100_000).unwrap();
        }
        committee
    }

    #[test]
    fn test_rotation_period() {
        let config = RotationConfig::default();
        let manager = RotationManager::new(config);

        assert!(manager.should_rotate(0));
        assert!(!manager.should_rotate(1));
        assert!(!manager.should_rotate(5));
        assert!(manager.should_rotate(6));
        assert!(manager.should_rotate(12));
    }

    #[test]
    fn test_rotation_round() {
        let config = RotationConfig::default();
        let manager = RotationManager::new(config);

        assert_eq!(manager.get_rotation_round(0), 0);
        assert_eq!(manager.get_rotation_round(5), 0);
        assert_eq!(manager.get_rotation_round(6), 1);
        assert_eq!(manager.get_rotation_round(12), 2);
    }

    #[test]
    fn test_round_robin_selection() {
        let committee = create_test_committee(7);
        let config = RotationConfig {
            rotation_period: 6,
            strategy: RotationStrategy::RoundRobin,
            cooldown_period: 12,
        };
        let mut manager = RotationManager::new(config);

        // Select collators for multiple rounds
        let mut selections = Vec::new();
        for round in 0..14 {
            let block = round * 6;
            let seed = H256::random();
            let selection = manager.select_collator(&committee, block, seed).unwrap();
            selections.push(selection.collator.clone());
        }

        // Should rotate through all 7 collators twice
        assert_eq!(selections[0], selections[7]);
        assert_eq!(selections[1], selections[8]);
        assert_ne!(selections[0], selections[1]);
    }

    #[test]
    fn test_cooldown_period() {
        let committee = create_test_committee(7);
        let config = RotationConfig {
            rotation_period: 6,
            strategy: RotationStrategy::Hybrid,
            cooldown_period: 12,
        };
        let mut manager = RotationManager::new(config);

        let collator1 = committee.collators[0].clone();

        // Select at block 0
        let seed = H256::random();
        manager.select_collator(&committee, 0, seed).unwrap();

        // Should be in cooldown at block 6
        assert!(manager.is_in_cooldown(&collator1, 6));

        // Should be available at block 12
        assert!(!manager.is_in_cooldown(&collator1, 12));
    }

    #[test]
    fn test_randomness_seed_deterministic() {
        let para_id = 1000;
        let block_number = 100;
        let relay_parent = H256::from([42u8; 32]);

        let seed1 = calculate_randomness_seed(para_id, block_number, relay_parent);
        let seed2 = calculate_randomness_seed(para_id, block_number, relay_parent);

        // Same inputs should produce same seed
        assert_eq!(seed1, seed2);

        // Different block should produce different seed
        let seed3 = calculate_randomness_seed(para_id, block_number + 1, relay_parent);
        assert_ne!(seed1, seed3);
    }
}
