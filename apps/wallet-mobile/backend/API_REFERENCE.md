# Ëtrid Wallet Backend API Reference

Complete API endpoint documentation with request/response examples.

## Base URL

```
Production: https://api.etrid.io
Development: http://localhost:3000
```

## Authentication

Most endpoints require JWT authentication. Include the token in the Authorization header:

```
Authorization: Bearer <your_jwt_token>
```

---

## Auth Endpoints

### Get Nonce

Get a nonce for signing to prevent replay attacks.

**Endpoint:** `GET /api/v1/auth/nonce/:address`

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Sign this message to authenticate...\n\nNonce: 123456\nTimestamp: 1700000000000",
    "nonce": "123456"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Login

Authenticate with address signature.

**Endpoint:** `POST /api/v1/auth/login`

**Request Body:**
```json
{
  "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "signature": "0x...",
  "message": "Sign this message..."
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "email": "user@example.com",
      "kyc_status": "verified",
      "kyc_level": 2
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expiresIn": "24h"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Refresh Token

Get a new JWT token using refresh token.

**Endpoint:** `POST /api/v1/auth/refresh`

**Request Body:**
```json
{
  "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "token": "new_token...",
    "refreshToken": "new_refresh_token...",
    "expiresIn": "24h"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

---

## Account Endpoints

### Get Balance

Get account balance for all assets.

**Endpoint:** `GET /api/v1/accounts/:address/balance`

**Response:**
```json
{
  "success": true,
  "data": {
    "asset": "ETR",
    "free": "1000000000000000000",
    "reserved": "100000000000000000",
    "total": "1100000000000000000",
    "locked": "0"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Get Portfolio

Get complete portfolio including staked assets.

**Endpoint:** `GET /api/v1/accounts/:address/portfolio`

**Response:**
```json
{
  "success": true,
  "data": {
    "native": {
      "asset": "ETR",
      "free": "1000.5",
      "reserved": "100.2",
      "total": "1100.7"
    },
    "staking": {
      "staked": "500.0",
      "rewards": "25.5"
    },
    "bridged_assets": []
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Get Transactions

Get transaction history (paginated).

**Endpoint:** `GET /api/v1/accounts/:address/transactions?page=1&limit=20`

**Query Parameters:**
- `page` (number, default: 1) - Page number
- `limit` (number, default: 20, max: 100) - Items per page

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "tx_hash": "0x...",
      "block_number": 12345,
      "from_address": "5GrwvaEF...",
      "to_address": "5HpG9w8E...",
      "amount": "100.5",
      "asset": "ETR",
      "fee": "0.01",
      "status": "confirmed",
      "tx_type": "transfer",
      "created_at": "2025-11-18T12:00:00.000Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 100,
    "totalPages": 5
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Submit Transfer

Submit a transfer transaction.

**Endpoint:** `POST /api/v1/accounts/:address/transfer`

**Authentication:** Required

**Request Body:**
```json
{
  "to": "5HpG9w8EfRfYtVkeE3r4zE8v7Qjd9rK9D3cZ8qYt6pGw7VmH",
  "amount": "100.5",
  "asset": "ETR",
  "memo": "Payment for services"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "txHash": "0x...",
    "status": "pending",
    "from": "5GrwvaEF...",
    "to": "5HpG9w8E...",
    "amount": "100.5",
    "asset": "ETR"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

---

## Staking Endpoints

### Get Validators

Get list of active validators.

**Endpoint:** `GET /api/v1/staking/validators`

**Response:**
```json
{
  "success": true,
  "data": {
    "validators": [
      {
        "id": "uuid",
        "address": "5GNJqTPy...",
        "name": "Validator 1",
        "commission_rate": 5.0,
        "total_stake": "1000000.0",
        "apy": 12.5,
        "is_active": true,
        "uptime_percentage": 99.9,
        "delegator_count": 150
      }
    ],
    "count": 50
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Get Staking Positions

Get user's staking positions.

**Endpoint:** `GET /api/v1/staking/:address/positions`

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "validator_address": "5GNJqTPy...",
      "validator_name": "Validator 1",
      "amount": "500.0",
      "rewards_earned": "25.5",
      "apy": 12.5,
      "status": "active",
      "auto_compound": true,
      "start_date": "2025-10-01T00:00:00.000Z"
    }
  ],
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Stake Tokens

Stake tokens with a validator.

**Endpoint:** `POST /api/v1/staking/:address/stake`

**Authentication:** Required

**Request Body:**
```json
{
  "validator_address": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
  "amount": "500.0",
  "auto_compound": true
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "position": {
      "id": "uuid",
      "validator_address": "5GNJqTPy...",
      "amount": "500.0",
      "status": "active"
    },
    "txHash": "0x...",
    "status": "pending"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

---

## Governance Endpoints

### Get Proposals

Get active governance proposals.

**Endpoint:** `GET /api/v1/governance/proposals?page=1&limit=20`

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "title": "Runtime Upgrade v1.5",
      "description": "Upgrade runtime to version 1.5...",
      "proposer_address": "5GrwvaEF...",
      "status": "active",
      "yes_votes": "1000000.0",
      "no_votes": "500000.0",
      "voting_starts_at": "2025-11-01T00:00:00.000Z",
      "voting_ends_at": "2025-11-30T23:59:59.000Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 15,
    "totalPages": 1
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Vote on Proposal

Submit a vote on a proposal.

**Endpoint:** `POST /api/v1/governance/proposals/:id/vote`

**Authentication:** Required

**Request Body:**
```json
{
  "support": true,
  "conviction": 3,
  "balance": "1000.0"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "txHash": "0x...",
    "votingPower": "3000.0",
    "status": "pending"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

---

## ATM Endpoints

### Find Nearby ATMs

Find ATMs within a specified radius.

**Endpoint:** `GET /api/v1/atm/locations?lat=37.7749&lng=-122.4194&radius=5000`

**Query Parameters:**
- `lat` (number, required) - Latitude
- `lng` (number, required) - Longitude
- `radius` (number, default: 5000) - Search radius in meters
- `partner` (string, default: 'all') - Filter by partner

**Response:**
```json
{
  "success": true,
  "data": {
    "locations": [
      {
        "id": "loc123",
        "partner": "Coinme",
        "name": "Coinme ATM - San Francisco",
        "address": "123 Market St, San Francisco, CA",
        "lat": 37.7749,
        "lng": -122.4194,
        "distance": 250,
        "supported_assets": ["BTC", "ETH", "ETR"]
      }
    ],
    "count": 5
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Create ATM Withdrawal

Create a new ATM withdrawal.

**Endpoint:** `POST /api/v1/atm/withdraw`

**Authentication:** Required

**Request Body:**
```json
{
  "amount_usd": 100.0,
  "asset": "ETR",
  "atm_partner": "Coinme",
  "atm_location_id": "loc123"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "withdrawal_code": "ABCD1234EFGH",
    "amount_usd": 100.0,
    "amount_crypto": "40.0",
    "fee": 8.0,
    "exchange_rate": 2.5,
    "expires_at": "2025-11-18T12:30:00.000Z",
    "status": "pending"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

---

## GPU Endpoints

### Search GPUs

Search available GPU instances.

**Endpoint:** `GET /api/v1/gpu/search?min_vram=16&max_price=2.0`

**Query Parameters:**
- `min_vram` (number) - Minimum VRAM in GB
- `max_price` (number) - Maximum price per hour
- `gpu_type` (string) - GPU model filter
- `provider` (string, default: 'all') - Provider filter

**Response:**
```json
{
  "success": true,
  "data": {
    "gpus": [
      {
        "id": "gpu123",
        "name": "NVIDIA RTX 4090",
        "provider": "Vast.ai",
        "vram_gb": 24,
        "gpu_count": 1,
        "cpu_cores": 16,
        "ram_gb": 64,
        "disk_gb": 500,
        "price_per_hour": "1.2",
        "availability": true
      }
    ],
    "count": 10
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

### Rent GPU

Rent a GPU instance.

**Endpoint:** `POST /api/v1/gpu/:id/rent`

**Authentication:** Required

**Request Body:**
```json
{
  "duration_hours": 24
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "rental_id": "uuid",
    "status": "provisioning",
    "total_cost": "28.8",
    "duration_hours": 24,
    "connection": {
      "ssh_host": "12.34.56.78",
      "ssh_port": 22,
      "ssh_username": "root",
      "ssh_password": "secure_password"
    },
    "estimated_ready_time": "2-5 minutes"
  },
  "timestamp": "2025-11-18T12:00:00.000Z"
}
```

---

## Error Codes

| Code | Description |
|------|-------------|
| `VALIDATION_ERROR` | Request validation failed |
| `INVALID_SIGNATURE` | Signature verification failed |
| `UNAUTHORIZED` | Authentication required |
| `FORBIDDEN` | Insufficient permissions |
| `NOT_FOUND` | Resource not found |
| `RATE_LIMIT_EXCEEDED` | Too many requests |
| `INTERNAL_ERROR` | Internal server error |

## Rate Limits

- Global: 100 requests per 15 minutes
- Auth: 5 login attempts per hour
- ATM: 10 withdrawals per day
- Bridge: 20 transfers per hour
