"use client"

import { useState, useEffect, useCallback } from "react"
import { fiatRampService } from "@/lib/services/FiatRampService"
import type {
  Quote,
  BuyCryptoParams,
  SellCryptoParams,
  FiatTransaction,
  CryptoAsset,
  FiatCurrency,
} from "@/lib/types/features"
import { toast } from "sonner"

export function useFiatRamp() {
  const [quote, setQuote] = useState<Quote | null>(null)
  const [transactions, setTransactions] = useState<FiatTransaction[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * Get a quote for buying/selling crypto
   */
  const getQuote = useCallback(
    async (amount: number, asset: CryptoAsset, fiatCurrency: FiatCurrency = "USD", isFiatAmount: boolean = true) => {
      setIsLoading(true)
      setError(null)

      try {
        const newQuote = await fiatRampService.getQuote(amount, asset, fiatCurrency, isFiatAmount)
        setQuote(newQuote)
        return newQuote
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to get quote"
        setError(errorMsg)
        toast.error(errorMsg)
        return null
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Buy crypto with fiat
   */
  const buyCrypto = useCallback(async (params: BuyCryptoParams) => {
    setIsLoading(true)
    setError(null)

    try {
      const transaction = await fiatRampService.buyCrypto(params)
      toast.success(`Successfully initiated purchase of ${params.asset}`)

      // Refresh transactions
      loadTransactions()

      return transaction
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to buy crypto"
      setError(errorMsg)
      toast.error(errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Sell crypto for fiat
   */
  const sellCrypto = useCallback(async (params: SellCryptoParams) => {
    setIsLoading(true)
    setError(null)

    try {
      const transaction = await fiatRampService.sellCrypto(params)
      toast.success(`Successfully initiated sale of ${params.asset}`)

      // Refresh transactions
      loadTransactions()

      return transaction
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to sell crypto"
      setError(errorMsg)
      toast.error(errorMsg)
      throw err
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Load transaction history
   */
  const loadTransactions = useCallback(async (page: number = 1, pageSize: number = 20) => {
    setIsLoading(true)
    setError(null)

    try {
      const response = await fiatRampService.getTransactions(page, pageSize)
      setTransactions(response.items)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load transactions"
      setError(errorMsg)
      toast.error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Get a specific transaction
   */
  const getTransaction = useCallback(async (id: string) => {
    setIsLoading(true)
    setError(null)

    try {
      const transaction = await fiatRampService.getTransaction(id)
      return transaction
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to get transaction"
      setError(errorMsg)
      toast.error(errorMsg)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Clear current quote
   */
  const clearQuote = useCallback(() => {
    setQuote(null)
  }, [])

  return {
    // State
    quote,
    transactions,
    isLoading,
    error,

    // Actions
    getQuote,
    buyCrypto,
    sellCrypto,
    loadTransactions,
    getTransaction,
    clearQuote,
  }
}
