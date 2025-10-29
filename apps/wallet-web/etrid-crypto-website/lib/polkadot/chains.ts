/**
 * Ëtrid Multichain Configuration
 * FlareChain (relay) + 12 Partition Burst Chains (PBCs)
 */

export type ChainId =
  | 'flarechain'
  | 'btc-pbc'
  | 'eth-pbc'
  | 'doge-pbc'
  | 'sol-pbc'
  | 'xlm-pbc'
  | 'xrp-pbc'
  | 'bnb-pbc'
  | 'trx-pbc'
  | 'ada-pbc'
  | 'link-pbc'
  | 'matic-pbc'
  | 'sc-usdt-pbc'
  | 'edsc-pbc';

export interface ChainConfig {
  id: ChainId;
  name: string;
  symbol: string;
  decimals: number;
  rpc: string;
  isRelay: boolean;
  color: string;
  icon?: string;
}

export const CHAINS: Record<ChainId, ChainConfig> = {
  flarechain: {
    id: 'flarechain',
    name: 'FlareChain',
    symbol: 'ÉTR',
    decimals: 12,
    rpc: 'ws://20.186.91.207:9944', // VM #1 (Alice) - Primary bootstrap node
    isRelay: true,
    color: '#FF6B35',
  },
  'btc-pbc': {
    id: 'btc-pbc',
    name: 'Bitcoin PBC',
    symbol: 'BTC',
    decimals: 8,
    rpc: 'ws://localhost:9945',
    isRelay: false,
    color: '#F7931A',
  },
  'eth-pbc': {
    id: 'eth-pbc',
    name: 'Ethereum PBC',
    symbol: 'ETH',
    decimals: 18,
    rpc: 'ws://localhost:9946',
    isRelay: false,
    color: '#627EEA',
  },
  'doge-pbc': {
    id: 'doge-pbc',
    name: 'Dogecoin PBC',
    symbol: 'DOGE',
    decimals: 8,
    rpc: 'ws://localhost:9947',
    isRelay: false,
    color: '#C2A633',
  },
  'sol-pbc': {
    id: 'sol-pbc',
    name: 'Solana PBC',
    symbol: 'SOL',
    decimals: 9,
    rpc: 'ws://localhost:9948',
    isRelay: false,
    color: '#00FFA3',
  },
  'xlm-pbc': {
    id: 'xlm-pbc',
    name: 'Stellar PBC',
    symbol: 'XLM',
    decimals: 7,
    rpc: 'ws://localhost:9949',
    isRelay: false,
    color: '#000000',
  },
  'xrp-pbc': {
    id: 'xrp-pbc',
    name: 'Ripple PBC',
    symbol: 'XRP',
    decimals: 6,
    rpc: 'ws://localhost:9950',
    isRelay: false,
    color: '#23292F',
  },
  'bnb-pbc': {
    id: 'bnb-pbc',
    name: 'Binance PBC',
    symbol: 'BNB',
    decimals: 18,
    rpc: 'ws://localhost:9951',
    isRelay: false,
    color: '#F3BA2F',
  },
  'trx-pbc': {
    id: 'trx-pbc',
    name: 'Tron PBC',
    symbol: 'TRX',
    decimals: 6,
    rpc: 'ws://localhost:9952',
    isRelay: false,
    color: '#EB0029',
  },
  'ada-pbc': {
    id: 'ada-pbc',
    name: 'Cardano PBC',
    symbol: 'ADA',
    decimals: 6,
    rpc: 'ws://localhost:9953',
    isRelay: false,
    color: '#0033AD',
  },
  'link-pbc': {
    id: 'link-pbc',
    name: 'Chainlink PBC',
    symbol: 'LINK',
    decimals: 18,
    rpc: 'ws://localhost:9954',
    isRelay: false,
    color: '#2A5ADA',
  },
  'matic-pbc': {
    id: 'matic-pbc',
    name: 'Polygon PBC',
    symbol: 'MATIC',
    decimals: 18,
    rpc: 'ws://localhost:9955',
    isRelay: false,
    color: '#8247E5',
  },
  'sc-usdt-pbc': {
    id: 'sc-usdt-pbc',
    name: 'USDT PBC',
    symbol: 'SC-USDT',
    decimals: 6,
    rpc: 'ws://localhost:9956',
    isRelay: false,
    color: '#26A17B',
  },
  'edsc-pbc': {
    id: 'edsc-pbc',
    name: 'EDSC Stablecoin PBC',
    symbol: 'EDSC',
    decimals: 12,
    rpc: 'ws://localhost:9957',
    isRelay: false,
    color: '#4A90E2',
  },
};

export const DEFAULT_CHAIN: ChainId = 'flarechain';

export function getChainById(id: ChainId): ChainConfig {
  return CHAINS[id];
}

export function getAllChains(): ChainConfig[] {
  return Object.values(CHAINS);
}

export function getRelayChain(): ChainConfig {
  return CHAINS.flarechain;
}

export function getParachains(): ChainConfig[] {
  return getAllChains().filter((chain) => !chain.isRelay);
}
