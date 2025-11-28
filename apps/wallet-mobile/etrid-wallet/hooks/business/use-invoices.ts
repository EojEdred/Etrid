"use client"

import { useState, useEffect } from 'react'
import { invoiceService } from '@/lib/services/InvoiceService'
import type { Invoice, InvoiceInput, InvoiceFilter } from '@/lib/types/business'

export function useInvoices(filter?: InvoiceFilter) {
  const [invoices, setInvoices] = useState<Invoice[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchInvoices = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await invoiceService.getInvoices(filter)
      setInvoices(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const createInvoice = async (invoice: InvoiceInput) => {
    try {
      setError(null)
      const newInvoice = await invoiceService.createInvoice(invoice)
      setInvoices([newInvoice, ...invoices])
      return newInvoice
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const updateInvoice = async (id: string, updates: Partial<Invoice>) => {
    try {
      setError(null)
      const updated = await invoiceService.updateInvoice(id, updates)
      setInvoices(invoices.map((inv) => (inv.id === id ? updated : inv)))
      return updated
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const sendInvoice = async (id: string, recipient: string) => {
    try {
      setError(null)
      await invoiceService.sendInvoice(id, recipient)
      await fetchInvoices()
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const markPaid = async (id: string) => {
    try {
      setError(null)
      await invoiceService.markPaid(id)
      await fetchInvoices()
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const deleteInvoice = async (id: string) => {
    try {
      setError(null)
      await invoiceService.deleteInvoice(id)
      setInvoices(invoices.filter((inv) => inv.id !== id))
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  useEffect(() => {
    fetchInvoices()
  }, [filter?.status, filter?.client])

  return {
    invoices,
    loading,
    error,
    refetch: fetchInvoices,
    createInvoice,
    updateInvoice,
    sendInvoice,
    markPaid,
    deleteInvoice,
  }
}
