"use client"

import { useState, useEffect } from 'react'
import { refundService } from '@/lib/services/RefundService'
import type { Refund, RefundRequest, RefundStatus } from '@/lib/types/merchant'

export function useRefunds(filter?: RefundStatus) {
  const [refunds, setRefunds] = useState<Refund[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchRefunds = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await refundService.getRefunds(filter)
      setRefunds(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const requestRefund = async (request: RefundRequest) => {
    try {
      setError(null)
      const newRefund = await refundService.requestRefund(request)
      setRefunds([newRefund, ...refunds])
      return newRefund
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const processRefund = async (refundId: string, amount: number) => {
    try {
      setError(null)
      const result = await refundService.processRefund(refundId, amount)
      await fetchRefunds()
      return result
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const approveRefund = async (refundId: string) => {
    try {
      setError(null)
      await refundService.approveRefund(refundId)
      await fetchRefunds()
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const rejectRefund = async (refundId: string, reason: string) => {
    try {
      setError(null)
      await refundService.rejectRefund(refundId, reason)
      await fetchRefunds()
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  useEffect(() => {
    fetchRefunds()
  }, [filter])

  return {
    refunds,
    loading,
    error,
    refetch: fetchRefunds,
    requestRefund,
    processRefund,
    approveRefund,
    rejectRefund,
  }
}
