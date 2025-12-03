import { Network, DEX, PoolConfig, Token } from './types'

// wETR Pool Addresses
export const WETR_POOLS = {
  [Network.SOLANA]: {
    address: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
    dex: DEX.RAYDIUM,
  },
  [Network.BSC]: {
    address: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    dex: DEX.PANCAKESWAP,
  },
  [Network.ETHEREUM]: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    dex: DEX.UNISWAP,
  },
  [Network.POLYGON]: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    dex: DEX.QUICKSWAP,
  },
  [Network.ARBITRUM]: {
    address: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
    dex: DEX.CAMELOT,
  },
}

// Network Configuration
export const NETWORK_CONFIG = {
  [Network.ETHEREUM]: {
    name: 'Ethereum',
    chainId: 1,
    rpcUrl: 'https://eth.llamarpc.com',
    explorer: 'https://etherscan.io',
    nativeCurrency: {
      name: 'Ether',
      symbol: 'ETH',
      decimals: 18,
    },
  },
  [Network.BSC]: {
    name: 'BNB Smart Chain',
    chainId: 56,
    rpcUrl: 'https://bsc-dataseed.binance.org',
    explorer: 'https://bscscan.com',
    nativeCurrency: {
      name: 'BNB',
      symbol: 'BNB',
      decimals: 18,
    },
  },
  [Network.POLYGON]: {
    name: 'Polygon',
    chainId: 137,
    rpcUrl: 'https://polygon-rpc.com',
    explorer: 'https://polygonscan.com',
    nativeCurrency: {
      name: 'MATIC',
      symbol: 'MATIC',
      decimals: 18,
    },
  },
  [Network.ARBITRUM]: {
    name: 'Arbitrum One',
    chainId: 42161,
    rpcUrl: 'https://arb1.arbitrum.io/rpc',
    explorer: 'https://arbiscan.io',
    nativeCurrency: {
      name: 'Ether',
      symbol: 'ETH',
      decimals: 18,
    },
  },
  [Network.SOLANA]: {
    name: 'Solana',
    chainId: 101,
    rpcUrl: 'https://api.mainnet-beta.solana.com',
    explorer: 'https://solscan.io',
    nativeCurrency: {
      name: 'SOL',
      symbol: 'SOL',
      decimals: 9,
    },
  },
}

// DEX Router Addresses
export const DEX_ROUTERS = {
  [DEX.UNISWAP]: {
    [Network.ETHEREUM]: '0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45', // SwapRouter02
  },
  [DEX.PANCAKESWAP]: {
    [Network.BSC]: '0x10ED43C718714eb63d5aA57B78B54704E256024E',
  },
  [DEX.QUICKSWAP]: {
    [Network.POLYGON]: '0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff',
  },
  [DEX.CAMELOT]: {
    [Network.ARBITRUM]: '0xc873fEcbd354f5A56E00E710B90EF4201db2448d',
  },
  [DEX.RAYDIUM]: {
    [Network.SOLANA]: '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8', // Raydium AMM Program
  },
}

// Slippage Presets
export const SLIPPAGE_PRESETS = [0.1, 0.5, 1.0]

// Default Slippage
export const DEFAULT_SLIPPAGE = 0.5

// Price Impact Thresholds
export const PRICE_IMPACT = {
  LOW: 1,
  MEDIUM: 3,
  HIGH: 5,
  CRITICAL: 10,
}

// wETR Token Configuration
export const WETR_TOKEN: Record<Network, Token> = {
  [Network.ETHEREUM]: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    symbol: 'wETR',
    decimals: 18,
    name: 'Wrapped ETRID',
    chainId: 1,
  },
  [Network.BSC]: {
    address: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    symbol: 'wETR',
    decimals: 18,
    name: 'Wrapped ETRID',
    chainId: 56,
  },
  [Network.POLYGON]: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    symbol: 'wETR',
    decimals: 18,
    name: 'Wrapped ETRID',
    chainId: 137,
  },
  [Network.ARBITRUM]: {
    address: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
    symbol: 'wETR',
    decimals: 18,
    name: 'Wrapped ETRID',
    chainId: 42161,
  },
  [Network.SOLANA]: {
    address: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
    symbol: 'wETR',
    decimals: 9,
    name: 'Wrapped ETRID',
    chainId: 101,
  },
}

// Common Base Tokens for each network
export const BASE_TOKENS: Record<Network, Token[]> = {
  [Network.ETHEREUM]: [
    {
      address: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2',
      symbol: 'WETH',
      decimals: 18,
      name: 'Wrapped Ether',
      chainId: 1,
    },
    {
      address: '0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48',
      symbol: 'USDC',
      decimals: 6,
      name: 'USD Coin',
      chainId: 1,
    },
    {
      address: '0xdAC17F958D2ee523a2206206994597C13D831ec7',
      symbol: 'USDT',
      decimals: 6,
      name: 'Tether USD',
      chainId: 1,
    },
  ],
  [Network.BSC]: [
    {
      address: '0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c',
      symbol: 'WBNB',
      decimals: 18,
      name: 'Wrapped BNB',
      chainId: 56,
    },
    {
      address: '0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d',
      symbol: 'USDC',
      decimals: 18,
      name: 'USD Coin',
      chainId: 56,
    },
    {
      address: '0x55d398326f99059fF775485246999027B3197955',
      symbol: 'USDT',
      decimals: 18,
      name: 'Tether USD',
      chainId: 56,
    },
  ],
  [Network.POLYGON]: [
    {
      address: '0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270',
      symbol: 'WMATIC',
      decimals: 18,
      name: 'Wrapped Matic',
      chainId: 137,
    },
    {
      address: '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174',
      symbol: 'USDC',
      decimals: 6,
      name: 'USD Coin',
      chainId: 137,
    },
    {
      address: '0xc2132D05D31c914a87C6611C10748AEb04B58e8F',
      symbol: 'USDT',
      decimals: 6,
      name: 'Tether USD',
      chainId: 137,
    },
  ],
  [Network.ARBITRUM]: [
    {
      address: '0x82aF49447D8a07e3bd95BD0d56f35241523fBab1',
      symbol: 'WETH',
      decimals: 18,
      name: 'Wrapped Ether',
      chainId: 42161,
    },
    {
      address: '0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8',
      symbol: 'USDC',
      decimals: 6,
      name: 'USD Coin',
      chainId: 42161,
    },
    {
      address: '0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9',
      symbol: 'USDT',
      decimals: 6,
      name: 'Tether USD',
      chainId: 42161,
    },
  ],
  [Network.SOLANA]: [
    {
      address: 'So11111111111111111111111111111111111111112',
      symbol: 'SOL',
      decimals: 9,
      name: 'Solana',
      chainId: 101,
    },
    {
      address: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
      symbol: 'USDC',
      decimals: 6,
      name: 'USD Coin',
      chainId: 101,
    },
    {
      address: 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB',
      symbol: 'USDT',
      decimals: 6,
      name: 'Tether USD',
      chainId: 101,
    },
  ],
}
