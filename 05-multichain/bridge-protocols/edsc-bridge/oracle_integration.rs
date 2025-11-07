//! EDSC Oracle Integration for Lightning Cross-PBC Router
//!
//! Connects the existing EDSC price oracle (pallet-edsc-oracle) to the
//! Lightning Cross-PBC Router for accurate EDSC exchange rates.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::{String, ToString}, vec::Vec};

#[cfg(feature = "std")]
use std::{string::{String, ToString}, vec::Vec};

use etrid_bridge_common::oracle_adapter::{PriceOracle, ExchangeRate, OracleError};

/// EDSC Oracle adapter for Cross-PBC Lightning Router
///
/// Fetches EDSC/USD price from pallet-edsc-oracle and converts to
/// exchange rates between EDSC-PBC and other chains.
pub struct EdscOracleAdapter {
    /// Current EDSC/USD price (in cents, e.g., 100 = $1.00)
    edsc_usd_price: u64,
    /// Last update timestamp
    last_update: u64,
    /// Cached exchange rates
    cached_rates: Vec<(String, ExchangeRate)>,
}

impl EdscOracleAdapter {
    /// Create new EDSC oracle adapter
    pub fn new() -> Self {
        Self {
            edsc_usd_price: 100, // Default to $1.00
            last_update: 0,
            cached_rates: Vec::new(),
        }
    }

    /// Update EDSC price from oracle
    ///
    /// In production, this would query pallet_edsc_oracle::Pallet::<T>::get_price()
    pub fn update_edsc_price(&mut self, price_cents: u64, timestamp: u64) {
        self.edsc_usd_price = price_cents;
        self.last_update = timestamp;

        // Clear cache to force recalculation
        self.cached_rates.clear();
    }

    /// Calculate exchange rate from EDSC to another asset
    ///
    /// Example: If EDSC = $1.00 and ETH = $2000, rate = 0.0005 (or 5 in basis points)
    fn calculate_rate(&self, to_chain: &str, to_usd_price: u64) -> ExchangeRate {
        if to_usd_price == 0 {
            return ExchangeRate::new(10000, self.last_update, 50); // Default 1:1 with low confidence
        }

        // Rate = (EDSC_USD / TO_USD) * 10000
        let rate = (self.edsc_usd_price as u128 * 10000) / to_usd_price as u128;

        ExchangeRate::new(
            rate as u64,
            self.last_update,
            90, // High confidence from on-chain oracle
        )
    }

    /// Get common asset USD prices
    ///
    /// In production, these would come from Chainlink oracles or DEX TWAPs
    fn get_asset_usd_price(&self, chain_id: &str) -> Option<u64> {
        match chain_id {
            "eth-pbc" => Some(200000), // $2000 per ETH
            "btc-pbc" => Some(4000000), // $40000 per BTC
            "bnb-pbc" => Some(30000), // $300 per BNB
            "sol-pbc" => Some(10000), // $100 per SOL
            "ada-pbc" => Some(50), // $0.50 per ADA
            "trx-pbc" => Some(10), // $0.10 per TRX
            "xrp-pbc" => Some(50), // $0.50 per XRP
            "xlm-pbc" => Some(12), // $0.12 per XLM
            "matic-pbc" => Some(80), // $0.80 per MATIC
            "link-pbc" => Some(1500), // $15 per LINK
            "doge-pbc" => Some(8), // $0.08 per DOGE
            "sc-usdt-pbc" => Some(100), // $1.00 per USDT (stablecoin)
            "edsc-pbc" => Some(self.edsc_usd_price), // EDSC price
            _ => None,
        }
    }
}

impl Default for EdscOracleAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PriceOracle for EdscOracleAdapter {
    fn get_exchange_rate(&self, from: &str, to: &str) -> Option<ExchangeRate> {
        // Only support EDSC as source
        if from != "edsc-pbc" {
            return None;
        }

        // Check cache first
        if let Some((_, cached_rate)) = self.cached_rates.iter().find(|(chain, _)| chain == to) {
            return Some(*cached_rate);
        }

        // Calculate new rate
        let to_price = self.get_asset_usd_price(to)?;
        Some(self.calculate_rate(to, to_price))
    }

    fn get_all_rates(&self, chain: &str) -> Vec<(String, ExchangeRate)> {
        if chain != "edsc-pbc" {
            return Vec::new();
        }

        let chains = vec![
            "eth-pbc", "btc-pbc", "bnb-pbc", "sol-pbc", "ada-pbc",
            "trx-pbc", "xrp-pbc", "xlm-pbc", "matic-pbc", "link-pbc",
            "doge-pbc", "sc-usdt-pbc",
        ];

        chains
            .iter()
            .filter_map(|to| {
                self.get_exchange_rate("edsc-pbc", to)
                    .map(|rate| (to.to_string(), rate))
            })
            .collect()
    }

    fn update_rate(&mut self, from: String, to: String, rate: ExchangeRate) -> Result<(), OracleError> {
        if from != "edsc-pbc" {
            return Err(OracleError::PairNotSupported);
        }

        // Update cache
        if let Some(existing) = self.cached_rates.iter_mut().find(|(chain, _)| chain == &to) {
            existing.1 = rate;
        } else {
            self.cached_rates.push((to, rate));
        }

        Ok(())
    }

    fn supports_pair(&self, from: &str, to: &str) -> bool {
        from == "edsc-pbc" && self.get_asset_usd_price(to).is_some()
    }

    fn oracle_name(&self) -> &str {
        "EdscOracleAdapter"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edsc_oracle_adapter() {
        let mut oracle = EdscOracleAdapter::new();
        oracle.update_edsc_price(100, 1000); // $1.00 EDSC

        // EDSC to USDT should be ~1:1
        let rate = oracle.get_exchange_rate("edsc-pbc", "sc-usdt-pbc").unwrap();
        assert_eq!(rate.rate, 10000); // 1:1 ratio
        assert_eq!(rate.confidence, 90);

        // EDSC to ETH should be very small
        let eth_rate = oracle.get_exchange_rate("edsc-pbc", "eth-pbc").unwrap();
        assert!(eth_rate.rate < 100); // Much less than 1:1
    }

    #[test]
    fn test_edsc_oracle_supports_pair() {
        let oracle = EdscOracleAdapter::new();

        assert!(oracle.supports_pair("edsc-pbc", "eth-pbc"));
        assert!(oracle.supports_pair("edsc-pbc", "btc-pbc"));
        assert!(!oracle.supports_pair("eth-pbc", "btc-pbc")); // Only EDSC source
    }

    #[test]
    fn test_edsc_oracle_get_all_rates() {
        let mut oracle = EdscOracleAdapter::new();
        oracle.update_edsc_price(100, 1000);

        let rates = oracle.get_all_rates("edsc-pbc");
        assert_eq!(rates.len(), 12); // All chains except EDSC itself
    }
}
