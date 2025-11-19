// Alert Service - Handle price alerts and notifications

import type { Alert, AlertInput, DCABot, DCABotInput } from '@/lib/types/trading'

export class AlertService {
  private apiUrl: string

  constructor(apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api') {
    this.apiUrl = apiUrl
  }

  /**
   * Create a new price alert
   */
  async createAlert(alert: AlertInput): Promise<Alert> {
    try {
      const response = await fetch(`${this.apiUrl}/alerts`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(alert),
      })

      if (!response.ok) {
        throw new Error(`Failed to create alert: ${response.statusText}`)
      }

      const data = await response.json()
      return data.alert
    } catch (error) {
      console.error('Error creating alert:', error)
      throw error
    }
  }

  /**
   * Get all alerts for the user
   */
  async getAlerts(status?: 'active' | 'triggered'): Promise<Alert[]> {
    try {
      const params = new URLSearchParams()
      if (status) {
        params.append('status', status)
      }

      const response = await fetch(`${this.apiUrl}/alerts?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch alerts: ${response.statusText}`)
      }

      const data = await response.json()
      return data.alerts || []
    } catch (error) {
      console.error('Error fetching alerts:', error)
      throw error
    }
  }

  /**
   * Delete an alert
   */
  async deleteAlert(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/alerts/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error(`Failed to delete alert: ${response.statusText}`)
      }
    } catch (error) {
      console.error('Error deleting alert:', error)
      throw error
    }
  }

  /**
   * Check which alerts should be triggered based on current price
   */
  async checkAlerts(pair: string, currentPrice: number): Promise<Alert[]> {
    try {
      const response = await fetch(`${this.apiUrl}/alerts/check`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ pair, current_price: currentPrice }),
      })

      if (!response.ok) {
        throw new Error(`Failed to check alerts: ${response.statusText}`)
      }

      const data = await response.json()
      return data.triggered_alerts || []
    } catch (error) {
      console.error('Error checking alerts:', error)
      throw error
    }
  }

  /**
   * Update alert status
   */
  async updateAlert(id: string, status: 'active' | 'cancelled'): Promise<Alert> {
    try {
      const response = await fetch(`${this.apiUrl}/alerts/${id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ status }),
      })

      if (!response.ok) {
        throw new Error(`Failed to update alert: ${response.statusText}`)
      }

      const data = await response.json()
      return data.alert
    } catch (error) {
      console.error('Error updating alert:', error)
      throw error
    }
  }

  /**
   * Create a DCA (Dollar Cost Averaging) bot
   */
  async createDCABot(bot: DCABotInput): Promise<DCABot> {
    try {
      const response = await fetch(`${this.apiUrl}/bots/dca`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(bot),
      })

      if (!response.ok) {
        throw new Error(`Failed to create DCA bot: ${response.statusText}`)
      }

      const data = await response.json()
      return data.bot
    } catch (error) {
      console.error('Error creating DCA bot:', error)
      throw error
    }
  }

  /**
   * Get all DCA bots
   */
  async getDCABots(): Promise<DCABot[]> {
    try {
      const response = await fetch(`${this.apiUrl}/bots/dca`)

      if (!response.ok) {
        throw new Error(`Failed to fetch DCA bots: ${response.statusText}`)
      }

      const data = await response.json()
      return data.bots || []
    } catch (error) {
      console.error('Error fetching DCA bots:', error)
      throw error
    }
  }

  /**
   * Start/Stop DCA bot
   */
  async toggleDCABot(id: string, active: boolean): Promise<DCABot> {
    try {
      const response = await fetch(`${this.apiUrl}/bots/dca/${id}/toggle`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ active }),
      })

      if (!response.ok) {
        throw new Error(`Failed to toggle DCA bot: ${response.statusText}`)
      }

      const data = await response.json()
      return data.bot
    } catch (error) {
      console.error('Error toggling DCA bot:', error)
      throw error
    }
  }

  /**
   * Delete DCA bot
   */
  async deleteDCABot(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/bots/dca/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error(`Failed to delete DCA bot: ${response.statusText}`)
      }
    } catch (error) {
      console.error('Error deleting DCA bot:', error)
      throw error
    }
  }

  /**
   * Get DCA bot performance metrics
   */
  async getDCABotMetrics(id: string): Promise<{
    total_invested: number
    total_tokens: number
    average_price: number
    current_value: number
    pnl: number
    pnl_percentage: number
    execution_history: Array<{
      amount: number
      price: number
      timestamp: string
    }>
  }> {
    try {
      const response = await fetch(`${this.apiUrl}/bots/dca/${id}/metrics`)

      if (!response.ok) {
        throw new Error(`Failed to fetch DCA bot metrics: ${response.statusText}`)
      }

      const data = await response.json()
      return data.metrics
    } catch (error) {
      console.error('Error fetching DCA bot metrics:', error)
      throw error
    }
  }

  /**
   * Evaluate alert condition
   */
  evaluateCondition(
    condition: 'above' | 'below' | 'crosses_above' | 'crosses_below',
    currentPrice: number,
    targetPrice: number,
    previousPrice?: number
  ): boolean {
    switch (condition) {
      case 'above':
        return currentPrice > targetPrice
      case 'below':
        return currentPrice < targetPrice
      case 'crosses_above':
        return (
          previousPrice !== undefined &&
          previousPrice <= targetPrice &&
          currentPrice > targetPrice
        )
      case 'crosses_below':
        return (
          previousPrice !== undefined &&
          previousPrice >= targetPrice &&
          currentPrice < targetPrice
        )
      default:
        return false
    }
  }
}

// Export singleton instance
export const alertService = new AlertService()
