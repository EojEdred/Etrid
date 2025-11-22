"use client"

import { useState, useEffect, useCallback } from "react"
import { MixingSession, MixingStatus, MIXING_FEE_PERCENT } from "@/lib/types/privacy"
import { privacyService } from "@/lib/services/PrivacyService"
import { toast } from "sonner"

export function useCoinMix() {
  const [sessions, setSessions] = useState<MixingSession[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch mixing sessions
  const fetchSessions = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await privacyService.getMixingSessions()
      setSessions(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch sessions"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Start mixing
  const startMixing = useCallback(async (amount: string, rounds: number) => {
    setLoading(true)
    setError(null)
    try {
      const session = await privacyService.mixCoins(amount, rounds)
      setSessions((prev) => [session, ...prev])
      toast.success("Mixing session started!")
      return session
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to start mixing"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [])

  // Get mixing status
  const getMixingStatus = useCallback(async (sessionId: string): Promise<MixingStatus> => {
    try {
      return await privacyService.getMixingStatus(sessionId)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch status"
      console.error(message)
      throw err
    }
  }, [])

  // Cancel mixing
  const cancelMixing = useCallback(async (sessionId: string) => {
    setLoading(true)
    setError(null)
    try {
      await privacyService.cancelMixing(sessionId)
      await fetchSessions() // Refresh sessions
      toast.info("Mixing session cancelled")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to cancel mixing"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchSessions])

  // Calculate mixing fee
  const calculateFee = useCallback((amount: string, rounds: number): string => {
    const amountNum = parseFloat(amount)
    const totalFee = amountNum * (MIXING_FEE_PERCENT / 100) * rounds
    return totalFee.toFixed(2)
  }, [])

  // Get active sessions
  const getActiveSessions = useCallback(() => {
    return sessions.filter(
      (session) => session.status === "pending" || session.status === "mixing"
    )
  }, [sessions])

  // Initial fetch
  useEffect(() => {
    fetchSessions()
  }, [fetchSessions])

  return {
    sessions,
    loading,
    error,
    startMixing,
    getMixingStatus,
    cancelMixing,
    calculateFee,
    getActiveSessions,
    fetchSessions,
  }
}
