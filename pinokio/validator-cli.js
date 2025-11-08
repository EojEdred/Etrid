#!/usr/bin/env node

/**
 * ÉTRID Validator CLI Manager for Pinokio
 * Provides remote access to validator VMs with AI-powered assistance
 */

const { spawn, exec } = require('child_process');
const fs = require('fs').promises;
const path = require('path');

const VALIDATORS_CONFIG = path.join(__dirname, '../infrastructure/config/validator-ips.json');

class ValidatorCLI {
  constructor() {
    this.validators = [];
    this.sshKeyPath = process.env.SSH_KEY_PATH || '~/.ssh/id_rsa';
  }

  async loadValidators() {
    try {
      const data = await fs.readFile(VALIDATORS_CONFIG, 'utf8');
      const config = JSON.parse(data);
      this.validators = config.validators;
      return this.validators;
    } catch (error) {
      console.error('Failed to load validator configuration:', error.message);
      return [];
    }
  }

  getAccessibleValidators() {
    return this.validators.filter(v => v.accessible === true);
  }

  async sshExec(validator, command) {
    return new Promise((resolve, reject) => {
      const sshCommand = `ssh -o StrictHostKeyChecking=no -i ${this.sshKeyPath} ${validator.ssh_user}@${validator.ip} "${command}"`;

      exec(sshCommand, (error, stdout, stderr) => {
        if (error) {
          reject({ error, stderr, validator: validator.name });
        } else {
          resolve({ stdout, stderr, validator: validator.name });
        }
      });
    });
  }

  async getValidatorStatus(validator) {
    const commands = {
      systemd: 'systemctl status etrid-validator 2>/dev/null || echo "NOT_RUNNING"',
      peers: 'journalctl -u etrid-validator --since "1 minute ago" | grep -i "peers" | tail -1 || echo "NO_PEER_INFO"',
      blockHeight: 'journalctl -u etrid-validator --since "10 seconds ago" | grep -i "best" | tail -1 || echo "NO_BLOCK_INFO"',
      uptime: 'systemctl show etrid-validator -p ActiveEnterTimestamp --value 2>/dev/null || echo "UNKNOWN"',
      diskUsage: 'df -h /var/lib/etrid 2>/dev/null || df -h ~ | tail -1',
      memoryUsage: 'free -h | grep Mem',
    };

    const results = {};

    for (const [key, cmd] of Object.entries(commands)) {
      try {
        const result = await this.sshExec(validator, cmd);
        results[key] = result.stdout.trim();
      } catch (error) {
        results[key] = `ERROR: ${error.stderr || error.message}`;
      }
    }

    return {
      validator: validator.name,
      number: validator.number,
      ip: validator.ip,
      region: validator.region,
      role: validator.role,
      ...results
    };
  }

  async getAllValidatorStatuses() {
    const accessible = this.getAccessibleValidators();
    console.log(`\nChecking ${accessible.length} accessible validators...\n`);

    const statusPromises = accessible.map(v => this.getValidatorStatus(v));
    const statuses = await Promise.allSettled(statusPromises);

    return statuses.map((result, index) => {
      if (result.status === 'fulfilled') {
        return result.value;
      } else {
        return {
          validator: accessible[index].name,
          error: result.reason
        };
      }
    });
  }

  async executeOnValidator(validatorNumber, command) {
    const validator = this.validators.find(v => v.number === validatorNumber);

    if (!validator) {
      throw new Error(`Validator ${validatorNumber} not found`);
    }

    if (!validator.accessible) {
      throw new Error(`Validator ${validatorNumber} (${validator.name}) is not accessible`);
    }

    return await this.sshExec(validator, command);
  }

  async executeOnAll(command) {
    const accessible = this.getAccessibleValidators();
    const results = [];

    for (const validator of accessible) {
      try {
        const result = await this.sshExec(validator, command);
        results.push({
          validator: validator.name,
          number: validator.number,
          success: true,
          output: result.stdout
        });
      } catch (error) {
        results.push({
          validator: validator.name,
          number: validator.number,
          success: false,
          error: error.stderr || error.message
        });
      }
    }

    return results;
  }

  async restartValidator(validatorNumber) {
    const command = 'sudo systemctl restart etrid-validator';
    return await this.executeOnValidator(validatorNumber, command);
  }

  async viewLogs(validatorNumber, lines = 50) {
    const command = `journalctl -u etrid-validator -n ${lines} --no-pager`;
    return await this.executeOnValidator(validatorNumber, command);
  }

  async getNodeInfo(validatorNumber) {
    const commands = [
      'cat ~/.etrid/keys/node_info.txt 2>/dev/null || echo "Node info not found"',
      'ps aux | grep flarechain-node | grep -v grep || echo "Process not running"',
    ];

    const results = {};
    for (const cmd of commands) {
      try {
        const result = await this.executeOnValidator(validatorNumber, cmd);
        results[cmd.split(' ')[1]] = result.stdout;
      } catch (error) {
        results.error = error.message;
      }
    }

    return results;
  }

  formatStatus(status) {
    console.log('\n' + '='.repeat(80));
    console.log(`Validator #${status.number}: ${status.validator}`);
    console.log('='.repeat(80));
    console.log(`IP:          ${status.ip}`);
    console.log(`Region:      ${status.region}`);
    console.log(`Role:        ${status.role}`);
    console.log(`\nService:     ${status.systemd?.includes('active (running)') ? '🟢 RUNNING' : '🔴 STOPPED'}`);
    console.log(`Uptime:      ${status.uptime}`);
    console.log(`\nPeers:       ${status.peers}`);
    console.log(`Block:       ${status.blockHeight}`);
    console.log(`\nDisk:        ${status.diskUsage}`);
    console.log(`Memory:      ${status.memoryUsage}`);
    console.log('='.repeat(80) + '\n');
  }
}

// CLI Interface
async function main() {
  const cli = new ValidatorCLI();
  await cli.loadValidators();

  const args = process.argv.slice(2);
  const command = args[0];

  switch (command) {
    case 'list':
      console.log('\n📋 ÉTRID Validators:\n');
      cli.validators.forEach(v => {
        const access = v.accessible ? '✅' : '❌';
        console.log(`${access} #${v.number.toString().padStart(2)}: ${v.name.padEnd(20)} - ${v.ip.padEnd(15)} (${v.region})`);
      });
      console.log(`\n✅ Accessible: ${cli.getAccessibleValidators().length}/${cli.validators.length}\n`);
      break;

    case 'status':
      const validatorNum = parseInt(args[1]);
      if (validatorNum) {
        const status = await cli.getValidatorStatus(
          cli.validators.find(v => v.number === validatorNum)
        );
        cli.formatStatus(status);
      } else {
        const statuses = await cli.getAllValidatorStatuses();
        statuses.forEach(s => cli.formatStatus(s));
      }
      break;

    case 'exec':
      const target = parseInt(args[1]);
      const cmd = args.slice(2).join(' ');
      if (!cmd) {
        console.error('Usage: validator-cli exec <validator_number> <command>');
        process.exit(1);
      }
      const result = await cli.executeOnValidator(target, cmd);
      console.log(result.stdout);
      break;

    case 'exec-all':
      const cmdAll = args.slice(1).join(' ');
      if (!cmdAll) {
        console.error('Usage: validator-cli exec-all <command>');
        process.exit(1);
      }
      const results = await cli.executeOnAll(cmdAll);
      results.forEach(r => {
        console.log(`\n${'='.repeat(40)}`);
        console.log(`Validator #${r.number}: ${r.validator}`);
        console.log('='.repeat(40));
        if (r.success) {
          console.log(r.output);
        } else {
          console.error(`ERROR: ${r.error}`);
        }
      });
      break;

    case 'logs':
      const logValidator = parseInt(args[1]);
      const logLines = parseInt(args[2]) || 50;
      const logs = await cli.viewLogs(logValidator, logLines);
      console.log(logs.stdout);
      break;

    case 'restart':
      const restartNum = parseInt(args[1]);
      console.log(`Restarting validator #${restartNum}...`);
      const restartResult = await cli.restartValidator(restartNum);
      console.log(restartResult.stdout);
      break;

    case 'info':
      const infoNum = parseInt(args[1]);
      const info = await cli.getNodeInfo(infoNum);
      console.log(JSON.stringify(info, null, 2));
      break;

    default:
      console.log(`
ÉTRID Validator CLI Manager

Usage:
  node validator-cli.js <command> [options]

Commands:
  list                          List all validators
  status [validator_number]     Get status of validator(s)
  exec <num> <command>          Execute command on specific validator
  exec-all <command>            Execute command on all accessible validators
  logs <num> [lines]            View validator logs (default: 50 lines)
  restart <num>                 Restart validator service
  info <num>                    Get node information

Examples:
  node validator-cli.js list
  node validator-cli.js status 7
  node validator-cli.js exec 7 "systemctl status etrid-validator"
  node validator-cli.js exec-all "df -h"
  node validator-cli.js logs 7 100
  node validator-cli.js restart 7

Environment Variables:
  SSH_KEY_PATH                  Path to SSH private key (default: ~/.ssh/id_rsa)
      `);
  }
}

if (require.main === module) {
  main().catch(error => {
    console.error('Error:', error.message);
    process.exit(1);
  });
}

module.exports = ValidatorCLI;
