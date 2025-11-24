/**
 * Wallet account
 */
export interface WalletAccount {
  address: string
  name?: string
  source: string
  type?: string
  meta?: Record<string, any>
}

/**
 * Token information
 */
export interface Token {
  symbol: string
  name: string
  decimals: number
  address?: string
  balance: string
  value?: string
  icon?: string
}

/**
 * Token swap
 */
export interface TokenSwap {
  id: string
  from: Token
  to: Token
  fromAmount: string
  toAmount: string
  rate: string
  slippage: number
  timestamp: Date
  status: 'pending' | 'completed' | 'failed'
  txHash?: string
  fee?: string
}

/**
 * Staking position
 */
export interface StakingPosition {
  validator: string
  validatorName?: string
  stake: string
  rewards: string
  startDate: Date
  unlocking?: {
    amount: string
    unlockAt: Date
  }[]
  status: 'active' | 'unlocking' | 'withdrawable'
}

/**
 * Liquidity pool
 */
export interface LiquidityPool {
  id: string
  name: string
  token0: Token
  token1: Token
  reserve0: string
  reserve1: string
  totalLiquidity: string
  apr: number
  volume24h: string
  fee: number
}

/**
 * LP position
 */
export interface LPPosition {
  poolId: string
  pool: LiquidityPool
  liquidity: string
  token0Amount: string
  token1Amount: string
  rewards: string
  apr: number
  timestamp: Date
}
