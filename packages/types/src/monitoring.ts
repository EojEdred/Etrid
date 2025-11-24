/**
 * Network statistics
 */
export interface NetworkStats {
  blockHeight: number
  finalizedHeight: number
  peersCount: number
  validatorCount: number
  activeValidatorCount: number
  tps: number
  avgBlockTime: number
  totalIssuance: string
  totalStaked: string
  stakingRatio: number
}

/**
 * Validator health status
 */
export interface ValidatorHealth {
  address: string
  name: string
  status: 'healthy' | 'degraded' | 'offline'
  uptime: number
  lastSeen: Date
  missedBlocks: number
  producedBlocks: number
  responseTime?: number
  version?: string
  peers?: number
}

/**
 * System alert
 */
export interface Alert {
  id: string
  level: 'info' | 'warning' | 'error' | 'critical'
  title: string
  message: string
  timestamp: Date
  source: string
  resolved: boolean
  resolvedAt?: Date
}

/**
 * Performance metric
 */
export interface PerformanceMetric {
  timestamp: Date
  metric: string
  value: number
  unit: string
  tags?: Record<string, string>
}

/**
 * Node information
 */
export interface NodeInfo {
  nodeId: string
  name: string
  version: string
  roles: string[]
  peerId: string
  listenAddr: string[]
  externalAddr: string[]
  connectedPeers: number
  syncState: 'syncing' | 'synced' | 'downloading'
  bestBlock: number
  finalizedBlock: number
}

/**
 * Telemetry data
 */
export interface TelemetryData {
  nodeId: string
  timestamp: Date
  blockNumber: number
  blockHash: string
  blockTime: number
  peers: number
  txpoolSize: number
  bandwidth: {
    in: number
    out: number
  }
  system: {
    cpu: number
    memory: number
    disk: number
  }
}
