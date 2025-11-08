#!/usr/bin/env node

/**
 * ËTRID Validator CLI Manager
 * Remote command-line access to distributed validators
 */

const { NodeSSH } = require('node-ssh');
const fs = require('fs');
const path = require('path');

// Load validator configuration
const VALIDATORS_CONFIG = require('../infrastructure/config/validator-ips.json');

// SSH configuration
const SSH_KEY_PATH = process.env.SSH_KEY_PATH || path.join(process.env.HOME, '.ssh', 'id_rsa');
const SSH_TIMEOUT = 30000; // 30 seconds

class ValidatorCLI {
  constructor() {
    this.validators = VALIDATORS_CONFIG.validators || [];
    this.ssh = new NodeSSH();
  }

  /**
   * List all validators
   */
  listValidators() {
    console.log('\n' + '='.repeat(80));
    console.log('📋 ËTRID VALIDATOR NETWORK');
    console.log('='.repeat(80) + '\n');

    const accessible = this.validators.filter(v => v.accessible);
    const inaccessible = this.validators.filter(v => !v.accessible);

    console.log(`Total Validators: ${this.validators.length}`);
    console.log(`✅ Accessible: ${accessible.length}`);
    console.log(`❌ Inaccessible: ${inaccessible.length}\n`);

    console.log('ID  | Name                 | Region              | Role           | Status');
    console.log('-'.repeat(80));

    this.validators.forEach(v => {
      const status = v.accessible ? '✅' : '❌';
      const id = String(v.id).padEnd(3);
      const name = v.name.padEnd(20);
      const region = v.region.padEnd(19);
      const role = v.role.padEnd(14);
      console.log(`${id} | ${name} | ${region} | ${role} | ${status}`);
    });

    console.log('\n' + '='.repeat(80) + '\n');
  }

  /**
   * Connect to a validator via SSH
   */
  async connectToValidator(validatorId) {
    const validator = this.validators.find(v => v.id === validatorId);

    if (!validator) {
      throw new Error(`Validator #${validatorId} not found`);
    }

    if (!validator.accessible) {
      throw new Error(`Validator #${validatorId} (${validator.name}) is not accessible`);
    }

    if (!fs.existsSync(SSH_KEY_PATH)) {
      throw new Error(`SSH key not found at ${SSH_KEY_PATH}. Set SSH_KEY_PATH environment variable.`);
    }

    try {
      await this.ssh.connect({
        host: validator.ip,
        username: validator.sshUser,
        privateKeyPath: SSH_KEY_PATH,
        timeout: SSH_TIMEOUT,
      });

      return validator;
    } catch (error) {
      throw new Error(`Failed to connect to ${validator.name}: ${error.message}`);
    }
  }

  /**
   * Execute command on a validator
   */
  async executeCommand(validatorId, command) {
    console.log(`\n🔧 Executing on Validator #${validatorId}...\n`);

    const validator = await this.connectToValidator(validatorId);

    try {
      const result = await this.ssh.execCommand(command);

      console.log(`✅ ${validator.name} (${validator.region})`);
      console.log('-'.repeat(80));

      if (result.stdout) {
        console.log(result.stdout);
      }

      if (result.stderr) {
        console.error('⚠️  STDERR:', result.stderr);
      }

      console.log('-'.repeat(80));
      console.log(`Exit code: ${result.code}\n`);

      return result;
    } finally {
      this.ssh.dispose();
    }
  }

  /**
   * Execute command on all accessible validators
   */
  async executeOnAll(command) {
    const accessible = this.validators.filter(v => v.accessible);

    console.log(`\n🚀 Executing on ${accessible.length} validators...\n`);
    console.log('='.repeat(80));

    const results = [];

    for (const validator of accessible) {
      try {
        const ssh = new NodeSSH();

        await ssh.connect({
          host: validator.ip,
          username: validator.sshUser,
          privateKeyPath: SSH_KEY_PATH,
          timeout: SSH_TIMEOUT,
        });

        const result = await ssh.execCommand(command);

        results.push({
          validator,
          success: result.code === 0,
          output: result.stdout,
          error: result.stderr,
          code: result.code,
        });

        console.log(`✅ #${validator.id} ${validator.name}`);
        if (result.stdout) {
          console.log(result.stdout.split('\n').slice(0, 3).join('\n'));
        }
        console.log('');

        ssh.dispose();
      } catch (error) {
        console.error(`❌ #${validator.id} ${validator.name}: ${error.message}\n`);
        results.push({
          validator,
          success: false,
          error: error.message,
        });
      }
    }

    console.log('='.repeat(80));
    console.log(`\n✅ Completed: ${results.filter(r => r.success).length}/${accessible.length}\n`);

    return results;
  }

  /**
   * Get validator status
   */
  async getStatus(validatorId) {
    console.log(`\n📊 Getting status for Validator #${validatorId}...\n`);

    const validator = await this.connectToValidator(validatorId);

    try {
      const commands = {
        service: 'systemctl status etrid-validator 2>/dev/null || systemctl status etrid-node 2>/dev/null || echo "Service not found"',
        peers: 'journalctl -u etrid-validator -n 100 2>/dev/null | grep -i "peer" | tail -5 || echo "No peer info"',
        disk: 'df -h / | tail -1',
        memory: 'free -h | grep Mem',
        uptime: 'uptime',
      };

      const results = {};

      for (const [key, cmd] of Object.entries(commands)) {
        const result = await this.ssh.execCommand(cmd);
        results[key] = result.stdout || result.stderr || 'N/A';
      }

      console.log(`✅ ${validator.name} (${validator.region})`);
      console.log('='.repeat(80));
      console.log('\n📊 SERVICE STATUS:');
      console.log(results.service.split('\n').slice(0, 5).join('\n'));
      console.log('\n🌐 PEER INFO:');
      console.log(results.peers);
      console.log('\n💾 DISK USAGE:');
      console.log(results.disk);
      console.log('\n🧠 MEMORY:');
      console.log(results.memory);
      console.log('\n⏱️  UPTIME:');
      console.log(results.uptime);
      console.log('='.repeat(80) + '\n');

      return results;
    } finally {
      this.ssh.dispose();
    }
  }

  /**
   * Get logs from validator
   */
  async getLogs(validatorId, lines = 50) {
    console.log(`\n📜 Getting last ${lines} lines from Validator #${validatorId}...\n`);

    const validator = await this.connectToValidator(validatorId);

    try {
      const result = await this.ssh.execCommand(
        `journalctl -u etrid-validator -n ${lines} 2>/dev/null || journalctl -u etrid-node -n ${lines} 2>/dev/null || echo "No logs found"`
      );

      console.log(`✅ ${validator.name} (${validator.region})`);
      console.log('='.repeat(80));
      console.log(result.stdout || result.stderr || 'No logs available');
      console.log('='.repeat(80) + '\n');

      return result.stdout;
    } finally {
      this.ssh.dispose();
    }
  }

  /**
   * Restart validator service
   */
  async restart(validatorId) {
    console.log(`\n🔄 Restarting Validator #${validatorId}...\n`);

    const validator = await this.connectToValidator(validatorId);

    try {
      const result = await this.ssh.execCommand(
        'sudo systemctl restart etrid-validator 2>/dev/null || sudo systemctl restart etrid-node 2>/dev/null || echo "Service not found"'
      );

      console.log(`✅ ${validator.name} (${validator.region})`);
      console.log('='.repeat(80));
      console.log(result.stdout || 'Restart command executed');

      if (result.stderr) {
        console.error('⚠️  Errors:', result.stderr);
      }

      console.log('='.repeat(80) + '\n');

      // Wait and check status
      console.log('⏳ Waiting 5 seconds before checking status...\n');
      await new Promise(resolve => setTimeout(resolve, 5000));

      await this.getStatus(validatorId);

      return result;
    } finally {
      this.ssh.dispose();
    }
  }

  /**
   * Get status of all accessible validators
   */
  async getAllStatus() {
    const accessible = this.validators.filter(v => v.accessible);

    console.log(`\n📊 Getting status for ${accessible.length} validators...\n`);
    console.log('='.repeat(80));

    const statuses = [];

    for (const validator of accessible) {
      try {
        const ssh = new NodeSSH();

        await ssh.connect({
          host: validator.ip,
          username: validator.sshUser,
          privateKeyPath: SSH_KEY_PATH,
          timeout: SSH_TIMEOUT,
        });

        // Quick status check
        const serviceCheck = await ssh.execCommand(
          'systemctl is-active etrid-validator 2>/dev/null || systemctl is-active etrid-node 2>/dev/null || echo "unknown"'
        );

        const isRunning = serviceCheck.stdout.trim() === 'active';

        statuses.push({
          id: validator.id,
          name: validator.name,
          region: validator.region,
          status: isRunning ? '🟢 Running' : '🔴 Stopped',
          accessible: true,
        });

        console.log(`${isRunning ? '🟢' : '🔴'} #${validator.id} ${validator.name.padEnd(20)} - ${validator.region}`);

        ssh.dispose();
      } catch (error) {
        console.log(`❌ #${validator.id} ${validator.name.padEnd(20)} - Connection failed`);
        statuses.push({
          id: validator.id,
          name: validator.name,
          region: validator.region,
          status: '❌ Connection Failed',
          accessible: false,
        });
      }
    }

    console.log('='.repeat(80));

    const running = statuses.filter(s => s.status === '🟢 Running').length;
    const stopped = statuses.filter(s => s.status === '🔴 Stopped').length;
    const failed = statuses.filter(s => s.status === '❌ Connection Failed').length;

    console.log(`\n✅ Running: ${running} | 🔴 Stopped: ${stopped} | ❌ Failed: ${failed}\n`);

    return statuses;
  }
}

// CLI Interface
async function main() {
  const args = process.argv.slice(2);
  const command = args[0];
  const cli = new ValidatorCLI();

  try {
    switch (command) {
      case 'list':
        cli.listValidators();
        break;

      case 'status':
        if (!args[1]) {
          console.error('❌ Usage: validator-cli.js status <validator-id>');
          process.exit(1);
        }
        await cli.getStatus(parseInt(args[1]));
        break;

      case 'status-all':
        await cli.getAllStatus();
        break;

      case 'exec':
        if (!args[1] || !args[2]) {
          console.error('❌ Usage: validator-cli.js exec <validator-id> "<command>"');
          process.exit(1);
        }
        await cli.executeCommand(parseInt(args[1]), args[2]);
        break;

      case 'exec-all':
        if (!args[1]) {
          console.error('❌ Usage: validator-cli.js exec-all "<command>"');
          process.exit(1);
        }
        await cli.executeOnAll(args[1]);
        break;

      case 'logs':
        const validatorId = parseInt(args[1]);
        const lines = parseInt(args[2]) || 50;
        await cli.getLogs(validatorId, lines);
        break;

      case 'restart':
        if (!args[1]) {
          console.error('❌ Usage: validator-cli.js restart <validator-id>');
          process.exit(1);
        }
        await cli.restart(parseInt(args[1]));
        break;

      default:
        console.log('\n📋 ËTRID Validator CLI Manager\n');
        console.log('Usage:');
        console.log('  node validator-cli.js list                          - List all validators');
        console.log('  node validator-cli.js status <id>                   - Get validator status');
        console.log('  node validator-cli.js status-all                    - Get all validator statuses');
        console.log('  node validator-cli.js exec <id> "<command>"         - Execute command');
        console.log('  node validator-cli.js exec-all "<command>"          - Execute on all validators');
        console.log('  node validator-cli.js logs <id> [lines]             - View logs');
        console.log('  node validator-cli.js restart <id>                  - Restart validator\n');
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

module.exports = ValidatorCLI;
