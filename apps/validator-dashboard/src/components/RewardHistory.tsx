import React, { useState } from 'react';
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
  ResponsiveContainer,
  Legend,
} from 'recharts';
import { TrendingUp, Calendar, DollarSign } from 'lucide-react';
import type { Reward } from '@/types';
import { formatTokenAmount, formatDateTime, formatNumber } from '@/utils/format';

interface RewardHistoryProps {
  rewards: Reward[];
  isLoading?: boolean;
}

type ChartType = 'line' | 'area' | 'bar';
type TimeRange = '7d' | '30d' | '90d' | 'all';

export default function RewardHistory({ rewards, isLoading = false }: RewardHistoryProps) {
  const [chartType, setChartType] = useState<ChartType>('area');
  const [timeRange, setTimeRange] = useState<TimeRange>('30d');

  const filteredRewards = React.useMemo(() => {
    const now = Date.now();
    let cutoff = 0;

    switch (timeRange) {
      case '7d':
        cutoff = now - 7 * 24 * 60 * 60 * 1000;
        break;
      case '30d':
        cutoff = now - 30 * 24 * 60 * 60 * 1000;
        break;
      case '90d':
        cutoff = now - 90 * 24 * 60 * 60 * 1000;
        break;
      default:
        cutoff = 0;
    }

    return rewards
      .filter((r) => r.timestamp >= cutoff)
      .reverse()
      .map((r) => ({
        era: r.era,
        amount: Number(r.amount) / 1e18,
        timestamp: r.timestamp,
        date: new Date(r.timestamp).toLocaleDateString('en-US', {
          month: 'short',
          day: 'numeric',
        }),
      }));
  }, [rewards, timeRange]);

  const totalRewards = rewards.reduce((sum, r) => sum + r.amount, 0n);
  const averageReward = rewards.length > 0 ? totalRewards / BigInt(rewards.length) : 0n;
  const lastReward = rewards.length > 0 ? rewards[0].amount : 0n;

  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div className="bg-white p-4 rounded-lg shadow-lg border border-gray-200">
          <p className="text-sm font-semibold text-gray-900">Era {data.era}</p>
          <p className="text-sm text-gray-600">{data.date}</p>
          <p className="text-sm font-bold text-etrid-600 mt-2">
            {data.amount.toFixed(4)} ETRID
          </p>
        </div>
      );
    }
    return null;
  };

  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <div className="animate-pulse space-y-4">
          <div className="h-6 bg-gray-200 rounded w-1/4"></div>
          <div className="h-64 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }

  const renderChart = () => {
    const commonProps = {
      data: filteredRewards,
      margin: { top: 10, right: 30, left: 0, bottom: 0 },
    };

    switch (chartType) {
      case 'line':
        return (
          <LineChart {...commonProps}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="date" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Line
              type="monotone"
              dataKey="amount"
              stroke="#0ea5e9"
              strokeWidth={2}
              dot={{ fill: '#0ea5e9', r: 4 }}
              activeDot={{ r: 6 }}
              name="Reward (ETRID)"
            />
          </LineChart>
        );

      case 'area':
        return (
          <AreaChart {...commonProps}>
            <defs>
              <linearGradient id="colorReward" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#0ea5e9" stopOpacity={0.8} />
                <stop offset="95%" stopColor="#0ea5e9" stopOpacity={0.1} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="date" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Area
              type="monotone"
              dataKey="amount"
              stroke="#0ea5e9"
              fillOpacity={1}
              fill="url(#colorReward)"
              name="Reward (ETRID)"
            />
          </AreaChart>
        );

      case 'bar':
        return (
          <BarChart {...commonProps}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis dataKey="date" stroke="#6b7280" fontSize={12} />
            <YAxis stroke="#6b7280" fontSize={12} />
            <Tooltip content={<CustomTooltip />} />
            <Legend />
            <Bar dataKey="amount" fill="#0ea5e9" radius={[8, 8, 0, 0]} name="Reward (ETRID)" />
          </BarChart>
        );
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden">
      <div className="p-6 border-b border-gray-200">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-bold text-gray-900">Reward History</h2>
          <div className="flex space-x-2">
            {(['7d', '30d', '90d', 'all'] as TimeRange[]).map((range) => (
              <button
                key={range}
                onClick={() => setTimeRange(range)}
                className={`px-3 py-1 text-sm font-medium rounded-lg transition-colors ${
                  timeRange === range
                    ? 'bg-etrid-600 text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                {range.toUpperCase()}
              </button>
            ))}
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
          <div className="bg-gradient-to-br from-etrid-50 to-etrid-100 rounded-lg p-4">
            <div className="flex items-center space-x-2 mb-2">
              <DollarSign className="w-5 h-5 text-etrid-600" />
              <span className="text-sm font-medium text-gray-600">Total Rewards</span>
            </div>
            <p className="text-2xl font-bold text-gray-900">
              {formatTokenAmount(totalRewards)}
            </p>
          </div>

          <div className="bg-gradient-to-br from-blue-50 to-blue-100 rounded-lg p-4">
            <div className="flex items-center space-x-2 mb-2">
              <TrendingUp className="w-5 h-5 text-blue-600" />
              <span className="text-sm font-medium text-gray-600">Average Reward</span>
            </div>
            <p className="text-2xl font-bold text-gray-900">
              {formatTokenAmount(averageReward)}
            </p>
          </div>

          <div className="bg-gradient-to-br from-purple-50 to-purple-100 rounded-lg p-4">
            <div className="flex items-center space-x-2 mb-2">
              <Calendar className="w-5 h-5 text-purple-600" />
              <span className="text-sm font-medium text-gray-600">Last Reward</span>
            </div>
            <p className="text-2xl font-bold text-gray-900">
              {formatTokenAmount(lastReward)}
            </p>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <div className="text-sm text-gray-600">
            Showing {filteredRewards.length} eras
          </div>
          <div className="flex space-x-2">
            {(['line', 'area', 'bar'] as ChartType[]).map((type) => (
              <button
                key={type}
                onClick={() => setChartType(type)}
                className={`px-3 py-1 text-sm font-medium rounded-lg capitalize transition-colors ${
                  chartType === type
                    ? 'bg-etrid-600 text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                {type}
              </button>
            ))}
          </div>
        </div>
      </div>

      <div className="p-6">
        {filteredRewards.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-64 text-gray-500">
            <Calendar className="w-16 h-16 mb-4 text-gray-300" />
            <p className="text-lg font-medium">No rewards in this time period</p>
            <p className="text-sm">Try selecting a different time range</p>
          </div>
        ) : (
          <ResponsiveContainer width="100%" height={300}>
            {renderChart()}
          </ResponsiveContainer>
        )}
      </div>
    </div>
  );
}
