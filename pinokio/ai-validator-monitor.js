#!/usr/bin/env node

/**
 * AI-Powered Validator Monitoring for ÉTRID
 * Uses AI to analyze validator health and provide recommendations
 */

const ValidatorCLI = require('./validator-cli');
const fs = require('fs').promises;
const path = require('path');

class AIValidatorMonitor {
  constructor() {
    this.cli = new ValidatorCLI();
    this.alertThresholds = {
      minPeers: 3,
      maxBlockLag: 10,
      minDiskSpace: 10, // GB
      maxMemoryUsage: 90, // percentage
    };
  }

  async initialize() {
    await this.cli.loadValidators();
  }

  parseMetrics(status) {
    const metrics = {
      validator: status.validator,
      number: status.number,
      isRunning: status.systemd?.includes('active (running)'),
      alerts: [],
      recommendations: [],
      healthScore: 100,
    };

    // Parse peer count
    const peerMatch = status.peers?.match(/(\d+)\s+peers/i);
    if (peerMatch) {
      metrics.peerCount = parseInt(peerMatch[1]);
      if (metrics.peerCount < this.alertThresholds.minPeers) {
        metrics.alerts.push(`⚠️  LOW PEER COUNT: ${metrics.peerCount} peers (minimum recommended: ${this.alertThresholds.minPeers})`);
        metrics.recommendations.push('Check network connectivity and firewall rules for port 30333');
        metrics.healthScore -= 20;
      }
    } else {
      metrics.alerts.push('❌ Unable to determine peer count');
      metrics.healthScore -= 10;
    }

    // Parse block height
    const blockMatch = status.blockHeight?.match(/#(\d+)/);
    if (blockMatch) {
      metrics.blockHeight = parseInt(blockMatch[1]);
    }

    // Parse disk usage
    const diskMatch = status.diskUsage?.match(/(\d+)%/);
    if (diskMatch) {
      metrics.diskUsage = parseInt(diskMatch[1]);
      if (metrics.diskUsage > 85) {
        metrics.alerts.push(`⚠️  HIGH DISK USAGE: ${metrics.diskUsage}%`);
        metrics.recommendations.push('Consider pruning old blocks or increasing disk space');
        metrics.healthScore -= 15;
      }
    }

    // Parse memory usage
    const memMatch = status.memoryUsage?.match(/(\d+\.\d+)Gi?\s+(\d+\.\d+)Gi?/);
    if (memMatch) {
      const used = parseFloat(memMatch[1]);
      const total = parseFloat(memMatch[2]);
      metrics.memoryUsagePercent = ((used / total) * 100).toFixed(1);

      if (metrics.memoryUsagePercent > this.alertThresholds.maxMemoryUsage) {
        metrics.alerts.push(`⚠️  HIGH MEMORY USAGE: ${metrics.memoryUsagePercent}%`);
        metrics.recommendations.push('Monitor for memory leaks or consider upgrading RAM');
        metrics.healthScore -= 10;
      }
    }

    // Check if service is running
    if (!metrics.isRunning) {
      metrics.alerts.push('🔴 CRITICAL: Validator service is not running');
      metrics.recommendations.push('Restart the validator service immediately: systemctl restart etrid-validator');
      metrics.healthScore -= 50;
    }

    return metrics;
  }

  analyzeNetworkHealth(allMetrics) {
    const analysis = {
      totalValidators: allMetrics.length,
      runningValidators: allMetrics.filter(m => m.isRunning).length,
      averagePeers: 0,
      averageHealthScore: 0,
      criticalAlerts: [],
      networkRecommendations: [],
    };

    const peerCounts = allMetrics
      .filter(m => m.peerCount !== undefined)
      .map(m => m.peerCount);

    if (peerCounts.length > 0) {
      analysis.averagePeers = (peerCounts.reduce((a, b) => a + b, 0) / peerCounts.length).toFixed(1);
    }

    const healthScores = allMetrics.map(m => m.healthScore);
    analysis.averageHealthScore = (healthScores.reduce((a, b) => a + b, 0) / healthScores.length).toFixed(1);

    // Check for network-wide issues
    const lowPeerValidators = allMetrics.filter(m => m.peerCount && m.peerCount < 2);
    if (lowPeerValidators.length > allMetrics.length * 0.3) {
      analysis.criticalAlerts.push('⚠️  NETWORK ISSUE: More than 30% of validators have low peer count');
      analysis.networkRecommendations.push('Check if there is a network-wide connectivity issue');
    }

    const stoppedValidators = allMetrics.filter(m => !m.isRunning);
    if (stoppedValidators.length > 0) {
      analysis.criticalAlerts.push(`🔴 ${stoppedValidators.length} validator(s) are not running`);
      analysis.networkRecommendations.push(`Restart stopped validators: ${stoppedValidators.map(v => `#${v.number}`).join(', ')}`);
    }

    // Check for block synchronization issues
    const blockHeights = allMetrics
      .filter(m => m.blockHeight !== undefined)
      .map(m => ({ validator: m.number, height: m.blockHeight }));

    if (blockHeights.length > 1) {
      const maxHeight = Math.max(...blockHeights.map(b => b.height));
      const minHeight = Math.min(...blockHeights.map(b => b.height));
      const heightDiff = maxHeight - minHeight;

      if (heightDiff > this.alertThresholds.maxBlockLag) {
        analysis.criticalAlerts.push(`⚠️  SYNC ISSUE: Block height difference of ${heightDiff} blocks`);
        const laggingValidators = blockHeights.filter(b => b.height < maxHeight - 5);
        analysis.networkRecommendations.push(`Check validators with low block height: ${laggingValidators.map(v => `#${v.validator}`).join(', ')}`);
      }
    }

    return analysis;
  }

  generateAIReport(metrics, networkAnalysis) {
    const report = {
      timestamp: new Date().toISOString(),
      summary: {
        overall_health: networkAnalysis.averageHealthScore >= 80 ? '🟢 HEALTHY' :
                       networkAnalysis.averageHealthScore >= 60 ? '🟡 WARNING' : '🔴 CRITICAL',
        total_validators: networkAnalysis.totalValidators,
        running_validators: networkAnalysis.runningValidators,
        average_peers: networkAnalysis.averagePeers,
        average_health_score: networkAnalysis.averageHealthScore,
      },
      critical_alerts: networkAnalysis.criticalAlerts,
      network_recommendations: networkAnalysis.networkRecommendations,
      validator_details: metrics.map(m => ({
        number: m.number,
        name: m.validator,
        health_score: m.healthScore,
        status: m.isRunning ? '🟢 Running' : '🔴 Stopped',
        peers: m.peerCount || 'Unknown',
        block_height: m.blockHeight || 'Unknown',
        disk_usage: m.diskUsage ? `${m.diskUsage}%` : 'Unknown',
        memory_usage: m.memoryUsagePercent ? `${m.memoryUsagePercent}%` : 'Unknown',
        alerts: m.alerts,
        recommendations: m.recommendations,
      })),
    };

    return report;
  }

  printReport(report) {
    console.log('\n' + '='.repeat(100));
    console.log('🤖 AI-POWERED VALIDATOR MONITORING REPORT');
    console.log('='.repeat(100));
    console.log(`\n📊 NETWORK SUMMARY`);
    console.log(`   Overall Health:      ${report.summary.overall_health} (Score: ${report.summary.average_health_score}/100)`);
    console.log(`   Total Validators:    ${report.summary.total_validators}`);
    console.log(`   Running Validators:  ${report.summary.running_validators}/${report.summary.total_validators}`);
    console.log(`   Average Peer Count:  ${report.summary.average_peers}`);
    console.log(`   Timestamp:           ${new Date(report.timestamp).toLocaleString()}`);

    if (report.critical_alerts.length > 0) {
      console.log(`\n🚨 CRITICAL ALERTS (${report.critical_alerts.length})`);
      report.critical_alerts.forEach(alert => console.log(`   ${alert}`));
    }

    if (report.network_recommendations.length > 0) {
      console.log(`\n💡 NETWORK-WIDE RECOMMENDATIONS`);
      report.network_recommendations.forEach((rec, i) => console.log(`   ${i + 1}. ${rec}`));
    }

    console.log(`\n📋 VALIDATOR DETAILS\n`);

    report.validator_details.forEach(v => {
      const healthColor = v.health_score >= 80 ? '🟢' : v.health_score >= 60 ? '🟡' : '🔴';
      console.log(`${healthColor} Validator #${v.number}: ${v.name} (Health: ${v.health_score}/100)`);
      console.log(`   Status: ${v.status} | Peers: ${v.peers} | Block: ${v.block_height}`);
      console.log(`   Disk: ${v.disk_usage} | Memory: ${v.memory_usage}`);

      if (v.alerts.length > 0) {
        console.log(`   Alerts:`);
        v.alerts.forEach(alert => console.log(`      ${alert}`));
      }

      if (v.recommendations.length > 0) {
        console.log(`   Recommendations:`);
        v.recommendations.forEach(rec => console.log(`      💡 ${rec}`));
      }

      console.log('');
    });

    console.log('='.repeat(100) + '\n');
  }

  async saveReport(report, filename) {
    const reportsDir = path.join(__dirname, '../reports');
    try {
      await fs.mkdir(reportsDir, { recursive: true });
      const filepath = path.join(reportsDir, filename);
      await fs.writeFile(filepath, JSON.stringify(report, null, 2));
      console.log(`📄 Report saved to: ${filepath}\n`);
    } catch (error) {
      console.error(`Failed to save report: ${error.message}`);
    }
  }

  async monitorOnce() {
    console.log('🔍 Gathering validator data...\n');

    const statuses = await this.cli.getAllValidatorStatuses();
    const metrics = statuses.map(s => this.parseMetrics(s));
    const networkAnalysis = this.analyzeNetworkHealth(metrics);
    const report = this.generateAIReport(metrics, networkAnalysis);

    this.printReport(report);

    // Save report
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    await this.saveReport(report, `validator-report-${timestamp}.json`);

    return report;
  }

  async monitorContinuous(intervalMinutes = 5) {
    console.log(`🔄 Starting continuous monitoring (interval: ${intervalMinutes} minutes)\n`);
    console.log('Press Ctrl+C to stop\n');

    while (true) {
      await this.monitorOnce();

      console.log(`⏳ Next check in ${intervalMinutes} minutes...\n`);
      await new Promise(resolve => setTimeout(resolve, intervalMinutes * 60 * 1000));
    }
  }
}

// CLI Interface
async function main() {
  const monitor = new AIValidatorMonitor();
  await monitor.initialize();

  const args = process.argv.slice(2);
  const command = args[0];

  switch (command) {
    case 'monitor':
      await monitor.monitorOnce();
      break;

    case 'continuous':
      const interval = parseInt(args[1]) || 5;
      await monitor.monitorContinuous(interval);
      break;

    default:
      console.log(`
🤖 AI-Powered Validator Monitor

Usage:
  node ai-validator-monitor.js <command> [options]

Commands:
  monitor                       Run a single monitoring check
  continuous [interval_min]     Continuous monitoring (default: 5 minutes)

Examples:
  node ai-validator-monitor.js monitor
  node ai-validator-monitor.js continuous 10

Reports are saved to: ./reports/validator-report-*.json
      `);
  }
}

if (require.main === module) {
  main().catch(error => {
    console.error('Error:', error.message);
    process.exit(1);
  });
}

module.exports = AIValidatorMonitor;
