'use client';

import { useState } from 'react';
import { useValidatorStats } from '@/hooks/validator/useValidatorStats';
import ValidatorStats from '@/components/validator/validator-stats';
import NominatorList from '@/components/validator/nominator-list';
import RewardHistory from '@/components/validator/reward-history';
import AlertsPanel from '@/components/validator/alerts-panel';
import { RefreshCw, TrendingUp, Clock, Award } from 'lucide-react';
import { formatDuration } from '@/lib/validator/format';

export default function ValidatorDashboard() {
  const [validatorAddress, setValidatorAddress] = useState<string | undefined>(
    process.env.NEXT_PUBLIC_VALIDATOR_ADDRESS
  );

  const {
    isConnected,
    isLoading,
    error,
    validatorInfo,
    nominators,
    rewards,
    performance,
    sessionInfo,
    networkStats,
    refreshData,
  } = useValidatorStats(validatorAddress);

  const [isRefreshing, setIsRefreshing] = useState(false);

  const handleRefresh = async () => {
    setIsRefreshing(true);
    await refreshData();
    setTimeout(() => setIsRefreshing(false), 1000);
  };

  return (
    <div className="container mx-auto p-8">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-bold text-zinc-900 dark:text-white">Validator Dashboard</h1>
          <p className="text-zinc-600 dark:text-zinc-400 mt-1">
            Monitor your validator performance and manage settings
          </p>
        </div>

        <button
          onClick={handleRefresh}
          disabled={isRefreshing}
          className="flex items-center space-x-2 px-4 py-2 bg-white dark:bg-zinc-800 border border-zinc-300 dark:border-zinc-700 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-700 transition-colors disabled:opacity-50"
        >
          <RefreshCw className={`w-4 h-4 ${isRefreshing ? 'animate-spin' : ''}`} />
          <span className="text-zinc-900 dark:text-white">Refresh</span>
        </button>
      </div>

      {/* Error State */}
      {error && (
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-6">
          <p className="text-sm font-medium text-red-800 dark:text-red-400">Connection Error</p>
          <p className="text-sm text-red-700 dark:text-red-500 mt-1">{error}</p>
        </div>
      )}

      {/* Session Info Banner */}
      {sessionInfo && (
        <div className="bg-gradient-to-r from-blue-500 to-blue-700 dark:from-blue-600 dark:to-blue-800 rounded-lg p-6 text-white mb-6">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <div>
              <div className="flex items-center space-x-2 mb-2">
                <Clock className="w-5 h-5" />
                <span className="text-sm font-medium opacity-90">Current Era</span>
              </div>
              <p className="text-2xl font-bold">{sessionInfo.currentEra}</p>
              <p className="text-xs opacity-75 mt-1">
                {(sessionInfo.eraProgress * 100).toFixed(1)}% complete
              </p>
            </div>

            <div>
              <div className="flex items-center space-x-2 mb-2">
                <TrendingUp className="w-5 h-5" />
                <span className="text-sm font-medium opacity-90">Session</span>
              </div>
              <p className="text-2xl font-bold">{sessionInfo.currentSession}</p>
              <p className="text-xs opacity-75 mt-1">
                {(sessionInfo.sessionProgress * 100).toFixed(1)}% complete
              </p>
            </div>

            <div>
              <div className="flex items-center space-x-2 mb-2">
                <Clock className="w-5 h-5" />
                <span className="text-sm font-medium opacity-90">Next Era</span>
              </div>
              <p className="text-2xl font-bold">
                {formatDuration(sessionInfo.timeToNextEra)}
              </p>
              <p className="text-xs opacity-75 mt-1">Approximately</p>
            </div>

            <div>
              <div className="flex items-center space-x-2 mb-2">
                <Award className="w-5 h-5" />
                <span className="text-sm font-medium opacity-90">Active Validators</span>
              </div>
              <p className="text-2xl font-bold">{networkStats?.activeValidators || 0}</p>
              <p className="text-xs opacity-75 mt-1">
                {networkStats?.waitingValidators || 0} waiting
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Validator Stats */}
      <ValidatorStats
        validatorInfo={validatorInfo}
        performance={performance}
        isLoading={isLoading}
      />

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-6">
        {/* Left Column - 2/3 width */}
        <div className="lg:col-span-2 space-y-6">
          <RewardHistory rewards={rewards} isLoading={isLoading} />
          <NominatorList nominators={nominators} isLoading={isLoading} />
        </div>

        {/* Right Column - 1/3 width */}
        <div className="space-y-6">
          <AlertsPanel isLoading={isLoading} />

          {/* Quick Stats Card */}
          <div className="bg-white dark:bg-zinc-900 rounded-lg shadow-md p-6">
            <h3 className="text-lg font-semibold mb-4 text-zinc-900 dark:text-white">Quick Stats</h3>
            <div className="space-y-4">
              <div className="flex justify-between items-center pb-3 border-b border-zinc-200 dark:border-zinc-800">
                <span className="text-sm text-zinc-600 dark:text-zinc-400">Uptime</span>
                <span className="text-sm font-semibold text-zinc-900 dark:text-white">
                  {performance?.uptime.toFixed(2)}%
                </span>
              </div>
              <div className="flex justify-between items-center pb-3 border-b border-zinc-200 dark:border-zinc-800">
                <span className="text-sm text-zinc-600 dark:text-zinc-400">Rank</span>
                <span className="text-sm font-semibold text-zinc-900 dark:text-white">
                  #{performance?.rank || 0} / {performance?.totalValidators || 0}
                </span>
              </div>
              <div className="flex justify-between items-center pb-3 border-b border-zinc-200 dark:border-zinc-800">
                <span className="text-sm text-zinc-600 dark:text-zinc-400">Blocks Produced</span>
                <span className="text-sm font-semibold text-zinc-900 dark:text-white">
                  {performance?.blocksProduced.toLocaleString() || 0}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-zinc-600 dark:text-zinc-400">Missed Blocks</span>
                <span className="text-sm font-semibold text-red-600 dark:text-red-400">
                  {performance?.missedBlocks || 0}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
