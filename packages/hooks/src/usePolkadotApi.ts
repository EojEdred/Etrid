import { useState, useEffect } from 'react'
import { ApiPromise, WsProvider } from '@polkadot/api'

export interface UsePolkadotApiReturn {
  api: ApiPromise | null
  isConnected: boolean
  isConnecting: boolean
  error: Error | null
  disconnect: () => void
}

/**
 * Hook to connect to Polkadot.js API
 * @param endpoint WebSocket endpoint URL
 * @returns API instance and connection state
 */
export function usePolkadotApi(endpoint: string): UsePolkadotApiReturn {
  const [api, setApi] = useState<ApiPromise | null>(null)
  const [isConnected, setIsConnected] = useState(false)
  const [isConnecting, setIsConnecting] = useState(false)
  const [error, setError] = useState<Error | null>(null)

  useEffect(() => {
    if (!endpoint) return

    let isMounted = true
    let apiInstance: ApiPromise | null = null

    const connect = async () => {
      try {
        setIsConnecting(true)
        setError(null)

        const provider = new WsProvider(endpoint)
        apiInstance = await ApiPromise.create({ provider })

        if (isMounted) {
          setApi(apiInstance)
          setIsConnected(true)
          setIsConnecting(false)
        }
      } catch (err) {
        if (isMounted) {
          setError(err as Error)
          setIsConnected(false)
          setIsConnecting(false)
        }
      }
    }

    connect()

    return () => {
      isMounted = false
      if (apiInstance) {
        apiInstance.disconnect()
      }
    }
  }, [endpoint])

  const disconnect = () => {
    if (api) {
      api.disconnect()
      setApi(null)
      setIsConnected(false)
    }
  }

  return { api, isConnected, isConnecting, error, disconnect }
}
