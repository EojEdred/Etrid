"use client"

import { useState, useEffect, useCallback } from "react"
import { SpendingLimit } from "@/lib/types/security"
import { securityService } from "@/lib/services/SecurityService"
import { toast } from "sonner"

export function useSpendingLimits() {
  const [limits, setLimits] = useState<{
    daily: SpendingLimit
    weekly: SpendingLimit
    monthly: SpendingLimit
  } | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch spending limits
  const fetchLimits = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const settings = await securityService.getSecuritySettings()

      // Mock spending limit data (in real app, this would come from backend)
      setLimits({
        daily: {
          period: "daily",
          limit: settings.dailyLimit,
          spent: "0", // Would come from backend
          remaining: settings.dailyLimit,
          resetsAt: new Date(Date.now() + 24 * 60 * 60 * 1000),
        },
        weekly: {
          period: "weekly",
          limit: settings.weeklyLimit,
          spent: "0",
          remaining: settings.weeklyLimit,
          resetsAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
        },
        monthly: {
          period: "monthly",
          limit: settings.monthlyLimit,
          spent: "0",
          remaining: settings.monthlyLimit,
          resetsAt: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000),
        },
      })
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch limits"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Update limit
  const updateLimit = useCallback(
    async (period: "daily" | "weekly" | "monthly", newLimit: string) => {
      setLoading(true)
      setError(null)
      try {
        const updateKey =
          period === "daily"
            ? "dailyLimit"
            : period === "weekly"
              ? "weeklyLimit"
              : "monthlyLimit"

        await securityService.updateSettings({ [updateKey]: newLimit })
        await fetchLimits() // Refresh limits
        toast.success(`${period.charAt(0).toUpperCase() + period.slice(1)} limit updated!`)
      } catch (err) {
        const message = err instanceof Error ? err.message : "Failed to update limit"
        setError(message)
        toast.error(message)
        throw err
      } finally {
        setLoading(false)
      }
    },
    [fetchLimits]
  )

  // Check if amount exceeds limit
  const checkLimit = useCallback(
    (period: "daily" | "weekly" | "monthly", amount: string): boolean => {
      if (!limits) return false
      const limit = limits[period]
      const totalSpent = parseFloat(limit.spent) + parseFloat(amount)
      return totalSpent > parseFloat(limit.limit)
    },
    [limits]
  )

  // Initial fetch
  useEffect(() => {
    fetchLimits()
  }, [fetchLimits])

  return {
    limits,
    loading,
    error,
    updateLimit,
    checkLimit,
    fetchLimits,
  }
}
