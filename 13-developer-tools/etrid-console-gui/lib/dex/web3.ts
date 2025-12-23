/**
 * Web3 Integration Utilities
 *
 * This file provides utilities for interacting with blockchain wallets and executing swaps.
 * It's designed to work with both EVM chains (via ethers/viem) and Solana.
 *
 * Note: Full implementation requires wallet connection libraries:
 * - For EVM: wagmi, viem, or ethers.js
 * - For Solana: @solana/wallet-adapter
 */

import { Network, SwapParams } from './types'

/**
 * Check if user has the required network added to their wallet
 */
export async function checkNetwork(network: Network): Promise<boolean> {
  if (typeof window === 'undefined' || !window.ethereum) {
    return false
  }

  try {
    const chainId = await window.ethereum.request({ method: 'eth_chainId' })
    return chainId === getChainIdHex(network)
  } catch (error) {
    console.error('Failed to check network:', error)
    return false
  }
}

/**
 * Switch to the required network
 */
export async function switchNetwork(network: Network): Promise<boolean> {
  if (typeof window === 'undefined' || !window.ethereum) {
    throw new Error('No wallet detected')
  }

  try {
    await window.ethereum.request({
      method: 'wallet_switchEthereumChain',
      params: [{ chainId: getChainIdHex(network) }],
    })
    return true
  } catch (error: any) {
    // This error code indicates that the chain has not been added to the wallet
    if (error.code === 4902) {
      return await addNetwork(network)
    }
    throw error
  }
}

/**
 * Add a network to the user's wallet
 */
async function addNetwork(network: Network): Promise<boolean> {
  if (typeof window === 'undefined' || !window.ethereum) {
    throw new Error('No wallet detected')
  }

  const networkConfigs: Record<Network, any> = {
    [Network.ETHEREUM]: null, // Ethereum is already in most wallets
    [Network.BSC]: {
      chainId: '0x38',
      chainName: 'BNB Smart Chain',
      nativeCurrency: {
        name: 'BNB',
        symbol: 'BNB',
        decimals: 18,
      },
      rpcUrls: ['https://bsc-dataseed.binance.org'],
      blockExplorerUrls: ['https://bscscan.com'],
    },
    [Network.POLYGON]: {
      chainId: '0x89',
      chainName: 'Polygon',
      nativeCurrency: {
        name: 'MATIC',
        symbol: 'MATIC',
        decimals: 18,
      },
      rpcUrls: ['https://polygon-rpc.com'],
      blockExplorerUrls: ['https://polygonscan.com'],
    },
    [Network.ARBITRUM]: {
      chainId: '0xa4b1',
      chainName: 'Arbitrum One',
      nativeCurrency: {
        name: 'ETH',
        symbol: 'ETH',
        decimals: 18,
      },
      rpcUrls: ['https://arb1.arbitrum.io/rpc'],
      blockExplorerUrls: ['https://arbiscan.io'],
    },
    [Network.SOLANA]: null, // Solana uses different wallet connection
  }

  const config = networkConfigs[network]
  if (!config) {
    throw new Error(`Network ${network} cannot be added automatically`)
  }

  try {
    await window.ethereum.request({
      method: 'wallet_addEthereumChain',
      params: [config],
    })
    return true
  } catch (error) {
    console.error('Failed to add network:', error)
    return false
  }
}

/**
 * Get chain ID in hex format
 */
function getChainIdHex(network: Network): string {
  const chainIds: Record<Network, string> = {
    [Network.ETHEREUM]: '0x1',
    [Network.BSC]: '0x38',
    [Network.POLYGON]: '0x89',
    [Network.ARBITRUM]: '0xa4b1',
    [Network.SOLANA]: '', // Solana doesn't use EVM chain IDs
  }
  return chainIds[network]
}

/**
 * Approve token spending (ERC-20)
 */
export async function approveToken(
  tokenAddress: string,
  spenderAddress: string,
  amount: string
): Promise<string> {
  if (typeof window === 'undefined' || !window.ethereum) {
    throw new Error('No wallet detected')
  }

  // ERC-20 approve function signature
  const data = encodeApproveData(spenderAddress, amount)

  try {
    const txHash = await window.ethereum.request({
      method: 'eth_sendTransaction',
      params: [
        {
          to: tokenAddress,
          data,
        },
      ],
    })
    return txHash
  } catch (error) {
    console.error('Token approval failed:', error)
    throw error
  }
}

/**
 * Encode approve function data
 */
function encodeApproveData(spender: string, amount: string): string {
  // This is a simplified version. In production, use ethers.js or viem
  const functionSelector = '0x095ea7b3' // approve(address,uint256)
  const paddedSpender = spender.slice(2).padStart(64, '0')
  const paddedAmount = BigInt(amount).toString(16).padStart(64, '0')
  return `${functionSelector}${paddedSpender}${paddedAmount}`
}

/**
 * Check token allowance
 */
export async function checkAllowance(
  tokenAddress: string,
  ownerAddress: string,
  spenderAddress: string
): Promise<string> {
  if (typeof window === 'undefined' || !window.ethereum) {
    throw new Error('No wallet detected')
  }

  // ERC-20 allowance function signature
  const data = encodeAllowanceData(ownerAddress, spenderAddress)

  try {
    const result = await window.ethereum.request({
      method: 'eth_call',
      params: [
        {
          to: tokenAddress,
          data,
        },
        'latest',
      ],
    })
    return result
  } catch (error) {
    console.error('Failed to check allowance:', error)
    return '0x0'
  }
}

/**
 * Encode allowance function data
 */
function encodeAllowanceData(owner: string, spender: string): string {
  const functionSelector = '0xdd62ed3e' // allowance(address,address)
  const paddedOwner = owner.slice(2).padStart(64, '0')
  const paddedSpender = spender.slice(2).padStart(64, '0')
  return `${functionSelector}${paddedOwner}${paddedSpender}`
}

/**
 * Get current wallet address
 */
export async function getCurrentAddress(): Promise<string | null> {
  if (typeof window === 'undefined' || !window.ethereum) {
    return null
  }

  try {
    const accounts = await window.ethereum.request({ method: 'eth_accounts' })
    return accounts[0] || null
  } catch (error) {
    console.error('Failed to get current address:', error)
    return null
  }
}

/**
 * Request wallet connection
 */
export async function connectWallet(): Promise<string | null> {
  if (typeof window === 'undefined' || !window.ethereum) {
    throw new Error('No wallet detected. Please install MetaMask or another Web3 wallet.')
  }

  try {
    const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' })
    return accounts[0] || null
  } catch (error) {
    console.error('Wallet connection failed:', error)
    throw error
  }
}

/**
 * Sign message with wallet
 */
export async function signMessage(message: string): Promise<string> {
  if (typeof window === 'undefined' || !window.ethereum) {
    throw new Error('No wallet detected')
  }

  const address = await getCurrentAddress()
  if (!address) {
    throw new Error('No wallet connected')
  }

  try {
    const signature = await window.ethereum.request({
      method: 'personal_sign',
      params: [message, address],
    })
    return signature
  } catch (error) {
    console.error('Message signing failed:', error)
    throw error
  }
}

// Type declaration for window.ethereum
declare global {
  interface Window {
    ethereum?: any
  }
}

/**
 * Utility to format transaction errors
 */
export function formatTransactionError(error: any): string {
  if (error.code === 4001) {
    return 'Transaction rejected by user'
  }
  if (error.code === -32603) {
    return 'Transaction failed - insufficient funds or gas'
  }
  if (error.message) {
    return error.message
  }
  return 'Transaction failed'
}
