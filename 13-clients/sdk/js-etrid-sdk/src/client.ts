/**
 * RPC client for connecting to Ëtrid blockchain nodes
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Balance } from './types';

/**
 * Ëtrid blockchain client
 */
export class EtridClient {
  private api?: ApiPromise;
  private provider: WsProvider;

  /**
   * Create a new client instance
   * @param endpoint - WebSocket endpoint (e.g., 'ws://localhost:9944')
   */
  constructor(private endpoint: string) {
    this.provider = new WsProvider(endpoint);
  }

  /**
   * Connect to the blockchain
   */
  async connect(): Promise<void> {
    this.api = await ApiPromise.create({ provider: this.provider });
  }

  /**
   * Disconnect from the blockchain
   */
  async disconnect(): Promise<void> {
    await this.api?.disconnect();
  }

  /**
   * Get the current block number
   */
  async getBlockNumber(): Promise<number> {
    if (!this.api) throw new Error('Not connected');
    const header = await this.api.rpc.chain.getHeader();
    return header.number.toNumber();
  }

  /**
   * Get account balance
   */
  async getBalance(address: string): Promise<Balance> {
    if (!this.api) throw new Error('Not connected');
    const account = await this.api.query.system.account(address);
    return {
      free: account.data.free.toBigInt(),
      reserved: account.data.reserved.toBigInt(),
      frozen: account.data.frozen.toBigInt(),
    };
  }

  /**
   * Get the chain name
   */
  async getChainName(): Promise<string> {
    if (!this.api) throw new Error('Not connected');
    return (await this.api.rpc.system.chain()).toString();
  }

  /**
   * Query interface
   */
  get query() {
    return {
      balance: this.getBalance.bind(this),
      blockNumber: this.getBlockNumber.bind(this),
    };
  }
}
