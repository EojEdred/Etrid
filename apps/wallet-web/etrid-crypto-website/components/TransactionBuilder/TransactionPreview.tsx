/**
 * TransactionPreview - Enhanced Transaction Preview Component
 * Provides detailed preview before signing with warnings and estimations
 */

'use client';

import React from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Separator } from '@/components/ui/separator';
import { Button } from '@/components/ui/button';
import { CHAINS, ChainId } from '@/lib/polkadot/chains';
import {
  Eye,
  AlertTriangle,
  Info,
  Clock,
  DollarSign,
  Send,
  Lock,
  Vote,
  Layers,
  CheckCircle2,
  TrendingUp,
} from 'lucide-react';

export interface TransactionPreviewProps {
  transaction: {
    type: 'transfer' | 'staking' | 'governance' | 'channel' | 'batch';
    chainId: ChainId;
    data: any;
  };
  estimatedFee?: string;
  estimatedTime?: string;
  onConfirm: () => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function TransactionPreview({
  transaction,
  estimatedFee = '0.01',
  estimatedTime = '10-20s',
  onConfirm,
  onCancel,
  isLoading = false,
}: TransactionPreviewProps) {
  const chainConfig = CHAINS[transaction.chainId];

  // Get transaction icon
  const getTransactionIcon = () => {
    switch (transaction.type) {
      case 'transfer':
        return <Send className="w-5 h-5" />;
      case 'staking':
        return <Lock className="w-5 h-5" />;
      case 'governance':
        return <Vote className="w-5 h-5" />;
      case 'channel':
        return <Layers className="w-5 h-5" />;
      case 'batch':
        return <Send className="w-5 h-5" />;
      default:
        return <Eye className="w-5 h-5" />;
    }
  };

  // Generate warnings based on transaction type and data
  const getWarnings = (): string[] => {
    const warnings: string[] = [];

    if (transaction.type === 'transfer' || transaction.type === 'batch') {
      const amount = parseFloat(transaction.data.amount || transaction.data.totalAmount || '0');
      if (amount > 100) {
        warnings.push('Large transfer amount - please verify the recipient address carefully');
      }
    }

    if (transaction.type === 'staking') {
      if (transaction.data.operation === 'stake') {
        warnings.push('Staked tokens will be locked for 28 days unbonding period');
      }
      if (transaction.data.operation === 'unstake') {
        warnings.push(
          'Unstaking will begin a 28-day unbonding period before tokens become transferable'
        );
      }
    }

    if (transaction.type === 'governance') {
      if (transaction.data.action === 'vote' && transaction.data.conviction > 1) {
        warnings.push(
          `Tokens will be locked for ${transaction.data.conviction * 7} days due to conviction voting`
        );
      }
    }

    if (transaction.type === 'channel') {
      if (transaction.data.operation === 'close') {
        warnings.push('Closing a payment channel is irreversible');
      }
    }

    return warnings;
  };

  // Render transaction-specific details
  const renderTransactionDetails = () => {
    switch (transaction.type) {
      case 'transfer':
        return (
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Recipient</span>
              <code className="text-sm font-mono">
                {transaction.data.recipient?.slice(0, 10)}...
                {transaction.data.recipient?.slice(-8)}
              </code>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Amount</span>
              <span className="font-medium">
                {transaction.data.amount} {chainConfig.symbol}
              </span>
            </div>
            {transaction.data.memo && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Memo</span>
                <span className="text-sm">{transaction.data.memo}</span>
              </div>
            )}
          </div>
        );

      case 'batch':
        return (
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Total Recipients</span>
              <Badge variant="secondary">{transaction.data.transactions?.length || 0}</Badge>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Total Amount</span>
              <span className="font-medium">
                {transaction.data.totalAmount} {chainConfig.symbol}
              </span>
            </div>
            <Separator />
            <div className="space-y-2">
              <p className="text-sm font-medium">Recipients:</p>
              <div className="space-y-1 max-h-32 overflow-y-auto">
                {transaction.data.transactions?.map((tx: any, index: number) => (
                  <div key={index} className="text-xs flex justify-between bg-muted/30 p-2 rounded">
                    <code className="font-mono">
                      {tx.recipient.slice(0, 8)}...{tx.recipient.slice(-6)}
                    </code>
                    <span>
                      {tx.amount} {chainConfig.symbol}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        );

      case 'staking':
        return (
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Operation</span>
              <Badge variant="secondary" className="capitalize">
                {transaction.data.operation}
              </Badge>
            </div>
            {transaction.data.validator && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Validator</span>
                <code className="text-sm font-mono">
                  {transaction.data.validator.slice(0, 10)}...
                  {transaction.data.validator.slice(-8)}
                </code>
              </div>
            )}
            {transaction.data.amount && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Amount</span>
                <span className="font-medium">
                  {transaction.data.amount} {chainConfig.symbol}
                </span>
              </div>
            )}
            {transaction.data.apy && (
              <div className="flex justify-between items-center">
                <span className="text-muted-foreground">Estimated APY</span>
                <div className="flex items-center gap-1 text-green-600">
                  <TrendingUp className="w-3 h-3" />
                  <span className="font-medium">{transaction.data.apy}%</span>
                </div>
              </div>
            )}
          </div>
        );

      case 'governance':
        return (
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Action</span>
              <Badge variant="secondary" className="capitalize">
                {transaction.data.action}
              </Badge>
            </div>
            {transaction.data.proposalId && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Proposal ID</span>
                <span className="font-mono">#{transaction.data.proposalId}</span>
              </div>
            )}
            {transaction.data.voteType && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Vote Type</span>
                <Badge
                  variant={
                    transaction.data.voteType === 'aye'
                      ? 'default'
                      : transaction.data.voteType === 'nay'
                      ? 'destructive'
                      : 'secondary'
                  }
                  className="capitalize"
                >
                  {transaction.data.voteType}
                </Badge>
              </div>
            )}
            {transaction.data.amount && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Vote Amount</span>
                <span className="font-medium">
                  {transaction.data.amount} {chainConfig.symbol}
                </span>
              </div>
            )}
            {transaction.data.conviction && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Conviction</span>
                <span className="font-medium">{transaction.data.conviction}x</span>
              </div>
            )}
          </div>
        );

      case 'channel':
        return (
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Operation</span>
              <Badge variant="secondary" className="capitalize">
                {transaction.data.operation}
              </Badge>
            </div>
            {transaction.data.counterparty && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Counterparty</span>
                <code className="text-sm font-mono">
                  {transaction.data.counterparty.slice(0, 10)}...
                  {transaction.data.counterparty.slice(-8)}
                </code>
              </div>
            )}
            {transaction.data.amount && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Deposit</span>
                <span className="font-medium">
                  {transaction.data.amount} {chainConfig.symbol}
                </span>
              </div>
            )}
            {transaction.data.duration && (
              <div className="flex justify-between">
                <span className="text-muted-foreground">Duration</span>
                <span className="font-medium">{transaction.data.duration} blocks</span>
              </div>
            )}
          </div>
        );

      default:
        return <div className="text-sm text-muted-foreground">No details available</div>;
    }
  };

  const warnings = getWarnings();

  return (
    <div className="space-y-4">
      {/* Header */}
      <Card>
        <CardHeader>
          <div className="flex items-center gap-3">
            <div className="p-3 rounded-lg bg-primary/10 text-primary">
              {getTransactionIcon()}
            </div>
            <div className="flex-1">
              <CardTitle className="capitalize">{transaction.type} Transaction</CardTitle>
              <CardDescription>Review the details before confirming</CardDescription>
            </div>
            <Badge variant="outline" className="gap-1">
              <div
                className="w-2 h-2 rounded-full"
                style={{ backgroundColor: chainConfig.color }}
              />
              {chainConfig.name}
            </Badge>
          </div>
        </CardHeader>
      </Card>

      {/* Transaction Details */}
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Transaction Details</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {renderTransactionDetails()}

          <Separator />

          {/* Fee & Time Estimation */}
          <div className="space-y-3">
            <div className="flex justify-between items-center">
              <div className="flex items-center gap-2 text-muted-foreground">
                <DollarSign className="w-4 h-4" />
                <span>Estimated Fee</span>
              </div>
              <Badge variant="secondary" className="font-mono">
                {estimatedFee} {chainConfig.symbol}
              </Badge>
            </div>
            <div className="flex justify-between items-center">
              <div className="flex items-center gap-2 text-muted-foreground">
                <Clock className="w-4 h-4" />
                <span>Estimated Time</span>
              </div>
              <span className="text-sm font-medium">{estimatedTime}</span>
            </div>
          </div>

          <Separator />

          {/* Total Cost */}
          <div className="flex justify-between items-center text-lg font-semibold">
            <span>Total Cost</span>
            <span>
              {transaction.type === 'transfer' || transaction.type === 'batch'
                ? (
                    parseFloat(transaction.data.amount || transaction.data.totalAmount || '0') +
                    parseFloat(estimatedFee)
                  ).toFixed(6)
                : estimatedFee}{' '}
              {chainConfig.symbol}
            </span>
          </div>
        </CardContent>
      </Card>

      {/* Warnings */}
      {warnings.length > 0 && (
        <Alert variant="destructive">
          <AlertTriangle className="h-4 w-4" />
          <AlertDescription className="space-y-2">
            <p className="font-medium">Important Warnings:</p>
            <ul className="list-disc list-inside space-y-1 text-xs">
              {warnings.map((warning, index) => (
                <li key={index}>{warning}</li>
              ))}
            </ul>
          </AlertDescription>
        </Alert>
      )}

      {/* Info */}
      <Alert>
        <Info className="h-4 w-4" />
        <AlertDescription className="text-xs">
          <p className="font-medium mb-1">Before you confirm:</p>
          <ul className="list-disc list-inside space-y-0.5">
            <li>Double-check all transaction details</li>
            <li>Ensure you have sufficient balance for fees</li>
            <li>Transaction cannot be reversed once confirmed</li>
            <li>You will need to sign with your wallet</li>
          </ul>
        </AlertDescription>
      </Alert>

      {/* Action Buttons */}
      <div className="flex gap-3">
        <Button variant="outline" className="flex-1" onClick={onCancel} disabled={isLoading}>
          Cancel
        </Button>
        <Button className="flex-1 gap-2" onClick={onConfirm} disabled={isLoading}>
          {isLoading ? (
            <>
              <div className="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin" />
              Processing...
            </>
          ) : (
            <>
              <CheckCircle2 className="w-4 h-4" />
              Confirm & Sign
            </>
          )}
        </Button>
      </div>
    </div>
  );
}
