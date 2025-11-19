/**
 * Finality API Client
 * Connects to Gizzi VM's ASF Finality Dashboard
 * http://100.96.84.69:5000/api/status
 */

const axios = require('axios');

class FinalityClient {
  constructor(config = {}) {
    this.gizziUrl = config.gizziUrl || 'http://100.96.84.69:5000/api/status';
    this.timeout = config.timeout || 5000;
    this.cache = null;
    this.cacheExpiry = 0;
    this.cacheDuration = 10000; // 10 seconds (matching Gizzi's update rate)
  }

  /**
   * Get current finality metrics from Gizzi API
   */
  async getMetrics() {
    // Return cached data if still valid
    if (this.cache && Date.now() < this.cacheExpiry) {
      return this.cache;
    }

    try {
      const response = await axios.get(this.gizziUrl, {
        timeout: this.timeout,
        headers: {
          'User-Agent': 'Etrid-Ops-Center/1.0'
        }
      });

      const data = response.data;

      // Transform to consistent format
      const metrics = {
        finalityTime: parseFloat(data.finality_time) || 0,
        bestBlock: parseInt(data.best_block) || 0,
        finalizedBlock: parseInt(data.finalized_block) || 0,
        validatorCount: parseInt(data.validator_count) || 21,
        blockTime: parseFloat(data.block_time) || 5.0,
        networkTps: parseFloat(data.network_tps) || 0,
        timestamp: data.timestamp || Date.now(),
        isHealthy: this.assessHealth(data),
        status: 'online'
      };

      // Cache the result
      this.cache = metrics;
      this.cacheExpiry = Date.now() + this.cacheDuration;

      return metrics;

    } catch (err) {
      console.error('Failed to fetch finality metrics:', err.message);

      // Return last cached data if available
      if (this.cache) {
        return { ...this.cache, status: 'stale' };
      }

      // Return offline status
      return {
        finalityTime: 0,
        bestBlock: 0,
        finalizedBlock: 0,
        validatorCount: 0,
        blockTime: 0,
        networkTps: 0,
        timestamp: Date.now(),
        isHealthy: false,
        status: 'offline',
        error: err.message
      };
    }
  }

  /**
   * Assess network health based on finality metrics
   */
  assessHealth(data) {
    const finalityTime = parseFloat(data.finality_time) || 0;
    const blockTime = parseFloat(data.block_time) || 0;

    // Health criteria
    const finalityHealthy = finalityTime > 0 && finalityTime < 20; // Target: <20s
    const blockTimeHealthy = blockTime > 0 && blockTime < 7; // Target: ~5s, allow up to 7s
    const validatorsHealthy = parseInt(data.validator_count) >= 15; // At least 15/21 validators

    return finalityHealthy && blockTimeHealthy && validatorsHealthy;
  }

  /**
   * Get finality status with color coding
   */
  getFinalityStatus(finalityTime) {
    if (finalityTime === 0) return { status: 'unknown', color: 'gray' };
    if (finalityTime < 15) return { status: 'excellent', color: 'green' };
    if (finalityTime < 20) return { status: 'good', color: 'yellow' };
    return { status: 'slow', color: 'red' };
  }

  /**
   * Calculate finality lag (blocks behind)
   */
  calculateFinalityLag(bestBlock, finalizedBlock) {
    return Math.max(0, bestBlock - finalizedBlock);
  }

  /**
   * Get historical finality data (simulated for now)
   * In production, this would query a time-series database
   */
  async getHistoricalMetrics(hours = 24) {
    // TODO: Implement actual historical data storage
    // For now, return simulated data
    const dataPoints = [];
    const now = Date.now();

    for (let i = hours; i >= 0; i--) {
      dataPoints.push({
        timestamp: now - (i * 60 * 60 * 1000),
        finalityTime: 14 + Math.random() * 4, // 14-18s
        blockTime: 4.5 + Math.random() * 1, // 4.5-5.5s
        tps: 30 + Math.random() * 30 // 30-60 TPS
      });
    }

    return dataPoints;
  }

  /**
   * Stream finality metrics (for WebSocket)
   */
  async *streamMetrics(interval = 10000) {
    while (true) {
      yield await this.getMetrics();
      await new Promise(resolve => setTimeout(resolve, interval));
    }
  }

  /**
   * Check if finality dashboard is reachable
   */
  async healthCheck() {
    try {
      const response = await axios.get(this.gizziUrl, {
        timeout: 2000
      });
      return {
        reachable: true,
        responseTime: response.headers['x-response-time'] || 'N/A',
        timestamp: Date.now()
      };
    } catch (err) {
      return {
        reachable: false,
        error: err.message,
        timestamp: Date.now()
      };
    }
  }
}

module.exports = { FinalityClient };
