"use client"

import { useState, useEffect, useCallback } from "react"
import { WhitelistedAddress } from "@/lib/types/security"
import { whitelistService } from "@/lib/services/WhitelistService"
import { toast } from "sonner"

export function useWhitelist() {
  const [addresses, setAddresses] = useState<WhitelistedAddress[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch whitelist
  const fetchWhitelist = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await whitelistService.getWhitelist()
      setAddresses(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch whitelist"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Add address
  const addAddress = useCallback(async (address: string, label?: string) => {
    setLoading(true)
    setError(null)
    try {
      await whitelistService.addAddress(address, label)
      toast.success("Address added to whitelist!")
      await fetchWhitelist() // Refresh list
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to add address"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchWhitelist])

  // Remove address
  const removeAddress = useCallback(async (address: string) => {
    setLoading(true)
    setError(null)
    try {
      await whitelistService.removeAddress(address)
      toast.success("Address removed from whitelist")
      await fetchWhitelist() // Refresh list
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to remove address"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchWhitelist])

  // Update label
  const updateLabel = useCallback(async (address: string, label: string) => {
    setLoading(true)
    setError(null)
    try {
      await whitelistService.updateLabel(address, label)
      toast.success("Label updated!")
      await fetchWhitelist() // Refresh list
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to update label"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchWhitelist])

  // Check if whitelisted
  const isWhitelisted = useCallback(async (address: string) => {
    try {
      return await whitelistService.isWhitelisted(address)
    } catch (err) {
      console.error("Failed to check whitelist status:", err)
      return false
    }
  }, [])

  // Initial fetch
  useEffect(() => {
    fetchWhitelist()
  }, [fetchWhitelist])

  return {
    addresses,
    loading,
    error,
    addAddress,
    removeAddress,
    updateLabel,
    isWhitelisted,
    fetchWhitelist,
  }
}
