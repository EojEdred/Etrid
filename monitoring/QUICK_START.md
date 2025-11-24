# Ëtrid PrimeArc Core Chain Monitoring - Quick Start Guide

Get monitoring up and running in 5 minutes.

## Prerequisites

- Docker and Docker Compose installed (for Docker setup)
- OR macOS/Linux with internet connection (for native setup)
- Access to PrimeArc Core Chain validator at 100.93.43.18:9615

## Option 1: Docker Setup (Recommended)

**Fastest way to get started - everything in containers!**

### 1. Start the Stack

```bash
cd /Users/macbook/Desktop/etrid/monitoring
docker-compose up -d
```

### 2. Verify Services

```bash
docker-compose ps
```

You should see:
- `etrid-prometheus` - Running on port 9090
- `etrid-grafana` - Running on port 3000

### 3. Access Dashboards

**Prometheus**: http://localhost:9090
- Check targets: http://localhost:9090/targets
- Verify validator is "UP" (green)

**Grafana**: http://localhost:3000
- Username: `admin`
- Password: `etrid2025`
- Navigate to: Dashboards > Ëtrid PrimeArc Core Chain Validators

### 4. Done!

Your monitoring is now live. The dashboard will auto-refresh every 10 seconds.

### Managing Docker Services

```bash
# View logs
docker-compose logs -f prometheus
docker-compose logs -f grafana

# Restart services
docker-compose restart

# Stop services
docker-compose down

# Stop and remove data
docker-compose down -v
```

## Option 2: Native Setup (macOS/Linux)

**For production deployments or when Docker isn't available.**

### 1. Install Prometheus

```bash
cd /Users/macbook/Desktop/etrid/monitoring
./install-prometheus.sh
```

Wait for installation to complete (~2 minutes).

### 2. Install Grafana

```bash
./install-grafana.sh
```

Wait for installation to complete (~2 minutes).

### 3. Verify Services

**macOS:**
```bash
brew services list | grep -E "prometheus|grafana"
```

**Linux:**
```bash
sudo systemctl status prometheus
sudo systemctl status grafana-server
```

### 4. Access Dashboards

Same as Docker setup:
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/etrid2025)

## Quick Verification Checklist

After setup, verify everything is working:

- [ ] Prometheus is accessible at http://localhost:9090
- [ ] Targets page shows validator as "UP": http://localhost:9090/targets
- [ ] Grafana is accessible at http://localhost:3000
- [ ] Can login to Grafana with admin/etrid2025
- [ ] PrimeArc Core Chain dashboard exists in Grafana
- [ ] Dashboard shows metrics (block height, peers, etc.)
- [ ] Block height is increasing over time
- [ ] No critical alerts are firing

## What You're Monitoring

The dashboard shows:

1. **Block Height** - Current blockchain height (#16,949+)
2. **Finalization** - Blocks being finalized
3. **Peers** - Connected validators (should be 5+)
4. **Block Rate** - ~0.167 blocks/sec (1 block per 6 seconds)
5. **CPU/Memory** - System resource usage
6. **Network** - Bandwidth and latency

## Troubleshooting

### Prometheus shows validator as "DOWN"

```bash
# Test if metrics endpoint is accessible
curl http://100.93.43.18:9615/metrics

# If this fails, check:
# 1. Validator is running
# 2. Tailscale is connected
# 3. Port 9615 is open
```

### Grafana shows "No Data"

```bash
# Test Prometheus is working
curl http://localhost:9090/api/v1/query?query=up

# In Grafana:
# 1. Go to Configuration > Data Sources
# 2. Click on Prometheus
# 3. Click "Test" - should show "Data source is working"
```

### Dashboard not showing

```bash
# Docker: Restart Grafana
docker-compose restart grafana

# Native: Restart Grafana
brew services restart grafana  # macOS
sudo systemctl restart grafana-server  # Linux

# Then refresh browser
```

## Next Steps

1. **Change Grafana Password**
   - Login to Grafana
   - Go to Profile > Change Password
   - Use a strong password

2. **Add More Validators**
   - Edit `prometheus-primearc_core_chain.yml`
   - Add new scrape jobs
   - Restart Prometheus

3. **Set Up Alerts**
   - Configure Alertmanager (optional)
   - Add Slack/email notifications
   - Test alert routing

4. **Customize Dashboard**
   - Add more panels
   - Adjust time ranges
   - Create custom queries

## Key Metrics to Watch

| Metric | Good | Warning | Critical |
|--------|------|---------|----------|
| Block Production | ~0.167/s | <0.15/s | 0/s |
| Finalization Lag | <10 blocks | 10-20 blocks | >20 blocks |
| Peer Count | 5+ peers | 3-4 peers | <3 peers |
| CPU Usage | <50% | 50-80% | >80% |
| Memory Usage | <6 GB | 6-8 GB | >8 GB |

## Getting Help

If you encounter issues:

1. Check the full README: `cat README.md`
2. Review Prometheus logs
3. Review Grafana logs
4. Check validator is exposing metrics
5. Verify network connectivity

## Useful Commands

```bash
# Docker: View all metrics being collected
docker exec etrid-prometheus promtool query instant \
  http://localhost:9090 'substrate_block_height'

# Native: Test Prometheus query
curl 'http://localhost:9090/api/v1/query?query=substrate_block_height'

# Check disk space
df -h  # Prometheus data can grow over time

# Backup configuration
cp prometheus-primearc_core_chain.yml ~/backup/
cp grafana-dashboard-primearc_core_chain.json ~/backup/
```

---

**That's it!** You now have full monitoring for your PrimeArc Core Chain validators.

For advanced configuration, alerts, and production deployment, see the full [README.md](README.md).
