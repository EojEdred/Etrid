/**
 * ETH PBC (Ethereum Partition Burst Chain) Configuration
 *
 * ETH PBC is a Substrate-based chain with full EVM compatibility via Frontier
 * - Part of the Ëtrid multichain ecosystem
 * - Dedicated to Ethereum-compatible DeFi applications
 * - Hosts MasterChef LP staking with cross-chain reward bridging via EDSC
 */

import { defineChain } from 'viem'

export const ethPBC = defineChain({
  id: 8888, // Update with actual chain ID after genesis
  name: 'ETH Partition Burst Chain',
  network: 'eth-pbc',
  nativeCurrency: {
    decimals: 18,
    name: 'Ëtrid',
    symbol: 'ETR',
  },
  rpcUrls: {
    default: {
      http: [process.env.NEXT_PUBLIC_ETH_PBC_RPC_HTTP || 'http://127.0.0.1:9944'],
      webSocket: [process.env.NEXT_PUBLIC_ETH_PBC_RPC_WS || 'ws://127.0.0.1:9944'],
    },
    public: {
      http: [process.env.NEXT_PUBLIC_ETH_PBC_RPC_HTTP || 'http://127.0.0.1:9944'],
      webSocket: [process.env.NEXT_PUBLIC_ETH_PBC_RPC_WS || 'ws://127.0.0.1:9944'],
    },
  },
  blockExplorers: {
    default: {
      name: 'ETH PBC Explorer',
      url: process.env.NEXT_PUBLIC_ETH_PBC_EXPLORER || 'http://localhost:3001',
    },
  },
  contracts: {
    // MasterChef LP Staking Contract
    masterChef: {
      address: (process.env.NEXT_PUBLIC_MASTERCHEF_ADDRESS as `0x${string}`) || '0x0000000000000000000000000000000000000000',
      blockCreated: 0, // Update after deployment
    },
    // EDSC Bridge Adapter Contract
    bridgeAdapter: {
      address: (process.env.NEXT_PUBLIC_BRIDGE_ADAPTER_ADDRESS as `0x${string}`) || '0x0000000000000000000000000000000000000000',
      blockCreated: 0, // Update after deployment
    },
    // ETR Reward Token Contract
    etrToken: {
      address: (process.env.NEXT_PUBLIC_ETR_TOKEN_ADDRESS as `0x${string}`) || '0x0000000000000000000000000000000000000000',
      blockCreated: 0,
    },
    // EDSC Stablecoin Contract
    edscToken: {
      address: (process.env.NEXT_PUBLIC_EDSC_TOKEN_ADDRESS as `0x${string}`) || '0x0000000000000000000000000000000000000000',
      blockCreated: 0,
    },
  },
  testnet: true,
})

/**
 * ETH PBC Chain Metadata
 */
export const ETH_PBC_METADATA = {
  chainId: ethPBC.id,
  chainName: ethPBC.name,
  nativeCurrency: ethPBC.nativeCurrency,
  rpcUrls: ethPBC.rpcUrls.default.http,
  blockExplorerUrls: [ethPBC.blockExplorers.default.url],
  iconUrls: [], // Add chain icon URL when available
} as const

/**
 * Contract Addresses (exported for convenience)
 */
export const ETH_PBC_CONTRACTS = {
  MASTERCHEF: ethPBC.contracts?.masterChef?.address || ('0x0000000000000000000000000000000000000000' as `0x${string}`),
  BRIDGE_ADAPTER: ethPBC.contracts?.bridgeAdapter?.address || ('0x0000000000000000000000000000000000000000' as `0x${string}`),
  ETR_TOKEN: ethPBC.contracts?.etrToken?.address || ('0x0000000000000000000000000000000000000000' as `0x${string}`),
  EDSC_TOKEN: ethPBC.contracts?.edscToken?.address || ('0x0000000000000000000000000000000000000000' as `0x${string}`),
} as const

/**
 * Check if address is a valid contract address
 */
export function isValidContractAddress(address: `0x${string}` | undefined): address is `0x${string}` {
  return !!address && address !== '0x0000000000000000000000000000000000000000'
}
