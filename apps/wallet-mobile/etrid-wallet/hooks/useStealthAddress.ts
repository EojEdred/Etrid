"use client"

import { useState, useEffect, useCallback } from "react"
import { StealthAddress } from "@/lib/types/privacy"
import { privacyService } from "@/lib/services/PrivacyService"
import { toast } from "sonner"

export function useStealthAddress() {
  const [addresses, setAddresses] = useState<StealthAddress[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch stealth addresses
  const fetchAddresses = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await privacyService.getStealthAddresses()
      setAddresses(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch addresses"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Generate new stealth address
  const generateAddress = useCallback(async (label?: string) => {
    setLoading(true)
    setError(null)
    try {
      const newAddress = await privacyService.generateStealthAddress(label)
      setAddresses((prev) => [newAddress, ...prev])
      toast.success("New stealth address generated!")
      return newAddress
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to generate address"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [])

  // Get unused address
  const getUnusedAddress = useCallback((): StealthAddress | null => {
    return addresses.find((addr) => addr.status === "unused") || null
  }, [addresses])

  // Count by status
  const getCounts = useCallback(() => {
    return {
      unused: addresses.filter((addr) => addr.status === "unused").length,
      used: addresses.filter((addr) => addr.status === "used").length,
      expired: addresses.filter((addr) => addr.status === "expired").length,
      total: addresses.length,
    }
  }, [addresses])

  // Initial fetch
  useEffect(() => {
    fetchAddresses()
  }, [fetchAddresses])

  return {
    addresses,
    loading,
    error,
    generateAddress,
    getUnusedAddress,
    getCounts,
    fetchAddresses,
  }
}
