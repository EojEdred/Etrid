import React, { useState, useMemo } from 'react';
import Head from 'next/head';
import Layout from '@/components/Layout';
import { useValidatorStats } from '@/hooks/useValidatorStats';
import { Download, TrendingUp, Calendar, Coins, Award } from 'lucide-react';
import { format } from 'date-fns';
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
import type { Reward } from '@/types';

export default function RewardsPage() {
  const [validatorAddress] = useState<string | undefined>(
    process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS
  );

  const { isConnected, isLoading, rewards } = useValidatorStats(validatorAddress);

  const [timeRange, setTimeRange] = useState<'7d' | '30d' | '90d' | 'all'>('30d');
  const [chartType, setChartType] = useState<'line' | 'area' | 'bar'>('area');

  // Filter rewards by time range
  const filteredRewards = useMemo(() => {
    const now = Date.now();
    const ranges = {
      '7d': 7 * 24 * 60 * 60 * 1000,
      '30d': 30 * 24 * 60 * 60 * 1000,
      '90d': 90 * 24 * 60 * 60 * 1000,
      all: Infinity,
    };

    return rewards
      .filter((r) => now - r.timestamp <= ranges[timeRange])
      .sort((a, b) => a.timestamp - b.timestamp);
  }, [rewards, timeRange]);

  // Calculate statistics
  const stats = useMemo(() => {
    const totalRewards = filteredRewards.reduce(
      (sum, r) => sum + r.amount,
      BigInt(0)
    );
    const averageReward =
      filteredRewards.length > 0
        ? totalRewards / BigInt(filteredRewards.length)
        : BigInt(0);
    const totalEras = filteredRewards.length;
    const latestReward =
      filteredRewards.length > 0
        ? filteredRewards[filteredRewards.length - 1].amount
        : BigInt(0);

    return {
      totalRewards,
      averageReward,
      totalEras,
      latestReward,
    };
  }, [filteredRewards]);

  const formatBalance = (balance: bigint) => {
    return (Number(balance) / 1e18).toFixed(4);
  };

  // Prepare chart data
  const chartData = filteredRewards.map((reward) => ({
    era: reward.era,
    amount: Number(reward.amount) / 1e18,
    timestamp: reward.timestamp,
    date: format(new Date(reward.timestamp), 'MMM dd'),
    commission: reward.commission,
  }));

  const handleExportData = () => {
    const csvData = [
      ['Era', 'Amount (ETR)', 'Date', 'Nominators', 'Commission (%)'].join(','),
      ...filteredRewards.map((reward) =>
        [
          reward.era,
          formatBalance(reward.amount),
          new Date(reward.timestamp).toISOString(),
          reward.nominators,
          reward.commission,
        ].join(',')
      ),
    ].join('\n');

    const blob = new Blob([csvData], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `rewards-${Date.now()}.csv`;
    a.click();
  };

  const handleConnectWallet = () => {
    console.log('Connect wallet');
  };

  const renderChart = () => {
    const commonProps = {
      data: chartData,
      margin: { top: 5, right: 30, left: 20, bottom: 5 },
    };

    const customTooltip = ({ active, payload }: any) => {
      if (active && payload && payload.length) {
        return (
          <div className="bg-white p-4 rounded-lg shadow-lg border border-gray-200">
            <p className="text-sm font-semibold text-gray-900">Era {payload[0].payload.era}</p>
            <p className="text-sm text-gray-600">{payload[0].payload.date}</p>
            <p className="text-sm font-bold text-etrid-600 mt-2">
              {payload[0].value.toFixed(4)} ETR
            </p>
          </div>
        );
      }
      return null;
    };

    switch (chartType) {
      case 'line':
        return (
          <LineChart {...commonProps}>
            <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
            <XAxis dataKey="date" stroke="#6b7280" />
            <YAxis stroke="#6b7280" />
            <Tooltip content={customTooltip} />
            <Legend />
            <Line
              type="monotone"
              dataKey="amount"
              stroke="#0ea5e9"
              strokeWidth={2}
              dot={{ fill: '#0ea5e9', r: 4 }}
              activeDot={{ r: 6 }}
              name="Rewards (ETR)"
            />
          </LineChart>
        );

      case 'area':
        return (
          <AreaChart {...commonProps}>
            <defs>
              <linearGradient id="colorRewards" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#0ea5e9" stopOpacity={0.8} />
                <stop offset="95%" stopColor="#0ea5e9" stopOpacity={0.1} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
            <XAxis dataKey="date" stroke="#6b7280" />
            <YAxis stroke="#6b7280" />
            <Tooltip content={customTooltip} />
            <Legend />
            <Area
              type="monotone"
              dataKey="amount"
              stroke="#0ea5e9"
              strokeWidth={2}
              fillOpacity={1}
              fill="url(#colorRewards)"
              name="Rewards (ETR)"
            />
          </AreaChart>
        );

      case 'bar':
        return (
          <BarChart {...commonProps}>
            <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
            <XAxis dataKey="date" stroke="#6b7280" />
            <YAxis stroke="#6b7280" />
            <Tooltip content={customTooltip} />
            <Legend />
            <Bar dataKey="amount" fill="#0ea5e9" name="Rewards (ETR)" />
          </BarChart>
        );
    }
  };

  return (
    <>
      <Head>
        <title>Rewards - Validator Dashboard</title>
        <meta name="description" content="Track your validator rewards and earnings" />
      </Head>

      <Layout isConnected={isConnected} onConnectWallet={handleConnectWallet}>
        <div className="space-y-6">
          {/* Page Header */}
          <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">Rewards Tracker</h1>
              <p className="text-gray-600 mt-1">
                View and analyze your validator rewards over time
              </p>
            </div>

            <button
              onClick={handleExportData}
              className="flex items-center space-x-2 px-4 py-2 bg-etrid-600 text-white rounded-lg hover:bg-etrid-700 transition-colors"
            >
              <Download className="w-4 h-4" />
              <span>Export Data</span>
            </button>
          </div>

          {/* Statistics Cards */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <div className="bg-white rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600 mb-1">Total Rewards</p>
                  <p className="text-2xl font-bold text-gray-900">
                    {formatBalance(stats.totalRewards)}
                  </p>
                  <p className="text-xs text-gray-500 mt-1">ETR</p>
                </div>
                <div className="w-12 h-12 bg-etrid-100 rounded-lg flex items-center justify-center">
                  <Coins className="w-6 h-6 text-etrid-600" />
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600 mb-1">Average Reward</p>
                  <p className="text-2xl font-bold text-gray-900">
                    {formatBalance(stats.averageReward)}
                  </p>
                  <p className="text-xs text-gray-500 mt-1">ETR per era</p>
                </div>
                <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
                  <TrendingUp className="w-6 h-6 text-purple-600" />
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600 mb-1">Latest Reward</p>
                  <p className="text-2xl font-bold text-gray-900">
                    {formatBalance(stats.latestReward)}
                  </p>
                  <p className="text-xs text-gray-500 mt-1">ETR</p>
                </div>
                <div className="w-12 h-12 bg-success-100 rounded-lg flex items-center justify-center">
                  <Award className="w-6 h-6 text-success-600" />
                </div>
              </div>
            </div>

            <div className="bg-white rounded-lg shadow-md p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600 mb-1">Eras Tracked</p>
                  <p className="text-2xl font-bold text-gray-900">{stats.totalEras}</p>
                  <p className="text-xs text-gray-500 mt-1">Total eras</p>
                </div>
                <div className="w-12 h-12 bg-warning-100 rounded-lg flex items-center justify-center">
                  <Calendar className="w-6 h-6 text-warning-600" />
                </div>
              </div>
            </div>
          </div>

          {/* Chart Controls */}
          <div className="bg-white rounded-lg shadow-md p-6">
            <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
              {/* Time Range Selector */}
              <div className="flex items-center space-x-2">
                <span className="text-sm text-gray-600 font-medium">Time Range:</span>
                <div className="flex space-x-2">
                  {(['7d', '30d', '90d', 'all'] as const).map((range) => (
                    <button
                      key={range}
                      onClick={() => setTimeRange(range)}
                      className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                        timeRange === range
                          ? 'bg-etrid-600 text-white'
                          : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                      }`}
                    >
                      {range === 'all' ? 'All Time' : range.toUpperCase()}
                    </button>
                  ))}
                </div>
              </div>

              {/* Chart Type Selector */}
              <div className="flex items-center space-x-2">
                <span className="text-sm text-gray-600 font-medium">Chart Type:</span>
                <div className="flex space-x-2">
                  {(['line', 'area', 'bar'] as const).map((type) => (
                    <button
                      key={type}
                      onClick={() => setChartType(type)}
                      className={`px-4 py-2 rounded-lg text-sm font-medium capitalize transition-colors ${
                        chartType === type
                          ? 'bg-etrid-600 text-white'
                          : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                      }`}
                    >
                      {type}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          </div>

          {/* Chart */}
          <div className="bg-white rounded-lg shadow-md p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-6">Rewards History</h3>
            {isLoading ? (
              <div className="flex justify-center items-center h-80">
                <div className="w-8 h-8 border-4 border-etrid-600 border-t-transparent rounded-full animate-spin"></div>
              </div>
            ) : chartData.length === 0 ? (
              <div className="flex justify-center items-center h-80 text-gray-500">
                No reward data available for the selected time range
              </div>
            ) : (
              <ResponsiveContainer width="100%" height={400}>
                {renderChart()}
              </ResponsiveContainer>
            )}
          </div>

          {/* Rewards Table */}
          <div className="bg-white rounded-lg shadow-md overflow-hidden">
            <div className="px-6 py-4 border-b border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900">Recent Rewards</h3>
            </div>

            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-50 border-b border-gray-200">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Era
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Date
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Amount
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Nominators
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Commission
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200">
                  {isLoading ? (
                    <tr>
                      <td colSpan={5} className="px-6 py-12 text-center">
                        <div className="flex justify-center">
                          <div className="w-8 h-8 border-4 border-etrid-600 border-t-transparent rounded-full animate-spin"></div>
                        </div>
                      </td>
                    </tr>
                  ) : filteredRewards.length === 0 ? (
                    <tr>
                      <td colSpan={5} className="px-6 py-12 text-center text-gray-500">
                        No rewards data available
                      </td>
                    </tr>
                  ) : (
                    filteredRewards
                      .slice()
                      .reverse()
                      .slice(0, 20)
                      .map((reward) => (
                        <tr
                          key={reward.era}
                          className="hover:bg-gray-50 transition-colors"
                        >
                          <td className="px-6 py-4 whitespace-nowrap">
                            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-etrid-100 text-etrid-800">
                              Era {reward.era}
                            </span>
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap">
                            <p className="text-sm text-gray-900">
                              {format(new Date(reward.timestamp), 'MMM dd, yyyy')}
                            </p>
                            <p className="text-xs text-gray-500">
                              {format(new Date(reward.timestamp), 'HH:mm:ss')}
                            </p>
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap">
                            <p className="text-sm font-semibold text-gray-900">
                              {formatBalance(reward.amount)} ETR
                            </p>
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap">
                            <p className="text-sm text-gray-900">{reward.nominators}</p>
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap">
                            <p className="text-sm text-gray-900">{reward.commission}%</p>
                          </td>
                        </tr>
                      ))
                  )}
                </tbody>
              </table>
            </div>

            {filteredRewards.length > 20 && (
              <div className="px-6 py-4 bg-gray-50 border-t border-gray-200">
                <p className="text-sm text-gray-600">
                  Showing 20 most recent of {filteredRewards.length} rewards
                </p>
              </div>
            )}
          </div>
        </div>
      </Layout>
    </>
  );
}
