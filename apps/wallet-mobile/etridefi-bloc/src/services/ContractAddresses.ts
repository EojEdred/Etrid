// Contract Addresses Configuration
// Deployed contract addresses for ETRID multichain ecosystem

export enum Network {
  PRIMEARC = 'primearc',
  ETHEREUM = 'ethereum',
  BNB_CHAIN = 'bsc',
  POLYGON = 'polygon',
  ARBITRUM = 'arbitrum',
  OPTIMISM = 'optimism',
  BASE = 'base',
  SOLANA = 'solana',
}

export interface NetworkConfig {
  chainId: number;
  rpcURL: string;
  explorerURL: string;
  name: string;
  symbol: string;
  decimals: number;
  icon: string;
}

export const NETWORK_CONFIG: Record<Network, NetworkConfig> = {
  [Network.PRIMEARC]: {
    chainId: 5765,
    rpcURL: 'https://rpc.primearc.etrid.org',
    explorerURL: 'https://explorer.primearc.etrid.org',
    name: 'Primearc',
    symbol: 'ETR',
    decimals: 18,
    icon: 'P',
  },
  [Network.ETHEREUM]: {
    chainId: 1,
    rpcURL: 'https://eth.llamarpc.com',
    explorerURL: 'https://etherscan.io',
    name: 'Ethereum',
    symbol: 'ETH',
    decimals: 18,
    icon: 'E',
  },
  [Network.BNB_CHAIN]: {
    chainId: 56,
    rpcURL: 'https://bsc-dataseed.binance.org',
    explorerURL: 'https://bscscan.com',
    name: 'BNB Chain',
    symbol: 'BNB',
    decimals: 18,
    icon: 'B',
  },
  [Network.POLYGON]: {
    chainId: 137,
    rpcURL: 'https://polygon-rpc.com',
    explorerURL: 'https://polygonscan.com',
    name: 'Polygon',
    symbol: 'MATIC',
    decimals: 18,
    icon: 'P',
  },
  [Network.ARBITRUM]: {
    chainId: 42161,
    rpcURL: 'https://arb1.arbitrum.io/rpc',
    explorerURL: 'https://arbiscan.io',
    name: 'Arbitrum',
    symbol: 'ETH',
    decimals: 18,
    icon: 'A',
  },
  [Network.OPTIMISM]: {
    chainId: 10,
    rpcURL: 'https://mainnet.optimism.io',
    explorerURL: 'https://optimistic.etherscan.io',
    name: 'Optimism',
    symbol: 'ETH',
    decimals: 18,
    icon: 'O',
  },
  [Network.BASE]: {
    chainId: 8453,
    rpcURL: 'https://mainnet.base.org',
    explorerURL: 'https://basescan.org',
    name: 'Base',
    symbol: 'ETH',
    decimals: 18,
    icon: 'B',
  },
  [Network.SOLANA]: {
    chainId: 0, // Solana uses different addressing
    rpcURL: 'https://api.mainnet-beta.solana.com',
    explorerURL: 'https://solscan.io',
    name: 'Solana',
    symbol: 'SOL',
    decimals: 9,
    icon: 'S',
  },
};

// Wrapped ETR (wETR) contract addresses on each chain - DEPLOYED CONTRACTS
export const WRAPPED_ETR: Partial<Record<Network, string>> = {
  [Network.SOLANA]: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
  [Network.BNB_CHAIN]: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
  [Network.POLYGON]: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
  [Network.ETHEREUM]: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
  [Network.ARBITRUM]: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
};

// EDSC (ETRID Stablecoin) contract addresses
export const EDSC: Partial<Record<Network, string>> = {
  [Network.PRIMEARC]: 'native',
  [Network.ETHEREUM]: '0x0000000000000000000000000000000000000000',
  [Network.BNB_CHAIN]: '0x0000000000000000000000000000000000000000',
};

// MasterChef (Yield Farming) contract addresses
export const MASTER_CHEF: Partial<Record<Network, string>> = {
  [Network.PRIMEARC]: '0x0000000000000000000000000000000000000000',
  [Network.ETHEREUM]: '0x0000000000000000000000000000000000000000',
  [Network.BNB_CHAIN]: '0x0000000000000000000000000000000000000000',
};

// PrimeSwap Router (DEX) contract addresses
export const PRIME_SWAP_ROUTER: Partial<Record<Network, string>> = {
  [Network.PRIMEARC]: '0x0000000000000000000000000000000000000000',
  [Network.ETHEREUM]: '0x0000000000000000000000000000000000000000',
  [Network.BNB_CHAIN]: '0x0000000000000000000000000000000000000000',
};

// DEX Router addresses on different chains
export const DEX_ROUTERS: Record<string, {address: string; name: string}> = {
  '1inch_eth': {address: '0x1111111254EEB25477B68fb85Ed929f73A960582', name: '1inch'},
  '1inch_bsc': {address: '0x1111111254EEB25477B68fb85Ed929f73A960582', name: '1inch'},
  uniswap_v3: {address: '0xE592427A0AEce92De3Edee1F18E0157C05861564', name: 'Uniswap V3'},
  pancakeswap: {address: '0x10ED43C718714eb63d5aA57B78B54704E256024E', name: 'PancakeSwap'},
  jupiter: {address: 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4', name: 'Jupiter'},
};

// LP Token pools
export interface LPToken {
  address: string;
  token0: string;
  token1: string;
  name: string;
  network: Network;
}

export const LP_POOLS: LPToken[] = [
  {
    address: '0x0000000000000000000000000000000000000000',
    token0: 'ETR',
    token1: 'USDT',
    name: 'ETR-USDT LP',
    network: Network.PRIMEARC,
  },
  {
    address: '0x0000000000000000000000000000000000000000',
    token0: 'ETR',
    token1: 'EDSC',
    name: 'ETR-EDSC LP',
    network: Network.PRIMEARC,
  },
  {
    address: '0x0000000000000000000000000000000000000000',
    token0: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    token1: 'ETH',
    name: 'wETR-ETH LP',
    network: Network.ETHEREUM,
  },
  {
    address: '0x0000000000000000000000000000000000000000',
    token0: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    token1: 'BNB',
    name: 'wETR-BNB LP',
    network: Network.BNB_CHAIN,
  },
];

// ERC20 ABI for balance and approval calls
export const ERC20_ABI = [
  'function balanceOf(address) view returns (uint256)',
  'function approve(address spender, uint256 amount) returns (bool)',
  'function allowance(address owner, address spender) view returns (uint256)',
  'function transfer(address to, uint256 amount) returns (bool)',
  'function decimals() view returns (uint8)',
  'function symbol() view returns (string)',
  'function name() view returns (string)',
];

// Helper to get contract address
export function getContractAddress(
  contractType: 'wrappedETR' | 'edsc' | 'masterChef' | 'primeSwapRouter',
  network: Network,
): string | undefined {
  switch (contractType) {
    case 'wrappedETR':
      return WRAPPED_ETR[network];
    case 'edsc':
      return EDSC[network];
    case 'masterChef':
      return MASTER_CHEF[network];
    case 'primeSwapRouter':
      return PRIME_SWAP_ROUTER[network];
    default:
      return undefined;
  }
}

// Helper to get network config
export function getNetworkConfig(network: Network): NetworkConfig {
  return NETWORK_CONFIG[network];
}

// Supported swap chains
export const SWAP_SUPPORTED_CHAINS: Network[] = [
  Network.ETHEREUM,
  Network.BNB_CHAIN,
  Network.POLYGON,
  Network.ARBITRUM,
  Network.SOLANA,
];
