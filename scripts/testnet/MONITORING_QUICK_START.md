# Ëtrid Monitoring - Quick Start Guide

## Start Monitoring Stack

### Using Docker Compose (Recommended)

```bash
# Start all services (nodes + monitoring)
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f prometheus grafana
```

### Using Scripts

```bash
# Install monitoring stack (first time only)
./scripts/setup-monitoring-stack.sh

# Start testnet with 3 nodes
./scripts/start-testnet.sh --validators 3

# Check monitoring status
./scripts/monitoring-status.sh
```

## Access Dashboards

| Service | URL | Credentials |
|---------|-----|-------------|
| **Grafana** | http://localhost:3001 | admin / etrid2025 |
| **Prometheus** | http://localhost:9090 | No auth |
| **Alice Metrics** | http://localhost:9615/metrics | Raw metrics |
| **Bob Metrics** | http://localhost:9616/metrics | Raw metrics |
| **Charlie Metrics** | http://localhost:9617/metrics | Raw metrics |

## Key Metrics to Monitor

### Block Production Health
```promql
# Block height (should increase steadily)
substrate_block_height{job=~"flarechain.*"}

# Block production rate (should be ~1 block/sec)
rate(substrate_block_height[5m])

# Finalization lag (should be <10 blocks)
substrate_block_height - substrate_finalized_height
```

### Network Health
```promql
# Connected peers (should be 2-3 per node)
substrate_sub_libp2p_peers_count

# Network latency p95 (should be <100ms)
histogram_quantile(0.95, rate(substrate_sub_libp2p_notifications_total_bucket[5m]))
```

### System Resources
```promql
# CPU usage % (normal: 10-40%)
rate(process_cpu_seconds_total[5m]) * 100

# Memory usage GB (normal: 2-4 GB)
process_resident_memory_bytes / 1024 / 1024 / 1024
```

## Quick Health Checks

```bash
# Check if all services are responding
curl -s http://localhost:9090/-/healthy && echo "Prometheus: OK"
curl -s http://localhost:3001/api/health && echo "Grafana: OK"
curl -s http://localhost:9615/metrics | head -1 && echo "Alice: OK"
curl -s http://localhost:9616/metrics | head -1 && echo "Bob: OK"
curl -s http://localhost:9617/metrics | head -1 && echo "Charlie: OK"

# Check Prometheus targets status
curl -s http://localhost:9090/api/v1/targets | jq '.data.activeTargets[] | {job: .labels.job, health: .health}'

# View active alerts
curl -s http://localhost:9090/api/v1/alerts | jq '.data.alerts[] | {name: .labels.alertname, state: .state}'
```

## Common Operations

### Restart Services
```bash
# Restart all
docker-compose restart

# Restart specific service
docker-compose restart prometheus
docker-compose restart grafana
docker-compose restart validator-alice
```

### View Logs
```bash
# Tail all logs
docker-compose logs -f

# Specific service logs
docker-compose logs -f prometheus
docker-compose logs -f grafana
docker-compose logs -f validator-alice

# Last 100 lines
docker-compose logs --tail=100 prometheus
```

### Stop Services
```bash
# Stop all services
docker-compose down

# Stop and remove volumes (clean slate)
docker-compose down -v
```

### Update Configuration
```bash
# After editing prometheus.yml or alerting-rules.yml
docker-compose restart prometheus

# After editing grafana-dashboard.json
docker-compose restart grafana

# Reload Prometheus config (without restart)
curl -X POST http://localhost:9090/-/reload
```

## Troubleshooting Quick Fixes

### No Data in Grafana
```bash
# 1. Check Prometheus is scraping
curl http://localhost:9090/api/v1/targets

# 2. Check nodes are exposing metrics
curl http://localhost:9615/metrics | grep substrate_block_height

# 3. Test Prometheus datasource in Grafana
# Go to: Configuration > Data Sources > Prometheus > Test
```

### Alerts Not Working
```bash
# Check alert rules are loaded
curl http://localhost:9090/api/v1/rules | jq '.data.groups[].rules[].name'

# Validate alerting rules syntax
docker exec etrid-prometheus promtool check rules /etc/prometheus/alerting-rules.yml
```

### High Memory Usage
```bash
# Check Prometheus memory
docker stats etrid-prometheus

# Reduce retention (edit docker-compose.yml)
# Change: --storage.tsdb.retention.time=30d
# To:     --storage.tsdb.retention.time=7d

# Restart Prometheus
docker-compose restart prometheus
```

### Dashboard Not Loading
```bash
# Check Grafana logs
docker-compose logs grafana | tail -50

# Restart Grafana
docker-compose restart grafana

# Clear Grafana cache
docker exec etrid-grafana rm -rf /var/lib/grafana/cache/*
docker-compose restart grafana
```

## Expected Metric Values (Testnet)

| Metric | Expected Range | Critical Threshold |
|--------|---------------|-------------------|
| Block production rate | 0.8 - 1.2 blocks/sec | < 0.5 blocks/sec |
| Finalization lag | 0 - 5 blocks | > 10 blocks |
| Peer count | 2 - 4 peers | < 2 peers |
| CPU usage | 10 - 40% | > 80% |
| Memory usage | 2 - 4 GB | > 8 GB |
| Network latency (p95) | 10 - 100 ms | > 500 ms |
| Transaction pool | 0 - 100 txs | > 1000 txs |

## Alert Severity Guide

### Critical Alerts (Immediate Action)
- NoBlocksProduced
- FinalizationStalled
- NoPeersConnected
- ValidatorOffline
- PPFASealingFailure

### Warning Alerts (Check Soon)
- SlowBlockProduction
- FinalizationLag
- LowPeerCount
- HighMemoryUsage
- HighCPUUsage

### Info Alerts (FYI)
- SubstrateRuntimeUpgraded

## Quick Reference Commands

```bash
# Get current block height
curl -s http://localhost:9615/metrics | grep '^substrate_block_height '

# Get peer count
curl -s http://localhost:9615/metrics | grep '^substrate_sub_libp2p_peers_count '

# Get memory usage (MB)
curl -s http://localhost:9615/metrics | grep '^process_resident_memory_bytes' | awk '{print $2/1024/1024 " MB"}'

# Query Prometheus for latest block height
curl -s 'http://localhost:9090/api/v1/query?query=substrate_block_height' | jq -r '.data.result[] | "\(.metric.instance): \(.value[1])"'

# Export Grafana dashboard
curl -s -H "Authorization: Bearer <api-key>" http://localhost:3001/api/dashboards/uid/<dashboard-uid> | jq .

# Backup Prometheus data
docker run --rm -v etrid_prometheus-data:/data -v $(pwd):/backup ubuntu tar czf /backup/prometheus-backup.tar.gz /data
```

## Performance Optimization Tips

1. **Reduce scrape interval** for less active metrics (30s instead of 15s)
2. **Use recording rules** for expensive queries
3. **Limit retention period** based on disk space (7-30 days)
4. **Enable compression** in Prometheus
5. **Use SSD storage** for Prometheus data directory
6. **Increase Grafana cache** for faster dashboard loading

## Support Resources

- Full Documentation: `/Users/macbook/Desktop/etrid/docs/MONITORING_GUIDE.md`
- Prometheus Config: `/Users/macbook/Desktop/etrid/scripts/testnet/prometheus.yml`
- Alerting Rules: `/Users/macbook/Desktop/etrid/scripts/testnet/alerting-rules.yml`
- Grafana Dashboard: `/Users/macbook/Desktop/etrid/scripts/testnet/grafana-dashboard.json`

---

**Quick Help**: Run `./scripts/monitoring-status.sh` for a summary of monitoring stack health.
