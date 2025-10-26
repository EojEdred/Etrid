'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import {
  TrendingUp,
  Users,
  Award,
  AlertCircle,
  ArrowUpRight,
  ChevronRight,
  Wallet,
  Info
} from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

interface NominationStats {
  totalStaked: string;
  activeNominations: number;
  totalRewards: string;
  estimatedAPY: number;
  pendingRewards: string;
  nextRewardIn: string;
}

interface ActiveNomination {
  validatorId: string;
  validatorName: string;
  stakedAmount: string;
  apy: number;
  commission: number;
  status: 'active' | 'waiting' | 'inactive';
  rewards: string;
  uptime: number;
}

interface PerformanceAlert {
  id: string;
  type: 'warning' | 'info' | 'critical';
  validatorName: string;
  message: string;
  timestamp: string;
}

export default function NominatorDashboard() {
  const [stats] = useState<NominationStats>({
    totalStaked: '10,000 ETD',
    activeNominations: 5,
    totalRewards: '247.83 ETD',
    estimatedAPY: 12.5,
    pendingRewards: '18.42 ETD',
    nextRewardIn: '6h 24m'
  });

  const [nominations] = useState<ActiveNomination[]>([
    {
      validatorId: '0x1234...5678',
      validatorName: 'Validator Alpha',
      stakedAmount: '2,500 ETD',
      apy: 13.2,
      commission: 5,
      status: 'active',
      rewards: '82.15 ETD',
      uptime: 99.8
    },
    {
      validatorId: '0x2345...6789',
      validatorName: 'Beta Staking',
      stakedAmount: '2,000 ETD',
      apy: 12.8,
      commission: 7,
      status: 'active',
      rewards: '64.32 ETD',
      uptime: 99.5
    },
    {
      validatorId: '0x3456...7890',
      validatorName: 'Gamma Node',
      stakedAmount: '2,000 ETD',
      apy: 12.1,
      commission: 8,
      status: 'active',
      rewards: '60.24 ETD',
      uptime: 98.9
    },
    {
      validatorId: '0x4567...8901',
      validatorName: 'Delta Validator',
      stakedAmount: '1,500 ETD',
      apy: 11.9,
      commission: 6,
      status: 'waiting',
      rewards: '22.56 ETD',
      uptime: 99.2
    },
    {
      validatorId: '0x5678...9012',
      validatorName: 'Epsilon Staking',
      stakedAmount: '2,000 ETD',
      apy: 12.4,
      commission: 5,
      status: 'active',
      rewards: '18.56 ETD',
      uptime: 99.7
    }
  ]);

  const [alerts] = useState<PerformanceAlert[]>([
    {
      id: '1',
      type: 'warning',
      validatorName: 'Gamma Node',
      message: 'Uptime dropped below 99% in the last epoch',
      timestamp: '2 hours ago'
    },
    {
      id: '2',
      type: 'info',
      validatorName: 'Delta Validator',
      message: 'Nomination is in waiting queue, will be active in next era',
      timestamp: '5 hours ago'
    }
  ]);

  const getStatusBadge = (status: ActiveNomination['status']) => {
    const variants = {
      active: 'default',
      waiting: 'secondary',
      inactive: 'destructive'
    };
    return (
      <Badge variant={variants[status] as any}>
        {status.charAt(0).toUpperCase() + status.slice(1)}
      </Badge>
    );
  };

  const getAlertVariant = (type: PerformanceAlert['type']) => {
    return type === 'critical' ? 'destructive' : 'default';
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-background to-muted/20 p-4 md:p-8">
      <div className="max-w-7xl mx-auto space-y-8">
        {/* Header */}
        <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
          <div>
            <h1 className="text-4xl font-bold tracking-tight">Nominator Dashboard</h1>
            <p className="text-muted-foreground mt-2">
              Manage your delegated staking and track rewards
            </p>
          </div>
          <div className="flex gap-3">
            <Link href="/staking/validator-browser">
              <Button variant="outline">
                <Users className="mr-2 h-4 w-4" />
                Browse Validators
              </Button>
            </Link>
            <Link href="/staking/nomination-manager">
              <Button>
                <Wallet className="mr-2 h-4 w-4" />
                New Nomination
              </Button>
            </Link>
          </div>
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Total Staked</CardTitle>
              <Wallet className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.totalStaked}</div>
              <p className="text-xs text-muted-foreground mt-1">
                Across {stats.activeNominations} validators
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Total Rewards</CardTitle>
              <Award className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.totalRewards}</div>
              <div className="flex items-center text-xs text-green-600 mt-1">
                <ArrowUpRight className="h-3 w-3 mr-1" />
                <span>+{stats.pendingRewards} pending</span>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Estimated APY</CardTitle>
              <TrendingUp className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.estimatedAPY}%</div>
              <p className="text-xs text-muted-foreground mt-1">
                Weighted average
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium">Next Reward</CardTitle>
              <Users className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.nextRewardIn}</div>
              <p className="text-xs text-muted-foreground mt-1">
                Era ends in 6h 24m
              </p>
            </CardContent>
          </Card>
        </div>

        {/* Performance Alerts */}
        {alerts.length > 0 && (
          <div className="space-y-3">
            <h2 className="text-xl font-semibold">Performance Alerts</h2>
            {alerts.map((alert) => (
              <Alert key={alert.id} variant={getAlertVariant(alert.type)}>
                <AlertCircle className="h-4 w-4" />
                <AlertDescription>
                  <div className="flex justify-between items-start">
                    <div>
                      <span className="font-medium">{alert.validatorName}:</span>{' '}
                      {alert.message}
                    </div>
                    <span className="text-xs text-muted-foreground">
                      {alert.timestamp}
                    </span>
                  </div>
                </AlertDescription>
              </Alert>
            ))}
          </div>
        )}

        {/* Active Nominations */}
        <Card>
          <CardHeader>
            <div className="flex justify-between items-center">
              <div>
                <CardTitle>Active Nominations</CardTitle>
                <CardDescription>
                  Your current validator delegations and performance
                </CardDescription>
              </div>
              <Link href="/staking/nomination-manager">
                <Button variant="ghost" size="sm">
                  Manage All
                  <ChevronRight className="ml-2 h-4 w-4" />
                </Button>
              </Link>
            </div>
          </CardHeader>
          <CardContent>
            <Tabs defaultValue="active" className="w-full">
              <TabsList className="grid w-full grid-cols-3">
                <TabsTrigger value="active">
                  Active ({nominations.filter(n => n.status === 'active').length})
                </TabsTrigger>
                <TabsTrigger value="waiting">
                  Waiting ({nominations.filter(n => n.status === 'waiting').length})
                </TabsTrigger>
                <TabsTrigger value="all">All ({nominations.length})</TabsTrigger>
              </TabsList>

              <TabsContent value="active" className="space-y-4 mt-6">
                {nominations
                  .filter(n => n.status === 'active')
                  .map((nomination) => (
                    <NominationCard key={nomination.validatorId} nomination={nomination} />
                  ))}
              </TabsContent>

              <TabsContent value="waiting" className="space-y-4 mt-6">
                {nominations
                  .filter(n => n.status === 'waiting')
                  .map((nomination) => (
                    <NominationCard key={nomination.validatorId} nomination={nomination} />
                  ))}
              </TabsContent>

              <TabsContent value="all" className="space-y-4 mt-6">
                {nominations.map((nomination) => (
                  <NominationCard key={nomination.validatorId} nomination={nomination} />
                ))}
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>

        {/* Quick Actions */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <Link href="/staking/validator-browser" className="block">
            <Card className="hover:bg-accent transition-colors cursor-pointer h-full">
              <CardHeader>
                <Users className="h-8 w-8 mb-2 text-primary" />
                <CardTitle>Discover Validators</CardTitle>
                <CardDescription>
                  Browse and compare top-performing validators
                </CardDescription>
              </CardHeader>
            </Card>
          </Link>

          <Link href="/staking/rewards-tracker" className="block">
            <Card className="hover:bg-accent transition-colors cursor-pointer h-full">
              <CardHeader>
                <Award className="h-8 w-8 mb-2 text-primary" />
                <CardTitle>Track Rewards</CardTitle>
                <CardDescription>
                  View detailed reward history and analytics
                </CardDescription>
              </CardHeader>
            </Card>
          </Link>

          <Link href="/staking/apy-calculator" className="block">
            <Card className="hover:bg-accent transition-colors cursor-pointer h-full">
              <CardHeader>
                <TrendingUp className="h-8 w-8 mb-2 text-primary" />
                <CardTitle>Calculate APY</CardTitle>
                <CardDescription>
                  Estimate potential returns on your stake
                </CardDescription>
              </CardHeader>
            </Card>
          </Link>
        </div>
      </div>
    </div>
  );
}

function NominationCard({ nomination }: { nomination: ActiveNomination }) {
  return (
    <div className="border rounded-lg p-4 space-y-4 hover:bg-accent/50 transition-colors">
      <div className="flex justify-between items-start">
        <div className="space-y-1">
          <div className="flex items-center gap-2">
            <h3 className="font-semibold text-lg">{nomination.validatorName}</h3>
            {nomination.status && (
              <Badge variant={nomination.status === 'active' ? 'default' : 'secondary'}>
                {nomination.status}
              </Badge>
            )}
          </div>
          <p className="text-sm text-muted-foreground font-mono">
            {nomination.validatorId}
          </p>
        </div>
        <div className="text-right">
          <div className="text-2xl font-bold">{nomination.stakedAmount}</div>
          <p className="text-sm text-muted-foreground">Staked</p>
        </div>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div>
          <p className="text-xs text-muted-foreground">APY</p>
          <p className="text-lg font-semibold text-green-600">{nomination.apy}%</p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">Commission</p>
          <p className="text-lg font-semibold">{nomination.commission}%</p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">Rewards Earned</p>
          <p className="text-lg font-semibold">{nomination.rewards}</p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">Uptime</p>
          <p className="text-lg font-semibold">{nomination.uptime}%</p>
        </div>
      </div>

      <div className="space-y-2">
        <div className="flex justify-between text-xs">
          <span className="text-muted-foreground">Performance</span>
          <span className="font-medium">{nomination.uptime}%</span>
        </div>
        <Progress value={nomination.uptime} className="h-2" />
      </div>

      <div className="flex gap-2 pt-2">
        <Button variant="outline" size="sm" className="flex-1">
          Add Stake
        </Button>
        <Button variant="outline" size="sm" className="flex-1">
          Unstake
        </Button>
        <Button variant="ghost" size="sm">
          <Info className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}
