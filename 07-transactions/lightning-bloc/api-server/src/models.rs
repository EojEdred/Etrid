use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Channel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub chain: String,
    pub counterparty: String,
    pub capacity: String,
    pub local_balance: String,
    pub remote_balance: String,
    pub state: ChannelState,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelState {
    Pending,
    Active,
    Closing,
    Closed,
}

/// Payment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    #[serde(rename = "type")]
    pub payment_type: PaymentType,
    pub source_chain: String,
    pub dest_chain: String,
    pub source_address: String,
    pub dest_address: String,
    pub source_amount: String,
    pub dest_amount: String,
    pub status: PaymentStatus,
    pub route: Option<CrossPBCRoute>,
    pub timestamp: i64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentType {
    Send,
    Receive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_channels: u64,
    pub total_capacity: String,
    pub average_channel_size: String,
    pub active_chains: u32,
    pub recent_payments: u64,
    pub success_rate: f64,
}

/// Cross-PBC route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPBCRoute {
    pub source_chain: String,
    pub dest_chain: String,
    pub segments: Vec<RouteSegment>,
    pub total_fees: String,
    pub estimated_time: u64,
    pub exchange_rate: ExchangeRate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSegment {
    pub from_chain: String,
    pub to_chain: String,
    pub channel_id: String,
    pub amount: String,
    pub fee: String,
    pub exchange_rate: ExchangeRate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub rate: u64,
    pub timestamp: i64,
    pub source: String,
}

/// API Requests
#[derive(Debug, Deserialize)]
pub struct OpenChannelRequest {
    pub chain: String,
    pub counterparty: String,
    pub capacity: String,
}

#[derive(Debug, Deserialize)]
pub struct FindRouteRequest {
    pub source_chain: String,
    pub dest_chain: String,
    pub source_address: String,
    pub dest_address: String,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct SendPaymentRequest {
    pub route: CrossPBCRoute,
    pub source_address: String,
    pub dest_address: String,
}

#[derive(Debug, Deserialize)]
pub struct RateQuery {
    pub from: String,
    pub to: String,
}

/// API Responses
#[derive(Debug, Serialize)]
pub struct ChannelsResponse {
    pub channels: Vec<Channel>,
}

#[derive(Debug, Serialize)]
pub struct PaymentsResponse {
    pub payments: Vec<Payment>,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub stats: NetworkStats,
}

#[derive(Debug, Serialize)]
pub struct RouteResponse {
    pub route: CrossPBCRoute,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub payment_id: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct RateResponse {
    pub rate: ExchangeRate,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}
