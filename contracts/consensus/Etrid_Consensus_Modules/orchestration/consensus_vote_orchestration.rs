// consensus_vote_orchestration.rs

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Vote {
    pub voter_id: String,
    pub proposal_id: String,
    pub stake: u64,
    pub coinage_days: u64,
}

#[derive(Default)]
pub struct VoteRegistry {
    pub votes: Vec<Vote>,
}

impl VoteRegistry {
    pub fn register_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
    }

    pub fn calculate_dilution(&self, total_staked: u64) -> HashMap<String, f64> {
        let mut dilution_map = HashMap::new();
        for vote in &self.votes {
            let dilution = total_staked as f64 / vote.coinage_days.max(1) as f64;
            dilution_map.insert(vote.voter_id.clone(), dilution);
        }
        dilution_map
    }
}