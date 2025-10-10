use std::collections::HashMap;
use chrono::Utc;

#[derive(Debug)]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub created_at: i64,
    pub votes: HashMap<String, bool>,
}

impl Proposal {
    pub fn new(title: &str, description: &str) -> Self {
        Proposal {
            title: title.into(),
            description: description.into(),
            created_at: Utc::now().timestamp(),
            votes: HashMap::new(),
        }
    }

    pub fn cast_vote(&mut self, voter_id: &str, choice: bool) {
        self.votes.insert(voter_id.into(), choice);
    }

    pub fn summary(&self) -> (usize, usize) {
        let yes = self.votes.values().filter(|&&v| v).count();
        let no = self.votes.len() - yes;
        (yes, no)
    }
}
