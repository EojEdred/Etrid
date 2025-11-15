//! # Committee Management
//!
//! Manages collator committee lifecycle, including:
//! - Committee registration and updates
//! - Stake management
//! - Slashing for misbehavior
//! - Session transitions

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::{
    CollatorId, CollatorCommittee, ParaId, BlockNumber, Balance,
    AsfError, AsfResult,
};

/// Collator session information
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CollatorSession {
    /// Session index
    pub session_index: u64,
    /// Session start block
    pub start_block: BlockNumber,
    /// Session end block (estimated)
    pub end_block: BlockNumber,
    /// Active committee for this session
    pub committee: CollatorCommittee,
}

impl CollatorSession {
    /// Create new session
    pub fn new(
        session_index: u64,
        start_block: BlockNumber,
        session_length: BlockNumber,
        committee: CollatorCommittee,
    ) -> Self {
        Self {
            session_index,
            start_block,
            end_block: start_block + session_length,
            committee,
        }
    }

    /// Check if session is active at block
    pub fn is_active_at(&self, block_number: BlockNumber) -> bool {
        block_number >= self.start_block && block_number < self.end_block
    }

    /// Check if session should end
    pub fn should_end(&self, block_number: BlockNumber) -> bool {
        block_number >= self.end_block
    }
}

/// Collator stake information
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CollatorStake {
    /// Collator ID
    pub collator: CollatorId,
    /// Staked amount
    pub stake: Balance,
    /// Locked (cannot be withdrawn)
    pub locked: bool,
    /// Slash history
    pub slashed: Balance,
}

impl CollatorStake {
    /// Create new stake record
    pub fn new(collator: CollatorId, stake: Balance) -> Self {
        Self {
            collator,
            stake,
            locked: false,
            slashed: 0,
        }
    }

    /// Apply slash
    pub fn slash(&mut self, amount: Balance) -> Balance {
        let slashed = amount.min(self.stake);
        self.stake -= slashed;
        self.slashed += slashed;
        slashed
    }

    /// Check if has minimum stake
    pub fn meets_minimum(&self, minimum: Balance) -> bool {
        self.stake >= minimum
    }
}

/// Committee change request
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum CommitteeChange {
    /// Add new collator
    AddCollator {
        collator: CollatorId,
        stake: Balance,
    },
    /// Remove collator
    RemoveCollator {
        collator: CollatorId,
    },
    /// Update collator stake
    UpdateStake {
        collator: CollatorId,
        new_stake: Balance,
    },
}

/// Session manager for collator committees
pub struct SessionManager {
    /// Parachain ID
    para_id: ParaId,
    /// Current session
    current_session: Option<CollatorSession>,
    /// Next session (being prepared)
    next_session: Option<CollatorSession>,
    /// Session length in blocks
    session_length: BlockNumber,
    /// Minimum collators required
    min_collators: u32,
    /// Maximum collators allowed
    max_collators: u32,
    /// Minimum stake required
    min_stake: Balance,
}

impl SessionManager {
    /// Create new session manager
    pub fn new(
        para_id: ParaId,
        session_length: BlockNumber,
        min_collators: u32,
        max_collators: u32,
        min_stake: Balance,
    ) -> Self {
        Self {
            para_id,
            current_session: None,
            next_session: None,
            session_length,
            min_collators,
            max_collators,
            min_stake,
        }
    }

    /// Initialize first session
    pub fn initialize_session(
        &mut self,
        start_block: BlockNumber,
        initial_collators: Vec<(CollatorId, Balance)>,
    ) -> AsfResult<()> {
        if initial_collators.len() < self.min_collators as usize {
            return Err(AsfError::InsufficientVotes {
                got: initial_collators.len() as u32,
                need: self.min_collators,
            });
        }

        let mut committee = CollatorCommittee::new(
            self.para_id,
            self.min_collators,
            self.max_collators,
        );

        for (collator, stake) in initial_collators {
            if stake < self.min_stake {
                return Err(AsfError::InsufficientStake {
                    got: stake,
                    need: self.min_stake,
                });
            }
            committee.add_collator(collator, stake)?;
        }

        let session = CollatorSession::new(
            0,
            start_block,
            self.session_length,
            committee,
        );

        self.current_session = Some(session);
        Ok(())
    }

    /// Prepare next session
    pub fn prepare_next_session(
        &mut self,
        changes: Vec<CommitteeChange>,
    ) -> AsfResult<()> {
        let current = self.current_session.as_ref()
            .ok_or(AsfError::InvalidVote("No current session"))?;

        let mut new_committee = current.committee.clone();

        // Apply changes
        for change in changes {
            match change {
                CommitteeChange::AddCollator { collator, stake } => {
                    if stake < self.min_stake {
                        return Err(AsfError::InsufficientStake {
                            got: stake,
                            need: self.min_stake,
                        });
                    }
                    new_committee.add_collator(collator, stake)?;
                }
                CommitteeChange::RemoveCollator { collator } => {
                    // Find collator stake (simplified - would need stake map in production)
                    let stake = 0; // Placeholder
                    new_committee.remove_collator(&collator, stake)?;
                }
                CommitteeChange::UpdateStake { collator, new_stake } => {
                    // Remove and re-add with new stake (simplified)
                    let old_stake = 0; // Placeholder
                    new_committee.remove_collator(&collator, old_stake)?;
                    new_committee.add_collator(collator, new_stake)?;
                }
            }
        }

        // Validate new committee
        if !new_committee.is_valid() {
            return Err(AsfError::InsufficientVotes {
                got: new_committee.collators.len() as u32,
                need: self.min_collators,
            });
        }

        let next_session = CollatorSession::new(
            current.session_index + 1,
            current.end_block,
            self.session_length,
            new_committee,
        );

        self.next_session = Some(next_session);
        Ok(())
    }

    /// Rotate to next session
    pub fn rotate_session(&mut self, block_number: BlockNumber) -> AsfResult<()> {
        let current = self.current_session.as_ref()
            .ok_or(AsfError::InvalidVote("No current session"))?;

        if !current.should_end(block_number) {
            return Err(AsfError::InvalidPhaseTransition {
                from: crate::ConsensusPhase::Prepare,
                to: crate::ConsensusPhase::Prepare,
            });
        }

        let next = self.next_session.take()
            .ok_or(AsfError::InvalidVote("No next session prepared"))?;

        self.current_session = Some(next);
        Ok(())
    }

    /// Get current session
    pub fn current_session(&self) -> Option<&CollatorSession> {
        self.current_session.as_ref()
    }

    /// Get current committee
    pub fn current_committee(&self) -> Option<&CollatorCommittee> {
        self.current_session.as_ref().map(|s| &s.committee)
    }

    /// Check if should rotate at block
    pub fn should_rotate(&self, block_number: BlockNumber) -> bool {
        self.current_session
            .as_ref()
            .map(|s| s.should_end(block_number))
            .unwrap_or(false)
    }
}

/// Stake manager for collator stakes
pub struct StakeManager {
    /// Collator stakes (collator_id -> stake)
    stakes: BTreeMap<CollatorId, CollatorStake>,
    /// Minimum stake required
    min_stake: Balance,
    /// Total staked
    total_staked: Balance,
}

impl StakeManager {
    /// Create new stake manager
    pub fn new(min_stake: Balance) -> Self {
        Self {
            stakes: BTreeMap::new(),
            min_stake,
            total_staked: 0,
        }
    }

    /// Add or update stake
    pub fn stake(
        &mut self,
        collator: CollatorId,
        amount: Balance,
    ) -> AsfResult<()> {
        if amount < self.min_stake {
            return Err(AsfError::InsufficientStake {
                got: amount,
                need: self.min_stake,
            });
        }

        let stake = self.stakes.entry(collator.clone())
            .or_insert_with(|| CollatorStake::new(collator, 0));

        stake.stake += amount;
        self.total_staked += amount;
        Ok(())
    }

    /// Unstake (if not locked)
    pub fn unstake(
        &mut self,
        collator: &CollatorId,
        amount: Balance,
    ) -> AsfResult<Balance> {
        let stake = self.stakes.get_mut(collator)
            .ok_or(AsfError::BlockNotFound)?;

        if stake.locked {
            return Err(AsfError::InvalidVote("Stake is locked"));
        }

        let unstaked = amount.min(stake.stake);
        stake.stake -= unstaked;
        self.total_staked -= unstaked;

        Ok(unstaked)
    }

    /// Slash collator stake
    pub fn slash(
        &mut self,
        collator: &CollatorId,
        amount: Balance,
    ) -> AsfResult<Balance> {
        let stake = self.stakes.get_mut(collator)
            .ok_or(AsfError::BlockNotFound)?;

        let slashed = stake.slash(amount);
        self.total_staked -= slashed;

        Ok(slashed)
    }

    /// Lock stake (prevent unstaking)
    pub fn lock_stake(&mut self, collator: &CollatorId) -> AsfResult<()> {
        let stake = self.stakes.get_mut(collator)
            .ok_or(AsfError::BlockNotFound)?;

        stake.locked = true;
        Ok(())
    }

    /// Unlock stake
    pub fn unlock_stake(&mut self, collator: &CollatorId) -> AsfResult<()> {
        let stake = self.stakes.get_mut(collator)
            .ok_or(AsfError::BlockNotFound)?;

        stake.locked = false;
        Ok(())
    }

    /// Get collator stake
    pub fn get_stake(&self, collator: &CollatorId) -> Option<&CollatorStake> {
        self.stakes.get(collator)
    }

    /// Check if collator meets minimum stake
    pub fn meets_minimum(&self, collator: &CollatorId) -> bool {
        self.stakes
            .get(collator)
            .map(|s| s.meets_minimum(self.min_stake))
            .unwrap_or(false)
    }

    /// Get total staked
    pub fn total_staked(&self) -> Balance {
        self.total_staked
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    #[test]
    fn test_collator_session() {
        let committee = CollatorCommittee::new(1000, 7, 11);
        let session = CollatorSession::new(0, 100, 600, committee);

        assert!(session.is_active_at(100));
        assert!(session.is_active_at(500));
        assert!(!session.is_active_at(700));
        assert!(session.should_end(700));
        assert!(!session.should_end(500));
    }

    #[test]
    fn test_session_manager_initialization() {
        let mut manager = SessionManager::new(1000, 600, 7, 11, 1_000_000);

        let collators: Vec<_> = (0..7)
            .map(|i| {
                let collator = AccountId32::new([i as u8; 32]);
                (collator, 1_000_000)
            })
            .collect();

        manager.initialize_session(100, collators).unwrap();

        assert!(manager.current_session().is_some());
        assert_eq!(manager.current_committee().unwrap().collators.len(), 7);
    }

    #[test]
    fn test_session_rotation() {
        let mut manager = SessionManager::new(1000, 600, 7, 11, 1_000_000);

        let collators: Vec<_> = (0..7)
            .map(|i| {
                let collator = AccountId32::new([i as u8; 32]);
                (collator, 1_000_000)
            })
            .collect();

        manager.initialize_session(100, collators).unwrap();

        // Prepare next session with no changes
        manager.prepare_next_session(Vec::new()).unwrap();

        // Should not rotate before end
        assert!(manager.rotate_session(500).is_err());

        // Should rotate at end
        manager.rotate_session(700).unwrap();
        assert_eq!(manager.current_session().unwrap().session_index, 1);
    }

    #[test]
    fn test_stake_manager() {
        let mut manager = StakeManager::new(1_000_000);
        let collator = AccountId32::new([1u8; 32]);

        // Stake
        manager.stake(collator.clone(), 5_000_000).unwrap();
        assert_eq!(manager.get_stake(&collator).unwrap().stake, 5_000_000);
        assert_eq!(manager.total_staked(), 5_000_000);

        // Unstake partial
        let unstaked = manager.unstake(&collator, 2_000_000).unwrap();
        assert_eq!(unstaked, 2_000_000);
        assert_eq!(manager.get_stake(&collator).unwrap().stake, 3_000_000);
    }

    #[test]
    fn test_stake_slashing() {
        let mut manager = StakeManager::new(1_000_000);
        let collator = AccountId32::new([1u8; 32]);

        manager.stake(collator.clone(), 5_000_000).unwrap();

        // Slash
        let slashed = manager.slash(&collator, 1_000_000).unwrap();
        assert_eq!(slashed, 1_000_000);
        assert_eq!(manager.get_stake(&collator).unwrap().stake, 4_000_000);
        assert_eq!(manager.get_stake(&collator).unwrap().slashed, 1_000_000);
    }

    #[test]
    fn test_stake_locking() {
        let mut manager = StakeManager::new(1_000_000);
        let collator = AccountId32::new([1u8; 32]);

        manager.stake(collator.clone(), 5_000_000).unwrap();

        // Lock stake
        manager.lock_stake(&collator).unwrap();

        // Cannot unstake when locked
        assert!(manager.unstake(&collator, 1_000_000).is_err());

        // Unlock and try again
        manager.unlock_stake(&collator).unwrap();
        assert!(manager.unstake(&collator, 1_000_000).is_ok());
    }
}
