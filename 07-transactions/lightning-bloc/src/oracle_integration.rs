//! Oracle Integration for Lightning Cross-PBC Router
//!
//! Connects multiple price oracle sources to the Cross-PBC Router:
//! - Bridge-specific oracles (EDSC, Chainlink, etc.)
//! - Static rates for stablecoins
//! - DEX TWAP feeds
//! - Aggregated multi-source pricing

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use alloc::{
    vec,
    vec::Vec,
    string::String,
    boxed::Box,
};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String, boxed::Box};

use crate::cross_pbc_router::{ChainId, ExchangeRate};

/// Price oracle trait (simplified from bridge-common)
pub trait LightningPriceOracle {
    /// Get exchange rate between two chains
    fn get_exchange_rate(&self, from: &ChainId, to: &ChainId) -> Option<ExchangeRate>;

    /// Check if oracle supports this pair
    fn supports_pair(&self, from: &ChainId, to: &ChainId) -> bool;

    /// Oracle name for debugging
    fn name(&self) -> &str;
}

/// Oracle manager for Cross-PBC Router
pub struct OracleManager {
    /// Registered oracle sources
    oracles: Vec<Box<dyn LightningPriceOracle>>,
    /// Maximum rate age (seconds)
    max_rate_age: u64,
    /// Minimum confidence score
    min_confidence: u8,
}

impl OracleManager {
    /// Create new oracle manager
    pub fn new(max_rate_age: u64, min_confidence: u8) -> Self {
        Self {
            oracles: Vec::new(),
            max_rate_age,
            min_confidence,
        }
    }

    /// Add oracle source
    pub fn add_oracle(&mut self, oracle: Box<dyn LightningPriceOracle>) {
        self.oracles.push(oracle);
    }

    /// Get exchange rate (first available from any oracle)
    pub fn get_rate(
        &self,
        from: &ChainId,
        to: &ChainId,
        current_time: u64,
    ) -> Option<ExchangeRate> {
        for oracle in &self.oracles {
            if let Some(rate) = oracle.get_exchange_rate(from, to) {
                // Check staleness
                if !rate.is_stale(self.max_rate_age, current_time) {
                    return Some(rate);
                }
            }
        }
        None
    }

    /// Get all available rates for a chain
    pub fn get_all_rates(&self, chain: &ChainId) -> Vec<(ChainId, ExchangeRate)> {
        let mut rates = Vec::new();

        // Try to get rates to all known chains
        let all_chains = vec![
            "eth-pbc", "btc-pbc", "bnb-pbc", "sol-pbc", "ada-pbc",
            "trx-pbc", "xrp-pbc", "xlm-pbc", "matic-pbc", "link-pbc",
            "doge-pbc", "sc-usdt-pbc", "edsc-pbc",
        ];

        for to_chain in all_chains {
            if to_chain != chain.as_str() {
                if let Some(rate) = self.get_rate(chain, &to_chain.into(), 0) {
                    rates.push((to_chain.into(), rate));
                }
            }
        }

        rates
    }

    /// Number of registered oracles
    pub fn oracle_count(&self) -> usize {
        self.oracles.len()
    }
}

/// Mock oracle for testing and initial deployment
pub struct MockOracle {
    /// Pre-defined rates
    rates: Vec<((ChainId, ChainId), ExchangeRate)>,
}

impl MockOracle {
    /// Create mock oracle with sample rates
    pub fn new() -> Self {
        let mut oracle = Self {
            rates: Vec::new(),
        };

        let now = Self::current_time();

        // Add some sample rates
        // ETH to BTC: ~0.05 (1 ETH = 0.05 BTC)
        oracle.add_rate("eth-pbc", "btc-pbc", ExchangeRate::new(500, now));

        // ETH to SOL: ~20 (1 ETH = 20 SOL)
        oracle.add_rate("eth-pbc", "sol-pbc", ExchangeRate::new(200000, now));

        // All USDT pairs are 1:1
        let usdt_chains = vec!["eth-pbc", "trx-pbc", "bnb-pbc", "sc-usdt-pbc"];
        for from in &usdt_chains {
            for to in &usdt_chains {
                if from != to {
                    oracle.add_rate(from, to, ExchangeRate::new(10000, now));
                }
            }
        }

        // EDSC to USDT: 1:1
        oracle.add_rate("edsc-pbc", "sc-usdt-pbc", ExchangeRate::new(10000, now));
        oracle.add_rate("sc-usdt-pbc", "edsc-pbc", ExchangeRate::new(10000, now));

        oracle
    }

    fn add_rate(&mut self, from: &str, to: &str, rate: ExchangeRate) {
        self.rates.push(((from.into(), to.into()), rate));
    }

    #[cfg(feature = "std")]
    fn current_time() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[cfg(not(feature = "std"))]
    fn current_time() -> u64 {
        0
    }
}

impl Default for MockOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl LightningPriceOracle for MockOracle {
    fn get_exchange_rate(&self, from: &ChainId, to: &ChainId) -> Option<ExchangeRate> {
        self.rates
            .iter()
            .find(|((f, t), _)| f == from && t == to)
            .map(|(_, rate)| *rate)
    }

    fn supports_pair(&self, from: &ChainId, to: &ChainId) -> bool {
        self.rates.iter().any(|((f, t), _)| f == from && t == to)
    }

    fn name(&self) -> &str {
        "MockOracle"
    }
}

/// Integration helper to connect oracles to Cross-PBC Router
pub fn setup_oracles_for_router() -> OracleManager {
    let mut manager = OracleManager::new(
        600,  // 10 minute max age
        80,   // 80% minimum confidence
    );

    // Add mock oracle for testing
    manager.add_oracle(Box::new(MockOracle::new()));

    // TODO: Add real oracle sources:
    // - EdscOracleAdapter
    // - ChainlinkAdapter
    // - DEX TWAP adapters
    // - Bridge-specific oracles

    manager
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_oracle() {
        let oracle = MockOracle::new();

        // Test ETH to BTC rate
        let rate = oracle.get_exchange_rate(&"eth-pbc".into(), &"btc-pbc".into());
        assert!(rate.is_some());
        assert_eq!(rate.unwrap().rate, 500);

        // Test USDT 1:1 rates
        let usdt_rate = oracle.get_exchange_rate(&"eth-pbc".into(), &"sc-usdt-pbc".into());
        assert!(usdt_rate.is_none()); // Mock doesn't have this specific pair yet
    }

    #[test]
    fn test_oracle_manager() {
        let mut manager = OracleManager::new(600, 80);
        manager.add_oracle(Box::new(MockOracle::new()));

        assert_eq!(manager.oracle_count(), 1);

        let rate = manager.get_rate(&"eth-pbc".into(), &"btc-pbc".into(), 0);
        assert!(rate.is_some());
    }

    #[test]
    fn test_setup_oracles() {
        let manager = setup_oracles_for_router();
        assert!(manager.oracle_count() > 0);
    }
}
