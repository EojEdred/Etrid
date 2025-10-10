use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub timestamp: u64,
    pub category: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub amount_staked: u64,
    pub coinage: u64,
    pub support: bool,
}

#[derive(Default)]
pub struct GovernanceState {
    pub proposals: HashMap<String, Proposal>,
    pub votes: HashMap<String, Vec<Vote>>,
}

impl GovernanceState {
    pub fn submit_proposal(&mut self, proposal: Proposal) {
        self.proposals.insert(proposal.id.clone(), proposal);
    }

    pub fn commit_vote(&mut self, vote: Vote) {
        self.votes.entry(vote.proposal_id.clone()).or_default().push(vote);
    }

    pub fn tally_votes(&mut self, proposal_id: &str) -> Option<bool> {
        if let Some(votes) = self.votes.get(proposal_id) {
            let mut for_votes = 0u64;
            let mut against_votes = 0u64;
            for vote in votes {
                let weight = vote.amount_staked * vote.coinage;
                if vote.support {
                    for_votes += weight;
                } else {
                    against_votes += weight;
                }
            }
            return Some(for_votes > against_votes);
        }
        None
    }

    pub fn execute_proposal(&mut self, proposal_id: &str) -> bool {
        if let Some(approved) = self.tally_votes(proposal_id) {
            if approved {
                if let Some(proposal) = self.proposals.get_mut(proposal_id) {
                    proposal.executed = true;
                    return true;
                }
            }
        }
        false
    }
}
