'use client'

import { useState, useCallback, useEffect } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import {
  getStakingInfo,
  getValidators,
  getWaitingValidators,
  getEraInfo,
  bond,
  bondExtra,
  unbond,
  withdrawUnbonded,
  nominate,
  chill,
  getMinimumStake,
  getBondingDuration,
  type StakingInfo,
  type Validator,
  type EraInfo,
} from '@/lib/polkadot/staking'
import { initApi } from '@/lib/polkadot/api'
import { formatBalance } from '@polkadot/util'

/**
 * Hook for fetching staking information for a specific address
 */
export function useStakingInfo(address?: string) {
  return useQuery<StakingInfo | null>({
    queryKey: ['stakingInfo', address],
    queryFn: async () => {
      if (!address) return null
      await initApi()
      return getStakingInfo(address)
    },
    enabled: !!address,
    refetchInterval: 12000, // Refetch every 12 seconds (2 blocks)
  })
}

/**
 * Hook for fetching active validators
 */
export function useValidators() {
  return useQuery<Validator[]>({
    queryKey: ['validators'],
    queryFn: async () => {
      await initApi()
      return getValidators()
    },
    staleTime: 60000, // Consider data fresh for 1 minute
    refetchInterval: 60000, // Refetch every minute
  })
}

/**
 * Hook for fetching waiting validators (not in active set)
 */
export function useWaitingValidators() {
  return useQuery<Validator[]>({
    queryKey: ['waitingValidators'],
    queryFn: async () => {
      await initApi()
      return getWaitingValidators()
    },
    staleTime: 60000,
    refetchInterval: 60000,
  })
}

/**
 * Hook for fetching current era information
 */
export function useEraInfo() {
  return useQuery<EraInfo>({
    queryKey: ['eraInfo'],
    queryFn: async () => {
      await initApi()
      return getEraInfo()
    },
    refetchInterval: 12000, // Refetch every 12 seconds
  })
}

/**
 * Hook for fetching minimum stake amount
 */
export function useMinimumStake() {
  return useQuery<string>({
    queryKey: ['minimumStake'],
    queryFn: async () => {
      await initApi()
      return getMinimumStake()
    },
    staleTime: 300000, // Consider data fresh for 5 minutes
  })
}

/**
 * Hook for fetching bonding duration (in eras)
 */
export function useBondingDuration() {
  return useQuery<number>({
    queryKey: ['bondingDuration'],
    queryFn: async () => {
      await initApi()
      return getBondingDuration()
    },
    staleTime: 300000, // Consider data fresh for 5 minutes
  })
}

/**
 * Hook for bonding tokens (initial bond)
 */
export function useBond() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({
      address,
      amount,
      payee,
    }: {
      address: string
      amount: string
      payee: 'Staked' | 'Stash' | 'Controller' | { Account: string }
    }) => {
      await initApi()
      const result = await bond(address, amount, payee)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      // Invalidate staking info to refetch after successful bond
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
    },
  })
}

/**
 * Hook for bonding additional tokens
 */
export function useBondExtra() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({
      address,
      amount,
    }: {
      address: string
      amount: string
    }) => {
      await initApi()
      const result = await bondExtra(address, amount)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
    },
  })
}

/**
 * Hook for unbonding tokens
 */
export function useUnbond() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({
      address,
      amount,
    }: {
      address: string
      amount: string
    }) => {
      await initApi()
      const result = await unbond(address, amount)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
    },
  })
}

/**
 * Hook for withdrawing unbonded tokens
 */
export function useWithdrawUnbonded() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({
      address,
      numSlashingSpans = 0,
    }: {
      address: string
      numSlashingSpans?: number
    }) => {
      await initApi()
      const result = await withdrawUnbonded(address, numSlashingSpans)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
    },
  })
}

/**
 * Hook for nominating validators
 */
export function useNominate() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({
      address,
      targets,
    }: {
      address: string
      targets: string[]
    }) => {
      await initApi()
      const result = await nominate(address, targets)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
    },
  })
}

/**
 * Hook for stopping nominations (chill)
 */
export function useChill() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({ address }: { address: string }) => {
      await initApi()
      const result = await chill(address)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
    },
  })
}

/**
 * Utility function to format ETR token amounts
 */
export function formatETR(amount: string | bigint, decimals = 2): string {
  try {
    const formatted = formatBalance(amount, {
      decimals: 12,
      withUnit: false,
      forceUnit: '-',
    })

    const num = parseFloat(formatted.replace(/,/g, ''))

    if (num >= 1000000) {
      return `${(num / 1000000).toFixed(decimals)}M`
    }
    if (num >= 1000) {
      return `${(num / 1000).toFixed(decimals)}K`
    }
    return num.toFixed(decimals)
  } catch (error) {
    return '0'
  }
}

/**
 * Utility function to calculate commission percentage
 */
export function formatCommission(commission: string): string {
  try {
    // Commission is in perbill (parts per billion)
    // 1,000,000,000 = 100%
    const perbill = BigInt(commission)
    const percentage = Number(perbill) / 10000000
    return percentage.toFixed(2)
  } catch (error) {
    return '0.00'
  }
}

/**
 * Utility function to truncate addresses
 */
export function truncateAddress(address: string, startLength = 6, endLength = 4): string {
  if (address.length <= startLength + endLength) return address
  return `${address.slice(0, startLength)}...${address.slice(-endLength)}`
}

/**
 * Hook for fetching minimum validator bond
 */
export function useMinimumValidatorBond() {
  return useQuery<string>({
    queryKey: ['minimumValidatorBond'],
    queryFn: async () => {
      await initApi()
      const { getMinimumValidatorBond } = await import('@/lib/polkadot/staking')
      return getMinimumValidatorBond()
    },
    staleTime: 300000, // Consider data fresh for 5 minutes
  })
}

/**
 * Hook for checking if address is a validator
 */
export function useIsValidator(address?: string) {
  return useQuery<boolean>({
    queryKey: ['isValidator', address],
    queryFn: async () => {
      if (!address) return false
      await initApi()
      const { isValidator } = await import('@/lib/polkadot/staking')
      return isValidator(address)
    },
    enabled: !!address,
    refetchInterval: 30000, // Refetch every 30 seconds
  })
}

/**
 * Hook for getting validator counts
 */
export function useValidatorCounts() {
  return useQuery<{
    current: number
    max: number
    slotsAvailable: number
  }>({
    queryKey: ['validatorCounts'],
    queryFn: async () => {
      await initApi()
      const { getValidatorCounts } = await import('@/lib/polkadot/staking')
      return getValidatorCounts()
    },
    refetchInterval: 30000,
  })
}

/**
 * Hook for setting session keys
 */
export function useSetKeys() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({
      address,
      keys,
      proof = '0x00',
    }: {
      address: string
      keys: string
      proof?: string
    }) => {
      await initApi()
      const { setKeys } = await import('@/lib/polkadot/staking')
      const result = await setKeys(address, keys, proof)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
    },
  })
}

/**
 * Hook for declaring intention to validate
 */
export function useValidate() {
  const queryClient = useQueryClient()
  const [txHash, setTxHash] = useState<string | null>(null)

  return useMutation({
    mutationFn: async ({
      address,
      commission,
    }: {
      address: string
      commission: number
    }) => {
      await initApi()
      const { validate } = await import('@/lib/polkadot/staking')
      const result = await validate(address, commission)
      setTxHash(result.txHash)
      return result
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['stakingInfo', variables.address] })
      queryClient.invalidateQueries({ queryKey: ['isValidator', variables.address] })
      queryClient.invalidateQueries({ queryKey: ['validators'] })
    },
  })
}
