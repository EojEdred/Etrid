# 📊 Historical Data Tracking & Database

Complete historical metrics tracking system for MasterChef with SQLite database storage.

---

## 🎯 What This Does

Collects and stores **all MasterChef metrics over time** to enable:

- **Trend Analysis**: Track TVL, APR, and rewards over days/weeks/months
- **Reporting**: Generate performance reports for stakeholders
- **Decision Making**: Data-driven decisions on emission rates, pool allocations
- **Alerts**: Detect anomalies and trigger alerts
- **Auditing**: Complete audit trail of all state changes

---

## 📦 Components

### 1. Database Schema (`database/schema.sql`)

SQLite database with 6 tables and 4 views:

**Tables:**
- `metrics_snapshots` - Hourly MasterChef state snapshots
- `pool_snapshots` - Hourly pool metrics
- `events` - Blockchain events log (deposits, withdrawals, etc.)
- `health_checks` - Automated health check results
- `alerts` - Alert management and tracking
- `backups` - Backup log

**Views:**
- `latest_metrics` - Latest metrics by network
- `latest_pools` - Latest pool data
- `tvl_daily` - Daily TVL trends
- `active_alerts` - Unacknowledged/unresolved alerts

### 2. Database Library (`scripts/lib/database.ts`)

TypeScript API for all database operations:

```typescript
// Save metrics
saveMetricsSnapshot(metrics: MetricsSnapshot): number
savePoolSnapshot(pool: PoolSnapshot): void

// Log events
logEvent(event: Event): void

// Health & alerts
saveHealthCheck(check: HealthCheck): void
createAlert(alert: Alert): number
acknowledgeAlert(alertId: number, acknowledgedBy: string): void
resolveAlert(alertId: number, resolutionNotes?: string): void

// Query data
getTVLHistory(network: string, days?: number): any[]
getPoolTVLHistory(network: string, poolId: number, days?: number): any[]
getAPRHistory(network: string, poolId: number, days?: number): any[]
getLatestMetrics(network: string): any
getLatestPools(network: string): any[]
getRecentEvents(network: string, limit?: number): any[]
getDatabaseStats(): any
```

### 3. Metrics Collection (`scripts/collect-metrics.ts`)

Automated script that runs via cron to collect metrics:

```bash
# Collects and stores:
- MasterChef state (pools, emissions, balance, etc.)
- BNB and ÉTR prices (mainnet only)
- Pool TVL and APR (mainnet only)
- Total TVL across all pools
- Days remaining until rewards depleted
```

**Runs every hour via cron job**

### 4. Query Tool (`scripts/query-history.ts`)

Interactive CLI tool to query historical data:

```bash
# Query options:
1. Latest metrics
2. TVL history (with trends)
3. Pool TVL history
4. APR history
5. Recent events
6. Export data
```

---

## 🚀 Quick Start

### 1. Install Dependencies

```bash
npm install
# Installs better-sqlite3 for database operations
```

### 2. Initialize Database

The database initializes automatically on first use. Schema is applied from `database/schema.sql`.

**Location**: `05-multichain/bridge/adapters/bsc/database/masterchef.db`

### 3. Collect First Snapshot

```bash
# Testnet
npm run collect-metrics:testnet

# Mainnet
npm run collect-metrics:mainnet
```

### 4. Query Data

```bash
npm run query-history
```

---

## 📅 Automated Collection (Cron)

### Setup Hourly Collection

**On Linux/Mac:**

```bash
# Edit crontab
crontab -e

# Add this line for hourly mainnet collection
0 * * * * cd /path/to/etrid/05-multichain/bridge/adapters/bsc && npm run collect-metrics:mainnet >> /var/log/masterchef-collector.log 2>&1
```

**On Windows (Task Scheduler):**

1. Open Task Scheduler
2. Create Basic Task
3. Trigger: Daily, repeat every 1 hour
4. Action: Start a program
   - Program: `npm`
   - Arguments: `run collect-metrics:mainnet`
   - Start in: `C:\path\to\etrid\05-multichain\bridge\adapters\bsc`

**Using systemd timer (Linux):**

```bash
# Create timer unit
sudo nano /etc/systemd/system/masterchef-collector.timer

[Unit]
Description=MasterChef Metrics Collector Timer

[Timer]
OnCalendar=hourly
Persistent=true

[Install]
WantedBy=timers.target

# Create service unit
sudo nano /etc/systemd/system/masterchef-collector.service

[Unit]
Description=MasterChef Metrics Collector

[Service]
Type=oneshot
User=youruser
WorkingDirectory=/path/to/etrid/05-multichain/bridge/adapters/bsc
ExecStart=/usr/bin/npm run collect-metrics:mainnet

# Enable and start
sudo systemctl enable masterchef-collector.timer
sudo systemctl start masterchef-collector.timer
```

---

## 📊 Usage Examples

### Collect Metrics

```bash
# Testnet (no price data)
npm run collect-metrics:testnet

# Mainnet (includes BNB/ÉTR prices and TVL)
npm run collect-metrics:mainnet
```

**Output:**
```
📊 COLLECTING METRICS FOR HISTORICAL STORAGE

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📍 Network: BSC Mainnet
   Block: 34567890

🔍 Fetching on-chain data...

   BNB Price: $312.45
   ÉTR Price: $0.000123

💾 Saving to database...

   ✅ Saved metrics snapshot (ID: 42)
   ✅ Saved 3 pool snapshot(s)
   ✅ Logged collection event

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ METRICS COLLECTION COMPLETE

📊 Summary:
   Snapshot ID: 42
   Pools: 3
   Total TVL: $1,234,567
   Total Staked: 98765.4321 LP
   Days Remaining: 156 days
```

### Query Historical Data

```bash
npm run query-history
```

**Interactive Menu:**
```
📊 HISTORICAL DATA QUERY TOOL

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📈 Database Statistics:

   Metrics Snapshots: 1,234
   Pool Snapshots:    3,702
   Events:            567
   Health Checks:     234
   Alerts:            12
   Backups:           8

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 QUERY OPTIONS

1. Latest metrics
2. TVL history
3. Pool TVL history
4. APR history
5. Recent events
6. Export data

Select option (1-6):
```

### Example: TVL History

```bash
Select option: 2
Network (mainnet/testnet): mainnet
Days of history (default: 30): 7

📈 TVL HISTORY (Last 7 days)

   Summary:
     Current: $1,234,567
     Average: $1,198,432
     Min:     $1,089,234
     Max:     $1,298,765
     Points:  168

   Recent Data Points:

     10/24/2025, 2:00:00 PM: $1,234,567
     10/24/2025, 1:00:00 PM: $1,229,834
     10/24/2025, 12:00:00 PM: $1,221,098
     ...
```

### Example: Pool APR History

```bash
Select option: 4
Network (mainnet/testnet): mainnet
Pool ID: 0
Days of history (default: 30): 7

📊 POOL 0 APR HISTORY (Last 7 days)

   Summary:
     Current: 142.56%
     Average: 145.23%
     Min:     138.90%
     Max:     150.12%
     Points:  168

   Recent Data Points:

     10/24/2025, 2:00:00 PM: 142.56%
     10/24/2025, 1:00:00 PM: 143.21%
     10/24/2025, 12:00:00 PM: 144.05%
     ...
```

---

## 🔍 Direct Database Access

### Using SQLite CLI

```bash
# Open database
sqlite3 database/masterchef.db

# Show tables
.tables

# Query latest metrics
SELECT * FROM latest_metrics;

# Query TVL trends
SELECT * FROM tvl_daily ORDER BY date DESC LIMIT 30;

# Export to CSV
.headers on
.mode csv
.output tvl_export.csv
SELECT * FROM metrics_snapshots WHERE network = 'mainnet';
.quit
```

### Using GUI Tools

**DB Browser for SQLite** (Recommended):
- Download: https://sqlitebrowser.org/
- Open: `database/masterchef.db`
- Browse data, run queries, visualize

**DBeaver**:
- Download: https://dbeaver.io/
- Connect to SQLite database
- Full SQL IDE with charts

---

## 📈 Analytics Queries

### TVL Growth Rate

```sql
WITH daily_tvl AS (
  SELECT
    DATE(timestamp) as date,
    AVG(total_tvl_usd) as tvl
  FROM metrics_snapshots
  WHERE network = 'mainnet'
  GROUP BY DATE(timestamp)
)
SELECT
  date,
  tvl,
  LAG(tvl) OVER (ORDER BY date) as prev_tvl,
  ((tvl - LAG(tvl) OVER (ORDER BY date)) / LAG(tvl) OVER (ORDER BY date)) * 100 as growth_pct
FROM daily_tvl
ORDER BY date DESC
LIMIT 30;
```

### Pool Performance Comparison

```sql
SELECT
  pool_id,
  lp_symbol,
  AVG(tvl_usd) as avg_tvl,
  AVG(apr_percent) as avg_apr,
  COUNT(*) as snapshots
FROM pool_snapshots
WHERE network = 'mainnet'
  AND timestamp >= datetime('now', '-7 days')
GROUP BY pool_id, lp_symbol
ORDER BY avg_tvl DESC;
```

### Emission Efficiency

```sql
SELECT
  DATE(timestamp) as date,
  total_tvl_usd / CAST(masterchef_balance as REAL) as tvl_per_etr
FROM metrics_snapshots
WHERE network = 'mainnet'
  AND total_tvl_usd IS NOT NULL
ORDER BY date DESC
LIMIT 30;
```

---

## 🔗 Integration with Other Systems

### Export for Grafana/Prometheus

The historical data can be exported in formats compatible with monitoring tools:

```bash
# Export as Prometheus metrics (coming in Analytics API feature)
curl http://localhost:3000/metrics

# Export as JSON
npm run export-metrics:mainnet
```

### Webhook Integration (coming soon)

Real-time webhooks when metrics change significantly:

```javascript
// Will be added in Analytics API feature
POST /api/webhooks
{
  "url": "https://your-app.com/webhook",
  "events": ["tvl_change", "apr_change", "low_balance"],
  "threshold": 5 // 5% change
}
```

---

## 🛠️ Maintenance

### Database Backups

```bash
# Manual backup
sqlite3 database/masterchef.db ".backup database/backups/masterchef_$(date +%Y%m%d).db"

# Automated daily backup (cron)
0 0 * * * sqlite3 /path/to/database/masterchef.db ".backup /path/to/backups/masterchef_\$(date +\%Y\%m\%d).db"
```

### Vacuum Database (Optimize)

```bash
# Reclaim space and optimize
sqlite3 database/masterchef.db "VACUUM;"
```

### Retention Policy

**Archive old data** to keep database size manageable:

```sql
-- Keep only last 90 days of hourly snapshots
DELETE FROM metrics_snapshots
WHERE timestamp < datetime('now', '-90 days');

-- Keep only last 180 days of events
DELETE FROM events
WHERE timestamp < datetime('now', '-180 days');

-- Vacuum to reclaim space
VACUUM;
```

---

## 📊 Database Schema Details

### Metrics Snapshots

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| timestamp | DATETIME | Snapshot time |
| network | TEXT | mainnet/testnet |
| block_number | INTEGER | Block height |
| total_pools | INTEGER | Number of pools |
| reward_per_block | TEXT | ÉTR per block |
| total_alloc_point | TEXT | Sum of allocPoints |
| masterchef_balance | TEXT | ÉTR balance |
| days_remaining | INTEGER | Days until depleted |
| is_paused | BOOLEAN | Paused state |
| bnb_price | REAL | BNB price USD |
| etr_price | REAL | ÉTR price USD |
| total_tvl_usd | REAL | Total TVL USD |
| total_staked_lp | TEXT | Total LP staked |

### Pool Snapshots

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| snapshot_id | INTEGER | FK to metrics_snapshots |
| timestamp | DATETIME | Snapshot time |
| network | TEXT | mainnet/testnet |
| pool_id | INTEGER | Pool index |
| lp_token | TEXT | LP token address |
| lp_symbol | TEXT | LP token symbol |
| lp_name | TEXT | LP token name |
| total_staked | TEXT | LP tokens staked |
| alloc_point | TEXT | Allocation points |
| reward_share | REAL | % of rewards |
| lp_price | REAL | LP price USD |
| tvl_usd | REAL | Pool TVL USD |
| apr_percent | REAL | APR % |
| daily_rewards | TEXT | ÉTR per day |
| monthly_rewards | TEXT | ÉTR per month |

---

## 🎓 Best Practices

1. **Run collection hourly** for accurate trend data
2. **Monitor database size** - archive old data if needed
3. **Back up database daily** before maintenance
4. **Use views** for common queries instead of complex SQL
5. **Index properly** - schema includes optimal indexes
6. **Export regularly** for external analytics tools
7. **Set up alerts** for missing data (gap detection)

---

## 🚨 Troubleshooting

### Database locked error

```bash
# Check for stale connections
lsof database/masterchef.db

# Kill processes if needed
kill <PID>
```

### Missing price data

Price feeds only work on **mainnet** where PancakeSwap liquidity exists. Testnet snapshots won't include price data.

### No data in database

```bash
# Check if collection ran
npm run query-history
# Look at Database Statistics

# Run collection manually
npm run collect-metrics:mainnet
```

### Cron job not running

```bash
# Check cron logs
grep CRON /var/log/syslog

# Test manually
npm run collect-metrics:mainnet
```

---

## 📚 Related Documentation

- **SCRIPTS_README.md** - All available scripts
- **AUTOMATED_MONITORING_SETUP.md** - Health check automation
- **MASTERCHEF_GUIDE.md** - MasterChef operations

---

## 🎉 Summary

✅ **Complete historical tracking** of all MasterChef metrics
✅ **SQLite database** with optimized schema
✅ **TypeScript API** for type-safe operations
✅ **Automated collection** via cron
✅ **Interactive queries** with trend analysis
✅ **Export capabilities** for external tools
✅ **Alert integration** for proactive monitoring

**Next Features:**
- Telegram Bot (coming next)
- Advanced Alert System
- Backup & Recovery
- Analytics API

---

**Ready to track your MasterChef metrics?** Run `npm run collect-metrics:mainnet` to start!
