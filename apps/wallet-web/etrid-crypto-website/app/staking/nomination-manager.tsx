'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import {
  Plus,
  Minus,
  AlertCircle,
  Info,
  CheckCircle2,
  Clock,
  TrendingUp,
  Shield,
  ArrowLeft
} from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Progress } from '@/components/ui/progress';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Separator } from '@/components/ui/separator';
import { NominationForm } from '@/components/staking/nomination-form';

interface ExistingNomination {
  id: string;
  validatorName: string;
  validatorAddress: string;
  stakedAmount: string;
  apy: number;
  commission: number;
  status: 'active' | 'waiting' | 'unbonding';
  unbondingPeriod?: string;
  rewards: string;
}

interface PendingAction {
  id: string;
  type: 'stake' | 'unstake' | 'rebond';
  validatorName: string;
  amount: string;
  executionTime: string;
  canCancel: boolean;
}

export default function NominationManager() {
  const [activeTab, setActiveTab] = useState('new');
  const [existingNominations] = useState<ExistingNomination[]>([
    {
      id: '1',
      validatorName: 'Validator Alpha',
      validatorAddress: '0x1234...5678',
      stakedAmount: '2,500 ETD',
      apy: 13.2,
      commission: 5,
      status: 'active',
      rewards: '82.15 ETD'
    },
    {
      id: '2',
      validatorName: 'Beta Staking',
      validatorAddress: '0x2345...6789',
      stakedAmount: '2,000 ETD',
      apy: 12.8,
      commission: 7,
      status: 'active',
      rewards: '64.32 ETD'
    },
    {
      id: '3',
      validatorName: 'Gamma Node',
      validatorAddress: '0x3456...7890',
      stakedAmount: '1,000 ETD',
      apy: 0,
      commission: 8,
      status: 'unbonding',
      unbondingPeriod: '12 days',
      rewards: '0 ETD'
    }
  ]);

  const [pendingActions] = useState<PendingAction[]>([
    {
      id: '1',
      type: 'stake',
      validatorName: 'Delta Validator',
      amount: '1,500 ETD',
      executionTime: '~6 hours',
      canCancel: true
    }
  ]);

  const [unstakeAmount, setUnstakeAmount] = useState<{ [key: string]: string }>({});
  const [addStakeAmount, setAddStakeAmount] = useState<{ [key: string]: string }>({});

  const handleUnstake = (nominationId: string) => {
    const amount = unstakeAmount[nominationId];
    console.log('Unstaking', amount, 'from nomination', nominationId);
    // Implement unstaking logic
  };

  const handleAddStake = (nominationId: string) => {
    const amount = addStakeAmount[nominationId];
    console.log('Adding', amount, 'stake to nomination', nominationId);
    // Implement add stake logic
  };

  const handleCancelAction = (actionId: string) => {
    console.log('Cancelling action', actionId);
    // Implement cancel logic
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-background to-muted/20 p-4 md:p-8">
      <div className="max-w-5xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center gap-4">
          <Link href="/staking/nominator-dashboard">
            <Button variant="ghost" size="icon">
              <ArrowLeft className="h-5 w-5" />
            </Button>
          </Link>
          <div className="flex-1">
            <h1 className="text-4xl font-bold tracking-tight">Nomination Manager</h1>
            <p className="text-muted-foreground mt-2">
              Create new nominations or manage existing stakes
            </p>
          </div>
        </div>

        {/* Pending Actions Alert */}
        {pendingActions.length > 0 && (
          <Alert>
            <Clock className="h-4 w-4" />
            <AlertDescription>
              <div className="space-y-2">
                <p className="font-medium">
                  You have {pendingActions.length} pending action(s)
                </p>
                {pendingActions.map((action) => (
                  <div key={action.id} className="flex items-center justify-between text-sm">
                    <span>
                      {action.type === 'stake' && 'Staking'}{' '}
                      {action.type === 'unstake' && 'Unstaking'}{' '}
                      {action.type === 'rebond' && 'Rebonding'}{' '}
                      {action.amount} to {action.validatorName} - Executes in {action.executionTime}
                    </span>
                    {action.canCancel && (
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleCancelAction(action.id)}
                      >
                        Cancel
                      </Button>
                    )}
                  </div>
                ))}
              </div>
            </AlertDescription>
          </Alert>
        )}

        {/* Main Tabs */}
        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="new">
              <Plus className="mr-2 h-4 w-4" />
              New Nomination
            </TabsTrigger>
            <TabsTrigger value="manage">
              Manage Existing ({existingNominations.filter(n => n.status !== 'unbonding').length})
            </TabsTrigger>
          </TabsList>

          {/* New Nomination Tab */}
          <TabsContent value="new" className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>Create New Nomination</CardTitle>
                <CardDescription>
                  Delegate your ETD tokens to validators and earn staking rewards
                </CardDescription>
              </CardHeader>
              <CardContent>
                <NominationForm />
              </CardContent>
            </Card>

            {/* Info Cards */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Card>
                <CardHeader>
                  <div className="flex items-center gap-2">
                    <Info className="h-5 w-5 text-blue-500" />
                    <CardTitle className="text-base">How it Works</CardTitle>
                  </div>
                </CardHeader>
                <CardContent className="space-y-2 text-sm">
                  <p>1. Select one or more validators to delegate to</p>
                  <p>2. Enter the amount of ETD you want to stake</p>
                  <p>3. Your nomination becomes active in the next era (~6 hours)</p>
                  <p>4. Earn rewards automatically every era</p>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <div className="flex items-center gap-2">
                    <AlertCircle className="h-5 w-5 text-amber-500" />
                    <CardTitle className="text-base">Important Notes</CardTitle>
                  </div>
                </CardHeader>
                <CardContent className="space-y-2 text-sm">
                  <p>Minimum stake: 10 ETD per validator</p>
                  <p>Unbonding period: 28 days</p>
                  <p>You can nominate up to 16 validators</p>
                  <p>Rewards are paid at the end of each era</p>
                </CardContent>
              </Card>
            </div>
          </TabsContent>

          {/* Manage Existing Tab */}
          <TabsContent value="manage" className="space-y-4">
            {existingNominations.filter(n => n.status !== 'unbonding').length === 0 ? (
              <Card>
                <CardContent className="py-12 text-center">
                  <p className="text-muted-foreground mb-4">
                    You don't have any active nominations yet
                  </p>
                  <Button onClick={() => setActiveTab('new')}>
                    <Plus className="mr-2 h-4 w-4" />
                    Create Your First Nomination
                  </Button>
                </CardContent>
              </Card>
            ) : (
              <>
                {existingNominations
                  .filter(n => n.status !== 'unbonding')
                  .map((nomination) => (
                    <Card key={nomination.id}>
                      <CardHeader>
                        <div className="flex justify-between items-start">
                          <div className="space-y-1">
                            <div className="flex items-center gap-2">
                              <CardTitle>{nomination.validatorName}</CardTitle>
                              <Badge variant={nomination.status === 'active' ? 'default' : 'secondary'}>
                                {nomination.status}
                              </Badge>
                            </div>
                            <CardDescription className="font-mono">
                              {nomination.validatorAddress}
                            </CardDescription>
                          </div>
                          <div className="text-right">
                            <p className="text-2xl font-bold">{nomination.stakedAmount}</p>
                            <p className="text-sm text-muted-foreground">Staked</p>
                          </div>
                        </div>
                      </CardHeader>
                      <CardContent className="space-y-6">
                        {/* Stats */}
                        <div className="grid grid-cols-3 gap-4">
                          <div>
                            <p className="text-xs text-muted-foreground">APY</p>
                            <p className="text-lg font-semibold text-green-600">
                              {nomination.apy}%
                            </p>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">Commission</p>
                            <p className="text-lg font-semibold">{nomination.commission}%</p>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">Rewards Earned</p>
                            <p className="text-lg font-semibold">{nomination.rewards}</p>
                          </div>
                        </div>

                        <Separator />

                        {/* Add Stake Section */}
                        <div className="space-y-3">
                          <Label htmlFor={`add-${nomination.id}`}>Add More Stake</Label>
                          <div className="flex gap-2">
                            <Input
                              id={`add-${nomination.id}`}
                              type="number"
                              placeholder="Amount to add"
                              value={addStakeAmount[nomination.id] || ''}
                              onChange={(e) =>
                                setAddStakeAmount({
                                  ...addStakeAmount,
                                  [nomination.id]: e.target.value
                                })
                              }
                            />
                            <Button
                              onClick={() => handleAddStake(nomination.id)}
                              disabled={!addStakeAmount[nomination.id]}
                            >
                              <Plus className="mr-2 h-4 w-4" />
                              Add Stake
                            </Button>
                          </div>
                        </div>

                        <Separator />

                        {/* Unstake Section */}
                        <div className="space-y-3">
                          <Label htmlFor={`unstake-${nomination.id}`}>Unstake Tokens</Label>
                          <div className="flex gap-2">
                            <Input
                              id={`unstake-${nomination.id}`}
                              type="number"
                              placeholder="Amount to unstake"
                              value={unstakeAmount[nomination.id] || ''}
                              onChange={(e) =>
                                setUnstakeAmount({
                                  ...unstakeAmount,
                                  [nomination.id]: e.target.value
                                })
                              }
                            />
                            <Button
                              variant="destructive"
                              onClick={() => handleUnstake(nomination.id)}
                              disabled={!unstakeAmount[nomination.id]}
                            >
                              <Minus className="mr-2 h-4 w-4" />
                              Unstake
                            </Button>
                          </div>
                          <Alert>
                            <AlertCircle className="h-4 w-4" />
                            <AlertDescription className="text-xs">
                              Unstaked tokens will be locked for 28 days before they can be withdrawn
                            </AlertDescription>
                          </Alert>
                        </div>
                      </CardContent>
                    </Card>
                  ))}
              </>
            )}

            {/* Unbonding Nominations */}
            {existingNominations.filter(n => n.status === 'unbonding').length > 0 && (
              <div className="space-y-3">
                <h2 className="text-xl font-semibold">Unbonding</h2>
                {existingNominations
                  .filter(n => n.status === 'unbonding')
                  .map((nomination) => (
                    <Card key={nomination.id}>
                      <CardContent className="py-4">
                        <div className="flex justify-between items-center">
                          <div>
                            <p className="font-semibold">{nomination.validatorName}</p>
                            <p className="text-sm text-muted-foreground">
                              {nomination.stakedAmount} unbonding
                            </p>
                          </div>
                          <div className="text-right">
                            <Badge variant="secondary">Unbonding</Badge>
                            <p className="text-sm text-muted-foreground mt-1">
                              {nomination.unbondingPeriod} remaining
                            </p>
                          </div>
                        </div>
                        <Progress value={57} className="h-2 mt-3" />
                      </CardContent>
                    </Card>
                  ))}
              </div>
            )}
          </TabsContent>
        </Tabs>

        {/* Quick Stats */}
        <Card>
          <CardHeader>
            <CardTitle>Your Staking Summary</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div className="space-y-1">
                <p className="text-sm text-muted-foreground">Active Nominations</p>
                <p className="text-2xl font-bold">
                  {existingNominations.filter(n => n.status === 'active').length}
                </p>
              </div>
              <div className="space-y-1">
                <p className="text-sm text-muted-foreground">Total Staked</p>
                <p className="text-2xl font-bold">
                  {existingNominations
                    .filter(n => n.status === 'active')
                    .reduce((acc, n) => {
                      const amount = parseFloat(n.stakedAmount.replace(/[^0-9.]/g, ''));
                      return acc + amount;
                    }, 0)
                    .toLocaleString()}{' '}
                  ETD
                </p>
              </div>
              <div className="space-y-1">
                <p className="text-sm text-muted-foreground">Total Rewards</p>
                <p className="text-2xl font-bold text-green-600">
                  {existingNominations
                    .reduce((acc, n) => {
                      const amount = parseFloat(n.rewards.replace(/[^0-9.]/g, ''));
                      return acc + amount;
                    }, 0)
                    .toFixed(2)}{' '}
                  ETD
                </p>
              </div>
              <div className="space-y-1">
                <p className="text-sm text-muted-foreground">Avg APY</p>
                <p className="text-2xl font-bold text-green-600">
                  {existingNominations
                    .filter(n => n.status === 'active')
                    .reduce((acc, n) => acc + n.apy, 0) /
                    existingNominations.filter(n => n.status === 'active').length || 0}
                  %
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
