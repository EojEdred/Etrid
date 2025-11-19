/**
 * useP2PLending - P2P lending marketplace operations
 */

'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  p2pLendingService,
  type LoanOffer,
  type P2PLoan,
  type LoanOfferInput,
  type LoanFilters,
} from '@/lib/services/p2p-lending-service'
import { useToast } from '@/hooks/use-toast'

export function useP2PLending() {
  const [offers, setOffers] = useState<LoanOffer[]>([])
  const [myOffers, setMyOffers] = useState<LoanOffer[]>([])
  const [myLoans, setMyLoans] = useState<{ asLender: P2PLoan[]; asBorrower: P2PLoan[] }>({
    asLender: [],
    asBorrower: [],
  })
  const [loading, setLoading] = useState(true)
  const { toast } = useToast()

  const loadOffers = useCallback(async (filters?: LoanFilters) => {
    try {
      setLoading(true)
      const offersData = await p2pLendingService.getLoanOffers(filters)
      setOffers(offersData)
    } catch (err) {
      console.error('Failed to load offers:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  const loadMyOffers = useCallback(async () => {
    try {
      const offersData = await p2pLendingService.getMyOffers()
      setMyOffers(offersData)
    } catch (err) {
      console.error('Failed to load my offers:', err)
    }
  }, [])

  const loadMyLoans = useCallback(async () => {
    try {
      const [asLender, asBorrower] = await Promise.all([
        p2pLendingService.getMyLoans('lender'),
        p2pLendingService.getMyLoans('borrower'),
      ])

      setMyLoans({ asLender, asBorrower })
    } catch (err) {
      console.error('Failed to load my loans:', err)
    }
  }, [])

  const createOffer = useCallback(async (input: LoanOfferInput) => {
    try {
      setLoading(true)
      const offer = await p2pLendingService.createLoanOffer(input)

      toast({
        title: 'Offer Created',
        description: `Created loan offer for ${input.amount} ${input.asset}`,
      })

      await loadMyOffers()
      await loadOffers()
      return offer
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create offer'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadMyOffers, loadOffers])

  const acceptOffer = useCallback(async (
    offerId: string,
    collateral: { asset: string; amount: number }[]
  ) => {
    try {
      setLoading(true)
      const tx = await p2pLendingService.acceptLoanOffer(offerId, collateral)

      toast({
        title: 'Loan Accepted',
        description: 'Successfully accepted loan offer',
      })

      await loadOffers()
      await loadMyLoans()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to accept offer'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadOffers, loadMyLoans])

  const cancelOffer = useCallback(async (offerId: string) => {
    try {
      setLoading(true)
      await p2pLendingService.cancelOffer(offerId)

      toast({
        title: 'Offer Cancelled',
        description: 'Successfully cancelled loan offer',
      })

      await loadMyOffers()
      await loadOffers()
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to cancel offer'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadMyOffers, loadOffers])

  const repayLoan = useCallback(async (loanId: string, amount: number) => {
    try {
      setLoading(true)
      const tx = await p2pLendingService.repayLoan(loanId, amount)

      toast({
        title: 'Repayment Successful',
        description: `Repaid ${amount}`,
      })

      await loadMyLoans()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to repay loan'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadMyLoans])

  useEffect(() => {
    loadOffers()
    loadMyOffers()
    loadMyLoans()
  }, [loadOffers, loadMyOffers, loadMyLoans])

  return {
    offers,
    myOffers,
    myLoans,
    loading,
    createOffer,
    acceptOffer,
    cancelOffer,
    repayLoan,
    refresh: () => {
      loadOffers()
      loadMyOffers()
      loadMyLoans()
    },
    filterOffers: loadOffers,
  }
}
