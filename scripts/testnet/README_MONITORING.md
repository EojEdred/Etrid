# Ëtrid Testnet Monitoring Setup

## Overview

This directory contains the monitoring infrastructure configuration for the Ëtrid testnet, including Prometheus metrics collection, Grafana dashboards, and alerting rules.

## Files in This Directory

### Configuration Files

| File | Purpose | Used By |
|------|---------|---------|
| `prometheus.yml` | Prometheus scraping configuration | Prometheus |
| `alerting-rules.yml` | Alert definitions for critical conditions | Prometheus |
| `grafana-dashboard.json` | Comprehensive Grafana dashboard | Grafana |

### Documentation

| File | Description |
|------|-------------|
| `MONITORING_QUICK_START.md` | Quick reference for common operations |
| `README_MONITORING.md` | This file - overview and setup guide |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Ëtrid Monitoring Stack                    │
└─────────────────────────────────────────────────────────────────┘

                         ┌──────────────┐
                         │   Grafana    │
                         │  Port: 3001  │
                         │ (Dashboards) │
                         └──────┬───────┘
                                │
                                │ queries
                                ▼
                         ┌──────────────┐
                         │  Prometheus  │
                         │  Port: 9090  │
                         │  (Metrics)   │
                         └──────┬───────┘
                                │
                    ┌───────────┼───────────┐
                    │           │           │
                    ▼           ▼           ▼
            ┌────────────┬────────────┬────────────┐
            │   Alice    │    Bob     │  Charlie   │
            │ Validator  │ Validator  │ Full Node  │
            │ Port: 9615 │ Port: 9616 │ Port: 9617 │
            │ (Metrics)  │ (Metrics)  │ (Metrics)  │
            └────────────┴────────────┴────────────┘
```

## Quick Setup

### Prerequisites

- Docker and Docker Compose installed
- OR Prometheus and Grafana installed locally
- Ëtrid node binary built

### Start Monitoring (Docker)

```bash
# From project root
docker-compose up -d

# Verify services are running
docker-compose ps

# Expected output:
# etrid-validator-alice    Up
# etrid-validator-bob      Up
# etrid-fullnode-charlie   Up
# etrid-prometheus         Up
# etrid-grafana           Up
```

### Access Dashboards

1. **Grafana Dashboard**: http://localhost:3001
   - Username: `admin`
   - Password: `etrid2025`
   - Navigate to: Dashboards → Ëtrid Testnet - Comprehensive Monitoring

2. **Prometheus UI**: http://localhost:9090
   - Status → Targets (should show 3 nodes UP)
   - Alerts (shows active alerts)
   - Graph (for custom queries)

3. **Raw Metrics**:
   - Alice: http://localhost:9615/metrics
   - Bob: http://localhost:9616/metrics
   - Charlie: http://localhost:9617/metrics

## Configuration Details

### Prometheus (prometheus.yml)

Scrapes metrics from 3 FlareChain nodes:
- **Alice** (Validator): localhost:9615
- **Bob** (Validator): localhost:9616
- **Charlie** (Full Node): localhost:9617

Plus optional PBC collators:
- **BTC PBC**: localhost:9618
- **ETH PBC**: localhost:9619
- **DOGE PBC**: localhost:9620

**Key Settings**:
- Scrape interval: 15 seconds
- Evaluation interval: 15 seconds
- External labels: monitor='etrid-testnet'

### Alerting Rules (alerting-rules.yml)

**6 Alert Groups** covering:

1. **Block Production** (4 rules)
   - No blocks produced
   - Slow block production
   - Finalization lag
   - Finalization stalled

2. **Network** (3 rules)
   - Low peer count
   - No peers connected
   - High network errors

3. **Transaction Pool** (2 rules)
   - Pool nearly full
   - Pool not draining

4. **System Resources** (3 rules)
   - High memory usage
   - High CPU usage
   - Disk space warning

5. **Consensus** (3 rules)
   - Validator offline
   - No block authoring
   - High uncle rate

6. **PPFA Specific** (2 rules)
   - Sealing failures
   - High latency

### Grafana Dashboard (grafana-dashboard.json)

**17 Monitoring Panels**:

| Panel | Metric | Purpose |
|-------|--------|---------|
| 1 | Block Height | Track block production |
| 2 | Finalized Height | Track finalization |
| 3 | Block Production Rate | Verify ~1 block/sec |
| 4 | Finalization Rate | Verify finalization speed |
| 5 | Finalization Lag | Monitor unfinalized blocks |
| 6 | Transaction Throughput | TPS measurement |
| 7 | Peer Connections | Network health |
| 8 | Transaction Pool | Pool size |
| 9 | Network Latency | p50/p95 latency |
| 10 | PPFA Block Construction | Consensus activity |
| 11 | PPFA Sealing Latency | Sealing performance |
| 12 | CPU Usage | System resources |
| 13 | Memory Usage | RAM consumption |
| 14 | Disk I/O | Storage operations |
| 15 | Network Bandwidth In | Inbound traffic |
| 16 | Network Bandwidth Out | Outbound traffic |
| 17 | Active Alerts | Current alerts |

## Metrics Tracked

### Block Production
- `substrate_block_height` - Current best block
- `substrate_finalized_height` - Last finalized block
- `substrate_proposer_block_constructed_total` - Blocks constructed

### Transactions
- `substrate_proposer_number_of_transactions` - TX count
- `substrate_ready_transactions_number` - Pool size
- `substrate_sub_txpool_validations_scheduled` - Validations

### Network
- `substrate_sub_libp2p_peers_count` - Connected peers
- `substrate_sub_libp2p_network_bytes_total` - Traffic
- `substrate_sub_libp2p_notifications_total` - Messages

### System
- `process_cpu_seconds_total` - CPU time
- `process_resident_memory_bytes` - Memory
- `substrate_state_db_cache_bytes` - DB cache

### PPFA Consensus
- `substrate_proposer_block_constructed_total` - Block construction
- `substrate_proposer_block_constructed_bucket` - Sealing latency histogram

## Alert Thresholds

### Critical (Immediate Action)

```yaml
NoBlocksProduced:
  threshold: rate = 0 for 2m
  action: Check validator status

FinalizationStalled:
  threshold: rate = 0 for 3m
  action: Check consensus

NoPeersConnected:
  threshold: peers = 0 for 1m
  action: Check network

ValidatorOffline:
  threshold: up = 0 for 1m
  action: Restart node

PPFASealingFailure:
  threshold: failures > 0 for 2m
  action: Check PPFA logs
```

### Warning (Check Soon)

```yaml
SlowBlockProduction:
  threshold: rate < 0.5 blocks/sec for 3m
  action: Monitor CPU/memory

FinalizationLag:
  threshold: lag > 10 blocks for 5m
  action: Monitor validators

LowPeerCount:
  threshold: peers < 2 for 2m
  action: Check connectivity

HighMemoryUsage:
  threshold: memory > 8 GB for 5m
  action: Consider restart

HighCPUUsage:
  threshold: cpu > 80% for 10m
  action: Check workload
```

## Operational Procedures

### Daily Checks

```bash
# 1. Check service health
./scripts/monitoring-status.sh

# 2. View Grafana dashboard
open http://localhost:3001

# 3. Check for active alerts
curl -s http://localhost:9090/api/v1/alerts | jq '.data.alerts[] | select(.state=="firing")'

# 4. Review resource usage
docker stats --no-stream etrid-validator-alice etrid-validator-bob etrid-fullnode-charlie
```

### Weekly Maintenance

```bash
# 1. Review alert history
# Check Grafana → Alerting → Alert Rules

# 2. Check disk usage
docker exec etrid-prometheus du -sh /prometheus

# 3. Verify all metrics are being collected
curl -s http://localhost:9090/api/v1/targets | jq -r '.data.activeTargets[] | "\(.labels.job): \(.health)"'

# 4. Backup configuration
cp scripts/testnet/*.yml backups/
cp scripts/testnet/*.json backups/
```

### Monthly Reviews

- Review and tune alert thresholds
- Archive old Prometheus data
- Update documentation
- Test disaster recovery procedures

## Troubleshooting

### Problem: No data in Grafana

**Solution 1**: Check Prometheus is scraping
```bash
curl http://localhost:9090/api/v1/targets
# All targets should show "up"
```

**Solution 2**: Check nodes are exposing metrics
```bash
curl http://localhost:9615/metrics | grep substrate_block_height
# Should return current block height
```

**Solution 3**: Verify Grafana datasource
```
Grafana → Configuration → Data Sources → Prometheus → Test
Should show: "Data source is working"
```

### Problem: Alerts not firing

**Solution 1**: Check rules are loaded
```bash
curl http://localhost:9090/api/v1/rules | jq '.data.groups[].rules[].name'
# Should show all alert rule names
```

**Solution 2**: Validate rule syntax
```bash
docker exec etrid-prometheus promtool check rules /etc/prometheus/alerting-rules.yml
```

**Solution 3**: Check evaluation
```bash
# In Prometheus UI: http://localhost:9090/alerts
# Should show all rules with state: inactive/pending/firing
```

### Problem: High memory usage

**Solution**: Reduce retention period
```yaml
# In docker-compose.yml, change:
--storage.tsdb.retention.time=30d
# To:
--storage.tsdb.retention.time=7d

# Then restart:
docker-compose restart prometheus
```

### Problem: Slow dashboard loading

**Solutions**:
1. Reduce time range (use 30m instead of 24h)
2. Decrease scrape interval to 30s
3. Use recording rules for complex queries
4. Increase Grafana resources in docker-compose.yml

## Customization

### Adding New Metrics

1. **Edit prometheus.yml**:
   ```yaml
   scrape_configs:
     - job_name: 'my-new-service'
       static_configs:
         - targets: ['localhost:9621']
   ```

2. **Restart Prometheus**:
   ```bash
   docker-compose restart prometheus
   ```

3. **Verify in Prometheus UI**:
   - Go to: http://localhost:9090/targets
   - New job should appear

### Adding New Alerts

1. **Edit alerting-rules.yml**:
   ```yaml
   - alert: MyNewAlert
     expr: my_metric > 100
     for: 5m
     labels:
       severity: warning
     annotations:
       summary: "My metric is high"
   ```

2. **Validate syntax**:
   ```bash
   docker exec etrid-prometheus promtool check rules /etc/prometheus/alerting-rules.yml
   ```

3. **Reload Prometheus**:
   ```bash
   curl -X POST http://localhost:9090/-/reload
   ```

### Customizing Dashboard

1. **Edit in Grafana UI**:
   - Make changes in Grafana
   - Settings → JSON Model
   - Copy JSON

2. **Save to file**:
   ```bash
   # Save to grafana-dashboard.json
   ```

3. **Restart Grafana**:
   ```bash
   docker-compose restart grafana
   ```

## Production Deployment

### Security Checklist

- [ ] Change default Grafana password
- [ ] Enable HTTPS with valid certificates
- [ ] Restrict network access to monitoring ports
- [ ] Enable authentication on Prometheus
- [ ] Configure Alertmanager for notifications
- [ ] Set up backup procedures
- [ ] Enable audit logging

### Recommended Changes

1. **Use external datasources**:
   - PostgreSQL for Grafana
   - Remote storage for Prometheus

2. **High Availability**:
   - Deploy 2+ Prometheus instances
   - Use Thanos for federation
   - Load balance Grafana

3. **Enhanced Alerting**:
   - Configure Alertmanager
   - Set up PagerDuty/Slack integration
   - Define escalation policies

4. **Resource Planning**:
   - Prometheus: 4 CPU, 8 GB RAM, 200 GB SSD
   - Grafana: 2 CPU, 4 GB RAM, 20 GB SSD
   - Plan for 90-day retention

## Additional Resources

### Documentation
- Full Guide: `/Users/macbook/Desktop/etrid/docs/MONITORING_GUIDE.md`
- Quick Start: `/Users/macbook/Desktop/etrid/scripts/testnet/MONITORING_QUICK_START.md`

### External Links
- Prometheus Docs: https://prometheus.io/docs/
- Grafana Docs: https://grafana.com/docs/
- PromQL Reference: https://prometheus.io/docs/prometheus/latest/querying/basics/
- Substrate Metrics: https://docs.substrate.io/reference/command-line-tools/

### Example Queries

```promql
# Average block time over 5 minutes
60 / rate(substrate_block_height[5m])

# Memory growth rate (MB/hour)
rate(process_resident_memory_bytes[1h]) * 3600 / 1024 / 1024

# Network saturation (MB/s)
rate(substrate_sub_libp2p_network_bytes_total[5m]) / 1024 / 1024

# Validator participation rate
sum(rate(substrate_proposer_block_constructed_total[1h])) / sum(rate(substrate_block_height[1h]))

# p99 transaction execution time
histogram_quantile(0.99, rate(substrate_extrinsic_execution_time_bucket[5m]))
```

## Support

For issues or questions:
- Check troubleshooting section above
- Review full documentation: `docs/MONITORING_GUIDE.md`
- Check GitHub issues: https://github.com/etrid/etrid/issues
- Contact: Ëtrid Protocol Team

---

**Last Updated**: 2025-10-23
**Version**: 1.0.0
**Maintained By**: Ëtrid Protocol Team
