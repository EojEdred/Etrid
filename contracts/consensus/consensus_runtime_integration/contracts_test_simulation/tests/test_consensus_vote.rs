#[cfg(test)]
mod tests {
    use super::*;
    use consensus::vote_storage::{store_vote, Vote};

    #[test]
    fn test_vote_storage() {
        let vote = Vote { voter_id: "peer1".to_string(), proposal_id: 1, weight: 100 };
        let result = store_vote(vote.clone());
        assert!(result.is_ok());
    }
}