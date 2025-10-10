
#[derive(Debug)]
pub struct RuntimeConfig {
    pub consensus_day_epoch: u64,
    pub min_stake_amount: u64,
    pub reward_rate: f64,
}

impl RuntimeConfig {
    pub fn default() -> Self {
        Self {
            consensus_day_epoch: 1735728000, // Example: Dec 1, 2024
            min_stake_amount: 1,
            reward_rate: 0.08,
        }
    }
}
