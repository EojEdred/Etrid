"use client"

import { useState, useEffect, useCallback } from "react"
import { Proposal } from "@/lib/types/multisig"
import { multiSigService } from "@/lib/services/MultiSigService"
import { toast } from "sonner"

export function usePendingApprovals(walletId?: string) {
  const [approvals, setApprovals] = useState<Proposal[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch pending approvals
  const fetchApprovals = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = walletId
        ? await multiSigService.getPendingApprovals(walletId)
        : await multiSigService.getAllPendingApprovals()
      setApprovals(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch approvals"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [walletId])

  // Sign transaction
  const signTransaction = useCallback(async (proposalId: string) => {
    setLoading(true)
    setError(null)
    try {
      await multiSigService.signTransaction(proposalId)
      toast.success("Transaction signed successfully!")
      await fetchApprovals() // Refresh approvals
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to sign transaction"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchApprovals])

  // Reject transaction
  const rejectTransaction = useCallback(async (proposalId: string) => {
    setLoading(true)
    setError(null)
    try {
      await multiSigService.rejectTransaction(proposalId)
      toast.success("Transaction rejected")
      await fetchApprovals() // Refresh approvals
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to reject transaction"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchApprovals])

  // Execute transaction
  const executeTransaction = useCallback(async (proposalId: string) => {
    setLoading(true)
    setError(null)
    try {
      await multiSigService.executeTransaction(proposalId)
      toast.success("Transaction executed successfully!")
      await fetchApprovals() // Refresh approvals
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to execute transaction"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchApprovals])

  // Initial fetch
  useEffect(() => {
    fetchApprovals()
  }, [fetchApprovals])

  return {
    approvals,
    loading,
    error,
    fetchApprovals,
    signTransaction,
    rejectTransaction,
    executeTransaction,
  }
}
