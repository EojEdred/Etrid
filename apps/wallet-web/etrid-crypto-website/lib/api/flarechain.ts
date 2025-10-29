/**
 * FlareChain API Wrapper
 *
 * Provides typed interfaces for interacting with FlareChain and all 13 PBCs
 * using Polkadot.js API
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

export interface ChainConfig {
  id: string;
  name: string;
  endpoint: string;
  symbol: string;
  decimals: number;
  type: 'relay' | 'pbc';
}

// Dual Bootstrap Nodes (VM #1 Alice, VM #2 Bob)
export const BOOTSTRAP_NODES = [
  'ws://20.186.91.207:9944', // VM #1 (Alice) - Primary
  'ws://172.177.44.73:9944',  // VM #2 (Bob) - Fallback
];

export const CHAIN_CONFIGS: Record<string, ChainConfig> = {
  flarechain: {
    id: 'flarechain',
    name: 'FlareChain',
    endpoint: 'ws://20.186.91.207:9944', // VM #1 (Alice) - Primary bootstrap node
    symbol: 'ÉTR',
    decimals: 18,
    type: 'relay',
  },
  'btc-pbc': {
    id: 'btc-pbc',
    name: 'Bitcoin PBC',
    endpoint: 'ws://127.0.0.1:8000',
    symbol: 'BTC',
    decimals: 8,
    type: 'pbc',
  },
  'eth-pbc': {
    id: 'eth-pbc',
    name: 'Ethereum PBC',
    endpoint: 'ws://127.0.0.1:8001',
    symbol: 'ETH',
    decimals: 18,
    type: 'pbc',
  },
  'doge-pbc': {
    id: 'doge-pbc',
    name: 'Dogecoin PBC',
    endpoint: 'ws://127.0.0.1:8002',
    symbol: 'DOGE',
    decimals: 8,
    type: 'pbc',
  },
  'sol-pbc': {
    id: 'sol-pbc',
    name: 'Solana PBC',
    endpoint: 'ws://127.0.0.1:8003',
    symbol: 'SOL',
    decimals: 9,
    type: 'pbc',
  },
  'xlm-pbc': {
    id: 'xlm-pbc',
    name: 'Stellar PBC',
    endpoint: 'ws://127.0.0.1:8004',
    symbol: 'XLM',
    decimals: 7,
    type: 'pbc',
  },
  'xrp-pbc': {
    id: 'xrp-pbc',
    name: 'XRP PBC',
    endpoint: 'ws://127.0.0.1:8005',
    symbol: 'XRP',
    decimals: 6,
    type: 'pbc',
  },
  'bnb-pbc': {
    id: 'bnb-pbc',
    name: 'BNB PBC',
    endpoint: 'ws://127.0.0.1:8006',
    symbol: 'BNB',
    decimals: 18,
    type: 'pbc',
  },
  'trx-pbc': {
    id: 'trx-pbc',
    name: 'Tron PBC',
    endpoint: 'ws://127.0.0.1:8007',
    symbol: 'TRX',
    decimals: 6,
    type: 'pbc',
  },
  'ada-pbc': {
    id: 'ada-pbc',
    name: 'Cardano PBC',
    endpoint: 'ws://127.0.0.1:8008',
    symbol: 'ADA',
    decimals: 6,
    type: 'pbc',
  },
  'link-pbc': {
    id: 'link-pbc',
    name: 'Chainlink PBC',
    endpoint: 'ws://127.0.0.1:8009',
    symbol: 'LINK',
    decimals: 18,
    type: 'pbc',
  },
  'matic-pbc': {
    id: 'matic-pbc',
    name: 'Polygon PBC',
    endpoint: 'ws://127.0.0.1:8010',
    symbol: 'MATIC',
    decimals: 18,
    type: 'pbc',
  },
  'sc-usdt-pbc': {
    id: 'sc-usdt-pbc',
    name: 'Tether PBC',
    endpoint: 'ws://127.0.0.1:8011',
    symbol: 'USDT',
    decimals: 6,
    type: 'pbc',
  },
  'edsc-pbc': {
    id: 'edsc-pbc',
    name: 'EDSC PBC',
    endpoint: 'ws://127.0.0.1:8012',
    symbol: 'EDSC',
    decimals: 18,
    type: 'pbc',
  },
};

export class FlareChainAPI {
  private apis: Map<string, ApiPromise> = new Map();
  private providers: Map<string, WsProvider> = new Map();

  /**
   * Connect to a specific chain with automatic failover
   */
  async connect(chainId: string): Promise<ApiPromise> {
    const existing = this.apis.get(chainId);
    if (existing && existing.isConnected) {
      return existing;
    }

    const config = CHAIN_CONFIGS[chainId];
    if (!config) {
      throw new Error(`Unknown chain ID: ${chainId}`);
    }

    // For FlareChain, use bootstrap nodes with failover
    const endpoints = chainId === 'flarechain' ? BOOTSTRAP_NODES : [config.endpoint];

    let lastError: Error | null = null;
    for (const endpoint of endpoints) {
      try {
        console.log(`🔄 Attempting connection to ${config.name} at ${endpoint}...`);
        const provider = new WsProvider(endpoint);
        this.providers.set(chainId, provider);

        const api = await ApiPromise.create({ provider });
        this.apis.set(chainId, api);

        console.log(`✅ Connected to ${config.name} at ${endpoint}`);
        return api;
      } catch (error) {
        console.warn(`⚠️ Failed to connect to ${endpoint}:`, error);
        lastError = error as Error;
        // Try next endpoint
      }
    }

    throw new Error(`Failed to connect to ${config.name}. Tried all endpoints. Last error: ${lastError?.message}`);
  }

  /**
   * Connect to FlareChain
   */
  async connectToFlareChain(): Promise<ApiPromise> {
    return this.connect('flarechain');
  }

  /**
   * Connect to multiple chains in parallel
   */
  async connectMultiple(chainIds: string[]): Promise<Map<string, ApiPromise>> {
    const promises = chainIds.map(id => this.connect(id));
    const apis = await Promise.all(promises);

    const result = new Map<string, ApiPromise>();
    chainIds.forEach((id, index) => {
      result.set(id, apis[index]);
    });

    return result;
  }

  /**
   * Get balance for an address on a specific chain
   */
  async getBalance(chainId: string, address: string): Promise<string> {
    const api = await this.connect(chainId);
    const { data: balance } = await api.query.system.account(address);

    const config = CHAIN_CONFIGS[chainId];
    const free = balance.free.toString();
    const decimals = config.decimals;

    // Convert from smallest unit to token unit
    const balanceNum = parseInt(free) / Math.pow(10, decimals);
    return balanceNum.toFixed(decimals);
  }

  /**
   * Get balances across all chains for an address
   */
  async getMultiChainBalance(address: string): Promise<Record<string, string>> {
    const chainIds = Object.keys(CHAIN_CONFIGS);
    const balancePromises = chainIds.map(async (chainId) => {
      try {
        const balance = await this.getBalance(chainId, address);
        return { chainId, balance };
      } catch (error) {
        console.warn(`Failed to get balance for ${chainId}:`, error);
        return { chainId, balance: '0' };
      }
    });

    const results = await Promise.all(balancePromises);
    const balances: Record<string, string> = {};

    results.forEach(({ chainId, balance }) => {
      balances[chainId] = balance;
    });

    return balances;
  }

  /**
   * Transfer tokens on a specific chain
   */
  async transfer(
    chainId: string,
    from: string,
    to: string,
    amount: string
  ): Promise<string> {
    const api = await this.connect(chainId);
    const config = CHAIN_CONFIGS[chainId];

    // Convert amount to smallest unit
    const amountInSmallestUnit = BigInt(parseFloat(amount) * Math.pow(10, config.decimals));

    // Get injected accounts
    const injector = await web3FromAddress(from);

    // Create transfer
    const transfer = api.tx.balances.transferKeepAlive(to, amountInSmallestUnit);

    // Sign and send
    const hash = await transfer.signAndSend(from, { signer: injector.signer });

    return hash.toString();
  }

  /**
   * Get staking info (FlareChain only)
   */
  async getStakingInfo(address: string): Promise<any> {
    const api = await this.connectToFlareChain();

    // Get validator status
    const validators = await api.query.session.validators();
    const isValidator = validators.some((v) => v.toString() === address);

    // Get staking info
    const stakingInfo = await api.query.staking.ledger(address);

    return {
      isValidator,
      staked: stakingInfo.toHuman(),
    };
  }

  /**
   * Get EDSC-specific info (minting, redemption, etc.)
   */
  async getEdscInfo(address: string): Promise<any> {
    const api = await this.connect('edsc-pbc');

    // Get EDSC balance
    const balance = await this.getBalance('edsc-pbc', address);

    // Get total supply
    const totalSupply = await api.query.edscToken.totalSupply();

    // Get redemption status (if user has pending redemptions)
    const redemptions = await api.query.edscRedemption.userRedemptions(address);

    return {
      balance,
      totalSupply: totalSupply.toString(),
      redemptions: redemptions.toHuman(),
    };
  }

  /**
   * Disconnect from all chains
   */
  async disconnectAll(): Promise<void> {
    for (const [chainId, api] of this.apis) {
      await api.disconnect();
      console.log(`❌ Disconnected from ${CHAIN_CONFIGS[chainId].name}`);
    }

    this.apis.clear();
    this.providers.clear();
  }

  /**
   * Get chain info
   */
  async getChainInfo(chainId: string): Promise<any> {
    const api = await this.connect(chainId);

    const [chain, nodeName, nodeVersion, bestNumber, bestHash] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.system.name(),
      api.rpc.system.version(),
      api.derive.chain.bestNumber(),
      api.rpc.chain.getBlockHash(),
    ]);

    return {
      chain: chain.toString(),
      nodeName: nodeName.toString(),
      nodeVersion: nodeVersion.toString(),
      bestNumber: bestNumber.toString(),
      bestHash: bestHash.toString(),
    };
  }
}

// Singleton instance
export const flarechainApi = new FlareChainAPI();
