/**
 * useHealthFactor - Monitor loan health and liquidation risk
 */

'use client'

import { useState, useEffect, useCallback } from 'react'
import { collateralService, type HealthFactorResult } from '@/lib/services/collateral-service'
import { lendingService } from '@/lib/services/lending-service'

export function useHealthFactor(loanId?: string) {
  const [healthFactor, setHealthFactor] = useState<number>(0)
  const [status, setStatus] = useState<'safe' | 'warning' | 'danger' | 'liquidation'>('safe')
  const [liquidationRisk, setLiquidationRisk] = useState<number>(0)
  const [liquidationPrice, setLiquidationPrice] = useState<number>(0)
  const [loading, setLoading] = useState(false)

  const calculateHealthFactor = useCallback(async (
    collateralValue: number,
    borrowedValue: number
  ): Promise<HealthFactorResult> => {
    return await collateralService.calculateHealthFactor(collateralValue, borrowedValue)
  }, [])

  const loadHealthFactor = useCallback(async () => {
    if (!loanId) return

    try {
      setLoading(true)
      const hf = await lendingService.getHealthFactor(loanId)
      const result = await collateralService.calculateHealthFactor(100, 100 / (hf / 100))

      setHealthFactor(hf)
      setStatus(result.status)
      setLiquidationRisk(result.liquidationRisk)
    } catch (err) {
      console.error('Failed to load health factor:', err)
    } finally {
      setLoading(false)
    }
  }, [loanId])

  const simulateAddCollateral = useCallback(async (
    currentCollateral: number,
    additionalCollateral: number,
    borrowed: number
  ) => {
    return await collateralService.simulateAddCollateral(
      currentCollateral,
      additionalCollateral,
      borrowed
    )
  }, [])

  const simulateRemoveCollateral = useCallback(async (
    currentCollateral: number,
    removeCollateral: number,
    borrowed: number
  ) => {
    return await collateralService.simulateRemoveCollateral(
      currentCollateral,
      removeCollateral,
      borrowed
    )
  }, [])

  useEffect(() => {
    if (loanId) {
      loadHealthFactor()

      // Poll every 30 seconds for health factor updates
      const interval = setInterval(loadHealthFactor, 30000)
      return () => clearInterval(interval)
    }
  }, [loanId, loadHealthFactor])

  return {
    healthFactor,
    status,
    liquidationRisk,
    liquidationPrice,
    loading,
    calculateHealthFactor,
    simulateAddCollateral,
    simulateRemoveCollateral,
    refresh: loadHealthFactor,
  }
}
