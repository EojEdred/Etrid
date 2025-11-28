import React, { useState } from 'react';
import Head from 'next/head';
import Layout from '@/components/Layout';
import { useValidatorStats } from '@/hooks/useValidatorStats';
import {
  LineChart,
  Line,
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend,
  PieChart,
  Pie,
  Cell,
} from 'recharts';
import {
  Activity,
  TrendingUp,
  TrendingDown,
  Award,
  AlertTriangle,
  CheckCircle,
  Clock,
  Zap,
} from 'lucide-react';
import { formatNumber, formatPercentage } from '@/utils/format';

export default function Performance() {
  const [validatorAddress, setValidatorAddress] = useState<string | undefined>(
    process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS
  );
  const [timeRange, setTimeRange] = useState<'24h' | '7d' | '30d' | '90d'>('7d');

  const {
    isConnected,
    isLoading,
    validatorInfo,
    performance,
    rewards,
  } = useValidatorStats(validatorAddress);

  // Mock data for uptime history
  const uptimeData = Array.from({ length: 30 }, (_, i) => ({
    day: `Day ${i + 1}`,
    uptime: 98 + Math.random() * 2,
    blocks: Math.floor(Math.random() * 100) + 50,
  }));

  // Mock data for block production
  const blockProductionData = [
    { name: 'Produced', value: performance?.blocksProduced || 0, color: '#22c55e' },
    { name: 'Missed', value: performance?.missedBlocks || 0, color: '#ef4444' },
  ];

  // Performance metrics
  const metrics = [
    {
      title: 'Blocks Produced',
      value: formatNumber(performance?.blocksProduced || 0),
      change: '+12.5%',
      trend: 'up',
      icon: Award,
      color: 'text-success-600',
      bgColor: 'bg-success-50',
    },
    {
      title: 'Uptime',
      value: `${performance?.uptime.toFixed(2)}%`,
      change: '+0.3%',
      trend: 'up',
      icon: Activity,
      color: 'text-etrid-600',
      bgColor: 'bg-etrid-50',
    },
    {
      title: 'Avg Block Time',
      value: `${performance?.averageBlockTime.toFixed(2)}s`,
      change: '-0.1s',
      trend: 'up',
      icon: Clock,
      color: 'text-blue-600',
      bgColor: 'bg-blue-50',
    },
    {
      title: 'Era Points',
      value: formatNumber(performance?.eraPoints || 0),
      change: '+8.2%',
      trend: 'up',
      icon: Zap,
      color: 'text-purple-600',
      bgColor: 'bg-purple-50',
    },
  ];

  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      return (
        <div className="bg-white p-3 rounded-lg shadow-lg border border-gray-200">
          <p className="text-sm font-semibold text-gray-900">{payload[0].payload.day}</p>
          <p className="text-sm text-gray-600 mt-1">
            Uptime: <span className="font-semibold">{payload[0].value.toFixed(2)}%</span>
          </p>
          {payload[1] && (
            <p className="text-sm text-gray-600">
              Blocks: <span className="font-semibold">{payload[1].value}</span>
            </p>
          )}
        </div>
      );
    }
    return null;
  };

  return (
    <>
      <Head>
        <title>Performance Analytics - Ëtrid Validator Dashboard</title>
        <meta name="description" content="Detailed performance metrics for your validator" />
      </Head>

      <Layout isConnected={isConnected}>
        <div className="space-y-6">
          {/* Page Header */}
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">Performance Analytics</h1>
              <p className="text-gray-600 mt-1">
                Detailed metrics and insights for your validator
              </p>
            </div>

            <div className="flex space-x-2">
              {(['24h', '7d', '30d', '90d'] as const).map((range) => (
                <button
                  key={range}
                  onClick={() => setTimeRange(range)}
                  className={`px-4 py-2 text-sm font-medium rounded-lg transition-colors ${
                    timeRange === range
                      ? 'bg-etrid-600 text-white'
                      : 'bg-white border border-gray-300 text-gray-700 hover:bg-gray-50'
                  }`}
                >
                  {range.toUpperCase()}
                </button>
              ))}
            </div>
          </div>

          {/* Performance Metrics */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {metrics.map((metric, index) => {
              const Icon = metric.icon;
              const TrendIcon = metric.trend === 'up' ? TrendingUp : TrendingDown;
              return (
                <div key={index} className="bg-white rounded-lg shadow-md p-6">
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-sm font-medium text-gray-600">{metric.title}</h3>
                    <div className={`p-2 rounded-lg ${metric.bgColor}`}>
                      <Icon className={`w-5 h-5 ${metric.color}`} />
                    </div>
                  </div>
                  <div className="flex items-end justify-between">
                    <div>
                      <p className="text-2xl font-bold text-gray-900">{metric.value}</p>
                      <div className="flex items-center space-x-1 mt-1">
                        <TrendIcon
                          className={`w-4 h-4 ${
                            metric.trend === 'up' ? 'text-success-600' : 'text-danger-600'
                          }`}
                        />
                        <span
                          className={`text-sm font-medium ${
                            metric.trend === 'up' ? 'text-success-600' : 'text-danger-600'
                          }`}
                        >
                          {metric.change}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>

          {/* Performance Score */}
          <div className="bg-gradient-to-br from-etrid-500 to-etrid-700 rounded-lg p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <h2 className="text-xl font-semibold mb-2">Overall Performance Score</h2>
                <p className="text-etrid-100 text-sm">
                  Based on uptime, block production, and era points
                </p>
              </div>
              <div className="text-right">
                <div className="text-5xl font-bold">98.5</div>
                <div className="text-etrid-100 text-sm mt-1">out of 100</div>
              </div>
            </div>
            <div className="mt-6 bg-white/20 rounded-full h-3 overflow-hidden">
              <div
                className="bg-white h-full rounded-full transition-all duration-500"
                style={{ width: '98.5%' }}
              />
            </div>
            <div className="grid grid-cols-3 gap-4 mt-6">
              <div className="text-center">
                <CheckCircle className="w-6 h-6 mx-auto mb-2" />
                <p className="text-sm font-semibold">Excellent</p>
                <p className="text-xs text-etrid-100">Uptime</p>
              </div>
              <div className="text-center">
                <CheckCircle className="w-6 h-6 mx-auto mb-2" />
                <p className="text-sm font-semibold">Optimal</p>
                <p className="text-xs text-etrid-100">Block Production</p>
              </div>
              <div className="text-center">
                <CheckCircle className="w-6 h-6 mx-auto mb-2" />
                <p className="text-sm font-semibold">High</p>
                <p className="text-xs text-etrid-100">Era Points</p>
              </div>
            </div>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            {/* Uptime History */}
            <div className="lg:col-span-2 bg-white rounded-lg shadow-md p-6">
              <h2 className="text-xl font-bold text-gray-900 mb-6">Uptime & Block Production</h2>
              <ResponsiveContainer width="100%" height={300}>
                <AreaChart data={uptimeData}>
                  <defs>
                    <linearGradient id="colorUptime" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#0ea5e9" stopOpacity={0.8} />
                      <stop offset="95%" stopColor="#0ea5e9" stopOpacity={0.1} />
                    </linearGradient>
                    <linearGradient id="colorBlocks" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#22c55e" stopOpacity={0.8} />
                      <stop offset="95%" stopColor="#22c55e" stopOpacity={0.1} />
                    </linearGradient>
                  </defs>
                  <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
                  <XAxis dataKey="day" stroke="#6b7280" fontSize={12} />
                  <YAxis stroke="#6b7280" fontSize={12} />
                  <Tooltip content={<CustomTooltip />} />
                  <Legend />
                  <Area
                    type="monotone"
                    dataKey="uptime"
                    stroke="#0ea5e9"
                    fillOpacity={1}
                    fill="url(#colorUptime)"
                    name="Uptime (%)"
                  />
                  <Area
                    type="monotone"
                    dataKey="blocks"
                    stroke="#22c55e"
                    fillOpacity={1}
                    fill="url(#colorBlocks)"
                    name="Blocks Produced"
                  />
                </AreaChart>
              </ResponsiveContainer>
            </div>

            {/* Block Production Pie Chart */}
            <div className="bg-white rounded-lg shadow-md p-6">
              <h2 className="text-xl font-bold text-gray-900 mb-6">Block Production</h2>
              <ResponsiveContainer width="100%" height={200}>
                <PieChart>
                  <Pie
                    data={blockProductionData}
                    cx="50%"
                    cy="50%"
                    innerRadius={60}
                    outerRadius={80}
                    paddingAngle={5}
                    dataKey="value"
                  >
                    {blockProductionData.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={entry.color} />
                    ))}
                  </Pie>
                  <Tooltip />
                </PieChart>
              </ResponsiveContainer>
              <div className="space-y-3 mt-6">
                {blockProductionData.map((item, index) => (
                  <div key={index} className="flex items-center justify-between">
                    <div className="flex items-center space-x-2">
                      <div
                        className="w-3 h-3 rounded-full"
                        style={{ backgroundColor: item.color }}
                      />
                      <span className="text-sm text-gray-700">{item.name}</span>
                    </div>
                    <span className="text-sm font-semibold text-gray-900">
                      {item.value.toLocaleString()}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Validator Ranking */}
          <div className="bg-white rounded-lg shadow-md p-6">
            <h2 className="text-xl font-bold text-gray-900 mb-6">Validator Ranking</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="bg-gradient-to-br from-yellow-50 to-yellow-100 rounded-lg p-6 text-center">
                <Award className="w-12 h-12 text-yellow-600 mx-auto mb-3" />
                <p className="text-4xl font-bold text-yellow-700">#{performance?.rank || 0}</p>
                <p className="text-sm text-gray-600 mt-2">Current Rank</p>
              </div>
              <div className="bg-gradient-to-br from-blue-50 to-blue-100 rounded-lg p-6 text-center">
                <Activity className="w-12 h-12 text-blue-600 mx-auto mb-3" />
                <p className="text-4xl font-bold text-blue-700">
                  {performance?.totalValidators || 0}
                </p>
                <p className="text-sm text-gray-600 mt-2">Total Validators</p>
              </div>
              <div className="bg-gradient-to-br from-purple-50 to-purple-100 rounded-lg p-6 text-center">
                <TrendingUp className="w-12 h-12 text-purple-600 mx-auto mb-3" />
                <p className="text-4xl font-bold text-purple-700">
                  {performance?.rank && performance?.totalValidators
                    ? formatPercentage(
                        1 - (performance.rank - 1) / performance.totalValidators,
                        0
                      )
                    : '0%'}
                </p>
                <p className="text-sm text-gray-600 mt-2">Top Percentile</p>
              </div>
            </div>
          </div>

          {/* Performance Insights */}
          <div className="bg-white rounded-lg shadow-md p-6">
            <h2 className="text-xl font-bold text-gray-900 mb-6">Performance Insights</h2>
            <div className="space-y-4">
              <div className="flex items-start space-x-3 p-4 bg-success-50 border border-success-200 rounded-lg">
                <CheckCircle className="w-5 h-5 text-success-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="text-sm font-semibold text-success-900">
                    Excellent Uptime Performance
                  </p>
                  <p className="text-sm text-success-700 mt-1">
                    Your validator has maintained 99.9% uptime over the last 30 days, exceeding
                    network averages.
                  </p>
                </div>
              </div>

              <div className="flex items-start space-x-3 p-4 bg-blue-50 border border-blue-200 rounded-lg">
                <CheckCircle className="w-5 h-5 text-blue-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="text-sm font-semibold text-blue-900">
                    Consistent Block Production
                  </p>
                  <p className="text-sm text-blue-700 mt-1">
                    You&apos;re producing blocks consistently with minimal misses. Keep up the great
                    work!
                  </p>
                </div>
              </div>

              <div className="flex items-start space-x-3 p-4 bg-warning-50 border border-warning-200 rounded-lg">
                <AlertTriangle className="w-5 h-5 text-warning-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="text-sm font-semibold text-warning-900">
                    Commission Rate Optimization
                  </p>
                  <p className="text-sm text-warning-700 mt-1">
                    Consider lowering your commission by 1-2% to attract more nominators and
                    increase your total stake.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Layout>
    </>
  );
}
