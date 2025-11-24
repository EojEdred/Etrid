/**
 * Validator information
 */
export interface Validator {
  address: string
  name: string
  commission: number
  totalStake: string
  ownStake: string
  nominatorCount: number
  status: 'active' | 'inactive' | 'offline'
  isElected: boolean
  lastSeen?: Date
}

/**
 * Nominator information
 */
export interface Nominator {
  address: string
  stake: string
  validator: string
  rewards: string
  lastNominatedAt: Date
}

/**
 * Staking reward
 */
export interface Reward {
  era: number
  amount: string
  timestamp: Date
  validator?: string
}

/**
 * Account balance information
 */
export interface Balance {
  free: string
  reserved: string
  frozen: string
  total: string
}

/**
 * Block information
 */
export interface Block {
  number: number
  hash: string
  parentHash: string
  timestamp: Date
  extrinsicCount: number
  eventCount: number
  validator?: string
}

/**
 * Transaction/Extrinsic information
 */
export interface Transaction {
  hash: string
  blockNumber: number
  blockHash: string
  timestamp: Date
  from: string
  to?: string
  amount?: string
  success: boolean
  method: string
  section: string
  args: any
}

/**
 * Chain information
 */
export interface ChainInfo {
  name: string
  version: string
  runtimeVersion: number
  specVersion: number
  ss58Format: number
  tokenSymbol: string
  tokenDecimals: number
}
