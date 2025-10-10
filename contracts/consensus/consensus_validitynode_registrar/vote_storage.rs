
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub peer_id: String,
    pub stake: u64,
    pub coinage: u64,
    pub committed: bool,
}

#[derive(Default)]
pub struct VoteStorage {
    pub votes: HashMap<String, Vote>,
}

impl VoteStorage {
    pub fn store_vote(&mut self, vote: Vote) {
        self.votes.insert(vote.peer_id.clone(), vote);
    }

    pub fn get_vote(&self, peer_id: &str) -> Option<&Vote> {
        self.votes.get(peer_id)
    }

    pub fn all_committed_votes(&self) -> Vec<&Vote> {
        self.votes.values().filter(|v| v.committed).collect()
    }
}
