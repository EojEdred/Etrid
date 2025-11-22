// Trading Service - Handle order execution and management

import type {
  Order,
  OrderInput,
  OrderBook,
  OrderFilters,
  Trade,
  Position,
} from '@/lib/types/trading'

export class TradingService {
  private apiUrl: string
  private wsUrl: string

  constructor(
    apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api',
    wsUrl: string = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3000/ws'
  ) {
    this.apiUrl = apiUrl
    this.wsUrl = wsUrl
  }

  /**
   * Place a new order
   */
  async placeOrder(order: OrderInput): Promise<Order> {
    try {
      // Validate order
      this.validateOrder(order)

      const response = await fetch(`${this.apiUrl}/trading/order`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(order),
      })

      if (!response.ok) {
        const errorData = await response.json()
        throw new Error(errorData.message || `Failed to place order: ${response.statusText}`)
      }

      const data = await response.json()
      return data.order
    } catch (error) {
      console.error('Error placing order:', error)
      throw error
    }
  }

  /**
   * Cancel an order
   */
  async cancelOrder(orderId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/trading/order/${orderId}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error(`Failed to cancel order: ${response.statusText}`)
      }
    } catch (error) {
      console.error('Error cancelling order:', error)
      throw error
    }
  }

  /**
   * Get open orders
   */
  async getOpenOrders(pair?: string): Promise<Order[]> {
    try {
      const params = new URLSearchParams({ status: 'open' })
      if (pair) {
        params.append('pair', pair)
      }

      const response = await fetch(`${this.apiUrl}/trading/orders?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch open orders: ${response.statusText}`)
      }

      const data = await response.json()
      return data.orders || []
    } catch (error) {
      console.error('Error fetching open orders:', error)
      throw error
    }
  }

  /**
   * Get order history
   */
  async getOrderHistory(filters?: OrderFilters): Promise<Order[]> {
    try {
      const params = new URLSearchParams()

      if (filters) {
        Object.entries(filters).forEach(([key, value]) => {
          if (value !== undefined) {
            params.append(key, value.toString())
          }
        })
      }

      const response = await fetch(`${this.apiUrl}/trading/orders?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch order history: ${response.statusText}`)
      }

      const data = await response.json()
      return data.orders || []
    } catch (error) {
      console.error('Error fetching order history:', error)
      throw error
    }
  }

  /**
   * Get order book for a trading pair
   */
  async getOrderBook(pair: string, depth: number = 20): Promise<OrderBook> {
    try {
      const params = new URLSearchParams({
        pair,
        depth: depth.toString(),
      })

      const response = await fetch(`${this.apiUrl}/trading/orderbook?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch order book: ${response.statusText}`)
      }

      const data = await response.json()
      return data.orderbook
    } catch (error) {
      console.error('Error fetching order book:', error)
      throw error
    }
  }

  /**
   * Get recent trades for a pair
   */
  async getRecentTrades(pair: string, limit: number = 50): Promise<Trade[]> {
    try {
      const params = new URLSearchParams({
        pair,
        limit: limit.toString(),
      })

      const response = await fetch(`${this.apiUrl}/trading/trades?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch recent trades: ${response.statusText}`)
      }

      const data = await response.json()
      return data.trades || []
    } catch (error) {
      console.error('Error fetching recent trades:', error)
      throw error
    }
  }

  /**
   * Get open positions
   */
  async getPositions(): Promise<Position[]> {
    try {
      const response = await fetch(`${this.apiUrl}/trading/positions`)

      if (!response.ok) {
        throw new Error(`Failed to fetch positions: ${response.statusText}`)
      }

      const data = await response.json()
      return data.positions || []
    } catch (error) {
      console.error('Error fetching positions:', error)
      throw error
    }
  }

  /**
   * Close a position
   */
  async closePosition(positionId: string): Promise<Order> {
    try {
      const response = await fetch(`${this.apiUrl}/trading/position/${positionId}/close`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error(`Failed to close position: ${response.statusText}`)
      }

      const data = await response.json()
      return data.order
    } catch (error) {
      console.error('Error closing position:', error)
      throw error
    }
  }

  /**
   * Set take profit and stop loss for a position
   */
  async setTPSL(
    positionId: string,
    takeProfit?: number,
    stopLoss?: number
  ): Promise<Position> {
    try {
      const response = await fetch(`${this.apiUrl}/trading/position/${positionId}/tpsl`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          take_profit: takeProfit,
          stop_loss: stopLoss,
        }),
      })

      if (!response.ok) {
        throw new Error(`Failed to set TP/SL: ${response.statusText}`)
      }

      const data = await response.json()
      return data.position
    } catch (error) {
      console.error('Error setting TP/SL:', error)
      throw error
    }
  }

  /**
   * Modify an existing order
   */
  async modifyOrder(
    orderId: string,
    updates: { price?: number; amount?: number }
  ): Promise<Order> {
    try {
      const response = await fetch(`${this.apiUrl}/trading/order/${orderId}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      })

      if (!response.ok) {
        throw new Error(`Failed to modify order: ${response.statusText}`)
      }

      const data = await response.json()
      return data.order
    } catch (error) {
      console.error('Error modifying order:', error)
      throw error
    }
  }

  /**
   * Get order details
   */
  async getOrder(orderId: string): Promise<Order> {
    try {
      const response = await fetch(`${this.apiUrl}/trading/order/${orderId}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch order: ${response.statusText}`)
      }

      const data = await response.json()
      return data.order
    } catch (error) {
      console.error('Error fetching order:', error)
      throw error
    }
  }

  /**
   * Calculate order total
   */
  calculateOrderTotal(amount: number, price: number, side: 'buy' | 'sell'): {
    total: number
    fee: number
    net: number
  } {
    const total = amount * price
    const feePercentage = 0.001 // 0.1% fee
    const fee = total * feePercentage

    return {
      total,
      fee,
      net: side === 'buy' ? total + fee : total - fee,
    }
  }

  /**
   * Validate order input
   */
  private validateOrder(order: OrderInput): void {
    if (order.amount <= 0) {
      throw new Error('Order amount must be greater than 0')
    }

    if (order.type === 'limit' && (!order.price || order.price <= 0)) {
      throw new Error('Limit orders must have a valid price')
    }

    if (order.type === 'stop_loss' && (!order.stop_price || order.stop_price <= 0)) {
      throw new Error('Stop loss orders must have a valid stop price')
    }

    if (order.type === 'stop_limit') {
      if (!order.price || order.price <= 0) {
        throw new Error('Stop limit orders must have a valid limit price')
      }
      if (!order.stop_price || order.stop_price <= 0) {
        throw new Error('Stop limit orders must have a valid stop price')
      }
    }
  }

  /**
   * Subscribe to order updates via WebSocket
   */
  subscribeToOrders(callback: (order: Order) => void): () => void {
    const ws = new WebSocket(`${this.wsUrl}/orders`)

    ws.onmessage = (event) => {
      try {
        const order = JSON.parse(event.data)
        callback(order)
      } catch (error) {
        console.error('Error parsing order update:', error)
      }
    }

    ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    // Return cleanup function
    return () => {
      ws.close()
    }
  }
}

// Export singleton instance
export const tradingService = new TradingService()
