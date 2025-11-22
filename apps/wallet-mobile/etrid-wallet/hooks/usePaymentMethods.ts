"use client"

import { useState, useEffect, useCallback } from "react"
import { fiatRampService } from "@/lib/services/FiatRampService"
import type { PaymentMethod, PaymentMethodInput } from "@/lib/types/features"
import { toast } from "sonner"

export function usePaymentMethods() {
  const [paymentMethods, setPaymentMethods] = useState<PaymentMethod[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  /**
   * Load all payment methods
   */
  const loadPaymentMethods = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const methods = await fiatRampService.getPaymentMethods()
      setPaymentMethods(methods)
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Failed to load payment methods"
      setError(errorMsg)
      toast.error(errorMsg)
    } finally {
      setIsLoading(false)
    }
  }, [])

  /**
   * Add a new payment method
   */
  const addPaymentMethod = useCallback(
    async (method: PaymentMethodInput) => {
      setIsLoading(true)
      setError(null)

      try {
        const newMethod = await fiatRampService.addPaymentMethod(method)
        setPaymentMethods((prev) => [...prev, newMethod])
        toast.success("Payment method added successfully")
        return newMethod
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to add payment method"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Delete a payment method
   */
  const deletePaymentMethod = useCallback(
    async (id: string) => {
      setIsLoading(true)
      setError(null)

      try {
        await fiatRampService.deletePaymentMethod(id)
        setPaymentMethods((prev) => prev.filter((method) => method.id !== id))
        toast.success("Payment method deleted")
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to delete payment method"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Set a payment method as default
   */
  const setDefaultPaymentMethod = useCallback(
    async (id: string) => {
      setIsLoading(true)
      setError(null)

      try {
        await fiatRampService.setDefaultPaymentMethod(id)
        setPaymentMethods((prev) =>
          prev.map((method) => ({
            ...method,
            isDefault: method.id === id,
          })),
        )
        toast.success("Default payment method updated")
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : "Failed to set default payment method"
        setError(errorMsg)
        toast.error(errorMsg)
        throw err
      } finally {
        setIsLoading(false)
      }
    },
    [],
  )

  /**
   * Get default payment method
   */
  const getDefaultPaymentMethod = useCallback((): PaymentMethod | null => {
    return paymentMethods.find((method) => method.isDefault) || paymentMethods[0] || null
  }, [paymentMethods])

  /**
   * Get payment method by ID
   */
  const getPaymentMethod = useCallback(
    (id: string): PaymentMethod | null => {
      return paymentMethods.find((method) => method.id === id) || null
    },
    [paymentMethods],
  )

  // Load payment methods on mount
  useEffect(() => {
    loadPaymentMethods()
  }, [loadPaymentMethods])

  return {
    // State
    paymentMethods,
    isLoading,
    error,

    // Actions
    loadPaymentMethods,
    addPaymentMethod,
    deletePaymentMethod,
    setDefaultPaymentMethod,
    getDefaultPaymentMethod,
    getPaymentMethod,
  }
}
