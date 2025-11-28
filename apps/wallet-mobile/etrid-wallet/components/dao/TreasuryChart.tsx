/**
 * TreasuryChart Component
 * Displays treasury asset allocation with pie chart
 */

'use client';

import React from 'react';
import { TreasuryAsset } from '@/types/dao';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { PieChart, Pie, Cell, ResponsiveContainer, Legend, Tooltip } from 'recharts';

interface TreasuryChartProps {
  assets: TreasuryAsset[];
  totalValue: string;
  change24h?: number;
}

const COLORS = [
  '#8b5cf6', // purple
  '#3b82f6', // blue
  '#10b981', // green
  '#f59e0b', // amber
  '#ef4444', // red
  '#ec4899', // pink
  '#14b8a6', // teal
  '#f97316', // orange
];

export function TreasuryChart({ assets, totalValue, change24h }: TreasuryChartProps) {
  const chartData = assets.map((asset) => ({
    name: asset.symbol,
    value: parseFloat(asset.valueUsd),
    percentage: asset.percentage,
  }));

  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div className="bg-popover border rounded-lg shadow-lg p-3">
          <p className="font-semibold">{data.name}</p>
          <p className="text-sm text-muted-foreground">
            ${data.value.toLocaleString()}
          </p>
          <p className="text-xs text-muted-foreground">
            {data.percentage.toFixed(2)}%
          </p>
        </div>
      );
    }
    return null;
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Treasury Overview</CardTitle>
        <div className="flex items-baseline gap-2">
          <span className="text-3xl font-bold">
            ${parseFloat(totalValue).toLocaleString()}
          </span>
          {change24h !== undefined && (
            <span
              className={`text-sm font-medium ${
                change24h >= 0 ? 'text-green-600' : 'text-red-600'
              }`}
            >
              {change24h >= 0 ? '+' : ''}
              {change24h.toFixed(2)}%
            </span>
          )}
        </div>
      </CardHeader>

      <CardContent>
        {assets.length > 0 ? (
          <div className="space-y-6">
            {/* Pie Chart */}
            <div className="h-64">
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={chartData}
                    cx="50%"
                    cy="50%"
                    labelLine={false}
                    label={({ percentage }) =>
                      percentage > 5 ? `${percentage.toFixed(0)}%` : ''
                    }
                    outerRadius={80}
                    fill="#8884d8"
                    dataKey="value"
                  >
                    {chartData.map((entry, index) => (
                      <Cell
                        key={`cell-${index}`}
                        fill={COLORS[index % COLORS.length]}
                      />
                    ))}
                  </Pie>
                  <Tooltip content={<CustomTooltip />} />
                </PieChart>
              </ResponsiveContainer>
            </div>

            {/* Asset List */}
            <div className="space-y-2">
              {assets.map((asset, index) => (
                <div
                  key={asset.asset}
                  className="flex items-center justify-between p-3 rounded-lg bg-muted/50"
                >
                  <div className="flex items-center gap-3">
                    <div
                      className="w-3 h-3 rounded-full"
                      style={{ backgroundColor: COLORS[index % COLORS.length] }}
                    />
                    <div>
                      <p className="font-medium">{asset.symbol}</p>
                      <p className="text-xs text-muted-foreground">
                        {parseFloat(asset.amount).toLocaleString()} {asset.symbol}
                      </p>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="font-medium">
                      ${parseFloat(asset.valueUsd).toLocaleString()}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {asset.percentage.toFixed(2)}%
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        ) : (
          <div className="text-center py-12 text-muted-foreground">
            <p>No assets in treasury</p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
