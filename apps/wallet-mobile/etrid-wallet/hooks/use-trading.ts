// useTrading Hook - Manage orders and positions

import { useState, useEffect, useCallback } from 'react'
import { tradingService } from '@/lib/services/trading.service'
import type { Order, OrderInput, Position, OrderFilters } from '@/lib/types/trading'

export function useTrading(pair?: string) {
  const [openOrders, setOpenOrders] = useState<Order[]>([])
  const [orderHistory, setOrderHistory] = useState<Order[]>([])
  const [positions, setPositions] = useState<Position[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchOpenOrders = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const data = await tradingService.getOpenOrders(pair)
      setOpenOrders(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [pair])

  const fetchOrderHistory = useCallback(async (filters?: OrderFilters) => {
    try {
      const data = await tradingService.getOrderHistory(filters)
      setOrderHistory(data)
    } catch (err) {
      setError(err as Error)
    }
  }, [])

  const fetchPositions = useCallback(async () => {
    try {
      const data = await tradingService.getPositions()
      setPositions(data)
    } catch (err) {
      setError(err as Error)
    }
  }, [])

  useEffect(() => {
    fetchOpenOrders()
    fetchPositions()
  }, [fetchOpenOrders, fetchPositions])

  const placeOrder = useCallback(
    async (order: OrderInput) => {
      try {
        const newOrder = await tradingService.placeOrder(order)
        await fetchOpenOrders()
        await fetchPositions()
        return newOrder
      } catch (err) {
        throw err
      }
    },
    [fetchOpenOrders, fetchPositions]
  )

  const cancelOrder = useCallback(
    async (orderId: string) => {
      try {
        await tradingService.cancelOrder(orderId)
        await fetchOpenOrders()
      } catch (err) {
        throw err
      }
    },
    [fetchOpenOrders]
  )

  const modifyOrder = useCallback(
    async (orderId: string, updates: { price?: number; amount?: number }) => {
      try {
        const updatedOrder = await tradingService.modifyOrder(orderId, updates)
        await fetchOpenOrders()
        return updatedOrder
      } catch (err) {
        throw err
      }
    },
    [fetchOpenOrders]
  )

  const closePosition = useCallback(
    async (positionId: string) => {
      try {
        await tradingService.closePosition(positionId)
        await fetchPositions()
        await fetchOpenOrders()
      } catch (err) {
        throw err
      }
    },
    [fetchPositions, fetchOpenOrders]
  )

  const setTPSL = useCallback(
    async (positionId: string, takeProfit?: number, stopLoss?: number) => {
      try {
        const updatedPosition = await tradingService.setTPSL(
          positionId,
          takeProfit,
          stopLoss
        )
        await fetchPositions()
        return updatedPosition
      } catch (err) {
        throw err
      }
    },
    [fetchPositions]
  )

  const calculateOrderTotal = useCallback(
    (amount: number, price: number, side: 'buy' | 'sell') => {
      return tradingService.calculateOrderTotal(amount, price, side)
    },
    []
  )

  // Subscribe to order updates
  useEffect(() => {
    const unsubscribe = tradingService.subscribeToOrders((order) => {
      setOpenOrders((prev) => {
        const index = prev.findIndex((o) => o.id === order.id)
        if (index >= 0) {
          // Update existing order
          const newOrders = [...prev]
          newOrders[index] = order
          return newOrders
        }
        // Add new order
        return [...prev, order]
      })
    })

    return () => {
      unsubscribe()
    }
  }, [])

  return {
    openOrders,
    orderHistory,
    positions,
    loading,
    error,
    placeOrder,
    cancelOrder,
    modifyOrder,
    closePosition,
    setTPSL,
    calculateOrderTotal,
    fetchOrderHistory,
    refresh: fetchOpenOrders,
  }
}
