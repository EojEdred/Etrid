//! # Committee Management
//!
//! This module manages the PPFA (Proposing Panel for Attestation) committee,
//! including member selection, rotation, and membership checks.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use codec::{Decode, Encode};

use crate::{
    Balance, CommitteeMember, ValidatorError, ValidatorId, ValidatorInfo, ValidatorResult,
    MAX_COMMITTEE_SIZE, MIN_COMMITTEE_SIZE, MIN_REPUTATION_FOR_COMMITTEE,
};

// ═══════════════════════════════════════════════════════════════════════════════
// COMMITTEE MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manages PPFA committee membership and rotation
#[derive(Debug, Clone)]
pub struct CommitteeManager {
    /// Current committee members
    current_committee: Vec<CommitteeMember>,
    
    /// All validators (pool for selection)
    validator_pool: BTreeMap<ValidatorId, ValidatorInfo>,
    
    /// Current epoch
    current_epoch: u32,
    
    /// Current PPFA index (which validator is proposing)
    ppfa_index: u32,
    
    /// Committee size target
    target_size: u32,
}

impl CommitteeManager {
    /// Create a new committee manager
    pub fn new(target_size: u32) -> Self {
        let actual_size = target_size.max(MIN_COMMITTEE_SIZE).min(MAX_COMMITTEE_SIZE);
        
        Self {
            current_committee: Vec::new(),
            validator_pool: BTreeMap::new(),
            current_epoch: 0,
            ppfa_index: 0,
            target_size: actual_size,
        }
    }

    /// Add a validator to the pool
    pub fn add_validator(&mut self, info: ValidatorInfo) -> ValidatorResult<()> {
        if !info.can_participate() {
            return Err(ValidatorError::Inactive);
        }

        self.validator_pool.insert(info.id.clone(), info);
        Ok(())
    }

    /// Remove a validator from the pool
    pub fn remove_validator(&mut self, validator: &ValidatorId) -> ValidatorResult<()> {
        self.validator_pool
            .remove(validator)
            .ok_or(ValidatorError::NotFound)?;

        // Remove from committee if present
        self.current_committee.retain(|m| &m.validator != validator);

        Ok(())
    }

    /// Get validator info
    pub fn get_validator(&self, validator: &ValidatorId) -> Option<&ValidatorInfo> {
        self.validator_pool.get(validator)
    }

    /// Update validator info
    pub fn update_validator<F>(&mut self, validator: &ValidatorId, update: F) -> ValidatorResult<()>
    where
        F: FnOnce(&mut ValidatorInfo),
    {
        self.validator_pool
            .get_mut(validator)
            .ok_or(ValidatorError::NotFound)
            .map(update)
    }

    /// Check if validator is in current committee
    pub fn is_in_committee(&self, validator: &ValidatorId) -> bool {
        self.current_committee.iter().any(|m| &m.validator == validator)
    }

    /// Get validator's PPFA index (if in committee)
    pub fn get_ppfa_index(&self, validator: &ValidatorId) -> Option<u32> {
        self.current_committee
            .iter()
            .find(|m| &m.validator == validator)
            .map(|m| m.ppfa_index)
    }

    /// Get current committee size
    pub fn committee_size(&self) -> u32 {
        self.current_committee.len() as u32
    }

    /// Get current committee members
    pub fn current_committee(&self) -> &[CommitteeMember] {
        &self.current_committee
    }

    /// Get current PPFA proposer
    pub fn current_proposer(&self) -> Option<&CommitteeMember> {
        self.current_committee
            .get(self.ppfa_index as usize)
    }

    /// Get current PPFA index
    pub fn current_ppfa_index(&self) -> u32 {
        self.ppfa_index
    }

    /// Advance PPFA index to next proposer
    pub fn advance_ppfa_index(&mut self) {
        if !self.current_committee.is_empty() {
            self.ppfa_index = (self.ppfa_index + 1) % (self.current_committee.len() as u32);
        }
    }

    /// Rotate committee (epoch boundary)
    pub fn rotate_committee(&mut self, new_epoch: u32) -> ValidatorResult<()> {
        self.current_epoch = new_epoch;
        self.ppfa_index = 0; // Reset to first proposer

        // Select new committee based on stake and reputation
        let new_committee = self.select_committee()?;
        self.current_committee = new_committee;

        Ok(())
    }

    /// Select committee members (stake-weighted, reputation-filtered)
    fn select_committee(&self) -> ValidatorResult<Vec<CommitteeMember>> {
        // Get eligible validators (can participate, good reputation, validator type)
        let mut eligible: Vec<_> = self
            .validator_pool
            .values()
            .filter(|v| {
                v.can_participate()
                    && v.reputation >= MIN_REPUTATION_FOR_COMMITTEE
                    && v.peer_type.can_be_in_committee()
            })
            .collect();

        if eligible.is_empty() {
            return Err(ValidatorError::CommitteeFull);
        }

        // Sort by stake (descending) then reputation (descending)
        eligible.sort_by(|a, b| {
            b.stake
                .cmp(&a.stake)
                .then_with(|| b.reputation.cmp(&a.reputation))
        });

        // Take top validators up to target size
        let selected = eligible
            .into_iter()
            .take(self.target_size as usize)
            .enumerate()
            .map(|(index, validator)| CommitteeMember {
                validator: validator.id.clone(),
                stake: validator.stake,
                ppfa_index: index as u32,
                joined_epoch: self.current_epoch,
            })
            .collect();

        Ok(selected)
    }

    /// Get total stake in committee
    pub fn total_committee_stake(&self) -> Balance {
        self.current_committee.iter().map(|m| m.stake).sum()
    }

    /// Get total validators in pool
    pub fn total_validators(&self) -> usize {
        self.validator_pool.len()
    }

    /// Get active validators count
    pub fn active_validators(&self) -> usize {
        self.validator_pool.values().filter(|v| v.active).count()
    }

    /// Get eligible validators count
    pub fn eligible_validators(&self) -> usize {
        self.validator_pool
            .values()
            .filter(|v| {
                v.can_participate()
                    && v.reputation >= MIN_REPUTATION_FOR_COMMITTEE
                    && v.peer_type.can_be_in_committee()
            })
            .count()
    }

    /// Get all validator IDs in the pool
    pub fn all_validator_ids(&self) -> Vec<ValidatorId> {
        self.validator_pool.keys().cloned().collect()
    }

    /// Force add a validator to committee (for genesis/testing)
    pub fn force_add_to_committee(&mut self, validator: ValidatorId, stake: Balance) -> ValidatorResult<()> {
        if self.current_committee.len() >= MAX_COMMITTEE_SIZE as usize {
            return Err(ValidatorError::CommitteeFull);
        }

        let ppfa_index = self.current_committee.len() as u32;
        let member = CommitteeMember {
            validator,
            stake,
            ppfa_index,
            joined_epoch: self.current_epoch,
        };

        self.current_committee.push(member);
        Ok(())
    }

    /// Clear committee (for testing)
    pub fn clear_committee(&mut self) {
        self.current_committee.clear();
        self.ppfa_index = 0;
    }
}

impl Default for CommitteeManager {
    fn default() -> Self {
        Self::new(21) // Default PPFA size
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMITTEE SELECTION STRATEGY
// ═══════════════════════════════════════════════════════════════════════════════

/// Strategy for selecting committee members
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionStrategy {
    /// Pure stake-weighted (highest stake wins)
    StakeWeighted,
    
    /// Reputation-weighted (consider both stake and reputation)
    ReputationWeighted,
    
    /// Hybrid (stake primary, reputation as tiebreaker)
    Hybrid,
}

impl SelectionStrategy {
    /// Calculate selection score for a validator
    pub fn calculate_score(&self, validator: &ValidatorInfo) -> u128 {
        match self {
            SelectionStrategy::StakeWeighted => validator.stake,
            
            SelectionStrategy::ReputationWeighted => {
                // Score = stake × (reputation / 100)
                (validator.stake as u128 * validator.reputation as u128) / 100
            }
            
            SelectionStrategy::Hybrid => {
                // Primary: stake, Secondary: reputation bonus
                let base = validator.stake as u128;
                let bonus = (base * validator.reputation as u128) / 1000; // 10% max bonus
                base + bonus
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PeerType;

    fn create_test_validator(id: u8, stake: Balance, reputation: u64) -> ValidatorInfo {
        let validator_id = ValidatorId::from([id; 32]);
        let mut info = ValidatorInfo::new(validator_id, stake, PeerType::ValidityNode);
        info.reputation = reputation;
        info
    }

    #[test]
    fn test_committee_manager_creation() {
        let manager = CommitteeManager::new(21);
        assert_eq!(manager.committee_size(), 0);
        assert_eq!(manager.target_size, 21);
    }

    #[test]
    fn test_add_validator() {
        let mut manager = CommitteeManager::new(21);
        let validator = create_test_validator(1, 100_000, 100);
        
        assert!(manager.add_validator(validator.clone()).is_ok());
        assert_eq!(manager.total_validators(), 1);
        assert!(manager.get_validator(&validator.id).is_some());
    }

    #[test]
    fn test_remove_validator() {
        let mut manager = CommitteeManager::new(21);
        let validator = create_test_validator(1, 100_000, 100);
        let id = validator.id.clone();
        
        manager.add_validator(validator).unwrap();
        assert!(manager.remove_validator(&id).is_ok());
        assert_eq!(manager.total_validators(), 0);
    }

    #[test]
    fn test_committee_membership() {
        let mut manager = CommitteeManager::new(21);
        let validator = create_test_validator(1, 100_000, 100);
        let id = validator.id.clone();
        
        manager.add_validator(validator).unwrap();
        assert!(!manager.is_in_committee(&id));
        
        // Rotate to select committee
        manager.rotate_committee(1).unwrap();
        assert!(manager.is_in_committee(&id));
    }

    #[test]
    fn test_committee_rotation() {
        let mut manager = CommitteeManager::new(3);
        
        // Add 5 validators with different stakes
        for i in 0..5 {
            let stake = (5 - i) as Balance * 10_000; // Descending stakes
            let validator = create_test_validator(i, stake, 100);
            manager.add_validator(validator).unwrap();
        }
        
        // Rotate - should select top 3 by stake
        manager.rotate_committee(1).unwrap();
        assert_eq!(manager.committee_size(), 3);
        
        // Verify top stakers are selected
        let committee = manager.current_committee();
        assert_eq!(committee[0].stake, 50_000); // Highest stake
        assert_eq!(committee[1].stake, 40_000);
        assert_eq!(committee[2].stake, 30_000);
    }

    #[test]
    fn test_ppfa_index_advancement() {
        let mut manager = CommitteeManager::new(3);
        
        for i in 0..3 {
            let validator = create_test_validator(i, 10_000, 100);
            manager.add_validator(validator).unwrap();
        }
        
        manager.rotate_committee(1).unwrap();
        
        assert_eq!(manager.current_ppfa_index(), 0);
        manager.advance_ppfa_index();
        assert_eq!(manager.current_ppfa_index(), 1);
        manager.advance_ppfa_index();
        assert_eq!(manager.current_ppfa_index(), 2);
        manager.advance_ppfa_index();
        assert_eq!(manager.current_ppfa_index(), 0); // Wraps around
    }

    #[test]
    fn test_current_proposer() {
        let mut manager = CommitteeManager::new(3);
        
        for i in 0..3 {
            let validator = create_test_validator(i, 10_000, 100);
            manager.add_validator(validator).unwrap();
        }
        
        manager.rotate_committee(1).unwrap();
        
        let proposer = manager.current_proposer();
        assert!(proposer.is_some());
        assert_eq!(proposer.unwrap().ppfa_index, 0);
    }

    #[test]
    fn test_reputation_filtering() {
        let mut manager = CommitteeManager::new(3);
        
        // Add validators with different reputations
        let good = create_test_validator(1, 10_000, 80); // Above threshold
        let bad = create_test_validator(2, 10_000, 30);  // Below threshold
        
        manager.add_validator(good).unwrap();
        manager.add_validator(bad).unwrap();
        
        manager.rotate_committee(1).unwrap();
        
        // Only good validator should be in committee
        assert_eq!(manager.committee_size(), 1);
        assert!(manager.is_in_committee(&ValidatorId::from([1u8; 32])));
        assert!(!manager.is_in_committee(&ValidatorId::from([2u8; 32])));
    }

    #[test]
    fn test_selection_strategy_stake_weighted() {
        let strategy = SelectionStrategy::StakeWeighted;
        let validator = create_test_validator(1, 100_000, 50);
        
        let score = strategy.calculate_score(&validator);
        assert_eq!(score, 100_000);
    }

    #[test]
    fn test_selection_strategy_reputation_weighted() {
        let strategy = SelectionStrategy::ReputationWeighted;
        let validator = create_test_validator(1, 100_000, 50);
        
        let score = strategy.calculate_score(&validator);
        assert_eq!(score, 50_000); // 100_000 * 50 / 100
    }

    #[test]
    fn test_selection_strategy_hybrid() {
        let strategy = SelectionStrategy::Hybrid;
        let validator = create_test_validator(1, 100_000, 50);
        
        let score = strategy.calculate_score(&validator);
        assert_eq!(score, 105_000); // 100_000 + (100_000 * 50 / 1000)
    }

    #[test]
    fn test_total_committee_stake() {
        let mut manager = CommitteeManager::new(3);
        
        for i in 0..3 {
            let validator = create_test_validator(i, 10_000, 100);
            manager.add_validator(validator).unwrap();
        }
        
        manager.rotate_committee(1).unwrap();
        assert_eq!(manager.total_committee_stake(), 30_000);
    }
}
