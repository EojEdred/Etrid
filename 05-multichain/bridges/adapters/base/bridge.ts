/**
 * Base L2 Bridge Adapter
 *
 * Monitors Ëtrid Primearc Core Chain for lock events and mints tokens on Base.
 * Monitors Base for burn events and releases tokens on Ëtrid.
 */

import { ethers } from 'ethers';
import { ApiPromise, WsProvider } from '@polkadot/api';

interface BridgeConfig {
  etridWsUrl: string;
  baseRpcUrl: string;
  etrTokenAddress: string;
  edscTokenAddress: string;
  bridgePrivateKey: string;
}

interface LockEvent {
  user: string;
  amount: bigint;
  token: 'ETR' | 'EDSC';
  baseAddress: string;
  txHash: string;
}

interface BurnEvent {
  user: string;
  amount: bigint;
  token: 'ETR' | 'EDSC';
  etridAddress: string;
  txHash: string;
}

class BaseBridgeAdapter {
  private etridApi: ApiPromise | null = null;
  private baseProvider: ethers.JsonRpcProvider;
  private baseSigner: ethers.Wallet;
  private etrContract: ethers.Contract;
  private edscContract: ethers.Contract;

  constructor(private config: BridgeConfig) {
    this.baseProvider = new ethers.JsonRpcProvider(config.baseRpcUrl);
    this.baseSigner = new ethers.Wallet(config.bridgePrivateKey, this.baseProvider);

    // Load contract ABIs (simplified for demo)
    const tokenAbi = [
      "function bridgeMint(address to, uint256 amount, bytes32 txHash) external",
      "event BridgeBurn(address indexed from, uint256 amount, string etridAddress)"
    ];

    this.etrContract = new ethers.Contract(
      config.etrTokenAddress,
      tokenAbi,
      this.baseSigner
    );

    this.edscContract = new ethers.Contract(
      config.edscTokenAddress,
      tokenAbi,
      this.baseSigner
    );
  }

  /**
   * Start bridge adapter
   */
  async start() {
    console.log('🌉 Starting Base Bridge Adapter...');

    // Connect to Ëtrid
    const provider = new WsProvider(this.config.etridWsUrl);
    this.etridApi = await ApiPromise.create({ provider });
    console.log('✅ Connected to Ëtrid Primearc Core Chain');

    // Connect to Base
    const network = await this.baseProvider.getNetwork();
    console.log(`✅ Connected to Base (Chain ID: ${network.chainId})`);

    // Start monitoring both chains
    await Promise.all([
      this.monitorEtridLocks(),
      this.monitorBaseBurns()
    ]);
  }

  /**
   * Monitor Ëtrid for lock events → Mint on Base
   */
  private async monitorEtridLocks() {
    console.log('👀 Monitoring Ëtrid for lock events...');

    if (!this.etridApi) throw new Error('Ëtrid API not connected');

    // Subscribe to new blocks
    await this.etridApi.rpc.chain.subscribeNewHeads(async (header) => {
      const blockHash = header.hash.toString();
      const blockNumber = header.number.toNumber();

      // Get events for this block
      const events = await this.etridApi!.query.system.events.at(blockHash);

      for (const record of events) {
        const { event } = record;

        // Check for bridge lock events
        if (event.section === 'baseBridge' && event.method === 'TokensLocked') {
          const [user, token, amount, baseAddress, txHash] = event.data as any;

          const lockEvent: LockEvent = {
            user: user.toString(),
            amount: BigInt(amount.toString()),
            token: token.toString() as 'ETR' | 'EDSC',
            baseAddress: baseAddress.toString(),
            txHash: txHash.toString()
          };

          console.log(`🔒 Lock detected: ${lockEvent.amount} ${lockEvent.token}`);
          await this.handleLock(lockEvent);
        }
      }
    });
  }

  /**
   * Handle lock event: Mint tokens on Base
   */
  private async handleLock(event: LockEvent) {
    try {
      const contract = event.token === 'ETR' ? this.etrContract : this.edscContract;

      console.log(`🌉 Minting ${event.amount} ${event.token} on Base...`);

      const tx = await contract.bridgeMint(
        event.baseAddress,
        event.amount,
        ethers.toBeHex(event.txHash, 32)
      );

      const receipt = await tx.wait();
      console.log(`✅ Minted! TX: ${receipt.hash}`);

    } catch (error) {
      console.error('❌ Mint failed:', error);
      // TODO: Add retry logic or dead letter queue
    }
  }

  /**
   * Monitor Base for burn events → Release on Ëtrid
   */
  private async monitorBaseBurns() {
    console.log('👀 Monitoring Base for burn events...');

    // Listen to ETR burns
    this.etrContract.on('BridgeBurn', async (from, amount, etridAddress) => {
      const burnEvent: BurnEvent = {
        user: from,
        amount: BigInt(amount.toString()),
        token: 'ETR',
        etridAddress,
        txHash: '' // Will be filled from transaction context
      };

      console.log(`🔥 Burn detected: ${amount} ETR`);
      await this.handleBurn(burnEvent);
    });

    // Listen to EDSC burns
    this.edscContract.on('BridgeBurn', async (from, amount, etridAddress) => {
      const burnEvent: BurnEvent = {
        user: from,
        amount: BigInt(amount.toString()),
        token: 'EDSC',
        etridAddress,
        txHash: ''
      };

      console.log(`🔥 Burn detected: ${amount} EDSC`);
      await this.handleBurn(burnEvent);
    });
  }

  /**
   * Handle burn event: Release tokens on Ëtrid
   */
  private async handleBurn(event: BurnEvent) {
    try {
      if (!this.etridApi) throw new Error('Ëtrid API not connected');

      console.log(`🌉 Releasing ${event.amount} ${event.token} on Ëtrid...`);

      // Submit extrinsic to release tokens on Ëtrid
      // NOTE: This requires bridge pallet integration
      // const tx = await this.etridApi.tx.baseBridge.releaseTokens(
      //   event.etridAddress,
      //   event.token,
      //   event.amount,
      //   event.txHash
      // ).signAndSend(bridgeAccount);

      console.log('✅ Released on Ëtrid');

    } catch (error) {
      console.error('❌ Release failed:', error);
    }
  }

  /**
   * Stop bridge adapter
   */
  async stop() {
    console.log('⏹️  Stopping Base Bridge Adapter...');
    
    if (this.etridApi) {
      await this.etridApi.disconnect();
    }

    this.etrContract.removeAllListeners();
    this.edscContract.removeAllListeners();

    console.log('✅ Stopped');
  }
}

// Export
export default BaseBridgeAdapter;

// CLI entry point
if (require.main === module) {
  const config: BridgeConfig = {
    etridWsUrl: process.env.ETRID_WS_URL || 'ws://localhost:9944',
    baseRpcUrl: process.env.BASE_RPC_URL || 'https://mainnet.base.org',
    etrTokenAddress: process.env.ETR_BASE_ADDRESS || '',
    edscTokenAddress: process.env.EDSC_BASE_ADDRESS || '',
    bridgePrivateKey: process.env.BRIDGE_PRIVATE_KEY || ''
  };

  const adapter = new BaseBridgeAdapter(config);
  
  adapter.start().catch((error) => {
    console.error('❌ Bridge adapter error:', error);
    process.exit(1);
  });

  // Graceful shutdown
  process.on('SIGINT', async () => {
    await adapter.stop();
    process.exit(0);
  });
}
