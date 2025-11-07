/**
 * Database - Historical data storage
 * SQLite for simplicity and portability
 */

const sqlite3 = require('sqlite3').verbose();
const path = require('path');
const os = require('os');
const fs = require('fs');

class Database {
  constructor(config) {
    this.config = config;
    this.dbPath = config.database?.path || path.join(os.homedir(), 'pinokio', 'etrid-data.db');
    this.db = null;

    this.ensureDbDirectory();
  }

  ensureDbDirectory() {
    const dir = path.dirname(this.dbPath);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
  }

  /**
   * Initialize database
   */
  async init() {
    return new Promise((resolve, reject) => {
      this.db = new sqlite3.Database(this.dbPath, async (err) => {
        if (err) {
          reject(err);
          return;
        }

        console.log(`✅ Database connected: ${this.dbPath}`);

        try {
          await this.createTables();
          resolve();
        } catch (err) {
          reject(err);
        }
      });
    });
  }

  /**
   * Create tables
   */
  async createTables() {
    const tables = [
      // Node status history
      `CREATE TABLE IF NOT EXISTS node_status (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        chain TEXT NOT NULL,
        node TEXT NOT NULL,
        status TEXT NOT NULL,
        block_height INTEGER,
        peers INTEGER,
        syncing INTEGER,
        timestamp INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )`,

      // Metrics history
      `CREATE TABLE IF NOT EXISTS metrics (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        chain TEXT NOT NULL,
        node TEXT NOT NULL,
        cpu REAL,
        memory REAL,
        disk REAL,
        network_in REAL,
        network_out REAL,
        timestamp INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )`,

      // Alerts history
      `CREATE TABLE IF NOT EXISTS alerts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        severity TEXT NOT NULL,
        title TEXT NOT NULL,
        message TEXT NOT NULL,
        chain TEXT,
        node TEXT,
        details TEXT,
        sent_channels TEXT,
        timestamp INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )`,

      // Health check results
      `CREATE TABLE IF NOT EXISTS health_checks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        chain TEXT NOT NULL,
        healthy INTEGER NOT NULL,
        issues_count INTEGER,
        critical_count INTEGER,
        warnings_count INTEGER,
        details TEXT,
        timestamp INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )`,

      // Events log
      `CREATE TABLE IF NOT EXISTS events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        event_type TEXT NOT NULL,
        chain TEXT,
        node TEXT,
        description TEXT,
        data TEXT,
        timestamp INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )`,

      // Performance analytics
      `CREATE TABLE IF NOT EXISTS performance (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        chain TEXT NOT NULL,
        avg_block_time REAL,
        blocks_produced INTEGER,
        missed_blocks INTEGER,
        uptime REAL,
        period_start INTEGER NOT NULL,
        period_end INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )`
    ];

    for (const table of tables) {
      await this.run(table);
    }

    // Create indexes
    await this.run('CREATE INDEX IF NOT EXISTS idx_node_status_timestamp ON node_status(timestamp)');
    await this.run('CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics(timestamp)');
    await this.run('CREATE INDEX IF NOT EXISTS idx_alerts_timestamp ON alerts(timestamp)');
    await this.run('CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp)');

    console.log('✅ Database tables created');
  }

  /**
   * Store node status
   */
  async storeNodeStatus(chain, node, status) {
    return await this.run(
      `INSERT INTO node_status (chain, node, status, block_height, peers, syncing, timestamp)
       VALUES (?, ?, ?, ?, ?, ?, ?)`,
      [
        chain,
        node,
        status.status,
        status.blockHeight || null,
        status.peers || null,
        status.syncing ? 1 : 0,
        Date.now()
      ]
    );
  }

  /**
   * Store metrics
   */
  async storeMetrics(metricsData) {
    const statements = [];

    for (const [chain, chainMetrics] of Object.entries(metricsData)) {
      for (const nodeMetrics of chainMetrics) {
        statements.push({
          sql: `INSERT INTO metrics (chain, node, cpu, memory, disk, network_in, network_out, timestamp)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
          params: [
            chain,
            nodeMetrics.node,
            nodeMetrics.cpu || null,
            nodeMetrics.memory || null,
            nodeMetrics.disk || null,
            nodeMetrics.network?.in || null,
            nodeMetrics.network?.out || null,
            Date.now()
          ]
        });
      }
    }

    return await this.runBatch(statements);
  }

  /**
   * Store alert
   */
  async storeAlert(alert, sentChannels = []) {
    return await this.run(
      `INSERT INTO alerts (severity, title, message, chain, node, details, sent_channels, timestamp)
       VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
      [
        alert.severity,
        alert.title,
        alert.message,
        alert.chain || null,
        alert.node || null,
        JSON.stringify(alert.details || {}),
        sentChannels.join(','),
        Date.now()
      ]
    );
  }

  /**
   * Store health check result
   */
  async storeHealthCheck(chain, result) {
    return await this.run(
      `INSERT INTO health_checks (chain, healthy, issues_count, critical_count, warnings_count, details, timestamp)
       VALUES (?, ?, ?, ?, ?, ?, ?)`,
      [
        chain,
        result.healthy ? 1 : 0,
        result.issues?.length || 0,
        result.criticalIssues?.length || 0,
        result.warnings?.length || 0,
        JSON.stringify(result),
        Date.now()
      ]
    );
  }

  /**
   * Log event
   */
  async logEvent(eventType, description, data = {}, chain = null, node = null) {
    return await this.run(
      `INSERT INTO events (event_type, chain, node, description, data, timestamp)
       VALUES (?, ?, ?, ?, ?, ?)`,
      [
        eventType,
        chain,
        node,
        description,
        JSON.stringify(data),
        Date.now()
      ]
    );
  }

  /**
   * Query methods
   */

  async getNodeStatusHistory(chain, node, since = null, limit = 1000) {
    let sql = 'SELECT * FROM node_status WHERE chain = ? AND node = ?';
    const params = [chain, node];

    if (since) {
      sql += ' AND timestamp > ?';
      params.push(since);
    }

    sql += ' ORDER BY timestamp DESC LIMIT ?';
    params.push(limit);

    return await this.all(sql, params);
  }

  async getMetricsHistory(chain, node, since = null, limit = 1000) {
    let sql = 'SELECT * FROM metrics WHERE chain = ? AND node = ?';
    const params = [chain, node];

    if (since) {
      sql += ' AND timestamp > ?';
      params.push(since);
    }

    sql += ' ORDER BY timestamp DESC LIMIT ?';
    params.push(limit);

    return await this.all(sql, params);
  }

  async getAlertHistory(severity = null, limit = 100) {
    let sql = 'SELECT * FROM alerts';
    const params = [];

    if (severity) {
      sql += ' WHERE severity = ?';
      params.push(severity);
    }

    sql += ' ORDER BY timestamp DESC LIMIT ?';
    params.push(limit);

    return await this.all(sql, params);
  }

  async getHealthCheckHistory(chain, since = null, limit = 100) {
    let sql = 'SELECT * FROM health_checks WHERE chain = ?';
    const params = [chain];

    if (since) {
      sql += ' AND timestamp > ?';
      params.push(since);
    }

    sql += ' ORDER BY timestamp DESC LIMIT ?';
    params.push(limit);

    return await this.all(sql, params);
  }

  async getEventHistory(eventType = null, limit = 100) {
    let sql = 'SELECT * FROM events';
    const params = [];

    if (eventType) {
      sql += ' WHERE event_type = ?';
      params.push(eventType);
    }

    sql += ' ORDER BY timestamp DESC LIMIT ?';
    params.push(limit);

    return await this.all(sql, params);
  }

  /**
   * Analytics queries
   */

  async getNodeUptime(chain, node, periodDays = 7) {
    const since = Date.now() - (periodDays * 24 * 60 * 60 * 1000);

    const result = await this.get(
      `SELECT
        COUNT(*) as total_checks,
        SUM(CASE WHEN status = 'online' THEN 1 ELSE 0 END) as online_checks
       FROM node_status
       WHERE chain = ? AND node = ? AND timestamp > ?`,
      [chain, node, since]
    );

    if (!result || result.total_checks === 0) return 0;

    return (result.online_checks / result.total_checks) * 100;
  }

  async getAvgMetrics(chain, node, periodDays = 1) {
    const since = Date.now() - (periodDays * 24 * 60 * 60 * 1000);

    return await this.get(
      `SELECT
        AVG(cpu) as avg_cpu,
        AVG(memory) as avg_memory,
        AVG(disk) as avg_disk,
        MAX(cpu) as max_cpu,
        MAX(memory) as max_memory
       FROM metrics
       WHERE chain = ? AND node = ? AND timestamp > ?`,
      [chain, node, since]
    );
  }

  async getAlertStats(periodDays = 7) {
    const since = Date.now() - (periodDays * 24 * 60 * 60 * 1000);

    return await this.all(
      `SELECT
        severity,
        COUNT(*) as count
       FROM alerts
       WHERE timestamp > ?
       GROUP BY severity`,
      [since]
    );
  }

  /**
   * Data cleanup
   */

  async cleanOldData(retentionDays = 30) {
    const cutoff = Date.now() - (retentionDays * 24 * 60 * 60 * 1000);

    const tables = ['node_status', 'metrics', 'alerts', 'health_checks', 'events'];
    let totalDeleted = 0;

    for (const table of tables) {
      const result = await this.run(
        `DELETE FROM ${table} WHERE timestamp < ?`,
        [cutoff]
      );
      totalDeleted += result.changes;
    }

    console.log(`Cleaned ${totalDeleted} old records (older than ${retentionDays} days)`);

    // Vacuum to reclaim space
    await this.run('VACUUM');

    return totalDeleted;
  }

  /**
   * Database utilities
   */

  run(sql, params = []) {
    return new Promise((resolve, reject) => {
      this.db.run(sql, params, function(err) {
        if (err) {
          reject(err);
        } else {
          resolve({ id: this.lastID, changes: this.changes });
        }
      });
    });
  }

  get(sql, params = []) {
    return new Promise((resolve, reject) => {
      this.db.get(sql, params, (err, row) => {
        if (err) reject(err);
        else resolve(row);
      });
    });
  }

  all(sql, params = []) {
    return new Promise((resolve, reject) => {
      this.db.all(sql, params, (err, rows) => {
        if (err) reject(err);
        else resolve(rows);
      });
    });
  }

  async runBatch(statements) {
    return new Promise((resolve, reject) => {
      this.db.serialize(() => {
        this.db.run('BEGIN TRANSACTION');

        for (const stmt of statements) {
          this.db.run(stmt.sql, stmt.params, (err) => {
            if (err) {
              this.db.run('ROLLBACK');
              reject(err);
            }
          });
        }

        this.db.run('COMMIT', (err) => {
          if (err) reject(err);
          else resolve();
        });
      });
    });
  }

  /**
   * Close database
   */
  async close() {
    return new Promise((resolve, reject) => {
      this.db.close((err) => {
        if (err) reject(err);
        else {
          console.log('✅ Database closed');
          resolve();
        }
      });
    });
  }

  /**
   * Backup database
   */
  async backup(backupPath) {
    return new Promise((resolve, reject) => {
      const backup = fs.createWriteStream(backupPath);
      const read = fs.createReadStream(this.dbPath);

      read.pipe(backup);

      backup.on('finish', () => {
        console.log(`✅ Database backed up to: ${backupPath}`);
        resolve();
      });

      backup.on('error', reject);
    });
  }
}

module.exports = { Database };
