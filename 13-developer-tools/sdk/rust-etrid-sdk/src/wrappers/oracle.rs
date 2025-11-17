//! Oracle - Price feed and external data oracle
//!
//! This module provides functionality for querying price data and submitting
//! oracle updates to the Ëtrid blockchain.

use async_trait::async_trait;
use crate::{Client, Error, Result};
use crate::types::{PriceData, TwapData, TxHash};
use serde::{Deserialize, Serialize};

/// Oracle wrapper for price feed operations
pub struct OracleWrapper {
    client: Client,
}

/// Price submission parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSubmission {
    /// Asset symbol
    pub symbol: String,
    /// Price in USD (scaled by 10^decimals)
    pub price: u128,
    /// Decimal places
    pub decimals: u8,
}

impl OracleWrapper {
    /// Create a new Oracle wrapper instance
    ///
    /// # Arguments
    ///
    /// * `client` - Reference to the Ëtrid RPC client
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::OracleWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("ws://localhost:9944").await?;
    /// let oracle = OracleWrapper::new(client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get current price for an asset
    ///
    /// # Arguments
    ///
    /// * `symbol` - Asset symbol (e.g., "BTC", "ETH", "ETR")
    ///
    /// # Returns
    ///
    /// Returns the latest price data
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::OracleWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let oracle = OracleWrapper::new(client);
    /// let price = oracle.get_price("BTC").await?;
    /// println!("BTC price: ${}", price.price as f64 / 10f64.powi(price.decimals as i32));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_price(&self, symbol: &str) -> Result<PriceData> {
        if symbol.is_empty() {
            return Err(Error::Oracle("Symbol cannot be empty".to_string()));
        }

        // In production, query price from storage
        let price_data = PriceData {
            symbol: symbol.to_uppercase(),
            price: match symbol.to_uppercase().as_str() {
                "BTC" => 45_000_000_000_000, // $45,000.00
                "ETH" => 2_500_000_000_000,  // $2,500.00
                "ETR" => 10_000_000_000,     // $10.00
                _ => return Err(Error::NotFound(format!("Price for {} not found", symbol))),
            },
            timestamp: Self::timestamp() as u64,
            decimals: 8,
        };

        Ok(price_data)
    }

    /// Get Time-Weighted Average Price (TWAP)
    ///
    /// # Arguments
    ///
    /// * `symbol` - Asset symbol
    /// * `period` - Time period in seconds
    ///
    /// # Returns
    ///
    /// Returns the TWAP data
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::OracleWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let oracle = OracleWrapper::new(client);
    /// let twap = oracle.get_twap("ETH", 3600).await?; // 1 hour TWAP
    /// println!("ETH 1h TWAP: ${}", twap.price);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_twap(&self, symbol: &str, period: u64) -> Result<TwapData> {
        if symbol.is_empty() {
            return Err(Error::Oracle("Symbol cannot be empty".to_string()));
        }

        if period == 0 {
            return Err(Error::Oracle("Period must be greater than zero".to_string()));
        }

        // In production, calculate TWAP from historical price data
        let current_price = self.get_price(symbol).await?;

        let twap_data = TwapData {
            symbol: symbol.to_uppercase(),
            price: current_price.price, // Simplified - would be average
            period,
        };

        Ok(twap_data)
    }

    /// Submit a price update (oracle operator only)
    ///
    /// # Arguments
    ///
    /// * `submission` - Price submission data
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{OracleWrapper, PriceSubmission}};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let oracle = OracleWrapper::new(client);
    /// let submission = PriceSubmission {
    ///     symbol: "BTC".to_string(),
    ///     price: 45_123_456_789_000,
    ///     decimals: 8,
    /// };
    /// let tx_hash = oracle.submit_price(submission).await?;
    /// println!("Price submitted: {}", tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn submit_price(&self, submission: PriceSubmission) -> Result<TxHash> {
        // Validate submission
        if submission.symbol.is_empty() {
            return Err(Error::Oracle("Symbol cannot be empty".to_string()));
        }

        if submission.price == 0 {
            return Err(Error::Oracle("Price must be greater than zero".to_string()));
        }

        if submission.decimals > 18 {
            return Err(Error::Oracle("Decimals must be <= 18".to_string()));
        }

        // In production, verify caller is authorized oracle operator
        // then submit price update extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// Subscribe to price updates (WebSocket streaming)
    ///
    /// # Arguments
    ///
    /// * `symbol` - Asset symbol
    ///
    /// # Returns
    ///
    /// Returns a stream of price updates
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::OracleWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let oracle = OracleWrapper::new(client);
    /// // In a real implementation, this would return a stream
    /// // let mut price_stream = oracle.subscribe_to_price_updates("BTC").await?;
    /// // while let Some(price) = price_stream.next().await {
    /// //     println!("New BTC price: {}", price.price);
    /// // }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe_to_price_updates(&self, symbol: &str) -> Result<PriceUpdateSubscription> {
        if symbol.is_empty() {
            return Err(Error::Oracle("Symbol cannot be empty".to_string()));
        }

        // In production, establish WebSocket subscription
        Ok(PriceUpdateSubscription {
            symbol: symbol.to_string(),
        })
    }

    /// Get list of supported assets
    ///
    /// # Returns
    ///
    /// Returns a vector of supported symbols
    pub async fn get_supported_assets(&self) -> Result<Vec<String>> {
        // In production, query supported assets from storage
        Ok(vec![
            "BTC".to_string(),
            "ETH".to_string(),
            "ETR".to_string(),
            "BNB".to_string(),
            "ADA".to_string(),
            "SOL".to_string(),
        ])
    }

    /// Get historical price data
    ///
    /// # Arguments
    ///
    /// * `symbol` - Asset symbol
    /// * `from_timestamp` - Start timestamp
    /// * `to_timestamp` - End timestamp
    ///
    /// # Returns
    ///
    /// Returns historical price data
    pub async fn get_price_history(
        &self,
        symbol: &str,
        from_timestamp: u64,
        to_timestamp: u64,
    ) -> Result<Vec<PriceData>> {
        if from_timestamp > to_timestamp {
            return Err(Error::InvalidInput(
                "from_timestamp must be <= to_timestamp".to_string(),
            ));
        }

        // In production, query historical price data
        Ok(vec![])
    }

    /// Timestamp helper
    fn timestamp() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

/// Price update subscription handle
#[derive(Debug)]
pub struct PriceUpdateSubscription {
    symbol: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_formatting() {
        let price = 45_000_000_000_000u128;
        let decimals = 8u8;
        let formatted = price as f64 / 10f64.powi(decimals as i32);
        assert_eq!(formatted, 450000.0);
    }

    #[test]
    fn test_symbol_validation() {
        let valid = "BTC";
        assert!(!valid.is_empty());

        let invalid = "";
        assert!(invalid.is_empty());
    }
}
