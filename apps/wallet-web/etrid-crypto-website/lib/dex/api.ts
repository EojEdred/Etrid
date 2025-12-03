import { Network, DEX, QuoteResult, SwapParams, TransactionHistory, PriceData, PoolStats } from './types'
import { WETR_POOLS, DEX_ROUTERS, NETWORK_CONFIG, PRICE_IMPACT } from './constants'

/**
 * Get quote from DEX pool
 */
export async function getQuote(params: {
  network: Network
  tokenIn: string
  tokenOut: string
  amountIn: string
  slippage: number
}): Promise<QuoteResult> {
  const { network, tokenIn, tokenOut, amountIn, slippage } = params

  if (network === Network.SOLANA) {
    return getSolanaQuote(params)
  }

  return getEvmQuote(params)
}

/**
 * Get quote from Raydium (Solana)
 */
async function getSolanaQuote(params: {
  network: Network
  tokenIn: string
  tokenOut: string
  amountIn: string
  slippage: number
}): Promise<QuoteResult> {
  const { tokenIn, tokenOut, amountIn, slippage } = params

  try {
    // Use Jupiter aggregator API for Solana quotes
    const response = await fetch(
      `https://quote-api.jup.ag/v6/quote?inputMint=${tokenIn}&outputMint=${tokenOut}&amount=${parseFloat(amountIn) * 1e9}&slippageBps=${slippage * 100}`
    )

    if (!response.ok) {
      throw new Error('Failed to fetch Solana quote')
    }

    const data = await response.json()

    const outputAmount = (parseInt(data.outAmount) / 1e9).toString()
    const priceImpact = parseFloat(data.priceImpactPct || '0')
    const minimumOutputAmount = (parseInt(data.otherAmountThreshold) / 1e9).toString()

    return {
      inputAmount: amountIn,
      outputAmount,
      priceImpact,
      minimumOutputAmount,
      route: data.routePlan?.map((r: any) => r.swapInfo.label) || [],
      gas: '0.00001', // Approximate SOL gas
      fee: (parseFloat(amountIn) * 0.0025).toString(), // 0.25% fee
    }
  } catch (error) {
    console.error('Solana quote error:', error)
    throw new Error('Failed to get quote from Solana DEX')
  }
}

/**
 * Get quote from EVM-based DEX (Uniswap, PancakeSwap, etc.)
 */
async function getEvmQuote(params: {
  network: Network
  tokenIn: string
  tokenOut: string
  amountIn: string
  slippage: number
}): Promise<QuoteResult> {
  const { network, tokenIn, tokenOut, amountIn, slippage } = params
  const poolConfig = WETR_POOLS[network]

  try {
    // For demo purposes, calculate based on pool reserves
    // In production, you'd query the actual pool contract or use DEX APIs
    const mockReserve0 = 1000000 // Mock pool reserves
    const mockReserve1 = 8000000 // Mock pool reserves (1:8 ratio)

    const amountInFloat = parseFloat(amountIn)
    const amountInWithFee = amountInFloat * 0.997 // 0.3% fee

    // Constant product formula: x * y = k
    const outputAmount = (mockReserve1 * amountInWithFee) / (mockReserve0 + amountInWithFee)
    const priceImpact = (amountInFloat / mockReserve0) * 100

    const minimumOutput = outputAmount * (1 - slippage / 100)

    return {
      inputAmount: amountIn,
      outputAmount: outputAmount.toFixed(6),
      priceImpact: parseFloat(priceImpact.toFixed(2)),
      minimumOutputAmount: minimumOutput.toFixed(6),
      route: [tokenIn, tokenOut],
      gas: estimateEvmGas(network),
      fee: (amountInFloat * 0.003).toFixed(6),
    }
  } catch (error) {
    console.error('EVM quote error:', error)
    throw new Error(`Failed to get quote from ${network} DEX`)
  }
}

/**
 * Execute swap transaction
 */
export async function executeSwap(params: SwapParams): Promise<string> {
  const { network, dex, tokenIn, tokenOut, amountIn, slippage, recipient } = params

  if (network === Network.SOLANA) {
    return executeSolanaSwap(params)
  }

  return executeEvmSwap(params)
}

/**
 * Execute Solana swap via Jupiter
 */
async function executeSolanaSwap(params: SwapParams): Promise<string> {
  // This would integrate with Solana wallet adapter and Jupiter
  // For now, return a mock transaction hash
  console.log('Executing Solana swap:', params)

  throw new Error('Solana swap execution requires wallet connection')
}

/**
 * Execute EVM swap
 */
async function executeEvmSwap(params: SwapParams): Promise<string> {
  const { network, tokenIn, tokenOut, amountIn, slippage, recipient } = params

  // This would integrate with Web3 provider (ethers.js or viem)
  // For now, return a mock transaction hash
  console.log('Executing EVM swap:', params)

  throw new Error('EVM swap execution requires wallet connection')
}

/**
 * Get token balance
 */
export async function getTokenBalance(
  network: Network,
  tokenAddress: string,
  walletAddress: string
): Promise<string> {
  if (network === Network.SOLANA) {
    return getSolanaTokenBalance(tokenAddress, walletAddress)
  }

  return getEvmTokenBalance(network, tokenAddress, walletAddress)
}

/**
 * Get Solana token balance
 */
async function getSolanaTokenBalance(tokenAddress: string, walletAddress: string): Promise<string> {
  try {
    const response = await fetch(NETWORK_CONFIG[Network.SOLANA].rpcUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: 1,
        method: 'getTokenAccountsByOwner',
        params: [
          walletAddress,
          { mint: tokenAddress },
          { encoding: 'jsonParsed' },
        ],
      }),
    })

    const data = await response.json()

    if (data.result?.value?.length > 0) {
      const balance = data.result.value[0].account.data.parsed.info.tokenAmount.uiAmount
      return balance.toString()
    }

    return '0'
  } catch (error) {
    console.error('Failed to fetch Solana balance:', error)
    return '0'
  }
}

/**
 * Get EVM token balance
 */
async function getEvmTokenBalance(
  network: Network,
  tokenAddress: string,
  walletAddress: string
): Promise<string> {
  try {
    // ERC-20 balanceOf function signature
    const data = `0x70a08231000000000000000000000000${walletAddress.slice(2)}`

    const response = await fetch(NETWORK_CONFIG[network].rpcUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: 1,
        method: 'eth_call',
        params: [
          {
            to: tokenAddress,
            data,
          },
          'latest',
        ],
      }),
    })

    const result = await response.json()

    if (result.result) {
      // Convert hex to decimal and adjust for 18 decimals
      const balance = parseInt(result.result, 16) / 1e18
      return balance.toString()
    }

    return '0'
  } catch (error) {
    console.error('Failed to fetch EVM balance:', error)
    return '0'
  }
}

/**
 * Get transaction history
 */
export async function getTransactionHistory(
  network: Network,
  walletAddress: string
): Promise<TransactionHistory[]> {
  // This would query blockchain explorers or indexers
  // For now, return empty array
  return []
}

/**
 * Get price history for charts
 */
export async function getPriceHistory(
  network: Network,
  tokenAddress: string,
  timeRange: '1H' | '24H' | '7D' | '30D' | 'All'
): Promise<PriceData[]> {
  const poolConfig = WETR_POOLS[network]

  // Mock price data for demonstration
  // In production, you'd query from DEX subgraphs or price oracles
  const now = Date.now()
  const intervals = {
    '1H': { count: 12, step: 5 * 60 * 1000 }, // 5 min intervals
    '24H': { count: 24, step: 60 * 60 * 1000 }, // 1 hour intervals
    '7D': { count: 7, step: 24 * 60 * 60 * 1000 }, // 1 day intervals
    '30D': { count: 30, step: 24 * 60 * 60 * 1000 }, // 1 day intervals
    'All': { count: 90, step: 24 * 60 * 60 * 1000 }, // 1 day intervals
  }

  const { count, step } = intervals[timeRange]
  const basePrice = 8.0 // Base wETR price

  return Array.from({ length: count }, (_, i) => ({
    timestamp: now - (count - i - 1) * step,
    price: basePrice + (Math.random() - 0.5) * 0.4, // Random variation
    volume24h: Math.random() * 1000000,
  }))
}

/**
 * Get pool statistics
 */
export async function getPoolStats(
  network: Network,
  poolAddress: string
): Promise<PoolStats> {
  // Mock pool stats
  // In production, query from DEX subgraphs
  return {
    tvl: '$' + (Math.random() * 10000000).toFixed(0),
    volume24h: '$' + (Math.random() * 1000000).toFixed(0),
    fees24h: '$' + (Math.random() * 10000).toFixed(0),
    apy: (Math.random() * 50 + 10).toFixed(2) + '%',
  }
}

/**
 * Estimate gas for swap
 */
export async function estimateGas(params: SwapParams): Promise<string> {
  const { network } = params

  if (network === Network.SOLANA) {
    return '0.00001' // ~0.00001 SOL
  }

  return estimateEvmGas(network)
}

/**
 * Estimate EVM gas costs
 */
function estimateEvmGas(network: Network): string {
  const gasEstimates: Record<Network, string> = {
    [Network.ETHEREUM]: '0.003',
    [Network.BSC]: '0.0005',
    [Network.POLYGON]: '0.01',
    [Network.ARBITRUM]: '0.0002',
    [Network.SOLANA]: '0.00001',
  }

  return gasEstimates[network] || '0.001'
}

/**
 * Calculate price impact level
 */
export function getPriceImpactLevel(impact: number): {
  level: 'low' | 'medium' | 'high' | 'critical'
  color: string
} {
  if (impact < PRICE_IMPACT.LOW) {
    return { level: 'low', color: 'text-green-500' }
  } else if (impact < PRICE_IMPACT.MEDIUM) {
    return { level: 'medium', color: 'text-yellow-500' }
  } else if (impact < PRICE_IMPACT.HIGH) {
    return { level: 'high', color: 'text-orange-500' }
  } else {
    return { level: 'critical', color: 'text-red-500' }
  }
}

/**
 * Format token amount with proper decimals
 */
export function formatTokenAmount(amount: string, decimals: number = 18): string {
  const num = parseFloat(amount)
  if (num === 0) return '0'
  if (num < 0.000001) return '<0.000001'
  if (num < 1) return num.toFixed(6)
  if (num < 1000) return num.toFixed(4)
  return num.toLocaleString(undefined, { maximumFractionDigits: 2 })
}

/**
 * Format USD value
 */
export function formatUsdValue(value: number): string {
  if (value < 0.01) return '<$0.01'
  if (value < 1000) return `$${value.toFixed(2)}`
  if (value < 1000000) return `$${(value / 1000).toFixed(2)}K`
  return `$${(value / 1000000).toFixed(2)}M`
}
