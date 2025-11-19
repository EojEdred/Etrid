/**
 * Network Aggregator
 * Collects and aggregates data from all validators in the network
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');

class NetworkAggregator {
  constructor(database) {
    this.database = database;
    this.cache = new Map();
    this.cacheDuration = 30000; // 30 seconds
  }

  /**
   * Get all validators with real-time status
   */
  async getAllValidators() {
    const cacheKey = 'all_validators';
    const cached = this.getFromCache(cacheKey);
    if (cached) return cached;

    try {
      // Get all validators from database (across all users)
      const validators = await this.database.getAllValidators();

      // Fetch status for each validator in parallel
      const validatorStatuses = await Promise.all(
        validators.map(v => this.getValidatorStatus(v))
      );

      this.setCache(cacheKey, validatorStatuses);
      return validatorStatuses;

    } catch (err) {
      console.error('Failed to get all validators:', err);
      return [];
    }
  }

  /**
   * Get status for a single validator
   */
  async getValidatorStatus(validator) {
    try {
      const config = JSON.parse(validator.node_config);
      const provider = new WsProvider(config.rpcEndpoint, 3000);
      const api = await ApiPromise.create({ provider });

      const [header, health, peers, syncState, nodeVersion] = await Promise.all([
        api.rpc.chain.getHeader(),
        api.rpc.system.health(),
        api.rpc.system.peers(),
        api.rpc.system.syncState(),
        api.rpc.system.version()
      ]);

      await api.disconnect();

      return {
        id: validator.id,
        nodeName: validator.node_name,
        chain: validator.chain,
        status: 'online',
        blockHeight: header.number.toNumber(),
        peers: peers.length,
        isSyncing: syncState.currentBlock.toNumber() < syncState.highestBlock.toNumber(),
        version: nodeVersion.toString(),
        health: {
          peers: health.peers.toNumber(),
          isSyncing: health.isSyncing.valueOf(),
          shouldHavePeers: health.shouldHavePeers.valueOf()
        },
        rpcEndpoint: config.rpcEndpoint,
        location: config.location || this.estimateLocation(config.ssh?.host),
        lastUpdated: Date.now()
      };

    } catch (err) {
      return {
        id: validator.id,
        nodeName: validator.node_name,
        chain: validator.chain,
        status: 'offline',
        blockHeight: 0,
        peers: 0,
        isSyncing: false,
        version: 'unknown',
        error: err.message,
        lastUpdated: Date.now()
      };
    }
  }

  /**
   * Get network-wide statistics
   */
  async getNetworkStats() {
    const cacheKey = 'network_stats';
    const cached = this.getFromCache(cacheKey);
    if (cached) return cached;

    try {
      const validators = await this.getAllValidators();

      const stats = {
        total: validators.length,
        online: validators.filter(v => v.status === 'online').length,
        offline: validators.filter(v => v.status === 'offline').length,
        syncing: validators.filter(v => v.isSyncing).length,

        // Block stats
        highestBlock: Math.max(...validators.map(v => v.blockHeight || 0)),
        lowestBlock: Math.min(...validators.filter(v => v.blockHeight > 0).map(v => v.blockHeight) || [0]),
        avgBlockHeight: this.average(validators.map(v => v.blockHeight || 0)),

        // Peer stats
        totalPeers: validators.reduce((sum, v) => sum + (v.peers || 0), 0),
        avgPeers: this.average(validators.map(v => v.peers || 0)),

        // Version distribution
        versions: this.groupBy(validators, 'version'),

        // Geographic distribution
        locations: this.getLocationDistribution(validators),

        timestamp: Date.now()
      };

      this.setCache(cacheKey, stats);
      return stats;

    } catch (err) {
      console.error('Failed to get network stats:', err);
      return null;
    }
  }

  /**
   * Get validator leaderboard (for Phase 3)
   */
  async getLeaderboard(metric = 'blocks', limit = 10) {
    const validators = await this.getAllValidators();

    // Sort by specified metric
    const sorted = validators.sort((a, b) => {
      switch (metric) {
        case 'blocks':
          return (b.blockHeight || 0) - (a.blockHeight || 0);
        case 'peers':
          return (b.peers || 0) - (a.peers || 0);
        case 'uptime':
          return (b.uptime || 0) - (a.uptime || 0);
        default:
          return 0;
      }
    });

    return sorted.slice(0, limit).map((v, index) => ({
      rank: index + 1,
      nodeName: v.nodeName,
      value: v[metric] || 0,
      status: v.status
    }));
  }

  /**
   * Compare validators
   */
  async compareValidators(validatorIds) {
    const validators = await this.getAllValidators();
    const selected = validators.filter(v => validatorIds.includes(v.id));

    return {
      validators: selected,
      comparison: {
        blocks: this.compareMetric(selected, 'blockHeight'),
        peers: this.compareMetric(selected, 'peers'),
        versions: selected.map(v => v.version)
      }
    };
  }

  /**
   * Detect network issues
   */
  async detectNetworkIssues() {
    const validators = await this.getAllValidators();
    const stats = await this.getNetworkStats();
    const issues = [];

    // Check for offline validators
    const offlineCount = validators.filter(v => v.status === 'offline').length;
    if (offlineCount > 3) {
      issues.push({
        severity: 'high',
        type: 'offline_validators',
        message: `${offlineCount} validators are offline`,
        affectedValidators: validators.filter(v => v.status === 'offline').map(v => v.nodeName)
      });
    }

    // Check for block height discrepancies
    const blockHeights = validators.filter(v => v.blockHeight > 0).map(v => v.blockHeight);
    const blockDiff = Math.max(...blockHeights) - Math.min(...blockHeights);
    if (blockDiff > 10) {
      issues.push({
        severity: 'medium',
        type: 'sync_issues',
        message: `Block height variance: ${blockDiff} blocks`,
        details: `Some validators are behind by ${blockDiff} blocks`
      });
    }

    // Check for low peer counts
    const lowPeerValidators = validators.filter(v => v.peers > 0 && v.peers < 3);
    if (lowPeerValidators.length > 0) {
      issues.push({
        severity: 'medium',
        type: 'connectivity',
        message: `${lowPeerValidators.length} validators have low peer counts`,
        affectedValidators: lowPeerValidators.map(v => v.nodeName)
      });
    }

    // Check for version fragmentation
    const versionCounts = Object.values(stats.versions);
    if (versionCounts.length > 2) {
      issues.push({
        severity: 'low',
        type: 'version_fragmentation',
        message: `Network is running ${versionCounts.length} different versions`,
        versions: Object.keys(stats.versions)
      });
    }

    return {
      issues,
      healthy: issues.filter(i => i.severity === 'high').length === 0,
      timestamp: Date.now()
    };
  }

  /**
   * Helper: Estimate location from IP
   */
  estimateLocation(ip) {
    if (!ip) return { lat: 0, lon: 0, region: 'Unknown' };

    // Tailscale IPs (100.x.x.x) - return configured locations
    // In production, this would query a GeoIP database or be configured per-validator

    // Default to datacenter regions based on common patterns
    const defaultLocations = [
      { lat: 40.7128, lon: -74.0060, region: 'US-East' },
      { lat: 37.7749, lon: -122.4194, region: 'US-West' },
      { lat: 51.5074, lon: -0.1278, region: 'EU-West' },
      { lat: 35.6762, lon: 139.6503, region: 'Asia-Pacific' }
    ];

    // Pseudo-random but consistent location based on IP
    const hash = ip.split('.').reduce((sum, byte) => sum + parseInt(byte), 0);
    return defaultLocations[hash % defaultLocations.length];
  }

  /**
   * Helper: Group validators by property
   */
  groupBy(array, property) {
    return array.reduce((groups, item) => {
      const key = item[property] || 'unknown';
      groups[key] = (groups[key] || 0) + 1;
      return groups;
    }, {});
  }

  /**
   * Helper: Get location distribution
   */
  getLocationDistribution(validators) {
    return validators.reduce((dist, v) => {
      const region = v.location?.region || 'Unknown';
      dist[region] = (dist[region] || 0) + 1;
      return dist;
    }, {});
  }

  /**
   * Helper: Calculate average
   */
  average(numbers) {
    const valid = numbers.filter(n => n > 0);
    return valid.length > 0 ? valid.reduce((sum, n) => sum + n, 0) / valid.length : 0;
  }

  /**
   * Helper: Compare metric across validators
   */
  compareMetric(validators, metric) {
    const values = validators.map(v => v[metric] || 0);
    return {
      min: Math.min(...values),
      max: Math.max(...values),
      avg: this.average(values),
      diff: Math.max(...values) - Math.min(...values)
    };
  }

  /**
   * Cache helpers
   */
  getFromCache(key) {
    const cached = this.cache.get(key);
    if (cached && Date.now() < cached.expiry) {
      return cached.data;
    }
    return null;
  }

  setCache(key, data) {
    this.cache.set(key, {
      data,
      expiry: Date.now() + this.cacheDuration
    });
  }
}

module.exports = { NetworkAggregator };
