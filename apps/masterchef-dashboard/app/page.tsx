'use client';

import { useState, useEffect } from 'react';
import { PoolCard } from '../components/PoolCard';
import { StatsOverview } from '../components/StatsOverview';
import { TVLChart } from '../components/TVLChart';
import { HowItWorksSection } from '../components/HowItWorksSection';
import { fetchMetrics } from '../lib/api';
import type { MetricsData } from '../types';

export default function Dashboard() {
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
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Loading MasterChef metrics...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="bg-red-50 border border-red-200 rounded-lg p-6 max-w-md">
          <h2 className="text-red-800 font-semibold mb-2">Error Loading Dashboard</h2>
          <p className="text-red-600">{error}</p>
        </div>
      </div>
    );
  }

  if (!metrics) {
    return null;
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">MasterChef Dashboard</h1>
              <p className="text-sm text-gray-500 mt-1">
                Real-time LP Rewards Monitoring • {metrics.network.toUpperCase()}
              </p>
            </div>
            <div className="text-right">
              <p className="text-xs text-gray-500">Last Updated</p>
              <p className="text-sm font-medium text-gray-700">
                {new Date(metrics.timestamp).toLocaleTimeString()}
              </p>
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Stats Overview */}
        <StatsOverview metrics={metrics} />

        {/* TVL Chart */}
        <div className="mt-8">
          <TVLChart pools={metrics.pools} />
        </div>

        {/* How It Works Section */}
        <div className="mt-8">
          <HowItWorksSection />
        </div>

        {/* Pool Cards */}
        <div className="mt-8">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">LP Pools</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {metrics.pools.map((pool) => (
              <PoolCard key={pool.poolId} pool={pool} emissions={metrics.emissions} />
            ))}
          </div>
        </div>

        {/* Contract Info */}
        <div className="mt-8 bg-white rounded-lg shadow-sm p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Contract Information</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
            <div>
              <p className="text-gray-500">MasterChef Contract</p>
              <p className="font-mono text-xs text-gray-700 break-all">
                {metrics.contracts.masterChef}
              </p>
            </div>
            <div>
              <p className="text-gray-500">ÉTR Token Contract</p>
              <p className="font-mono text-xs text-gray-700 break-all">
                {metrics.contracts.etrToken}
              </p>
            </div>
            <div>
              <p className="text-gray-500">Block Number</p>
              <p className="font-medium text-gray-700">{metrics.blockNumber.toLocaleString()}</p>
            </div>
            <div>
              <p className="text-gray-500">Chain ID</p>
              <p className="font-medium text-gray-700">{metrics.chainId}</p>
            </div>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white border-t border-gray-200 mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <p className="text-center text-sm text-gray-500">
            Ëtrid Protocol • MasterChef LP Rewards • Built with Next.js
          </p>
        </div>
      </footer>
    </div>
  );
}
