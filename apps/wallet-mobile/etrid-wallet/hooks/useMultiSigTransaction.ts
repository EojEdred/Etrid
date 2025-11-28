"use client"

import { useState, useCallback } from "react"
import { TransactionInput, Proposal } from "@/lib/types/multisig"
import { multiSigService } from "@/lib/services/MultiSigService"
import { toast } from "sonner"

export function useMultiSigTransaction(walletId: string) {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Propose transaction
  const proposeTransaction = useCallback(async (tx: TransactionInput): Promise<Proposal> => {
    setLoading(true)
    setError(null)
    try {
      const proposal = await multiSigService.proposeTransaction(walletId, tx)
      toast.success("Transaction proposed successfully!")
      return proposal
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to propose transaction"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [walletId])

  // Get proposal details
  const getProposal = useCallback(async (proposalId: string): Promise<Proposal> => {
    setLoading(true)
    setError(null)
    try {
      const proposal = await multiSigService.getProposal(proposalId)
      return proposal
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch proposal"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [])

  return {
    loading,
    error,
    proposeTransaction,
    getProposal,
  }
}
