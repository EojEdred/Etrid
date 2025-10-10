// distribution_execution.rs
// Handles Distribution Pay Schedule Execution
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionSchedule {
    pub recipient: String,
    pub time_slot: String,
    pub role: String,
    pub percent: f64,
}

pub fn execute_distribution(schedule: Vec<DistributionSchedule>) {
    for entry in schedule {
        println!("Distributing {}% to {} at {}", entry.percent, entry.recipient, entry.time_slot);
    }
}