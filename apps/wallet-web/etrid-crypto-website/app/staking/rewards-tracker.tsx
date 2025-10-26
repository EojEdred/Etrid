'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import {
  ArrowLeft,
  Download,
  Calendar,
  TrendingUp,
  Award,
  DollarSign,
  Filter,
  ChevronDown
} from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select';
import { RewardChart } from '@/components/staking/reward-chart';

interface RewardEntry {
  id: string;
  date: string;
  era: number;
  validatorName: string;
  validatorAddress: string;
  amount: string;
  apy: number;
  type: 'staking' | 'commission';
  txHash: string;
}

interface RewardSummary {
  period: string;
  totalRewards: string;
  avgAPY: number;
  bestValidator: string;
  rewardCount: number;
}

export default function RewardsTracker() {
  const [timeframe, setTimeframe] = useState('30d');
  const [selectedValidator, setSelectedValidator] = useState('all');

  const [rewardHistory] = useState<RewardEntry[]>([
    {
      id: '1',
      date: '2025-10-21',
      era: 1245,
      validatorName: 'Validator Alpha',
      validatorAddress: '0x1234...5678',
      amount: '8.42 ETD',
      apy: 13.2,
      type: 'staking',
      txHash: '0xabc...def'
    },
    {
      id: '2',
      date: '2025-10-21',
      era: 1245,
      validatorName: 'Beta Staking',
      validatorAddress: '0x2345...6789',
      amount: '6.54 ETD',
      apy: 12.8,
      type: 'staking',
      txHash: '0xbcd...efg'
    },
    {
      id: '3',
      date: '2025-10-21',
      era: 1245,
      validatorName: 'Epsilon Staking',
      validatorAddress: '0x5678...9012',
      amount: '7.23 ETD',
      apy: 12.4,
      type: 'staking',
      txHash: '0xcde...fgh'
    },
    {
      id: '4',
      date: '2025-10-20',
      era: 1244,
      validatorName: 'Validator Alpha',
      validatorAddress: '0x1234...5678',
      amount: '8.38 ETD',
      apy: 13.1,
      type: 'staking',
      txHash: '0xdef...ghi'
    },
    {
      id: '5',
      date: '2025-10-20',
      era: 1244,
      validatorName: 'Beta Staking',
      validatorAddress: '0x2345...6789',
      amount: '6.51 ETD',
      apy: 12.7,
      type: 'staking',
      txHash: '0xefg...hij'
    },
    {
      id: '6',
      date: '2025-10-19',
      era: 1243,
      validatorName: 'Validator Alpha',
      validatorAddress: '0x1234...5678',
      amount: '8.41 ETD',
      apy: 13.2,
      type: 'staking',
      txHash: '0xfgh...ijk'
    },
    {
      id: '7',
      date: '2025-10-19',
      era: 1243,
      validatorName: 'Epsilon Staking',
      validatorAddress: '0x5678...9012',
      amount: '7.19 ETD',
      apy: 12.3,
      type: 'staking',
      txHash: '0xghi...jkl'
    },
    {
      id: '8',
      date: '2025-10-18',
      era: 1242,
      validatorName: 'Beta Staking',
      validatorAddress: '0x2345...6789',
      amount: '6.49 ETD',
      apy: 12.6,
      type: 'staking',
      txHash: '0xhij...klm'
    }
  ]);

  const [summaries] = useState<{ [key: string]: RewardSummary }>({
    '7d': {
      period: 'Last 7 Days',
      totalRewards: '45.23 ETD',
      avgAPY: 12.8,
      bestValidator: 'Validator Alpha',
      rewardCount: 21
    },
    '30d': {
      period: 'Last 30 Days',
      totalRewards: '187.64 ETD',
      avgAPY: 12.9,
      bestValidator: 'Validator Alpha',
      rewardCount: 90
    },
    '90d': {
      period: 'Last 90 Days',
      totalRewards: '562.91 ETD',
      avgAPY: 13.1,
      bestValidator: 'Validator Alpha',
      rewardCount: 270
    },
    all: {
      period: 'All Time',
      totalRewards: '1,247.83 ETD',
      avgAPY: 12.7,
      bestValidator: 'Validator Alpha',
      rewardCount: 378
    }
  });

  const currentSummary = summaries[timeframe];

  const validators = Array.from(
    new Set(rewardHistory.map((r) => r.validatorName))
  );

  const filteredRewards = rewardHistory.filter((reward) => {
    if (selectedValidator === 'all') return true;
    return reward.validatorName === selectedValidator;
  });

  const handleExport = () => {
    console.log('Exporting rewards data...');
    // Implement CSV export logic
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-background to-muted/20 p-4 md:p-8">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center gap-4">
          <Link href="/staking/nominator-dashboard">
            <Button variant="ghost" size="icon">
              <ArrowLeft className="h-5 w-5" />
            </Button>
          </Link>
          <div className="flex-1">
            <h1 className="text-4xl font-bold tracking-tight">Rewards Tracker</h1>
            <p className="text-muted-foreground mt-2">
              Monitor your staking rewards and performance analytics
            </p>
          </div>
          <Button variant="outline" onClick={handleExport}>
            <Download className="mr-2 h-4 w-4" />
            Export
          </Button>
        </div>

        {/* Timeframe Selector */}
        <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
          <Tabs value={timeframe} onValueChange={setTimeframe} className="w-full sm:w-auto">
            <TabsList>
              <TabsTrigger value="7d">7 Days</TabsTrigger>
              <TabsTrigger value="30d">30 Days</TabsTrigger>
              <TabsTrigger value="90d">90 Days</TabsTrigger>
              <TabsTrigger value="all">All Time</TabsTrigger>
            </TabsList>
          </Tabs>

          <Select value={selectedValidator} onValueChange={setSelectedValidator}>
            <SelectTrigger className="w-full sm:w-[200px]">
              <SelectValue placeholder="All Validators" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Validators</SelectItem>
              {validators.map((validator) => (
                <SelectItem key={validator} value={validator}>
                  {validator}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Summary Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Total Rewards</CardTitle>
              <Award className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{currentSummary.totalRewards}</div>
              <p className="text-xs text-muted-foreground mt-1">
                {currentSummary.period}
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Average APY</CardTitle>
              <TrendingUp className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-green-600">
                {currentSummary.avgAPY}%
              </div>
              <p className="text-xs text-muted-foreground mt-1">
                Weighted average
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Best Performer</CardTitle>
              <Award className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-lg font-bold truncate">
                {currentSummary.bestValidator}
              </div>
              <p className="text-xs text-muted-foreground mt-1">
                Top validator
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Reward Events</CardTitle>
              <Calendar className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{currentSummary.rewardCount}</div>
              <p className="text-xs text-muted-foreground mt-1">
                Total payouts
              </p>
            </CardContent>
          </Card>
        </div>

        {/* Reward Chart */}
        <Card>
          <CardHeader>
            <CardTitle>Rewards Over Time</CardTitle>
            <CardDescription>
              Daily reward accumulation and APY trends
            </CardDescription>
          </CardHeader>
          <CardContent>
            <RewardChart timeframe={timeframe} validatorFilter={selectedValidator} />
          </CardContent>
        </Card>

        {/* Validator Performance Breakdown */}
        <Card>
          <CardHeader>
            <CardTitle>Validator Performance</CardTitle>
            <CardDescription>
              Rewards earned per validator
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {validators.map((validator) => {
                const validatorRewards = rewardHistory.filter(
                  (r) => r.validatorName === validator
                );
                const totalRewards = validatorRewards.reduce((acc, r) => {
                  const amount = parseFloat(r.amount.replace(/[^0-9.]/g, ''));
                  return acc + amount;
                }, 0);
                const avgAPY =
                  validatorRewards.reduce((acc, r) => acc + r.apy, 0) /
                  validatorRewards.length;

                return (
                  <div
                    key={validator}
                    className="flex items-center justify-between p-4 border rounded-lg hover:bg-accent transition-colors"
                  >
                    <div className="space-y-1">
                      <p className="font-semibold">{validator}</p>
                      <p className="text-sm text-muted-foreground">
                        {validatorRewards.length} rewards received
                      </p>
                    </div>
                    <div className="text-right space-y-1">
                      <p className="text-xl font-bold">{totalRewards.toFixed(2)} ETD</p>
                      <p className="text-sm text-green-600 font-medium">
                        {avgAPY.toFixed(1)}% APY
                      </p>
                    </div>
                  </div>
                );
              })}
            </div>
          </CardContent>
        </Card>

        {/* Reward History Table */}
        <Card>
          <CardHeader>
            <div className="flex justify-between items-center">
              <div>
                <CardTitle>Reward History</CardTitle>
                <CardDescription>
                  Detailed transaction log of all staking rewards
                </CardDescription>
              </div>
              <Badge variant="secondary">
                {filteredRewards.length} transactions
              </Badge>
            </div>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {filteredRewards.length === 0 ? (
                <p className="text-center text-muted-foreground py-8">
                  No rewards found for the selected filters
                </p>
              ) : (
                filteredRewards.map((reward) => (
                  <div
                    key={reward.id}
                    className="flex flex-col md:flex-row md:items-center justify-between p-4 border rounded-lg hover:bg-accent transition-colors gap-3"
                  >
                    <div className="space-y-1 flex-1">
                      <div className="flex items-center gap-2">
                        <p className="font-semibold">{reward.validatorName}</p>
                        <Badge variant="outline" className="text-xs">
                          Era {reward.era}
                        </Badge>
                      </div>
                      <p className="text-sm text-muted-foreground font-mono">
                        {reward.validatorAddress}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        {new Date(reward.date).toLocaleDateString('en-US', {
                          year: 'numeric',
                          month: 'long',
                          day: 'numeric'
                        })}
                      </p>
                    </div>

                    <div className="flex items-center gap-6">
                      <div className="text-right">
                        <p className="text-xs text-muted-foreground">Amount</p>
                        <p className="text-lg font-bold text-green-600">
                          +{reward.amount}
                        </p>
                      </div>
                      <div className="text-right">
                        <p className="text-xs text-muted-foreground">APY</p>
                        <p className="text-lg font-semibold">{reward.apy}%</p>
                      </div>
                      <Link
                        href={`https://explorer.etrid.network/tx/${reward.txHash}`}
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <Button variant="ghost" size="sm">
                          View Tx
                        </Button>
                      </Link>
                    </div>
                  </div>
                ))
              )}
            </div>
          </CardContent>
        </Card>

        {/* Projected Earnings */}
        <Card>
          <CardHeader>
            <CardTitle>Projected Earnings</CardTitle>
            <CardDescription>
              Estimated future rewards based on current performance
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="p-4 border rounded-lg">
                <p className="text-sm text-muted-foreground mb-2">Next 7 Days</p>
                <p className="text-2xl font-bold text-green-600">~45.23 ETD</p>
                <p className="text-xs text-muted-foreground mt-1">
                  Based on {currentSummary.avgAPY}% APY
                </p>
              </div>
              <div className="p-4 border rounded-lg">
                <p className="text-sm text-muted-foreground mb-2">Next 30 Days</p>
                <p className="text-2xl font-bold text-green-600">~193.84 ETD</p>
                <p className="text-xs text-muted-foreground mt-1">
                  Based on {currentSummary.avgAPY}% APY
                </p>
              </div>
              <div className="p-4 border rounded-lg">
                <p className="text-sm text-muted-foreground mb-2">Next 365 Days</p>
                <p className="text-2xl font-bold text-green-600">~2,364.54 ETD</p>
                <p className="text-xs text-muted-foreground mt-1">
                  Based on {currentSummary.avgAPY}% APY
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
