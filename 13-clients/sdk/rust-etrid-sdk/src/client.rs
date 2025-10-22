//! RPC client for connecting to Ëtrid blockchain nodes

use jsonrpsee::{
    ws_client::{WsClient, WsClientBuilder},
    core::client::ClientT,
    rpc_params,
};
use crate::{Error, Result};

/// Ëtrid blockchain RPC client
pub struct Client {
    inner: WsClient,
    endpoint: String,
}

impl Client {
    /// Create a new client connected to the specified endpoint
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("ws://localhost:9944").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(endpoint: &str) -> Result<Self> {
        let inner = WsClientBuilder::default()
            .build(endpoint)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        Ok(Self {
            inner,
            endpoint: endpoint.to_string(),
        })
    }

    /// Get the current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        let result: String = self.inner
            .request("chain_getBlockNumber", rpc_params![])
            .await
            .map_err(|e| Error::Rpc(e.to_string()))?;

        Ok(u64::from_str_radix(&result[2..], 16)
            .map_err(|e| Error::Parse(e.to_string()))?)
    }

    /// Get account balance
    pub async fn get_balance(&self, address: &str) -> Result<u128> {
        // Placeholder implementation
        // In real implementation, would call system_accountBalance
        Ok(0)
    }

    /// Get the chain name
    pub async fn get_chain_name(&self) -> Result<String> {
        self.inner
            .request("system_chain", rpc_params![])
            .await
            .map_err(|e| Error::Rpc(e.to_string()))
    }

    /// Get the endpoint this client is connected to
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires running node
    async fn test_client_connection() {
        let client = Client::new("ws://localhost:9944").await;
        assert!(client.is_ok());
    }
}
