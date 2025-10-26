/**
 * Transaction Builder Usage Examples
 * Practical examples for implementing the Transaction Builder components
 */

'use client';

import React, { useState } from 'react';
import { TransactionBuilder } from './TransactionBuilder';
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Send, Lock, Vote, Layers } from 'lucide-react';

// Example 1: Basic Usage
export function BasicExample() {
  return (
    <div className="max-w-4xl mx-auto p-6">
      <TransactionBuilder />
    </div>
  );
}

// Example 2: With Dialog/Modal
export function ModalExample() {
  const [open, setOpen] = useState(false);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>New Transaction</Button>
      </DialogTrigger>
      <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
        <TransactionBuilder onClose={() => setOpen(false)} />
      </DialogContent>
    </Dialog>
  );
}

// Example 3: Quick Action Buttons
export function QuickActionExample() {
  const [isOpen, setIsOpen] = useState(false);
  const [initialType, setInitialType] = useState<'transfer' | 'staking' | 'governance' | 'channel'>('transfer');

  const handleQuickAction = (type: typeof initialType) => {
    setInitialType(type);
    setIsOpen(true);
  };

  return (
    <div className="space-y-6">
      {/* Quick Action Cards */}
      <div className="grid grid-cols-4 gap-4">
        <Card
          className="cursor-pointer hover:border-primary transition-colors"
          onClick={() => handleQuickAction('transfer')}
        >
          <CardContent className="pt-6 flex flex-col items-center gap-2">
            <Send className="w-8 h-8 text-primary" />
            <h3 className="font-medium">Send</h3>
            <p className="text-xs text-muted-foreground text-center">
              Transfer tokens
            </p>
          </CardContent>
        </Card>

        <Card
          className="cursor-pointer hover:border-primary transition-colors"
          onClick={() => handleQuickAction('staking')}
        >
          <CardContent className="pt-6 flex flex-col items-center gap-2">
            <Lock className="w-8 h-8 text-primary" />
            <h3 className="font-medium">Stake</h3>
            <p className="text-xs text-muted-foreground text-center">
              Earn rewards
            </p>
          </CardContent>
        </Card>

        <Card
          className="cursor-pointer hover:border-primary transition-colors"
          onClick={() => handleQuickAction('governance')}
        >
          <CardContent className="pt-6 flex flex-col items-center gap-2">
            <Vote className="w-8 h-8 text-primary" />
            <h3 className="font-medium">Vote</h3>
            <p className="text-xs text-muted-foreground text-center">
              Participate
            </p>
          </CardContent>
        </Card>

        <Card
          className="cursor-pointer hover:border-primary transition-colors"
          onClick={() => handleQuickAction('channel')}
        >
          <CardContent className="pt-6 flex flex-col items-center gap-2">
            <Layers className="w-8 h-8 text-primary" />
            <h3 className="font-medium">Channel</h3>
            <p className="text-xs text-muted-foreground text-center">
              Fast payments
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Transaction Builder Dialog */}
      {isOpen && (
        <Dialog open={isOpen} onOpenChange={setIsOpen}>
          <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
            <TransactionBuilder
              initialType={initialType}
              onClose={() => setIsOpen(false)}
            />
          </DialogContent>
        </Dialog>
      )}
    </div>
  );
}

// Example 4: Dashboard Integration
export function DashboardExample() {
  const [showBuilder, setShowBuilder] = useState(false);
  const [recentTransactions] = useState([
    { hash: '0x1234...5678', type: 'Transfer', status: 'Finalized', amount: '10.5 ETR' },
    { hash: '0xabcd...efgh', type: 'Staking', status: 'In Block', amount: '50.0 ETR' },
  ]);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Wallet Dashboard</h2>
          <p className="text-muted-foreground">Manage your Etrid Protocol assets</p>
        </div>
        <Button onClick={() => setShowBuilder(true)}>
          New Transaction
        </Button>
      </div>

      {/* Recent Transactions */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Transactions</CardTitle>
          <CardDescription>Your latest blockchain activity</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            {recentTransactions.map((tx, i) => (
              <div key={i} className="flex items-center justify-between p-3 rounded-lg border">
                <div>
                  <p className="font-medium">{tx.type}</p>
                  <p className="text-xs text-muted-foreground font-mono">{tx.hash}</p>
                </div>
                <div className="text-right">
                  <p className="font-medium">{tx.amount}</p>
                  <p className="text-xs text-muted-foreground">{tx.status}</p>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Transaction Builder */}
      {showBuilder && (
        <Dialog open={showBuilder} onOpenChange={setShowBuilder}>
          <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
            <TransactionBuilder onClose={() => setShowBuilder(false)} />
          </DialogContent>
        </Dialog>
      )}
    </div>
  );
}

// Example 5: Programmatic Transaction
export function ProgrammaticExample() {
  const [status, setStatus] = useState<string>('');

  const handleTransactionComplete = (txHash: { hash: string }) => {
    setStatus(`Transaction completed: ${txHash.hash}`);
    console.log('Transaction hash:', txHash.hash);

    // You can now:
    // - Update UI to show success
    // - Refresh balances
    // - Navigate to transaction history
    // - Show notifications
  };

  return (
    <div className="space-y-4">
      <TransactionBuilder
        initialType="transfer"
        onClose={() => console.log('Transaction builder closed')}
      />

      {status && (
        <Card className="bg-green-50 dark:bg-green-950/20 border-green-200">
          <CardContent className="pt-4">
            <p className="text-sm text-green-800 dark:text-green-200">{status}</p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

// Example 6: Custom Styling
export function CustomStyledExample() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-50 to-blue-50 dark:from-gray-900 dark:to-gray-800 p-6">
      <div className="max-w-4xl mx-auto">
        <Card className="border-2 shadow-2xl">
          <CardContent className="p-8">
            <TransactionBuilder />
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

// Example 7: With Error Handling
export function ErrorHandlingExample() {
  const [error, setError] = useState<string | null>(null);

  const handleError = (err: Error) => {
    setError(err.message);
    console.error('Transaction error:', err);
  };

  return (
    <div className="space-y-4">
      {error && (
        <Card className="bg-red-50 dark:bg-red-950/20 border-red-200">
          <CardContent className="pt-4">
            <p className="text-sm text-red-800 dark:text-red-200">{error}</p>
            <Button
              variant="outline"
              size="sm"
              className="mt-2"
              onClick={() => setError(null)}
            >
              Dismiss
            </Button>
          </CardContent>
        </Card>
      )}

      <TransactionBuilder />
    </div>
  );
}

// Example 8: Multi-Step Wizard in Custom Component
export function CustomWizardExample() {
  return (
    <Card className="max-w-4xl mx-auto">
      <CardHeader className="text-center">
        <CardTitle className="text-3xl">Create Transaction</CardTitle>
        <CardDescription>
          Follow the steps to create and submit your transaction
        </CardDescription>
      </CardHeader>
      <CardContent>
        <TransactionBuilder />
      </CardContent>
    </Card>
  );
}
