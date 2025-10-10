use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub category: String,
    pub vote_count: u64,
}

pub fn get_top_proposals() -> Vec<Proposal> {
    vec![
        Proposal { id: 1, title: "Mint 500k ETR", category: "Economy", vote_count: 380 },
        Proposal { id: 2, title: "Add DD salary cap", category: "Governance", vote_count: 260 },
    ]
}