'use client';

import React, { useState, useMemo } from 'react';
import {
  TrendingUp,
  Calculator,
  Info,
  DollarSign,
  Calendar,
  Percent
} from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Slider } from '@/components/ui/slider';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
} from '@/components/ui/tooltip';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip as RechartsTooltip,
  ResponsiveContainer,
  Legend
} from 'recharts';

interface CalculationResult {
  daily: number;
  weekly: number;
  monthly: number;
  quarterly: number;
  yearly: number;
  afterOneYear: number;
  afterTwoYears: number;
  afterFiveYears: number;
}

export function APYCalculator() {
  const [stakeAmount, setStakeAmount] = useState<string>('10000');
  const [apy, setApy] = useState([12.5]);
  const [commission, setCommission] = useState([5]);
  const [duration, setDuration] = useState<string>('1');
  const [compound, setCompound] = useState<string>('daily');
  const [additionalStake, setAdditionalStake] = useState<string>('0');
  const [additionalFrequency, setAdditionalFrequency] = useState<string>('monthly');

  const results: CalculationResult = useMemo(() => {
    const principal = parseFloat(stakeAmount) || 0;
    const rate = (apy[0] - commission[0]) / 100;
    const additional = parseFloat(additionalStake) || 0;

    // Simple calculations
    const yearlyReward = principal * rate;
    const daily = yearlyReward / 365;
    const weekly = yearlyReward / 52;
    const monthly = yearlyReward / 12;
    const quarterly = yearlyReward / 4;

    // Compound interest with additional stakes
    const calculateCompound = (years: number) => {
      let total = principal;
      const daysInPeriod = years * 365;
      const dailyRate = rate / 365;

      // Determine how many times to add additional stake
      let additionalTimes = 0;
      switch (additionalFrequency) {
        case 'daily':
          additionalTimes = daysInPeriod;
          break;
        case 'weekly':
          additionalTimes = Math.floor(daysInPeriod / 7);
          break;
        case 'monthly':
          additionalTimes = Math.floor(daysInPeriod / 30);
          break;
        case 'yearly':
          additionalTimes = years;
          break;
      }

      // Calculate compound with additional stakes
      for (let day = 0; day < daysInPeriod; day++) {
        total = total * (1 + dailyRate);

        // Add additional stake at the right frequency
        if (additional > 0) {
          if (additionalFrequency === 'daily' && day > 0) {
            total += additional;
          } else if (additionalFrequency === 'weekly' && day % 7 === 0 && day > 0) {
            total += additional;
          } else if (additionalFrequency === 'monthly' && day % 30 === 0 && day > 0) {
            total += additional;
          } else if (additionalFrequency === 'yearly' && day % 365 === 0 && day > 0) {
            total += additional;
          }
        }
      }

      return total;
    };

    const afterOneYear = calculateCompound(1);
    const afterTwoYears = calculateCompound(2);
    const afterFiveYears = calculateCompound(5);

    return {
      daily,
      weekly,
      monthly,
      quarterly,
      yearly: yearlyReward,
      afterOneYear,
      afterTwoYears,
      afterFiveYears
    };
  }, [stakeAmount, apy, commission, additionalStake, additionalFrequency]);

  // Generate chart data
  const chartData = useMemo(() => {
    const principal = parseFloat(stakeAmount) || 0;
    const rate = (apy[0] - commission[0]) / 100;
    const additional = parseFloat(additionalStake) || 0;
    const years = parseInt(duration) || 1;
    const data = [];

    let total = principal;
    const dailyRate = rate / 365;

    for (let month = 0; month <= years * 12; month++) {
      const days = month * 30;

      // Calculate compound
      for (let day = 0; day < 30 && (month === 0 ? day === 0 : true); day++) {
        total = total * (1 + dailyRate);

        // Add additional stake
        if (additional > 0 && month > 0) {
          if (additionalFrequency === 'daily' && day > 0) {
            total += additional;
          } else if (additionalFrequency === 'weekly' && day % 7 === 0 && day > 0) {
            total += additional;
          } else if (
            additionalFrequency === 'monthly' &&
            day === 0 &&
            month > 0
          ) {
            total += additional;
          }
        }
      }

      data.push({
        month: `M${month}`,
        principal:
          principal + (additionalFrequency === 'monthly' ? additional * month : 0),
        total: total,
        earnings: total - principal - (additionalFrequency === 'monthly' ? additional * month : 0)
      });
    }

    return data;
  }, [stakeAmount, apy, commission, duration, additionalStake, additionalFrequency]);

  const effectiveAPY = apy[0] - commission[0];

  return (
    <div className="space-y-6">
      {/* Input Section */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Calculator className="h-5 w-5" />
            Staking Calculator
          </CardTitle>
          <CardDescription>
            Calculate your potential staking rewards with custom parameters
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Stake Amount */}
          <div className="space-y-2">
            <Label htmlFor="stake-amount">Stake Amount (ETD)</Label>
            <Input
              id="stake-amount"
              type="number"
              value={stakeAmount}
              onChange={(e) => setStakeAmount(e.target.value)}
              placeholder="Enter amount"
            />
          </div>

          {/* APY Slider */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>Expected APY: {apy[0]}%</Label>
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger>
                    <Info className="h-4 w-4 text-muted-foreground" />
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>Average validator APY ranges from 10-15%</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>
            <Slider
              value={apy}
              onValueChange={setApy}
              max={20}
              min={5}
              step={0.1}
              className="py-4"
            />
          </div>

          {/* Commission Slider */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>Validator Commission: {commission[0]}%</Label>
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger>
                    <Info className="h-4 w-4 text-muted-foreground" />
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>Typical validator commission is 5-10%</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>
            <Slider
              value={commission}
              onValueChange={setCommission}
              max={20}
              min={0}
              step={0.5}
              className="py-4"
            />
          </div>

          {/* Duration */}
          <div className="space-y-2">
            <Label htmlFor="duration">Duration (Years)</Label>
            <Select value={duration} onValueChange={setDuration}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="1">1 Year</SelectItem>
                <SelectItem value="2">2 Years</SelectItem>
                <SelectItem value="3">3 Years</SelectItem>
                <SelectItem value="5">5 Years</SelectItem>
                <SelectItem value="10">10 Years</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {/* Additional Stake */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="additional">Additional Stake (ETD)</Label>
              <Input
                id="additional"
                type="number"
                value={additionalStake}
                onChange={(e) => setAdditionalStake(e.target.value)}
                placeholder="0"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="frequency">Frequency</Label>
              <Select value={additionalFrequency} onValueChange={setAdditionalFrequency}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="daily">Daily</SelectItem>
                  <SelectItem value="weekly">Weekly</SelectItem>
                  <SelectItem value="monthly">Monthly</SelectItem>
                  <SelectItem value="yearly">Yearly</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Effective APY */}
      <Card className="border-green-200 bg-green-50 dark:bg-green-950/20 dark:border-green-900">
        <CardContent className="pt-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Percent className="h-5 w-5 text-green-600" />
              <span className="text-sm font-medium">Effective APY (after commission)</span>
            </div>
            <span className="text-2xl font-bold text-green-600">{effectiveAPY.toFixed(2)}%</span>
          </div>
        </CardContent>
      </Card>

      {/* Results Tabs */}
      <Tabs defaultValue="summary" className="w-full">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="summary">Summary</TabsTrigger>
          <TabsTrigger value="projection">Projection</TabsTrigger>
        </TabsList>

        <TabsContent value="summary" className="space-y-4">
          {/* Quick Returns */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <DollarSign className="h-5 w-5" />
                Estimated Returns
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                <div className="p-3 border rounded-lg">
                  <p className="text-xs text-muted-foreground mb-1">Daily</p>
                  <p className="text-lg font-bold text-green-600">
                    {results.daily.toFixed(2)} ETD
                  </p>
                </div>
                <div className="p-3 border rounded-lg">
                  <p className="text-xs text-muted-foreground mb-1">Weekly</p>
                  <p className="text-lg font-bold text-green-600">
                    {results.weekly.toFixed(2)} ETD
                  </p>
                </div>
                <div className="p-3 border rounded-lg">
                  <p className="text-xs text-muted-foreground mb-1">Monthly</p>
                  <p className="text-lg font-bold text-green-600">
                    {results.monthly.toFixed(2)} ETD
                  </p>
                </div>
                <div className="p-3 border rounded-lg">
                  <p className="text-xs text-muted-foreground mb-1">Quarterly</p>
                  <p className="text-lg font-bold text-green-600">
                    {results.quarterly.toFixed(2)} ETD
                  </p>
                </div>
                <div className="p-3 border rounded-lg">
                  <p className="text-xs text-muted-foreground mb-1">Yearly</p>
                  <p className="text-lg font-bold text-green-600">
                    {results.yearly.toFixed(2)} ETD
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Long-term Projection */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Calendar className="h-5 w-5" />
                Long-term Growth (with compounding)
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex items-center justify-between p-4 border rounded-lg">
                  <div>
                    <p className="font-semibold">After 1 Year</p>
                    <p className="text-sm text-muted-foreground">
                      Initial: {parseFloat(stakeAmount).toLocaleString()} ETD
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="text-2xl font-bold text-green-600">
                      {results.afterOneYear.toLocaleString(undefined, {
                        maximumFractionDigits: 2
                      })}{' '}
                      ETD
                    </p>
                    <p className="text-sm text-green-600">
                      +
                      {(results.afterOneYear - parseFloat(stakeAmount)).toLocaleString(
                        undefined,
                        { maximumFractionDigits: 2 }
                      )}{' '}
                      ETD
                    </p>
                  </div>
                </div>

                <div className="flex items-center justify-between p-4 border rounded-lg">
                  <div>
                    <p className="font-semibold">After 2 Years</p>
                    <p className="text-sm text-muted-foreground">
                      Initial: {parseFloat(stakeAmount).toLocaleString()} ETD
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="text-2xl font-bold text-green-600">
                      {results.afterTwoYears.toLocaleString(undefined, {
                        maximumFractionDigits: 2
                      })}{' '}
                      ETD
                    </p>
                    <p className="text-sm text-green-600">
                      +
                      {(results.afterTwoYears - parseFloat(stakeAmount)).toLocaleString(
                        undefined,
                        { maximumFractionDigits: 2 }
                      )}{' '}
                      ETD
                    </p>
                  </div>
                </div>

                <div className="flex items-center justify-between p-4 border rounded-lg">
                  <div>
                    <p className="font-semibold">After 5 Years</p>
                    <p className="text-sm text-muted-foreground">
                      Initial: {parseFloat(stakeAmount).toLocaleString()} ETD
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="text-2xl font-bold text-green-600">
                      {results.afterFiveYears.toLocaleString(undefined, {
                        maximumFractionDigits: 2
                      })}{' '}
                      ETD
                    </p>
                    <p className="text-sm text-green-600">
                      +
                      {(results.afterFiveYears - parseFloat(stakeAmount)).toLocaleString(
                        undefined,
                        { maximumFractionDigits: 2 }
                      )}{' '}
                      ETD
                    </p>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="projection" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Growth Projection</CardTitle>
              <CardDescription>
                Your stake growth over {duration} year(s) with{' '}
                {parseFloat(additionalStake) > 0
                  ? `${additionalFrequency} additional stakes`
                  : 'no additional stakes'}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={400}>
                <AreaChart data={chartData}>
                  <defs>
                    <linearGradient id="colorTotal" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#10b981" stopOpacity={0.3} />
                      <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                    </linearGradient>
                    <linearGradient id="colorEarnings" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3} />
                      <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
                    </linearGradient>
                  </defs>
                  <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                  <XAxis
                    dataKey="month"
                    className="text-xs"
                    tick={{ fill: 'hsl(var(--muted-foreground))' }}
                  />
                  <YAxis
                    className="text-xs"
                    tick={{ fill: 'hsl(var(--muted-foreground))' }}
                    label={{ value: 'ETD', angle: -90, position: 'insideLeft' }}
                  />
                  <RechartsTooltip />
                  <Legend />
                  <Area
                    type="monotone"
                    dataKey="total"
                    stroke="#10b981"
                    strokeWidth={2}
                    fill="url(#colorTotal)"
                    name="Total Value"
                  />
                  <Area
                    type="monotone"
                    dataKey="earnings"
                    stroke="#3b82f6"
                    strokeWidth={2}
                    fill="url(#colorEarnings)"
                    name="Earnings"
                  />
                </AreaChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
