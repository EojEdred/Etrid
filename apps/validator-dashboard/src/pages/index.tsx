import React, { useState } from 'react';
import Head from 'next/head';
import Layout from '@/components/Layout';
import ValidatorStats from '@/components/ValidatorStats';
import NominatorList from '@/components/NominatorList';
import RewardHistory from '@/components/RewardHistory';
import AlertsPanel from '@/components/AlertsPanel';
import { useValidatorStats } from '@/hooks/useValidatorStats';
import { RefreshCw, TrendingUp, Clock, Award } from 'lucide-react';
import { formatDuration } from '@/utils/format';

export default function Dashboard() {
  // Replace with actual validator address from wallet connection
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

  const [isRefreshing, setIsRefreshing] = React.useState(false);

  const handleRefresh = async () => {
    setIsRefreshing(true);
    await refreshData();
    setTimeout(() => setIsRefreshing(false), 1000);
  };

  const handleConnectWallet = () => {
    // Implement wallet connection logic
    console.log('Connect wallet');
  };

  return (
    <>
      <Head>
        <title>Validator Dashboard - Ëtrid Protocol</title>
        <meta name="description" content="Monitor and manage your Ëtrid validator" />
      </Head>

      <Layout isConnected={isConnected} onConnectWallet={handleConnectWallet}>
        <div className="space-y-6">
          {/* Page Header */}
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">Validator Dashboard</h1>
              <p className="text-gray-600 mt-1">
                Monitor your validator performance and manage settings
              </p>
            </div>

            <button
              onClick={handleRefresh}
              disabled={isRefreshing}
              className="flex items-center space-x-2 px-4 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors disabled:opacity-50"
            >
              <RefreshCw className={`w-4 h-4 ${isRefreshing ? 'animate-spin' : ''}`} />
              <span>Refresh</span>
            </button>
          </div>

          {/* Error State */}
          {error && (
            <div className="bg-danger-50 border border-danger-200 rounded-lg p-4">
              <p className="text-sm font-medium text-danger-800">Connection Error</p>
              <p className="text-sm text-danger-700 mt-1">{error}</p>
            </div>
          )}

          {/* Session Info Banner */}
          {sessionInfo && (
            <div className="bg-gradient-to-r from-etrid-500 to-etrid-700 rounded-lg p-6 text-white">
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
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            {/* Left Column - 2/3 width */}
            <div className="lg:col-span-2 space-y-6">
              {/* Reward History Chart */}
              <RewardHistory rewards={rewards} isLoading={isLoading} />

              {/* Nominator List */}
              <NominatorList nominators={nominators} isLoading={isLoading} />
            </div>

            {/* Right Column - 1/3 width */}
            <div className="space-y-6">
              {/* Alerts Panel */}
              <AlertsPanel isLoading={isLoading} />

              {/* Quick Stats Card */}
              <div className="bg-white rounded-lg shadow-md p-6">
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Quick Stats</h3>
                <div className="space-y-4">
                  <div className="flex justify-between items-center pb-3 border-b border-gray-200">
                    <span className="text-sm text-gray-600">Uptime</span>
                    <span className="text-sm font-semibold text-gray-900">
                      {performance?.uptime.toFixed(2)}%
                    </span>
                  </div>
                  <div className="flex justify-between items-center pb-3 border-b border-gray-200">
                    <span className="text-sm text-gray-600">Rank</span>
                    <span className="text-sm font-semibold text-gray-900">
                      #{performance?.rank || 0} / {performance?.totalValidators || 0}
                    </span>
                  </div>
                  <div className="flex justify-between items-center pb-3 border-b border-gray-200">
                    <span className="text-sm text-gray-600">Blocks Produced</span>
                    <span className="text-sm font-semibold text-gray-900">
                      {performance?.blocksProduced.toLocaleString() || 0}
                    </span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-sm text-gray-600">Missed Blocks</span>
                    <span className="text-sm font-semibold text-danger-600">
                      {performance?.missedBlocks || 0}
                    </span>
                  </div>
                </div>
              </div>

              {/* Network Stats Card */}
              {networkStats && (
                <div className="bg-gradient-to-br from-purple-50 to-purple-100 rounded-lg p-6">
                  <h3 className="text-lg font-semibold text-gray-900 mb-4">Network Stats</h3>
                  <div className="space-y-3">
                    <div>
                      <p className="text-xs text-gray-600 mb-1">Staking Rate</p>
                      <p className="text-xl font-bold text-purple-700">
                        {(networkStats.stakingRate * 100).toFixed(2)}%
                      </p>
                    </div>
                    <div>
                      <p className="text-xs text-gray-600 mb-1">Inflation Rate</p>
                      <p className="text-xl font-bold text-purple-700">
                        {(networkStats.inflationRate * 100).toFixed(2)}%
                      </p>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </Layout>
    </>
  );
}
