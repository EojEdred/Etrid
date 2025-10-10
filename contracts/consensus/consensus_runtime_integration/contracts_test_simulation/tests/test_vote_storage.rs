#[cfg(test)]
mod tests {
    use super::*;
    use consensus::vote_storage::{VoteStorage, Vote};

    #[test]
    fn test_vote_insert_and_fetch() {
        let mut storage = VoteStorage::new();
        let vote = Vote { voter_id: "voterX".to_string(), proposal_id: 101, weight: 200 };
        storage.insert(vote.clone());
        let fetched = storage.get("voterX").unwrap();
        assert_eq!(fetched.proposal_id, vote.proposal_id);
    }
}