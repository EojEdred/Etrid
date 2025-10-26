# EDSC Bridge Monitoring Stack

Complete monitoring, alerting, and observability infrastructure for the EDSC cross-chain bridge.

## Overview

This monitoring stack includes:
- **Prometheus**: Metrics collection and storage
- **Grafana**: Visualization and dashboards
- **Alertmanager**: Alert routing and notifications
- **Node Exporter**: System metrics
- **cAdvisor**: Container metrics
- **Nginx Exporter**: Web server metrics

## Quick Start

### Prerequisites

- Docker and Docker Compose installed
- Ports available: 9090 (Prometheus), 3000 (Grafana), 9093 (Alertmanager)
- SMTP credentials for email alerts (optional)
- Slack webhook URL (optional)
- PagerDuty service key (optional)

### 1. Configure Environment Variables

Create `.env` file:

```bash
# Grafana
GRAFANA_ADMIN_PASSWORD=your-secure-password

# Email alerts
SMTP_USER=your-email@gmail.com
SMTP_PASSWORD=your-app-password

# Slack alerts (optional)
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/YOUR/WEBHOOK/URL

# PagerDuty (optional)
PAGERDUTY_SERVICE_KEY=your-service-key
PAGERDUTY_SECURITY_KEY=your-security-key
```

### 2. Start Monitoring Stack

```bash
cd monitoring
docker-compose up -d
```

### 3. Access Dashboards

- **Grafana**: http://localhost:3000
  - Username: `admin`
  - Password: (from `.env`)

- **Prometheus**: http://localhost:9090
- **Alertmanager**: http://localhost:9093

### 4. Import Dashboard

The main dashboard is automatically provisioned from `grafana-dashboard.json`.

Navigate to: **Grafana → Dashboards → EDSC Bridge Overview**

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Monitored Services                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Attestation  │  │   Relayer    │  │  Substrate   │      │
│  │  Services    │  │   Services   │  │    Nodes     │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │ /metrics        │ /metrics        │ /metrics      │
│         └─────────────────┴─────────────────┘               │
│                            │                                 │
│                            ▼                                 │
│         ┌──────────────────────────────────┐                │
│         │         Prometheus               │                │
│         │  (Metrics Collection & Storage)  │                │
│         └──────────┬───────────────────────┘                │
│                    │                                         │
│         ┌──────────┴──────────┬────────────────────┐        │
│         ▼                     ▼                    ▼        │
│  ┌─────────────┐       ┌─────────────┐     ┌─────────────┐ │
│  │  Grafana    │       │Alertmanager │     │ Recording   │ │
│  │(Dashboards) │       │  (Alerts)   │     │   Rules     │ │
│  └─────────────┘       └──────┬──────┘     └─────────────┘ │
│                               │                             │
│                    ┌──────────┴──────────┐                  │
│                    ▼                     ▼                  │
│            ┌───────────────┐     ┌──────────────┐          │
│            │  PagerDuty    │     │    Slack     │          │
│            │   (On-call)   │     │  (Alerts)    │          │
│            └───────────────┘     └──────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

---

## Metrics Exposed

### Attestation Service Metrics

Exposed at `https://attestation-X.etrid.io/metrics`:

```
# Service health
attestation_service_up{instance="attestation-0"} 1

# Message processing
attestation_messages_seen_total{source_domain="0"} 142
attestation_messages_seen_total{source_domain="2"} 138
attestation_signatures_created_total 280
attestation_threshold_reached_total 140

# Chain monitoring
attestation_ethereum_block_height 4567890
attestation_substrate_block_height 123456
attestation_ethereum_connected 1
attestation_substrate_connected 1

# API metrics
attestation_api_requests_total{endpoint="/health",status="200"} 5234
attestation_api_request_duration_seconds_bucket{endpoint="/attestation",le="0.1"} 4500
```

### Relayer Service Metrics

Exposed at `https://relayer-X.etrid.io/metrics`:

```
# Relay operations
relayer_messages_relayed_total{destination="ethereum"} 68
relayer_messages_relayed_total{destination="substrate"} 72
relayer_relay_failures_total{destination="ethereum",reason="gas_too_high"} 3
relayer_relay_duration_seconds_sum{destination="ethereum"} 840.5
relayer_relay_duration_seconds_count{destination="ethereum"} 68

# Balances
relayer_balance_eth{instance="relayer-1"} 0.4523
relayer_balance_edsc{instance="relayer-1"} 45.23

# Fetcher
relayer_attestations_fetched_total 140
relayer_attestations_ready_current 0
```

### System Metrics (Node Exporter)

```
# CPU
node_cpu_seconds_total{mode="idle"}
node_cpu_seconds_total{mode="user"}
node_cpu_seconds_total{mode="system"}

# Memory
node_memory_MemTotal_bytes
node_memory_MemAvailable_bytes
node_memory_MemFree_bytes

# Disk
node_filesystem_size_bytes{mountpoint="/"}
node_filesystem_avail_bytes{mountpoint="/"}

# Network
node_network_receive_bytes_total{device="eth0"}
node_network_transmit_bytes_total{device="eth0"}
```

---

## Implementing Metrics in Services

### Adding Metrics to Attestation Service

1. Install Prometheus client:
```bash
npm install prom-client
```

2. Create metrics registry (`src/metrics/index.ts`):
```typescript
import { Registry, Counter, Gauge, Histogram } from 'prom-client';

export const register = new Registry();

// Service up
export const serviceUp = new Gauge({
  name: 'attestation_service_up',
  help: 'Service is running',
  registers: [register],
});
serviceUp.set(1);

// Messages seen
export const messagesSeen = new Counter({
  name: 'attestation_messages_seen_total',
  help: 'Total messages detected',
  labelNames: ['source_domain'],
  registers: [register],
});

// Signatures created
export const signaturesCreated = new Counter({
  name: 'attestation_signatures_created_total',
  help: 'Total signatures created',
  registers: [register],
});

// Chain connectivity
export const ethereumConnected = new Gauge({
  name: 'attestation_ethereum_connected',
  help: 'Connected to Ethereum RPC',
  registers: [register],
});

export const substrateConnected = new Gauge({
  name: 'attestation_substrate_connected',
  help: 'Connected to Substrate RPC',
  registers: [register],
});

// API metrics
export const apiRequests = new Counter({
  name: 'attestation_api_requests_total',
  help: 'Total API requests',
  labelNames: ['endpoint', 'status'],
  registers: [register],
});

export const apiDuration = new Histogram({
  name: 'attestation_api_request_duration_seconds',
  help: 'API request duration',
  labelNames: ['endpoint'],
  buckets: [0.01, 0.05, 0.1, 0.5, 1, 5],
  registers: [register],
});
```

3. Expose `/metrics` endpoint in Express:
```typescript
import { register } from './metrics';

app.get('/metrics', async (req, res) => {
  res.set('Content-Type', register.contentType);
  res.end(await register.metrics());
});
```

4. Instrument your code:
```typescript
import { messagesSeen, signaturesCreated } from './metrics';

// When message detected
messagesSeen.inc({ source_domain: sourceDomain.toString() });

// When signature created
signaturesCreated.inc();
```

### Adding Metrics to Relayer Service

Similar pattern, but with relayer-specific metrics:

```typescript
import { Counter, Gauge, Histogram } from 'prom-client';

export const messagesRelayed = new Counter({
  name: 'relayer_messages_relayed_total',
  help: 'Total messages successfully relayed',
  labelNames: ['destination'],
});

export const relayFailures = new Counter({
  name: 'relayer_relay_failures_total',
  help: 'Total relay failures',
  labelNames: ['destination', 'reason'],
});

export const relayDuration = new Histogram({
  name: 'relayer_relay_duration_seconds',
  help: 'Time to relay message',
  labelNames: ['destination'],
  buckets: [1, 5, 10, 30, 60, 120, 300],
});

export const ethBalance = new Gauge({
  name: 'relayer_balance_eth',
  help: 'ETH balance of relayer',
});

export const edscBalance = new Gauge({
  name: 'relayer_balance_edsc',
  help: 'EDSC balance of relayer',
});

// Update balances periodically
setInterval(async () => {
  const ethBal = await getEthBalance();
  ethBalance.set(parseFloat(ethers.formatEther(ethBal)));
}, 60000); // Every minute
```

---

## Alert Configuration

### Alert Severity Levels

- **Critical**: Immediate action required, page on-call engineer
- **Warning**: Important but not urgent, notify team
- **Info**: Informational, logged only

### Configuring PagerDuty

1. Create PagerDuty service
2. Get integration key
3. Add to `.env`:
```bash
PAGERDUTY_SERVICE_KEY=your-key
```

4. Test:
```bash
curl -X POST https://events.pagerduty.com/v2/enqueue \
  -H 'Content-Type: application/json' \
  -d '{
    "routing_key": "YOUR-KEY",
    "event_action": "trigger",
    "payload": {
      "summary": "Test alert from EDSC Bridge",
      "severity": "critical",
      "source": "monitoring"
    }
  }'
```

### Configuring Slack

1. Create Slack app
2. Add Incoming Webhook
3. Add webhook URL to `.env`:
```bash
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/YOUR/WEBHOOK/URL
```

4. Test:
```bash
curl -X POST $SLACK_WEBHOOK_URL \
  -H 'Content-Type: application/json' \
  -d '{"text":"Test alert from EDSC Bridge monitoring"}'
```

### Email Alerts

Using Gmail:

1. Enable 2FA on Google account
2. Generate app password: https://myaccount.google.com/apppasswords
3. Add to `.env`:
```bash
SMTP_USER=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```

---

## Dashboard Customization

### Adding a Panel

1. Edit `grafana-dashboard.json`
2. Add new panel object:
```json
{
  "id": 17,
  "title": "Your New Metric",
  "type": "graph",
  "gridPos": {"x": 0, "y": 36, "w": 12, "h": 6},
  "targets": [
    {
      "expr": "your_metric_name",
      "legendFormat": "{{label}}",
      "refId": "A"
    }
  ]
}
```

3. Restart Grafana:
```bash
docker-compose restart grafana
```

### Creating Custom Dashboards

In Grafana UI:
1. Click "+" → "Dashboard"
2. Add panels with queries
3. Export JSON: Settings → JSON Model
4. Save to `monitoring/custom-dashboards/`

---

## Troubleshooting

### Prometheus Not Scraping Targets

**Check target status**:
```bash
# Visit Prometheus UI
open http://localhost:9090/targets

# Or via API
curl http://localhost:9090/api/v1/targets | jq
```

**Common issues**:
- Target URL unreachable → Check firewall, DNS
- `/metrics` endpoint 404 → Verify endpoint exists
- SSL certificate errors → Add `insecure_skip_verify: true` to scrape config (testnet only)

### Alerts Not Firing

**Check alert rules**:
```bash
# Visit Prometheus UI
open http://localhost:9090/alerts

# Or via API
curl http://localhost:9090/api/v1/rules | jq
```

**Common issues**:
- Alert expression syntax error → Validate PromQL
- `for` duration not met → Wait longer
- Alert already resolved → Check `repeat_interval`

### Grafana Dashboard Not Loading

**Check datasource**:
1. Grafana → Configuration → Data Sources
2. Click "Prometheus"
3. Click "Test" button

**Common issues**:
- Prometheus unreachable → Check Docker network
- No data in time range → Adjust time picker
- Query timeout → Reduce time range or optimize query

---

## Production Deployment

### 1. Deploy on Dedicated Server

```bash
# Install Docker
curl -fsSL https://get.docker.com | sh

# Clone monitoring configs
cd /opt
git clone https://github.com/etrid/etrid.git
cd etrid/monitoring

# Configure environment
cp .env.example .env
vim .env

# Start stack
docker-compose up -d

# Verify
docker-compose ps
curl http://localhost:9090/-/healthy
curl http://localhost:3000/api/health
```

### 2. Configure Reverse Proxy (Nginx)

```nginx
# /etc/nginx/sites-available/monitoring.etrid.io

server {
    listen 443 ssl http2;
    server_name monitoring.etrid.io;

    ssl_certificate /etc/letsencrypt/live/monitoring.etrid.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/monitoring.etrid.io/privkey.pem;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}

server {
    listen 443 ssl http2;
    server_name prometheus.etrid.io;

    ssl_certificate /etc/letsencrypt/live/prometheus.etrid.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/prometheus.etrid.io/privkey.pem;

    # Basic auth for security
    auth_basic "Prometheus";
    auth_basic_user_file /etc/nginx/.htpasswd;

    location / {
        proxy_pass http://localhost:9090;
        proxy_set_header Host $host;
    }
}
```

### 3. Enable HTTPS with Let's Encrypt

```bash
# Install certbot
apt install certbot python3-certbot-nginx

# Get certificates
certbot --nginx -d monitoring.etrid.io
certbot --nginx -d prometheus.etrid.io

# Auto-renewal is configured automatically
systemctl status certbot.timer
```

### 4. Backup Configuration

```bash
# Backup Prometheus data
docker run --rm \
  -v monitoring_prometheus_data:/data \
  -v /backup:/backup \
  alpine tar czf /backup/prometheus-$(date +%Y%m%d).tar.gz /data

# Backup Grafana data
docker run --rm \
  -v monitoring_grafana_data:/data \
  -v /backup:/backup \
  alpine tar czf /backup/grafana-$(date +%Y%m%d).tar.gz /data
```

---

## Monitoring Best Practices

### 1. Alert Fatigue Prevention

- Set appropriate thresholds
- Use `for` duration to avoid flapping
- Group related alerts
- Use inhibition rules
- Regular alert review (monthly)

### 2. Dashboard Organization

- One dashboard per audience (ops, devs, execs)
- Use template variables for filtering
- Show only actionable metrics
- Include links to runbooks
- Keep it simple

### 3. Metrics Cardinality

- Avoid high-cardinality labels (user IDs, transaction hashes)
- Use recording rules for complex queries
- Set appropriate retention period
- Monitor Prometheus memory usage

### 4. Security

- Enable authentication on Grafana
- Use HTTPS for all endpoints
- Restrict Prometheus to internal network
- Regular credential rotation
- Audit dashboard access logs

---

## Useful Queries

### Bridge Performance

```promql
# Messages per hour
sum(rate(attestation_messages_seen_total[1h])) * 3600

# Average relay time (last hour)
avg(rate(relayer_relay_duration_seconds_sum[1h]) / rate(relayer_relay_duration_seconds_count[1h]))

# Success rate
sum(rate(relayer_messages_relayed_total[5m])) / sum(rate(relayer_relay_attempts_total[5m]))
```

### System Health

```promql
# CPU usage
100 - (avg(rate(node_cpu_seconds_total{mode="idle"}[5m])) * 100)

# Memory usage
(1 - (node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes)) * 100

# Disk usage
(1 - (node_filesystem_avail_bytes / node_filesystem_size_bytes)) * 100
```

### Service Availability

```promql
# Uptime percentage (24h)
avg_over_time(up{job="attestation-service"}[24h]) * 100

# Services down
count(up{job="attestation-service"} == 0)
```

---

## Support

- **Documentation**: See [OPERATIONS.md](../OPERATIONS.md)
- **Issues**: GitHub Issues
- **Runbooks**: https://docs.etrid.io/runbooks/

---

## License

Apache-2.0
