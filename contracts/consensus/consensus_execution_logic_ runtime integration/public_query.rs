
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProposalStatus {
    pub id: String,
    pub description: String,
    pub total_votes: usize,
}

pub fn query_proposals() -> Vec<ProposalStatus> {
    vec![
        ProposalStatus {
            id: "p001".to_string(),
            description: "Increase minting supply by 2%".to_string(),
            total_votes: 94,
        },
        ProposalStatus {
            id: "p002".to_string(),
            description: "Add new decentralized director".to_string(),
            total_votes: 57,
        },
    ]
}
