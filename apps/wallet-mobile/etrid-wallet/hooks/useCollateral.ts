"use client"

import { useState, useEffect, useCallback } from "react"
import { collateralService } from "@/lib/services/CollateralService"
import type { CollateralPosition, CryptoAsset, HealthFactorSimulation } from "@/lib/types/features"
import { toast } from "sonner"

export function useCollateral() {
  const [positions, setPositions] = useState<CollateralPosition[]>([])
  const [healthFactor, setHealthFactor] = useState<number>(0)
  const [totalValue, setTotalValue] = useState<number>(0)
  const [spendingLimit, setSpendingLimit] = useState<number>(0)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * Load collateral positions
   */
  const loadCollateral = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const data = await collateralService.getCollateral()
      setPositions(data)

      // Calculate total value
      const total = data.reduce((sum, pos) => sum + pos.valueUsd, 0)
      setTotalValue(total)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load collateral"
      setError(errorMsg)
      // Don't show toast for initial load failures
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Load health factor
   */
  const loadHealthFactor = useCallback(async () => {
    try {
      const hf = await collateralService.calculateHealthFactor()
      setHealthFactor(hf)
    } catch (err) {
      console.error("Failed to load health factor:", err)
    }
  }, [])

  /**
   * Load spending limit
   */
  const loadSpendingLimit = useCallback(async () => {
    try {
      const limit = await collateralService.getSpendingLimit()
      setSpendingLimit(limit)
    } catch (err) {
      console.error("Failed to load spending limit:", err)
    }
  }, [])

  /**
   * Add collateral
   */
  const addCollateral = useCallback(
    async (asset: CryptoAsset, amount: number) => {
      setIsLoading(true)
      setError(null)

      try {
        const newPosition = await collateralService.addCollateral(asset, amount)
        toast.success(`Added ${amount} ${asset} as collateral`)

        // Reload all data
        await Promise.all([loadCollateral(), loadHealthFactor(), loadSpendingLimit()])

        return newPosition
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to add collateral"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [loadCollateral, loadHealthFactor, loadSpendingLimit],
  )

  /**
   * Withdraw collateral
   */
  const withdrawCollateral = useCallback(
    async (asset: CryptoAsset, amount: number) => {
      setIsLoading(true)
      setError(null)

      try {
        // Check if withdrawal is safe
        const canWithdraw = await collateralService.canWithdraw(asset, amount)
        if (!canWithdraw.canWithdraw) {
          toast.error(canWithdraw.reason || "Cannot withdraw this amount")
          throw new Error(canWithdraw.reason || "Withdrawal not safe")
        }

        await collateralService.withdrawCollateral(asset, amount)
        toast.success(`Withdrew ${amount} ${asset} from collateral`)

        // Reload all data
        await Promise.all([loadCollateral(), loadHealthFactor(), loadSpendingLimit()])
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to withdraw collateral"
        setError(errorMsg)
        if (!err.message.includes("Cannot withdraw")) {
          toast.error(errorMsg)
        }
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [loadCollateral, loadHealthFactor, loadSpendingLimit],
  )

  /**
   * Simulate withdrawal
   */
  const simulateWithdrawal = useCallback(async (asset: CryptoAsset, amount: number): Promise<HealthFactorSimulation | null> => {
    setIsLoading(true)
    setError(null)

    try {
      const simulation = await collateralService.simulateWithdrawal(asset, amount)
      return simulation
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to simulate withdrawal"
      setError(errorMsg)
      toast.error(errorMsg)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Get health factor status
   */
  const getHealthFactorStatus = useCallback(() => {
    return collateralService.getHealthFactorStatus(healthFactor)
  }, [healthFactor])

  /**
   * Get collateral breakdown
   */
  const getCollateralBreakdown = useCallback(async () => {
    try {
      return await collateralService.getCollateralBreakdown()
    } catch (err) {
      console.error("Failed to get collateral breakdown:", err)
      return []
    }
  }, [])

  /**
   * Get position by asset
   */
  const getPosition = useCallback(
    (asset: CryptoAsset): CollateralPosition | null => {
      return positions.find((pos) => pos.asset === asset) || null
    },
    [positions],
  )

  /**
   * Check if health factor is healthy
   */
  const isHealthy = useCallback((): boolean => {
    return healthFactor >= collateralService.safeHealthFactor
  }, [healthFactor])

  /**
   * Check if health factor is in warning zone
   */
  const isWarning = useCallback((): boolean => {
    return healthFactor >= collateralService.warningHealthFactor && healthFactor < collateralService.safeHealthFactor
  }, [healthFactor])

  /**
   * Check if health factor is in danger zone
   */
  const isDanger = useCallback((): boolean => {
    return healthFactor < collateralService.warningHealthFactor
  }, [healthFactor])

  // Load data on mount
  useEffect(() => {
    loadCollateral()
    loadHealthFactor()
    loadSpendingLimit()
  }, [loadCollateral, loadHealthFactor, loadSpendingLimit])

  return {
    // State
    positions,
    healthFactor,
    totalValue,
    spendingLimit,
    isLoading,
    error,

    // Actions
    loadCollateral,
    loadHealthFactor,
    loadSpendingLimit,
    addCollateral,
    withdrawCollateral,
    simulateWithdrawal,
    getCollateralBreakdown,

    // Helpers
    getHealthFactorStatus,
    getPosition,
    isHealthy,
    isWarning,
    isDanger,
  }
}
