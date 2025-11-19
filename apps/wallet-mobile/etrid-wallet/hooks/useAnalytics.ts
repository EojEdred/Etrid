'use client'

import { useState, useEffect } from 'react'
import { analyticsService } from '@/lib/services/AnalyticsService'
import {
  PortfolioMetrics,
  PerformanceData,
  AllocationData,
  TimePeriod,
} from '@/lib/types/analytics'

export function useAnalytics() {
  const [metrics, setMetrics] = useState<PortfolioMetrics | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadMetrics()
  }, [])

  const loadMetrics = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await analyticsService.getPortfolioMetrics()
      setMetrics(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load analytics')
    } finally {
      setLoading(false)
    }
  }

  const refresh = () => {
    loadMetrics()
  }

  return {
    metrics,
    loading,
    error,
    refresh,
  }
}

export function usePerformance(period: TimePeriod = '30d') {
  const [performance, setPerformance] = useState<PerformanceData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadPerformance()
  }, [period])

  const loadPerformance = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await analyticsService.getPerformance(period)
      setPerformance(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load performance')
    } finally {
      setLoading(false)
    }
  }

  return {
    performance,
    loading,
    error,
    refresh: loadPerformance,
  }
}

export function useAssetAllocation() {
  const [allocation, setAllocation] = useState<AllocationData[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadAllocation()
  }, [])

  const loadAllocation = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await analyticsService.getAssetAllocation()
      setAllocation(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load allocation')
    } finally {
      setLoading(false)
    }
  }

  return {
    allocation,
    loading,
    error,
    refresh: loadAllocation,
  }
}
