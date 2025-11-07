//! Oracle Adapter for Cross-PBC Lightning Router
//!
//! Provides price feeds from multiple sources (Chainlink, bridge oracles, DEX TWAPs)
//! to the Lightning Cross-PBC Router for accurate exchange rate calculations.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_attributes)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec, vec::Vec, boxed::Box};

#[cfg(feature = "std")]
use std::{string::String, vec::Vec, boxed::Box};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Chain identifier (matches Lightning Cross-PBC Router)
pub type ChainId = String;

/// Price in basis points (10000 = 1.0)
pub type Price = u64;

/// Timestamp in seconds since UNIX epoch
pub type Timestamp = u64;

/// Exchange rate between two chains/assets
#[derive(Clone, Copy, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct ExchangeRate {
    /// Rate in basis points (10000 = 1:1 ratio)
    pub rate: u64,
    /// Timestamp when rate was fetched
    pub timestamp: u64,
    /// Confidence score (0-100)
    pub confidence: u8,
}

impl ExchangeRate {
    /// Create new exchange rate
    pub fn new(rate: u64, timestamp: u64, confidence: u8) -> Self {
        Self {
            rate,
            timestamp,
            confidence: confidence.min(100),
        }
    }

    /// Convert amount using this rate
    pub fn convert(&self, amount: u128) -> u128 {
        (amount * self.rate as u128) / 10000
    }

    /// Check if rate is stale
    pub fn is_stale(&self, max_age: u64, current_time: u64) -> bool {
        current_time.saturating_sub(self.timestamp) > max_age
    }

    /// Check if confidence is acceptable
    pub fn is_reliable(&self, min_confidence: u8) -> bool {
        self.confidence >= min_confidence
    }
}

/// Price oracle trait for bridge protocols
pub trait PriceOracle {
    /// Get exchange rate between two chains
    fn get_exchange_rate(&self, from: &ChainId, to: &ChainId) -> Option<ExchangeRate>;

    /// Get all available exchange rates for a chain
    fn get_all_rates(&self, chain: &ChainId) -> Vec<(ChainId, ExchangeRate)>;

    /// Update exchange rate (for oracles that cache)
    fn update_rate(&mut self, from: ChainId, to: ChainId, rate: ExchangeRate) -> Result<(), OracleError>;

    /// Check if oracle supports a chain pair
    fn supports_pair(&self, from: &ChainId, to: &ChainId) -> bool;

    /// Get oracle name for debugging
    fn oracle_name(&self) -> &str;
}

/// Oracle error types
#[derive(Clone, Copy, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub enum OracleError {
    /// Rate not available for this pair
    PairNotSupported,
    /// Rate is too stale
    StaleRate,
    /// Confidence too low
    LowConfidence,
    /// Oracle temporarily unavailable
    Unavailable,
    /// Invalid rate data
    InvalidData,
}

/// Multi-source oracle aggregator
pub struct OracleAggregator {
    /// List of oracle sources
    oracles: Vec<Box<dyn PriceOracle>>,
    /// Minimum number of sources required
    min_sources: usize,
    /// Maximum rate age in seconds
    max_rate_age: u64,
    /// Minimum confidence score
    min_confidence: u8,
}

impl OracleAggregator {
    /// Create new aggregator
    pub fn new(min_sources: usize, max_rate_age: u64, min_confidence: u8) -> Self {
        Self {
            oracles: Vec::new(),
            min_sources,
            max_rate_age,
            min_confidence,
        }
    }

    /// Add oracle source
    pub fn add_oracle(&mut self, oracle: Box<dyn PriceOracle>) {
        self.oracles.push(oracle);
    }

    /// Get aggregated exchange rate (median of all sources)
    pub fn get_aggregated_rate(
        &self,
        from: &ChainId,
        to: &ChainId,
        current_time: u64,
    ) -> Result<ExchangeRate, OracleError> {
        let mut rates = Vec::new();

        // Collect rates from all sources
        for oracle in &self.oracles {
            if let Some(rate) = oracle.get_exchange_rate(from, to) {
                // Filter out stale or low-confidence rates
                if !rate.is_stale(self.max_rate_age, current_time)
                    && rate.is_reliable(self.min_confidence)
                {
                    rates.push(rate);
                }
            }
        }

        // Check minimum sources requirement
        if rates.len() < self.min_sources {
            return Err(OracleError::Unavailable);
        }

        // Calculate median rate
        rates.sort_by_key(|r| r.rate);
        let median_idx = rates.len() / 2;
        let median_rate = rates[median_idx];

        // Calculate average confidence
        let avg_confidence = (rates.iter().map(|r| r.confidence as u32).sum::<u32>()
            / rates.len() as u32) as u8;

        Ok(ExchangeRate::new(
            median_rate.rate,
            current_time,
            avg_confidence,
        ))
    }

    /// Get number of active oracles
    pub fn oracle_count(&self) -> usize {
        self.oracles.len()
    }

    /// Get all supported pairs
    pub fn supported_pairs(&self) -> Vec<(ChainId, ChainId)> {
        // TODO: Implement based on oracle capabilities
        Vec::new()
    }
}

/// Static exchange rates for stablecoins and fixed-rate pairs
pub struct StaticRateOracle {
    rates: Vec<((ChainId, ChainId), ExchangeRate)>,
}

impl StaticRateOracle {
    /// Create new static rate oracle
    pub fn new() -> Self {
        let mut oracle = Self { rates: Vec::new() };

        // Add USDT 1:1 rates across all chains
        let usdt_chains = vec![
            "eth-pbc", "trx-pbc", "bnb-pbc", "matic-pbc", "sol-pbc", "sc-usdt-pbc",
        ];

        let now = Self::current_timestamp();
        for i in 0..usdt_chains.len() {
            for j in 0..usdt_chains.len() {
                if i != j {
                    oracle.rates.push((
                        (usdt_chains[i].into(), usdt_chains[j].into()),
                        ExchangeRate::new(10000, now, 100), // 1:1 with 100% confidence
                    ));
                }
            }
        }

        oracle
    }

    #[cfg(feature = "std")]
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[cfg(not(feature = "std"))]
    fn current_timestamp() -> u64 {
        0 // In no_std, must be provided externally
    }
}

impl Default for StaticRateOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl PriceOracle for StaticRateOracle {
    fn get_exchange_rate(&self, from: &ChainId, to: &ChainId) -> Option<ExchangeRate> {
        self.rates
            .iter()
            .find(|((f, t), _)| f == from && t == to)
            .map(|(_, rate)| *rate)
    }

    fn get_all_rates(&self, chain: &ChainId) -> Vec<(ChainId, ExchangeRate)> {
        self.rates
            .iter()
            .filter(|((f, _), _)| f == chain)
            .map(|((_, t), rate)| (t.clone(), *rate))
            .collect()
    }

    fn update_rate(&mut self, from: ChainId, to: ChainId, rate: ExchangeRate) -> Result<(), OracleError> {
        // Find existing rate or add new one
        if let Some(existing) = self.rates.iter_mut().find(|((f, t), _)| f == &from && t == &to) {
            existing.1 = rate;
        } else {
            self.rates.push(((from, to), rate));
        }
        Ok(())
    }

    fn supports_pair(&self, from: &ChainId, to: &ChainId) -> bool {
        self.rates.iter().any(|((f, t), _)| f == from && t == to)
    }

    fn oracle_name(&self) -> &str {
        "StaticRateOracle"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_rate_conversion() {
        let rate = ExchangeRate::new(20000, 1000, 100); // 2:1
        assert_eq!(rate.convert(100), 200);

        let rate2 = ExchangeRate::new(5000, 1000, 100); // 0.5:1
        assert_eq!(rate2.convert(100), 50);
    }

    #[test]
    fn test_exchange_rate_staleness() {
        let rate = ExchangeRate::new(10000, 1000, 100);
        assert!(!rate.is_stale(600, 1500)); // Within 10 min
        assert!(rate.is_stale(600, 2000)); // Older than 10 min
    }

    #[test]
    fn test_exchange_rate_reliability() {
        let high_conf = ExchangeRate::new(10000, 1000, 95);
        let low_conf = ExchangeRate::new(10000, 1000, 50);

        assert!(high_conf.is_reliable(80));
        assert!(!low_conf.is_reliable(80));
    }

    #[test]
    fn test_static_rate_oracle() {
        let oracle = StaticRateOracle::new();

        // USDT should be 1:1 across chains
        let rate = oracle.get_exchange_rate(&"eth-pbc".into(), &"trx-pbc".into());
        assert!(rate.is_some());
        assert_eq!(rate.unwrap().rate, 10000); // 1:1
        assert_eq!(rate.unwrap().confidence, 100);
    }

    #[test]
    fn test_static_rate_oracle_supports_pair() {
        let oracle = StaticRateOracle::new();
        assert!(oracle.supports_pair(&"eth-pbc".into(), &"trx-pbc".into()));
        assert!(!oracle.supports_pair(&"eth-pbc".into(), &"random-chain".into()));
    }
}
