'use client';

import type { PoolData, EmissionsData } from './types';
import { ExternalLink } from 'lucide-react';

interface PoolCardProps {
  pool: PoolData;
  emissions: EmissionsData;
}

export function PoolCard({ pool, emissions }: PoolCardProps) {
  return (
    <div className="bg-zinc-900 rounded-lg border border-zinc-800 p-6 hover:border-zinc-700 transition-colors">
      {/* Pool Header */}
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold text-white">{pool.lpSymbol}</h3>
          <p className="text-sm text-zinc-400">{pool.lpName}</p>
        </div>
        <div className="bg-blue-500/10 text-blue-400 text-xs font-medium px-2.5 py-0.5 rounded">
          Pool {pool.poolId}
        </div>
      </div>

      {/* TVL */}
      {pool.tvlUSD !== null && (
        <div className="mb-4">
          <p className="text-sm text-zinc-400">Total Value Locked</p>
          <p className="text-2xl font-bold text-white">
            ${pool.tvlUSD.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
          </p>
        </div>
      )}

      {/* APR */}
      {pool.aprPercent !== null && (
        <div className="mb-4">
          <p className="text-sm text-zinc-400">Annual Percentage Rate</p>
          <p className="text-3xl font-bold text-green-400">{pool.aprPercent.toFixed(2)}%</p>
        </div>
      )}

      {/* Stats Grid */}
      <div className="grid grid-cols-2 gap-4 pt-4 border-t border-zinc-800">
        <div>
          <p className="text-xs text-zinc-400">LP Staked</p>
          <p className="text-sm font-medium text-white">
            {parseFloat(pool.totalStaked).toLocaleString(undefined, { maximumFractionDigits: 2 })}
          </p>
        </div>
        <div>
          <p className="text-xs text-zinc-400">Reward Share</p>
          <p className="text-sm font-medium text-white">{pool.rewardShare.toFixed(1)}%</p>
        </div>
        <div>
          <p className="text-xs text-zinc-400">Daily Rewards</p>
          <p className="text-sm font-medium text-white">
            {parseFloat(pool.dailyRewards).toLocaleString(undefined, { maximumFractionDigits: 0 })} ÉTR
          </p>
        </div>
        <div>
          <p className="text-xs text-zinc-400">Monthly Rewards</p>
          <p className="text-sm font-medium text-white">
            {parseFloat(pool.monthlyRewards).toLocaleString(undefined, { maximumFractionDigits: 0 })} ÉTR
          </p>
        </div>
      </div>

      {/* LP Price */}
      {pool.lpPrice !== null && (
        <div className="mt-4 pt-4 border-t border-zinc-800">
          <p className="text-xs text-zinc-400">LP Token Price</p>
          <p className="text-sm font-medium text-white">${pool.lpPrice.toFixed(4)}</p>
        </div>
      )}

      {/* LP Token Address */}
      <div className="mt-4">
        <p className="text-xs text-zinc-400 mb-1">LP Token Address</p>
        <a
          href={`https://bscscan.com/address/${pool.lpToken}`}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-1 text-xs font-mono text-blue-400 hover:text-blue-300 break-all transition-colors"
        >
          {pool.lpToken.slice(0, 10)}...{pool.lpToken.slice(-8)}
          <ExternalLink className="w-3 h-3" />
        </a>
      </div>
    </div>
  );
}
