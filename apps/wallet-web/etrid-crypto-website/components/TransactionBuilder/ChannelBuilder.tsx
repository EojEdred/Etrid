/**
 * ChannelBuilder - Payment Channel Transaction Builder Component
 * Handles opening, closing, and updating payment channels
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
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS, ChainId } from '@/lib/polkadot/chains';
import { AlertCircle, ArrowRight, Loader2, Info, Plus, X, RefreshCw } from 'lucide-react';

type ChannelOperation = 'open' | 'close' | 'update';

interface PaymentChannel {
  id: string;
  counterparty: string;
  balance: string;
  status: 'open' | 'closing' | 'closed';
  lastUpdate: string;
}

interface ChannelFormData {
  operation: ChannelOperation;
  channelId?: string;
  counterparty?: string;
  depositAmount?: string;
  updateAmount?: string;
  duration?: number; // in blocks
  chainId: ChainId;
}

interface ChannelBuilderProps {
  onComplete: (data: ChannelFormData & { estimatedFee: string }) => void;
}

export function ChannelBuilder({ onComplete }: ChannelBuilderProps) {
  const { selectedAccount, selectedChain } = useWallet();
  const [estimatedFee, setEstimatedFee] = useState<string>('0');
  const [isCalculating, setIsCalculating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [channels, setChannels] = useState<PaymentChannel[]>([]);
  const [selectedChannel, setSelectedChannel] = useState<PaymentChannel | null>(null);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
    setValue,
  } = useForm<ChannelFormData>({
    mode: 'onChange',
    defaultValues: {
      operation: 'open',
      duration: 14400, // ~24 hours (assuming 6s block time)
      chainId: selectedChain,
    },
  });

  const watchedOperation = watch('operation');
  const watchedChannelId = watch('channelId');
  const watchedDepositAmount = watch('depositAmount');
  const watchedCounterparty = watch('counterparty');

  // Fetch user's payment channels
  useEffect(() => {
    if (!selectedAccount) return;

    const fetchChannels = async () => {
      try {
        // Simulated channels for demonstration
        const mockChannels: PaymentChannel[] = [
          {
            id: 'ch_1',
            counterparty: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
            balance: '50.5',
            status: 'open',
            lastUpdate: '2025-10-20',
          },
          {
            id: 'ch_2',
            counterparty: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
            balance: '25.75',
            status: 'open',
            lastUpdate: '2025-10-21',
          },
        ];
        setChannels(mockChannels);
      } catch (err) {
        console.error('Failed to fetch channels:', err);
      }
    };

    fetchChannels();
  }, [selectedAccount]);

  // Update selected channel
  useEffect(() => {
    if (watchedChannelId) {
      const channel = channels.find((c) => c.id === watchedChannelId);
      setSelectedChannel(channel || null);
    }
  }, [watchedChannelId, channels]);

  // Estimate transaction fee
  useEffect(() => {
    const calculateFee = async () => {
      setIsCalculating(true);
      try {
        const config = CHAINS[selectedChain];
        let baseFee = '0.01';

        if (watchedOperation === 'open') {
          baseFee = '0.05'; // Higher fee for opening channels
        } else if (watchedOperation === 'close') {
          baseFee = '0.03';
        } else if (watchedOperation === 'update') {
          baseFee = '0.002'; // Very low fee for updates
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
  }, [watchedOperation, selectedChain]);

  // Validate address
  const validateAddress = (address: string | undefined): boolean => {
    if (!address) return false;
    return address.length >= 47 && address.length <= 48;
  };

  // Validate amount
  const validateAmount = (amount: string | undefined): boolean => {
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

  const onSubmit = (data: ChannelFormData) => {
    setError(null);

    // Validation based on operation
    if (data.operation === 'open') {
      if (!validateAddress(data.counterparty)) {
        setError('Invalid counterparty address');
        return;
      }
      if (!validateAmount(data.depositAmount)) {
        setError('Invalid deposit amount or insufficient balance');
        return;
      }
      if (!data.duration || data.duration < 100) {
        setError('Channel duration must be at least 100 blocks');
        return;
      }
    }

    if (data.operation === 'close' && !data.channelId) {
      setError('Please select a channel to close');
      return;
    }

    if (data.operation === 'update') {
      if (!data.channelId) {
        setError('Please select a channel to update');
        return;
      }
      if (!validateAmount(data.updateAmount)) {
        setError('Invalid update amount');
        return;
      }
    }

    // Pass data to parent
    onComplete({
      ...data,
      estimatedFee,
    });
  };

  const chainConfig = CHAINS[selectedChain];

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      {/* Operation Selection */}
      <div className="space-y-3">
        <Label>Channel Operation</Label>
        <RadioGroup
          value={watchedOperation}
          onValueChange={(v) => setValue('operation', v as ChannelOperation)}
        >
          <div className="grid grid-cols-3 gap-3">
            <Card
              className={`cursor-pointer transition-all ${
                watchedOperation === 'open'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setValue('operation', 'open')}
            >
              <CardContent className="pt-6 pb-6 flex flex-col items-center gap-2">
                <RadioGroupItem value="open" id="open" className="sr-only" />
                <Plus className="w-8 h-8 text-green-500" />
                <Label htmlFor="open" className="cursor-pointer font-medium">
                  Open Channel
                </Label>
              </CardContent>
            </Card>

            <Card
              className={`cursor-pointer transition-all ${
                watchedOperation === 'close'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setValue('operation', 'close')}
            >
              <CardContent className="pt-6 pb-6 flex flex-col items-center gap-2">
                <RadioGroupItem value="close" id="close" className="sr-only" />
                <X className="w-8 h-8 text-red-500" />
                <Label htmlFor="close" className="cursor-pointer font-medium">
                  Close Channel
                </Label>
              </CardContent>
            </Card>

            <Card
              className={`cursor-pointer transition-all ${
                watchedOperation === 'update'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setValue('operation', 'update')}
            >
              <CardContent className="pt-6 pb-6 flex flex-col items-center gap-2">
                <RadioGroupItem value="update" id="update" className="sr-only" />
                <RefreshCw className="w-8 h-8 text-blue-500" />
                <Label htmlFor="update" className="cursor-pointer font-medium">
                  Update Channel
                </Label>
              </CardContent>
            </Card>
          </div>
        </RadioGroup>
      </div>

      {/* Open Channel Form */}
      {watchedOperation === 'open' && (
        <>
          <div className="space-y-2">
            <Label htmlFor="counterparty">Counterparty Address</Label>
            <Input
              id="counterparty"
              placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
              {...register('counterparty', {
                required: watchedOperation === 'open',
                validate: (value) => validateAddress(value) || 'Invalid address format',
              })}
              className={errors.counterparty ? 'border-destructive' : ''}
            />
            {errors.counterparty && (
              <p className="text-sm text-destructive">{errors.counterparty.message}</p>
            )}
          </div>

          <div className="space-y-2">
            <Label htmlFor="depositAmount">Initial Deposit</Label>
            <div className="relative">
              <Input
                id="depositAmount"
                type="number"
                step="any"
                placeholder="0.00"
                {...register('depositAmount', {
                  required: watchedOperation === 'open',
                  validate: (value) => validateAmount(value) || 'Invalid amount or insufficient balance',
                })}
                className={`pr-16 ${errors.depositAmount ? 'border-destructive' : ''}`}
              />
              <div className="absolute right-3 top-1/2 -translate-y-1/2">
                <Badge variant="secondary">{chainConfig.symbol}</Badge>
              </div>
            </div>
            {errors.depositAmount && (
              <p className="text-sm text-destructive">{errors.depositAmount.message}</p>
            )}
            {selectedAccount?.balance && (
              <p className="text-xs text-muted-foreground">
                Available: {selectedAccount.balance} {chainConfig.symbol}
              </p>
            )}
          </div>

          <div className="space-y-2">
            <Label htmlFor="duration">Channel Duration (blocks)</Label>
            <Input
              id="duration"
              type="number"
              placeholder="14400"
              {...register('duration', {
                required: watchedOperation === 'open',
                min: { value: 100, message: 'Minimum 100 blocks' },
              })}
              className={errors.duration ? 'border-destructive' : ''}
            />
            {errors.duration && (
              <p className="text-sm text-destructive">{errors.duration.message}</p>
            )}
            <p className="text-xs text-muted-foreground">
              Approx. {((watch('duration') || 0) * 6) / 3600} hours (6s block time)
            </p>
          </div>

          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription className="text-xs">
              Payment channels enable instant, low-fee transactions. Minimum duration: 100 blocks.
            </AlertDescription>
          </Alert>
        </>
      )}

      {/* Close/Update Channel Form */}
      {(watchedOperation === 'close' || watchedOperation === 'update') && (
        <>
          <div className="space-y-3">
            <Label>Select Channel</Label>
            {channels.length === 0 ? (
              <Alert>
                <Info className="h-4 w-4" />
                <AlertDescription>You don't have any open payment channels</AlertDescription>
              </Alert>
            ) : (
              <div className="space-y-2">
                {channels
                  .filter((ch) => ch.status === 'open')
                  .map((channel) => (
                    <Card
                      key={channel.id}
                      className={`cursor-pointer transition-all ${
                        watchedChannelId === channel.id
                          ? 'border-primary ring-2 ring-primary/20'
                          : 'hover:border-primary/50'
                      }`}
                      onClick={() => setValue('channelId', channel.id)}
                    >
                      <CardContent className="pt-4 pb-4">
                        <div className="flex items-center justify-between">
                          <div className="space-y-1">
                            <div className="flex items-center gap-2">
                              <Badge variant="secondary">{channel.id}</Badge>
                              <Badge
                                variant={channel.status === 'open' ? 'default' : 'outline'}
                                className="capitalize"
                              >
                                {channel.status}
                              </Badge>
                            </div>
                            <p className="text-xs text-muted-foreground font-mono">
                              {channel.counterparty.slice(0, 8)}...
                              {channel.counterparty.slice(-8)}
                            </p>
                          </div>
                          <div className="text-right">
                            <p className="text-sm font-medium">
                              {channel.balance} {chainConfig.symbol}
                            </p>
                            <p className="text-xs text-muted-foreground">
                              Updated: {channel.lastUpdate}
                            </p>
                          </div>
                        </div>
                      </CardContent>
                    </Card>
                  ))}
              </div>
            )}
          </div>

          {watchedOperation === 'update' && watchedChannelId && (
            <div className="space-y-2">
              <Label htmlFor="updateAmount">Update Amount</Label>
              <div className="relative">
                <Input
                  id="updateAmount"
                  type="number"
                  step="any"
                  placeholder="0.00"
                  {...register('updateAmount', {
                    required: watchedOperation === 'update',
                    validate: (value) => validateAmount(value) || 'Invalid amount',
                  })}
                  className={`pr-16 ${errors.updateAmount ? 'border-destructive' : ''}`}
                />
                <div className="absolute right-3 top-1/2 -translate-y-1/2">
                  <Badge variant="secondary">{chainConfig.symbol}</Badge>
                </div>
              </div>
              {errors.updateAmount && (
                <p className="text-sm text-destructive">{errors.updateAmount.message}</p>
              )}
              <Alert>
                <Info className="h-4 w-4" />
                <AlertDescription className="text-xs">
                  Channel updates are instant and have minimal fees
                </AlertDescription>
              </Alert>
            </div>
          )}

          {watchedOperation === 'close' && watchedChannelId && selectedChannel && (
            <Alert className="bg-orange-50 dark:bg-orange-950/20 border-orange-200 dark:border-orange-800">
              <AlertCircle className="h-4 w-4 text-orange-600" />
              <AlertDescription>
                Closing this channel will return {selectedChannel.balance} {chainConfig.symbol} to
                your account. This action cannot be undone.
              </AlertDescription>
            </Alert>
          )}
        </>
      )}

      {/* Active Channels Info */}
      {channels.length > 0 && watchedOperation === 'open' && (
        <Card className="bg-muted/50">
          <CardContent className="pt-4 pb-4">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Your Active Channels</span>
              <Badge variant="secondary">{channels.filter((c) => c.status === 'open').length}</Badge>
            </div>
          </CardContent>
        </Card>
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

            {watchedOperation === 'open' && watchedDepositAmount && (
              <div className="flex items-center justify-between text-sm pt-2 border-t">
                <span className="text-muted-foreground">Total (Deposit + Fee)</span>
                <span className="font-medium">
                  {(parseFloat(watchedDepositAmount || '0') + parseFloat(estimatedFee)).toFixed(6)}{' '}
                  {chainConfig.symbol}
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
        <Button type="submit" className="flex-1" disabled={!isValid || isCalculating}>
          Review {watchedOperation === 'open' ? 'Channel Opening' : watchedOperation === 'close' ? 'Channel Closing' : 'Channel Update'}
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </form>
  );
}
