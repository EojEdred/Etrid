// runtime_config.rs

use chrono::{NaiveTime, Timelike, Utc};

pub fn is_within_consensus_window() -> bool {
    let now = Utc::now().time();
    let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    now >= start && now <= end
}