/**
 * StakingBuilder - Staking Transaction Builder Component
 * Handles staking, unstaking, and reward claiming operations
 */

'use client';

import React, { useState, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS, ChainId } from '@/lib/polkadot/chains';
import { AlertCircle, ArrowRight, Lock, Unlock, Award, Loader2, Info } from 'lucide-react';

type StakingOperation = 'stake' | 'unstake' | 'claim';

interface StakingFormData {
  operation: StakingOperation;
  amount: string;
  validator?: string;
  chainId: ChainId;
}

interface StakingBuilderProps {
  onComplete: (data: StakingFormData & { estimatedFee: string; estimatedRewards?: string }) => void;
}

export function StakingBuilder({ onComplete }: StakingBuilderProps) {
  const { selectedAccount, selectedChain } = useWallet();
  const [estimatedFee, setEstimatedFee] = useState<string>('0');
  const [estimatedRewards, setEstimatedRewards] = useState<string>('0');
  const [stakedAmount, setStakedAmount] = useState<string>('0');
  const [isCalculating, setIsCalculating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
    setValue,
  } = useForm<StakingFormData>({
    mode: 'onChange',
    defaultValues: {
      operation: 'stake',
      amount: '',
      validator: '',
      chainId: selectedChain,
    },
  });

  const watchedOperation = watch('operation');
  const watchedAmount = watch('amount');
  const watchedValidator = watch('validator');

  // Simulate fetching staked amount and rewards
  useEffect(() => {
    if (!selectedAccount) return;

    const fetchStakingInfo = async () => {
      try {
        // In a real implementation, this would fetch from the chain
        // Simulated values for demonstration
        setStakedAmount('100.5');
        setEstimatedRewards('5.25');
      } catch (err) {
        console.error('Failed to fetch staking info:', err);
      }
    };

    fetchStakingInfo();
  }, [selectedAccount]);

  // Estimate transaction fee
  useEffect(() => {
    if (!watchedAmount && watchedOperation !== 'claim') {
      setEstimatedFee('0');
      return;
    }

    const calculateFee = async () => {
      setIsCalculating(true);
      try {
        const config = CHAINS[selectedChain];
        // Different operations have different fees
        let baseFee = '0.01';
        if (watchedOperation === 'stake') {
          baseFee = '0.02';
        } else if (watchedOperation === 'unstake') {
          baseFee = '0.015';
        } else if (watchedOperation === 'claim') {
          baseFee = '0.005';
        }
        setEstimatedFee(baseFee);
      } catch (err) {
        console.error('Fee calculation error:', err);
        setEstimatedFee('0.01');
      } finally {
        setIsCalculating(false);
      }
    };

    const timer = setTimeout(calculateFee, 500);
    return () => clearTimeout(timer);
  }, [watchedAmount, watchedOperation, selectedChain]);

  // Validate amount
  const validateAmount = (amount: string): boolean => {
    if (!amount && watchedOperation !== 'claim') return false;
    if (watchedOperation === 'claim') return true; // No amount needed for claim

    const num = parseFloat(amount);
    if (isNaN(num) || num <= 0) return false;

    // Check minimum staking amount
    if (watchedOperation === 'stake' && num < 1) {
      return false;
    }

    // Check if unstake amount exceeds staked
    if (watchedOperation === 'unstake') {
      const staked = parseFloat(stakedAmount);
      return num <= staked;
    }

    // Check if stake amount exceeds balance
    if (watchedOperation === 'stake' && selectedAccount?.balance) {
      const balance = parseFloat(selectedAccount.balance);
      const fee = parseFloat(estimatedFee);
      return num + fee <= balance;
    }

    return true;
  };

  // Validate validator address
  const validateValidator = (address: string | undefined): boolean => {
    if (watchedOperation !== 'stake') return true; // Only required for staking
    if (!address) return false;
    return address.length >= 47 && address.length <= 48;
  };

  const onSubmit = (data: StakingFormData) => {
    setError(null);

    // Validate amount for stake/unstake
    if (data.operation !== 'claim' && !validateAmount(data.amount)) {
      setError('Invalid amount');
      return;
    }

    // Validate validator for staking
    if (data.operation === 'stake' && !validateValidator(data.validator)) {
      setError('Invalid validator address');
      return;
    }

    // Pass data to parent
    onComplete({
      ...data,
      estimatedFee,
      estimatedRewards: watchedOperation === 'claim' ? estimatedRewards : undefined,
    });
  };

  const chainConfig = CHAINS[selectedChain];

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      {/* Staking Info Card */}
      <Card className="bg-gradient-to-br from-primary/5 to-primary/10 border-primary/20">
        <CardContent className="pt-6">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1">
              <p className="text-sm text-muted-foreground">Currently Staked</p>
              <p className="text-2xl font-bold">
                {stakedAmount} {chainConfig.symbol}
              </p>
            </div>
            <div className="space-y-1">
              <p className="text-sm text-muted-foreground">Pending Rewards</p>
              <p className="text-2xl font-bold text-green-600 dark:text-green-400">
                {estimatedRewards} {chainConfig.symbol}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Operation Selection */}
      <div className="space-y-3">
        <Label>Select Operation</Label>
        <RadioGroup
          value={watchedOperation}
          onValueChange={(value) => setValue('operation', value as StakingOperation)}
        >
          <div className="grid grid-cols-3 gap-3">
            <Card
              className={`cursor-pointer transition-all ${
                watchedOperation === 'stake'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setValue('operation', 'stake')}
            >
              <CardContent className="pt-6 pb-6 flex flex-col items-center gap-2">
                <RadioGroupItem value="stake" id="stake" className="sr-only" />
                <Lock className="w-8 h-8 text-primary" />
                <Label htmlFor="stake" className="cursor-pointer font-medium">
                  Stake
                </Label>
              </CardContent>
            </Card>

            <Card
              className={`cursor-pointer transition-all ${
                watchedOperation === 'unstake'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setValue('operation', 'unstake')}
            >
              <CardContent className="pt-6 pb-6 flex flex-col items-center gap-2">
                <RadioGroupItem value="unstake" id="unstake" className="sr-only" />
                <Unlock className="w-8 h-8 text-orange-500" />
                <Label htmlFor="unstake" className="cursor-pointer font-medium">
                  Unstake
                </Label>
              </CardContent>
            </Card>

            <Card
              className={`cursor-pointer transition-all ${
                watchedOperation === 'claim'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setValue('operation', 'claim')}
            >
              <CardContent className="pt-6 pb-6 flex flex-col items-center gap-2">
                <RadioGroupItem value="claim" id="claim" className="sr-only" />
                <Award className="w-8 h-8 text-green-500" />
                <Label htmlFor="claim" className="cursor-pointer font-medium">
                  Claim Rewards
                </Label>
              </CardContent>
            </Card>
          </div>
        </RadioGroup>
      </div>

      {/* Validator Selection (only for staking) */}
      {watchedOperation === 'stake' && (
        <div className="space-y-2">
          <Label htmlFor="validator">Validator Address</Label>
          <Input
            id="validator"
            placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
            {...register('validator', {
              required: watchedOperation === 'stake',
              validate: (value) => validateValidator(value) || 'Invalid validator address',
            })}
            className={errors.validator ? 'border-destructive' : ''}
          />
          {errors.validator && (
            <p className="text-sm text-destructive">{errors.validator.message}</p>
          )}
          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription className="text-xs">
              Choose a trusted validator. Higher commission means lower rewards.
            </AlertDescription>
          </Alert>
        </div>
      )}

      {/* Amount Input (not needed for claim) */}
      {watchedOperation !== 'claim' && (
        <div className="space-y-2">
          <Label htmlFor="amount">
            {watchedOperation === 'stake' ? 'Amount to Stake' : 'Amount to Unstake'}
          </Label>
          <div className="relative">
            <Input
              id="amount"
              type="number"
              step="any"
              placeholder="0.00"
              {...register('amount', {
                required: watchedOperation !== 'claim',
                validate: (value) => validateAmount(value) || 'Invalid amount',
              })}
              className={`pr-16 ${errors.amount ? 'border-destructive' : ''}`}
            />
            <div className="absolute right-3 top-1/2 -translate-y-1/2">
              <Badge variant="secondary">{chainConfig.symbol}</Badge>
            </div>
          </div>
          {errors.amount && (
            <p className="text-sm text-destructive">{errors.amount.message}</p>
          )}

          {/* Helper text */}
          {watchedOperation === 'stake' && selectedAccount?.balance && (
            <div className="flex items-center justify-between text-xs">
              <span className="text-muted-foreground">
                Available: {selectedAccount.balance} {chainConfig.symbol}
              </span>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                className="h-auto p-0 text-xs text-primary"
                onClick={() => {
                  const maxAmount = parseFloat(selectedAccount.balance || '0') - parseFloat(estimatedFee);
                  setValue('amount', maxAmount > 0 ? maxAmount.toString() : '0');
                }}
              >
                Use Max
              </Button>
            </div>
          )}

          {watchedOperation === 'unstake' && (
            <p className="text-xs text-muted-foreground">
              Maximum: {stakedAmount} {chainConfig.symbol}
            </p>
          )}

          {watchedOperation === 'stake' && (
            <Alert>
              <Info className="h-4 w-4" />
              <AlertDescription className="text-xs">
                Minimum stake: 1 {chainConfig.symbol}. Unbonding period: 28 days.
              </AlertDescription>
            </Alert>
          )}
        </div>
      )}

      {/* Claim Rewards Info */}
      {watchedOperation === 'claim' && (
        <Alert className="bg-green-50 dark:bg-green-950/20 border-green-200 dark:border-green-800">
          <Award className="h-4 w-4 text-green-600" />
          <AlertDescription>
            You will claim {estimatedRewards} {chainConfig.symbol} in rewards
          </AlertDescription>
        </Alert>
      )}

      {/* Fee Estimation */}
      <Card className="bg-muted/50">
        <CardContent className="pt-6">
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Estimated Fee</span>
              {isCalculating ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                <Badge variant="secondary" className="font-mono">
                  {estimatedFee} {chainConfig.symbol}
                </Badge>
              )}
            </div>

            {/* Staking APY Info */}
            {watchedOperation === 'stake' && watchedAmount && (
              <div className="space-y-2 pt-2 border-t">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">Estimated APY</span>
                  <span className="font-medium text-green-600 dark:text-green-400">~12.5%</span>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">Yearly Rewards (est.)</span>
                  <span className="font-medium">
                    {(parseFloat(watchedAmount) * 0.125).toFixed(4)} {chainConfig.symbol}
                  </span>
                </div>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Error Display */}
      {error && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Submit Button */}
      <div className="flex gap-3 pt-4">
        <Button type="submit" className="flex-1" disabled={!isValid || isCalculating}>
          Review {watchedOperation === 'stake' ? 'Staking' : watchedOperation === 'unstake' ? 'Unstaking' : 'Claim'}
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </form>
  );
}
