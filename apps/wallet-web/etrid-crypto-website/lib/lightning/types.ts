/**
 * Lightning-Bloc TypeScript Types
 *
 * Mirrors the Rust types from lightning-bloc/src/
 */

export interface Channel {
  id: string
  chain: string
  counterparty: string
  capacity: string
  localBalance: string
  remoteBalance: string
  state: "pending" | "active" | "closing" | "closed"
  createdAt: number
  updatedAt: number
}

export interface Payment {
  id: string
  type: "send" | "receive"
  sourceChain: string
  destChain: string
  sourceAddress: string
  destAddress: string
  sourceAmount: string
  destAmount: string
  status: "pending" | "completed" | "failed"
  route?: CrossPBCRoute
  timestamp: number
  error?: string
}

export interface NetworkStats {
  totalChannels: number
  totalCapacity: string
  averageChannelSize: string
  activeChains: number
  recentPayments: number
  successRate: number
}

export interface CrossPBCRoute {
  sourceChain: string
  destChain: string
  segments: RouteSegment[]
  totalFees: string
  estimatedTime: number
  exchangeRate: ExchangeRate
}

export interface RouteSegment {
  fromChain: string
  toChain: string
  channelId: string
  amount: string
  fee: string
  exchangeRate: ExchangeRate
}

export interface ExchangeRate {
  rate: number // Basis points (10000 = 1:1)
  timestamp: number
  source: string // "oracle" | "manual" | "mock"
}

export interface OpenChannelParams {
  chain: string
  counterparty: string
  capacity: string
}

export interface SendPaymentParams {
  route: CrossPBCRoute
  sourceAddress: string
  destAddress: string
}

export interface FindRouteParams {
  sourceChain: string
  destChain: string
  sourceAddress: string
  destAddress: string
  amount: string
}

export interface HTLC {
  id: string
  sourceChannel: string
  destChannel: string
  sourceAmount: string
  destAmount: string
  hashLock: string
  timeLock: number
  status: "pending" | "claimed" | "refunded" | "expired"
}

// Supported PBC chains
export const SUPPORTED_CHAINS = [
  { id: "eth-pbc", name: "Ethereum", symbol: "ETH", decimals: 18 },
  { id: "btc-pbc", name: "Bitcoin", symbol: "BTC", decimals: 8 },
  { id: "bnb-pbc", name: "BNB Chain", symbol: "BNB", decimals: 18 },
  { id: "sol-pbc", name: "Solana", symbol: "SOL", decimals: 9 },
  { id: "ada-pbc", name: "Cardano", symbol: "ADA", decimals: 6 },
  { id: "trx-pbc", name: "Tron", symbol: "TRX", decimals: 6 },
  { id: "xrp-pbc", name: "XRP", symbol: "XRP", decimals: 6 },
  { id: "xlm-pbc", name: "Stellar", symbol: "XLM", decimals: 7 },
  { id: "matic-pbc", name: "Polygon", symbol: "MATIC", decimals: 18 },
  { id: "link-pbc", name: "Chainlink", symbol: "LINK", decimals: 18 },
  { id: "doge-pbc", name: "Dogecoin", symbol: "DOGE", decimals: 8 },
  { id: "sc-usdt-pbc", name: "USDT", symbol: "USDT", decimals: 6 },
  { id: "edsc-pbc", name: "EDSC", symbol: "EDSC", decimals: 18 },
] as const

export type ChainId = typeof SUPPORTED_CHAINS[number]["id"]
