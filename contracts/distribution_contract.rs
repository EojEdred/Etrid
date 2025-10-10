
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionConfig {
    pub registered_voting_peers_reward_pct: f64,
    pub flare_nodes_reward_pct: f64,
    pub validity_nodes_reward_pct: f64,
    pub common_stake_peers_reward_pct: f64,
    pub dd_base_salary_pct: f64,
    pub dd_flare_node_reward_pct: f64,
}

impl Default for DistributionConfig {
    fn default() -> Self {
        Self {
            registered_voting_peers_reward_pct: 0.10,
            flare_nodes_reward_pct: 0.20,
            validity_nodes_reward_pct: 0.25,
            common_stake_peers_reward_pct: 0.15,
            dd_base_salary_pct: 0.05,
            dd_flare_node_reward_pct: 0.05,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub stake: f64,
    pub coinage: u64,
    pub votes_committed: bool,
    pub penalties: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionSchedule {
    pub timestamp: String,
    pub payout_map: HashMap<String, f64>,
}

pub fn calculate_distribution(peers: &HashMap<String, Peer>, config: &DistributionConfig) -> DistributionSchedule {
    let mut payout_map = HashMap::new();
    for (id, peer) in peers {
        if peer.votes_committed {
            let reward = (peer.stake * peer.coinage as f64).sqrt() * (1.0 - 0.01 * peer.penalties as f64)
                * config.registered_voting_peers_reward_pct;
            payout_map.insert(id.clone(), reward);
        }
    }
    DistributionSchedule {
        timestamp: "2025-12-01T00:01:00Z".to_string(),
        payout_map,
    }
}
