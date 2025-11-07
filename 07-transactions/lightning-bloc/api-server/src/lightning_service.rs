use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use etrid_lightning_bloc::{
    CrossPBCRouter, OracleManager, MockOracle, LightningPriceOracle,
    ExchangeRate as LightningExchangeRate,
};

use crate::models::*;

/// Lightning service that wraps the Lightning-Bloc Rust modules
pub struct LightningService {
    /// Cross-PBC Router for multi-chain payments
    pub router: Arc<RwLock<CrossPBCRouter>>,

    /// Oracle manager for exchange rates
    pub oracle_manager: Arc<RwLock<OracleManager>>,

    /// Active channels (indexed by chain)
    channels: Arc<RwLock<HashMap<String, Vec<Channel>>>>,
}

impl LightningService {
    /// Create new Lightning service
    pub fn new() -> Self {
        // Initialize oracle manager with mock oracle
        let mut oracle_manager = OracleManager::new(600, 80);
        oracle_manager.add_oracle(Box::new(MockOracle::new()));

        // Initialize Cross-PBC Router with oracles
        let router = CrossPBCRouter::with_oracles(oracle_manager.clone());

        Self {
            router: Arc::new(RwLock::new(router)),
            oracle_manager: Arc::new(RwLock::new(oracle_manager)),
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Find cross-chain route
    pub async fn find_route(
        &self,
        source_chain: &str,
        dest_chain: &str,
        source_address: &str,
        dest_address: &str,
        amount: u128,
    ) -> Result<CrossPBCRoute, String> {
        let router = self.router.read().await;

        // Get current timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Find route using Lightning-Bloc router
        let route = router
            .find_cross_pbc_route(
                &source_chain.to_string(),
                &dest_chain.to_string(),
                source_address,
                dest_address,
                amount,
                timestamp,
            )
            .map_err(|e| format!("Route not found: {}", e))?;

        // Convert to API model
        Ok(self.convert_route(route))
    }

    /// Get exchange rate between two chains
    pub async fn get_exchange_rate(
        &self,
        from_chain: &str,
        to_chain: &str,
    ) -> Result<ExchangeRate, String> {
        let router = self.router.read().await;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let rate = router
            .get_exchange_rate(
                &from_chain.to_string(),
                &to_chain.to_string(),
                timestamp,
            )
            .ok_or_else(|| "Exchange rate not available".to_string())?;

        Ok(ExchangeRate {
            rate: rate.rate,
            timestamp: rate.timestamp as i64,
            source: "oracle".to_string(),
        })
    }

    /// Open Lightning channel
    pub async fn open_channel(
        &self,
        chain: &str,
        counterparty: &str,
        capacity: u128,
    ) -> Result<String, String> {
        // In production, this would:
        // 1. Create on-chain transaction
        // 2. Register channel with Lightning pallet
        // 3. Update router's network graph

        let channel_id = format!("0x{:x}", rand::random::<u64>());

        let mut channels = self.channels.write().await;
        let chain_channels = channels.entry(chain.to_string()).or_insert_with(Vec::new);

        let now = chrono::Utc::now().timestamp();

        chain_channels.push(Channel {
            id: channel_id.clone(),
            chain: chain.to_string(),
            counterparty: counterparty.to_string(),
            capacity: capacity.to_string(),
            local_balance: capacity.to_string(),
            remote_balance: "0".to_string(),
            state: ChannelState::Pending,
            created_at: now,
            updated_at: now,
        });

        Ok(channel_id)
    }

    /// Get all channels
    pub async fn get_channels(&self) -> Vec<Channel> {
        let channels = self.channels.read().await;
        channels
            .values()
            .flat_map(|v| v.clone())
            .collect()
    }

    /// Close channel
    pub async fn close_channel(&self, channel_id: &str) -> Result<(), String> {
        let mut channels = self.channels.write().await;

        for chain_channels in channels.values_mut() {
            if let Some(channel) = chain_channels.iter_mut().find(|c| c.id == channel_id) {
                channel.state = ChannelState::Closing;
                channel.updated_at = chrono::Utc::now().timestamp();
                return Ok(());
            }
        }

        Err("Channel not found".to_string())
    }

    /// Send payment through Lightning Network
    pub async fn send_payment(
        &self,
        route: &CrossPBCRoute,
        source_address: &str,
        dest_address: &str,
    ) -> Result<String, String> {
        // In production, this would:
        // 1. Create HTLCs along the route
        // 2. Lock funds in source channel
        // 3. Forward payment through route segments
        // 4. Claim funds at destination

        let payment_id = format!("0x{:x}", rand::random::<u64>());

        // Simulate payment processing
        // In real implementation, this would interact with the Lightning pallet

        Ok(payment_id)
    }

    /// Convert Lightning-Bloc route to API model
    fn convert_route(&self, route: etrid_lightning_bloc::CrossPBCRoute) -> CrossPBCRoute {
        let segments: Vec<RouteSegment> = route
            .segments
            .iter()
            .map(|seg| RouteSegment {
                from_chain: seg.source_chain.clone(),
                to_chain: seg.dest_chain.clone(),
                channel_id: seg.channel_id.clone(),
                amount: seg.amount.to_string(),
                fee: seg.fee.to_string(),
                exchange_rate: ExchangeRate {
                    rate: seg.exchange_rate.rate,
                    timestamp: seg.exchange_rate.timestamp as i64,
                    source: "oracle".to_string(),
                },
            })
            .collect();

        CrossPBCRoute {
            source_chain: route.source_chain,
            dest_chain: route.dest_chain,
            segments,
            total_fees: route.total_fees.to_string(),
            estimated_time: route.estimated_time,
            exchange_rate: ExchangeRate {
                rate: route.segments.first().map(|s| s.exchange_rate.rate).unwrap_or(10000),
                timestamp: chrono::Utc::now().timestamp(),
                source: "oracle".to_string(),
            },
        }
    }
}

impl Default for LightningService {
    fn default() -> Self {
        Self::new()
    }
}

// Mock random for channel IDs
mod rand {
    pub fn random<T>() -> T
    where
        T: From<u64>,
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        T::from(timestamp)
    }
}
