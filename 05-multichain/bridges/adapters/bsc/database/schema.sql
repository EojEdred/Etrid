-- MasterChef Historical Data Schema
-- SQLite compatible (can be adapted for PostgreSQL)

-- Metrics snapshots (hourly)
CREATE TABLE IF NOT EXISTS metrics_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    network TEXT NOT NULL,
    block_number INTEGER NOT NULL,

    -- MasterChef state
    total_pools INTEGER NOT NULL,
    reward_per_block TEXT NOT NULL,
    total_alloc_point TEXT NOT NULL,
    masterchef_balance TEXT NOT NULL,
    days_remaining INTEGER NOT NULL,
    is_paused BOOLEAN NOT NULL,

    -- Prices
    bnb_price REAL,
    etr_price REAL,

    -- Aggregates
    total_tvl_usd REAL,
    total_staked_lp TEXT NOT NULL,

    UNIQUE(timestamp, network)
);

-- Pool snapshots (hourly)
CREATE TABLE IF NOT EXISTS pool_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    snapshot_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    network TEXT NOT NULL,

    pool_id INTEGER NOT NULL,
    lp_token TEXT NOT NULL,
    lp_symbol TEXT NOT NULL,
    lp_name TEXT NOT NULL,

    total_staked TEXT NOT NULL,
    alloc_point TEXT NOT NULL,
    reward_share REAL NOT NULL,

    lp_price REAL,
    tvl_usd REAL,
    apr_percent REAL,

    daily_rewards TEXT NOT NULL,
    monthly_rewards TEXT NOT NULL,

    FOREIGN KEY (snapshot_id) REFERENCES metrics_snapshots(id) ON DELETE CASCADE
);

-- Events log
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    network TEXT NOT NULL,
    event_type TEXT NOT NULL, -- 'deposit', 'withdraw', 'harvest', 'pool_added', 'emission_updated', 'ownership_transferred', 'paused', 'unpaused'
    pool_id INTEGER,
    user_address TEXT,
    amount TEXT,
    tx_hash TEXT,
    block_number INTEGER,
    details TEXT -- JSON
);

-- Health checks log
CREATE TABLE IF NOT EXISTS health_checks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    network TEXT NOT NULL,

    total_checks INTEGER NOT NULL,
    passed INTEGER NOT NULL,
    warnings INTEGER NOT NULL,
    critical_issues INTEGER NOT NULL,

    is_healthy BOOLEAN NOT NULL,
    details TEXT NOT NULL -- JSON with all check results
);

-- Alerts log
CREATE TABLE IF NOT EXISTS alerts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    network TEXT NOT NULL,

    severity TEXT NOT NULL, -- 'info', 'warning', 'critical'
    alert_type TEXT NOT NULL,
    message TEXT NOT NULL,

    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_at DATETIME,
    acknowledged_by TEXT,

    resolved BOOLEAN DEFAULT FALSE,
    resolved_at DATETIME,
    resolution_notes TEXT
);

-- Backup log
CREATE TABLE IF NOT EXISTS backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    backup_type TEXT NOT NULL, -- 'full', 'contracts', 'config', 'database'
    file_path TEXT NOT NULL,
    file_size INTEGER,
    checksum TEXT,
    is_encrypted BOOLEAN DEFAULT FALSE,
    status TEXT NOT NULL, -- 'success', 'failed'
    error_message TEXT
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics_snapshots(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_network ON metrics_snapshots(network);

CREATE INDEX IF NOT EXISTS idx_pool_snapshot_id ON pool_snapshots(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_pool_timestamp ON pool_snapshots(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_pool_network_id ON pool_snapshots(network, pool_id);

CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_pool ON events(pool_id);

CREATE INDEX IF NOT EXISTS idx_health_timestamp ON health_checks(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_health_network ON health_checks(network);

CREATE INDEX IF NOT EXISTS idx_alerts_timestamp ON alerts(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_alerts_severity ON alerts(severity);
CREATE INDEX IF NOT EXISTS idx_alerts_acknowledged ON alerts(acknowledged);

CREATE INDEX IF NOT EXISTS idx_backups_timestamp ON backups(timestamp DESC);

-- Views for common queries

-- Latest metrics by network
CREATE VIEW IF NOT EXISTS latest_metrics AS
SELECT *
FROM metrics_snapshots
WHERE id IN (
    SELECT MAX(id)
    FROM metrics_snapshots
    GROUP BY network
);

-- Latest pool data
CREATE VIEW IF NOT EXISTS latest_pools AS
SELECT p.*
FROM pool_snapshots p
INNER JOIN (
    SELECT network, pool_id, MAX(timestamp) as max_timestamp
    FROM pool_snapshots
    GROUP BY network, pool_id
) latest ON p.network = latest.network
    AND p.pool_id = latest.pool_id
    AND p.timestamp = latest.max_timestamp;

-- TVL trends (daily)
CREATE VIEW IF NOT EXISTS tvl_daily AS
SELECT
    DATE(timestamp) as date,
    network,
    AVG(total_tvl_usd) as avg_tvl,
    MIN(total_tvl_usd) as min_tvl,
    MAX(total_tvl_usd) as max_tvl
FROM metrics_snapshots
WHERE total_tvl_usd IS NOT NULL
GROUP BY DATE(timestamp), network;

-- Active alerts
CREATE VIEW IF NOT EXISTS active_alerts AS
SELECT *
FROM alerts
WHERE acknowledged = FALSE AND resolved = FALSE
ORDER BY
    CASE severity
        WHEN 'critical' THEN 1
        WHEN 'warning' THEN 2
        WHEN 'info' THEN 3
    END,
    timestamp DESC;
