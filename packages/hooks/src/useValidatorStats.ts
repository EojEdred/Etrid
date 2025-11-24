import { useState, useEffect } from 'react'
import type { ApiPromise } from '@polkadot/api'

export interface ValidatorStats {
  commission: number
  totalStake: string
  ownStake: string
  nominatorCount: number
  isElected: boolean
  isActive: boolean
}

export interface UseValidatorStatsReturn {
  stats: ValidatorStats | null
  isLoading: boolean
  error: Error | null
  refetch: () => void
}

/**
 * Hook to fetch validator statistics
 * @param api Polkadot.js API instance
 * @param validatorAddress Validator account address
 * @returns Validator statistics
 */
export function useValidatorStats(
  api: ApiPromise | null,
  validatorAddress: string | null
): UseValidatorStatsReturn {
  const [stats, setStats] = useState<ValidatorStats | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)
  const [refetchTrigger, setRefetchTrigger] = useState(0)

  useEffect(() => {
    if (!api || !validatorAddress) {
      setStats(null)
      return
    }

    const fetchStats = async () => {
      try {
        setIsLoading(true)
        setError(null)

        // Get validator preferences (commission)
        const prefs = await api.query.staking.validators(validatorAddress)

        // Get current era
        const currentEra = await api.query.staking.currentEra()

        // Get exposure (stake info)
        const exposure = currentEra
          ? await api.query.staking.erasStakers(currentEra.unwrap(), validatorAddress)
          : null

        // Get elected validators
        const validators = await api.query.session.validators()
        const isElected = validators.some(v => v.toString() === validatorAddress)

        if (exposure) {
          setStats({
            commission: prefs.commission.toNumber() / 10000000, // Convert from Perbill
            totalStake: exposure.total.toString(),
            ownStake: exposure.own.toString(),
            nominatorCount: exposure.others.length,
            isElected,
            isActive: isElected
          })
        }

        setIsLoading(false)
      } catch (err) {
        setError(err as Error)
        setIsLoading(false)
      }
    }

    fetchStats()
  }, [api, validatorAddress, refetchTrigger])

  const refetch = () => setRefetchTrigger(prev => prev + 1)

  return { stats, isLoading, error, refetch }
}
