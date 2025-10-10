// public_query.rs
// Public query endpoint contract stubs

use serde::{Serialize};

#[derive(Serialize)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub status: String,
}

pub fn query_proposals() -> Vec<Proposal> {
    vec![
        Proposal { id: 1, title: "Mint More Etrid", status: "Pending" },
        Proposal { id: 2, title: "Change DD Salary", status: "Passed" },
    ]
}