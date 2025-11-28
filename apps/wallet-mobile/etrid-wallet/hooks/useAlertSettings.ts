'use client'

import { useState, useEffect } from 'react'
import { alertEngine } from '@/lib/services/AlertEngine'
import { Alert, AlertSettings, AlertType } from '@/lib/types/notifications'

export function useAlertSettings() {
  const [alerts, setAlerts] = useState<Alert[]>([])
  const [settings, setSettings] = useState<AlertSettings | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadAlertData()
  }, [])

  const loadAlertData = async () => {
    try {
      setLoading(true)
      setError(null)
      const [alertsData, settingsData] = await Promise.all([
        alertEngine.getAlerts(),
        alertEngine.getAlertSettings(),
      ])
      setAlerts(alertsData)
      setSettings(settingsData)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load alert settings')
    } finally {
      setLoading(false)
    }
  }

  const createAlert = async (alert: any) => {
    try {
      const newAlert = await alertEngine.createAlert(alert)
      setAlerts((prev) => [...prev, newAlert])
      return newAlert
    } catch (err) {
      throw err
    }
  }

  const updateAlert = async (id: string, updates: Partial<Alert>) => {
    try {
      const updatedAlert = await alertEngine.updateAlert(id, updates)
      setAlerts((prev) => prev.map((a) => (a.id === id ? updatedAlert : a)))
      return updatedAlert
    } catch (err) {
      throw err
    }
  }

  const deleteAlert = async (id: string) => {
    try {
      await alertEngine.deleteAlert(id)
      setAlerts((prev) => prev.filter((a) => a.id !== id))
    } catch (err) {
      throw err
    }
  }

  const toggleAlert = async (id: string) => {
    const alert = alerts.find((a) => a.id === id)
    if (!alert) return

    await updateAlert(id, { enabled: !alert.enabled })
  }

  const updateSettings = async (newSettings: Partial<AlertSettings>) => {
    try {
      await alertEngine.updateAlertSettings(newSettings)
      setSettings((prev) => (prev ? { ...prev, ...newSettings } : null))
    } catch (err) {
      throw err
    }
  }

  const testAlert = async (id: string) => {
    try {
      await alertEngine.testAlert(id)
    } catch (err) {
      throw err
    }
  }

  return {
    alerts,
    settings,
    loading,
    error,
    createAlert,
    updateAlert,
    deleteAlert,
    toggleAlert,
    updateSettings,
    testAlert,
    refresh: loadAlertData,
  }
}
