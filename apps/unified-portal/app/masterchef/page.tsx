'use client';

import { useState, useEffect } from 'react';
import { PoolCard } from './pool-card';
import { StatsOverview } from './stats-overview';
import { TVLChart } from './tvl-chart';
import { fetchMetrics } from './api';
import type { MetricsData } from './types';
import { Loader2 } from 'lucide-react';

export default function MasterChefPage() {
  const [metrics, setMetrics] = useState<MetricsData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadMetrics = async () => {
      try {
        setLoading(true);
        const data = await fetchMetrics();
        setMetrics(data);
        setError(null);
      } catch (err) {
        console.error('Failed to load metrics:', err);
        setError('Failed to load metrics. Please try again later.');
      } finally {
        setLoading(false);
      }
    };

    loadMetrics();

    // Refresh every 60 seconds
    const interval = setInterval(loadMetrics, 60000);
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return (
      <div className="min-h-[600px] bg-black flex items-center justify-center">
        <div className="text-center">
          <Loader2 className="w-12 h-12 text-purple-500 animate-spin mx-auto mb-4" />
          <p className="text-zinc-400">Loading MasterChef metrics...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-[600px] bg-black flex items-center justify-center">
        <div className="bg-red-950 border border-red-700 rounded-lg p-6 max-w-md">
          <h2 className="text-red-400 font-semibold mb-2">Error Loading Dashboard</h2>
          <p className="text-red-300">{error}</p>
        </div>
      </div>
    );
  }

  if (!metrics) {
    return null;
  }

  return (
    <div className="min-h-screen bg-black">
      {/* Header */}
      <header className="bg-zinc-900 border-b border-zinc-800 sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-white">MasterChef Dashboard</h1>
              <p className="text-sm text-zinc-400 mt-1">
                Real-time LP Rewards Monitoring • {metrics.network.toUpperCase()}
              </p>
            </div>
            <div className="text-right">
              <p className="text-xs text-zinc-400">Last Updated</p>
              <p className="text-sm font-medium text-white">
                {new Date(metrics.timestamp).toLocaleTimeString()}
              </p>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Stats Overview */}
        <StatsOverview metrics={metrics} />

        {/* TVL Chart */}
        <div className="mt-8">
          <TVLChart pools={metrics.pools} />
        </div>

        {/* Pool Cards */}
        <div className="mt-8">
          <h2 className="text-lg font-semibold text-white mb-4">LP Pools</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {metrics.pools.map((pool) => (
              <PoolCard key={pool.poolId} pool={pool} emissions={metrics.emissions} />
            ))}
          </div>
        </div>

        {/* Contract Info */}
        <div className="mt-8 bg-zinc-900 rounded-lg border border-zinc-800 p-6">
          <h2 className="text-lg font-semibold text-white mb-4">Contract Information</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
            <div>
              <p className="text-zinc-400">MasterChef Contract</p>
              <p className="font-mono text-xs text-zinc-300 break-all mt-1">
                {metrics.contracts.masterChef}
              </p>
            </div>
            <div>
              <p className="text-zinc-400">ÉTR Token Contract</p>
              <p className="font-mono text-xs text-zinc-300 break-all mt-1">
                {metrics.contracts.etrToken}
              </p>
            </div>
            <div>
              <p className="text-zinc-400">Block Number</p>
              <p className="font-medium text-white mt-1">{metrics.blockNumber.toLocaleString()}</p>
            </div>
            <div>
              <p className="text-zinc-400">Chain ID</p>
              <p className="font-medium text-white mt-1">{metrics.chainId}</p>
            </div>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-zinc-900 border-t border-zinc-800 mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <p className="text-center text-sm text-zinc-500">
            ÉTRID Protocol &bull; MasterChef LP Rewards &bull; Built with Next.js 16
          </p>
        </div>
      </footer>
    </div>
  );
}
