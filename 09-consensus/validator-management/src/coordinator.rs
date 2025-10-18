//! # Validator Management Coordinator
//!
//! This module provides the main coordination loop for validator management in Ëtrid FlareChain.
//! It integrates committee management, health monitoring, rewards, and networking.

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::time::Duration;

use crate::{
    committee::CommitteeManager,
    health::HealthMonitor,
    networking::NetworkCoordinator,
    rewards::RewardCalculator,
    state_sync::StateSync,
    ValidatorInfo, ValidatorResult, BlockNumber, EPOCH_DURATION, HEALTH_CHECK_INTERVAL,
};

/// Validator management coordinator configuration
#[derive(Clone)]
pub struct CoordinatorConfig {
    /// Maximum committee size
    pub max_committee_size: u32,

    /// Epoch duration in blocks
    pub epoch_duration: u32,

    /// Health check interval in blocks
    pub health_check_interval: u32,

    /// Enable reward distribution
    pub enable_rewards: bool,

    /// Enable state synchronization
    pub enable_state_sync: bool,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            max_committee_size: 21,
            epoch_duration: EPOCH_DURATION,
            health_check_interval: HEALTH_CHECK_INTERVAL,
            enable_rewards: true,
            enable_state_sync: true,
        }
    }
}

/// Main validator management coordinator
pub struct ValidatorCoordinator {
    /// Committee manager
    committee: CommitteeManager,

    /// Health monitor
    health_monitor: HealthMonitor,

    /// Network coordinator
    network: NetworkCoordinator,

    /// Reward calculator
    reward_calculator: RewardCalculator,

    /// State synchronization
    state_sync: StateSync,

    /// Configuration
    config: CoordinatorConfig,

    /// Current block number
    current_block: BlockNumber,

    /// Current epoch
    current_epoch: u32,
}

impl ValidatorCoordinator {
    /// Create a new validator coordinator
    pub fn new(config: CoordinatorConfig) -> Self {
        Self {
            committee: CommitteeManager::new(config.max_committee_size),
            health_monitor: HealthMonitor::default(),
            network: NetworkCoordinator::new(100, 30000), // 100 max peers, 30s timeout
            reward_calculator: RewardCalculator::default(),
            state_sync: StateSync::new(config.epoch_duration), // Sync every epoch
            config,
            current_block: 0,
            current_epoch: 0,
        }
    }

    /// Initialize coordinator with genesis validators
    pub fn initialize(&mut self, genesis_validators: Vec<ValidatorInfo>) -> ValidatorResult<()> {
        log::info!("Initializing validator coordinator with {} genesis validators", genesis_validators.len());

        // Add genesis validators to committee
        for validator in genesis_validators {
            if validator.can_participate() {
                self.committee.add_validator(validator)?;
            }
        }

        // Perform initial committee rotation
        self.committee.rotate_committee(0)?;

        log::info!("✅ Validator coordinator initialized");
        log::info!("   - Committee size: {}", self.committee.current_committee().len());
        log::info!("   - Epoch duration: {} blocks", self.config.epoch_duration);

        Ok(())
    }

    /// Main coordinator loop - processes blocks and handles validator management
    pub fn process_block(&mut self, block_number: BlockNumber) -> ValidatorResult<()> {
        self.current_block = block_number;
        let epoch = (block_number / self.config.epoch_duration as u64) as u32;

        // Check for epoch transition
        if epoch != self.current_epoch {
            log::info!("🔄 Epoch transition: {} → {}", self.current_epoch, epoch);
            self.handle_epoch_transition(epoch)?;
            self.current_epoch = epoch;
        }

        // Periodic health checks
        if block_number % self.config.health_check_interval as u64 == 0 {
            self.perform_health_check()?;
        }

        // Update state sync (if enabled)
        if self.config.enable_state_sync && self.state_sync.needs_sync(block_number) {
            self.state_sync.mark_synced(block_number);
        }

        Ok(())
    }

    /// Handle epoch transition
    fn handle_epoch_transition(&mut self, new_epoch: u32) -> ValidatorResult<()> {
        log::debug!("Handling epoch transition to epoch {}", new_epoch);

        // Distribute rewards for previous epoch
        if self.config.enable_rewards && new_epoch > 0 {
            self.distribute_epoch_rewards(new_epoch - 1)?;
        }

        // Rotate committee for new epoch
        self.committee.rotate_committee(new_epoch)?;

        // Update validator states
        self.update_validator_states_for_epoch(new_epoch)?;

        log::info!("✅ Epoch {} started with {} committee members",
            new_epoch,
            self.committee.current_committee().len()
        );

        Ok(())
    }

    /// Perform health check on all validators
    fn perform_health_check(&mut self) -> ValidatorResult<()> {
        let committee = self.committee.current_committee().to_vec();
        let mut unhealthy_count = 0;

        // Check overall network health
        let is_healthy = self.health_monitor.is_healthy();

        if !is_healthy {
            // Network is unhealthy, penalize all committee members slightly
            for member in committee.iter() {
                unhealthy_count += 1;
                log::warn!("Validator {:?} in unhealthy network", member.validator);

                // Update reputation (penalize for being unhealthy)
                let validator_id = member.validator.clone();
                self.committee.update_validator(&validator_id, |val| {
                    val.update_reputation(-5);
                })?;
            }
        }

        if unhealthy_count > 0 {
            log::info!("Health check: {}/{} validators unhealthy",
                unhealthy_count,
                committee.len()
            );
        }

        Ok(())
    }

    /// Distribute rewards for completed epoch
    fn distribute_epoch_rewards(&mut self, epoch: u32) -> ValidatorResult<()> {
        log::debug!("Distributing rewards for epoch {}", epoch);

        let committee = self.committee.current_committee().to_vec();
        let block_number = self.current_block;

        // Record rewards for all committee members
        for member in committee {
            // Record committee participation reward
            let reward = self.reward_calculator.record_reward(
                member.validator.clone(),
                crate::rewards::RewardType::CommitteeParticipation,
                block_number,
                epoch,
            );

            log::trace!("Validator {:?} earned {} for epoch {}", member.validator, reward, epoch);
        }

        Ok(())
    }

    /// Update validator states for new epoch
    fn update_validator_states_for_epoch(&mut self, epoch: u32) -> ValidatorResult<()> {
        let all_validators = self.committee.all_validator_ids();

        for validator_id in all_validators {
            self.committee.update_validator(&validator_id, |validator| {
                validator.last_epoch = epoch;

                // Reset epoch-specific stats
                // (In production, these would be accumulated over time)
            })?;
        }

        Ok(())
    }

    /// Get current committee size
    pub fn get_committee_size(&self) -> usize {
        self.committee.current_committee().len()
    }

    /// Get current epoch
    pub fn get_current_epoch(&self) -> u32 {
        self.current_epoch
    }

    /// Get validator statistics
    pub fn get_validator_stats(&self) -> crate::ValidatorStats {
        let all_validators = self.committee.all_validator_ids();
        let committee = self.committee.current_committee();

        let mut total_stake = 0u128;
        let mut total_reputation = 0u64;
        let mut active_count = 0u32;

        for validator_id in &all_validators {
            if let Some(validator) = self.committee.get_validator(validator_id) {
                total_stake += validator.stake;
                total_reputation += validator.reputation;
                if validator.active {
                    active_count += 1;
                }
            }
        }

        let total_validators = all_validators.len() as u32;

        crate::ValidatorStats {
            total_validators,
            active_validators: active_count,
            committee_size: committee.len() as u32,
            avg_reputation: if total_validators > 0 {
                total_reputation / total_validators as u64
            } else {
                0
            },
            total_stake,
            avg_stake: if total_validators > 0 {
                total_stake / total_validators as u128
            } else {
                0
            },
        }
    }
}

/// Run the validator coordinator in a loop
///
/// This function is called by the FlareChain node to run the validator management worker.
/// It processes blocks and coordinates all validator-related activities.
pub async fn run_coordinator(
    config: CoordinatorConfig,
    genesis_validators: Vec<ValidatorInfo>,
) {
    log::info!("🚀 Starting Validator Management Coordinator");

    let mut coordinator = ValidatorCoordinator::new(config.clone());

    // Initialize with genesis validators
    if let Err(e) = coordinator.initialize(genesis_validators) {
        log::error!("Failed to initialize coordinator: {:?}", e);
        return;
    }

    // Main coordinator loop
    let mut current_block = 0u64;

    loop {
        // In production, this would be triggered by actual block imports
        // For now, we simulate block progression
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Process block
        if let Err(e) = coordinator.process_block(current_block) {
            log::error!("Error processing block #{}: {:?}", current_block, e);
        }

        // Log status periodically
        if current_block % 100 == 0 {
            let stats = coordinator.get_validator_stats();
            log::debug!(
                "Validator stats: epoch={}, validators={}/{}, committee={}, avg_rep={}",
                coordinator.get_current_epoch(),
                stats.active_validators,
                stats.total_validators,
                stats.committee_size,
                stats.avg_reputation
            );
        }

        current_block += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ValidatorId, PeerType};

    #[test]
    fn test_coordinator_creation() {
        let config = CoordinatorConfig::default();
        let coordinator = ValidatorCoordinator::new(config);

        assert_eq!(coordinator.get_current_epoch(), 0);
        assert_eq!(coordinator.get_committee_size(), 0);
    }

    #[test]
    fn test_coordinator_initialization() {
        let config = CoordinatorConfig::default();
        let mut coordinator = ValidatorCoordinator::new(config);

        // Create test validators
        let validators = vec![
            ValidatorInfo::new(
                ValidatorId::from([1u8; 32]),
                64_000_000_000_000_000_000_000,
                PeerType::ValidityNode
            ),
            ValidatorInfo::new(
                ValidatorId::from([2u8; 32]),
                64_000_000_000_000_000_000_000,
                PeerType::ValidityNode
            ),
        ];

        assert!(coordinator.initialize(validators).is_ok());
        assert!(coordinator.get_committee_size() > 0);
    }

    #[test]
    fn test_block_processing() {
        let config = CoordinatorConfig {
            epoch_duration: 10,
            health_check_interval: 5,
            ..Default::default()
        };
        let mut coordinator = ValidatorCoordinator::new(config);

        // Initialize with test validator
        let validators = vec![
            ValidatorInfo::new(
                ValidatorId::from([1u8; 32]),
                64_000_000_000_000_000_000_000,
                PeerType::ValidityNode
            ),
        ];
        coordinator.initialize(validators).unwrap();

        // Process blocks
        assert!(coordinator.process_block(1).is_ok());
        assert_eq!(coordinator.current_block, 1);

        // Test epoch transition
        assert!(coordinator.process_block(10).is_ok());
        assert_eq!(coordinator.get_current_epoch(), 1);
    }
}
