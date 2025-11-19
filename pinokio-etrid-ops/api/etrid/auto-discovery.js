/**
 * Auto-Discovery System
 * Automatically detects and configures local validator node
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');
const os = require('os');

class AutoDiscovery {
  constructor() {
    this.localEndpoints = [
      'ws://localhost:9944',
      'ws://127.0.0.1:9944',
      'ws://0.0.0.0:9944'
    ];
  }

  /**
   * Detect local validator node
   * Returns node info if found, null otherwise
   */
  async detectLocalValidator() {
    console.log('🔍 Scanning for local validator node...');

    for (const endpoint of this.localEndpoints) {
      try {
        const provider = new WsProvider(endpoint, 2000); // 2 second timeout
        const api = await ApiPromise.create({ provider });

        // Get node information
        const [chain, nodeName, nodeVersion, properties] = await Promise.all([
          api.rpc.system.chain(),
          api.rpc.system.name(),
          api.rpc.system.version(),
          api.rpc.system.properties()
        ]);

        // Check if this is an Etrid node
        const chainName = chain.toString().toLowerCase();
        const isEtridNode = chainName.includes('etrid') ||
                           chainName.includes('flare');

        if (!isEtridNode) {
          console.log(`  Found ${chain} node at ${endpoint} (not Etrid)`);
          await api.disconnect();
          continue;
        }

        console.log(`✓ Found Etrid validator: ${chain} v${nodeVersion}`);

        // Determine chain type
        let detectedChain = 'etrid-mainnet';
        if (chainName.includes('test')) {
          detectedChain = 'etrid-testnet';
        } else if (chainName.includes('dev')) {
          detectedChain = 'etrid-devnet';
        }

        // Build node configuration
        const nodeConfig = {
          rpcEndpoint: endpoint,
          httpEndpoint: endpoint.replace('ws://', 'http://').replace('9944', '9933'),
          ssh: {
            host: '127.0.0.1',
            port: 22,
            user: os.userInfo().username
          },
          cloudProvider: null, // Local/self-hosted
          monitoring: {
            enableAlerts: true,
            autoHealthCheck: true,
            checkInterval: 300 // 5 minutes
          },
          metadata: {
            chain: chain.toString(),
            nodeName: nodeName.toString(),
            nodeVersion: nodeVersion.toString(),
            autoDiscovered: true,
            discoveredAt: Date.now()
          }
        };

        await api.disconnect();

        return {
          nodeName: 'local-validator',
          chain: detectedChain,
          nodeConfig,
          metadata: {
            chain: chain.toString(),
            version: nodeVersion.toString()
          }
        };

      } catch (err) {
        // Node not found at this endpoint, continue
        continue;
      }
    }

    console.log('⚠ No local validator detected');
    return null;
  }

  /**
   * Auto-configure validator for new user
   * Called during first-time registration
   */
  async autoConfigureValidator(database, userId) {
    console.log(`🔧 Auto-configuring validator for user ${userId}...`);

    try {
      const validatorInfo = await this.detectLocalValidator();

      if (!validatorInfo) {
        console.log('  No local validator to configure');
        return { success: false, message: 'No local validator detected' };
      }

      // Check if node already exists for this user
      const existingNodes = await database.getUserNodes(userId);
      const nodeExists = existingNodes.some(n => n.node_name === validatorInfo.nodeName);

      if (nodeExists) {
        console.log('  Validator already configured');
        return { success: true, message: 'Validator already configured', existing: true };
      }

      // Add validator to user's account
      await database.saveUserNode(
        userId,
        validatorInfo.chain,
        validatorInfo.nodeName,
        JSON.stringify(validatorInfo.nodeConfig)
      );

      console.log(`✓ Validator auto-configured successfully`);
      console.log(`  Chain: ${validatorInfo.chain}`);
      console.log(`  Name: ${validatorInfo.nodeName}`);
      console.log(`  RPC: ${validatorInfo.nodeConfig.rpcEndpoint}`);

      return {
        success: true,
        message: 'Validator auto-configured',
        validator: validatorInfo
      };

    } catch (err) {
      console.error('❌ Auto-configuration failed:', err.message);
      return {
        success: false,
        message: 'Auto-configuration failed: ' + err.message,
        error: err
      };
    }
  }

  /**
   * Get validator status
   * Returns current status of the local validator
   */
  async getValidatorStatus() {
    const validatorInfo = await this.detectLocalValidator();

    if (!validatorInfo) {
      return {
        online: false,
        message: 'Validator not running or not reachable'
      };
    }

    try {
      const provider = new WsProvider(validatorInfo.nodeConfig.rpcEndpoint);
      const api = await ApiPromise.create({ provider });

      const [
        header,
        health,
        peers,
        isSyncing
      ] = await Promise.all([
        api.rpc.chain.getHeader(),
        api.rpc.system.health(),
        api.rpc.system.peers(),
        api.rpc.system.syncState()
      ]);

      await api.disconnect();

      return {
        online: true,
        blockHeight: header.number.toNumber(),
        peers: peers.length,
        isSyncing: isSyncing.currentBlock.toNumber() < isSyncing.highestBlock.toNumber(),
        health: {
          peers: health.peers.toNumber(),
          isSyncing: health.isSyncing.valueOf(),
          shouldHavePeers: health.shouldHavePeers.valueOf()
        }
      };

    } catch (err) {
      return {
        online: false,
        error: err.message
      };
    }
  }
}

module.exports = { AutoDiscovery };
