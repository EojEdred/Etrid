'use client';

import React from 'react';
import { Activity, Users, TrendingUp, Award } from 'lucide-react';
import type { ValidatorInfo, PerformanceMetrics } from '@/types/validator';
import { formatTokenAmount, formatCommission } from '@/lib/validator/format';

interface ValidatorStatsProps {
  validatorInfo: ValidatorInfo | null;
  performance: PerformanceMetrics | null;
  isLoading?: boolean;
}

export default function ValidatorStats({
  validatorInfo,
  performance,
  isLoading = false,
}: ValidatorStatsProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {[...Array(4)].map((_, i) => (
          <div key={i} className="bg-white dark:bg-zinc-900 rounded-lg shadow-md p-6 animate-pulse">
            <div className="h-4 bg-gray-200 dark:bg-zinc-700 rounded w-1/2 mb-4"></div>
            <div className="h-8 bg-gray-200 dark:bg-zinc-700 rounded w-3/4"></div>
          </div>
        ))}
      </div>
    );
  }

  const stats = [
    {
      title: 'Total Stake',
      value: validatorInfo ? formatTokenAmount(validatorInfo.totalStake) : '0 ETRID',
      icon: TrendingUp,
      color: 'text-blue-600 dark:text-blue-400',
      bgColor: 'bg-blue-50 dark:bg-blue-900/20',
      subtext: validatorInfo
        ? `Own: ${formatTokenAmount(validatorInfo.ownStake)}`
        : 'No data',
    },
    {
      title: 'Nominators',
      value: validatorInfo?.nominatorCount.toString() || '0',
      icon: Users,
      color: 'text-green-600 dark:text-green-400',
      bgColor: 'bg-green-50 dark:bg-green-900/20',
      subtext: validatorInfo?.isActive ? 'Active' : 'Inactive',
    },
    {
      title: 'Commission',
      value: validatorInfo ? formatCommission(validatorInfo.commission) : '0%',
      icon: Activity,
      color: 'text-purple-600 dark:text-purple-400',
      bgColor: 'bg-purple-50 dark:bg-purple-900/20',
      subtext: validatorInfo?.isElected ? 'Elected' : 'Waiting',
    },
    {
      title: 'Era Points',
      value: performance?.eraPoints.toLocaleString() || '0',
      icon: Award,
      color: 'text-orange-600 dark:text-orange-400',
      bgColor: 'bg-orange-50 dark:bg-orange-900/20',
      subtext: performance ? `Rank #${performance.rank}/${performance.totalValidators}` : 'N/A',
    },
  ];

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      {stats.map((stat, index) => {
        const Icon = stat.icon;
        return (
          <div
            key={index}
            className="bg-white dark:bg-zinc-900 rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow"
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-medium text-gray-600 dark:text-zinc-400">{stat.title}</h3>
              <div className={`p-2 rounded-lg ${stat.bgColor}`}>
                <Icon className={`w-5 h-5 ${stat.color}`} />
              </div>
            </div>
            <div className="space-y-1">
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{stat.value}</p>
              <p className="text-xs text-gray-500 dark:text-zinc-500">{stat.subtext}</p>
            </div>
          </div>
        );
      })}
    </div>
  );
}
