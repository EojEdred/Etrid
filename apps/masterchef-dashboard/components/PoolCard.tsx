import type { PoolData, EmissionsData } from '../types';

interface PoolCardProps {
  pool: PoolData;
  emissions: EmissionsData;
}

export function PoolCard({ pool, emissions }: PoolCardProps) {
  return (
    <div className="bg-white rounded-lg shadow-sm p-6 hover:shadow-md transition-shadow">
      {/* Pool Header */}
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">{pool.lpSymbol}</h3>
          <p className="text-sm text-gray-500">{pool.lpName}</p>
        </div>
        <div className="bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded">
          Pool {pool.poolId}
        </div>
      </div>

      {/* TVL */}
      {pool.tvlUSD !== null && (
        <div className="mb-4">
          <p className="text-sm text-gray-500">Total Value Locked</p>
          <p className="text-2xl font-bold text-gray-900">
            ${pool.tvlUSD.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
          </p>
        </div>
      )}

      {/* APR */}
      {pool.aprPercent !== null && (
        <div className="mb-4">
          <p className="text-sm text-gray-500">Annual Percentage Rate</p>
          <p className="text-3xl font-bold text-green-600">
            {pool.aprPercent.toFixed(2)}%
          </p>
        </div>
      )}

      {/* Stats Grid */}
      <div className="grid grid-cols-2 gap-4 pt-4 border-t border-gray-100">
        <div>
          <p className="text-xs text-gray-500">LP Staked</p>
          <p className="text-sm font-medium text-gray-700">
            {parseFloat(pool.totalStaked).toLocaleString(undefined, {
              maximumFractionDigits: 2
            })}
          </p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Reward Share</p>
          <p className="text-sm font-medium text-gray-700">
            {pool.rewardShare.toFixed(1)}%
          </p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Daily Rewards</p>
          <p className="text-sm font-medium text-gray-700">
            {parseFloat(pool.dailyRewards).toLocaleString(undefined, {
              maximumFractionDigits: 0
            })} ÉTR
          </p>
        </div>
        <div>
          <p className="text-xs text-gray-500">Monthly Rewards</p>
          <p className="text-sm font-medium text-gray-700">
            {parseFloat(pool.monthlyRewards).toLocaleString(undefined, {
              maximumFractionDigits: 0
            })} ÉTR
          </p>
        </div>
      </div>

      {/* LP Price */}
      {pool.lpPrice !== null && (
        <div className="mt-4 pt-4 border-t border-gray-100">
          <p className="text-xs text-gray-500">LP Token Price</p>
          <p className="text-sm font-medium text-gray-700">
            ${pool.lpPrice.toFixed(4)}
          </p>
        </div>
      )}

      {/* LP Token Address */}
      <div className="mt-4">
        <p className="text-xs text-gray-500 mb-1">LP Token Address</p>
        <a
          href={`https://bscscan.com/address/${pool.lpToken}`}
          target="_blank"
          rel="noopener noreferrer"
          className="text-xs font-mono text-blue-600 hover:text-blue-800 break-all"
        >
          {pool.lpToken}
        </a>
      </div>
    </div>
  );
}
