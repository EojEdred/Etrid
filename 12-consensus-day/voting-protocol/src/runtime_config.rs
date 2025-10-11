pub struct RuntimeConfig {
    pub consensus_day: &'static str,
    pub dilution_base: u64,
    pub min_stake: u64,
}

impl RuntimeConfig {
    pub fn default() -> Self {
        RuntimeConfig {
            consensus_day: "December 1st, 12:00 AM PST",
            dilution_base: 100_000,
            min_stake: 1,
        }
    }
}