'use client';

import { PieChart, Pie, Cell, ResponsiveContainer, Legend, Tooltip } from 'recharts';
import type { PoolData } from '../types';

interface TVLChartProps {
  pools: PoolData[];
}

const COLORS = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899'];

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
      <div className="bg-white rounded-lg shadow-sm p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">TVL Distribution</h2>
        <div className="text-center py-12 text-gray-500">
          <p>TVL data not available</p>
          <p className="text-sm mt-2">Price feeds may not be configured</p>
        </div>
      </div>
    );
  }

  const totalTVL = chartData.reduce((sum, item) => sum + item.value, 0);

  return (
    <div className="bg-white rounded-lg shadow-sm p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold text-gray-900">TVL Distribution</h2>
        <p className="text-sm text-gray-500">
          Total: ${totalTVL.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
        </p>
      </div>

      <ResponsiveContainer width="100%" height={300}>
        <PieChart>
          <Pie
            data={chartData}
            cx="50%"
            cy="50%"
            labelLine={false}
            label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
            outerRadius={100}
            fill="#8884d8"
            dataKey="value"
          >
            {chartData.map((entry, index) => (
              <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
            ))}
          </Pie>
          <Tooltip
            formatter={(value: number) => [
              `$${value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`,
              'TVL'
            ]}
          />
          <Legend />
        </PieChart>
      </ResponsiveContainer>

      {/* Pool Breakdown */}
      <div className="mt-6 space-y-2">
        {chartData.map((pool, index) => (
          <div key={pool.name} className="flex items-center justify-between text-sm">
            <div className="flex items-center">
              <div
                className="w-3 h-3 rounded-full mr-2"
                style={{ backgroundColor: COLORS[index % COLORS.length] }}
              ></div>
              <span className="text-gray-700 font-medium">{pool.name}</span>
            </div>
            <div className="text-right">
              <p className="text-gray-900 font-medium">
                ${pool.value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
              </p>
              <p className="text-gray-500 text-xs">
                {((pool.value / totalTVL) * 100).toFixed(1)}%
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
