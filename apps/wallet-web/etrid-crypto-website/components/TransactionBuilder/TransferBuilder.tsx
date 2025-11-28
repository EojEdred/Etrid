/**
 * TransferBuilder - Transfer Transaction Builder Component
 * Handles token transfers between accounts
 */

'use client';

import React, { useState, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS, ChainId, getAllChains } from '@/lib/polkadot/chains';
import { getBalance, formatBalance, parseBalance } from '@/lib/polkadot/api';
import { AlertCircle, ArrowRight, Loader2 } from 'lucide-react';

interface TransferFormData {
  recipient: string;
  amount: string;
  chainId: ChainId;
  memo?: string;
}

interface TransferBuilderProps {
  onComplete: (data: TransferFormData & { estimatedFee: string }) => void;
}

export function TransferBuilder({ onComplete }: TransferBuilderProps) {
  const { selectedAccount, selectedChain, setSelectedChain } = useWallet();
  const [estimatedFee, setEstimatedFee] = useState<string>('0');
  const [isCalculatingFee, setIsCalculatingFee] = useState(false);
  const [recipientBalance, setRecipientBalance] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
    setValue,
  } = useForm<TransferFormData>({
    mode: 'onChange',
    defaultValues: {
      chainId: selectedChain,
      amount: '',
      recipient: '',
      memo: '',
    },
  });

  const watchedChainId = watch('chainId');
  const watchedRecipient = watch('recipient');
  const watchedAmount = watch('amount');

  // Update chain when changed
  useEffect(() => {
    if (watchedChainId) {
      setSelectedChain(watchedChainId);
    }
  }, [watchedChainId, setSelectedChain]);

  // Fetch recipient balance
  useEffect(() => {
    if (!watchedRecipient || watchedRecipient.length < 47) {
      setRecipientBalance(null);
      return;
    }

    const fetchRecipientBalance = async () => {
      try {
        const balance = await getBalance(watchedChainId, watchedRecipient);
        const config = CHAINS[watchedChainId];
        setRecipientBalance(formatBalance(balance, config.decimals));
      } catch (err) {
        setRecipientBalance(null);
      }
    };

    fetchRecipientBalance();
  }, [watchedRecipient, watchedChainId]);

  // Estimate transaction fee
  useEffect(() => {
    if (!watchedAmount || !watchedRecipient || !selectedAccount) {
      setEstimatedFee('0');
      return;
    }

    const calculateFee = async () => {
      setIsCalculatingFee(true);
      try {
        // In a real implementation, this would call the API to get actual fee estimation
        // For now, we'll use a fixed fee based on chain
        const config = CHAINS[watchedChainId];
        const baseFee = config.isRelay ? '0.01' : '0.001';
        setEstimatedFee(baseFee);
      } catch (err) {
        console.error('Fee calculation error:', err);
        setEstimatedFee('0.01');
      } finally {
        setIsCalculatingFee(false);
      }
    };

    const timer = setTimeout(calculateFee, 500);
    return () => clearTimeout(timer);
  }, [watchedAmount, watchedRecipient, selectedAccount, watchedChainId]);

  // Validate address format (basic Substrate address validation)
  const validateAddress = (address: string): boolean => {
    if (!address) return false;
    // Basic check: Substrate addresses are typically 47-48 characters
    return address.length >= 47 && address.length <= 48;
  };

  // Validate amount
  const validateAmount = (amount: string): boolean => {
    if (!amount) return false;
    const num = parseFloat(amount);
    if (isNaN(num) || num <= 0) return false;

    // Check if amount exceeds balance
    if (selectedAccount?.balance) {
      const balance = parseFloat(selectedAccount.balance);
      const fee = parseFloat(estimatedFee);
      return num + fee <= balance;
    }

    return true;
  };

  const onSubmit = (data: TransferFormData) => {
    setError(null);

    // Final validation
    if (!validateAddress(data.recipient)) {
      setError('Invalid recipient address');
      return;
    }

    if (!validateAmount(data.amount)) {
      setError('Invalid amount or insufficient balance');
      return;
    }

    // Pass data to parent
    onComplete({
      ...data,
      estimatedFee,
    });
  };

  const chains = getAllChains();
  const selectedChainConfig = CHAINS[watchedChainId];

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      {/* Chain Selection */}
      <div className="space-y-2">
        <Label htmlFor="chainId">Select Chain</Label>
        <Select
          value={watchedChainId}
          onValueChange={(value) => setValue('chainId', value as ChainId)}
        >
          <SelectTrigger id="chainId">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {chains.map((chain) => (
              <SelectItem key={chain.id} value={chain.id}>
                <div className="flex items-center gap-2">
                  <div
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: chain.color }}
                  />
                  <span>{chain.name}</span>
                  <Badge variant="secondary" className="ml-auto">
                    {chain.symbol}
                  </Badge>
                </div>
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      {/* Recipient Address */}
      <div className="space-y-2">
        <Label htmlFor="recipient">Recipient Address</Label>
        <Input
          id="recipient"
          placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
          {...register('recipient', {
            required: 'Recipient address is required',
            validate: (value) => validateAddress(value) || 'Invalid address format',
          })}
          className={errors.recipient ? 'border-destructive' : ''}
        />
        {errors.recipient && (
          <p className="text-sm text-destructive">{errors.recipient.message}</p>
        )}
        {recipientBalance && (
          <p className="text-xs text-muted-foreground">
            Recipient balance: {recipientBalance} {selectedChainConfig.symbol}
          </p>
        )}
      </div>

      {/* Amount */}
      <div className="space-y-2">
        <Label htmlFor="amount">Amount</Label>
        <div className="relative">
          <Input
            id="amount"
            type="number"
            step="any"
            placeholder="0.00"
            {...register('amount', {
              required: 'Amount is required',
              validate: (value) => validateAmount(value) || 'Invalid amount or insufficient balance',
            })}
            className={`pr-16 ${errors.amount ? 'border-destructive' : ''}`}
          />
          <div className="absolute right-3 top-1/2 -translate-y-1/2">
            <Badge variant="secondary">{selectedChainConfig.symbol}</Badge>
          </div>
        </div>
        {errors.amount && (
          <p className="text-sm text-destructive">{errors.amount.message}</p>
        )}
        {selectedAccount?.balance && (
          <div className="flex items-center justify-between text-xs">
            <span className="text-muted-foreground">
              Available: {selectedAccount.balance} {selectedChainConfig.symbol}
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
      </div>

      {/* Memo (Optional) */}
      <div className="space-y-2">
        <Label htmlFor="memo">Memo (Optional)</Label>
        <Input
          id="memo"
          placeholder="Add a note to this transfer"
          {...register('memo')}
          maxLength={256}
        />
        <p className="text-xs text-muted-foreground">
          Optional message attached to the transfer
        </p>
      </div>

      {/* Fee Estimation */}
      <Card className="bg-muted/50">
        <CardContent className="pt-6">
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Estimated Fee</span>
              {isCalculatingFee ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                <Badge variant="secondary" className="font-mono">
                  {estimatedFee} {selectedChainConfig.symbol}
                </Badge>
              )}
            </div>
            {watchedAmount && (
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Total (Amount + Fee)</span>
                <span className="font-medium">
                  {(parseFloat(watchedAmount || '0') + parseFloat(estimatedFee)).toFixed(6)}{' '}
                  {selectedChainConfig.symbol}
                </span>
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
        <Button
          type="submit"
          className="flex-1"
          disabled={!isValid || isCalculatingFee}
        >
          Review Transfer
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </form>
  );
}
