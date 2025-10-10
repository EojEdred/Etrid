
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub staked_amount: u64,
    pub coinage_timestamp: u64,
}

#[derive(Default, Serialize, Deserialize)]
pub struct VoteStorage {
    pub votes: HashMap<String, Vec<Vote>>,
}

impl VoteStorage {
    pub fn add_vote(&mut self, proposal_id: &str, voter: &str, stake: u64, coinage_timestamp: u64) {
        let vote = Vote {
            proposal_id: proposal_id.to_string(),
            voter: voter.to_string(),
            staked_amount: stake,
            coinage_timestamp,
        };
        self.votes.entry(proposal_id.to_string()).or_default().push(vote);
    }

    pub fn calculate_vote_dilution(&self, proposal_id: &str) -> HashMap<String, f64> {
        if let Some(votes) = self.votes.get(proposal_id) {
            let total_staked: u64 = votes.iter().map(|v| v.staked_amount).sum();
            votes.iter().map(|v| {
                let dilution = if v.coinage_timestamp > 0 {
                    (v.staked_amount as f64) / (v.coinage_timestamp as f64)
                } else {
                    0.0
                };
                (v.voter.clone(), dilution / (total_staked as f64))
            }).collect()
        } else {
            HashMap::new()
        }
    }
}
