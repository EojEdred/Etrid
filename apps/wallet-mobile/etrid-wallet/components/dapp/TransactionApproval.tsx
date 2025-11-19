/**
 * TransactionApproval Component
 * Modal for approving/rejecting transaction requests from dApps
 */

'use client';

import React, { useState } from 'react';
import { TransactionRequest, TransactionSimulation } from '@/types/dapp';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import {
  AlertTriangle,
  CheckCircle2,
  ChevronDown,
  XCircle,
  Zap,
  ArrowDownUp,
} from 'lucide-react';

interface TransactionApprovalProps {
  transaction: TransactionRequest | null;
  simulation?: TransactionSimulation;
  dAppName: string;
  dAppIcon?: string;
  open: boolean;
  onApprove: () => void;
  onReject: () => void;
  onClose: () => void;
}

export function TransactionApproval({
  transaction,
  simulation,
  dAppName,
  dAppIcon,
  open,
  onApprove,
  onReject,
  onClose,
}: TransactionApprovalProps) {
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  if (!transaction) return null;

  const handleApprove = async () => {
    setIsLoading(true);
    try {
      await onApprove();
    } finally {
      setIsLoading(false);
    }
  };

  const handleReject = async () => {
    setIsLoading(true);
    try {
      await onReject();
    } finally {
      setIsLoading(false);
    }
  };

  const totalCost = simulation
    ? (
        parseFloat(transaction.value || '0') +
        parseFloat(simulation.gasEstimate || '0')
      ).toString()
    : transaction.value || '0';

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Confirm Transaction</DialogTitle>
          <DialogDescription>
            Review transaction details before signing
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          {/* dApp Info */}
          <div className="flex items-center gap-3 p-3 rounded-lg bg-muted">
            {dAppIcon && (
              <img
                src={dAppIcon}
                alt={dAppName}
                className="w-10 h-10 rounded-lg"
              />
            )}
            <div className="flex-1">
              <h3 className="font-semibold">{dAppName}</h3>
              <p className="text-xs text-muted-foreground">
                Requesting transaction
              </p>
            </div>
          </div>

          {/* Transaction Details */}
          <div className="space-y-3">
            {/* To Address */}
            <div>
              <p className="text-xs text-muted-foreground mb-1">To:</p>
              <p className="text-sm font-mono break-all">{transaction.to}</p>
            </div>

            {/* Amount */}
            {transaction.value && parseFloat(transaction.value) > 0 && (
              <div>
                <p className="text-xs text-muted-foreground mb-1">Amount:</p>
                <p className="text-2xl font-bold">
                  {formatValue(transaction.value)} ETH
                </p>
              </div>
            )}

            <Separator />

            {/* Gas Estimate */}
            {simulation && (
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Zap className="w-4 h-4 text-muted-foreground" />
                  <span className="text-sm">Estimated Gas:</span>
                </div>
                <span className="text-sm font-medium">
                  {formatValue(simulation.gasEstimate)} ETH
                </span>
              </div>
            )}

            {/* Total Cost */}
            <div className="flex items-center justify-between p-3 rounded-lg bg-muted">
              <span className="font-semibold">Total Cost:</span>
              <span className="text-lg font-bold">{formatValue(totalCost)} ETH</span>
            </div>

            {/* Balance Changes (from simulation) */}
            {simulation && simulation.balanceChanges.length > 0 && (
              <div>
                <p className="text-xs font-medium text-muted-foreground mb-2">
                  Estimated Changes:
                </p>
                <div className="space-y-1">
                  {simulation.balanceChanges.map((change, idx) => (
                    <div
                      key={idx}
                      className="flex items-center justify-between text-sm p-2 rounded bg-muted/50"
                    >
                      <div className="flex items-center gap-2">
                        <ArrowDownUp
                          className={`w-4 h-4 ${
                            change.direction === 'in'
                              ? 'text-green-500'
                              : 'text-red-500'
                          }`}
                        />
                        <span>{change.asset}</span>
                      </div>
                      <span
                        className={
                          change.direction === 'in'
                            ? 'text-green-600'
                            : 'text-red-600'
                        }
                      >
                        {change.direction === 'in' ? '+' : '-'}
                        {change.amount}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Warnings */}
          {simulation && simulation.warnings.length > 0 && (
            <div className="space-y-2">
              {simulation.warnings.map((warning, idx) => (
                <div
                  key={idx}
                  className="flex items-start gap-2 p-3 rounded-lg bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800"
                >
                  <AlertTriangle className="w-4 h-4 text-yellow-600 dark:text-yellow-400 mt-0.5 flex-shrink-0" />
                  <p className="text-xs text-yellow-700 dark:text-yellow-300">
                    {warning}
                  </p>
                </div>
              ))}
            </div>
          )}

          {/* Advanced Details */}
          <Collapsible open={showAdvanced} onOpenChange={setShowAdvanced}>
            <CollapsibleTrigger asChild>
              <Button
                variant="ghost"
                className="w-full flex items-center justify-between p-0 h-auto hover:bg-transparent"
              >
                <span className="text-sm text-muted-foreground">
                  Advanced Details
                </span>
                <ChevronDown
                  className={`w-4 h-4 transition-transform ${
                    showAdvanced ? 'rotate-180' : ''
                  }`}
                />
              </Button>
            </CollapsibleTrigger>
            <CollapsibleContent className="mt-2 space-y-2">
              {transaction.data && (
                <div>
                  <p className="text-xs text-muted-foreground mb-1">Data:</p>
                  <pre className="text-xs font-mono bg-muted p-2 rounded overflow-x-auto max-h-24">
                    {transaction.data}
                  </pre>
                </div>
              )}
              {transaction.gas && (
                <div>
                  <p className="text-xs text-muted-foreground mb-1">Gas Limit:</p>
                  <p className="text-sm font-mono">{transaction.gas}</p>
                </div>
              )}
              {transaction.gasPrice && (
                <div>
                  <p className="text-xs text-muted-foreground mb-1">Gas Price:</p>
                  <p className="text-sm font-mono">{transaction.gasPrice}</p>
                </div>
              )}
              {transaction.nonce !== undefined && (
                <div>
                  <p className="text-xs text-muted-foreground mb-1">Nonce:</p>
                  <p className="text-sm font-mono">{transaction.nonce}</p>
                </div>
              )}
            </CollapsibleContent>
          </Collapsible>
        </div>

        <DialogFooter className="gap-2 sm:gap-0">
          <Button
            variant="outline"
            onClick={handleReject}
            disabled={isLoading}
            className="flex-1"
          >
            <XCircle className="w-4 h-4 mr-2" />
            Reject
          </Button>
          <Button onClick={handleApprove} disabled={isLoading} className="flex-1">
            <CheckCircle2 className="w-4 h-4 mr-2" />
            {isLoading ? 'Signing...' : 'Sign'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function formatValue(value: string): string {
  const num = parseFloat(value);
  if (isNaN(num)) return '0';
  return num.toFixed(6);
}
