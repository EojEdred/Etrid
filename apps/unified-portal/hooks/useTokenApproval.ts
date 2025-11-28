'use client'

import { useAccount, useReadContract, useWriteContract, useWaitForTransactionReceipt } from 'wagmi'
import { parseEther, maxUint256 } from 'viem'
import { useCallback } from 'react'

// Standard ERC20 ABI for approve and allowance
const ERC20_ABI = [
  {
    inputs: [
      { name: 'spender', type: 'address' },
      { name: 'amount', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', type: 'bool' }],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      { name: 'owner', type: 'address' },
      { name: 'spender', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function',
  },
  {
    inputs: [{ name: 'account', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const

/**
 * Hook for managing ERC20 token approvals
 * @param tokenAddress - The address of the ERC20 token to approve
 * @param spenderAddress - The address that will be allowed to spend tokens (e.g., MasterChef contract)
 */
export function useTokenApproval(
  tokenAddress: `0x${string}` | undefined,
  spenderAddress: `0x${string}` | undefined
) {
  const { address } = useAccount()

  // Read current allowance
  const {
    data: allowance,
    isLoading: isLoadingAllowance,
    refetch: refetchAllowance,
  } = useReadContract({
    address: tokenAddress,
    abi: ERC20_ABI,
    functionName: 'allowance',
    args: address && spenderAddress ? [address, spenderAddress] : undefined,
    query: {
      enabled: !!address && !!tokenAddress && !!spenderAddress,
    },
  })

  // Read token balance
  const {
    data: balance,
    isLoading: isLoadingBalance,
    refetch: refetchBalance,
  } = useReadContract({
    address: tokenAddress,
    abi: ERC20_ABI,
    functionName: 'balanceOf',
    args: address ? [address] : undefined,
    query: {
      enabled: !!address && !!tokenAddress,
    },
  })

  // Write approve
  const {
    writeContract: approveWrite,
    data: approveData,
    isPending: isApprovePending,
    error: approveError,
  } = useWriteContract()

  // Wait for approval transaction
  const { isLoading: isApproveConfirming, isSuccess: isApproveSuccess } =
    useWaitForTransactionReceipt({
      hash: approveData,
    })

  // Approve function with amount
  const approve = useCallback(
    (amount: string) => {
      if (!address || !tokenAddress || !spenderAddress) {
        throw new Error('Missing required parameters')
      }
      approveWrite({
        address: tokenAddress,
        abi: ERC20_ABI,
        functionName: 'approve',
        args: [spenderAddress, parseEther(amount)],
      })
    },
    [address, tokenAddress, spenderAddress, approveWrite]
  )

  // Approve infinite amount (max uint256)
  const approveMax = useCallback(() => {
    if (!address || !tokenAddress || !spenderAddress) {
      throw new Error('Missing required parameters')
    }
    approveWrite({
      address: tokenAddress,
      abi: ERC20_ABI,
      functionName: 'approve',
      args: [spenderAddress, maxUint256],
    })
  }, [address, tokenAddress, spenderAddress, approveWrite])

  // Check if approval is needed for a specific amount
  const needsApproval = useCallback(
    (amount: string): boolean => {
      if (!allowance) return true
      try {
        const amountBigInt = parseEther(amount)
        return (allowance as bigint) < amountBigInt
      } catch {
        return true
      }
    },
    [allowance]
  )

  return {
    // Read data
    allowance: allowance as bigint | undefined,
    balance: balance as bigint | undefined,
    isLoadingAllowance,
    isLoadingBalance,

    // Write functions
    approve,
    approveMax,
    needsApproval,

    // Transaction states
    isApprovePending: isApprovePending || isApproveConfirming,
    isApproveSuccess,
    approveError,
    approveTxHash: approveData,

    // Utility
    refetchAllowance,
    refetchBalance,
  }
}
