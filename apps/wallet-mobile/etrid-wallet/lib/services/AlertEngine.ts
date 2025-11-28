import { Alert, AlertSettings, AlertType, AlertCondition, TriggeredAlert } from '../types/notifications'

interface AlertInput {
  alertType: AlertType
  conditions: AlertCondition
  channels: ('push' | 'email' | 'sms')[]
}

export class AlertEngine {
  private static instance: AlertEngine

  static getInstance(): AlertEngine {
    if (!AlertEngine.instance) {
      AlertEngine.instance = new AlertEngine()
    }
    return AlertEngine.instance
  }

  async getAlerts(): Promise<Alert[]> {
    try {
      const response = await fetch('/api/alerts')
      if (!response.ok) throw new Error('Failed to fetch alerts')
      return await response.json()
    } catch (error) {
      console.error('Error fetching alerts:', error)
      return this.getMockAlerts()
    }
  }

  async createAlert(alert: AlertInput): Promise<Alert> {
    try {
      const response = await fetch('/api/alerts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(alert),
      })
      if (!response.ok) throw new Error('Failed to create alert')
      return await response.json()
    } catch (error) {
      console.error('Error creating alert:', error)
      throw error
    }
  }

  async updateAlert(id: string, updates: Partial<Alert>): Promise<Alert> {
    try {
      const response = await fetch(`/api/alerts/${id}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates),
      })
      if (!response.ok) throw new Error('Failed to update alert')
      return await response.json()
    } catch (error) {
      console.error('Error updating alert:', error)
      throw error
    }
  }

  async deleteAlert(id: string): Promise<void> {
    try {
      const response = await fetch(`/api/alerts/${id}`, {
        method: 'DELETE',
      })
      if (!response.ok) throw new Error('Failed to delete alert')
    } catch (error) {
      console.error('Error deleting alert:', error)
      throw error
    }
  }

  async getAlertSettings(): Promise<AlertSettings> {
    try {
      const response = await fetch('/api/alerts/settings')
      if (!response.ok) throw new Error('Failed to fetch alert settings')
      return await response.json()
    } catch (error) {
      console.error('Error fetching alert settings:', error)
      return this.getMockAlertSettings()
    }
  }

  async updateAlertSettings(settings: Partial<AlertSettings>): Promise<void> {
    try {
      const response = await fetch('/api/alerts/settings', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(settings),
      })
      if (!response.ok) throw new Error('Failed to update alert settings')
    } catch (error) {
      console.error('Error updating alert settings:', error)
      throw error
    }
  }

  async checkAlerts(): Promise<TriggeredAlert[]> {
    try {
      const response = await fetch('/api/alerts/check')
      if (!response.ok) throw new Error('Failed to check alerts')
      return await response.json()
    } catch (error) {
      console.error('Error checking alerts:', error)
      return []
    }
  }

  async testAlert(alertId: string): Promise<void> {
    try {
      const response = await fetch(`/api/alerts/${alertId}/test`, {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Failed to test alert')
    } catch (error) {
      console.error('Error testing alert:', error)
      throw error
    }
  }

  private getMockAlerts(): Alert[] {
    return [
      {
        id: '1',
        userId: 'user-1',
        alertType: 'price_above',
        enabled: true,
        conditions: {
          asset: 'ÉTR',
          threshold: 50,
          comparison: 'above',
        },
        channels: ['push', 'email'],
        createdAt: new Date('2024-01-15'),
        lastTriggered: new Date(),
      },
      {
        id: '2',
        userId: 'user-1',
        alertType: 'whale_movement',
        enabled: true,
        conditions: {
          asset: 'ÉTR',
          threshold: 1000000,
        },
        channels: ['push'],
        createdAt: new Date('2024-01-20'),
      },
      {
        id: '3',
        userId: 'user-1',
        alertType: 'governance_new',
        enabled: true,
        conditions: {},
        channels: ['push', 'email'],
        createdAt: new Date('2024-02-01'),
      },
    ]
  }

  private getMockAlertSettings(): AlertSettings {
    return {
      enabledTypes: [
        'price_above',
        'price_below',
        'whale_movement',
        'governance_new',
        'staking_reward',
        'security_login',
      ],
      quietHoursStart: '22:00',
      quietHoursEnd: '08:00',
      maxAlertsPerHour: 10,
      defaultChannels: ['push'],
    }
  }
}

export const alertEngine = AlertEngine.getInstance()
