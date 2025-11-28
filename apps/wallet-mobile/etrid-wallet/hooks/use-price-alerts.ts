// usePriceAlerts Hook - Manage price alerts

import { useState, useEffect, useCallback } from 'react'
import { alertService } from '@/lib/services/alert.service'
import type { Alert, AlertInput } from '@/lib/types/trading'

export function usePriceAlerts() {
  const [alerts, setAlerts] = useState<Alert[]>([])
  const [activeAlerts, setActiveAlerts] = useState<Alert[]>([])
  const [triggeredAlerts, setTriggeredAlerts] = useState<Alert[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchAlerts = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const [active, triggered] = await Promise.all([
        alertService.getAlerts('active'),
        alertService.getAlerts('triggered'),
      ])

      setActiveAlerts(active)
      setTriggeredAlerts(triggered)
      setAlerts([...active, ...triggered])
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchAlerts()
  }, [fetchAlerts])

  const createAlert = useCallback(
    async (alert: AlertInput) => {
      try {
        const newAlert = await alertService.createAlert(alert)
        await fetchAlerts()
        return newAlert
      } catch (err) {
        throw err
      }
    },
    [fetchAlerts]
  )

  const deleteAlert = useCallback(
    async (id: string) => {
      try {
        await alertService.deleteAlert(id)
        await fetchAlerts()
      } catch (err) {
        throw err
      }
    },
    [fetchAlerts]
  )

  const pauseAlert = useCallback(
    async (id: string) => {
      try {
        await alertService.updateAlert(id, 'cancelled')
        await fetchAlerts()
      } catch (err) {
        throw err
      }
    },
    [fetchAlerts]
  )

  const resumeAlert = useCallback(
    async (id: string) => {
      try {
        await alertService.updateAlert(id, 'active')
        await fetchAlerts()
      } catch (err) {
        throw err
      }
    },
    [fetchAlerts]
  )

  const checkAlerts = useCallback(
    async (pair: string, currentPrice: number) => {
      try {
        const triggered = await alertService.checkAlerts(pair, currentPrice)
        if (triggered.length > 0) {
          await fetchAlerts()
        }
        return triggered
      } catch (err) {
        throw err
      }
    },
    [fetchAlerts]
  )

  return {
    alerts,
    activeAlerts,
    triggeredAlerts,
    loading,
    error,
    createAlert,
    deleteAlert,
    pauseAlert,
    resumeAlert,
    checkAlerts,
    refresh: fetchAlerts,
  }
}
