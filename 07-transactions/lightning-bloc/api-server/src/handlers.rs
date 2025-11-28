use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::state::AppState;

/// Get all Lightning channels
pub async fn get_channels(
    State(state): State<AppState>,
) -> Result<Json<ChannelsResponse>, Response> {
    let channels = state.get_channels().await;
    Ok(Json(ChannelsResponse { channels }))
}

/// Open new Lightning channel
pub async fn open_channel(
    State(state): State<AppState>,
    Json(req): Json<OpenChannelRequest>,
) -> Result<Json<serde_json::Value>, Response> {
    let channel_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let channel = Channel {
        id: channel_id.clone(),
        chain: req.chain,
        counterparty: req.counterparty,
        capacity: req.capacity.clone(),
        local_balance: req.capacity,
        remote_balance: "0".to_string(),
        state: ChannelState::Pending,
        created_at: now,
        updated_at: now,
    };

    state.add_channel(channel).await;

    // Broadcast event
    state.broadcast_event(serde_json::json!({
        "type": "channel_opened",
        "channel_id": channel_id,
    }).to_string()).await;

    Ok(Json(serde_json::json!({
        "channel_id": channel_id,
        "status": "pending",
    })))
}

/// Close Lightning channel
pub async fn close_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
) -> Result<Json<serde_json::Value>, Response> {
    // In production, this would actually close the channel
    // For now, just update the state

    // Broadcast event
    state.broadcast_event(serde_json::json!({
        "type": "channel_closed",
        "channel_id": channel_id,
    }).to_string()).await;

    Ok(Json(serde_json::json!({
        "success": true,
    })))
}

/// Get payment history
pub async fn get_payments(
    State(state): State<AppState>,
) -> Result<Json<PaymentsResponse>, Response> {
    let payments = state.get_payments().await;
    Ok(Json(PaymentsResponse { payments }))
}

/// Find cross-chain route
pub async fn find_route(
    State(state): State<AppState>,
    Json(req): Json<FindRouteRequest>,
) -> Result<Json<RouteResponse>, Response> {
    // In production, this would call the CrossPBCRouter
    // For now, return a mock route

    let now = Utc::now().timestamp();

    // Mock exchange rate (1 ETH = 0.05 BTC)
    let exchange_rate = ExchangeRate {
        rate: 500, // basis points
        timestamp: now,
        source: "mock".to_string(),
    };

    // Calculate destination amount
    let source_amount: u128 = req.amount.parse().unwrap_or(0);
    let dest_amount = source_amount * exchange_rate.rate as u128 / 10000;

    let route = CrossPBCRoute {
        source_chain: req.source_chain.clone(),
        dest_chain: req.dest_chain.clone(),
        segments: vec![RouteSegment {
            from_chain: req.source_chain,
            to_chain: req.dest_chain,
            channel_id: Uuid::new_v4().to_string(),
            amount: dest_amount.to_string(),
            fee: "1000000000000000".to_string(), // 0.001 ETH
            exchange_rate: exchange_rate.clone(),
        }],
        total_fees: "1000000000000000".to_string(),
        estimated_time: 30,
        exchange_rate,
    };

    Ok(Json(RouteResponse { route }))
}

/// Send Lightning payment
pub async fn send_payment(
    State(state): State<AppState>,
    Json(req): Json<SendPaymentRequest>,
) -> Result<Json<PaymentResponse>, Response> {
    let payment_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    // Calculate source amount from route
    let source_amount = req.route.segments.first()
        .map(|s| s.amount.clone())
        .unwrap_or_else(|| "0".to_string());

    let dest_amount = req.route.segments.last()
        .map(|s| s.amount.clone())
        .unwrap_or_else(|| "0".to_string());

    let payment = Payment {
        id: payment_id.clone(),
        payment_type: PaymentType::Send,
        source_chain: req.route.source_chain.clone(),
        dest_chain: req.route.dest_chain.clone(),
        source_address: req.source_address,
        dest_address: req.dest_address,
        source_amount,
        dest_amount,
        status: PaymentStatus::Pending,
        route: Some(req.route),
        timestamp: now,
        error: None,
    };

    state.add_payment(payment.clone()).await;

    // Simulate payment processing
    let state_clone = state.clone();
    let payment_id_clone = payment_id.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Broadcast completion event
        state_clone.broadcast_event(serde_json::json!({
            "type": "payment_update",
            "payment_id": payment_id_clone,
            "status": "completed",
        }).to_string()).await;
    });

    Ok(Json(PaymentResponse {
        payment_id,
        status: "pending".to_string(),
    }))
}

/// Get network statistics
pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<StatsResponse>, Response> {
    let stats = state.get_stats().await;
    Ok(Json(StatsResponse { stats }))
}

/// Get exchange rate between two chains
pub async fn get_exchange_rate(
    State(state): State<AppState>,
    Query(query): Query<RateQuery>,
) -> Result<Json<RateResponse>, Response> {
    // In production, this would query the OracleManager
    // For now, return mock rates

    let now = Utc::now().timestamp();

    let rate = match (query.from.as_str(), query.to.as_str()) {
        ("eth-pbc", "btc-pbc") => 500,      // 0.05
        ("eth-pbc", "sol-pbc") => 200000,   // 20.0
        ("btc-pbc", "eth-pbc") => 200000,   // 20.0
        _ => 10000, // 1:1 default
    };

    Ok(Json(RateResponse {
        rate: ExchangeRate {
            rate,
            timestamp: now,
            source: "mock".to_string(),
        },
    }))
}
