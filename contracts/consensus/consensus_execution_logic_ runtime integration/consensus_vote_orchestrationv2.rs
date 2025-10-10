
use crate::vote_storage::VoteStorage;
use crate::runtime_config::RuntimeConfig;

pub struct ConsensusOrchestrator {
    pub config: RuntimeConfig,
    pub storage: VoteStorage,
}

impl ConsensusOrchestrator {
    pub fn new() -> Self {
        Self {
            config: RuntimeConfig::default(),
            storage: VoteStorage::default(),
        }
    }

    pub fn cast_vote(&mut self, proposal_id: &str, voter: &str, stake: u64, coinage: u64) {
        self.storage.add_vote(proposal_id, voter, stake, coinage);
    }

    pub fn finalize(&self, proposal_id: &str) {
        let results = self.storage.calculate_vote_dilution(proposal_id);
        println!("Finalized Vote Results:");
        for (voter, score) in results {
            println!("{} => {}", voter, score);
        }
    }
}
