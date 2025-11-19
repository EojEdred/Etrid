'use client'

import { useState, useEffect } from 'react'
import { riskService } from '@/lib/services/RiskService'
import { RiskMetrics } from '@/lib/types/analytics'

export function useRiskMetrics() {
  const [riskMetrics, setRiskMetrics] = useState<RiskMetrics | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadRiskMetrics()
  }, [])

  const loadRiskMetrics = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await riskService.getRiskMetrics()
      setRiskMetrics(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load risk metrics')
    } finally {
      setLoading(false)
    }
  }

  return {
    riskMetrics,
    loading,
    error,
    refresh: loadRiskMetrics,
  }
}
