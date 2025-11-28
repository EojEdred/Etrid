"use client"

import { useState, useEffect } from 'react'
import { businessService } from '@/lib/services/BusinessService'
import type { BusinessAccount, DashboardStats } from '@/lib/types/business'

export function useBusiness() {
  const [account, setAccount] = useState<BusinessAccount | null>(null)
  const [stats, setStats] = useState<DashboardStats | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchAccount = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await businessService.getBusinessAccount()
      setAccount(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const fetchStats = async () => {
    try {
      const data = await businessService.getDashboardStats()
      setStats(data)
    } catch (err) {
      console.error('Error fetching stats:', err)
    }
  }

  const updateAccount = async (updates: Partial<BusinessAccount>) => {
    try {
      setError(null)
      const updated = await businessService.updateBusinessAccount(updates)
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
  }, [])

  return {
    account,
    stats,
    loading,
    error,
    refetch: fetchAccount,
    refetchStats: fetchStats,
    updateAccount,
  }
}
