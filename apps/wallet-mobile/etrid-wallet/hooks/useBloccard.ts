"use client"

import { useState, useEffect, useCallback } from "react"
import { bloccardService } from "@/lib/services/BloccardService"
import type { BloccardAccount, BloccardStatus, SpendingLimit, CardType } from "@/lib/types/features"
import { toast } from "sonner"

export function useBloccard() {
  const [account, setAccount] = useState<BloccardAccount | null>(null)
  const [status, setStatus] = useState<BloccardStatus | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * Load card status and details
   */
  const loadStatus = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const data = await bloccardService.getCardStatus()
      setStatus(data)
      setAccount(data.account)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load card status"
      setError(errorMsg)
      // Don't show toast for initial load failures (user might not have a card yet)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Apply for a new card
   */
  const applyForCard = useCallback(async (cardType: CardType = "virtual") => {
    setIsLoading(true)
    setError(null)

    try {
      const newAccount = await bloccardService.applyForCard(cardType)
      setAccount(newAccount)
      toast.success("Card application submitted successfully!")

      // Reload full status
      loadStatus()

      return newAccount
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to apply for card"
      setError(errorMsg)
      toast.error(errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Freeze the card
   */
  const freezeCard = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      await bloccardService.freezeCard()
      if (account) {
        setAccount({ ...account, status: "frozen" })
      }
      toast.success("Card frozen successfully")
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to freeze card"
      setError(errorMsg)
      toast.error(errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [account])

  /**
   * Unfreeze the card
   */
  const unfreezeCard = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      await bloccardService.unfreezeCard()
      if (account) {
        setAccount({ ...account, status: "active" })
      }
      toast.success("Card unfrozen successfully")
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to unfreeze card"
      setError(errorMsg)
      toast.error(errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [account])

  /**
   * Update spending limits
   */
  const updateSpendingLimits = useCallback(
    async (limits: SpendingLimit) => {
      setIsLoading(true)
      setError(null)

      try {
        await bloccardService.setSpendingLimit(limits)
        if (account) {
          setAccount({
            ...account,
            dailyLimit: limits.daily || account.dailyLimit,
            weeklyLimit: limits.weekly || account.weeklyLimit,
            monthlyLimit: limits.monthly || account.monthlyLimit,
          })
        }
        toast.success("Spending limits updated")
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to update spending limits"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [account],
  )

  /**
   * Get spending analytics
   */
  const getSpendingAnalytics = useCallback(async (startDate: Date, endDate: Date) => {
    setIsLoading(true)
    setError(null)

    try {
      const analytics = await bloccardService.getSpendingAnalytics(startDate, endDate)
      return analytics
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load spending analytics"
      setError(errorMsg)
      toast.error(errorMsg)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Request physical card
   */
  const requestPhysicalCard = useCallback(async (shippingAddress: {
    line1: string
    line2?: string
    city: string
    state: string
    postalCode: string
    country: string
  }) => {
    setIsLoading(true)
    setError(null)

    try {
      const result = await bloccardService.requestPhysicalCard(shippingAddress)
      toast.success("Physical card requested! Check your email for tracking details.")
      return result
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to request physical card"
      setError(errorMsg)
      toast.error(errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Check if user has a card
   */
  const hasCard = useCallback((): boolean => {
    return account !== null
  }, [account])

  /**
   * Check if card is active
   */
  const isCardActive = useCallback((): boolean => {
    return account?.status === "active"
  }, [account])

  /**
   * Check if card is frozen
   */
  const isCardFrozen = useCallback((): boolean => {
    return account?.status === "frozen"
  }, [account])

  // Load status on mount
  useEffect(() => {
    loadStatus()
  }, [loadStatus])

  return {
    // State
    account,
    status,
    isLoading,
    error,

    // Actions
    loadStatus,
    applyForCard,
    freezeCard,
    unfreezeCard,
    updateSpendingLimits,
    getSpendingAnalytics,
    requestPhysicalCard,

    // Helpers
    hasCard,
    isCardActive,
    isCardFrozen,
  }
}
