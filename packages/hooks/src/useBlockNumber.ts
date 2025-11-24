import { useState, useEffect } from 'react'
import type { ApiPromise } from '@polkadot/api'

export interface UseBlockNumberReturn {
  blockNumber: number | null
  finalizedBlock: number | null
  isLoading: boolean
  error: Error | null
}

/**
 * Hook to subscribe to block number updates
 * @param api Polkadot.js API instance
 * @returns Current block number and finalized block
 */
export function useBlockNumber(api: ApiPromise | null): UseBlockNumberReturn {
  const [blockNumber, setBlockNumber] = useState<number | null>(null)
  const [finalizedBlock, setFinalizedBlock] = useState<number | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)

  useEffect(() => {
    if (!api) {
      setBlockNumber(null)
      setFinalizedBlock(null)
      return
    }

    let unsubscribeNewHeads: (() => void) | null = null
    let unsubscribeFinalized: (() => void) | null = null

    const subscribe = async () => {
      try {
        setIsLoading(true)
        setError(null)

        // Subscribe to new block headers
        unsubscribeNewHeads = await api.rpc.chain.subscribeNewHeads((header) => {
          setBlockNumber(header.number.toNumber())
          setIsLoading(false)
        })

        // Subscribe to finalized block headers
        unsubscribeFinalized = await api.rpc.chain.subscribeFinalizedHeads((header) => {
          setFinalizedBlock(header.number.toNumber())
        })
      } catch (err) {
        setError(err as Error)
        setIsLoading(false)
      }
    }

    subscribe()

    return () => {
      if (unsubscribeNewHeads) {
        unsubscribeNewHeads()
      }
      if (unsubscribeFinalized) {
        unsubscribeFinalized()
      }
    }
  }, [api])

  return { blockNumber, finalizedBlock, isLoading, error }
}
