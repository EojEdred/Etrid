/**
 * TransactionBuilder - Main Transaction Builder Component
 * Provides a step-by-step wizard interface for creating and submitting transactions
 */

'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Badge } from '@/components/ui/badge';
import { TransferBuilder } from './TransferBuilder';
import { StakingBuilder } from './StakingBuilder';
import { GovernanceBuilder } from './GovernanceBuilder';
import { ChannelBuilder } from './ChannelBuilder';
import { TransactionReview } from './TransactionReview';
import { useWallet } from '@/lib/polkadot/useWallet';
import { useToast } from '@/hooks/use-toast';
import { ArrowLeft, Send, Vote, Layers, Lock } from 'lucide-react';

export type TransactionType = 'transfer' | 'staking' | 'governance' | 'channel';

export interface TransactionData {
  type: TransactionType;
  chainId: string;
  data: any;
}

export interface TransactionStep {
  id: string;
  title: string;
  description: string;
  status: 'pending' | 'active' | 'completed';
}

interface TransactionBuilderProps {
  onClose?: () => void;
  initialType?: TransactionType;
}

export function TransactionBuilder({ onClose, initialType = 'transfer' }: TransactionBuilderProps) {
  const [currentStep, setCurrentStep] = useState<'build' | 'review'>('build');
  const [transactionType, setTransactionType] = useState<TransactionType>(initialType);
  const [transactionData, setTransactionData] = useState<TransactionData | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [txHash, setTxHash] = useState<string | null>(null);

  const { selectedAccount, isConnected } = useWallet();
  const { toast } = useToast();

  // Handle transaction data from builder components
  const handleBuildComplete = (data: any) => {
    setTransactionData({
      type: transactionType,
      chainId: data.chainId,
      data,
    });
    setCurrentStep('review');
  };

  // Handle transaction submission
  const handleSubmit = async (signedTx: any) => {
    setIsSubmitting(true);
    try {
      // Transaction submission is handled by the review component
      // This callback receives the transaction hash after successful submission
      setTxHash(signedTx.hash);
      toast({
        title: 'Transaction Submitted',
        description: `Transaction hash: ${signedTx.hash.slice(0, 10)}...`,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to submit transaction';
      toast({
        title: 'Transaction Failed',
        description: message,
        variant: 'destructive',
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  // Handle going back from review to builder
  const handleBack = () => {
    setCurrentStep('build');
    setTransactionData(null);
  };

  // Reset and close
  const handleReset = () => {
    setCurrentStep('build');
    setTransactionData(null);
    setTxHash(null);
    if (onClose) onClose();
  };

  if (!isConnected) {
    return (
      <Card className="w-full max-w-2xl mx-auto">
        <CardHeader>
          <CardTitle>Connect Wallet</CardTitle>
          <CardDescription>
            Please connect your wallet to create transactions
          </CardDescription>
        </CardHeader>
      </Card>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          <h2 className="text-2xl font-bold tracking-tight">Transaction Builder</h2>
          <p className="text-sm text-muted-foreground">
            Create and submit transactions on the Etrid Protocol
          </p>
        </div>
        {onClose && (
          <Button variant="ghost" onClick={onClose}>
            Close
          </Button>
        )}
      </div>

      {/* Progress Indicator */}
      <div className="flex items-center gap-4">
        <div className={`flex items-center gap-2 ${currentStep === 'build' ? 'text-primary' : 'text-muted-foreground'}`}>
          <div className={`w-8 h-8 rounded-full flex items-center justify-center border-2 ${
            currentStep === 'build' ? 'border-primary bg-primary/10' : 'border-muted-foreground'
          }`}>
            1
          </div>
          <span className="font-medium">Build</span>
        </div>
        <div className="flex-1 h-0.5 bg-border" />
        <div className={`flex items-center gap-2 ${currentStep === 'review' ? 'text-primary' : 'text-muted-foreground'}`}>
          <div className={`w-8 h-8 rounded-full flex items-center justify-center border-2 ${
            currentStep === 'review' ? 'border-primary bg-primary/10' : 'border-muted-foreground'
          }`}>
            2
          </div>
          <span className="font-medium">Review & Submit</span>
        </div>
      </div>

      {/* Main Content */}
      {currentStep === 'build' && (
        <Card>
          <CardHeader>
            <CardTitle>Select Transaction Type</CardTitle>
            <CardDescription>
              Choose the type of transaction you want to create
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Tabs value={transactionType} onValueChange={(v) => setTransactionType(v as TransactionType)}>
              <TabsList className="grid w-full grid-cols-4">
                <TabsTrigger value="transfer" className="gap-2">
                  <Send className="w-4 h-4" />
                  Transfer
                </TabsTrigger>
                <TabsTrigger value="staking" className="gap-2">
                  <Lock className="w-4 h-4" />
                  Staking
                </TabsTrigger>
                <TabsTrigger value="governance" className="gap-2">
                  <Vote className="w-4 h-4" />
                  Governance
                </TabsTrigger>
                <TabsTrigger value="channel" className="gap-2">
                  <Layers className="w-4 h-4" />
                  Channel
                </TabsTrigger>
              </TabsList>

              <TabsContent value="transfer" className="mt-6">
                <TransferBuilder onComplete={handleBuildComplete} />
              </TabsContent>

              <TabsContent value="staking" className="mt-6">
                <StakingBuilder onComplete={handleBuildComplete} />
              </TabsContent>

              <TabsContent value="governance" className="mt-6">
                <GovernanceBuilder onComplete={handleBuildComplete} />
              </TabsContent>

              <TabsContent value="channel" className="mt-6">
                <ChannelBuilder onComplete={handleBuildComplete} />
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>
      )}

      {currentStep === 'review' && transactionData && (
        <div className="space-y-4">
          <Button variant="ghost" onClick={handleBack} className="gap-2">
            <ArrowLeft className="w-4 h-4" />
            Back to Builder
          </Button>
          <TransactionReview
            transaction={transactionData}
            onSubmit={handleSubmit}
            onCancel={handleBack}
            isSubmitting={isSubmitting}
            txHash={txHash}
          />
        </div>
      )}

      {/* Account Info */}
      {selectedAccount && (
        <Card className="bg-muted/50">
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div className="space-y-1">
                <p className="text-sm font-medium">Connected Account</p>
                <p className="text-xs text-muted-foreground font-mono">
                  {selectedAccount.address.slice(0, 8)}...{selectedAccount.address.slice(-8)}
                </p>
              </div>
              {selectedAccount.balance && (
                <Badge variant="secondary" className="font-mono">
                  {selectedAccount.balance} ETR
                </Badge>
              )}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
