/**
 * Etrid Blockchain Operations Center - Main API
 * Custom Pinokio API for managing Etrid mainnet infrastructure
 */

const { NodeMonitor } = require('./node-monitor');
const { SSHManager } = require('./ssh-manager');
const { ChainHealth } = require('./chain-health');
const { ClaudeIntegration } = require('./claude-integration');
const { AlertSystem } = require('./alert-system');
const { Scheduler } = require('./scheduler');
const { Database } = require('./database');
const fs = require('fs');
const path = require('path');

class EtridAPI {
  constructor() {
    this.configPath = path.join(__dirname, 'config.json');
    this.config = this.loadConfig();
    this.monitor = new NodeMonitor(this.config);
    this.ssh = new SSHManager(this.config);
    this.health = new ChainHealth(this.config);
    this.claude = new ClaudeIntegration(this.config);
    this.alerts = new AlertSystem(this.config);
    this.scheduler = new Scheduler(this);
    this.database = new Database(this.config);

    // Initialize database and scheduler
    this.init();
  }

  async init() {
    try {
      await this.database.init();

      // Start scheduler if enabled in config
      if (this.config.scheduler?.enabled !== false) {
        this.scheduler.start();
      }

      console.log('✅ Etrid Operations Center initialized');
    } catch (err) {
      console.error('Initialization error:', err);
    }
  }

  loadConfig() {
    try {
      return JSON.parse(fs.readFileSync(this.configPath, 'utf8'));
    } catch (err) {
      console.error('Error loading config:', err);
      return this.getDefaultConfig();
    }
  }

  getDefaultConfig() {
    return {
      chains: {
        flarechain: {
          name: "FlareChain Mainnet",
          type: "main",
          nodes: []
        },
        pbcs: [
          { name: "BTC-PBC", ticker: "BTC" },
          { name: "ETH-PBC", ticker: "ETH" },
          { name: "SOL-PBC", ticker: "SOL" },
          { name: "BNB-PBC", ticker: "BNB" },
          { name: "ADA-PBC", ticker: "ADA" },
          { name: "DOGE-PBC", ticker: "DOGE" },
          { name: "LINK-PBC", ticker: "LINK" },
          { name: "MATIC-PBC", ticker: "MATIC" },
          { name: "SC-USDT-PBC", ticker: "USDT" },
          { name: "TRX-PBC", ticker: "TRX" },
          { name: "XLM-PBC", ticker: "XLM" },
          { name: "XRP-PBC", ticker: "XRP" }
        ]
      },
      clouds: {
        aws: { nodes: [] },
        gcp: { nodes: [] },
        azure: { nodes: [] },
        digitalocean: { nodes: [] }
      }
    };
  }

  /**
   * Get status of all nodes across all chains
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async status(request, ondata, kernel) {
    const { chains = 'all', verbose = false } = request.params || {};

    ondata({ raw: '🔍 Checking node status across all chains...\n' });

    try {
      const status = await this.monitor.getAllStatus(chains, (update) => {
        ondata({ raw: update });
      });

      ondata({
        raw: '\n✅ Status check complete\n',
        state: { status }
      });

      return status;
    } catch (err) {
      ondata({ raw: `\n❌ Error: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * Connect to a node via SSH
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async connect(request, ondata, kernel) {
    const { node, cloud, command } = request.params || {};

    if (!node) {
      throw new Error('Node parameter required');
    }

    ondata({ raw: `🔐 Connecting to ${node}...\n` });

    try {
      const connection = await this.ssh.connect(node, cloud, (output) => {
        ondata({ raw: output });
      });

      if (command) {
        const result = await connection.exec(command);
        ondata({ raw: result });
      }

      return {
        success: true,
        node,
        message: 'Connected successfully'
      };
    } catch (err) {
      ondata({ raw: `\n❌ Connection failed: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * Run health check on specified chains
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async healthcheck(request, ondata, kernel) {
    const { chains = 'all', fix = false } = request.params || {};

    ondata({ raw: '🏥 Running health checks...\n' });

    try {
      const results = await this.health.check(chains, {
        onProgress: (update) => ondata({ raw: update }),
        autoFix: fix
      });

      const summary = this.health.summarize(results);
      ondata({ raw: `\n${summary}\n` });

      return results;
    } catch (err) {
      ondata({ raw: `\n❌ Health check failed: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * Aggregate logs from all nodes
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async logs(request, ondata, kernel) {
    const {
      chains = 'all',
      filter,
      since = '1h',
      analyze = false
    } = request.params || {};

    ondata({ raw: '📋 Fetching logs...\n' });

    try {
      const logs = await this.monitor.getLogs(chains, { since, filter }, (update) => {
        ondata({ raw: update });
      });

      if (analyze) {
        ondata({ raw: '\n🤖 Analyzing logs with Claude Code...\n' });
        const analysis = await this.claude.analyzeLogs(logs);
        ondata({ raw: `\n${analysis}\n` });
        return { logs, analysis };
      }

      return logs;
    } catch (err) {
      ondata({ raw: `\n❌ Error fetching logs: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * List all nodes with their configuration
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async list(request, ondata, kernel) {
    const { cloud, chain, format = 'table' } = request.params || {};

    ondata({ raw: '📝 Listing nodes...\n\n' });

    const nodes = this.getAllNodes(cloud, chain);

    if (format === 'table') {
      const table = this.formatNodesTable(nodes);
      ondata({ raw: table });
    } else {
      ondata({ raw: JSON.stringify(nodes, null, 2) });
    }

    return nodes;
  }

  /**
   * Execute command on multiple nodes
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async exec(request, ondata, kernel) {
    const { nodes, command, parallel = true } = request.params || {};

    if (!command) {
      throw new Error('Command parameter required');
    }

    ondata({ raw: `⚡ Executing on ${nodes.length} nodes...\n\n` });

    try {
      const results = await this.ssh.execMultiple(nodes, command, {
        parallel,
        onProgress: (node, output) => {
          ondata({ raw: `[${node}] ${output}` });
        }
      });

      ondata({ raw: '\n✅ Execution complete\n' });
      return results;
    } catch (err) {
      ondata({ raw: `\n❌ Execution failed: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * Update node software across chains
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async update(request, ondata, kernel) {
    const {
      chains = 'all',
      version,
      dryRun = false,
      rollback = false
    } = request.params || {};

    ondata({ raw: `🔄 ${dryRun ? 'Simulating' : 'Starting'} update process...\n` });

    try {
      const plan = await this.health.createUpdatePlan(chains, version);

      ondata({ raw: `\nUpdate plan:\n${JSON.stringify(plan, null, 2)}\n\n` });

      if (dryRun) {
        return { plan, dryRun: true };
      }

      const results = await this.health.executeUpdate(plan, (update) => {
        ondata({ raw: update });
      });

      ondata({ raw: '\n✅ Update complete\n' });
      return results;
    } catch (err) {
      ondata({ raw: `\n❌ Update failed: ${err.message}\n` });
      if (!dryRun) {
        ondata({ raw: '\n⚠️  Consider running rollback\n' });
      }
      throw err;
    }
  }

  /**
   * Get AI assistance from Claude Code
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async claude(request, ondata, kernel) {
    const { query, context = {} } = request.params || {};

    if (!query) {
      throw new Error('Query parameter required');
    }

    ondata({ raw: '🤖 Asking Claude Code...\n\n' });

    try {
      const response = await this.claude.ask(query, context, (chunk) => {
        ondata({ raw: chunk });
      });

      return response;
    } catch (err) {
      ondata({ raw: `\n❌ Error: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * Send alert through configured channels
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async alert(request, ondata, kernel) {
    const alert = request.params || {};

    ondata({ raw: `📢 Sending alert: ${alert.title}\n` });

    try {
      const result = await this.alerts.sendAlert(alert);

      // Store in database
      if (this.database) {
        await this.database.storeAlert(alert, result.results?.map(r => r.channel) || []);
      }

      ondata({ raw: `✅ Alert sent to ${result.results?.length || 0} channels\n` });
      return result;
    } catch (err) {
      ondata({ raw: `\n❌ Alert failed: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * Get scheduler status and job history
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async scheduler(request, ondata, kernel) {
    const { action, job } = request.params || {};

    if (action === 'status') {
      const status = this.scheduler.getJobStatus();
      ondata({ raw: JSON.stringify(status, null, 2) });
      return status;
    }

    if (action === 'history') {
      const history = this.scheduler.getJobHistory(job);
      ondata({ raw: JSON.stringify(history, null, 2) });
      return history;
    }

    if (action === 'run' && job) {
      ondata({ raw: `▶️  Running job: ${job}\n` });
      const result = await this.scheduler.runJob(job);
      ondata({ raw: `✅ Job complete\n` });
      return result;
    }

    if (action === 'start') {
      this.scheduler.start();
      ondata({ raw: '✅ Scheduler started\n' });
      return { success: true };
    }

    if (action === 'stop') {
      this.scheduler.stop();
      ondata({ raw: '⏹️  Scheduler stopped\n' });
      return { success: true };
    }

    throw new Error('Invalid scheduler action. Use: status, history, run, start, stop');
  }

  /**
   * Query database for historical data
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async history(request, ondata, kernel) {
    const { type, chain, node, since, limit } = request.params || {};

    ondata({ raw: `📊 Fetching ${type} history...\n` });

    try {
      let data;

      switch (type) {
        case 'status':
          data = await this.database.getNodeStatusHistory(chain, node, since, limit);
          break;
        case 'metrics':
          data = await this.database.getMetricsHistory(chain, node, since, limit);
          break;
        case 'alerts':
          data = await this.database.getAlertHistory(chain, limit);
          break;
        case 'health':
          data = await this.database.getHealthCheckHistory(chain, since, limit);
          break;
        case 'events':
          data = await this.database.getEventHistory(null, limit);
          break;
        case 'uptime':
          const uptime = await this.database.getNodeUptime(chain, node, 7);
          data = { chain, node, uptime: `${uptime.toFixed(2)}%` };
          break;
        default:
          throw new Error('Invalid type. Use: status, metrics, alerts, health, events, uptime');
      }

      ondata({ raw: `\n✅ Found ${Array.isArray(data) ? data.length : 1} records\n` });
      return data;
    } catch (err) {
      ondata({ raw: `\n❌ Error: ${err.message}\n` });
      throw err;
    }
  }

  /**
   * Test alert system
   * @param {Object} request - Pinokio request object
   * @param {Function} ondata - Callback for streaming updates
   * @param {Object} kernel - Pinokio kernel
   */
  async test(request, ondata, kernel) {
    const { type = 'alert' } = request.params || {};

    if (type === 'alert') {
      ondata({ raw: '🔔 Sending test alert...\n' });
      const result = await this.alerts.sendTestAlert();
      ondata({ raw: '✅ Test alert sent!\n' });
      return result;
    }

    throw new Error('Invalid test type. Use: alert');
  }

  // Helper methods
  getAllNodes(cloud, chain) {
    // Implementation to get all nodes filtered by cloud/chain
    const nodes = [];

    for (const [cloudName, cloudConfig] of Object.entries(this.config.clouds)) {
      if (cloud && cloud !== cloudName) continue;

      for (const node of cloudConfig.nodes || []) {
        if (chain && node.chain !== chain) continue;
        nodes.push({ ...node, cloud: cloudName });
      }
    }

    return nodes;
  }

  formatNodesTable(nodes) {
    // Simple table formatting
    const header = '| Node | Chain | Cloud | Status | IP |\n|------|-------|-------|--------|----|\n';
    const rows = nodes.map(n =>
      `| ${n.name} | ${n.chain} | ${n.cloud} | ${n.status || 'unknown'} | ${n.ip || 'N/A'} |`
    ).join('\n');

    return header + rows + '\n';
  }
}

module.exports = EtridAPI;
