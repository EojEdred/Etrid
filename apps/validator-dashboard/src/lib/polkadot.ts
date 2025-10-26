import { ApiPromise, WsProvider } from '@polkadot/api';

/**
 * Polkadot API Connection Utilities
 *
 * This module provides connection management and helper functions
 * for interacting with the Ëtrid blockchain via Polkadot.js API.
 */

export interface ChainConfig {
  name: string;
  wsEndpoint: string;
  decimals: number;
  symbol: string;
}

// Default Ëtrid chain configuration
export const ETRID_CHAIN: ChainConfig = {
  name: 'Ëtrid',
  wsEndpoint: process.env.NEXT_PUBLIC_WS_PROVIDER || 'ws://localhost:9944',
  decimals: 18,
  symbol: 'ETR',
};

let apiInstance: ApiPromise | null = null;
let connectionPromise: Promise<ApiPromise> | null = null;

/**
 * Get or create a Polkadot API connection
 *
 * @param wsEndpoint - WebSocket endpoint (defaults to env variable)
 * @returns Promise resolving to ApiPromise instance
 */
export async function getApi(wsEndpoint?: string): Promise<ApiPromise> {
  const endpoint = wsEndpoint || ETRID_CHAIN.wsEndpoint;

  // Return existing instance if connected to same endpoint
  if (apiInstance && apiInstance.isConnected) {
    const currentEndpoint = (apiInstance as any)._options?.provider?.endpoint;
    if (currentEndpoint === endpoint) {
      return apiInstance;
    }
  }

  // Return existing connection promise if one is in progress
  if (connectionPromise) {
    return connectionPromise;
  }

  // Create new connection
  connectionPromise = (async () => {
    try {
      const provider = new WsProvider(endpoint);
      const api = await ApiPromise.create({ provider });

      // Wait for API to be ready
      await api.isReady;

      apiInstance = api;
      connectionPromise = null;

      return api;
    } catch (error) {
      connectionPromise = null;
      throw error;
    }
  })();

  return connectionPromise;
}

/**
 * Disconnect from the Polkadot API
 */
export async function disconnectApi(): Promise<void> {
  if (apiInstance) {
    await apiInstance.disconnect();
    apiInstance = null;
  }
}

/**
 * Check if API is connected
 */
export function isConnected(): boolean {
  return apiInstance?.isConnected || false;
}

/**
 * Format balance from chain units to human-readable format
 *
 * @param balance - Balance in smallest unit (e.g., Wei)
 * @param decimals - Number of decimal places (default: 18)
 * @returns Formatted balance string
 */
export function formatBalance(balance: bigint | string | number, decimals: number = 18): string {
  const balanceBigInt = typeof balance === 'bigint' ? balance : BigInt(balance);
  const divisor = BigInt(10 ** decimals);
  const whole = balanceBigInt / divisor;
  const remainder = balanceBigInt % divisor;

  const fractionStr = remainder.toString().padStart(decimals, '0');
  const trimmedFraction = fractionStr.replace(/0+$/, '').slice(0, 4);

  return trimmedFraction
    ? `${whole}.${trimmedFraction}`
    : whole.toString();
}

/**
 * Parse human-readable balance to chain units
 *
 * @param balance - Human-readable balance (e.g., "10.5")
 * @param decimals - Number of decimal places (default: 18)
 * @returns Balance in smallest unit
 */
export function parseBalance(balance: string, decimals: number = 18): bigint {
  const [whole, fraction = ''] = balance.split('.');
  const paddedFraction = fraction.padEnd(decimals, '0').slice(0, decimals);
  return BigInt(whole + paddedFraction);
}

/**
 * Get current block number
 */
export async function getCurrentBlock(): Promise<number> {
  const api = await getApi();
  const header = await api.rpc.chain.getHeader();
  return header.number.toNumber();
}

/**
 * Subscribe to new blocks
 *
 * @param callback - Function to call on each new block
 * @returns Unsubscribe function
 */
export async function subscribeToBlocks(
  callback: (blockNumber: number) => void
): Promise<() => void> {
  const api = await getApi();
  const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
    callback(header.number.toNumber());
  });
  return unsubscribe;
}

/**
 * Get account balance
 *
 * @param address - Account address
 * @returns Balance information
 */
export async function getBalance(address: string) {
  const api = await getApi();
  const account = await api.query.system.account(address);
  const accountData = account.toJSON() as any;
  const balance = accountData?.data || {};

  return {
    free: BigInt(balance.free || 0),
    reserved: BigInt(balance.reserved || 0),
    frozen: BigInt(balance.frozen || 0),
    total: BigInt(balance.free || 0) + BigInt(balance.reserved || 0),
  };
}

/**
 * Get validator information
 *
 * @param validatorAddress - Validator stash address
 * @returns Validator details
 */
export async function getValidatorInfo(validatorAddress: string) {
  const api = await getApi();

  const [
    ledger,
    prefs,
    sessionKeys,
    currentEra,
  ] = await Promise.all([
    api.query.staking.ledger(validatorAddress),
    api.query.staking.validators(validatorAddress),
    api.query.session.nextKeys(validatorAddress),
    api.query.staking.currentEra(),
  ]);

  const currentEraData = currentEra.toJSON() as any;
  const exposure = await api.query.staking.erasStakers(
    currentEraData,
    validatorAddress
  );

  // Convert all Codec types to JSON for type safety
  const ledgerData = ledger.toJSON() as any;
  const prefsData = prefs.toJSON() as any;
  const sessionKeysData = sessionKeys.toJSON() as any;
  const exposureData = exposure.toJSON() as any;

  return {
    stash: validatorAddress,
    controller: ledgerData?.controller || '',
    sessionKeys: sessionKeysData || '',
    commission: prefsData?.commission ? Number(prefsData.commission) / 10_000_000 : 0,
    blocked: prefsData?.blocked || false,
    totalStake: BigInt(exposureData?.total || 0),
    ownStake: BigInt(exposureData?.own || 0),
    nominatorCount: exposureData?.others?.length || 0,
    nominators: (exposureData?.others || []).map((nom: any) => ({
      address: nom.who || '',
      stake: BigInt(nom.value || 0),
    })),
  };
}

/**
 * Get current era information
 */
export async function getCurrentEra() {
  const api = await getApi();
  const currentEra = await api.query.staking.currentEra();
  return Number(currentEra.toJSON() || 0);
}

/**
 * Get era rewards for a validator
 *
 * @param validatorAddress - Validator stash address
 * @param eraIndex - Era number
 * @returns Era reward information
 */
export async function getEraReward(validatorAddress: string, eraIndex: number) {
  const api = await getApi();

  const [eraReward, eraPoints] = await Promise.all([
    api.query.staking.erasValidatorReward(eraIndex),
    api.query.staking.erasRewardPoints(eraIndex),
  ]);

  const eraRewardData = eraReward.toJSON() as any;

  if (!eraRewardData) {
    return null;
  }

  const eraPointsData = eraPoints.toJSON() as any;
  const validatorPoints = Number(eraPointsData?.individual?.[validatorAddress] || 0);
  const totalPoints = Number(eraPointsData?.total || 0);
  const totalReward = BigInt(eraRewardData || 0);

  // Calculate validator's share
  const validatorReward = totalPoints > 0
    ? (totalReward * BigInt(validatorPoints)) / BigInt(totalPoints)
    : BigInt(0);

  return {
    era: eraIndex,
    totalReward,
    validatorPoints,
    totalPoints,
    validatorReward,
  };
}

/**
 * Get network statistics
 */
export async function getNetworkStats() {
  const api = await getApi();

  const [
    totalIssuance,
    currentEra,
    validatorCount,
    minNominatorBond,
    minValidatorBond,
  ] = await Promise.all([
    api.query.balances.totalIssuance(),
    api.query.staking.currentEra(),
    api.query.staking.validatorCount(),
    api.query.staking.minNominatorBond(),
    api.query.staking.minValidatorBond(),
  ]);

  const currentEraData = currentEra.toJSON() as any;
  const totalStake = await api.query.staking.erasTotalStake(currentEraData);

  // Convert all to JSON for type safety
  const totalIssuanceData = BigInt(totalIssuance.toJSON() as any || 0);
  const totalStakeData = BigInt(totalStake.toJSON() as any || 0);
  const validatorCountData = Number(validatorCount.toJSON() || 0);
  const minNominatorBondData = BigInt(minNominatorBond.toJSON() as any || 0);
  const minValidatorBondData = BigInt(minValidatorBond.toJSON() as any || 0);

  return {
    totalIssuance: totalIssuanceData,
    totalStaked: totalStakeData,
    stakingRate: Number(totalStakeData) / Number(totalIssuanceData),
    activeValidators: validatorCountData,
    minNominatorBond: minNominatorBondData,
    minValidatorBond: minValidatorBondData,
  };
}

/**
 * Health check for API connection
 *
 * @returns Connection status and chain info
 */
export async function healthCheck() {
  try {
    const api = await getApi();
    const [chain, nodeName, nodeVersion, health] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.system.name(),
      api.rpc.system.version(),
      api.rpc.system.health(),
    ]);

    return {
      connected: true,
      chain: chain.toString(),
      nodeName: nodeName.toString(),
      nodeVersion: nodeVersion.toString(),
      peers: health.peers.toNumber(),
      isSyncing: health.isSyncing.isTrue,
    };
  } catch (error) {
    return {
      connected: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

export default {
  getApi,
  disconnectApi,
  isConnected,
  formatBalance,
  parseBalance,
  getCurrentBlock,
  subscribeToBlocks,
  getBalance,
  getValidatorInfo,
  getCurrentEra,
  getEraReward,
  getNetworkStats,
  healthCheck,
};
