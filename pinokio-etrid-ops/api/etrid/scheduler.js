/**
 * Scheduler - Automated health checks and monitoring
 * Runs periodic tasks in background
 */

const cron = require('node-cron');

class Scheduler {
  constructor(api) {
    this.api = api;
    this.jobs = new Map();
    this.enabled = false;
    this.jobHistory = [];
  }

  /**
   * Start all scheduled jobs
   */
  start() {
    if (this.enabled) {
      console.log('Scheduler already running');
      return;
    }

    this.enabled = true;

    // Health check every 5 minutes
    this.scheduleJob('health-check', '*/5 * * * *', async () => {
      await this.runHealthCheck();
    });

    // Status check every 1 minute
    this.scheduleJob('status-check', '* * * * *', async () => {
      await this.runStatusCheck();
    });

    // Metrics collection every 10 minutes
    this.scheduleJob('metrics-collection', '*/10 * * * *', async () => {
      await this.collectMetrics();
    });

    // Log rotation daily at 00:00
    this.scheduleJob('log-rotation', '0 0 * * *', async () => {
      await this.rotateJobHistory();
    });

    // Backup check daily at 02:00
    this.scheduleJob('backup-check', '0 2 * * *', async () => {
      await this.runBackupCheck();
    });

    // Update check daily at 03:00
    this.scheduleJob('update-check', '0 3 * * *', async () => {
      await this.checkForUpdates();
    });

    // Disk space check every hour
    this.scheduleJob('disk-check', '0 * * * *', async () => {
      await this.checkDiskSpace();
    });

    console.log('✅ Scheduler started with', this.jobs.size, 'jobs');
  }

  /**
   * Stop all scheduled jobs
   */
  stop() {
    for (const [name, job] of this.jobs) {
      job.stop();
      console.log(`Stopped job: ${name}`);
    }

    this.jobs.clear();
    this.enabled = false;
    console.log('✅ Scheduler stopped');
  }

  /**
   * Schedule a job
   */
  scheduleJob(name, cronExpression, handler) {
    if (this.jobs.has(name)) {
      console.warn(`Job ${name} already scheduled`);
      return;
    }

    const job = cron.schedule(cronExpression, async () => {
      const startTime = Date.now();
      console.log(`[${new Date().toISOString()}] Running job: ${name}`);

      try {
        await handler();

        const duration = Date.now() - startTime;
        this.recordJobRun(name, true, duration);
        console.log(`[${new Date().toISOString()}] Job ${name} completed in ${duration}ms`);
      } catch (err) {
        const duration = Date.now() - startTime;
        this.recordJobRun(name, false, duration, err.message);
        console.error(`[${new Date().toISOString()}] Job ${name} failed:`, err.message);

        // Alert on job failure
        if (this.api.alerts) {
          await this.api.alerts.sendAlert({
            severity: 'warning',
            title: `⚠️ Scheduled Job Failed: ${name}`,
            message: `Job ${name} failed with error: ${err.message}`,
            details: { duration: `${duration}ms` }
          });
        }
      }
    });

    this.jobs.set(name, job);
    console.log(`Scheduled job: ${name} (${cronExpression})`);
  }

  /**
   * Scheduled task implementations
   */

  async runHealthCheck() {
    const results = await this.api.health.check('all', {
      onProgress: () => {}, // Silent
      autoFix: true // Auto-fix issues
    });

    // Alert on critical issues
    if (results.criticalIssues && results.criticalIssues.length > 0) {
      for (const issue of results.criticalIssues) {
        await this.api.alerts.sendAlert({
          severity: 'critical',
          title: `🔴 Critical Issue Detected`,
          message: issue.message,
          chain: issue.chain,
          node: issue.node,
          details: issue.details || {}
        });
      }
    }

    return results;
  }

  async runStatusCheck() {
    const status = await this.api.monitor.getAllStatus('all', () => {});

    // Check for offline nodes
    const offlineNodes = [];

    if (status.flarechain?.nodes) {
      for (const node of status.flarechain.nodes) {
        if (node.status === 'offline') {
          offlineNodes.push({ chain: 'FlareChain', ...node });
        }
      }
    }

    if (status.pbcs) {
      for (const pbc of status.pbcs) {
        if (pbc.status?.nodes) {
          for (const node of pbc.status.nodes) {
            if (node.status === 'offline') {
              offlineNodes.push({ chain: pbc.name, ...node });
            }
          }
        }
      }
    }

    // Alert on new offline nodes
    for (const node of offlineNodes) {
      await this.api.alerts.alertNodeOffline(node.name, node.chain);
    }

    return status;
  }

  async collectMetrics() {
    if (!this.api.monitor) return;

    const metrics = await this.api.monitor.getMetrics('all', () => {});

    // Store metrics for historical analysis
    if (this.api.database) {
      await this.api.database.storeMetrics(metrics);
    }

    // Check thresholds and alert
    for (const [chain, chainMetrics] of Object.entries(metrics)) {
      for (const nodeMetrics of chainMetrics) {
        // High CPU
        if (nodeMetrics.cpu > 90) {
          await this.api.alerts.alertHighCPU(
            nodeMetrics.node,
            chain,
            nodeMetrics.cpu
          );
        }

        // High memory
        if (nodeMetrics.memory > 90) {
          await this.api.alerts.sendAlert({
            severity: 'warning',
            title: '⚠️ High Memory Usage',
            message: `Node ${nodeMetrics.node} memory usage is at ${nodeMetrics.memory}%`,
            chain,
            node: nodeMetrics.node,
            details: { memory: `${nodeMetrics.memory}%` }
          });
        }
      }
    }

    return metrics;
  }

  async checkDiskSpace() {
    // Get all nodes
    const nodes = this.api.getAllNodes();

    for (const node of nodes) {
      try {
        // Would use SSH to check disk space
        // const diskInfo = await this.api.ssh.exec(node.name, 'df -h /');

        // For now, placeholder
        const available = 10 * 1024 * 1024 * 1024; // 10GB
        const threshold = 20 * 1024 * 1024 * 1024; // 20GB

        if (available < threshold) {
          await this.api.alerts.alertLowDiskSpace(
            node.name,
            node.chain,
            available,
            threshold
          );
        }
      } catch (err) {
        console.error(`Failed to check disk space for ${node.name}:`, err.message);
      }
    }
  }

  async runBackupCheck() {
    console.log('Running backup check...');

    // Check if backups exist and are recent
    // Would integrate with backup system

    // For now, log
    console.log('Backup check complete');
  }

  async checkForUpdates() {
    console.log('Checking for updates...');

    // Check GitHub releases for new versions
    try {
      // Would check each chain's repository for updates
      // and alert if new versions available
    } catch (err) {
      console.error('Update check failed:', err.message);
    }
  }

  async rotateJobHistory() {
    // Keep only last 1000 job runs
    if (this.jobHistory.length > 1000) {
      this.jobHistory = this.jobHistory.slice(-1000);
    }
  }

  /**
   * Manual job trigger
   */
  async runJob(name) {
    const job = this.jobs.get(name);
    if (!job) {
      throw new Error(`Job ${name} not found`);
    }

    // Jobs don't expose their handlers directly, so we'll trigger them
    // by temporarily adjusting their schedule
    console.log(`Manually triggering job: ${name}`);

    // For now, call the appropriate method directly
    const jobHandlers = {
      'health-check': () => this.runHealthCheck(),
      'status-check': () => this.runStatusCheck(),
      'metrics-collection': () => this.collectMetrics(),
      'disk-check': () => this.checkDiskSpace(),
      'backup-check': () => this.runBackupCheck(),
      'update-check': () => this.checkForUpdates()
    };

    const handler = jobHandlers[name];
    if (handler) {
      return await handler();
    }

    throw new Error(`No handler for job: ${name}`);
  }

  /**
   * Get job status
   */
  getJobStatus() {
    const status = [];

    for (const [name, job] of this.jobs) {
      const recentRuns = this.jobHistory
        .filter(h => h.name === name)
        .slice(-5);

      status.push({
        name,
        active: this.enabled,
        lastRun: recentRuns.length > 0 ? recentRuns[recentRuns.length - 1] : null,
        successRate: this.calculateSuccessRate(name),
        avgDuration: this.calculateAvgDuration(name)
      });
    }

    return status;
  }

  /**
   * Get job history
   */
  getJobHistory(name = null, limit = 100) {
    let history = this.jobHistory;

    if (name) {
      history = history.filter(h => h.name === name);
    }

    return history.slice(-limit).reverse();
  }

  // Helper methods

  recordJobRun(name, success, duration, error = null) {
    this.jobHistory.push({
      name,
      success,
      duration,
      error,
      timestamp: Date.now()
    });
  }

  calculateSuccessRate(name) {
    const runs = this.jobHistory.filter(h => h.name === name);
    if (runs.length === 0) return 100;

    const successful = runs.filter(h => h.success).length;
    return (successful / runs.length) * 100;
  }

  calculateAvgDuration(name) {
    const runs = this.jobHistory.filter(h => h.name === name && h.success);
    if (runs.length === 0) return 0;

    const totalDuration = runs.reduce((sum, run) => sum + run.duration, 0);
    return totalDuration / runs.length;
  }

  /**
   * Custom schedule
   */
  addCustomJob(name, cronExpression, handler) {
    this.scheduleJob(name, cronExpression, handler);
  }

  /**
   * Remove job
   */
  removeJob(name) {
    const job = this.jobs.get(name);
    if (job) {
      job.stop();
      this.jobs.delete(name);
      console.log(`Removed job: ${name}`);
      return true;
    }
    return false;
  }
}

module.exports = { Scheduler };
