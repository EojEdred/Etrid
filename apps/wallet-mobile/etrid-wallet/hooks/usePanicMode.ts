"use client"

import { useState, useEffect, useCallback } from "react"
import { PanicModeStatus, GuardianApproval } from "@/lib/types/security"
import { securityService } from "@/lib/services/SecurityService"
import { toast } from "sonner"

export function usePanicMode() {
  const [status, setStatus] = useState<PanicModeStatus | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch panic mode status
  const fetchStatus = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await securityService.getPanicModeStatus()
      setStatus(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch panic mode status"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Activate panic mode
  const activate = useCallback(async (reason?: string) => {
    setLoading(true)
    setError(null)
    try {
      await securityService.activatePanicMode(reason)
      await fetchStatus()
      toast.error("⚠️ PANIC MODE ACTIVATED\nAll transactions are frozen!")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to activate panic mode"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchStatus])

  // Deactivate panic mode
  const deactivate = useCallback(async (guardianApprovals: GuardianApproval[]) => {
    setLoading(true)
    setError(null)
    try {
      await securityService.deactivatePanicMode(guardianApprovals)
      await fetchStatus()
      toast.success("Panic mode deactivated. Normal operations resumed.")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to deactivate panic mode"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchStatus])

  // Initial fetch
  useEffect(() => {
    fetchStatus()
  }, [fetchStatus])

  return {
    status,
    loading,
    error,
    activate,
    deactivate,
    fetchStatus,
  }
}
