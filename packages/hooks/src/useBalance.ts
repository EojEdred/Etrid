import { useState, useEffect } from 'react'
import type { ApiPromise } from '@polkadot/api'

export interface Balance {
  free: string
  reserved: string
  frozen: string
  total: string
}

export interface UseBalanceReturn {
  balance: Balance | null
  isLoading: boolean
  error: Error | null
  refetch: () => void
}

/**
 * Hook to fetch account balance
 * @param api Polkadot.js API instance
 * @param address Account address
 * @returns Balance data and loading state
 */
export function useBalance(
  api: ApiPromise | null,
  address: string | null
): UseBalanceReturn {
  const [balance, setBalance] = useState<Balance | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)
  const [refetchTrigger, setRefetchTrigger] = useState(0)

  useEffect(() => {
    if (!api || !address) {
      setBalance(null)
      return
    }

    let unsubscribe: (() => void) | null = null

    const fetchBalance = async () => {
      try {
        setIsLoading(true)
        setError(null)

        // Subscribe to balance changes
        unsubscribe = await api.query.system.account(address, ({ data }) => {
          setBalance({
            free: data.free.toString(),
            reserved: data.reserved.toString(),
            frozen: data.frozen.toString(),
            total: data.free.add(data.reserved).toString()
          })
          setIsLoading(false)
        })
      } catch (err) {
        setError(err as Error)
        setIsLoading(false)
      }
    }

    fetchBalance()

    return () => {
      if (unsubscribe) {
        unsubscribe()
      }
    }
  }, [api, address, refetchTrigger])

  const refetch = () => setRefetchTrigger(prev => prev + 1)

  return { balance, isLoading, error, refetch }
}
