/**
 * TransactionSimulator - Transaction Simulation Component
 * Simulates transaction execution and shows expected state changes
 */

'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS } from '@/lib/polkadot/chains';
import {
  Play,
  CheckCircle2,
  AlertCircle,
  Info,
  TrendingDown,
  TrendingUp,
  Loader2,
  RefreshCw,
} from 'lucide-react';
import type { TransactionData } from './TransactionBuilder';

interface SimulationResult {
  success: boolean;
  balanceChanges: {
    sender: {
      before: string;
      after: string;
      change: string;
    };
    recipient?: {
      before: string;
      after: string;
      change: string;
    };
  };
  stateChanges?: {
    key: string;
    before: any;
    after: any;
  }[];
  events?: {
    name: string;
    data: any;
  }[];
  gasUsed?: string;
  warnings?: string[];
  errors?: string[];
}

interface TransactionSimulatorProps {
  transaction: TransactionData;
  onClose?: () => void;
}

export function TransactionSimulator({ transaction, onClose }: TransactionSimulatorProps) {
  const [isSimulating, setIsSimulating] = useState(false);
  const [result, setResult] = useState<SimulationResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const { selectedAccount } = useWallet();
  const chainConfig = CHAINS[transaction.chainId];

  // Simulate transaction
  const simulateTransaction = async () => {
    setIsSimulating(true);
    setError(null);
    setResult(null);

    try {
      // Simulate delay for realistic feel
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // Mock simulation based on transaction type
      let simulationResult: SimulationResult;

      switch (transaction.type) {
        case 'transfer':
          simulationResult = simulateTransfer();
          break;
        case 'staking':
          simulationResult = simulateStaking();
          break;
        case 'governance':
          simulationResult = simulateGovernance();
          break;
        case 'channel':
          simulationResult = simulateChannel();
          break;
        default:
          throw new Error('Unknown transaction type');
      }

      setResult(simulationResult);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Simulation failed';
      setError(message);
    } finally {
      setIsSimulating(false);
    }
  };

  // Simulate transfer transaction
  const simulateTransfer = (): SimulationResult => {
    const { amount, recipient } = transaction.data;
    const senderBalance = parseFloat(selectedAccount?.balance || '0');
    const amountNum = parseFloat(amount);
    const fee = parseFloat(transaction.data.estimatedFee);

    const warnings: string[] = [];
    const errors: string[] = [];

    // Check for potential issues
    if (senderBalance - (amountNum + fee) < 1) {
      warnings.push('Low balance after transaction. Consider keeping a larger reserve.');
    }

    if (amountNum > senderBalance * 0.9) {
      warnings.push('Transferring more than 90% of your balance.');
    }

    // Check for errors
    if (amountNum + fee > senderBalance) {
      errors.push('Insufficient balance to complete transaction');
    }

    return {
      success: errors.length === 0,
      balanceChanges: {
        sender: {
          before: senderBalance.toFixed(6),
          after: (senderBalance - amountNum - fee).toFixed(6),
          change: `${(-(amountNum + fee)).toFixed(6)}`,
        },
        recipient: {
          before: '0.000000', // Simulated
          after: amountNum.toFixed(6),
          change: `+${amountNum.toFixed(6)}`,
        },
      },
      events: [
        {
          name: 'Transfer',
          data: {
            from: selectedAccount?.address.slice(0, 10) + '...',
            to: recipient.slice(0, 10) + '...',
            amount: amountNum.toFixed(6),
          },
        },
      ],
      gasUsed: fee.toFixed(6),
      warnings,
      errors,
    };
  };

  // Simulate staking transaction
  const simulateStaking = (): SimulationResult => {
    const { operation, amount, validator } = transaction.data;
    const senderBalance = parseFloat(selectedAccount?.balance || '0');
    const amountNum = amount ? parseFloat(amount) : 0;
    const fee = parseFloat(transaction.data.estimatedFee);

    const warnings: string[] = [];
    const errors: string[] = [];
    const stateChanges: any[] = [];

    if (operation === 'stake') {
      if (amountNum < 1) {
        errors.push('Minimum stake amount is 1 ETR');
      }
      stateChanges.push({
        key: 'Staking.Bonded',
        before: '0',
        after: amountNum.toFixed(6),
      });
      warnings.push('Staked tokens will be locked for 28 days unbonding period');
    } else if (operation === 'unstake') {
      warnings.push('Unstaking will begin a 28-day unbonding period');
      stateChanges.push({
        key: 'Staking.Unbonding',
        before: '0',
        after: amountNum.toFixed(6),
      });
    }

    return {
      success: errors.length === 0,
      balanceChanges: {
        sender: {
          before: senderBalance.toFixed(6),
          after: (senderBalance - fee).toFixed(6),
          change: `${(-fee).toFixed(6)}`,
        },
      },
      stateChanges,
      events: [
        {
          name: operation === 'stake' ? 'Bonded' : 'Unbonded',
          data: {
            staker: selectedAccount?.address.slice(0, 10) + '...',
            amount: amountNum.toFixed(6),
          },
        },
      ],
      gasUsed: fee.toFixed(6),
      warnings,
      errors,
    };
  };

  // Simulate governance transaction
  const simulateGovernance = (): SimulationResult => {
    const { action, proposalId, voteType } = transaction.data;
    const senderBalance = parseFloat(selectedAccount?.balance || '0');
    const fee = parseFloat(transaction.data.estimatedFee);

    const warnings: string[] = [];

    if (action === 'vote' && transaction.data.conviction > 1) {
      const lockDays = [0, 7, 14, 28, 56, 112][transaction.data.conviction - 1];
      warnings.push(`Tokens will be locked for ${lockDays} days due to conviction voting`);
    }

    return {
      success: true,
      balanceChanges: {
        sender: {
          before: senderBalance.toFixed(6),
          after: (senderBalance - fee).toFixed(6),
          change: `${(-fee).toFixed(6)}`,
        },
      },
      events: [
        {
          name: action === 'vote' ? 'Voted' : action === 'propose' ? 'Proposed' : 'Delegated',
          data: {
            account: selectedAccount?.address.slice(0, 10) + '...',
            proposalId: proposalId || 'N/A',
            vote: voteType || 'N/A',
          },
        },
      ],
      gasUsed: fee.toFixed(6),
      warnings,
      errors: [],
    };
  };

  // Simulate channel transaction
  const simulateChannel = (): SimulationResult => {
    const { operation, depositAmount } = transaction.data;
    const senderBalance = parseFloat(selectedAccount?.balance || '0');
    const amountNum = depositAmount ? parseFloat(depositAmount) : 0;
    const fee = parseFloat(transaction.data.estimatedFee);

    const warnings: string[] = [];

    if (operation === 'open') {
      warnings.push('Channel deposits are locked until channel is closed');
    }

    return {
      success: true,
      balanceChanges: {
        sender: {
          before: senderBalance.toFixed(6),
          after: (senderBalance - amountNum - fee).toFixed(6),
          change: `${(-(amountNum + fee)).toFixed(6)}`,
        },
      },
      stateChanges: [
        {
          key: 'PaymentChannels.Channels',
          before: 'null',
          after: { status: 'open', balance: amountNum.toFixed(6) },
        },
      ],
      events: [
        {
          name: operation === 'open' ? 'ChannelOpened' : 'ChannelClosed',
          data: {
            account: selectedAccount?.address.slice(0, 10) + '...',
            amount: amountNum.toFixed(6),
          },
        },
      ],
      gasUsed: fee.toFixed(6),
      warnings,
      errors: [],
    };
  };

  // Auto-simulate on mount
  useEffect(() => {
    simulateTransaction();
  }, []);

  return (
    <div className="space-y-6">
      {/* Header */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="flex items-center gap-2">
                <Play className="w-5 h-5" />
                Transaction Simulation
              </CardTitle>
              <CardDescription>
                Preview the expected outcome before submitting
              </CardDescription>
            </div>
            {result && (
              <Button
                variant="outline"
                size="sm"
                onClick={simulateTransaction}
                disabled={isSimulating}
              >
                <RefreshCw className={`w-4 h-4 mr-2 ${isSimulating ? 'animate-spin' : ''}`} />
                Re-simulate
              </Button>
            )}
          </div>
        </CardHeader>
      </Card>

      {/* Loading State */}
      {isSimulating && (
        <Card className="border-primary bg-primary/5">
          <CardContent className="pt-6">
            <div className="flex items-center gap-3">
              <Loader2 className="w-5 h-5 animate-spin text-primary" />
              <div>
                <p className="font-medium">Simulating transaction...</p>
                <p className="text-sm text-muted-foreground">
                  Checking balances, state changes, and potential issues
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Error State */}
      {error && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertTitle>Simulation Error</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Simulation Result */}
      {result && !isSimulating && (
        <>
          {/* Success/Failure Status */}
          <Alert
            variant={result.success ? 'default' : 'destructive'}
            className={
              result.success
                ? 'border-green-500 bg-green-50 dark:bg-green-950/20'
                : ''
            }
          >
            {result.success ? (
              <CheckCircle2 className="h-5 w-5 text-green-600" />
            ) : (
              <AlertCircle className="h-5 w-5" />
            )}
            <AlertTitle>
              {result.success ? 'Simulation Successful' : 'Simulation Failed'}
            </AlertTitle>
            <AlertDescription>
              {result.success
                ? 'Transaction is expected to execute successfully'
                : 'Transaction will fail if submitted'}
            </AlertDescription>
          </Alert>

          {/* Errors */}
          {result.errors && result.errors.length > 0 && (
            <Alert variant="destructive">
              <AlertCircle className="h-4 w-4" />
              <AlertTitle>Errors Detected</AlertTitle>
              <AlertDescription>
                <ul className="list-disc list-inside space-y-1">
                  {result.errors.map((err, idx) => (
                    <li key={idx}>{err}</li>
                  ))}
                </ul>
              </AlertDescription>
            </Alert>
          )}

          {/* Warnings */}
          {result.warnings && result.warnings.length > 0 && (
            <Alert>
              <Info className="h-4 w-4" />
              <AlertTitle>Warnings</AlertTitle>
              <AlertDescription>
                <ul className="list-disc list-inside space-y-1">
                  {result.warnings.map((warning, idx) => (
                    <li key={idx}>{warning}</li>
                  ))}
                </ul>
              </AlertDescription>
            </Alert>
          )}

          {/* Balance Changes */}
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Expected Balance Changes</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {/* Sender Balance */}
              <div className="space-y-2">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">Your Account</span>
                  <Badge variant="secondary">Sender</Badge>
                </div>
                <div className="grid grid-cols-3 gap-4 text-sm">
                  <div>
                    <p className="text-muted-foreground">Before</p>
                    <p className="font-mono font-medium">
                      {result.balanceChanges.sender.before} {chainConfig.symbol}
                    </p>
                  </div>
                  <div className="flex items-center justify-center">
                    <TrendingDown className="w-4 h-4 text-destructive" />
                  </div>
                  <div>
                    <p className="text-muted-foreground">After</p>
                    <p className="font-mono font-medium">
                      {result.balanceChanges.sender.after} {chainConfig.symbol}
                    </p>
                  </div>
                </div>
                <div className="text-sm">
                  <span className="text-destructive font-medium">
                    {result.balanceChanges.sender.change} {chainConfig.symbol}
                  </span>
                </div>
              </div>

              {/* Recipient Balance (if applicable) */}
              {result.balanceChanges.recipient && (
                <>
                  <Separator />
                  <div className="space-y-2">
                    <div className="flex items-center justify-between text-sm">
                      <span className="text-muted-foreground">Recipient Account</span>
                      <Badge variant="secondary">Receiver</Badge>
                    </div>
                    <div className="grid grid-cols-3 gap-4 text-sm">
                      <div>
                        <p className="text-muted-foreground">Before</p>
                        <p className="font-mono font-medium">
                          {result.balanceChanges.recipient.before} {chainConfig.symbol}
                        </p>
                      </div>
                      <div className="flex items-center justify-center">
                        <TrendingUp className="w-4 h-4 text-green-600" />
                      </div>
                      <div>
                        <p className="text-muted-foreground">After</p>
                        <p className="font-mono font-medium">
                          {result.balanceChanges.recipient.after} {chainConfig.symbol}
                        </p>
                      </div>
                    </div>
                    <div className="text-sm">
                      <span className="text-green-600 font-medium">
                        {result.balanceChanges.recipient.change} {chainConfig.symbol}
                      </span>
                    </div>
                  </div>
                </>
              )}
            </CardContent>
          </Card>

          {/* State Changes */}
          {result.stateChanges && result.stateChanges.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base">State Changes</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {result.stateChanges.map((change, idx) => (
                    <div key={idx} className="space-y-1">
                      <p className="text-sm font-medium font-mono">{change.key}</p>
                      <div className="grid grid-cols-2 gap-4 text-xs">
                        <div className="p-2 bg-muted rounded">
                          <p className="text-muted-foreground mb-1">Before</p>
                          <p className="font-mono">{JSON.stringify(change.before)}</p>
                        </div>
                        <div className="p-2 bg-muted rounded">
                          <p className="text-muted-foreground mb-1">After</p>
                          <p className="font-mono">{JSON.stringify(change.after)}</p>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* Expected Events */}
          {result.events && result.events.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base">Expected Events</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  {result.events.map((event, idx) => (
                    <div
                      key={idx}
                      className="flex items-start gap-3 p-3 bg-muted rounded-lg"
                    >
                      <CheckCircle2 className="w-4 h-4 text-green-600 mt-0.5" />
                      <div className="flex-1">
                        <p className="font-medium text-sm">{event.name}</p>
                        <pre className="text-xs text-muted-foreground mt-1 font-mono">
                          {JSON.stringify(event.data, null, 2)}
                        </pre>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {/* Gas/Fee Info */}
          {result.gasUsed && (
            <Card className="bg-muted/50">
              <CardContent className="pt-6">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Estimated Gas/Fee</span>
                  <span className="font-mono font-medium">
                    {result.gasUsed} {chainConfig.symbol}
                  </span>
                </div>
              </CardContent>
            </Card>
          )}
        </>
      )}

      {/* Action Buttons */}
      {onClose && (
        <div className="flex gap-3">
          <Button variant="outline" onClick={onClose} className="flex-1">
            Close
          </Button>
          {result?.success && (
            <Button className="flex-1">
              Continue to Sign
            </Button>
          )}
        </div>
      )}
    </div>
  );
}
