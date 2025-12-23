"use client"

import { useState, useEffect } from "react"
import { LightningClient } from "./client"
import type { Channel, Payment, NetworkStats, CrossPBCRoute } from "./types"

export function useLightning() {
  const [client] = useState(() => new LightningClient())
  const [channels, setChannels] = useState<Channel[]>([])
  const [payments, setPayments] = useState<Payment[]>([])
  const [networkStats, setNetworkStats] = useState<NetworkStats | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Load user's channels
  useEffect(() => {
    loadChannels()
    loadPayments()
    loadNetworkStats()
  }, [])

  async function loadChannels() {
    try {
      const data = await client.getChannels()
      setChannels(data)
    } catch (err) {
      console.error("Failed to load channels:", err)
    }
  }

  async function loadPayments() {
    try {
      const data = await client.getPayments()
      setPayments(data)
    } catch (err) {
      console.error("Failed to load payments:", err)
    }
  }

  async function loadNetworkStats() {
    try {
      const data = await client.getNetworkStats()
      setNetworkStats(data)
    } catch (err) {
      console.error("Failed to load network stats:", err)
    }
  }

  // Find cross-chain route
  async function findRoute(params: {
    sourceChain: string
    destChain: string
    sourceAddress: string
    destAddress: string
    amount: string
  }): Promise<CrossPBCRoute | null> {
    try {
      setLoading(true)
      setError(null)
      const route = await client.findCrossPBCRoute(params)
      return route
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to find route")
      return null
    } finally {
      setLoading(false)
    }
  }

  // Send cross-chain payment
  async function sendPayment(params: {
    route: CrossPBCRoute
    sourceAddress: string
    destAddress: string
  }) {
    try {
      setLoading(true)
      setError(null)
      const result = await client.sendPayment(params)
      await loadPayments() // Refresh payment history
      return result
    } catch (err) {
      setError(err instanceof Error ? err.message : "Payment failed")
      throw err
    } finally {
      setLoading(false)
    }
  }

  // Open new Lightning channel
  async function openChannel(params: {
    chain: string
    counterparty: string
    capacity: string
  }) {
    try {
      setLoading(true)
      setError(null)
      const result = await client.openChannel(params)
      await loadChannels() // Refresh channels
      return result
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to open channel")
      throw err
    } finally {
      setLoading(false)
    }
  }

  // Close Lightning channel
  async function closeChannel(channelId: string) {
    try {
      setLoading(true)
      setError(null)
      await client.closeChannel(channelId)
      await loadChannels() // Refresh channels
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to close channel")
      throw err
    } finally {
      setLoading(false)
    }
  }

  return {
    channels,
    payments,
    networkStats,
    loading,
    error,
    findRoute,
    sendPayment,
    openChannel,
    closeChannel,
    refresh: () => {
      loadChannels()
      loadPayments()
      loadNetworkStats()
    },
  }
}
