#!/usr/bin/env node

/**
 * ËTRID AI-Powered Validator Monitor
 * Intelligent health analysis with automatic recommendations
 */

const { NodeSSH } = require('node-ssh');
const fs = require('fs');
const path = require('path');

// Load validator configuration
const VALIDATORS_CONFIG = require('../infrastructure/config/validator-ips.json');

// Configuration
const SSH_KEY_PATH = process.env.SSH_KEY_PATH || path.join(process.env.HOME, '.ssh', 'id_rsa');
const SSH_TIMEOUT = 30000;
const REPORTS_DIR = path.join(__dirname, 'reports');

// Health thresholds
const THRESHOLDS = {
  PEER_COUNT_MIN: 4,
  DISK_USAGE_MAX: 80,
  MEMORY_USAGE_MAX: 85,
  BLOCK_HEIGHT_TOLERANCE: 10,
};

class AIValidatorMonitor {
  constructor() {
    this.validators = VALIDATORS_CONFIG.validators.filter(v => v.accessible) || [];

    // Ensure reports directory exists
    if (!fs.existsSync(REPORTS_DIR)) {
      fs.mkdirSync(REPORTS_DIR, { recursive: true });
    }
  }

  /**
   * Calculate health score for a validator
   */
  calculateHealthScore(metrics) {
    let score = 100;
    const alerts = [];

    // Service status
    if (!metrics.isRunning) {
      score -= 50;
      alerts.push({ severity: 'critical', message: 'Service is not running' });
    }

    // Peer count
    if (metrics.peerCount !== null && metrics.peerCount < THRESHOLDS.PEER_COUNT_MIN) {
      score -= 20;
      alerts.push({ severity: 'warning', message: `Low peer count: ${metrics.peerCount}` });
    }

    // Disk usage
    if (metrics.diskUsage !== null && metrics.diskUsage > THRESHOLDS.DISK_USAGE_MAX) {
      score -= 15;
      alerts.push({ severity: 'warning', message: `High disk usage: ${metrics.diskUsage}%` });
    }

    // Memory usage
    if (metrics.memoryUsage !== null && metrics.memoryUsage > THRESHOLDS.MEMORY_USAGE_MAX) {
      score -= 10;
      alerts.push({ severity: 'warning', message: `High memory usage: ${metrics.memoryUsage}%` });
    }

    // Ensure score doesn't go below 0
    score = Math.max(0, score);

    return { score, alerts };
  }

  /**
   * Parse metrics from command outputs
   */
  parseMetrics(outputs) {
    const metrics = {
      isRunning: false,
      peerCount: null,
      blockHeight: null,
      diskUsage: null,
      memoryUsage: null,
      uptime: null,
    };

    // Service status
    if (outputs.service) {
      metrics.isRunning = outputs.service.includes('active') || outputs.service.includes('running');
    }

    // Peer count (extract from logs)
    if (outputs.peers) {
      const peerMatch = outputs.peers.match(/(\d+)\s*peer/i);
      if (peerMatch) {
        metrics.peerCount = parseInt(peerMatch[1]);
      }
    }

    // Block height (extract from logs)
    if (outputs.block) {
      const blockMatch = outputs.block.match(/#(\d+)/);
      if (blockMatch) {
        metrics.blockHeight = parseInt(blockMatch[1]);
      }
    }

    // Disk usage
    if (outputs.disk) {
      const diskMatch = outputs.disk.match(/(\d+)%/);
      if (diskMatch) {
        metrics.diskUsage = parseInt(diskMatch[1]);
      }
    }

    // Memory usage
    if (outputs.memory) {
      const parts = outputs.memory.trim().split(/\s+/);
      if (parts.length >= 3) {
        const total = parseFloat(parts[1]);
        const used = parseFloat(parts[2]);
        if (!isNaN(total) && !isNaN(used)) {
          metrics.memoryUsage = Math.round((used / total) * 100);
        }
      }
    }

    // Uptime
    if (outputs.uptime) {
      metrics.uptime = outputs.uptime.trim();
    }

    return metrics;
  }

  /**
   * Collect metrics from a validator
   */
  async collectMetrics(validator) {
    const ssh = new NodeSSH();

    try {
      await ssh.connect({
        host: validator.ip,
        username: validator.sshUser,
        privateKeyPath: SSH_KEY_PATH,
        timeout: SSH_TIMEOUT,
      });

      const commands = {
        service: 'systemctl status etrid-validator 2>/dev/null || systemctl status etrid-node 2>/dev/null || echo "not found"',
        peers: 'journalctl -u etrid-validator -n 50 2>/dev/null | grep -i "peer" | tail -3 || echo "no peer info"',
        block: 'journalctl -u etrid-validator -n 20 2>/dev/null | grep -E "#[0-9]+" | tail -1 || echo "no block info"',
        disk: 'df -h / | tail -1',
        memory: 'free -h | grep Mem',
        uptime: 'uptime',
      };

      const outputs = {};

      for (const [key, cmd] of Object.entries(commands)) {
        const result = await ssh.execCommand(cmd);
        outputs[key] = result.stdout || result.stderr || '';
      }

      const metrics = this.parseMetrics(outputs);
      const health = this.calculateHealthScore(metrics);

      return {
        validator,
        metrics,
        health: health.score,
        alerts: health.alerts,
        timestamp: new Date().toISOString(),
        accessible: true,
      };
    } catch (error) {
      return {
        validator,
        error: error.message,
        health: 0,
        alerts: [{ severity: 'critical', message: `Connection failed: ${error.message}` }],
        timestamp: new Date().toISOString(),
        accessible: false,
      };
    } finally {
      ssh.dispose();
    }
  }

  /**
   * Generate AI recommendations based on metrics
   */
  generateRecommendations(results) {
    const recommendations = [];

    // Critical issues
    const stoppedValidators = results.filter(r => r.metrics && !r.metrics.isRunning);
    if (stoppedValidators.length > 0) {
      recommendations.push({
        priority: 'high',
        category: 'Service',
        message: `Restart stopped validators: ${stoppedValidators.map(v => `#${v.validator.id}`).join(', ')}`,
      });
    }

    // Low peer count
    const lowPeerValidators = results.filter(r =>
      r.metrics && r.metrics.peerCount !== null && r.metrics.peerCount < THRESHOLDS.PEER_COUNT_MIN
    );
    if (lowPeerValidators.length > 0) {
      recommendations.push({
        priority: 'medium',
        category: 'Network',
        message: `Check network connectivity for validators with low peer count: ${lowPeerValidators.map(v => `#${v.validator.id}`).join(', ')}`,
      });
    }

    // High disk usage
    const highDiskValidators = results.filter(r =>
      r.metrics && r.metrics.diskUsage !== null && r.metrics.diskUsage > THRESHOLDS.DISK_USAGE_MAX
    );
    if (highDiskValidators.length > 0) {
      recommendations.push({
        priority: 'medium',
        category: 'Storage',
        message: `Clean up disk space on validators: ${highDiskValidators.map(v => `#${v.validator.id} (${v.metrics.diskUsage}%)`).join(', ')}`,
      });
    }

    // High memory usage
    const highMemValidators = results.filter(r =>
      r.metrics && r.metrics.memoryUsage !== null && r.metrics.memoryUsage > THRESHOLDS.MEMORY_USAGE_MAX
    );
    if (highMemValidators.length > 0) {
      recommendations.push({
        priority: 'low',
        category: 'Resources',
        message: `Monitor memory usage on validators: ${highMemValidators.map(v => `#${v.validator.id} (${v.metrics.memoryUsage}%)`).join(', ')}`,
      });
    }

    // Connection failures
    const failedConnections = results.filter(r => !r.accessible);
    if (failedConnections.length > 0) {
      recommendations.push({
        priority: 'high',
        category: 'Connectivity',
        message: `Check SSH access to validators: ${failedConnections.map(v => `#${v.validator.id}`).join(', ')}`,
      });
    }

    return recommendations;
  }

  /**
   * Monitor all validators
   */
  async monitor() {
    console.log('\n' + '='.repeat(80));
    console.log('🤖 AI-POWERED VALIDATOR MONITORING');
    console.log('='.repeat(80) + '\n');

    console.log(`⏳ Collecting metrics from ${this.validators.length} validators...\n`);

    const results = [];

    for (const validator of this.validators) {
      process.stdout.write(`📊 Checking #${validator.id} ${validator.name}...`);
      const result = await this.collectMetrics(validator);
      results.push(result);

      const statusIcon = result.health >= 80 ? '🟢' : result.health >= 50 ? '🟡' : '🔴';
      console.log(` ${statusIcon} Health: ${result.health}/100`);
    }

    // Calculate network summary
    const avgHealth = Math.round(results.reduce((sum, r) => sum + r.health, 0) / results.length);
    const running = results.filter(r => r.metrics && r.metrics.isRunning).length;
    const avgPeers = results
      .filter(r => r.metrics && r.metrics.peerCount !== null)
      .reduce((sum, r) => sum + r.metrics.peerCount, 0) /
      results.filter(r => r.metrics && r.metrics.peerCount !== null).length;

    // Generate recommendations
    const recommendations = this.generateRecommendations(results);

    // Count alerts by severity
    const criticalAlerts = results.reduce((sum, r) =>
      sum + r.alerts.filter(a => a.severity === 'critical').length, 0
    );
    const warningAlerts = results.reduce((sum, r) =>
      sum + r.alerts.filter(a => a.severity === 'warning').length, 0
    );

    // Print report
    console.log('\n' + '='.repeat(80));
    console.log('📊 NETWORK SUMMARY');
    console.log('='.repeat(80));

    const healthIcon = avgHealth >= 80 ? '🟢 HEALTHY' : avgHealth >= 50 ? '🟡 WARNING' : '🔴 CRITICAL';
    console.log(`   Overall Health:      ${healthIcon} (Score: ${avgHealth}/100)`);
    console.log(`   Total Validators:    ${this.validators.length}`);
    console.log(`   Running Validators:  ${running}/${this.validators.length}`);
    console.log(`   Average Peer Count:  ${avgPeers.toFixed(1)}`);

    if (criticalAlerts > 0 || warningAlerts > 0) {
      console.log('\n' + '='.repeat(80));
      console.log('🚨 ALERTS');
      console.log('='.repeat(80));
      if (criticalAlerts > 0) {
        console.log(`   🔴 CRITICAL: ${criticalAlerts} alert(s)`);
      }
      if (warningAlerts > 0) {
        console.log(`   🟡 WARNING: ${warningAlerts} alert(s)`);
      }
    }

    if (recommendations.length > 0) {
      console.log('\n' + '='.repeat(80));
      console.log('💡 AI RECOMMENDATIONS');
      console.log('='.repeat(80));
      recommendations.forEach((rec, i) => {
        const icon = rec.priority === 'high' ? '🔴' : rec.priority === 'medium' ? '🟡' : '🔵';
        console.log(`   ${i + 1}. ${icon} [${rec.category}] ${rec.message}`);
      });
    }

    // Detailed validator info
    console.log('\n' + '='.repeat(80));
    console.log('📋 VALIDATOR DETAILS');
    console.log('='.repeat(80) + '\n');

    for (const result of results) {
      const healthIcon = result.health >= 80 ? '🟢' : result.health >= 50 ? '🟡' : '🔴';

      console.log(`${healthIcon} Validator #${result.validator.id}: ${result.validator.name} (Health: ${result.health}/100)`);

      if (result.metrics) {
        const status = result.metrics.isRunning ? '🟢 Running' : '🔴 Stopped';
        const peers = result.metrics.peerCount !== null ? result.metrics.peerCount : 'N/A';
        const block = result.metrics.blockHeight !== null ? `#${result.metrics.blockHeight}` : 'N/A';
        const disk = result.metrics.diskUsage !== null ? `${result.metrics.diskUsage}%` : 'N/A';
        const mem = result.metrics.memoryUsage !== null ? `${result.metrics.memoryUsage}%` : 'N/A';

        console.log(`   Status: ${status} | Peers: ${peers} | Block: ${block}`);
        console.log(`   Disk: ${disk} | Memory: ${mem}`);
      }

      if (result.alerts.length > 0) {
        result.alerts.forEach(alert => {
          const icon = alert.severity === 'critical' ? '🔴' : '🟡';
          console.log(`   ${icon} ${alert.message}`);
        });
      }

      console.log('');
    }

    console.log('='.repeat(80) + '\n');

    // Save report
    const report = {
      timestamp: new Date().toISOString(),
      summary: {
        totalValidators: this.validators.length,
        runningValidators: running,
        averageHealth: avgHealth,
        averagePeerCount: avgPeers,
        criticalAlerts,
        warningAlerts,
      },
      recommendations,
      validators: results,
    };

    const reportPath = path.join(REPORTS_DIR, `validator-report-${Date.now()}.json`);
    fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
    console.log(`💾 Report saved to: ${reportPath}\n`);

    return report;
  }

  /**
   * Continuous monitoring
   */
  async continuousMonitor(intervalMinutes = 10) {
    console.log(`\n🔄 Starting continuous monitoring (every ${intervalMinutes} minutes)...`);
    console.log('Press Ctrl+C to stop\n');

    while (true) {
      await this.monitor();
      console.log(`⏳ Waiting ${intervalMinutes} minutes until next check...\n`);
      await new Promise(resolve => setTimeout(resolve, intervalMinutes * 60 * 1000));
    }
  }
}

// CLI Interface
async function main() {
  const args = process.argv.slice(2);
  const command = args[0];
  const monitor = new AIValidatorMonitor();

  try {
    switch (command) {
      case 'monitor':
        await monitor.monitor();
        break;

      case 'continuous':
        const interval = parseInt(args[1]) || 10;
        await monitor.continuousMonitor(interval);
        break;

      default:
        console.log('\n🤖 ËTRID AI Validator Monitor\n');
        console.log('Usage:');
        console.log('  node ai-validator-monitor.js monitor                    - Run monitoring once');
        console.log('  node ai-validator-monitor.js continuous [minutes]       - Continuous monitoring\n');
        console.log('Environment Variables:');
        console.log('  SSH_KEY_PATH - Path to SSH private key (default: ~/.ssh/id_rsa)\n');
        process.exit(1);
    }
  } catch (error) {
    console.error(`\n❌ Error: ${error.message}\n`);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}

module.exports = AIValidatorMonitor;
