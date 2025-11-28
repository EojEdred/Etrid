'use client';

import React, { useMemo } from 'react';
import {
  LineChart,
  Line,
  AreaChart,
  Area,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  ComposedChart
} from 'recharts';
import { Card } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

interface RewardChartProps {
  timeframe: string;
  validatorFilter?: string;
}

export function RewardChart({ timeframe, validatorFilter = 'all' }: RewardChartProps) {
  // Generate mock data based on timeframe
  const chartData = useMemo(() => {
    const generateData = (days: number) => {
      const data = [];
      const baseReward = 6.5;
      const today = new Date();

      for (let i = days - 1; i >= 0; i--) {
        const date = new Date(today);
        date.setDate(date.getDate() - i);

        const variance = Math.random() * 2 - 1; // -1 to 1
        const validatorAlpha = baseReward * 1.3 + variance;
        const betaStaking = baseReward * 1.0 + variance * 0.8;
        const epsilonStaking = baseReward * 1.1 + variance * 0.9;

        const totalReward =
          validatorFilter === 'all'
            ? validatorAlpha + betaStaking + epsilonStaking
            : validatorFilter === 'Validator Alpha'
            ? validatorAlpha
            : validatorFilter === 'Beta Staking'
            ? betaStaking
            : epsilonStaking;

        data.push({
          date: date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
          fullDate: date.toISOString().split('T')[0],
          'Validator Alpha': Number(validatorAlpha.toFixed(2)),
          'Beta Staking': Number(betaStaking.toFixed(2)),
          'Epsilon Staking': Number(epsilonStaking.toFixed(2)),
          Total: Number(totalReward.toFixed(2)),
          APY: Number((12.5 + Math.random() * 1.5).toFixed(2)),
          cumulativeRewards: 0 // Will be calculated below
        });
      }

      // Calculate cumulative rewards
      let cumulative = 0;
      data.forEach((item) => {
        cumulative += item.Total;
        item.cumulativeRewards = Number(cumulative.toFixed(2));
      });

      return data;
    };

    switch (timeframe) {
      case '7d':
        return generateData(7);
      case '30d':
        return generateData(30);
      case '90d':
        return generateData(90);
      case 'all':
        return generateData(365);
      default:
        return generateData(30);
    }
  }, [timeframe, validatorFilter]);

  const CustomTooltip = ({ active, payload, label }: any) => {
    if (active && payload && payload.length) {
      return (
        <Card className="p-3 shadow-lg">
          <p className="font-semibold mb-2">{label}</p>
          {payload.map((entry: any, index: number) => (
            <div key={index} className="flex items-center justify-between gap-4 text-sm">
              <span style={{ color: entry.color }}>{entry.name}:</span>
              <span className="font-semibold">
                {typeof entry.value === 'number' && entry.value > 100
                  ? entry.value.toFixed(2)
                  : entry.value}
                {entry.name === 'APY' ? '%' : ' ETD'}
              </span>
            </div>
          ))}
        </Card>
      );
    }
    return null;
  };

  return (
    <Tabs defaultValue="daily" className="w-full">
      <TabsList className="grid w-full grid-cols-4">
        <TabsTrigger value="daily">Daily Rewards</TabsTrigger>
        <TabsTrigger value="cumulative">Cumulative</TabsTrigger>
        <TabsTrigger value="apy">APY Trend</TabsTrigger>
        <TabsTrigger value="breakdown">Breakdown</TabsTrigger>
      </TabsList>

      {/* Daily Rewards Chart */}
      <TabsContent value="daily" className="mt-6">
        <ResponsiveContainer width="100%" height={400}>
          <AreaChart data={chartData}>
            <defs>
              <linearGradient id="colorTotal" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#10b981" stopOpacity={0.3} />
                <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
            <XAxis
              dataKey="date"
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
            />
            <YAxis
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
              label={{ value: 'ETD', angle: -90, position: 'insideLeft' }}
            />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Area
              type="monotone"
              dataKey="Total"
              stroke="#10b981"
              strokeWidth={2}
              fill="url(#colorTotal)"
              name="Daily Rewards"
            />
          </AreaChart>
        </ResponsiveContainer>
      </TabsContent>

      {/* Cumulative Rewards Chart */}
      <TabsContent value="cumulative" className="mt-6">
        <ResponsiveContainer width="100%" height={400}>
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
            <XAxis
              dataKey="date"
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
            />
            <YAxis
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
              label={{ value: 'ETD', angle: -90, position: 'insideLeft' }}
            />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Line
              type="monotone"
              dataKey="cumulativeRewards"
              stroke="#8b5cf6"
              strokeWidth={3}
              dot={false}
              name="Cumulative Rewards"
            />
          </LineChart>
        </ResponsiveContainer>
      </TabsContent>

      {/* APY Trend Chart */}
      <TabsContent value="apy" className="mt-6">
        <ResponsiveContainer width="100%" height={400}>
          <ComposedChart data={chartData}>
            <defs>
              <linearGradient id="colorAPY" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3} />
                <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
            <XAxis
              dataKey="date"
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
            />
            <YAxis
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
              label={{ value: 'APY %', angle: -90, position: 'insideLeft' }}
              domain={[10, 15]}
            />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Area
              type="monotone"
              dataKey="APY"
              stroke="#3b82f6"
              strokeWidth={2}
              fill="url(#colorAPY)"
              name="APY"
            />
          </ComposedChart>
        </ResponsiveContainer>
      </TabsContent>

      {/* Validator Breakdown Chart */}
      <TabsContent value="breakdown" className="mt-6">
        <ResponsiveContainer width="100%" height={400}>
          <BarChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
            <XAxis
              dataKey="date"
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
            />
            <YAxis
              className="text-xs"
              tick={{ fill: 'hsl(var(--muted-foreground))' }}
              label={{ value: 'ETD', angle: -90, position: 'insideLeft' }}
            />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            {validatorFilter === 'all' && (
              <>
                <Bar
                  dataKey="Validator Alpha"
                  stackId="rewards"
                  fill="#10b981"
                  name="Validator Alpha"
                />
                <Bar
                  dataKey="Beta Staking"
                  stackId="rewards"
                  fill="#3b82f6"
                  name="Beta Staking"
                />
                <Bar
                  dataKey="Epsilon Staking"
                  stackId="rewards"
                  fill="#8b5cf6"
                  name="Epsilon Staking"
                />
              </>
            )}
            {validatorFilter !== 'all' && (
              <Bar dataKey="Total" fill="#10b981" name={validatorFilter} />
            )}
          </BarChart>
        </ResponsiveContainer>
      </TabsContent>
    </Tabs>
  );
}
