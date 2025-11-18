import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import config from '../config';
import logger from '../utils/logger';
import { Balance } from '../types';
import db from '../database/client';

class BlockchainService {
  private api: ApiPromise | null = null;
  private isConnected: boolean = false;

  /**
   * Initialize connection to FlareChain
   */
  async connect(): Promise<void> {
    try {
      const provider = new WsProvider(config.blockchain.wsUrl);

      this.api = await ApiPromise.create({ provider });

      await this.api.isReady;

      this.isConnected = true;
      logger.info('Connected to FlareChain', {
        chain: (await this.api.rpc.system.chain()).toString(),
        version: (await this.api.rpc.system.version()).toString(),
      });

      // Subscribe to new blocks
      this.subscribeToBlocks();
    } catch (error: any) {
      logger.error('Failed to connect to blockchain', { error: error.message });
      throw error;
    }
  }

  /**
   * Subscribe to new blocks for indexing
   */
  private async subscribeToBlocks(): Promise<void> {
    if (!this.api) return;

    await this.api.rpc.chain.subscribeNewHeads(async (header) => {
      logger.info('New block', { number: header.number.toNumber() });

      // Index block transactions
      try {
        await this.indexBlock(header.number.toNumber());
      } catch (error: any) {
        logger.error('Error indexing block', {
          blockNumber: header.number.toNumber(),
          error: error.message,
        });
      }
    });
  }

  /**
   * Index block transactions
   */
  private async indexBlock(blockNumber: number): Promise<void> {
    if (!this.api) return;

    const blockHash = await this.api.rpc.chain.getBlockHash(blockNumber);
    const block = await this.api.rpc.chain.getBlock(blockHash);

    // Process each extrinsic (transaction)
    for (const extrinsic of block.block.extrinsics) {
      const { method: { method, section } } = extrinsic;

      // Only index relevant transactions
      if (section === 'balances' || section === 'staking' || section === 'democracy') {
        try {
          const txHash = extrinsic.hash.toHex();

          // Check if already indexed
          const existing = await db.query(
            'SELECT id FROM transactions WHERE tx_hash = $1',
            [txHash]
          );

          if (existing.rows.length === 0) {
            // Parse and store transaction
            // This is a simplified version - real implementation would be more complex
            logger.info('New transaction to index', { txHash, method: `${section}.${method}` });
          }
        } catch (error: any) {
          logger.error('Error processing extrinsic', { error: error.message });
        }
      }
    }
  }

  /**
   * Get account balance
   */
  async getBalance(address: string): Promise<Balance> {
    if (!this.api) {
      throw new Error('Blockchain not connected');
    }

    try {
      const account = await this.api.query.system.account(address);
      const balance = account.data;

      return {
        asset: 'ETR',
        free: balance.free.toString(),
        reserved: balance.reserved.toString(),
        total: balance.free.add(balance.reserved).toString(),
        locked: balance.frozen?.toString() || '0',
      };
    } catch (error: any) {
      logger.error('Error getting balance', { address, error: error.message });
      throw error;
    }
  }

  /**
   * Get complete portfolio (all assets)
   */
  async getPortfolio(address: string): Promise<any> {
    // Get native balance
    const nativeBalance = await this.getBalance(address);

    // Get other assets (from bridge, staking, etc.)
    const stakingResult = await db.query(
      `SELECT
         COALESCE(SUM(amount), 0) as staked,
         COALESCE(SUM(rewards_earned), 0) as rewards
       FROM staking_positions
       WHERE user_id = (SELECT id FROM users WHERE address = $1)
         AND status = 'active'`,
      [address]
    );

    return {
      native: nativeBalance,
      staking: {
        staked: stakingResult.rows[0].staked,
        rewards: stakingResult.rows[0].rewards,
      },
      // Add other assets (BTC, ETH, etc.) from bridge
      bridged_assets: [],
    };
  }

  /**
   * Submit transfer transaction
   */
  async submitTransfer(params: {
    from: string;
    to: string;
    amount: string;
    asset: string;
    memo?: string;
  }): Promise<string> {
    if (!this.api) {
      throw new Error('Blockchain not connected');
    }

    // This is a placeholder - in production, the transaction would be signed client-side
    // and submitted as a signed extrinsic
    const mockTxHash = `0x${Math.random().toString(16).substring(2, 66)}`;

    logger.info('Transfer submitted', {
      from: params.from,
      to: params.to,
      amount: params.amount,
      txHash: mockTxHash,
    });

    return mockTxHash;
  }

  /**
   * Submit stake transaction
   */
  async submitStake(params: {
    from: string;
    validator: string;
    amount: string;
  }): Promise<string> {
    if (!this.api) {
      throw new Error('Blockchain not connected');
    }

    // Placeholder - real implementation would submit to chain
    const mockTxHash = `0x${Math.random().toString(16).substring(2, 66)}`;

    logger.info('Stake submitted', {
      from: params.from,
      validator: params.validator,
      amount: params.amount,
      txHash: mockTxHash,
    });

    return mockTxHash;
  }

  /**
   * Submit unstake transaction
   */
  async submitUnstake(params: {
    from: string;
    validator: string;
    amount: string;
  }): Promise<string> {
    if (!this.api) {
      throw new Error('Blockchain not connected');
    }

    const mockTxHash = `0x${Math.random().toString(16).substring(2, 66)}`;

    logger.info('Unstake submitted', {
      from: params.from,
      validator: params.validator,
      amount: params.amount,
      txHash: mockTxHash,
    });

    return mockTxHash;
  }

  /**
   * Submit governance vote
   */
  async submitVote(params: {
    proposalId: number;
    support: boolean;
    conviction: number;
    balance: string;
  }): Promise<string> {
    if (!this.api) {
      throw new Error('Blockchain not connected');
    }

    const mockTxHash = `0x${Math.random().toString(16).substring(2, 66)}`;

    logger.info('Vote submitted', {
      proposalId: params.proposalId,
      support: params.support,
      conviction: params.conviction,
      txHash: mockTxHash,
    });

    return mockTxHash;
  }

  /**
   * Get current block number
   */
  async getCurrentBlock(): Promise<number> {
    if (!this.api) {
      throw new Error('Blockchain not connected');
    }

    const header = await this.api.rpc.chain.getHeader();
    return header.number.toNumber();
  }

  /**
   * Disconnect from blockchain
   */
  async disconnect(): Promise<void> {
    if (this.api) {
      await this.api.disconnect();
      this.isConnected = false;
      logger.info('Disconnected from blockchain');
    }
  }
}

export default new BlockchainService();
