use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

    pub async fn fetch_network_status(&self) -> Result<NetworkStatus> {
        // Fetch chain name
        let chain = self.rpc_call::<String>("system_chain", json!([])).await
            .unwrap_or_else(|_| "ËTRID".to_string());

        // Fetch current block
        let block_number = self.rpc_call::<u64>("chain_getBlockNumber", json!([])).await
            .unwrap_or(0);

        // Fetch peer count
        let peer_count = self.rpc_call::<u32>("system_peerCount", json!([])).await
            .unwrap_or(0);

        // Fetch sync status
        let is_syncing = self.rpc_call::<bool>("system_isSyncing", json!([])).await
            .unwrap_or(false);

        Ok(NetworkStatus {
            chain,
            block_number,
            peer_count,
            is_syncing,
        })
    }

    pub async fn fetch_validator_status(&self, address: &str) -> Result<ValidatorStatus> {
        // These would be actual RPC calls to the ASF pallet
        // For now, returning mock data structure

        // In production, these would be calls like:
        // - asf_validatorInfo(address)
        // - asf_committeeMembers()
        // - asf_validatorMetrics(address)

        Ok(ValidatorStatus {
            address: address.to_string(),
            is_active: true,
            in_committee: true,
            is_slashed: false,
            votes_cast: 1234,
            certificates: 567,
            blocks_signed: 890,
            missed_blocks: 10,
            health_score: 95,
            reputation: 98,
            finality_level: 3,
        })
    }

    async fn rpc_call<T: for<'de> Deserialize<'de>>(&self, method: &str, params: serde_json::Value) -> Result<T> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1,
        });

        let response = self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;

        let json: RpcResponse<T> = response.json().await?;

        if let Some(result) = json.result {
            Ok(result)
        } else {
            Err(anyhow::anyhow!("RPC error: {:?}", json.error))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RPC TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub chain: String,
    pub block_number: u64,
    pub peer_count: u32,
    pub is_syncing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorStatus {
    pub address: String,
    pub is_active: bool,
    pub in_committee: bool,
    pub is_slashed: bool,
    pub votes_cast: u64,
    pub certificates: u64,
    pub blocks_signed: u64,
    pub missed_blocks: u64,
    pub health_score: u8,
    pub reputation: u8,
    pub finality_level: u8,
}
