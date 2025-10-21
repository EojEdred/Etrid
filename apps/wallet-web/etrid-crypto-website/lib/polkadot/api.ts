/**
 * Polkadot.js API Connection Manager
 * Handles connections to FlareChain and all PBCs
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import type { ChainId, ChainConfig } from './chains';
import { CHAINS } from './chains';

// API instance cache
const apiInstances: Map<ChainId, ApiPromise> = new Map();
const connectionPromises: Map<ChainId, Promise<ApiPromise>> = new Map();

/**
 * Create or retrieve cached API instance for a specific chain
 */
export async function createApi(chainId: ChainId): Promise<ApiPromise> {
  // Return cached instance if exists and connected
  const existing = apiInstances.get(chainId);
  if (existing && existing.isConnected) {
    return existing;
  }

  // Return in-progress connection if exists
  const pending = connectionPromises.get(chainId);
  if (pending) {
    return pending;
  }

  // Create new connection
  const config = CHAINS[chainId];
  const promise = connectToChain(config);
  connectionPromises.set(chainId, promise);

  try {
    const api = await promise;
    apiInstances.set(chainId, api);
    connectionPromises.delete(chainId);
    return api;
  } catch (error) {
    connectionPromises.delete(chainId);
    throw error;
  }
}

/**
 * Internal: Connect to a chain via WebSocket
 */
async function connectToChain(config: ChainConfig): Promise<ApiPromise> {
  const provider = new WsProvider(config.rpc);

  const api = await ApiPromise.create({
    provider,
    throwOnConnect: false,
  });

  await api.isReady;

  console.log(`[Polkadot] Connected to ${config.name} (${config.symbol})`);

  return api;
}

/**
 * Disconnect from a specific chain
 */
export async function disconnectApi(chainId: ChainId): Promise<void> {
  const api = apiInstances.get(chainId);
  if (api) {
    await api.disconnect();
    apiInstances.delete(chainId);
    console.log(`[Polkadot] Disconnected from ${chainId}`);
  }
}

/**
 * Disconnect from all chains
 */
export async function disconnectAll(): Promise<void> {
  const promises = Array.from(apiInstances.keys()).map((chainId) =>
    disconnectApi(chainId)
  );
  await Promise.all(promises);
}

/**
 * Check if connected to a specific chain
 */
export function isConnected(chainId: ChainId): boolean {
  const api = apiInstances.get(chainId);
  return api ? api.isConnected : false;
}

/**
 * Get chain info (name, version, etc.)
 */
export async function getChainInfo(chainId: ChainId) {
  const api = await createApi(chainId);
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version(),
  ]);

  return {
    chain: chain.toString(),
    nodeName: nodeName.toString(),
    nodeVersion: nodeVersion.toString(),
  };
}

/**
 * Get account balance for a specific chain
 */
export async function getBalance(
  chainId: ChainId,
  address: string
): Promise<bigint> {
  const api = await createApi(chainId);
  const account = await api.query.system.account(address);
  return BigInt(account.data.free.toString());
}

/**
 * Format balance with proper decimals
 */
export function formatBalance(balance: bigint, decimals: number): string {
  const divisor = BigInt(10 ** decimals);
  const whole = balance / divisor;
  const fraction = balance % divisor;

  const fractionStr = fraction.toString().padStart(decimals, '0');
  const trimmed = fractionStr.replace(/0+$/, '');

  if (trimmed === '') {
    return whole.toString();
  }

  return `${whole}.${trimmed}`;
}

/**
 * Parse formatted balance string to bigint
 */
export function parseBalance(amount: string, decimals: number): bigint {
  const [whole = '0', fraction = '0'] = amount.split('.');
  const paddedFraction = fraction.padEnd(decimals, '0').slice(0, decimals);
  const value = BigInt(whole + paddedFraction);
  return value;
}

/**
 * Subscribe to balance changes
 */
export async function subscribeBalance(
  chainId: ChainId,
  address: string,
  callback: (balance: bigint) => void
): Promise<() => void> {
  const api = await createApi(chainId);

  const unsubscribe = await api.query.system.account(
    address,
    (account) => {
      const balance = BigInt(account.data.free.toString());
      callback(balance);
    }
  );

  return () => unsubscribe();
}

/**
 * Get latest block number
 */
export async function getBlockNumber(chainId: ChainId): Promise<number> {
  const api = await createApi(chainId);
  const header = await api.rpc.chain.getHeader();
  return header.number.toNumber();
}

/**
 * Subscribe to new blocks
 */
export async function subscribeNewBlocks(
  chainId: ChainId,
  callback: (blockNumber: number) => void
): Promise<() => void> {
  const api = await createApi(chainId);

  const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
    callback(header.number.toNumber());
  });

  return () => unsubscribe();
}
