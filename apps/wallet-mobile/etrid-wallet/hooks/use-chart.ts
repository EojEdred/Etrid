// useChart Hook - Manage price charts and indicators

import { useState, useEffect, useCallback, useRef } from 'react'
import { chartService } from '@/lib/services/chart.service'
import type {
  Candle,
  CandleInterval,
  Trade,
  IndicatorConfig,
  TradingPair,
} from '@/lib/types/trading'

export function useChart(pair: string, interval: CandleInterval = '1h') {
  const [candles, setCandles] = useState<Candle[]>([])
  const [currentPrice, setCurrentPrice] = useState<number>(0)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  const [indicators, setIndicators] = useState<IndicatorConfig[]>([])
  const subscriptionRef = useRef<{ unsubscribe: () => void } | null>(null)

  const fetchCandles = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const data = await chartService.getCandles(pair, interval, 500)
      setCandles(data)

      if (data.length > 0) {
        setCurrentPrice(data[data.length - 1].close)
      }
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [pair, interval])

  useEffect(() => {
    fetchCandles()
  }, [fetchCandles])

  // Subscribe to real-time candle updates
  useEffect(() => {
    // Unsubscribe from previous subscription
    if (subscriptionRef.current) {
      subscriptionRef.current.unsubscribe()
    }

    const subscription = chartService.subscribeToCandles(
      pair,
      interval,
      (newCandle) => {
        setCandles((prev) => {
          const updated = [...prev]
          const lastIndex = updated.length - 1

          // Update last candle if timestamps match, otherwise add new
          if (
            lastIndex >= 0 &&
            updated[lastIndex].timestamp === newCandle.timestamp
          ) {
            updated[lastIndex] = newCandle
          } else {
            updated.push(newCandle)
            // Keep only last 500 candles
            if (updated.length > 500) {
              updated.shift()
            }
          }

          return updated
        })

        setCurrentPrice(newCandle.close)
      }
    )

    subscriptionRef.current = subscription

    return () => {
      if (subscriptionRef.current) {
        subscriptionRef.current.unsubscribe()
      }
    }
  }, [pair, interval])

  const addIndicator = useCallback((indicator: IndicatorConfig) => {
    setIndicators((prev) => [...prev, indicator])
  }, [])

  const removeIndicator = useCallback((type: string) => {
    setIndicators((prev) => prev.filter((ind) => ind.type !== type))
  }, [])

  const toggleIndicator = useCallback((type: string) => {
    setIndicators((prev) =>
      prev.map((ind) =>
        ind.type === type ? { ...ind, visible: !ind.visible } : ind
      )
    )
  }, [])

  const updateIndicatorParams = useCallback(
    (type: string, params: Record<string, number>) => {
      setIndicators((prev) =>
        prev.map((ind) => (ind.type === type ? { ...ind, params } : ind))
      )
    },
    []
  )

  // Calculate indicator data
  const getIndicatorData = useCallback(
    (type: string) => {
      const indicator = indicators.find((ind) => ind.type === type)
      if (!indicator || !candles.length) return []

      switch (type) {
        case 'SMA':
          return chartService.calculateSMA(candles, indicator.params.period || 20)
        case 'EMA':
          return chartService.calculateEMA(candles, indicator.params.period || 20)
        case 'RSI':
          return chartService.calculateRSI(candles, indicator.params.period || 14)
        case 'MACD':
          return chartService.calculateMACD(
            candles,
            indicator.params.fast || 12,
            indicator.params.slow || 26,
            indicator.params.signal || 9
          )
        case 'BOLLINGER_BANDS':
          return chartService.calculateBollingerBands(
            candles,
            indicator.params.period || 20,
            indicator.params.stdDev || 2
          )
        default:
          return []
      }
    },
    [candles, indicators]
  )

  return {
    candles,
    currentPrice,
    loading,
    error,
    indicators,
    addIndicator,
    removeIndicator,
    toggleIndicator,
    updateIndicatorParams,
    getIndicatorData,
    refresh: fetchCandles,
  }
}
