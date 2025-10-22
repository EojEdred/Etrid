//! # HotStuff Consensus Protocol
//!
//! This module implements the 4-phase HotStuff Byzantine consensus protocol:
//! Prepare → PreCommit → Commit → Decide
//!
//! Reference: "HotStuff: BFT Consensus in the Lens of Blockchain" (2018)
//! Adapted for Ëtrid's Ascending Scale of Finality

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::{
    AsfError, AsfResult, Balance, BlockNumber, CertificateCollection, CertificateGenerator,
    ConsensusPhase, FinalityLevel, Hash, ValidityCertificate, Vote,
    VoteCollection,
};

// ═══════════════════════════════════════════════════════════════════════════════
// HOTSTUFF STATE MACHINE
// ═══════════════════════════════════════════════════════════════════════════════

/// HotStuff consensus state for a single block
#[derive(Debug, Clone)]
pub struct HotStuffState {
    /// Block being processed
    pub block_hash: Hash,
    
    /// Block number
    pub block_number: BlockNumber,
    
    /// Current consensus phase
    pub current_phase: ConsensusPhase,
    
    /// Votes collected for each phase
    pub prepare_votes: VoteCollection,
    pub precommit_votes: VoteCollection,
    pub commit_votes: VoteCollection,
    
    /// Certificates generated for each phase
    pub certificates: CertificateCollection,
    
    /// Whether this block has reached finality
    pub finalized: bool,
    
    /// Current epoch
    pub epoch: u32,
}

impl HotStuffState {
    /// Create a new HotStuff state for a block
    pub fn new(block_hash: Hash, block_number: BlockNumber, epoch: u32) -> Self {
        Self {
            block_hash,
            block_number,
            current_phase: ConsensusPhase::Prepare,
            prepare_votes: VoteCollection::new(),
            precommit_votes: VoteCollection::new(),
            commit_votes: VoteCollection::new(),
            certificates: CertificateCollection::new(),
            finalized: false,
            epoch,
        }
    }

    /// Get votes for the current phase
    pub fn current_votes(&mut self) -> &mut VoteCollection {
        match self.current_phase {
            ConsensusPhase::Prepare => &mut self.prepare_votes,
            ConsensusPhase::PreCommit => &mut self.precommit_votes,
            ConsensusPhase::Commit => &mut self.commit_votes,
            ConsensusPhase::Decide => &mut self.commit_votes, // Decide doesn't collect votes
        }
    }

    /// Advance to the next phase
    pub fn advance_phase(&mut self) -> AsfResult<()> {
        if let Some(next_phase) = self.current_phase.next() {
            self.current_phase = next_phase;
            Ok(())
        } else {
            // Already in Decide phase
            self.finalized = true;
            Ok(())
        }
    }

    /// Reset to Prepare phase (for view change)
    pub fn reset(&mut self) {
        self.current_phase = ConsensusPhase::Prepare;
        self.prepare_votes.clear();
        self.precommit_votes.clear();
        self.commit_votes.clear();
        self.finalized = false;
    }

    /// Get finality level based on certificates
    pub fn finality_level(&self) -> FinalityLevel {
        self.certificates.finality_level()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HOTSTUFF ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// HotStuff consensus engine managing multiple blocks
pub struct HotStuffEngine {
    /// States for all blocks currently in consensus
    block_states: BTreeMap<Hash, HotStuffState>,
    
    /// Certificate generator
    cert_generator: CertificateGenerator,
    
    /// Total validators in committee
    total_validators: u32,
    
    /// Total stake in committee
    total_stake: Balance,
    
    /// Current epoch
    current_epoch: u32,
}

impl HotStuffEngine {
    /// Create a new HotStuff engine
    pub fn new(total_validators: u32, total_stake: Balance, epoch: u32) -> Self {
        Self {
            block_states: BTreeMap::new(),
            cert_generator: CertificateGenerator::new(total_validators, total_stake, epoch),
            total_validators,
            total_stake,
            current_epoch: epoch,
        }
    }

    /// Start consensus for a new block
    pub fn start_consensus(
        &mut self,
        block_hash: Hash,
        block_number: BlockNumber,
    ) -> AsfResult<()> {
        if self.block_states.contains_key(&block_hash) {
            return Err(AsfError::InvalidVote("Block already in consensus"));
        }

        let state = HotStuffState::new(block_hash, block_number, self.current_epoch);
        self.block_states.insert(block_hash, state);

        Ok(())
    }

    /// Process a vote for a block
    pub fn process_vote(&mut self, vote: Vote) -> AsfResult<Option<ValidityCertificate>> {
        // Validate vote
        vote.validate(self.current_epoch)?;

        // Get or create state for this block
        let state = self
            .block_states
            .get_mut(&vote.block_hash)
            .ok_or(AsfError::BlockNotFound)?;

        // Ensure vote is for current phase
        if vote.phase != state.current_phase {
            return Err(AsfError::InvalidPhaseTransition {
                from: state.current_phase,
                to: vote.phase,
            });
        }

        // Add vote to current phase collection
        state.current_votes().add_vote(vote)?;

        // Check if we have enough votes to generate certificate
        let votes = state.current_votes();
        if votes.meets_threshold(self.total_validators)
            && votes.meets_stake_threshold(self.total_stake)
        {
            // Generate certificate
            let all_votes = votes.votes().to_vec();
            let cert = self.cert_generator.try_generate(
                &all_votes,
                all_votes[0].validator.clone(), // Use first voter as issuer (simplified)
                all_votes[0].stake_weight,
                all_votes[0].timestamp,
            )?;

            // Add certificate to collection
            state.certificates.add_certificate(cert.clone())?;

            // Advance to next phase
            state.advance_phase()?;

            return Ok(Some(cert));
        }

        Ok(None)
    }

    /// Process a certificate from another validator
    pub fn process_certificate(
        &mut self,
        cert: ValidityCertificate,
    ) -> AsfResult<()> {
        // Validate certificate
        cert.validate(self.total_validators, self.total_stake, self.current_epoch)?;

        // Get state for this block
        let state = self
            .block_states
            .get_mut(&cert.block_hash)
            .ok_or(AsfError::BlockNotFound)?;

        // Add certificate
        state.certificates.add_certificate(cert)?;

        // Check if we should advance phase based on certificate count
        let cert_count = state.certificates.count_for_phase(state.current_phase);
        if cert_count >= crate::bft_threshold(self.total_validators) {
            state.advance_phase()?;
        }

        Ok(())
    }

    /// Get the current state for a block
    pub fn get_state(&self, block_hash: &Hash) -> Option<&HotStuffState> {
        self.block_states.get(block_hash)
    }

    /// Get mutable state for a block
    pub fn get_state_mut(&mut self, block_hash: &Hash) -> Option<&mut HotStuffState> {
        self.block_states.get_mut(block_hash)
    }

    /// Check if a block has reached finality
    pub fn is_finalized(&self, block_hash: &Hash) -> bool {
        self.block_states
            .get(block_hash)
            .map(|s| s.finalized || s.finality_level().is_finalized())
            .unwrap_or(false)
    }

    /// Get finality level for a block
    pub fn finality_level(&self, block_hash: &Hash) -> FinalityLevel {
        self.block_states
            .get(block_hash)
            .map(|s| s.finality_level())
            .unwrap_or(FinalityLevel::None)
    }

    /// Update epoch (for PPFA rotation)
    pub fn update_epoch(&mut self, epoch: u32) {
        self.current_epoch = epoch;
        self.cert_generator.set_epoch(epoch);
    }

    /// Update committee (when validators change)
    pub fn update_committee(&mut self, validators: u32, stake: Balance) {
        self.total_validators = validators;
        self.total_stake = stake;
        self.cert_generator.update_committee(validators, stake);
    }

    /// Clean up finalized blocks
    pub fn prune_finalized(&mut self, keep_last_n: usize) {
        let finalized: Vec<Hash> = self
            .block_states
            .iter()
            .filter(|(_, s)| s.finalized)
            .map(|(h, _)| *h)
            .collect();

        if finalized.len() > keep_last_n {
            for hash in finalized.iter().take(finalized.len() - keep_last_n) {
                self.block_states.remove(hash);
            }
        }
    }

    /// Get all blocks currently in consensus
    pub fn active_blocks(&self) -> Vec<Hash> {
        self.block_states.keys().copied().collect()
    }

    /// Get total number of blocks in consensus
    pub fn active_count(&self) -> usize {
        self.block_states.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HOTSTUFF HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Helper to check if phase transition is valid
pub fn is_valid_transition(from: ConsensusPhase, to: ConsensusPhase) -> bool {
    match (from, to) {
        (ConsensusPhase::Prepare, ConsensusPhase::PreCommit) => true,
        (ConsensusPhase::PreCommit, ConsensusPhase::Commit) => true,
        (ConsensusPhase::Commit, ConsensusPhase::Decide) => true,
        _ => false,
    }
}

/// Calculate timeout for a phase (in milliseconds)
pub fn phase_timeout(phase: ConsensusPhase, base_timeout: u64) -> u64 {
    match phase {
        ConsensusPhase::Prepare => base_timeout,
        ConsensusPhase::PreCommit => base_timeout * 2,
        ConsensusPhase::Commit => base_timeout * 3,
        ConsensusPhase::Decide => base_timeout * 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    fn create_test_vote(
        validator_id: u8,
        stake: Balance,
        block_hash: Hash,
        phase: ConsensusPhase,
    ) -> Vote {
        let mut account_bytes = [0u8; 32];
        account_bytes[0] = validator_id;
        
        Vote::new(
            block_hash,
            1,
            phase,
            AccountId32::from(account_bytes),
            stake,
            1,
            1000,
        )
    }

    #[test]
    fn test_hotstuff_state_creation() {
        let state = HotStuffState::new(Hash::default(), 1, 1);
        assert_eq!(state.current_phase, ConsensusPhase::Prepare);
        assert!(!state.finalized);
    }

    #[test]
    fn test_phase_advancement() {
        let mut state = HotStuffState::new(Hash::default(), 1, 1);
        
        assert_eq!(state.current_phase, ConsensusPhase::Prepare);
        state.advance_phase().unwrap();
        assert_eq!(state.current_phase, ConsensusPhase::PreCommit);
        state.advance_phase().unwrap();
        assert_eq!(state.current_phase, ConsensusPhase::Commit);
        state.advance_phase().unwrap();
        assert_eq!(state.current_phase, ConsensusPhase::Decide);
        state.advance_phase().unwrap();
        assert!(state.finalized);
    }

    #[test]
    fn test_hotstuff_engine_creation() {
        let engine = HotStuffEngine::new(21, 21_000, 1);
        assert_eq!(engine.active_count(), 0);
    }

    #[test]
    fn test_start_consensus() {
        let mut engine = HotStuffEngine::new(21, 21_000, 1);
        let block_hash = Hash::default();
        
        assert!(engine.start_consensus(block_hash, 1).is_ok());
        assert_eq!(engine.active_count(), 1);
        assert!(engine.get_state(&block_hash).is_some());
    }

    #[test]
    fn test_process_votes() {
        let mut engine = HotStuffEngine::new(3, 3_000, 1);
        let block_hash = Hash::default();
        
        engine.start_consensus(block_hash, 1).unwrap();
        
        // Add 3 votes (meets threshold for 3 validators)
        let vote1 = create_test_vote(1, 1000, block_hash, ConsensusPhase::Prepare);
        let vote2 = create_test_vote(2, 1000, block_hash, ConsensusPhase::Prepare);
        let vote3 = create_test_vote(3, 1000, block_hash, ConsensusPhase::Prepare);
        
        assert!(engine.process_vote(vote1).is_ok());
        assert!(engine.process_vote(vote2).is_ok());
        
        // Third vote should generate certificate
        let result = engine.process_vote(vote3);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        
        // Should have advanced to PreCommit
        let state = engine.get_state(&block_hash).unwrap();
        assert_eq!(state.current_phase, ConsensusPhase::PreCommit);
    }

    #[test]
    fn test_invalid_phase_vote() {
        let mut engine = HotStuffEngine::new(3, 3_000, 1);
        let block_hash = Hash::default();
        
        engine.start_consensus(block_hash, 1).unwrap();
        
        // Try to vote for PreCommit when in Prepare phase
        let vote = create_test_vote(1, 1000, block_hash, ConsensusPhase::PreCommit);
        assert!(engine.process_vote(vote).is_err());
    }

    #[test]
    fn test_finality_progression() {
        let mut engine = HotStuffEngine::new(3, 3_000, 1);
        let block_hash = Hash::default();
        
        engine.start_consensus(block_hash, 1).unwrap();
        
        assert_eq!(engine.finality_level(&block_hash), FinalityLevel::None);
        assert!(!engine.is_finalized(&block_hash));
    }

    #[test]
    fn test_phase_transition_validation() {
        assert!(is_valid_transition(
            ConsensusPhase::Prepare,
            ConsensusPhase::PreCommit
        ));
        assert!(is_valid_transition(
            ConsensusPhase::PreCommit,
            ConsensusPhase::Commit
        ));
        assert!(!is_valid_transition(
            ConsensusPhase::Prepare,
            ConsensusPhase::Commit
        ));
        assert!(!is_valid_transition(
            ConsensusPhase::Decide,
            ConsensusPhase::Prepare
        ));
    }

    #[test]
    fn test_phase_timeouts() {
        let base = 1000u64;
        assert_eq!(phase_timeout(ConsensusPhase::Prepare, base), 1000);
        assert_eq!(phase_timeout(ConsensusPhase::PreCommit, base), 2000);
        assert_eq!(phase_timeout(ConsensusPhase::Commit, base), 3000);
        assert_eq!(phase_timeout(ConsensusPhase::Decide, base), 4000);
    }

    #[test]
    fn test_epoch_update() {
        let mut engine = HotStuffEngine::new(21, 21_000, 1);
        engine.update_epoch(2);
        assert_eq!(engine.current_epoch, 2);
    }

    #[test]
    fn test_committee_update() {
        let mut engine = HotStuffEngine::new(21, 21_000, 1);
        engine.update_committee(25, 25_000);
        assert_eq!(engine.total_validators, 25);
        assert_eq!(engine.total_stake, 25_000);
    }
}
