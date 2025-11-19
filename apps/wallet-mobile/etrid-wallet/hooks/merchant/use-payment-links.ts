"use client"

import { useState, useEffect } from 'react'
import { paymentLinkService } from '@/lib/services/PaymentLinkService'
import type {
  PaymentLink,
  PaymentLinkInput,
  PaymentLinkStatus,
} from '@/lib/types/merchant'

export function usePaymentLinks(filter?: PaymentLinkStatus) {
  const [links, setLinks] = useState<PaymentLink[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchLinks = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await paymentLinkService.getPaymentLinks(filter)
      setLinks(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const createLink = async (link: PaymentLinkInput) => {
    try {
      setError(null)
      const newLink = await paymentLinkService.createPaymentLink(link)
      setLinks([newLink, ...links])
      return newLink
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const deactivateLink = async (id: string) => {
    try {
      setError(null)
      await paymentLinkService.deactivateLink(id)
      await fetchLinks()
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const deleteLink = async (id: string) => {
    try {
      setError(null)
      await paymentLinkService.deletePaymentLink(id)
      setLinks(links.filter((link) => link.id !== id))
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const getPaymentUrl = (linkCode: string) => {
    return paymentLinkService.getPaymentLinkUrl(linkCode)
  }

  useEffect(() => {
    fetchLinks()
  }, [filter])

  return {
    links,
    loading,
    error,
    refetch: fetchLinks,
    createLink,
    deactivateLink,
    deleteLink,
    getPaymentUrl,
  }
}
