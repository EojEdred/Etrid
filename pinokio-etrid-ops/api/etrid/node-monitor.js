/**
 * Node Monitoring System
 * Real-time status monitoring for FlareChain and PBC nodes
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');
const axios = require('axios');

class NodeMonitor {
  constructor(config) {
    this.config = config;
    this.apiConnections = new Map();
    this.statusCache = new Map();
    this.cacheTTL = 30000; // 30 seconds
  }

  /**
   * Get status of all nodes
   */
  async getAllStatus(chains = 'all', onProgress = () => {}) {
    const results = {
      flarechain: null,
      pbcs: [],
      summary: {
        total: 0,
        online: 0,
        offline: 0,
        syncing: 0
      }
    };

    // Check FlareChain
    if (chains === 'all' || chains === 'flarechain') {
      onProgress('📡 Checking FlareChain mainnet...\n');
      results.flarechain = await this.getChainStatus('flarechain', onProgress);
      this.updateSummary(results.summary, results.flarechain);
    }

    // Check PBC chains
    if (chains === 'all' || chains === 'pbcs') {
      for (const pbc of this.config.chains.pbcs) {
        onProgress(`📡 Checking ${pbc.name}...\n`);
        const status = await this.getChainStatus(pbc.name.toLowerCase(), onProgress);
        results.pbcs.push({
          ...pbc,
          status
        });
        this.updateSummary(results.summary, status);
      }
    }

    return results;
  }

  /**
   * Get status of a specific chain
   */
  async getChainStatus(chainName, onProgress = () => {}) {
    const cached = this.getCachedStatus(chainName);
    if (cached) {
      onProgress(`  ♻️  Using cached status\n`);
      return cached;
    }

    const nodes = this.getChainNodes(chainName);
    const status = {
      chain: chainName,
      nodes: [],
      consensus: null,
      blockHeight: null,
      timestamp: Date.now()
    };

    for (const node of nodes) {
      try {
        const nodeStatus = await this.getNodeStatus(node, onProgress);
        status.nodes.push(nodeStatus);

        // Update consensus info from first healthy node
        if (!status.blockHeight && nodeStatus.blockHeight) {
          status.blockHeight = nodeStatus.blockHeight;
          status.consensus = nodeStatus.consensus;
        }
      } catch (err) {
        onProgress(`  ⚠️  ${node.name}: ${err.message}\n`);
        status.nodes.push({
          name: node.name,
          status: 'offline',
          error: err.message
        });
      }
    }

    this.cacheStatus(chainName, status);
    return status;
  }

  /**
   * Get status of individual node
   */
  async getNodeStatus(node, onProgress = () => {}) {
    const result = {
      name: node.name,
      ip: node.ip,
      status: 'unknown',
      blockHeight: null,
      peers: null,
      version: null,
      syncing: false,
      consensus: null,
      timestamp: Date.now()
    };

    try {
      // Connect to node RPC
      const api = await this.connectToNode(node);

      // Get basic info
      const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
      ]);

      result.chain = chain.toString();
      result.version = nodeVersion.toString();

      // Get block info
      const header = await api.rpc.chain.getHeader();
      result.blockHeight = header.number.toNumber();

      // Get sync status
      const health = await api.rpc.system.health();
      result.syncing = health.isSyncing.isTrue;
      result.peers = health.peers.toNumber();

      // Get validator info if applicable
      if (node.type === 'validator') {
        const validators = await api.query.session.validators();
        const isValidator = validators.some(
          (v) => v.toString() === node.validatorAddress
        );
        result.isActiveValidator = isValidator;
      }

      result.status = result.syncing ? 'syncing' : 'online';
      onProgress(`  ✅ ${node.name}: Block #${result.blockHeight}, ${result.peers} peers\n`);
    } catch (err) {
      result.status = 'offline';
      result.error = err.message;
      onProgress(`  ❌ ${node.name}: ${err.message}\n`);
    }

    return result;
  }

  /**
   * Connect to node via WebSocket RPC
   */
  async connectToNode(node) {
    const wsUrl = `ws://${node.ip}:${node.wsPort || 9945}`;
    const cacheKey = wsUrl;

    if (this.apiConnections.has(cacheKey)) {
      const api = this.apiConnections.get(cacheKey);
      if (api.isConnected) {
        return api;
      } else {
        this.apiConnections.delete(cacheKey);
      }
    }

    const provider = new WsProvider(wsUrl, 5000);
    const api = await ApiPromise.create({ provider });

    this.apiConnections.set(cacheKey, api);

    // Clean up on disconnect
    provider.on('disconnected', () => {
      this.apiConnections.delete(cacheKey);
    });

    return api;
  }

  /**
   * Get logs from nodes
   */
  async getLogs(chains, options = {}, onProgress = () => {}) {
    const { since = '1h', filter, maxLines = 1000 } = options;

    const logs = {};

    const chainList = chains === 'all'
      ? ['flarechain', ...this.config.chains.pbcs.map(p => p.name.toLowerCase())]
      : Array.isArray(chains) ? chains : [chains];

    for (const chain of chainList) {
      onProgress(`📋 Fetching logs for ${chain}...\n`);

      const nodes = this.getChainNodes(chain);
      logs[chain] = [];

      for (const node of nodes) {
        try {
          const nodeLogs = await this.getNodeLogs(node, { since, filter, maxLines });
          logs[chain].push({
            node: node.name,
            lines: nodeLogs
          });
          onProgress(`  ✅ ${node.name}: ${nodeLogs.length} lines\n`);
        } catch (err) {
          onProgress(`  ⚠️  ${node.name}: ${err.message}\n`);
          logs[chain].push({
            node: node.name,
            error: err.message
          });
        }
      }
    }

    return logs;
  }

  /**
   * Get logs from individual node via SSH
   */
  async getNodeLogs(node, options = {}) {
    const { since = '1h', filter, maxLines = 1000 } = options;

    // This requires SSH connection - will integrate with SSHManager
    const logPath = node.logPath || '/var/log/substrate/substrate.log';

    // Build journalctl or tail command based on setup
    let command;
    if (node.useJournalctl) {
      command = `journalctl -u ${node.serviceName || 'substrate'} --since="${since}"`;
      if (filter) {
        command += ` | grep "${filter}"`;
      }
      command += ` | tail -n ${maxLines}`;
    } else {
      command = `tail -n ${maxLines} ${logPath}`;
      if (filter) {
        command = `grep "${filter}" ${logPath} | tail -n ${maxLines}`;
      }
    }

    // Note: This should use SSHManager.exec() in practice
    // For now, return placeholder
    return [
      `[${new Date().toISOString()}] Node started`,
      `[${new Date().toISOString()}] Syncing blocks...`,
      // Actual implementation will SSH and fetch real logs
    ];
  }

  /**
   * Get performance metrics
   */
  async getMetrics(chains, onProgress = () => {}) {
    const metrics = {};

    const chainList = chains === 'all'
      ? ['flarechain', ...this.config.chains.pbcs.map(p => p.name.toLowerCase())]
      : Array.isArray(chains) ? chains : [chains];

    for (const chain of chainList) {
      onProgress(`📊 Fetching metrics for ${chain}...\n`);

      const nodes = this.getChainNodes(chain);
      metrics[chain] = [];

      for (const node of nodes) {
        try {
          const nodeMetrics = await this.getNodeMetrics(node);
          metrics[chain].push({
            node: node.name,
            ...nodeMetrics
          });
          onProgress(`  ✅ ${node.name}: CPU ${nodeMetrics.cpu}%, RAM ${nodeMetrics.memory}%\n`);
        } catch (err) {
          onProgress(`  ⚠️  ${node.name}: ${err.message}\n`);
        }
      }
    }

    return metrics;
  }

  /**
   * Get metrics from individual node
   */
  async getNodeMetrics(node) {
    // Fetch from Prometheus endpoint if available
    if (node.metricsPort) {
      try {
        const url = `http://${node.ip}:${node.metricsPort}/metrics`;
        const response = await axios.get(url, { timeout: 5000 });
        return this.parsePrometheusMetrics(response.data);
      } catch (err) {
        // Fall back to SSH-based monitoring
      }
    }

    // Fallback: use SSH to get system metrics
    // This should integrate with SSHManager
    return {
      cpu: 0,
      memory: 0,
      disk: 0,
      network: {
        in: 0,
        out: 0
      }
    };
  }

  /**
   * Parse Prometheus metrics format
   */
  parsePrometheusMetrics(data) {
    const metrics = {
      cpu: 0,
      memory: 0,
      disk: 0,
      blocks: 0,
      peers: 0
    };

    const lines = data.split('\n');
    for (const line of lines) {
      if (line.startsWith('#')) continue;

      if (line.includes('substrate_cpu_usage_percentage')) {
        metrics.cpu = parseFloat(line.split(' ')[1]);
      } else if (line.includes('substrate_memory_usage_bytes')) {
        metrics.memory = parseFloat(line.split(' ')[1]);
      } else if (line.includes('substrate_block_height')) {
        metrics.blocks = parseInt(line.split(' ')[1]);
      } else if (line.includes('substrate_sub_libp2p_peers_count')) {
        metrics.peers = parseInt(line.split(' ')[1]);
      }
    }

    return metrics;
  }

  /**
   * Disconnect all API connections
   */
  async disconnectAll() {
    for (const [url, api] of this.apiConnections) {
      try {
        await api.disconnect();
      } catch (err) {
        console.error(`Error disconnecting from ${url}:`, err);
      }
    }
    this.apiConnections.clear();
  }

  // Helper methods
  getChainNodes(chainName) {
    const chain = chainName.toLowerCase();

    if (chain === 'flarechain') {
      return this.config.chains.flarechain.nodes || [];
    }

    const pbc = this.config.chains.pbcs.find(
      (p) => p.name.toLowerCase() === chain
    );

    return pbc?.nodes || [];
  }

  updateSummary(summary, status) {
    if (!status || !status.nodes) return;

    for (const node of status.nodes) {
      summary.total++;
      if (node.status === 'online') {
        summary.online++;
      } else if (node.status === 'syncing') {
        summary.syncing++;
      } else {
        summary.offline++;
      }
    }
  }

  getCachedStatus(chainName) {
    const cached = this.statusCache.get(chainName);
    if (!cached) return null;

    const age = Date.now() - cached.timestamp;
    if (age > this.cacheTTL) {
      this.statusCache.delete(chainName);
      return null;
    }

    return cached;
  }

  cacheStatus(chainName, status) {
    this.statusCache.set(chainName, status);
  }
}

module.exports = { NodeMonitor };
