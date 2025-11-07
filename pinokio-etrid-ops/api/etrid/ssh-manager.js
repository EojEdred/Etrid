/**
 * Multi-Cloud SSH Connection Manager
 * Handles SSH connections to nodes across AWS, GCP, Azure, DigitalOcean
 */

const { Client } = require('ssh2');
const fs = require('fs');
const path = require('path');
const os = require('os');

class SSHManager {
  constructor(config) {
    this.config = config;
    this.connections = new Map();
    this.cloudProfiles = this.loadCloudProfiles();
  }

  loadCloudProfiles() {
    return {
      aws: {
        defaultUser: 'ubuntu',
        keyPath: path.join(os.homedir(), '.ssh', 'aws-etrid.pem'),
        bastionHost: null
      },
      gcp: {
        defaultUser: 'etrid',
        keyPath: path.join(os.homedir(), '.ssh', 'gcp-etrid'),
        bastionHost: null
      },
      azure: {
        defaultUser: 'azureuser',
        keyPath: path.join(os.homedir(), '.ssh', 'azure-etrid'),
        bastionHost: null
      },
      digitalocean: {
        defaultUser: 'root',
        keyPath: path.join(os.homedir(), '.ssh', 'do-etrid'),
        bastionHost: null
      }
    };
  }

  /**
   * Connect to a node
   * @param {string} nodeName - Name of the node
   * @param {string} cloud - Cloud provider
   * @param {Function} onOutput - Callback for output streaming
   */
  async connect(nodeName, cloud, onOutput = () => {}) {
    const node = this.findNode(nodeName, cloud);
    if (!node) {
      throw new Error(`Node ${nodeName} not found`);
    }

    const connectionKey = `${cloud}-${nodeName}`;

    // Reuse existing connection if available
    if (this.connections.has(connectionKey)) {
      const conn = this.connections.get(connectionKey);
      if (conn.isReady) {
        onOutput(`♻️  Reusing existing connection to ${nodeName}\n`);
        return conn;
      } else {
        this.connections.delete(connectionKey);
      }
    }

    onOutput(`🔐 Establishing SSH connection to ${nodeName} (${node.ip})...\n`);

    const conn = await this.createConnection(node, cloud, onOutput);
    this.connections.set(connectionKey, conn);

    return conn;
  }

  /**
   * Create SSH connection
   */
  async createConnection(node, cloud, onOutput) {
    return new Promise((resolve, reject) => {
      const client = new Client();
      const profile = this.cloudProfiles[cloud];

      const config = {
        host: node.ip || node.host,
        port: node.sshPort || 22,
        username: node.sshUser || profile.defaultUser,
        privateKey: this.loadKey(node.sshKey || profile.keyPath),
        readyTimeout: 30000,
        keepaliveInterval: 10000
      };

      client
        .on('ready', () => {
          onOutput(`✅ Connected to ${node.name}\n`);

          // Add helper methods to client
          client.isReady = true;
          client.nodeName = node.name;
          client.cloud = cloud;

          client.exec = (command) => this.execCommand(client, command);
          client.execStream = (command, onData) =>
            this.execCommandStream(client, command, onData);

          resolve(client);
        })
        .on('error', (err) => {
          onOutput(`❌ Connection error: ${err.message}\n`);
          reject(err);
        })
        .on('end', () => {
          client.isReady = false;
          onOutput(`🔌 Connection closed to ${node.name}\n`);
        })
        .connect(config);
    });
  }

  /**
   * Execute command on connection
   */
  async execCommand(client, command) {
    return new Promise((resolve, reject) => {
      client.exec(command, (err, stream) => {
        if (err) {
          reject(err);
          return;
        }

        let stdout = '';
        let stderr = '';

        stream
          .on('close', (code, signal) => {
            resolve({
              code,
              signal,
              stdout: stdout.trim(),
              stderr: stderr.trim()
            });
          })
          .on('data', (data) => {
            stdout += data.toString();
          })
          .stderr.on('data', (data) => {
            stderr += data.toString();
          });
      });
    });
  }

  /**
   * Execute command with streaming output
   */
  async execCommandStream(client, command, onData) {
    return new Promise((resolve, reject) => {
      client.exec(command, (err, stream) => {
        if (err) {
          reject(err);
          return;
        }

        stream
          .on('close', (code, signal) => {
            resolve({ code, signal });
          })
          .on('data', (data) => {
            onData(data.toString(), 'stdout');
          })
          .stderr.on('data', (data) => {
            onData(data.toString(), 'stderr');
          });
      });
    });
  }

  /**
   * Execute command on multiple nodes
   */
  async execMultiple(nodeNames, command, options = {}) {
    const { parallel = true, onProgress = () => {} } = options;

    if (parallel) {
      // Execute in parallel
      const promises = nodeNames.map(async (nodeName) => {
        try {
          const [name, cloud] = this.parseNodeIdentifier(nodeName);
          const conn = await this.connect(name, cloud, (output) => {
            onProgress(nodeName, output);
          });

          const result = await conn.exec(command);
          onProgress(nodeName, `\n--- Result ---\n${result.stdout}\n`);

          return {
            node: nodeName,
            success: result.code === 0,
            result
          };
        } catch (err) {
          onProgress(nodeName, `\n❌ Error: ${err.message}\n`);
          return {
            node: nodeName,
            success: false,
            error: err.message
          };
        }
      });

      return Promise.all(promises);
    } else {
      // Execute sequentially
      const results = [];

      for (const nodeName of nodeNames) {
        try {
          const [name, cloud] = this.parseNodeIdentifier(nodeName);
          const conn = await this.connect(name, cloud, (output) => {
            onProgress(nodeName, output);
          });

          const result = await conn.exec(command);
          onProgress(nodeName, `\n--- Result ---\n${result.stdout}\n`);

          results.push({
            node: nodeName,
            success: result.code === 0,
            result
          });
        } catch (err) {
          onProgress(nodeName, `\n❌ Error: ${err.message}\n`);
          results.push({
            node: nodeName,
            success: false,
            error: err.message
          });
        }
      }

      return results;
    }
  }

  /**
   * Get interactive shell (for terminal integration)
   */
  async getShell(nodeName, cloud, onOutput = () => {}) {
    const conn = await this.connect(nodeName, cloud, onOutput);

    return new Promise((resolve, reject) => {
      conn.shell((err, stream) => {
        if (err) {
          reject(err);
          return;
        }

        stream.on('data', (data) => {
          onOutput(data.toString());
        });

        resolve({
          write: (data) => stream.write(data),
          end: () => stream.end(),
          stream
        });
      });
    });
  }

  /**
   * Copy file to node (SCP)
   */
  async copyFile(nodeName, cloud, localPath, remotePath, onProgress = () => {}) {
    const conn = await this.connect(nodeName, cloud, onProgress);

    return new Promise((resolve, reject) => {
      conn.sftp((err, sftp) => {
        if (err) {
          reject(err);
          return;
        }

        const readStream = fs.createReadStream(localPath);
        const writeStream = sftp.createWriteStream(remotePath);

        readStream
          .on('error', reject)
          .pipe(writeStream)
          .on('error', reject)
          .on('close', () => {
            onProgress(`✅ File copied: ${localPath} → ${remotePath}\n`);
            resolve();
          });
      });
    });
  }

  /**
   * Download file from node
   */
  async downloadFile(nodeName, cloud, remotePath, localPath, onProgress = () => {}) {
    const conn = await this.connect(nodeName, cloud, onProgress);

    return new Promise((resolve, reject) => {
      conn.sftp((err, sftp) => {
        if (err) {
          reject(err);
          return;
        }

        const readStream = sftp.createReadStream(remotePath);
        const writeStream = fs.createWriteStream(localPath);

        readStream
          .on('error', reject)
          .pipe(writeStream)
          .on('error', reject)
          .on('close', () => {
            onProgress(`✅ File downloaded: ${remotePath} → ${localPath}\n`);
            resolve();
          });
      });
    });
  }

  /**
   * Close all connections
   */
  closeAll() {
    for (const [key, conn] of this.connections) {
      try {
        conn.end();
      } catch (err) {
        console.error(`Error closing connection ${key}:`, err);
      }
    }
    this.connections.clear();
  }

  /**
   * Close specific connection
   */
  close(nodeName, cloud) {
    const key = `${cloud}-${nodeName}`;
    const conn = this.connections.get(key);

    if (conn) {
      conn.end();
      this.connections.delete(key);
    }
  }

  // Helper methods
  findNode(nodeName, cloud) {
    if (cloud) {
      const cloudConfig = this.config.clouds[cloud];
      return cloudConfig?.nodes?.find((n) => n.name === nodeName);
    }

    // Search all clouds
    for (const cloudConfig of Object.values(this.config.clouds)) {
      const node = cloudConfig.nodes?.find((n) => n.name === nodeName);
      if (node) return node;
    }

    return null;
  }

  parseNodeIdentifier(identifier) {
    // Format: "nodeName" or "cloud:nodeName"
    const parts = identifier.split(':');
    if (parts.length === 2) {
      return [parts[1], parts[0]];
    }
    return [identifier, null];
  }

  loadKey(keyPath) {
    try {
      return fs.readFileSync(path.resolve(keyPath));
    } catch (err) {
      throw new Error(`Failed to load SSH key from ${keyPath}: ${err.message}`);
    }
  }
}

module.exports = { SSHManager };
