export enum Network {
  ETHEREUM = 'ethereum',
  BSC = 'bsc',
  POLYGON = 'polygon',
  ARBITRUM = 'arbitrum',
  SOLANA = 'solana',
}

export enum DEX {
  UNISWAP = 'uniswap',
  PANCAKESWAP = 'pancakeswap',
  QUICKSWAP = 'quickswap',
  CAMELOT = 'camelot',
  RAYDIUM = 'raydium',
}

export interface Token {
  address: string
  symbol: string
  decimals: number
  name: string
  logoURI?: string
  chainId: number
}

export interface PoolConfig {
  network: Network
  dex: DEX
  poolAddress: string
  token0: Token
  token1: Token
  fee?: number
}

export interface QuoteResult {
  inputAmount: string
  outputAmount: string
  priceImpact: number
  minimumOutputAmount: string
  route: string[]
  gas: string
  fee: string
}

export interface SwapParams {
  network: Network
  dex: DEX
  tokenIn: string
  tokenOut: string
  amountIn: string
  slippage: number
  recipient: string
  deadline?: number
}

export interface TransactionHistory {
  hash: string
  network: Network
  dex: DEX
  tokenIn: Token
  tokenOut: Token
  amountIn: string
  amountOut: string
  timestamp: number
  status: 'pending' | 'success' | 'failed'
}

export interface PriceData {
  timestamp: number
  price: number
  volume24h?: number
}

export interface PoolStats {
  tvl: string
  volume24h: string
  fees24h: string
  apy: string
}
