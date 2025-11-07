# Ëtrid Testnet Monitoring Guide

## Overview

This guide provides comprehensive instructions for setting up, configuring, and using the Prometheus and Grafana monitoring infrastructure for the Ëtrid testnet.

## Table of Contents

1. [Architecture](#architecture)
2. [Quick Start](#quick-start)
3. [Configuration](#configuration)
4. [Metrics Reference](#metrics-reference)
5. [Alerting Rules](#alerting-rules)
6. [Dashboard Guide](#dashboard-guide)
7. [Troubleshooting](#troubleshooting)
8. [Production Deployment](#production-deployment)

---

## Architecture

### Monitoring Stack Components

The Ëtrid monitoring stack consists of:

- **Prometheus** (Port 9090): Time-series database and metrics collection
- **Grafana** (Port 3001): Visualization and dashboarding
- **Node Exporters**: Etrid nodes exposing metrics on ports 9615-9617

### Node Configuration

| Node | Type | RPC Port | P2P Port | Metrics Port |
|------|------|----------|----------|--------------|
| Alice | Validator | 9944 | 30333 | 9615 |
| Bob | Validator | 9945 | 30334 | 9616 |
| Charlie | Full Node | 9946 | 30335 | 9617 |

---

## Quick Start

### Option 1: Docker Compose (Recommended)

Start the entire testnet with monitoring:

```bash
# From the project root
docker-compose up -d

# View logs
docker-compose logs -f prometheus
docker-compose logs -f grafana

# Check services are running
docker-compose ps
```

### Option 2: Automated Setup Script

For local/native installations:

```bash
# Install and configure Prometheus + Grafana
./scripts/setup-monitoring-stack.sh

# Check status
./scripts/monitoring-status.sh
```

### Option 3: Manual Setup

1. **Start the testnet nodes:**
   ```bash
   ./scripts/start-testnet.sh --validators 3
   ```

2. **Start Prometheus:**
   ```bash
   prometheus --config.file=./scripts/testnet/prometheus.yml \
              --storage.tsdb.path=./monitoring/prometheus/data
   ```

3. **Start Grafana:**
   ```bash
   grafana-server --config=/etc/grafana/grafana.ini \
                  --homepath=/usr/share/grafana
   ```

---

## Configuration

### Prometheus Configuration

Location: `/Users/macbook/Desktop/etrid/scripts/testnet/prometheus.yml`

Key settings:
- **Scrape interval**: 15s (how often to collect metrics)
- **Evaluation interval**: 15s (how often to evaluate alerting rules)
- **Retention**: 30 days (how long to store metrics)

### Alerting Rules

Location: `/Users/macbook/Desktop/etrid/scripts/testnet/alerting-rules.yml`

The alerting rules are organized into groups:

1. **Block Production Alerts**
   - NoBlocksProduced (critical)
   - SlowBlockProduction (warning)
   - FinalizationLag (warning)
   - FinalizationStalled (critical)

2. **Network Connectivity Alerts**
   - LowPeerCount (warning)
   - NoPeersConnected (critical)
   - HighNetworkErrors (warning)

3. **Transaction Pool Alerts**
   - TransactionPoolFull (warning)
   - TransactionPoolStalled (warning)

4. **System Resource Alerts**
   - HighMemoryUsage (warning)
   - HighCPUUsage (warning)
   - DiskSpaceWarning (warning)

5. **Consensus & Validator Alerts**
   - ValidatorOffline (critical)
   - NoBlockAuthoring (warning)
   - HighUncleRate (warning)

6. **PPFA Consensus Specific Alerts**
   - PPFASealingFailure (critical)
   - HighPPFALatency (warning)

### Grafana Dashboard

Location: `/Users/macbook/Desktop/etrid/scripts/testnet/grafana-dashboard.json`

The dashboard includes 17 panels organized into sections:

1. **Block Production** (Panels 1-5)
2. **Transaction Processing** (Panel 6)
3. **Network Health** (Panels 7, 9, 15-16)
4. **Transaction Pool** (Panel 8)
5. **PPFA Consensus** (Panels 10-11)
6. **System Resources** (Panels 12-14)
7. **Alerts** (Panel 17)

---

## Metrics Reference

### Core Blockchain Metrics

#### Block Production

```promql
# Current block height
substrate_block_height{job="flarechain-alice"}

# Finalized block height
substrate_finalized_height{job="flarechain-alice"}

# Block production rate (blocks/sec)
rate(substrate_block_height[5m])

# Finalization lag (unfinalized blocks)
substrate_block_height - substrate_finalized_height
```

#### Transaction Throughput

```promql
# Transaction throughput (TPS)
rate(substrate_proposer_number_of_transactions[5m])

# Ready transactions in pool
substrate_ready_transactions_number

# Transaction pool size
substrate_sub_txpool_validations_scheduled
```

#### PPFA Consensus

```promql
# Block construction rate
rate(substrate_proposer_block_constructed_total[5m])

# Block sealing latency (p95)
histogram_quantile(0.95, rate(substrate_proposer_block_constructed_bucket[5m]))

# PPFA sealing failures
rate(substrate_proposer_block_constructed_total{status="failure"}[5m])
```

### Network Metrics

```promql
# Connected peers
substrate_sub_libp2p_peers_count

# Network bandwidth in (bytes/sec)
rate(substrate_sub_libp2p_network_bytes_total{direction="in"}[5m])

# Network bandwidth out (bytes/sec)
rate(substrate_sub_libp2p_network_bytes_total{direction="out"}[5m])

# Network latency (p95)
histogram_quantile(0.95, rate(substrate_sub_libp2p_notifications_total_bucket[5m]))
```

### System Resource Metrics

```promql
# CPU usage (%)
rate(process_cpu_seconds_total[5m]) * 100

# Memory usage (GB)
process_resident_memory_bytes / 1024 / 1024 / 1024

# Database cache size
substrate_state_db_cache_bytes

# Disk I/O rate
rate(substrate_state_db_cache_bytes[5m])
```

---

## Alerting Rules

### Alert Severity Levels

- **Critical**: Immediate attention required, service disruption likely
- **Warning**: Attention needed soon, potential issues detected
- **Info**: Informational, no action required

### Alert Response Guide

#### Critical Alerts

**NoBlocksProduced**
- **Cause**: Block production has stopped
- **Action**:
  1. Check validator node status
  2. Verify validator keys are loaded
  3. Check if validator is in active set
  4. Review node logs for errors

**FinalizationStalled**
- **Cause**: PPFA consensus cannot finalize blocks
- **Action**:
  1. Check if 2/3 validators are online
  2. Verify network connectivity between validators
  3. Review consensus logs
  4. Check for runtime errors

**NoPeersConnected**
- **Cause**: Node is isolated from network
- **Action**:
  1. Check network connectivity
  2. Verify P2P ports are open (30333-30335)
  3. Check bootnode configuration
  4. Review libp2p logs

**ValidatorOffline**
- **Cause**: Validator node is not responding
- **Action**:
  1. Check if node process is running
  2. Restart node if needed
  3. Check system resources
  4. Review crash logs

#### Warning Alerts

**SlowBlockProduction**
- **Cause**: Block production rate below expected
- **Action**:
  1. Check CPU/memory usage
  2. Review transaction pool size
  3. Check for resource contention
  4. Monitor network latency

**FinalizationLag**
- **Cause**: Too many unfinalized blocks
- **Action**:
  1. Monitor if lag is increasing
  2. Check validator voting participation
  3. Verify network conditions
  4. Review consensus logs

**LowPeerCount**
- **Cause**: Fewer peers than expected
- **Action**:
  1. Check network connectivity
  2. Verify other nodes are running
  3. Check for firewall issues
  4. Review mDNS/discovery logs

---

## Dashboard Guide

### Accessing Dashboards

1. **Grafana**: http://localhost:3001
   - Username: `admin`
   - Password: `etrid2025`

2. **Prometheus**: http://localhost:9090

3. **Node Metrics (Raw)**:
   - Alice: http://localhost:9615/metrics
   - Bob: http://localhost:9616/metrics
   - Charlie: http://localhost:9617/metrics

### Dashboard Sections

#### 1. Block Production Overview (Top Row)

Shows block height and finalization status across all nodes. Look for:
- All lines moving upward (blocks being produced)
- Lines close together (nodes in sync)
- Consistent finalization (finalized height tracking block height)

#### 2. Performance Metrics (Second Row)

Displays block production and finalization rates. Expected values:
- Block production: ~1 block/sec
- Finalization: ~1 block/sec
- Finalization lag: <5 blocks

#### 3. Transaction Processing (Third Row)

Shows finalization lag and transaction throughput. Monitor for:
- Finalization lag staying low (<10 blocks)
- TPS matching expected load
- Consistent transaction processing

#### 4. Network Health (Fourth Row)

Displays peer connections, transaction pool, and network latency. Look for:
- Peer count: 2-3 peers per node
- Transaction pool: <1000 ready transactions
- Network latency: <100ms (p95)

#### 5. PPFA Consensus (Fifth Row)

Shows PPFA-specific metrics. Monitor for:
- Block construction rate matching block production
- Sealing latency: <1 second (p95)
- No sealing failures

#### 6. System Resources (Sixth Row)

Displays CPU, memory, and disk usage. Typical values:
- CPU: 10-30% (idle), 40-80% (under load)
- Memory: 2-4 GB
- Disk I/O: Varies with block production

#### 7. Network Bandwidth (Seventh Row)

Shows network traffic in/out. Expected:
- Inbound: 0.5-2 MB/s
- Outbound: 0.5-2 MB/s
- Spikes during block production/sync

#### 8. Active Alerts (Bottom)

Shows currently firing alerts. Ideally should be empty (green).

### Custom Time Ranges

Use the time picker in top-right to view:
- Last 5 minutes: Real-time monitoring
- Last 30 minutes: Recent trends
- Last 1 hour: Short-term analysis
- Last 24 hours: Daily patterns
- Custom range: Historical analysis

### Templating Variables

Use the `node` dropdown to filter by specific nodes:
- All: Show all nodes
- alice: Show only Alice validator
- bob: Show only Bob validator
- charlie: Show only Charlie full node

---

## Troubleshooting

### Common Issues

#### 1. Prometheus Not Scraping Metrics

**Symptoms**: No data in Grafana, "No data" errors

**Solutions**:
```bash
# Check if nodes are exposing metrics
curl http://localhost:9615/metrics
curl http://localhost:9616/metrics
curl http://localhost:9617/metrics

# Check Prometheus targets
# Visit: http://localhost:9090/targets
# All targets should show "UP"

# Check Prometheus logs
docker-compose logs prometheus
# or
journalctl -u prometheus-etrid -f
```

#### 2. Grafana Shows "No Data"

**Symptoms**: Dashboard panels empty or show "No data"

**Solutions**:
```bash
# Check Prometheus datasource in Grafana
# Settings > Data Sources > Prometheus
# Click "Test" - should show "Data source is working"

# Verify Prometheus is running
curl http://localhost:9090/-/healthy

# Check query in Explore tab
# Use: substrate_block_height
```

#### 3. Alerts Not Firing

**Symptoms**: Expected alerts don't appear

**Solutions**:
```bash
# Check alert rules are loaded
curl http://localhost:9090/api/v1/rules

# Verify rule syntax
promtool check rules scripts/testnet/alerting-rules.yml

# Check Prometheus logs for rule errors
docker-compose logs prometheus | grep -i error
```

#### 4. High Memory Usage

**Symptoms**: Prometheus using excessive memory

**Solutions**:
```bash
# Reduce retention period in docker-compose.yml
# Change: --storage.tsdb.retention.time=30d
# To:     --storage.tsdb.retention.time=7d

# Reduce scrape interval in prometheus.yml
# Change: scrape_interval: 15s
# To:     scrape_interval: 30s

# Restart Prometheus
docker-compose restart prometheus
```

#### 5. Slow Dashboard Loading

**Symptoms**: Grafana dashboard takes long to load

**Solutions**:
```bash
# Reduce time range (use 30m instead of 24h)
# Reduce scrape interval
# Use recording rules for complex queries
# Increase Grafana memory limit in docker-compose.yml
```

### Debug Commands

```bash
# Check all containers status
docker-compose ps

# View Prometheus config
docker exec etrid-prometheus cat /etc/prometheus/prometheus.yml

# Check Prometheus targets
curl http://localhost:9090/api/v1/targets | jq

# Query Prometheus directly
curl -G http://localhost:9090/api/v1/query \
  --data-urlencode 'query=substrate_block_height' | jq

# Check Grafana health
curl http://localhost:3001/api/health

# View all metrics from a node
curl -s http://localhost:9615/metrics | grep substrate_

# Check Docker network
docker network inspect etrid-network

# Test connectivity between containers
docker exec etrid-prometheus ping validator-alice
```

---

## Production Deployment

### Security Hardening

1. **Change Default Passwords**
   ```yaml
   # In docker-compose.yml
   environment:
     - GF_SECURITY_ADMIN_PASSWORD=<strong-password>
   ```

2. **Enable HTTPS**
   ```yaml
   # Add reverse proxy (nginx/traefik)
   # Configure SSL certificates
   # Redirect HTTP to HTTPS
   ```

3. **Restrict Network Access**
   ```bash
   # Use firewall rules
   sudo ufw allow from <monitoring-subnet> to any port 9090
   sudo ufw allow from <monitoring-subnet> to any port 3001

   # Or use Docker networks
   # Create separate monitoring network
   ```

4. **Enable Authentication**
   ```yaml
   # Prometheus: Add basic auth via reverse proxy
   # Grafana: Enable OAuth/LDAP integration
   ```

### Alertmanager Configuration

For production, configure Alertmanager to send notifications:

```yaml
# alertmanager.yml
global:
  resolve_timeout: 5m
  slack_api_url: '<slack-webhook>'

route:
  group_by: ['alertname', 'cluster']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'slack-critical'
  routes:
    - match:
        severity: critical
      receiver: 'pagerduty'
    - match:
        severity: warning
      receiver: 'slack-warnings'

receivers:
  - name: 'slack-critical'
    slack_configs:
      - channel: '#etrid-alerts-critical'
        title: 'Critical Alert'
        text: '{{ range .Alerts }}{{ .Annotations.summary }}{{ end }}'

  - name: 'pagerduty'
    pagerduty_configs:
      - service_key: '<pagerduty-key>'

  - name: 'slack-warnings'
    slack_configs:
      - channel: '#etrid-alerts-warnings'
```

### High Availability Setup

For production deployments:

1. **Prometheus HA**
   - Deploy 2+ Prometheus instances
   - Use Thanos for long-term storage
   - Configure Prometheus federation

2. **Grafana HA**
   - Use external database (PostgreSQL)
   - Deploy behind load balancer
   - Share dashboard via provisioning

3. **Monitoring Multiple Clusters**
   - Use Prometheus federation
   - Configure remote write
   - Centralize alerting

### Resource Recommendations

| Component | CPU | Memory | Disk | Notes |
|-----------|-----|--------|------|-------|
| Prometheus | 2 cores | 4-8 GB | 100 GB SSD | Scales with metrics |
| Grafana | 1 core | 2 GB | 10 GB | Minimal resources |
| Alertmanager | 1 core | 512 MB | 10 GB | Lightweight |

### Backup and Retention

```bash
# Backup Prometheus data
docker run --rm -v etrid_prometheus-data:/data -v $(pwd):/backup \
  ubuntu tar czf /backup/prometheus-backup-$(date +%Y%m%d).tar.gz /data

# Backup Grafana dashboards
docker exec etrid-grafana grafana-cli admin export-dashboard

# Set retention policy
# In prometheus.yml:
storage:
  tsdb:
    retention.time: 90d
    retention.size: 100GB
```

### Monitoring Best Practices

1. **Regular Reviews**
   - Review alerts weekly
   - Tune thresholds based on actual behavior
   - Remove noisy alerts

2. **Documentation**
   - Document alert response procedures
   - Maintain runbooks for critical alerts
   - Keep metric definitions updated

3. **Testing**
   - Test alerting regularly
   - Simulate failure scenarios
   - Verify alert routing

4. **Capacity Planning**
   - Monitor metric cardinality
   - Plan for growth (30% buffer)
   - Regular disk space checks

---

## Additional Resources

### Useful Links

- **Prometheus Documentation**: https://prometheus.io/docs/
- **Grafana Documentation**: https://grafana.com/docs/
- **PromQL Tutorial**: https://prometheus.io/docs/prometheus/latest/querying/basics/
- **Substrate Metrics**: https://docs.substrate.io/reference/command-line-tools/node-template/#monitoring

### Example Queries

```promql
# Average block time
rate(substrate_block_height[5m]) / 60

# Memory usage trend
increase(process_resident_memory_bytes[1h])

# Network saturation
rate(substrate_sub_libp2p_network_bytes_total[5m]) / 1024 / 1024

# Top 5 slowest extrinsics
topk(5, substrate_extrinsic_execution_time)

# Validator uptime percentage
avg_over_time(up{node_type="validator"}[24h]) * 100
```

### Contact and Support

For issues or questions:
- GitHub Issues: https://github.com/etrid/etrid/issues
- Documentation: https://docs.etrid.io
- Community Discord: [Link to Discord]

---

## Appendix

### Metric Naming Conventions

Substrate metrics follow this pattern:
- `substrate_<subsystem>_<metric_name>{labels}`

Examples:
- `substrate_block_height{chain="dev"}` - Block height for dev chain
- `substrate_sub_libp2p_peers_count{protocol="substrate"}` - Peer count
- `substrate_proposer_block_constructed_total{status="success"}` - Blocks constructed

### Label Reference

Common labels used in metrics:
- `instance`: Node identifier (alice, bob, charlie)
- `job`: Prometheus job name (flarechain-alice)
- `node_type`: Type of node (validator, full_node, collator)
- `chain`: Chain name (flarechain, btc-pbc, eth-pbc)
- `status`: Operation status (success, failure)
- `direction`: Network direction (in, out)

---

**Last Updated**: 2025-10-23
**Version**: 1.0.0
**Maintained By**: Ëtrid Protocol Team
