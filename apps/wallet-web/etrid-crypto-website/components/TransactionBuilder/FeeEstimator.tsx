/**
 * FeeEstimator - Advanced Fee Estimation Component
 * Provides detailed fee estimation with priority options and gas optimization
 */

'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { CHAINS, ChainId } from '@/lib/polkadot/chains';
import {
  Zap,
  Clock,
  DollarSign,
  TrendingUp,
  TrendingDown,
  Info,
  Loader2,
  AlertCircle,
} from 'lucide-react';

export type FeePriority = 'low' | 'medium' | 'high' | 'custom';

interface FeeEstimate {
  priority: FeePriority;
  baseFee: string;
  priorityFee: string;
  totalFee: string;
  estimatedTime: string; // in seconds
  confidence: number; // 0-100
}

interface FeeEstimatorProps {
  chainId: ChainId;
  transactionType: string;
  transactionSize?: number; // in bytes
  onFeeSelected: (fee: FeeEstimate) => void;
  value?: FeePriority;
}

export function FeeEstimator({
  chainId,
  transactionType,
  transactionSize = 200,
  onFeeSelected,
  value = 'medium',
}: FeeEstimatorProps) {
  const [selectedPriority, setSelectedPriority] = useState<FeePriority>(value);
  const [feeEstimates, setFeeEstimates] = useState<FeeEstimate[]>([]);
  const [isCalculating, setIsCalculating] = useState(false);
  const [networkCongestion, setNetworkCongestion] = useState<'low' | 'medium' | 'high'>('medium');
  const [error, setError] = useState<string | null>(null);

  const chainConfig = CHAINS[chainId];

  // Calculate fee estimates
  useEffect(() => {
    const calculateFees = async () => {
      setIsCalculating(true);
      setError(null);

      try {
        // Simulate API delay
        await new Promise((resolve) => setTimeout(resolve, 800));

        // Base fee calculation based on chain and transaction type
        let baseFeeMultiplier = 1;
        switch (transactionType) {
          case 'transfer':
            baseFeeMultiplier = 1;
            break;
          case 'staking':
            baseFeeMultiplier = 1.5;
            break;
          case 'governance':
            baseFeeMultiplier = 0.8;
            break;
          case 'channel':
            baseFeeMultiplier = 2;
            break;
          case 'batch':
            baseFeeMultiplier = 1.2;
            break;
          default:
            baseFeeMultiplier = 1;
        }

        const baseAmount = chainConfig.isRelay ? 0.01 : 0.001;
        const baseFee = baseAmount * baseFeeMultiplier;

        // Size-based adjustment (per byte)
        const sizeFee = (transactionSize / 1000) * 0.0001;

        // Network congestion multiplier
        const congestionMultiplier = {
          low: 0.8,
          medium: 1.0,
          high: 1.5,
        }[networkCongestion];

        // Calculate priority-based estimates
        const estimates: FeeEstimate[] = [
          {
            priority: 'low',
            baseFee: baseFee.toFixed(6),
            priorityFee: (baseFee * 0.1 * congestionMultiplier).toFixed(6),
            totalFee: ((baseFee + sizeFee) * 0.9 * congestionMultiplier).toFixed(6),
            estimatedTime: '30-60',
            confidence: 75,
          },
          {
            priority: 'medium',
            baseFee: baseFee.toFixed(6),
            priorityFee: (baseFee * 0.3 * congestionMultiplier).toFixed(6),
            totalFee: ((baseFee + sizeFee) * 1.0 * congestionMultiplier).toFixed(6),
            estimatedTime: '10-20',
            confidence: 90,
          },
          {
            priority: 'high',
            baseFee: baseFee.toFixed(6),
            priorityFee: (baseFee * 0.6 * congestionMultiplier).toFixed(6),
            totalFee: ((baseFee + sizeFee) * 1.3 * congestionMultiplier).toFixed(6),
            estimatedTime: '3-6',
            confidence: 98,
          },
        ];

        setFeeEstimates(estimates);

        // Auto-select and notify parent of medium priority
        const selectedEstimate = estimates.find((e) => e.priority === selectedPriority);
        if (selectedEstimate) {
          onFeeSelected(selectedEstimate);
        }
      } catch (err) {
        console.error('Fee calculation error:', err);
        setError('Failed to calculate fees. Using default values.');
      } finally {
        setIsCalculating(false);
      }
    };

    calculateFees();
  }, [chainId, transactionType, transactionSize, networkCongestion]);

  // Update parent when priority changes
  useEffect(() => {
    const selectedEstimate = feeEstimates.find((e) => e.priority === selectedPriority);
    if (selectedEstimate) {
      onFeeSelected(selectedEstimate);
    }
  }, [selectedPriority, feeEstimates]);

  // Simulate network congestion updates
  useEffect(() => {
    const interval = setInterval(() => {
      const congestionLevels: Array<'low' | 'medium' | 'high'> = ['low', 'medium', 'high'];
      const random = Math.random();
      if (random < 0.3) {
        setNetworkCongestion('low');
      } else if (random < 0.8) {
        setNetworkCongestion('medium');
      } else {
        setNetworkCongestion('high');
      }
    }, 30000); // Update every 30 seconds

    return () => clearInterval(interval);
  }, []);

  const getPriorityIcon = (priority: FeePriority) => {
    switch (priority) {
      case 'low':
        return <Clock className="w-4 h-4" />;
      case 'medium':
        return <DollarSign className="w-4 h-4" />;
      case 'high':
        return <Zap className="w-4 h-4" />;
      default:
        return null;
    }
  };

  const getPriorityColor = (priority: FeePriority) => {
    switch (priority) {
      case 'low':
        return 'text-blue-600 dark:text-blue-400';
      case 'medium':
        return 'text-green-600 dark:text-green-400';
      case 'high':
        return 'text-orange-600 dark:text-orange-400';
      default:
        return '';
    }
  };

  const getCongestionBadge = () => {
    const variants = {
      low: { variant: 'default' as const, icon: <TrendingDown className="w-3 h-3" /> },
      medium: { variant: 'secondary' as const, icon: <DollarSign className="w-3 h-3" /> },
      high: { variant: 'destructive' as const, icon: <TrendingUp className="w-3 h-3" /> },
    };

    const config = variants[networkCongestion];

    return (
      <Badge variant={config.variant} className="gap-1">
        {config.icon}
        <span className="capitalize">{networkCongestion}</span> Congestion
      </Badge>
    );
  };

  if (isCalculating && feeEstimates.length === 0) {
    return (
      <Card>
        <CardContent className="pt-6">
          <div className="flex items-center justify-center gap-3 py-8">
            <Loader2 className="w-5 h-5 animate-spin text-primary" />
            <p className="text-muted-foreground">Calculating optimal fees...</p>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle className="text-base">Fee Estimation</CardTitle>
            <CardDescription className="text-xs">
              Choose your preferred transaction speed
            </CardDescription>
          </div>
          {getCongestionBadge()}
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {error && (
          <Alert variant="destructive">
            <AlertCircle className="h-4 w-4" />
            <AlertDescription className="text-xs">{error}</AlertDescription>
          </Alert>
        )}

        {/* Priority Selection */}
        <RadioGroup
          value={selectedPriority}
          onValueChange={(value) => setSelectedPriority(value as FeePriority)}
        >
          <div className="space-y-3">
            {feeEstimates.map((estimate) => (
              <Card
                key={estimate.priority}
                className={`cursor-pointer transition-all ${
                  selectedPriority === estimate.priority
                    ? 'border-primary ring-2 ring-primary/20'
                    : 'hover:border-primary/50'
                }`}
                onClick={() => setSelectedPriority(estimate.priority)}
              >
                <CardContent className="pt-4 pb-4">
                  <div className="flex items-center justify-between gap-4">
                    <div className="flex items-center gap-3 flex-1">
                      <RadioGroupItem
                        value={estimate.priority}
                        id={estimate.priority}
                        className="mt-0.5"
                      />
                      <div
                        className={`p-2 rounded-lg bg-primary/10 ${getPriorityColor(
                          estimate.priority
                        )}`}
                      >
                        {getPriorityIcon(estimate.priority)}
                      </div>
                      <div className="flex-1">
                        <Label
                          htmlFor={estimate.priority}
                          className="font-medium capitalize cursor-pointer"
                        >
                          {estimate.priority} Priority
                        </Label>
                        <p className="text-xs text-muted-foreground">
                          ~{estimate.estimatedTime}s confirmation
                        </p>
                      </div>
                    </div>
                    <div className="text-right">
                      <p className="font-mono font-medium">
                        {estimate.totalFee} {chainConfig.symbol}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        {estimate.confidence}% confidence
                      </p>
                    </div>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </RadioGroup>

        <Separator />

        {/* Fee Breakdown */}
        {feeEstimates.find((e) => e.priority === selectedPriority) && (
          <div className="space-y-2">
            <h4 className="text-sm font-medium">Fee Breakdown</h4>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Base Fee</span>
                <span className="font-mono">
                  {feeEstimates.find((e) => e.priority === selectedPriority)?.baseFee}{' '}
                  {chainConfig.symbol}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Priority Fee</span>
                <span className="font-mono">
                  {feeEstimates.find((e) => e.priority === selectedPriority)?.priorityFee}{' '}
                  {chainConfig.symbol}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Size ({transactionSize} bytes)</span>
                <span className="font-mono">
                  {((transactionSize / 1000) * 0.0001).toFixed(6)} {chainConfig.symbol}
                </span>
              </div>
              <Separator />
              <div className="flex justify-between font-medium">
                <span>Total Fee</span>
                <span className="font-mono">
                  {feeEstimates.find((e) => e.priority === selectedPriority)?.totalFee}{' '}
                  {chainConfig.symbol}
                </span>
              </div>
            </div>
          </div>
        )}

        {/* Gas Optimization Tips */}
        <Alert>
          <Info className="h-4 w-4" />
          <AlertDescription className="text-xs space-y-1">
            <p className="font-medium">Gas Optimization Tips:</p>
            <ul className="list-disc list-inside space-y-0.5 text-muted-foreground">
              <li>Low priority is best for non-urgent transactions</li>
              <li>Network congestion affects confirmation times</li>
              <li>Batch multiple transactions to save on fees</li>
            </ul>
          </AlertDescription>
        </Alert>

        {/* Historical Average (mock data) */}
        <div className="pt-2">
          <p className="text-xs text-muted-foreground">
            24h average: 0.008 {chainConfig.symbol} • Current block: #1,234,567
          </p>
        </div>
      </CardContent>
    </Card>
  );
}
