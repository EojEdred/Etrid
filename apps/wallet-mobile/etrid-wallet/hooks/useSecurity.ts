"use client"

import { useState, useEffect, useCallback } from "react"
import { SecuritySettings, SecurityScore, SecurityEvent, PanicModeStatus } from "@/lib/types/security"
import { securityService } from "@/lib/services/SecurityService"
import { toast } from "sonner"

export function useSecurity() {
  const [settings, setSettings] = useState<SecuritySettings | null>(null)
  const [score, setScore] = useState<SecurityScore | null>(null)
  const [events, setEvents] = useState<SecurityEvent[]>([])
  const [panicMode, setPanicMode] = useState<PanicModeStatus | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch security settings
  const fetchSettings = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await securityService.getSecuritySettings()
      setSettings(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch settings"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Fetch security score
  const fetchScore = useCallback(async () => {
    try {
      const data = await securityService.getSecurityScore()
      setScore(data)
    } catch (err) {
      console.error("Failed to fetch security score:", err)
    }
  }, [])

  // Fetch security events
  const fetchEvents = useCallback(async (limit = 50) => {
    try {
      const data = await securityService.getSecurityEvents(limit)
      setEvents(data)
    } catch (err) {
      console.error("Failed to fetch security events:", err)
    }
  }, [])

  // Fetch panic mode status
  const fetchPanicMode = useCallback(async () => {
    try {
      const data = await securityService.getPanicModeStatus()
      setPanicMode(data)
    } catch (err) {
      console.error("Failed to fetch panic mode status:", err)
    }
  }, [])

  // Update settings
  const updateSettings = useCallback(async (newSettings: Partial<SecuritySettings>) => {
    setLoading(true)
    setError(null)
    try {
      await securityService.updateSettings(newSettings)
      await fetchSettings() // Refresh settings
      await fetchScore() // Refresh score
      toast.success("Security settings updated!")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to update settings"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchSettings, fetchScore])

  // Activate panic mode
  const activatePanicMode = useCallback(async (reason?: string) => {
    setLoading(true)
    setError(null)
    try {
      await securityService.activatePanicMode(reason)
      await fetchPanicMode()
      toast.error("Panic mode activated! All transactions are frozen.")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to activate panic mode"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchPanicMode])

  // Deactivate panic mode
  const deactivatePanicMode = useCallback(async (guardianApprovals: any[]) => {
    setLoading(true)
    setError(null)
    try {
      await securityService.deactivatePanicMode(guardianApprovals)
      await fetchPanicMode()
      toast.success("Panic mode deactivated")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to deactivate panic mode"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchPanicMode])

  // Set biometrics
  const setBiometrics = useCallback(async (enabled: boolean) => {
    setLoading(true)
    setError(null)
    try {
      await securityService.setBiometrics(enabled)
      await fetchSettings()
      await fetchScore()
      toast.success(`Biometrics ${enabled ? "enabled" : "disabled"}`)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to update biometrics"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchSettings, fetchScore])

  // Set 2FA
  const setTwoFactor = useCallback(async (enabled: boolean) => {
    setLoading(true)
    setError(null)
    try {
      await securityService.setTwoFactor(enabled)
      await fetchSettings()
      await fetchScore()
      toast.success(`Two-factor authentication ${enabled ? "enabled" : "disabled"}`)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to update 2FA"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchSettings, fetchScore])

  // Initial fetch
  useEffect(() => {
    fetchSettings()
    fetchScore()
    fetchEvents()
    fetchPanicMode()
  }, [fetchSettings, fetchScore, fetchEvents, fetchPanicMode])

  return {
    settings,
    score,
    events,
    panicMode,
    loading,
    error,
    updateSettings,
    activatePanicMode,
    deactivatePanicMode,
    setBiometrics,
    setTwoFactor,
    fetchSettings,
    fetchScore,
    fetchEvents,
  }
}
