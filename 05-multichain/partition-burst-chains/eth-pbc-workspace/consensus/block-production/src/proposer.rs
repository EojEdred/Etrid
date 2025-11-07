//! # PPFA Proposer Selection
//!
//! This module handles the selection of block proposers using the
//! PPFA (Proposing Panel for Attestation) system.

use alloc::vec::Vec;

use crate::{
    BlockNumber, BlockProductionError, BlockProductionResult, CommitteeManager, SlotInfo,
    ValidatorId,
};

// ═══════════════════════════════════════════════════════════════════════════════
// PROPOSER SELECTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Handles PPFA proposer selection
#[derive(Debug, Clone)]
pub struct ProposerSelector {
    /// Committee manager for PPFA panel
    committee: CommitteeManager,
    
    /// Current slot number
    current_slot: u64,
    
    /// Last block number
    last_block: BlockNumber,
}

impl ProposerSelector {
    /// Create a new proposer selector
    pub fn new(committee: CommitteeManager) -> Self {
        Self {
            committee,
            current_slot: 0,
            last_block: 0,
        }
    }

    /// Check if validator is the current proposer
    pub fn is_proposer(&self, validator: &ValidatorId) -> bool {
        self.committee
            .current_proposer()
            .map(|p| &p.validator == validator)
            .unwrap_or(false)
    }

    /// Get current proposer
    pub fn current_proposer(&self) -> BlockProductionResult<ValidatorId> {
        self.committee
            .current_proposer()
            .map(|p| p.validator.clone())
            .ok_or(BlockProductionError::NoCommittee)
    }

    /// Get current PPFA index
    pub fn current_ppfa_index(&self) -> u32 {
        self.committee.current_ppfa_index()
    }

    /// Get slot info for current slot
    pub fn current_slot_info(&self, slot_duration: u64) -> BlockProductionResult<SlotInfo> {
        let proposer = self.current_proposer()?;
        let ppfa_index = self.current_ppfa_index();
        
        Ok(SlotInfo {
            slot: self.current_slot,
            duration: slot_duration,
            ppfa_index,
            proposer,
        })
    }

    /// Advance to next proposer (called after each block)
    pub fn advance(&mut self, block_number: BlockNumber) {
        self.committee.advance_ppfa_index();
        self.current_slot += 1;
        self.last_block = block_number;
    }

    /// Force set PPFA index (for recovery/testing)
    pub fn set_ppfa_index(&mut self, index: u32) {
        // Note: This would need to be implemented in CommitteeManager
        // For now, we advance to the desired index
        let current = self.current_ppfa_index();
        let committee_size = self.committee.committee_size();
        
        if committee_size == 0 {
            return;
        }
        
        let target = index % committee_size;
        let mut current_idx = current % committee_size;
        
        while current_idx != target {
            self.committee.advance_ppfa_index();
            current_idx = (current_idx + 1) % committee_size;
        }
    }

    /// Get proposer for specific slot
    pub fn proposer_for_slot(&self, slot: u64) -> BlockProductionResult<ValidatorId> {
        let committee_size = self.committee.committee_size();
        if committee_size == 0 {
            return Err(BlockProductionError::NoCommittee);
        }

        // Calculate PPFA index for this slot
        let ppfa_index = (slot % committee_size as u64) as u32;
        
        // Get committee member at this index
        self.committee
            .current_committee()
            .get(ppfa_index as usize)
            .map(|member| member.validator.clone())
            .ok_or(BlockProductionError::NoCommittee)
    }

    /// Get all proposers in order
    pub fn all_proposers(&self) -> Vec<ValidatorId> {
        self.committee
            .current_committee()
            .iter()
            .map(|m| m.validator.clone())
            .collect()
    }

    /// Get committee size
    pub fn committee_size(&self) -> u32 {
        self.committee.committee_size()
    }

    /// Update committee (on epoch boundary)
    pub fn rotate_committee(&mut self, epoch: u32) -> BlockProductionResult<()> {
        self.committee
            .rotate_committee(epoch)
            .map_err(|_| BlockProductionError::NoCommittee)
    }

    /// Get current slot
    pub fn current_slot(&self) -> u64 {
        self.current_slot
    }

    /// Get last block number
    pub fn last_block(&self) -> BlockNumber {
        self.last_block
    }

    /// Update current slot
    pub fn update_slot(&mut self, slot: u64) {
        self.current_slot = slot;
    }
}

impl Default for ProposerSelector {
    fn default() -> Self {
        Self::new(CommitteeManager::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPOSER SCHEDULE
// ═══════════════════════════════════════════════════════════════════════════════

/// Represents a schedule of proposers for upcoming slots
#[derive(Debug, Clone)]
pub struct ProposerSchedule {
    /// Schedule of (slot, validator) pairs
    schedule: Vec<(u64, ValidatorId)>,
    
    /// Starting slot
    start_slot: u64,
}

impl ProposerSchedule {
    /// Create a new proposer schedule
    pub fn new(start_slot: u64) -> Self {
        Self {
            schedule: Vec::new(),
            start_slot,
        }
    }

    /// Add a proposer for a slot
    pub fn add(&mut self, slot: u64, validator: ValidatorId) {
        self.schedule.push((slot, validator));
    }

    /// Get proposer for a slot
    pub fn get_proposer(&self, slot: u64) -> Option<&ValidatorId> {
        self.schedule
            .iter()
            .find(|(s, _)| *s == slot)
            .map(|(_, v)| v)
    }

    /// Generate schedule from selector
    pub fn generate(
        selector: &ProposerSelector,
        start_slot: u64,
        num_slots: usize,
    ) -> BlockProductionResult<Self> {
        let mut schedule = Self::new(start_slot);
        
        for i in 0..num_slots {
            let slot = start_slot + i as u64;
            let proposer = selector.proposer_for_slot(slot)?;
            schedule.add(slot, proposer);
        }
        
        Ok(schedule)
    }

    /// Get schedule length
    pub fn len(&self) -> usize {
        self.schedule.len()
    }

    /// Check if schedule is empty
    pub fn is_empty(&self) -> bool {
        self.schedule.is_empty()
    }

    /// Get all scheduled slots
    pub fn slots(&self) -> Vec<u64> {
        self.schedule.iter().map(|(s, _)| *s).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator_management::{PeerType, ValidatorInfo};

    fn create_test_validator(id: u8, stake: u128) -> ValidatorInfo {
        ValidatorInfo::new(
            ValidatorId::from([id; 32]),
            stake,
            PeerType::ValidityNode,
        )
    }

    fn setup_committee() -> CommitteeManager {
        let mut committee = CommitteeManager::new(3);
        
        // Add 3 validators
        for i in 0..3 {
            let validator = create_test_validator(i, 100_000);
            committee.add_validator(validator).unwrap();
        }
        
        // Rotate to initialize committee
        committee.rotate_committee(1).unwrap();
        
        committee
    }

    #[test]
    fn test_proposer_selector_creation() {
        let committee = setup_committee();
        let selector = ProposerSelector::new(committee);
        
        assert_eq!(selector.committee_size(), 3);
        assert_eq!(selector.current_slot(), 0);
    }

    #[test]
    fn test_is_proposer() {
        let committee = setup_committee();
        let mut selector = ProposerSelector::new(committee);
        
        let proposer = selector.current_proposer().unwrap();
        assert!(selector.is_proposer(&proposer));
        
        // Check non-proposer
        let non_proposer = ValidatorId::from([99u8; 32]);
        assert!(!selector.is_proposer(&non_proposer));
    }

    #[test]
    fn test_advance_proposer() {
        let committee = setup_committee();
        let mut selector = ProposerSelector::new(committee);
        
        let first_proposer = selector.current_proposer().unwrap();
        assert_eq!(selector.current_ppfa_index(), 0);
        
        selector.advance(1);
        let second_proposer = selector.current_proposer().unwrap();
        assert_eq!(selector.current_ppfa_index(), 1);
        
        assert_ne!(first_proposer, second_proposer);
    }

    #[test]
    fn test_proposer_rotation() {
        let committee = setup_committee();
        let mut selector = ProposerSelector::new(committee);
        
        let mut proposers = Vec::new();
        
        // Collect 6 proposers (2 full rotations of 3)
        for i in 0..6 {
            proposers.push(selector.current_proposer().unwrap());
            selector.advance(i as u64);
        }
        
        // First and fourth should be the same (full rotation)
        assert_eq!(proposers[0], proposers[3]);
        assert_eq!(proposers[1], proposers[4]);
        assert_eq!(proposers[2], proposers[5]);
    }

    #[test]
    fn test_proposer_for_slot() {
        let committee = setup_committee();
        let selector = ProposerSelector::new(committee);
        
        let proposer_slot_0 = selector.proposer_for_slot(0).unwrap();
        let proposer_slot_3 = selector.proposer_for_slot(3).unwrap();
        
        // Slot 0 and 3 should have same proposer (committee size = 3)
        assert_eq!(proposer_slot_0, proposer_slot_3);
    }

    #[test]
    fn test_all_proposers() {
        let committee = setup_committee();
        let selector = ProposerSelector::new(committee);
        
        let all = selector.all_proposers();
        assert_eq!(all.len(), 3);
        
        // All should be unique
        let unique: alloc::collections::BTreeSet<_> = all.iter().collect();
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn test_proposer_schedule_generation() {
        let committee = setup_committee();
        let selector = ProposerSelector::new(committee);
        
        let schedule = ProposerSchedule::generate(&selector, 0, 6).unwrap();
        
        assert_eq!(schedule.len(), 6);
        assert_eq!(schedule.slots(), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_proposer_schedule_lookup() {
        let committee = setup_committee();
        let selector = ProposerSelector::new(committee);
        
        let schedule = ProposerSchedule::generate(&selector, 0, 3).unwrap();
        
        let proposer_0 = schedule.get_proposer(0).unwrap();
        let proposer_1 = schedule.get_proposer(1).unwrap();
        let proposer_2 = schedule.get_proposer(2).unwrap();
        
        // All should be different (committee size = 3)
        assert_ne!(proposer_0, proposer_1);
        assert_ne!(proposer_1, proposer_2);
        assert_ne!(proposer_0, proposer_2);
    }

    #[test]
    fn test_empty_committee() {
        let committee = CommitteeManager::new(3);
        let mut selector = ProposerSelector::new(committee);
        
        assert!(selector.current_proposer().is_err());
        assert!(selector.proposer_for_slot(0).is_err());
    }

    #[test]
    fn test_set_ppfa_index() {
        let committee = setup_committee();
        let mut selector = ProposerSelector::new(committee);
        
        selector.set_ppfa_index(2);
        assert_eq!(selector.current_ppfa_index(), 2);
        
        selector.set_ppfa_index(5); // Wraps to 2 (5 % 3)
        assert_eq!(selector.current_ppfa_index(), 2);
    }

    #[test]
    fn test_update_slot() {
        let committee = setup_committee();
        let mut selector = ProposerSelector::new(committee);
        
        assert_eq!(selector.current_slot(), 0);
        
        selector.update_slot(100);
        assert_eq!(selector.current_slot(), 100);
    }
}
