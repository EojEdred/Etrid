/**
 * INTEGRATION_EXAMPLE.tsx
 * Complete example showing how to integrate all new Transaction Builder enhancements
 */

'use client';

import React, { useState } from 'react';
import {
  // Core Components
  TransactionBuilder,
  TransferBuilder,

  // New Enhanced Components
  ChainSelector,
  TokenSelector,
  TransactionPreview,
  TransactionExport,

  // Existing Enhanced Features
  FeeEstimator,
  TransactionHistory,
  BatchBuilder,
  TransactionSimulator,

  // Types
  type TransactionType,
  type TransactionData,
  type Token,
  type FeePriority,
  type ExportTransaction,
} from '@/components/TransactionBuilder';

import { ChainId } from '@/lib/polkadot/chains';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

/**
 * Example 1: Enhanced Transfer Builder with Chain and Token Selection
 */
export function EnhancedTransferExample() {
  const [chainId, setChainId] = useState<ChainId>('eth-pbc');
  const [selectedToken, setSelectedToken] = useState<Token | null>(null);
  const [amount, setAmount] = useState('');
  const [feePriority, setFeePriority] = useState<FeePriority>('medium');

  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">Enhanced Transfer Example</h2>

      {/* Step 1: Chain Selection with Network Detection */}
      <ChainSelector
        value={chainId}
        onChange={setChainId}
        label="Select Network"
        showBalance={true}
        balance="100.50"
        showNetworkStatus={true}
      />

      {/* Step 2: Token Selection (Native + ERC-20) */}
      <TokenSelector
        chainId={chainId}
        value={selectedToken}
        onChange={setSelectedToken}
        label="Select Token to Send"
        showBalance={true}
        allowCustomTokens={true}
      />

      {/* Step 3: Fee Estimation with Priority Options */}
      <FeeEstimator
        chainId={chainId}
        transactionType="transfer"
        transactionSize={250}
        onFeeSelected={(fee) => console.log('Fee selected:', fee)}
        value={feePriority}
      />
    </div>
  );
}

/**
 * Example 2: Transaction Preview Before Signing
 */
export function TransactionPreviewExample() {
  const [showPreview, setShowPreview] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const transactionData: TransactionData = {
    type: 'transfer',
    chainId: 'flarechain',
    data: {
      recipient: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
      amount: '150.75',
      memo: 'Payment for services',
    },
  };

  const handleConfirm = async () => {
    setIsSubmitting(true);
    try {
      // Sign and submit transaction
      console.log('Transaction confirmed');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold">Transaction Preview Example</h2>

      <Button onClick={() => setShowPreview(true)}>
        Preview Transaction
      </Button>

      {showPreview && (
        <TransactionPreview
          transaction={transactionData}
          estimatedFee="0.01"
          estimatedTime="10-20s"
          onConfirm={handleConfirm}
          onCancel={() => setShowPreview(false)}
          isLoading={isSubmitting}
        />
      )}
    </div>
  );
}

/**
 * Example 3: Transaction History with Export
 */
export function HistoryWithExportExample() {
  const [showExport, setShowExport] = useState(false);
  const [selectedTransactions, setSelectedTransactions] = useState<ExportTransaction[]>([]);

  // Mock transaction history
  const mockTransactions: ExportTransaction[] = [
    {
      hash: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
      type: 'transfer',
      chainId: 'flarechain',
      timestamp: Date.now() - 3600000,
      status: 'confirmed',
      blockNumber: 1234567,
      from: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
      to: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
      amount: '10.5',
      fee: '0.01',
      details: {},
    },
    // Add more transactions...
  ];

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold">Transaction History</h2>
        <Button onClick={() => {
          setSelectedTransactions(mockTransactions);
          setShowExport(true);
        }}>
          Export All Transactions
        </Button>
      </div>

      {/* Transaction History Component */}
      <TransactionHistory
        onReuse={(tx) => console.log('Reusing transaction:', tx)}
      />

      {/* Export Dialog */}
      {showExport && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
          <div className="bg-background rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto p-6">
            <TransactionExport
              transactions={selectedTransactions}
              onClose={() => setShowExport(false)}
            />
          </div>
        </div>
      )}
    </div>
  );
}

/**
 * Example 4: Complete Multi-Chain Transaction Flow
 */
export function CompleteTransactionFlowExample() {
  const [step, setStep] = useState<'select' | 'build' | 'preview' | 'simulate' | 'sign'>('select');
  const [chainId, setChainId] = useState<ChainId>('eth-pbc');
  const [selectedToken, setSelectedToken] = useState<Token | null>(null);
  const [transactionData, setTransactionData] = useState<TransactionData | null>(null);

  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">Complete Transaction Flow</h2>

      {/* Progress Indicator */}
      <div className="flex items-center gap-2">
        {['Select', 'Build', 'Preview', 'Simulate', 'Sign'].map((label, index) => (
          <div key={label} className="flex items-center gap-2">
            <div className={`px-4 py-2 rounded ${
              step === label.toLowerCase() ? 'bg-primary text-white' : 'bg-muted'
            }`}>
              {label}
            </div>
            {index < 4 && <div className="w-8 h-0.5 bg-border" />}
          </div>
        ))}
      </div>

      {/* Step 1: Chain & Token Selection */}
      {step === 'select' && (
        <div className="space-y-4">
          <ChainSelector
            value={chainId}
            onChange={setChainId}
            showNetworkStatus={true}
            showBalance={true}
            balance="100.50"
          />
          <TokenSelector
            chainId={chainId}
            value={selectedToken}
            onChange={setSelectedToken}
            allowCustomTokens={true}
          />
          <Button onClick={() => setStep('build')}>
            Next: Build Transaction
          </Button>
        </div>
      )}

      {/* Step 2: Build Transaction */}
      {step === 'build' && (
        <div className="space-y-4">
          <TransferBuilder
            onComplete={(data) => {
              setTransactionData({
                type: 'transfer',
                chainId,
                data,
              });
              setStep('preview');
            }}
          />
        </div>
      )}

      {/* Step 3: Preview Transaction */}
      {step === 'preview' && transactionData && (
        <TransactionPreview
          transaction={transactionData}
          estimatedFee="0.01"
          estimatedTime="10-20s"
          onConfirm={() => setStep('simulate')}
          onCancel={() => setStep('build')}
        />
      )}

      {/* Step 4: Simulate Transaction */}
      {step === 'simulate' && transactionData && (
        <div className="space-y-4">
          <TransactionSimulator
            transaction={transactionData}
            onClose={() => setStep('sign')}
          />
        </div>
      )}

      {/* Step 5: Sign Transaction */}
      {step === 'sign' && transactionData && (
        <div className="space-y-4">
          <p>Final step: Sign with your wallet...</p>
          <Button onClick={() => console.log('Transaction submitted')}>
            Sign & Submit
          </Button>
        </div>
      )}
    </div>
  );
}

/**
 * Example 5: Batch Transaction with Multi-Chain Support
 */
export function BatchTransactionExample() {
  const [chainId, setChainId] = useState<ChainId>('flarechain');

  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">Batch Transaction Example</h2>

      {/* Chain Selection */}
      <ChainSelector
        value={chainId}
        onChange={setChainId}
        label="Select Chain for Batch Transfer"
        showNetworkStatus={true}
      />

      {/* Batch Builder */}
      <BatchBuilder
        onComplete={(data) => {
          console.log('Batch transaction data:', data);
          console.log('Total recipients:', data.transactions.length);
          console.log('Total amount:', data.totalAmount);
          console.log('Estimated fee:', data.estimatedFee);
        }}
      />
    </div>
  );
}

/**
 * Example 6: Full-Featured Transaction Dashboard
 */
export function TransactionDashboard() {
  const [activeTab, setActiveTab] = useState('new');
  const [showExport, setShowExport] = useState(false);

  return (
    <div className="container mx-auto py-8 space-y-6">
      <h1 className="text-3xl font-bold">Transaction Dashboard</h1>

      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="new">New Transaction</TabsTrigger>
          <TabsTrigger value="batch">Batch Transfer</TabsTrigger>
          <TabsTrigger value="history">History</TabsTrigger>
          <TabsTrigger value="export">Export</TabsTrigger>
        </TabsList>

        <TabsContent value="new" className="mt-6">
          <CompleteTransactionFlowExample />
        </TabsContent>

        <TabsContent value="batch" className="mt-6">
          <BatchTransactionExample />
        </TabsContent>

        <TabsContent value="history" className="mt-6">
          <HistoryWithExportExample />
        </TabsContent>

        <TabsContent value="export" className="mt-6">
          <TransactionExport
            transactions={[]}
            onClose={() => setActiveTab('history')}
          />
        </TabsContent>
      </Tabs>
    </div>
  );
}

/**
 * Main Export - Use this in your app
 */
export default function TransactionBuilderIntegration() {
  return (
    <div className="min-h-screen bg-background">
      <TransactionDashboard />
    </div>
  );
}
