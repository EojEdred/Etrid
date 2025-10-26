'use client';

import React from 'react';
import { Trophy, TrendingUp, Eye, Shield, Zap, Award } from 'lucide-react';
import { formatNumber, formatPercentage, formatDuration } from '../lib/utils';
import { LineChart, Line, AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

// Mock historical data
const scoreHistory = [
  { date: 'Mon', score: 75 },
  { date: 'Tue', score: 78 },
  { date: 'Wed', score: 82 },
  { date: 'Thu', score: 85 },
  { date: 'Fri', score: 88 },
  { date: 'Sat', score: 90 },
  { date: 'Sun', score: 92 },
];

export default function ReputationScore() {
  const metrics = {
    watchtowerId: 'WT-' + Math.random().toString(36).substring(7).toUpperCase(),
    totalChannelsMonitored: 47,
    fraudDetections: 12,
    falsePositives: 1,
    uptime: 99.8,
    responseTime: 342,
    successfulInterventions: 11,
    score: 92,
    rank: 7,
    earnings: 5420,
  };

  const getScoreColor = (score: number) => {
    if (score >= 90) return 'text-green-400';
    if (score >= 75) return 'text-blue-400';
    if (score >= 60) return 'text-yellow-400';
    return 'text-red-400';
  };

  const getScoreGrade = (score: number) => {
    if (score >= 90) return 'A+';
    if (score >= 85) return 'A';
    if (score >= 80) return 'B+';
    if (score >= 75) return 'B';
    if (score >= 70) return 'C+';
    return 'C';
  };

  const accuracy = metrics.fraudDetections > 0
    ? ((metrics.fraudDetections - metrics.falsePositives) / metrics.fraudDetections) * 100
    : 0;

  return (
    <div className="bg-white/10 backdrop-blur-lg rounded-xl border border-white/20">
      <div className="p-6 border-b border-white/20">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Trophy className="w-5 h-5 text-yellow-400" />
            Reputation & Performance
          </h2>
          <div className="flex items-center gap-2">
            <Award className="w-5 h-5 text-purple-400" />
            <span className="text-sm text-gray-400">
              Rank <span className="text-white font-semibold">#{metrics.rank}</span>
            </span>
          </div>
        </div>
      </div>

      <div className="p-6">
        {/* Score Display */}
        <div className="flex items-center justify-center mb-8">
          <div className="relative">
            <svg className="w-48 h-48 transform -rotate-90">
              <circle
                cx="96"
                cy="96"
                r="88"
                stroke="currentColor"
                strokeWidth="12"
                fill="none"
                className="text-white/10"
              />
              <circle
                cx="96"
                cy="96"
                r="88"
                stroke="currentColor"
                strokeWidth="12"
                fill="none"
                strokeDasharray={`${(metrics.score / 100) * 553} 553`}
                className={getScoreColor(metrics.score)}
                strokeLinecap="round"
              />
            </svg>
            <div className="absolute inset-0 flex items-center justify-center flex-col">
              <p className={`text-5xl font-bold ${getScoreColor(metrics.score)}`}>
                {metrics.score}
              </p>
              <p className="text-sm text-gray-400">Reputation Score</p>
              <p className={`text-2xl font-bold mt-1 ${getScoreColor(metrics.score)}`}>
                {getScoreGrade(metrics.score)}
              </p>
            </div>
          </div>
        </div>

        {/* Metrics Grid */}
        <div className="grid grid-cols-2 md:grid-cols-3 gap-4 mb-6">
          <div className="bg-white/5 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Eye className="w-4 h-4 text-blue-400" />
              <p className="text-xs text-gray-400">Channels Monitored</p>
            </div>
            <p className="text-2xl font-bold">{formatNumber(metrics.totalChannelsMonitored)}</p>
          </div>

          <div className="bg-white/5 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Shield className="w-4 h-4 text-red-400" />
              <p className="text-xs text-gray-400">Fraud Detections</p>
            </div>
            <p className="text-2xl font-bold text-red-400">{formatNumber(metrics.fraudDetections)}</p>
          </div>

          <div className="bg-white/5 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Zap className="w-4 h-4 text-green-400" />
              <p className="text-xs text-gray-400">Interventions</p>
            </div>
            <p className="text-2xl font-bold text-green-400">
              {formatNumber(metrics.successfulInterventions)}
            </p>
          </div>

          <div className="bg-white/5 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <TrendingUp className="w-4 h-4 text-purple-400" />
              <p className="text-xs text-gray-400">Accuracy</p>
            </div>
            <p className="text-2xl font-bold text-purple-400">{formatPercentage(accuracy)}</p>
          </div>

          <div className="bg-white/5 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <TrendingUp className="w-4 h-4 text-green-400" />
              <p className="text-xs text-gray-400">Uptime</p>
            </div>
            <p className="text-2xl font-bold text-green-400">{formatPercentage(metrics.uptime)}</p>
          </div>

          <div className="bg-white/5 rounded-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Zap className="w-4 h-4 text-yellow-400" />
              <p className="text-xs text-gray-400">Response Time</p>
            </div>
            <p className="text-2xl font-bold text-yellow-400">{formatDuration(metrics.responseTime)}</p>
          </div>
        </div>

        {/* Score History Chart */}
        <div className="mt-6">
          <h3 className="text-sm font-semibold mb-4 text-gray-400">Score Trend (Last 7 Days)</h3>
          <ResponsiveContainer width="100%" height={200}>
            <AreaChart data={scoreHistory}>
              <defs>
                <linearGradient id="scoreGradient" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff20" />
              <XAxis dataKey="date" stroke="#9ca3af" style={{ fontSize: '12px' }} />
              <YAxis stroke="#9ca3af" style={{ fontSize: '12px' }} domain={[0, 100]} />
              <Tooltip
                contentStyle={{
                  backgroundColor: '#1e293b',
                  border: '1px solid #334155',
                  borderRadius: '8px',
                }}
              />
              <Area
                type="monotone"
                dataKey="score"
                stroke="#3b82f6"
                strokeWidth={2}
                fill="url(#scoreGradient)"
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        {/* Performance Indicators */}
        <div className="mt-6 pt-6 border-t border-white/10">
          <h3 className="text-sm font-semibold mb-4 text-gray-400">Performance Breakdown</h3>
          <div className="space-y-3">
            <div>
              <div className="flex justify-between text-sm mb-1">
                <span className="text-gray-400">Detection Accuracy</span>
                <span className="font-semibold">{formatPercentage(accuracy)}</span>
              </div>
              <div className="h-2 bg-white/10 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-purple-500 to-blue-500"
                  style={{ width: `${accuracy}%` }}
                />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-sm mb-1">
                <span className="text-gray-400">Uptime</span>
                <span className="font-semibold">{formatPercentage(metrics.uptime)}</span>
              </div>
              <div className="h-2 bg-white/10 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-green-500 to-emerald-500"
                  style={{ width: `${metrics.uptime}%` }}
                />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-sm mb-1">
                <span className="text-gray-400">Intervention Success Rate</span>
                <span className="font-semibold">
                  {formatPercentage((metrics.successfulInterventions / metrics.fraudDetections) * 100)}
                </span>
              </div>
              <div className="h-2 bg-white/10 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-blue-500 to-cyan-500"
                  style={{ width: `${(metrics.successfulInterventions / metrics.fraudDetections) * 100}%` }}
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
