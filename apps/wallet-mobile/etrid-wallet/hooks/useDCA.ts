"use client"

import { useState, useEffect, useCallback } from "react"
import { dcaService } from "@/lib/services/DCAService"
import type { DCASchedule, CryptoAsset, DCAFrequency } from "@/lib/types/features"
import { toast } from "sonner"

export function useDCA() {
  const [schedules, setSchedules] = useState<DCASchedule[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [stats, setStats] = useState<{
    totalSchedules: number
    activeSchedules: number
    totalInvested: number
    totalPurchases: number
  } | null>(null)

  /**
   * Load all DCA schedules
   */
  const loadSchedules = useCallback(async (includeInactive: boolean = false) => {
    setIsLoading(true)
    setError(null)

    try {
      const data = await dcaService.getSchedules(includeInactive)
      setSchedules(data)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load DCA schedules"
      setError(errorMsg)
      toast.error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Create a new DCA schedule
   */
  const createSchedule = useCallback(
    async (schedule: {
      asset: CryptoAsset
      amountUsd: number
      frequency: DCAFrequency
      paymentMethodId: string
      startDate: Date
      endDate?: Date
    }) => {
      setIsLoading(true)
      setError(null)

      try {
        const newSchedule = await dcaService.createSchedule(schedule)
        setSchedules((prev) => [...prev, newSchedule])
        toast.success("DCA schedule created successfully")

        // Reload stats
        loadStats()

        return newSchedule
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to create DCA schedule"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Update a DCA schedule
   */
  const updateSchedule = useCallback(
    async (
      id: string,
      updates: {
        amountUsd?: number
        frequency?: DCAFrequency
        paymentMethodId?: string
        endDate?: Date
      },
    ) => {
      setIsLoading(true)
      setError(null)

      try {
        const updated = await dcaService.updateSchedule(id, updates)
        setSchedules((prev) => prev.map((s) => (s.id === id ? updated : s)))
        toast.success("DCA schedule updated")
        return updated
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to update DCA schedule"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Pause a DCA schedule
   */
  const pauseSchedule = useCallback(
    async (id: string) => {
      setIsLoading(true)
      setError(null)

      try {
        await dcaService.pauseSchedule(id)
        setSchedules((prev) =>
          prev.map((s) => (s.id === id ? { ...s, isActive: false } : s)),
        )
        toast.success("DCA schedule paused")
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to pause DCA schedule"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Resume a paused DCA schedule
   */
  const resumeSchedule = useCallback(
    async (id: string) => {
      setIsLoading(true)
      setError(null)

      try {
        await dcaService.resumeSchedule(id)
        setSchedules((prev) =>
          prev.map((s) => (s.id === id ? { ...s, isActive: true } : s)),
        )
        toast.success("DCA schedule resumed")
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to resume DCA schedule"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Delete a DCA schedule
   */
  const deleteSchedule = useCallback(
    async (id: string) => {
      setIsLoading(true)
      setError(null)

      try {
        await dcaService.deleteSchedule(id)
        setSchedules((prev) => prev.filter((s) => s.id !== id))
        toast.success("DCA schedule deleted")

        // Reload stats
        loadStats()
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to delete DCA schedule"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Load DCA statistics
   */
  const loadStats = useCallback(async () => {
    try {
      const data = await dcaService.getStats()
      setStats(data)
    } catch (err) {
      console.error("Failed to load DCA stats:", err)
    }
  }, [])

  /**
   * Get schedule by ID
   */
  const getSchedule = useCallback(
    (id: string): DCASchedule | null => {
      return schedules.find((s) => s.id === id) || null
    },
    [schedules],
  )

  /**
   * Get active schedules
   */
  const getActiveSchedules = useCallback((): DCASchedule[] => {
    return schedules.filter((s) => s.isActive)
  }, [schedules])

  // Load schedules and stats on mount
  useEffect(() => {
    loadSchedules(false)
    loadStats()
  }, [loadSchedules, loadStats])

  return {
    // State
    schedules,
    stats,
    isLoading,
    error,

    // Actions
    loadSchedules,
    createSchedule,
    updateSchedule,
    pauseSchedule,
    resumeSchedule,
    deleteSchedule,
    loadStats,
    getSchedule,
    getActiveSchedules,
  }
}
