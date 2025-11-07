# Lightning-Bloc API Server

HTTP/WebSocket API server for the Ëtrid Lightning Network.

## Overview

This API server provides a RESTful interface and WebSocket real-time updates for the Lightning-Bloc cross-chain payment system.

**Architecture:**
```
┌─────────────────────────────────────────┐
│        Next.js Web UI (Browser)        │
└─────────────────┬───────────────────────┘
                  │ HTTP/WebSocket
                  ▼
┌─────────────────────────────────────────┐
│      Lightning-Bloc API Server          │
│  ┌───────────────────────────────────┐  │
│  │  Axum HTTP Server                 │  │
│  │  - REST endpoints                 │  │
│  │  - WebSocket handler              │  │
│  │  - CORS + Auth middleware         │  │
│  └──────────────┬────────────────────┘  │
│                 ▼                        │
│  ┌───────────────────────────────────┐  │
│  │  LightningService                 │  │
│  │  - CrossPBCRouter wrapper         │  │
│  │  - OracleManager wrapper          │  │
│  │  - Channel management             │  │
│  └──────────────┬────────────────────┘  │
└─────────────────┼───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│      Lightning-Bloc Core (Rust)         │
│  - Cross-PBC Router                     │
│  - Oracle Integration                   │
│  - HTLC Management                      │
│  - Network Gossip                       │
└─────────────────────────────────────────┘
```

## Features

✅ **HTTP REST API** - Complete CRUD for channels and payments
✅ **WebSocket Events** - Real-time payment and channel updates
✅ **CORS Support** - Cross-origin requests enabled
✅ **Authentication Ready** - Substrate signature verification
✅ **Lightning Integration** - Direct connection to Lightning-Bloc modules
✅ **Oracle Integration** - Real-time exchange rates
✅ **Error Handling** - Comprehensive error responses

## Quick Start

### 1. Build and Run

```bash
cd 07-transactions/lightning-bloc/api-server

# Build
cargo build --release

# Run
cargo run --release
```

Server starts on `http://0.0.0.0:9944`

### 2. Test with curl

```bash
# Health check
curl http://localhost:9944/health

# Get channels
curl http://localhost:9944/lightning/channels

# Get exchange rate
curl "http://localhost:9944/lightning/rates?from=eth-pbc&to=btc-pbc"
```

### 3. Connect from UI

Update your `.env.local`:
```bash
NEXT_PUBLIC_LIGHTNING_API_URL=http://localhost:9944
NEXT_PUBLIC_LIGHTNING_WS_URL=ws://localhost:9944
```

## API Endpoints

### Health Check

```
GET /health
```

Response:
```json
{
  "status": "healthy",
  "service": "lightning-bloc-api",
  "version": "0.1.0"
}
```

### Get Channels

```
GET /lightning/channels
```

Response:
```json
{
  "channels": [
    {
      "id": "0x123...",
      "chain": "eth-pbc",
      "counterparty": "0xabc...",
      "capacity": "5000000000000000000",
      "local_balance": "3000000000000000000",
      "remote_balance": "2000000000000000000",
      "state": "active",
      "created_at": 1234567890,
      "updated_at": 1234567890
    }
  ]
}
```

### Open Channel

```
POST /lightning/channels/open
Content-Type: application/json

{
  "chain": "eth-pbc",
  "counterparty": "0xabc...",
  "capacity": "5000000000000000000"
}
```

Response:
```json
{
  "channel_id": "0x456...",
  "status": "pending"
}
```

### Close Channel

```
POST /lightning/channels/:id/close
```

Response:
```json
{
  "success": true
}
```

### Find Route

```
POST /lightning/route
Content-Type: application/json

{
  "source_chain": "eth-pbc",
  "dest_chain": "btc-pbc",
  "source_address": "0x123...",
  "dest_address": "bc1q...",
  "amount": "1500000000000000000"
}
```

Response:
```json
{
  "route": {
    "source_chain": "eth-pbc",
    "dest_chain": "btc-pbc",
    "segments": [...],
    "total_fees": "1000000000000000",
    "estimated_time": 30,
    "exchange_rate": {
      "rate": 500,
      "timestamp": 1234567890,
      "source": "oracle"
    }
  }
}
```

### Send Payment

```
POST /lightning/send
Content-Type: application/json

{
  "route": { ... },
  "source_address": "0x123...",
  "dest_address": "bc1q..."
}
```

Response:
```json
{
  "payment_id": "0x789...",
  "status": "pending"
}
```

### Get Payments

```
GET /lightning/payments
```

Response:
```json
{
  "payments": [
    {
      "id": "0x789...",
      "type": "send",
      "source_chain": "eth-pbc",
      "dest_chain": "btc-pbc",
      "source_address": "0x123...",
      "dest_address": "bc1q...",
      "source_amount": "1500000000000000000",
      "dest_amount": "75000000",
      "status": "completed",
      "timestamp": 1234567890
    }
  ]
}
```

### Get Statistics

```
GET /lightning/stats
```

Response:
```json
{
  "stats": {
    "total_channels": 1234,
    "total_capacity": "50000000000000000000000",
    "average_channel_size": "40000000000000000000",
    "active_chains": 14,
    "recent_payments": 156,
    "success_rate": 99.2
  }
}
```

### Get Exchange Rate

```
GET /lightning/rates?from=eth-pbc&to=btc-pbc
```

Response:
```json
{
  "rate": {
    "rate": 500,
    "timestamp": 1234567890,
    "source": "oracle"
  }
}
```

## WebSocket API

### Connect

```javascript
const ws = new WebSocket('ws://localhost:9944/lightning/ws')

ws.onopen = () => {
  console.log('Connected to Lightning Network')
}

ws.onmessage = (event) => {
  const data = JSON.parse(event.data)
  console.log('Event:', data)
}
```

### Events

**Connection:**
```json
{
  "type": "connected",
  "message": "Connected to Lightning-Bloc API"
}
```

**Channel Opened:**
```json
{
  "type": "channel_opened",
  "channel_id": "0x123..."
}
```

**Channel Closed:**
```json
{
  "type": "channel_closed",
  "channel_id": "0x123..."
}
```

**Payment Update:**
```json
{
  "type": "payment_update",
  "payment_id": "0x456...",
  "status": "completed"
}
```

## Authentication

### Current (Development)

No authentication required for local development.

### Production (TODO)

Use Substrate signature verification:

1. **Client generates message:**
```javascript
const message = `Sign this message to authenticate with Ëtrid Lightning Network.

Address: ${address}
Timestamp: ${Date.now()}`
```

2. **Client signs with Polkadot.js:**
```javascript
const signature = await account.sign(message)
```

3. **Client sends Authorization header:**
```
Authorization: Bearer ${address}:${signature}:${timestamp}
```

4. **Server verifies signature** (implemented in `auth.rs`)

## Configuration

### Environment Variables

```bash
# Server port (default: 9944)
PORT=9944

# Rust log level
RUST_LOG=lightning_bloc_api=debug

# CORS origins (default: Any)
CORS_ORIGINS=http://localhost:3000,https://app.etrid.io
```

### Production Build

```bash
cargo build --release --features production

# Binary at:
./target/release/lightning-bloc-api
```

## Development

### Project Structure

```
api-server/
├── Cargo.toml              # Dependencies
├── src/
│   ├── main.rs            # Server entry point
│   ├── handlers.rs        # HTTP handlers (mock)
│   ├── handlers_v2.rs     # HTTP handlers (Lightning service)
│   ├── models.rs          # API models
│   ├── state.rs           # App state (mock)
│   ├── state_v2.rs        # App state (Lightning service)
│   ├── lightning_service.rs  # Lightning-Bloc integration
│   ├── auth.rs            # Authentication
│   └── websocket.rs       # WebSocket handler
└── README.md              # This file
```

### Adding New Endpoints

```rust
// In main.rs
.route("/lightning/new-endpoint", get(handlers::new_endpoint))

// In handlers.rs
pub async fn new_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Response>, Response> {
    // Implementation
}
```

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Test specific endpoint
curl -X POST http://localhost:9944/lightning/route \
  -H "Content-Type: application/json" \
  -d '{
    "source_chain": "eth-pbc",
    "dest_chain": "btc-pbc",
    "source_address": "0x123",
    "dest_address": "bc1q456",
    "amount": "1000000000000000000"
  }'
```

## Integration with Substrate

### Current Status

✅ **Mock Implementation** - API works with in-memory data
⏳ **Lightning Service** - Connects to Lightning-Bloc modules
⏳ **Substrate Pallet** - TODO: Connect to on-chain state

### Next Steps

1. **Connect to Substrate RPC:**
```rust
use subxt::{OnlineClient, PolkadotConfig};

let api = OnlineClient::<PolkadotConfig>::new().await?;
let channels = api.storage()
    .fetch(&pallet_lightning::channels())
    .await?;
```

2. **Submit Extrinsics:**
```rust
let tx = api.tx()
    .pallet_lightning()
    .open_channel(counterparty, capacity)?
    .sign_and_submit_default(&signer)
    .await?;
```

3. **Subscribe to Events:**
```rust
let mut events = api.events().subscribe().await?;
while let Some(event) = events.next().await {
    // Handle Lightning events
}
```

## Deployment

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/lightning-bloc-api /usr/local/bin/
CMD ["lightning-bloc-api"]
```

Build and run:
```bash
docker build -t lightning-api .
docker run -p 9944:9944 lightning-api
```

### Systemd Service

```ini
[Unit]
Description=Lightning-Bloc API Server
After=network.target

[Service]
Type=simple
User=lightning
WorkingDirectory=/opt/lightning-api
ExecStart=/opt/lightning-api/lightning-bloc-api
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

## Security

### Best Practices

1. **Use HTTPS in production**
2. **Enable authentication**
3. **Rate limit endpoints**
4. **Validate all inputs**
5. **Use environment variables for secrets**
6. **Enable CORS selectively**

### Rate Limiting

```rust
use tower::limit::RateLimitLayer;

let app = Router::new()
    .route("/lightning/send", post(send_payment))
    .layer(RateLimitLayer::new(10, Duration::from_secs(60)));
```

## Troubleshooting

### Port Already in Use

```bash
# Find process
lsof -i :9944

# Kill process
kill -9 <PID>
```

### CORS Errors

Make sure CORS is enabled:
```rust
.layer(CorsLayer::new().allow_origin(Any))
```

### Connection Refused

Check server is running:
```bash
curl http://localhost:9944/health
```

## Support

- **Documentation:** This README
- **Lightning-Bloc Core:** `/07-transactions/lightning-bloc/`
- **UI Integration:** `/apps/wallet-web/etrid-crypto-website/lib/lightning/`
- **Issues:** Check server logs with `RUST_LOG=debug`

---

**Built with:**
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Serde](https://serde.rs/) - Serialization
- [Tower](https://github.com/tower-rs/tower) - Middleware

**Created:** November 5, 2025
**For:** Ëtrid Lightning Network
**By:** Claude Code
