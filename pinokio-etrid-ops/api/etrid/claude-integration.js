/**
 * Claude Code Integration
 * AI-powered analysis and automation using Claude Code CLI
 */

const { spawn, exec } = require('child_process');
const util = require('util');
const execPromise = util.promisify(exec);

class ClaudeIntegration {
  constructor(config) {
    this.config = config;
    this.claudeCommand = 'claude'; // Assumes Claude Code CLI is in PATH
  }

  /**
   * Ask Claude Code a question with context
   */
  async ask(query, context = {}, onChunk = () => {}) {
    const prompt = this.buildPrompt(query, context);

    try {
      const response = await this.executeClaudeCommand(prompt, onChunk);
      return response;
    } catch (err) {
      throw new Error(`Claude Code integration failed: ${err.message}`);
    }
  }

  /**
   * Analyze logs with Claude
   */
  async analyzeLogs(logs, onChunk = () => {}) {
    const summary = this.summarizeLogs(logs);

    const prompt = `You are analyzing logs from Etrid blockchain nodes.

${summary}

Please analyze these logs and provide:
1. Any errors or warnings that need attention
2. Performance issues or anomalies
3. Recommended actions
4. Priority level (low/medium/high/critical)

Be concise and actionable.`;

    return await this.ask(prompt, { logs }, onChunk);
  }

  /**
   * Debug an issue with Claude
   */
  async debug(issue, context = {}, onChunk = () => {}) {
    const prompt = `You are debugging an issue on the Etrid blockchain infrastructure.

Issue: ${issue.message}
Node: ${issue.node || 'Unknown'}
Chain: ${issue.chain || 'Unknown'}
Severity: ${issue.severity || 'Unknown'}

Context:
${JSON.stringify(context, null, 2)}

Please provide:
1. Root cause analysis
2. Step-by-step fix instructions
3. Prevention measures for the future
4. Any relevant code changes needed`;

    return await this.ask(prompt, context, onChunk);
  }

  /**
   * Optimize node configuration with Claude
   */
  async optimizeConfig(chain, currentConfig, metrics, onChunk = () => {}) {
    const prompt = `You are optimizing the configuration for an Etrid ${chain} node.

Current Configuration:
${JSON.stringify(currentConfig, null, 2)}

Recent Performance Metrics:
${JSON.stringify(metrics, null, 2)}

Please analyze and suggest:
1. Configuration improvements for better performance
2. Resource allocation optimization
3. Network/P2P settings tuning
4. Any security enhancements

Provide specific configuration values with explanations.`;

    return await this.ask(prompt, { chain, currentConfig, metrics }, onChunk);
  }

  /**
   * Generate runbook for operations
   */
  async generateRunbook(operations, onChunk = () => {}) {
    const prompt = `Generate a detailed runbook for the following Etrid blockchain operations:

${operations.map((op, i) => `${i + 1}. ${op}`).join('\n')}

Include:
- Prerequisites and requirements
- Step-by-step instructions
- Expected outcomes
- Troubleshooting steps
- Rollback procedures (if applicable)

Format in markdown.`;

    return await this.ask(prompt, { operations }, onChunk);
  }

  /**
   * Review code changes with Claude
   */
  async reviewCode(filePath, diff, onChunk = () => {}) {
    const prompt = `Review this code change for the Etrid blockchain project:

File: ${filePath}

Changes:
\`\`\`diff
${diff}
\`\`\`

Please review for:
1. Security vulnerabilities
2. Performance implications
3. Best practices compliance
4. Potential bugs
5. Suggestions for improvement`;

    return await this.ask(prompt, { filePath, diff }, onChunk);
  }

  /**
   * Get deployment recommendations
   */
  async getDeploymentPlan(changes, targetChains, onChunk = () => {}) {
    const prompt = `Create a deployment plan for Etrid blockchain changes.

Changes:
${changes.map(c => `- ${c}`).join('\n')}

Target Chains: ${targetChains.join(', ')}

Provide:
1. Deployment strategy (rolling, blue-green, etc.)
2. Rollback plan
3. Testing checklist
4. Monitoring points
5. Timeline estimate`;

    return await this.ask(prompt, { changes, targetChains }, onChunk);
  }

  /**
   * Analyze security alerts
   */
  async analyzeSecurity(alerts, onChunk = () => {}) {
    const prompt = `Analyze these security alerts from Etrid blockchain infrastructure:

${JSON.stringify(alerts, null, 2)}

For each alert, provide:
1. Severity assessment
2. Immediate actions required
3. Long-term mitigation
4. False positive likelihood`;

    return await this.ask(prompt, { alerts }, onChunk);
  }

  /**
   * Generate documentation
   */
  async generateDocs(codeContext, docType, onChunk = () => {}) {
    const prompt = `Generate ${docType} documentation for Etrid blockchain:

Context:
${JSON.stringify(codeContext, null, 2)}

Create comprehensive documentation in markdown format.`;

    return await this.ask(prompt, { codeContext, docType }, onChunk);
  }

  /**
   * Execute Claude Code CLI command
   */
  async executeClaudeCommand(prompt, onChunk = () => {}) {
    return new Promise((resolve, reject) => {
      let output = '';

      // Use Claude Code CLI with streaming
      const proc = spawn(this.claudeCommand, ['--non-interactive'], {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      // Send prompt
      proc.stdin.write(prompt);
      proc.stdin.end();

      // Capture stdout
      proc.stdout.on('data', (data) => {
        const chunk = data.toString();
        output += chunk;
        onChunk(chunk);
      });

      // Capture stderr
      proc.stderr.on('data', (data) => {
        console.error('Claude stderr:', data.toString());
      });

      // Handle completion
      proc.on('close', (code) => {
        if (code === 0) {
          resolve(output);
        } else {
          reject(new Error(`Claude Code exited with code ${code}`));
        }
      });

      proc.on('error', (err) => {
        reject(err);
      });
    });
  }

  /**
   * Check if Claude Code is available
   */
  async isAvailable() {
    try {
      const { stdout } = await execPromise(`${this.claudeCommand} --version`);
      return true;
    } catch (err) {
      return false;
    }
  }

  /**
   * Run Claude Code in project directory
   */
  async runInProject(command, projectPath, onChunk = () => {}) {
    return new Promise((resolve, reject) => {
      const proc = spawn(this.claudeCommand, [command], {
        cwd: projectPath,
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let output = '';

      proc.stdout.on('data', (data) => {
        const chunk = data.toString();
        output += chunk;
        onChunk(chunk);
      });

      proc.stderr.on('data', (data) => {
        console.error('Claude stderr:', data.toString());
      });

      proc.on('close', (code) => {
        if (code === 0) {
          resolve(output);
        } else {
          reject(new Error(`Command failed with code ${code}`));
        }
      });

      proc.on('error', (err) => {
        reject(err);
      });
    });
  }

  // Helper methods

  buildPrompt(query, context) {
    let prompt = query;

    if (Object.keys(context).length > 0) {
      prompt += '\n\nAdditional Context:\n';
      prompt += JSON.stringify(context, null, 2);
    }

    return prompt;
  }

  summarizeLogs(logs) {
    let summary = 'Logs Summary:\n\n';

    for (const [chain, chainLogs] of Object.entries(logs)) {
      summary += `Chain: ${chain}\n`;

      for (const nodeLog of chainLogs) {
        if (nodeLog.error) {
          summary += `  - ${nodeLog.node}: Error - ${nodeLog.error}\n`;
        } else {
          const errorCount = nodeLog.lines.filter(l =>
            l.toLowerCase().includes('error')
          ).length;
          const warningCount = nodeLog.lines.filter(l =>
            l.toLowerCase().includes('warn')
          ).length;

          summary += `  - ${nodeLog.node}: ${nodeLog.lines.length} lines`;
          if (errorCount > 0) summary += `, ${errorCount} errors`;
          if (warningCount > 0) summary += `, ${warningCount} warnings`;
          summary += '\n';

          // Include sample of errors/warnings
          if (errorCount > 0 || warningCount > 0) {
            const relevantLines = nodeLog.lines.filter(l =>
              l.toLowerCase().includes('error') ||
              l.toLowerCase().includes('warn')
            ).slice(0, 5);

            summary += relevantLines.map(l => `    ${l}`).join('\n') + '\n';
          }
        }
      }

      summary += '\n';
    }

    return summary;
  }
}

module.exports = { ClaudeIntegration };
