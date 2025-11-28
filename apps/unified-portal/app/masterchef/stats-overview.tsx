'use client';

import type { MetricsData } from './types';
import { DollarSign, TrendingUp, Coins, Clock } from 'lucide-react';

interface StatsOverviewProps {
  metrics: MetricsData;
}

export function StatsOverview({ metrics }: StatsOverviewProps) {
  const totalTVL = metrics.pools.reduce((sum, pool) => sum + (pool.tvlUSD || 0), 0);

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      {/* Total TVL */}
      <div className="bg-zinc-900 rounded-lg shadow-sm p-6 border border-zinc-800">
        <div className="flex items-center justify-between mb-2">
          <p className="text-sm font-medium text-zinc-400">Total TVL</p>
          <div className="bg-green-500/10 p-2 rounded-lg">
            <DollarSign className="w-5 h-5 text-green-500" />
          </div>
        </div>
        <p className="text-3xl font-bold text-white">
          ${totalTVL > 0 ? totalTVL.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 }) : 'N/A'}
        </p>
        <p className="mt-2 text-sm text-zinc-400">
          Across {metrics.overview.totalPools} pool{metrics.overview.totalPools !== 1 ? 's' : ''}
        </p>
      </div>

      {/* Daily Emissions */}
      <div className="bg-zinc-900 rounded-lg shadow-sm p-6 border border-zinc-800">
        <div className="flex items-center justify-between mb-2">
          <p className="text-sm font-medium text-zinc-400">Daily Emissions</p>
          <div className="bg-blue-500/10 p-2 rounded-lg">
            <TrendingUp className="w-5 h-5 text-blue-500" />
          </div>
        </div>
        <p className="text-3xl font-bold text-white">
          {parseFloat(metrics.emissions.perDay).toLocaleString(undefined, { maximumFractionDigits: 0 })}
        </p>
        <p className="mt-2 text-sm text-zinc-400">ÉTR per day</p>
      </div>

      {/* MasterChef Balance */}
      <div className="bg-zinc-900 rounded-lg shadow-sm p-6 border border-zinc-800">
        <div className="flex items-center justify-between mb-2">
          <p className="text-sm font-medium text-zinc-400">MasterChef Balance</p>
          <div className="bg-purple-500/10 p-2 rounded-lg">
            <Coins className="w-5 h-5 text-purple-500" />
          </div>
        </div>
        <p className="text-3xl font-bold text-white">
          {parseFloat(metrics.balance.masterChefETR).toLocaleString(undefined, { maximumFractionDigits: 0 })}
        </p>
        <p className="mt-2 text-sm text-zinc-400">ÉTR available</p>
      </div>

      {/* Days Remaining */}
      <div className="bg-zinc-900 rounded-lg shadow-sm p-6 border border-zinc-800">
        <div className="flex items-center justify-between mb-2">
          <p className="text-sm font-medium text-zinc-400">Days Remaining</p>
          <div
            className={`p-2 rounded-lg ${
              metrics.balance.daysRemaining < 7
                ? 'bg-red-500/10'
                : metrics.balance.daysRemaining < 30
                  ? 'bg-yellow-500/10'
                  : 'bg-green-500/10'
            }`}
          >
            <Clock
              className={`w-5 h-5 ${
                metrics.balance.daysRemaining < 7
                  ? 'text-red-500'
                  : metrics.balance.daysRemaining < 30
                    ? 'text-yellow-500'
                    : 'text-green-500'
              }`}
            />
          </div>
        </div>
        <p className="text-3xl font-bold text-white">{metrics.balance.daysRemaining}</p>
        <p
          className={`mt-2 text-sm ${
            metrics.balance.daysRemaining < 7
              ? 'text-red-400 font-medium'
              : metrics.balance.daysRemaining < 30
                ? 'text-yellow-400'
                : 'text-zinc-400'
          }`}
        >
          {metrics.balance.daysRemaining < 7
            ? 'Top up soon!'
            : metrics.balance.daysRemaining < 30
              ? 'Low balance'
              : 'At current rate'}
        </p>
      </div>

      {/* ÉTR Price */}
      {metrics.prices && metrics.prices.etr > 0 && (
        <div className="bg-zinc-900 rounded-lg shadow-sm p-6 border border-zinc-800">
          <div className="flex items-center justify-between mb-2">
            <p className="text-sm font-medium text-zinc-400">ÉTR Price</p>
            <div className="bg-indigo-500/10 p-2 rounded-lg">
              <Coins className="w-5 h-5 text-indigo-500" />
            </div>
          </div>
          <p className="text-3xl font-bold text-white">${metrics.prices.etr.toFixed(6)}</p>
          <p className="mt-2 text-sm text-zinc-400">Per ÉTR</p>
        </div>
      )}

      {/* BNB Price */}
      {metrics.prices && metrics.prices.bnb > 0 && (
        <div className="bg-zinc-900 rounded-lg shadow-sm p-6 border border-zinc-800">
          <div className="flex items-center justify-between mb-2">
            <p className="text-sm font-medium text-zinc-400">BNB Price</p>
            <div className="bg-yellow-500/10 p-2 rounded-lg">
              <Coins className="w-5 h-5 text-yellow-500" />
            </div>
          </div>
          <p className="text-3xl font-bold text-white">${metrics.prices.bnb.toFixed(2)}</p>
          <p className="mt-2 text-sm text-zinc-400">Per BNB</p>
        </div>
      )}
    </div>
  );
}
