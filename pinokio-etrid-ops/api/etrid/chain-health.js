/**
 * Chain Health & Maintenance Automation
 * Automated health checks, updates, and maintenance for Etrid chains
 */

class ChainHealth {
  constructor(config) {
    this.config = config;
    this.healthThresholds = {
      blockTimeDrift: 5000, // ms
      minPeers: 3,
      maxBlockAge: 300000, // 5 minutes
      minDiskSpace: 10 * 1024 * 1024 * 1024, // 10GB
      maxCpuUsage: 90, // percent
      maxMemoryUsage: 90 // percent
    };
  }

  /**
   * Run comprehensive health check
   */
  async check(chains, options = {}) {
    const { onProgress = () => {}, autoFix = false } = options;

    const results = {
      timestamp: Date.now(),
      chains: {},
      issues: [],
      criticalIssues: []
    };

    const chainList = chains === 'all'
      ? ['flarechain', ...this.config.chains.pbcs.map(p => p.name.toLowerCase())]
      : Array.isArray(chains) ? chains : [chains];

    for (const chain of chainList) {
      onProgress(`🏥 Health check: ${chain}...\n`);

      const chainHealth = await this.checkChain(chain, onProgress);
      results.chains[chain] = chainHealth;

      // Collect issues
      for (const issue of chainHealth.issues) {
        if (issue.severity === 'critical') {
          results.criticalIssues.push({ chain, ...issue });
        } else {
          results.issues.push({ chain, ...issue });
        }
      }

      // Auto-fix if requested
      if (autoFix && chainHealth.issues.length > 0) {
        onProgress(`🔧 Attempting auto-fix for ${chain}...\n`);
        await this.autoFix(chain, chainHealth.issues, onProgress);
      }
    }

    return results;
  }

  /**
   * Check health of specific chain
   */
  async checkChain(chainName, onProgress = () => {}) {
    const health = {
      chain: chainName,
      healthy: true,
      issues: [],
      nodes: []
    };

    const nodes = this.getChainNodes(chainName);

    for (const node of nodes) {
      onProgress(`  Checking ${node.name}...\n`);

      const nodeHealth = await this.checkNode(node);
      health.nodes.push(nodeHealth);

      if (!nodeHealth.healthy) {
        health.healthy = false;
        health.issues.push(...nodeHealth.issues);
      }
    }

    // Check consensus
    const consensusIssue = this.checkConsensus(health.nodes);
    if (consensusIssue) {
      health.healthy = false;
      health.issues.push(consensusIssue);
    }

    return health;
  }

  /**
   * Check health of individual node
   */
  async checkNode(node) {
    const health = {
      node: node.name,
      healthy: true,
      issues: [],
      checks: {}
    };

    try {
      // Check 1: Node is reachable
      health.checks.reachable = await this.checkReachable(node);
      if (!health.checks.reachable.passed) {
        health.healthy = false;
        health.issues.push({
          check: 'reachable',
          severity: 'critical',
          message: 'Node is unreachable',
          details: health.checks.reachable.error
        });
        return health; // Skip other checks if unreachable
      }

      // Check 2: Block height is recent
      health.checks.blockHeight = await this.checkBlockHeight(node);
      if (!health.checks.blockHeight.passed) {
        health.healthy = false;
        health.issues.push({
          check: 'blockHeight',
          severity: 'critical',
          message: 'Node is not producing/syncing blocks',
          details: health.checks.blockHeight
        });
      }

      // Check 3: Peer count
      health.checks.peers = await this.checkPeers(node);
      if (!health.checks.peers.passed) {
        health.healthy = false;
        health.issues.push({
          check: 'peers',
          severity: 'warning',
          message: 'Low peer count',
          details: health.checks.peers
        });
      }

      // Check 4: System resources
      health.checks.resources = await this.checkResources(node);
      if (!health.checks.resources.passed) {
        health.healthy = false;
        health.issues.push({
          check: 'resources',
          severity: 'warning',
          message: 'System resource issues',
          details: health.checks.resources
        });
      }

      // Check 5: Disk space
      health.checks.diskSpace = await this.checkDiskSpace(node);
      if (!health.checks.diskSpace.passed) {
        health.healthy = false;
        health.issues.push({
          check: 'diskSpace',
          severity: health.checks.diskSpace.critical ? 'critical' : 'warning',
          message: 'Low disk space',
          details: health.checks.diskSpace
        });
      }

      // Check 6: Service status
      health.checks.service = await this.checkService(node);
      if (!health.checks.service.passed) {
        health.healthy = false;
        health.issues.push({
          check: 'service',
          severity: 'critical',
          message: 'Service not running properly',
          details: health.checks.service
        });
      }

    } catch (err) {
      health.healthy = false;
      health.issues.push({
        check: 'general',
        severity: 'critical',
        message: `Health check failed: ${err.message}`
      });
    }

    return health;
  }

  /**
   * Individual health checks
   */

  async checkReachable(node) {
    // Try to connect to RPC
    try {
      // Implementation would use NodeMonitor
      return { passed: true };
    } catch (err) {
      return { passed: false, error: err.message };
    }
  }

  async checkBlockHeight(node) {
    // Check if block is recent
    try {
      // Get current block and timestamp
      const blockAge = 0; // Would fetch from NodeMonitor

      if (blockAge > this.healthThresholds.maxBlockAge) {
        return {
          passed: false,
          blockAge,
          threshold: this.healthThresholds.maxBlockAge
        };
      }

      return { passed: true, blockAge };
    } catch (err) {
      return { passed: false, error: err.message };
    }
  }

  async checkPeers(node) {
    try {
      const peers = 0; // Would fetch from NodeMonitor

      if (peers < this.healthThresholds.minPeers) {
        return {
          passed: false,
          peers,
          threshold: this.healthThresholds.minPeers
        };
      }

      return { passed: true, peers };
    } catch (err) {
      return { passed: false, error: err.message };
    }
  }

  async checkResources(node) {
    try {
      // Would fetch from NodeMonitor metrics
      const cpu = 0;
      const memory = 0;

      const issues = [];

      if (cpu > this.healthThresholds.maxCpuUsage) {
        issues.push(`High CPU: ${cpu}%`);
      }

      if (memory > this.healthThresholds.maxMemoryUsage) {
        issues.push(`High memory: ${memory}%`);
      }

      return {
        passed: issues.length === 0,
        cpu,
        memory,
        issues
      };
    } catch (err) {
      return { passed: false, error: err.message };
    }
  }

  async checkDiskSpace(node) {
    try {
      // Would SSH to node and check df
      const available = 100 * 1024 * 1024 * 1024; // placeholder

      const critical = available < this.healthThresholds.minDiskSpace;

      return {
        passed: !critical,
        critical,
        available,
        threshold: this.healthThresholds.minDiskSpace
      };
    } catch (err) {
      return { passed: false, error: err.message };
    }
  }

  async checkService(node) {
    try {
      // Would SSH and check systemctl status
      const running = true; // placeholder

      return { passed: running, running };
    } catch (err) {
      return { passed: false, error: err.message };
    }
  }

  checkConsensus(nodes) {
    // Check if all nodes agree on block height
    const blockHeights = nodes
      .filter(n => n.healthy)
      .map(n => n.checks.blockHeight?.blockAge)
      .filter(h => h !== undefined);

    if (blockHeights.length < 2) return null;

    const max = Math.max(...blockHeights);
    const min = Math.min(...blockHeights);
    const drift = max - min;

    if (drift > this.healthThresholds.blockTimeDrift) {
      return {
        check: 'consensus',
        severity: 'critical',
        message: 'Block height drift detected',
        details: { drift, blockHeights }
      };
    }

    return null;
  }

  /**
   * Auto-fix issues
   */
  async autoFix(chain, issues, onProgress = () => {}) {
    const fixes = [];

    for (const issue of issues) {
      onProgress(`  Attempting fix: ${issue.message}...\n`);

      try {
        const fix = await this.applyFix(chain, issue);
        fixes.push(fix);
        onProgress(`  ✅ Fixed: ${issue.message}\n`);
      } catch (err) {
        onProgress(`  ❌ Fix failed: ${err.message}\n`);
        fixes.push({
          issue,
          success: false,
          error: err.message
        });
      }
    }

    return fixes;
  }

  async applyFix(chain, issue) {
    // Apply fixes based on issue type
    switch (issue.check) {
      case 'service':
        // Restart service
        return await this.restartService(chain, issue.node);

      case 'peers':
        // Add bootstrap nodes
        return await this.addBootstrapNodes(chain, issue.node);

      case 'diskSpace':
        // Clean old blocks/logs
        return await this.cleanDiskSpace(chain, issue.node);

      default:
        throw new Error(`No auto-fix available for: ${issue.check}`);
    }
  }

  async restartService(chain, nodeName) {
    // Would use SSHManager to restart
    return { success: true, action: 'restart' };
  }

  async addBootstrapNodes(chain, nodeName) {
    // Would add bootstrap nodes to config
    return { success: true, action: 'bootstrap' };
  }

  async cleanDiskSpace(chain, nodeName) {
    // Would prune old data
    return { success: true, action: 'prune' };
  }

  /**
   * Create update plan for chain updates
   */
  async createUpdatePlan(chains, version) {
    const plan = {
      version,
      chains: [],
      steps: [],
      estimatedTime: 0
    };

    const chainList = chains === 'all'
      ? ['flarechain', ...this.config.chains.pbcs.map(p => p.name.toLowerCase())]
      : Array.isArray(chains) ? chains : [chains];

    for (const chain of chainList) {
      const nodes = this.getChainNodes(chain);
      const chainPlan = {
        chain,
        nodes: nodes.map(n => n.name),
        strategy: this.getUpdateStrategy(chain, nodes)
      };

      plan.chains.push(chainPlan);
      plan.steps.push(...this.generateUpdateSteps(chainPlan));
    }

    // Calculate estimated time
    plan.estimatedTime = plan.steps.length * 5 * 60000; // 5 min per step

    return plan;
  }

  getUpdateStrategy(chain, nodes) {
    // Determine update strategy based on node type and count
    if (chain === 'flarechain' && nodes.length > 3) {
      return 'rolling'; // Rolling updates for validators
    }
    return 'sequential'; // Sequential for smaller setups
  }

  generateUpdateSteps(chainPlan) {
    const steps = [];

    for (const node of chainPlan.nodes) {
      steps.push(
        { action: 'backup', node },
        { action: 'stop', node },
        { action: 'update', node },
        { action: 'start', node },
        { action: 'verify', node }
      );

      if (chainPlan.strategy === 'rolling') {
        steps.push({ action: 'wait', duration: '5m' });
      }
    }

    return steps;
  }

  /**
   * Execute update plan
   */
  async executeUpdate(plan, onProgress = () => {}) {
    const results = [];

    for (const step of plan.steps) {
      onProgress(`⚡ ${step.action}: ${step.node || step.duration || ''}\n`);

      try {
        const result = await this.executeStep(step);
        results.push({ step, success: true, result });
        onProgress(`  ✅ Complete\n`);
      } catch (err) {
        onProgress(`  ❌ Failed: ${err.message}\n`);
        results.push({ step, success: false, error: err.message });

        // Stop on critical failures
        if (['stop', 'start'].includes(step.action)) {
          throw new Error(`Critical step failed: ${step.action} on ${step.node}`);
        }
      }
    }

    return results;
  }

  async executeStep(step) {
    // Execute individual update step
    // Would integrate with SSHManager
    switch (step.action) {
      case 'backup':
        return await this.backupNode(step.node);
      case 'stop':
        return await this.stopNode(step.node);
      case 'update':
        return await this.updateNode(step.node);
      case 'start':
        return await this.startNode(step.node);
      case 'verify':
        return await this.verifyNode(step.node);
      case 'wait':
        return await this.wait(step.duration);
      default:
        throw new Error(`Unknown action: ${step.action}`);
    }
  }

  async backupNode(nodeName) {
    // Backup node data
    return { success: true };
  }

  async stopNode(nodeName) {
    // Stop node service
    return { success: true };
  }

  async updateNode(nodeName) {
    // Update node binary
    return { success: true };
  }

  async startNode(nodeName) {
    // Start node service
    return { success: true };
  }

  async verifyNode(nodeName) {
    // Verify node is running correctly
    return { success: true };
  }

  async wait(duration) {
    // Parse duration and wait
    const ms = this.parseDuration(duration);
    await new Promise(resolve => setTimeout(resolve, ms));
    return { success: true };
  }

  parseDuration(duration) {
    const match = duration.match(/^(\d+)([smh])$/);
    if (!match) return 0;

    const value = parseInt(match[1]);
    const unit = match[2];

    const multipliers = { s: 1000, m: 60000, h: 3600000 };
    return value * multipliers[unit];
  }

  /**
   * Summarize health check results
   */
  summarize(results) {
    let summary = '\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n';
    summary += '       HEALTH CHECK SUMMARY\n';
    summary += '━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n';

    const totalChains = Object.keys(results.chains).length;
    const healthyChains = Object.values(results.chains).filter(c => c.healthy).length;

    summary += `Chains: ${healthyChains}/${totalChains} healthy\n`;
    summary += `Issues: ${results.issues.length} warnings, ${results.criticalIssues.length} critical\n\n`;

    if (results.criticalIssues.length > 0) {
      summary += '❌ CRITICAL ISSUES:\n';
      for (const issue of results.criticalIssues) {
        summary += `  • ${issue.chain}: ${issue.message}\n`;
      }
      summary += '\n';
    }

    if (results.issues.length > 0 && results.issues.length <= 10) {
      summary += '⚠️  WARNINGS:\n';
      for (const issue of results.issues) {
        summary += `  • ${issue.chain}: ${issue.message}\n`;
      }
    }

    summary += '\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n';

    return summary;
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
}

module.exports = { ChainHealth };
