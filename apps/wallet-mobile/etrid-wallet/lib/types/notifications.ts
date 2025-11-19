// Notifications Types

export interface Notification {
  id: string
  userId: string
  type: NotificationType
  title: string
  message: string
  read: boolean
  createdAt: Date
  actionUrl?: string
  metadata?: Record<string, any>
}

export type NotificationType =
  | 'price'
  | 'whale'
  | 'governance'
  | 'staking'
  | 'security'
  | 'transaction'
  | 'general'

export interface NotificationFilter {
  type?: NotificationType
  read?: boolean
  startDate?: Date
  endDate?: Date
}

export interface Alert {
  id: string
  userId: string
  alertType: AlertType
  enabled: boolean
  conditions: AlertCondition
  channels: NotificationChannel[]
  createdAt: Date
  lastTriggered?: Date
}

export type AlertType =
  | 'price_above'
  | 'price_below'
  | 'price_crosses'
  | 'percent_change'
  | 'whale_movement'
  | 'exchange_flow'
  | 'governance_new'
  | 'governance_ending'
  | 'staking_reward'
  | 'staking_unbond'
  | 'security_login'
  | 'security_transaction'

export interface AlertCondition {
  asset?: string
  threshold?: number
  percentage?: number
  timeframe?: string
  comparison?: 'above' | 'below' | 'crosses'
}

export type NotificationChannel = 'push' | 'email' | 'sms'

export interface AlertSettings {
  enabledTypes: AlertType[]
  quietHoursStart?: string
  quietHoursEnd?: string
  maxAlertsPerHour?: number
  defaultChannels: NotificationChannel[]
}

export interface TriggeredAlert {
  alert: Alert
  timestamp: Date
  value: number
  message: string
}

export interface PushToken {
  id: string
  userId: string
  token: string
  platform: 'ios' | 'android' | 'web'
  deviceId: string
  createdAt: Date
}
