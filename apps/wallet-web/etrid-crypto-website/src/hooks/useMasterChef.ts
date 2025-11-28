'use client'

import { useAccount, useReadContract, useWriteContract, useWaitForTransactionReceipt } from 'wagmi'
import { parseEther, formatEther } from 'viem'
import { CONTRACTS } from '@/config/contracts'
import MasterChefABI from '@/abis/MasterChef.json'
import { useState, useCallback } from 'react'

export interface PoolInfo {
  lpToken: string
  allocPoint: bigint
  lastRewardBlock: bigint
  accRewardPerShare: bigint
}

export interface UserInfo {
  amount: bigint
  rewardDebt: bigint
}

/**
 * Hook for interacting with the MasterChef staking contract
 * @param poolId - The ID of the staking pool
 */
export function useMasterChef(poolId: number = 0) {
  const { address } = useAccount()
  const [lastTxHash, setLastTxHash] = useState<`0x${string}` | undefined>()

  // Read pending rewards
  const {
    data: pendingReward,
    isLoading: isLoadingRewards,
    refetch: refetchRewards,
  } = useReadContract({
    address: CONTRACTS.MASTERCHEF,
    abi: MasterChefABI.abi,
    functionName: 'pendingReward',
    args: [poolId, address],
    query: {
      enabled: !!address,
      refetchInterval: 10000, // Refetch every 10 seconds
    },
  })

  // Read user info
  const {
    data: userInfo,
    isLoading: isLoadingUserInfo,
    refetch: refetchUserInfo,
  } = useReadContract({
    address: CONTRACTS.MASTERCHEF,
    abi: MasterChefABI.abi,
    functionName: 'userInfo',
    args: [poolId, address],
    query: {
      enabled: !!address,
    },
  }) as { data: UserInfo | undefined; isLoading: boolean; refetch: () => void }

  // Read pool info
  const {
    data: poolInfo,
    isLoading: isLoadingPoolInfo,
    refetch: refetchPoolInfo,
  } = useReadContract({
    address: CONTRACTS.MASTERCHEF,
    abi: MasterChefABI.abi,
    functionName: 'poolInfo',
    args: [poolId],
  }) as { data: PoolInfo | undefined; isLoading: boolean; refetch: () => void }

  // Write contract hooks
  const {
    writeContract: depositWrite,
    data: depositData,
    isPending: isDepositPending,
    error: depositError,
  } = useWriteContract()

  const {
    writeContract: withdrawWrite,
    data: withdrawData,
    isPending: isWithdrawPending,
    error: withdrawError,
  } = useWriteContract()

  const {
    writeContract: harvestWrite,
    data: harvestData,
    isPending: isHarvestPending,
    error: harvestError,
  } = useWriteContract()

  const {
    writeContract: emergencyWithdrawWrite,
    data: emergencyWithdrawData,
    isPending: isEmergencyWithdrawPending,
    error: emergencyWithdrawError,
  } = useWriteContract()

  // Wait for transaction receipts
  const { isLoading: isDepositConfirming, isSuccess: isDepositSuccess } =
    useWaitForTransactionReceipt({
      hash: depositData,
    })

  const { isLoading: isWithdrawConfirming, isSuccess: isWithdrawSuccess } =
    useWaitForTransactionReceipt({
      hash: withdrawData,
    })

  const { isLoading: isHarvestConfirming, isSuccess: isHarvestSuccess } =
    useWaitForTransactionReceipt({
      hash: harvestData,
    })

  const {
    isLoading: isEmergencyWithdrawConfirming,
    isSuccess: isEmergencyWithdrawSuccess,
  } = useWaitForTransactionReceipt({
    hash: emergencyWithdrawData,
  })

  // Deposit (stake) function
  const deposit = useCallback(
    (amount: string) => {
      if (!address) {
        throw new Error('Wallet not connected')
      }
      depositWrite({
        address: CONTRACTS.MASTERCHEF,
        abi: MasterChefABI.abi,
        functionName: 'deposit',
        args: [poolId, parseEther(amount)],
      })
    },
    [address, depositWrite, poolId]
  )

  // Withdraw (unstake) function
  const withdraw = useCallback(
    (amount: string) => {
      if (!address) {
        throw new Error('Wallet not connected')
      }
      withdrawWrite({
        address: CONTRACTS.MASTERCHEF,
        abi: MasterChefABI.abi,
        functionName: 'withdraw',
        args: [poolId, parseEther(amount)],
      })
    },
    [address, withdrawWrite, poolId]
  )

  // Harvest rewards function
  const harvest = useCallback(() => {
    if (!address) {
      throw new Error('Wallet not connected')
    }
    harvestWrite({
      address: CONTRACTS.MASTERCHEF,
      abi: MasterChefABI.abi,
      functionName: 'harvest',
      args: [poolId],
    })
  }, [address, harvestWrite, poolId])

  // Emergency withdraw function (forfeit rewards)
  const emergencyWithdraw = useCallback(() => {
    if (!address) {
      throw new Error('Wallet not connected')
    }
    emergencyWithdrawWrite({
      address: CONTRACTS.MASTERCHEF,
      abi: MasterChefABI.abi,
      functionName: 'emergencyWithdraw',
      args: [poolId],
    })
  }, [address, emergencyWithdrawWrite, poolId])

  // Refetch all data
  const refetchAll = useCallback(() => {
    refetchRewards()
    refetchUserInfo()
    refetchPoolInfo()
  }, [refetchRewards, refetchUserInfo, refetchPoolInfo])

  return {
    // Read data
    pendingReward: pendingReward ? formatEther(pendingReward as bigint) : '0',
    pendingRewardRaw: pendingReward as bigint | undefined,
    userInfo,
    stakedAmount: userInfo ? formatEther(userInfo.amount) : '0',
    stakedAmountRaw: userInfo?.amount,
    poolInfo,

    // Loading states
    isLoadingRewards,
    isLoadingUserInfo,
    isLoadingPoolInfo,
    isLoading: isLoadingRewards || isLoadingUserInfo || isLoadingPoolInfo,

    // Write functions
    deposit,
    withdraw,
    harvest,
    emergencyWithdraw,

    // Transaction states
    isDepositPending: isDepositPending || isDepositConfirming,
    isDepositSuccess,
    depositError,
    depositTxHash: depositData,

    isWithdrawPending: isWithdrawPending || isWithdrawConfirming,
    isWithdrawSuccess,
    withdrawError,
    withdrawTxHash: withdrawData,

    isHarvestPending: isHarvestPending || isHarvestConfirming,
    isHarvestSuccess,
    harvestError,
    harvestTxHash: harvestData,

    isEmergencyWithdrawPending: isEmergencyWithdrawPending || isEmergencyWithdrawConfirming,
    isEmergencyWithdrawSuccess,
    emergencyWithdrawError,
    emergencyWithdrawTxHash: emergencyWithdrawData,

    // Utility
    refetchAll,
  }
}
