'use client'

import { useEffect, useState } from 'react'
import { ApiPromise, WsProvider } from '@polkadot/api'

export function usePolkadotApi(endpoint: string) {
  const [api, setApi] = useState<ApiPromise | null>(null)
  const [isConnected, setIsConnected] = useState(false)
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    let mounted = true
    let apiInstance: ApiPromise | null = null

    async function connect() {
      try {
        setIsLoading(true)
        setError(null)

        const provider = new WsProvider(endpoint, false)
        apiInstance = await ApiPromise.create({ provider })

        await apiInstance.isReady

        if (mounted) {
          setApi(apiInstance)
          setIsConnected(true)
          setIsLoading(false)
        }
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err.message : 'Failed to connect')
          setIsLoading(false)
          setIsConnected(false)
        }
      }
    }

    connect()

    return () => {
      mounted = false
      if (apiInstance) {
        apiInstance.disconnect()
      }
    }
  }, [endpoint])

  return { api, isConnected, isLoading, error }
}
