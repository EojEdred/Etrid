# Monitoring Infrastructure Guide

**Date:** October 25, 2025
**Status:** 📋 Implementation guide ready
**Purpose:** Set up production monitoring for Ëtrid FlareChain

---

## Overview

This guide covers setting up a complete monitoring stack for Ëtrid blockchain:
- Prometheus (metrics collection)
- Grafana (visualization dashboards)
- Alertmanager (alerting system)
- Node exporters (system metrics)
- Custom metrics (blockchain-specific)

---

## Architecture

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Validator  │────▶│  Prometheus  │────▶│   Grafana    │
│    Nodes     │     │   (Metrics)  │     │ (Dashboard)  │
└──────────────┘     └──────────────┘     └──────────────┘
       │                     │                      │
       │                     ▼                      │
       │             ┌──────────────┐               │
       │             │ Alertmanager │◀──────────────┘
       │             │  (Alerts)    │
       │             └──────────────┘
       │                     │
       ▼                     ▼
┌──────────────┐     ┌──────────────┐
│ Node Exporter│     │   Email/Slack│
│ (System Metrics)    │   Webhook    │
└──────────────┘     └──────────────┘
```

---

## Part 1: Prometheus Setup

### Install Prometheus

```bash
# Download Prometheus
cd /opt
wget https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
tar xvfz prometheus-*.tar.gz
cd prometheus-*
```

### Configure Prometheus

Create `/etc/prometheus/prometheus.yml`:

```yaml
global:
  scrape_interval: 15s # Scrape targets every 15 seconds
  evaluation_interval: 15s # Evaluate rules every 15 seconds
  external_labels:
    cluster: 'etrid-mainnet'
    environment: 'production'

# Alertmanager configuration
alerting:
  alertmanagers:
    - static_configs:
        - targets: ['localhost:9093']

# Load rules
rule_files:
  - 'alerts.yml'

# Scrape configurations
scrape_configs:
  # Validator node 1 (North America)
  - job_name: 'validator-na-1'
    static_configs:
      - targets: ['validator-na-1.etrid.network:9615']
        labels:
          instance: 'validator-na-1'
          region: 'us-east-1'
          role: 'validator'

  # Validator node 2 (Europe)
  - job_name: 'validator-eu-1'
    static_configs:
      - targets: ['validator-eu-1.etrid.network:9615']
        labels:
          instance: 'validator-eu-1'
          region: 'eu-west-1'
          role: 'validator'

  # Archive node
  - job_name: 'archive-node'
    static_configs:
      - targets: ['archive.etrid.network:9615']
        labels:
          instance: 'archive-1'
          role: 'archive'

  # RPC nodes
  - job_name: 'rpc-nodes'
    static_configs:
      - targets:
        - 'rpc-1.etrid.network:9615'
        - 'rpc-2.etrid.network:9615'
        labels:
          role: 'rpc'

  # Node exporter (system metrics)
  - job_name: 'node-exporter'
    static_configs:
      - targets:
        - 'validator-na-1.etrid.network:9100'
        - 'validator-eu-1.etrid.network:9100'
        - 'archive.etrid.network:9100'

  # Prometheus itself
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
```

### Configure Alerts

Create `/etc/prometheus/alerts.yml`:

```yaml
groups:
  - name: blockchain_alerts
    interval: 30s
    rules:
      # Node down alert
      - alert: NodeDown
        expr: up == 0
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Node {{ $labels.instance }} is down"
          description: "{{ $labels.instance }} has been down for more than 5 minutes."

      # Block production stopped
      - alert: BlockProductionStopped
        expr: increase(substrate_block_height[5m]) == 0
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Block production stopped on {{ $labels.instance }}"
          description: "No new blocks produced in 5 minutes."

      # High memory usage
      - alert: HighMemoryUsage
        expr: (node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes) < 0.1
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage on {{ $labels.instance }}"
          description: "Less than 10% memory available."

      # High CPU usage
      - alert: HighCPUUsage
        expr: 100 - (avg by (instance) (rate(node_cpu_seconds_total{mode="idle"}[5m])) * 100) > 90
        for: 15m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage on {{ $labels.instance }}"
          description: "CPU usage above 90% for 15 minutes."

      # Disk space low
      - alert: DiskSpaceLow
        expr: (node_filesystem_avail_bytes / node_filesystem_size_bytes) < 0.1
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Low disk space on {{ $labels.instance }}"
          description: "Less than 10% disk space available."

      # Peer count low
      - alert: LowPeerCount
        expr: substrate_sub_libp2p_peers_count < 3
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Low peer count on {{ $labels.instance }}"
          description: "Connected to less than 3 peers."

      # Finality lag
      - alert: FinalityLag
        expr: substrate_block_height - substrate_finalized_height > 10
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Finality lag on {{ $labels.instance }}"
          description: "More than 10 blocks behind finalized height."

      # Reserve ratio critical (EDSC oracle)
      - alert: ReserveRatioCritical
        expr: edsc_reserve_ratio < 10000
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "EDSC reserve ratio below 100%"
          description: "Reserve ratio: {{ $value }} basis points (critical threshold)."

      # Oracle price stale
      - alert: OraclePriceStale
        expr: time() - edsc_oracle_last_update > 3600
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Oracle price data is stale"
          description: "No price update in over 1 hour."
```

### Start Prometheus

```bash
# Create systemd service
sudo tee /etc/systemd/system/prometheus.service > /dev/null <<EOF
[Unit]
Description=Prometheus
Wants=network-online.target
After=network-online.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/opt/prometheus/prometheus \
  --config.file=/etc/prometheus/prometheus.yml \
  --storage.tsdb.path=/var/lib/prometheus/ \
  --web.console.templates=/opt/prometheus/consoles \
  --web.console.libraries=/opt/prometheus/console_libraries

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl enable prometheus
sudo systemctl start prometheus

# Check status
sudo systemctl status prometheus

# Access web UI
http://localhost:9090
```

---

## Part 2: Grafana Setup

### Install Grafana

```bash
# Add Grafana repository
sudo apt-get install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -

# Install
sudo apt-get update
sudo apt-get install grafana

# Start service
sudo systemctl enable grafana-server
sudo systemctl start grafana-server

# Access web UI (default credentials: admin/admin)
http://localhost:3000
```

### Configure Data Source

1. Log in to Grafana (http://localhost:3000)
2. Go to Configuration → Data Sources
3. Click "Add data source"
4. Select "Prometheus"
5. Configure:
   - Name: Prometheus
   - URL: http://localhost:9090
   - Access: Server (default)
6. Click "Save & Test"

### Import Substrate Dashboard

**Option A: Use official Substrate dashboard**

1. Go to Dashboards → Import
2. Enter dashboard ID: `13759` (Substrate Node Metrics)
3. Select Prometheus data source
4. Click "Import"

**Option B: Create custom Ëtrid dashboard**

Create `/etc/grafana/dashboards/etrid-mainnet.json`:

```json
{
  "dashboard": {
    "title": "Ëtrid FlareChain Mainnet",
    "panels": [
      {
        "title": "Block Height",
        "targets": [
          {
            "expr": "substrate_block_height{job=\"validator-na-1\"}"
          }
        ],
        "type": "graph"
      },
      {
        "title": "Finalized Height",
        "targets": [
          {
            "expr": "substrate_finalized_height"
          }
        ],
        "type": "graph"
      },
      {
        "title": "Peer Count",
        "targets": [
          {
            "expr": "substrate_sub_libp2p_peers_count"
          }
        ],
        "type": "graph"
      },
      {
        "title": "EDSC Reserve Ratio",
        "targets": [
          {
            "expr": "edsc_reserve_ratio / 100"
          }
        ],
        "type": "gauge",
        "fieldConfig": {
          "defaults": {
            "unit": "percent",
            "thresholds": {
              "mode": "absolute",
              "steps": [
                {"value": 0, "color": "red"},
                {"value": 100, "color": "yellow"},
                {"value": 110, "color": "green"}
              ]
            }
          }
        }
      },
      {
        "title": "Treasury Balance",
        "targets": [
          {
            "expr": "treasury_balance / 1000000000000"
          }
        ],
        "type": "stat",
        "fieldConfig": {
          "defaults": {
            "unit": "ÉTR"
          }
        }
      }
    ]
  }
}
```

---

## Part 3: Node Exporter Setup

### Install Node Exporter (on each node)

```bash
# Download node exporter
cd /opt
wget https://github.com/prometheus/node_exporter/releases/download/v1.6.0/node_exporter-1.6.0.linux-amd64.tar.gz
tar xvfz node_exporter-*.tar.gz
cd node_exporter-*

# Create systemd service
sudo tee /etc/systemd/system/node-exporter.service > /dev/null <<EOF
[Unit]
Description=Node Exporter
Wants=network-online.target
After=network-online.target

[Service]
User=node_exporter
Group=node_exporter
Type=simple
ExecStart=/opt/node_exporter/node_exporter

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl enable node-exporter
sudo systemctl start node-exporter

# Verify
curl http://localhost:9100/metrics
```

---

## Part 4: Substrate Metrics Configuration

### Enable Metrics in Ëtrid Node

Start validator with metrics enabled:

```bash
./etrid \
  --validator \
  --chain flarechain-mainnet-raw.json \
  --name "Foundation-NA-1" \
  --prometheus-external \
  --prometheus-port 9615 \
  --base-path /data/etrid \
  --rpc-port 9933 \
  --port 30333
```

**Key flags:**
- `--prometheus-external`: Expose metrics externally (not just localhost)
- `--prometheus-port 9615`: Metrics port (default: 9615)

### Custom Metrics (EDSC Oracle)

Add custom metrics to pallet-reserve-oracle:

```rust
// In pallet-reserve-oracle/src/lib.rs

use prometheus::{Registry, Gauge, Counter};

pub struct Metrics {
    pub reserve_ratio: Gauge,
    pub total_reserves: Gauge,
    pub last_update: Gauge,
}

impl Metrics {
    pub fn register(registry: &Registry) -> Result<Self, prometheus::Error> {
        Ok(Self {
            reserve_ratio: Gauge::new(
                "edsc_reserve_ratio",
                "EDSC reserve ratio in basis points"
            )?.register_with(registry)?,

            total_reserves: Gauge::new(
                "edsc_total_reserves",
                "Total EDSC reserves in USD cents"
            )?.register_with(registry)?,

            last_update: Gauge::new(
                "edsc_oracle_last_update",
                "Timestamp of last oracle update"
            )?.register_with(registry)?,
        })
    }
}

// Update metrics on each snapshot
fn update_metrics(snapshot: &ReserveSnapshot) {
    if let Some(metrics) = METRICS.as_ref() {
        metrics.reserve_ratio.set(snapshot.reserve_ratio as f64);
        metrics.total_reserves.set(snapshot.total_reserves as f64);
        metrics.last_update.set(snapshot.timestamp as f64);
    }
}
```

---

## Part 5: Alertmanager Setup

### Install Alertmanager

```bash
# Download Alertmanager
cd /opt
wget https://github.com/prometheus/alertmanager/releases/download/v0.26.0/alertmanager-0.26.0.linux-amd64.tar.gz
tar xvfz alertmanager-*.tar.gz
cd alertmanager-*
```

### Configure Alertmanager

Create `/etc/alertmanager/alertmanager.yml`:

```yaml
global:
  resolve_timeout: 5m
  slack_api_url: 'YOUR_SLACK_WEBHOOK_URL'

route:
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'team-notifications'
  routes:
    - match:
        severity: critical
      receiver: 'critical-alerts'
    - match:
        severity: warning
      receiver: 'warning-alerts'

receivers:
  - name: 'team-notifications'
    slack_configs:
      - channel: '#etrid-alerts'
        title: 'Ëtrid Monitoring Alert'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'

  - name: 'critical-alerts'
    slack_configs:
      - channel: '#etrid-critical'
        title: '🚨 CRITICAL ALERT'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'
    email_configs:
      - to: 'ops@etrid.network'
        from: 'alertmanager@etrid.network'
        smarthost: 'smtp.gmail.com:587'
        auth_username: 'alertmanager@etrid.network'
        auth_password: 'YOUR_PASSWORD'

  - name: 'warning-alerts'
    slack_configs:
      - channel: '#etrid-warnings'
        title: '⚠️  Warning Alert'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'

inhibit_rules:
  - source_match:
      severity: 'critical'
    target_match:
      severity: 'warning'
    equal: ['alertname', 'instance']
```

### Start Alertmanager

```bash
# Create systemd service
sudo tee /etc/systemd/system/alertmanager.service > /dev/null <<EOF
[Unit]
Description=Alertmanager
Wants=network-online.target
After=network-online.target

[Service]
User=alertmanager
Group=alertmanager
Type=simple
ExecStart=/opt/alertmanager/alertmanager \
  --config.file=/etc/alertmanager/alertmanager.yml \
  --storage.path=/var/lib/alertmanager/

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl enable alertmanager
sudo systemctl start alertmanager

# Access web UI
http://localhost:9093
```

---

## Part 6: Quick Deployment Script

Create `scripts/deploy-monitoring.sh`:

```bash
#!/bin/bash
set -e

echo "🚀 Deploying Ëtrid monitoring stack..."

# Install dependencies
sudo apt-get update
sudo apt-get install -y wget curl

# Install Prometheus
echo "📊 Installing Prometheus..."
cd /opt
wget -q https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
tar xzf prometheus-*.tar.gz
sudo mv prometheus-* prometheus
sudo useradd --no-create-home --shell /bin/false prometheus
sudo mkdir -p /var/lib/prometheus
sudo chown -R prometheus:prometheus /var/lib/prometheus

# Install Grafana
echo "📈 Installing Grafana..."
sudo apt-get install -y software-properties-common
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
sudo apt-get update
sudo apt-get install -y grafana

# Install Node Exporter
echo "💻 Installing Node Exporter..."
cd /opt
wget -q https://github.com/prometheus/node_exporter/releases/download/v1.6.0/node_exporter-1.6.0.linux-amd64.tar.gz
tar xzf node_exporter-*.tar.gz
sudo mv node_exporter-* node_exporter
sudo useradd --no-create-home --shell /bin/false node_exporter

# Install Alertmanager
echo "🔔 Installing Alertmanager..."
cd /opt
wget -q https://github.com/prometheus/alertmanager/releases/download/v0.26.0/alertmanager-0.26.0.linux-amd64.tar.gz
tar xzf alertmanager-*.tar.gz
sudo mv alertmanager-* alertmanager
sudo useradd --no-create-home --shell /bin/false alertmanager
sudo mkdir -p /var/lib/alertmanager
sudo chown -R alertmanager:alertmanager /var/lib/alertmanager

# Start services
echo "▶️  Starting services..."
sudo systemctl daemon-reload
sudo systemctl enable prometheus grafana-server node-exporter alertmanager
sudo systemctl start prometheus grafana-server node-exporter alertmanager

echo "✅ Monitoring stack deployed!"
echo ""
echo "Access points:"
echo "  Prometheus: http://localhost:9090"
echo "  Grafana:    http://localhost:3000 (admin/admin)"
echo "  Alertmanager: http://localhost:9093"
echo ""
echo "Next steps:"
echo "  1. Configure Grafana data source (Prometheus)"
echo "  2. Import Substrate dashboard (ID: 13759)"
echo "  3. Configure Slack webhook in alertmanager.yml"
echo "  4. Start Ëtrid node with --prometheus-external"
```

Make executable:
```bash
chmod +x scripts/deploy-monitoring.sh
```

---

## Part 7: Key Metrics to Monitor

### Blockchain Metrics

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| `substrate_block_height` | Current block number | No increase in 5min |
| `substrate_finalized_height` | Finalized block number | >10 blocks behind |
| `substrate_sub_libp2p_peers_count` | Connected peers | <3 peers |
| `substrate_ready_transactions_number` | Pending transactions | >1000 |
| `substrate_block_import_time` | Block import duration | >5 seconds |

### System Metrics

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| `node_cpu_seconds_total` | CPU usage | >90% for 15min |
| `node_memory_MemAvailable_bytes` | Available memory | <10% |
| `node_filesystem_avail_bytes` | Available disk space | <10% |
| `node_network_receive_bytes_total` | Network ingress | N/A (trending) |
| `node_network_transmit_bytes_total` | Network egress | N/A (trending) |

### EDSC Oracle Metrics

| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| `edsc_reserve_ratio` | Reserve ratio (basis points) | <10000 (100%) |
| `edsc_total_reserves` | Total reserves (USD cents) | N/A (trending) |
| `edsc_oracle_last_update` | Last update timestamp | >1 hour stale |
| `edsc_price_sources_count` | Active price sources | <3 sources |

---

## Summary

**Monitoring stack includes:**
- ✅ Prometheus (metrics collection)
- ✅ Grafana (visualization)
- ✅ Alertmanager (alerting)
- ✅ Node Exporter (system metrics)
- ✅ Custom EDSC metrics

**Deployment time:** ~30 minutes
**Cost:** Free (open-source)
**Maintenance:** Low (automated)

**Status:** 📋 Guide complete, ready for deployment
