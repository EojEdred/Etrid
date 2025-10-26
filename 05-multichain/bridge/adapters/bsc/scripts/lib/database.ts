import Database from 'better-sqlite3';
import { existsSync, mkdirSync } from 'fs';
import { join } from 'path';

/**
 * Database Library for Historical Data
 *
 * SQLite-based storage for metrics, events, health checks, and alerts
 */

const DB_PATH = join(__dirname, '../../database/masterchef.db');
const SCHEMA_PATH = join(__dirname, '../../database/schema.sql');

// Ensure database directory exists
const dbDir = join(__dirname, '../../database');
if (!existsSync(dbDir)) {
  mkdirSync(dbDir, { recursive: true });
}

// Initialize database connection
let db: Database.Database | null = null;

export function getDatabase(): Database.Database {
  if (!db) {
    db = new Database(DB_PATH);
    db.pragma('journal_mode = WAL'); // Better concurrency
    initializeSchema();
  }
  return db;
}

function initializeSchema() {
  if (!db) return;

  const fs = require('fs');
  if (existsSync(SCHEMA_PATH)) {
    const schema = fs.readFileSync(SCHEMA_PATH, 'utf-8');
    db.exec(schema);
  }
}

export function closeDatabase() {
  if (db) {
    db.close();
    db = null;
  }
}

// ===== Metrics Snapshots =====

export interface MetricsSnapshot {
  timestamp?: string;
  network: string;
  block_number: number;
  total_pools: number;
  reward_per_block: string;
  total_alloc_point: string;
  masterchef_balance: string;
  days_remaining: number;
  is_paused: boolean;
  bnb_price?: number;
  etr_price?: number;
  total_tvl_usd?: number;
  total_staked_lp: string;
}

export interface PoolSnapshot {
  snapshot_id: number;
  timestamp: string;
  network: string;
  pool_id: number;
  lp_token: string;
  lp_symbol: string;
  lp_name: string;
  total_staked: string;
  alloc_point: string;
  reward_share: number;
  lp_price?: number;
  tvl_usd?: number;
  apr_percent?: number;
  daily_rewards: string;
  monthly_rewards: string;
}

export function saveMetricsSnapshot(metrics: MetricsSnapshot): number {
  const db = getDatabase();

  const stmt = db.prepare(`
    INSERT INTO metrics_snapshots (
      timestamp, network, block_number, total_pools, reward_per_block,
      total_alloc_point, masterchef_balance, days_remaining, is_paused,
      bnb_price, etr_price, total_tvl_usd, total_staked_lp
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
  `);

  const result = stmt.run(
    metrics.timestamp || new Date().toISOString(),
    metrics.network,
    metrics.block_number,
    metrics.total_pools,
    metrics.reward_per_block,
    metrics.total_alloc_point,
    metrics.masterchef_balance,
    metrics.days_remaining,
    metrics.is_paused ? 1 : 0,
    metrics.bnb_price || null,
    metrics.etr_price || null,
    metrics.total_tvl_usd || null,
    metrics.total_staked_lp
  );

  return result.lastInsertRowid as number;
}

export function savePoolSnapshot(pool: PoolSnapshot) {
  const db = getDatabase();

  const stmt = db.prepare(`
    INSERT INTO pool_snapshots (
      snapshot_id, timestamp, network, pool_id, lp_token, lp_symbol, lp_name,
      total_staked, alloc_point, reward_share, lp_price, tvl_usd, apr_percent,
      daily_rewards, monthly_rewards
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
  `);

  stmt.run(
    pool.snapshot_id,
    pool.timestamp,
    pool.network,
    pool.pool_id,
    pool.lp_token,
    pool.lp_symbol,
    pool.lp_name,
    pool.total_staked,
    pool.alloc_point,
    pool.reward_share,
    pool.lp_price || null,
    pool.tvl_usd || null,
    pool.apr_percent || null,
    pool.daily_rewards,
    pool.monthly_rewards
  );
}

// ===== Events =====

export interface Event {
  network: string;
  event_type: string;
  pool_id?: number;
  user_address?: string;
  amount?: string;
  tx_hash?: string;
  block_number?: number;
  details?: any;
}

export function logEvent(event: Event) {
  const db = getDatabase();

  const stmt = db.prepare(`
    INSERT INTO events (network, event_type, pool_id, user_address, amount, tx_hash, block_number, details)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
  `);

  stmt.run(
    event.network,
    event.event_type,
    event.pool_id || null,
    event.user_address || null,
    event.amount || null,
    event.tx_hash || null,
    event.block_number || null,
    event.details ? JSON.stringify(event.details) : null
  );
}

// ===== Health Checks =====

export interface HealthCheck {
  network: string;
  total_checks: number;
  passed: number;
  warnings: number;
  critical_issues: number;
  is_healthy: boolean;
  details: any;
}

export function saveHealthCheck(check: HealthCheck) {
  const db = getDatabase();

  const stmt = db.prepare(`
    INSERT INTO health_checks (network, total_checks, passed, warnings, critical_issues, is_healthy, details)
    VALUES (?, ?, ?, ?, ?, ?, ?)
  `);

  stmt.run(
    check.network,
    check.total_checks,
    check.passed,
    check.warnings,
    check.critical_issues,
    check.is_healthy ? 1 : 0,
    JSON.stringify(check.details)
  );
}

// ===== Alerts =====

export interface Alert {
  network: string;
  severity: 'info' | 'warning' | 'critical';
  alert_type: string;
  message: string;
}

export function createAlert(alert: Alert): number {
  const db = getDatabase();

  const stmt = db.prepare(`
    INSERT INTO alerts (network, severity, alert_type, message)
    VALUES (?, ?, ?, ?)
  `);

  const result = stmt.run(alert.network, alert.severity, alert.alert_type, alert.message);
  return result.lastInsertRowid as number;
}

export function acknowledgeAlert(alertId: number, acknowledgedBy: string) {
  const db = getDatabase();

  const stmt = db.prepare(`
    UPDATE alerts
    SET acknowledged = TRUE, acknowledged_at = CURRENT_TIMESTAMP, acknowledged_by = ?
    WHERE id = ?
  `);

  stmt.run(acknowledgedBy, alertId);
}

export function resolveAlert(alertId: number, resolutionNotes?: string) {
  const db = getDatabase();

  const stmt = db.prepare(`
    UPDATE alerts
    SET resolved = TRUE, resolved_at = CURRENT_TIMESTAMP, resolution_notes = ?
    WHERE id = ?
  `);

  stmt.run(resolutionNotes || null, alertId);
}

export function getActiveAlerts(network?: string): any[] {
  const db = getDatabase();

  let query = `SELECT * FROM active_alerts`;
  if (network) {
    query += ` WHERE network = ?`;
  }

  const stmt = db.prepare(query);
  return network ? stmt.all(network) : stmt.all();
}

// ===== Queries =====

export function getLatestMetrics(network: string): any | null {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT * FROM latest_metrics WHERE network = ?
  `);

  return stmt.get(network);
}

export function getLatestPools(network: string): any[] {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT * FROM latest_pools WHERE network = ? ORDER BY pool_id
  `);

  return stmt.all(network);
}

export function getTVLHistory(network: string, days: number = 30): any[] {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT
      timestamp,
      total_tvl_usd
    FROM metrics_snapshots
    WHERE network = ? AND total_tvl_usd IS NOT NULL
    AND timestamp >= datetime('now', '-' || ? || ' days')
    ORDER BY timestamp ASC
  `);

  return stmt.all(network, days);
}

export function getPoolTVLHistory(network: string, poolId: number, days: number = 30): any[] {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT
      timestamp,
      tvl_usd,
      apr_percent,
      total_staked
    FROM pool_snapshots
    WHERE network = ? AND pool_id = ? AND tvl_usd IS NOT NULL
    AND timestamp >= datetime('now', '-' || ? || ' days')
    ORDER BY timestamp ASC
  `);

  return stmt.all(network, poolId, days);
}

export function getAPRHistory(network: string, poolId: number, days: number = 30): any[] {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT
      timestamp,
      apr_percent
    FROM pool_snapshots
    WHERE network = ? AND pool_id = ? AND apr_percent IS NOT NULL
    AND timestamp >= datetime('now', '-' || ? || ' days')
    ORDER BY timestamp ASC
  `);

  return stmt.all(network, poolId, days);
}

export function getRecentEvents(network: string, limit: number = 100): any[] {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT * FROM events
    WHERE network = ?
    ORDER BY timestamp DESC
    LIMIT ?
  `);

  return stmt.all(network, limit);
}

export function getHealthCheckHistory(network: string, days: number = 7): any[] {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT * FROM health_checks
    WHERE network = ?
    AND timestamp >= datetime('now', '-' || ? || ' days')
    ORDER BY timestamp DESC
  `);

  return stmt.all(network, days);
}

// ===== Backups =====

export function logBackup(backup: {
  backup_type: string;
  file_path: string;
  file_size?: number;
  checksum?: string;
  is_encrypted?: boolean;
  status: string;
  error_message?: string;
}) {
  const db = getDatabase();

  const stmt = db.prepare(`
    INSERT INTO backups (backup_type, file_path, file_size, checksum, is_encrypted, status, error_message)
    VALUES (?, ?, ?, ?, ?, ?, ?)
  `);

  stmt.run(
    backup.backup_type,
    backup.file_path,
    backup.file_size || null,
    backup.checksum || null,
    backup.is_encrypted ? 1 : 0,
    backup.status,
    backup.error_message || null
  );
}

export function getRecentBackups(limit: number = 10): any[] {
  const db = getDatabase();

  const stmt = db.prepare(`
    SELECT * FROM backups
    ORDER BY timestamp DESC
    LIMIT ?
  `);

  return stmt.all(limit);
}

// ===== Statistics =====

export function getDatabaseStats(): any {
  const db = getDatabase();

  const metrics = db.prepare('SELECT COUNT(*) as count FROM metrics_snapshots').get();
  const pools = db.prepare('SELECT COUNT(*) as count FROM pool_snapshots').get();
  const events = db.prepare('SELECT COUNT(*) as count FROM events').get();
  const healthChecks = db.prepare('SELECT COUNT(*) as count FROM health_checks').get();
  const alerts = db.prepare('SELECT COUNT(*) as count FROM alerts').get();
  const backups = db.prepare('SELECT COUNT(*) as count FROM backups').get();

  return {
    metrics_snapshots: (metrics as any).count,
    pool_snapshots: (pools as any).count,
    events: (events as any).count,
    health_checks: (healthChecks as any).count,
    alerts: (alerts as any).count,
    backups: (backups as any).count,
  };
}
