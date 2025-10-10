
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingPeer {
    pub stake: f64,
    pub coinage: u64,
    pub vote: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteTally {
    pub total_votes: HashMap<String, f64>,
}

pub fn calculate_vote_dilution(peers: &Vec<VotingPeer>) -> VoteTally {
    let mut tally = HashMap::new();
    for peer in peers {
        if let Some(ref vote) = peer.vote {
            let dilution = peer.stake / peer.coinage.max(1) as f64;
            *tally.entry(vote.clone()).or_insert(0.0) += dilution;
        }
    }
    VoteTally {
        total_votes: tally,
    }
}
