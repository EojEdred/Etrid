'use client';

import React, { useState } from 'react';
import { DollarSign, TrendingUp, Download, Calendar, Wallet } from 'lucide-react';
import { formatCurrency, formatNumber } from '../lib/utils';
import { BarChart, Bar, LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from 'recharts';
import type { Earnings, EarningEntry } from '../types';

// Mock data
const earningsData = {
  totalEarned: 12450.75,
  pendingRewards: 842.30,
  lastPayout: new Date(Date.now() - 86400000 * 3),
  earningsHistory: [
    {
      id: '1',
      timestamp: new Date(Date.now() - 86400000),
      amount: 125.50,
      type: 'subscription_fee' as const,
      channelId: 'ch_abc123',
      description: 'Monthly subscription from channel ch_abc123',
    },
    {
      id: '2',
      timestamp: new Date(Date.now() - 86400000 * 2),
      amount: 500.00,
      type: 'fraud_detection_reward' as const,
      channelId: 'ch_def456',
      description: 'Fraud detection reward for intervention',
    },
    {
      id: '3',
      timestamp: new Date(Date.now() - 86400000 * 3),
      amount: 150.00,
      type: 'uptime_bonus' as const,
      description: 'Weekly uptime bonus (99.8% uptime)',
    },
    {
      id: '4',
      timestamp: new Date(Date.now() - 86400000 * 5),
      amount: 225.75,
      type: 'subscription_fee' as const,
      channelId: 'ch_ghi789',
      description: 'Premium tier subscription renewal',
    },
  ],
};

const monthlyData = [
  { month: 'Jan', subscriptions: 1200, rewards: 800, bonuses: 200 },
  { month: 'Feb', subscriptions: 1400, rewards: 1100, bonuses: 250 },
  { month: 'Mar', subscriptions: 1600, rewards: 900, bonuses: 300 },
  { month: 'Apr', subscriptions: 1800, rewards: 1400, bonuses: 350 },
  { month: 'May', subscriptions: 2100, rewards: 1600, bonuses: 400 },
  { month: 'Jun', subscriptions: 2300, rewards: 1200, bonuses: 450 },
];

export default function EarningsTracker() {
  const [timeRange, setTimeRange] = useState<'week' | 'month' | 'year'>('month');
  const earnings = earningsData;

  const getTypeColor = (type: string) => {
    const colors: Record<string, string> = {
      subscription_fee: 'bg-blue-500/20 text-blue-400',
      fraud_detection_reward: 'bg-green-500/20 text-green-400',
      uptime_bonus: 'bg-purple-500/20 text-purple-400',
    };
    return colors[type] || 'bg-gray-500/20 text-gray-400';
  };

  const getTypeIcon = (type: string): React.ReactElement => {
    const icons: Record<string, React.ReactElement> = {
      subscription_fee: <Calendar className="w-3 h-3" />,
      fraud_detection_reward: <TrendingUp className="w-3 h-3" />,
      uptime_bonus: <Wallet className="w-3 h-3" />,
    };
    return icons[type] || <DollarSign className="w-3 h-3" />;
  };

  return (
    <div className="bg-white/10 backdrop-blur-lg rounded-xl border border-white/20">
      <div className="p-6 border-b border-white/20">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <DollarSign className="w-5 h-5 text-green-400" />
            Earnings Tracker
          </h2>
          <button className="flex items-center gap-2 px-3 py-1.5 bg-blue-500 hover:bg-blue-600 rounded-lg text-sm transition-colors">
            <Download className="w-4 h-4" />
            Export
          </button>
        </div>
      </div>

      <div className="p-6">
        {/* Summary Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
          <div className="bg-gradient-to-br from-green-500/20 to-emerald-500/20 rounded-xl p-5 border border-green-500/30">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-gray-300">Total Earned</p>
              <Wallet className="w-5 h-5 text-green-400" />
            </div>
            <p className="text-3xl font-bold text-green-400">
              {formatCurrency(earnings.totalEarned)}
            </p>
            <p className="text-xs text-gray-400 mt-1">All time</p>
          </div>

          <div className="bg-gradient-to-br from-blue-500/20 to-cyan-500/20 rounded-xl p-5 border border-blue-500/30">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-gray-300">Pending Rewards</p>
              <TrendingUp className="w-5 h-5 text-blue-400" />
            </div>
            <p className="text-3xl font-bold text-blue-400">
              {formatCurrency(earnings.pendingRewards)}
            </p>
            <p className="text-xs text-gray-400 mt-1">Awaiting payout</p>
          </div>

          <div className="bg-gradient-to-br from-purple-500/20 to-pink-500/20 rounded-xl p-5 border border-purple-500/30">
            <div className="flex items-center justify-between mb-2">
              <p className="text-sm text-gray-300">Last Payout</p>
              <Calendar className="w-5 h-5 text-purple-400" />
            </div>
            <p className="text-2xl font-bold text-purple-400">
              {earnings.lastPayout.toLocaleDateString()}
            </p>
            <p className="text-xs text-gray-400 mt-1">
              {Math.floor((Date.now() - earnings.lastPayout.getTime()) / (1000 * 60 * 60 * 24))} days ago
            </p>
          </div>
        </div>

        {/* Time Range Selector */}
        <div className="flex gap-2 mb-6">
          <button
            onClick={() => setTimeRange('week')}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
              timeRange === 'week'
                ? 'bg-blue-500 text-white'
                : 'bg-white/5 text-gray-400 hover:bg-white/10'
            }`}
          >
            Week
          </button>
          <button
            onClick={() => setTimeRange('month')}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
              timeRange === 'month'
                ? 'bg-blue-500 text-white'
                : 'bg-white/5 text-gray-400 hover:bg-white/10'
            }`}
          >
            Month
          </button>
          <button
            onClick={() => setTimeRange('year')}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
              timeRange === 'year'
                ? 'bg-blue-500 text-white'
                : 'bg-white/5 text-gray-400 hover:bg-white/10'
            }`}
          >
            Year
          </button>
        </div>

        {/* Earnings Chart */}
        <div className="mb-6">
          <h3 className="text-sm font-semibold mb-4 text-gray-400">Earnings Breakdown (Last 6 Months)</h3>
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={monthlyData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff20" />
              <XAxis dataKey="month" stroke="#9ca3af" style={{ fontSize: '12px' }} />
              <YAxis stroke="#9ca3af" style={{ fontSize: '12px' }} />
              <Tooltip
                contentStyle={{
                  backgroundColor: '#1e293b',
                  border: '1px solid #334155',
                  borderRadius: '8px',
                }}
              />
              <Legend />
              <Bar dataKey="subscriptions" stackId="a" fill="#3b82f6" name="Subscriptions" />
              <Bar dataKey="rewards" stackId="a" fill="#10b981" name="Fraud Rewards" />
              <Bar dataKey="bonuses" stackId="a" fill="#8b5cf6" name="Bonuses" />
            </BarChart>
          </ResponsiveContainer>
        </div>

        {/* Recent Earnings */}
        <div>
          <h3 className="text-sm font-semibold mb-4 text-gray-400">Recent Earnings</h3>
          <div className="space-y-3">
            {earnings.earningsHistory.map((entry) => (
              <div
                key={entry.id}
                className="flex items-center justify-between p-4 bg-white/5 rounded-lg hover:bg-white/10 transition-colors"
              >
                <div className="flex items-start gap-3 flex-1">
                  <div className={`p-2 rounded-lg ${getTypeColor(entry.type)}`}>
                    {getTypeIcon(entry.type)}
                  </div>
                  <div className="flex-1">
                    <p className="font-medium text-sm">{entry.description}</p>
                    <div className="flex items-center gap-2 mt-1">
                      <span className="text-xs text-gray-400">
                        {entry.timestamp.toLocaleDateString()}
                      </span>
                      {entry.channelId && (
                        <>
                          <span className="text-xs text-gray-500">•</span>
                          <span className="text-xs text-gray-400 font-mono">
                            {entry.channelId}
                          </span>
                        </>
                      )}
                    </div>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-lg font-bold text-green-400">
                    +{formatCurrency(entry.amount)}
                  </p>
                  <span className={`text-xs px-2 py-0.5 rounded ${getTypeColor(entry.type)}`}>
                    {entry.type.replace(/_/g, ' ')}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Summary Stats */}
        <div className="mt-6 pt-6 border-t border-white/10">
          <div className="grid grid-cols-3 gap-4 text-center">
            <div>
              <p className="text-xs text-gray-400 mb-1">Avg. Monthly</p>
              <p className="text-xl font-bold text-blue-400">
                {formatCurrency(earnings.totalEarned / 6)}
              </p>
            </div>
            <div>
              <p className="text-xs text-gray-400 mb-1">Total Transactions</p>
              <p className="text-xl font-bold text-purple-400">
                {formatNumber(earnings.earningsHistory.length)}
              </p>
            </div>
            <div>
              <p className="text-xs text-gray-400 mb-1">Growth Rate</p>
              <p className="text-xl font-bold text-green-400">+24.5%</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
