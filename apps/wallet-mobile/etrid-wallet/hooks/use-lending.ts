/**
 * useLending - Hook for lending/borrowing operations
 */

'use client'

import { useState, useEffect, useCallback } from 'react'
import { lendingService, type LendingPosition } from '@/lib/services/lending-service'
import { useToast } from '@/hooks/use-toast'

export function useLending() {
  const [positions, setPositions] = useState<LendingPosition[]>([])
  const [totalSupplied, setTotalSupplied] = useState(0)
  const [totalBorrowed, setTotalBorrowed] = useState(0)
  const [netAPY, setNetAPY] = useState(0)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const { toast } = useToast()

  const loadPositions = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const [positionsData, supplied, borrowed, apy] = await Promise.all([
        lendingService.getPositions(),
        lendingService.getTotalSupplied(),
        lendingService.getTotalBorrowed(),
        lendingService.getNetAPY(),
      ])

      setPositions(positionsData)
      setTotalSupplied(supplied)
      setTotalBorrowed(borrowed)
      setNetAPY(apy)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to load positions'
      setError(errorMessage)
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
    } finally {
      setLoading(false)
    }
  }, [toast])

  const supply = useCallback(async (asset: string, amount: number) => {
    try {
      setLoading(true)
      const tx = await lendingService.supply(asset, amount)

      toast({
        title: 'Supply Successful',
        description: `Supplied ${amount} ${asset}`,
      })

      await loadPositions()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to supply'
      toast({
        title: 'Supply Failed',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadPositions])

  const withdraw = useCallback(async (asset: string, amount: number) => {
    try {
      setLoading(true)
      const tx = await lendingService.withdraw(asset, amount)

      toast({
        title: 'Withdrawal Successful',
        description: `Withdrew ${amount} ${asset}`,
      })

      await loadPositions()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to withdraw'
      toast({
        title: 'Withdrawal Failed',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadPositions])

  const borrow = useCallback(async (
    asset: string,
    amount: number,
    collateral: { asset: string; amount: number; valueUSD: number }[]
  ) => {
    try {
      setLoading(true)
      const tx = await lendingService.borrow(asset, amount, collateral)

      toast({
        title: 'Borrow Successful',
        description: `Borrowed ${amount} ${asset}`,
      })

      await loadPositions()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to borrow'
      toast({
        title: 'Borrow Failed',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadPositions])

  const repay = useCallback(async (loanId: string, amount: number) => {
    try {
      setLoading(true)
      const tx = await lendingService.repay(loanId, amount)

      toast({
        title: 'Repayment Successful',
        description: `Repaid ${amount}`,
      })

      await loadPositions()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to repay'
      toast({
        title: 'Repayment Failed',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadPositions])

  const getAPY = useCallback(async (asset: string, type: 'supply' | 'borrow') => {
    try {
      if (type === 'supply') {
        return await lendingService.getSupplyAPY(asset)
      } else {
        return await lendingService.getBorrowAPY(asset)
      }
    } catch (err) {
      console.error('Failed to get APY:', err)
      return 0
    }
  }, [])

  useEffect(() => {
    loadPositions()
  }, [loadPositions])

  return {
    positions,
    totalSupplied,
    totalBorrowed,
    netAPY,
    loading,
    error,
    supply,
    withdraw,
    borrow,
    repay,
    getAPY,
    refresh: loadPositions,
  }
}
