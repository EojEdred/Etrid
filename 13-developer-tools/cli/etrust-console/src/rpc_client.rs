// ═══════════════════════════════════════════════════════════════════════════════
// RPC Client for ËTRID Chain
// JSON-RPC WebSocket client for interacting with ËTRID node
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::{Context, Result};
use jsonrpsee::{
    core::client::ClientT,
    rpc_params,
    ws_client::{WsClient, WsClientBuilder},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// RPC Client for ËTRID chain
pub struct EtridRpcClient {
    client: WsClient,
}

impl EtridRpcClient {
    /// Create a new RPC client
    pub async fn new(endpoint: &str) -> Result<Self> {
        let client = WsClientBuilder::default()
            .build(endpoint)
            .await
            .context(format!("Failed to connect to RPC endpoint: {}", endpoint))?;

        Ok(Self { client })
    }

    /// Get block by number or hash
    pub async fn get_block(&self, block_id: &str) -> Result<Value> {
        let response: Value = self
            .client
            .request("chain_getBlock", rpc_params![block_id])
            .await
            .context("Failed to get block")?;

        Ok(response)
    }

    /// Get block hash by number
    pub async fn get_block_hash(&self, block_number: Option<u64>) -> Result<String> {
        let response: String = self
            .client
            .request("chain_getBlockHash", rpc_params![block_number])
            .await
            .context("Failed to get block hash")?;

        Ok(response)
    }

    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        let header: Value = self
            .client
            .request("chain_getHeader", rpc_params![])
            .await
            .context("Failed to get header")?;

        // Extract block number from header
        let number_hex = header["number"]
            .as_str()
            .context("Invalid block number in header")?;

        // Remove 0x prefix and parse
        let number_hex = number_hex.trim_start_matches("0x");
        let number = u64::from_str_radix(number_hex, 16)?;

        Ok(number)
    }

    /// Get account balance
    pub async fn get_balance(&self, account: &str) -> Result<Balance> {
        let response: Value = self
            .client
            .request("system_account", rpc_params![account])
            .await
            .context("Failed to get account balance")?;

        // Parse balance from response
        let data = &response["data"];
        let free = data["free"]
            .as_str()
            .unwrap_or("0")
            .trim_start_matches("0x");
        let reserved = data["reserved"]
            .as_str()
            .unwrap_or("0")
            .trim_start_matches("0x");

        let free = u128::from_str_radix(free, 16).unwrap_or(0);
        let reserved = u128::from_str_radix(reserved, 16).unwrap_or(0);

        Ok(Balance { free, reserved })
    }

    /// Send a signed transaction
    pub async fn send_transaction(&self, extrinsic: &str) -> Result<String> {
        let response: String = self
            .client
            .request("author_submitExtrinsic", rpc_params![extrinsic])
            .await
            .context("Failed to submit transaction")?;

        Ok(response)
    }

    /// Query storage at a key
    pub async fn query_storage(&self, key: &str, block_hash: Option<&str>) -> Result<Option<String>> {
        let response: Option<String> = self
            .client
            .request("state_getStorage", rpc_params![key, block_hash])
            .await
            .context("Failed to query storage")?;

        Ok(response)
    }

    /// Get chain metadata
    pub async fn get_metadata(&self) -> Result<String> {
        let response: String = self
            .client
            .request("state_getMetadata", rpc_params![])
            .await
            .context("Failed to get metadata")?;

        Ok(response)
    }

    /// Get system properties (chain info)
    pub async fn get_system_properties(&self) -> Result<Value> {
        let response: Value = self
            .client
            .request("system_properties", rpc_params![])
            .await
            .context("Failed to get system properties")?;

        Ok(response)
    }

    /// Get system chain name
    pub async fn get_chain_name(&self) -> Result<String> {
        let response: String = self
            .client
            .request("system_chain", rpc_params![])
            .await
            .context("Failed to get chain name")?;

        Ok(response)
    }

    /// Get system version
    pub async fn get_version(&self) -> Result<String> {
        let response: String = self
            .client
            .request("system_version", rpc_params![])
            .await
            .context("Failed to get version")?;

        Ok(response)
    }

    /// Get network peers
    pub async fn get_peers(&self) -> Result<Vec<PeerInfo>> {
        let response: Value = self
            .client
            .request("system_peers", rpc_params![])
            .await
            .context("Failed to get peers")?;

        // Parse peers from response
        let peers = response
            .as_array()
            .context("Invalid peers response")?
            .iter()
            .map(|peer| PeerInfo {
                peer_id: peer["peerId"].as_str().unwrap_or("unknown").to_string(),
                roles: peer["roles"].as_str().unwrap_or("unknown").to_string(),
                best_hash: peer["bestHash"].as_str().unwrap_or("unknown").to_string(),
                best_number: peer["bestNumber"].as_u64().unwrap_or(0),
            })
            .collect();

        Ok(peers)
    }

    /// Get pending extrinsics in transaction pool
    pub async fn get_pending_extrinsics(&self) -> Result<Vec<String>> {
        let response: Vec<String> = self
            .client
            .request("author_pendingExtrinsics", rpc_params![])
            .await
            .context("Failed to get pending extrinsics")?;

        Ok(response)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Data Structures
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub free: u128,
    pub reserved: u128,
}

impl Balance {
    pub fn total(&self) -> u128 {
        self.free + self.reserved
    }

    /// Format balance in ETR (assuming 18 decimals)
    pub fn to_etr(&self, balance: u128) -> f64 {
        balance as f64 / 1_000_000_000_000_000_000.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub roles: String,
    pub best_hash: String,
    pub best_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub status: String,
    pub hash: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════════════════════

/// Format balance with proper decimals
pub fn format_balance(balance: u128) -> String {
    let etr = balance as f64 / 1_000_000_000_000_000_000.0;
    format!("{:.6} ETR", etr)
}

/// Parse balance from string (supports ETR, mETR, uETR)
pub fn parse_balance(amount: &str) -> Result<u128> {
    let amount = amount.trim().to_lowercase();

    if amount.ends_with("etr") {
        let value = amount.trim_end_matches("etr").trim().parse::<f64>()?;
        Ok((value * 1_000_000_000_000_000_000.0) as u128)
    } else if amount.ends_with("metr") {
        let value = amount.trim_end_matches("metr").trim().parse::<f64>()?;
        Ok((value * 1_000_000_000_000_000.0) as u128)
    } else if amount.ends_with("uetr") {
        let value = amount.trim_end_matches("uetr").trim().parse::<f64>()?;
        Ok((value * 1_000_000_000_000.0) as u128)
    } else {
        // Assume ETR if no unit specified
        let value = amount.parse::<f64>()?;
        Ok((value * 1_000_000_000_000_000_000.0) as u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_balance() {
        assert_eq!(parse_balance("1 ETR").unwrap(), 1_000_000_000_000_000_000);
        assert_eq!(parse_balance("1.5 ETR").unwrap(), 1_500_000_000_000_000_000);
        assert_eq!(parse_balance("1000 mETR").unwrap(), 1_000_000_000_000_000_000);
    }

    #[test]
    fn test_format_balance() {
        assert_eq!(format_balance(1_000_000_000_000_000_000), "1.000000 ETR");
        assert_eq!(format_balance(1_500_000_000_000_000_000), "1.500000 ETR");
    }
}
