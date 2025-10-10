use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteInput {
    pub peer_id: String,
    pub proposal_id: String,
    pub stake: u64,
    pub coinage: u64,
    pub weight: u64,
}

pub fn store_vote(vote: VoteInput) {
    println!("Storing vote for peer: {:?}", vote);
}

pub struct Vote {
    pub peer_id: String,
    pub vote_value: bool,
}

pub struct VoteStorage {
    pub votes: Vec<Vote>,
}

pub fn store_vote(storage: &mut VoteStorage, vote: Vote) {
    storage.votes.push(vote);
}