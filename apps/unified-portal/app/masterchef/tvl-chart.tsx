'use client';

import type { PoolData } from './types';

interface TVLChartProps {
  pools: PoolData[];
}

export function TVLChart({ pools }: TVLChartProps) {
  const chartData = pools
    .filter(pool => pool.tvlUSD && pool.tvlUSD > 0)
    .map(pool => ({
      name: pool.lpSymbol,
      value: pool.tvlUSD || 0,
      fullName: pool.lpName,
    }));

  if (chartData.length === 0) {
    return (
      <div className="bg-zinc-900 rounded-lg border border-zinc-800 p-6">
        <h2 className="text-lg font-semibold text-white mb-4">TVL Distribution</h2>
        <div className="text-center py-12 text-zinc-400">
          <p>TVL data not available</p>
          <p className="text-sm mt-2">Price feeds may not be configured</p>
        </div>
      </div>
    );
  }

  const totalTVL = chartData.reduce((sum, item) => sum + item.value, 0);
  const colors = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899'];

  return (
    <div className="bg-zinc-900 rounded-lg border border-zinc-800 p-6">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-lg font-semibold text-white">TVL Distribution</h2>
        <p className="text-sm text-zinc-400">
          Total: ${totalTVL.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
        </p>
      </div>

      {/* Pool Breakdown */}
      <div className="space-y-3">
        {chartData.map((pool, index) => {
          const percentage = (pool.value / totalTVL) * 100;
          const color = colors[index % colors.length];

          return (
            <div key={pool.name}>
              <div className="flex items-center justify-between mb-1">
                <div className="flex items-center gap-2">
                  <div
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: color }}
                  />
                  <span className="text-sm text-white font-medium">{pool.name}</span>
                </div>
                <div className="text-right">
                  <p className="text-sm text-white font-medium">
                    ${pool.value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                  </p>
                  <p className="text-xs text-zinc-400">{percentage.toFixed(1)}%</p>
                </div>
              </div>
              <div className="w-full bg-zinc-800 rounded-full h-2 overflow-hidden">
                <div
                  className="h-full rounded-full"
                  style={{
                    width: `${percentage}%`,
                    backgroundColor: color,
                  }}
                />
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
