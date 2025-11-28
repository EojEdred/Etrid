/**
 * TransactionReview - Transaction Review and Submission Component
 * Displays transaction details and handles signing/submission
 */

'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Progress } from '@/components/ui/progress';
import { useWallet } from '@/lib/polkadot/useWallet';
import { CHAINS } from '@/lib/polkadot/chains';
import { createApi } from '@/lib/polkadot/api';
import { useToast } from '@/hooks/use-toast';
import {
  AlertCircle,
  CheckCircle2,
  Loader2,
  Send,
  Lock,
  Vote,
  Layers,
  ExternalLink,
  Copy,
  Clock,
  Info,
} from 'lucide-react';
import type { TransactionData } from './TransactionBuilder';

interface TransactionReviewProps {
  transaction: TransactionData;
  onSubmit: (txHash: { hash: string }) => void;
  onCancel: () => void;
  isSubmitting?: boolean;
  txHash?: string | null;
}

type TransactionStatus =
  | 'ready'
  | 'signing'
  | 'broadcasting'
  | 'in-block'
  | 'finalized'
  | 'failed';

export function TransactionReview({
  transaction,
  onSubmit,
  onCancel,
  isSubmitting: externalIsSubmitting,
  txHash: externalTxHash,
}: TransactionReviewProps) {
  const [status, setStatus] = useState<TransactionStatus>('ready');
  const [txHash, setTxHash] = useState<string | null>(externalTxHash || null);
  const [error, setError] = useState<string | null>(null);
  const [blockNumber, setBlockNumber] = useState<number | null>(null);
  const [progress, setProgress] = useState(0);

  const { selectedAccount } = useWallet();
  const { toast } = useToast();

  const chainConfig = CHAINS[transaction.chainId];

  // Dynamic import for Polkadot extension
  const [web3FromAddress, setWeb3FromAddress] = useState<any>(null);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      import('@polkadot/extension-dapp').then((module) => {
        setWeb3FromAddress(() => module.web3FromAddress);
      });
    }
  }, []);

  // Update progress based on status
  useEffect(() => {
    const progressMap: Record<TransactionStatus, number> = {
      ready: 0,
      signing: 20,
      broadcasting: 40,
      'in-block': 70,
      finalized: 100,
      failed: 0,
    };
    setProgress(progressMap[status]);
  }, [status]);

  // Handle transaction submission
  const handleSubmit = async () => {
    if (!selectedAccount || !web3FromAddress) {
      setError('Wallet not connected or extension not available');
      return;
    }

    try {
      setStatus('signing');
      setError(null);

      // Get API instance
      const api = await createApi(transaction.chainId);

      // Get injector for signing
      const injector = await web3FromAddress(selectedAccount.address);

      // Build the transaction based on type
      let tx: any;

      switch (transaction.type) {
        case 'transfer': {
          const { recipient, amount } = transaction.data;
          // Convert amount to planck (smallest unit)
          const amountPlanck = BigInt(parseFloat(amount) * 10 ** chainConfig.decimals);
          tx = api.tx.balances.transferKeepAlive(recipient, amountPlanck.toString());
          break;
        }

        case 'staking': {
          const { operation, amount, validator } = transaction.data;
          const amountPlanck = amount
            ? BigInt(parseFloat(amount) * 10 ** chainConfig.decimals)
            : null;

          if (operation === 'stake' && validator) {
            tx = api.tx.staking.bond(validator, amountPlanck?.toString() || '0', 'Staked');
          } else if (operation === 'unstake') {
            tx = api.tx.staking.unbond(amountPlanck?.toString() || '0');
          } else if (operation === 'claim') {
            tx = api.tx.staking.payoutStakers(selectedAccount.address, 0);
          }
          break;
        }

        case 'governance': {
          const { action, proposalId, voteType, voteAmount, conviction } = transaction.data;

          if (action === 'vote' && proposalId && voteType) {
            const amountPlanck = voteAmount
              ? BigInt(parseFloat(voteAmount) * 10 ** chainConfig.decimals)
              : BigInt(0);
            const vote = {
              Standard: {
                vote: voteType === 'aye' ? { aye: true } : { nay: true },
                balance: amountPlanck.toString(),
                conviction: conviction || 1,
              },
            };
            tx = api.tx.democracy.vote(proposalId, vote);
          } else if (action === 'propose') {
            const { proposalDeposit, proposalTitle, proposalDescription } = transaction.data;
            const depositPlanck = BigInt(parseFloat(proposalDeposit) * 10 ** chainConfig.decimals);
            // Note: Actual proposal creation would include the proposal hash
            tx = api.tx.democracy.propose(
              api.createType('Hash', proposalTitle),
              depositPlanck.toString()
            );
          }
          break;
        }

        case 'channel': {
          const { operation, counterparty, depositAmount, channelId, updateAmount } =
            transaction.data;

          if (operation === 'open' && counterparty && depositAmount) {
            const amountPlanck = BigInt(parseFloat(depositAmount) * 10 ** chainConfig.decimals);
            // Note: This is a simplified example - actual implementation would use proper pallet
            tx = api.tx.paymentChannels?.open(counterparty, amountPlanck.toString());
          } else if (operation === 'close' && channelId) {
            tx = api.tx.paymentChannels?.close(channelId);
          } else if (operation === 'update' && channelId && updateAmount) {
            const amountPlanck = BigInt(parseFloat(updateAmount) * 10 ** chainConfig.decimals);
            tx = api.tx.paymentChannels?.update(channelId, amountPlanck.toString());
          }
          break;
        }

        default:
          throw new Error(`Unknown transaction type: ${transaction.type}`);
      }

      if (!tx) {
        throw new Error('Failed to build transaction');
      }

      setStatus('broadcasting');

      // Sign and send transaction
      const unsub = await tx.signAndSend(
        selectedAccount.address,
        { signer: injector.signer },
        ({ status: txStatus, events }: any) => {
          if (txStatus.isInBlock) {
            setStatus('in-block');
            setTxHash(tx.hash.toHex());
            setBlockNumber(txStatus.asInBlock.toNumber?.() || null);

            toast({
              title: 'Transaction In Block',
              description: 'Your transaction has been included in a block',
            });
          } else if (txStatus.isFinalized) {
            setStatus('finalized');
            onSubmit({ hash: tx.hash.toHex() });

            toast({
              title: 'Transaction Finalized',
              description: 'Your transaction has been finalized',
            });

            unsub();
          } else if (txStatus.isInvalid) {
            setStatus('failed');
            setError('Transaction is invalid');
            unsub();
          }

          // Check for errors in events
          events.forEach(({ event }: any) => {
            if (api.events.system.ExtrinsicFailed.is(event)) {
              const [dispatchError] = event.data;
              let errorMsg = 'Transaction failed';

              if (dispatchError.isModule) {
                const decoded = api.registry.findMetaError(dispatchError.asModule);
                errorMsg = `${decoded.section}.${decoded.name}: ${decoded.docs}`;
              }

              setStatus('failed');
              setError(errorMsg);
              unsub();
            }
          });
        }
      );
    } catch (err) {
      console.error('Transaction error:', err);
      setStatus('failed');
      const message = err instanceof Error ? err.message : 'Transaction failed';
      setError(message);

      toast({
        title: 'Transaction Failed',
        description: message,
        variant: 'destructive',
      });
    }
  };

  // Copy transaction hash to clipboard
  const copyTxHash = () => {
    if (txHash) {
      navigator.clipboard.writeText(txHash);
      toast({
        title: 'Copied',
        description: 'Transaction hash copied to clipboard',
      });
    }
  };

  // Get transaction type icon
  const getTypeIcon = () => {
    switch (transaction.type) {
      case 'transfer':
        return <Send className="w-5 h-5" />;
      case 'staking':
        return <Lock className="w-5 h-5" />;
      case 'governance':
        return <Vote className="w-5 h-5" />;
      case 'channel':
        return <Layers className="w-5 h-5" />;
      default:
        return null;
    }
  };

  // Get transaction type label
  const getTypeLabel = () => {
    const labels = {
      transfer: 'Transfer',
      staking: 'Staking',
      governance: 'Governance',
      channel: 'Payment Channel',
    };
    return labels[transaction.type];
  };

  // Render transaction details based on type
  const renderTransactionDetails = () => {
    switch (transaction.type) {
      case 'transfer':
        return (
          <>
            <div className="flex justify-between py-2">
              <span className="text-muted-foreground">Recipient</span>
              <span className="font-mono text-sm">
                {transaction.data.recipient.slice(0, 8)}...
                {transaction.data.recipient.slice(-8)}
              </span>
            </div>
            <div className="flex justify-between py-2">
              <span className="text-muted-foreground">Amount</span>
              <span className="font-medium">
                {transaction.data.amount} {chainConfig.symbol}
              </span>
            </div>
            {transaction.data.memo && (
              <div className="flex justify-between py-2">
                <span className="text-muted-foreground">Memo</span>
                <span className="text-sm">{transaction.data.memo}</span>
              </div>
            )}
          </>
        );

      case 'staking':
        return (
          <>
            <div className="flex justify-between py-2">
              <span className="text-muted-foreground">Operation</span>
              <Badge variant="secondary" className="capitalize">
                {transaction.data.operation}
              </Badge>
            </div>
            {transaction.data.amount && (
              <div className="flex justify-between py-2">
                <span className="text-muted-foreground">Amount</span>
                <span className="font-medium">
                  {transaction.data.amount} {chainConfig.symbol}
                </span>
              </div>
            )}
            {transaction.data.validator && (
              <div className="flex justify-between py-2">
                <span className="text-muted-foreground">Validator</span>
                <span className="font-mono text-sm">
                  {transaction.data.validator.slice(0, 8)}...
                  {transaction.data.validator.slice(-8)}
                </span>
              </div>
            )}
          </>
        );

      case 'governance':
        return (
          <>
            <div className="flex justify-between py-2">
              <span className="text-muted-foreground">Action</span>
              <Badge variant="secondary" className="capitalize">
                {transaction.data.action}
              </Badge>
            </div>
            {transaction.data.proposalId && (
              <div className="flex justify-between py-2">
                <span className="text-muted-foreground">Proposal ID</span>
                <span className="font-medium">#{transaction.data.proposalId}</span>
              </div>
            )}
            {transaction.data.voteType && (
              <div className="flex justify-between py-2">
                <span className="text-muted-foreground">Vote</span>
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
          </>
        );

      case 'channel':
        return (
          <>
            <div className="flex justify-between py-2">
              <span className="text-muted-foreground">Operation</span>
              <Badge variant="secondary" className="capitalize">
                {transaction.data.operation}
              </Badge>
            </div>
            {transaction.data.counterparty && (
              <div className="flex justify-between py-2">
                <span className="text-muted-foreground">Counterparty</span>
                <span className="font-mono text-sm">
                  {transaction.data.counterparty.slice(0, 8)}...
                  {transaction.data.counterparty.slice(-8)}
                </span>
              </div>
            )}
            {transaction.data.depositAmount && (
              <div className="flex justify-between py-2">
                <span className="text-muted-foreground">Deposit</span>
                <span className="font-medium">
                  {transaction.data.depositAmount} {chainConfig.symbol}
                </span>
              </div>
            )}
          </>
        );

      default:
        return null;
    }
  };

  const isProcessing = status !== 'ready' && status !== 'failed' && status !== 'finalized';
  const isComplete = status === 'finalized';
  const hasFailed = status === 'failed';

  return (
    <div className="space-y-6">
      {/* Status Header */}
      {isProcessing && (
        <Card className="border-primary bg-primary/5">
          <CardContent className="pt-6">
            <div className="space-y-4">
              <div className="flex items-center gap-3">
                <Loader2 className="w-5 h-5 animate-spin text-primary" />
                <div className="flex-1">
                  <p className="font-medium capitalize">{status.replace('-', ' ')}</p>
                  <p className="text-sm text-muted-foreground">
                    {status === 'signing' && 'Please sign the transaction in your wallet'}
                    {status === 'broadcasting' && 'Broadcasting transaction to the network'}
                    {status === 'in-block' && 'Transaction included in block, awaiting finalization'}
                  </p>
                </div>
              </div>
              <Progress value={progress} className="h-2" />
            </div>
          </CardContent>
        </Card>
      )}

      {/* Success Status */}
      {isComplete && (
        <Alert className="border-green-500 bg-green-50 dark:bg-green-950/20">
          <CheckCircle2 className="h-5 w-5 text-green-600" />
          <AlertTitle className="text-green-900 dark:text-green-100">
            Transaction Finalized
          </AlertTitle>
          <AlertDescription className="text-green-800 dark:text-green-200">
            Your transaction has been successfully finalized on the blockchain
          </AlertDescription>
        </Alert>
      )}

      {/* Error Status */}
      {hasFailed && error && (
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertTitle>Transaction Failed</AlertTitle>
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Transaction Details */}
      <Card>
        <CardHeader>
          <div className="flex items-center gap-3">
            <div className="p-2 rounded-lg bg-primary/10 text-primary">{getTypeIcon()}</div>
            <div className="flex-1">
              <CardTitle>{getTypeLabel()} Transaction</CardTitle>
              <CardDescription>Review the transaction details below</CardDescription>
            </div>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Chain Info */}
          <div className="flex items-center justify-between py-2">
            <span className="text-muted-foreground">Chain</span>
            <div className="flex items-center gap-2">
              <div
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: chainConfig.color }}
              />
              <span className="font-medium">{chainConfig.name}</span>
            </div>
          </div>

          <Separator />

          {/* Transaction-specific details */}
          {renderTransactionDetails()}

          <Separator />

          {/* Fee Information */}
          <div className="flex justify-between py-2">
            <span className="text-muted-foreground">Estimated Fee</span>
            <span className="font-mono">
              {transaction.data.estimatedFee} {chainConfig.symbol}
            </span>
          </div>

          {/* Total (if applicable) */}
          {transaction.data.amount && (
            <div className="flex justify-between py-2 text-lg font-semibold">
              <span>Total</span>
              <span>
                {(
                  parseFloat(transaction.data.amount) + parseFloat(transaction.data.estimatedFee)
                ).toFixed(6)}{' '}
                {chainConfig.symbol}
              </span>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Transaction Hash */}
      {txHash && (
        <Card>
          <CardContent className="pt-6">
            <div className="space-y-2">
              <Label className="text-sm font-medium">Transaction Hash</Label>
              <div className="flex items-center gap-2">
                <code className="flex-1 p-2 bg-muted rounded text-xs font-mono break-all">
                  {txHash}
                </code>
                <Button variant="outline" size="icon" onClick={copyTxHash}>
                  <Copy className="w-4 h-4" />
                </Button>
              </div>
              {blockNumber && (
                <p className="text-xs text-muted-foreground flex items-center gap-1">
                  <Clock className="w-3 h-3" />
                  Block: {blockNumber}
                </p>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Warning for high-value transactions */}
      {!isComplete && !isProcessing && transaction.data.amount && parseFloat(transaction.data.amount) > 100 && (
        <Alert>
          <Info className="h-4 w-4" />
          <AlertDescription className="text-sm">
            You're about to send a large transaction. Please double-check all details before confirming.
          </AlertDescription>
        </Alert>
      )}

      {/* Action Buttons */}
      {!isComplete && (
        <div className="flex gap-3">
          <Button
            variant="outline"
            onClick={onCancel}
            disabled={isProcessing}
            className="flex-1"
          >
            Cancel
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={isProcessing || !web3FromAddress}
            className="flex-1"
          >
            {isProcessing ? (
              <>
                <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                Processing...
              </>
            ) : (
              <>
                Sign & Submit
                <Send className="w-4 h-4 ml-2" />
              </>
            )}
          </Button>
        </div>
      )}

      {isComplete && (
        <div className="flex gap-3">
          <Button variant="outline" onClick={onCancel} className="flex-1">
            Close
          </Button>
          {txHash && (
            <Button variant="outline" asChild className="flex-1">
              <a
                href={`https://polkadot.js.org/apps/?rpc=${chainConfig.rpc}#/explorer/query/${txHash}`}
                target="_blank"
                rel="noopener noreferrer"
              >
                View on Explorer
                <ExternalLink className="w-4 h-4 ml-2" />
              </a>
            </Button>
          )}
        </div>
      )}
    </div>
  );
}

function Label({ children, className }: { children: React.ReactNode; className?: string }) {
  return <label className={className}>{children}</label>;
}
