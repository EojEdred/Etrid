/**
 * BatchBuilder - Batch Transaction Builder Component
 * Allows users to create and submit multiple transactions at once
 */

'use client';

import React, { useState } from 'react';
import { useForm, useFieldArray } from 'react-hook-form';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS, ChainId } from '@/lib/polkadot/chains';
import { AlertCircle, ArrowRight, Loader2, Plus, Trash2, Info } from 'lucide-react';

interface BatchTransaction {
  recipient: string;
  amount: string;
  memo?: string;
}

interface BatchFormData {
  chainId: ChainId;
  transactions: BatchTransaction[];
}

interface BatchBuilderProps {
  onComplete: (data: BatchFormData & { estimatedFee: string; totalAmount: string }) => void;
}

export function BatchBuilder({ onComplete }: BatchBuilderProps) {
  const { selectedAccount, selectedChain, setSelectedChain } = useWallet();
  const [estimatedFee, setEstimatedFee] = useState<string>('0');
  const [isCalculatingFee, setIsCalculatingFee] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const {
    register,
    control,
    handleSubmit,
    watch,
    formState: { errors, isValid },
    setValue,
  } = useForm<BatchFormData>({
    mode: 'onChange',
    defaultValues: {
      chainId: selectedChain,
      transactions: [{ recipient: '', amount: '', memo: '' }],
    },
  });

  const { fields, append, remove } = useFieldArray({
    control,
    name: 'transactions',
  });

  const watchedChainId = watch('chainId');
  const watchedTransactions = watch('transactions');

  // Validate address format
  const validateAddress = (address: string): boolean => {
    if (!address) return false;
    return address.length >= 47 && address.length <= 48;
  };

  // Validate amount
  const validateAmount = (amount: string): boolean => {
    if (!amount) return false;
    const num = parseFloat(amount);
    return !isNaN(num) && num > 0;
  };

  // Calculate total amount
  const calculateTotalAmount = (): string => {
    return watchedTransactions
      .reduce((sum, tx) => sum + (parseFloat(tx.amount) || 0), 0)
      .toFixed(6);
  };

  // Estimate fee based on number of transactions
  React.useEffect(() => {
    const calculateFee = async () => {
      setIsCalculatingFee(true);
      try {
        const config = CHAINS[watchedChainId];
        // Base fee per transaction
        const baseFeePerTx = config.isRelay ? 0.01 : 0.001;
        // Batch overhead
        const batchOverhead = 0.005;
        // Total fee = (base * count) + overhead
        const totalFee = (baseFeePerTx * watchedTransactions.length + batchOverhead).toFixed(6);
        setEstimatedFee(totalFee);
      } catch (err) {
        console.error('Fee calculation error:', err);
        setEstimatedFee('0.01');
      } finally {
        setIsCalculatingFee(false);
      }
    };

    const timer = setTimeout(calculateFee, 500);
    return () => clearTimeout(timer);
  }, [watchedTransactions.length, watchedChainId]);

  const onSubmit = (data: BatchFormData) => {
    setError(null);

    // Validate all transactions
    for (let i = 0; i < data.transactions.length; i++) {
      const tx = data.transactions[i];
      if (!validateAddress(tx.recipient)) {
        setError(`Invalid recipient address in transaction ${i + 1}`);
        return;
      }
      if (!validateAmount(tx.amount)) {
        setError(`Invalid amount in transaction ${i + 1}`);
        return;
      }
    }

    // Check total amount + fee doesn't exceed balance
    const totalAmount = calculateTotalAmount();
    const balance = parseFloat(selectedAccount?.balance || '0');
    const fee = parseFloat(estimatedFee);

    if (parseFloat(totalAmount) + fee > balance) {
      setError('Insufficient balance for batch transaction');
      return;
    }

    // Pass data to parent
    onComplete({
      ...data,
      estimatedFee,
      totalAmount,
    });
  };

  const chainConfig = CHAINS[watchedChainId];

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
            <SelectItem value="flarechain">FlareChain</SelectItem>
            <SelectItem value="btc-pbc">BTC-PBC</SelectItem>
            <SelectItem value="eth-pbc">ETH-PBC</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {/* Batch Info Alert */}
      <Alert>
        <Info className="h-4 w-4" />
        <AlertDescription className="text-xs">
          Batch transactions allow you to send to multiple recipients in a single transaction,
          saving on fees compared to individual transfers.
        </AlertDescription>
      </Alert>

      {/* Transaction List */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <Label>Transactions ({fields.length})</Label>
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={() => append({ recipient: '', amount: '', memo: '' })}
            disabled={fields.length >= 10}
          >
            <Plus className="w-4 h-4 mr-2" />
            Add Transaction
          </Button>
        </div>

        {fields.map((field, index) => (
          <Card key={field.id} className="relative">
            <CardContent className="pt-6 space-y-4">
              <div className="flex items-center justify-between mb-4">
                <Badge variant="secondary">Transaction #{index + 1}</Badge>
                {fields.length > 1 && (
                  <Button
                    type="button"
                    variant="ghost"
                    size="sm"
                    onClick={() => remove(index)}
                  >
                    <Trash2 className="w-4 h-4 text-destructive" />
                  </Button>
                )}
              </div>

              {/* Recipient */}
              <div className="space-y-2">
                <Label htmlFor={`transactions.${index}.recipient`}>
                  Recipient Address
                </Label>
                <Input
                  id={`transactions.${index}.recipient`}
                  placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
                  {...register(`transactions.${index}.recipient`, {
                    required: 'Recipient address is required',
                    validate: (value) =>
                      validateAddress(value) || 'Invalid address format',
                  })}
                  className={
                    errors.transactions?.[index]?.recipient ? 'border-destructive' : ''
                  }
                />
                {errors.transactions?.[index]?.recipient && (
                  <p className="text-sm text-destructive">
                    {errors.transactions[index]?.recipient?.message}
                  </p>
                )}
              </div>

              {/* Amount */}
              <div className="space-y-2">
                <Label htmlFor={`transactions.${index}.amount`}>Amount</Label>
                <div className="relative">
                  <Input
                    id={`transactions.${index}.amount`}
                    type="number"
                    step="any"
                    placeholder="0.00"
                    {...register(`transactions.${index}.amount`, {
                      required: 'Amount is required',
                      validate: (value) =>
                        validateAmount(value) || 'Invalid amount',
                    })}
                    className={`pr-16 ${
                      errors.transactions?.[index]?.amount ? 'border-destructive' : ''
                    }`}
                  />
                  <div className="absolute right-3 top-1/2 -translate-y-1/2">
                    <Badge variant="secondary">{chainConfig.symbol}</Badge>
                  </div>
                </div>
                {errors.transactions?.[index]?.amount && (
                  <p className="text-sm text-destructive">
                    {errors.transactions[index]?.amount?.message}
                  </p>
                )}
              </div>

              {/* Memo (Optional) */}
              <div className="space-y-2">
                <Label htmlFor={`transactions.${index}.memo`}>Memo (Optional)</Label>
                <Input
                  id={`transactions.${index}.memo`}
                  placeholder="Add a note"
                  {...register(`transactions.${index}.memo`)}
                  maxLength={128}
                />
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Summary Card */}
      <Card className="bg-muted/50">
        <CardContent className="pt-6">
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Total Recipients</span>
              <Badge variant="secondary">{fields.length}</Badge>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Total Amount</span>
              <span className="font-mono">
                {calculateTotalAmount()} {chainConfig.symbol}
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Estimated Fee</span>
              {isCalculatingFee ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                <Badge variant="secondary" className="font-mono">
                  {estimatedFee} {chainConfig.symbol}
                </Badge>
              )}
            </div>
            <div className="flex items-center justify-between text-lg font-semibold pt-2 border-t">
              <span>Total Cost</span>
              <span>
                {(parseFloat(calculateTotalAmount()) + parseFloat(estimatedFee)).toFixed(6)}{' '}
                {chainConfig.symbol}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Balance Check */}
      {selectedAccount?.balance && (
        <Alert
          variant={
            parseFloat(calculateTotalAmount()) + parseFloat(estimatedFee) >
            parseFloat(selectedAccount.balance)
              ? 'destructive'
              : 'default'
          }
        >
          <Info className="h-4 w-4" />
          <AlertDescription className="text-xs">
            Available balance: {selectedAccount.balance} {chainConfig.symbol}
            {parseFloat(calculateTotalAmount()) + parseFloat(estimatedFee) >
              parseFloat(selectedAccount.balance) && ' - Insufficient balance!'}
          </AlertDescription>
        </Alert>
      )}

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
          disabled={!isValid || isCalculatingFee || fields.length === 0}
        >
          Review Batch Transaction
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </form>
  );
}
