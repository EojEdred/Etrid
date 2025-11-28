# 📊 Analytics REST API

Production-ready REST API for accessing MasterChef metrics and historical data.

**Integrate MasterChef data into your apps, dashboards, and monitoring tools!**

---

## 🎯 Features

✅ **RESTful API** - Standard HTTP endpoints
✅ **Real-Time Metrics** - Latest MasterChef state
✅ **Historical Data** - TVL, APR trends over time
✅ **Prometheus Integration** - Native metrics export
✅ **CORS Enabled** - Use from web apps
✅ **Health Checks** - Monitor API availability
✅ **Rate Limiting Ready** - Production-safe

---

## 🚀 Quick Start

### Start API Server

```bash
npm run api
```

**Output:**
```
🚀 ANALYTICS API SERVER STARTED

   Port: 3000
   URL: http://localhost:3000

📋 Available Endpoints:

   GET  /api/metrics/:network
   GET  /api/pools/:network
   GET  /api/tvl/:network?days=30
   GET  /api/tvl/:network/pool/:poolId?days=30
   GET  /api/apr/:network/pool/:poolId?days=30
   GET  /api/events/:network?limit=100
   GET  /api/alerts/:network
   GET  /api/stats
   GET  /api/health
   GET  /metrics (Prometheus)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Test Endpoints

```bash
# Latest metrics
curl http://localhost:3000/api/metrics/mainnet

# Latest pools
curl http://localhost:3000/api/pools/mainnet

# TVL history (last 7 days)
curl http://localhost:3000/api/tvl/mainnet?days=7

# Health check
curl http://localhost:3000/api/health
```

---

## 📋 API Endpoints

### GET /api/metrics/:network

Get latest MasterChef metrics for a network.

**Parameters:**
- `network` - `mainnet` or `testnet`

**Example:**
```bash
curl http://localhost:3000/api/metrics/mainnet
```

**Response:**
```json
{
  "success": true,
  "data": {
    "timestamp": "2025-10-24T14:15:32.000Z",
    "network": "mainnet",
    "block_number": 34567890,
    "total_pools": 3,
    "reward_per_block": "11.57",
    "masterchef_balance": "18456789.12",
    "days_remaining": 156,
    "is_paused": false,
    "bnb_price": 312.45,
    "etr_price": 0.000123,
    "total_tvl_usd": 1234567.89
  }
}
```

### GET /api/pools/:network

Get latest pool data for a network.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "pool_id": 0,
      "lp_symbol": "ÉTR-BNB LP",
      "total_staked": "45678.90",
      "tvl_usd": 567890.12,
      "apr_percent": 142.56,
      "reward_share": 50.0
    }
  ]
}
```

### GET /api/tvl/:network?days=30

Get TVL history for a network.

**Query Parameters:**
- `days` - Number of days (default: 30)

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "timestamp": "2025-10-24T14:00:00.000Z",
      "total_tvl_usd": 1234567.89
    }
  ]
}
```

### GET /api/tvl/:network/pool/:poolId?days=30

Get TVL history for a specific pool.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "timestamp": "2025-10-24T14:00:00.000Z",
      "tvl_usd": 567890.12,
      "apr_percent": 142.56,
      "total_staked": "45678.90"
    }
  ]
}
```

### GET /api/apr/:network/pool/:poolId?days=30

Get APR history for a specific pool.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "timestamp": "2025-10-24T14:00:00.000Z",
      "apr_percent": 142.56
    }
  ]
}
```

### GET /api/events/:network?limit=100

Get recent events for a network.

**Query Parameters:**
- `limit` - Number of events (default: 100)

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "timestamp": "2025-10-24T14:15:32.000Z",
      "event_type": "metrics_collected",
      "pool_id": null,
      "details": "{\"snapshot_id\":42}"
    }
  ]
}
```

### GET /api/alerts/:network

Get active alerts for a network.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "timestamp": "2025-10-24T14:00:00.000Z",
      "network": "mainnet",
      "severity": "warning",
      "alert_type": "low_balance",
      "message": "MasterChef balance below 1M ÉTR",
      "acknowledged": false
    }
  ]
}
```

### GET /api/stats

Get database statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "metrics_snapshots": 1234,
    "pool_snapshots": 3702,
    "events": 567,
    "health_checks": 234,
    "alerts": 12,
    "backups": 8
  }
}
```

### GET /api/health

Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "timestamp": "2025-10-24T14:15:32.000Z",
  "database": {
    "connected": true,
    "snapshots": 1234
  }
}
```

### GET /metrics

Prometheus metrics export.

**Response:**
```
# HELP masterchef_balance MasterChef ÉTR balance
# TYPE masterchef_balance gauge
masterchef_balance{network="mainnet"} 18456789.12

# HELP masterchef_days_remaining Days of rewards remaining
# TYPE masterchef_days_remaining gauge
masterchef_days_remaining{network="mainnet"} 156

# HELP masterchef_tvl_total Total Value Locked in USD
# TYPE masterchef_tvl_total gauge
masterchef_tvl_total{network="mainnet"} 1234567.89
```

---

## 🔗 Integration Examples

### JavaScript/TypeScript

```typescript
// Fetch latest metrics
const response = await fetch('http://localhost:3000/api/metrics/mainnet');
const { data } = await response.json();

console.log(`TVL: $${data.total_tvl_usd.toLocaleString()}`);
console.log(`Balance: ${data.masterchef_balance} ÉTR`);
```

### React

```tsx
function MasterChefStats() {
  const [metrics, setMetrics] = useState(null);

  useEffect(() => {
    fetch('http://localhost:3000/api/metrics/mainnet')
      .then(res => res.json())
      .then(({ data }) => setMetrics(data));
  }, []);

  return (
    <div>
      <h2>MasterChef Stats</h2>
      <p>TVL: ${metrics?.total_tvl_usd.toLocaleString()}</p>
      <p>Days Remaining: {metrics?.days_remaining}</p>
    </div>
  );
}
```

### Python

```python
import requests

# Fetch metrics
response = requests.get('http://localhost:3000/api/metrics/mainnet')
data = response.json()['data']

print(f"TVL: ${data['total_tvl_usd']:,.2f}")
print(f"Balance: {data['masterchef_balance']} ÉTR")
```

### cURL

```bash
# Get metrics and extract TVL
curl -s http://localhost:3000/api/metrics/mainnet | jq '.data.total_tvl_usd'
```

---

## 🔧 Configuration

### Environment Variables

```bash
# API port (default: 3000)
API_PORT=3000

# CORS origins (optional, allows all by default)
CORS_ORIGINS=https://yourdomain.com,https://app.yourdomain.com

# Rate limiting (optional)
RATE_LIMIT_WINDOW_MS=60000  # 1 minute
RATE_LIMIT_MAX_REQUESTS=100  # 100 requests per minute
```

---

## 🚀 Production Deployment

### Using PM2

```bash
# Start API server
pm2 start npm --name "analytics-api" -- run api

# Save config
pm2 save

# Setup auto-restart
pm2 startup

# Monitor
pm2 logs analytics-api
```

### Using Docker

```dockerfile
FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --production

COPY . .

EXPOSE 3000

CMD ["npm", "run", "api"]
```

```bash
# Build
docker build -t analytics-api .

# Run
docker run -d \
  --name analytics-api \
  -p 3000:3000 \
  --env-file .env \
  --restart unless-stopped \
  analytics-api

# Logs
docker logs -f analytics-api
```

### Nginx Reverse Proxy

```nginx
server {
    listen 80;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

---

## 📈 Prometheus Integration

### Scrape Configuration

Add to `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'masterchef'
    static_configs:
      - targets: ['localhost:3000']
    metrics_path: '/metrics'
    scrape_interval: 60s
```

### Grafana Dashboard

Import metrics into Grafana:

1. Add Prometheus data source
2. Create dashboard
3. Add panels for TVL, balance, APR, etc.

**Example Query:**
```promql
# TVL over time
masterchef_tvl_total{network="mainnet"}

# Balance remaining
masterchef_balance{network="mainnet"}

# Days until depleted
masterchef_days_remaining{network="mainnet"}
```

---

## 🛡️ Security

### Enable HTTPS

Use a reverse proxy (Nginx, Caddy) with Let's Encrypt.

### Add Authentication

```typescript
// Add to server.ts
const apiKey = process.env.API_KEY;

app.use((req, res, next) => {
  const key = req.headers['x-api-key'];

  if (key !== apiKey) {
    return res.status(401).json({ error: 'Unauthorized' });
  }

  next();
});
```

### Rate Limiting

```typescript
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
  windowMs: 60 * 1000, // 1 minute
  max: 100 // 100 requests per minute
});

app.use(limiter);
```

---

## 📚 Summary

✅ **REST API** with 10+ endpoints
✅ **Real-time metrics** access
✅ **Historical data** queries
✅ **Prometheus export** for monitoring
✅ **Production-ready** with PM2/Docker
✅ **Easy integration** into any app
✅ **CORS enabled** for web apps

**Setup Time**: 2 minutes
**Cost**: $0
**Integrations**: Unlimited

---

**Ready to integrate?** Run `npm run api` and start building! 🚀
