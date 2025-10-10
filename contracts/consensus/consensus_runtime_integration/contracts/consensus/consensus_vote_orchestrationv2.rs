use crate::distribution::distribution_execution::{DistributionInput, calculate_distribution};
use crate::vote_storage::{store_vote, VoteInput};
use crate::runtime::runtime_config::RuntimeConfig;

pub fn execute_consensus_flow(input: VoteInput, dist: DistributionInput) -> u64 {
    store_vote(input);
    calculate_distribution(dist)
}