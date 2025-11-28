// useOrderBook Hook - Manage order book data

import { useState, useEffect, useCallback, useRef } from 'react'
import { tradingService } from '@/lib/services/trading.service'
import type { OrderBook, Trade } from '@/lib/types/trading'

export function useOrderBook(pair: string, depth: number = 20) {
  const [orderBook, setOrderBook] = useState<OrderBook | null>(null)
  const [recentTrades, setRecentTrades] = useState<Trade[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  const wsRef = useRef<WebSocket | null>(null)

  const fetchOrderBook = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const data = await tradingService.getOrderBook(pair, depth)
      setOrderBook(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [pair, depth])

  const fetchRecentTrades = useCallback(async () => {
    try {
      const data = await tradingService.getRecentTrades(pair, 50)
      setRecentTrades(data)
    } catch (err) {
      console.error('Error fetching recent trades:', err)
    }
  }, [pair])

  useEffect(() => {
    fetchOrderBook()
    fetchRecentTrades()
  }, [fetchOrderBook, fetchRecentTrades])

  // Subscribe to real-time order book updates
  useEffect(() => {
    const wsUrl =
      process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3000/ws'
    const ws = new WebSocket(`${wsUrl}/orderbook/${pair}`)

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data)
        setOrderBook(data)
      } catch (err) {
        console.error('Error parsing order book update:', err)
      }
    }

    ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    wsRef.current = ws

    return () => {
      if (wsRef.current) {
        wsRef.current.close()
      }
    }
  }, [pair])

  // Subscribe to real-time trades
  useEffect(() => {
    const wsUrl =
      process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3000/ws'
    const ws = new WebSocket(`${wsUrl}/trades/${pair}`)

    ws.onmessage = (event) => {
      try {
        const trade = JSON.parse(event.data)
        setRecentTrades((prev) => {
          const updated = [trade, ...prev]
          // Keep only last 50 trades
          return updated.slice(0, 50)
        })
      } catch (err) {
        console.error('Error parsing trade update:', err)
      }
    }

    ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    return () => {
      ws.close()
    }
  }, [pair])

  const fillOrderFormFromBook = useCallback(
    (price: number, side: 'buy' | 'sell') => {
      // This is a helper function that can be used by the UI
      // to autofill the order form when clicking on order book levels
      return { price, side }
    },
    []
  )

  return {
    orderBook,
    recentTrades,
    loading,
    error,
    fillOrderFormFromBook,
    refresh: fetchOrderBook,
  }
}
