/**
 * Bill Split Screen
 * Create and manage group expenses and split bills
 */

'use client';

import { useState } from 'react';
import { ArrowLeft, Plus, DollarSign, Users, TrendingDown, TrendingUp } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { BillSplitItem } from '@/components/social/bill-split-item';
import { useBillSplit } from '@/hooks/useBillSplit';
import { useWallet } from '@/lib/polkadot/useWallet';
import type { BillSplitInput, BillSplitType } from '@/lib/social/types';

interface BillSplitScreenProps {
  onBack: () => void;
}

export function BillSplitScreen({ onBack }: BillSplitScreenProps) {
  const { selectedAccount, sendTransaction } = useWallet();
  const userId = selectedAccount?.address || '';

  const {
    pendingSplits,
    completedSplits,
    owedSplits,
    receivableSplits,
    summary,
    isLoading,
    createSplit,
    payShare,
    remindParticipants,
  } = useBillSplit(userId);

  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [newSplit, setNewSplit] = useState<BillSplitInput>({
    name: '',
    description: '',
    totalAmount: 0,
    splitType: 'equal',
    participants: [],
  });

  const [participantAddress, setParticipantAddress] = useState('');

  const handleCreateSplit = async () => {
    try {
      await createSplit(newSplit);
      setIsCreateDialogOpen(false);
      setNewSplit({
        name: '',
        description: '',
        totalAmount: 0,
        splitType: 'equal',
        participants: [],
      });
      setParticipantAddress('');
    } catch (error) {
      console.error('Failed to create split:', error);
    }
  };

  const handleAddParticipant = () => {
    if (participantAddress) {
      setNewSplit({
        ...newSplit,
        participants: [
          ...newSplit.participants,
          { address: participantAddress },
        ],
      });
      setParticipantAddress('');
    }
  };

  const handlePayShare = async (split: any) => {
    if (!selectedAccount) return;

    try {
      const userParticipant = split.participants.find((p: any) => p.userId === userId);
      if (!userParticipant) return;

      const amount = userParticipant.amountOwed - userParticipant.amountPaid;

      // Send transaction
      const txHash = await sendTransaction(split.creatorAddress, amount.toString());

      // Record payment
      await payShare(split.id, txHash);
    } catch (error) {
      console.error('Failed to pay share:', error);
    }
  };

  const handleRemind = async (split: any) => {
    try {
      await remindParticipants(split.id);
    } catch (error) {
      console.error('Failed to send reminders:', error);
    }
  };

  return (
    <div className="min-h-screen bg-background pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 glass-strong border-b border-border">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-xl font-bold">Bill Split</h1>
            <p className="text-sm text-muted-foreground">Manage group expenses</p>
          </div>
          <Dialog open={isCreateDialogOpen} onOpenChange={setIsCreateDialogOpen}>
            <DialogTrigger asChild>
              <Button size="icon">
                <Plus className="w-5 h-5" />
              </Button>
            </DialogTrigger>
            <DialogContent className="max-w-lg">
              <DialogHeader>
                <DialogTitle>Create Bill Split</DialogTitle>
                <DialogDescription>
                  Split expenses with friends and track payments
                </DialogDescription>
              </DialogHeader>
              <div className="space-y-4 py-4">
                <div className="space-y-2">
                  <Label htmlFor="name">Event Name</Label>
                  <Input
                    id="name"
                    placeholder="Dinner at Restaurant"
                    value={newSplit.name}
                    onChange={(e) => setNewSplit({ ...newSplit, name: e.target.value })}
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="description">Description (optional)</Label>
                  <Input
                    id="description"
                    placeholder="Team celebration dinner"
                    value={newSplit.description}
                    onChange={(e) =>
                      setNewSplit({ ...newSplit, description: e.target.value })
                    }
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="amount">Total Amount (ÉTR)</Label>
                  <Input
                    id="amount"
                    type="number"
                    placeholder="150"
                    value={newSplit.totalAmount || ''}
                    onChange={(e) =>
                      setNewSplit({
                        ...newSplit,
                        totalAmount: parseFloat(e.target.value) || 0,
                      })
                    }
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="splitType">Split Type</Label>
                  <Select
                    value={newSplit.splitType}
                    onValueChange={(value) =>
                      setNewSplit({ ...newSplit, splitType: value as BillSplitType })
                    }
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="equal">Split Equally</SelectItem>
                      <SelectItem value="custom">Custom Amounts</SelectItem>
                      <SelectItem value="percentage">By Percentage</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div className="space-y-2">
                  <Label>Participants ({newSplit.participants.length})</Label>
                  <div className="flex gap-2">
                    <Input
                      placeholder="Wallet address or username"
                      value={participantAddress}
                      onChange={(e) => setParticipantAddress(e.target.value)}
                    />
                    <Button onClick={handleAddParticipant} variant="outline">
                      Add
                    </Button>
                  </div>
                  {newSplit.participants.length > 0 && (
                    <div className="space-y-1 mt-2">
                      {newSplit.participants.map((p, i) => (
                        <div
                          key={i}
                          className="text-sm p-2 rounded bg-accent/10 flex items-center justify-between"
                        >
                          <span className="font-mono text-xs">{p.address.slice(0, 20)}...</span>
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={() =>
                              setNewSplit({
                                ...newSplit,
                                participants: newSplit.participants.filter((_, idx) => idx !== i),
                              })
                            }
                          >
                            Remove
                          </Button>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>
              <DialogFooter>
                <Button variant="outline" onClick={() => setIsCreateDialogOpen(false)}>
                  Cancel
                </Button>
                <Button
                  onClick={handleCreateSplit}
                  disabled={
                    !newSplit.name ||
                    newSplit.totalAmount <= 0 ||
                    newSplit.participants.length === 0
                  }
                >
                  Create Split
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
      </header>

      {/* Summary Cards */}
      {summary && (
        <div className="p-6 grid grid-cols-2 gap-3">
          <Card className="bg-gradient-to-br from-red-500/10 to-red-500/5 border-red-500/20">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium flex items-center gap-2">
                <TrendingDown className="w-4 h-4 text-red-500" />
                You Owe
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-2xl font-bold text-red-600 dark:text-red-400">
                {summary.totalOwed.toFixed(2)} ÉTR
              </p>
            </CardContent>
          </Card>

          <Card className="bg-gradient-to-br from-green-500/10 to-green-500/5 border-green-500/20">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium flex items-center gap-2">
                <TrendingUp className="w-4 h-4 text-green-500" />
                Owed to You
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-2xl font-bold text-green-600 dark:text-green-400">
                {summary.totalReceivable.toFixed(2)} ÉTR
              </p>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Content */}
      <main className="p-6">
        <Tabs defaultValue="pending" className="space-y-6">
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="pending">
              Pending ({summary?.pending || 0})
            </TabsTrigger>
            <TabsTrigger value="owed">
              You Owe ({owedSplits.length})
            </TabsTrigger>
            <TabsTrigger value="completed">
              Completed ({summary?.completed || 0})
            </TabsTrigger>
          </TabsList>

          {/* Pending Splits */}
          <TabsContent value="pending" className="space-y-3">
            {isLoading ? (
              <div className="text-center py-12">
                <p className="text-muted-foreground">Loading splits...</p>
              </div>
            ) : pendingSplits.length === 0 ? (
              <div className="text-center py-12">
                <Users className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">No pending splits</p>
              </div>
            ) : (
              pendingSplits.map((split) => (
                <BillSplitItem
                  key={split.id}
                  split={split}
                  currentUserId={userId}
                  onPay={handlePayShare}
                  onRemind={handleRemind}
                />
              ))
            )}
          </TabsContent>

          {/* You Owe */}
          <TabsContent value="owed" className="space-y-3">
            {owedSplits.length === 0 ? (
              <div className="text-center py-12">
                <DollarSign className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">You're all caught up!</p>
              </div>
            ) : (
              owedSplits.map((split) => (
                <BillSplitItem
                  key={split.id}
                  split={split}
                  currentUserId={userId}
                  onPay={handlePayShare}
                />
              ))
            )}
          </TabsContent>

          {/* Completed */}
          <TabsContent value="completed" className="space-y-3">
            {completedSplits.length === 0 ? (
              <div className="text-center py-12">
                <Users className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">No completed splits yet</p>
              </div>
            ) : (
              completedSplits.map((split) => (
                <BillSplitItem key={split.id} split={split} currentUserId={userId} />
              ))
            )}
          </TabsContent>
        </Tabs>
      </main>
    </div>
  );
}
