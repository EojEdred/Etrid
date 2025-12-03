/**
 * ETRID Chain Configuration
 * Real RPC endpoints and contract addresses for production use
 */

// ETRID Native Chain Configuration
export const ETRID_CHAIN = {
  name: 'ETRID',
  symbol: 'ETR',
  decimals: 18,
  rpc: {
    primary: 'wss://rpc.etrid.org',
    fallback: 'ws://157.173.200.80:9944',
    http: 'https://rpc.etrid.org',
  },
  ss58Prefix: 42, // Substrate default, update if ETRID has custom prefix
  genesisHash: '', // Will be populated on connect
}

// Wrapped ETR (wETR) Contract Addresses on External Chains
export const WETR_CONTRACTS = {
  solana: {
    address: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
    dex: 'Raydium',
    decimals: 9,
  },
  bsc: {
    address: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    dex: 'PancakeSwap V2',
    decimals: 18,
    chainId: 56,
  },
  ethereum: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    dex: 'Uniswap V2',
    decimals: 18,
    chainId: 1,
  },
  polygon: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    dex: 'QuickSwap',
    decimals: 18,
    chainId: 137,
  },
  arbitrum: {
    address: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
    dex: 'Camelot',
    decimals: 18,
    chainId: 42161,
  },
} as const

// External Chain RPC Endpoints
export const EXTERNAL_CHAINS = {
  ethereum: {
    name: 'Ethereum',
    symbol: 'ETH',
    chainId: 1,
    rpc: 'https://eth.llamarpc.com',
    explorer: 'https://etherscan.io',
  },
  bsc: {
    name: 'BNB Smart Chain',
    symbol: 'BNB',
    chainId: 56,
    rpc: 'https://bsc-dataseed.binance.org',
    explorer: 'https://bscscan.com',
  },
  polygon: {
    name: 'Polygon',
    symbol: 'MATIC',
    chainId: 137,
    rpc: 'https://polygon-rpc.com',
    explorer: 'https://polygonscan.com',
  },
  arbitrum: {
    name: 'Arbitrum One',
    symbol: 'ETH',
    chainId: 42161,
    rpc: 'https://arb1.arbitrum.io/rpc',
    explorer: 'https://arbiscan.io',
  },
  solana: {
    name: 'Solana',
    symbol: 'SOL',
    rpc: 'https://api.mainnet-beta.solana.com',
    explorer: 'https://solscan.io',
  },
} as const

// DEX Router Addresses
export const DEX_ROUTERS = {
  uniswapV2: '0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D',
  pancakeswapV2: '0x10ED43C718714eb63d5aA57B78B54704E256024E',
  quickswap: '0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff',
  camelot: '0xc873fEcbd354f5A56E00E710B90EF4201db2448d',
  raydium: 'CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK', // Raydium AMM program
} as const

// Governance Configuration
export const GOVERNANCE_CONFIG = {
  proposalCategories: [
    { id: 'inflation_rate', name: 'Inflation Rate', description: 'Modify network inflation parameters' },
    { id: 'parameter_change', name: 'Parameter Change', description: 'Update runtime parameters' },
    { id: 'budget_allocation', name: 'Budget Allocation', description: 'Treasury fund allocation' },
    { id: 'protocol_upgrade', name: 'Protocol Upgrade', description: 'Runtime and protocol upgrades' },
    { id: 'director_election', name: 'Director Election', description: 'Elect network directors' },
    { id: 'emergency_action', name: 'Emergency Action', description: 'Emergency protocol actions' },
  ],
  votingPeriod: 7 * 24 * 60 * 60 * 1000, // 7 days in ms
  minProposalDeposit: '1000000000000000000000', // 1000 ETR
  convictionLevels: [
    { multiplier: 0.1, lockPeriod: 0, name: '0.1x (No Lock)' },
    { multiplier: 1, lockPeriod: 7, name: '1x (7 days)' },
    { multiplier: 2, lockPeriod: 14, name: '2x (14 days)' },
    { multiplier: 3, lockPeriod: 28, name: '3x (28 days)' },
    { multiplier: 4, lockPeriod: 56, name: '4x (56 days)' },
    { multiplier: 5, lockPeriod: 112, name: '5x (112 days)' },
    { multiplier: 6, lockPeriod: 224, name: '6x (224 days)' },
  ],
}

export type ChainId = keyof typeof EXTERNAL_CHAINS
export type WetrChain = keyof typeof WETR_CONTRACTS
