use anyhow::Result;
use serde_json::json;
use std::time::Instant;

// ═══════════════════════════════════════════════════════════════════════════════
// RPC CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

pub struct RpcClient {
    endpoint: String,
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    pub async fn ping(&self) -> Result<u64> {
        let start = Instant::now();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "system_health",
            "params": [],
            "id": 1,
        });

        self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;

        let latency = start.elapsed().as_millis() as u64;
        Ok(latency)
    }

    pub async fn get_validator_stake(&self, validator: &str) -> Result<u128> {
        // In production, this would call the actual RPC method
        // For now, returning a mock value
        Ok(150_000_000_000_000) // 150,000 ETR
    }

    pub async fn is_slashed(&self, validator: &str) -> Result<bool> {
        // In production, this would call: asf_isSlashed(validator)
        Ok(false)
    }

    pub async fn get_peer_count(&self) -> Result<u32> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "system_health",
            "params": [],
            "id": 1,
        });

        let response = self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;

        // Parse response and extract peer count
        // For now, return mock value
        Ok(15)
    }

    pub async fn get_sync_status(&self) -> Result<(bool, u64, u64)> {
        // Returns: (is_syncing, current_block, highest_block)
        let request = json!({
            "jsonrpc": "2.0",
            "method": "system_syncState",
            "params": [],
            "id": 1,
        });

        let response = self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;

        // Parse response
        // For now, return mock values
        Ok((false, 12345, 12345))
    }
}
