/**
 * Multi-PBC Chain Configuration for ËTRID Network
 *
 * ËTRID uses a Relay Chain (Primearc Core) + 12 Partition Burst Chains (PBCs)
 * Each PBC handles specific asset bridging and transactions
 */

export interface ChainConfig {
  /** Unique chain identifier */
  id: string;
  /** Display name for the chain */
  name: string;
  /** Native token symbol */
  token: string;
  /** Token decimals for formatting */
  decimals: number;
  /** WebSocket RPC endpoint */
  wsEndpoint: string;
  /** HTTP RPC endpoint (fallback) */
  httpEndpoint?: string;
  /** Is this the relay chain (Primearc Core)? */
  isRelay: boolean;
  /** Chain icon color for UI */
  color: string;
  /** Short description */
  description: string;
  /** RPC port for easy access */
  port?: number;
}

/**
 * Primearc Core Chain - ËTRID Relay Chain
 * Handles native ETR token and coordinates all PBCs
 */
export const PRIMEARC_CORE: ChainConfig = {
  id: 'primearc-core',
  name: 'Primearc Core',
  token: 'ETR',
  decimals: 12,
  // Use the stable public RPC by default (the `ws.etrid.org` hostname may not resolve
  // in some desktop/Tauri environments).
  wsEndpoint: 'wss://rpc.etrid.org',
  httpEndpoint: 'https://rpc.etrid.org',
  isRelay: true,
  color: '#66D9E6',
  description: 'ËTRID Relay Chain - Native ETR token',
};

/**
 * Partition Burst Chains (PBCs)
 * Each PBC is a specialized parachain for bridging specific assets
 */
export const PBC_CHAINS: ChainConfig[] = [
  {
    id: 'xrp-pbc',
    name: 'XRP-PBC',
    token: 'bXRP',
    decimals: 6,
    wsEndpoint: 'wss://rpc.etrid.org:9945',
    httpEndpoint: 'https://rpc.etrid.org:9945',
    isRelay: false,
    color: '#23292F',
    description: 'Bridged XRP from Ripple network',
    port: 9945,
  },
  {
    id: 'btc-pbc',
    name: 'BTC-PBC',
    token: 'bBTC',
    decimals: 8,
    wsEndpoint: 'wss://rpc.etrid.org:9946',
    httpEndpoint: 'https://rpc.etrid.org:9946',
    isRelay: false,
    color: '#F7931A',
    description: 'Bridged Bitcoin',
    port: 9946,
  },
  {
    id: 'ada-pbc',
    name: 'ADA-PBC',
    token: 'bADA',
    decimals: 6,
    wsEndpoint: 'wss://rpc.etrid.org:9947',
    httpEndpoint: 'https://rpc.etrid.org:9947',
    isRelay: false,
    color: '#0033AD',
    description: 'Bridged Cardano',
    port: 9947,
  },
  {
    id: 'doge-pbc',
    name: 'DOGE-PBC',
    token: 'bDOGE',
    decimals: 8,
    wsEndpoint: 'wss://rpc.etrid.org:9948',
    httpEndpoint: 'https://rpc.etrid.org:9948',
    isRelay: false,
    color: '#C2A633',
    description: 'Bridged Dogecoin',
    port: 9948,
  },
  {
    id: 'trx-pbc',
    name: 'TRX-PBC',
    token: 'bTRX',
    decimals: 6,
    wsEndpoint: 'wss://rpc.etrid.org:9949',
    httpEndpoint: 'https://rpc.etrid.org:9949',
    isRelay: false,
    color: '#FF060A',
    description: 'Bridged TRON',
    port: 9949,
  },
  {
    id: 'matic-pbc',
    name: 'MATIC-PBC',
    token: 'bMATIC',
    decimals: 18,
    wsEndpoint: 'wss://rpc.etrid.org:9950',
    httpEndpoint: 'https://rpc.etrid.org:9950',
    isRelay: false,
    color: '#8247E5',
    description: 'Bridged Polygon',
    port: 9950,
  },
  {
    id: 'bnb-pbc',
    name: 'BNB-PBC',
    token: 'bBNB',
    decimals: 18,
    wsEndpoint: 'wss://rpc.etrid.org:9951',
    httpEndpoint: 'https://rpc.etrid.org:9951',
    isRelay: false,
    color: '#F3BA2F',
    description: 'Bridged BNB from BSC',
    port: 9951,
  },
  {
    id: 'link-pbc',
    name: 'LINK-PBC',
    token: 'bLINK',
    decimals: 18,
    wsEndpoint: 'wss://rpc.etrid.org:9952',
    httpEndpoint: 'https://rpc.etrid.org:9952',
    isRelay: false,
    color: '#2A5ADA',
    description: 'Bridged Chainlink',
    port: 9952,
  },
  {
    id: 'scusdt-pbc',
    name: 'SC-USDT-PBC',
    token: 'scUSDT',
    decimals: 6,
    wsEndpoint: 'wss://rpc.etrid.org:9953',
    httpEndpoint: 'https://rpc.etrid.org:9953',
    isRelay: false,
    color: '#26A17B',
    description: 'Stablecoin USDT',
    port: 9953,
  },
  {
    id: 'edsc-pbc',
    name: 'EDSC-PBC',
    token: 'EDSC',
    decimals: 18,
    wsEndpoint: 'wss://rpc.etrid.org:9954',
    httpEndpoint: 'https://rpc.etrid.org:9954',
    isRelay: false,
    color: '#4A90E2',
    description: 'ËTRID DeFi Stablecoin',
    port: 9954,
  },
  {
    id: 'sol-pbc',
    name: 'SOL-PBC',
    token: 'bSOL',
    decimals: 9,
    wsEndpoint: 'wss://rpc.etrid.org:9955',
    httpEndpoint: 'https://rpc.etrid.org:9955',
    isRelay: false,
    color: '#14F195',
    description: 'Bridged Solana',
    port: 9955,
  },
  {
    id: 'xlm-pbc',
    name: 'XLM-PBC',
    token: 'bXLM',
    decimals: 7,
    wsEndpoint: 'wss://rpc.etrid.org:9956',
    httpEndpoint: 'https://rpc.etrid.org:9956',
    isRelay: false,
    color: '#14B6E7',
    description: 'Bridged Stellar Lumens',
    port: 9956,
  },
];

/**
 * All available chains (Relay + PBCs)
 */
export const ALL_CHAINS: ChainConfig[] = [PRIMEARC_CORE, ...PBC_CHAINS];

/**
 * Get chain configuration by ID
 * @param chainId - The chain identifier
 * @returns Chain configuration or undefined if not found
 */
export function getChainConfig(chainId: string): ChainConfig | undefined {
  return ALL_CHAINS.find((chain) => chain.id === chainId);
}

/**
 * Get chain configuration by token symbol
 * @param tokenSymbol - The token symbol (e.g., 'ETR', 'bBTC')
 * @returns Chain configuration or undefined if not found
 */
export function getChainByToken(tokenSymbol: string): ChainConfig | undefined {
  return ALL_CHAINS.find((chain) => chain.token === tokenSymbol);
}

/**
 * Get all PBC chains (excluding relay)
 */
export function getPBCChains(): ChainConfig[] {
  return PBC_CHAINS;
}

/**
 * Check if a chain ID is valid
 */
export function isValidChainId(chainId: string): boolean {
  return ALL_CHAINS.some((chain) => chain.id === chainId);
}

/**
 * Format token amount with proper decimals
 * @param amount - Raw token amount as string or bigint
 * @param decimals - Token decimals
 * @returns Formatted amount as string
 */
export function formatTokenAmount(
  amount: string | bigint,
  decimals: number
): string {
  const amountBigInt = typeof amount === 'string' ? BigInt(amount) : amount;
  const divisor = BigInt(10 ** decimals);
  const wholePart = amountBigInt / divisor;
  const fractionalPart = amountBigInt % divisor;

  if (fractionalPart === BigInt(0)) {
    return wholePart.toString();
  }

  const fractionalStr = fractionalPart.toString().padStart(decimals, '0');
  const trimmedFractional = fractionalStr.replace(/0+$/, '');

  return `${wholePart}.${trimmedFractional}`;
}

/**
 * Parse token amount to raw units
 * @param amount - Human-readable amount (e.g., "1.5")
 * @param decimals - Token decimals
 * @returns Raw amount as bigint
 */
export function parseTokenAmount(amount: string, decimals: number): bigint {
  const [whole = '0', fractional = '0'] = amount.split('.');
  const fractionalPadded = fractional.padEnd(decimals, '0').slice(0, decimals);
  const rawAmount = whole + fractionalPadded;
  return BigInt(rawAmount);
}
