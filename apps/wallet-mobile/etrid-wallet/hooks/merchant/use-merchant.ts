"use client"

import { useState, useEffect } from 'react'
import { merchantService } from '@/lib/services/MerchantService'
import type { MerchantAccount, SalesStats, Sale } from '@/lib/types/merchant'

export function useMerchant() {
  const [account, setAccount] = useState<MerchantAccount | null>(null)
  const [stats, setStats] = useState<SalesStats | null>(null)
  const [recentSales, setRecentSales] = useState<Sale[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchAccount = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await merchantService.getMerchantAccount()
      setAccount(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const fetchStats = async (period: 'day' | 'week' | 'month' | 'year' = 'month') => {
    try {
      const data = await merchantService.getSalesStats(period)
      setStats(data)
    } catch (err) {
      console.error('Error fetching stats:', err)
    }
  }

  const fetchRecentSales = async (limit: number = 10) => {
    try {
      const data = await merchantService.getRecentSales(limit)
      setRecentSales(data)
    } catch (err) {
      console.error('Error fetching recent sales:', err)
    }
  }

  const updateAccount = async (updates: Partial<MerchantAccount>) => {
    try {
      setError(null)
      const updated = await merchantService.updateMerchantAccount(updates)
      setAccount(updated)
      return updated
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  useEffect(() => {
    fetchAccount()
    fetchStats()
    fetchRecentSales()
  }, [])

  return {
    account,
    stats,
    recentSales,
    loading,
    error,
    refetch: fetchAccount,
    refetchStats: fetchStats,
    refetchRecentSales: fetchRecentSales,
    updateAccount,
  }
}
