"use client"

import { useState, useEffect, useCallback } from "react"
import { MultiSigWallet, MultiSigParams, MultiSigStats } from "@/lib/types/multisig"
import { multiSigService } from "@/lib/services/MultiSigService"
import { toast } from "sonner"

export function useMultiSig() {
  const [wallets, setWallets] = useState<MultiSigWallet[]>([])
  const [stats, setStats] = useState<MultiSigStats | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch all wallets
  const fetchWallets = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await multiSigService.getWallets()
      setWallets(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch wallets"
      setError(message)
      toast.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Fetch stats
  const fetchStats = useCallback(async () => {
    try {
      const data = await multiSigService.getStats()
      setStats(data)
    } catch (err) {
      console.error("Failed to fetch stats:", err)
    }
  }, [])

  // Create new wallet
  const createWallet = useCallback(async (params: MultiSigParams) => {
    setLoading(true)
    setError(null)
    try {
      const newWallet = await multiSigService.createWallet(params)
      setWallets((prev) => [...prev, newWallet])
      toast.success("Multi-sig wallet created successfully!")
      return newWallet
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to create wallet"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [])

  // Get specific wallet
  const getWallet = useCallback(
    async (walletId: string) => {
      setLoading(true)
      setError(null)
      try {
        const wallet = await multiSigService.getWallet(walletId)
        return wallet
      } catch (err) {
        const message = err instanceof Error ? err.message : "Failed to fetch wallet"
        setError(message)
        toast.error(message)
        throw err
      } finally {
        setLoading(false)
      }
    },
    []
  )

  // Add signer
  const addSigner = useCallback(async (walletId: string, signer: string) => {
    setLoading(true)
    setError(null)
    try {
      await multiSigService.addSigner(walletId, signer)
      toast.success("Signer added successfully!")
      await fetchWallets() // Refresh wallets
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to add signer"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchWallets])

  // Remove signer
  const removeSigner = useCallback(async (walletId: string, signer: string) => {
    setLoading(true)
    setError(null)
    try {
      await multiSigService.removeSigner(walletId, signer)
      toast.success("Signer removed successfully!")
      await fetchWallets() // Refresh wallets
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to remove signer"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchWallets])

  // Change threshold
  const changeThreshold = useCallback(async (walletId: string, newThreshold: number) => {
    setLoading(true)
    setError(null)
    try {
      await multiSigService.changeThreshold(walletId, newThreshold)
      toast.success("Threshold changed successfully!")
      await fetchWallets() // Refresh wallets
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to change threshold"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchWallets])

  // Initial fetch
  useEffect(() => {
    fetchWallets()
    fetchStats()
  }, [fetchWallets, fetchStats])

  return {
    wallets,
    stats,
    loading,
    error,
    fetchWallets,
    createWallet,
    getWallet,
    addSigner,
    removeSigner,
    changeThreshold,
  }
}
