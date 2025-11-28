'use client'

import { useState, useEffect } from 'react'
import { useAccount } from 'wagmi'
import { ConnectButton } from '@rainbow-me/rainbowkit'
import { useMasterChef } from '@/hooks/useMasterChef'
import { useTokenApproval } from '@/hooks/useTokenApproval'
import { CONTRACTS } from '@/config/contracts'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Loader2, CheckCircle2, AlertCircle } from 'lucide-react'
import { useToast } from '@/hooks/use-toast'

interface MasterChefStakingProps {
  poolId?: number
  lpTokenAddress?: `0x${string}`
  lpTokenSymbol?: string
}

export function MasterChefStaking({
  poolId = 0,
  lpTokenAddress,
  lpTokenSymbol = 'LP',
}: MasterChefStakingProps) {
  const { address, isConnected } = useAccount()
  const { toast } = useToast()
  const [stakeAmount, setStakeAmount] = useState('')
  const [unstakeAmount, setUnstakeAmount] = useState('')

  // MasterChef hooks
  const {
    pendingReward,
    stakedAmount,
    deposit,
    withdraw,
    harvest,
    emergencyWithdraw,
    isDepositPending,
    isDepositSuccess,
    depositError,
    isWithdrawPending,
    isWithdrawSuccess,
    withdrawError,
    isHarvestPending,
    isHarvestSuccess,
    harvestError,
    isEmergencyWithdrawPending,
    isEmergencyWithdrawSuccess,
    emergencyWithdrawError,
    isLoading,
    refetchAll,
  } = useMasterChef(poolId)

  // Token approval hooks
  const {
    needsApproval,
    approve,
    approveMax,
    isApprovePending,
    isApproveSuccess,
    approveError,
  } = useTokenApproval(lpTokenAddress, CONTRACTS.MASTERCHEF)

  // Handle successful transactions
  useEffect(() => {
    if (isDepositSuccess) {
      toast({
        title: 'Stake Successful',
        description: `Successfully staked ${stakeAmount} ${lpTokenSymbol}`,
      })
      setStakeAmount('')
      refetchAll()
    }
  }, [isDepositSuccess, stakeAmount, lpTokenSymbol, toast, refetchAll])

  useEffect(() => {
    if (isWithdrawSuccess) {
      toast({
        title: 'Unstake Successful',
        description: `Successfully unstaked ${unstakeAmount} ${lpTokenSymbol}`,
      })
      setUnstakeAmount('')
      refetchAll()
    }
  }, [isWithdrawSuccess, unstakeAmount, lpTokenSymbol, toast, refetchAll])

  useEffect(() => {
    if (isHarvestSuccess) {
      toast({
        title: 'Harvest Successful',
        description: `Successfully harvested ${pendingReward} ETR`,
      })
      refetchAll()
    }
  }, [isHarvestSuccess, pendingReward, toast, refetchAll])

  useEffect(() => {
    if (isApproveSuccess) {
      toast({
        title: 'Approval Successful',
        description: `${lpTokenSymbol} spending approved`,
      })
    }
  }, [isApproveSuccess, lpTokenSymbol, toast])

  // Handle errors
  useEffect(() => {
    if (depositError) {
      toast({
        title: 'Stake Failed',
        description: depositError.message,
        variant: 'destructive',
      })
    }
  }, [depositError, toast])

  useEffect(() => {
    if (withdrawError) {
      toast({
        title: 'Unstake Failed',
        description: withdrawError.message,
        variant: 'destructive',
      })
    }
  }, [withdrawError, toast])

  useEffect(() => {
    if (harvestError) {
      toast({
        title: 'Harvest Failed',
        description: harvestError.message,
        variant: 'destructive',
      })
    }
  }, [harvestError, toast])

  useEffect(() => {
    if (approveError) {
      toast({
        title: 'Approval Failed',
        description: approveError.message,
        variant: 'destructive',
      })
    }
  }, [approveError, toast])

  const handleStake = async () => {
    try {
      if (!stakeAmount || parseFloat(stakeAmount) <= 0) {
        toast({
          title: 'Invalid Amount',
          description: 'Please enter a valid amount to stake',
          variant: 'destructive',
        })
        return
      }

      if (lpTokenAddress && needsApproval(stakeAmount)) {
        toast({
          title: 'Approval Required',
          description: 'Please approve token spending first',
        })
        return
      }

      deposit(stakeAmount)
    } catch (error: any) {
      toast({
        title: 'Error',
        description: error.message || 'Failed to stake',
        variant: 'destructive',
      })
    }
  }

  const handleUnstake = async () => {
    try {
      if (!unstakeAmount || parseFloat(unstakeAmount) <= 0) {
        toast({
          title: 'Invalid Amount',
          description: 'Please enter a valid amount to unstake',
          variant: 'destructive',
        })
        return
      }

      if (parseFloat(unstakeAmount) > parseFloat(stakedAmount)) {
        toast({
          title: 'Insufficient Balance',
          description: 'You cannot unstake more than your staked amount',
          variant: 'destructive',
        })
        return
      }

      withdraw(unstakeAmount)
    } catch (error: any) {
      toast({
        title: 'Error',
        description: error.message || 'Failed to unstake',
        variant: 'destructive',
      })
    }
  }

  const handleHarvest = async () => {
    try {
      harvest()
    } catch (error: any) {
      toast({
        title: 'Error',
        description: error.message || 'Failed to harvest',
        variant: 'destructive',
      })
    }
  }

  const handleApprove = async () => {
    try {
      approveMax()
    } catch (error: any) {
      toast({
        title: 'Error',
        description: error.message || 'Failed to approve',
        variant: 'destructive',
      })
    }
  }

  if (!isConnected) {
    return (
      <Card className="w-full max-w-2xl mx-auto">
        <CardHeader>
          <CardTitle>MasterChef LP Staking</CardTitle>
          <CardDescription>
            Connect your wallet to start staking and earning rewards on ETH PBC
          </CardDescription>
        </CardHeader>
        <CardContent className="flex justify-center py-8">
          <ConnectButton />
        </CardContent>
      </Card>
    )
  }

  return (
    <div className="w-full max-w-2xl mx-auto space-y-6">
      {/* Rewards Card */}
      <Card>
        <CardHeader>
          <CardTitle>Your Rewards</CardTitle>
          <CardDescription>Claim your earned ETR rewards</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
            <div>
              <p className="text-sm text-muted-foreground">Pending Rewards</p>
              <p className="text-2xl font-bold">
                {isLoading ? (
                  <Loader2 className="h-6 w-6 animate-spin" />
                ) : (
                  `${parseFloat(pendingReward).toFixed(6)} ETR`
                )}
              </p>
            </div>
            <Button
              onClick={handleHarvest}
              disabled={isHarvestPending || parseFloat(pendingReward) === 0}
              size="lg"
            >
              {isHarvestPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Harvest
            </Button>
          </div>

          <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
            <div>
              <p className="text-sm text-muted-foreground">Staked Amount</p>
              <p className="text-2xl font-bold">
                {isLoading ? (
                  <Loader2 className="h-6 w-6 animate-spin" />
                ) : (
                  `${parseFloat(stakedAmount).toFixed(6)} ${lpTokenSymbol}`
                )}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Token Approval Card */}
      {lpTokenAddress && needsApproval(stakeAmount || '0') && parseFloat(stakeAmount || '0') > 0 && (
        <Alert>
          <AlertCircle className="h-4 w-4" />
          <AlertDescription className="flex items-center justify-between">
            <span>You need to approve {lpTokenSymbol} spending before staking</span>
            <Button onClick={handleApprove} disabled={isApprovePending} size="sm">
              {isApprovePending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Approve
            </Button>
          </AlertDescription>
        </Alert>
      )}

      {/* Stake Card */}
      <Card>
        <CardHeader>
          <CardTitle>Stake {lpTokenSymbol} Tokens</CardTitle>
          <CardDescription>Deposit your LP tokens to earn ETR rewards</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="stake-amount">Amount to Stake</Label>
            <Input
              id="stake-amount"
              type="number"
              placeholder="0.0"
              value={stakeAmount}
              onChange={(e) => setStakeAmount(e.target.value)}
              disabled={isDepositPending}
            />
          </div>
          <Button
            onClick={handleStake}
            disabled={
              isDepositPending ||
              !stakeAmount ||
              parseFloat(stakeAmount) <= 0 ||
              (lpTokenAddress && needsApproval(stakeAmount))
            }
            className="w-full"
            size="lg"
          >
            {isDepositPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Stake
          </Button>
        </CardContent>
      </Card>

      {/* Unstake Card */}
      <Card>
        <CardHeader>
          <CardTitle>Unstake {lpTokenSymbol} Tokens</CardTitle>
          <CardDescription>Withdraw your staked LP tokens</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="unstake-amount">Amount to Unstake</Label>
            <Input
              id="unstake-amount"
              type="number"
              placeholder="0.0"
              value={unstakeAmount}
              onChange={(e) => setUnstakeAmount(e.target.value)}
              disabled={isWithdrawPending}
            />
            <Button
              variant="outline"
              size="sm"
              onClick={() => setUnstakeAmount(stakedAmount)}
              disabled={parseFloat(stakedAmount) === 0}
            >
              Max
            </Button>
          </div>
          <Button
            onClick={handleUnstake}
            disabled={
              isWithdrawPending ||
              !unstakeAmount ||
              parseFloat(unstakeAmount) <= 0 ||
              parseFloat(unstakeAmount) > parseFloat(stakedAmount)
            }
            className="w-full"
            size="lg"
            variant="secondary"
          >
            {isWithdrawPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Unstake
          </Button>
        </CardContent>
      </Card>

      {/* Emergency Withdraw Card */}
      <Card className="border-destructive">
        <CardHeader>
          <CardTitle className="text-destructive">Emergency Withdraw</CardTitle>
          <CardDescription>
            Withdraw all staked tokens immediately. WARNING: You will forfeit all pending rewards!
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Button
            onClick={emergencyWithdraw}
            disabled={isEmergencyWithdrawPending || parseFloat(stakedAmount) === 0}
            variant="destructive"
            className="w-full"
          >
            {isEmergencyWithdrawPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Emergency Withdraw
          </Button>
        </CardContent>
      </Card>
    </div>
  )
}
