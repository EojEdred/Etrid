/**
 * Lightning channel
 */
export interface LightningChannel {
  id: string
  channelId: string
  status: 'active' | 'inactive' | 'pending' | 'closed'
  capacity: string
  localBalance: string
  remoteBalance: string
  peerId: string
  peerAlias?: string
  initiator: boolean
  private: boolean
  createdAt: Date
  closedAt?: Date
}

/**
 * Lightning payment
 */
export interface LightningPayment {
  id: string
  paymentHash: string
  amount: string
  fee: string
  timestamp: Date
  status: 'pending' | 'succeeded' | 'failed'
  direction: 'sent' | 'received'
  destination?: string
  memo?: string
}

/**
 * Lightning invoice
 */
export interface LightningInvoice {
  id: string
  paymentRequest: string
  paymentHash: string
  amount: string
  description?: string
  createdAt: Date
  expiresAt: Date
  settled: boolean
  settledAt?: Date
}

/**
 * Lightning node info
 */
export interface LightningNodeInfo {
  nodeId: string
  alias: string
  color: string
  publicKey: string
  address: string[]
  channelCount: number
  activeChannelCount: number
  totalCapacity: string
  version: string
}

/**
 * Watchtower subscription
 */
export interface WatchtowerSubscription {
  id: string
  channelId: string
  startDate: Date
  endDate: Date
  fee: string
  status: 'active' | 'expired' | 'cancelled'
  towerPubkey: string
}

/**
 * Fraud alert
 */
export interface FraudAlert {
  id: string
  channelId: string
  detectedAt: Date
  txHash: string
  type: 'breach_attempt' | 'penalty_transaction'
  status: 'detected' | 'resolved' | 'penalized'
  penalty?: string
}
