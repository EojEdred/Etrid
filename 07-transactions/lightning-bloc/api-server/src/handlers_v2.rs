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
use crate::lightning_service::LightningService;

/// Get all Lightning channels (using Lightning service)
pub async fn get_channels_v2(
    State(state): State<AppState>,
) -> Result<Json<ChannelsResponse>, Response> {
    let lightning = state.lightning_service();
    let channels = lightning.get_channels().await;
    Ok(Json(ChannelsResponse { channels }))
}

/// Open new Lightning channel (using Lightning service)
pub async fn open_channel_v2(
    State(state): State<AppState>,
    Json(req): Json<OpenChannelRequest>,
) -> Result<Json<serde_json::Value>, Response> {
    let lightning = state.lightning_service();

    let capacity: u128 = req.capacity.parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid capacity").into_response())?;

    let channel_id = lightning.open_channel(&req.chain, &req.counterparty, capacity)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())?;

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

/// Close Lightning channel (using Lightning service)
pub async fn close_channel_v2(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
) -> Result<Json<serde_json::Value>, Response> {
    let lightning = state.lightning_service();

    lightning.close_channel(&channel_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())?;

    // Broadcast event
    state.broadcast_event(serde_json::json!({
        "type": "channel_closed",
        "channel_id": channel_id,
    }).to_string()).await;

    Ok(Json(serde_json::json!({
        "success": true,
    })))
}

/// Find cross-chain route (using Lightning service)
pub async fn find_route_v2(
    State(state): State<AppState>,
    Json(req): Json<FindRouteRequest>,
) -> Result<Json<RouteResponse>, Response> {
    let lightning = state.lightning_service();

    let amount: u128 = req.amount.parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid amount").into_response())?;

    let route = lightning.find_route(
        &req.source_chain,
        &req.dest_chain,
        &req.source_address,
        &req.dest_address,
        amount,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())?;

    Ok(Json(RouteResponse { route }))
}

/// Send Lightning payment (using Lightning service)
pub async fn send_payment_v2(
    State(state): State<AppState>,
    Json(req): Json<SendPaymentRequest>,
) -> Result<Json<PaymentResponse>, Response> {
    let lightning = state.lightning_service();

    let payment_id = lightning.send_payment(
        &req.route,
        &req.source_address,
        &req.dest_address,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())?;

    let now = Utc::now().timestamp();

    // Store payment in state
    let payment = Payment {
        id: payment_id.clone(),
        payment_type: PaymentType::Send,
        source_chain: req.route.source_chain.clone(),
        dest_chain: req.route.dest_chain.clone(),
        source_address: req.source_address,
        dest_address: req.dest_address,
        source_amount: req.route.segments.first()
            .map(|s| s.amount.clone())
            .unwrap_or_else(|| "0".to_string()),
        dest_amount: req.route.segments.last()
            .map(|s| s.amount.clone())
            .unwrap_or_else(|| "0".to_string()),
        status: PaymentStatus::Pending,
        route: Some(req.route),
        timestamp: now,
        error: None,
    };

    state.add_payment(payment).await;

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

/// Get exchange rate (using Lightning service)
pub async fn get_exchange_rate_v2(
    State(state): State<AppState>,
    Query(query): Query<RateQuery>,
) -> Result<Json<RateResponse>, Response> {
    let lightning = state.lightning_service();

    let rate = lightning.get_exchange_rate(&query.from, &query.to)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())?;

    Ok(Json(RateResponse { rate }))
}
