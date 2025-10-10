
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DistributionInput {
    pub voter_stake: u64,
    pub coinage: u64,
    pub vote_committed: bool,
    pub penalties: u64,
}

pub fn calculate_distribution(input: DistributionInput) -> u64 {
    if !input.vote_committed {
        return 0;
    }
    let diluted = (input.voter_stake * input.coinage) / 100_000;
    diluted.saturating_sub(input.penalties)
}
