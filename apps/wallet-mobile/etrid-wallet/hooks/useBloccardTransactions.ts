"use client"

import { useState, useEffect, useCallback } from "react"
import { bloccardService } from "@/lib/services/BloccardService"
import type { BloccardTransaction, TransactionFilters } from "@/lib/types/features"
import { toast } from "sonner"

export function useBloccardTransactions() {
  const [transactions, setTransactions] = useState<BloccardTransaction[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [hasMore, setHasMore] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)

  /**
   * Load transactions with optional filters
   */
  const loadTransactions = useCallback(async (filters?: TransactionFilters, page: number = 1, pageSize: number = 20) => {
    setIsLoading(true)
    setError(null)

    try {
      const response = await bloccardService.getTransactions(filters, page, pageSize)
      setTransactions(response.items)
      setHasMore(response.hasMore)
      setCurrentPage(page)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load transactions"
      setError(errorMsg)
      toast.error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Load more transactions (pagination)
   */
  const loadMore = useCallback(
    async (filters?: TransactionFilters, pageSize: number = 20) => {
      if (!hasMore) return

      setIsLoading(true)
      setError(null)

      try {
        const nextPage = currentPage + 1
        const response = await bloccardService.getTransactions(filters, nextPage, pageSize)
        setTransactions((prev) => [...prev, ...response.items])
        setHasMore(response.hasMore)
        setCurrentPage(nextPage)
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to load more transactions"
        setError(errorMsg)
        toast.error(errorMsg)
      } finally {
        setIsLoading(false)
      }
    },
    [currentPage, hasMore],
  )

  /**
   * Get a specific transaction
   */
  const getTransaction = useCallback(async (id: string) => {
    setIsLoading(true)
    setError(null)

    try {
      const transaction = await bloccardService.getTransaction(id)
      return transaction
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load transaction"
      setError(errorMsg)
      toast.error(errorMsg)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Export transactions to CSV
   */
  const exportTransactions = useCallback(async (filters?: TransactionFilters) => {
    setIsLoading(true)
    setError(null)

    try {
      const blob = await bloccardService.exportTransactions(filters)

      // Create download link
      const url = window.URL.createObjectURL(blob)
      const a = document.createElement("a")
      a.href = url
      a.download = `bloccard-transactions-${new Date().toISOString().split("T")[0]}.csv`
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      window.URL.revokeObjectURL(url)

      toast.success("Transactions exported successfully")
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to export transactions"
      setError(errorMsg)
      toast.error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Get transactions by category
   */
  const getTransactionsByCategory = useCallback(
    (category: string): BloccardTransaction[] => {
      return transactions.filter((tx) => tx.merchantCategory === category)
    },
    [transactions],
  )

  /**
   * Get total spent
   */
  const getTotalSpent = useCallback((): number => {
    return transactions.reduce((sum, tx) => sum + tx.amountUsd, 0)
  }, [transactions])

  /**
   * Get transactions grouped by date
   */
  const getTransactionsGroupedByDate = useCallback((): Record<string, BloccardTransaction[]> => {
    const grouped: Record<string, BloccardTransaction[]> = {}

    transactions.forEach((tx) => {
      const date = new Date(tx.timestamp).toISOString().split("T")[0]
      if (!grouped[date]) {
        grouped[date] = []
      }
      grouped[date].push(tx)
    })

    return grouped
  }, [transactions])

  /**
   * Refresh transactions
   */
  const refresh = useCallback(() => {
    setCurrentPage(1)
    loadTransactions()
  }, [loadTransactions])

  // Load initial transactions on mount
  useEffect(() => {
    loadTransactions()
  }, [loadTransactions])

  return {
    // State
    transactions,
    isLoading,
    error,
    hasMore,
    currentPage,

    // Actions
    loadTransactions,
    loadMore,
    getTransaction,
    exportTransactions,
    refresh,

    // Helpers
    getTransactionsByCategory,
    getTotalSpent,
    getTransactionsGroupedByDate,
  }
}
