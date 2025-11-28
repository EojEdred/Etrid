import type { MetricsData } from '../types';

interface StatsOverviewProps {
  metrics: MetricsData;
}

export function StatsOverview({ metrics }: StatsOverviewProps) {
  const totalTVL = metrics.pools.reduce((sum, pool) => sum + (pool.tvlUSD || 0), 0);

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      {/* Total TVL */}
      <div className="bg-white rounded-lg shadow-sm p-6">
        <div className="flex items-center justify-between">
          <p className="text-sm font-medium text-gray-500">Total TVL</p>
          <div className="bg-green-100 p-2 rounded-lg">
            <svg className="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
        </div>
        <p className="mt-4 text-3xl font-bold text-gray-900">
          ${totalTVL > 0 ? totalTVL.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 }) : 'N/A'}
        </p>
        <p className="mt-2 text-sm text-gray-500">
          Across {metrics.overview.totalPools} pool{metrics.overview.totalPools !== 1 ? 's' : ''}
        </p>
      </div>

      {/* Daily Emissions */}
      <div className="bg-white rounded-lg shadow-sm p-6">
        <div className="flex items-center justify-between">
          <p className="text-sm font-medium text-gray-500">Daily Emissions</p>
          <div className="bg-blue-100 p-2 rounded-lg">
            <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
            </svg>
          </div>
        </div>
        <p className="mt-4 text-3xl font-bold text-gray-900">
          {parseFloat(metrics.emissions.perDay).toLocaleString(undefined, { maximumFractionDigits: 0 })}
        </p>
        <p className="mt-2 text-sm text-gray-500">ÉTR per day</p>
      </div>

      {/* MasterChef Balance */}
      <div className="bg-white rounded-lg shadow-sm p-6">
        <div className="flex items-center justify-between">
          <p className="text-sm font-medium text-gray-500">MasterChef Balance</p>
          <div className="bg-purple-100 p-2 rounded-lg">
            <svg className="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
          </div>
        </div>
        <p className="mt-4 text-3xl font-bold text-gray-900">
          {parseFloat(metrics.balance.masterChefETR).toLocaleString(undefined, { maximumFractionDigits: 0 })}
        </p>
        <p className="mt-2 text-sm text-gray-500">ÉTR available</p>
      </div>

      {/* Days Remaining */}
      <div className="bg-white rounded-lg shadow-sm p-6">
        <div className="flex items-center justify-between">
          <p className="text-sm font-medium text-gray-500">Days Remaining</p>
          <div className={`p-2 rounded-lg ${
            metrics.balance.daysRemaining < 7 ? 'bg-red-100' :
            metrics.balance.daysRemaining < 30 ? 'bg-yellow-100' :
            'bg-green-100'
          }`}>
            <svg className={`w-5 h-5 ${
              metrics.balance.daysRemaining < 7 ? 'text-red-600' :
              metrics.balance.daysRemaining < 30 ? 'text-yellow-600' :
              'text-green-600'
            }`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
        </div>
        <p className="mt-4 text-3xl font-bold text-gray-900">
          {metrics.balance.daysRemaining}
        </p>
        <p className={`mt-2 text-sm ${
          metrics.balance.daysRemaining < 7 ? 'text-red-600 font-medium' :
          metrics.balance.daysRemaining < 30 ? 'text-yellow-600' :
          'text-gray-500'
        }`}>
          {metrics.balance.daysRemaining < 7 ? '⚠️ Top up soon!' :
           metrics.balance.daysRemaining < 30 ? 'Low balance' :
           'At current rate'}
        </p>
      </div>

      {/* ÉTR Price (if available) */}
      {metrics.prices && metrics.prices.etr > 0 && (
        <div className="bg-white rounded-lg shadow-sm p-6">
          <div className="flex items-center justify-between">
            <p className="text-sm font-medium text-gray-500">ÉTR Price</p>
            <div className="bg-indigo-100 p-2 rounded-lg">
              <svg className="w-5 h-5 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              </svg>
            </div>
          </div>
          <p className="mt-4 text-3xl font-bold text-gray-900">
            ${metrics.prices.etr.toFixed(6)}
          </p>
          <p className="mt-2 text-sm text-gray-500">Per ÉTR</p>
        </div>
      )}

      {/* BNB Price (if available) */}
      {metrics.prices && metrics.prices.bnb > 0 && (
        <div className="bg-white rounded-lg shadow-sm p-6">
          <div className="flex items-center justify-between">
            <p className="text-sm font-medium text-gray-500">BNB Price</p>
            <div className="bg-yellow-100 p-2 rounded-lg">
              <svg className="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <p className="mt-4 text-3xl font-bold text-gray-900">
            ${metrics.prices.bnb.toFixed(2)}
          </p>
          <p className="mt-2 text-sm text-gray-500">Per BNB</p>
        </div>
      )}
    </div>
  );
}
